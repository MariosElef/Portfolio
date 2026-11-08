use topcoat::{
    Result,
    view::{View, component, view},
};

/// Code, such as commands, in a box that scrolls sideways rather than wrapping.
#[component]
pub async fn code_block(lang: &str, code: &str) -> Result<impl View> {
    Ok(view! {
        <pre class="overflow-x-auto rounded-xl border border-border bg-card p-4 text-sm"><code data-lang=(lang)>(code)</code></pre>
    })
}
