use topcoat::{
    Result,
    view::{Child, View, attributes, class, component, view},
};

use super::{
    card::{card, card_content, card_description, card_footer, card_title},
    skeleton::skeleton,
    tile::{SQUIRCLE, TileSize, TileTone, tile},
};

/// One project in a grid: tile, name, description, stack, links and an action.
#[component]
pub async fn project_card(
    #[default] initials: &str,
    #[default] tone: TileTone,
    #[default] name: &str,
    #[default] description: &str,
    #[default] tags: Vec<&str>,
    #[default] loading: bool,
    #[default] links: Child<'_>,
    #[default] action: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        card(
            attrs: attributes! { aria-hidden=(loading.then_some("true")) class="h-full" },
            <div class="flex items-center gap-3 px-6">
                if loading {
                    skeleton(attrs: attributes! { class=(class!("size-[40px]", SQUIRCLE)) })
                    skeleton(attrs: attributes! { class="h-4 w-2/5" })
                } else {
                    tile(initials: initials, tone: tone, size: TileSize::Lg, decorative: true)
                    card_title(attrs: attributes! { class="flex-1" }, (name))
                }
            </div>
            if loading {
                card_content(
                    attrs: attributes! { class="space-y-2" },
                    skeleton(attrs: attributes! { class="h-3.5 w-full" })
                    skeleton(attrs: attributes! { class="h-3.5 w-3/4" })
                )
                card_footer(
                    attrs: attributes! { class="mt-auto justify-between gap-3" },
                    skeleton(attrs: attributes! { class="size-7 rounded-full" })
                    skeleton(attrs: attributes! { class="h-7 w-16 rounded-full" })
                )
            } else {
                card_content(
                    attrs: attributes! { class="flex flex-col gap-4" },
                    card_description((description))
                    if !tags.is_empty() {
                        <ul class="flex flex-wrap gap-2">
                            for tag in tags {
                                <li class="rounded-full bg-muted px-2.5 py-1 text-xs text-muted-foreground">
                                    (tag)
                                </li>
                            }
                        </ul>
                    }
                )
                card_footer(
                    attrs: attributes! { class="mt-auto justify-between gap-3" },
                    <div class="flex items-center gap-1">(links)</div>
                    (action)
                )
            }
        )
    })
}
