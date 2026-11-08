use std::collections::HashMap;

use crate::{
    models::{Block, MediaRef, StoredBlock},
    repositories::postgres::media,
    state::AppState,
};

/// Blocks as the site renders them: media files resolved to URLs, and blocks pointing at
/// files `media` does not know dropped.
pub async fn resolve(state: &AppState, blocks: Vec<StoredBlock>) -> anyhow::Result<Vec<Block>> {
    let names: Vec<String> = blocks
        .iter()
        .flat_map(StoredBlock::files)
        .cloned()
        .collect();
    let files: HashMap<String, MediaRef> = media::find_by_names(&state.pool, &names)
        .await?
        .into_iter()
        .map(|file| {
            (
                file.file_name.clone(),
                MediaRef::new(&file.file_name, file.content_type),
            )
        })
        .collect();
    let lookup = |name: &str| files.get(name).cloned();

    Ok(blocks
        .into_iter()
        .filter_map(|block| {
            Some(match block {
                StoredBlock::Text { text } => Block::Text { text },
                StoredBlock::Heading { text } => Block::Heading { text },
                StoredBlock::Image { file, caption } => Block::Image {
                    media: lookup(&file)?,
                    caption,
                },
                StoredBlock::Gallery { files } => Block::Gallery {
                    media: files.iter().filter_map(|file| lookup(file)).collect(),
                },
                StoredBlock::Code { lang, code } => Block::Code { lang, code },
                StoredBlock::Quote { text, author } => Block::Quote { text, author },
                StoredBlock::List { items } => Block::List { items },
                StoredBlock::Steps { items } => Block::Steps { items },
                StoredBlock::Diagram(diagram) => Block::Diagram(diagram),
                StoredBlock::Faq { items } => Block::Faq { items },
            })
        })
        .collect())
}
