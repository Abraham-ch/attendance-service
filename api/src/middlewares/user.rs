use std::{collections::HashMap, sync::Arc};

use axum::{
    extract::{Query, Request, State},
    http::{StatusCode, header},
    middleware::Next,
    response::Response,
};

use crate::{schema::app::AppState, utils::validate_token};

pub async fn auth_middleware(
    State(state): State<Arc<AppState>>,
    Query(query): Query<HashMap<String, String>>,
    req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let token = req
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix("Bearer "))
        .map(str::to_owned)
        .or_else(|| query.get("token").cloned())
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let token = token.trim_start_matches("Bearer ");

    // Validate token against your state
    if !validate_token(state, token) {
        return Err(StatusCode::UNAUTHORIZED);
    }

    Ok(next.run(req).await)
}
