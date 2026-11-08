use topcoat::{
    Result,
    view::{Child, View, attributes, component, view},
};

use super::panel::panel;

/// A headline number with what it counts, on a panel. Place several in a `<dl>`.
#[component]
pub async fn stat(label: &str, value: &str) -> Result<impl View> {
    Ok(view! {
        panel(
            attrs: attributes! { class="flex flex-col justify-between gap-8" },
            <dt class="text-sm text-muted-foreground">(label)</dt>
            <dd class="text-3xl font-bold tracking-tight sm:text-4xl">(value)</dd>
        )
    })
}

/// [`stat`]s side by side: two to a row, four on large screens.
#[component]
pub async fn stat_grid(#[default] child: Child<'_>) -> Result<impl View> {
    Ok(view! {
        <dl class="grid grid-cols-2 gap-3 lg:grid-cols-4">(child)</dl>
    })
}
