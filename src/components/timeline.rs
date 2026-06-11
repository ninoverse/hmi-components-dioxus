use super::json_string;
use dioxus::prelude::*;

/// Marker colour for an [`TimelineEntry`].
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum TimelineColor {
    #[default]
    Default,
    Primary,
    Success,
    Warning,
    Error,
}

impl TimelineColor {
    pub fn as_str(self) -> &'static str {
        match self {
            TimelineColor::Default => "default",
            TimelineColor::Primary => "primary",
            TimelineColor::Success => "success",
            TimelineColor::Warning => "warning",
            TimelineColor::Error => "error",
        }
    }
}

/// A single entry in an [`HmiTimeline`].
#[derive(Clone, PartialEq, Debug)]
pub struct TimelineEntry {
    /// Primary heading for the entry.
    pub title: String,
    /// Optional timestamp shown beside the title.
    pub time: Option<String>,
    /// Optional body text below the title.
    pub description: Option<String>,
    /// Optional marker icon (emoji or short text).
    pub icon: Option<String>,
    /// Marker colour.
    pub color: TimelineColor,
}

impl TimelineEntry {
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            time: None,
            description: None,
            icon: None,
            color: TimelineColor::Default,
        }
    }

    pub fn time(mut self, time: impl Into<String>) -> Self {
        self.time = Some(time.into());
        self
    }

    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    pub fn icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    pub fn color(mut self, color: TimelineColor) -> Self {
        self.color = color;
        self
    }
}

fn items_to_json(items: &[TimelineEntry]) -> String {
    let mut out = String::from("[");
    for (i, item) in items.iter().enumerate() {
        if i > 0 {
            out.push(',');
        }
        out.push_str("{\"title\":");
        out.push_str(&json_string(&item.title));
        if let Some(time) = &item.time {
            out.push_str(",\"time\":");
            out.push_str(&json_string(time));
        }
        if let Some(desc) = &item.description {
            out.push_str(",\"description\":");
            out.push_str(&json_string(desc));
        }
        if let Some(icon) = &item.icon {
            out.push_str(",\"icon\":");
            out.push_str(&json_string(icon));
        }
        if item.color != TimelineColor::Default {
            out.push_str(",\"color\":");
            out.push_str(&json_string(item.color.as_str()));
        }
        out.push('}');
    }
    out.push(']');
    out
}

/// Typed wrapper for the `<hmi-timeline>` web component.
///
/// Renders a vertical list of events, each with a coloured marker, title,
/// optional timestamp, and optional description.
#[component]
pub fn HmiTimeline(items: Vec<TimelineEntry>) -> Element {
    let items_json = items_to_json(&items);
    rsx! {
        hmi-timeline { "items": items_json }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn color_as_str() {
        assert_eq!(TimelineColor::Primary.as_str(), "primary");
        assert_eq!(TimelineColor::Error.as_str(), "error");
    }

    #[test]
    fn items_to_json_full() {
        let items = vec![TimelineEntry::new("Deployed")
            .time("12:30")
            .description("v1.6.0 shipped")
            .icon("🚀")
            .color(TimelineColor::Success)];
        assert_eq!(
            items_to_json(&items),
            r#"[{"title":"Deployed","time":"12:30","description":"v1.6.0 shipped","icon":"🚀","color":"success"}]"#
        );
    }

    #[test]
    fn items_to_json_default_color_omitted() {
        let items = vec![TimelineEntry::new("Created")];
        assert_eq!(items_to_json(&items), r#"[{"title":"Created"}]"#);
    }
}
