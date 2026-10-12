use chrono::{DateTime, Utc};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use sqlx::prelude::Type;
use uuid::Uuid;

#[derive(Debug, Type, Deserialize, Serialize, Clone, JsonSchema)]
pub struct Period {
    pub id: Uuid,
    pub name: String,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
}

#[derive(Debug, Type, Deserialize, Serialize, Clone, JsonSchema)]
pub struct NewPeriod {
    pub name: String,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
}

#[derive(Debug, Type, Deserialize, Serialize, Clone, JsonSchema)]
pub struct DeletePeriod {
    pub id: Uuid,
}
