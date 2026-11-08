use super::cache::cached;
use crate::{models::Contributions, repositories::mongo::github_snapshots, state::AppState};

/// GitHub counts contributions in near real time, but an hour is fresh enough for a
/// portfolio and keeps us far from its rate limits.
const TTL_SECONDS: Option<u64> = Some(60 * 60);

/// The contribution calendar, or `None` when GitHub is not configured. Each calendar
/// GitHub gives is kept in MongoDB, so when GitHub cannot be reached (an expired token,
/// GitHub being down) the last one is shown instead; only with neither is the graph
/// hidden. A failure is not cached, so the next request retries.
pub async fn contributions(state: &AppState) -> anyhow::Result<Option<Contributions>> {
    if !state.github.is_enabled() {
        return Ok(None);
    }
    let login = state.github.login();
    let result = cached(state, "github:contributions", TTL_SECONDS, async {
        let fresh = state.github.contributions().await?;
        if let Some(calendar) = &fresh
            && let Err(error) = github_snapshots::save(&state.mongo, login, calendar).await
        {
            tracing::warn!(%error, "could not keep the contribution snapshot");
        }
        Ok(fresh)
    })
    .await;
    match result {
        Ok(calendar) => Ok(calendar),
        Err(error) => {
            tracing::warn!(%error, "GitHub contributions unavailable; showing the last snapshot");
            Ok(github_snapshots::latest(&state.mongo, login)
                .await
                .inspect_err(|error| tracing::warn!(%error, "no contribution snapshot"))
                .ok()
                .flatten())
        }
    }
}
