use dioxus::prelude::*;

/// Gap between children of an [`HmiGrid`]. Mirrors the upstream `grid--gap-*` modifiers.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum GridGap {
    Small,
    Medium,
    Large,
}

impl GridGap {
    /// The string the `<hmi-grid>` `gap` attribute expects.
    pub fn as_str(self) -> &'static str {
        match self {
            GridGap::Small => "small",
            GridGap::Medium => "medium",
            GridGap::Large => "large",
        }
    }
}

/// Typed wrapper for the `<hmi-grid>` web component.
///
/// A CSS grid layout primitive. `tag` maps to the upstream `as` attribute
/// (renamed because `as` is a Rust keyword) and selects the rendered element;
/// the upstream defaults to `<div>`. `columns` accepts any
/// `grid-template-columns` value (e.g. `"3"`, `"repeat(2, 1fr)"`, `"1fr 2fr"`).
#[component]
pub fn HmiGrid(
    /// HTML element to render as (e.g. `"section"`); defaults to `"div"`.
    tag: Option<String>,
    columns: Option<String>,
    gap: Option<GridGap>,
    children: Element,
) -> Element {
    rsx! {
        hmi-grid { "as": tag, "columns": columns, "gap": gap.map(GridGap::as_str), {children} }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gap_as_str_maps_every_value() {
        assert_eq!(GridGap::Small.as_str(), "small");
        assert_eq!(GridGap::Medium.as_str(), "medium");
        assert_eq!(GridGap::Large.as_str(), "large");
    }
}
