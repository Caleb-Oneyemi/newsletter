use newsletter::{
    config::{get_config, DatabaseSettings},
    providers::EmailClient,
    startup::run,
    telemetry::{get_tracing_subscriber, init_tracing_subscriber},
};
use once_cell::sync::Lazy;
use secrecy::ExposeSecret;
use sqlx::{Connection, Executor, PgConnection, PgPool};
use std::net::TcpListener;
use uuid::Uuid;

pub struct TestApp {
    pub address: String,
    pub db_pool: PgPool,
}

//ensures tracing is only invoked once in tests
static TRACING: Lazy<()> = Lazy::new(|| {
    let name = "newsletter_tests".into();
    let log_level = "info".into();

    //run tests with `ALLOW_TEST_LOGS=true` to enable logging in tests
    if std::env::var("ALLOW_TEST_LOGS").is_ok() {
        let subscriber = get_tracing_subscriber(name, log_level, std::io::stdout);
        init_tracing_subscriber(subscriber);
    } else {
        let subscriber = get_tracing_subscriber(name, log_level, std::io::sink);
        init_tracing_subscriber(subscriber);
    }
});

pub async fn spawn_app() -> TestApp {
    Lazy::force(&TRACING);

    let listener = TcpListener::bind("127.0.0.1:0").expect("failed to bind random port");
    let port = listener.local_addr().unwrap().port();

    let address = format!("http://127.0.0.1:{}", port);
    let mut config = get_config().expect("failed to read config.yaml");
    config.db.name = Uuid::new_v4().to_string();

    let connection_pool = configure_database(&config.db).await;

    let sender = config
        .email_client
        .get_sender()
        .expect("invalid sender email address");

    let email_client = EmailClient::new(config.email_client.base_url, sender);
    let server =
        run(listener, connection_pool.clone(), email_client).expect("failed to bind address");

    let _ = tokio::spawn(server);
    TestApp {
        address,
        db_pool: connection_pool,
    }
}

pub async fn configure_database(config: &DatabaseSettings) -> PgPool {
    // Create database
    let mut conn = PgConnection::connect(&config.get_test_connection_string().expose_secret())
        .await
        .expect("Failed to connect to Postgres");

    conn.execute(format!(r#"CREATE DATABASE "{}";"#, config.name).as_str())
        .await
        .expect("Failed to create database.");

    // Migrate database
    let conn_pool = PgPool::connect(&config.get_connection_string().expose_secret())
        .await
        .expect("Failed to connect to Postgres.");

    sqlx::migrate!("./migrations")
        .run(&conn_pool)
        .await
        .expect("Failed to migrate the database");

    conn_pool
}
