use topcoat::{
    Result,
    view::{View, component, view},
};

use super::motion::ident;

/// A zone's title, laid over its hairline at `place`, on the page's colour so the hairline
/// and any line crossing pass behind it: just the name, kept short so lines rarely do.
#[component]
pub async fn zone_title(id: &str, name: &str, place: String) -> Result<impl View> {
    Ok(view! {
        <div class="absolute flex items-center" style=(place)>
            <span data-zone-part=(id) class=(format!("z-{} bg-background pr-[calc(10*var(--u))] text-[length:calc(11*var(--u))] whitespace-nowrap text-muted-foreground", ident(id)))>
                <span class="font-bold tracking-widest uppercase">(name)</span>
            </span>
        </div>
    })
}
