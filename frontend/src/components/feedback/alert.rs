use topcoat::{
    Result,
    view::{Child, View, class, component, view},
};

use crate::components::tone::Color;

/// A short message in a tinted box, such as a form's result. Errors and warnings are
/// announced at once; anything else politely.
#[component]
pub async fn alert(#[default] color: Color, #[default] child: Child<'_>) -> Result<impl View> {
    let role = match color {
        Color::Error | Color::Warning => "alert",
        _ => "status",
    };
    Ok(view! {
        <p role=(role) class=(class!("rounded-lg bg-(--tone)/15 px-4 py-3 text-(--tone-emphasis)", color.tone()))>
            (child)
        </p>
    })
}
