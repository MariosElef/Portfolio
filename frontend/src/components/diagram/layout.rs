use std::collections::HashMap;

use crate::types::content::{Diagram, DiagramNode};

/// How far an arc between two parts of one row rises above them.
const ARC_LIFT: f32 = 44.0;
/// Room around the drawing, so arcs at the edges are not cut off.
const MARGIN: f32 = 24.0;

/// An arrow, routed as an SVG path.
#[derive(Debug, PartialEq)]
pub struct Route {
    pub path: String,
    /// The path's length, in diagram units.
    pub length: f32,
    pub kind: String,
    pub two_way: bool,
    /// The ids the arrow leads from and to.
    pub from: String,
    pub to: String,
}

/// The area a diagram covers, with a margin all round so arcs at the edges are not cut
/// off.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Bounds {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

impl Bounds {
    pub fn of(diagram: &Diagram) -> Self {
        let rects = diagram
            .zones
            .iter()
            .map(|z| Rect::new(z.x, z.y, z.w, z.h))
            .chain(diagram.nodes.iter().map(|n| Rect::new(n.x, n.y, n.w, n.h)));
        let (mut left, mut top, mut right, mut bottom) = (f32::MAX, f32::MAX, f32::MIN, f32::MIN);
        for r in rects {
            left = left.min(r.x);
            top = top.min(r.y);
            right = right.max(r.x + r.w);
            bottom = bottom.max(r.y + r.h);
        }
        if left > right {
            return Self {
                x: 0.0,
                y: 0.0,
                w: 0.0,
                h: 0.0,
            };
        }
        Self {
            x: left - MARGIN,
            y: top - MARGIN,
            w: right - left + 2.0 * MARGIN,
            h: bottom - top + 2.0 * MARGIN,
        }
    }

    /// As an SVG `viewBox`.
    pub fn view_box(self) -> String {
        format!("{} {} {} {}", self.x, self.y, self.w, self.h)
    }

    /// CSS placing a box at diagram coordinates, as percentages of the whole, so HTML laid
    /// over the drawing scales with it.
    pub fn place(self, x: f32, y: f32, w: f32, h: f32) -> String {
        format!(
            "left: {:.3}%; top: {:.3}%; width: {:.3}%; height: {:.3}%",
            (x - self.x) / self.w * 100.0,
            (y - self.y) / self.h * 100.0,
            w / self.w * 100.0,
            h / self.h * 100.0
        )
    }
}

/// The smallest zone wholly containing `node`, or "" when none does.
pub fn zone_of<'a>(diagram: &'a Diagram, node: &DiagramNode) -> &'a str {
    diagram
        .zones
        .iter()
        .filter(|z| {
            z.x <= node.x
                && z.y <= node.y
                && node.x + node.w <= z.x + z.w
                && node.y + node.h <= z.y + z.h
        })
        .min_by_key(|z| z.w * z.h)
        .map_or("", |z| z.id.as_str())
}

/// Routes every edge whose ends exist. Where the two ends overlap, the arrow is one
/// straight line. Otherwise it leaves the side of its source that faces the target: top or bottom between rows, left or right within one, where it runs
/// straight across. Two parts of one row with another part between them are joined by an
/// arc over the top instead. Arrows of one kind on the same side share one point and fork
/// from it; different kinds sit side by side.
pub fn routes(diagram: &Diagram) -> Vec<Route> {
    let rects: HashMap<&str, Rect> = diagram
        .zones
        .iter()
        .map(|z| (z.id.as_str(), Rect::new(z.x, z.y, z.w, z.h)))
        .chain(
            diagram
                .nodes
                .iter()
                .map(|n| (n.id.as_str(), Rect::new(n.x, n.y, n.w, n.h))),
        )
        .collect();
    let nodes: Vec<Rect> = diagram
        .nodes
        .iter()
        .map(|n| Rect::new(n.x, n.y, n.w, n.h))
        .collect();

    let steps: std::collections::HashSet<&str> = diagram
        .nodes
        .iter()
        .filter(|n| n.step)
        .map(|n| n.id.as_str())
        .collect();

    // Each edge's two ends: (element, side), plus the other end's centre to sort by.
    let mut ends = Vec::new();
    for edge in &diagram.edges {
        let (Some(&a), Some(&b)) = (rects.get(edge.from.as_str()), rects.get(edge.to.as_str()))
        else {
            continue;
        };
        // Pipeline steps flow left to right, from side to side, as in a workflow view.
        let flow = steps.contains(edge.from.as_str()) && steps.contains(edge.to.as_str());
        let (from_side, to_side) = match flow {
            true if a.x <= b.x => (Side::Right, Side::Left),
            true => (Side::Left, Side::Right),
            false => sides(a, b, &nodes),
        };
        ends.push((
            edge,
            (edge.from.as_str(), from_side, b),
            (edge.to.as_str(), to_side, a),
        ));
    }

    // The ends on each side of each element, by kind. Lines of one kind meet at one point,
    // like a railway forking; kinds sit side by side, ordered by where they lead.
    let mut shared: HashMap<(&str, Side), Vec<(&str, f32)>> = HashMap::new();
    for (edge, from, to) in &ends {
        for (id, side, other) in [from, to] {
            shared
                .entry((id, *side))
                .or_default()
                .push((edge.kind.as_str(), along(*side, *other)));
        }
    }
    let groups: HashMap<(&str, Side), Vec<&str>> = shared
        .iter()
        .map(|(&key, list)| {
            let mut kinds: Vec<(&str, f32)> = Vec::new();
            for &(kind, _) in list {
                if !kinds.iter().any(|&(k, _)| k == kind) {
                    let all: Vec<f32> = list.iter().filter(|e| e.0 == kind).map(|e| e.1).collect();
                    kinds.push((kind, all.iter().sum::<f32>() / all.len() as f32));
                }
            }
            kinds.sort_by(|a, b| a.1.total_cmp(&b.1));
            (key, kinds.into_iter().map(|(k, _)| k).collect())
        })
        .collect();
    let alone = |id: &str, side: Side| shared[&(id, side)].len() == 1;
    let anchor = |id: &str, side: Side, kind: &str| {
        let rect = rects[id];
        // A step's lines all meet at the middle of its side.
        if steps.contains(id) {
            return rect.anchor(side, 0.5);
        }
        let kinds = &groups[&(id, side)];
        let index = kinds.iter().position(|&k| k == kind).unwrap_or(0);
        // One kind in the middle; several spread from a quarter to three quarters along.
        let t = match kinds.len() {
            1 => 0.5,
            n => 0.25 + 0.5 * index as f32 / (n - 1) as f32,
        };
        rect.anchor(side, t)
    };

    ends.into_iter()
        .map(|(edge, (from_id, from_side, _), (to_id, to_side, _))| {
            let (mut start, mut end) = (
                anchor(from_id, from_side, &edge.kind),
                anchor(to_id, to_side, &edge.kind),
            );
            // One above the other, the arrow drops straight down the middle of what they
            // share, without a turn.
            if matches!(from_side, Side::Top | Side::Bottom)
                && alone(from_id, from_side)
                && alone(to_id, to_side)
                && start.1 != end.1
                && !matches!((from_side, to_side), (Side::Top, Side::Top))
            {
                let (from, to) = (rects[from_id], rects[to_id]);
                let (left, right) = (from.x.max(to.x), (from.x + from.w).min(to.x + to.w));
                if left < right {
                    let x = (left + right) / 2.0;
                    (start.0, end.0) = (x, x);
                }
            }
            // Side by side, the arrow runs straight across the middle of what they share.
            if matches!(from_side, Side::Left | Side::Right)
                && rects[from_id].overlaps_vertically(rects[to_id])
                && !steps.contains(from_id)
            {
                let (from, to) = (rects[from_id], rects[to_id]);
                let y = (from.y.max(to.y) + (from.y + from.h).min(to.y + to.h)) / 2.0;
                (start.1, end.1) = (y, y);
            }
            let points = waypoints(start, end, from_side, track(&edge.kind));
            Route {
                path: polyline(&points),
                length: points
                    .windows(2)
                    .map(|w| (w[1].0 - w[0].0).hypot(w[1].1 - w[0].1))
                    .sum(),
                kind: edge.kind.clone(),
                two_way: edge.two_way,
                from: edge.from.clone(),
                to: edge.to.clone(),
            }
        })
        .collect()
}

/// An SVG path through `points`: straight runs meeting at square corners.
fn polyline(points: &[Point]) -> String {
    points
        .iter()
        .enumerate()
        .map(|(i, p)| format!("{}{:.1} {:.1}", if i == 0 { "M" } else { " L" }, p.0, p.1))
        .collect()
}

type Point = (f32, f32);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Side {
    Top,
    Bottom,
    Left,
    Right,
}

#[derive(Clone, Copy, Debug)]
struct Rect {
    x: f32,
    y: f32,
    w: f32,
    h: f32,
}

impl Rect {
    fn new(x: i32, y: i32, w: i32, h: i32) -> Self {
        Self {
            x: x as f32,
            y: y as f32,
            w: w as f32,
            h: h as f32,
        }
    }

    fn centre(self) -> Point {
        (self.x + self.w / 2.0, self.y + self.h / 2.0)
    }

    /// The point a fraction `t` of the way along `side`.
    fn anchor(self, side: Side, t: f32) -> Point {
        match side {
            Side::Top => (self.x + self.w * t, self.y),
            Side::Bottom => (self.x + self.w * t, self.y + self.h),
            Side::Left => (self.x, self.y + self.h * t),
            Side::Right => (self.x + self.w, self.y + self.h * t),
        }
    }

    fn overlaps_vertically(self, other: Rect) -> bool {
        self.y < other.y + other.h && other.y < self.y + self.h
    }
}

/// The sides an arrow from `a` to `b` leaves and arrives by.
fn sides(a: Rect, b: Rect, nodes: &[Rect]) -> (Side, Side) {
    if a.overlaps_vertically(b) {
        let (left, right) = if a.x <= b.x { (a, b) } else { (b, a) };
        let blocked = nodes.iter().any(|n| {
            n.overlaps_vertically(a)
                && n.overlaps_vertically(b)
                && n.x >= left.x + left.w
                && n.x + n.w <= right.x
        });
        if blocked {
            (Side::Top, Side::Top)
        } else if a.x <= b.x {
            (Side::Right, Side::Left)
        } else {
            (Side::Left, Side::Right)
        }
    } else if b.y >= a.y + a.h {
        (Side::Bottom, Side::Top)
    } else {
        (Side::Top, Side::Bottom)
    }
}

/// Where the other end lies along a side, to order the arrows sharing it.
fn along(side: Side, other: Rect) -> f32 {
    match side {
        Side::Top | Side::Bottom => other.centre().0,
        Side::Left | Side::Right => other.centre().1,
    }
}

/// How far from the middle of a row gap arrows of a kind run, so kinds never overlap.
fn track(kind: &str) -> f32 {
    match kind {
        "identity" => -14.0,
        "agent" => -48.0,
        "state" | "session" => 14.0,
        _ => 0.0,
    }
}

/// The corners an arrow turns at: out of its side, across, and into the other. Arrows
/// between rows run across the row gap's middle, offset by `track`, so arrows fanning
/// out run together and arrows of different kinds run side by side, never on top of
/// each other.
fn waypoints(start: Point, end: Point, from: Side, track: f32) -> Vec<Point> {
    let straight = (start.0 - end.0).abs() < 0.5 || (start.1 - end.1).abs() < 0.5;
    match from {
        Side::Left | Side::Right if (start.1 - end.1).abs() < 0.5 => vec![start, end],
        // Between rows, sideways: across to halfway, then up or down, then on.
        Side::Left | Side::Right => {
            let mid = (start.0 + end.0) / 2.0;
            vec![start, (mid, start.1), (mid, end.1), end]
        }
        Side::Top if end.1 >= start.1 => {
            // Over the row: up, across, down.
            let top = start.1.min(end.1) - ARC_LIFT;
            vec![start, (start.0, top), (end.0, top), end]
        }
        _ if straight => vec![start, end],
        _ => {
            let mid = (start.1 + end.1) / 2.0 + track;
            vec![start, (start.0, mid), (end.0, mid), end]
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Bounds, routes, zone_of};
    use crate::types::content::{Diagram, DiagramEdge, DiagramNode, DiagramZone};

    fn node(id: &str, x: i32, y: i32) -> DiagramNode {
        DiagramNode {
            id: id.into(),
            name: id.into(),
            detail: String::new(),
            icon: String::new(),
            x,
            y,
            w: 100,
            h: 50,
            emphasis: false,
            dashed: false,
            plain: false,
            step: false,
        }
    }

    fn edge(from: &str, to: &str) -> DiagramEdge {
        DiagramEdge {
            from: from.into(),
            to: to.into(),
            kind: "request".into(),
            two_way: false,
        }
    }

    fn diagram(nodes: Vec<DiagramNode>, edges: Vec<DiagramEdge>) -> Diagram {
        Diagram {
            title: String::new(),
            zones: Vec::new(),
            nodes,
            edges,
            legend: Default::default(),
        }
    }

    #[test]
    fn joins_rows_bottom_to_top() {
        let d = diagram(
            vec![node("a", 0, 0), node("b", 0, 200)],
            vec![edge("a", "b")],
        );
        assert_eq!(routes(&d)[0].path, "M50.0 50.0 L50.0 200.0");
    }

    #[test]
    fn arcs_over_a_part_in_between() {
        let d = diagram(
            vec![node("a", 0, 0), node("m", 150, 0), node("b", 300, 0)],
            vec![edge("a", "b")],
        );
        assert!(routes(&d)[0].path.starts_with("M50.0 0.0 L50.0 -44.0 L"));
    }

    #[test]
    fn forks_lines_of_one_kind_from_one_point() {
        let d = diagram(
            vec![node("a", 100, 0), node("l", 0, 200), node("r", 200, 200)],
            vec![edge("a", "r"), edge("a", "l")],
        );
        let starts: Vec<_> = routes(&d).iter().map(|r| r.path[..10].to_owned()).collect();
        assert_eq!(starts, ["M150.0 50.", "M150.0 50."]);
    }

    #[test]
    fn keeps_kinds_apart_on_one_side() {
        let other = DiagramEdge {
            kind: "agent".into(),
            ..edge("a", "r")
        };
        let d = diagram(
            vec![node("a", 100, 0), node("l", 0, 200), node("r", 200, 200)],
            vec![other, edge("a", "l")],
        );
        let starts: Vec<_> = routes(&d).iter().map(|r| r.path[..10].to_owned()).collect();
        assert_eq!(starts, ["M175.0 50.", "M125.0 50."]);
    }

    #[test]
    fn runs_straight_across_between_neighbours() {
        let d = diagram(
            vec![
                node("a", 0, 0),
                DiagramNode {
                    h: 400,
                    ..node("b", 150, -100)
                },
            ],
            vec![edge("a", "b")],
        );
        assert_eq!(routes(&d)[0].path, "M100.0 25.0 L150.0 25.0");
    }

    #[test]
    fn finds_the_innermost_zone() {
        let zone = |id: &str, w| DiagramZone {
            id: id.into(),
            name: id.into(),
            x: 0,
            y: 0,
            w,
            h: 400,
        };
        let mut d = diagram(vec![node("a", 10, 10)], Vec::new());
        d.zones = vec![zone("outer", 400), zone("inner", 200)];
        assert_eq!(zone_of(&d, &d.nodes[0]), "inner");
    }

    #[test]
    fn skips_edges_to_unknown_ids() {
        let d = diagram(vec![node("a", 0, 0)], vec![edge("a", "nowhere")]);
        assert!(routes(&d).is_empty());
        assert_eq!(Bounds::of(&d).view_box(), "-24 -24 148 98");
    }
}
