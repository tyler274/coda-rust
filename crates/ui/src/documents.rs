//! Pages, forms and their frame: the bar atop a pushed screen,
//! a document page, choices and settings as rows, the notices that
//! say what actually happened, small tags, and the actions pinned to a screen's foot.

use dioxus::prelude::*;

use crate::{Disclosure, Icon, IconName, StatePill, Tone};

/// The bar atop a pushed screen: an optional way back on the leading edge, the title centred with
/// an optional mark (who a conversation is with), and the screen's one action on the trailing edge:
/// in words, as `action` with `on_action`, or instead, as `trailing`, a control of its own such as
/// the button that opens the screen's menu. The two share that edge, so a screen sets one or the
/// other, never both.
#[component]
pub fn TopBar(
    title: String,
    #[props(default)] mark: Option<Element>,
    #[props(default)] back: Option<String>,
    #[props(default)] on_back: Option<EventHandler<MouseEvent>>,
    #[props(default)] action: Option<String>,
    #[props(default)] on_action: Option<EventHandler<MouseEvent>>,
    #[props(default)] trailing: Option<Element>,
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
            if let Some(control) = trailing {
                div { class: "top-bar-trailing", {control} }
            }
        }
    }
}

/// A document with its own hierarchy inside the shared family: titled, in sections, on paper.
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

/// One setting that is on or off: what it does, and a switch.
#[component]
pub fn SettingRow(
    title: String,
    #[props(default)] detail: Option<String>,
    on: bool,
    #[props(default)] disabled: bool,
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
            input { class: "switch", r#type: "checkbox", role: "switch", checked: on, disabled, onchange: move |event| on_change.call(event.checked()) }
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

/// One section of a proposed change, to read against what it replaces: what kind of change it is
/// (a `tag`: "Updated", "Through Oct 21"), the section's name, the proposed words, the earlier
/// words in a detail that opens (`previous`: its label, then the words), and the `sources` it
/// rests on. A section left as it was says so in `unchanged`'s words, and shows the words that stay.
#[component]
pub fn ProposedChange(
    title: String,
    #[props(default)] tag: Option<String>,
    #[props(default)] previous: Option<(String, String)>,
    #[props(default)] unchanged: Option<String>,
    #[props(default)] sources: Option<Element>,
    children: Element,
) -> Element {
    let is_unchanged = unchanged.is_some();
    rsx! {
        section { class: if is_unchanged { "proposed-change is-unchanged" } else { "proposed-change" },
            header { class: "proposed-change-heading",
                if let Some(text) = tag.filter(|_| !is_unchanged) {
                    StatePill { "{text}" }
                }
                h3 { "{title}" }
                if let Some(state) = unchanged {
                    span { class: "proposed-change-state", "{state}" }
                }
            }
            div { class: "proposed-change-words", {children} }
            if let Some((label, words)) = previous.filter(|_| !is_unchanged) {
                Disclosure { title: label, detail: true,
                    p { class: "proposed-change-previous", "{words}" }
                }
            }
            if let Some(sources) = sources {
                div { class: "proposed-change-sources", {sources} }
            }
        }
    }
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
