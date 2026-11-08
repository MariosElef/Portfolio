use topcoat::{
    Result,
    view::{Attributes, View, class, component, view},
};

/// A block holding the place of something still loading: grey, gently pulsing (still,
/// for visitors who prefer less motion), and hidden from screen readers.
#[component]
pub async fn skeleton(#[default] mut attrs: Attributes) -> Result<impl View> {
    Ok(view! {
        <div
            aria-hidden="true"
            class=(class!("rounded-lg bg-white/[0.06] motion-safe:animate-pulse", attrs.remove("class")))
            (attrs)
        ></div>
    })
}
