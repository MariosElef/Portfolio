use topcoat::context::Cx;

use super::api::api;
use crate::{services::client::ApiResult, types::content::StackItem};

/// One category of the stack and its tools, in order.
pub struct StackGroup {
    pub category: String,
    pub note: String,
    pub items: Vec<StackItem>,
}

/// The stack, grouped by category in the order the backend returns it.
pub async fn stack_groups(cx: &Cx) -> ApiResult<Vec<StackGroup>> {
    Ok(group(api(cx).stack().await?))
}

fn group(items: Vec<StackItem>) -> Vec<StackGroup> {
    let mut groups: Vec<StackGroup> = Vec::new();
    for item in items {
        match groups.last_mut() {
            Some(last) if last.category == item.category => last.items.push(item),
            _ => groups.push(StackGroup {
                category: item.category.clone(),
                note: item.category_note.clone(),
                items: vec![item],
            }),
        }
    }
    groups
}

#[cfg(test)]
mod tests {
    use super::group;
    use crate::types::content::StackItem;

    fn item(category: &str, name: &str) -> StackItem {
        StackItem {
            category: category.into(),
            category_note: String::new(),
            name: name.into(),
            icon: String::new(),
        }
    }

    #[test]
    fn groups_consecutive_items_by_category() {
        let groups = group(vec![
            item("Backend", "Rust"),
            item("Backend", "Axum"),
            item("Databases", "Redis"),
        ]);
        let shape: Vec<_> = groups
            .iter()
            .map(|g| (g.category.as_str(), g.items.len()))
            .collect();
        assert_eq!(shape, [("Backend", 2), ("Databases", 1)]);
    }
}
