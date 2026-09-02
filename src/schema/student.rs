use chrono::{DateTime, Utc};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, prelude::Type};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Type, Deserialize, Serialize, Clone, JsonSchema)]
#[sqlx(type_name = "gender")]
#[sqlx(rename_all = "snake_case")]
pub enum Gender {
    Male,
    Female
}

#[derive(FromRow, Debug, Deserialize, Serialize, Validate, JsonSchema)]

pub struct Student{
    pub id: Uuid,
    pub dni: i64,
    pub first_name: String,
    pub last_name: String,
    pub gender: Gender,
    pub phone: Option<i64>,
    pub address: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>
}

#[derive(FromRow, Debug, Deserialize, Serialize, Validate, JsonSchema)]
pub struct NewStudent{
    pub dni: i64,
    #[validate(length(min=3, max=20))]
    pub first_name: String,
    #[validate(length(min=3, max=20))]
    pub last_name: String,
    pub gender: Gender,
    pub phone: Option<i64>,
    #[validate(length(min=3, max=100))]
    pub address: Option<String>  
}

#[derive(FromRow, Debug, Deserialize, Serialize, Validate, JsonSchema)]
pub struct UpdateStudent{
    pub phone: Option<i64>,
    #[validate(length(min=3, max=100))]
    pub address: Option<String>
}

#[derive(FromRow, Debug, Deserialize, Serialize, Validate, JsonSchema)]
pub struct DeleteStudent{
    pub id: Uuid
}