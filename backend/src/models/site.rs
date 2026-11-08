//! The copy the whole site shares: its name, the footer, the links out.

use serde::{Deserialize, Serialize};

/// One document in MongoDB, shown on every page.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Site {
    pub name: String,
    /// The browser tab's title.
    pub title: String,
    /// The footer's last line.
    pub footer: String,
    /// Why the site is built as it is, shown in the footer. Rich text.
    pub built_with: String,
    /// The footer's list of what runs the site.
    pub stack: String,
    pub github_url: String,
    pub linkedin_url: String,
    pub source_url: String,
    /// The alt text of the portrait on `about`.
    pub portrait_alt: String,
}
