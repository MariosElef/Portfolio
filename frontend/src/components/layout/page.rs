use topcoat::{
    Result,
    view::{Attributes, Child, View, class, component, view},
};

use super::container::CONTAINER;

/// How wide a [`page`] is.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum PageWidth {
    /// The site's width, like every section.
    #[default]
    Wide,
    /// Edge to edge: a page of sections, each at the site's width itself.
    Full,
}

/// The space above and below a [`page`].
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum PagePadding {
    /// None: its sections bring their own.
    #[default]
    None,
    /// A page of its own, such as a list or a form.
    Standard,
    /// A page that starts close under the bar, such as a project's.
    Compact,
}

/// The space between a [`page`]'s parts.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum PageGap {
    #[default]
    Normal,
    /// Between whole sections.
    Sections,
}

/// A page's main content, its parts stacked.
#[component]
pub async fn page(
    #[default] width: PageWidth,
    #[default] gap: PageGap,
    #[default] padding: PagePadding,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <main class=(class!(
            match width {
                PageWidth::Wide => CONTAINER,
                PageWidth::Full => class!("w-full"),
            },
            "flex flex-col",
            match gap {
                PageGap::Normal => class!("gap-8"),
                PageGap::Sections => class!("gap-16"),
            },
            match padding {
                PagePadding::None => None,
                PagePadding::Standard => Some(class!("py-16")),
                PagePadding::Compact => Some(class!("pt-6 pb-16")),
            },
            attrs.remove("class"),
        )) (attrs)>
            (child)
        </main>
    })
}
