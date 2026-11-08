//! Redis: speed, never the only copy. A failing Redis never fails a request: errors are
//! logged, and callers fall back to the databases or let the request through.

use redis::aio::ConnectionManager;

pub mod cache;
pub mod challenge;
pub mod keys;
pub mod rate_limit;

/// The connection, shared by every request.
#[derive(Clone)]
pub struct Redis(ConnectionManager);

impl Redis {
    pub async fn connect(url: &str) -> anyhow::Result<Self> {
        let manager = redis::Client::open(url)?.get_connection_manager().await?;
        Ok(Self(manager))
    }
}
