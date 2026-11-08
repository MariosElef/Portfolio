//! What the site shows: hero slides, projects, the stack, skills and work history.

use serde::{Deserialize, Serialize};

/// A media file as the site references it: a same-origin URL Caddy serves.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MediaRef {
    pub url: String,
    pub content_type: String,
}

impl MediaRef {
    pub fn new(file_name: &str, content_type: String) -> Self {
        Self {
            url: format!("/media/{file_name}"),
            content_type,
        }
    }
}

/// One slide of the homepage hero, and the tab that shows it.
#[derive(Clone, Debug, Serialize, Deserialize)]
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
    pub code: String,
    /// `rust` or `python`.
    pub code_language: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Project {
    pub slug: String,
    pub initials: String,
    pub tone: String,
    pub name: String,
    pub summary: String,
    /// The tools it is built with, in order.
    pub tags: Vec<String>,
    pub repo: Option<String>,
    pub featured: bool,
    /// The photo the card shows on hover.
    pub cover: Option<MediaRef>,
    /// The mark its page shows large; without one, its initials.
    pub logo: Option<MediaRef>,
    /// Without a link: `coming_soon` or `private`.
    pub availability: String,
}

/// One tool in the stack, under its category.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StackItem {
    pub category: String,
    pub category_note: String,
    pub name: String,
    /// The key of its logo in the frontend's logo map.
    pub icon: String,
}

/// A skill in the stack experience: how much experience, 1 to 5 stars.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Skill {
    pub area: String,
    pub name: String,
    pub level: i16,
}

/// One role in the work history.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Experience {
    pub year: String,
    pub role: String,
    pub company: String,
    pub period: String,
    pub summary: String,
    /// `#rrggbb`: the dot and company name. `accent`, if set, is a second colour.
    pub color: String,
    pub accent: String,
    pub logo: Option<MediaRef>,
    pub did: Vec<String>,
    pub learned: Vec<String>,
    /// The tools used in the role, in order.
    pub tags: Vec<String>,
}
