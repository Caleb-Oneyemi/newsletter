use newsletter::{config::get_config, startup::run};
use sqlx::PgPool;
use std::net::TcpListener;

#[tokio::test]
async fn health_check_works() {
    let address = spawn_app().await;
    let client = reqwest::Client::new();

    let response = client
        .get(&format!("{}/health", &address))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(
        Some("server is live".chars().count() as u64),
        response.content_length()
    );
}

async fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();

    let config = get_config().expect("failed to read config.yaml");
    let db_url = config.db.get_connection_string();
    let conn_pool = PgPool::connect(&db_url)
        .await
        .expect("failed to connect to test db");

    let server = run(listener, conn_pool).expect("integration test failed to bind address");

    let _ = tokio::spawn(server);

    format!("http://127.0.0.1:{}", port)
}
