use chrono::{DateTime, NaiveDate, Utc};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use sqlx::prelude::Type;
use uuid::Uuid;

#[derive(Debug, Type, Deserialize, Serialize, Clone, JsonSchema)]
#[sqlx(type_name = "assistance_status", rename_all = "lowercase")]
pub enum Status {
    Present,
    Absent,
    Late,
    Excused,
}

#[derive(Debug, Type, Deserialize, Serialize, Clone, JsonSchema)]
pub struct Assistance {
    pub id: Uuid,
    pub student_id: Uuid,
    pub period_id: Uuid,
    pub date: NaiveDate, //date and created_at cannot represent the same, a student can be excused from a different day that the date of assistance belongs
    pub status: Status,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>, //an assistance can be updated in case a student were excused
}

#[derive(Debug, Type, Deserialize, Serialize, Clone, JsonSchema)]
pub struct NewAssistance {
    pub student_id: Uuid,
    pub period_id: Uuid,
}

#[derive(Debug, Type, Deserialize, Serialize, Clone, JsonSchema)]
pub struct UpdateAssistance {
    pub status: Option<Status>,
    pub notes: Option<String>,
}
