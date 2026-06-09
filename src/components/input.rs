use dioxus::prelude::*;

use crate::event::{on_input_event, ListenerGuard};

/// Typed wrapper for the `<hmi-input>` web component.
///
/// Binding: `value` seeds the initial text; the control is then **uncontrolled**
/// (the DOM owns the live value) and reports edits through `on_change`, which
/// fires on every keystroke with the current text. `input_type` maps to the
/// upstream `type` attribute (renamed because `type` is a Rust keyword). The
/// upstream `leftIcon`/`rightIcon` JSON descriptors are omitted for now.
#[component]
pub fn HmiInput(
    #[props(default)] value: String,
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
            "default-value": if !value.is_empty() { value },
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
