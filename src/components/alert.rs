use super::json_string;
use dioxus::prelude::*;

/// Visual tone of an [`HmiAlert`]. Mirrors the upstream `alert--*` modifiers.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum AlertVariant {
    #[default]
    Info,
    Success,
    Warning,
    Danger,
}

impl AlertVariant {
    /// The string the `<hmi-alert>` `variant` attribute expects.
    pub fn as_str(self) -> &'static str {
        match self {
            AlertVariant::Info => "info",
            AlertVariant::Success => "success",
            AlertVariant::Warning => "warning",
            AlertVariant::Danger => "danger",
        }
    }
}

/// Typed wrapper for the `<hmi-alert>` web component.
///
/// Renders a contextual message banner. Pass the body as `children`; `title`
/// renders as a bold heading above it. The upstream `action` slot (a button
/// with a JS callback) is not expressible as an attribute and is omitted.
#[component]
pub fn HmiAlert(
    #[props(default)] variant: AlertVariant,
    /// Optional bold title shown above the body.
    title: Option<String>,
    children: Element,
) -> Element {
    rsx! {
        hmi-alert {
            "variant": variant.as_str(),
            "title": title.as_deref().map(json_string),
            {children}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn variant_as_str_maps_every_value() {
        assert_eq!(AlertVariant::Info.as_str(), "info");
        assert_eq!(AlertVariant::Success.as_str(), "success");
        assert_eq!(AlertVariant::Warning.as_str(), "warning");
        assert_eq!(AlertVariant::Danger.as_str(), "danger");
    }

    #[test]
    fn variant_defaults_to_info() {
        assert_eq!(AlertVariant::default(), AlertVariant::Info);
    }
}
