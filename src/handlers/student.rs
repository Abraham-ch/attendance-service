use axum::{
    Json,
    extract::{Path, State},
};
use uuid::Uuid;
use validator::Validate;

use crate::schema::{
    app::{AppError, AppState},
    student::{NewStudent, Student, StudentResponse, UpdateStudent},
};

#[axum::debug_handler]
pub async fn list_students(State(state): State<AppState>) -> Result<Json<Vec<Student>>, AppError> {
    let full_students = Student::find_all(&state.pool).await?;
    Ok(Json(full_students))
}

#[axum::debug_handler]
pub async fn create_student(
    State(state): State<AppState>,
    Json(new_student): Json<NewStudent>,
) -> Result<Json<StudentResponse>, AppError> {
    new_student.validate()?;
    let create_student = Student::new(state, new_student).await?;
    Ok(Json(create_student))
}

#[axum::debug_handler]
pub async fn get_student_by_id(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<Student>, AppError> {
    let student_by_id = Student::get_by_id(&state.pool, id).await?;
    Ok(Json(student_by_id))
}

#[axum::debug_handler]
pub async fn update_student(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(student_to_update): Json<UpdateStudent>,
) -> Result<Json<UpdateStudent>, AppError> {
    student_to_update.validate()?;
    let updated_student = Student::update(&state.pool, id, student_to_update).await?;
    Ok(Json(updated_student))
}

#[axum::debug_handler]
pub async fn delete_student(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<String, AppError> {
    Student::delete(&state.pool, id).await?;
    Ok("Student deleted".to_string())
}
