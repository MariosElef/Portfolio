use topcoat::{
    Result,
    icon::{icon, iconify::iconify_icon},
    runtime::link,
    view::{AttributeValueViewParts, Attributes, Child, View, attributes, class, component, view},
};

use super::button::{ButtonShape, ButtonSize, ButtonVariant, button_variants};
use super::external_link::external_link;
use crate::components::tone::Color;

/// Where a [`link_button`] goes.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum LinkKind {
    /// A page of this site, opened in place.
    #[default]
    Page,
    /// Another site, in a new tab, with an arrow to say so.
    External,
    /// Nowhere yet: drawn faded, and not a link at all.
    Disabled,
}

/// A link that looks like a [`button`](super::button::button).
#[component]
pub async fn link_button<H>(
    href: H,
    #[default] kind: LinkKind,
    #[default] color: Color,
    #[default] shape: ButtonShape,
    #[default] variant: ButtonVariant,
    #[default] size: ButtonSize,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View>
where
    H: AttributeValueViewParts + Send,
{
    let classes = class!(
        button_variants(color, shape, variant, size),
        attrs.remove("class")
    );
    Ok(view! {
        match kind {
            LinkKind::Page => link(href: href, attrs: attributes! { class=(classes) (attrs) }, (child)),
            LinkKind::External => {
                external_link(href: href, attrs: attributes! { class=(classes) (attrs) },
                    (child)
                    icon(data: iconify_icon!("lucide:arrow-up-right"))
                )
            },
            LinkKind::Disabled => {
                <span aria-disabled="true" class=(class!(classes, "cursor-default opacity-50")) (attrs)>(child)</span>
            },
        }
    })
}
