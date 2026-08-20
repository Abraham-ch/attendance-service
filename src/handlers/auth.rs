use std::time::{SystemTime, UNIX_EPOCH};

use axum::{Json, extract::State, http::StatusCode};
use jsonwebtoken::{EncodingKey, Header, encode};

use crate::{repository::user::find_by_email, schema::{app::AppState, user::{AuthUser, Claims}}, utils::verify_password};

#[axum::debug_handler]
pub async fn login_user(State(state): State<AppState>, Json(user): Json<AuthUser>) -> Result<(StatusCode, Json<serde_json::Value>), (StatusCode, String)> {
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

    let exp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as usize + 86400;

    let claim = Claims{
        sub: log_user.id.to_string(),
        role: log_user.role,
        exp: exp
    };

    let token = encode(&Header::default(), &claim, &EncodingKey::from_secret(state.secret.as_ref()));

    match token {
        Ok(result) => Ok((StatusCode::OK, Json(serde_json::json!({ "token": result })))),
        Err(_) => Err((StatusCode::INTERNAL_SERVER_ERROR, "Token couldn't be generated.".to_string()))
    }
}
