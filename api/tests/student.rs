mod common;
use crate::common::{
    app::spawn_app,
    helpers::{admin_logged, create_student, user_logged},
};
use attendance_service::schema::{
    pagination::Page,
    student::{Gender, Student, StudentResponse, UpdateStudent},
};
use axum::http::StatusCode;
use chrono::Utc;
use uuid::Uuid;

#[tokio::test]
async fn test_create_student_without_email() {
    let app = spawn_app().await;
    let token = admin_logged(&app).await;

    let response = create_student(&app.server, &token).await;
    response.assert_status(StatusCode::OK);
}

#[tokio::test]
async fn test_create_student_with_email() {
    let app = spawn_app().await;
    let token = admin_logged(&app).await;

    let student = Student {
        id: Uuid::new_v4(),
        dni: 12345678,
        first_name: "Jhon".to_string(),
        last_name: "Doe".to_string(),
        email: Some("jhon.doe@example.com".to_string()),
        gender: Gender::Male,
        phone: None,
        address: None,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    let response = app
        .server
        .post("/student")
        .authorization_bearer(token)
        .json(&student)
        .await;

    response.assert_status(StatusCode::OK);
}

#[tokio::test]
async fn test_update_student() {
    let app = spawn_app().await;
    let token = admin_logged(&app).await;
    let student = create_student(&app.server, &token).await;
    let student_id = student.json::<StudentResponse>().student.id.to_string();
    let student_id_route = format!("/student/{}", student_id);

    let student_update = UpdateStudent {
        address: Some("123 Main St".to_string()),
        phone: Some(1234567890),
    };

    let response = app
        .server
        .patch(&student_id_route)
        .authorization_bearer(token)
        .json(&student_update)
        .await;

    response.assert_status(StatusCode::OK);
}

#[tokio::test]
async fn test_delete_student() {
    let app = spawn_app().await;
    let token = admin_logged(&app).await;
    let student = create_student(&app.server, &token).await;
    let student_id = student.json::<StudentResponse>().student.id.to_string();
    let student_id_route = format!("/student/{}", student_id);

    let response = app
        .server
        .delete(&student_id_route)
        .authorization_bearer(token)
        .await;

    response.assert_status(StatusCode::OK);
}

#[tokio::test]
async fn test_get_student_by_id() {
    let app = spawn_app().await;
    let token = admin_logged(&app).await;
    let student = create_student(&app.server, &token).await;
    let student_id = student.json::<StudentResponse>().student.id.to_string();
    let student_id_route = format!("/student/{}", student_id);

    let response = app
        .server
        .get(&student_id_route)
        .authorization_bearer(token)
        .await;

    response.assert_status(StatusCode::OK);
}

#[tokio::test]
async fn test_list_students() {
    let app = spawn_app().await;
    let token = admin_logged(&app).await;
    let student = create_student(&app.server, &token).await;

    student.assert_status(StatusCode::OK);

    let response = app.server.get("/student").authorization_bearer(token).await;

    response.assert_status(StatusCode::OK);
}

#[tokio::test]
async fn test_user_role_cannot_write_students() {
    let app = spawn_app().await;
    let token = user_logged(&app).await;

    create_student(&app.server, &token)
        .await
        .assert_status(StatusCode::FORBIDDEN);

    // reading stays open to every logged-in user
    app.server
        .get("/student")
        .authorization_bearer(token)
        .await
        .assert_status(StatusCode::OK);
}

#[tokio::test]
async fn test_search_and_paginate_students() {
    let app = spawn_app().await;
    let token = admin_logged(&app).await;
    create_student(&app.server, &token).await; // Jhon Doe, dni 12345678

    let found = |search: &'static str| {
        let request = app
            .server
            .get("/student")
            .add_query_params([("search", search), ("per_page", "1")])
            .authorization_bearer(token.clone());
        async move { request.await.json::<Page<Student>>().total }
    };

    assert_eq!(found("jhon doe").await, 1); // full name, any case
    assert_eq!(found("3456").await, 1); // part of the dni
    assert_eq!(found("nobody").await, 0);
    assert_eq!(found("%").await, 0); // wildcards are literal

    let page = app
        .server
        .get("/student")
        .add_query_params([("page", "2"), ("per_page", "1")])
        .authorization_bearer(token)
        .await
        .json::<Page<Student>>();
    assert_eq!((page.total, page.items.len(), page.page), (1, 0, 2));
}
