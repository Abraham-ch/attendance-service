use sqlx::PgPool;
use uuid::Uuid;

use crate::schema::period::{NewPeriod, Period};

pub async fn create_one(pool: &PgPool, period: NewPeriod) -> Result<Period, sqlx::Error> {
    let new_period = Period {
        id: Uuid::new_v4(),
        name: period.name,
        start_date: period.start_date,
        end_date: period.end_date,
    };

    sqlx::query_as!(
        Period,
        r#"INSERT INTO periods (id, name, start_date, end_date) VALUES ($1, $2, $3, $4) RETURNING *"#,
        new_period.id,
        new_period.name,
        new_period.start_date,
        new_period.end_date,
    )
    .fetch_one(pool)
    .await
}

pub async fn get_all(pool: &PgPool) -> Result<Vec<Period>, sqlx::Error> {
    sqlx::query_as!(Period, "SELECT * FROM periods")
        .fetch_all(pool)
        .await
}

pub async fn delete_one(pool: &PgPool, id: Uuid) -> Result<(), sqlx::Error> {
    sqlx::query!(r#"DELETE FROM periods WHERE id = $1"#, id)
        .execute(pool)
        .await
        .map(|_| ())
}
