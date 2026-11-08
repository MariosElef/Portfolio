use topcoat::{
    Result,
    view::{View, class, component, view},
};

use crate::{components::layout::container::CONTAINER, types::content::HeroSlide};

mod code;
mod controls;
mod copy;
mod photo;
mod timing;

use code::slide_code;
use controls::{pause_toggle, slide_tab};
use copy::slide_copy;
use photo::slide_photo;

/// Which slide is held while a tab is hovered or focused, or a slide's button focused:
/// `--hero-at` names it. One rule per slide, written out so Tailwind finds each class.
const HELD: &str = "\
    has-[#hero-tab-0:is(:hover,:focus)]:[--hero-at:0] has-[#hero-slide-0:focus-within]:[--hero-at:0] \
    has-[#hero-tab-1:is(:hover,:focus)]:[--hero-at:1] has-[#hero-slide-1:focus-within]:[--hero-at:1] \
    has-[#hero-tab-2:is(:hover,:focus)]:[--hero-at:2] has-[#hero-slide-2:focus-within]:[--hero-at:2] \
    has-[#hero-tab-3:is(:hover,:focus)]:[--hero-at:3] has-[#hero-slide-3:focus-within]:[--hero-at:3] \
    has-[#hero-tab-4:is(:hover,:focus)]:[--hero-at:4] has-[#hero-slide-4:focus-within]:[--hero-at:4] \
    has-[#hero-tab-5:is(:hover,:focus)]:[--hero-at:5] has-[#hero-slide-5:focus-within]:[--hero-at:5] \
    has-[#hero-tab-6:is(:hover,:focus)]:[--hero-at:6] has-[#hero-slide-6:focus-within]:[--hero-at:6] \
    has-[#hero-tab-7:is(:hover,:focus)]:[--hero-at:7] has-[#hero-slide-7:focus-within]:[--hero-at:7]";

/// A slide's text, where it stands with the others in one place: it fades in on its
/// turn, as the cycle times it. While a slide is held, every other one is hidden, and
/// for visitors who prefer less motion only the first shows.
const PANEL: &str = "col-start-1 row-start-1 flex flex-col justify-center gap-6 [animation:hero-show_var(--hero-cycle)_var(--hero-ease)_infinite_both] \
    [--hero-line:hero-enter_var(--hero-cycle)_var(--hero-lines)_infinite_both] \
    group-has-[[data-hero-tab]:is(:hover,:focus),[data-hero-slide]:focus-within]/hero:invisible! \
    group-has-[[data-hero-tab]:is(:hover,:focus),[data-hero-slide]:focus-within]/hero:opacity-0! \
    group-has-[[data-hero-tab]:is(:hover,:focus),[data-hero-slide]:focus-within]/hero:[animation-play-state:paused] \
    group-has-[#hero-pause:checked]/hero:[animation-play-state:paused] \
    motion-reduce:animate-none";

/// The held slide's text, shown over the rule hiding the rest, its lines arriving afresh
/// as each slide is chosen. One per slide, written
/// out so Tailwind finds each class.
const PANEL_HELD: [&str; 8] = [
    "group-has-[#hero-tab-0:is(:hover,:focus),#hero-slide-0:focus-within]/hero:visible! group-has-[#hero-tab-0:is(:hover,:focus),#hero-slide-0:focus-within]/hero:opacity-100! group-has-[#hero-tab-0:is(:hover,:focus),#hero-slide-0:focus-within]/hero:[--hero-line:var(--animate-hero-arrive)]!",
    "group-has-[#hero-tab-1:is(:hover,:focus),#hero-slide-1:focus-within]/hero:visible! group-has-[#hero-tab-1:is(:hover,:focus),#hero-slide-1:focus-within]/hero:opacity-100! group-has-[#hero-tab-1:is(:hover,:focus),#hero-slide-1:focus-within]/hero:[--hero-line:var(--animate-hero-arrive)]!",
    "group-has-[#hero-tab-2:is(:hover,:focus),#hero-slide-2:focus-within]/hero:visible! group-has-[#hero-tab-2:is(:hover,:focus),#hero-slide-2:focus-within]/hero:opacity-100! group-has-[#hero-tab-2:is(:hover,:focus),#hero-slide-2:focus-within]/hero:[--hero-line:var(--animate-hero-arrive)]!",
    "group-has-[#hero-tab-3:is(:hover,:focus),#hero-slide-3:focus-within]/hero:visible! group-has-[#hero-tab-3:is(:hover,:focus),#hero-slide-3:focus-within]/hero:opacity-100! group-has-[#hero-tab-3:is(:hover,:focus),#hero-slide-3:focus-within]/hero:[--hero-line:var(--animate-hero-arrive)]!",
    "group-has-[#hero-tab-4:is(:hover,:focus),#hero-slide-4:focus-within]/hero:visible! group-has-[#hero-tab-4:is(:hover,:focus),#hero-slide-4:focus-within]/hero:opacity-100! group-has-[#hero-tab-4:is(:hover,:focus),#hero-slide-4:focus-within]/hero:[--hero-line:var(--animate-hero-arrive)]!",
    "group-has-[#hero-tab-5:is(:hover,:focus),#hero-slide-5:focus-within]/hero:visible! group-has-[#hero-tab-5:is(:hover,:focus),#hero-slide-5:focus-within]/hero:opacity-100! group-has-[#hero-tab-5:is(:hover,:focus),#hero-slide-5:focus-within]/hero:[--hero-line:var(--animate-hero-arrive)]!",
    "group-has-[#hero-tab-6:is(:hover,:focus),#hero-slide-6:focus-within]/hero:visible! group-has-[#hero-tab-6:is(:hover,:focus),#hero-slide-6:focus-within]/hero:opacity-100! group-has-[#hero-tab-6:is(:hover,:focus),#hero-slide-6:focus-within]/hero:[--hero-line:var(--animate-hero-arrive)]!",
    "group-has-[#hero-tab-7:is(:hover,:focus),#hero-slide-7:focus-within]/hero:visible! group-has-[#hero-tab-7:is(:hover,:focus),#hero-slide-7:focus-within]/hero:opacity-100! group-has-[#hero-tab-7:is(:hover,:focus),#hero-slide-7:focus-within]/hero:[--hero-line:var(--animate-hero-arrive)]!",
];

/// The row of every slide's photo side by side, plus a copy of the first, moved along by
/// the cycle, so each slides in from the right as the last leaves to the left; held where `--hero-at` says while a slide is chosen, and still when paused or
/// for visitors who prefer less motion.
const STRIP: &str = "flex h-full w-[calc((var(--hero-n)+1)*100%)] shrink-0 [animation:hero-shift_var(--hero-cycle)_var(--hero-ease)_infinite] [--hero-to:-100%] \
    group-has-[[data-hero-tab]:is(:hover,:focus),[data-hero-slide]:focus-within]/hero:[translate:calc(var(--hero-at)*-100%/(var(--hero-n)+1))_0]! \
    group-has-[[data-hero-tab]:is(:hover,:focus),[data-hero-slide]:focus-within]/hero:[animation-play-state:paused] \
    group-has-[#hero-pause:checked]/hero:[animation-play-state:paused] \
    motion-reduce:animate-none";

/// A full-viewport slideshow: each slide has its own photo or video, text and button,
/// and a tab along the bottom that shows it. The slides move on by themselves, one every
/// few seconds, with a bar filling under the current tab; hovering or focusing a tab
/// holds its slide, and the round button pauses them. All of it is Tailwind: no signals
/// and no script, so the page carries no state and links from it prefetch cleanly.
#[component]
pub async fn hero(
    slides: Vec<HeroSlide>,
    /// Accessible names for the row of tabs and the pause button.
    tabs_label: &str,
    pause_label: &str,
) -> Result<impl View> {
    let count = slides.len();
    // A copy of the first slide closes the strip, so the cycle starting over is seamless.
    let strip: Vec<(usize, HeroSlide)> = slides
        .iter()
        .cloned()
        .enumerate()
        .chain(slides.first().cloned().map(|first| (count, first)))
        .collect();
    let photos = strip;
    let panels: Vec<_> = slides
        .iter()
        .cloned()
        .enumerate()
        .map(|(index, slide)| {
            let timing = format!(
                "--hero-ease: {}; --hero-lines: {}",
                timing::shown(index, count),
                timing::arriving(index, count)
            );
            (index, slide, timing)
        })
        .collect();
    let timing = format!(
        "--hero-n: {count}; --hero-cycle: {}ms; --hero-strip: {}",
        count as u32 * timing::SLIDE_MS,
        timing::ease(count, false),
    );
    let tab_rail = slides.clone();

    Ok(view! {
        <section
            style=(timing)
            class=(class!(HELD, "group/hero relative isolate flex min-h-dvh w-full min-w-0 flex-col overflow-hidden bg-[#0b0d14] text-white"))
        >
            <div class="absolute inset-0 -z-10 overflow-clip">
                <div class=(STRIP) style="--hero-ease: var(--hero-strip)">
                    for (index, slide) in photos {
                        <div class="h-full min-w-0 flex-1" aria-hidden=(if index == count { Some("true") } else { None })>
                            if slide.code.is_empty() {
                                slide_photo(slide: slide, first: index == 0)
                            } else {
                                slide_code(
                                    code: &slide.code,
                                    language: &slide.code_language,
                                    timings: (index < count).then(|| typing_for(index, count, &slide.code)),
                                )
                            }
                        </div>
                    }
                </div>
            </div>
            <div class="absolute inset-0 -z-10 bg-linear-to-r from-black/85 via-black/50 to-black/10"></div>
            <div class="absolute inset-x-0 bottom-0 -z-10 h-1/2 bg-linear-to-t from-black/70 to-transparent"></div>

            <div class=(class!(CONTAINER, "grid flex-1 pt-28 pb-16"))>
                for (index, slide, timing) in panels {
                    <div
                        id=(format!("hero-slide-{index}"))
                        data-hero-slide=""
                        style=(timing)
                        class=(format!("{PANEL} {} {}", PANEL_HELD[index % PANEL_HELD.len()], if index == 0 { "" } else { "invisible opacity-0" }))
                    >
                        slide_copy(slide: slide)
                    </div>
                }
            </div>

            <div class=(class!(CONTAINER, "flex items-center gap-4 pb-10"))>
                <div aria-label=(tabs_label) role="group" class="flex min-w-0 flex-1 gap-2 overflow-x-auto pb-1">
                    for (index, slide) in tab_rail.into_iter().enumerate() {
                        slide_tab(
                            index: index,
                            label: &slide.tab_label,
                            timing: format!(
                                "--hero-ease: {}; --hero-fill: {}",
                                timing::shown(index, count),
                                timing::filling(index, count)
                            ),
                        )
                    }
                </div>
                pause_toggle(label: pause_label)
            </div>
        </section>
    })
}

/// When each line of a slide's code types out, after its indentation.
fn typing_for(index: usize, count: usize, code: &str) -> Vec<Option<(String, String)>> {
    let lengths: Vec<usize> = code
        .lines()
        .map(|line| line.trim_start().chars().count())
        .collect();
    timing::typing(index, count, &lengths)
}
