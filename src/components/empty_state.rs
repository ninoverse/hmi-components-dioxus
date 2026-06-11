use super::json_string;
use dioxus::prelude::*;

/// Typed wrapper for the `<hmi-empty-state>` web component.
///
/// A centred placeholder for empty views: an optional `icon` (e.g. an emoji)
/// above a `title` and optional `description`. The upstream `action` slot is a
/// rendered node that can't carry an interactive button across the
/// web-component boundary, so it is omitted — render your own action button
/// beside this element instead.
#[component]
pub fn HmiEmptyState(
    /// Leading icon, e.g. an emoji like `"📭"`.
    icon: Option<String>,
    title: String,
    description: Option<String>,
) -> Element {
    rsx! {
        hmi-empty-state {
            "icon": icon.as_deref().map(json_string),
            "title": json_string(&title),
            "description": description.as_deref().map(json_string),
        }
    }
}
