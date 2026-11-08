//! Addresses as people read them.

/// `url` without its scheme, `www.` or a trailing slash: `github.com/MariosElef`.
pub fn display(url: &str) -> &str {
    let rest = url.split_once("://").map_or(url, |(_, rest)| rest);
    rest.strip_prefix("www.")
        .unwrap_or(rest)
        .trim_end_matches('/')
}

#[cfg(test)]
mod tests {
    use super::display;

    #[test]
    fn drops_the_scheme_www_and_trailing_slash() {
        assert_eq!(
            display("https://www.linkedin.com/in/someone/"),
            "linkedin.com/in/someone"
        );
        assert_eq!(
            display("https://github.com/MariosElef"),
            "github.com/MariosElef"
        );
    }
}
