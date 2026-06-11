use super::json_string;
use dioxus::prelude::*;

/// How an [`HmiImage`] fills its box. Mirrors the CSS `object-fit` values the
/// upstream `fit` attribute accepts.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum ImageFit {
    #[default]
    Cover,
    Contain,
    Fill,
    None,
    ScaleDown,
}

impl ImageFit {
    /// The string the `<hmi-image>` `fit` attribute expects.
    pub fn as_str(self) -> &'static str {
        match self {
            ImageFit::Cover => "cover",
            ImageFit::Contain => "contain",
            ImageFit::Fill => "fill",
            ImageFit::None => "none",
            ImageFit::ScaleDown => "scale-down",
        }
    }
}

/// Corner radius of an [`HmiImage`]. Mirrors the upstream `image--radius-*` modifiers.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum ImageRadius {
    #[default]
    None,
    Small,
    Medium,
    Large,
    Full,
}

impl ImageRadius {
    /// The string the `<hmi-image>` `radius` attribute expects.
    pub fn as_str(self) -> &'static str {
        match self {
            ImageRadius::None => "none",
            ImageRadius::Small => "small",
            ImageRadius::Medium => "medium",
            ImageRadius::Large => "large",
            ImageRadius::Full => "full",
        }
    }
}

/// Browser loading strategy for an [`HmiImage`]. Mirrors the upstream `loading`
/// attribute (the native `<img loading>` values).
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum ImageLoading {
    #[default]
    Lazy,
    Eager,
}

impl ImageLoading {
    /// The string the `<hmi-image>` `loading` attribute expects.
    pub fn as_str(self) -> &'static str {
        match self {
            ImageLoading::Lazy => "lazy",
            ImageLoading::Eager => "eager",
        }
    }
}

/// Typed wrapper for the `<hmi-image>` web component.
///
/// `width`/`height` accept any CSS length; `ratio` sets an aspect ratio (e.g.
/// `16.0 / 9.0`). `fallback` is shown if `src` fails to load.
#[component]
pub fn HmiImage(
    src: String,
    alt: Option<String>,
    #[props(default)] fit: ImageFit,
    #[props(default)] radius: ImageRadius,
    /// Aspect ratio, e.g. `16.0 / 9.0`.
    ratio: Option<f64>,
    width: Option<String>,
    height: Option<String>,
    #[props(default)] loading: ImageLoading,
    /// Image shown if `src` fails to load.
    fallback: Option<String>,
) -> Element {
    rsx! {
        hmi-image {
            "src": src,
            "alt": alt,
            "fit": fit.as_str(),
            "radius": radius.as_str(),
            "ratio": ratio,
            "width": width,
            "height": height,
            "loading": loading.as_str(),
            "fallback": fallback.as_deref().map(json_string),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fit_as_str_maps_every_value() {
        assert_eq!(ImageFit::Cover.as_str(), "cover");
        assert_eq!(ImageFit::Contain.as_str(), "contain");
        assert_eq!(ImageFit::Fill.as_str(), "fill");
        assert_eq!(ImageFit::None.as_str(), "none");
        assert_eq!(ImageFit::ScaleDown.as_str(), "scale-down");
    }

    #[test]
    fn radius_as_str_maps_every_value() {
        assert_eq!(ImageRadius::None.as_str(), "none");
        assert_eq!(ImageRadius::Small.as_str(), "small");
        assert_eq!(ImageRadius::Medium.as_str(), "medium");
        assert_eq!(ImageRadius::Large.as_str(), "large");
        assert_eq!(ImageRadius::Full.as_str(), "full");
    }

    #[test]
    fn loading_as_str_maps_every_value() {
        assert_eq!(ImageLoading::Lazy.as_str(), "lazy");
        assert_eq!(ImageLoading::Eager.as_str(), "eager");
    }

    #[test]
    fn defaults_match_upstream() {
        assert_eq!(ImageFit::default(), ImageFit::Cover);
        assert_eq!(ImageRadius::default(), ImageRadius::None);
        assert_eq!(ImageLoading::default(), ImageLoading::Lazy);
    }
}
