use dioxus::prelude::*;

/// Background surface of an [`HmiBox`]. Mirrors the upstream `box--bg-*` modifiers.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum BoxBackground {
    Surface,
    SurfaceContainer,
    SurfaceContainerHigh,
    SurfaceContainerLow,
    SurfaceVariant,
}

impl BoxBackground {
    /// The string the `<hmi-box>` `background` attribute expects.
    pub fn as_str(self) -> &'static str {
        match self {
            BoxBackground::Surface => "surface",
            BoxBackground::SurfaceContainer => "surface-container",
            BoxBackground::SurfaceContainerHigh => "surface-container-high",
            BoxBackground::SurfaceContainerLow => "surface-container-low",
            BoxBackground::SurfaceVariant => "surface-variant",
        }
    }
}

/// Inner padding of an [`HmiBox`]. Mirrors the upstream `box--pad-*` modifiers.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum BoxPadding {
    Small,
    Medium,
    Large,
}

impl BoxPadding {
    /// The string the `<hmi-box>` `padding` attribute expects.
    pub fn as_str(self) -> &'static str {
        match self {
            BoxPadding::Small => "small",
            BoxPadding::Medium => "medium",
            BoxPadding::Large => "large",
        }
    }
}

/// Corner radius of an [`HmiBox`]. Mirrors the upstream `box--radius-*` modifiers.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum BoxRadius {
    Small,
    Medium,
    Large,
    Full,
    Leaf,
}

impl BoxRadius {
    /// The string the `<hmi-box>` `radius` attribute expects.
    pub fn as_str(self) -> &'static str {
        match self {
            BoxRadius::Small => "small",
            BoxRadius::Medium => "medium",
            BoxRadius::Large => "large",
            BoxRadius::Full => "full",
            BoxRadius::Leaf => "leaf",
        }
    }
}

/// Typed wrapper for the `<hmi-box>` web component.
///
/// A generic surface container. `tag` maps to the upstream `as` attribute
/// (renamed because `as` is a Rust keyword) and selects the rendered element;
/// the upstream defaults to `<div>`.
#[component]
pub fn HmiBox(
    /// HTML element to render as (e.g. `"section"`); defaults to `"div"`.
    tag: Option<String>,
    background: Option<BoxBackground>,
    padding: Option<BoxPadding>,
    radius: Option<BoxRadius>,
    #[props(default)] bordered: bool,
    children: Element,
) -> Element {
    rsx! {
        // `hmibox` resolves through the local `dioxus_elements` shim below to the
        // literal `hmi-box` tag — see that module for why the hyphenated form
        // cannot be used here.
        hmibox {
            "as": tag,
            "background": background.map(BoxBackground::as_str),
            "padding": padding.map(BoxPadding::as_str),
            "radius": radius.map(BoxRadius::as_str),
            "bordered": if bordered { "true" },
            {children}
        }
    }
}

/// Shim that lets `box.rs` emit the literal `hmi-box` custom-element tag.
///
/// Writing `hmi-box { .. }` in `rsx!` does not work: the macro parses the
/// hyphenated name as raw identifiers and joins their `to_string()` values, so
/// the `box` keyword segment becomes `r#box` and the rendered tag is the
/// unregistered `hmi-r#box`. `rsx!` only trusts an explicit `TAG_NAME` for a
/// single-identifier element resolved through `dioxus_elements`, so this module
/// shadows that path (a local item shadows the `dioxus::prelude::*` glob) and
/// maps the non-keyword identifier `hmibox` to the correct tag.
mod dioxus_elements {
    pub mod elements {
        pub mod hmibox {
            pub const TAG_NAME: &str = "hmi-box";
            pub const NAME_SPACE: Option<&'static str> = None;
        }
    }
    // `rsx!` reads the namespace as `dioxus_elements::hmibox::NAME_SPACE`.
    pub use elements::*;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn background_as_str_maps_every_value() {
        assert_eq!(BoxBackground::Surface.as_str(), "surface");
        assert_eq!(
            BoxBackground::SurfaceContainer.as_str(),
            "surface-container"
        );
        assert_eq!(
            BoxBackground::SurfaceContainerHigh.as_str(),
            "surface-container-high"
        );
        assert_eq!(
            BoxBackground::SurfaceContainerLow.as_str(),
            "surface-container-low"
        );
        assert_eq!(BoxBackground::SurfaceVariant.as_str(), "surface-variant");
    }

    #[test]
    fn padding_as_str_maps_every_value() {
        assert_eq!(BoxPadding::Small.as_str(), "small");
        assert_eq!(BoxPadding::Medium.as_str(), "medium");
        assert_eq!(BoxPadding::Large.as_str(), "large");
    }

    #[test]
    fn radius_as_str_maps_every_value() {
        assert_eq!(BoxRadius::Small.as_str(), "small");
        assert_eq!(BoxRadius::Medium.as_str(), "medium");
        assert_eq!(BoxRadius::Large.as_str(), "large");
        assert_eq!(BoxRadius::Full.as_str(), "full");
        assert_eq!(BoxRadius::Leaf.as_str(), "leaf");
    }
}
