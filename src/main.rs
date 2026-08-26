use std::{sync::Arc, time::Duration};

use anyhow::{Context, Ok};
use dotenvy::{dotenv, var};
use axum::{Router, http::{HeaderValue, StatusCode}, routing::{get, patch, post}};

use attendance_service::{handlers::{auth::login_user, user::{create_user, delete_user, get_user_by_id, list_users, update_user}}, middlewares::user::auth_middleware, schema::app::AppState};
use sqlx::{postgres::PgPoolOptions};
use tower_http::{compression::CompressionLayer, cors::{Any, CorsLayer}, limit::RequestBodyLimitLayer, timeout::TimeoutLayer, trace::TraceLayer};

async fn index() -> &'static str { "Home" }

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();

    let db = var("DATABASE_URL").context("Expected database url.")?;
    let api = var("API_URL").context("Expected API url")?;
    let origin = var("ALLOWED_ORIGIN").context("Expected origin url for cors.")?;
    let secret_key = var("SECRET_KEY").context("Expected secret key.")?;

    /* 
    in case we have multiple origins just
    let origins = [
        "http://example.com".parse().unwrap(),
        ...
    ];
    */

    let pool = PgPoolOptions::new()
        .connect(&db)
        .await
        .expect("Failed to connect to database");

    // Run migrations
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    println!("Connected on {}", &db);

    let cors = CorsLayer::new()
    .allow_origin(origin.parse::<HeaderValue>().unwrap())
    .allow_methods(Any)
    .allow_headers(Any);

    let appstate = AppState {
        pool,
        secret: secret_key
    };

    let user_routes = Router::new()
        .route("/", patch(update_user).post(create_user));

    let admin_routes: Router<AppState> = Router::new()
        .route("/", get(list_users))
        .route("/{id}", get(get_user_by_id).delete(delete_user))
        .route_layer(axum::middleware::from_fn_with_state(Arc::new(appstate.clone()),auth_middleware));

    let auth_route: Router<AppState> = Router::new()
        .route("/", post(login_user));

    let app = Router::new()
        .route("/", get(index))
        .nest("/user", admin_routes)
        .nest("/user", user_routes)
        .nest("/auth", auth_route)
        .layer(TraceLayer::new_for_http())
        .layer(CompressionLayer::new())
        .layer(TimeoutLayer::with_status_code(StatusCode::REQUEST_TIMEOUT, Duration::from_secs(30)))
        .layer(RequestBodyLimitLayer::new(1024 * 1024))
        .layer(cors)
        .with_state(appstate);

    let listener = tokio::net::TcpListener::bind(&api).await.context(format!("Failed to listen on port: {}", &api))?;
    println!("Listening on http://{}", &api);
    axum::serve(listener, app).await.context("Failed to serve the app")?;

    Ok(())
}
