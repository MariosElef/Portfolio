//! The home page: a collection of one document.

use mongodb::{Database, bson::doc};

use crate::models::HomeDocument;

pub const COLLECTION: &str = "home";

pub async fn find(db: &Database) -> mongodb::error::Result<Option<HomeDocument>> {
    db.collection::<HomeDocument>(COLLECTION)
        .find_one(doc! {})
        .await
}

/// Writes `home`, only into an empty collection, so edits made in the database stay.
pub async fn insert_if_empty(db: &Database, home: &HomeDocument) -> mongodb::error::Result<()> {
    let collection = db.collection::<HomeDocument>(COLLECTION);
    if collection.estimated_document_count().await? == 0 {
        collection.insert_one(home).await?;
    }
    Ok(())
}
