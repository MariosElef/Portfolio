use topcoat::{context::Cx, router::request::uri};

/// The path of the page being rendered.
pub fn current_path(cx: &Cx) -> &str {
    uri(cx).path()
}

pub fn is_home(cx: &Cx) -> bool {
    current_path(cx) == "/"
}

/// Whether `path` is the current page or a section it is in: `/projects` is current on
/// `/projects/stackup`. `/` is current only on the home page itself.
pub fn is_current(cx: &Cx, path: &str) -> bool {
    path_matches(current_path(cx), path)
}

fn path_matches(current: &str, path: &str) -> bool {
    let path = path.trim_end_matches('/');
    if path.is_empty() {
        return current == "/";
    }
    current
        .strip_prefix(path)
        .is_some_and(|rest| rest.is_empty() || rest.starts_with('/'))
}

#[cfg(test)]
mod tests {
    use super::path_matches;

    #[test]
    fn home_matches_only_itself() {
        assert!(path_matches("/", "/"));
        assert!(!path_matches("/projects", "/"));
    }

    #[test]
    fn sections_match_their_pages() {
        assert!(path_matches("/projects", "/projects"));
        assert!(path_matches("/projects/stackup", "/projects"));
        assert!(!path_matches("/projects-old", "/projects"));
        assert!(!path_matches("/about", "/projects"));
    }
}
