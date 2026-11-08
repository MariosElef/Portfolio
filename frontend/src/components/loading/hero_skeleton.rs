use topcoat::{
    Result,
    view::{View, attributes, class, component, view},
};

use super::skeleton::skeleton;
use crate::components::layout::container::CONTAINER;

/// The home page's slideshow still loading: its black, the slide's words as bars, and
/// its row of tabs.
#[component]
pub async fn hero_skeleton() -> Result<impl View> {
    Ok(view! {
        <section aria-busy="true" class="flex min-h-dvh w-full flex-col bg-[#0b0d14]">
            <div class=(class!(CONTAINER, "flex flex-1 flex-col justify-center gap-6 pt-28 pb-16"))>
                skeleton(attrs: attributes! { class="h-5 w-40" })
                skeleton(attrs: attributes! { class="h-16 w-full max-w-2xl sm:h-24" })
                skeleton(attrs: attributes! { class="h-6 w-full max-w-xl" })
                skeleton(attrs: attributes! { class="mt-6 h-10 w-36 rounded-full" })
            </div>
            <div class=(class!(CONTAINER, "flex gap-2 pb-10"))>
                for _ in 0..4 {
                    skeleton(attrs: attributes! { class="h-[76px] w-60 shrink-0 rounded-sm" })
                }
            </div>
        </section>
    })
}
