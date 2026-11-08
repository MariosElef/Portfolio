use topcoat::{
    Result,
    view::{View, component, view},
};

use crate::{components::typography::rich_text::rich_text, utils::text::paragraphs};

/// Paragraphs of running text, at a readable line length, each rising in as it scrolls
/// into view.
#[component]
pub async fn text_block(text: &str) -> Result<impl View> {
    Ok(view! {
        for paragraph in paragraphs(text) {
            <p data-reveal="" class="max-w-3xl leading-relaxed">rich_text(text: &paragraph)</p>
        }
    })
}
