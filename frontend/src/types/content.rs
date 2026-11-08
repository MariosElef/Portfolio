use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize)]
pub struct MediaRef {
    pub url: String,
    pub content_type: String,
}

impl MediaRef {
    pub fn is_video(&self) -> bool {
        self.content_type.starts_with("video/")
    }
}

/// One slide of the homepage hero, and the tab that shows it.
#[derive(Clone, Debug, Deserialize)]
pub struct HeroSlide {
    pub tab_label: String,
    pub eyebrow: String,
    pub title: String,
    pub subtitle: String,
    pub callout: String,
    pub cta_label: String,
    pub cta_href: String,
    pub media: Option<MediaRef>,
    pub poster: Option<MediaRef>,
    /// Code typed out in place of the photo; empty for none.
    #[serde(default)]
    pub code: String,
    /// `rust` or `python`.
    #[serde(default)]
    pub code_language: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct Project {
    pub slug: String,
    pub initials: String,
    pub tone: String,
    pub name: String,
    pub summary: String,
    pub tags: Vec<String>,
    pub repo: Option<String>,
    pub featured: bool,
    /// The photo the card shows on hover.
    pub cover: Option<MediaRef>,
    /// The mark its page shows large; without one, its initials.
    #[serde(default)]
    pub logo: Option<MediaRef>,
    /// Without a link: `coming_soon` or `private`.
    #[serde(default)]
    pub availability: String,
}

/// One block of a project's case study.
#[derive(Clone, Debug, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Block {
    Text {
        text: String,
    },
    Heading {
        text: String,
    },
    Image {
        media: MediaRef,
        caption: String,
    },
    Gallery {
        media: Vec<MediaRef>,
    },
    Code {
        lang: String,
        code: String,
    },
    Quote {
        text: String,
        author: String,
    },
    List {
        items: Vec<String>,
    },
    /// A numbered sequence, such as a request traced step by step.
    Steps {
        items: Vec<String>,
    },
    Diagram(Diagram),
    Faq {
        items: Vec<FaqItem>,
    },
}

/// A question and its answer, opened in place.
#[derive(Clone, Debug, Deserialize)]
pub struct FaqItem {
    pub question: String,
    pub answer: String,
}

/// A graph drawn to scale: labelled zones, the nodes inside them, and arrows between
/// nodes or zones by id.
#[derive(Clone, Debug, Deserialize)]
pub struct Diagram {
    pub title: String,
    pub zones: Vec<DiagramZone>,
    pub nodes: Vec<DiagramNode>,
    pub edges: Vec<DiagramEdge>,
    /// What each kind of line it draws means, by kind.
    #[serde(default)]
    pub legend: std::collections::BTreeMap<String, LegendEntry>,
}

/// One kind of line, explained under the diagram.
#[derive(Clone, Debug, Deserialize)]
pub struct LegendEntry {
    pub label: String,
    pub detail: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct DiagramZone {
    pub id: String,
    pub name: String,
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
}

#[derive(Clone, Debug, Deserialize)]
pub struct DiagramNode {
    pub id: String,
    pub name: String,
    pub detail: String,
    /// A Lucide icon name, from the set the site knows.
    pub icon: String,
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
    pub emphasis: bool,
    pub dashed: bool,
    /// No card: just the icon, name and a small description, for a supporting part.
    pub plain: bool,
    /// A step in a pipeline: a circle with a check, its name and description below.
    pub step: bool,
}

/// `kind` is `request`, `identity`, `agent`, `delivery`, `state` or `telemetry`.
#[derive(Clone, Debug, Deserialize)]
pub struct DiagramEdge {
    pub from: String,
    pub to: String,
    pub kind: String,
    pub two_way: bool,
}

#[derive(Clone, Debug, Serialize)]
pub struct NewMessage {
    pub name: String,
    pub email: String,
    pub message: String,
    /// The signed token of the question the form asked, and the visitor's answer.
    pub challenge: String,
    pub answer: String,
}

/// A small sum the contact form asks, `a + b`, with the token to send back.
#[derive(Clone, Debug, Deserialize)]
pub struct Challenge {
    pub a: u8,
    pub b: u8,
    pub token: String,
}

/// One tool in the tech stack.
#[derive(Clone, Debug, Deserialize)]
pub struct StackItem {
    pub category: String,
    pub category_note: String,
    pub name: String,
    pub icon: String,
}

/// A skill in the stack experience: how much experience, 1 to 5 stars.
#[derive(Clone, Debug, Deserialize)]
pub struct Skill {
    pub area: String,
    pub name: String,
    pub level: u8,
}

/// A GitHub contribution calendar: the last year, one entry per week, Sunday first.
#[derive(Clone, Debug, Deserialize)]
pub struct Contributions {
    pub total: u32,
    pub weeks: Vec<Vec<ContributionDay>>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ContributionDay {
    /// `YYYY-MM-DD`.
    pub date: String,
    /// 0 is Sunday.
    pub weekday: u8,
    pub count: u32,
    /// 0 for none, then 1–4 by quartile.
    pub level: u8,
}

/// One role in the work history.
#[derive(Clone, Debug, Deserialize)]
pub struct Experience {
    /// Shown beside the line, e.g. "2026" or "Now".
    pub year: String,
    pub role: String,
    pub company: String,
    pub period: String,
    /// The one line under the role.
    pub summary: String,
    /// `#rrggbb`: the dot and company name. `accent`, if set, is a second colour.
    pub color: String,
    pub accent: String,
    /// The company's logo, if one is set.
    pub logo: Option<MediaRef>,
    pub did: Vec<String>,
    pub learned: Vec<String>,
    pub tags: Vec<String>,
}

/// A page made of blocks, with its picture if it has one.
#[derive(Clone, Debug, Deserialize)]
pub struct Page {
    pub image: Option<MediaRef>,
    pub blocks: Vec<Block>,
}
