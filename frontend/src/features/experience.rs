use rust_i18n::t;
use topcoat::{
    Result,
    context::Cx,
    view::{View, component, view},
};

use crate::{
    components::{
        experience_timeline::{TimelineLabels, experience_timeline},
        layout::section::section,
    },
    composables::experience::experience,
};

/// "Experience": the work history as a timeline, newest first.
#[component]
pub async fn experience_section(cx: &Cx) -> Result<impl View> {
    let roles = experience(cx).await?;
    let labels = TimelineLabels {
        read_more: t!("experience.read_more").into_owned(),
        did: t!("experience.did").into_owned(),
        learned: t!("experience.learned").into_owned(),
    };

    let title = t!("experience.title");

    Ok(view! {
        if !roles.is_empty() {
            section(title: &title, experience_timeline(roles: roles, labels: labels))
        }
    })
}
