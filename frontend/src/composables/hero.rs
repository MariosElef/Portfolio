use topcoat::context::Cx;

use super::api::api;
use crate::{services::client::ApiResult, types::content::HeroSlide};

/// The homepage hero's slides, in order.
pub async fn hero_slides(cx: &Cx) -> ApiResult<Vec<HeroSlide>> {
    api(cx).hero().await
}
