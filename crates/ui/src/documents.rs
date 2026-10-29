//! Pages, forms and their frame, after the October 21 Duet mockups: the bar atop a pushed screen,
//! the document a person prepares for a clinician, choices and settings as rows, the notices that
//! say what actually happened, small tags, and the actions pinned to a screen's foot.

use dioxus::prelude::*;

use crate::{Icon, IconName, Tone};

/// The bar atop a pushed screen: an optional way back on the leading edge, the title centred with
/// an optional mark (who a conversation is with), and the screen's one action on the trailing edge.
#[component]
pub fn TopBar(
    title: String,
    #[props(default)] mark: Option<Element>,
    #[props(default)] back: Option<String>,
    #[props(default)] on_back: Option<EventHandler<MouseEvent>>,
    #[props(default)] action: Option<String>,
    #[props(default)] on_action: Option<EventHandler<MouseEvent>>,
) -> Element {
    rsx! {
        header { class: "top-bar",
            if let (Some(back), Some(on_back)) = (back, on_back) {
                button { class: "top-bar-back", r#type: "button", onclick: move |event| on_back.call(event),
                    span { class: "top-bar-back-chevron", aria_hidden: "true", Icon { name: IconName::ChevronDown } }
                    "{back}"
                }
            }
            h1 { class: "top-bar-title", {mark} "{title}" }
            if let (Some(action), Some(on_action)) = (action, on_action) {
                button { class: "top-bar-action", r#type: "button", onclick: move |event| on_action.call(event), "{action}" }
            }
        }
    }
}

/// A document with its own hierarchy inside the shared family: what a person prepares for a
/// clinician, titled, in sections, on paper.
#[component]
pub fn Paper(
    title: String,
    #[props(default)] detail: Option<String>,
    children: Element,
) -> Element {
    rsx! {
        article { class: "paper",
            h2 { class: "paper-title", "{title}" }
            if let Some(detail) = detail {
                p { class: "card-detail", "{detail}" }
            }
            {children}
        }
    }
}

/// One section of a `Paper`, under its small-capital heading.
#[component]
pub fn PaperSection(heading: String, children: Element) -> Element {
    rsx! {
        section { class: "paper-section",
            h3 { class: "overline", "{heading}" }
            {children}
        }
    }
}

/// One choice in a list of choices: a checkbox and what choosing it includes.
#[component]
pub fn SelectionRow(
    title: String,
    #[props(default)] detail: Option<String>,
    checked: bool,
    on_change: EventHandler<bool>,
) -> Element {
    rsx! {
        label { class: "selection-row",
            input { r#type: "checkbox", checked, onchange: move |event| on_change.call(event.checked()) }
            span { class: "list-row-text",
                strong { "{title}" }
                if let Some(detail) = detail {
                    small { "{detail}" }
                }
            }
        }
    }
}

/// One setting that is on or off: what it does, and a switch.
#[component]
pub fn SettingRow(
    title: String,
    #[props(default)] detail: Option<String>,
    on: bool,
    on_change: EventHandler<bool>,
) -> Element {
    rsx! {
        label { class: "setting-row",
            span { class: "list-row-text",
                strong { "{title}" }
                if let Some(detail) = detail {
                    small { "{detail}" }
                }
            }
            input { class: "switch", r#type: "checkbox", role: "switch", checked: on, onchange: move |event| on_change.call(event.checked()) }
        }
    }
}

/// What actually happened, said once: saved, failed with the draft kept, limited evidence, quiet.
/// Its tone carries the state; its actions, if any, follow the words. A failure announces itself.
#[component]
pub fn Notice(
    #[props(default)] tone: Tone,
    #[props(default)] icon: Option<IconName>,
    title: String,
    #[props(default)] detail: Option<String>,
    children: Element,
) -> Element {
    let class = match tone {
        Tone::Neutral => "notice tone-neutral",
        Tone::Primary => "notice tone-primary",
        Tone::Warning => "notice tone-warning",
        Tone::Error => "notice tone-error",
    };
    rsx! {
        div { class: "{class}", role: if tone == Tone::Error { "alert" } else { "status" },
            if let Some(name) = icon {
                Icon { name }
            }
            div { class: "notice-text",
                strong { "{title}" }
                if let Some(detail) = detail {
                    span { "{detail}" }
                }
                {children}
            }
        }
    }
}

/// A small label on content: what period it covers, or that it changed.
#[component]
pub fn Tag(text: String) -> Element {
    rsx! { span { class: "tag", "{text}" } }
}

/// The screen's actions, pinned to its foot, with an optional note on what they do and don't do.
#[component]
pub fn FooterActions(#[props(default)] note: Option<String>, children: Element) -> Element {
    rsx! {
        div { class: "footer-actions",
            {children}
            if let Some(note) = note {
                p { class: "footer-note", "{note}" }
            }
        }
    }
}
