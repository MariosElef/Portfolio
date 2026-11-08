use topcoat::{
    Result,
    view::{View, component, view},
};

use super::tech_icon::tech_icon;

/// A tool named in running text: its logo, in its colour, then the words.
#[component]
pub async fn tech_mention(icon_name: &str, label: &str) -> Result<impl View> {
    Ok(view! {
        <span class="inline-flex items-baseline gap-1 font-bold whitespace-nowrap">
            tech_icon(key: icon_name)
            (label)
        </span>
    })
}
