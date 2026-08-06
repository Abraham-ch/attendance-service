use std::println;

use chrono::Utc;
use dotenvy::var;
use attendance_service::schema::user::{User, Role};
use sqlx::{Pool, Postgres, postgres::PgPoolOptions};
use uuid::Uuid;



#[tokio::main]
async fn main() {
  let db = var("DATABASE_URL").expect("DATABASE_URL needed");
  let pool:Pool<Postgres> = PgPoolOptions::new()
    .connect(&db)
    .await
    .expect("Failed to connect to DB");
  
  let mut tx = pool.begin().await.expect("Failed to initialize transaction");

  let users = vec![
    (
        "John",
        "Doe",
        "https://example.com/avatars/john.png",
        "john.doe@example.com",
        Role::User,
    ),
    (
        "Abraham",
        "Chafloque",
        "https://example.com/avatars/abraham.png",
        "abraham.chafloque@example.com",
        Role::SuperAdmin,
    ),
    (
        "Alice",
        "Johnson",
        "https://example.com/avatars/alice.png",
        "alice.johnson@example.com",
        Role::Admin,
    ),
    (
        "Bob",
        "Smith",
        "https://example.com/avatars/bob.png",
        "bob.smith@example.com",
        Role::User,
    ),
    (
        "Charlie",
        "Brown",
        "https://example.com/avatars/charlie.png",
        "charlie.brown@example.com",
        Role::User,
    ),
    (
        "Diana",
        "Prince",
        "https://example.com/avatars/diana.png",
        "diana.prince@example.com",
        Role::Admin,
    ),
    (
        "Ethan",
        "Williams",
        "https://example.com/avatars/ethan.png",
        "ethan.williams@example.com",
        Role::User,
    ),
    (
        "Fiona",
        "Davis",
        "https://example.com/avatars/fiona.png",
        "fiona.davis@example.com",
        Role::User,
    ),
    (
        "George",
        "Miller",
        "https://example.com/avatars/george.png",
        "george.miller@example.com",
        Role::Admin,
    ),
    (
        "Hannah",
        "Wilson",
        "https://example.com/avatars/hannah.png",
        "hannah.wilson@example.com",
        Role::User,
    ),
    (
        "Isaac",
        "Taylor",
        "https://example.com/avatars/isaac.png",
        "isaac.taylor@example.com",
        Role::User,
    ),
    (
        "Julia",
        "Anderson",
        "https://example.com/avatars/julia.png",
        "julia.anderson@example.com",
        Role::Admin,
    ),
    (
        "Kevin",
        "Moore",
        "https://example.com/avatars/kevin.png",
        "kevin.moore@example.com",
        Role::User,
    ),
    (
        "Laura",
        "Thomas",
        "https://example.com/avatars/laura.png",
        "laura.thomas@example.com",
        Role::User,
    ),
    (
        "Michael",
        "Scott",
        "https://example.com/avatars/michael.png",
        "michael.scott@example.com",
        Role::Admin,
    ),
    (
        "Natalie",
        "Clark",
        "https://example.com/avatars/natalie.png",
        "natalie.clark@example.com",
        Role::User,
    ),
    (
        "Oliver",
        "Hall",
        "https://example.com/avatars/oliver.png",
        "oliver.hall@example.com",
        Role::User,
    ),
    (
        "Patricia",
        "Lewis",
        "https://example.com/avatars/patricia.png",
        "patricia.lewis@example.com",
        Role::Admin,
    ),
    (
        "Ryan",
        "Walker",
        "https://example.com/avatars/ryan.png",
        "ryan.walker@example.com",
        Role::User,
    ),
    (
        "Sophia",
        "Young",
        "https://example.com/avatars/sophia.png",
        "sophia.young@example.com",
        Role::User,
    ),
  ];

  for user in users {
    let new_user = User {
      id: Uuid::new_v4(),
      first_name: user.0.to_string(),
      last_name: user.1.to_string(),
      avatar: user.2.to_string(),
      email: user.3.to_string(),
      role: user.4,
      created_at: Utc::now(),
      updated_at: Utc::now()
    };
    
    sqlx::query("INSERT INTO users (id, first_name, last_name, avatar, email, role, created_at, updated_at) VALUES ($1, $2, $3, $4, $5, $6 ,$7, $8) ON CONFLICT DO NOTHING")
      .bind(&new_user.id)
      .bind(&new_user.first_name)
      .bind(&new_user.last_name)
      .bind(&new_user.avatar)
      .bind(&new_user.email)
      .bind(&new_user.role)
      .bind(&new_user.created_at)
      .bind(&new_user.updated_at)
      .execute(&mut *tx)
      .await
      .expect("Failed to insert users");

  }

  tx.commit().await.expect("Failed to commit transaction.");
  println!("Users seeded succesfully!")

}