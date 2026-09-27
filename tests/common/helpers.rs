use crate::common::app::TestApp;
use attendance_service::schema::user::{Role, User};
use axum::http::StatusCode;
use chrono::Utc;
use uuid::Uuid;

pub async fn create_user(server: &TestApp) -> (String, String, StatusCode) {
    let email = "john.doe@example.com".to_string();
    let password = "JohnPassword123!".to_string();

    let user_data = serde_json::json!({
        "id": Uuid::new_v4(),
        "first_name": "John",
        "last_name": "Doe",
        "email": email,
        "password": password,
        "avatar": "https://images.unsplash.com/photo-1701615004837-40d8573b6652?q=80&w=1160&auto=format&fit=crop&ixlib=rb-4.1.0&ixid=M3wxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8fA%3D%3D",
        "role": Role::User,
        "created_at": Utc::now(),
        "updated_at": Utc::now(),
    });

    let user: User = serde_json::from_value(user_data).unwrap();

    let response = server.server.post("/user").json(&user).await;

    (email, password, response.status_code())
}
