use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

use crate::domain::NewSubscriber;

#[tracing::instrument(
    name = "saving new subscriber details in the db",
    skip(pool, new_subscriber)
)]
pub async fn create_subscriber(
    pool: &PgPool,
    new_subscriber: &NewSubscriber
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at) 
        VALUES ($1, $2, $3, $4)
    "#,
        Uuid::new_v4(),
        new_subscriber.email,
        new_subscriber.name.as_ref(),
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
