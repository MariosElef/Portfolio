//! Email: a note to me for each message sent through the contact form, sent through a
//! Gmail account to my inbox, with the visitor to reply to.

use lettre::{
    AsyncSmtpTransport, AsyncTransport, Message, Tokio1Executor,
    message::{Mailbox, header::ContentType},
    transport::smtp::authentication::Credentials,
};

use crate::{config::Smtp, models::NewMessage};

/// Gmail's SMTP server, over TLS from the first byte (port 465).
const GMAIL: &str = "smtp.gmail.com";

/// Sends the notes, or does nothing when no account is configured.
#[derive(Clone)]
pub struct Mailer(Option<Account>);

#[derive(Clone)]
struct Account {
    transport: AsyncSmtpTransport<Tokio1Executor>,
    from: Mailbox,
    to: Mailbox,
}

impl Mailer {
    /// Ready to send through `smtp`, or off without it.
    pub fn new(smtp: Option<Smtp>) -> anyhow::Result<Self> {
        let Some(smtp) = smtp else {
            return Ok(Self(None));
        };
        let transport = AsyncSmtpTransport::<Tokio1Executor>::relay(GMAIL)?
            .credentials(Credentials::new(smtp.user.clone(), smtp.password))
            .build();
        Ok(Self(Some(Account {
            transport,
            from: Mailbox::new(Some("Portfolio".into()), smtp.user.parse()?),
            to: smtp.notify.parse()?,
        })))
    }

    /// Emails me `message`; nothing when no account is configured.
    pub async fn notify(&self, message: &NewMessage) -> anyhow::Result<()> {
        let Some(account) = &self.0 else {
            return Ok(());
        };
        let email = note(account.from.clone(), account.to.clone(), message)?;
        account.transport.send(email).await?;
        Ok(())
    }
}

/// The note about `message`: from my own account, so it is never taken for spoofed,
/// with the visitor as Reply-To, so replying answers them.
fn note(from: Mailbox, to: Mailbox, message: &NewMessage) -> anyhow::Result<Message> {
    // One line, whatever was typed into the name.
    let name: String = message
        .name
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ");
    let who = if name.is_empty() {
        message.email.as_str()
    } else {
        name.as_str()
    };
    let mut builder = Message::builder()
        .from(from)
        .to(to)
        .subject(format!("New message from {who}"))
        .header(ContentType::TEXT_PLAIN);
    if let Ok(address) = message.email.parse() {
        builder = builder.reply_to(Mailbox::new(
            (!name.is_empty()).then(|| name.clone()),
            address,
        ));
    }
    Ok(builder.body(format!(
        "{who} <{}> wrote, through the contact form:\n\n{}\n",
        message.email, message.message
    ))?)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn message(name: &str) -> NewMessage {
        NewMessage {
            name: name.into(),
            email: "visitor@example.com".into(),
            message: "Hello!".into(),
        }
    }

    fn sent(name: &str) -> String {
        let email = note(
            "Portfolio <me@example.com>".parse().unwrap(),
            "inbox@example.org".parse().unwrap(),
            &message(name),
        )
        .unwrap();
        String::from_utf8(email.formatted()).unwrap()
    }

    #[test]
    fn replies_go_to_the_visitor() {
        let email = sent("Ada");
        assert!(email.contains("Subject: New message from Ada"), "{email}");
        assert!(
            email.contains("Reply-To: Ada <visitor@example.com>"),
            "{email}"
        );
        assert!(email.contains("To: inbox@example.org"), "{email}");
        assert!(email.contains("Hello!"), "{email}");
    }

    #[test]
    fn a_name_cannot_add_headers() {
        let email = sent("Ada\r\nBcc: someone@example.com");
        assert!(!email.contains("\r\nBcc:"), "{email}");
    }

    #[test]
    fn without_a_name_the_subject_names_the_address() {
        assert!(sent("").contains("Subject: New message from visitor@example.com"));
    }
}
