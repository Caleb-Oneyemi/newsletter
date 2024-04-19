use crate::helpers::spawn_app;

#[tokio::test]
async fn subscription_works() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let body = "name=caleb%20oneyemi&email=caleb%40gmail.com";
    let response = client
        .post(&format!("{}/subscriptions", &app.address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(200, response.status().as_u16());

    let saved = sqlx::query!("SELECT email, name FROM subscriptions",)
        .fetch_one(&app.db_pool)
        .await
        .expect("Failed to fetch saved subscription.");

    assert_eq!(saved.email, "caleb@gmail.com");
    assert_eq!(saved.name, "caleb oneyemi");
}

#[tokio::test]
async fn subscription_fails_for_incomplete_data() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let body = "email=caleb%40gmail.com";
    let response = client
        .post(&format!("{}/subscriptions", &app.address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(400, response.status().as_u16());
}

#[tokio::test]
async fn subscription_fails_for_duplicate_data() {
    let app = spawn_app().await;
    let client = reqwest::Client::new();

    let body = "name=caleb%20oneyemi&email=caleb%40gmail.com";
    let response = client
        .post(&format!("{}/subscriptions", &app.address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");

    let response2 = client
        .post(&format!("{}/subscriptions", &app.address))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(200, response.status().as_u16());
    assert_eq!(409, response2.status().as_u16());
}
