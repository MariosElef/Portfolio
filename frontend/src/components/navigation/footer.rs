use topcoat::{
    Result,
    view::{Child, View, attributes, component, view},
};

use crate::components::layout::container::container;

/// The site's footer: its columns, as Adobe lays them out (two across on a phone, four on
/// wide screens), then a slim row with `note` (the copyright) and a link back to the top
/// labelled `top`.
#[component]
pub async fn footer(note: &str, top: &str, #[default] child: Child<'_>) -> Result<impl View> {
    Ok(view! {
        <footer class="mt-16 border-t border-border sm:mt-24">
            container(
                attrs: attributes! { class="grid grid-cols-2 gap-x-6 gap-y-10 py-12 sm:py-16 lg:grid-cols-4" },
                (child)
            )
            <div class="border-t border-border">
                container(
                    attrs: attributes! { class="flex flex-wrap items-center justify-between gap-4 py-6 text-xs text-muted-foreground" },
                    <p>(note)</p>
                    <a href="#" class="hover:text-foreground">(top)</a>
                )
            </div>
        </footer>
    })
}
