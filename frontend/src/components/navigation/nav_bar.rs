use topcoat::{
    Result,
    view::{Child, View, attributes, component, view},
};

use crate::components::layout::container::container;

/// The site's bar: dark glass across the full width of the top of the page, its contents
/// in line with the page's. As Adobe lays it out: the brand and the pages (named `label`)
/// on the left, the one call to action on the right.
#[component]
pub async fn nav_bar(
    label: &str,
    brand: Child<'_>,
    action: Child<'_>,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <header class="w-full border-b border-white/10 bg-ink-950/60 text-white shadow-lg shadow-black/20 backdrop-blur-xl">
            container(
                attrs: attributes! { class="flex h-16 items-center justify-between gap-4 sm:h-[4.5rem]" },
                <div class="flex min-w-0 items-center gap-2 sm:gap-8">
                    (brand)
                    <nav aria-label=(label) class="flex min-w-0 items-center">(child)</nav>
                </div>
                (action)
            )
        </header>
    })
}
