use topcoat::{
    Result,
    context::Cx,
    router::{Body, Next, response::Response},
};

/// Headers every response carries: no MIME sniffing, no framing by other sites, a
/// conservative referrer, and no access to device features the site never uses.
const HEADERS: [(&str, &str); 4] = [
    ("x-content-type-options", "nosniff"),
    ("x-frame-options", "DENY"),
    ("referrer-policy", "strict-origin-when-cross-origin"),
    (
        "permissions-policy",
        "camera=(), microphone=(), geolocation=()",
    ),
];

pub async fn security_headers(cx: &Cx, body: Body, next: Next<'_>) -> Result<Response> {
    let mut response = next.run(cx, body).await?;
    let headers = response.headers_mut();
    for (name, value) in HEADERS {
        headers.insert(name, value.parse().expect("static header value"));
    }
    Ok(response)
}
