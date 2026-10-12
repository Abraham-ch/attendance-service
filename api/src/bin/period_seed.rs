use attendance_service::schema::period::Period;
use chrono::{TimeZone, Utc};
use dotenvy::var;
use sqlx::{Pool, Postgres, postgres::PgPoolOptions};
use uuid::Uuid;

#[tokio::main]
async fn main() {
    let db = var("DATABASE_URL").expect("DATABASE_URL needed");
    let pool: Pool<Postgres> = PgPoolOptions::new()
        .connect(&db)
        .await
        .expect("Failed to connect to DB");

    let mut tx = pool
        .begin()
        .await
        .expect("Failed to initialize transaction");

    // (name, (year, month, day) start, (year, month, day) end)
    let periods = vec![
        ("2025-I", (2025, 3, 10), (2025, 7, 18)),
        ("2025-II", (2025, 8, 11), (2025, 12, 19)),
        ("2026-I", (2026, 3, 9), (2026, 7, 17)),
        ("2026-II", (2026, 8, 10), (2026, 12, 18)),
        ("2027-I", (2027, 3, 8), (2027, 7, 16)),
    ];

    for period in periods {
        let (sy, sm, sd) = period.1;
        let (ey, em, ed) = period.2;
        let new_period = Period {
            id: Uuid::new_v4(),
            name: period.0.to_string(),
            start_date: Utc.with_ymd_and_hms(sy, sm, sd, 0, 0, 0).unwrap(),
            end_date: Utc.with_ymd_and_hms(ey, em, ed, 23, 59, 59).unwrap(),
        };

        // periods.name has no unique constraint, so ON CONFLICT can't dedupe re-runs
        sqlx::query(
            r#"
                INSERT INTO periods
                    (id, name, start_date, end_date)
                SELECT $1, $2, $3, $4
                WHERE NOT EXISTS (SELECT 1 FROM periods WHERE name = $2)
            "#,
        )
        .bind(&new_period.id)
        .bind(&new_period.name)
        .bind(&new_period.start_date)
        .bind(&new_period.end_date)
        .execute(&mut *tx)
        .await
        .expect("Failed to insert periods");
    }

    tx.commit().await.expect("Failed to commit transaction.");
    println!("Periods seeded succesfully!")
}
