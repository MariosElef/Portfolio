use rust_i18n::t;
use topcoat::{
    Result,
    icon::{icon, iconify::iconify_icon},
    view::{Child, View, component, view},
};

use crate::components::typography::{lead::lead, prose::prose};
use crate::{
    components::{
        actions::{
            button::ButtonSize,
            button_row::button_row,
            icon_link::icon_link,
            link_button::{LinkKind, link_button},
        },
        layout::spread::spread,
        media::{brand_logo::MarkSize, project_mark::project_mark, tile::TileTone},
        typography::page_title::{TitleSize, page_title},
    },
    types::content::Project,
};

/// A project's own page: back to the list, its mark with its name at the top right, its
/// summary, its story, and its source when public. `back` is where the list is.
///
/// In layers: the title, the summary larger and a little heavier, then the story in a
/// softer white with its key words standing out.
#[component]
pub async fn project_detail(
    project: Project,
    /// The story under the summary, which may still be streaming in.
    story: Child<'_>,
    back: String,
) -> Result<impl View> {
    Ok(view! {
        icon_link(href: back, icon_data: iconify_icon!("lucide:arrow-left"), (t!("projects.all")))
        spread(
            project_mark(
                logo: project.logo.clone(),
                initials: &project.initials,
                tone: TileTone::from_name(&project.tone),
                size: MarkSize::Lg,
            )
            page_title(size: TitleSize::Lg, (project.name.as_str()))
        )
        lead((project.summary.as_str()))

        prose(soft: true, (story))

        if let Some(repo) = project.repo.clone() {
            button_row(
                link_button(
                    href: repo,
                    kind: LinkKind::External,
                    size: ButtonSize::Lg,
                    icon(data: iconify_icon!("lucide:github"))
                    (t!("projects.source"))
                )
            )
        }
    })
}
