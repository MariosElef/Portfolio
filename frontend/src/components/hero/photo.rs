use topcoat::{
    Result,
    view::{View, component, view},
};

use crate::types::content::HeroSlide;

/// A slide's photo, or its video looping silently, which gives way to its poster for
/// visitors who prefer reduced motion. Only the first slide's image loads eagerly.
#[component]
pub async fn slide_photo(slide: HeroSlide, first: bool) -> Result<impl View> {
    let video = slide.media.clone().filter(|m| m.is_video());
    let image = slide.media.filter(|m| !m.is_video());
    let poster = slide.poster.map(|p| p.url);
    Ok(view! {
        if let Some(video) = video {
            <video
                class="size-full object-cover motion-reduce:hidden"
                autoplay=""
                muted=""
                loop=""
                playsinline=""
                preload="metadata"
                poster=(poster.clone())
                aria-hidden="true"
            >
                <source src=(video.url) type=(video.content_type)>
            </video>
            if let Some(poster) = poster {
                <img src=(poster) alt="" class="hidden size-full object-cover motion-reduce:block">
            }
        } else if let Some(image) = image {
            <img
                src=(image.url)
                alt=""
                class="size-full object-cover"
                fetchpriority=(if first { "high" } else { "low" })
            >
        }
    })
}
