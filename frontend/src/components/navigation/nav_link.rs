use topcoat::{
    Result,
    runtime::link,
    view::{AttributeValueViewParts, Child, StaticClass, View, attributes, class, component, view},
};

/// How much a [`nav_link`] stands out.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum NavEmphasis {
    /// An ordinary item.
    #[default]
    Plain,
    /// The call to action, drawn as a pill.
    Pill,
}

fn classes(emphasis: NavEmphasis) -> StaticClass {
    match emphasis {
        // Plain text on the dark bar, brightening on hover; a thin line under it grows from
        // the middle, faint on hover and solid on the current page.
        NavEmphasis::Plain => class!(
            "relative px-2.5 py-2 font-display text-sm font-medium tracking-[-0.01em] \
             text-white/65 transition-colors sm:px-3.5 sm:text-[15px] \
             after:absolute after:inset-x-2.5 after:-bottom-px after:h-0.5 after:origin-center \
             after:scale-x-0 after:rounded-full after:bg-white/40 after:transition-transform \
             after:duration-300 sm:after:inset-x-3.5 \
             hover:text-white hover:after:scale-x-100 \
             aria-[current=page]:text-white aria-[current=page]:after:scale-x-100 \
             aria-[current=page]:after:bg-white"
        ),
        // The white pill, as `ButtonVariant::Inverse` draws it, a little roomier.
        NavEmphasis::Pill => class!(
            "ml-1 rounded-full bg-white px-4 py-2 text-sm font-bold text-ink-950 sm:ml-2 sm:px-5 \
             sm:py-2.5 sm:text-base hover:bg-white/90"
        ),
    }
}

/// A link in the site's bar. `current` marks it as the visitor's page with
/// `aria-current`, which also highlights it.
#[component]
pub async fn nav_link<H>(
    href: H,
    #[default] emphasis: NavEmphasis,
    #[default] current: bool,
    #[default] child: Child<'_>,
) -> Result<impl View>
where
    H: AttributeValueViewParts + Send,
{
    Ok(view! {
        link(
            href: href,
            attrs: attributes! {
                class=(classes(emphasis))
                aria-current=(current.then_some("page"))
            },
            (child)
        )
    })
}
