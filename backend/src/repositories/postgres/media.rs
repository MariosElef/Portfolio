use sqlx::PgPool;
use uuid::Uuid;

use crate::models::MediaRef;

/// Records a stored file and returns its id. The same bytes uploaded twice keep their
/// first id, since the file name is their hash.
pub async fn insert(
    pool: &PgPool,
    file_name: &str,
    content_type: &str,
    size_bytes: i64,
) -> sqlx::Result<Uuid> {
    sqlx::query_file_scalar!(
        "src/repositories/postgres/sql/media/insert.sql",
        Uuid::now_v7(),
        file_name,
        content_type,
        size_bytes
    )
    .fetch_one(pool)
    .await
}

/// Records a file found in the media folder, unless it is already known.
pub async fn register(
    pool: &PgPool,
    file_name: &str,
    content_type: &str,
    size_bytes: i64,
) -> sqlx::Result<()> {
    sqlx::query_file!(
        "src/repositories/postgres/sql/media/register.sql",
        Uuid::now_v7(),
        file_name,
        content_type,
        size_bytes
    )
    .execute(pool)
    .await?;
    Ok(())
}

pub struct MediaFile {
    pub id: Uuid,
    pub file_name: String,
    pub content_type: String,
}

/// The files with these names, as the site references them.
pub async fn find_by_names(pool: &PgPool, names: &[String]) -> sqlx::Result<Vec<MediaFile>> {
    sqlx::query_file_as!(
        MediaFile,
        "src/repositories/postgres/sql/media/find_by_names.sql",
        names
    )
    .fetch_all(pool)
    .await
}

/// Every file `media` knows.
pub async fn all(pool: &PgPool) -> sqlx::Result<Vec<MediaFile>> {
    sqlx::query_file_as!(MediaFile, "src/repositories/postgres/sql/media/all.sql")
        .fetch_all(pool)
        .await
}

/// Every file name `media` knows.
pub async fn file_names(pool: &PgPool) -> sqlx::Result<Vec<String>> {
    sqlx::query_file_scalar!("src/repositories/postgres/sql/media/file_names.sql")
        .fetch_all(pool)
        .await
}

/// A media reference from the two columns a `LEFT JOIN media` brings, or `None` without it.
pub fn media_ref(file: Option<String>, content_type: Option<String>) -> Option<MediaRef> {
    Some(MediaRef::new(&file?, content_type?))
}
