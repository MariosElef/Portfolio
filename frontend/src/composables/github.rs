use topcoat::context::Cx;

use super::api::api;
use crate::{services::client::ApiResult, types::content::Contributions};

/// The GitHub contribution calendar, or `None` when GitHub is not configured.
pub async fn contributions(cx: &Cx) -> ApiResult<Option<Contributions>> {
    api(cx).contributions().await
}
