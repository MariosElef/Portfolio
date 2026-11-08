//! Bringing data stored in an older shape up to date, once, at start-up. Each step finds
//! only what is still in the old shape, so running it again changes nothing.

use std::collections::HashMap;

use mongodb::bson::{self, Bson, Document};

use crate::{
    models::NewMessage,
    repositories::{
        mongo::{case_studies, documents, pages},
        postgres::{media, messages},
    },
    seeds::legend::LEGEND,
};

/// Every upgrade, in order.
pub async fn run(pool: &sqlx::PgPool, mongo: &mongodb::Database) -> anyhow::Result<()> {
    // Case studies were keyed by `slug`; they are keyed by the project they belong to.
    documents::rename_key(mongo, case_studies::COLLECTION, "slug", case_studies::KEY).await?;

    let files: HashMap<String, String> = media::all(pool)
        .await?
        .into_iter()
        .map(|file| (file.id.to_string(), file.file_name))
        .collect();
    for collection in [case_studies::COLLECTION, pages::COLLECTION] {
        for mut document in documents::all_raw(mongo, collection).await? {
            if upgrade_document(&mut document, &files) {
                documents::replace_raw(mongo, collection, &document).await?;
                tracing::info!(collection, id = ?document.get("_id"), "document upgraded");
            }
        }
    }

    move_messages(pool, mongo).await
}

/// Rewrites one document of blocks from its older shape: media named by `media` id
/// becomes media named by file, zone notes go, and a diagram without a legend gets one
/// for the kinds of line it draws. Returns whether anything changed. An id `files` does
/// not know is kept as it was, so the start-up check names it.
fn upgrade_document(document: &mut Document, files: &HashMap<String, String>) -> bool {
    let file = |id: &str| files.get(id).cloned().unwrap_or_else(|| id.to_owned());
    let mut changed = false;

    if let Some(Bson::String(id)) = document.remove("image_id") {
        document.insert("image", file(&id));
        changed = true;
    }
    let Ok(blocks) = document.get_array_mut("blocks") else {
        return changed;
    };
    for block in blocks.iter_mut().filter_map(Bson::as_document_mut) {
        if let Some(Bson::String(id)) = block.remove("media_id") {
            block.insert("file", file(&id));
            changed = true;
        }
        if let Some(Bson::Array(ids)) = block.remove("media_ids") {
            let named: Vec<String> = ids.iter().filter_map(Bson::as_str).map(file).collect();
            block.insert("files", named);
            changed = true;
        }
        if block.get_str("type") != Ok("diagram") {
            continue;
        }
        if let Ok(zones) = block.get_array_mut("zones") {
            for zone in zones.iter_mut().filter_map(Bson::as_document_mut) {
                changed |= zone.remove("note").is_some();
            }
        }
        if !block.contains_key("legend") {
            let kinds: Vec<&str> = block
                .get_array("edges")
                .map(|edges| {
                    edges
                        .iter()
                        .filter_map(|edge| edge.as_document()?.get_str("kind").ok())
                        .collect()
                })
                .unwrap_or_default();
            let mut legend = Document::new();
            for &(kind, label, detail) in LEGEND.iter().filter(|(kind, ..)| kinds.contains(kind)) {
                legend.insert(kind, bson::doc! { "label": label, "detail": detail });
            }
            block.insert("legend", legend);
            changed = true;
        }
    }
    changed
}

/// The contact inbox lived in MongoDB; each message still there moves to Postgres,
/// keeping when it was sent.
async fn move_messages(pool: &sqlx::PgPool, mongo: &mongodb::Database) -> anyhow::Result<()> {
    const OLD_INBOX: &str = "messages";
    let mut moved = Vec::new();
    for document in documents::all_raw(mongo, OLD_INBOX).await? {
        let text = |field: &str| document.get_str(field).unwrap_or_default().to_owned();
        let message = NewMessage {
            name: text("name"),
            email: text("email"),
            message: text("message"),
        };
        let sent = document
            .get_datetime("created_at")
            .ok()
            .and_then(|at| chrono::DateTime::from_timestamp_millis(at.timestamp_millis()))
            .unwrap_or_else(chrono::Utc::now);
        let user_agent = document.get_str("user_agent").ok();
        messages::insert_moved(pool, &message, user_agent, sent).await?;
        moved.extend(document.get("_id").cloned());
    }
    if !moved.is_empty() {
        tracing::info!(count = moved.len(), "messages moved to Postgres");
        documents::delete_raw(mongo, OLD_INBOX, moved).await?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use mongodb::bson::doc;

    use super::*;

    #[test]
    fn media_ids_become_file_names() {
        let files = HashMap::from([("id-1".to_owned(), "abc.png".to_owned())]);
        let mut page = doc! {
            "slug": "about",
            "image_id": "id-1",
            "blocks": [{ "type": "gallery", "media_ids": ["id-1", "gone"] }],
        };
        assert!(upgrade_document(&mut page, &files));
        assert_eq!(page.get_str("image"), Ok("abc.png"));
        let gallery = page.get_array("blocks").unwrap()[0].as_document().unwrap();
        assert_eq!(
            gallery.get_array("files").unwrap(),
            &vec![Bson::from("abc.png"), Bson::from("gone")]
        );
        assert!(
            !upgrade_document(&mut page, &files),
            "a second run changes nothing"
        );
    }

    #[test]
    fn diagrams_get_the_legend_for_their_lines() {
        let mut study = doc! {
            "project": "x",
            "blocks": [{
                "type": "diagram",
                "zones": [{ "id": "a", "name": "A", "note": "gone", "x": 0, "y": 0, "w": 1, "h": 1 }],
                "edges": [{ "from": "a", "to": "b", "kind": "request" }],
            }],
        };
        assert!(upgrade_document(&mut study, &HashMap::new()));
        let diagram = study.get_array("blocks").unwrap()[0].as_document().unwrap();
        let legend = diagram.get_document("legend").unwrap();
        assert_eq!(legend.keys().collect::<Vec<_>>(), ["request"]);
        let zone = diagram.get_array("zones").unwrap()[0]
            .as_document()
            .unwrap();
        assert!(!zone.contains_key("note"));
    }
}
