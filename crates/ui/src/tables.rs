//! Things of one kind read across their columns. The rows and their order are the host's: a
//! heading asks for a sort, and the host answers with the rows in that order.

use dioxus::prelude::*;

use crate::{Icon, IconName};

/// Which way a sorted column runs.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SortOrder {
    /// Lowest or first at the top.
    Ascending,
    /// Highest or last at the top.
    Descending,
}

/// One column of a `Table`.
#[derive(Clone, PartialEq, Debug, Default)]
pub struct Column {
    /// What its cells hold.
    pub label: String,
    /// It holds figures: they keep the trailing edge, their digits one width, so they line up.
    pub numeric: bool,
    /// Its heading asks for a sort, when the table has an `on_sort`.
    pub sortable: bool,
}

impl From<&str> for Column {
    fn from(label: &str) -> Self {
        Self {
            label: label.into(),
            ..Self::default()
        }
    }
}

/// One row of a `Table`.
#[derive(Clone, PartialEq, Default)]
pub struct TableRow {
    /// Which row it is, as the order changes.
    pub key: String,
    /// One for each column, the first naming the row.
    pub cells: Vec<Element>,
    /// The whole row is one way in, so such a row holds no other controls.
    pub onclick: Option<EventHandler<MouseEvent>>,
}

/// A table named by `label`, scrolling across on its own where it doesn't fit. `sorted` says which
/// column the rows are in the order of and which way; `on_sort` gets the column whose heading was
/// chosen.
#[component]
pub fn Table(
    label: String,
    columns: Vec<Column>,
    rows: Vec<TableRow>,
    sorted: Option<(usize, SortOrder)>,
    on_sort: Option<EventHandler<usize>>,
) -> Element {
    let cell_class = |column: &Column| column.numeric.then_some("is-numeric");
    rsx! {
        div { class: "table-frame",
            table { class: "table", aria_label: "{label}",
                thead {
                    tr {
                        for (index, column) in columns.iter().enumerate() {
                            th {
                                class: cell_class(column),
                                scope: "col",
                                aria_sort: sorted.filter(|sorted| sorted.0 == index).map(|sorted| match sorted.1 {
                                    SortOrder::Ascending => "ascending",
                                    SortOrder::Descending => "descending",
                                }),
                                if let Some(sort) = on_sort.filter(|_| column.sortable) {
                                    button {
                                        class: "table-sort",
                                        r#type: "button",
                                        onclick: move |_| sort.call(index),
                                        "{column.label}"
                                        Icon { name: IconName::ChevronDown }
                                    }
                                } else {
                                    "{column.label}"
                                }
                            }
                        }
                    }
                }
                tbody {
                    for row in rows {
                        tr { key: "{row.key}",
                            for (index, (cell, column)) in row.cells.into_iter().zip(&columns).enumerate() {
                                if index > 0 {
                                    td { class: cell_class(column), {cell} }
                                } else if let Some(open) = row.onclick {
                                    th { class: cell_class(column), scope: "row",
                                        button { class: "table-open", r#type: "button", onclick: move |event| open.call(event), {cell} }
                                    }
                                } else {
                                    th { class: cell_class(column), scope: "row", {cell} }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn html(app: fn() -> Element) -> String {
        let mut dom = VirtualDom::new(app);
        dom.rebuild_in_place();
        dioxus_ssr::render(&dom)
    }

    fn columns() -> Vec<Column> {
        vec![
            Column {
                sortable: true,
                .."Clinic".into()
            },
            Column {
                numeric: true,
                sortable: true,
                .."Visits".into()
            },
        ]
    }

    fn cells() -> Vec<Element> {
        vec![rsx! { "Harbor" }, rsx! { "64" }]
    }

    #[test]
    fn a_table_nobody_answers_offers_no_sort_and_no_way_in() {
        let html = html(|| {
            rsx! {
                Table {
                    label: "Clinics",
                    columns: columns(),
                    rows: vec![TableRow { key: "harbor".into(), cells: cells(), onclick: None }],
                }
            }
        });
        assert!(!html.contains("<button"), "{html}");
        assert!(!html.contains("aria-sort"), "{html}");
    }

    #[test]
    fn only_the_sorted_column_says_so_and_a_row_opens_from_its_name() {
        let html = html(|| {
            rsx! {
                Table {
                    label: "Clinics",
                    columns: columns(),
                    rows: vec![TableRow {
                        key: "harbor".into(),
                        cells: cells(),
                        onclick: Some(EventHandler::new(|_| {})),
                    }],
                    sorted: Some((1, SortOrder::Descending)),
                    on_sort: |_| {},
                }
            }
        });
        assert_eq!(html.matches("aria-sort").count(), 1, "{html}");
        assert!(html.contains(r#"aria-sort="descending""#), "{html}");
        assert_eq!(html.matches("table-sort").count(), 2, "{html}");
        assert_eq!(html.matches("table-open").count(), 1, "{html}");
        assert!(html.contains(r#"scope="row""#), "{html}");
    }
}
