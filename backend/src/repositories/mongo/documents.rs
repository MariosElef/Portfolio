//! Documents in MongoDB, each found by one unique field: what every such collection
//! shares, and the raw access the start-up upgrade needs.

use mongodb::{
    Collection, Database, IndexModel,
    bson::{Bson, Document, doc},
    options::IndexOptions,
};
use serde::{Serialize, de::DeserializeOwned};

use crate::models::Keyed;

fn collection<T: Send + Sync>(db: &Database, name: &str) -> Collection<T> {
    db.collection(name)
}

/// The document whose key is `key` in collection `name`.
pub async fn find<T>(db: &Database, name: &str, key: &str) -> mongodb::error::Result<Option<T>>
where
    T: Keyed + DeserializeOwned + Send + Sync,
{
    collection::<T>(db, name)
        .find_one(doc! { T::KEY: key })
        .await
}

/// Every document in collection `name`.
pub async fn all<T>(db: &Database, name: &str) -> mongodb::error::Result<Vec<T>>
where
    T: DeserializeOwned + Send + Sync,
{
    use futures_util::TryStreamExt;
    collection::<T>(db, name)
        .find(doc! {})
        .await?
        .try_collect()
        .await
}

/// One document per key.
pub async fn ensure_key_index<T: Keyed>(db: &Database, name: &str) -> mongodb::error::Result<()> {
    collection::<Document>(db, name)
        .create_index(
            IndexModel::builder()
                .keys(doc! { T::KEY: 1 })
                .options(IndexOptions::builder().unique(true).build())
                .build(),
        )
        .await?;
    Ok(())
}

/// Inserts each document whose key is not there yet; the rest are left as they are.
pub async fn insert_missing<T>(
    db: &Database,
    name: &str,
    documents: Vec<T>,
) -> mongodb::error::Result<()>
where
    T: Keyed + Serialize + Send + Sync,
{
    let collection = collection::<T>(db, name);
    for document in documents {
        let filter = doc! { T::KEY: document.key() };
        if collection
            .clone_with_type::<Document>()
            .find_one(filter)
            .await?
            .is_none()
        {
            collection.insert_one(document).await?;
        }
    }
    Ok(())
}

pub async fn is_empty(db: &Database, name: &str) -> mongodb::error::Result<bool> {
    Ok(collection::<Document>(db, name)
        .estimated_document_count()
        .await?
        == 0)
}

/// Every document in collection `name`, as stored.
pub async fn all_raw(db: &Database, name: &str) -> mongodb::error::Result<Vec<Document>> {
    all(db, name).await
}

/// Writes `document` over the one with its `_id`.
pub async fn replace_raw(
    db: &Database,
    name: &str,
    document: &Document,
) -> mongodb::error::Result<()> {
    let id = document.get("_id").cloned().unwrap_or(Bson::Null);
    collection::<Document>(db, name)
        .replace_one(doc! { "_id": id }, document)
        .await?;
    Ok(())
}

/// Renames field `from` to `to` in every document that still has `from`, first dropping
/// the unique index on `from`, if there is one.
pub async fn rename_key(
    db: &Database,
    name: &str,
    from: &str,
    to: &str,
) -> mongodb::error::Result<()> {
    let collection = collection::<Document>(db, name);
    // No such index, on a fresh database, is fine.
    let _ = collection.drop_index(format!("{from}_1")).await;
    collection
        .update_many(
            doc! { from: { "$exists": true }, to: { "$exists": false } },
            doc! { "$rename": { from: to } },
        )
        .await?;
    Ok(())
}

/// Deletes the documents with these `_id`s.
pub async fn delete_raw(db: &Database, name: &str, ids: Vec<Bson>) -> mongodb::error::Result<()> {
    collection::<Document>(db, name)
        .delete_many(doc! { "_id": { "$in": ids } })
        .await?;
    Ok(())
}
