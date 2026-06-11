use dioxus::prelude::*;

use crate::event::{on_input_event, ListenerGuard};

/// Typed wrapper for the `<hmi-password-input>` web component.
///
/// A text input with a built-in show/hide toggle. Pass `value` for controlled
/// mode; each keystroke fires `on_change` with the full requested text.
#[component]
pub fn HmiPasswordInput(
    /// Controlled text.
    value: Option<String>,
    /// Initial value for uncontrolled usage.
    default_value: Option<String>,
    placeholder: Option<String>,
    #[props(default)] disabled: bool,
    #[props(default)] error: bool,
    #[props(default)] on_change: EventHandler<String>,
) -> Element {
    let mut listener = use_signal(|| Option::<ListenerGuard>::None);
    rsx! {
        hmi-password-input {
            "value": value,
            "defaultValue": default_value,
            "placeholder": placeholder,
            "disabled": if disabled { "true" },
            "error": if error { "true" },
            onmounted: move |m| {
                listener.set(on_input_event(&m, "change", on_change, |d| d.string()));
            },
        }
    }
}
