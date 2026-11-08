//! How the routes answer: the JSON, a 404, or a bare 500.

use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};

/// 200 with the JSON, 404 when absent, 500 on failure.
pub fn found<T: serde::Serialize>(result: anyhow::Result<Option<T>>) -> Response {
    match result {
        Ok(Some(value)) => Json(value).into_response(),
        Ok(None) => StatusCode::NOT_FOUND.into_response(),
        Err(error) => Internal(error).into_response(),
    }
}

/// A failure the client can do nothing about: logged, and answered with a bare 500.
pub struct Internal(pub anyhow::Error);

impl IntoResponse for Internal {
    fn into_response(self) -> Response {
        tracing::error!(error = %self.0, "request failed");
        StatusCode::INTERNAL_SERVER_ERROR.into_response()
    }
}

impl<E: Into<anyhow::Error>> From<E> for Internal {
    fn from(error: E) -> Self {
        Self(error.into())
    }
}
