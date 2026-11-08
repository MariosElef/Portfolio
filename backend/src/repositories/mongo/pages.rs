//! Pages made of blocks, such as `about`.

use mongodb::Database;

use super::documents;
use crate::models::PageDocument;

pub const COLLECTION: &str = "pages";

pub async fn find(db: &Database, slug: &str) -> mongodb::error::Result<Option<PageDocument>> {
    documents::find(db, COLLECTION, slug).await
}

pub async fn all(db: &Database) -> mongodb::error::Result<Vec<PageDocument>> {
    documents::all(db, COLLECTION).await
}
