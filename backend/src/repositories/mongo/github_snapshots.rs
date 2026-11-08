//! The last contribution calendar GitHub gave, per login, to show when it cannot be
//! reached.

use mongodb::{
    Database, IndexModel,
    bson::{DateTime, doc},
    options::IndexOptions,
};
use serde::{Deserialize, Serialize};

use crate::models::Contributions;

pub const COLLECTION: &str = "github_snapshots";

#[derive(Serialize, Deserialize)]
struct Snapshot {
    login: String,
    fetched_at: DateTime,
    calendar: Contributions,
}

/// One snapshot per login.
pub async fn ensure_index(db: &Database) -> mongodb::error::Result<()> {
    db.collection::<Snapshot>(COLLECTION)
        .create_index(
            IndexModel::builder()
                .keys(doc! { "login": 1 })
                .options(IndexOptions::builder().unique(true).build())
                .build(),
        )
        .await?;
    Ok(())
}

/// Keeps `calendar` as the latest for `login`.
pub async fn save(
    db: &Database,
    login: &str,
    calendar: &Contributions,
) -> mongodb::error::Result<()> {
    let snapshot = Snapshot {
        login: login.into(),
        fetched_at: DateTime::now(),
        calendar: calendar.clone(),
    };
    db.collection::<Snapshot>(COLLECTION)
        .replace_one(doc! { "login": login }, snapshot)
        .upsert(true)
        .await?;
    Ok(())
}

pub async fn latest(db: &Database, login: &str) -> mongodb::error::Result<Option<Contributions>> {
    Ok(db
        .collection::<Snapshot>(COLLECTION)
        .find_one(doc! { "login": login })
        .await?
        .map(|snapshot| snapshot.calendar))
}
