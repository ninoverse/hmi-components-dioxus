use dioxus::prelude::*;

/// Orientation of an [`HmiDivider`]. Mirrors the upstream `orientation` attribute.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum DividerOrientation {
    #[default]
    Horizontal,
    Vertical,
}

impl DividerOrientation {
    /// The string the `<hmi-divider>` `orientation` attribute expects.
    pub fn as_str(self) -> &'static str {
        match self {
            DividerOrientation::Horizontal => "horizontal",
            DividerOrientation::Vertical => "vertical",
        }
    }
}

/// Label alignment of a horizontal [`HmiDivider`]. Mirrors the upstream `align` attribute.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum DividerAlign {
    Start,
    #[default]
    Center,
    End,
}

impl DividerAlign {
    /// The string the `<hmi-divider>` `align` attribute expects.
    pub fn as_str(self) -> &'static str {
        match self {
            DividerAlign::Start => "start",
            DividerAlign::Center => "center",
            DividerAlign::End => "end",
        }
    }
}

/// Typed wrapper for the `<hmi-divider>` web component.
///
/// Passing `children` on a horizontal divider renders them as a centered label;
/// `align` controls the label position and is ignored when vertical.
#[component]
pub fn HmiDivider(
    #[props(default)] orientation: DividerOrientation,
    #[props(default)] align: DividerAlign,
    children: Element,
) -> Element {
    rsx! {
        hmi-divider {
            "orientation": orientation.as_str(),
            "align": align.as_str(),
            {children}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn orientation_as_str_maps_every_value() {
        assert_eq!(DividerOrientation::Horizontal.as_str(), "horizontal");
        assert_eq!(DividerOrientation::Vertical.as_str(), "vertical");
    }

    #[test]
    fn align_as_str_maps_every_value() {
        assert_eq!(DividerAlign::Start.as_str(), "start");
        assert_eq!(DividerAlign::Center.as_str(), "center");
        assert_eq!(DividerAlign::End.as_str(), "end");
    }

    #[test]
    fn defaults_match_upstream() {
        assert_eq!(DividerOrientation::default(), DividerOrientation::Horizontal);
        assert_eq!(DividerAlign::default(), DividerAlign::Center);
    }
}
