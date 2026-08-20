use chrono::Utc;
use sqlx::{PgPool, postgres::{PgQueryResult}};
use uuid::Uuid;

use crate::schema::user::{NewUser, Role, UpdateUser, User};

pub async fn find_all(pool: &PgPool) -> Result<Vec<User>, sqlx::Error>{ 
    sqlx::query_as!(
        User,
        r#"
            SELECT
                id,
                first_name,
                last_name,
                email,
                password,
                avatar,
                role AS "role: Role",
                created_at,
                updated_at
            FROM users
        "#
        )
        .fetch_all(pool)
        .await
}

pub async fn create_one(pool: &PgPool, new_user: NewUser) -> Result<User, sqlx::Error> {
    let user = User{
        id: Uuid::new_v4(),
        first_name: new_user.first_name,
        last_name: new_user.last_name,
        email: new_user.email,
        password: new_user.password,
        avatar: new_user.avatar,
        role: new_user.role,
        created_at: Utc::now(),
        updated_at: Utc::now()
    };

    sqlx::query_as!(
        User, 
        r#"
            INSERT INTO users
                (
                    id,
                    first_name,
                    last_name,
                    email,
                    password,
                    avatar,
                    role,
                    created_at,
                    updated_at
                )
            VALUES
                ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            RETURNING
                id,
                first_name,
                last_name,
                email,
                avatar,
                password,
                role AS "role: Role",
                created_at,
                updated_at
        "#,
        user.id,
        user.first_name,
        user.last_name,
        user.email,
        user.password,
        user.avatar,
        user.role as Role,
        user.created_at,
        user.updated_at
        )
        .fetch_one(pool)
        .await
}

pub async fn get_by_id(pool: &PgPool, id: Uuid) -> Result<User, sqlx::Error> {
    sqlx::query_as!(
        User, 
        r#"
            SELECT
                id,
                first_name,
                last_name,
                email,
                password,
                avatar,
                role AS "role: Role",
                created_at,
                updated_at
            FROM users 
            WHERE id = $1
        "#,
        id
        )
        .fetch_one(pool)
        .await
}

pub async fn update_one(pool: &PgPool, id: Uuid, user_to_update: UpdateUser) -> Result<UpdateUser, sqlx::Error> {
    sqlx::query_as!(
        UpdateUser, 
        r#"
            UPDATE
                users
                SET
                    first_name = COALESCE($1, first_name),
                    last_name = COALESCE($2, last_name),
                    email = COALESCE($3, email),
                    password = COALESCE($4, password),
                    avatar = COALESCE($5, avatar),
                    role = COALESCE($6, role)
            WHERE id = $7
            RETURNING 
                first_name,
                last_name,
                email,
                password,
                avatar,
                role AS "role: Role"
        "#,
        user_to_update.first_name,
        user_to_update.last_name,
        user_to_update.email,
        user_to_update.password,
        user_to_update.avatar,
        user_to_update.role as Option<Role>,
        id
        )
        .fetch_one(pool)
        .await
}

pub async fn delete_one(pool: &PgPool, id: Uuid) -> Result<PgQueryResult, sqlx::Error> {
    sqlx::query!("DELETE FROM users WHERE id = $1", id)
        .execute(pool)
        .await
}

pub async fn find_by_email(pool: &PgPool, email: String) -> Result<User, sqlx::Error> {
    sqlx::query_as!(
        User,
        r#"
            SELECT
                id,
                first_name,
                last_name,
                email,
                password,
                avatar,
                role AS "role: Role",
                created_at,
                updated_at
            FROM users 
            WHERE email = $1
        "#,
        email
    )
    .fetch_one(pool)
    .await
}
