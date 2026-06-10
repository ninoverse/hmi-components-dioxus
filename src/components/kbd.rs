use dioxus::prelude::*;

/// Size of an [`HmiKbd`]. Mirrors the upstream `KbdSize` union.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum KbdSize {
    Small,
    #[default]
    Medium,
}

impl KbdSize {
    /// The string the `<hmi-kbd>` `size` attribute expects.
    pub fn as_str(self) -> &'static str {
        match self {
            KbdSize::Small => "small",
            KbdSize::Medium => "medium",
        }
    }
}

/// Typed wrapper for the `<hmi-kbd>` web component.
///
/// Renders a keyboard key. Pass the key label as `children`.
#[component]
pub fn HmiKbd(#[props(default)] size: KbdSize, children: Element) -> Element {
    rsx! {
        hmi-kbd { "size": size.as_str(), {children} }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn size_as_str_maps_every_value() {
        assert_eq!(KbdSize::Small.as_str(), "small");
        assert_eq!(KbdSize::Medium.as_str(), "medium");
    }

    #[test]
    fn size_defaults_to_medium() {
        assert_eq!(KbdSize::default(), KbdSize::Medium);
    }
}
