//! A graph drawn to scale: zones, each a titled hairline rather than a box, the parts
//! inside them, and lines between them coloured by what crosses, with a legend under it
//! describing the kinds it uses; the lines carry no text.
//!
//! The zones and lines are SVG; the parts and titles are HTML laid over it at the same
//! scale (HTML inside SVG renders badly in Safari once it animates). One unit, `--u`, is a
//! diagram unit in CSS, so text scales with the drawing. On narrow screens it scrolls
//! sideways rather than shrinking past reading.
//!
//! Its motion, the build-up and then the light of requests moving through it (or a
//! pipeline's runs), is worked out in Rust as it renders (`motion`) and played as CSS.

use topcoat::{
    Result,
    view::{View, component, view},
};

use crate::{components::typography::eyebrow::EYEBROW, types::content::Diagram};

mod edges;
mod glyphs;
pub mod kinds;
pub mod layout;
mod legend;
mod motion;
mod node;
mod zone;

use edges::{current, edge};
use kinds::style;
use layout::{Bounds, routes};
use node::{card_node, step_node};
use zone::zone_title;

#[component]
pub async fn diagram(graph: Diagram) -> Result<impl View> {
    let bounds = Bounds::of(&graph);
    let routes = routes(&graph);
    let used: Vec<String> = routes.iter().map(|route| route.kind.clone()).collect();
    let canvas = format!(
        "aspect-ratio: {} / {}; --u: calc(100cqw / {})",
        bounds.w, bounds.h, bounds.w
    );
    // Ids must be unique on the page, which may hold several diagrams.
    let root = format!("d-{}", motion::ident(&graph.title));
    let glow_id = format!("{root}-glow");
    let motion = motion::css(&root, &graph, &routes);
    let pipeline = graph.nodes.iter().all(|node| node.step).then_some("");
    let glow = format!("url(#{glow_id})");
    // Each part with where it sits, and (for a plain part) its icon's colour:
    // the colour of the line leading into it.
    let parts: Vec<_> = graph
        .nodes
        .iter()
        .map(|node| {
            let into = graph.edges.iter().find(|line| line.to == node.id);
            let icon_class = match into {
                Some(line) if node.plain => style(&line.kind).text,
                _ => "text-muted-foreground",
            };
            let place = bounds.place(node.x as f32, node.y as f32, node.w as f32, node.h as f32);
            (node.clone(), place, icon_class)
        })
        .collect();

    Ok(view! {
        <figure class="flex flex-col gap-5">
            <style>(motion)</style>
            if !graph.title.is_empty() {
                <figcaption class=(EYEBROW)>(graph.title.clone())</figcaption>
            }
            <div class="overflow-x-auto">
                <div id=(root) data-diagram="" data-pipeline=(pipeline) style=(canvas) class="relative min-w-[56rem] [container-type:inline-size]">
                    <svg xmlns="http://www.w3.org/2000/svg" viewBox=(bounds.view_box()) aria-hidden="true" class="absolute inset-0 size-full overflow-visible">
                        // A soft blur, for the light the current gives off. In user space, as
                        // a straight line's own box has no height to blur into.
                        <defs>
                            <filter id=(glow_id) filterUnits="userSpaceOnUse" x=(bounds.x) y=(bounds.y) width=(bounds.w) height=(bounds.h)>
                                <feGaussianBlur stdDeviation="3"></feGaussianBlur>
                            </filter>
                        </defs>
                        // A zone is a hairline across its top, behind its title.
                        for zone in &graph.zones {
                            <line data-zone-part=(zone.id.clone()) class=(format!("z-{} stroke-white/10", motion::ident(&zone.id))) x1=(zone.x) y1=(zone.y + 12) x2=(zone.x + zone.w) y2=(zone.y + 12) stroke-width="1"></line>
                        }
                        for (index, route) in routes.iter().enumerate() {
                            edge(index: index, route: route)
                        }
                        for (index, route) in routes.iter().enumerate() {
                            current(index: index, route: route, glow: &glow)
                        }
                    </svg>
                    for zone in graph.zones.iter() {
                        zone_title(id: &zone.id, name: &zone.name, place: bounds.place(zone.x as f32, zone.y as f32, zone.w as f32, 24.0))
                    }
                    for (node, place, icon_class) in parts {
                        if node.step {
                            step_node(node: node, place: place)
                        } else {
                            card_node(node: node, place: place, icon_class: icon_class)
                        }
                    }
                </div>
            </div>
            legend::legend(used: used, explained: &graph.legend)
        </figure>
    })
}
