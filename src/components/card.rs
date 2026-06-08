use dioxus::prelude::*;

/// Visual style of an [`HmiCard`]. Mirrors the upstream `card--*` modifiers.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum CardVariant {
    #[default]
    Default,
    Flat,
    Ink,
    Accent,
}

impl CardVariant {
    /// The string the `<hmi-card>` `variant` attribute expects.
    pub fn as_str(self) -> &'static str {
        match self {
            CardVariant::Default => "default",
            CardVariant::Flat => "flat",
            CardVariant::Ink => "ink",
            CardVariant::Accent => "accent",
        }
    }
}

/// Typed wrapper for the `<hmi-card>` web component.
#[component]
pub fn HmiCard(
    #[props(default)] variant: CardVariant,
    children: Element,
) -> Element {
    rsx! {
        hmi-card { "variant": variant.as_str(), {children} }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn variant_as_str_maps_every_value() {
        assert_eq!(CardVariant::Default.as_str(), "default");
        assert_eq!(CardVariant::Flat.as_str(), "flat");
        assert_eq!(CardVariant::Ink.as_str(), "ink");
        assert_eq!(CardVariant::Accent.as_str(), "accent");
    }

    #[test]
    fn variant_defaults_to_default() {
        assert_eq!(CardVariant::default(), CardVariant::Default);
    }
}
