//! Messages sent through the contact form, and the question that proves a person sent
//! them.

use serde::{Deserialize, Serialize};

/// A message sent through the contact form.
#[derive(Clone, Debug, Deserialize)]
pub struct NewMessage {
    #[serde(default)]
    pub name: String,
    pub email: String,
    pub message: String,
}

/// A message as the form posts it: with the question it was given, and the answer.
#[derive(Debug, Deserialize)]
pub struct PostedMessage {
    #[serde(flatten)]
    pub message: NewMessage,
    /// The signed token of the question asked.
    #[serde(default)]
    pub challenge: String,
    #[serde(default)]
    pub answer: String,
}

/// A small sum for a person to answer, `a + b`, and its signed token to send back.
#[derive(Debug, Serialize)]
pub struct Challenge {
    pub a: u8,
    pub b: u8,
    pub token: String,
}
