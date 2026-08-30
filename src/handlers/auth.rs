use axum::{Json, extract::State, http::StatusCode};

use crate::{repository::user::find_by_email, schema::{app::AppState, user::{AuthResponse, AuthUser}}, utils::{create_token, verify_password}};

#[axum::debug_handler]
pub async fn login_user(State(state): State<AppState>, Json(user): Json<AuthUser>) -> Result<(StatusCode, Json<AuthResponse>), (StatusCode, String)> {
    let log_user = match find_by_email(&state.pool, user.email).await {
        Ok(user) => user,
        Err(_) => return Err((StatusCode::NOT_FOUND, "User not found".to_string()))
    };

    let hash = log_user.password.as_str();
    let password = user.password.as_str();

    let is_password_ok = verify_password(password, hash)
        .map_err(|_| (StatusCode::INTERNAL_SERVER_ERROR, "Failed to verify password".to_string()))?;

    if !is_password_ok {
        return Err((StatusCode::UNAUTHORIZED, "Invalid password".to_string()));
    }

    let token = create_token(log_user, state);

    token
}
