use attendance_service::schema::user::{Role, User};
use axum::http::StatusCode;
use chrono::Utc;
use uuid::Uuid;

mod common;

#[tokio::test]
async fn log_user() {
    let server = common::server().await;

    let response = server
        .post("/auth")
        .json(&serde_json::json!({
            "email": "john.doe@example.com",
            "password": "JohnPassword123!"
        }))
        .await;

    response.assert_status_ok();
}

#[tokio::test]
async fn create_user() {
    let server = common::server().await;

    let user_data = serde_json::json!({
        "id": Uuid::new_v4(),
        "first_name": "John",
        "last_name": "Doe",
        "email": "john.doe@example.com",
        "password": "JohnPassword123!",
        "avatar": "https://images.unsplash.com/photo-1701615004837-40d8573b6652?q=80&w=1160&auto=format&fit=crop&ixlib=rb-4.1.0&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D",
        "role": Role::User,
        "created_at": Utc::now(),
        "updated_at": Utc::now(),
    });

    let user: User = serde_json::from_value(user_data).unwrap();

    let response = server.post("/user").json(&user).await;

    assert_eq!(response.status_code(), StatusCode::CREATED)
}
