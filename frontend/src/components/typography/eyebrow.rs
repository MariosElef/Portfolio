use topcoat::{
    Result,
    view::{Child, StaticClass, View, class, component, view},
};

/// Small capitals. For an element no component draws, such as a diagram's caption, put
/// it on that element.
pub const EYEBROW: StaticClass =
    class!("text-xs font-bold tracking-widest text-muted-foreground uppercase");

/// What an [`eyebrow`] is in the page's outline.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum EyebrowLevel {
    /// A label, not a heading.
    #[default]
    Text,
    /// Heads a group inside a section.
    H3,
    /// Heads a group inside an `H3`'s.
    H4,
}

/// A short label in small capitals, as a label or a heading.
#[component]
pub async fn eyebrow(
    #[default] level: EyebrowLevel,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        match level {
            EyebrowLevel::Text => <p class=(EYEBROW)>(child)</p>,
            EyebrowLevel::H3 => <h3 class=(EYEBROW)>(child)</h3>,
            EyebrowLevel::H4 => <h4 class=(EYEBROW)>(child)</h4>,
        }
    })
}
