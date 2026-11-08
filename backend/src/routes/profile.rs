use axum::{Json, Router, extract::State, routing::get};

use super::respond::Internal;
use crate::{
    models::{Experience, Skill, StackItem},
    services::profile,
    state::AppState,
};

/// Mounted at `/profile`.
pub fn router() -> Router<AppState> {
    Router::new()
        .route("/experience", get(experience))
        .route("/stack", get(stack))
        .route("/skills", get(skills))
}

async fn experience(State(state): State<AppState>) -> Result<Json<Vec<Experience>>, Internal> {
    Ok(Json(profile::experience(&state).await?))
}

async fn stack(State(state): State<AppState>) -> Result<Json<Vec<StackItem>>, Internal> {
    Ok(Json(profile::stack(&state).await?))
}

async fn skills(State(state): State<AppState>) -> Result<Json<Vec<Skill>>, Internal> {
    Ok(Json(profile::skills(&state).await?))
}
