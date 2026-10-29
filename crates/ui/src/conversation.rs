//! The conversation's frame, after the October 21 Duet mockups: who the conversation is with, the
//! day it is on, what the person brought into it, a way to begin, and a receipt for saved work
//! that leads back to the same object.

use dioxus::prelude::*;

use crate::{Icon, IconName};

/// The conversation's header: an optional way back on the leading edge, then a mark and the name
/// of who the conversation is with, centred.
#[component]
pub fn ChatHeader(
    name: String,
    #[props(default)] mark: Option<Element>,
    #[props(default)] back: Option<String>,
    #[props(default)] on_back: Option<EventHandler<MouseEvent>>,
) -> Element {
    rsx! {
        header { class: "chat-header",
            if let (Some(back), Some(on_back)) = (back, on_back) {
                button { class: "chat-back", r#type: "button", onclick: move |event| on_back.call(event),
                    span { class: "chat-back-chevron", aria_hidden: "true", Icon { name: IconName::ChevronDown } }
                    "{back}"
                }
            }
            h1 { class: "chat-name", {mark} "{name}" }
        }
    }
}

/// The day the messages beneath were written, centred in the thread.
#[component]
pub fn DateSeparator(date: String) -> Element {
    rsx! { p { class: "date-separator", "{date}" } }
}

/// What the person brought into the conversation, held above the words until they send or remove
/// it: a selected measurement, a record, a page.
#[component]
pub fn ContextChip(
    icon: IconName,
    text: String,
    remove_label: String,
    on_remove: EventHandler<MouseEvent>,
) -> Element {
    rsx! {
        div { class: "context-chip",
            Icon { name: icon }
            span { class: "context-chip-text", "{text}" }
            button { class: "context-chip-remove", r#type: "button", aria_label: "{remove_label}", onclick: move |event| on_remove.call(event),
                Icon { name: IconName::X }
            }
        }
    }
}

/// A way to begin, in the person's own words, that they can send as it stands.
#[component]
pub fn Suggestion(text: String, onclick: EventHandler<MouseEvent>) -> Element {
    rsx! {
        button { class: "suggestion navigation-row", r#type: "button", onclick: move |event| onclick.call(event), "{text}" }
    }
}

/// A receipt for work the conversation saved: what it is, what happened to it, and the way back to
/// its current version. It appears only once the owning service confirms the save.
#[component]
pub fn Receipt(
    icon: IconName,
    title: String,
    detail: String,
    onclick: EventHandler<MouseEvent>,
) -> Element {
    rsx! {
        button { class: "receipt navigation-row", r#type: "button", onclick: move |event| onclick.call(event),
            span { class: "receipt-icon", aria_hidden: "true", Icon { name: icon } }
            span { class: "receipt-text",
                strong { "{title}" }
                small { "{detail}" }
            }
            span { class: "list-row-chevron", aria_hidden: "true", Icon { name: IconName::ChevronDown } }
        }
    }
}
