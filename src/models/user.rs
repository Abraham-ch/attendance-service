use axum::{Json, extract::{Path, State}};
use sqlx::PgPool;
use uuid::Uuid;

use crate::schema::user::User;

#[axum::debug_handler]
pub async fn list_users(State(pool): State<PgPool>) -> Json<Vec<User>>{ 
  let full_users = sqlx::query_as::<_,User>("SELECT * FROM users")
    .fetch_all(&pool)
    .await
    .expect("Failed to get users list.");

  Json(full_users)
}

pub async fn create_user() -> &'static str { "Create user" }

#[axum::debug_handler]
pub async fn get_user_by_id(State(pool): State<PgPool>, Path(id): Path<Uuid>) -> Json<User> {
  
  let user_by_id: User = sqlx::query_as("SELECT * FROM users WHERE id = $1")
    .bind(id)
    .fetch_one(&pool)
    .await
    .expect(format!("Failed to get user with id: {:?}", id).as_str());

  Json(user_by_id)
}

pub async fn update_user(Path(id): Path<User>) -> String {
  return format!( "User with id {:?} updated", id)
}

pub async fn delete_user(Path(id): Path<User>) -> String {
  return format!( "User with id {:?} deleted", id)
}