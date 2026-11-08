use std::collections::BTreeMap;

use topcoat::{
    Result,
    view::{View, component, view},
};

use super::kinds::KINDS;
use crate::types::content::LegendEntry;

/// What each kind of line means, as the diagram explains it, for the kinds it uses, in
/// the table's order. On wide screens, one even row: a column per kind.
#[component]
pub async fn legend(
    used: Vec<String>,
    explained: &BTreeMap<String, LegendEntry>,
) -> Result<impl View> {
    let entries: Vec<_> = KINDS
        .iter()
        .filter(|(kind, _)| used.iter().any(|used| used == kind))
        .filter_map(|(kind, style)| {
            let entry = explained.get(*kind)?;
            Some((style.border, entry.label.clone(), entry.detail.clone()))
        })
        .collect();
    Ok(view! {
        <ul style=(format!("--kinds: {}", entries.len())) class="grid gap-x-8 gap-y-4 sm:grid-cols-2 xl:grid-cols-[repeat(var(--kinds),minmax(0,1fr))] xl:gap-x-6">
            for (border, label, detail) in entries {
                <li class="flex flex-col gap-1.5">
                    <span class="inline-flex items-center gap-2 text-xs font-bold">
                        <span aria-hidden="true" class=(format!("w-6 border-t-2 {border}"))></span>
                        (label)
                    </span>
                    <span class="text-xs leading-relaxed text-muted-foreground">(detail)</span>
                </li>
            }
        </ul>
    })
}
