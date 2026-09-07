use chrono::{Duration, Utc};
use sqlx::{PgPool, postgres::PgQueryResult};
use uuid::Uuid;

use crate::{schema::{app::AppState, student::{Gender, InviteToken, NewStudent, Student, StudentResponse, UpdateStudent}, user::Claims}, utils::create_token};

pub async fn create_one(state: AppState, new_student: NewStudent) -> Result<StudentResponse, sqlx::Error>{
    let mut tx = state.pool.begin().await?;

    let student = Student{
        id: Uuid::new_v4(),
        dni: new_student.dni,
        first_name: new_student.first_name,
        last_name: new_student.last_name,
        email: new_student.email as Option<String>,
        gender: new_student.gender as Gender,
        phone: new_student.phone,
        address: new_student.address,
        created_at: Utc::now(),
        updated_at: Utc::now()
    };

    let insert_student = sqlx::query_as!(
        Student,
        r#"
            INSERT INTO students
                (
                    id,
                    dni,
                    first_name,
                    last_name,
                    email,
                    gender,
                    phone,
                    address,
                    created_at,
                    updated_at
                )
            VALUES
                ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
            RETURNING
                id,
                dni,
                first_name,
                last_name,
                email,
                gender as "gender: Gender",
                phone,
                address,
                created_at,
                updated_at
        "#,
        student.id,
        student.dni,
        student.first_name,
        student.last_name,
        student.email,
        student.gender as Gender,
        student.phone,
        student.address,
        student.created_at,
        student.updated_at
    )
    .fetch_one(&mut *tx)
    .await?;

    let invite_token = if student.email.is_some(){
        let claim = Claims{
            id: student.id.to_string(),
            param: student.dni.to_string(),
            exp: 259200 //3 days
        };

        let new_token = create_token(claim, state).unwrap();
        let expires_at = Utc::now() + Duration::days(3);

        let invite_token = InviteToken{
            id: Uuid::new_v4(),
            student_id: student.id,
            token: new_token,
            expires_at: expires_at,
            used_at: None,
            created_at: Utc::now(),
        };

        let create_invite_token = sqlx::query_as!(
            InviteToken,
            r#"
                INSERT INTO invite_tokens
                    (
                        id,
                        student_id,
                        token,
                        expires_at,
                        used_at,
                        created_at
                    )
                VALUES
                    ($1, $2, $3, $4, $5, $6)
                RETURNING
                    id,
                    student_id,
                    token,
                    expires_at,
                    used_at,
                    created_at
            "#,
            invite_token.id,
            invite_token.student_id,
            invite_token.token,
            invite_token.expires_at,
            invite_token.used_at,
            invite_token.created_at
        )
        .fetch_one(&mut *tx)
        .await?;

        Some(create_invite_token)
    } else {
        None
    };

    tx.commit().await?;

    Ok(StudentResponse {
    student: insert_student,
    token: invite_token
    })
}

pub async fn find_all(pool: &PgPool) -> Result<Vec<Student>, sqlx::Error>{
    sqlx::query_as!(
        Student,
        r#"
            SELECT
                id,
                dni,
                first_name,
                last_name,
                email,
                gender AS "gender:Gender",
                phone,
                address,
                created_at,
                updated_at
            FROM students
        "#
    )
    .fetch_all(pool)
    .await
}

pub async fn get_by_id(pool: &PgPool, id: Uuid) -> Result<Student, sqlx::Error> {
    sqlx::query_as!(
        Student, 
        r#"
            SELECT
                id,
                dni,
                first_name,
                last_name,
                email,
                gender AS "gender:Gender",
                phone,
                address,
                created_at,
                updated_at
            FROM students 
            WHERE id = $1
        "#,
        id
    )
    .fetch_one(pool)
    .await
}

pub async fn update_one(pool: &PgPool, id: Uuid, student_to_update: UpdateStudent) -> Result<UpdateStudent, sqlx::Error> {
    sqlx::query_as!(
        UpdateStudent, 
        r#"
            UPDATE
                students
                SET
                    phone = COALESCE($1, phone),
                    address = COALESCE($2, address)
            WHERE id = $3
            RETURNING 
                phone,
                address
        "#,
        student_to_update.phone,
        student_to_update.address,
        id
    )
    .fetch_one(pool)
    .await
}

pub async fn delete_one(pool: &PgPool, id: Uuid) -> Result<PgQueryResult, sqlx::Error> {
    sqlx::query!("DELETE FROM students WHERE id = $1", id)
        .execute(pool)
        .await
}
