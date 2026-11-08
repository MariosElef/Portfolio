//! How much experience I have with each skill, by area.

use crate::models::SkillAreaRecord;

pub const AREAS: &[SkillAreaRecord] = &[
    SkillAreaRecord {
        name: "Backend",
        sort: 1,
        skills: &[
            ("Rust", 4),
            ("Axum", 4),
            ("Python", 4),
            ("FastAPI", 4),
            ("API design", 4),
            ("Microservices", 4),
            ("Auth (OIDC, OAuth 2.0)", 4),
            ("Async (Tokio, asyncio)", 4),
            ("Message brokers", 4),
            ("Testing", 4),
        ],
    },
    SkillAreaRecord {
        name: "Data",
        sort: 2,
        skills: &[
            ("PostgreSQL", 4),
            ("SQL", 4),
            ("Data modelling & migrations", 4),
            ("Redis", 4),
            ("MongoDB", 4),
            ("MariaDB", 4),
        ],
    },
    SkillAreaRecord {
        name: "Infrastructure",
        sort: 3,
        skills: &[
            ("Docker", 3),
            ("Linux", 3),
            ("Kubernetes", 3),
            ("CI/CD & GitOps", 3),
            ("Observability", 3),
        ],
    },
    SkillAreaRecord {
        name: "Frontend",
        sort: 4,
        skills: &[
            ("Nuxt / Vue", 2),
            ("TypeScript", 2),
            ("Topcoat", 2),
            ("Tailwind CSS", 2),
        ],
    },
];
