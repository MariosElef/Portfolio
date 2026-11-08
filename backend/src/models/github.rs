//! The GitHub contribution calendar.

use serde::{Deserialize, Serialize};

/// A GitHub contribution calendar: the last year, one entry per week, Sunday first.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Contributions {
    pub total: u32,
    pub weeks: Vec<Vec<ContributionDay>>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ContributionDay {
    /// `YYYY-MM-DD`.
    pub date: String,
    /// 0 is Sunday.
    pub weekday: u8,
    pub count: u32,
    /// 0 for none, then 1–4 by quartile, as GitHub shades it.
    pub level: u8,
}
