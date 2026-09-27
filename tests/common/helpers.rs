#![allow(dead_code)]

use attendance_service::schema::{
    representative::Representative,
    student::{Gender, Student},
    user::{AuthResponse, Role, User},
};
use axum::http::StatusCode;
use axum_test::{TestResponse, TestServer};
use chrono::Utc;
use uuid::Uuid;

pub async fn create_user(server: &TestServer) -> (String, String, StatusCode) {
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

    let response = server.post("/user").json(&user).await;

    (email, password, response.status_code())
}

pub async fn user_logged(server: &TestServer) -> String {
    let (email, password, _) = create_user(server).await;

    let response = server
        .post("/auth")
        .json(&serde_json::json!({
            "email": email,
            "password": password
        }))
        .await;

    let auth_response = response.json::<AuthResponse>();
    auth_response.token
}

pub async fn create_student(server: &TestServer, token: String) -> TestResponse {
    let student = Student {
        id: Uuid::new_v4(),
        dni: 12345678,
        first_name: "Jhon".to_string(),
        last_name: "Doe".to_string(),
        email: None,
        gender: Gender::Male,
        phone: None,
        address: None,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    let response = server
        .post("/student")
        .authorization_bearer(token)
        .json(&student)
        .await;

    response
}
