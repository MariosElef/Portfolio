use topcoat::{
    Result,
    icon::{icon, iconify::iconify_icon},
    runtime::link,
    view::{
        AttributeValueViewParts, Child, StaticClass, View, ViewExt, attributes, class, component,
        view,
    },
};

use crate::components::actions::external_link::external_link;
use crate::components::layout::{
    spread::spread,
    stack::{StackGap, stack},
};
use crate::types::content::MediaRef;

const CARD: StaticClass = class!(
    "group relative isolate flex h-72 flex-col justify-between overflow-hidden rounded-xl p-6 \
     bg-card text-card-foreground transition-colors hover:text-white focus-visible:text-white \
     focus-visible:ring-3 focus-visible:ring-ring/50 focus-visible:outline-none",
);

/// Shown only on hover or keyboard focus.
const REVEAL: StaticClass = class!(
    "absolute inset-0 -z-10 opacity-0 transition-opacity duration-300 \
     group-hover:opacity-100 group-focus-visible:opacity-100",
);

/// What a [`feature_card`] shows: its photo behind, its mark (with an arrow for another
/// site) at the top, and its title and line at the bottom.
#[component]
async fn card_face(
    title: &str,
    description: &str,
    image: Option<MediaRef>,
    mark: Child<'_>,
    external: bool,
    wide: bool,
) -> Result<impl View> {
    Ok(view! {
        if let Some(image) = image {
            <img
                src=(image.url)
                alt=""
                loading="lazy"
                class=(class!(REVEAL, "size-full scale-105 object-cover transition-[opacity,scale] duration-500 group-hover:scale-100"))
            >
            <div class=(class!(REVEAL, "bg-linear-to-t from-black/85 via-black/35 to-black/10"))></div>
        }
        spread(
            (mark)
            if external {
                icon(
                    data: iconify_icon!("lucide:arrow-up-right"),
                    attrs: attributes! { class="size-5 text-muted-foreground transition-colors group-hover:text-white" }
                )
            }
        )
        stack(gap: StackGap::Snug,
            <h3 class=(class!("font-bold tracking-tight", "text-3xl sm:text-4xl" if wide else "text-2xl"))>(title)</h3>
            if !description.is_empty() {
                <p class="text-sm text-muted-foreground transition-colors group-hover:text-white/85 group-focus-visible:text-white/85">
                    (description)
                </p>
            }
        )
    }
    .boxed())
}

/// A tall linked card: an icon at the top, the title and a line at the bottom. With an
/// `image`, the photo fades in behind the text on hover, under a gradient that keeps
/// the text readable.
#[component]
pub async fn feature_card<H>(
    href: H,
    title: &str,
    #[default] description: &str,
    #[default] image: Option<MediaRef>,
    /// The mark in the top corner, such as a tile.
    #[default]
    mark: Child<'_>,
    /// Another site: opens in a new tab, with an arrow in the corner.
    #[default]
    external: bool,
    /// Spans two columns of a three-column grid, for the one card to lead with.
    #[default]
    wide: bool,
) -> Result<impl View>
where
    H: AttributeValueViewParts + Send,
{
    let classes = class!(CARD, "lg:col-span-2" if wide);
    Ok(view! {
        if external {
            external_link(
                href: href,
                attrs: attributes! { class=(classes) },
                card_face(title: title, description: description, image: image, mark: mark, external: true, wide: wide)
            )
        } else {
            link(
                href: href,
                attrs: attributes! { class=(classes) },
                card_face(title: title, description: description, image: image, mark: mark, external: false, wide: wide)
            )
        }
    })
}

/// The last card in a grid of [`feature_card`]s: an arrow to the full list.
#[component]
pub async fn more_card<H>(href: H, label: &str) -> Result<impl View>
where
    H: AttributeValueViewParts + Send,
{
    Ok(view! {
        link(
            href: href,
            attrs: attributes! { class=(class!(CARD, "hover:bg-accent focus-visible:bg-accent")) },
            <span></span>
            <span class="flex items-center justify-between gap-4">
                <span class="text-2xl font-bold tracking-tight">(label)</span>
                icon(
                    data: iconify_icon!("lucide:arrow-right"),
                    attrs: attributes! { class="size-7 transition-transform group-hover:translate-x-1" }
                )
            </span>
        )
    })
}
