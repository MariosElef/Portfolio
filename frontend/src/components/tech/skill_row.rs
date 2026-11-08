use topcoat::{
    Result,
    view::{View, attributes, component, view},
};

use super::skill_stars::skill_stars;
use crate::components::lists::ruled::{RowSpacing, ruled_item};

/// A skill in a [`ruled_list`](crate::components::lists::ruled::ruled_list): its name, and
/// its stars at the end of the row, read out as `label`.
#[component]
pub async fn skill_row(name: &str, level: u8, label: &str) -> Result<impl View> {
    Ok(view! {
        ruled_item(
            spacing: RowSpacing::Snug,
            attrs: attributes! { class="flex items-center justify-between gap-4 text-sm" },
            (name)
            skill_stars(level: level, label: label)
        )
    })
}
