//! The kinds of line a diagram draws, in legend order, each with its colour. One table,
//! so a kind is added in one place. The classes are written out whole so Tailwind finds
//! them in the source.

/// A kind's colour as the classes each part needs.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct KindStyle {
    /// The line and its glow.
    pub stroke: &'static str,
    /// The legend's swatch.
    pub border: &'static str,
    /// A plain part's icon, in the colour of the line leading into it.
    pub text: &'static str,
}

pub const KINDS: [(&str, KindStyle); 13] = [
    (
        "request",
        KindStyle {
            stroke: "stroke-sky-400",
            border: "border-sky-400",
            text: "text-sky-400",
        },
    ),
    (
        "webhook",
        KindStyle {
            stroke: "stroke-indigo-400",
            border: "border-indigo-400",
            text: "text-indigo-400",
        },
    ),
    (
        "identity",
        KindStyle {
            stroke: "stroke-violet-400",
            border: "border-violet-400",
            text: "text-violet-400",
        },
    ),
    (
        "agent",
        KindStyle {
            stroke: "stroke-rose-400",
            border: "border-rose-400",
            text: "text-rose-400",
        },
    ),
    (
        "delivery",
        KindStyle {
            stroke: "stroke-amber-400",
            border: "border-amber-400",
            text: "text-amber-400",
        },
    ),
    (
        "session",
        KindStyle {
            stroke: "stroke-yellow-300",
            border: "border-yellow-300",
            text: "text-yellow-300",
        },
    ),
    (
        "state",
        KindStyle {
            stroke: "stroke-zinc-500",
            border: "border-zinc-500",
            text: "text-zinc-500",
        },
    ),
    (
        "alert",
        KindStyle {
            stroke: "stroke-orange-400",
            border: "border-orange-400",
            text: "text-orange-400",
        },
    ),
    (
        "stream",
        KindStyle {
            stroke: "stroke-cyan-400",
            border: "border-cyan-400",
            text: "text-cyan-400",
        },
    ),
    (
        "integrity",
        KindStyle {
            stroke: "stroke-lime-400",
            border: "border-lime-400",
            text: "text-lime-400",
        },
    ),
    (
        "intel",
        KindStyle {
            stroke: "stroke-fuchsia-400",
            border: "border-fuchsia-400",
            text: "text-fuchsia-400",
        },
    ),
    (
        "tamper",
        KindStyle {
            stroke: "stroke-red-400",
            border: "border-red-400",
            text: "text-red-400",
        },
    ),
    (
        "telemetry",
        KindStyle {
            stroke: "stroke-emerald-400",
            border: "border-emerald-400",
            text: "text-emerald-400",
        },
    ),
];

/// For a kind not in the table.
const OTHER: KindStyle = KindStyle {
    stroke: "stroke-zinc-500",
    border: "border-zinc-500",
    text: "text-muted-foreground",
};

impl KindStyle {
    /// The colour itself, as a CSS value, for an animation that changes it.
    pub fn color(self) -> String {
        format!("var(--color-{})", self.stroke.trim_start_matches("stroke-"))
    }
}

/// The colour of `kind`.
pub fn style(kind: &str) -> KindStyle {
    KINDS
        .iter()
        .find(|(name, _)| *name == kind)
        .map_or(OTHER, |(_, style)| *style)
}

#[cfg(test)]
mod tests {
    use super::{KINDS, style};

    #[test]
    fn knows_every_kind_and_falls_back_for_others() {
        assert_eq!(style("request").stroke, "stroke-sky-400");
        assert_eq!(style("unknown").border, "border-zinc-500");
        let mut names: Vec<_> = KINDS.iter().map(|(name, _)| *name).collect();
        names.dedup();
        assert_eq!(names.len(), KINDS.len());
    }
}
