use topcoat::{
    Result,
    view::{View, component, view},
};

/// A cut-out portrait standing straight on the page, the full height of its place, over
/// a soft glow, fading out at the bottom.
#[component]
pub async fn portrait(src: &str, alt: &str) -> Result<impl View> {
    Ok(view! {
        <div aria-hidden="true" class="absolute top-[18%] left-1/2 -z-10 size-[80%] -translate-x-1/2 rounded-full bg-brand-600/30 blur-[100px]"></div>
        <div aria-hidden="true" class="absolute top-[38%] left-[60%] -z-10 size-[45%] -translate-x-1/2 rounded-full bg-plum-600/25 blur-[90px]"></div>
        <img
            src=(src)
            alt=(alt)
            class="absolute inset-0 size-full object-contain object-bottom [mask-image:linear-gradient(to_bottom,#000_72%,transparent)]"
        >
    })
}
