use topcoat::{
    Result,
    view::{View, component, view},
};

use super::{
    logos::{key_for, logo},
    tech_icon::{TechIconSize, tech_icon},
};

/// Tools as a square of logos: as many columns as rows. A known tool is just its logo,
/// in its brand colour, named on hover and to screen readers; anything else is its name.
#[component]
pub async fn tool_grid(tools: Vec<String>) -> Result<impl View> {
    let columns = (tools.len() as f64).sqrt().ceil() as usize;
    Ok(view! {
        if !tools.is_empty() {
            <ul
                style=(format!("grid-template-columns: repeat({columns}, auto)"))
                class="grid content-start items-center justify-items-center gap-4 self-start lg:justify-end lg:pt-2"
            >
                for tool in tools {
                    let key = key_for(&tool);
                    // Only a tool the logos know shows a logo; a fallback icon would say nothing.
                    if logo(&key).1.is_some() {
                        <li title=(tool.clone()) class="inline-flex">
                            tech_icon(key: &key, label: &tool, size: TechIconSize::Large)
                        </li>
                    } else {
                        <li class="text-xs text-muted-foreground">(tool)</li>
                    }
                }
            </ul>
        }
    })
}
