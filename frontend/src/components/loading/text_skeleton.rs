use topcoat::{
    Result,
    view::{View, attributes, component, view},
};

use super::skeleton::skeleton;

/// Lines of text still loading: `lines` bars, the last one shorter.
#[component]
pub async fn text_skeleton(#[default(3)] lines: usize) -> Result<impl View> {
    Ok(view! {
        <div class="flex flex-col gap-3">
            for line in 0..lines {
                skeleton(attrs: attributes! { class=(if line + 1 == lines { "h-4 w-2/3" } else { "h-4 w-full" }) })
            }
        </div>
    })
}
