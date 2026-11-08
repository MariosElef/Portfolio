use topcoat::context::Cx;

use super::api::api;
use crate::{
    services::client::ApiResult,
    types::content::{Block, Project},
};

/// Every project, in the order set in the database.
pub async fn projects(cx: &Cx) -> ApiResult<Vec<Project>> {
    api(cx).projects().await
}

/// The projects marked `featured`.
pub async fn featured_projects(cx: &Cx) -> ApiResult<Vec<Project>> {
    Ok(projects(cx)
        .await?
        .into_iter()
        .filter(|p| p.featured)
        .collect())
}

/// A project, or `None` when no project has this slug.
pub async fn project(cx: &Cx, slug: &str) -> ApiResult<Option<Project>> {
    api(cx).project(slug).await
}

/// A project's case study: its blocks, none when it has no study yet.
pub async fn story(cx: &Cx, slug: &str) -> ApiResult<Vec<Block>> {
    api(cx).story(slug).await
}
