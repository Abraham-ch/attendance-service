use axum::{Json, extract::State, http::StatusCode};

use crate::{repository::user::find_by_email, schema::{app::AppState, user::{AuthResponse, AuthUser, Claims}}, utils::{create_token, verify_password}};

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

    let claim = Claims{
        id: log_user.id.to_string(),
        param: format!("{:?}", log_user.role),
        exp: 86400
    };

    let token = create_token(claim, state);

    match token {
        Ok(result) => Ok((StatusCode::OK, Json(AuthResponse {user: log_user, token: result}))),
        Err(_) => Err((StatusCode::INTERNAL_SERVER_ERROR, "Token couldn't be generated.".to_string()))
    }
}
