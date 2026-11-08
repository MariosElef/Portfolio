use topcoat::{
    Result,
    view::{Child, View, component, view},
};

/// A row of buttons, each at its own width, such as a form's submit or a link to a
/// project's source.
#[component]
pub async fn button_row(#[default] child: Child<'_>) -> Result<impl View> {
    Ok(view! {
        <div class="flex flex-wrap items-center gap-3">(child)</div>
    })
}
