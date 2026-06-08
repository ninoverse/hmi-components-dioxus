use dioxus::prelude::*;

/// Visual style of an [`HmiButton`]. Mirrors the upstream `ButtonVariant` union.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum ButtonVariant {
    #[default]
    Primary,
    Secondary,
    Ghost,
    Soft,
    Danger,
    Link,
}

impl ButtonVariant {
    /// The string the `<hmi-button>` `variant` attribute expects.
    pub fn as_str(self) -> &'static str {
        match self {
            ButtonVariant::Primary => "primary",
            ButtonVariant::Secondary => "secondary",
            ButtonVariant::Ghost => "ghost",
            ButtonVariant::Soft => "soft",
            ButtonVariant::Danger => "danger",
            ButtonVariant::Link => "link",
        }
    }
}

/// Size of an [`HmiButton`]. Mirrors the upstream `ButtonSize` union.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum ButtonSize {
    Small,
    #[default]
    Medium,
    Large,
}

impl ButtonSize {
    /// The string the `<hmi-button>` `size` attribute expects.
    pub fn as_str(self) -> &'static str {
        match self {
            ButtonSize::Small => "small",
            ButtonSize::Medium => "medium",
            ButtonSize::Large => "large",
        }
    }
}

/// The native `type` of an [`HmiButton`].
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum ButtonType {
    #[default]
    Button,
    Submit,
    Reset,
}

impl ButtonType {
    /// The string the `<hmi-button>` `type` attribute expects.
    pub fn as_str(self) -> &'static str {
        match self {
            ButtonType::Button => "button",
            ButtonType::Submit => "submit",
            ButtonType::Reset => "reset",
        }
    }
}

/// Typed wrapper for the `<hmi-button>` web component.
///
/// The `onSelect`/click behaviour is handled by attaching standard event
/// handlers to the rendered element; the upstream `leftIcon`/`rightIcon` props
/// are JSON-encoded node descriptors passed through verbatim.
#[component]
pub fn HmiButton(
    #[props(default)] variant: ButtonVariant,
    #[props(default)] size: ButtonSize,
    #[props(default)] r#type: ButtonType,
    #[props(default)] disabled: bool,
    /// Render the button as a square icon-only control.
    #[props(default)]
    as_icon: bool,
    /// JSON-encoded icon descriptor rendered before the label.
    #[props(default)]
    left_icon: Option<String>,
    /// JSON-encoded icon descriptor rendered after the label.
    #[props(default)]
    right_icon: Option<String>,
    children: Element,
) -> Element {
    rsx! {
        hmi-button {
            "variant": variant.as_str(),
            "size": size.as_str(),
            "type": r#type.as_str(),
            "disabled": if disabled { "true" },
            "as-icon": if as_icon { "true" },
            "left-icon": left_icon,
            "right-icon": right_icon,
            {children}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn variant_as_str_maps_every_value() {
        assert_eq!(ButtonVariant::Primary.as_str(), "primary");
        assert_eq!(ButtonVariant::Secondary.as_str(), "secondary");
        assert_eq!(ButtonVariant::Ghost.as_str(), "ghost");
        assert_eq!(ButtonVariant::Soft.as_str(), "soft");
        assert_eq!(ButtonVariant::Danger.as_str(), "danger");
        assert_eq!(ButtonVariant::Link.as_str(), "link");
    }

    #[test]
    fn size_as_str_maps_every_value() {
        assert_eq!(ButtonSize::Small.as_str(), "small");
        assert_eq!(ButtonSize::Medium.as_str(), "medium");
        assert_eq!(ButtonSize::Large.as_str(), "large");
    }

    #[test]
    fn type_as_str_maps_every_value() {
        assert_eq!(ButtonType::Button.as_str(), "button");
        assert_eq!(ButtonType::Submit.as_str(), "submit");
        assert_eq!(ButtonType::Reset.as_str(), "reset");
    }

    #[test]
    fn defaults_match_upstream() {
        assert_eq!(ButtonVariant::default(), ButtonVariant::Primary);
        assert_eq!(ButtonSize::default(), ButtonSize::Medium);
        assert_eq!(ButtonType::default(), ButtonType::Button);
    }
}
