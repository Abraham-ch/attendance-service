use axum::http::StatusCode;

use crate::common::{app::spawn_app, helpers::create_user};

mod common;

#[tokio::test]
async fn test_log_user() {
    let app = spawn_app().await;
    let (email, password, _) = create_user(&app.server).await;

    let response = app
        .server
        .post("/auth")
        .json(&serde_json::json!({
            "email": email,
            "password": password
        }))
        .await;

    response.assert_status_ok();
}

#[tokio::test]
async fn test_create_user() {
    let app = spawn_app().await;
    let (_, _, status_code) = create_user(&app.server).await;

    assert_eq!(status_code, StatusCode::CREATED)
}
