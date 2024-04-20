use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

#[tracing::instrument(
    name = "saving new subscriber details in the db",
    skip(pool, email, name)
)]
pub async fn create_subscriber(
    pool: &PgPool,
    email: String,
    name: String,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
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
    .await
    .map_err(|e| {
        tracing::error!("Failed to execute query: {:?}", e);
        e
    })?;

    Ok(())
}
