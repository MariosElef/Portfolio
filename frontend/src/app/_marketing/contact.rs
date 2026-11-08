use topcoat::{
    Result,
    context::Cx,
    router::{content::Form, page},
    view::{View, attributes, suspense, view},
};

use crate::{
    components::{
        layout::split::split,
        loading::{skeleton::skeleton, text_skeleton::text_skeleton},
    },
    composables::contact::{ContactForm, Draft, send_contact},
    features::contact::contact_form,
};

/// The contact page; its words and form stream in under a skeleton of their shape.
#[page]
pub async fn page() -> Result<impl View> {
    Ok(view! {
        suspense(
            fallback: view! {
                split(
                    aside: view! {
                        skeleton(attrs: attributes! { class="h-7 w-40 rounded-full" })
                        skeleton(attrs: attributes! { class="h-40 w-full max-w-md" })
                        text_skeleton(lines: 2)
                    }.into(),
                    skeleton(attrs: attributes! { class="h-[32rem] rounded-xl" })
                )
            },
            contact_form(status: None)
        )
    })
}

#[page(POST)]
pub async fn send(cx: &Cx, Form(form): Form<ContactForm>) -> Result<impl View> {
    // What was typed goes back into the form if it is shown again.
    let draft = Draft::from(&form);
    let status = send_contact(cx, form).await;
    Ok(view! { contact_form(status: Some(status), draft: Some(draft)) })
}
