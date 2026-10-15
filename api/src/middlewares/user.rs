use std::{collections::HashMap, sync::Arc};

use axum::{
    extract::{Query, Request, State},
    http::{StatusCode, header},
    middleware::Next,
    response::Response,
};

use crate::{
    schema::{app::AppState, user::Claims},
    utils::decode_token,
};

/// Rejects requests without a valid token and stores its `Claims` on the request
/// so later layers and handlers can read who is calling.
pub async fn auth_middleware(
    State(state): State<Arc<AppState>>,
    Query(query): Query<HashMap<String, String>>,
    mut req: Request,
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

    let claims = decode_token(state.as_ref().clone(), token).ok_or(StatusCode::UNAUTHORIZED)?;
    req.extensions_mut().insert(claims);

    Ok(next.run(req).await)
}

/// Only Admin and SuperAdmin get through. Layer it on a route group after `auth_middleware`:
/// `.route_layer(from_fn(admin_only))` then `.route_layer(from_fn_with_state(.., auth_middleware))`
pub async fn admin_only(req: Request, next: Next) -> Result<Response, StatusCode> {
    let Some(claims) = req.extensions().get::<Claims>() else {
        return Err(StatusCode::UNAUTHORIZED);
    };
    match claims.role() {
        Some(role) if role.is_admin() => Ok(next.run(req).await),
        _ => Err(StatusCode::FORBIDDEN),
    }
}
