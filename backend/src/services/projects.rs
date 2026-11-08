use super::{
    blocks::resolve,
    cache::{UNTIL_CHANGED, cached, cached_unless_missing},
};
use crate::{
    models::{Block, Project},
    repositories::redis::keys,
    repositories::{mongo::case_studies, postgres::projects},
    state::AppState,
};

pub async fn projects(state: &AppState) -> anyhow::Result<Vec<Project>> {
    cached(state, keys::PROJECTS, UNTIL_CHANGED, async {
        Ok(projects::projects(&state.pool).await?)
    })
    .await
}

pub async fn project(state: &AppState, slug: &str) -> anyhow::Result<Option<Project>> {
    let key = keys::project(slug);
    cached_unless_missing(state, &key, UNTIL_CHANGED, Option::is_none, async {
        Ok(projects::project(&state.pool, slug).await?)
    })
    .await
}

/// The project's case study, with media resolved to URLs. No document means no blocks.
pub async fn case_study(state: &AppState, slug: &str) -> anyhow::Result<Vec<Block>> {
    let key = keys::case_study(slug);
    cached_unless_missing(state, &key, UNTIL_CHANGED, Vec::is_empty, async {
        let Some(study) = case_studies::find(&state.mongo, slug).await? else {
            return Ok(Vec::new());
        };
        resolve(state, study.blocks).await
    })
    .await
}
