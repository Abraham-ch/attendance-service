use attendance_service::schema::{
    pagination::Page,
    student::StudentResponse,
    user::{AuthResponse, InviteInfo, Role, User},
};
use axum::http::StatusCode;
use chrono::{Duration, Utc};
use uuid::Uuid;

use crate::common::{
    app::spawn_app,
    helpers::{admin_logged, create_student, create_user, superadmin_logged, user_logged},
};

mod common;

fn new_user(email: &str, role: Role) -> serde_json::Value {
    serde_json::json!({
        "first_name": "Ana",
        "last_name": "Torres",
        "email": email,
        "password": "AnaPassword123!",
        "role": role,
    })
}

#[tokio::test]
async fn test_log_user() {
    let app = spawn_app().await;
    let (email, password) = create_user(&app).await;

    let response = app
        .server
        .post("/auth")
        .json(&serde_json::json!({
            "email": email,
            "password": password
        }))
        .await;

    response.assert_status_ok();
    assert!(
        !response.text().contains("password"),
        "hash must not leave the API"
    );
}

#[tokio::test]
async fn test_admin_creates_users_but_not_admins() {
    let app = spawn_app().await;
    let token = admin_logged(&app).await;

    let create = |email: &'static str, role: Role| {
        app.server
            .post("/user")
            .authorization_bearer(token.clone())
            .json(&new_user(email, role))
    };

    create("ana@example.com", Role::User)
        .await
        .assert_status_ok();
    create("boss@example.com", Role::Admin)
        .await
        .assert_status(StatusCode::FORBIDDEN);
}

#[tokio::test]
async fn test_superadmin_creates_admins_but_not_superadmins() {
    let app = spawn_app().await;
    let token = superadmin_logged(&app).await;

    let create = |email: &'static str, role: Role| {
        app.server
            .post("/user")
            .authorization_bearer(token.clone())
            .json(&new_user(email, role))
    };

    create("teacher@example.com", Role::Admin)
        .await
        .assert_status_ok();
    create("root2@example.com", Role::SuperAdmin)
        .await
        .assert_status(StatusCode::FORBIDDEN);
}

#[tokio::test]
async fn test_only_admins_manage_users() {
    let app = spawn_app().await;

    app.server
        .post("/user")
        .json(&new_user("ana@example.com", Role::User))
        .await
        .assert_status(StatusCode::UNAUTHORIZED);

    let token = user_logged(&app).await;
    app.server
        .get("/user")
        .authorization_bearer(token.clone())
        .await
        .assert_status(StatusCode::FORBIDDEN);

    // everyone can still read their own account
    app.server
        .get("/user/get-me")
        .authorization_bearer(token)
        .await
        .assert_status_ok();
}

#[tokio::test]
async fn test_list_users_paginated_without_passwords() {
    let app = spawn_app().await;
    let token = admin_logged(&app).await;
    create_user(&app).await; // John Doe

    let response = app
        .server
        .get("/user")
        .add_query_params([("search", "john.doe@"), ("per_page", "10")])
        .authorization_bearer(token)
        .await;

    assert!(!response.text().contains("password"));
    let page = response.json::<Page<User>>();
    assert_eq!(page.total, 1);
    assert_eq!(page.items[0].email, "john.doe@example.com");
}

#[tokio::test]
async fn test_register_with_invite_token() {
    let app = spawn_app().await;
    let admin = admin_logged(&app).await;

    // a student with an email and a pending invite, without sending a real email
    let student = create_student(&app.server, &admin)
        .await
        .json::<StudentResponse>()
        .student;
    sqlx::query("UPDATE students SET email = 'jhon@example.com' WHERE id = $1")
        .bind(student.id)
        .execute(&app.pool)
        .await
        .unwrap();
    for (token, expires_at) in [
        ("valid-token", Utc::now() + Duration::days(3)),
        ("old-token", Utc::now() - Duration::days(1)),
    ] {
        sqlx::query(
            "INSERT INTO invite_tokens (id, student_id, token, expires_at) VALUES ($1, $2, $3, $4)",
        )
        .bind(Uuid::new_v4())
        .bind(student.id)
        .bind(token)
        .bind(expires_at)
        .execute(&app.pool)
        .await
        .unwrap();
    }

    let invite = |token: &'static str| {
        app.server
            .get("/auth/invite")
            .add_query_param("token", token)
    };
    let register = |token: &'static str| {
        app.server.post("/auth/register").json(&serde_json::json!({
            "token": token,
            "password": "JhonPassword123!"
        }))
    };

    let info = invite("valid-token").await.json::<InviteInfo>();
    assert_eq!(info.email, "jhon@example.com");
    invite("old-token")
        .await
        .assert_status(StatusCode::NOT_FOUND);

    // weak password is rejected before the invite is spent
    app.server
        .post("/auth/register")
        .json(&serde_json::json!({ "token": "valid-token", "password": "short" }))
        .await
        .assert_status(StatusCode::UNPROCESSABLE_ENTITY);

    let session = register("valid-token").await.json::<AuthResponse>();
    assert_eq!(session.user.email, "jhon@example.com");
    assert_eq!(session.user.role, Role::User);

    // the link works once, and the new account can log in
    register("valid-token")
        .await
        .assert_status(StatusCode::NOT_FOUND);
    app.server
        .post("/auth")
        .json(&serde_json::json!({ "email": "jhon@example.com", "password": "JhonPassword123!" }))
        .await
        .assert_status_ok();
}
