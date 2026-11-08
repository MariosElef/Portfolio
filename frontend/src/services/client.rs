use std::fmt;

use reqwest::{Client, StatusCode, Url};
use serde::de::DeserializeOwned;

/// Why a backend call failed.
#[derive(Debug)]
pub enum ApiError {
    BadUrl(String),
    Http(reqwest::Error),
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::BadUrl(url) => write!(f, "API_URL is not a usable base URL: {url}"),
            Self::Http(error) => write!(f, "backend request failed: {error}"),
        }
    }
}

impl std::error::Error for ApiError {}

impl From<reqwest::Error> for ApiError {
    fn from(error: reqwest::Error) -> Self {
        Self::Http(error)
    }
}

pub type ApiResult<T> = Result<T, ApiError>;

/// The backend's HTTP API. Built once at startup and shared through the app context.
/// Its calls live beside it, one file per backend route: `hero`, `projects`, `profile`,
/// `pages`, `activity` and `messages`.
pub struct ApiClient {
    pub(super) http: Client,
    base: Url,
}

impl ApiClient {
    pub fn new(base: &str) -> ApiResult<Self> {
        let base = Url::parse(base).map_err(|_| ApiError::BadUrl(base.to_owned()))?;
        if base.cannot_be_a_base() {
            return Err(ApiError::BadUrl(base.into()));
        }
        Ok(Self {
            http: Client::new(),
            base,
        })
    }

    /// From `API_URL`: `http://backend:8000` in Docker, the local API in development.
    pub fn from_env() -> ApiResult<Self> {
        Self::new(&std::env::var("API_URL").unwrap_or_else(|_| "http://localhost:8000".into()))
    }

    /// The URL for `segments`, each percent-encoded, so a slug from the visitor's URL
    /// cannot reach another endpoint.
    pub(super) fn url(&self, segments: &[&str]) -> Url {
        let mut url = self.base.clone();
        url.path_segments_mut()
            .expect("checked in ApiClient::new")
            .pop_if_empty()
            .extend(segments);
        url
    }

    /// GETs JSON; a 404 is `None`.
    pub(super) async fn get<T: DeserializeOwned>(&self, segments: &[&str]) -> ApiResult<Option<T>> {
        let response = self.http.get(self.url(segments)).send().await?;
        if response.status() == StatusCode::NOT_FOUND {
            return Ok(None);
        }
        Ok(Some(response.error_for_status()?.json().await?))
    }
}
