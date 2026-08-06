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

#[derive(Debug)]
pub struct UpdateUser<'a>{
  pub first_name: &'a String,
  pub last_name: &'a String,
  pub email: &'a String,
  pub avatar: &'a String,
  pub role: &'a String,
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
#[sqlx(type_name = "text")]
pub enum Role {
  SuperAdmin,
  Admin,
  User,
}