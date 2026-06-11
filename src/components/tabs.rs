use super::json_string;
use dioxus::prelude::*;

use crate::event::{on_input_event, ListenerGuard};

/// Visual style of an [`HmiTabs`] bar.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum TabsVariant {
    /// Pill-shaped active indicator (the upstream default).
    #[default]
    Pill,
    /// Underline indicator beneath the active tab.
    Underline,
}

impl TabsVariant {
    pub fn as_str(self) -> &'static str {
        match self {
            TabsVariant::Pill => "pill",
            TabsVariant::Underline => "underline",
        }
    }
}

/// A single tab within an [`HmiTabs`] bar.
#[derive(Clone, PartialEq, Debug)]
pub struct TabItem {
    /// Value emitted when this tab is activated.
    pub value: String,
    /// Display label.
    pub label: String,
    /// Optional leading icon (emoji or short text).
    pub icon: Option<String>,
}

impl TabItem {
    pub fn new(value: impl Into<String>, label: impl Into<String>) -> Self {
        Self { value: value.into(), label: label.into(), icon: None }
    }

    pub fn icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }
}

fn options_to_json(options: &[TabItem]) -> String {
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

/// Typed wrapper for the `<hmi-tabs>` web component.
///
/// Renders a tab bar from `options`; the panel content for each tab is the
/// consumer's responsibility (render it conditionally on the selected value).
/// Pass `value` for controlled mode; activating a tab fires `on_change` with
/// the chosen `TabItem::value` string.
#[component]
pub fn HmiTabs(
    options: Vec<TabItem>,
    /// Controlled selection.
    value: Option<String>,
    #[props(default)] variant: TabsVariant,
    #[props(default)] on_change: EventHandler<String>,
) -> Element {
    let mut listener = use_signal(|| Option::<ListenerGuard>::None);
    let opts_json = options_to_json(&options);
    rsx! {
        hmi-tabs {
            "options": opts_json,
            "value": value,
            "variant": variant.as_str(),
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
    fn variant_as_str() {
        assert_eq!(TabsVariant::Pill.as_str(), "pill");
        assert_eq!(TabsVariant::Underline.as_str(), "underline");
    }

    #[test]
    fn variant_defaults_to_pill() {
        assert_eq!(TabsVariant::default(), TabsVariant::Pill);
    }

    #[test]
    fn options_to_json_with_icon() {
        let opts = vec![
            TabItem::new("home", "Home"),
            TabItem::new("profile", "Profile").icon("👤"),
        ];
        assert_eq!(
            options_to_json(&opts),
            r#"[{"value":"home","label":"Home"},{"value":"profile","label":"Profile","icon":"👤"}]"#
        );
    }
}
