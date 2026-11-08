use topcoat::{
    Result,
    view::{View, component, view},
};

/// An image at the full width, with its caption under it; the caption is also its text
/// for screen readers.
#[component]
pub async fn figure_block(src: &str, caption: &str) -> Result<impl View> {
    Ok(view! {
        <figure data-reveal="" class="flex flex-col gap-2">
            <img src=(src) alt=(caption) loading="lazy" class="w-full rounded-xl border border-border">
            if !caption.is_empty() {
                <figcaption class="text-sm text-muted-foreground">(caption)</figcaption>
            }
        </figure>
    })
}
