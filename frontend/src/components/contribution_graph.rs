use topcoat::{
    Result,
    view::{StaticClass, View, class, component, view},
};

use super::tone::Color;

/// One day square: how busy it was (0–4) and the text a hover shows.
pub struct GraphDay {
    pub weekday: u8,
    pub level: u8,
    pub label: String,
}

/// One column: its days, and the month to print above it, if any.
pub struct GraphWeek {
    pub month: String,
    pub days: Vec<GraphDay>,
}

/// Each level is the colour at a stronger mix, read through `--tone`.
fn shade(level: u8) -> StaticClass {
    match level {
        0 => class!("bg-accent"),
        1 => class!("bg-(--tone)/30"),
        2 => class!("bg-(--tone)/55"),
        3 => class!("bg-(--tone)/80"),
        _ => class!("bg-(--tone)"),
    }
}

const CELL: StaticClass = class!("aspect-square w-full rounded-[3px]");

/// The colours from fewest contributions to most, labelled `less` and `more` at its ends.
#[component]
async fn scale(less: &str, more: &str) -> Result<impl View> {
    Ok(view! {
        <div class="flex items-center justify-end gap-1 text-xs text-muted-foreground">
            <span class="mr-1">(less)</span>
            for level in 0..5u8 {
                <span class=(class!("size-3 rounded-[3px]", shade(level)))></span>
            }
            <span class="ml-1">(more)</span>
        </div>
    })
}

/// A year of activity: a column per week, a row per weekday, stronger by how much
/// happened. It stretches to its container's width. All text is passed in, already
/// translated.
#[component]
pub async fn contribution_graph(
    weeks: Vec<GraphWeek>,
    /// Seven labels, Sunday first; empty ones print nothing (Mon, Wed, Fri is usual).
    weekdays: Vec<String>,
    less: &str,
    more: &str,
    /// Which of the seven colours fills the squares.
    #[default(Color::Primary)]
    color: Color,
) -> Result<impl View> {
    // One grid, filled a column at a time: a month label then seven days per week,
    // after a first column of weekday labels. Squares take their height from their
    // width, so the labels line up with them.
    let grid = format!(
        "grid-template-columns: 2rem repeat({}, minmax(0, 1fr)); \
         grid-template-rows: auto repeat(7, auto)",
        weeks.len()
    );

    Ok(view! {
        <div class=(class!("flex flex-col gap-3", color.tone()))>
            <div class="overflow-x-auto pb-1" dir="rtl">
                // Scrolled from the right on narrow screens, so the latest weeks show first.
                <div dir="ltr" class="grid min-w-[640px] grid-flow-col gap-[3px]" style=(grid)>
                    <span></span>
                    for label in weekdays {
                        <span class="flex items-center pr-2 text-[11px] leading-none text-muted-foreground">(label)</span>
                    }
                    for week in weeks {
                        <span class="h-5 overflow-visible text-[11px] whitespace-nowrap text-muted-foreground">(week.month)</span>
                        for slot in 0..7u8 {
                            match week.days.iter().find(|d| d.weekday == slot) {
                                Some(day) => <span title=(day.label.clone()) class=(class!(CELL, shade(day.level)))></span>,
                                None => <span class=(CELL)></span>,
                            }
                        }
                    }
                </div>
            </div>
            scale(less: less, more: more)
        </div>
    })
}
