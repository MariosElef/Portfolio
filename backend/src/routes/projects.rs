use axum::{
    Json, Router,
    extract::{Path, State},
    response::Response,
    routing::get,
};

use super::respond::{Internal, found};
use crate::{
    models::{Block, Project},
    services::projects,
    state::AppState,
};

/// Mounted at `/projects`.
pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(list))
        .route("/{slug}", get(one))
        .route("/{slug}/story", get(story))
}

async fn list(State(state): State<AppState>) -> Result<Json<Vec<Project>>, Internal> {
    Ok(Json(projects::projects(&state).await?))
}

async fn one(State(state): State<AppState>, Path(slug): Path<String>) -> Response {
    found(projects::project(&state, &slug).await)
}

async fn story(
    State(state): State<AppState>,
    Path(slug): Path<String>,
) -> Result<Json<Vec<Block>>, Internal> {
    Ok(Json(projects::case_study(&state, &slug).await?))
}
