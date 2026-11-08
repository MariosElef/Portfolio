//! Records as the seeds describe them, for start-up to write where missing. Media is
//! named by its file (`<sha256>.<ext>`), never by id, so the links hold on any database
//! that has the files.

pub struct CategoryRecord {
    pub name: &'static str,
    pub note: &'static str,
    pub sort: i32,
}

pub struct ToolRecord {
    pub name: &'static str,
    /// The key of its logo in the frontend's logo map.
    pub icon: &'static str,
    /// Listed in the stack under this category; `None` for a tool only named elsewhere.
    pub category: Option<&'static str>,
    pub sort: i32,
}

pub struct ProjectRecord {
    pub slug: &'static str,
    pub initials: &'static str,
    /// `light`, `brand`, `mint` or `plum`.
    pub tone: &'static str,
    pub name: &'static str,
    pub summary: &'static str,
    pub repo: Option<&'static str>,
    pub featured: bool,
    pub sort: i32,
    /// `coming_soon` or `private`, for a project without a link.
    pub availability: &'static str,
    pub logo: Option<&'static str>,
    pub cover: Option<&'static str>,
    /// Tool names, in order.
    pub tools: &'static [&'static str],
}

pub struct RoleRecord {
    pub slug: &'static str,
    pub sort: i32,
    pub year: &'static str,
    pub role: &'static str,
    pub company: &'static str,
    pub period: &'static str,
    pub summary: &'static str,
    pub color: &'static str,
    pub accent: &'static str,
    pub logo: Option<&'static str>,
    pub did: &'static [&'static str],
    pub learned: &'static [&'static str],
    /// Tool names, in order.
    pub tools: &'static [&'static str],
}

pub struct SkillAreaRecord {
    pub name: &'static str,
    pub sort: i32,
    /// Each skill: its name and level, 1 to 5.
    pub skills: &'static [(&'static str, i16)],
}
