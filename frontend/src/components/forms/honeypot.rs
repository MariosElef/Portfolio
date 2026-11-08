use topcoat::{
    Result,
    view::{View, component, view},
};

/// A field people never see, so only a bot fills it in; the form's handler ignores any
/// submission that does.
#[component]
pub async fn honeypot(name: &str) -> Result<impl View> {
    Ok(view! {
        <div class="hidden" aria-hidden="true">
            <input name=(name) tabindex="-1" autocomplete="off">
        </div>
    })
}
