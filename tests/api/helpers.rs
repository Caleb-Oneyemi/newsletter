use newsletter::{
    config::{get_config, DatabaseSettings},
    startup::run,
};
use sqlx::{Connection, Executor, PgConnection, PgPool};
use std::net::TcpListener;
use uuid::Uuid;

pub struct TestApp {
    pub address: String,
    pub db_pool: PgPool,
}

pub async fn spawn_app() -> TestApp {
    let listener = TcpListener::bind("127.0.0.1:0").expect("failed to bind random port");
    let port = listener.local_addr().unwrap().port();

    let address = format!("http://127.0.0.1:{}", port);
    let mut config = get_config().expect("failed to read config.yaml");
    config.db.name = Uuid::new_v4().to_string();

    let connection_pool = configure_database(&config.db).await;

    let server = run(listener, connection_pool.clone()).expect("failed to bind address");
    let _ = tokio::spawn(server);
    TestApp {
        address,
        db_pool: connection_pool,
    }
}

pub async fn configure_database(config: &DatabaseSettings) -> PgPool {
    // Create database
    let mut conn = PgConnection::connect(&config.get_test_connection_string())
        .await
        .expect("Failed to connect to Postgres");

    conn.execute(format!(r#"CREATE DATABASE "{}";"#, config.name).as_str())
        .await
        .expect("Failed to create database.");

    // Migrate database
    let conn_pool = PgPool::connect(&config.get_connection_string())
        .await
        .expect("Failed to connect to Postgres.");

    sqlx::migrate!("./migrations")
        .run(&conn_pool)
        .await
        .expect("Failed to migrate the database");

    conn_pool
}
