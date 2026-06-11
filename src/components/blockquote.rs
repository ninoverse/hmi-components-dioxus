use super::json_string;
use dioxus::prelude::*;

/// Typed wrapper for the `<hmi-blockquote>` web component.
///
/// Renders a styled quotation. Pass the quote body as `children`; `cite`
/// renders as an attribution line beneath it.
#[component]
pub fn HmiBlockquote(
    /// Optional attribution shown below the quote.
    cite: Option<String>,
    children: Element,
) -> Element {
    rsx! {
        hmi-blockquote {
            "cite": cite.as_deref().map(json_string),
            {children}
        }
    }
}
