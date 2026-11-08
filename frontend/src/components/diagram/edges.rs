use topcoat::{
    Result,
    view::{View, component, view},
};

use super::{kinds::style, layout::Route};

/// How fast the current travels, in diagram units a second.
const CURRENT_SPEED: f32 = 520.0;

/// The least time the current takes along a line, so a short one still shows it.
const CURRENT_MIN_MS: f32 = 300.0;

/// How long the streak of light is, in diagram units, at most this share of its line.
const STREAK: f32 = 48.0;
const STREAK_SHARE: f32 = 0.4;

/// How long the current takes along a line: the same speed on every line, though never
/// too fast to see.
pub fn travel_ms(route: &Route) -> u32 {
    (route.length / CURRENT_SPEED * 1000.0).max(CURRENT_MIN_MS) as u32
}

/// The streak's length as a share of its line, whose length is drawn as 1.
fn streak(route: &Route) -> f32 {
    (STREAK / route.length).min(STREAK_SHARE)
}

/// A line between two parts. It draws out from its source while the diagram builds up,
/// then stays, in its kind's colour; `motion` times both by its class, `e<index>`.
#[component]
pub async fn edge(index: usize, route: &Route) -> Result<impl View> {
    let stroke = style(&route.kind).stroke;
    Ok(view! {
        <g data-edge=(index) class=(format!("e{index}"))>
            <path d=(route.path.as_str()) pathLength="1" fill="none" stroke-width="2.25" stroke-linecap="round" class=(format!("edge-draw {stroke}"))></path>
            <g class="edge-line">
                <path d=(route.path.as_str()) fill="none" stroke-width="2.25" stroke-linecap="round" class=(stroke)></path>
            </g>
        </g>
    })
}

/// A line's light, out and, when it goes both ways, back: a white core in a soft glow of
/// the line's colour, waiting at the start until `motion` sends it along. Drawn above
/// every line, so it stays in sight along a stretch lines share and around their turns.
#[component]
pub async fn current(index: usize, route: &Route, glow: &str) -> Result<impl View> {
    let stroke = style(&route.kind).stroke;
    let length = format!("--streak: {:.4}", streak(route));
    let directions: &[&str] = if route.two_way {
        &["out", "back"]
    } else {
        &["out"]
    };
    Ok(view! {
        <g class=(format!("c{index}"))>
            for direction in directions {
                <path d=(route.path.as_str()) pathLength="1" fill="none" stroke-width="8" filter=(glow) style=(length.clone()) class=(format!("edge-current glow current-{direction} {stroke}"))></path>
                <path d=(route.path.as_str()) pathLength="1" fill="none" stroke-width="1.25" style=(length.clone()) class=(format!("edge-current current-{direction} stroke-white"))></path>
            }
        </g>
    })
}
