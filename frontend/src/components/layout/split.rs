use topcoat::{
    Result,
    view::{Child, View, component, view},
};

/// Two columns on wide screens: `aside` on the left, staying in view while the main part
/// on the right scrolls; one above the other on a phone.
#[component]
pub async fn split(#[default] aside: Child<'_>, #[default] child: Child<'_>) -> Result<impl View> {
    Ok(view! {
        <div class="grid items-start gap-12 lg:grid-cols-[minmax(0,0.9fr)_minmax(0,1.1fr)] lg:gap-20">
            <div class="flex flex-col gap-8 lg:sticky lg:top-32">(aside)</div>
            <div class="min-w-0">(child)</div>
        </div>
    })
}
