use axum::extract::Path;

use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize)]
pub struct User {
  id: Uuid
}

pub async fn list_users() -> &'static str { "List users" }

pub async fn create_user() -> &'static str { "Create user" }

#[axum::debug_handler]
pub async fn get_user_by_id(Path(User{id}): Path<User>) -> String {

  return format!( "User with id: {:?}", id);
}

pub async fn update_user(Path(User{id}): Path<User>) -> String {
  return format!( "User with id {:?} updated", id)
}

pub async fn delete_user(Path(User{id}): Path<User>) -> String {
  return format!( "User with id {:?} deleted", id)
}