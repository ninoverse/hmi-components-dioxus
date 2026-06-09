use dioxus::prelude::*;

/// Size of an [`HmiSpinner`]. Mirrors the upstream `spinner--*` modifiers.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum SpinnerSize {
    Small,
    #[default]
    Medium,
    Large,
}

impl SpinnerSize {
    /// The string the `<hmi-spinner>` `size` attribute expects.
    pub fn as_str(self) -> &'static str {
        match self {
            SpinnerSize::Small => "small",
            SpinnerSize::Medium => "medium",
            SpinnerSize::Large => "large",
        }
    }
}

/// Typed wrapper for the `<hmi-spinner>` web component.
///
/// `label` sets the accessible status text; the upstream defaults to
/// `"Loading"` when omitted.
#[component]
pub fn HmiSpinner(#[props(default)] size: SpinnerSize, label: Option<String>) -> Element {
    rsx! {
        hmi-spinner { "size": size.as_str(), "label": label }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn size_as_str_maps_every_value() {
        assert_eq!(SpinnerSize::Small.as_str(), "small");
        assert_eq!(SpinnerSize::Medium.as_str(), "medium");
        assert_eq!(SpinnerSize::Large.as_str(), "large");
    }

    #[test]
    fn size_defaults_to_medium() {
        assert_eq!(SpinnerSize::default(), SpinnerSize::Medium);
    }
}
