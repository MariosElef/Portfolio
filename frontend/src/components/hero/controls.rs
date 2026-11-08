use topcoat::{
    Result,
    icon::{icon, iconify::iconify_icon},
    view::{View, attributes, component, view},
};

/// The held tab, white over the rule darkening the rest. One per slide, written out so
/// Tailwind finds each class.
const TAB_HELD: [&str; 8] = [
    "group-has-[#hero-tab-0:is(:hover,:focus),#hero-slide-0:focus-within]/hero:bg-white! group-has-[#hero-tab-0:is(:hover,:focus),#hero-slide-0:focus-within]/hero:text-ink-950!",
    "group-has-[#hero-tab-1:is(:hover,:focus),#hero-slide-1:focus-within]/hero:bg-white! group-has-[#hero-tab-1:is(:hover,:focus),#hero-slide-1:focus-within]/hero:text-ink-950!",
    "group-has-[#hero-tab-2:is(:hover,:focus),#hero-slide-2:focus-within]/hero:bg-white! group-has-[#hero-tab-2:is(:hover,:focus),#hero-slide-2:focus-within]/hero:text-ink-950!",
    "group-has-[#hero-tab-3:is(:hover,:focus),#hero-slide-3:focus-within]/hero:bg-white! group-has-[#hero-tab-3:is(:hover,:focus),#hero-slide-3:focus-within]/hero:text-ink-950!",
    "group-has-[#hero-tab-4:is(:hover,:focus),#hero-slide-4:focus-within]/hero:bg-white! group-has-[#hero-tab-4:is(:hover,:focus),#hero-slide-4:focus-within]/hero:text-ink-950!",
    "group-has-[#hero-tab-5:is(:hover,:focus),#hero-slide-5:focus-within]/hero:bg-white! group-has-[#hero-tab-5:is(:hover,:focus),#hero-slide-5:focus-within]/hero:text-ink-950!",
    "group-has-[#hero-tab-6:is(:hover,:focus),#hero-slide-6:focus-within]/hero:bg-white! group-has-[#hero-tab-6:is(:hover,:focus),#hero-slide-6:focus-within]/hero:text-ink-950!",
    "group-has-[#hero-tab-7:is(:hover,:focus),#hero-slide-7:focus-within]/hero:bg-white! group-has-[#hero-tab-7:is(:hover,:focus),#hero-slide-7:focus-within]/hero:text-ink-950!",
];

/// A slide's tab: white with dark text on its slide's turn, as the cycle times it
/// (`--hero-ease`), with a red bar filling along its foot for the slide's time
/// (`--hero-fill`). Hovering or focusing it holds its slide.
#[component]
pub async fn slide_tab(index: usize, label: &str, timing: String) -> Result<impl View> {
    let tab = format!(
        "relative flex h-[76px] w-60 shrink-0 cursor-pointer items-center gap-3 overflow-hidden rounded-sm px-4 text-left text-sm font-bold \
         {} [animation:hero-tab_var(--hero-cycle)_var(--hero-ease)_infinite_both] \
         group-has-[[data-hero-tab]:is(:hover,:focus),[data-hero-slide]:focus-within]/hero:bg-black/45! group-has-[[data-hero-tab]:is(:hover,:focus),[data-hero-slide]:focus-within]/hero:text-white! group-has-[[data-hero-tab]:is(:hover,:focus),[data-hero-slide]:focus-within]/hero:[animation-play-state:paused] {} \
         group-has-[#hero-pause:checked]/hero:[animation-play-state:paused] \
         focus-visible:ring-3 focus-visible:ring-white/60 focus-visible:outline-none motion-reduce:animate-none",
        if index == 0 {
            "bg-white text-ink-950"
        } else {
            "bg-black/45 text-white"
        },
        TAB_HELD[index % TAB_HELD.len()],
    );
    // Hidden while a slide is held; still while paused.
    let bar = "absolute bottom-0 left-0 h-1 w-0 bg-error [animation:hero-progress_var(--hero-cycle)_var(--hero-fill)_infinite_both] \
         group-has-[[data-hero-tab]:is(:hover,:focus),[data-hero-slide]:focus-within]/hero:invisible group-has-[#hero-pause:checked]/hero:[animation-play-state:paused] motion-reduce:hidden";
    Ok(view! {
        <button
            type="button"
            id=(format!("hero-tab-{index}"))
            data-hero-tab=""
            aria-controls=(format!("hero-slide-{index}"))
            style=(timing)
            class=(tab)
        >
            <span class="min-w-0 flex-1 truncate">(label)</span>
            icon(data: iconify_icon!("lucide:chevron-right"), attrs: attributes! { class="size-4 shrink-0" })
            <span aria-hidden="true" class=(bar)></span>
        </button>
    })
}

/// The round button that pauses the slides and plays them again: a checkbox, so it keeps
/// its state without a script, and is announced as checked while paused.
#[component]
pub async fn pause_toggle(label: &str) -> Result<impl View> {
    Ok(view! {
        <label class="grid size-11 shrink-0 cursor-pointer place-items-center rounded-full bg-black/50 backdrop-blur-md hover:bg-black/70 has-focus-visible:ring-3 has-focus-visible:ring-white/60 sm:-mr-6 lg:-mr-12">
            <input id="hero-pause" type="checkbox" aria-label=(label) class="sr-only">
            icon(data: iconify_icon!("lucide:pause"), attrs: attributes! { class="size-4 group-has-[#hero-pause:checked]/hero:hidden" })
            icon(data: iconify_icon!("lucide:play"), attrs: attributes! { class="hidden size-4 group-has-[#hero-pause:checked]/hero:block" })
        </label>
    })
}
