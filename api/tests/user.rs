use attendance_service::schema::{
    pagination::Page,
    student::StudentResponse,
    user::{AuthResponse, InviteInfo, Role, User},
};
use axum::http::StatusCode;
use chrono::{Duration, Utc};
use serde_json::json;
use uuid::Uuid;

use crate::common::{
    app::spawn_app,
    helpers::{
        admin_logged, create_student, create_user, create_user_with_role, superadmin_logged,
        user_logged,
    },
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

#[tokio::test]
async fn test_avatar_upload_only_takes_images_from_admins() {
    let app = spawn_app().await;
    let admin = admin_logged(&app).await;
    let user = user_logged(&app).await;

    app.server
        .post("/user/avatar")
        .authorization_bearer(&user)
        .content_type("image/jpeg")
        .bytes(vec![1, 2, 3].into())
        .await
        .assert_status(StatusCode::FORBIDDEN);

    // checked before anything is sent to Supabase
    app.server
        .post("/user/avatar")
        .authorization_bearer(&admin)
        .content_type("text/html")
        .bytes(b"<script>".to_vec().into())
        .await
        .assert_status(StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_any_user_can_send_their_own_avatar() {
    let app = spawn_app().await;
    let user = user_logged(&app).await;

    app.server
        .post("/user/me/avatar")
        .content_type("image/jpeg")
        .bytes(vec![1, 2, 3].into())
        .await
        .assert_status(StatusCode::UNAUTHORIZED);

    // a student gets past the role check (unlike /user/avatar), the type check still applies
    app.server
        .post("/user/me/avatar")
        .authorization_bearer(&user)
        .content_type("text/html")
        .bytes(b"<script>".to_vec().into())
        .await
        .assert_status(StatusCode::BAD_REQUEST);
}

async fn user_id(app: &crate::common::app::TestApp, email: &str) -> Uuid {
    User::find_by_email(&app.pool, email.to_string())
        .await
        .unwrap()
        .id
}

#[tokio::test]
async fn test_admins_edit_users_but_not_other_admins() {
    let app = spawn_app().await;
    let admin = admin_logged(&app).await;
    let (john, _) = create_user(&app).await;
    create_user_with_role(&app, "other.admin@example.com", Role::Admin).await;
    let john = user_id(&app, &john).await;
    let other_admin = user_id(&app, "other.admin@example.com").await;

    let response = app
        .server
        .patch(&format!("/user/{john}"))
        .authorization_bearer(&admin)
        .json(&json!({ "first_name": "Johnny" }))
        .await;
    response.assert_status_ok();
    assert_eq!(response.json::<User>().first_name, "Johnny");

    for (id, body) in [
        (john, json!({ "role": "Admin" })), // no promoting to their own rank
        (other_admin, json!({ "first_name": "Nope" })),
    ] {
        app.server
            .patch(&format!("/user/{id}"))
            .authorization_bearer(&admin)
            .json(&body)
            .await
            .assert_status(StatusCode::FORBIDDEN);
    }
    app.server
        .delete(&format!("/user/{other_admin}"))
        .authorization_bearer(&admin)
        .await
        .assert_status(StatusCode::FORBIDDEN);
}

#[tokio::test]
async fn test_superadmin_edits_admins_but_nobody_edits_the_superadmin() {
    let app = spawn_app().await;
    let root = superadmin_logged(&app).await;
    create_user_with_role(&app, "teacher@example.com", Role::Admin).await;
    let teacher = user_id(&app, "teacher@example.com").await;
    let root_id = user_id(&app, "root@example.com").await;

    app.server
        .patch(&format!("/user/{teacher}"))
        .authorization_bearer(&root)
        .json(&json!({ "role": "User" }))
        .await
        .assert_status_ok();

    app.server
        .patch(&format!("/user/{root_id}"))
        .authorization_bearer(&root)
        .json(&json!({ "role": "User" }))
        .await
        .assert_status(StatusCode::FORBIDDEN);
}

#[tokio::test]
async fn test_edited_password_is_hashed_and_never_sent_back() {
    let app = spawn_app().await;
    let admin = admin_logged(&app).await;
    let (email, _) = create_user(&app).await;
    let id = user_id(&app, &email).await;

    let response = app
        .server
        .patch(&format!("/user/{id}"))
        .authorization_bearer(&admin)
        .json(&json!({ "password": "NewPassword123!" }))
        .await;
    response.assert_status_ok();
    assert!(!response.text().contains("password"));

    // logging in only works if it was stored as an argon2 hash
    app.server
        .post("/auth")
        .json(&json!({ "email": email, "password": "NewPassword123!" }))
        .await
        .assert_status_ok();
}
