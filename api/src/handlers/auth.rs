use axum::{Json, extract::State};

use crate::{
    schema::{
        app::{AppError, AppState},
        user::{AuthResponse, AuthUser, Claims, User},
    },
    utils::{create_token, verify_password},
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

    let claim = Claims {
        id: log_user.id.to_string(),
        param: format!("{:?}", log_user.role),
        exp: 86400,
    };

    let token = create_token(claim, state)?;

    Ok(Json(AuthResponse {
        user: log_user,
        token,
    }))
}
