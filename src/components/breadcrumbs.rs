use super::json_string;
use dioxus::prelude::*;

/// A single navigation item rendered by [`HmiBreadcrumbs`].
#[derive(Clone, PartialEq, Debug)]
pub struct BreadcrumbItem {
    /// Display text for this crumb.
    pub label: String,
    /// Link destination. Omit for the current (last) crumb.
    pub href: Option<String>,
}

impl BreadcrumbItem {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            href: None,
        }
    }

    pub fn with_href(mut self, href: impl Into<String>) -> Self {
        self.href = Some(href.into());
        self
    }
}

fn items_to_json(items: &[BreadcrumbItem]) -> String {
    let mut out = String::from("[");
    for (i, item) in items.iter().enumerate() {
        if i > 0 {
            out.push(',');
        }
        out.push('{');
        out.push_str("\"label\":");
        out.push_str(&json_string(&item.label));
        if let Some(href) = &item.href {
            out.push_str(",\"href\":");
            out.push_str(&json_string(href));
        }
        out.push('}');
    }
    out.push(']');
    out
}

/// Typed wrapper for the `<hmi-breadcrumbs>` web component.
///
/// Renders a navigation trail. The last item in `items` is treated as the
/// current page (rendered as `<span>` without a link). All others get
/// `<a href>` anchors. `separator` defaults to `"/"`.
#[component]
pub fn HmiBreadcrumbs(
    items: Vec<BreadcrumbItem>,
    /// Character(s) shown between crumbs. Defaults to `"/"`.
    separator: Option<String>,
) -> Element {
    let items_json = items_to_json(&items);
    rsx! {
        hmi-breadcrumbs {
            "items": items_json,
            "separator": separator.as_deref().map(json_string),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn items_to_json_no_href() {
        let items = vec![BreadcrumbItem::new("Home")];
        assert_eq!(items_to_json(&items), r#"[{"label":"Home"}]"#);
    }

    #[test]
    fn items_to_json_with_href() {
        let items = vec![
            BreadcrumbItem::new("Home").with_href("/"),
            BreadcrumbItem::new("Docs").with_href("/docs"),
            BreadcrumbItem::new("Current"),
        ];
        let json = items_to_json(&items);
        assert_eq!(
            json,
            r#"[{"label":"Home","href":"/"},{"label":"Docs","href":"/docs"},{"label":"Current"}]"#
        );
    }

    #[test]
    fn items_to_json_escapes_special_chars() {
        let items = vec![BreadcrumbItem::new("A \"quoted\" label")];
        assert_eq!(items_to_json(&items), r#"[{"label":"A \"quoted\" label"}]"#);
    }
}
