use topcoat::{
    Result,
    view::{Child, StaticClass, View, class, component, view},
};

/// How many columns a [`card_grid`] grows to.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum GridColumns {
    /// Two, then three from large screens: project cards.
    #[default]
    Three,
    /// Two, then four on the widest screens: smaller panels.
    Four,
}

impl GridColumns {
    fn classes(self) -> StaticClass {
        match self {
            Self::Three => class!("sm:grid-cols-2 lg:grid-cols-3"),
            Self::Four => class!("sm:grid-cols-2 xl:grid-cols-4"),
        }
    }
}

/// Cards or panels in even columns, one column on phones.
#[component]
pub async fn card_grid(
    #[default] columns: GridColumns,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div class=(class!("grid gap-3", columns.classes()))>(child)</div>
    })
}
