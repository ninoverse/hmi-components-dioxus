use dioxus::prelude::*;

/// Visual size of an [`HmiHeading`], independent of its semantic `level`.
/// Mirrors the upstream `heading--size-*` modifiers.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum HeadingSize {
    Xsmall,
    Small,
    Medium,
    Large,
    Xlarge,
}

impl HeadingSize {
    /// The string the `<hmi-heading>` `size` attribute expects.
    pub fn as_str(self) -> &'static str {
        match self {
            HeadingSize::Xsmall => "xsmall",
            HeadingSize::Small => "small",
            HeadingSize::Medium => "medium",
            HeadingSize::Large => "large",
            HeadingSize::Xlarge => "xlarge",
        }
    }
}

/// Color tone of an [`HmiHeading`]. Mirrors the upstream `tone` attribute.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum HeadingTone {
    #[default]
    Default,
    Inherit,
    Muted,
    Primary,
}

impl HeadingTone {
    /// The string the `<hmi-heading>` `tone` attribute expects.
    pub fn as_str(self) -> &'static str {
        match self {
            HeadingTone::Default => "default",
            HeadingTone::Inherit => "inherit",
            HeadingTone::Muted => "muted",
            HeadingTone::Primary => "primary",
        }
    }
}

/// Typed wrapper for the `<hmi-heading>` web component.
///
/// `level` selects the semantic element (`<h1>`–`<h6>`). When `size` is
/// omitted the upstream derives it from `level`.
#[component]
pub fn HmiHeading(
    #[props(default = 2)] level: u8,
    size: Option<HeadingSize>,
    #[props(default)] tone: HeadingTone,
    #[props(default)] truncate: bool,
    children: Element,
) -> Element {
    rsx! {
        hmi-heading {
            "level": level as i64,
            "size": size.map(HeadingSize::as_str),
            "tone": tone.as_str(),
            "truncate": if truncate { "true" },
            {children}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn size_as_str_maps_every_value() {
        assert_eq!(HeadingSize::Xsmall.as_str(), "xsmall");
        assert_eq!(HeadingSize::Small.as_str(), "small");
        assert_eq!(HeadingSize::Medium.as_str(), "medium");
        assert_eq!(HeadingSize::Large.as_str(), "large");
        assert_eq!(HeadingSize::Xlarge.as_str(), "xlarge");
    }

    #[test]
    fn tone_as_str_maps_every_value() {
        assert_eq!(HeadingTone::Default.as_str(), "default");
        assert_eq!(HeadingTone::Inherit.as_str(), "inherit");
        assert_eq!(HeadingTone::Muted.as_str(), "muted");
        assert_eq!(HeadingTone::Primary.as_str(), "primary");
    }

    #[test]
    fn tone_defaults_to_default() {
        assert_eq!(HeadingTone::default(), HeadingTone::Default);
    }
}
