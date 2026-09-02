use chrono::Utc;
use sqlx::{PgPool, postgres::PgQueryResult};
use uuid::Uuid;

use crate::schema::student::{Gender, NewStudent, Student, UpdateStudent};

pub async fn create_one(pool: &PgPool, new_student: NewStudent) -> Result<Student, sqlx::Error>{
    let student = Student{
        id: Uuid::new_v4(),
        dni: new_student.dni,
        first_name: new_student.first_name,
        last_name: new_student.last_name,
        gender: new_student.gender as Gender,
        phone: new_student.phone,
        address: new_student.address,
        created_at: Utc::now(),
        updated_at: Utc::now()
    };
    
    sqlx::query_as!(
        Student,
        r#"
            INSERT INTO students
                (
                    id,
                    dni,
                    first_name,
                    last_name,
                    gender,
                    phone,
                    address,
                    created_at,
                    updated_at
                )
            VALUES
                ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            RETURNING
                id,
                dni,
                first_name,
                last_name,
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
        student.gender as Gender,
        student.phone,
        student.address,
        student.created_at,
        student.updated_at
    )
    .fetch_one(pool)
    .await
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