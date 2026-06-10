use dioxus::prelude::*;

/// Shape of an [`HmiSkeleton`] placeholder. Mirrors the upstream `SkeletonVariant` union.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum SkeletonVariant {
    #[default]
    Text,
    Rect,
    Circle,
}

impl SkeletonVariant {
    /// The string the `<hmi-skeleton>` `variant` attribute expects.
    pub fn as_str(self) -> &'static str {
        match self {
            SkeletonVariant::Text => "text",
            SkeletonVariant::Rect => "rect",
            SkeletonVariant::Circle => "circle",
        }
    }
}

/// Typed wrapper for the `<hmi-skeleton>` web component.
///
/// A loading placeholder. `width`/`height`/`radius` accept any CSS length
/// (e.g. `"200px"`, `"100%"`).
#[component]
pub fn HmiSkeleton(
    #[props(default)] variant: SkeletonVariant,
    width: Option<String>,
    height: Option<String>,
    radius: Option<String>,
) -> Element {
    rsx! {
        hmi-skeleton {
            "variant": variant.as_str(),
            "width": width,
            "height": height,
            "radius": radius,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn variant_as_str_maps_every_value() {
        assert_eq!(SkeletonVariant::Text.as_str(), "text");
        assert_eq!(SkeletonVariant::Rect.as_str(), "rect");
        assert_eq!(SkeletonVariant::Circle.as_str(), "circle");
    }

    #[test]
    fn variant_defaults_to_text() {
        assert_eq!(SkeletonVariant::default(), SkeletonVariant::Text);
    }
}
