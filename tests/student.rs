mod common;
use crate::common::{
    app::spawn_app,
    helpers::{create_student, user_logged},
};
use attendance_service::schema::student::{Gender, Student, StudentResponse, UpdateStudent};
use axum::http::StatusCode;
use chrono::Utc;
use uuid::Uuid;

#[tokio::test]
async fn test_create_student_without_email() {
    let app = spawn_app().await;
    let token = user_logged(&app.server).await;

    let response = create_student(&app.server, token).await;
    response.assert_status(StatusCode::CREATED);
}

#[tokio::test]
async fn test_create_student_with_email() {
    let app = spawn_app().await;
    let token = user_logged(&app.server).await;

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

    response.assert_status(StatusCode::CREATED);
}

#[tokio::test]
async fn test_update_student() {
    let app = spawn_app().await;
    let token = user_logged(&app.server).await;
    let student = create_student(&app.server, token.clone()).await;
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
    let token = user_logged(&app.server).await;
    let student = create_student(&app.server, token.clone()).await;
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
    let token = user_logged(&app.server).await;
    let student = create_student(&app.server, token.clone()).await;
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
    let token = user_logged(&app.server).await;
    let student = create_student(&app.server, token.clone()).await;

    student.assert_status(StatusCode::CREATED);

    let response = app.server.get("/student").authorization_bearer(token).await;

    response.assert_status(StatusCode::OK);
}
