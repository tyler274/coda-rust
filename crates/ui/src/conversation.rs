//! The conversation's frame: the day it is on, what the person
//! brought into it, and a way to begin. A receipt in the thread is a `ListRow` in a `RowGroup`;
//! who the conversation is with is the `TopBar`'s title.

use std::rc::Rc;

use dioxus::html::geometry::PixelsVector2D;
use dioxus::prelude::*;

use crate::{Button, ButtonVariant, Icon, IconName};

/// How near the end counts as at it: within this, new replies are followed.
const FOLLOW_PX: f64 = 48.0;

/// The conversation's scrolling region. At its end it
/// follows each new reply; scrolled back, it leaves the reading where it is and offers `jump_label`
/// instead. What the person sends always brings them to the end. The host counts its
/// `replies` and its person's `sent` messages; a change in either is what arrived.
#[component]
pub fn ConversationViewport(
    replies: usize,
    sent: usize,
    jump_label: String,
    children: Element,
) -> Element {
    let mut at_end = use_signal(|| true);
    let mut has_new = use_signal(|| false);
    let mut region = use_signal(|| None::<Rc<MountedData>>);
    let mut seen = use_signal(|| (replies, sent));
    // Scrolls only the region itself, never the page around it.
    let reveal = move |behavior: ScrollBehavior| {
        if let Some(region) = region.peek().clone() {
            spawn(async move {
                if let Ok(size) = region.get_scroll_size().await {
                    let _ = region
                        .scroll(PixelsVector2D::new(0.0, size.height), behavior)
                        .await;
                }
            });
        }
    };
    use_effect(use_reactive!(|replies, sent| {
        let (seen_replies, seen_sent) = *seen.peek();
        seen.set((replies, sent));
        if sent != seen_sent {
            has_new.set(false);
            reveal(ScrollBehavior::Instant);
        } else if replies != seen_replies {
            if *at_end.peek() {
                reveal(ScrollBehavior::Instant);
            } else {
                has_new.set(true);
            }
        }
    }));
    rsx! {
        div { class: "conversation-viewport",
            div {
                class: "conversation-scroll",
                onmounted: move |event| {
                    region.set(Some(event.data()));
                    reveal(ScrollBehavior::Instant);
                },
                onscroll: move |event| {
                    let data = event.data();
                    let from_end = f64::from(data.scroll_height()) - f64::from(data.client_height()) - data.scroll_top();
                    let here = from_end <= FOLLOW_PX;
                    at_end.set(here);
                    if here {
                        has_new.set(false);
                    }
                },
                {children}
            }
            if has_new() {
                Button {
                    variant: ButtonVariant::Secondary,
                    class: "jump-to-latest",
                    onclick: move |_| {
                        has_new.set(false);
                        // Scrolling asked for in code ignores the stylesheet's reduced-motion rule,
                        // so the preference is read here.
                        spawn(async move {
                            let reduced = document::eval(
                                "return matchMedia('(prefers-reduced-motion: reduce)').matches",
                            )
                            .join::<bool>()
                            .await
                            .unwrap_or(false);
                            reveal(if reduced { ScrollBehavior::Instant } else { ScrollBehavior::Smooth });
                        });
                    },
                    "{jump_label}"
                }
            }
        }
    }
}

/// The day the messages beneath were written, centred in the thread.
#[component]
pub fn DateSeparator(date: String) -> Element {
    rsx! { p { class: "date-separator", "{date}" } }
}

/// What the person brought into the conversation, above the words, with a way to remove it: a
/// selected measurement, a record, a page.
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

/// A suggested way to begin, as a quiet row.
#[component]
pub fn Suggestion(text: String, onclick: EventHandler<MouseEvent>) -> Element {
    rsx! {
        button { class: "suggestion navigation-row", r#type: "button", onclick: move |event| onclick.call(event), "{text}" }
    }
}
