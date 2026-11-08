use axum::{extract::State, response::Response};

use super::respond::found;
use crate::{services::site, state::AppState};

/// `GET /site`: the copy every page shares.
pub async fn one(State(state): State<AppState>) -> Response {
    found(site::site(&state).await)
}
