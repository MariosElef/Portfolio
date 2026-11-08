//! The home page: one document in MongoDB, its slides in order.

use serde::{Deserialize, Serialize};

/// The home page, as stored.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct HomeDocument {
    pub slides: Vec<StoredSlide>,
}

/// One slide of the hero and its tab, as stored. Media is named by its file in `media`.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StoredSlide {
    pub tab_label: String,
    #[serde(default)]
    pub eyebrow: String,
    pub title: String,
    #[serde(default)]
    pub subtitle: String,
    #[serde(default)]
    pub callout: String,
    pub cta_label: String,
    pub cta_href: String,
    /// A photo or video behind the slide.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub media: Option<String>,
    /// The still shown instead of a video for visitors who prefer less motion.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub poster: Option<String>,
    /// Code typed out in place of the photo; empty for none.
    #[serde(default)]
    pub code: String,
    /// `rust` or `python`.
    #[serde(default)]
    pub code_language: String,
}

impl HomeDocument {
    /// The media files the slides show.
    pub fn files(&self) -> impl Iterator<Item = &String> {
        self.slides
            .iter()
            .flat_map(|slide| slide.media.iter().chain(slide.poster.iter()))
    }
}
