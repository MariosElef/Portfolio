use topcoat::{
    Result,
    view::{Child, View, component, view},
};

/// Fields side by side on wider screens, such as a name and an email address.
#[component]
pub async fn field_row(#[default] child: Child<'_>) -> Result<impl View> {
    Ok(view! {
        <div class="grid gap-5 sm:grid-cols-2">(child)</div>
    })
}
