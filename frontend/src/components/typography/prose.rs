use topcoat::{
    Result,
    view::{Child, View, class, component, view},
};

/// Running text at reading size. `soft` sets it in a softer white, under a [`lead`], so the
/// words its `**strong**` marks stand out.
///
/// [`lead`]: super::lead::lead
#[component]
pub async fn prose(#[default] soft: bool, #[default] child: Child<'_>) -> Result<impl View> {
    Ok(view! {
        <div class=(class!("text-lg", "text-foreground/75" if soft))>(child)</div>
    })
}
