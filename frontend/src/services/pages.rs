//! `/pages`: pages made of blocks, such as `about`.

use super::client::{ApiClient, ApiResult};
use crate::types::content::Page;

impl ApiClient {
    /// A page of blocks such as `about`; `None` when it does not exist.
    pub async fn page(&self, slug: &str) -> ApiResult<Option<Page>> {
        self.get(&["pages", slug]).await
    }
}
