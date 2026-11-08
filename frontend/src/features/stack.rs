use rust_i18n::t;
use topcoat::{
    Result,
    context::Cx,
    view::{View, component, view},
};

use crate::{
    components::{
        layout::section::section,
        lists::terms::{term_list, term_row},
        tech::mention::tech_mention,
    },
    composables::stack::stack_groups,
};

/// "What I work with": one compact row per category, its tools as logos with their
/// names, separated by hairlines.
#[component]
pub async fn stack_section(cx: &Cx) -> Result<impl View> {
    let groups = stack_groups(cx).await?;

    let title = t!("home.stack");

    Ok(view! {
        section(
            title: &title,
            term_list(
                for group in groups {
                    term_row(term: &group.category, note: &group.note,
                        for item in group.items {
                            tech_mention(icon_name: &item.icon, label: &item.name)
                        }
                    )
                }
            )
        )
    })
}
