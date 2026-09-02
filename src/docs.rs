use std::sync::Arc;

use aide::{axum::{ApiRouter, IntoApiResponse, routing::{get, get_with}}, openapi::OpenApi, scalar::Scalar};
use axum::{Extension, Json, response::IntoResponse};
use crate::schema::app::AppState;

pub fn docs(state: AppState) -> ApiRouter<AppState>{
    aide::generate::infer_responses(true);

    let router: ApiRouter<AppState> = ApiRouter::new()
        .route(
            "/",
            get_with(
                Scalar::new("/docs/private/api.json")
                    .with_title("Aide Axum")
                    .axum_handler(),
                |op| op.description("This documentation page."),
            ))
        .route("/private/api.json", get(serve_docs))
        .with_state(state);

    aide::generate::infer_responses(false);

    router
}

async fn serve_docs(Extension(api): Extension<Arc<OpenApi>>) -> impl IntoApiResponse {
    Json(api).into_response()
}