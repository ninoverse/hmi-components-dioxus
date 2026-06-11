use super::json_string;
use dioxus::prelude::*;

/// Direction of an [`HmiStat`] delta. Mirrors the upstream `trend` values and
/// drives the delta arrow and colour.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum StatTrend {
    Up,
    Down,
}

impl StatTrend {
    /// The string the `<hmi-stat>` `trend` attribute expects.
    pub fn as_str(self) -> &'static str {
        match self {
            StatTrend::Up => "up",
            StatTrend::Down => "down",
        }
    }
}

/// Typed wrapper for the `<hmi-stat>` web component.
///
/// Renders a labelled metric. All text is supplied via props (the element has
/// no default slot): `value` is the headline figure, `label` describes it, and
/// `delta`/`trend` render an optional change indicator. The upstream `icon`
/// slot is omitted.
#[component]
pub fn HmiStat(
    /// Headline figure, e.g. `"$12,400"`.
    value: Option<String>,
    /// Caption describing the value.
    label: Option<String>,
    /// Change indicator text, e.g. `"+12%"`.
    delta: Option<String>,
    /// Direction the `delta` represents.
    trend: Option<StatTrend>,
    /// Secondary helper text shown beneath the value.
    help_text: Option<String>,
) -> Element {
    rsx! {
        hmi-stat {
            // `value` is special-cased by Dioxus and set as a DOM property, which
            // bypasses the element's `JSON.parse`, so it must be passed unencoded
            // — unlike the other props, which arrive as JSON-typed attributes.
            "value": value,
            "label": label.as_deref().map(json_string),
            "delta": delta.as_deref().map(json_string),
            "trend": trend.map(StatTrend::as_str),
            // The element observes the kebab-cased attribute name.
            "help-text": help_text.as_deref().map(json_string),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trend_as_str_maps_every_value() {
        assert_eq!(StatTrend::Up.as_str(), "up");
        assert_eq!(StatTrend::Down.as_str(), "down");
    }
}
