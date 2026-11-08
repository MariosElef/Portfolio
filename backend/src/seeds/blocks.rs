//! Small builders for writing documents by hand: blocks, and diagrams with their zones,
//! parts and lines. Positions are `(x, y, width, height)` in the diagram's own units.

use crate::models::{Diagram, DiagramEdge, DiagramNode, DiagramZone, FaqItem, StoredBlock};

use super::legend;

/// A position and size, in the diagram's units.
pub type Rect = (i32, i32, i32, i32);

pub fn heading(text: &str) -> StoredBlock {
    StoredBlock::Heading { text: text.into() }
}

/// A paragraph, or several separated by blank lines.
pub fn text(text: &str) -> StoredBlock {
    StoredBlock::Text { text: text.into() }
}

pub fn list(items: &[&str]) -> StoredBlock {
    StoredBlock::List {
        items: items.iter().map(|&item| item.into()).collect(),
    }
}

/// Questions and their answers.
pub fn faq(items: &[(&str, &str)]) -> StoredBlock {
    StoredBlock::Faq {
        items: items
            .iter()
            .map(|&(question, answer)| FaqItem {
                question: question.into(),
                answer: answer.into(),
            })
            .collect(),
    }
}

pub fn diagram(
    title: &str,
    zones: Vec<DiagramZone>,
    nodes: Vec<DiagramNode>,
    edges: Vec<DiagramEdge>,
) -> StoredBlock {
    StoredBlock::Diagram(Diagram {
        title: title.into(),
        legend: legend::for_edges(&edges),
        zones,
        nodes,
        edges,
    })
}

/// A labelled area of a diagram.
pub fn zone(id: &str, name: &str, (x, y, w, h): Rect) -> DiagramZone {
    DiagramZone {
        id: id.into(),
        name: name.into(),
        x,
        y,
        w,
        h,
    }
}

/// A part, drawn as a card.
pub fn node(id: &str, name: &str, icon: &str, detail: &str, (x, y, w, h): Rect) -> DiagramNode {
    DiagramNode {
        id: id.into(),
        name: name.into(),
        icon: icon.into(),
        detail: detail.into(),
        x,
        y,
        w,
        h,
        emphasis: false,
        dashed: false,
        plain: false,
        step: false,
    }
}

/// The part a diagram is about, with a brighter edge.
pub fn emphasis(id: &str, name: &str, icon: &str, detail: &str, rect: Rect) -> DiagramNode {
    DiagramNode {
        emphasis: true,
        ..node(id, name, icon, detail, rect)
    }
}

/// A supporting part, with no card.
pub fn plain(id: &str, name: &str, icon: &str, detail: &str, rect: Rect) -> DiagramNode {
    DiagramNode {
        plain: true,
        ..node(id, name, icon, detail, rect)
    }
}

/// A part outside the platform, or someone else's, with a dashed edge.
pub fn outside(id: &str, name: &str, icon: &str, detail: &str, rect: Rect) -> DiagramNode {
    DiagramNode {
        dashed: true,
        ..node(id, name, icon, detail, rect)
    }
}

/// A step in a pipeline.
pub fn step(id: &str, name: &str, detail: &str, rect: Rect) -> DiagramNode {
    DiagramNode {
        step: true,
        ..node(id, name, "", detail, rect)
    }
}

/// A line of one kind from one part to another.
pub fn edge(from: &str, to: &str, kind: &str) -> DiagramEdge {
    DiagramEdge {
        from: from.into(),
        to: to.into(),
        kind: kind.into(),
        two_way: false,
    }
}

/// A line that goes both ways.
pub fn both_ways(edge: DiagramEdge) -> DiagramEdge {
    DiagramEdge {
        two_way: true,
        ..edge
    }
}

/// A request and its answer.
pub fn round_trip(from: &str, to: &str) -> DiagramEdge {
    both_ways(edge(from, to, "request"))
}
