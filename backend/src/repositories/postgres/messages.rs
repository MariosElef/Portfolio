use sqlx::PgPool;
use uuid::Uuid;

use crate::models::NewMessage;

/// Stores a message as it was sent before the inbox moved here, keeping its time.
pub async fn insert_moved(
    pool: &PgPool,
    message: &NewMessage,
    user_agent: Option<&str>,
    created_at: chrono::DateTime<chrono::Utc>,
) -> sqlx::Result<()> {
    sqlx::query_file!(
        "src/repositories/postgres/sql/messages/insert_moved.sql",
        Uuid::now_v7(),
        message.name,
        message.email,
        message.message,
        user_agent,
        created_at
    )
    .execute(pool)
    .await?;
    Ok(())
}

/// Stores a message sent through the contact form.
pub async fn insert(
    pool: &PgPool,
    message: &NewMessage,
    user_agent: Option<&str>,
) -> sqlx::Result<()> {
    sqlx::query_file!(
        "src/repositories/postgres/sql/messages/insert.sql",
        Uuid::now_v7(),
        message.name,
        message.email,
        message.message,
        user_agent
    )
    .execute(pool)
    .await?;
    Ok(())
}
