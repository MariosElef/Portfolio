//! `/messages`: the contact form.

use reqwest::StatusCode;

use super::client::{ApiClient, ApiResult};
use crate::types::{
    content::{Challenge, NewMessage},
    visitor::ClientIp,
};

/// Whether the backend accepted a contact message.
pub enum Sent {
    Ok,
    Invalid,
    /// Too many messages from this visitor for now.
    Limited,
    /// The question was answered wrong, or no longer stands.
    Unanswered,
}

impl ApiClient {
    /// A question for the contact form to ask.
    pub async fn challenge(&self) -> ApiResult<Option<Challenge>> {
        self.get(&["messages", "challenge"]).await
    }

    /// Sends a message on behalf of the visitor at `client_ip`, whose address the
    /// backend's rate limit counts by.
    pub async fn send_message(
        &self,
        message: &NewMessage,
        client_ip: Option<&ClientIp>,
    ) -> ApiResult<Sent> {
        let mut request = self.http.post(self.url(&["messages"])).json(message);
        if let Some(ClientIp(ip)) = client_ip {
            request = request.header("x-client-ip", ip);
        }
        let response = request.send().await?;
        match response.status() {
            StatusCode::UNPROCESSABLE_ENTITY => return Ok(Sent::Invalid),
            StatusCode::TOO_MANY_REQUESTS => return Ok(Sent::Limited),
            StatusCode::FORBIDDEN => return Ok(Sent::Unanswered),
            _ => {}
        }
        response.error_for_status()?;
        Ok(Sent::Ok)
    }
}
