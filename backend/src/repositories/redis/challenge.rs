//! Questions answered: each one's id, kept until it would have expired anyway, so it is
//! answered only once.

use super::Redis;

impl Redis {
    /// Marks `key` spent for `seconds`; whether it was unspent until now. `None` when Redis
    /// cannot be reached.
    pub async fn spend(&self, key: &str, seconds: u64) -> Option<bool> {
        let mut conn = self.0.clone();
        let set: Option<String> = redis::cmd("SET")
            .arg(key)
            .arg(1)
            .arg("NX")
            .arg("EX")
            .arg(seconds)
            .query_async(&mut conn)
            .await
            .inspect_err(|error| tracing::warn!(%error, key, "could not spend the question"))
            .ok()?;
        Some(set.is_some())
    }
}
