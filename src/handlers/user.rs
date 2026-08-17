use axum::{Json, extract::{Path, State}, http::StatusCode};
use sqlx::PgPool;
use uuid::Uuid;
use validator::Validate;

use crate::{repository::user::{create_one, delete_one, find_all, get_by_id, update_one}, schema::user::{NewUser, UpdateUser, User}};

#[axum::debug_handler]
pub async fn list_users(State(pool): State<PgPool>) -> Result<(StatusCode, Json<Vec<User>>), (StatusCode, String)>{ 
  
  let full_users = find_all(&pool).await;

  match full_users {
    Ok(result) => Ok((StatusCode::OK, Json(result))),
    Err(_) => Err((StatusCode::NOT_FOUND, "User not found".to_string()))
  }
}

#[axum::debug_handler]
pub async fn create_user(State(pool): State<PgPool>, Json(new_user): Json<NewUser>) -> Result<(StatusCode, Json<User>), (StatusCode, String)> {

  new_user.validate().map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;
  let create_user = create_one(&pool, new_user).await;

  match create_user {
    Ok(result) => Ok((StatusCode::CREATED, Json(result))),
    Err(sqlx::Error::Database(db_err)) if db_err.is_unique_violation() =>
      Err((StatusCode::CONFLICT, "Email already exists".to_string())),
    Err(_) => Err((StatusCode::INTERNAL_SERVER_ERROR, "Failed to create user.".to_string()))
  }
}

#[axum::debug_handler]
pub async fn get_user_by_id(State(pool): State<PgPool>, Path(id): Path<Uuid>) -> Result<(StatusCode, Json<User>), (StatusCode, String)> {
  
  let user_by_id = get_by_id(&pool, id).await;

  match user_by_id {
    Ok(result) => Ok((StatusCode::OK, Json(result))),
    Err(_) => Err((StatusCode::NOT_FOUND, "User not found".to_string()))
  }
}

#[axum::debug_handler]
pub async fn update_user(State(pool): State<PgPool>, Path(id): Path<Uuid>, Json(user_to_update): Json<UpdateUser>) -> Result<(StatusCode, Json<UpdateUser>), (StatusCode, String)> {

  user_to_update.validate().map_err(|e| (StatusCode::BAD_REQUEST, e.to_string()))?;
  let updated_user = update_one(&pool, id, user_to_update).await;
  
  match updated_user {
    Ok(result) => Ok((StatusCode::OK, Json(result))),
    Err(sqlx::Error::Database(db_err)) if db_err.is_unique_violation() =>
      Err((StatusCode::CONFLICT, "Email already exists".to_string())),
    Err(sqlx::Error::RowNotFound) => Err((StatusCode::NOT_FOUND, "User not found".to_string())),
    Err(_) => Err((StatusCode::INTERNAL_SERVER_ERROR, "Failed to update user".to_string()))
  }
}

#[axum::debug_handler]
pub async fn delete_user(State(pool): State<PgPool>, Path(id): Path<Uuid>) -> Result<(StatusCode, String), (StatusCode, String)> {

  let deleted_user = delete_one(&pool, id).await;

  match deleted_user {
    Ok(result) if result.rows_affected() == 0 => Err((StatusCode::NOT_FOUND, "User not found".to_string())),
    Ok(_) => Ok((StatusCode::OK, "User deleted".to_string())),
    Err(_) => Err((StatusCode::INTERNAL_SERVER_ERROR, "Failed to delete user.".to_string()))
  }
}
