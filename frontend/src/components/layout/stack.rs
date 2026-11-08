use topcoat::{
    Result,
    view::{Child, StaticClass, View, class, component, view},
};

/// The space between the parts of a [`stack`].
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum StackGap {
    /// A label and what it labels.
    Snug,
    /// Parts that belong together, such as panels in one group.
    Tight,
    #[default]
    Normal,
}

impl StackGap {
    fn classes(self) -> StaticClass {
        match self {
            Self::Snug => class!("gap-2"),
            Self::Tight => class!("gap-3"),
            Self::Normal => class!("gap-4"),
        }
    }
}

/// Parts one above another.
#[component]
pub async fn stack(#[default] gap: StackGap, #[default] child: Child<'_>) -> Result<impl View> {
    Ok(view! {
        <div class=(class!("flex flex-col", gap.classes()))>(child)</div>
    })
}
