//! `/profile`: the stack, skills and work history.

use super::client::{ApiClient, ApiResult};
use crate::types::content::{Experience, Skill, StackItem};

impl ApiClient {
    pub async fn stack(&self) -> ApiResult<Vec<StackItem>> {
        Ok(self.get(&["profile", "stack"]).await?.unwrap_or_default())
    }

    pub async fn skills(&self) -> ApiResult<Vec<Skill>> {
        Ok(self.get(&["profile", "skills"]).await?.unwrap_or_default())
    }

    pub async fn experience(&self) -> ApiResult<Vec<Experience>> {
        Ok(self
            .get(&["profile", "experience"])
            .await?
            .unwrap_or_default())
    }
}
