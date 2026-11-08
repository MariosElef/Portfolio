use topcoat::{
    Result,
    view::{Child, View, component, view},
};

/// The page's content rising over a full-screen hero on a rounded sheet.
#[component]
pub async fn sheet(#[default] child: Child<'_>) -> Result<impl View> {
    Ok(view! {
        <main class="relative -mt-6 rounded-t-3xl bg-background">(child)</main>
    })
}
