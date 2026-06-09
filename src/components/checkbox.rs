use dioxus::prelude::*;

use super::json_string;
use crate::event::{host_element, on_input_event, ElementHandle, ListenerGuard};

/// Typed wrapper for the `<hmi-checkbox>` web component.
///
/// Two-way binding: push state in via `checked`, read toggles out via
/// `on_change` (the inner checkbox's native `change` event, payload `bool`).
/// `label` is plain text rendered beside the box. The `checked` state is synced
/// as an attribute imperatively — see [`crate::event`] for why.
#[component]
pub fn HmiCheckbox(
    #[props(default)] checked: bool,
    label: Option<String>,
    #[props(default)] disabled: bool,
    name: Option<String>,
    value: Option<String>,
    #[props(default)] on_change: EventHandler<bool>,
) -> Element {
    // Holds the DOM listener for this component's lifetime; dropped on unmount.
    let mut listener = use_signal(|| Option::<ListenerGuard>::None);
    let mut host = use_signal(|| Option::<ElementHandle>::None);
    // Drive the inner input's `checked` property whenever it (or the host)
    // changes; the element stays React-uncontrolled so the user can toggle it.
    use_effect(move || {
        if let Some(h) = host.read().as_ref() {
            h.set_inner_checked(checked);
        }
    });
    rsx! {
        hmi-checkbox {
            "label": label.as_deref().map(json_string),
            "disabled": if disabled { "true" },
            "name": name,
            "value": value,
            onmounted: move |m| {
                host.set(host_element(&m));
                listener.set(on_input_event(&m, "change", on_change, |el| Some(el.checked())));
            },
        }
    }
}
