//! Shared presentation for the Duet and physician Dioxus apps.
//! Tokens come from the physician stylesheet; product screens stay in each app.
//! Book: `book/src/ui-surface-map.md`.
#![deny(missing_docs)]

use dioxus::prelude::*;

/// The `STYLESHEET` constant.
pub const STYLESHEET: &str = concat!(
    include_str!("../assets/tokens.css"),
    include_str!("../assets/components.css"),
);

#[component]
pub fn PageTitle(text: String) -> Element {
    rsx! { h1 { class: "page-title", "{text}" } }
}

#[component]
pub fn EmptyState(message: String) -> Element {
    rsx! { p { class: "empty", "{message}" } }
}

#[component]
pub fn ErrorState(message: String) -> Element {
    rsx! { p { class: "error", role: "alert", "{message}" } }
}

#[component]
pub fn Card(children: Element) -> Element {
    rsx! { section { class: "card", div { class: "card-body", {children} } } }
}

#[component]
pub fn Row(name: String, detail: String) -> Element {
    rsx! {
        div { class: "row",
            div { class: "row-name", "{name}" }
            div { class: "row-detail", "{detail}" }
        }
    }
}

#[component]
pub fn TextButton(label: String, quiet: bool, on_press: EventHandler) -> Element {
    rsx! {
        button {
            class: if quiet { "text-button quiet" } else { "text-button" },
            r#type: "button",
            onclick: move |_| on_press.call(()),
            "{label}"
        }
    }
}

#[component]
pub fn TabBar(labels: Vec<String>, selected: usize, on_select: EventHandler<usize>) -> Element {
    rsx! {
        nav { class: "tabs", aria_label: "Sections",
            for (index, label) in labels.iter().enumerate() {
                button {
                    key: "{label}",
                    r#type: "button",
                    aria_current: if index == selected { "page" } else { "false" },
                    onclick: move |_| on_select.call(index),
                    "{label}"
                }
            }
        }
    }
}
