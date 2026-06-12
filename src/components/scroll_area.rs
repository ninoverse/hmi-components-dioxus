use dioxus::prelude::*;

/// Scroll direction of an [`HmiScrollArea`].
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum ScrollOrientation {
    #[default]
    Vertical,
    Horizontal,
}

impl ScrollOrientation {
    pub fn as_str(self) -> &'static str {
        match self {
            ScrollOrientation::Vertical => "vertical",
            ScrollOrientation::Horizontal => "horizontal",
        }
    }
}

/// Typed wrapper for the `<hmi-scroll-area>` web component.
///
/// A styled scroll container for its `children`. `max_height` (any CSS length,
/// e.g. `"12rem"`) caps the visible area before scrolling kicks in.
#[component]
pub fn HmiScrollArea(
    #[props(default)] orientation: ScrollOrientation,
    /// Maximum height before scrolling, e.g. `"12rem"`.
    max_height: Option<String>,
    children: Element,
) -> Element {
    rsx! {
        hmi-scroll-area {
            "orientation": orientation.as_str(),
            "max-height": max_height,
            {children}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn orientation_as_str() {
        assert_eq!(ScrollOrientation::Vertical.as_str(), "vertical");
        assert_eq!(ScrollOrientation::Horizontal.as_str(), "horizontal");
    }
}
