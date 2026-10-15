use axum::{
    Json,
    extract::{Path, Query, State},
};
use uuid::Uuid;
use validator::Validate;

use crate::schema::{
    app::{AppError, AppState},
    pagination::{Page, PageQuery},
    student::{NewStudent, Student, StudentResponse, UpdateStudent},
};

#[axum::debug_handler]
pub async fn list_students(
    State(state): State<AppState>,
    Query(query): Query<PageQuery>,
) -> Result<Json<Page<Student>>, AppError> {
    let students = Student::find_page(&state.pool, &query).await?;
    Ok(Json(students))
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
