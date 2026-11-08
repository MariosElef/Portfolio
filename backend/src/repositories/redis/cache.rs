//! The content cache: JSON copies of what the API serves, kept until they change.

use redis::AsyncCommands;
use serde::{Serialize, de::DeserializeOwned};

use super::Redis;

impl Redis {
    pub async fn get_json<T: DeserializeOwned>(&self, key: &str) -> Option<T> {
        let mut conn = self.0.clone();
        let raw: Option<String> = conn
            .get(key)
            .await
            .inspect_err(|error| tracing::warn!(%error, key, "cache read failed"))
            .ok()?;
        serde_json::from_str(&raw?).ok()
    }

    /// Stores `value` under `key`, until `ttl_seconds` pass or, with `None`, until deleted.
    pub async fn set_json<T: Serialize>(&self, key: &str, value: &T, ttl_seconds: Option<u64>) {
        let Ok(raw) = serde_json::to_string(value) else {
            return;
        };
        let mut conn = self.0.clone();
        let written = match ttl_seconds {
            Some(ttl) => conn.set_ex::<_, _, ()>(key, raw, ttl).await,
            None => conn.set::<_, _, ()>(key, raw).await,
        };
        if let Err(error) = written {
            tracing::warn!(%error, key, "cache write failed");
        }
    }

    /// Deletes `keys`.
    pub async fn delete(&self, keys: &[String]) -> redis::RedisResult<()> {
        if keys.is_empty() {
            return Ok(());
        }
        let mut conn = self.0.clone();
        conn.del::<_, ()>(keys).await
    }

    /// Deletes every key starting with `prefix`.
    pub async fn delete_prefix(&self, prefix: &str) -> redis::RedisResult<()> {
        let mut conn = self.0.clone();
        let pattern = format!("{prefix}*");
        let mut cursor = 0_u64;
        loop {
            let (next, keys): (u64, Vec<String>) = redis::cmd("SCAN")
                .arg(cursor)
                .arg("MATCH")
                .arg(&pattern)
                .arg("COUNT")
                .arg(100)
                .query_async(&mut conn)
                .await?;
            if !keys.is_empty() {
                conn.del::<_, ()>(keys).await?;
            }
            if next == 0 {
                return Ok(());
            }
            cursor = next;
        }
    }
}
