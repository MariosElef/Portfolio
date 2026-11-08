//! The pages the site starts with.

use crate::{
    models::{PageDocument, StoredBlock},
    seeds::blocks::text,
};

/// Drafted from the CV and the GitHub profile README. `[[icon:Label]]` shows a logo.
pub fn about() -> PageDocument {
    PageDocument {
        slug: "about".into(),
        image: None,
        blocks: vec![
            text(
                "I'm a backend-focused developer with professional experience in fintech, \
                 building microservices in [[python:Python]] ([[fastapi:FastAPI]]) with \
                 [[rabbitmq:RabbitMQ]], SQL and NoSQL databases, deployed with \
                 [[docker:Docker]] and [[kubernetes:Kubernetes]] on [[linux:Linux]]. \
                 [[rust:Rust]] (Axum) is my language of choice for my own projects.",
            ),
            text(
                "On the side I'm designing and building MC2, a multi-tenant B2B2C SaaS \
                 platform: a Nuxt platform in front of a Rust API gateway, FastAPI services \
                 and an AI agent, running on k3s with PostgreSQL, Redis and Traefik. I also \
                 built and run waterfilternet.com.",
            ),
            StoredBlock::List {
                items: [
                    "Backend-first mindset",
                    "Love working with [[kubernetes:Kubernetes]] and microservices to retain a clean, layered architecture",
                    "Passionate about performance, type safety, and clean architecture",
                    "Credential-less GitOps: merges trigger in-cluster builds via Argo, promoted with Kargo",
                    "Always learning, always building",
                ]
                .map(String::from)
                .into(),
            },
            text(
                "I'm finishing a degree in Computer Science (Information Technologies) at New \
                 Bulgarian University, and I speak Greek, English and Bulgarian.",
            ),
        ],
    }
}

/// The words above the contact form.
pub fn contact() -> PageDocument {
    PageDocument {
        slug: "contact".into(),
        image: None,
        blocks: vec![text("Send a message and I'll get back to you.")],
    }
}
