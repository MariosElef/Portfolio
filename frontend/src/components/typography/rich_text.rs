use topcoat::{
    Result,
    view::{View, ViewExt, component, view},
};

use crate::components::tech::mention::tech_mention;
use crate::utils::text::{Segment, segments};

/// Text that may mention tools as `[[icon:Label]]`, each showing its logo and never
/// wrapping away from the brackets or punctuation touching it, and make a few words
/// stand out as `**strong**`, bolder and brighter than the text around them.
#[component]
pub async fn rich_text(text: &str) -> Result<impl View> {
    // Boxed: it nests inside other deep views (lists inside the timeline), and an
    // unboxed type that deep overflows the compiler's trait solver.
    Ok(view! {
        for segment in segments(text) {
            match segment {
                Segment::Text(text) => (text),
                Segment::Strong(text) => <strong class="font-semibold text-foreground">(text)</strong>,
                Segment::Tech { icon, label, before, after } => {
                    // Kept on one line with the punctuation touching it.
                    <span class="whitespace-nowrap">(before) tech_mention(icon_name: &icon, label: &label) (after)</span>
                },
            }
        }
    }
    .boxed())
}
