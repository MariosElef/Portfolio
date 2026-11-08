//! The copy every page shares.

use crate::models::Site;

pub fn site() -> Site {
    Site {
        name: "Marios Elef".into(),
        title: "Marios Elef — Software engineer".into(),
        footer: "© Marios Elef".into(),
        built_with: "Rust is my favourite language, so this site is Rust end to end: \
                     [[rust:Topcoat]] on the frontend, [[rust:Axum]] on the backend."
            .into(),
        stack: "Topcoat · Axum · Postgres · Redis · MongoDB".into(),
        github_url: "https://github.com/MariosElef".into(),
        linkedin_url: "https://www.linkedin.com/in/marios-eleftheriou-59b399339/".into(),
        source_url: "https://github.com/MariosElef/Portfolio".into(),
        portrait_alt: "Portrait of Marios Eleftheriou".into(),
    }
}
