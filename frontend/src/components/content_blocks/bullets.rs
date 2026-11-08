use topcoat::{
    Result,
    view::{View, attributes, component, view},
};

use crate::components::{
    lists::ruled::{RowSpacing, ruled_item, ruled_list},
    typography::rich_text::rich_text,
};

/// Points in a list, a row each between hairlines.
#[component]
pub async fn bullet_list(items: Vec<String>) -> Result<impl View> {
    Ok(view! {
        ruled_list(narrow: true,
            for item in items {
                ruled_item(
                    spacing: RowSpacing::Roomy,
                    closed: true,
                    attrs: attributes! { data-reveal="" class="leading-relaxed" },
                    rich_text(text: &item)
                )
            }
        )
    })
}
