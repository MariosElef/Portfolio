use std::net::SocketAddr;

use axum::{
    extract::{ConnectInfo, Request, State},
    http::{HeaderMap, HeaderValue, StatusCode, header},
    middleware::Next,
    response::{IntoResponse, Response},
};

use crate::{repositories::redis::keys, state::AppState};

/// The visitor's address as the frontend saw it. Caddy strips it from requests arriving
/// from outside, so only the frontend, calling from inside, can set it.
pub const CLIENT_IP: &str = "x-client-ip";

/// How many requests one visitor may make to a route within a window. Counts live in Redis
/// under the limit's name, so every backend instance shares them.
#[derive(Clone, Copy)]
pub struct RateLimit {
    pub name: &'static str,
    pub max: u64,
    pub window_seconds: u64,
}

/// Lets a visitor through until they pass the limit, then answers 429 until the window
/// ends. Fails open: if Redis is down, requests go through rather than all failing.
pub async fn limit(
    State((state, limit)): State<(AppState, RateLimit)>,
    request: Request,
    next: Next,
) -> Response {
    let peer = request
        .extensions()
        .get::<ConnectInfo<SocketAddr>>()
        .map(|ConnectInfo(addr)| addr.ip().to_string());
    let visitor = visitor(request.headers(), peer);
    let key = keys::rate(limit.name, &visitor);

    match state.redis.hit(&key, limit.window_seconds).await {
        Some(count) if count > limit.max => {
            let mut response = StatusCode::TOO_MANY_REQUESTS.into_response();
            response
                .headers_mut()
                .insert(header::RETRY_AFTER, HeaderValue::from(limit.window_seconds));
            response
        }
        _ => next.run(request).await,
    }
}

/// Who is asking: the address the frontend passes on, else the one Caddy saw, else the
/// connection's own.
fn visitor(headers: &HeaderMap, peer: Option<String>) -> String {
    let header = |name: &str| {
        headers
            .get(name)
            .and_then(|value| value.to_str().ok())
            .map(str::trim)
            .filter(|value| !value.is_empty())
    };
    header(CLIENT_IP)
        // Caddy adds the address it saw last.
        .or_else(|| header("x-forwarded-for").and_then(|list| list.rsplit(',').next()))
        .map(|value| value.trim().to_owned())
        .or(peer)
        .unwrap_or_else(|| "unknown".to_owned())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn headers(pairs: &[(&'static str, &'static str)]) -> HeaderMap {
        pairs
            .iter()
            .map(|&(name, value)| (name.parse().unwrap(), HeaderValue::from_static(value)))
            .collect()
    }

    #[test]
    fn prefers_the_address_the_frontend_passes_on() {
        let headers = headers(&[(CLIENT_IP, "203.0.113.7"), ("x-forwarded-for", "10.0.0.2")]);
        assert_eq!(visitor(&headers, Some("10.0.0.9".into())), "203.0.113.7");
    }

    #[test]
    fn falls_back_to_the_last_forwarded_address() {
        let headers = headers(&[("x-forwarded-for", "198.51.100.1, 203.0.113.9")]);
        assert_eq!(visitor(&headers, None), "203.0.113.9");
    }

    #[test]
    fn falls_back_to_the_connection() {
        assert_eq!(
            visitor(&HeaderMap::new(), Some("10.0.0.9".into())),
            "10.0.0.9"
        );
        assert_eq!(visitor(&HeaderMap::new(), None), "unknown");
    }
}
