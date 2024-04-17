use newsletter::startup_server;

#[tokio::test]
async fn health_check_works() {
    spawn_app()
        .await
        .expect("integration test failed to spawn app");

    let client = reqwest::Client::new();
    let response = client
        .get("http://127.0.0.1:7777/health")
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(
        Some("server is live".chars().count() as u64),
        response.content_length()
    );
}

async fn spawn_app() -> std::io::Result<()> {
    let server = startup_server().expect("integration test failed to bind address");

    let _ = tokio::spawn(server);

    Ok(())
}
