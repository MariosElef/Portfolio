//! Serving content from Redis when it is there, and caching it when it is not.

use std::future::Future;

use serde::{Serialize, de::DeserializeOwned};

use crate::state::AppState;

/// Content is kept until it changes: each edit deletes the keys it touches
/// (`jobs::cache_invalidation`), and every reconnect of the listeners deletes them all,
/// in case an edit was missed meanwhile.
pub const UNTIL_CHANGED: Option<u64> = None;

/// How long a lookup that found nothing is kept: long enough to absorb a burst, short
/// enough that made-up slugs never pile up in Redis.
pub const MISSING_SECONDS: u64 = 60;

/// Cache-aside: serve the copy cached under `key`, or run `load` and cache its result
/// for `ttl_seconds`, or with `None` until something deletes it.
pub async fn cached<T, F>(
    state: &AppState,
    key: &str,
    ttl_seconds: Option<u64>,
    load: F,
) -> anyhow::Result<T>
where
    T: Serialize + DeserializeOwned,
    F: Future<Output = anyhow::Result<T>>,
{
    cached_unless_missing(state, key, ttl_seconds, |_| false, load).await
}

/// As [`cached`], but a result `is_missing` says found nothing is kept only for
/// [`MISSING_SECONDS`].
pub async fn cached_unless_missing<T, F>(
    state: &AppState,
    key: &str,
    ttl_seconds: Option<u64>,
    is_missing: impl Fn(&T) -> bool,
    load: F,
) -> anyhow::Result<T>
where
    T: Serialize + DeserializeOwned,
    F: Future<Output = anyhow::Result<T>>,
{
    if let Some(value) = state.redis.get_json(key).await {
        return Ok(value);
    }
    let value = load.await?;
    let ttl = if is_missing(&value) {
        Some(MISSING_SECONDS)
    } else {
        ttl_seconds
    };
    state.redis.set_json(key, &value, ttl).await;
    Ok(value)
}
