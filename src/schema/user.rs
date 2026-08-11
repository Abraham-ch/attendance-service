use sqlx::{FromRow, prelude::Type};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug)]
pub struct NewUser<'a>{
  pub id: &'a Uuid,
  pub first_name: &'a String,
  pub last_name: &'a String,
  pub email: &'a String,
  pub avatar: &'a String,
  pub role: &'a String,
  pub created_at: &'a DateTime<Utc>,
  pub updated_at: &'a DateTime<Utc> 
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct NewUserHandler{
  pub first_name: String,
  pub last_name: String,
  pub email: String,
  pub avatar: String,
  pub role: String,
}

#[derive(FromRow, Debug, Deserialize, Serialize)]
pub struct UpdateUser{
  pub first_name: Option<String>,
  pub last_name: Option<String>,
  pub email: Option<String>,
  pub avatar: Option<String>,
  pub role: Option<Role>,
}

#[derive(FromRow, Debug, Deserialize, Serialize)]
pub struct User{
  pub id: Uuid,
  pub first_name: String,
  pub last_name: String,
  pub email: String,
  pub avatar: String,
  pub role: Role,
  pub created_at: DateTime<Utc>,
  pub updated_at: DateTime<Utc> 
}

#[derive(Debug)]
pub struct DeleteUser{
  pub id: Uuid,
}

#[derive(Debug, Type, Deserialize, Serialize)]
#[sqlx(type_name = "role")]
#[sqlx(rename_all = "snake_case")]
pub enum Role {
  SuperAdmin,
  Admin,
  User,
}