use axum::{Json, Router, routing::get};
use serde_json::{Value, json};

use crate::state::AppState;

mod activity;
mod home;
mod media;
mod messages;
mod pages;
mod profile;
mod projects;
mod respond;
mod site;

/// One router per domain, each owning its own paths, as in the mc2 APIs.
pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/health", get(health))
        .route("/site", get(site::one))
        .nest("/projects", projects::router())
        .nest("/pages", pages::router())
        .nest("/home", home::router())
        .nest("/profile", profile::router())
        .nest("/activity", activity::router())
        .nest("/messages", messages::router(&state))
        .nest("/media", media::router(&state))
        .with_state(state)
}

async fn health() -> Json<Value> {
    Json(json!({ "status": "ok" }))
}
