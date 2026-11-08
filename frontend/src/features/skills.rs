use rust_i18n::t;
use topcoat::{
    Result,
    context::Cx,
    view::{View, component, view},
};

use crate::components::layout::{
    card_grid::{GridColumns, card_grid},
    stack::stack,
};
use crate::{
    components::{
        layout::section::section,
        lists::ruled::ruled_list,
        surfaces::panel::panel,
        tech::skill_row::skill_row,
        typography::eyebrow::{EyebrowLevel, eyebrow},
    },
    composables::skills::skill_areas,
};

/// "Stack experience": a card per area, each skill with how much experience I have with
/// it, out of five stars.
#[component]
pub async fn skills_section(cx: &Cx) -> Result<impl View> {
    let areas = skill_areas(cx).await?;

    let title = t!("skills.title");

    Ok(view! {
        if !areas.is_empty() {
            section(
                title: &title,
                card_grid(columns: GridColumns::Four,
                    for area in areas {
                        panel(
                            stack(
                                eyebrow(level: EyebrowLevel::H3, (area.name))
                                ruled_list(
                                    for skill in area.skills {
                                        let label = t!("skills.level", level = skill.level);
                                        skill_row(name: &skill.name, level: skill.level, label: &label)
                                    }
                                )
                            )
                        )
                    }
                )
            )
        }
    })
}
