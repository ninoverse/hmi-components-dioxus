use dioxus::prelude::*;

use crate::event::{on_input_event, ListenerGuard};

/// Typed wrapper for the `<hmi-textarea>` web component.
///
/// Pass `value` to render **controlled** — the element always shows exactly
/// `value` and each keystroke fires `on_change` with the full requested text,
/// which appears once the caller feeds it back into `value`.
#[component]
pub fn HmiTextarea(
    /// Controlled text: the textarea always displays exactly this value.
    value: Option<String>,
    placeholder: Option<String>,
    /// Visible row count.
    rows: Option<u32>,
    #[props(default)] disabled: bool,
    #[props(default)] error: bool,
    name: Option<String>,
    #[props(default)] on_change: EventHandler<String>,
) -> Element {
    let mut listener = use_signal(|| Option::<ListenerGuard>::None);
    rsx! {
        hmi-textarea {
            "value": value,
            "placeholder": placeholder,
            "rows": rows,
            "disabled": if disabled { "true" },
            "error": if error { "true" },
            "name": name,
            onmounted: move |m| {
                listener.set(on_input_event(&m, "change", on_change, |d| d.string()));
            },
        }
    }
}
