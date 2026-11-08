//! Projects: the grid on the home page, the full list, and a project's own page.
//!
//! Features know no routes, so each project arrives with the URL of its page.

use crate::types::content::Project;

pub mod detail;
pub mod grid;
pub mod list;

/// A project and where its page is.
pub struct ProjectEntry {
    pub project: Project,
    pub page: String,
}
