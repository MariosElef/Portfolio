use topcoat::{
    Result,
    view::{View, component, view},
};

/// Short labels in a row of small pills, such as the tools a project is built with.
#[component]
pub async fn tag_list(tags: Vec<&str>, #[default] label: &str) -> Result<impl View> {
    Ok(view! {
        if !tags.is_empty() {
            <ul aria-label=((!label.is_empty()).then_some(label)) class="flex flex-wrap gap-2">
                for tag in tags {
                    <li class="rounded-full bg-muted px-2.5 py-1 text-xs text-muted-foreground">(tag)</li>
                }
            </ul>
        }
    })
}
