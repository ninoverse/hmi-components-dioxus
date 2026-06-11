use super::json_string;
use dioxus::prelude::*;

use crate::event::{on_input_event, ListenerGuard};

/// A single option in an [`HmiSelect`] dropdown.
#[derive(Clone, PartialEq, Debug)]
pub struct SelectOption {
    /// The value emitted when this option is chosen.
    pub value: String,
    /// Display label shown in the dropdown list.
    pub label: String,
    /// Optional icon rendered before the label (emoji or short text).
    pub icon: Option<String>,
}

impl SelectOption {
    pub fn new(value: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            label: label.into(),
            icon: None,
        }
    }

    pub fn icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }
}

fn options_to_json(options: &[SelectOption]) -> String {
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
        out.push('}');
    }
    out.push(']');
    out
}

/// Typed wrapper for the `<hmi-select>` web component.
///
/// A styled dropdown backed by an internal floating panel. Pass `value` for
/// controlled mode; each selection fires `on_change` with the chosen
/// `SelectOption::value` string. The internal trigger is a React element owned
/// by the component, so the click→open flow works across the web-component
/// boundary (unlike slotted-trigger overlays).
#[component]
pub fn HmiSelect(
    options: Vec<SelectOption>,
    /// Controlled selection.
    value: Option<String>,
    /// Placeholder shown when nothing is selected.
    placeholder: Option<String>,
    #[props(default)] disabled: bool,
    #[props(default)] on_change: EventHandler<String>,
) -> Element {
    let mut listener = use_signal(|| Option::<ListenerGuard>::None);
    let opts_json = options_to_json(&options);
    rsx! {
        hmi-select {
            "options": opts_json,
            "value": value,
            "placeholder": placeholder.as_deref().map(json_string),
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
            SelectOption::new("a", "Alpha"),
            SelectOption::new("b", "Bravo"),
        ];
        assert_eq!(
            options_to_json(&opts),
            r#"[{"value":"a","label":"Alpha"},{"value":"b","label":"Bravo"}]"#
        );
    }

    #[test]
    fn options_to_json_with_icon() {
        let opts = vec![SelectOption::new("x", "Extra").icon("⭐")];
        assert_eq!(
            options_to_json(&opts),
            r#"[{"value":"x","label":"Extra","icon":"⭐"}]"#
        );
    }
}
