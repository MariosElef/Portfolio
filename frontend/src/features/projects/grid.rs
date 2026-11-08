use rust_i18n::t;
use topcoat::{
    Result,
    view::{View, component, view},
};

use super::ProjectEntry;
use crate::components::layout::card_grid::card_grid;
use crate::components::{
    media::{brand_logo::MarkSize, project_mark::project_mark, tile::TileTone},
    surfaces::feature_card::{feature_card, more_card},
};

/// Project cards that reveal their cover on hover. A card opens the project's repository
/// or live site in a new tab when it has one, and its page here otherwise. With `lead`,
/// the first card is wide; with `more`, the grid ends with an arrow to that page.
#[component]
pub async fn project_grid(
    entries: Vec<ProjectEntry>,
    #[default] lead: bool,
    #[default] more: Option<String>,
) -> Result<impl View> {
    let all_label = t!("projects.all");

    Ok(view! {
        card_grid(
            #[key(entry.project.slug.clone())]
            for (index, entry) in entries.into_iter().enumerate() {
                let ProjectEntry { project, page } = entry;
                let wide = lead && index == 0;
                let mark = view! {
                    project_mark(
                        logo: project.logo.clone(),
                        initials: &project.initials,
                        tone: TileTone::from_name(&project.tone),
                        size: MarkSize::Sm,
                    )
                };
                // Its repository or live site in a new tab, else its page here.
                let (href, external) = match project.repo.clone() {
                    Some(url) => (url, true),
                    None => (page, false),
                };
                feature_card(
                    href: href,
                    external: external,
                    wide: wide,
                    title: &project.name,
                    description: &project.summary,
                    image: project.cover.clone(),
                    mark: mark.into(),
                )
            }
            if let Some(more) = more {
                more_card(href: more, label: &all_label)
            }
        )
    })
}
