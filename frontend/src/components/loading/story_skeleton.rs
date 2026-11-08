use topcoat::{
    Result,
    view::{View, attributes, component, view},
};

use super::{skeleton::skeleton, text_skeleton::text_skeleton};

/// A project's story still loading: paragraphs, a heading and a large figure.
#[component]
pub async fn story_skeleton() -> Result<impl View> {
    Ok(view! {
        <div aria-busy="true" class="flex flex-col gap-8">
            text_skeleton(lines: 4)
            skeleton(attrs: attributes! { class="h-8 w-56" })
            skeleton(attrs: attributes! { class="h-96 w-full rounded-xl" })
            text_skeleton(lines: 3)
        </div>
    })
}
