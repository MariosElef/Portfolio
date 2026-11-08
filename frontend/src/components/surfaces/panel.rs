use topcoat::{
    Result,
    view::{Attributes, Child, View, class, component, view},
};

/// A quiet card-coloured surface, with no border, grouping what is inside it.
#[component]
pub async fn panel(
    /// More room inside on wide screens, for something large such as a graph.
    #[default]
    roomy: bool,
    /// A hairline border, a faint sheen and a soft shadow, to lift it off the page.
    #[default]
    outlined: bool,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div class=(class!("rounded-xl bg-card p-6", "sm:p-8" if roomy, "border border-white/10 bg-linear-to-b from-white/[0.04] to-transparent shadow-2xl shadow-black/50" if outlined, attrs.remove("class"))) (attrs)>(child)</div>
    })
}
