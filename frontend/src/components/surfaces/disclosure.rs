use topcoat::{
    Result,
    icon::{icon, iconify::iconify_icon},
    view::{Child, View, attributes, component, view},
};

/// Content that opens in place behind a short summary, with a chevron that turns as it
/// opens. The browser's own disclosure, so no script is needed.
#[component]
pub async fn disclosure(summary: &str, #[default] child: Child<'_>) -> Result<impl View> {
    Ok(view! {
        <details class="group">
            <summary class="inline-flex cursor-pointer list-none items-center gap-1 text-sm font-bold text-foreground hover:text-primary-emphasis [&::-webkit-details-marker]:hidden">
                icon(data: iconify_icon!("lucide:chevron-right"), attrs: attributes! { class="size-4 transition-transform group-open:rotate-90" })
                (summary)
            </summary>
            (child)
        </details>
    })
}
