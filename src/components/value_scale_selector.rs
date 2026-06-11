use super::json_string;
use dioxus::prelude::*;

use crate::event::{on_input_event, ListenerGuard};

/// Size of an [`HmiValueScaleSelector`].
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum ValueScaleSize {
    Small,
    #[default]
    Medium,
    Large,
}

impl ValueScaleSize {
    pub fn as_str(self) -> &'static str {
        match self {
            ValueScaleSize::Small => "small",
            ValueScaleSize::Medium => "medium",
            ValueScaleSize::Large => "large",
        }
    }
}

/// Typed wrapper for the `<hmi-value-scale-selector>` web component.
///
/// A rating-style selector (e.g. star ratings). Pass `value` for controlled
/// mode; picking a value fires `on_change` with the new `f64`. Set
/// `allow_half` for half-step granularity and `icon` to override the symbol.
#[component]
pub fn HmiValueScaleSelector(
    /// Controlled value.
    value: Option<f64>,
    /// Number of steps on the scale.
    #[props(default = 5.0)]
    max: f64,
    /// Allow selecting half values.
    #[props(default)]
    allow_half: bool,
    /// Override the scale symbol (e.g. `"★"`, `"♥"`). Defaults upstream.
    icon: Option<String>,
    #[props(default)] size: ValueScaleSize,
    #[props(default)] read_only: bool,
    #[props(default)] disabled: bool,
    #[props(default)] on_change: EventHandler<f64>,
) -> Element {
    let mut listener = use_signal(|| Option::<ListenerGuard>::None);
    rsx! {
        hmi-value-scale-selector {
            "value": value,
            "max": max,
            "allow-half": if allow_half { "true" },
            "icon": icon.as_deref().map(json_string),
            "size": size.as_str(),
            "read-only": if read_only { "true" },
            "disabled": if disabled { "true" },
            onmounted: move |m| {
                listener.set(on_input_event(&m, "change", on_change, |d| d.number()));
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn size_as_str() {
        assert_eq!(ValueScaleSize::Small.as_str(), "small");
        assert_eq!(ValueScaleSize::Medium.as_str(), "medium");
        assert_eq!(ValueScaleSize::Large.as_str(), "large");
    }

    #[test]
    fn size_defaults_to_medium() {
        assert_eq!(ValueScaleSize::default(), ValueScaleSize::Medium);
    }
}
