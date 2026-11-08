use rust_i18n::t;
use topcoat::{
    Result,
    context::Cx,
    view::{View, component, view},
};

use crate::{components::hero::hero, composables::hero::hero_slides};

/// The homepage hero: the slides from the database, labelled from the locale.
#[component]
pub async fn home_hero(cx: &Cx) -> Result<impl View> {
    let slides = hero_slides(cx).await?;
    let (tabs_label, pause) = (t!("hero.slides"), t!("hero.pause"));

    Ok(view! {
        hero(
            slides: slides,
            tabs_label: &tabs_label,
            pause_label: &pause,
        )
    })
}
