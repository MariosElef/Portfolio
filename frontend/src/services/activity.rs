//! `/activity`: the GitHub contribution calendar.

use super::client::{ApiClient, ApiResult};
use crate::types::content::Contributions;

impl ApiClient {
    /// `None` when GitHub is not configured or cannot be read.
    pub async fn contributions(&self) -> ApiResult<Option<Contributions>> {
        self.get(&["activity", "contributions"]).await
    }
}
