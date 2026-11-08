use topcoat::{
    Result,
    view::{Child, View, attributes, class, component, view},
};

use super::skeleton::skeleton;
use crate::components::layout::container::CONTAINER;

/// A section still loading, at the site's width: a bar for its heading, then `child`.
#[component]
pub async fn section_skeleton(#[default] child: Child<'_>) -> Result<impl View> {
    Ok(view! {
        <section aria-busy="true" class=(class!(CONTAINER, "flex flex-col gap-8 py-16"))>
            skeleton(attrs: attributes! { class="h-9 w-64" })
            (child)
        </section>
    })
}
