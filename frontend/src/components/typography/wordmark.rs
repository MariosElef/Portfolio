use topcoat::{
    Result,
    view::{View, component, view},
};

/// The site's name set as a wordmark: Inter, bold and tight, with no logo.
#[component]
pub async fn wordmark(name: &str) -> Result<impl View> {
    Ok(view! {
        <span class="font-display text-xl font-bold tracking-[-0.03em] whitespace-nowrap sm:text-2xl">(name)</span>
    })
}
