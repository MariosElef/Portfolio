//! Rate limits: a counter per visitor that expires with its window.

use super::Redis;

impl Redis {
    /// Counts one more hit under `key`, a counter that disappears `window_seconds` after
    /// its first hit, and returns the count so far; `None` when Redis cannot be reached.
    pub async fn hit(&self, key: &str, window_seconds: u64) -> Option<u64> {
        let mut conn = self.0.clone();
        let (count,): (u64,) = redis::pipe()
            .atomic()
            .incr(key, 1)
            // NX: only the first hit starts the window, so it never slides.
            .cmd("EXPIRE")
            .arg(key)
            .arg(window_seconds)
            .arg("NX")
            .ignore()
            .query_async(&mut conn)
            .await
            .inspect_err(|error| tracing::warn!(%error, key, "rate count failed"))
            .ok()?;
        Some(count)
    }
}
