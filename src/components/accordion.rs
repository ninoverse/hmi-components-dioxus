use super::json_string;
use dioxus::prelude::*;

/// A single panel in an [`HmiAccordion`].
#[derive(Clone, PartialEq, Debug)]
pub struct AccordionItem {
    /// Header text for the panel.
    pub title: String,
    /// Body text shown when the panel is expanded.
    pub body: String,
    pub disabled: bool,
    /// Whether this panel starts expanded.
    pub default_open: bool,
}

impl AccordionItem {
    pub fn new(title: impl Into<String>, body: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            body: body.into(),
            disabled: false,
            default_open: false,
        }
    }

    pub fn disabled(mut self) -> Self {
        self.disabled = true;
        self
    }

    /// Start this panel expanded.
    pub fn open(mut self) -> Self {
        self.default_open = true;
        self
    }
}

fn items_to_json(items: &[AccordionItem]) -> String {
    let mut out = String::from("[");
    for (i, item) in items.iter().enumerate() {
        if i > 0 {
            out.push(',');
        }
        out.push_str("{\"title\":");
        out.push_str(&json_string(&item.title));
        out.push_str(",\"body\":");
        out.push_str(&json_string(&item.body));
        if item.disabled {
            out.push_str(",\"disabled\":true");
        }
        out.push('}');
    }
    out.push(']');
    out
}

fn default_open_json(items: &[AccordionItem]) -> String {
    let mut out = String::from("[");
    let mut first = true;
    for (i, item) in items.iter().enumerate() {
        if item.default_open {
            if !first {
                out.push(',');
            }
            first = false;
            out.push_str(&i.to_string());
        }
    }
    out.push(']');
    out
}

/// Typed wrapper for the `<hmi-accordion>` web component.
///
/// A stack of expandable panels built from `items`. Expanding/collapsing is
/// handled internally by the element, so no callback is needed; mark panels to
/// start open with [`AccordionItem::open`]. Set `multiple` to allow more than
/// one panel open at a time.
#[component]
pub fn HmiAccordion(
    items: Vec<AccordionItem>,
    /// Allow multiple panels open simultaneously.
    #[props(default)]
    multiple: bool,
) -> Element {
    let items_json = items_to_json(&items);
    let default_open = default_open_json(&items);
    rsx! {
        hmi-accordion {
            "items": items_json,
            "multiple": if multiple { "true" },
            "default-open": default_open,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn items_to_json_basic() {
        let items = vec![
            AccordionItem::new("Q1", "A1"),
            AccordionItem::new("Q2", "A2").disabled(),
        ];
        assert_eq!(
            items_to_json(&items),
            r#"[{"title":"Q1","body":"A1"},{"title":"Q2","body":"A2","disabled":true}]"#
        );
    }

    #[test]
    fn default_open_collects_indices() {
        let items = vec![
            AccordionItem::new("a", "x").open(),
            AccordionItem::new("b", "y"),
            AccordionItem::new("c", "z").open(),
        ];
        assert_eq!(default_open_json(&items), "[0,2]");
    }
}
