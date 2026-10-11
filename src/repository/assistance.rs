use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

use crate::schema::assistance::{Assistance, NewAssistance, Status};

impl Assistance {
    pub async fn new(pool: &PgPool, new_period: NewAssistance) -> Result<Assistance, sqlx::Error> {
        let assistance = Assistance {
            id: Uuid::new_v4(),
            period_id: new_period.period_id,
            student_id: new_period.student_id,
            date: Utc::now().date_naive(),
            status: Status::Present,
            notes: None,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        sqlx::query_as!(
            Assistance,
            r#"
                INSERT INTO assistances
                    (id, period_id, student_id, date, status, notes, created_at, updated_at)
                VALUES
                    ($1, $2, $3, $4, $5, $6, $7, $8)
                RETURNING
                    id,
                    period_id,
                    student_id,
                    date,
                    status AS "status: Status",
                    notes,
                    created_at,
                    updated_at
            "#,
            assistance.id,
            assistance.period_id,
            assistance.student_id,
            assistance.date,
            assistance.status as Status,
            assistance.notes,
            assistance.created_at,
            assistance.updated_at
        )
        .fetch_one(pool)
        .await
    }

    pub async fn get_by_period(
        pool: &PgPool,
        period_id: Uuid,
    ) -> Result<Vec<Assistance>, sqlx::Error> {
        sqlx::query_as!(
            Assistance,
            r#"
                SELECT
                    id,
                    period_id,
                    student_id,
                    date,
                    status AS "status: Status",
                    notes,
                    created_at,
                    updated_at
                FROM assistances
                WHERE period_id = $1
            "#,
            period_id
        )
        .fetch_all(pool)
        .await
    }

    pub async fn get_by_student(
        pool: &PgPool,
        student_id: Uuid,
    ) -> Result<Vec<Assistance>, sqlx::Error> {
        sqlx::query_as!(
            Assistance,
            r#"
                SELECT
                    id,
                    period_id,
                    student_id,
                    date,
                    status AS "status: Status",
                    notes,
                    created_at,
                    updated_at
                FROM assistances
                WHERE student_id = $1
            "#,
            student_id
        )
        .fetch_all(pool)
        .await
    }

    pub async fn get_all(pool: &PgPool) -> Result<Vec<Assistance>, sqlx::Error> {
        sqlx::query_as!(
            Assistance,
            r#"
            SELECT
                id,
                period_id,
                student_id,
                date,
                status AS "status: Status",
                notes,
                created_at,
                updated_at
            FROM assistances
        "#,
        )
        .fetch_all(pool)
        .await
    }
}
