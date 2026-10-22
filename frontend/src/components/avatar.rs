use topcoat::{
    Result,
    view::{Attributes, Child, View, class, component, view},
};

use super::tone::Color;

/// A circular image with a fallback behind it.
#[component]
pub async fn avatar(
    #[default] color: Color,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <span
            data-slot="avatar"
            class=(class!(
                "relative flex size-8 shrink-0 overflow-hidden rounded-full",
                color.tone(),
                attrs.remove("class"),
            ))
            (attrs)
        >
            (child)
        </span>
    })
}

/// An image that fills the [`avatar`] and covers its fallback. Pass `src` in `attrs`.
#[component]
pub async fn avatar_image(
    #[into]
    #[default]
    alt: String,
    #[default] mut attrs: Attributes,
) -> Result<impl View> {
    Ok(view! {
        <img
            data-slot="avatar-image"
            alt=(alt)
            class=(class!("absolute inset-0 aspect-square size-full object-cover", attrs.remove("class")))
            (attrs)
        >
    })
}

/// Initials or another small view, shown behind the image.
#[component]
pub async fn avatar_fallback(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <span
            data-slot="avatar-fallback"
            class=(class!(
                "flex size-full items-center justify-center rounded-full bg-(--tone)/15 \
                 text-(--tone-emphasis) select-none",
                attrs.remove("class"),
            ))
            (attrs)
        >
            (child)
        </span>
    })
}
