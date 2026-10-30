//! Cards that hold one object: an observation and its evidence, a piece of saved work, an original
//! record and what was added to it later, as a tinted card inside it. A card names what it holds,
//! then shows it.

use dioxus::prelude::*;

use crate::{Icon, IconName, Overline};

/// One object on a white reading surface, or on the data container when `tinted`: an optional
/// mark and overline naming what it holds, its title, a detail line, its content, and the source
/// or receipt it closes on, after its icon.
///
/// With `onclick` the whole card is one way in: the title is the one button and its reach is the
/// card, so such a card holds no other controls. Otherwise `on_footer` makes the footer the one way
/// on, such as to where the object came from.
#[component]
pub fn Card(
    #[props(default)] overline: Option<String>,
    #[props(default)] mark: Option<Element>,
    #[props(default)] title: Option<String>,
    #[props(default)] detail: Option<String>,
    #[props(default)] footer: Option<String>,
    #[props(default)] footer_icon: Option<IconName>,
    #[props(default)] tinted: bool,
    #[props(default)] onclick: Option<EventHandler<MouseEvent>>,
    #[props(default)] on_footer: Option<EventHandler<MouseEvent>>,
    children: Element,
) -> Element {
    let opens = onclick.filter(|_| title.is_some());
    let mut class = String::from("card card-object");
    if tinted {
        class.push_str(" is-tinted");
    }
    if opens.is_some() {
        class.push_str(" is-openable");
    }
    let footer_lead = footer_icon.map(|name| rsx! { Icon { name } });
    rsx! {
        article { class: "{class}",
            if overline.is_some() || mark.is_some() {
                div { class: "card-lead",
                    {mark}
                    if let Some(text) = overline {
                        Overline { text }
                    }
                }
            }
            if let Some(title) = title {
                h3 { class: "card-object-title",
                    if let Some(open) = opens {
                        button { class: "card-open", r#type: "button", onclick: move |event| open.call(event), "{title}" }
                    } else {
                        "{title}"
                    }
                }
            }
            if let Some(detail) = detail {
                p { class: "card-detail", "{detail}" }
            }
            {children}
            if let Some(footer) = footer {
                p { class: "card-footer",
                    if opens.is_some() {
                        {footer_lead}
                        span { "{footer}" }
                        span { class: "card-footer-go", aria_hidden: "true", Icon { name: IconName::ArrowRight } }
                    } else if let Some(follow) = on_footer {
                        button { class: "navigation-control card-footer-link", r#type: "button", onclick: move |event| follow.call(event),
                            {footer_lead}
                            span { "{footer}" }
                            Icon { name: IconName::ArrowRight }
                        }
                    } else {
                        {footer_lead}
                        span { "{footer}" }
                    }
                }
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
