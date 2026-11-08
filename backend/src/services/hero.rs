use std::collections::HashMap;

use super::cache::{UNTIL_CHANGED, cached};
use crate::{
    models::{HeroSlide, MediaRef},
    repositories::{mongo::home, postgres::media, redis::keys},
    state::AppState,
};

/// The home page's slides, with their media files resolved to URLs. Media `media` does
/// not know is left out.
pub async fn hero(state: &AppState) -> anyhow::Result<Vec<HeroSlide>> {
    cached(state, keys::HERO, UNTIL_CHANGED, async {
        let Some(home) = home::find(&state.mongo).await? else {
            return Ok(Vec::new());
        };
        let names: Vec<String> = home.files().cloned().collect();
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
        let lookup =
            |name: &Option<String>| name.as_ref().and_then(|name| files.get(name).cloned());

        Ok(home
            .slides
            .into_iter()
            .map(|slide| HeroSlide {
                media: lookup(&slide.media),
                poster: lookup(&slide.poster),
                tab_label: slide.tab_label,
                eyebrow: slide.eyebrow,
                title: slide.title,
                subtitle: slide.subtitle,
                callout: slide.callout,
                cta_label: slide.cta_label,
                cta_href: slide.cta_href,
                code: slide.code,
                code_language: slide.code_language,
            })
            .collect())
    })
    .await
}
