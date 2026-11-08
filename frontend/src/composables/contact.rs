use serde::Deserialize;
use topcoat::context::{Cx, try_request_context};

use super::api::api;
use crate::{
    services::messages::Sent,
    types::{
        content::{Challenge, NewMessage},
        visitor::ClientIp,
    },
};

/// The contact form as posted.
#[derive(Deserialize)]
pub struct ContactForm {
    #[serde(default)]
    pub name: String,
    pub email: String,
    pub message: String,
    /// Hidden from people; a bot that fills it in is ignored.
    #[serde(default)]
    pub website: String,
    /// The question's signed token, and the visitor's answer.
    #[serde(default)]
    pub challenge: String,
    #[serde(default)]
    pub answer: String,
}

/// What the visitor typed, to put back in the form when it is shown again.
#[derive(Clone, Default)]
pub struct Draft {
    pub name: String,
    pub email: String,
    pub message: String,
}

impl From<&ContactForm> for Draft {
    fn from(form: &ContactForm) -> Self {
        Self {
            name: form.name.clone(),
            email: form.email.clone(),
            message: form.message.clone(),
        }
    }
}

/// A new question for the form to ask; `None` when the backend cannot give one.
pub async fn challenge(cx: &Cx) -> Option<Challenge> {
    api(cx).challenge().await.ok().flatten()
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ContactStatus {
    Sent,
    Invalid,
    /// Sent too many times in a short while; try again later.
    Limited,
    /// Sent before the question was answered.
    NeedsAnswer,
    /// The question was answered wrong, or no longer stands.
    Unanswered,
    Failed,
}

/// Sends the message to the backend. A bot is told it worked and nothing is stored.
pub async fn send_contact(cx: &Cx, form: ContactForm) -> ContactStatus {
    if !form.website.is_empty() {
        return ContactStatus::Sent;
    }
    if form.answer.trim().is_empty() {
        return ContactStatus::NeedsAnswer;
    }
    let message = NewMessage {
        name: form.name,
        email: form.email,
        message: form.message,
        challenge: form.challenge,
        answer: form.answer,
    };
    match api(cx)
        .send_message(&message, try_request_context::<ClientIp>(cx))
        .await
    {
        Ok(Sent::Ok) => ContactStatus::Sent,
        Ok(Sent::Invalid) => ContactStatus::Invalid,
        Ok(Sent::Limited) => ContactStatus::Limited,
        Ok(Sent::Unanswered) => ContactStatus::Unanswered,
        Err(error) => {
            eprintln!("contact message failed: {error}");
            ContactStatus::Failed
        }
    }
}
