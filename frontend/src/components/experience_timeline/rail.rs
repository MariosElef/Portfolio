use topcoat::{
    Result,
    view::{View, component, view},
};

/// A role's place in time: its year, and its dot on the line that runs down to the next
/// role (the last stops at its dot). The dot is the company's colour; with an accent, it
/// is split between both.
#[component]
pub async fn rail(year: &str, color: &str, accent: &str, last: bool) -> Result<impl View> {
    let dot = if accent.is_empty() {
        format!("background: {color}")
    } else {
        format!("background: linear-gradient(135deg, {color} 50%, {accent} 50%)")
    };
    Ok(view! {
        <p class="pt-1 text-right text-sm font-bold text-muted-foreground tabular-nums">(year)</p>
        <div class="relative flex justify-center">
            if !last {
                <span aria-hidden="true" class="absolute top-3 bottom-0 w-px bg-border"></span>
            }
            <span aria-hidden="true" style=(dot) class="relative mt-2 size-3.5 rounded-full ring-4 ring-background"></span>
        </div>
    })
}
