//! Cards that hold one object: an observation and its evidence, a piece of saved work, an original
//! record and what was added to it later, as a tinted card inside it. A card names what it holds,
//! then shows it.

use dioxus::prelude::*;

use crate::{
    Button, ButtonVariant, Count, Icon, IconName, IconTile, NavigationRow, Overline,
    host_attributes,
};

/// One object on a white reading surface, or on the data container when `tinted`: an optional
/// mark and overline naming what it holds, its title, a detail line, its content, and the source
/// or receipt it closes on, after its icon.
///
/// With `onclick` the whole card is one way in: the title is the one button and its reach is the
/// card, so such a card holds no other controls. Otherwise `on_footer` makes the footer the one way
/// on, such as to where the object came from. A host's class places it, and its other attributes
/// reach the element.
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
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
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
    let rest = host_attributes(&mut class, attributes);
    let footer_lead = footer_icon.map(|name| rsx! { Icon { name } });
    rsx! {
        article { class: "{class}", ..rest,
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

/// A card that stands beside a page and folds to a rail. Open, it shows its mark, its title with
/// how many things it holds in `count_words`, a control that folds it, and its content. Folded, it
/// is one row that opens it, with the mark and the count alone. The host decides when it folds and
/// keeps that; its class places the card, and its other attributes reach the element.
#[component]
pub fn RailCard(
    icon: IconName,
    title: String,
    #[props(default)] count: usize,
    #[props(default)] count_words: String,
    collapsed: bool,
    on_collapsed: EventHandler<bool>,
    /// The folded row's accessible name.
    expand_label: String,
    /// The fold control's accessible name.
    collapse_label: String,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut class = String::from("card");
    let rest = host_attributes(&mut class, attributes);
    rsx! {
        aside { class: "{class}", ..rest,
            if collapsed {
                NavigationRow {
                    class: "rail-card-expand",
                    aria_expanded: "false",
                    aria_label: "{expand_label}",
                    onclick: move |_| on_collapsed.call(false),
                    IconTile { name: icon }
                    if count > 0 {
                        Count { value: count }
                    }
                }
            } else {
                header { class: "card-header",
                    IconTile { name: icon }
                    div { class: "card-title title-with-count",
                        h2 { "{title}" }
                        if count > 0 {
                            Count { value: count, words: count_words }
                        }
                    }
                    Button {
                        variant: ButtonVariant::Quiet,
                        icon_only: true,
                        aria_expanded: "true",
                        aria_label: "{collapse_label}",
                        onclick: move |_| on_collapsed.call(true),
                        Icon { name: IconName::PanelRightClose }
                    }
                }
                {children}
            }
        }
    }
}

/// Words quoted from a record, in the record's own words.
#[component]
pub fn Quote(text: String) -> Element {
    rsx! { blockquote { class: "quote", "{text}" } }
}
