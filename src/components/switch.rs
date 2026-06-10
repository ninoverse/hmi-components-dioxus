use dioxus::prelude::*;

use super::json_string;
use crate::event::{on_input_event, ListenerGuard};

/// Typed wrapper for the `<hmi-switch>` web component.
///
/// Binding: pass `checked` to render **controlled** — the switch always
/// displays exactly `checked`, and a toggle only *requests* the flip through
/// `on_change` (payload the requested `bool`); it appears when the caller
/// feeds it back into `checked`. Omit `checked` for an uncontrolled switch
/// that owns its own state. `label` is plain text rendered beside the switch.
#[component]
pub fn HmiSwitch(
    /// Controlled state: the switch always shows exactly this value.
    checked: Option<bool>,
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
            // Dioxus special-cases `checked` as a DOM *property* write; the
            // 5.0.0 host forwards property writes to the React `checked` prop,
            // even when they land before the element upgrades.
            "checked": checked.map(|c| if c { "true" } else { "false" }),
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
