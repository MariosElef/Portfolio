use topcoat::{
    Result,
    runtime::link,
    view::{AttributeValueViewParts, View, attributes, component, view},
};

use crate::components::typography::wordmark::wordmark;

/// The site's name as a wordmark, linking home. `label` names the link for screen
/// readers, which then skip the wordmark itself.
#[component]
pub async fn brand_link<H>(href: H, label: &str, name: &str) -> Result<impl View>
where
    H: AttributeValueViewParts + Send,
{
    Ok(view! {
        link(
            href: href,
            attrs: attributes! { class="flex shrink-0 items-center px-1" aria-label=(label) },
            <span aria-hidden="true">wordmark(name: name)</span>
        )
    })
}
