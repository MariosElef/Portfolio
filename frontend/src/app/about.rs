use topcoat::{
    Result,
    router::page,
    view::{View, attributes, suspense, view},
};

use crate::components::{
    layout::page::{PageGap, PageWidth, page as page_layout},
    loading::{
        chips_skeleton::chips_skeleton, rows_skeleton::rows_skeleton,
        section_skeleton::section_skeleton, skeleton::skeleton, text_skeleton::text_skeleton,
    },
};
use crate::features::{
    about::about_section, activity::activity_section, experience::experience_section,
    skills::skills_section, stack::stack_section,
};

/// Everything about me: the intro and portrait, the work history, the tools I use, how
/// much experience I have with them, and the year on GitHub. Each part streams in on its
/// own, under a skeleton of its shape.
#[page]
pub async fn page() -> Result<impl View> {
    Ok(view! {
        page_layout(
            width: PageWidth::Full,
            gap: PageGap::Sections,
            suspense(fallback: view! { section_skeleton(text_skeleton(lines: 6)) }, about_section())
            suspense(fallback: view! { section_skeleton(rows_skeleton(count: 6)) }, experience_section())
            suspense(fallback: view! { section_skeleton(chips_skeleton()) }, stack_section())
            suspense(fallback: view! { section_skeleton(rows_skeleton(count: 4)) }, skills_section())
            suspense(
                fallback: view! { section_skeleton(skeleton(attrs: attributes! { class="h-48 rounded-xl" })) },
                activity_section()
            )
        )
    })
}
