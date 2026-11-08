//! What links the stores together, checked: a case study belongs to a project in
//! Postgres, and every media file a document names (the home page's slides too) is one
//! `media` knows. A broken link
//! only loses a picture or a page quietly, so each one is logged at start-up, and the
//! seeds are held to the same rules by the tests.

use std::collections::HashSet;

use crate::{
    models::{CaseStudyDocument, HomeDocument, PageDocument, StoredBlock},
    repositories::{
        mongo::{case_studies, home, pages},
        postgres::{media, projects},
    },
};

/// Each broken link, described.
fn problems(
    projects: &[String],
    studies: &[CaseStudyDocument],
    pages: &[PageDocument],
    home: Option<&HomeDocument>,
    media: &HashSet<String>,
) -> Vec<String> {
    let mut problems = Vec::new();
    let mut check_blocks = |owner: &str, blocks: &[StoredBlock]| {
        for file in blocks.iter().flat_map(StoredBlock::files) {
            if !media.contains(file) {
                problems.push(format!(
                    "{owner} shows media file {file}, which media does not know"
                ));
            }
        }
        for block in blocks {
            if let StoredBlock::Diagram(diagram) = block {
                for edge in &diagram.edges {
                    if !diagram.legend.contains_key(&edge.kind) {
                        problems.push(format!(
                            "{owner}: diagram \"{}\" draws {} lines, which its legend does not explain",
                            diagram.title, edge.kind
                        ));
                    }
                }
            }
        }
    };
    for study in studies {
        let owner = format!("case study {}", study.project);
        check_blocks(&owner, &study.blocks);
    }
    for page in pages {
        let owner = format!("page {}", page.slug);
        check_blocks(&owner, &page.blocks);
    }
    for page in pages {
        if let Some(file) = &page.image
            && !media.contains(file)
        {
            problems.push(format!(
                "page {} pictures media file {file}, which media does not know",
                page.slug
            ));
        }
    }
    for file in home.iter().flat_map(|home| home.files()) {
        if !media.contains(file) {
            problems.push(format!(
                "the home page shows media file {file}, which media does not know"
            ));
        }
    }
    for study in studies {
        if !projects.contains(&study.project) {
            problems.push(format!(
                "case study {} belongs to no project",
                study.project
            ));
        }
    }
    problems.dedup();
    problems
}

/// Logs each broken link between the stores, and each project without a case study.
pub async fn run(pool: &sqlx::PgPool, mongo: &mongodb::Database) -> anyhow::Result<()> {
    let projects = projects::slugs(pool).await?;
    let studies = case_studies::all(mongo).await?;
    let pages = pages::all(mongo).await?;
    let home = home::find(mongo).await?;
    let media: HashSet<String> = media::file_names(pool).await?.into_iter().collect();
    for problem in problems(&projects, &studies, &pages, home.as_ref(), &media) {
        tracing::warn!(problem, "integrity");
    }
    for project in &projects {
        if !studies.iter().any(|study| &study.project == project) {
            tracing::info!(project, "project has no case study yet");
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::seeds::{
        self,
        records::{experience::ROLES, projects::PROJECTS, tools},
    };

    #[test]
    fn the_seeds_link_up() {
        let projects: Vec<String> = PROJECTS.iter().map(|p| p.slug.to_owned()).collect();
        let pages = [seeds::pages::about(), seeds::pages::contact()];
        let found = problems(
            &projects,
            &seeds::case_studies::all(),
            &pages,
            Some(&seeds::home::home()),
            &HashSet::new(),
        );
        assert!(found.is_empty(), "{found:#?}");
    }

    #[test]
    fn every_tool_named_is_a_tool() {
        let known: HashSet<&str> = tools::TOOLS.iter().map(|tool| tool.name).collect();
        let categories: HashSet<&str> = tools::CATEGORIES.iter().map(|c| c.name).collect();
        let named = PROJECTS
            .iter()
            .flat_map(|p| p.tools)
            .chain(ROLES.iter().flat_map(|r| r.tools));
        for tool in named {
            assert!(known.contains(tool), "{tool} is named but is not a tool");
        }
        for tool in tools::TOOLS {
            if let Some(category) = tool.category {
                assert!(
                    categories.contains(category),
                    "{} is in unknown category {category}",
                    tool.name
                );
            }
        }
    }
}
