use attendance_service::schema::{
    representative::{RepresentativeWithRelation, UpdateRepresentative},
    student::StudentResponse,
};
use axum::http::StatusCode;

use crate::common::{
    app::spawn_app,
    helpers::{create_representative, create_student, user_logged},
};

mod common;

#[tokio::test]
async fn test_create_representative() {
    let app = spawn_app().await;
    let token = user_logged(&app.server).await;

    let student = create_student(&app.server, &token).await;
    let student_id = student.json::<StudentResponse>().student.id;
    let response = create_representative(&app.server, &token, &student_id).await;

    student.assert_status(StatusCode::CREATED);
    response.assert_status(StatusCode::CREATED);
}

#[tokio::test]
async fn test_update_representative() {
    let app = spawn_app().await;
    let token = user_logged(&app.server).await;

    let student = create_student(&app.server, &token).await;
    let student_id = student.json::<StudentResponse>().student.id;
    let representative = create_representative(&app.server, &token, &student_id).await;

    student.assert_status(StatusCode::CREATED);
    representative.assert_status(StatusCode::CREATED);

    let updated_representative = UpdateRepresentative {
        first_name: "Mother".to_string(),
        last_name: "Doe".to_string(),
        phone: serde_json::json!(vec![151515156, 123465789]),
    };

    let representative_id = representative
        .json::<RepresentativeWithRelation>()
        .representative
        .id;

    let response = app
        .server
        .patch(&format!("/representative/{}", representative_id))
        .authorization_bearer(&token)
        .json(&updated_representative)
        .await;

    response.assert_status_ok();
}

#[tokio::test]
async fn test_delete_representative() {
    let app = spawn_app().await;
    let token = user_logged(&app.server).await;

    let student = create_student(&app.server, &token).await;
    let student_id = student.json::<StudentResponse>().student.id;
    let representative = create_representative(&app.server, &token, &student_id).await;

    student.assert_status(StatusCode::CREATED);
    representative.assert_status(StatusCode::CREATED);

    let representative_id = representative
        .json::<RepresentativeWithRelation>()
        .representative
        .id;

    let response = app
        .server
        .delete(&format!("/representative/{}", representative_id))
        .authorization_bearer(&token)
        .await;

    response.assert_status_ok();
}

#[tokio::test]
async fn test_list_representatives() {
    let app = spawn_app().await;
    let token = user_logged(&app.server).await;

    let student = create_student(&app.server, &token).await;
    let student_id = student.json::<StudentResponse>().student.id;
    let representative = create_representative(&app.server, &token, &student_id).await;

    student.assert_status(StatusCode::CREATED);
    representative.assert_status(StatusCode::CREATED);

    let response = app
        .server
        .get("/representative")
        .authorization_bearer(&token)
        .await;

    response.assert_status_ok();
}

#[tokio::test]
async fn test_get_representative_by_id() {
    let app = spawn_app().await;
    let token = user_logged(&app.server).await;

    let student = create_student(&app.server, &token).await;
    let student_id = student.json::<StudentResponse>().student.id;
    let representative = create_representative(&app.server, &token, &student_id).await;

    student.assert_status(StatusCode::CREATED);
    representative.assert_status(StatusCode::CREATED);

    let representative_id = representative
        .json::<RepresentativeWithRelation>()
        .representative
        .id;

    let response = app
        .server
        .get(&format!("/representative/{}", representative_id))
        .authorization_bearer(&token)
        .await;

    response.assert_status_ok();
}
