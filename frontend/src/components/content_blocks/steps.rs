use topcoat::{
    Result,
    view::{View, attributes, component, view},
};

use crate::components::{
    lists::ruled::{RowSpacing, ruled_item, ruled_list},
    typography::rich_text::rich_text,
};

/// A step's number, in a small circle.
#[component]
async fn step_number(number: usize) -> Result<impl View> {
    Ok(view! {
        <span class="flex size-7 items-center justify-center rounded-full bg-muted text-xs font-bold tabular-nums">(number.to_string())</span>
    })
}

/// A numbered sequence, such as a request traced step by step, a row each between
/// hairlines.
#[component]
pub async fn step_list(items: Vec<String>) -> Result<impl View> {
    Ok(view! {
        ruled_list(ordered: true, narrow: true,
            for (index, item) in items.into_iter().enumerate() {
                ruled_item(
                    spacing: RowSpacing::Roomy,
                    closed: true,
                    attrs: attributes! { data-reveal="" class="grid grid-cols-[2rem_minmax(0,1fr)] gap-4" },
                    step_number(number: index + 1)
                    <p class="leading-relaxed">rich_text(text: &item)</p>
                )
            }
        )
    })
}
