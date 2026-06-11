use super::json_string;
use dioxus::prelude::*;

use crate::event::{on_input_event, ListenerGuard};

/// Size of an [`HmiSegmentedControl`].
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum SegmentedSize {
    Small,
    #[default]
    Medium,
    Large,
}

impl SegmentedSize {
    pub fn as_str(self) -> &'static str {
        match self {
            SegmentedSize::Small => "small",
            SegmentedSize::Medium => "medium",
            SegmentedSize::Large => "large",
        }
    }
}

/// A single segment within an [`HmiSegmentedControl`].
#[derive(Clone, PartialEq, Debug)]
pub struct SegmentOption {
    /// Value emitted when this segment is selected.
    pub value: String,
    /// Display label.
    pub label: String,
    /// Optional leading icon (emoji or short text).
    pub icon: Option<String>,
    pub disabled: bool,
}

impl SegmentOption {
    pub fn new(value: impl Into<String>, label: impl Into<String>) -> Self {
        Self { value: value.into(), label: label.into(), icon: None, disabled: false }
    }

    pub fn icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    pub fn disabled(mut self) -> Self {
        self.disabled = true;
        self
    }
}

fn options_to_json(options: &[SegmentOption]) -> String {
    let mut out = String::from("[");
    for (i, opt) in options.iter().enumerate() {
        if i > 0 {
            out.push(',');
        }
        out.push_str("{\"value\":");
        out.push_str(&json_string(&opt.value));
        out.push_str(",\"label\":");
        out.push_str(&json_string(&opt.label));
        if let Some(icon) = &opt.icon {
            out.push_str(",\"icon\":");
            out.push_str(&json_string(icon));
        }
        if opt.disabled {
            out.push_str(",\"disabled\":true");
        }
        out.push('}');
    }
    out.push(']');
    out
}

/// Typed wrapper for the `<hmi-segmented-control>` web component.
///
/// A compact single-select control rendered as adjacent buttons. Pass `value`
/// for controlled mode; selecting a segment fires `on_change` with the chosen
/// `SegmentOption::value` string. Set `full_width` to stretch across the
/// container.
#[component]
pub fn HmiSegmentedControl(
    options: Vec<SegmentOption>,
    /// Controlled selection.
    value: Option<String>,
    #[props(default)] size: SegmentedSize,
    #[props(default)] full_width: bool,
    #[props(default)] disabled: bool,
    #[props(default)] on_change: EventHandler<String>,
) -> Element {
    let mut listener = use_signal(|| Option::<ListenerGuard>::None);
    let opts_json = options_to_json(&options);
    rsx! {
        hmi-segmented-control {
            "options": opts_json,
            "value": value,
            "size": size.as_str(),
            "full-width": if full_width { "true" },
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
    fn size_as_str() {
        assert_eq!(SegmentedSize::Small.as_str(), "small");
        assert_eq!(SegmentedSize::Medium.as_str(), "medium");
        assert_eq!(SegmentedSize::Large.as_str(), "large");
    }

    #[test]
    fn size_defaults_to_medium() {
        assert_eq!(SegmentedSize::default(), SegmentedSize::Medium);
    }

    #[test]
    fn options_to_json_full() {
        let opts = vec![
            SegmentOption::new("l", "List"),
            SegmentOption::new("g", "Grid").icon("▦").disabled(),
        ];
        assert_eq!(
            options_to_json(&opts),
            r#"[{"value":"l","label":"List"},{"value":"g","label":"Grid","icon":"▦","disabled":true}]"#
        );
    }
}
