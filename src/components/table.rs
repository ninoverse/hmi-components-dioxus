use super::json_string;
use dioxus::prelude::*;

/// A column definition for [`HmiTable`].
#[derive(Clone, PartialEq, Debug)]
pub struct TableColumn {
    /// Key used to look up each row's cell value.
    pub key: String,
    /// Header label.
    pub label: String,
    /// Whether this column is sortable (when the table is sortable). Defaults
    /// to `true`.
    pub sortable: bool,
}

impl TableColumn {
    pub fn new(key: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            key: key.into(),
            label: label.into(),
            sortable: true,
        }
    }

    /// Disable sorting for this column.
    pub fn not_sortable(mut self) -> Self {
        self.sortable = false;
        self
    }
}

fn columns_to_json(columns: &[TableColumn]) -> String {
    let mut out = String::from("[");
    for (i, col) in columns.iter().enumerate() {
        if i > 0 {
            out.push(',');
        }
        out.push_str("{\"key\":");
        out.push_str(&json_string(&col.key));
        out.push_str(",\"label\":");
        out.push_str(&json_string(&col.label));
        if !col.sortable {
            out.push_str(",\"sortable\":false");
        }
        out.push('}');
    }
    out.push(']');
    out
}

/// Build the rows JSON: each row's cells are aligned positionally with
/// `columns`, producing `{ colKey: cell }` objects.
fn rows_to_json(columns: &[TableColumn], rows: &[Vec<String>]) -> String {
    let mut out = String::from("[");
    for (r, row) in rows.iter().enumerate() {
        if r > 0 {
            out.push(',');
        }
        out.push('{');
        for (c, col) in columns.iter().enumerate() {
            if c > 0 {
                out.push(',');
            }
            out.push_str(&json_string(&col.key));
            out.push(':');
            let cell = row.get(c).map(String::as_str).unwrap_or("");
            out.push_str(&json_string(cell));
        }
        out.push('}');
    }
    out.push(']');
    out
}

/// Typed wrapper for the `<hmi-table>` web component.
///
/// A data table built from `columns` and `rows` (each row's cells are aligned
/// positionally with the columns). Sorting is handled internally by the
/// element — clicking a sortable header re-sorts in place — so no callback is
/// needed. Set `sortable` to `false` to disable sorting for the whole table.
#[component]
pub fn HmiTable(
    columns: Vec<TableColumn>,
    /// Rows; each inner vec holds the cell text for that row, aligned with
    /// `columns`.
    rows: Vec<Vec<String>>,
    /// Enable header sorting (per-column override via [`TableColumn::not_sortable`]).
    /// Defaults to `true`.
    #[props(default = true)]
    sortable: bool,
) -> Element {
    let cols_json = columns_to_json(&columns);
    let rows_json = rows_to_json(&columns, &rows);
    rsx! {
        hmi-table {
            "columns": cols_json,
            "rows": rows_json,
            "sortable": if sortable { "true" } else { "false" },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn columns_to_json_with_override() {
        let cols = vec![
            TableColumn::new("name", "Name"),
            TableColumn::new("actions", "Actions").not_sortable(),
        ];
        assert_eq!(
            columns_to_json(&cols),
            r#"[{"key":"name","label":"Name"},{"key":"actions","label":"Actions","sortable":false}]"#
        );
    }

    #[test]
    fn rows_to_json_aligns_cells() {
        let cols = vec![TableColumn::new("a", "A"), TableColumn::new("b", "B")];
        let rows = vec![
            vec!["1".to_string(), "x".to_string()],
            vec!["2".to_string(), "y".to_string()],
        ];
        assert_eq!(
            rows_to_json(&cols, &rows),
            r#"[{"a":"1","b":"x"},{"a":"2","b":"y"}]"#
        );
    }

    #[test]
    fn rows_to_json_pads_missing_cells() {
        let cols = vec![TableColumn::new("a", "A"), TableColumn::new("b", "B")];
        let rows = vec![vec!["only".to_string()]];
        assert_eq!(rows_to_json(&cols, &rows), r#"[{"a":"only","b":""}]"#);
    }
}
