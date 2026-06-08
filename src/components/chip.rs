use dioxus::prelude::*;

/// Typed wrapper for the `<hmi-chip>` web component.
///
/// The upstream `onSelect`/`onClose` callbacks resolve JS globals by name and
/// are not expressible as attributes, so they are omitted; attach standard
/// event handlers to the rendered element instead. `icon` is a JSON-encoded
/// node descriptor passed through verbatim.
#[component]
pub fn HmiChip(
    /// Marks the chip as selected.
    #[props(default)]
    selected: bool,
    /// JSON-encoded icon descriptor rendered before the label.
    #[props(default)]
    icon: Option<String>,
    children: Element,
) -> Element {
    rsx! {
        hmi-chip { "selected": if selected { "true" }, "icon": icon, {children} }
    }
}
