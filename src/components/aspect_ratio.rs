use dioxus::prelude::*;

/// Typed wrapper for the `<hmi-aspect-ratio>` web component.
///
/// Constrains its `children` to a fixed aspect ratio via the CSS
/// `aspect-ratio` property. `ratio` is any valid CSS aspect-ratio value, e.g.
/// `"16/9"`, `"4/3"`, or `"1"` (the upstream default).
#[component]
pub fn HmiAspectRatio(
    /// CSS aspect-ratio value, e.g. `"16/9"`. Defaults to `"1"` upstream.
    ratio: Option<String>,
    children: Element,
) -> Element {
    rsx! {
        hmi-aspect-ratio {
            "ratio": ratio,
            {children}
        }
    }
}
