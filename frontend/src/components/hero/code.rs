use topcoat::{
    Result,
    view::{View, component, view},
};

use crate::utils::highlight::{Kind, highlight};

fn colour(kind: Kind) -> &'static str {
    match kind {
        Kind::Plain => "text-[#abb2bf]",
        Kind::Keyword => "font-bold text-[#c678dd]",
        Kind::Function => "text-[#61afef]",
        Kind::Type => "text-[#e5c07b]",
        Kind::Name => "text-[#e06c75]",
        Kind::Text => "text-[#98c379]",
        Kind::Number => "text-[#d19a66]",
        Kind::Comment => "text-[#7f848e]",
        Kind::Operator => "text-[#56b6c2]",
    }
}

/// A slide's code, plain on the dark, typed out a character at a time on the slide's
/// turn, with a cursor. `timings` gives each line's reveal and cursor (see
/// `timing::typing`); without them (the copy of the first slide that closes the strip)
/// the code is not yet typed, as the first slide starts.
#[component]
pub async fn slide_code(
    code: &str,
    language: &str,
    timings: Option<Vec<Option<(String, String)>>>,
) -> Result<impl View> {
    let lines: Vec<_> = highlight(code, language)
        .into_iter()
        .enumerate()
        .map(|(j, runs)| {
            let text: String = runs.iter().map(|(text, _)| text.as_str()).collect();
            let indent = text.len() - text.trim_start().len();
            let typed = text.chars().count() - indent;
            let timing = timings
                .as_ref()
                .and_then(|lines| lines.get(j).cloned().flatten());
            let width = if timings.is_some() { typed } else { 0 };
            (
                " ".repeat(indent),
                strip_indent(runs, indent),
                width,
                timing,
            )
        })
        .collect();

    // While a slide is held its code shows in full, without the cursor; while paused it
    // stops where it is.
    let reveal = "inline-block overflow-hidden align-top whitespace-pre w-[var(--w)] [animation:hero-type_var(--hero-cycle)_var(--hero-ease)_infinite_both] \
         group-has-[[data-hero-tab]:is(:hover,:focus),[data-hero-slide]:focus-within]/hero:[width:var(--w)]! group-has-[[data-hero-tab]:is(:hover,:focus),[data-hero-slide]:focus-within]/hero:[animation-play-state:paused] \
         group-has-[#hero-pause:checked]/hero:[animation-play-state:paused] motion-reduce:animate-none";
    let cursor = "ml-0.5 inline-block h-[1.2em] w-[0.14em] translate-y-[0.15em] bg-[#528bff] opacity-0 [animation:hero-caret_var(--hero-cycle)_var(--hero-ease)_infinite_both] \
         group-has-[[data-hero-tab]:is(:hover,:focus),[data-hero-slide]:focus-within]/hero:opacity-0! group-has-[[data-hero-tab]:is(:hover,:focus),[data-hero-slide]:focus-within]/hero:[animation-play-state:paused] \
         group-has-[#hero-pause:checked]/hero:[animation-play-state:paused] motion-reduce:hidden";

    Ok(view! {
        <div aria-hidden="true" class="flex size-full items-center justify-end bg-[#0b0d14] pr-[7vw] max-md:hidden">
            <pre class="font-mono text-[clamp(13px,1.45vw,30px)] leading-[1.65]">
                for (indent, runs, width, timing) in lines {
                    <div class="flex h-[1.65em]">
                        <span class="whitespace-pre">(indent)</span>
                        if let Some((type_ease, cursor_ease)) = timing {
                            <span class=(reveal) style=(format!("--w: {width}ch; --hero-ease: {type_ease}"))>
                                for (text, kind) in runs {
                                    <span class=(colour(kind))>(text)</span>
                                }
                            </span>
                            <span class=(cursor) style=(format!("--hero-ease: {cursor_ease}"))></span>
                        } else {
                            <span class="inline-block overflow-hidden align-top whitespace-pre" style=(format!("width: {width}ch"))>
                                for (text, kind) in runs {
                                    <span class=(colour(kind))>(text)</span>
                                }
                            </span>
                        }
                    </div>
                }
            </pre>
        </div>
    })
}

/// The runs of a line after its first `indent` characters, which are spaces.
fn strip_indent(runs: Vec<(String, Kind)>, mut indent: usize) -> Vec<(String, Kind)> {
    runs.into_iter()
        .filter_map(|(text, kind)| {
            let skip = indent.min(text.len());
            indent -= skip;
            let rest = text[skip..].to_owned();
            (!rest.is_empty()).then_some((rest, kind))
        })
        .collect()
}
