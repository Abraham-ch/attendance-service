use anyhow::{Context, Ok};
use dotenvy::{dotenv, var};
use axum::{Router, routing::{get}};

use attendance_service::handlers::user::{create_user, delete_user, get_user_by_id, list_users, update_user};
use sqlx::postgres::PgPoolOptions;

async fn index() -> &'static str { "Home" }

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();

    let db = var("DATABASE_URL").context("Expected database url.")?;
    
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

    let api = var("API_URL").context("Expected API url")?;
    
    let user_routes = Router::new()
        .route("/", get(list_users).post(create_user))
        .route("/{id}", get(get_user_by_id).patch(update_user).delete(delete_user))
        .with_state(pool); //like adding a prop for multiple routes

    let app = Router::new()
        .route("/", get(index))
        .nest("/user", user_routes);

    let listener = tokio::net::TcpListener::bind(&api).await.context(format!("Failed to listen on port: {}", &api))?;
    println!("Listening on {}", &api);
    axum::serve(listener, app).await.context("Failed to serve the app")?;

    Ok(())
}
