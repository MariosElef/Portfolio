use super::{
    blocks::resolve,
    cache::{UNTIL_CHANGED, cached_unless_missing},
};
use crate::{
    models::{MediaRef, Page},
    repositories::redis::keys,
    repositories::{mongo::pages, postgres::media},
    state::AppState,
};

/// A page made of blocks, such as `about`, with its picture and media resolved to URLs.
pub async fn page(state: &AppState, slug: &str) -> anyhow::Result<Option<Page>> {
    let key = keys::page(slug);
    cached_unless_missing(state, &key, UNTIL_CHANGED, Option::is_none, async {
        let Some(page) = pages::find(&state.mongo, slug).await? else {
            return Ok(None);
        };
        let image = match page.image {
            Some(name) => media::find_by_names(&state.pool, &[name])
                .await?
                .into_iter()
                .next()
                .map(|file| MediaRef::new(&file.file_name, file.content_type)),
            None => None,
        };
        Ok(Some(Page {
            image,
            blocks: resolve(state, page.blocks).await?,
        }))
    })
    .await
}
