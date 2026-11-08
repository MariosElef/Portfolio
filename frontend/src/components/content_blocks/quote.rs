use topcoat::{
    Result,
    view::{View, component, view},
};

/// A quotation, set off by a bar, with who said it under it.
#[component]
pub async fn quote_block(text: &str, author: &str) -> Result<impl View> {
    Ok(view! {
        <blockquote class="max-w-3xl border-l-4 border-primary pl-4">
            <p class="text-lg italic">(text)</p>
            if !author.is_empty() {
                <footer class="mt-2 text-sm text-muted-foreground">(author)</footer>
            }
        </blockquote>
    })
}
