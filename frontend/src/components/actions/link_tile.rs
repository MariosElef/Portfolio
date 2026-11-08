use topcoat::{
    Result,
    icon::{IconData, icon, iconify::iconify_icon},
    view::{View, attributes, component, view},
};

/// A link to another site as a tile: its icon in a circle, its name, where it goes, and
/// an arrow that lifts on hover. Opens in a new tab.
#[component]
pub async fn link_tile(
    href: &str,
    icon_data: IconData,
    title: &str,
    detail: &str,
) -> Result<impl View> {
    Ok(view! {
        <a
            href=(href)
            target="_blank"
            rel="noopener noreferrer"
            class="group flex items-center gap-4 rounded-xl border border-white/10 bg-card px-4 py-3 transition-colors hover:border-white/25 hover:bg-white/[0.04] focus-visible:ring-3 focus-visible:ring-white/40 focus-visible:outline-none"
        >
            <span class="grid size-10 shrink-0 place-items-center rounded-full bg-white/5 transition-colors group-hover:bg-white/10">
                icon(data: icon_data, attrs: attributes! { class="size-5" })
            </span>
            <span class="flex min-w-0 flex-1 flex-col">
                <span class="font-bold">(title)</span>
                <span class="truncate text-sm text-muted-foreground">(detail)</span>
            </span>
            icon(
                data: iconify_icon!("lucide:arrow-up-right"),
                attrs: attributes! { class="size-4 shrink-0 text-muted-foreground transition-[color,translate] group-hover:-translate-y-0.5 group-hover:translate-x-0.5 group-hover:text-white" }
            )
        </a>
    })
}
