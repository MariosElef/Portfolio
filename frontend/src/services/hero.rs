//! `/home`: the hero's slides.

use super::client::{ApiClient, ApiResult};
use crate::types::content::HeroSlide;

impl ApiClient {
    pub async fn hero(&self) -> ApiResult<Vec<HeroSlide>> {
        Ok(self.get(&["home", "hero"]).await?.unwrap_or_default())
    }
}
