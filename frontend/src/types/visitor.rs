/// The visitor's address, as the proxy in front of the site saw it. Set on every request
/// by `middleware::client_ip`, and passed on to the backend, whose rate limits count by it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ClientIp(pub String);
