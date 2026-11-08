use topcoat::{
    Result,
    view::{Attributes, Child, StaticClass, View, class, component, view},
};

use super::container::CONTAINER;
use crate::components::typography::section_heading::section_heading;

/// The space around a [`section`].
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum SectionSpacing {
    /// Below only: one of a page's sections, stacked.
    #[default]
    Below,
    /// Above and below: a section on its own.
    Around,
}

impl SectionSpacing {
    fn classes(self) -> StaticClass {
        match self {
            Self::Below => class!("pb-24"),
            Self::Around => class!("py-20"),
        }
    }
}

/// A titled section of a page at the site's width: its heading, with an optional action
/// beside it (such as a link to more), then its content.
#[component]
pub async fn section(
    title: &str,
    #[default] action: Child<'_>,
    #[default] spacing: SectionSpacing,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <section class=(class!(CONTAINER, "flex flex-col gap-8", spacing.classes(), attrs.remove("class"))) (attrs)>
            <div class="flex flex-wrap items-end justify-between gap-4">
                section_heading((title))
                (action)
            </div>
            (child)
        </section>
    })
}
