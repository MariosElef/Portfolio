use super::challenge;
use crate::{
    models::{NewMessage, PostedMessage},
    repositories::postgres::messages,
    state::AppState,
};

const MAX_NAME: usize = 200;
const MAX_EMAIL: usize = 320;
const MAX_MESSAGE: usize = 5000;

#[derive(Debug)]
pub enum MessageError {
    Invalid,
    /// The question was not answered, answered wrong, expired, or answered before.
    Unanswered,
    Failed(anyhow::Error),
}

/// Checks the form's question was answered, validates and stores the message, then
/// emails it to me.
pub async fn send(
    state: &AppState,
    posted: PostedMessage,
    user_agent: Option<&str>,
) -> Result<(), MessageError> {
    if !challenge::answered(state, &posted.challenge, &posted.answer).await {
        return Err(MessageError::Unanswered);
    }
    let mut message: NewMessage = posted.message;
    message.name = message.name.trim().to_owned();
    message.email = message.email.trim().to_owned();
    message.message = message.message.trim().to_owned();

    let email_ok = message.email.len() <= MAX_EMAIL
        && message
            .email
            .split_once('@')
            .is_some_and(|(user, domain)| !user.is_empty() && domain.contains('.'));
    let valid = email_ok
        && message.name.chars().count() <= MAX_NAME
        && !message.message.is_empty()
        && message.message.chars().count() <= MAX_MESSAGE;
    if !valid {
        return Err(MessageError::Invalid);
    }

    messages::insert(&state.pool, &message, user_agent)
        .await
        .map_err(|error| MessageError::Failed(error.into()))?;

    // Emailed after it is stored, without the visitor waiting: if the email fails, the
    // message is still in the inbox, and the failure is logged.
    let mailer = state.mailer.clone();
    tokio::spawn(async move {
        if let Err(error) = mailer.notify(&message).await {
            tracing::warn!(%error, "could not email the contact message");
        }
    });
    Ok(())
}
