use dioxus::prelude::*;

use crate::event::{host_element, on_input_event, ElementHandle, ListenerGuard};

/// Typed wrapper for the `<hmi-input>` web component.
///
/// Two-way binding: push a value in via `value`, read edits out via
/// `on_change`, which fires on every keystroke with the current text. `value`
/// is synced as an attribute imperatively (Dioxus would set a property the
/// custom element ignores) — see [`crate::event`] for the full story.
/// `input_type` maps to the upstream `type` attribute (renamed because `type`
/// is a Rust keyword). The upstream `leftIcon`/`rightIcon` JSON descriptors are
/// omitted for now.
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
    let mut host = use_signal(|| Option::<ElementHandle>::None);
    // Push `value` to the host as an attribute whenever it (or the host) changes.
    use_effect(move || {
        if let Some(h) = host.read().as_ref() {
            h.set_attr("value", Some(&value));
        }
    });
    rsx! {
        hmi-input {
            "placeholder": placeholder,
            "type": input_type,
            "disabled": if disabled { "true" },
            "error": if error { "true" },
            "name": name,
            onmounted: move |m| {
                host.set(host_element(&m));
                listener.set(on_input_event(&m, "input", on_change, |el| Some(el.value())));
            },
        }
    }
}
