use topcoat::{
    Result,
    view::{View, component, view},
};

use crate::types::content::MediaRef;

/// Images two to a row, cropped to the same shape.
#[component]
pub async fn gallery_block(images: Vec<MediaRef>) -> Result<impl View> {
    Ok(view! {
        <div class="grid gap-3 sm:grid-cols-2">
            for image in images {
                <img src=(image.url) alt="" loading="lazy" class="aspect-video w-full rounded-xl border border-border object-cover">
            }
        </div>
    })
}
