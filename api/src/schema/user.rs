use crate::utils::valid_password;
use chrono::{DateTime, Utc};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, prelude::Type};
use uuid::Uuid;
use validator::Validate;

#[derive(FromRow, Debug, Deserialize, Serialize, Validate, JsonSchema)]
pub struct NewUser {
    #[validate(length(min = 3, max = 20))]
    pub first_name: String,
    #[validate(length(min = 3, max = 20))]
    pub last_name: String,
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 10, max = 20), custom(function = "valid_password"))]
    pub password: String,
    #[serde(default)]
    pub avatar: String,
    pub role: Role,
}

#[derive(FromRow, Debug, Deserialize, Serialize, Validate, JsonSchema)]
pub struct UpdateUser {
    #[validate(length(min = 3, max = 20))]
    pub first_name: Option<String>,
    #[validate(length(min = 3, max = 20))]
    pub last_name: Option<String>,
    #[validate(email)]
    pub email: Option<String>,
    #[validate(length(min = 10, max = 20), custom(function = "valid_password"))]
    pub password: Option<String>,
    pub avatar: Option<String>,
    pub role: Option<Role>,
}

#[derive(FromRow, Debug, Deserialize, Serialize, Clone, JsonSchema)]
pub struct User {
    pub id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    /// argon2 hash, never sent to clients
    #[serde(skip_serializing, default)]
    pub password: String,
    pub avatar: String,
    pub role: Role,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug)]
pub struct DeleteUser {
    pub id: Uuid,
}

#[derive(Debug, Type, Deserialize, Serialize, Clone, PartialEq, JsonSchema)]
#[sqlx(type_name = "role")]
#[sqlx(rename_all = "snake_case")]
pub enum Role {
    SuperAdmin, //me
    Admin,      // teachers or instructors
    User,       // students or representatives
}

impl Role {
    pub fn is_admin(&self) -> bool {
        matches!(self, Role::Admin | Role::SuperAdmin)
    }

    /// Admins create plain users, only the SuperAdmin creates admins, nobody creates a SuperAdmin
    pub fn can_create(&self, new_role: &Role) -> bool {
        match new_role {
            Role::User => self.is_admin(),
            Role::Admin => *self == Role::SuperAdmin,
            Role::SuperAdmin => false,
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Validate, JsonSchema)]
pub struct AuthUser {
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 10, max = 20), custom(function = "valid_password"))]
    pub password: String,
}

#[derive(Serialize, Deserialize, JsonSchema)]
pub struct AuthResponse {
    pub user: User,
    pub token: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Claims {
    pub id: String, // user id
    pub param: String,
    pub exp: usize, // expiration timestamp
}

impl Claims {
    /// Login tokens carry the role in `param` (see login_user), invite tokens carry a DNI
    pub fn role(&self) -> Option<Role> {
        serde_json::from_value(serde_json::Value::String(self.param.clone())).ok()
    }
}

/// Body of POST /auth/register: the invite token from the email and the new password
#[derive(Debug, Deserialize, Serialize, Validate, JsonSchema)]
pub struct RegisterUser {
    pub token: String,
    #[validate(length(min = 10, max = 20), custom(function = "valid_password"))]
    pub password: String,
}

/// What the register page shows about the student before they pick a password
#[derive(Debug, Deserialize, Serialize, JsonSchema)]
pub struct InviteInfo {
    pub first_name: String,
    pub last_name: String,
    pub email: String,
}

#[derive(Debug, Deserialize, JsonSchema)]
pub struct InviteQuery {
    pub token: String,
}
