use anyhow::{Context, Ok};
use attendance_service::{app, schema::app::AppState};
use dotenvy::{dotenv, var};

use sqlx::postgres::PgPoolOptions;
use tokio::sync::broadcast;

mod tui;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tui::init();
    dotenv().ok();

    let db = var("DATABASE_URL").context("Expected database url.")?;
    let api = var("API_URL").context("Expected API url")?;
    let secret_key = var("SECRET_KEY").context("Expected secret key.")?;

    let pool = PgPoolOptions::new()
        .connect(&db)
        .await
        .expect("Failed to connect to database");

    let (tx, _) = broadcast::channel::<String>(100);

    tracing::info!("Connected to database");

    let appstate = AppState {
        pool,
        secret: secret_key,
        tx,
    };

    let listener = tokio::net::TcpListener::bind(&api)
        .await
        .context(format!("Failed to listen on port: {}", &api))?;
    tracing::info!("Listening on http://{}", &api);
    axum::serve(listener, app(appstate))
        .await
        .context("Failed to serve the app")?;

    Ok(())
}
