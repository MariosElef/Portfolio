use topcoat::{
    Result,
    icon::{IconData, icon},
    runtime::link,
    view::{AttributeValueViewParts, Child, View, attributes, component, view},
};

use super::external_link::external_link;

const CLASSES: &str =
    "inline-flex items-center gap-2 text-sm text-muted-foreground hover:text-foreground";

/// A quiet text link led by an icon, such as "← All projects" or a GitHub profile.
/// `external` opens it in a new tab.
#[component]
pub async fn icon_link<H>(
    href: H,
    icon_data: IconData,
    #[default] external: bool,
    #[default] child: Child<'_>,
) -> Result<impl View>
where
    H: AttributeValueViewParts + Send,
{
    Ok(view! {
        if external {
            external_link(href: href, attrs: attributes! { class=(CLASSES) },
                icon(data: icon_data)
                (child)
            )
        } else {
            link(href: href, attrs: attributes! { class=(CLASSES) }, icon(data: icon_data) (child))
        }
    })
}
