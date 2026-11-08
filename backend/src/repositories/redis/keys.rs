//! The name of every key the site keeps in Redis.

/// The prefix of every cached content key; `jobs::cache_invalidation` deletes the keys
/// whose content changed, and all of them after a reconnect.
pub const CONTENT_PREFIX: &str = "content:";

pub const PROJECTS: &str = "content:projects";
/// Every `project:{slug}` key.
pub const PROJECT_PREFIX: &str = "content:project:";
pub const HERO: &str = "content:hero";
pub const STACK: &str = "content:stack";
pub const SKILLS: &str = "content:skills";
pub const EXPERIENCE: &str = "content:experience";
pub const SITE: &str = "content:site";
/// Every `case-study:{slug}` key.
pub const CASE_STUDY_PREFIX: &str = "content:case-study:";
/// Every `page:{slug}` key.
pub const PAGE_PREFIX: &str = "content:page:";

pub fn project(slug: &str) -> String {
    format!("{PROJECT_PREFIX}{slug}")
}

pub fn case_study(project: &str) -> String {
    format!("{CASE_STUDY_PREFIX}{project}")
}

pub fn page(slug: &str) -> String {
    format!("{PAGE_PREFIX}{slug}")
}

/// A rate limit's counter: `name`'s hits from one visitor in the current window.
pub fn rate(name: &str, visitor: &str) -> String {
    format!("rate:{name}:{visitor}")
}

/// A contact form question, once answered: by its id.
pub fn challenge(id: &str) -> String {
    format!("challenge:{id}")
}
