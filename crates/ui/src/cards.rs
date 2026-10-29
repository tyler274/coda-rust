//! Cards that hold one object, after the October 21 Duet mockups: an observation and its evidence,
//! a piece of saved work, an original record and the person's later context. Useful work has a
//! shape, a name and a place to return; a card names what it holds, then shows it.

use dioxus::prelude::*;

use crate::Overline;

/// One object on a white reading surface, or on the data container when `tinted`: an optional
/// mark and overline naming what it holds, its title, a detail line, its content, and the source
/// or receipt it closes on.
#[component]
pub fn Card(
    #[props(default)] overline: Option<String>,
    #[props(default)] mark: Option<Element>,
    #[props(default)] title: Option<String>,
    #[props(default)] detail: Option<String>,
    #[props(default)] footer: Option<String>,
    #[props(default)] tinted: bool,
    children: Element,
) -> Element {
    rsx! {
        article { class: if tinted { "card card-object is-tinted" } else { "card card-object" },
            if overline.is_some() || mark.is_some() {
                div { class: "card-lead",
                    {mark}
                    if let Some(text) = overline {
                        Overline { text }
                    }
                }
            }
            if let Some(title) = title {
                h3 { class: "card-object-title", "{title}" }
            }
            if let Some(detail) = detail {
                p { class: "card-detail", "{detail}" }
            }
            {children}
            if let Some(footer) = footer {
                p { class: "card-footer", "{footer}" }
            }
        }
    }
}

/// The steps of a piece of saved work, numbered in order, one line apart.
#[component]
pub fn StepList(steps: Vec<String>) -> Element {
    rsx! {
        ol { class: "step-list",
            for (index, step) in steps.into_iter().enumerate() {
                li { key: "{index}",
                    span { class: "step-number", aria_hidden: "true", {format!("{:02}", index + 1)} }
                    span { "{step}" }
                }
            }
        }
    }
}

/// Words quoted from a record, in the record's own words.
#[component]
pub fn Quote(text: String) -> Element {
    rsx! { blockquote { class: "quote", "{text}" } }
}

/// What the person added after a record, beside it: it can change the understanding without
/// rewriting the dated record.
#[component]
pub fn LaterContext(overline: String, text: String, added: String) -> Element {
    rsx! {
        aside { class: "later-context",
            Overline { text: overline }
            Quote { text }
            p { class: "card-detail", "{added}" }
        }
    }
}
