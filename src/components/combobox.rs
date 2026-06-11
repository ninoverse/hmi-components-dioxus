use super::json_string;
use dioxus::prelude::*;

use crate::event::{on_input_event, ListenerGuard};

/// A single option in an [`HmiCombobox`].
#[derive(Clone, PartialEq, Debug)]
pub struct ComboboxOption {
    /// Value emitted when this option is chosen.
    pub value: String,
    /// Display label, also matched against the typed query.
    pub label: String,
}

impl ComboboxOption {
    pub fn new(value: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            label: label.into(),
        }
    }
}

fn options_to_json(options: &[ComboboxOption]) -> String {
    let mut out = String::from("[");
    for (i, opt) in options.iter().enumerate() {
        if i > 0 {
            out.push(',');
        }
        out.push_str("{\"value\":");
        out.push_str(&json_string(&opt.value));
        out.push_str(",\"label\":");
        out.push_str(&json_string(&opt.label));
        out.push('}');
    }
    out.push(']');
    out
}

/// Typed wrapper for the `<hmi-combobox>` web component.
///
/// A text input that filters `options` as you type and commits a selection
/// from the dropdown. Pass `value` for controlled mode; choosing an option
/// fires `on_change` with the selected `ComboboxOption::value` string. The
/// upstream `filterOption` callback uses the built-in label match.
#[component]
pub fn HmiCombobox(
    options: Vec<ComboboxOption>,
    /// Controlled selection.
    value: Option<String>,
    placeholder: Option<String>,
    /// Message shown when no option matches the query.
    empty_message: Option<String>,
    #[props(default)] disabled: bool,
    #[props(default)] on_change: EventHandler<String>,
) -> Element {
    let mut listener = use_signal(|| Option::<ListenerGuard>::None);
    let opts_json = options_to_json(&options);
    rsx! {
        hmi-combobox {
            "options": opts_json,
            "value": value,
            "placeholder": placeholder,
            "empty-message": empty_message.as_deref().map(json_string),
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
    fn options_to_json_basic() {
        let opts = vec![
            ComboboxOption::new("us", "United States"),
            ComboboxOption::new("ca", "Canada"),
        ];
        assert_eq!(
            options_to_json(&opts),
            r#"[{"value":"us","label":"United States"},{"value":"ca","label":"Canada"}]"#
        );
    }
}
