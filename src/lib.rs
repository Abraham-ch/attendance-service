use std::{sync::Arc, time::Duration};

use aide::{
    axum::{
        ApiRouter,
        routing::{get, patch, post},
    },
    openapi::OpenApi,
    transform::TransformOpenApi,
};
use axum::{
    Extension, Json, Router,
    http::{HeaderValue, Method, StatusCode},
};
use dotenvy::var;
use tower_http::{
    compression::CompressionLayer,
    cors::{Any, CorsLayer},
    limit::RequestBodyLimitLayer,
    timeout::TimeoutLayer,
    trace::TraceLayer,
};
use uuid::Uuid;

use crate::{
    docs::docs,
    handlers::{
        auth::login_user,
        representative::{
            create_representative, delete_representative, get_representative_by_id,
            list_representatives, update_representative,
        },
        student::{
            create_student, delete_student, get_student_by_id, list_students, update_student,
        },
        user::{create_user, delete_user, get_user_by_id, list_users, update_user},
    },
    middlewares::user::auth_middleware,
    schema::{app::AppState, errors::OtherError},
};

pub mod docs;
pub mod handlers;
pub mod middlewares;
pub mod queries;
pub mod repository;
pub mod schema;
pub mod utils;

async fn index() -> &'static str {
    "Home"
}

pub fn app(appstate: AppState) -> Router {
    let origin: String = var("ALLOWED_ORIGIN").expect("Expected origin url for cors.");
    /*
        in case we have multiple origins just
        let origins = [
            "http://example.com".parse().unwrap(),
            ...
        ];
    */
    let cors = CorsLayer::new()
        .allow_origin(origin.parse::<HeaderValue>().unwrap())
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PATCH,
            Method::DELETE,
            Method::OPTIONS,
        ])
        .allow_headers(Any);

    let mut open_api = OpenApi::default();

    let user_routes = ApiRouter::new().api_route("/", post(create_user));

    let student_routes = ApiRouter::new()
        .api_route("/", get(list_students).post(create_student))
        .api_route(
            "/{id}",
            patch(update_student)
                .get(get_student_by_id)
                .delete(delete_student),
        )
        .route_layer(axum::middleware::from_fn_with_state(
            Arc::new(appstate.clone()),
            auth_middleware,
        ));

    let representative_routes = ApiRouter::new()
        .api_route("/", get(list_representatives).post(create_representative))
        .api_route(
            "/{id}",
            patch(update_representative)
                .get(get_representative_by_id)
                .delete(delete_representative),
        )
        .route_layer(axum::middleware::from_fn_with_state(
            Arc::new(appstate.clone()),
            auth_middleware,
        ));

    let admin_routes: ApiRouter<AppState> = ApiRouter::new()
        .api_route_with("/", get(list_users), |op| op.description("List all users"))
        .api_route(
            "/{id}",
            get(get_user_by_id).patch(update_user).delete(delete_user),
        )
        .route_layer(axum::middleware::from_fn_with_state(
            Arc::new(appstate.clone()),
            auth_middleware,
        ));

    let auth_route: ApiRouter<AppState> = ApiRouter::new().api_route("/", post(login_user));

    ApiRouter::new()
        .route("/", get(index))
        .nest("/user", admin_routes)
        .nest("/user", user_routes)
        .nest("/auth", auth_route)
        .nest("/student", student_routes)
        .nest("/representative", representative_routes)
        .nest("/docs", docs(appstate.clone()).into())
        .finish_api_with(&mut open_api, api_docs)
        .layer(Extension(Arc::new(open_api)))
        .layer(TraceLayer::new_for_http())
        .layer(CompressionLayer::new())
        .layer(TimeoutLayer::with_status_code(
            StatusCode::REQUEST_TIMEOUT,
            Duration::from_secs(30),
        ))
        .layer(RequestBodyLimitLayer::new(1024 * 1024))
        .layer(cors)
        .with_state(appstate)
}

fn api_docs(api: TransformOpenApi) -> TransformOpenApi {
    api.title("Aide axum Open API")
        .summary("An example Todo application")
        .description(include_str!("../README.md"))
        .security_scheme(
            "ApiKey",
            aide::openapi::SecurityScheme::ApiKey {
                location: aide::openapi::ApiKeyLocation::Header,
                name: "X-Auth-Key".into(),
                description: Some("A key that is ignored.".into()),
                extensions: Default::default(),
            },
        )
        .default_response_with::<Json<OtherError>, _>(|res| {
            res.example(OtherError {
                error: "some error happened".to_string(),
                error_details: None,
                error_id: Uuid::nil(),
                // This is not visible.
                status: StatusCode::IM_A_TEAPOT,
            })
        })
}
