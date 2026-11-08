use axum::{
    Json, Router,
    body::Body,
    extract::State,
    http::{HeaderMap, StatusCode, header::CONTENT_TYPE},
    middleware::from_fn_with_state,
    response::{IntoResponse, Response},
    routing::post,
};

use super::respond::Internal;
use crate::{
    middleware::admin::require_token,
    services::media::{self, UploadError},
    state::AppState,
};

/// Mounted at `/media`. Uploading needs the admin token.
pub fn router(state: &AppState) -> Router<AppState> {
    Router::new()
        .route("/", post(upload))
        .layer(from_fn_with_state(state.admin_token.clone(), require_token))
}

async fn upload(State(state): State<AppState>, headers: HeaderMap, body: Body) -> Response {
    let content_type = headers
        .get(CONTENT_TYPE)
        .and_then(|value| value.to_str().ok())
        .unwrap_or_default();

    match media::upload(&state, content_type, body).await {
        Ok(uploaded) => (StatusCode::CREATED, Json(uploaded)).into_response(),
        Err(UploadError::UnsupportedType) => (
            StatusCode::UNSUPPORTED_MEDIA_TYPE,
            "expected video/mp4, video/webm, image/jpeg, image/png, image/webp or image/avif",
        )
            .into_response(),
        Err(UploadError::Empty) => (StatusCode::BAD_REQUEST, "empty body").into_response(),
        Err(UploadError::TooLarge) => StatusCode::PAYLOAD_TOO_LARGE.into_response(),
        Err(UploadError::Failed(error)) => Internal(error).into_response(),
    }
}
