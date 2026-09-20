use anyhow::{Context, Ok};
use attendance_service::{app, schema::{app::AppState}};
use dotenvy::{dotenv, var};

use sqlx::{postgres::PgPoolOptions};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();

    let db = var("DATABASE_URL").context("Expected database url.")?;
    let api = var("API_URL").context("Expected API url")?;
    let secret_key = var("SECRET_KEY").context("Expected secret key.")?;

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

    let appstate = AppState {
        pool,
        secret: secret_key
    };

    let listener = tokio::net::TcpListener::bind(&api).await.context(format!("Failed to listen on port: {}", &api))?;
    println!("Listening on http://{}", &api);
    axum::serve(listener, app(appstate)).await.context("Failed to serve the app")?;

    Ok(())
}
