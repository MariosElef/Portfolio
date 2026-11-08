//! What the About page shows: the stack, skills and work history.

use sqlx::PgPool;

use super::media::media_ref;
use crate::models::{Experience, Skill, StackItem};

/// The tools listed in the stack: those with a category, in its order.
pub async fn stack(pool: &PgPool) -> sqlx::Result<Vec<StackItem>> {
    sqlx::query_file_as!(StackItem, "src/repositories/postgres/sql/profile/stack.sql")
        .fetch_all(pool)
        .await
}

struct ExperienceRow {
    year: String,
    role: String,
    company: String,
    period: String,
    summary: String,
    color: String,
    accent: String,
    logo_file: Option<String>,
    logo_type: Option<String>,
    did: Vec<String>,
    learned: Vec<String>,
    tags: Vec<String>,
}

pub async fn experience(pool: &PgPool) -> sqlx::Result<Vec<Experience>> {
    let rows = sqlx::query_file_as!(
        ExperienceRow,
        "src/repositories/postgres/sql/profile/experience.sql"
    )
    .fetch_all(pool)
    .await?;

    Ok(rows
        .into_iter()
        .map(|row| Experience {
            year: row.year,
            role: row.role,
            company: row.company,
            period: row.period,
            summary: row.summary,
            color: row.color,
            accent: row.accent,
            logo: media_ref(row.logo_file, row.logo_type),
            did: row.did,
            learned: row.learned,
            tags: row.tags,
        })
        .collect())
}

pub async fn skills(pool: &PgPool) -> sqlx::Result<Vec<Skill>> {
    sqlx::query_file_as!(Skill, "src/repositories/postgres/sql/profile/skills.sql")
        .fetch_all(pool)
        .await
}
