//! Work history as a vertical timeline: the year beside a line with a dot per role; the
//! role beside it; and the tools used in a square on the right.

use topcoat::{
    Result,
    view::{View, component, view},
};

use super::tech::tool_grid::tool_grid;
use crate::types::content::Experience;

mod rail;
mod role;

/// The labels the timeline prints, already translated.
pub struct TimelineLabels {
    pub read_more: String,
    pub did: String,
    pub learned: String,
}

/// Every role, newest first.
#[component]
pub async fn experience_timeline(
    roles: Vec<Experience>,
    labels: TimelineLabels,
) -> Result<impl View> {
    let last = roles.len().saturating_sub(1);

    Ok(view! {
        <ol class="flex flex-col">
            for (index, entry) in roles.into_iter().enumerate() {
                let tools = entry.tags.clone();
                let (year, color, accent) = (entry.year.clone(), entry.color.clone(), entry.accent.clone());
                <li class="grid grid-cols-[3.5rem_1.25rem_minmax(0,1fr)] gap-x-4 sm:grid-cols-[5rem_1.25rem_minmax(0,1fr)] sm:gap-x-6">
                    rail::rail(year: &year, color: &color, accent: &accent, last: index == last)
                    // The role on the left; the tools in a column on the right.
                    <div class="grid gap-x-12 gap-y-6 pb-14 lg:grid-cols-[minmax(0,1fr)_minmax(0,22rem)]">
                        role::role_summary(role: entry, labels: &labels)
                        tool_grid(tools: tools)
                    </div>
                </li>
            }
        </ol>
    })
}
