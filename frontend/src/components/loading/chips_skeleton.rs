use topcoat::{
    Result,
    view::{View, attributes, component, view},
};

use super::skeleton::skeleton;

/// Small tiles still loading, wrapping across, such as the tools of the stack.
#[component]
pub async fn chips_skeleton(#[default(18)] count: usize) -> Result<impl View> {
    Ok(view! {
        <div class="flex flex-wrap gap-3">
            for _ in 0..count {
                skeleton(attrs: attributes! { class="size-20 rounded-xl" })
            }
        </div>
    })
}
