use axum::{
    Extension, Json,
    body::Bytes,
    extract::{Path, Query, State},
    http::{
        HeaderMap,
        header::{AUTHORIZATION, CONTENT_TYPE},
    },
};
use uuid::Uuid;
use validator::Validate;

use crate::{
    schema::{
        app::{AppError, AppState},
        pagination::{Page, PageQuery},
        user::{AvatarUpload, Claims, NewUser, Role, UpdateUser, User},
    },
    utils::{decode_token, hash_password, store_avatar},
};

#[axum::debug_handler]
pub async fn list_users(
    State(state): State<AppState>,
    Query(query): Query<PageQuery>,
) -> Result<Json<Page<User>>, AppError> {
    let users = User::find_page(&state.pool, &query).await?;
    Ok(Json(users))
}

#[axum::debug_handler]
pub async fn create_user(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(new_user): Json<NewUser>,
) -> Result<Json<User>, AppError> {
    let allowed = claims
        .role()
        .is_some_and(|role| role.can_create(&new_user.role));
    if !allowed {
        return Err(AppError::Forbidden);
    }
    new_user.validate()?;
    let create_user = User::new(&state.pool, new_user).await?;
    Ok(Json(create_user))
}

#[axum::debug_handler]
pub async fn get_user_by_id(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<User>, AppError> {
    let user_by_id = User::get_by_id(&state.pool, id).await?;
    Ok(Json(user_by_id))
}

/// Editing and deleting follow the same rule as creating (Role::can_create): the SuperAdmin
/// manages admins and users, an admin only users, and nobody manages the SuperAdmin here.
fn may_manage(claims: &Claims, role: &Role) -> Result<(), AppError> {
    match claims.role() {
        Some(caller) if caller.can_create(role) => Ok(()),
        _ => Err(AppError::Forbidden),
    }
}

#[axum::debug_handler]
pub async fn update_user(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<Uuid>,
    Json(mut changes): Json<UpdateUser>,
) -> Result<Json<User>, AppError> {
    changes.validate()?; // before hashing, the rules are for the plain password
    may_manage(&claims, &User::get_by_id(&state.pool, id).await?.role)?;
    // an admin can't promote a user to admin either
    if let Some(role) = &changes.role {
        may_manage(&claims, role)?;
    }
    if let Some(password) = changes.password.take() {
        changes.password =
            Some(hash_password(&password).map_err(|e| AppError::Internal(e.into()))?);
    }

    User::update(&state.pool, id, changes).await?;
    // the full user, not the UPDATE's RETURNING, which carries the password hash
    Ok(Json(User::get_by_id(&state.pool, id).await?))
}

#[axum::debug_handler]
pub async fn delete_user(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<Uuid>,
) -> Result<String, AppError> {
    may_manage(&claims, &User::get_by_id(&state.pool, id).await?.role)?;
    User::delete(&state.pool, id).await?;
    Ok("User deleted".to_string())
}

#[axum::debug_handler]
pub async fn get_me(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<Json<User>, AppError> {
    let token = headers
        .get(AUTHORIZATION)
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.strip_prefix("Bearer "))
        .unwrap();

    let claim = decode_token(state.clone(), &token).unwrap();
    let user_id: Uuid = claim.id.parse().unwrap();
    let my_user = User::get_by_id(&state.pool, user_id).await?;
    Ok(Json(my_user))
}

const AVATAR_TYPES: [(&str, &str); 3] = [
    ("image/jpeg", "jpg"),
    ("image/png", "png"),
    ("image/webp", "webp"),
];

/// Checks the image and stores it in the Supabase bucket, answers its public URL.
/// The body is the image itself (the web app sends a 256px JPEG), the request body
/// limit (1 MB, lib.rs) caps the file size.
// clean up unreferenced files with a job if storage ever matters
async fn stored_avatar(headers: &HeaderMap, image: Bytes) -> Result<String, AppError> {
    let content_type = headers
        .get(CONTENT_TYPE)
        .and_then(|value| value.to_str().ok())
        .unwrap_or_default();
    let Some((_, extension)) = AVATAR_TYPES.iter().find(|(kind, _)| *kind == content_type) else {
        return Err(AppError::BadRequest(
            "The avatar must be a JPEG, PNG or WebP image",
        ));
    };
    if image.is_empty() {
        return Err(AppError::BadRequest("The avatar file is empty"));
    }

    let name = format!("{}.{extension}", Uuid::new_v4());
    store_avatar(&name, content_type, image)
        .await
        .map_err(AppError::Internal)
}

/// POST /user/avatar (admins), the URL goes into NewUser.avatar of the user being created
pub async fn upload_avatar(
    headers: HeaderMap,
    image: Bytes,
) -> Result<Json<AvatarUpload>, AppError> {
    let url = stored_avatar(&headers, image).await?;
    Ok(Json(AvatarUpload { url }))
}

/// POST /user/me/avatar, any logged-in user sets their own picture. Students use it right
/// after activating their invite, the register page has no admin around to do it.
pub async fn upload_my_avatar(
    State(state): State<AppState>,
    Extension(claims): Extension<Claims>,
    headers: HeaderMap,
    image: Bytes,
) -> Result<Json<AvatarUpload>, AppError> {
    let user_id: Uuid = claims.id.parse().map_err(|_| AppError::Unauthenticated)?;
    let url = stored_avatar(&headers, image).await?;
    let avatar_only = UpdateUser {
        first_name: None,
        last_name: None,
        email: None,
        password: None,
        avatar: Some(url.clone()),
        role: None,
    };
    User::update(&state.pool, user_id, avatar_only).await?;
    Ok(Json(AvatarUpload { url }))
}
