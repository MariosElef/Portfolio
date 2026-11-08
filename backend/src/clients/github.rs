use reqwest::Client;

use crate::models::Contributions;

mod parse;

use parse::parse;

/// The contribution graph from a user's public GitHub profile: the same page a
/// logged-out visitor sees, so it needs no token and includes private contributions
/// when the user shows them. It is GitHub's HTML rather than an API, so a change to
/// their markup makes [`GitHub::contributions`] fail, which hides the graph.
#[derive(Clone)]
pub struct GitHub {
    http: Client,
    login: String,
}

impl GitHub {
    pub fn new(login: String) -> Self {
        Self {
            http: Client::new(),
            login,
        }
    }

    pub fn login(&self) -> &str {
        &self.login
    }

    pub fn is_enabled(&self) -> bool {
        !self.login.is_empty()
    }

    pub async fn contributions(&self) -> anyhow::Result<Option<Contributions>> {
        if !self.is_enabled() {
            return Ok(None);
        }
        let html = self
            .http
            .get(format!(
                "https://github.com/users/{}/contributions",
                self.login
            ))
            // GitHub rejects requests without one.
            .header(reqwest::header::USER_AGENT, "portfolio-backend")
            .send()
            .await?
            .error_for_status()?
            .text()
            .await?;
        parse(&html).map(Some)
    }
}
