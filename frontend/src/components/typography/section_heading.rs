use topcoat::{
    Result,
    view::{Attributes, Child, View, class, component, view},
};

/// A section's `<h2>`.
#[component]
pub async fn section_heading(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <h2 class=(class!("text-3xl font-bold tracking-tight text-foreground", attrs.remove("class"))) (attrs)>
            (child)
        </h2>
    })
}
