use topcoat::{
    Result,
    view::{Attributes, Child, StaticClass, View, class, component, view},
};

/// The hairline above a row. For a row that is not an `<li>` (a `<div>` in a `<dl>`, or a
/// `<details>`), put it on that element instead.
pub const RULED: StaticClass = class!("border-t border-border");

/// The hairline that closes a list after its last row, for a list that stands on its own
/// rather than inside a panel.
pub const CLOSED: StaticClass = class!("last:border-b");

/// How much room a [`ruled_item`] has above and below.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum RowSpacing {
    /// A dense list inside a panel.
    Snug,
    #[default]
    Normal,
    /// A list that is the content itself.
    Roomy,
}

impl RowSpacing {
    fn classes(self) -> StaticClass {
        match self {
            Self::Snug => class!("py-2.5"),
            Self::Normal => class!("py-3"),
            Self::Roomy => class!("py-4"),
        }
    }
}

/// A list of [`ruled_item`]s. `ordered` numbers it; `narrow` keeps it to a reading width.
#[component]
pub async fn ruled_list(
    #[default] ordered: bool,
    #[default] narrow: bool,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    let classes = class!("flex flex-col", "max-w-3xl" if narrow);
    Ok(view! {
        if ordered {
            <ol class=(classes)>(child)</ol>
        } else {
            <ul class=(classes)>(child)</ul>
        }
    })
}

/// A row of a list, under a hairline. With `closed`, the list's last row has one below
/// it too.
#[component]
pub async fn ruled_item(
    #[default] spacing: RowSpacing,
    #[default] closed: bool,
    /// Smaller text, for a dense list such as notes under a heading.
    #[default]
    small: bool,
    #[default] mut attrs: Attributes,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    let close = closed.then_some(CLOSED);
    let text = small.then_some(class!("text-sm leading-relaxed"));
    Ok(view! {
        <li class=(class!(RULED, spacing.classes(), close, text, attrs.remove("class"))) (attrs)>(child)</li>
    })
}
