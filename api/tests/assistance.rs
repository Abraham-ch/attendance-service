use attendance_service::schema::{
    assistance::{Assistance, NewAssistance},
    period::Period,
    student::StudentResponse,
};
use axum::http::StatusCode;
use serde_json::{Value, json};

use crate::common::{
    app::spawn_app,
    helpers::{admin_logged, create_period, create_student, create_user, login, user_logged},
};

mod common;

struct Spot {
    lat: f64,
    lng: f64,
}

const CLASSROOM: Spot = Spot {
    lat: -12.0464,
    lng: -77.0428,
};

impl serde::Serialize for Spot {
    fn serialize<S: serde::Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        json!({ "lat": self.lat, "lng": self.lng }).serialize(s)
    }
}

#[tokio::test]
async fn test_student_checks_in_once_with_the_daily_code() {
    let app = spawn_app().await;
    let admin = admin_logged(&app).await;
    let student = create_student(&app.server, &admin)
        .await
        .json::<StudentResponse>()
        .student;
    create_period(&app.server, &admin).await.assert_status_ok();

    // the student's own account, as register_from_invite links it
    let (email, password) = create_user(&app).await;
    sqlx::query("UPDATE users SET student_id = $1 WHERE email = $2")
        .bind(student.id)
        .bind(&email)
        .execute(&app.pool)
        .await
        .unwrap();
    let token = login(&app.server, email, password).await;

    let code = app
        .server
        .get("/assistance/daily-code")
        .add_query_params(CLASSROOM)
        .authorization_bearer(&admin)
        .await
        .json::<Value>()["code"]
        .as_str()
        .unwrap()
        .to_string();

    let check_in = |code: String| {
        app.server
            .post("/assistance/check-in")
            .authorization_bearer(&token)
            .json(&json!({ "code": code, "lat": CLASSROOM.lat, "lng": CLASSROOM.lng }))
    };

    check_in("not-the-code".into())
        .await
        .assert_status(StatusCode::BAD_REQUEST);

    let assistance = check_in(code.clone()).await.json::<Assistance>();
    assert_eq!(assistance.student_id, student.id);

    // one record per day
    check_in(code).await.assert_status(StatusCode::BAD_REQUEST);

    let mine = app
        .server
        .get("/assistance/me")
        .authorization_bearer(&token)
        .await
        .json::<Vec<Assistance>>();
    assert_eq!(mine.len(), 1);
}

#[tokio::test]
async fn test_account_without_student_cannot_check_in() {
    let app = spawn_app().await;
    let admin = admin_logged(&app).await;
    create_period(&app.server, &admin).await.assert_status_ok();
    let token = user_logged(&app).await;
    let code = app
        .server
        .get("/assistance/daily-code")
        .add_query_params(CLASSROOM)
        .authorization_bearer(&admin)
        .await
        .json::<Value>()["code"]
        .clone();

    app.server
        .post("/assistance/check-in")
        .authorization_bearer(&token)
        .json(&json!({ "code": code, "lat": CLASSROOM.lat, "lng": CLASSROOM.lng }))
        .await
        .assert_status(StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn test_students_cannot_use_staff_routes() {
    let app = spawn_app().await;
    let token = user_logged(&app).await;

    for path in [
        "/assistance/daily-code?lat=0&lng=0",
        "/assistance",
        "/representative",
    ] {
        app.server
            .get(path)
            .authorization_bearer(&token)
            .await
            .assert_status(StatusCode::FORBIDDEN);
    }
    create_period(&app.server, &token)
        .await
        .assert_status(StatusCode::FORBIDDEN);
}

#[tokio::test]
async fn test_database_keeps_one_record_per_student_and_day() {
    let app = spawn_app().await;
    let admin = admin_logged(&app).await;
    let student = create_student(&app.server, &admin)
        .await
        .json::<StudentResponse>()
        .student;
    let period = create_period(&app.server, &admin).await.json::<Period>();
    let scan = || NewAssistance {
        student_id: student.id,
        period_id: period.id,
    };

    // what the staff socket calls, no check before it: the constraint is the only guard
    Assistance::new(&app.pool, scan()).await.unwrap();
    let second = Assistance::new(&app.pool, scan()).await.unwrap_err();
    assert!(matches!(second, sqlx::Error::Database(e) if e.is_unique_violation()));
}

#[tokio::test]
async fn test_generating_the_daily_code_records_where() {
    let app = spawn_app().await;
    let admin = admin_logged(&app).await;

    app.server
        .get("/assistance/daily-code")
        .add_query_params(CLASSROOM)
        .authorization_bearer(&admin)
        .await
        .assert_status_ok();

    let (lat, lng): (f64, f64) = sqlx::query_as("SELECT lat, lng FROM daily_code_locations")
        .fetch_one(&app.pool)
        .await
        .unwrap();
    assert_eq!((lat, lng), (CLASSROOM.lat, CLASSROOM.lng));
}
