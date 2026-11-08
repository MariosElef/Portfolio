//! The home page the site starts with.

use crate::models::{HomeDocument, StoredSlide};

/// The hero's slides, each with code typed out in place of a photo.
pub fn home() -> HomeDocument {
    HomeDocument {
        slides: vec![
            StoredSlide {
                tab_label: "Software engineering".into(),
                eyebrow: "Software engineer".into(),
                title: "Building things that last.".into(),
                subtitle: "Replace this line with what you build and what you care about.".into(),
                callout: "Open to new work.".into(),
                cta_label: "See projects".into(),
                cta_href: "/projects".into(),
                media: None,
                poster: None,
                code: r#"fn welcome(name: &str) -> String {
    format!("Welcome, {name}! I'm Marios.")
}

fn main() {
    println!("{}", welcome("friend"));
}"#
                .into(),
                code_language: "rust".into(),
            },
            StoredSlide {
                tab_label: "Backend and APIs".into(),
                eyebrow: "Backend".into(),
                title: "Fast, typed, boring in the best way.".into(),
                subtitle: "Placeholder: the services and APIs you build, and what they run on."
                    .into(),
                callout: "".into(),
                cta_label: "See projects".into(),
                cta_href: "/projects".into(),
                media: None,
                poster: None,
                code: r#"def welcome(name: str) -> str:
    return f"Welcome, {name}! I'm Marios."


print(welcome("friend"))"#
                    .into(),
                code_language: "python".into(),
            },
            StoredSlide {
                tab_label: "Developer tools".into(),
                eyebrow: "Tooling".into(),
                title: "Tools that get out of the way.".into(),
                subtitle: "Placeholder: the CLIs and tooling you make for yourself and others."
                    .into(),
                callout: "".into(),
                cta_label: "See stackup".into(),
                cta_href: "/projects/stackup".into(),
                media: None,
                poster: None,
                code: r#"#[derive(Debug)]
enum Tool {
    Rust,
    Python,
    Docker,
}

fn main() {
    use Tool::*;
    for tool in [Rust, Python, Docker] {
        println!("Welcome to my {tool:?} tools.");
    }
}"#
                .into(),
                code_language: "rust".into(),
            },
            StoredSlide {
                tab_label: "Work with me".into(),
                eyebrow: "Available".into(),
                title: "Let's build something together.".into(),
                subtitle: "Placeholder: what kind of work you are looking for.".into(),
                callout: "".into(),
                cta_label: "Get in touch".into(),
                cta_href: "/contact".into(),
                media: None,
                poster: None,
                code: r#"def work_with_me(you: str) -> str:
    return f"Hi {you}, let's build something."


print(work_with_me("you"))"#
                    .into(),
                code_language: "python".into(),
            },
        ],
    }
}
