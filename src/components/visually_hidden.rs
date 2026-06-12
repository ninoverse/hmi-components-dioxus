use dioxus::prelude::*;

/// Typed wrapper for the `<hmi-visually-hidden>` web component.
///
/// Hides its `children` visually while keeping them available to screen
/// readers (the standard "visually hidden" accessibility pattern). `tag`
/// chooses the rendered element (maps to the upstream `as` prop, which is a
/// Rust keyword); defaults to `span` upstream.
#[component]
pub fn HmiVisuallyHidden(
    /// The element to render as, e.g. `"span"` or `"div"`. Defaults to `span`.
    tag: Option<String>,
    children: Element,
) -> Element {
    rsx! {
        hmi-visually-hidden {
            "as": tag,
            {children}
        }
    }
}
