use topcoat::context::Cx;

use super::api::api;
use crate::{services::client::ApiResult, types::content::Page};

/// A page made of blocks, such as `about`, with its picture; `None` when there is none.
pub async fn page(cx: &Cx, slug: &str) -> ApiResult<Option<Page>> {
    api(cx).page(slug).await
}
