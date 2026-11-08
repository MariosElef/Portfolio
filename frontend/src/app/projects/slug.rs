use topcoat::{
    Result,
    context::Cx,
    router::{error::RouterErrorExt, href, module_param, page, path_param},
    view::{View, component, suspense, view},
};

use crate::{
    components::{
        content_blocks::content_blocks,
        layout::page::{PagePadding, page as page_layout},
        loading::story_skeleton::story_skeleton,
    },
    composables::projects::{project, story},
    features::projects::detail::project_detail,
};

module_param!(pub slug);

/// A project's page. The project is looked up before anything is sent, so a slug with no
/// project answers 404; its story, the larger part, streams in under a skeleton.
#[page]
pub async fn page(cx: &Cx) -> Result<impl View> {
    let slug = path_param::<Slug>(cx).to_owned();
    let project = project(cx, &slug).await?.ok_or_not_found()?;

    Ok(view! {
        page_layout(
            padding: PagePadding::Compact,
            project_detail(
                project: project,
                back: href!(super::page).resolve(cx),
                story: view! {
                    suspense(fallback: view! { story_skeleton() }, project_story(slug: slug))
                }.into(),
            )
        )
    })
}

/// A project's story: its blocks, text, diagrams and the rest.
#[component]
async fn project_story(cx: &Cx, slug: String) -> Result<impl View> {
    let blocks = story(cx, &slug).await?;
    Ok(view! { content_blocks(blocks: blocks) })
}
