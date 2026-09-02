use chrono::{DateTime, Utc};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::prelude::{FromRow, Type};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Type, Deserialize, Serialize, Clone, JsonSchema)]
#[sqlx(type_name = "gender")]
#[sqlx(rename_all = "snake_case")]
pub enum Relationship {
    Mother,
    Father,
    StepMother,
    StepFather,
    GrandMother,
    GrandFather,
    Sibling,
    Aunt,
    Uncle,
    LegalGuardian,
    Other
}

#[derive(FromRow, Debug, Deserialize, Serialize, Validate, JsonSchema)]
pub struct Representative{
    pub id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub phone: Value,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>    
}

#[derive(FromRow, Debug, Deserialize, Serialize, Validate, JsonSchema)]
pub struct NewRepresentative{
    #[validate(length(min=3, max=20))]
    pub first_name: String,
    #[validate(length(min=3, max=20))]
    pub last_name: String,
    pub phone: Value,
}

#[derive(FromRow, Debug, Deserialize, Serialize, Validate, JsonSchema)]
pub struct UpdateRepresentative{
    #[validate(length(min=3, max=20))]
    pub first_name: String,
    #[validate(length(min=3, max=20))]
    pub last_name: String,
    pub phone: Value,
}

#[derive(FromRow, Debug, Deserialize, Serialize, Validate, JsonSchema)]
pub struct DeleteRepresentative{
    pub id: Uuid,
}

#[derive(Debug, Deserialize, Serialize, Validate, JsonSchema)]
pub struct StudentRepresentatives{
    pub student_id: Uuid,
    pub representative_id: Uuid,
    pub relationship: Relationship,
    pub is_primary: bool
}

#[derive(Debug, Deserialize, Serialize, Validate, JsonSchema)]
pub struct NewRepresentativeRelation{
    pub student_id: Uuid,
    pub relationship: Relationship,
    pub is_primary: bool    
}

#[derive(Debug, Deserialize, Serialize, Validate, JsonSchema)]
pub struct RepresentativeWithRelation{
    pub representative: Representative,
    pub relation: StudentRepresentatives
}

#[derive(Debug, Deserialize, Serialize, Validate, JsonSchema)]
pub struct CreateRepresentativeRequest {
    pub representative: NewRepresentative,
    pub relation: NewRepresentativeRelation,
}