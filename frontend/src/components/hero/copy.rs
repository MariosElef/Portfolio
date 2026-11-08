use topcoat::{
    Result,
    view::{View, component, view},
};

use crate::{
    components::actions::{
        button::{ButtonSize, ButtonVariant},
        link_button::link_button,
    },
    types::content::HeroSlide,
};

/// A line of slide text: it slides in from the right on its slide's turn, as the cycle
/// times it, and again whenever its slide is chosen (the slide sets which, as
/// `--hero-line`), each line a little after the one before. It rests in place when
/// paused, or for visitors who prefer less motion.
const LINE: &str = "[animation:var(--hero-line)] \
    group-has-[#hero-pause:checked]/hero:[animation-play-state:paused] \
    motion-reduce:[animation:none]";

/// A slide's words: its eyebrow, title, subtitle and callout, then its button.
#[component]
pub async fn slide_copy(slide: HeroSlide) -> Result<impl View> {
    Ok(view! {
        if !slide.eyebrow.is_empty() {
            <p class=(format!("{LINE} text-sm font-bold sm:text-base"))>(slide.eyebrow)</p>
        }
        <h1 class=(format!("{LINE} [animation-delay:60ms]! max-w-3xl text-5xl leading-[0.95] font-bold tracking-tighter text-balance sm:text-7xl lg:text-8xl"))>
            (slide.title)
        </h1>
        if !slide.subtitle.is_empty() {
            <p class=(format!("{LINE} [animation-delay:120ms]! max-w-xl text-lg text-white/85 sm:text-xl"))>(slide.subtitle)</p>
        }
        if !slide.callout.is_empty() {
            <p class=(format!("{LINE} [animation-delay:160ms]! max-w-xl text-lg font-bold sm:text-xl"))>(slide.callout)</p>
        }
        <div class=(format!("{LINE} [animation-delay:200ms]! mt-6"))>
            link_button(
                href: slide.cta_href.as_str(),
                variant: ButtonVariant::Inverse,
                size: ButtonSize::Xl,
                (slide.cta_label.as_str())
            )
        </div>
    })
}
