use schemars::JsonSchema;
use sqlx::{FromRow, prelude::Type};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;
use crate::utils::valid_password;

#[derive(FromRow, Debug, Deserialize, Serialize, Validate, JsonSchema)]
pub struct NewUser{
    #[validate(length(min=3, max=20))]
    pub first_name: String,
    #[validate(length(min=3, max=20))]
    pub last_name: String,
    #[validate(email)]
    pub email: String,
    #[validate(length(min=10, max=20), custom(function="valid_password"))]
    pub password: String,
    pub avatar: String,
    pub role: Role
}

#[derive(FromRow, Debug, Deserialize, Serialize, Validate, JsonSchema)]
pub struct UpdateUser{
    #[validate(length(min=3, max=20))]
    pub first_name: Option<String>,
    #[validate(length(min=3, max=20))]
    pub last_name: Option<String>,
    #[validate(email)]
    pub email: Option<String>,
    #[validate(length(min=10, max=20), custom(function="valid_password"))]
    pub password: Option<String>,
    pub avatar: Option<String>,
    pub role: Option<Role>
}

#[derive(FromRow, Debug, Deserialize, Serialize, Clone, JsonSchema)]
pub struct User{
    pub id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub password: String,
    pub avatar: String,
    pub role: Role,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>
}

#[derive(Debug)]
pub struct DeleteUser{
    pub id: Uuid,
}

#[derive(Debug, Type, Deserialize, Serialize, Clone, JsonSchema)]
#[sqlx(type_name = "role")]
#[sqlx(rename_all = "snake_case")]
pub enum Role {
    SuperAdmin, //me
    Admin, // teachers or instructors
    User, // students or representatives
}

#[derive(Debug, Deserialize, Serialize, Validate, JsonSchema)]
pub struct AuthUser{
    #[validate(email)]
    pub email: String,
    #[validate(length(min=10, max=20), custom(function="valid_password"))]
    pub password: String
}

#[derive(Serialize, Deserialize)]
pub struct AuthResponse{
    pub user: User,
    pub token: String,
}

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,  // user id
    pub role: Role,
    pub exp: usize,   // expiration timestamp
}
