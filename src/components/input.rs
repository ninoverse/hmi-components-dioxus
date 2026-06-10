use dioxus::prelude::*;

use crate::event::{on_input_event, ListenerGuard};

/// Typed wrapper for the `<hmi-input>` web component.
///
/// Binding: pass `value` to render **controlled** — the input always displays
/// exactly `value`, and a keystroke only *requests* an edit through
/// `on_change` (fired with the full requested text); the edit appears when
/// the caller feeds it back into `value`. Omit `value` for an uncontrolled
/// input that owns its own text. `input_type` maps to the upstream `type`
/// attribute (renamed because `type` is a Rust keyword). The upstream
/// `leftIcon`/`rightIcon` JSON descriptors are omitted for now.
#[component]
pub fn HmiInput(
    /// Controlled text: the input always shows exactly this value.
    value: Option<String>,
    placeholder: Option<String>,
    /// Maps to the upstream `type` attribute (e.g. `"email"`, `"password"`).
    input_type: Option<String>,
    #[props(default)] disabled: bool,
    #[props(default)] error: bool,
    name: Option<String>,
    #[props(default)] on_change: EventHandler<String>,
) -> Element {
    // Holds the DOM listener for this component's lifetime; dropped on unmount.
    let mut listener = use_signal(|| Option::<ListenerGuard>::None);
    rsx! {
        hmi-input {
            // Dioxus special-cases `value` as a DOM *property* write; the 5.0.0
            // host forwards property writes to the React `value` prop, even
            // when they land before the element upgrades.
            "value": value,
            "placeholder": placeholder,
            "type": input_type,
            "disabled": if disabled { "true" },
            "error": if error { "true" },
            "name": name,
            onmounted: move |m| {
                listener.set(on_input_event(&m, "change", on_change, |d| d.string()));
            },
        }
    }
}
