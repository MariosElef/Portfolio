use axum::{Router, extract::State, response::Response, routing::get};

use super::respond::found;
use crate::{services::github, state::AppState};

/// Mounted at `/activity`.
pub fn router() -> Router<AppState> {
    Router::new().route("/contributions", get(contributions))
}

/// The contribution calendar; 404 when GitHub is not configured.
async fn contributions(State(state): State<AppState>) -> Response {
    found(github::contributions(&state).await)
}
