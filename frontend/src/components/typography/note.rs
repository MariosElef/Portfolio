use topcoat::{
    Result,
    view::{Child, View, component, view},
};

/// A quiet line of small print, such as how a form's details are used.
#[component]
pub async fn note(#[default] child: Child<'_>) -> Result<impl View> {
    Ok(view! {
        <p class="text-sm text-muted-foreground">(child)</p>
    })
}
