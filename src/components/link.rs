use dioxus::prelude::*;

/// Underline behaviour of an [`HmiLink`]. Mirrors the upstream `LinkUnderline` union.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum LinkUnderline {
    #[default]
    Always,
    Hover,
    None,
}

impl LinkUnderline {
    /// The string the `<hmi-link>` `underline` attribute expects.
    pub fn as_str(self) -> &'static str {
        match self {
            LinkUnderline::Always => "always",
            LinkUnderline::Hover => "hover",
            LinkUnderline::None => "none",
        }
    }
}

/// Color tone of an [`HmiLink`]. Mirrors the upstream `LinkTone` union.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum LinkTone {
    #[default]
    Primary,
    Muted,
}

impl LinkTone {
    /// The string the `<hmi-link>` `tone` attribute expects.
    pub fn as_str(self) -> &'static str {
        match self {
            LinkTone::Primary => "primary",
            LinkTone::Muted => "muted",
        }
    }
}

/// Typed wrapper for the `<hmi-link>` web component.
///
/// Renders an anchor. When `target` is `"_blank"` the upstream defaults `rel`
/// to `"noopener noreferrer"` unless overridden.
#[component]
pub fn HmiLink(
    href: Option<String>,
    target: Option<String>,
    rel: Option<String>,
    #[props(default)] underline: LinkUnderline,
    #[props(default)] tone: LinkTone,
    children: Element,
) -> Element {
    rsx! {
        hmi-link {
            "href": href,
            "target": target,
            "rel": rel,
            "underline": underline.as_str(),
            "tone": tone.as_str(),
            {children}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn underline_as_str_maps_every_value() {
        assert_eq!(LinkUnderline::Always.as_str(), "always");
        assert_eq!(LinkUnderline::Hover.as_str(), "hover");
        assert_eq!(LinkUnderline::None.as_str(), "none");
    }

    #[test]
    fn tone_as_str_maps_every_value() {
        assert_eq!(LinkTone::Primary.as_str(), "primary");
        assert_eq!(LinkTone::Muted.as_str(), "muted");
    }

    #[test]
    fn defaults_match_upstream() {
        assert_eq!(LinkUnderline::default(), LinkUnderline::Always);
        assert_eq!(LinkTone::default(), LinkTone::Primary);
    }
}
