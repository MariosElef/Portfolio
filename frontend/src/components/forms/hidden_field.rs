use topcoat::{
    Result,
    view::{View, component, view},
};

/// A value the form sends back unseen, such as the token of the question it asked.
#[component]
pub async fn hidden_field(name: &str, value: &str) -> Result<impl View> {
    Ok(view! {
        <input type="hidden" name=(name) value=(value)>
    })
}
