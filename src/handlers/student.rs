use axum::{Json, extract::{Path, State}, http::StatusCode};
use uuid::Uuid;
use validator::Validate;

use crate::{repository::student::{create_one, delete_one, find_all, get_by_id, update_one}, schema::{app::AppState, student::{NewStudent, Student, UpdateStudent}}};

#[axum::debug_handler]
pub async fn list_students(State(state): State<AppState>) -> Result<(StatusCode, Json<Vec<Student>>), (StatusCode, String)>{ 

    let full_students = find_all(&state.pool).await;

    match full_students {
        Ok(result) => Ok((StatusCode::OK, Json(result))),
        Err(_) => Err((StatusCode::NOT_FOUND, "Student not found".to_string()))
    }
}

#[axum::debug_handler]
pub async fn create_student(State(state): State<AppState>, Json(new_student): Json<NewStudent>) -> Result<(StatusCode, Json<Student>), (StatusCode, String)> {

    new_student.validate().map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;
    let create_student: Result<Student, sqlx::Error> = create_one(&state.pool, new_student).await;

    match create_student {
        Ok(result) => Ok((StatusCode::CREATED, Json(result))),
        Err(sqlx::Error::Database(db_err)) if db_err.is_unique_violation() =>
        Err((StatusCode::CONFLICT, "dni already exists".to_string())),
        Err(_) => Err((StatusCode::INTERNAL_SERVER_ERROR, "Failed to create student.".to_string()))
    }
}

#[axum::debug_handler]
pub async fn get_student_by_id(State(state): State<AppState>, Path(id): Path<Uuid>) -> Result<(StatusCode, Json<Student>), (StatusCode, String)> {

    let student_by_id = get_by_id(&state.pool, id).await;

    match student_by_id {
        Ok(result) => Ok((StatusCode::OK, Json(result))),
        Err(_) => Err((StatusCode::NOT_FOUND, "Student not found".to_string()))
    }
}

#[axum::debug_handler]
pub async fn update_student(State(state): State<AppState>, Path(id): Path<Uuid>, Json(student_to_update): Json<UpdateStudent>) -> Result<(StatusCode, Json<UpdateStudent>), (StatusCode, String)> {

    student_to_update.validate().map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;
    let updated_student = update_one(&state.pool, id, student_to_update).await;
    
    match updated_student {
        Ok(result) => Ok((StatusCode::OK, Json(result))),
        Err(sqlx::Error::Database(db_err)) if db_err.is_unique_violation() =>
        Err((StatusCode::CONFLICT, "Email already exists".to_string())),
        Err(sqlx::Error::RowNotFound) => Err((StatusCode::NOT_FOUND, "Student not found".to_string())),
        Err(_) => Err((StatusCode::INTERNAL_SERVER_ERROR, "Failed to update student".to_string()))
    }
}

#[axum::debug_handler]
pub async fn delete_student(State(state): State<AppState>, Path(id): Path<Uuid>) -> Result<(StatusCode, String), (StatusCode, String)> {

    let deleted_student = delete_one(&state.pool, id).await;

    match deleted_student {
        Ok(result) if result.rows_affected() == 0 => Err((StatusCode::NOT_FOUND, "Student not found".to_string())),
        Ok(_) => Ok((StatusCode::OK, "Student deleted".to_string())),
        Err(_) => Err((StatusCode::INTERNAL_SERVER_ERROR, "Failed to delete student.".to_string()))
    }
}
