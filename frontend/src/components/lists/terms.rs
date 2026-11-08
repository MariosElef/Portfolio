use topcoat::{
    Result,
    view::{Child, View, class, component, view},
};

use super::ruled::{CLOSED, RULED};

/// Terms and what they stand for, a row each, separated by hairlines.
#[component]
pub async fn term_list(#[default] child: Child<'_>) -> Result<impl View> {
    Ok(view! {
        <dl class="flex flex-col">(child)</dl>
    })
}

/// A row of a [`term_list`]: the term, with a note under it when there is one, and beside
/// it on wide screens what it stands for.
#[component]
pub async fn term_row(
    term: &str,
    #[default] note: &str,
    #[default] child: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        <div class=(class!(RULED, CLOSED, "grid gap-x-8 gap-y-2 py-4 md:grid-cols-[14rem_minmax(0,1fr)]"))>
            <dt class="flex flex-col gap-0.5">
                <span class="text-sm font-bold">(term)</span>
                if !note.is_empty() {
                    <span class="text-xs text-muted-foreground italic">(note)</span>
                }
            </dt>
            <dd class="flex flex-wrap items-center gap-x-5 gap-y-2 text-sm">(child)</dd>
        </div>
    })
}
