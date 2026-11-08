use topcoat::{
    Result,
    view::{View, attributes, component, view},
};

use super::skeleton::skeleton;

/// A grid of cards still loading: `count` of them, three across on wide screens.
#[component]
pub async fn cards_skeleton(#[default(6)] count: usize) -> Result<impl View> {
    Ok(view! {
        <div class="grid gap-3 sm:grid-cols-2 lg:grid-cols-3">
            for _ in 0..count {
                skeleton(attrs: attributes! { class="h-72 rounded-xl" })
            }
        </div>
    })
}
