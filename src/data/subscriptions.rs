use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

pub async fn create_subscriber(pool: &PgPool, email: String, name: String) {
    let _ = sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at) 
        VALUES ($1, $2, $3, $4)
    "#,
        Uuid::new_v4(),
        email,
        name,
        Utc::now()
    )
    .execute(pool)
    .await;
}
