use topcoat::{
    Result,
    view::{Child, View, component, view},
};

/// A short status with a green dot, such as being open to new work.
#[component]
pub async fn status_badge(#[default] child: Child<'_>) -> Result<impl View> {
    Ok(view! {
        <p class="inline-flex w-fit items-center gap-2 rounded-full border border-white/10 bg-white/5 px-3 py-1 text-sm font-medium">
            <span aria-hidden="true" class="relative flex size-2">
                <span class="absolute inline-flex size-full animate-ping rounded-full bg-success/60 motion-reduce:hidden"></span>
                <span class="relative inline-flex size-2 rounded-full bg-success"></span>
            </span>
            (child)
        </p>
    })
}
