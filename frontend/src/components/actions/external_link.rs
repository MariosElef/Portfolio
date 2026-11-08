use topcoat::{
    Result,
    view::{AttributeValueViewParts, Attributes, Child, View, component, view},
};

/// A link to another site, opened in a new tab that cannot reach back into this one.
#[component]
pub async fn external_link<H>(
    href: H,
    #[default] attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View>
where
    H: AttributeValueViewParts + Send,
{
    Ok(view! {
        <a href=(href) target="_blank" rel="noopener noreferrer" (attrs)>(child)</a>
    })
}
