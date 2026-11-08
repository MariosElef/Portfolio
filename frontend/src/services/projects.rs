//! `/projects`: the projects and their case studies.

use super::client::{ApiClient, ApiResult};
use crate::types::content::{Block, Project};

impl ApiClient {
    pub async fn projects(&self) -> ApiResult<Vec<Project>> {
        Ok(self.get(&["projects"]).await?.unwrap_or_default())
    }

    pub async fn project(&self, slug: &str) -> ApiResult<Option<Project>> {
        self.get(&["projects", slug]).await
    }

    pub async fn story(&self, slug: &str) -> ApiResult<Vec<Block>> {
        Ok(self
            .get(&["projects", slug, "story"])
            .await?
            .unwrap_or_default())
    }
}
