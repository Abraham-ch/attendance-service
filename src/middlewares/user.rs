use std::sync::Arc;

use axum::{extract::{Request, State}, http::StatusCode, middleware::Next, response::Response};

use crate::{schema::app::AppState, utils::validate_token};

pub async fn auth_middleware(
    State(state): State<Arc<AppState>>,
    req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let token = req
        .headers()
        .get("authorization")
        .and_then(|v| v.to_str().ok())
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let token = token.trim_start_matches("Bearer ");

    // Validate token against your state
    if !validate_token(state, token) {
        return Err(StatusCode::UNAUTHORIZED);
    }
 
    Ok(next.run(req).await)
}
