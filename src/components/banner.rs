use super::json_string;
use dioxus::prelude::*;

/// Visual tone of an [`HmiBanner`]. Mirrors the upstream `banner--*` modifiers.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum BannerVariant {
    #[default]
    Info,
    Success,
    Warning,
    Danger,
}

impl BannerVariant {
    /// The string the `<hmi-banner>` `variant` attribute expects.
    pub fn as_str(self) -> &'static str {
        match self {
            BannerVariant::Info => "info",
            BannerVariant::Success => "success",
            BannerVariant::Warning => "warning",
            BannerVariant::Danger => "danger",
        }
    }
}

/// Typed wrapper for the `<hmi-banner>` web component.
///
/// A full-width contextual notice. Pass the body as `children`; `title` renders
/// as a bold line above it, and a variant icon is shown automatically. The
/// upstream dismiss button and `action` slot need JS callbacks and are omitted.
#[component]
pub fn HmiBanner(
    #[props(default)] variant: BannerVariant,
    /// Optional bold title shown above the body.
    title: Option<String>,
    children: Element,
) -> Element {
    rsx! {
        hmi-banner {
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
        assert_eq!(BannerVariant::Info.as_str(), "info");
        assert_eq!(BannerVariant::Success.as_str(), "success");
        assert_eq!(BannerVariant::Warning.as_str(), "warning");
        assert_eq!(BannerVariant::Danger.as_str(), "danger");
    }

    #[test]
    fn variant_defaults_to_info() {
        assert_eq!(BannerVariant::default(), BannerVariant::Info);
    }
}
