//! Architecture diagrams, drawn to scale.

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

/// A graph drawn to scale: zones are labelled areas, nodes the parts inside them, and
/// edges the arrows between nodes or zones, referred to by id. Positions are in the
/// diagram's own units; the site scales it to fit.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Diagram {
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub zones: Vec<DiagramZone>,
    pub nodes: Vec<DiagramNode>,
    #[serde(default)]
    pub edges: Vec<DiagramEdge>,
    /// What each kind of line it draws means, by kind.
    #[serde(default)]
    pub legend: BTreeMap<String, LegendEntry>,
}

/// One kind of line, explained under the diagram.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LegendEntry {
    pub label: String,
    pub detail: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DiagramZone {
    pub id: String,
    pub name: String,
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
}

/// A part. `emphasis` highlights the one the diagram is about; `dashed` marks one that
/// is planned or not everywhere.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DiagramNode {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub detail: String,
    /// A Lucide icon name, from the set the site knows.
    #[serde(default)]
    pub icon: String,
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
    #[serde(default)]
    pub emphasis: bool,
    #[serde(default)]
    pub dashed: bool,
    /// No card: just the icon, name and a small description, for a supporting part.
    #[serde(default)]
    pub plain: bool,
    /// A step in a pipeline: a circle with a check, its name and description below.
    #[serde(default)]
    pub step: bool,
}

/// An arrow from one node or zone to another. `kind` names what crosses it, such as a
/// `request` or `telemetry`; the frontend's `components/diagram/kinds.rs` lists them and
/// their colours.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DiagramEdge {
    pub from: String,
    pub to: String,
    pub kind: String,
    /// Arrowheads at both ends.
    #[serde(default)]
    pub two_way: bool,
}
