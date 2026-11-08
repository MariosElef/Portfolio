use topcoat::{
    Result,
    view::{Child, View, component, view},
};

/// The paragraph under a page's title that says what the page is: larger and a little
/// heavier than the text after it.
#[component]
pub async fn lead(#[default] child: Child<'_>) -> Result<impl View> {
    Ok(view! {
        <p class="max-w-3xl text-xl leading-snug font-medium text-foreground/90 sm:text-2xl">(child)</p>
    })
}
