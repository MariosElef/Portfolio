use topcoat::{
    Result,
    view::{View, attributes, component, view},
};

use super::skeleton::skeleton;

/// Rows still loading, one under another, such as the roles of a work history.
#[component]
pub async fn rows_skeleton(#[default(4)] count: usize) -> Result<impl View> {
    Ok(view! {
        <div class="flex flex-col gap-3">
            for _ in 0..count {
                skeleton(attrs: attributes! { class="h-24 rounded-xl" })
            }
        </div>
    })
}
