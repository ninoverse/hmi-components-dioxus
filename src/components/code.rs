use dioxus::prelude::*;

/// Typed wrapper for the `<hmi-code>` web component.
///
/// Renders inline `<code>` by default; setting `block` switches to a
/// `<pre><code>` block. Mirrors the upstream `code--block` modifier.
#[component]
pub fn HmiCode(#[props(default)] block: bool, children: Element) -> Element {
    rsx! {
        hmi-code { "block": if block { "true" }, {children} }
    }
}
