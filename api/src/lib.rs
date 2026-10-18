use std::{sync::Arc, time::Duration};

use aide::{
    axum::{
        ApiRouter,
        routing::{delete, get, patch, post},
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
    trace::{DefaultMakeSpan, DefaultOnRequest, DefaultOnResponse, TraceLayer},
};
use uuid::Uuid;

use crate::{
    docs::docs,
    handlers::{
        assistance::{
            check_in, get_daily_code, get_my_assistances, get_period_assistances, ws_handler,
        },
        auth::{get_invite, login_user, register_user},
        period::{create_period, delete_period, get_periods},
        representative::{
            create_representative, delete_representative, get_representative_by_id,
            list_representatives, update_representative,
        },
        student::{
            create_student, delete_student, get_student_by_id, list_students, update_student,
        },
        user::{
            create_user, delete_user, get_me, get_user_by_id, list_users, update_user,
            upload_avatar, upload_my_avatar,
        },
    },
    middlewares::user::{admin_only, auth_middleware},
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

    // Admin/SuperAdmin only, a student reads their own data through /assistance/me
    let student_routes = ApiRouter::new()
        .api_route("/", get(list_students).post(create_student))
        .api_route(
            "/{id}",
            get(get_student_by_id)
                .patch(update_student)
                .delete(delete_student),
        )
        .route_layer(axum::middleware::from_fn(admin_only))
        .route_layer(axum::middleware::from_fn_with_state(
            Arc::new(appstate.clone()),
            auth_middleware,
        ));

    // students only see their own data (/assistance/me), the records pages are staff only
    let representative_routes = ApiRouter::new()
        .api_route("/", get(list_representatives).post(create_representative))
        .api_route(
            "/{id}",
            patch(update_representative)
                .get(get_representative_by_id)
                .delete(delete_representative),
        )
        .route_layer(axum::middleware::from_fn(admin_only))
        .route_layer(axum::middleware::from_fn_with_state(
            Arc::new(appstate.clone()),
            auth_middleware,
        ));

    // everyone can read the periods (the dashboard shows the current one's name)
    let period_admin_routes = ApiRouter::new()
        .api_route("/", post(create_period))
        .api_route("/{id}", delete(delete_period))
        .api_route("/{id}/assistance", get(get_period_assistances))
        .route_layer(axum::middleware::from_fn(admin_only));

    let period_routes = ApiRouter::new()
        .api_route("/", get(get_periods))
        .merge(period_admin_routes)
        .route_layer(axum::middleware::from_fn_with_state(
            Arc::new(appstate.clone()),
            auth_middleware,
        ));

    // managing accounts is for Admin/SuperAdmin, create_user also checks which roles they may give
    let user_admin_routes = ApiRouter::new()
        .api_route_with("/", get(list_users).post(create_user), |op| {
            op.description("List or create users")
        })
        .api_route(
            "/{id}",
            get(get_user_by_id).patch(update_user).delete(delete_user),
        )
        .route("/avatar", axum::routing::post(upload_avatar))
        .route_layer(axum::middleware::from_fn(admin_only));

    let user_routes: ApiRouter<AppState> = ApiRouter::new()
        .api_route("/get-me", get(get_me))
        .route("/me/avatar", axum::routing::post(upload_my_avatar))
        .merge(user_admin_routes)
        .route_layer(axum::middleware::from_fn_with_state(
            Arc::new(appstate.clone()),
            auth_middleware,
        ));

    let auth_route: ApiRouter<AppState> = ApiRouter::new()
        .api_route("/", post(login_user))
        .api_route("/invite", get(get_invite))
        .api_route("/register", post(register_user));

    // the socket marks any student and streams the whole roll, so it is staff only;
    // students check themselves in with the daily QR
    let assistance_admin_routes: ApiRouter<AppState> = ApiRouter::new()
        .route("/", axum::routing::get(ws_handler))
        .api_route("/daily-code", get(get_daily_code))
        .route_layer(axum::middleware::from_fn(admin_only));

    let assistance_routes: ApiRouter<AppState> = ApiRouter::new()
        .api_route("/me", get(get_my_assistances))
        .api_route("/check-in", post(check_in))
        .merge(assistance_admin_routes)
        .route_layer(axum::middleware::from_fn_with_state(
            Arc::new(appstate.clone()),
            auth_middleware,
        ));

    ApiRouter::new()
        .route("/", get(index))
        .nest("/user", user_routes)
        .nest("/auth", auth_route)
        .nest("/student", student_routes)
        .nest("/representative", representative_routes)
        .nest("/period", period_routes)
        .nest("/docs", docs(appstate.clone()).into())
        .nest("/assistance", assistance_routes)
        .finish_api_with(&mut open_api, api_docs)
        .layer(Extension(Arc::new(open_api)))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::new().level(tracing::Level::INFO))
                .on_request(DefaultOnRequest::new().level(tracing::Level::INFO))
                .on_response(DefaultOnResponse::new().level(tracing::Level::INFO)),
        )
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
