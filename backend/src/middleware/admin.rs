use axum::{
    extract::{Request, State},
    http::{StatusCode, header},
    middleware::Next,
    response::Response,
};

use crate::state::AdminToken;

/// Lets a request through only with `Authorization: Bearer <ADMIN_TOKEN>`.
pub async fn require_token(
    State(AdminToken(token)): State<AdminToken>,
    request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let given = request
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.strip_prefix("Bearer "));

    match given {
        Some(given)
            if !token.is_empty() && constant_time_eq(given.as_bytes(), token.as_bytes()) =>
        {
            Ok(next.run(request).await)
        }
        _ => Err(StatusCode::UNAUTHORIZED),
    }
}

/// Compares without stopping at the first difference, so timing does not reveal the token.
fn constant_time_eq(a: &[u8], b: &[u8]) -> bool {
    a.len() == b.len() && a.iter().zip(b).fold(0, |acc, (x, y)| acc | (x ^ y)) == 0
}

#[cfg(test)]
mod tests {
    use axum::{Router, middleware::from_fn_with_state, routing::post};
    use axum_test::TestServer;

    use super::*;

    fn server(token: &str) -> TestServer {
        let app = Router::new()
            .route("/", post(|| async { "ok" }))
            .layer(from_fn_with_state(AdminToken(token.into()), require_token));
        TestServer::new(app)
    }

    #[tokio::test]
    async fn rejects_missing_and_wrong_tokens() {
        let server = server("secret");
        server
            .post("/")
            .await
            .assert_status(StatusCode::UNAUTHORIZED);
        server
            .post("/")
            .authorization_bearer("nope")
            .await
            .assert_status(StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn accepts_the_token() {
        server("secret")
            .post("/")
            .authorization_bearer("secret")
            .await
            .assert_status_ok();
    }

    #[tokio::test]
    async fn empty_token_disables_uploads() {
        server("")
            .post("/")
            .authorization_bearer("")
            .await
            .assert_status(StatusCode::UNAUTHORIZED);
    }
}
