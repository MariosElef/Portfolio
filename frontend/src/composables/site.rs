use topcoat::context::{Cx, memoize};

use super::api::api;
use crate::types::site::Site;

/// The copy every page shares, fetched once per page however many parts of it ask (the
/// layout, the bar, the footer, a section's link). Without it (the backend down, the
/// document missing) it is empty rather than an error: the shell must render whatever
/// else fails.
#[memoize]
async fn shared(cx: &Cx) -> Site {
    api(cx).site().await.ok().flatten().unwrap_or_default()
}

pub async fn site(cx: &Cx) -> Site {
    shared(cx).await.clone()
}
