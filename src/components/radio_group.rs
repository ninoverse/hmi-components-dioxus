use super::json_string;
use dioxus::prelude::*;

use crate::event::{on_input_event, ListenerGuard};

/// A single option within an [`HmiRadioGroup`].
#[derive(Clone, PartialEq, Debug)]
pub struct RadioOption {
    /// The value emitted when this option is selected.
    pub value: String,
    /// Display label shown next to the radio button.
    pub label: String,
    pub disabled: bool,
}

impl RadioOption {
    pub fn new(value: impl Into<String>, label: impl Into<String>) -> Self {
        Self { value: value.into(), label: label.into(), disabled: false }
    }

    pub fn disabled(mut self) -> Self {
        self.disabled = true;
        self
    }
}

fn options_to_json(options: &[RadioOption]) -> String {
    let mut out = String::from("[");
    for (i, opt) in options.iter().enumerate() {
        if i > 0 {
            out.push(',');
        }
        out.push_str("{\"value\":");
        out.push_str(&json_string(&opt.value));
        out.push_str(",\"label\":");
        out.push_str(&json_string(&opt.label));
        if opt.disabled {
            out.push_str(",\"disabled\":true");
        }
        out.push('}');
    }
    out.push(']');
    out
}

/// Typed wrapper for the `<hmi-radio-group>` web component.
///
/// Renders a group of radio buttons from `options`. Pass `value` for
/// controlled mode; each selection fires `on_change` with the chosen
/// `RadioOption::value` string. `name` is forwarded to every radio input for
/// form submission.
#[component]
pub fn HmiRadioGroup(
    options: Vec<RadioOption>,
    /// Controlled selection: the group always reflects this value.
    value: Option<String>,
    /// Initial value for uncontrolled usage.
    default_value: Option<String>,
    /// `name` attribute forwarded to every underlying radio input.
    name: Option<String>,
    #[props(default)] on_change: EventHandler<String>,
) -> Element {
    let mut listener = use_signal(|| Option::<ListenerGuard>::None);
    let opts_json = options_to_json(&options);
    rsx! {
        hmi-radio-group {
            "options": opts_json,
            "value": value,
            "defaultValue": default_value,
            "name": name,
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
    fn options_to_json_basic() {
        let opts = vec![
            RadioOption::new("a", "Option A"),
            RadioOption::new("b", "Option B"),
        ];
        assert_eq!(
            options_to_json(&opts),
            r#"[{"value":"a","label":"Option A"},{"value":"b","label":"Option B"}]"#
        );
    }

    #[test]
    fn options_to_json_disabled() {
        let opts = vec![RadioOption::new("x", "X").disabled()];
        assert_eq!(
            options_to_json(&opts),
            r#"[{"value":"x","label":"X","disabled":true}]"#
        );
    }
}
