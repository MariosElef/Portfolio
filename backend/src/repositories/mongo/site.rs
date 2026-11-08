//! The site's shared copy: a collection of one document.

use mongodb::{Database, bson::doc};

use crate::models::Site;

pub const COLLECTION: &str = "site";

pub async fn find(db: &Database) -> mongodb::error::Result<Option<Site>> {
    db.collection::<Site>(COLLECTION).find_one(doc! {}).await
}

/// Writes `site`, only into an empty collection, so edits made in the database stay.
pub async fn insert_if_empty(db: &Database, site: &Site) -> mongodb::error::Result<()> {
    let collection = db.collection::<Site>(COLLECTION);
    if collection.estimated_document_count().await? == 0 {
        collection.insert_one(site).await?;
    }
    Ok(())
}
