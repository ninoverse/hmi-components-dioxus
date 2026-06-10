use dioxus::prelude::*;

/// Size of an [`HmiSpacer`]. Mirrors the upstream `SpacerSize` union.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum SpacerSize {
    Small,
    #[default]
    Medium,
    Large,
}

impl SpacerSize {
    /// The string the `<hmi-spacer>` `size` attribute expects.
    pub fn as_str(self) -> &'static str {
        match self {
            SpacerSize::Small => "small",
            SpacerSize::Medium => "medium",
            SpacerSize::Large => "large",
        }
    }
}

/// Axis an [`HmiSpacer`] adds space along. Mirrors the upstream `SpacerAxis` union.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum SpacerAxis {
    #[default]
    Vertical,
    Horizontal,
}

impl SpacerAxis {
    /// The string the `<hmi-spacer>` `axis` attribute expects.
    pub fn as_str(self) -> &'static str {
        match self {
            SpacerAxis::Vertical => "vertical",
            SpacerAxis::Horizontal => "horizontal",
        }
    }
}

/// Typed wrapper for the `<hmi-spacer>` web component.
///
/// Inserts blank space sized by `size` along `axis`. Setting `grow` makes it
/// expand to fill the available space in a flex container, ignoring
/// `size`/`axis`.
#[component]
pub fn HmiSpacer(
    #[props(default)] size: SpacerSize,
    #[props(default)] axis: SpacerAxis,
    #[props(default)] grow: bool,
) -> Element {
    rsx! {
        hmi-spacer {
            "size": size.as_str(),
            "axis": axis.as_str(),
            "grow": if grow { "true" },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn size_as_str_maps_every_value() {
        assert_eq!(SpacerSize::Small.as_str(), "small");
        assert_eq!(SpacerSize::Medium.as_str(), "medium");
        assert_eq!(SpacerSize::Large.as_str(), "large");
    }

    #[test]
    fn axis_as_str_maps_every_value() {
        assert_eq!(SpacerAxis::Vertical.as_str(), "vertical");
        assert_eq!(SpacerAxis::Horizontal.as_str(), "horizontal");
    }

    #[test]
    fn defaults_match_upstream() {
        assert_eq!(SpacerSize::default(), SpacerSize::Medium);
        assert_eq!(SpacerAxis::default(), SpacerAxis::Vertical);
    }
}
