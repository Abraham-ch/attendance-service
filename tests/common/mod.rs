use attendance_service::{app, schema::app::AppState};
use axum_test::TestServer;
use sqlx::{PgPool, postgres::PgPoolOptions};
use testcontainers_modules::{
    postgres::Postgres,
    testcontainers::{ContainerAsync, runners::AsyncRunner},
};
use tokio::sync::OnceCell;
struct Db {
    url: String,
    _container: ContainerAsync<Postgres>, // kept alive for the whole run
}

static DB: OnceCell<Db> = OnceCell::const_new();

async fn db_url() -> &'static str {
    let db = DB
        .get_or_init(|| async {
            let container = Postgres::default().start().await.unwrap(); //shared container is initialized but never dropped, consider to go back to one container per test
            let port = container.get_host_port_ipv4(5432).await.unwrap();
            let url = format!("postgres://postgres:postgres@127.0.0.1:{port}/postgres");

            let pool = PgPool::connect(&url).await.unwrap();
            sqlx::migrate!().run(&pool).await.unwrap(); // runs once

            Db {
                url,
                _container: container,
            }
        })
        .await;

    &db.url
}

pub async fn server() -> TestServer {
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(db_url().await)
        .await
        .unwrap(); // per test

    let appstate = AppState {
        pool,
        secret: "test_secret_key".to_string(),
    };

    TestServer::new(app(appstate))
}
