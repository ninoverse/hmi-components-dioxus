use super::json_string;
use dioxus::prelude::*;

/// A single row rendered by [`HmiList`].
///
/// The upstream default renderer shows an optional leading avatar, a title and
/// subtitle, and optional trailing text. Each item needs a stable `id`.
#[derive(Clone, PartialEq, Debug)]
pub struct ListItem {
    /// Stable key for this row.
    pub id: String,
    /// Optional leading avatar, generated from this name.
    pub avatar: Option<String>,
    /// Primary line.
    pub title: Option<String>,
    /// Secondary line below the title.
    pub subtitle: Option<String>,
    /// Trailing text aligned to the row's end.
    pub right: Option<String>,
}

impl ListItem {
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            avatar: None,
            title: None,
            subtitle: None,
            right: None,
        }
    }

    pub fn avatar(mut self, name: impl Into<String>) -> Self {
        self.avatar = Some(name.into());
        self
    }

    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }

    pub fn subtitle(mut self, subtitle: impl Into<String>) -> Self {
        self.subtitle = Some(subtitle.into());
        self
    }

    pub fn right(mut self, right: impl Into<String>) -> Self {
        self.right = Some(right.into());
        self
    }
}

fn push_field(out: &mut String, first: &mut bool, key: &str, value: &Option<String>) {
    if let Some(v) = value {
        if !*first {
            out.push(',');
        }
        *first = false;
        out.push('"');
        out.push_str(key);
        out.push_str("\":");
        out.push_str(&json_string(v));
    }
}

fn items_to_json(items: &[ListItem]) -> String {
    let mut out = String::from("[");
    for (i, item) in items.iter().enumerate() {
        if i > 0 {
            out.push(',');
        }
        out.push_str("{\"id\":");
        out.push_str(&json_string(&item.id));
        let mut first = false;
        push_field(&mut out, &mut first, "avatar", &item.avatar);
        push_field(&mut out, &mut first, "title", &item.title);
        push_field(&mut out, &mut first, "subtitle", &item.subtitle);
        push_field(&mut out, &mut first, "right", &item.right);
        out.push('}');
    }
    out.push(']');
    out
}

/// Typed wrapper for the `<hmi-list>` web component.
///
/// Renders a vertical list with the upstream default row layout (leading
/// avatar, title/subtitle stack, trailing text). The upstream `renderItem`
/// render-prop and the `draggable`/`onReorder` drag-to-reorder pair require JS
/// callbacks that can't cross the web-component boundary, so they are omitted;
/// this wrapper covers the static-list case.
#[component]
pub fn HmiList(items: Vec<ListItem>) -> Element {
    let items_json = items_to_json(&items);
    rsx! {
        hmi-list { "items": items_json }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn items_to_json_id_only() {
        let items = vec![ListItem::new("a")];
        assert_eq!(items_to_json(&items), r#"[{"id":"a"}]"#);
    }

    #[test]
    fn items_to_json_full_row() {
        let items = vec![ListItem::new("u1")
            .avatar("Ada Lovelace")
            .title("Ada Lovelace")
            .subtitle("Mathematician")
            .right("Online")];
        assert_eq!(
            items_to_json(&items),
            r#"[{"id":"u1","avatar":"Ada Lovelace","title":"Ada Lovelace","subtitle":"Mathematician","right":"Online"}]"#
        );
    }

    #[test]
    fn items_to_json_multiple_rows() {
        let items = vec![
            ListItem::new("1").title("First"),
            ListItem::new("2").title("Second"),
        ];
        assert_eq!(
            items_to_json(&items),
            r#"[{"id":"1","title":"First"},{"id":"2","title":"Second"}]"#
        );
    }
}
