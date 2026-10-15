use axum::{
    Json,
    extract::{Query, State},
};
use validator::Validate;

use crate::{
    schema::{
        app::{AppError, AppState},
        user::{AuthResponse, AuthUser, Claims, InviteInfo, InviteQuery, RegisterUser, User},
    },
    utils::{create_token, hash_password, verify_password},
};

#[axum::debug_handler]
pub async fn login_user(
    State(state): State<AppState>,
    Json(user): Json<AuthUser>,
) -> Result<Json<AuthResponse>, AppError> {
    let log_user = match User::find_by_email(&state.pool, user.email).await {
        Ok(user) => user,
        Err(_) => return Err(AppError::Unauthenticated),
    };

    let hash = log_user.password.as_str();
    let password = user.password.as_str();

    let is_password_ok =
        verify_password(password, hash).map_err(|e| AppError::Internal(e.into()))?;

    if !is_password_ok {
        return Err(AppError::Unauthenticated);
    }

    session(log_user, state)
}

/// GET /auth/invite?token= tells the register page whose invite it is
pub async fn get_invite(
    State(state): State<AppState>,
    Query(query): Query<InviteQuery>,
) -> Result<Json<InviteInfo>, AppError> {
    let invite = User::find_invite(&state.pool, &query.token).await?;
    Ok(Json(invite))
}

/// POST /auth/register turns an emailed invite into a logged-in User account
pub async fn register_user(
    State(state): State<AppState>,
    Json(register): Json<RegisterUser>,
) -> Result<Json<AuthResponse>, AppError> {
    register.validate()?;
    let hash = hash_password(&register.password).map_err(|e| AppError::Internal(e.into()))?;
    let user = User::register_from_invite(&state.pool, &register.token, hash).await?;
    session(user, state)
}

fn session(user: User, state: AppState) -> Result<Json<AuthResponse>, AppError> {
    let claim = Claims {
        id: user.id.to_string(),
        param: format!("{:?}", user.role), // read back by Claims::role
        exp: 86400,
    };

    let token = create_token(claim, state)?;

    Ok(Json(AuthResponse { user, token }))
}
