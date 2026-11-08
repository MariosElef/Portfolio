use topcoat::{
    Result,
    view::{Attributes, Child, StaticClass, View, class, component, view},
};

/// The site's content width and side gutters: what every page and section lines up to.
pub const CONTAINER: StaticClass = class!("mx-auto w-full max-w-[96rem] px-4 sm:px-6");

/// A block at the site's content width, with its side gutters.
#[component]
pub async fn container(
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div class=(class!(CONTAINER, attrs.remove("class"))) (attrs)>(child)</div>
    })
}
