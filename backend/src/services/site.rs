use super::cache::{UNTIL_CHANGED, cached};
use crate::{models::Site, repositories::mongo::site, repositories::redis::keys, state::AppState};

/// The copy every page shares.
pub async fn site(state: &AppState) -> anyhow::Result<Option<Site>> {
    cached(state, keys::SITE, UNTIL_CHANGED, async {
        Ok(site::find(&state.mongo).await?)
    })
    .await
}
