use dioxus::prelude::*;

use super::json_string;
use crate::event::{on_input_event, ListenerGuard};

/// Typed wrapper for the `<hmi-radio>` web component.
///
/// A single radio button. Usually you want [`HmiRadioGroup`](super::HmiRadioGroup)
/// for a set of mutually-exclusive options; this is the standalone primitive.
/// Pass `checked` for controlled mode; a click fires `on_change` with the
/// requested `bool`. `name` groups radios for native form behaviour.
#[component]
pub fn HmiRadio(
    /// Controlled state: the radio always shows exactly this value.
    checked: Option<bool>,
    /// Initial checked state for uncontrolled usage.
    default_checked: Option<bool>,
    label: Option<String>,
    #[props(default)] disabled: bool,
    name: Option<String>,
    value: Option<String>,
    #[props(default)] on_change: EventHandler<bool>,
) -> Element {
    let mut listener = use_signal(|| Option::<ListenerGuard>::None);
    rsx! {
        hmi-radio {
            "checked": checked.map(|c| if c { "true" } else { "false" }),
            "defaultChecked": default_checked.map(|c| if c { "true" } else { "false" }),
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
