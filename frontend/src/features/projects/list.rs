use rust_i18n::t;
use topcoat::{
    Result,
    view::{View, component, view},
};

use super::ProjectEntry;
use crate::components::layout::card_grid::card_grid;
use crate::components::{
    actions::{
        button::{ButtonSize, ButtonVariant},
        link_button::{LinkKind, link_button},
    },
    media::tile::TileTone,
    surfaces::project_card::project_card,
};

/// Every project as a card: its mark, name, summary and tools, with its page here
/// bottom-left and "Open" bottom-right, which goes to its repository or live site in a
/// new tab. With neither, "Coming soon" or "Private" stands in its place, not a link.
#[component]
pub async fn project_list(entries: Vec<ProjectEntry>) -> Result<impl View> {
    let details = t!("projects.details").into_owned();
    let open = t!("projects.open").into_owned();
    let soon = t!("projects.coming_soon").into_owned();
    let private = t!("projects.private").into_owned();

    Ok(view! {
        card_grid(
            #[key(entry.project.slug.clone())]
            for entry in entries {
                // Each inner view moves what it uses, so it gets its own copies.
                let ProjectEntry { project, page } = entry;
                let details = details.clone();
                let (target, kind, label) = match project.repo.clone() {
                    Some(repo) => (repo, LinkKind::External, open.clone()),
                    None if project.availability == "private" => (String::new(), LinkKind::Disabled, private.clone()),
                    None => (String::new(), LinkKind::Disabled, soon.clone()),
                };
                project_card(
                    initials: &project.initials,
                    tone: TileTone::from_name(&project.tone),
                    logo: project.logo.clone(),
                    name: &project.name,
                    description: &project.summary,
                    tags: project.tags.iter().map(String::as_str).collect(),
                    links: view! {
                        link_button(href: page, variant: ButtonVariant::Ghost, size: ButtonSize::Md, (details))
                    }.into(),
                    action: view! {
                        link_button(href: target, kind: kind, size: ButtonSize::Md, (label))
                    }.into(),
                )
            }
        )
    })
}
