use dioxus::prelude::*;

use crate::event::{on_input_event, ListenerGuard};

/// Typed wrapper for the `<hmi-number-input>` web component.
///
/// A numeric input with increment/decrement buttons. Pass `value` for
/// controlled mode; each change fires `on_change` with the new `f64`. Bounds
/// (`min`, `max`) and `step` are enforced upstream before `on_change` fires.
#[component]
pub fn HmiNumberInput(
    /// Controlled value.
    value: Option<f64>,
    min: Option<f64>,
    max: Option<f64>,
    #[props(default = 1.0)] step: f64,
    placeholder: Option<String>,
    #[props(default)] disabled: bool,
    #[props(default)] error: bool,
    #[props(default)] on_change: EventHandler<f64>,
) -> Element {
    let mut listener = use_signal(|| Option::<ListenerGuard>::None);
    rsx! {
        hmi-number-input {
            "value": value,
            "min": min,
            "max": max,
            "step": step,
            "placeholder": placeholder,
            "disabled": if disabled { "true" },
            "error": if error { "true" },
            onmounted: move |m| {
                listener.set(on_input_event(&m, "change", on_change, |d| d.number()));
            },
        }
    }
}
