//! `/site`: the copy every page shares.

use super::client::{ApiClient, ApiResult};
use crate::types::site::Site;

impl ApiClient {
    pub async fn site(&self) -> ApiResult<Option<Site>> {
        self.get(&["site"]).await
    }
}
