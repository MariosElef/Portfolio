//! Writing the records the site starts with, each only where it is missing, so edits
//! made in the database stay. Media is linked by file name.

use sqlx::PgPool;

use crate::models::{CategoryRecord, ProjectRecord, RoleRecord, SkillAreaRecord, ToolRecord};

pub async fn categories(pool: &PgPool, records: &[CategoryRecord]) -> sqlx::Result<()> {
    for record in records {
        sqlx::query_file!(
            "src/repositories/postgres/sql/records/insert_category.sql",
            record.name,
            record.note,
            record.sort
        )
        .execute(pool)
        .await?;
    }
    Ok(())
}

pub async fn tools(pool: &PgPool, records: &[ToolRecord]) -> sqlx::Result<()> {
    for record in records {
        sqlx::query_file!(
            "src/repositories/postgres/sql/records/insert_tool.sql",
            record.name,
            record.icon,
            record.category,
            record.sort
        )
        .execute(pool)
        .await?;
    }
    Ok(())
}

/// Each new project, with its tools. A project already there keeps what it has, but gains
/// a logo or cover it is missing.
pub async fn projects(pool: &PgPool, records: &[ProjectRecord]) -> sqlx::Result<()> {
    for record in records {
        let inserted = sqlx::query_file_scalar!(
            "src/repositories/postgres/sql/records/insert_project.sql",
            record.slug,
            record.initials,
            record.tone,
            record.name,
            record.summary,
            record.repo,
            record.featured,
            record.sort,
            record.availability,
            record.logo,
            record.cover
        )
        .fetch_optional(pool)
        .await?;
        if inserted.is_none() {
            sqlx::query_file!(
                "src/repositories/postgres/sql/records/fill_project_media.sql",
                record.slug,
                record.logo,
                record.cover
            )
            .execute(pool)
            .await?;
            continue;
        }
        for (sort, tool) in record.tools.iter().enumerate() {
            sqlx::query_file!(
                "src/repositories/postgres/sql/records/link_project_tool.sql",
                record.slug,
                tool,
                sort as i32
            )
            .execute(pool)
            .await?;
        }
    }
    Ok(())
}

/// Each new role, with its tools. A role already there keeps what it has, but gains a logo
/// it is missing.
pub async fn roles(pool: &PgPool, records: &[RoleRecord]) -> sqlx::Result<()> {
    for record in records {
        let did: Vec<String> = record.did.iter().map(|line| line.to_string()).collect();
        let learned: Vec<String> = record.learned.iter().map(|line| line.to_string()).collect();
        let inserted = sqlx::query_file_scalar!(
            "src/repositories/postgres/sql/records/insert_role.sql",
            record.slug,
            record.sort,
            record.year,
            record.role,
            record.company,
            record.period,
            record.summary,
            record.color,
            record.accent,
            record.logo,
            &did,
            &learned
        )
        .fetch_optional(pool)
        .await?;
        let Some(id) = inserted else {
            sqlx::query_file!(
                "src/repositories/postgres/sql/records/fill_role_logo.sql",
                record.slug,
                record.logo
            )
            .execute(pool)
            .await?;
            continue;
        };
        for (sort, tool) in record.tools.iter().enumerate() {
            sqlx::query_file!(
                "src/repositories/postgres/sql/records/link_role_tool.sql",
                id,
                tool,
                sort as i32
            )
            .execute(pool)
            .await?;
        }
    }
    Ok(())
}

pub async fn skills(pool: &PgPool, records: &[SkillAreaRecord]) -> sqlx::Result<()> {
    for area in records {
        sqlx::query_file!(
            "src/repositories/postgres/sql/records/insert_skill_area.sql",
            area.name,
            area.sort
        )
        .execute(pool)
        .await?;
        for (sort, (name, level)) in area.skills.iter().enumerate() {
            sqlx::query_file!(
                "src/repositories/postgres/sql/records/insert_skill.sql",
                area.name,
                name,
                level,
                sort as i32
            )
            .execute(pool)
            .await?;
        }
    }
    Ok(())
}
