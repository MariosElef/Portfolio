use topcoat::{
    Result,
    icon::icon,
    view::{StaticClass, View, attributes, class, component, view},
};

use super::logos::logo;

/// How large a [`tech_icon`] is.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum TechIconSize {
    /// The size of the text around it.
    #[default]
    Inline,
    /// On its own, as in a grid of tools.
    Large,
}

impl TechIconSize {
    fn classes(self) -> StaticClass {
        match self {
            Self::Inline => class!("size-[0.95em]"),
            Self::Large => class!("size-6"),
        }
    }
}

/// A tool's logo in its brand colour, by its key in [`logos`](super::logos). With a
/// `label`, screen readers name it; without, it is decorative beside its name.
#[component]
pub async fn tech_icon(
    key: &str,
    #[default] label: &str,
    #[default] size: TechIconSize,
) -> Result<impl View> {
    let (data, color) = logo(key);
    let style = color.map(|c| format!("color: {c}"));
    Ok(view! {
        <span style=(style) class="inline-flex self-center">
            icon(data: data, label: label, attrs: attributes! { class=(size.classes()) })
        </span>
    })
}
