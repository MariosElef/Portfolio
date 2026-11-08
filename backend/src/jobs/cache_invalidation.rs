//! Keeping the content cache true: each change deletes the cached keys that show it.

use std::time::Duration;

use futures_util::StreamExt;
use mongodb::{
    bson::{Document, doc},
    change_stream::event::ChangeStreamEvent,
    options::FullDocumentType,
};
use serde::Deserialize;
use sqlx::{PgPool, postgres::PgListener};

use crate::{
    repositories::mongo::{case_studies, home, pages, site},
    repositories::redis::{
        Redis,
        keys::{self, CONTENT_PREFIX},
    },
};

/// What is cleared: these keys, and every key under these prefixes.
#[derive(Debug, Default, PartialEq)]
struct Stale {
    keys: Vec<String>,
    prefixes: Vec<&'static str>,
}

impl Stale {
    fn keys(keys: &[&str]) -> Self {
        Self {
            keys: keys.iter().map(|key| key.to_string()).collect(),
            prefixes: Vec::new(),
        }
    }

    fn and_prefix(mut self, prefix: &'static str) -> Self {
        self.prefixes.push(prefix);
        self
    }

    async fn clear(&self, redis: &Redis) -> redis::RedisResult<()> {
        redis.delete(&self.keys).await?;
        for prefix in &self.prefixes {
            redis.delete_prefix(prefix).await?;
        }
        Ok(())
    }
}

/// A Postgres notification: the table changed and the changed row's key.
#[derive(Debug, Deserialize)]
struct RecordChanged {
    table: String,
    key: Option<String>,
}

/// The keys showing a changed row. Anything unknown clears all content, to be safe.
fn stale_for_record(change: &RecordChanged) -> Stale {
    let key = change.key.as_deref().unwrap_or_default();
    match change.table.as_str() {
        // A rename changes the slug, so every project's key goes, not just the new one.
        "projects" => Stale::keys(&[keys::PROJECTS]).and_prefix(keys::PROJECT_PREFIX),
        "project_tools" => Stale::keys(&[keys::PROJECTS, &keys::project(key)]),
        "experience" | "experience_tools" => Stale::keys(&[keys::EXPERIENCE]),
        "skills" | "skill_areas" => Stale::keys(&[keys::SKILLS]),
        // Tools are named on the stack, the projects and the roles.
        "tools" | "tool_categories" => {
            Stale::keys(&[keys::STACK, keys::PROJECTS, keys::EXPERIENCE])
                .and_prefix(keys::PROJECT_PREFIX)
        }
        // Media can be shown by anything.
        _ => Stale::default().and_prefix(CONTENT_PREFIX),
    }
}

/// Clears the cached keys showing each change Postgres reports, so edits made straight
/// in the database show up at once. Runs until the process exits, reconnecting after
/// errors.
pub fn spawn_content_listener(pool: PgPool, redis: Redis) {
    tokio::spawn(async move {
        loop {
            if let Err(error) = listen(&pool, &redis).await {
                tracing::warn!(%error, "content listener stopped; reconnecting");
            }
            tokio::time::sleep(Duration::from_secs(2)).await;
        }
    });
}

async fn listen(pool: &PgPool, redis: &Redis) -> anyhow::Result<()> {
    let mut listener = PgListener::connect_with(pool).await?;
    listener.listen("content_changed").await?;
    // A change may have landed while no listener was connected.
    redis.delete_prefix(CONTENT_PREFIX).await?;

    loop {
        let notification = listener.recv().await?;
        let stale = match serde_json::from_str::<RecordChanged>(notification.payload()) {
            Ok(change) => stale_for_record(&change),
            Err(_) => Stale::default().and_prefix(CONTENT_PREFIX),
        };
        tracing::info!(change = notification.payload(), ?stale, "record changed");
        stale.clear(redis).await?;
    }
}

/// The keys showing a changed document in `collection`, found by its slug; without
/// one (a deleted document), every key of that collection.
fn stale_for_document(collection: &str, slug: Option<&str>) -> Stale {
    match (collection, slug) {
        (case_studies::COLLECTION, Some(slug)) => Stale::keys(&[&keys::case_study(slug)]),
        (case_studies::COLLECTION, None) => Stale::default().and_prefix(keys::CASE_STUDY_PREFIX),
        (pages::COLLECTION, Some(slug)) => Stale::keys(&[&keys::page(slug)]),
        (pages::COLLECTION, None) => Stale::default().and_prefix(keys::PAGE_PREFIX),
        (site::COLLECTION, _) => Stale::keys(&[keys::SITE]),
        (home::COLLECTION, _) => Stale::keys(&[keys::HERO]),
        _ => Stale::default().and_prefix(CONTENT_PREFIX),
    }
}

/// Clears the cached keys showing each document that changes in MongoDB (case studies,
/// pages, the site, the home page), through a change stream on the database, which is why Mongo runs
/// as a replica set.
pub fn spawn_document_watcher(db: mongodb::Database, redis: Redis) {
    tokio::spawn(async move {
        loop {
            if let Err(error) = watch(&db, &redis).await {
                tracing::warn!(%error, "document watcher stopped; reconnecting");
            }
            tokio::time::sleep(Duration::from_secs(2)).await;
        }
    });
}

async fn watch(db: &mongodb::Database, redis: &Redis) -> anyhow::Result<()> {
    let watched = [
        case_studies::COLLECTION,
        pages::COLLECTION,
        site::COLLECTION,
        home::COLLECTION,
    ];
    let mut stream = db
        .watch()
        .pipeline([doc! { "$match": { "ns.coll": { "$in": watched.as_slice() } } }])
        // The document as it is after an update, to know which one it was.
        .full_document(FullDocumentType::UpdateLookup)
        .await?;
    redis.delete_prefix(CONTENT_PREFIX).await?;

    while let Some(event) = stream.next().await.transpose()? {
        let event: ChangeStreamEvent<Document> = event;
        let collection = event
            .ns
            .as_ref()
            .and_then(|ns| ns.coll.as_deref())
            .unwrap_or_default();
        let slug = event.full_document.as_ref().and_then(|document| {
            document
                .get_str(case_studies::KEY)
                .or_else(|_| document.get_str("slug"))
                .ok()
        });
        let stale = stale_for_document(collection, slug);
        tracing::info!(collection, slug, ?stale, "document changed");
        stale.clear(redis).await?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn change(table: &str, key: &str) -> RecordChanged {
        RecordChanged {
            table: table.into(),
            key: Some(key.into()),
        }
    }

    #[test]
    fn a_project_tool_clears_only_that_project() {
        let stale = stale_for_record(&change("project_tools", "mc2"));
        assert_eq!(stale.keys, ["content:projects", "content:project:mc2"]);
        assert!(stale.prefixes.is_empty());
    }

    #[test]
    fn media_and_unknown_tables_clear_everything() {
        for table in ["media", "something_new"] {
            assert_eq!(
                stale_for_record(&change(table, "x")).prefixes,
                [CONTENT_PREFIX]
            );
        }
    }

    #[test]
    fn a_case_study_clears_only_itself() {
        let stale = stale_for_document(case_studies::COLLECTION, Some("mc2"));
        assert_eq!(stale.keys, ["content:case-study:mc2"]);
        assert!(stale.prefixes.is_empty());
    }
}
