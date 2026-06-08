use dioxus::prelude::*;

/// Typed wrapper for the `<hmi-progress>` web component.
///
/// `value` is a percentage (clamped to `0..=100` upstream). Setting
/// `indeterminate` shows the looping animation and ignores `value`.
#[component]
pub fn HmiProgress(
    #[props(default)] value: f64,
    #[props(default)] indeterminate: bool,
    label: Option<String>,
) -> Element {
    rsx! {
        hmi-progress {
            "value": value,
            "indeterminate": if indeterminate { "true" },
            "label": label,
        }
    }
}
