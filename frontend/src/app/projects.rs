use rust_i18n::t;
use topcoat::{
    Result,
    context::Cx,
    router::{href, page},
    view::{View, component, suspense, view},
};

use crate::{
    components::{
        layout::page::{PagePadding, page as page_layout},
        loading::cards_skeleton::cards_skeleton,
        typography::page_title::page_title,
    },
    composables::projects::projects as all_projects,
    features::projects::{ProjectEntry, list::project_list},
    types::content::Project,
};

pub mod slug;

#[page]
pub async fn page() -> Result<impl View> {
    Ok(view! {
        page_layout(
            padding: PagePadding::Standard,
            page_title((t!("projects.title")))
            suspense(fallback: view! { cards_skeleton(count: 6) }, every_project())
        )
    })
}

/// Every project, each linking to its page.
#[component]
async fn every_project(cx: &Cx) -> Result<impl View> {
    let entries = entries(cx, all_projects(cx).await?);
    Ok(view! { project_list(entries: entries) })
}

/// Each project with the URL of its page here, for the features that link to it.
pub fn entries(cx: &Cx, projects: Vec<Project>) -> Vec<ProjectEntry> {
    projects
        .into_iter()
        .map(|project| ProjectEntry {
            page: href!(slug::page, slug::Slug(project.slug.as_str())).resolve(cx),
            project,
        })
        .collect()
}
