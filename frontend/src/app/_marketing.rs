//! A group, not a URL segment: its pages serve `/contact`.

use topcoat::{
    Result,
    router::{Slot, layout},
    view::{View, view},
};

use crate::components::layout::page::{PagePadding, PageWidth, page};

pub mod contact;

#[layout]
async fn marketing(slot: Slot<'_>) -> Result<impl View> {
    Ok(view! {
        page(width: PageWidth::Wide, padding: PagePadding::Standard, (slot))
    })
}
