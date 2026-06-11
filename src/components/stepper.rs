use super::json_string;
use dioxus::prelude::*;

use crate::event::{on_input_event, ListenerGuard};

/// Layout orientation of an [`HmiStepper`].
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum StepperOrientation {
    #[default]
    Horizontal,
    Vertical,
}

impl StepperOrientation {
    pub fn as_str(self) -> &'static str {
        match self {
            StepperOrientation::Horizontal => "horizontal",
            StepperOrientation::Vertical => "vertical",
        }
    }
}

/// A single step within an [`HmiStepper`].
#[derive(Clone, PartialEq, Debug)]
pub struct StepItem {
    /// Identifier for this step; matched against `current`.
    pub value: String,
    /// Primary label.
    pub label: String,
    /// Optional secondary description.
    pub description: Option<String>,
}

impl StepItem {
    pub fn new(value: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            value: value.into(),
            label: label.into(),
            description: None,
        }
    }

    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }
}

fn steps_to_json(steps: &[StepItem]) -> String {
    let mut out = String::from("[");
    for (i, step) in steps.iter().enumerate() {
        if i > 0 {
            out.push(',');
        }
        out.push_str("{\"value\":");
        out.push_str(&json_string(&step.value));
        out.push_str(",\"label\":");
        out.push_str(&json_string(&step.label));
        if let Some(desc) = &step.description {
            out.push_str(",\"description\":");
            out.push_str(&json_string(desc));
        }
        out.push('}');
    }
    out.push(']');
    out
}

/// Typed wrapper for the `<hmi-stepper>` web component.
///
/// A progress indicator for multi-step flows. `current` marks the active step
/// by its `StepItem::value`; earlier steps render as completed and become
/// clickable, firing `on_change` with the target step's value (so the consumer
/// can navigate back).
#[component]
pub fn HmiStepper(
    steps: Vec<StepItem>,
    /// The active step's value (controlled).
    current: Option<String>,
    #[props(default)] orientation: StepperOrientation,
    #[props(default)] on_change: EventHandler<String>,
) -> Element {
    let mut listener = use_signal(|| Option::<ListenerGuard>::None);
    let steps_json = steps_to_json(&steps);
    rsx! {
        hmi-stepper {
            "steps": steps_json,
            "current": current,
            "orientation": orientation.as_str(),
            onmounted: move |m| {
                listener.set(on_input_event(&m, "change", on_change, |d| d.string()));
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn orientation_as_str() {
        assert_eq!(StepperOrientation::Horizontal.as_str(), "horizontal");
        assert_eq!(StepperOrientation::Vertical.as_str(), "vertical");
    }

    #[test]
    fn steps_to_json_with_description() {
        let steps = vec![
            StepItem::new("a", "Account"),
            StepItem::new("b", "Billing").description("Payment details"),
        ];
        assert_eq!(
            steps_to_json(&steps),
            r#"[{"value":"a","label":"Account"},{"value":"b","label":"Billing","description":"Payment details"}]"#
        );
    }
}
