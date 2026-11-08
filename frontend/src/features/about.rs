use rust_i18n::t;
use topcoat::{
    Result,
    context::Cx,
    view::{View, component, view},
};

use crate::components::typography::prose::prose;
use crate::{
    components::{
        content_blocks::content_blocks,
        layout::split_hero::split_hero,
        media::portrait::portrait,
        typography::page_title::{TitleSize, page_title},
    },
    composables::{pages::page, site::site},
};

/// "About me": the heading and the `about` page's blocks, with its portrait beside them
/// when it has one.
#[component]
pub async fn about_section(cx: &Cx) -> Result<impl View> {
    let about = page(cx, "about").await?;
    let (image, blocks) = about.map_or((None, Vec::new()), |page| (page.image, page.blocks));
    let portrait_alt = site(cx).await.portrait_alt;

    Ok(view! {
        split_hero(
            aside: view! {
                if let Some(image) = image {
                    portrait(src: &image.url, alt: &portrait_alt)
                }
            }.into(),
            page_title(size: TitleSize::Display, (t!("about.title")))
            prose(content_blocks(blocks: blocks))
        )
    })
}
