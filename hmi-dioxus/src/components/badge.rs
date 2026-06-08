use dioxus::prelude::*;

/// Visual style of an [`HmiBadge`]. Mirrors the upstream `BadgeVariant` union.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum BadgeVariant {
    #[default]
    Default,
    Primary,
    Success,
    Warning,
    Danger,
    Info,
}

impl BadgeVariant {
    /// The string the `<hmi-badge>` `variant` attribute expects.
    pub fn as_str(self) -> &'static str {
        match self {
            BadgeVariant::Default => "default",
            BadgeVariant::Primary => "primary",
            BadgeVariant::Success => "success",
            BadgeVariant::Warning => "warning",
            BadgeVariant::Danger => "danger",
            BadgeVariant::Info => "info",
        }
    }
}

/// Typed wrapper for the `<hmi-badge>` web component.
#[component]
pub fn HmiBadge(
    #[props(default)] variant: BadgeVariant,
    /// Renders the badge as a small status dot.
    #[props(default)]
    dot: bool,
    children: Element,
) -> Element {
    rsx! {
        hmi-badge { "variant": variant.as_str(), "dot": if dot { "true" }, {children} }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn variant_as_str_maps_every_value() {
        assert_eq!(BadgeVariant::Default.as_str(), "default");
        assert_eq!(BadgeVariant::Primary.as_str(), "primary");
        assert_eq!(BadgeVariant::Success.as_str(), "success");
        assert_eq!(BadgeVariant::Warning.as_str(), "warning");
        assert_eq!(BadgeVariant::Danger.as_str(), "danger");
        assert_eq!(BadgeVariant::Info.as_str(), "info");
    }

    #[test]
    fn variant_defaults_to_default() {
        assert_eq!(BadgeVariant::default(), BadgeVariant::Default);
    }
}
