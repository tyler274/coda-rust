//! The conversation's frame: the day it is on, what the person brought into it, a way to begin,
//! speech as it is taken down, and what can be done with a message once it is there. A receipt in
//! the thread is a `ListRow` in a `RowGroup`; who the conversation is with is the `TopBar`'s title.

use std::rc::Rc;
use std::sync::atomic::{AtomicUsize, Ordering};

use dioxus::html::geometry::PixelsVector2D;
use dioxus::prelude::*;

use crate::{Button, ButtonVariant, Icon, IconName, WorkingMark};

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

/// What can be done with one message, a row of quiet controls given as its `Message`'s
/// `actions`, named as a whole by `label`: a `CopyButton`, a retry, an edit, a `FeedbackRating`, a
/// `VersionSwitch`, a `Menu`'s button. On pointer devices the row shows while its message is
/// hovered or focused, or while one of its actions is busy, unless it is `shown` always, as under
/// the latest reply; on touch it always shows.
#[component]
pub fn MessageActions(label: String, #[props(default)] shown: bool, children: Element) -> Element {
    rsx! {
        div {
            class: if shown { "message-actions is-shown" } else { "message-actions" },
            role: "group",
            aria_label: "{label}",
            {children}
        }
    }
}

/// Which of a message's versions is shown, such as a reply asked again or a question edited, and
/// a way to the one before and after: `at` counts from zero among `count`. Its `label` says the
/// same in words, such as "Response 2 of 3"; the quiet "2 / 3" between the arrows is only seen.
/// Reaching the first or last, focus moves to the arrow that still goes somewhere.
#[component]
pub fn VersionSwitch(
    at: usize,
    count: usize,
    label: String,
    previous_label: String,
    next_label: String,
    on_change: EventHandler<usize>,
) -> Element {
    let id = use_hook(|| {
        format!(
            "version-switch-{}",
            SWITCHES.fetch_add(1, Ordering::Relaxed)
        )
    });
    let shown = at.min(count.saturating_sub(1));
    let switch = id.clone();
    let go = move |to: usize, end: bool, other: &'static str| {
        let id = switch.clone();
        move |_| {
            on_change.call(to);
            if end {
                document::eval(&format!("document.getElementById('{id}-{other}')?.focus()"));
            }
        }
    };
    rsx! {
        div { class: "version-switch", role: "group", aria_label: "{label}",
            Button {
                id: "{id}-previous",
                variant: ButtonVariant::Quiet,
                compact: true,
                icon_only: true,
                aria_label: "{previous_label}",
                disabled: shown == 0,
                onclick: go(shown.saturating_sub(1), shown <= 1, "next"),
                Icon { name: IconName::ChevronLeft }
            }
            span { class: "version-switch-place", aria_hidden: "true", "{shown + 1} / {count}" }
            Button {
                id: "{id}-next",
                variant: ButtonVariant::Quiet,
                compact: true,
                icon_only: true,
                aria_label: "{next_label}",
                disabled: shown + 1 >= count,
                onclick: go(shown + 1, shown + 2 >= count, "previous"),
                Icon { name: IconName::ChevronRight }
            }
        }
    }
}

static SWITCHES: AtomicUsize = AtomicUsize::new(0);

/// What has become of a message on its way, given as its `Message`'s `status`: its icon, its
/// words, such as "Not sent" or "Sends after this reply", and the host's ways on, such as Try again
/// or Cancel. When it says the message didn't go, it is `alert`, in red, and announced.
#[component]
pub fn MessageStatus(
    words: String,
    icon: Option<IconName>,
    #[props(default)] alert: bool,
    children: Element,
) -> Element {
    rsx! {
        div {
            class: if alert { "message-status is-alert" } else { "message-status" },
            role: if alert { "alert" } else { "status" },
            if let Some(name) = icon {
                Icon { name }
            }
            span { "{words}" }
            {children}
        }
    }
}

/// The person's words, being edited where they stood in the thread: the field the bubble's width,
/// holding the words and growing with them, an optional `note` on what saving does, such as that
/// the reply after it is asked again, then Cancel and Save. Escape cancels; Save waits for words.
/// The field takes focus with the caret at the end.
#[component]
pub fn MessageEditor(
    label: String,
    value: String,
    on_input: EventHandler<String>,
    on_save: EventHandler<String>,
    on_cancel: EventHandler,
    save_label: String,
    cancel_label: String,
    note: Option<String>,
) -> Element {
    let empty = value.trim().is_empty();
    let held = value.clone();
    rsx! {
        form {
            class: "message-editor field",
            onsubmit: move |event| {
                event.prevent_default();
                if !held.trim().is_empty() {
                    on_save.call(held.clone());
                }
            },
            textarea {
                class: "message-editor-input",
                aria_label: "{label}",
                rows: "2",
                value: "{value}",
                oninput: move |event| on_input.call(event.value()),
                onkeydown: move |event| {
                    if event.key() == Key::Escape && !event.is_composing() {
                        event.prevent_default();
                        on_cancel.call(());
                    }
                },
                onmounted: move |event| {
                    let field = event.data();
                    spawn(async move {
                        let _ = field.set_focus(true).await;
                        document::eval(
                            "const field = document.activeElement; if (field && field.setSelectionRange) field.setSelectionRange(field.value.length, field.value.length);",
                        );
                    });
                },
            }
            div { class: "message-editor-foot",
                if let Some(note) = note {
                    p { class: "message-editor-note", "{note}" }
                }
                div { class: "message-editor-actions",
                    Button { compact: true, onclick: move |_| on_cancel.call(()), "{cancel_label}" }
                    Button { variant: ButtonVariant::Primary, compact: true, submit: true, disabled: empty, "{save_label}" }
                }
            }
        }
    }
}

/// How many recent levels dictation draws.
const DICTATION_BARS: usize = 40;

/// Speech being taken down in a `Composer`'s place: what is happening in `label`'s words, such as
/// "Listening" or "Transcribing", the voice's recent loudness from the host's microphone as bars,
/// newest at the end, each `levels` value from 0 to 1, and how long it has run. Cancel drops what
/// was said; Done ends listening. While `transcribing` the bars hold still under the working mark
/// and only Cancel remains.
#[component]
pub fn Dictation(
    label: String,
    levels: Vec<f32>,
    elapsed: String,
    #[props(default)] transcribing: bool,
    cancel_label: String,
    done_label: String,
    on_cancel: EventHandler,
    on_done: EventHandler,
) -> Element {
    let recent = &levels[levels.len().saturating_sub(DICTATION_BARS)..];
    let bars: Vec<f32> = std::iter::repeat_n(0.0, DICTATION_BARS - recent.len())
        .chain(recent.iter().map(|level| level.clamp(0.0, 1.0)))
        .collect();
    rsx! {
        div { class: if transcribing { "dictation is-transcribing" } else { "dictation" },
            Button {
                variant: ButtonVariant::Quiet,
                compact: true,
                icon_only: true,
                aria_label: "{cancel_label}",
                onclick: move |_| on_cancel.call(()),
                Icon { name: IconName::X }
            }
            div { class: "dictation-wave", aria_hidden: "true",
                for (index, level) in bars.into_iter().enumerate() {
                    span { key: "{index}", style: "--level: {level}" }
                }
            }
            span { class: "dictation-status", role: "status",
                if transcribing {
                    WorkingMark {}
                }
                span { class: "dictation-label", "{label}" }
                span { class: "dictation-elapsed", "{elapsed}" }
            }
            Button {
                variant: ButtonVariant::Primary,
                compact: true,
                icon_only: true,
                aria_label: "{done_label}",
                disabled: transcribing,
                onclick: move |_| on_done.call(()),
                Icon { name: IconName::Check }
            }
        }
    }
}
