use topcoat::{
    Result,
    view::{View, component, view},
};

use super::TimelineLabels;
use crate::{
    components::{
        layout::stack::{StackGap, stack},
        lists::ruled::{ruled_item, ruled_list},
        media::brand_logo::{MarkSize, brand_logo},
        surfaces::disclosure::disclosure,
        typography::{
            eyebrow::{EyebrowLevel, eyebrow},
            rich_text::rich_text,
        },
    },
    types::content::Experience,
};

/// A role: the company's logo, the role, the company in its colour with the period, one
/// line of summary, and what I did and learned behind "Read more".
#[component]
pub async fn role_summary(role: Experience, labels: &TimelineLabels) -> Result<impl View> {
    let columns = [
        (labels.did.clone(), role.did),
        (labels.learned.clone(), role.learned),
    ];
    let company = format!("color: {}", role.color);
    Ok(view! {
        <div class="flex flex-col gap-1">
            <div class="flex items-center gap-4">
                if let Some(logo) = role.logo {
                    brand_logo(src: &logo.url, size: MarkSize::Sm)
                }
                <h3 class="text-2xl leading-tight font-bold tracking-tight sm:text-3xl">(role.role)</h3>
            </div>
            <p class="font-bold">
                <span style=(company)>(role.company)</span>
                <span class="font-normal text-muted-foreground">" · " (role.period)</span>
            </p>
            if !role.summary.is_empty() {
                <p class="mt-2 max-w-2xl text-lg text-muted-foreground">(role.summary)</p>
            }
            <div class="mt-3">
                disclosure(summary: &labels.read_more,
                    <div class="mt-5 grid max-w-3xl gap-8 sm:grid-cols-2">
                        for (label, items) in columns {
                            if !items.is_empty() {
                                <section>
                                    stack(gap: StackGap::Tight,
                                        eyebrow(level: EyebrowLevel::H4, (label))
                                        ruled_list(
                                            for item in items {
                                                ruled_item(small: true, rich_text(text: &item))
                                            }
                                        )
                                    )
                                </section>
                            }
                        }
                    </div>
                )
            </div>
        </div>
    })
}
