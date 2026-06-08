use dioxus::prelude::*;

/// Font size of an [`HmiText`]. Mirrors the upstream `text--size-*` modifiers.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum TextSize {
    Xsmall,
    Small,
    #[default]
    Medium,
    Large,
    Xlarge,
}

impl TextSize {
    /// The string the `<hmi-text>` `size` attribute expects.
    pub fn as_str(self) -> &'static str {
        match self {
            TextSize::Xsmall => "xsmall",
            TextSize::Small => "small",
            TextSize::Medium => "medium",
            TextSize::Large => "large",
            TextSize::Xlarge => "xlarge",
        }
    }
}

/// Font weight of an [`HmiText`]. Mirrors the upstream `text--weight-*` modifiers.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum TextWeight {
    #[default]
    Regular,
    Medium,
    Semibold,
    Bold,
}

impl TextWeight {
    /// The string the `<hmi-text>` `weight` attribute expects.
    pub fn as_str(self) -> &'static str {
        match self {
            TextWeight::Regular => "regular",
            TextWeight::Medium => "medium",
            TextWeight::Semibold => "semibold",
            TextWeight::Bold => "bold",
        }
    }
}

/// Color tone of an [`HmiText`]. Mirrors the upstream `tone` attribute.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum TextTone {
    #[default]
    Default,
    Inherit,
    Muted,
    Primary,
    Error,
}

impl TextTone {
    /// The string the `<hmi-text>` `tone` attribute expects.
    pub fn as_str(self) -> &'static str {
        match self {
            TextTone::Default => "default",
            TextTone::Inherit => "inherit",
            TextTone::Muted => "muted",
            TextTone::Primary => "primary",
            TextTone::Error => "error",
        }
    }
}

/// Text alignment of an [`HmiText`]. Mirrors the upstream `align` attribute.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum TextAlign {
    Start,
    Center,
    End,
}

impl TextAlign {
    /// The string the `<hmi-text>` `align` attribute expects.
    pub fn as_str(self) -> &'static str {
        match self {
            TextAlign::Start => "start",
            TextAlign::Center => "center",
            TextAlign::End => "end",
        }
    }
}

/// Typed wrapper for the `<hmi-text>` web component.
///
/// `tag` maps to the upstream `as` attribute (renamed because `as` is a Rust
/// keyword) and selects the rendered element; the upstream defaults to `<p>`.
#[component]
pub fn HmiText(
    /// HTML element to render as (e.g. `"span"`, `"label"`); defaults to `"p"`.
    tag: Option<String>,
    #[props(default)] size: TextSize,
    #[props(default)] weight: TextWeight,
    #[props(default)] tone: TextTone,
    align: Option<TextAlign>,
    #[props(default)] truncate: bool,
    children: Element,
) -> Element {
    rsx! {
        hmi-text {
            "as": tag,
            "size": size.as_str(),
            "weight": weight.as_str(),
            "tone": tone.as_str(),
            "align": align.map(TextAlign::as_str),
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
        assert_eq!(TextSize::Xsmall.as_str(), "xsmall");
        assert_eq!(TextSize::Small.as_str(), "small");
        assert_eq!(TextSize::Medium.as_str(), "medium");
        assert_eq!(TextSize::Large.as_str(), "large");
        assert_eq!(TextSize::Xlarge.as_str(), "xlarge");
    }

    #[test]
    fn weight_as_str_maps_every_value() {
        assert_eq!(TextWeight::Regular.as_str(), "regular");
        assert_eq!(TextWeight::Medium.as_str(), "medium");
        assert_eq!(TextWeight::Semibold.as_str(), "semibold");
        assert_eq!(TextWeight::Bold.as_str(), "bold");
    }

    #[test]
    fn tone_as_str_maps_every_value() {
        assert_eq!(TextTone::Default.as_str(), "default");
        assert_eq!(TextTone::Inherit.as_str(), "inherit");
        assert_eq!(TextTone::Muted.as_str(), "muted");
        assert_eq!(TextTone::Primary.as_str(), "primary");
        assert_eq!(TextTone::Error.as_str(), "error");
    }

    #[test]
    fn align_as_str_maps_every_value() {
        assert_eq!(TextAlign::Start.as_str(), "start");
        assert_eq!(TextAlign::Center.as_str(), "center");
        assert_eq!(TextAlign::End.as_str(), "end");
    }

    #[test]
    fn defaults_match_upstream() {
        assert_eq!(TextSize::default(), TextSize::Medium);
        assert_eq!(TextWeight::default(), TextWeight::Regular);
        assert_eq!(TextTone::default(), TextTone::Default);
    }
}
