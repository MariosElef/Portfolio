use topcoat::{
    context::Cx,
    router::request::{method, original_method},
    view::SuspenseMode,
};

/// The request's context with how its loading parts arrive. A link followed in the page
/// (the runtime's navigation: a `POST` it rewrites to a `GET`) streams: the page shows at
/// once with skeletons, each filled in as its part loads. Anything else (a first visit,
/// a reload, a search engine) waits for the whole page, so it works without JavaScript.
pub fn with_suspense_mode(cx: &Cx) -> Cx {
    if is_navigation(cx) {
        cx.with(SuspenseMode::Stream)
    } else {
        cx.with(SuspenseMode::Wait)
    }
}

fn is_navigation(cx: &Cx) -> bool {
    method(cx).as_str() == "GET" && original_method(cx).as_str() == "POST"
}
