use topcoat::context::Cx;

use super::api::api;
use crate::{services::client::ApiResult, types::content::Experience};

/// The work history, newest first.
pub async fn experience(cx: &Cx) -> ApiResult<Vec<Experience>> {
    api(cx).experience().await
}
