use axum::{
    Json,
    extract::{Path, State},
    http::{HeaderMap, header::AUTHORIZATION},
};
use uuid::Uuid;
use validator::Validate;

use crate::{
    schema::{
        app::{AppError, AppState},
        user::{NewUser, UpdateUser, User},
    },
    utils::decode_token,
};

#[axum::debug_handler]
pub async fn list_users(State(state): State<AppState>) -> Result<Json<Vec<User>>, AppError> {
    let full_users = User::find_all(&state.pool).await?;
    Ok(Json(full_users))
}

#[axum::debug_handler]
pub async fn create_user(
    State(state): State<AppState>,
    Json(new_user): Json<NewUser>,
) -> Result<Json<User>, AppError> {
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

#[axum::debug_handler]
pub async fn update_user(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(user_to_update): Json<UpdateUser>,
) -> Result<Json<UpdateUser>, AppError> {
    user_to_update.validate()?;
    let updated_user = User::update(&state.pool, id, user_to_update).await?;
    Ok(Json(updated_user))
}

#[axum::debug_handler]
pub async fn delete_user(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<String, AppError> {
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
