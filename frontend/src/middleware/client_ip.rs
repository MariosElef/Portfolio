use topcoat::{context::Cx, router::request::headers};

use crate::types::visitor::ClientIp;

/// The request's context with the visitor's address in it, as a [`ClientIp`], for the
/// rest of the request to read. Caddy, in front, adds the address it saw last to
/// `X-Forwarded-For`; without it (the frontend reached directly) there is none to add.
pub fn with_client_ip(cx: &Cx) -> Cx {
    match forwarded(cx) {
        Some(ip) => cx.with(ClientIp(ip)),
        None => cx.clone(),
    }
}

fn forwarded(cx: &Cx) -> Option<String> {
    let list = headers(cx).get("x-forwarded-for")?.to_str().ok()?;
    last_address(list)
}

/// The last address in an `X-Forwarded-For` list: the one the nearest proxy saw.
fn last_address(list: &str) -> Option<String> {
    let ip = list.rsplit(',').next()?.trim();
    (!ip.is_empty()).then(|| ip.to_owned())
}

#[cfg(test)]
mod tests {
    use super::last_address;

    #[test]
    fn takes_the_address_the_nearest_proxy_saw() {
        assert_eq!(
            last_address("198.51.100.1, 203.0.113.9").as_deref(),
            Some("203.0.113.9")
        );
        assert_eq!(last_address("203.0.113.9").as_deref(), Some("203.0.113.9"));
        assert_eq!(last_address(" "), None);
    }
}
