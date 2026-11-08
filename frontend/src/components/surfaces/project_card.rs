use topcoat::{
    Result,
    view::{Child, View, attributes, component, view},
};

use super::card::{card, card_content, card_description, card_footer, card_header, card_title};
use crate::{
    components::{
        actions::button_row::button_row,
        layout::{spread::spread, stack::stack},
        media::{brand_logo::MarkSize, project_mark::project_mark, tile::TileTone},
        typography::tag_list::tag_list,
    },
    types::content::MediaRef,
};

/// One project in a grid: its mark, name, description, stack, links and an action.
#[component]
pub async fn project_card(
    #[default] initials: &str,
    #[default] tone: TileTone,
    #[default] logo: Option<MediaRef>,
    #[default] name: &str,
    #[default] description: &str,
    #[default] tags: Vec<&str>,
    #[default] links: Child<'_>,
    #[default] action: Child<'_>,
) -> Result<impl View> {
    Ok(view! {
        card(
            attrs: attributes! { class="h-full" },
            // The mark on the left, the name at the top right.
            card_header(
                spread(
                    project_mark(logo: logo, initials: initials, tone: tone, size: MarkSize::Md)
                    card_title(
                        attrs: attributes! { class="font-display text-right text-xl font-extrabold tracking-[-0.03em]" },
                        (name)
                    )
                )
            )
            card_content(
                stack(
                    card_description((description))
                    tag_list(tags: tags)
                )
            )
            card_footer(
                attrs: attributes! { class="mt-auto justify-between gap-3" },
                button_row((links))
                (action)
            )
        )
    })
}
