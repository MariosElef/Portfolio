//! The backend's settings, from the environment.

use std::path::PathBuf;

/// Everything the backend is configured with.
pub struct Config {
    pub database_url: String,
    pub redis_url: String,
    pub mongodb_url: String,
    /// Where uploaded files are written; Caddy serves this directory at `/media`.
    pub media_dir: PathBuf,
    /// The secret that authorises uploads. Empty disables them.
    pub admin_token: String,
    /// Whose public GitHub contributions to show. Empty hides the graph.
    pub github_user: String,
    /// Signs the contact form's questions. Empty: a random key each start, which only
    /// means a question asked before a restart has to be asked again.
    pub challenge_secret: String,
    /// The account that emails me each contact message. Without it, messages are only
    /// stored.
    pub smtp: Option<Smtp>,
    pub bind_addr: String,
}

/// A Gmail account and its app password, and where its notes go.
pub struct Smtp {
    pub user: String,
    pub password: String,
    pub notify: String,
}

impl Config {
    /// From the environment. The three databases are required; the rest have defaults, and
    /// leaving out an optional feature's setting is logged, not fatal.
    pub fn from_env() -> anyhow::Result<Self> {
        let var = |name: &str| std::env::var(name);
        let or = |name: &str, default: &str| var(name).unwrap_or_else(|_| default.into());
        let config = Self {
            database_url: var("DATABASE_URL")?,
            redis_url: var("REDIS_URL")?,
            mongodb_url: var("MONGODB_URL")?,
            media_dir: PathBuf::from(or("MEDIA_DIR", "data/media")),
            admin_token: or("ADMIN_TOKEN", ""),
            github_user: or("GITHUB_USER", ""),
            challenge_secret: or("CHALLENGE_SECRET", ""),
            smtp: match (
                or("SMTP_USER", ""),
                or("SMTP_PASSWORD", ""),
                or("NOTIFY_EMAIL", ""),
            ) {
                (user, password, notify)
                    if !user.is_empty() && !password.is_empty() && !notify.is_empty() =>
                {
                    Some(Smtp {
                        user,
                        password,
                        notify,
                    })
                }
                _ => None,
            },
            bind_addr: or("BIND_ADDR", "0.0.0.0:8000"),
        };
        if config.admin_token.is_empty() {
            tracing::warn!("ADMIN_TOKEN is not set; uploads are disabled");
        }
        if config.smtp.is_none() {
            tracing::warn!(
                "SMTP_USER, SMTP_PASSWORD or NOTIFY_EMAIL is not set; messages are stored but not emailed"
            );
        }
        if config.github_user.is_empty() {
            tracing::warn!("GITHUB_USER is not set; the contribution graph is hidden");
        }
        Ok(config)
    }
}
