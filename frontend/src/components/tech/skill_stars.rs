use topcoat::{
    Result,
    view::{View, class, component, view},
};

/// How much experience, as five small stars with the first `level` filled. Read out as
/// `label`, such as "4 out of 5".
#[component]
pub async fn skill_stars(level: u8, label: &str) -> Result<impl View> {
    Ok(view! {
        <span role="img" aria-label=(label) class="inline-flex gap-px text-[10px] leading-none">
            for star in 1..=5u8 {
                <span aria-hidden="true" class=(class!("text-primary-emphasis" if star <= level else "text-muted-foreground/30"))>"★"</span>
            }
        </span>
    })
}
