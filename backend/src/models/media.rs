//! Uploaded media.

use serde::Serialize;
use uuid::Uuid;

#[derive(Clone, Debug, Serialize)]
pub struct Uploaded {
    pub id: Uuid,
    pub url: String,
}
