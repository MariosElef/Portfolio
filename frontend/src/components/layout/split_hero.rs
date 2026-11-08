use topcoat::{
    Result,
    view::{Child, View, class, component, view},
};

use super::container::CONTAINER;

/// A page's opening, tall on wide screens: its words on the left and `aside` (such as a
/// portrait) on the right, which comes first on a phone.
#[component]
pub async fn split_hero(
    #[default] aside: Child<'_>,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <section class="relative isolate overflow-hidden">
            <div class=(class!(CONTAINER, "grid items-center md:min-h-[46rem] md:grid-cols-[minmax(0,1fr)_minmax(0,1.15fr)]"))>
                <div class="relative z-10 flex flex-col gap-8 pb-8 md:py-16">(child)</div>
                <div class="relative order-first h-[28rem] empty:hidden sm:h-[34rem] md:order-none md:h-auto md:min-h-[46rem] md:self-stretch">(aside)</div>
            </div>
        </section>
    })
}
