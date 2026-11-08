use topcoat::context::{Cx, app_context};

use crate::services::client::ApiClient;

/// The backend client registered on the router.
pub fn api(cx: &Cx) -> &ApiClient {
    app_context(cx)
}
