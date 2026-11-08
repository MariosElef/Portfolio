use std::{path::PathBuf, sync::Arc};

use sqlx::PgPool;

use crate::{
    clients::{github::GitHub, mailer::Mailer},
    repositories::redis::Redis,
};

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
    pub mongo: mongodb::Database,
    pub redis: Redis,
    pub github: GitHub,
    /// Emails me each contact message, when an account is configured.
    pub mailer: Mailer,
    /// Where uploaded files are written; Caddy serves this directory at `/media`.
    pub media_dir: PathBuf,
    pub admin_token: AdminToken,
    /// Signs the contact form's questions.
    pub challenge_key: Arc<[u8]>,
}

/// The secret that authorises uploads. Empty disables them.
#[derive(Clone)]
pub struct AdminToken(pub Arc<str>);
