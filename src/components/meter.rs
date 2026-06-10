use dioxus::prelude::*;

use super::json_string;

/// Typed wrapper for the `<hmi-meter>` web component.
///
/// `value` sits between `min` (upstream default `0`) and `max` (upstream
/// default `1`). The optional `low`/`high`/`optimum` thresholds drive the
/// optimal/suboptimal/poor color, mirroring the native `<meter>` element.
/// `show_value` renders the numeric value beside the optional `label`.
#[component]
pub fn HmiMeter(
    value: f64,
    min: Option<f64>,
    max: Option<f64>,
    low: Option<f64>,
    high: Option<f64>,
    optimum: Option<f64>,
    label: Option<String>,
    #[props(default)] show_value: bool,
) -> Element {
    rsx! {
        hmi-meter {
            "value": value,
            "min": min,
            "max": max,
            "low": low,
            "high": high,
            "optimum": optimum,
            // `label` is a JSON-typed prop upstream (ReactNode); encode plain
            // text so the bridge's `JSON.parse` yields a string.
            "label": label.as_deref().map(json_string),
            "show-value": if show_value { "true" },
        }
    }
}
