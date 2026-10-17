#![allow(dead_code)]

use attendance_service::schema::{
    period::Period,
    representative::{
        Relationship, Representative, RepresentativeWithRelation, StudentRepresentatives,
    },
    student::{Gender, Student},
    user::{AuthResponse, NewUser, Role, User},
};
use axum_test::{TestResponse, TestServer};

use super::app::TestApp;
use chrono::Utc;
use uuid::Uuid;

pub async fn create_user(app: &TestApp) -> (String, String) {
    create_user_with_role(app, "john.doe@example.com", Role::User).await
}

/// Straight into the database: POST /user needs an admin, so the first accounts can't use it
pub async fn create_user_with_role(app: &TestApp, email: &str, role: Role) -> (String, String) {
    let password = "JohnPassword123!".to_string();
    let new_user = NewUser {
        first_name: "John".to_string(),
        last_name: "Doe".to_string(),
        email: email.to_string(),
        password: password.clone(),
        avatar: String::new(),
        role,
    };
    User::new(&app.pool, new_user).await.unwrap();
    (email.to_string(), password)
}

pub async fn user_logged(app: &TestApp) -> String {
    let (email, password) = create_user(app).await;
    login(&app.server, email, password).await
}

pub async fn admin_logged(app: &TestApp) -> String {
    let (email, password) = create_user_with_role(app, "admin@example.com", Role::Admin).await;
    login(&app.server, email, password).await
}

pub async fn superadmin_logged(app: &TestApp) -> String {
    let (email, password) = create_user_with_role(app, "root@example.com", Role::SuperAdmin).await;
    login(&app.server, email, password).await
}

pub async fn login(server: &TestServer, email: String, password: String) -> String {
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

pub async fn create_student(server: &TestServer, token: &String) -> TestResponse {
    let student = Student {
        id: Uuid::new_v4(),
        dni: 12345678,
        first_name: "Jhon".to_string(),
        last_name: "Doe".to_string(),
        email: None,
        gender: Gender::Male,
        phone: None,
        address: None,
        avatar: None,
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

pub async fn create_representative(
    server: &TestServer,
    token: &String,
    student_id: &Uuid,
) -> TestResponse {
    let representative = Representative {
        id: Uuid::new_v4(),
        first_name: "Father".to_string(),
        last_name: "Doe".to_string(),
        phone: serde_json::json!(vec![123456789, 987654321]),
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };

    let representative_relation = StudentRepresentatives {
        student_id: *student_id,
        representative_id: representative.id,
        is_primary: true,
        relationship: Relationship::Father,
    };

    let representative_with_relation = RepresentativeWithRelation {
        representative,
        relation: representative_relation,
    };

    let response = server
        .post("/representative")
        .authorization_bearer(token)
        .json(&representative_with_relation)
        .await;

    response
}

pub async fn create_period(server: &TestServer, token: &String) -> TestResponse {
    let period = Period {
        id: Uuid::new_v4(),
        name: "Verano".to_string(),
        start_date: Utc::now(),
        end_date: Utc::now() + chrono::Duration::days(90),
    };

    let response = server
        .post("/period")
        .authorization_bearer(token)
        .json(&period)
        .await;

    response
}
