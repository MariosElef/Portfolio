use topcoat::{
    Result,
    view::{Child, View, class, component, view},
};

/// Every page's frame: the bar pinned to the top, the page, and the footer. With
/// `offset`, the page starts below the bar; without, it runs beneath it, as the home
/// page's hero does.
#[component]
pub async fn frame(
    bar: Child<'_>,
    footer: Child<'_>,
    #[default] offset: bool,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div class="fixed inset-x-0 top-0 z-50">(bar)</div>
        <div class=(class!("min-w-0 flex-1", "pt-24" if offset))>(child)</div>
        (footer)
    })
}
