use topcoat::{
    Result,
    view::{View, component, view},
};

use super::{
    brand_logo::{MarkSize, brand_logo},
    tile::{TileSize, TileTone, tile},
};
use crate::types::content::MediaRef;

/// A project's identity: its logo, or without one its initials on its own colour.
#[component]
pub async fn project_mark(
    logo: Option<MediaRef>,
    initials: &str,
    #[default] tone: TileTone,
    #[default] size: MarkSize,
) -> Result<impl View> {
    let tile_size = match size {
        MarkSize::Sm => TileSize::Md,
        MarkSize::Md | MarkSize::Lg => TileSize::Lg,
    };
    Ok(view! {
        match logo {
            Some(logo) => brand_logo(src: &logo.url, size: size),
            None => tile(initials: initials, tone: tone, size: tile_size, decorative: true),
        }
    })
}
