use dioxus::prelude::*;

use crate::event::{on_input_event, ListenerGuard};

/// Typed wrapper for the `<hmi-slider>` web component.
///
/// A range slider. Pass `value` for controlled mode; dragging fires
/// `on_change` with the new `f64`. Set `show_value` to render the current
/// value beside the track. The upstream `formatValue` callback is omitted.
#[component]
pub fn HmiSlider(
    /// Controlled value.
    value: Option<f64>,
    #[props(default = 0.0)] min: f64,
    #[props(default = 100.0)] max: f64,
    #[props(default = 1.0)] step: f64,
    #[props(default)] show_value: bool,
    #[props(default)] disabled: bool,
    #[props(default)] on_change: EventHandler<f64>,
) -> Element {
    let mut listener = use_signal(|| Option::<ListenerGuard>::None);
    rsx! {
        hmi-slider {
            "value": value,
            "min": min,
            "max": max,
            "step": step,
            "show-value": if show_value { "true" },
            "disabled": if disabled { "true" },
            onmounted: move |m| {
                listener.set(on_input_event(&m, "change", on_change, |d| d.number()));
            },
        }
    }
}
