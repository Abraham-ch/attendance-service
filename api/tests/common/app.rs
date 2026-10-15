use attendance_service::{app, schema::app::AppState};
use axum_test::TestServer;
use sqlx::{PgPool, postgres::PgPoolOptions};
use testcontainers_modules::{
    postgres::{self, Postgres},
    testcontainers::{ContainerAsync, runners::AsyncRunner},
};
use tokio::sync::broadcast;

#[derive(Debug)]
pub struct TestApp {
    pub server: TestServer,
    pub pool: PgPool,
    _container: ContainerAsync<Postgres>,
}

//this will create a container per test and drop it when it ends
pub async fn spawn_app() -> TestApp {
    let container = postgres::Postgres::default().start().await.unwrap();
    let port = container.get_host_port_ipv4(5432).await.unwrap();
    let url = format!("postgres://postgres:postgres@127.0.0.1:{port}/postgres");
    let (tx, _) = broadcast::channel::<String>(100);

    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&url)
        .await
        .unwrap();

    sqlx::migrate!().run(&pool).await.unwrap();

    let appstate = AppState {
        pool,
        secret: "test_secret_key".to_string(),
        tx,
    };

    TestApp {
        pool: appstate.pool.clone(),
        server: TestServer::new(app(appstate)),
        _container: container,
    }
}
