use axum::{Json, extract::{Path, State}};
use sqlx::PgPool;
use uuid::Uuid;

use crate::schema::user::{Role, UpdateUser, User};

#[axum::debug_handler]
pub async fn list_users(State(pool): State<PgPool>) -> Json<Vec<User>>{ 
  let full_users = sqlx::query_as!(
    User,
    r#"
        SELECT
            id,
            first_name,
            last_name,
            email,
            avatar,
            role AS "role: Role",
            created_at,
            updated_at
        FROM users
    "#
    )
    .fetch_all(&pool)
    .await
    .expect("Failed to get users list.");

  Json(full_users)
}

pub async fn create_user() -> &'static str { "Create user" }

#[axum::debug_handler]
pub async fn get_user_by_id(State(pool): State<PgPool>, Path(id): Path<Uuid>) -> Json<User> {
  
  let user_by_id: User = sqlx::query_as!(
    User, 
    r#"
        SELECT
            id,
            first_name,
            last_name,
            email,
            avatar,
            role AS "role: Role",
            created_at,
            updated_at
        FROM users 
        WHERE id = $1
    "#,
    id
    )
    .fetch_one(&pool)
    .await
    .expect(format!("Failed to get user with id: {:?}", id).as_str());

  Json(user_by_id)
}

#[axum::debug_handler]
pub async fn update_user(State(pool): State<PgPool>, Path(id): Path<Uuid>, Json(user_to_update): Json<UpdateUser>) -> Json<UpdateUser> {
  
  let updated_user: UpdateUser = sqlx::query_as!(
    UpdateUser, 
    r#"
        UPDATE
            users
            SET
                first_name = $1,
                last_name = $2,
                email = $3,
                avatar = $4,
                role = $5
        WHERE id = $6
        RETURNING 
            id,
            first_name,
            last_name,
            email,
            avatar,
            role AS "role: Role"
    "#,
    user_to_update.first_name,
    user_to_update.last_name,
    user_to_update.email,
    user_to_update.avatar,
    user_to_update.role as Role,
    id
    )
    .fetch_one(&pool)
    .await
    .expect("Failed to update user.");

  Json(updated_user)
}

#[axum::debug_handler]
pub async fn delete_user(Path(id): Path<User>) -> String {
  return format!( "User with id {:?} deleted", id)
}