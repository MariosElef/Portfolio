use axum::{
    Json, Router,
    extract::State,
    http::{HeaderMap, StatusCode, header::USER_AGENT},
    middleware::from_fn_with_state,
    response::{IntoResponse, Response},
    routing::{get, post},
};

use super::respond::Internal;
use crate::{
    middleware::rate_limit::{self, RateLimit},
    models::{Challenge, PostedMessage},
    services::{
        challenge,
        messages::{self, MessageError},
    },
    state::AppState,
};

/// Five messages per visitor every ten minutes: plenty for a person, little for a flood.
const LIMIT: RateLimit = RateLimit {
    name: "messages",
    max: 5,
    window_seconds: 10 * 60,
};

/// Mounted at `/messages`. Sending is rate limited; asking for a question is not.
pub fn router(state: &AppState) -> Router<AppState> {
    Router::new()
        .route("/", post(send))
        .route_layer(from_fn_with_state(
            (state.clone(), LIMIT),
            rate_limit::limit,
        ))
        .route("/challenge", get(ask))
}

/// `GET /messages/challenge`: a question for the form to ask.
async fn ask(State(state): State<AppState>) -> Json<Challenge> {
    Json(challenge::ask(&state.challenge_key))
}

async fn send(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(posted): Json<PostedMessage>,
) -> Response {
    let user_agent = headers
        .get(USER_AGENT)
        .and_then(|value| value.to_str().ok());

    match messages::send(&state, posted, user_agent).await {
        Ok(()) => StatusCode::CREATED.into_response(),
        Err(MessageError::Invalid) => StatusCode::UNPROCESSABLE_ENTITY.into_response(),
        Err(MessageError::Unanswered) => StatusCode::FORBIDDEN.into_response(),
        Err(MessageError::Failed(error)) => Internal(error).into_response(),
    }
}
