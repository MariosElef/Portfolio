use topcoat::icon::{IconData, iconify::iconify_icon};

/// The logo key for a tool's display name: "Vue.js" → "vue", "PostgreSQL" →
/// "postgresql". Lowercased, with `.js` and anything but letters and digits dropped.
pub fn key_for(name: &str) -> String {
    name.to_lowercase()
        .replace(".js", "")
        .chars()
        .filter(char::is_ascii_alphanumeric)
        .collect()
}

/// A tool's logo by name (`rust`, `python`, `kubernetes`, …), and the colour it reads
/// well in on a dark page. Unknown names get a generic mark in the text colour.
pub fn logo(name: &str) -> (IconData, Option<&'static str>) {
    match name {
        "rust" => (iconify_icon!("simple-icons:rust"), Some("#dea584")),
        "python" => (iconify_icon!("simple-icons:python"), Some("#4b8bbe")),
        "fastapi" => (iconify_icon!("simple-icons:fastapi"), Some("#05998b")),
        "tokio" => (iconify_icon!("simple-icons:tokio"), Some("#ffffff")),
        "postgresql" => (iconify_icon!("simple-icons:postgresql"), Some("#6b8fe8")),
        "mariadb" => (iconify_icon!("simple-icons:mariadb"), Some("#c0765a")),
        "redis" => (iconify_icon!("simple-icons:redis"), Some("#ff4438")),
        "mongodb" => (iconify_icon!("simple-icons:mongodb"), Some("#47a248")),
        "rabbitmq" => (iconify_icon!("simple-icons:rabbitmq"), Some("#ff6600")),
        "sqlalchemy" => (iconify_icon!("simple-icons:sqlalchemy"), Some("#d71f00")),
        "opentelemetry" => (iconify_icon!("simple-icons:opentelemetry"), Some("#f5a800")),
        "grafana" => (iconify_icon!("simple-icons:grafana"), Some("#f46800")),
        "prometheus" => (iconify_icon!("simple-icons:prometheus"), Some("#e6522c")),
        "nuxt" => (iconify_icon!("simple-icons:nuxt"), Some("#00dc82")),
        "vue" => (iconify_icon!("simple-icons:vuedotjs"), Some("#4fc08d")),
        "typescript" => (iconify_icon!("simple-icons:typescript"), Some("#3178c6")),
        "javascript" => (iconify_icon!("simple-icons:javascript"), Some("#f7df1e")),
        "kubernetes" => (iconify_icon!("simple-icons:kubernetes"), Some("#326ce5")),
        "k3s" => (iconify_icon!("simple-icons:k3s"), Some("#ffc61c")),
        "docker" => (iconify_icon!("simple-icons:docker"), Some("#2496ed")),
        "traefik" => (iconify_icon!("simple-icons:traefikproxy"), Some("#24a1c1")),
        "linux" => (iconify_icon!("simple-icons:linux"), Some("#fcc624")),
        "git" => (iconify_icon!("simple-icons:git"), Some("#f05032")),
        "argo" => (iconify_icon!("simple-icons:argo"), Some("#ef7b4d")),
        "wordpress" => (iconify_icon!("simple-icons:wordpress"), Some("#3aa0d8")),
        "php" => (iconify_icon!("simple-icons:php"), Some("#8f93d8")),
        "keycloak" => (iconify_icon!("simple-icons:keycloak"), Some("#33c6f4")),
        "tailscale" => (iconify_icon!("simple-icons:tailscale"), Some("#ffffff")),
        "githubactions" => (iconify_icon!("simple-icons:githubactions"), Some("#2088ff")),
        "tailwindcss" => (iconify_icon!("simple-icons:tailwindcss"), Some("#06b6d4")),
        "storybook" => (iconify_icon!("simple-icons:storybook"), Some("#ff4785")),
        "pydantic" => (iconify_icon!("simple-icons:pydantic"), Some("#e92063")),
        "axum" => (iconify_icon!("simple-icons:rust"), Some("#dea584")),
        "anthropic" => (iconify_icon!("simple-icons:anthropic"), Some("#d97757")),
        "stripe" => (iconify_icon!("simple-icons:stripe"), Some("#635bff")),
        "terraform" => (iconify_icon!("simple-icons:terraform"), Some("#844fba")),
        _ => (iconify_icon!("lucide:box"), None),
    }
}

#[cfg(test)]
mod tests {
    use super::key_for;

    #[test]
    fn turns_display_names_into_keys() {
        assert_eq!(key_for("Vue.js"), "vue");
        assert_eq!(key_for("PostgreSQL"), "postgresql");
        assert_eq!(key_for("E-commerce"), "ecommerce");
    }
}
