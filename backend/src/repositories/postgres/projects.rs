use sqlx::PgPool;

use super::media::media_ref;
use crate::models::Project;

struct ProjectRow {
    slug: String,
    initials: String,
    tone: String,
    name: String,
    summary: String,
    tags: Vec<String>,
    repo: Option<String>,
    featured: bool,
    availability: String,
    cover_file: Option<String>,
    cover_type: Option<String>,
    logo_file: Option<String>,
    logo_type: Option<String>,
}

impl From<ProjectRow> for Project {
    fn from(row: ProjectRow) -> Self {
        Self {
            slug: row.slug,
            initials: row.initials,
            tone: row.tone,
            name: row.name,
            summary: row.summary,
            tags: row.tags,
            repo: row.repo,
            featured: row.featured,
            cover: media_ref(row.cover_file, row.cover_type),
            logo: media_ref(row.logo_file, row.logo_type),
            availability: row.availability,
        }
    }
}

pub async fn projects(pool: &PgPool) -> sqlx::Result<Vec<Project>> {
    let rows = sqlx::query_file_as!(
        ProjectRow,
        "src/repositories/postgres/sql/projects/list.sql"
    )
    .fetch_all(pool)
    .await?;
    Ok(rows.into_iter().map(Project::from).collect())
}

pub async fn project(pool: &PgPool, slug: &str) -> sqlx::Result<Option<Project>> {
    let row = sqlx::query_file_as!(
        ProjectRow,
        "src/repositories/postgres/sql/projects/find.sql",
        slug
    )
    .fetch_optional(pool)
    .await?;
    Ok(row.map(Project::from))
}

/// Every project's slug.
pub async fn slugs(pool: &PgPool) -> sqlx::Result<Vec<String>> {
    sqlx::query_file_scalar!("src/repositories/postgres/sql/projects/slugs.sql")
        .fetch_all(pool)
        .await
}
