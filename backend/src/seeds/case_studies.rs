//! The case studies the site starts with, one file per project.

use crate::models::CaseStudyDocument;

mod chess;
mod mc2;
mod pygame;
mod stackup;
mod wazuh_suricata_misp;

/// Every case study, by its project's slug.
pub fn all() -> Vec<CaseStudyDocument> {
    vec![
        mc2::study(),
        wazuh_suricata_misp::study(),
        stackup::study(),
        pygame::study(),
        chess::study(),
    ]
}
