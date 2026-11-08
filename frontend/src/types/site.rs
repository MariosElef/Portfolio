//! The copy every page shares.

use serde::Deserialize;

/// From `GET /site`. Empty when the backend cannot say, so the page around it still
/// renders.
#[derive(Clone, Debug, Default, Deserialize)]
pub struct Site {
    pub name: String,
    /// The browser tab's title.
    pub title: String,
    /// The footer's last line.
    pub footer: String,
    /// Why the site is built as it is. Rich text.
    pub built_with: String,
    /// What runs the site.
    pub stack: String,
    pub github_url: String,
    pub linkedin_url: String,
    pub source_url: String,
    /// The alt text of the portrait on `about`.
    pub portrait_alt: String,
}
