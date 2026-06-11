use dioxus::prelude::*;

use crate::event::{on_input_event, ListenerGuard};

/// Character set accepted by an [`HmiMultiInput`].
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum MultiInputType {
    /// Digits only (the upstream default) — e.g. numeric OTP codes.
    #[default]
    Numeric,
    /// Any non-whitespace character.
    Text,
}

impl MultiInputType {
    pub fn as_str(self) -> &'static str {
        match self {
            MultiInputType::Numeric => "numeric",
            MultiInputType::Text => "text",
        }
    }
}

/// Typed wrapper for the `<hmi-multi-input>` web component.
///
/// A segmented single-character input (e.g. for OTP / verification codes).
/// Pass `value` for controlled mode; editing fires `on_change` with the full
/// combined string. `length` sets the number of cells; `group_size` inserts a
/// `separator` between groups.
#[component]
pub fn HmiMultiInput(
    /// Number of single-character cells.
    #[props(default = 6)]
    length: u32,
    /// Insert a separator after every `group_size` cells.
    group_size: Option<u32>,
    /// Separator rendered between groups. Defaults to an en dash upstream.
    separator: Option<String>,
    /// Controlled value (the combined string).
    value: Option<String>,
    #[props(default)] input_type: MultiInputType,
    /// Render entered characters masked (like a password).
    #[props(default)]
    mask: bool,
    #[props(default)] disabled: bool,
    #[props(default)] on_change: EventHandler<String>,
) -> Element {
    let mut listener = use_signal(|| Option::<ListenerGuard>::None);
    rsx! {
        hmi-multi-input {
            "length": length,
            "group-size": group_size,
            "separator": separator,
            "value": value,
            "type": input_type.as_str(),
            "mask": if mask { "true" },
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
    fn type_as_str() {
        assert_eq!(MultiInputType::Numeric.as_str(), "numeric");
        assert_eq!(MultiInputType::Text.as_str(), "text");
    }

    #[test]
    fn type_defaults_to_numeric() {
        assert_eq!(MultiInputType::default(), MultiInputType::Numeric);
    }
}
