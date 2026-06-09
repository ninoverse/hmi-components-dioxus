use dioxus::prelude::*;

use super::json_string;
use crate::event::{on_input_event, ListenerGuard};

/// Typed wrapper for the `<hmi-switch>` web component.
///
/// Binding: `checked` seeds the initial state; the control is then
/// **uncontrolled** (the DOM owns the toggle) and reports changes through
/// `on_change` (the inner checkbox's native `change`, payload `bool`). `label`
/// is plain text rendered beside the switch.
#[component]
pub fn HmiSwitch(
    #[props(default)] checked: bool,
    label: Option<String>,
    #[props(default)] disabled: bool,
    name: Option<String>,
    value: Option<String>,
    #[props(default)] on_change: EventHandler<bool>,
) -> Element {
    // Holds the DOM listener for this component's lifetime; dropped on unmount.
    let mut listener = use_signal(|| Option::<ListenerGuard>::None);
    rsx! {
        hmi-switch {
            "default-checked": if checked { "true" } else { "false" },
            "label": label.as_deref().map(json_string),
            "disabled": if disabled { "true" },
            "name": name,
            "value": value,
            onmounted: move |m| {
                listener.set(on_input_event(&m, "change", on_change, |d| d.bool()));
            },
        }
    }
}
