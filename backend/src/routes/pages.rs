use axum::{
    Router,
    extract::{Path, State},
    response::Response,
    routing::get,
};

use super::respond::found;
use crate::{services::pages, state::AppState};

/// Mounted at `/pages`.
pub fn router() -> Router<AppState> {
    Router::new().route("/{slug}", get(one))
}

async fn one(State(state): State<AppState>, Path(slug): Path<String>) -> Response {
    found(pages::page(&state, &slug).await)
}
