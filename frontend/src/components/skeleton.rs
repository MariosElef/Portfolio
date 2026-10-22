use topcoat::{
    Result,
    view::{Attributes, Child, View, class, component, view},
};

/// A placeholder block that holds the place of content still loading.
#[component]
pub async fn skeleton(
    #[default(true)] loading: bool,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        if loading {
            <div
                data-slot="skeleton"
                aria-hidden="true"
                class=(class!("rounded-md bg-accent motion-safe:animate-pulse", attrs.remove("class")))
                (attrs)
            ></div>
        } else {
            (child)
        }
    })
}
