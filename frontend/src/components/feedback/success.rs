use topcoat::{
    Result,
    icon::{icon, iconify::iconify_icon},
    view::{Child, View, attributes, component, view},
};

/// Something done, such as a message sent: a check, a heading and a line under it.
#[component]
pub async fn success(title: &str, #[default] child: Child<'_>) -> Result<impl View> {
    Ok(view! {
        <div role="status" class="flex flex-col items-start gap-4 py-6">
            <span class="grid size-12 place-items-center rounded-full bg-success/15 text-success">
                icon(data: iconify_icon!("lucide:check"), attrs: attributes! { class="size-6" })
            </span>
            <p class="text-2xl font-bold tracking-tight">(title)</p>
            <p class="max-w-md text-muted-foreground">(child)</p>
        </div>
    })
}
