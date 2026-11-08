use rust_i18n::t;
use topcoat::{
    Result,
    context::Cx,
    icon::iconify::iconify_icon,
    view::{View, attributes, component, view},
};

use crate::{
    components::{
        actions::link_tile::link_tile,
        feedback::{alert::alert, status_badge::status_badge, success::success},
        forms::{
            field_row::field_row,
            floating_field::{floating_input, floating_textarea},
            form::form,
            hidden_field::hidden_field,
            honeypot::honeypot,
            send_step::send_step,
        },
        layout::{
            split::split,
            stack::{StackGap, stack},
        },
        surfaces::panel::panel,
        tone::Color,
        typography::{
            eyebrow::eyebrow,
            lead::lead,
            note::note,
            page_title::{TitleSize, page_title},
            rich_text::rich_text,
            section_heading::section_heading,
        },
    },
    composables::{
        contact::{ContactStatus, Draft, challenge},
        pages::page,
        site::site,
    },
    types::content::Block,
    utils::url::display,
};

/// The contact page: what it is for and where else to find me on the left, the form on
/// the right in a card, and a clear confirmation in its place once a message is sent.
#[component]
pub async fn contact_form(
    cx: &Cx,
    status: Option<ContactStatus>,
    /// What was typed, put back after a failed send.
    #[default]
    draft: Option<Draft>,
) -> Result<impl View> {
    let intro = page(cx, "contact")
        .await?
        .into_iter()
        .flat_map(|page| page.blocks)
        .find_map(|block| match block {
            Block::Text { text } => Some(text),
            _ => None,
        });
    let links = site(cx).await;
    let (name, email, message) = (
        t!("contact.name"),
        t!("contact.email"),
        t!("contact.message"),
    );
    let (elsewhere, sent_title) = (t!("contact.elsewhere"), t!("contact.sent_title"));
    let problem = status.and_then(|status| match status {
        ContactStatus::Sent => None,
        ContactStatus::Invalid => Some(t!("contact.invalid")),
        ContactStatus::Limited => Some(t!("contact.limited")),
        ContactStatus::NeedsAnswer => Some(t!("contact.needs_answer")),
        ContactStatus::Unanswered => Some(t!("contact.unanswered")),
        ContactStatus::Failed => Some(t!("contact.failed")),
    });
    let sent = status == Some(ContactStatus::Sent);
    let draft = draft.unwrap_or_default();
    // Back to have the question answered: it shows from the start.
    let asking = matches!(
        status,
        Some(ContactStatus::NeedsAnswer | ContactStatus::Unanswered)
    );
    let send_label = t!("contact.send");
    // A new question each time the form is shown.
    let question = if sent { None } else { challenge(cx).await };
    let (puzzle, token) = question.map_or((String::new(), String::new()), |question| {
        (
            t!("contact.puzzle", a = question.a, b = question.b).into_owned(),
            question.token,
        )
    });

    let (github, linkedin) = (t!("contact.github"), t!("contact.linkedin"));
    let (available, form_title, form_hint, privacy) = (
        t!("contact.available"),
        t!("contact.form_title"),
        t!("contact.form_hint"),
        t!("contact.privacy"),
    );

    Ok(view! {
        split(
            aside: view! {
                status_badge((available))
                page_title(size: TitleSize::Display, (t!("contact.heading")))
                if let Some(intro) = intro {
                    lead(rich_text(text: &intro))
                }
                stack(gap: StackGap::Tight,
                    eyebrow((elsewhere))
                    if !links.github_url.is_empty() {
                        link_tile(
                            href: &links.github_url,
                            icon_data: iconify_icon!("lucide:github"),
                            title: &github,
                            detail: display(&links.github_url),
                        )
                    }
                    if !links.linkedin_url.is_empty() {
                        link_tile(
                            href: &links.linkedin_url,
                            icon_data: iconify_icon!("lucide:linkedin"),
                            title: &linkedin,
                            detail: display(&links.linkedin_url),
                        )
                    }
                )
            }.into(),
            panel(roomy: true, outlined: true,
                if sent {
                    success(title: &sent_title, (t!("contact.sent")))
                } else {
                    form(
                        stack(gap: StackGap::Snug,
                            section_heading(attrs: attributes! { class="text-2xl" }, (form_title))
                            note((form_hint))
                        )
                        if let Some(problem) = problem {
                            alert(color: Color::Error, (problem))
                        }
                        field_row(
                            floating_input(
                                id: "name",
                                label_text: &name,
                                icon_data: Some(iconify_icon!("lucide:user")),
                                attrs: attributes! { name="name" autocomplete="name" maxlength="200" value=(draft.name.clone()) }
                            )
                            floating_input(
                                id: "email",
                                label_text: &email,
                                icon_data: Some(iconify_icon!("lucide:mail")),
                                attrs: attributes! { name="email" type="email" autocomplete="email" required="" value=(draft.email.clone()) }
                            )
                        )
                        floating_textarea(
                            id: "message",
                            label_text: &message,
                            value: draft.message.clone(),
                            attrs: attributes! { name="message" rows="7" maxlength="5000" required="" }
                        )
                        hidden_field(name: "challenge", value: &token)
                        honeypot(name: "website")
                        // To show a person sent it: a small sum, signed by the backend, asked
                        // once Send is pressed.
                        send_step(
                            label: &send_label,
                            open: asking,
                            step: view! {
                                floating_input(
                                    id: "answer",
                                    label_text: &puzzle,
                                    icon_data: Some(iconify_icon!("lucide:puzzle")),
                                    attrs: attributes! { name="answer" inputmode="numeric" pattern="[0-9]*" maxlength="3" autocomplete="off" }
                                )
                            }.into(),
                            note: view! { note((privacy)) }.into(),
                        )
                    )
                }
            )
        )
    })
}
