use axum::{Json, extract::{Path, State}};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

use crate::schema::user::{NewUser, Role, UpdateUser, User};

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

#[axum::debug_handler]
pub async fn create_user(State(pool): State<PgPool>, Json(new_user): Json<NewUser>) -> Json<User> {

    let user = User{
      id: Uuid::new_v4(),
      first_name: new_user.first_name,
      last_name: new_user.last_name,
      email: new_user.email,
      avatar: new_user.avatar,
      role: new_user.role,
      created_at: Utc::now(),
      updated_at: Utc::now()
    };

    let create_user: User = sqlx::query_as!(
      User, 
      r#"
          INSERT INTO users
              (
                  id,
                  first_name,
                  last_name,
                  email,
                  avatar,
                  role,
                  created_at,
                  updated_at
              )
          VALUES
              ($1, $2, $3, $4, $5, $6, $7, $8)
          RETURNING
              id,
              first_name,
              last_name,
              email,
              avatar,
              role AS "role: Role",
              created_at,
              updated_at
      "#,
      user.id,
      user.first_name,
      user.last_name,
      user.email,
      user.avatar,
      user.role as Role,
      user.created_at,
      user.updated_at
      )
      .fetch_one(&pool)
      .await
      .expect("Failed to create a new user.");

    Json(create_user)
}

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
                first_name = COALESCE($1, first_name),
                last_name = COALESCE($2, last_name),
                email = COALESCE($3, email),
                avatar = COALESCE($4, avatar),
                role = COALESCE($5, role)
        WHERE id = $6
        RETURNING 
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
    user_to_update.role as Option<Role>,
    id
    )
    .fetch_one(&pool)
    .await
    .expect("Failed to update user.");

  Json(updated_user)
}

#[axum::debug_handler]
pub async fn delete_user(State(pool): State<PgPool>, Path(id): Path<Uuid>) -> String {

  sqlx::query!("DELETE FROM users WHERE id = $1", id)
  .execute(&pool)
  .await
  .expect("Failed to delete user.");

  return format!( "User with id {:?} deleted", id)
}
