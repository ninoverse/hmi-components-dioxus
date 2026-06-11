use dioxus::prelude::*;

/// Main-axis direction of an [`HmiFlex`]. Mirrors the upstream `flex--*` modifiers.
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum FlexDirection {
    #[default]
    Row,
    Column,
    RowReverse,
    ColumnReverse,
}

impl FlexDirection {
    /// The string the `<hmi-flex>` `direction` attribute expects.
    pub fn as_str(self) -> &'static str {
        match self {
            FlexDirection::Row => "row",
            FlexDirection::Column => "column",
            FlexDirection::RowReverse => "row-reverse",
            FlexDirection::ColumnReverse => "column-reverse",
        }
    }
}

/// Cross-axis alignment of an [`HmiFlex`]. Mirrors the upstream `flex--align-*` modifiers.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum FlexAlign {
    Start,
    Center,
    End,
    Baseline,
}

impl FlexAlign {
    /// The string the `<hmi-flex>` `align` attribute expects.
    pub fn as_str(self) -> &'static str {
        match self {
            FlexAlign::Start => "start",
            FlexAlign::Center => "center",
            FlexAlign::End => "end",
            FlexAlign::Baseline => "baseline",
        }
    }
}

/// Main-axis distribution of an [`HmiFlex`]. Mirrors the upstream `flex--justify-*` modifiers.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum FlexJustify {
    Start,
    Center,
    End,
    Between,
    Around,
    Evenly,
}

impl FlexJustify {
    /// The string the `<hmi-flex>` `justify` attribute expects.
    pub fn as_str(self) -> &'static str {
        match self {
            FlexJustify::Start => "start",
            FlexJustify::Center => "center",
            FlexJustify::End => "end",
            FlexJustify::Between => "between",
            FlexJustify::Around => "around",
            FlexJustify::Evenly => "evenly",
        }
    }
}

/// Gap between children of an [`HmiFlex`]. Mirrors the upstream `flex--gap-*` modifiers.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum FlexGap {
    Small,
    Medium,
    Large,
}

impl FlexGap {
    /// The string the `<hmi-flex>` `gap` attribute expects.
    pub fn as_str(self) -> &'static str {
        match self {
            FlexGap::Small => "small",
            FlexGap::Medium => "medium",
            FlexGap::Large => "large",
        }
    }
}

/// Typed wrapper for the `<hmi-flex>` web component.
///
/// A flexbox layout primitive. `tag` maps to the upstream `as` attribute
/// (renamed because `as` is a Rust keyword) and selects the rendered element;
/// the upstream defaults to `<div>`.
#[component]
pub fn HmiFlex(
    /// HTML element to render as (e.g. `"nav"`); defaults to `"div"`.
    tag: Option<String>,
    #[props(default)] direction: FlexDirection,
    align: Option<FlexAlign>,
    justify: Option<FlexJustify>,
    gap: Option<FlexGap>,
    /// Allow children to wrap onto multiple lines.
    #[props(default)]
    wrap: bool,
    /// Render as an inline flex container.
    #[props(default)]
    inline: bool,
    children: Element,
) -> Element {
    rsx! {
        hmi-flex {
            "as": tag,
            "direction": direction.as_str(),
            "align": align.map(FlexAlign::as_str),
            "justify": justify.map(FlexJustify::as_str),
            "gap": gap.map(FlexGap::as_str),
            "wrap": if wrap { "true" },
            "inline": if inline { "true" },
            {children}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn direction_as_str_maps_every_value() {
        assert_eq!(FlexDirection::Row.as_str(), "row");
        assert_eq!(FlexDirection::Column.as_str(), "column");
        assert_eq!(FlexDirection::RowReverse.as_str(), "row-reverse");
        assert_eq!(FlexDirection::ColumnReverse.as_str(), "column-reverse");
    }

    #[test]
    fn align_as_str_maps_every_value() {
        assert_eq!(FlexAlign::Start.as_str(), "start");
        assert_eq!(FlexAlign::Center.as_str(), "center");
        assert_eq!(FlexAlign::End.as_str(), "end");
        assert_eq!(FlexAlign::Baseline.as_str(), "baseline");
    }

    #[test]
    fn justify_as_str_maps_every_value() {
        assert_eq!(FlexJustify::Start.as_str(), "start");
        assert_eq!(FlexJustify::Center.as_str(), "center");
        assert_eq!(FlexJustify::End.as_str(), "end");
        assert_eq!(FlexJustify::Between.as_str(), "between");
        assert_eq!(FlexJustify::Around.as_str(), "around");
        assert_eq!(FlexJustify::Evenly.as_str(), "evenly");
    }

    #[test]
    fn gap_as_str_maps_every_value() {
        assert_eq!(FlexGap::Small.as_str(), "small");
        assert_eq!(FlexGap::Medium.as_str(), "medium");
        assert_eq!(FlexGap::Large.as_str(), "large");
    }

    #[test]
    fn direction_defaults_to_row() {
        assert_eq!(FlexDirection::default(), FlexDirection::Row);
    }
}
