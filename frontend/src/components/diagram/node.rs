use topcoat::{
    Result,
    icon::{icon, iconify::iconify_icon},
    view::{View, ViewExt, attributes, class, component, view},
};

use super::{glyphs::glyph, motion::ident};
use crate::types::content::DiagramNode;

/// A card's raised look: a faint sheen over the card colour, and a shadow. The border's
/// strength is added per part.
const RAISED: &str =
    "bg-linear-to-b from-white/[0.07] to-white/[0.02] bg-card shadow-lg shadow-black/30";

/// A part's name, at the diagram's scale.
#[component]
async fn node_name(name: &str) -> Result<impl View> {
    Ok(view! {
        <p class="text-[length:calc(13*var(--u))] leading-tight font-bold">(name)</p>
    }
    .boxed())
}

/// A part's line of detail, if it has one, at the diagram's scale.
#[component]
async fn node_detail(detail: &str, #[default] gap: bool) -> Result<impl View> {
    let gap = gap.then_some("mt-[calc(2*var(--u))]");
    Ok(view! {
        if !detail.is_empty() {
            <p class=(class!(gap, "text-[length:calc(11*var(--u))] leading-snug text-muted-foreground"))>(detail)</p>
        }
    }
    .boxed())
}

/// A part's name and its line of detail, at the diagram's scale. Each view here is boxed:
/// unboxed, the diagram nests deep enough to overflow the compiler's trait solver.
#[component]
async fn node_text(name: &str, detail: &str, #[default] centred: bool) -> Result<impl View> {
    Ok(view! {
        node_name(name: name)
        node_detail(detail: detail, gap: centred)
    }
    .boxed())
}

/// A part as a card at `place`: its icon in `icon_class`, name and detail. Plain parts
/// have no card; emphasised ones a brighter edge; dashed ones belong to someone else.
#[component]
pub async fn card_node(node: DiagramNode, place: String, icon_class: &str) -> Result<impl View> {
    let tone = match (node.plain, node.emphasis, node.dashed) {
        (true, _, _) => "border-transparent".to_owned(),
        (_, true, _) => format!("border-white/40 {RAISED}"),
        (_, _, true) => "border-dashed border-white/20 bg-white/[0.02]".to_owned(),
        _ => format!("border-white/10 {RAISED}"),
    };
    Ok(view! {
        <div
            data-node=(node.id.clone())
            style=(place)
            class=(format!("n-{} absolute flex flex-col gap-[calc(6*var(--u))] overflow-hidden rounded-[calc(12*var(--u))] border p-[calc(12*var(--u))] {tone}", ident(&node.id)))
        >
            <div class="flex items-center gap-[calc(8*var(--u))]">
                if let Some(data) = glyph(&node.icon) {
                    icon(data: data, attrs: attributes! { class=(format!("size-[calc(16*var(--u))] shrink-0 {icon_class}")) })
                }
                node_name(name: &node.name)
            </div>
            node_detail(detail: &node.detail)
        </div>
    }
    .boxed())
}

/// A pipeline step at `place`: a circle with a check, named underneath. A run through
/// the pipeline (`motion`) shows its check as a run passes it, or a cross when one fails
/// there.
#[component]
pub async fn step_node(node: DiagramNode, place: String) -> Result<impl View> {
    Ok(view! {
        <div
            data-node=(node.id.clone())
            data-step=""
            style=(place)
            class=(format!("n-{} absolute flex items-center justify-center rounded-full border border-white/15 bg-card shadow-lg shadow-black/30", ident(&node.id)))
        >
            icon(data: iconify_icon!("lucide:check"), attrs: attributes! { class="step-pass size-[calc(24*var(--u))] text-emerald-300" })
            icon(data: iconify_icon!("lucide:x"), attrs: attributes! { class="step-fail absolute size-[calc(24*var(--u))] text-red-400" })
            <div class="absolute top-full left-1/2 mt-[calc(10*var(--u))] ml-[calc(-90*var(--u))] w-[calc(180*var(--u))] text-center">
                node_text(name: &node.name, detail: &node.detail, centred: true)
            </div>
        </div>
    }
    .boxed())
}
