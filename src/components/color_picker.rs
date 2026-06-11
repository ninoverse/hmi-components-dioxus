use super::json_string;
use dioxus::prelude::*;

use crate::event::{on_input_event, ListenerGuard};

fn swatches_to_json(swatches: &[String]) -> String {
    let mut out = String::from("[");
    for (i, s) in swatches.iter().enumerate() {
        if i > 0 {
            out.push(',');
        }
        out.push_str(&json_string(s));
    }
    out.push(']');
    out
}

/// Typed wrapper for the `<hmi-color-picker>` web component.
///
/// A swatch + popover colour selector backed by an internal trigger button, so
/// the open/close flow works across the web-component boundary. Pass `value`
/// (a CSS colour string like `"#e87a5d"`) for controlled mode; picking a colour
/// fires `on_change` with the new value. Provide `swatches` to override the
/// preset palette, and clear `show_input` to hide the hex input.
#[component]
pub fn HmiColorPicker(
    /// Controlled colour value (e.g. `"#e87a5d"`).
    value: Option<String>,
    /// Preset colour swatches. Defaults to the upstream palette.
    swatches: Option<Vec<String>>,
    /// Show the free-form hex input (defaults to `true` upstream).
    show_input: Option<bool>,
    #[props(default)] disabled: bool,
    #[props(default)] on_change: EventHandler<String>,
) -> Element {
    let mut listener = use_signal(|| Option::<ListenerGuard>::None);
    let swatches_json = swatches.as_deref().map(swatches_to_json);
    rsx! {
        hmi-color-picker {
            "value": value,
            "swatches": swatches_json,
            "show-input": show_input.map(|b| if b { "true" } else { "false" }),
            "disabled": if disabled { "true" },
            onmounted: move |m| {
                listener.set(on_input_event(&m, "change", on_change, |d| d.string()));
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn swatches_to_json_basic() {
        let s = vec!["#fff".to_string(), "#000".to_string()];
        assert_eq!(swatches_to_json(&s), r##"["#fff","#000"]"##);
    }
}
