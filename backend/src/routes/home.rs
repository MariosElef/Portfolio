use axum::{Json, Router, extract::State, routing::get};

use super::respond::Internal;
use crate::{models::HeroSlide, services::hero, state::AppState};

/// Mounted at `/home`.
pub fn router() -> Router<AppState> {
    Router::new().route("/hero", get(hero))
}

async fn hero(State(state): State<AppState>) -> Result<Json<Vec<HeroSlide>>, Internal> {
    Ok(Json(hero::hero(&state).await?))
}
