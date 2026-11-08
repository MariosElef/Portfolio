use topcoat::{
    Result,
    view::{StaticClass, View, class, component, view},
};

/// How large a logo or mark is drawn.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum MarkSize {
    /// Beside a role, or on a featured card.
    Sm,
    /// On a project card.
    #[default]
    Md,
    /// Leading a project's page.
    Lg,
}

impl MarkSize {
    pub(super) fn logo(self) -> StaticClass {
        match self {
            Self::Sm => class!("h-10 max-w-[7rem] rounded-lg"),
            Self::Md => class!("h-12 max-w-[8rem] rounded-xl"),
            Self::Lg => class!("h-20 max-w-[16rem] rounded-2xl sm:h-24"),
        }
    }
}

/// A brand's logo at its natural width, so a wide mark stays readable; its corners are
/// rounded, which only shows on a logo with a background of its own. Decorative: the
/// name it stands for is always written beside it.
#[component]
pub async fn brand_logo(src: &str, #[default] size: MarkSize) -> Result<impl View> {
    Ok(view! {
        <img src=(src) alt="" class=(class!("w-auto shrink-0 object-contain", size.logo()))>
    })
}
