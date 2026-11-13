//! The conversation's frame: the day it is on, what the person brought into it, a way to begin,
//! speech as it is taken down, and what can be done with a message once it is there. A receipt in
//! the thread is a `ListRow` in a `RowGroup`; who the conversation is with is the `TopBar`'s title.

use std::sync::atomic::{AtomicUsize, Ordering};

use dioxus::prelude::*;

use crate::{Avatar, AvatarSize, Button, ButtonVariant, Icon, IconName, WorkingMark, script};

/// Where a `ChatColumn` stands its queued messages.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum QueuePlace {
    /// At the end of the transcript, under the answer in progress.
    #[default]
    UnderAnswer,
    /// In the dock, directly above the composer.
    AboveComposer,
}

/// A conversation laid out as a window's main area: an optional `header` at the top, the
/// transcript in a `ConversationViewport` beneath it, and a dock at the foot holding the
/// `composer`. The transcript's turns, the `children`, and the dock share one centred column at
/// the reading measure. The dock never covers the transcript: as it grows upward the transcript
/// gives up the room, and the composer stays where it is. Queued messages, a `MessageQueue` given
/// as `queue`, stand where `queue_place` says: at the transcript's end, under the answer in
/// progress, or in the dock above the composer. `replies`, `sent` and `jump_label` are the
/// viewport's.
#[component]
pub fn ChatColumn(
    #[props(default)] header: Option<Element>,
    replies: usize,
    sent: usize,
    jump_label: String,
    #[props(default)] queue: Option<Element>,
    #[props(default)] queue_place: QueuePlace,
    composer: Element,
    children: Element,
) -> Element {
    let (under_answer, above_composer) = match queue_place {
        QueuePlace::UnderAnswer => (queue, None),
        QueuePlace::AboveComposer => (None, queue),
    };
    rsx! {
        div { class: "chat-column",
            {header}
            ConversationViewport { replies, sent, jump_label,
                div { class: "conversation-feed chat-measure",
                    {children}
                    {under_answer}
                }
            }
            div { class: "chat-dock chat-measure",
                {above_composer}
                {composer}
            }
        }
    }
}

/// One fact under a `ChatHeader`'s name: a value, such as `77 years`, or a label and its value,
/// such as `DOB` and `Mar 3, 1947`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HeaderFact {
    /// The quieter word that names the value, if the value needs one.
    pub label: Option<String>,
    /// The fact itself.
    pub value: String,
}

/// Who a conversation is about, atop its `ChatColumn`: their initials, their name at title size,
/// and their `facts` in one quiet row, such as their age, date of birth, sex and record number.
/// A gap sets the facts apart; each fact wraps whole, its label beside its value and quieter than
/// it. The header stays at the top while the transcript scrolls, with a hairline under it across
/// the column, edge to edge where the column fills the width. It has no room of its own above it;
/// the shell's toolbar row gives that room.
#[component]
pub fn ChatHeader(name: String, facts: Vec<HeaderFact>) -> Element {
    rsx! {
        header { class: "chat-header",
            div { class: "chat-header-inner",
                Avatar { name: name.clone(), size: AvatarSize::Large }
                div { class: "chat-header-text",
                    h1 { "{name}" }
                    if !facts.is_empty() {
                        p { class: "chat-header-facts",
                            for (index, fact) in facts.into_iter().enumerate() {
                                span { key: "{index}", class: "chat-header-fact",
                                    if let Some(label) = fact.label {
                                        span { class: "chat-header-fact-label", "{label}" }
                                        " "
                                    }
                                    "{fact.value}"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Messages waiting their turn, given as a `ChatColumn`'s `queue`: each queued `Message`, its
/// waiting line, such as "Sends after Coda's reply", and its actions in its bubble's own row; or
/// its `MessageEditor` while it is edited.
#[component]
pub fn MessageQueue(children: Element) -> Element {
    rsx! {
        div { class: "message-queue", {children} }
    }
}

/// The conversation's scrolling region. At its end it follows the end as replies arrive and as
/// the region or its content changes size, such as when a source opens beside it or the dock
/// grows, and waits while a button or finger is down in it. A reply that arrives there and whose
/// top the end would take out of sight stands instead with its top at the region's top, where
/// the jump would bring it, and focus stays where it was. Moving back lets go of the end at
/// once; scrolled back, the first turn in sight stays where it stood until the reader returns to
/// the end, and a new reply offers `jump_label`, which takes the reader to that reply's top and
/// focus to the reply. What the person sends always brings them to the end. Its turns are the
/// children of its one child. What stands in it when it first shows stands at rest; only messages
/// that arrive after run their entry. The host counts its `replies` and its person's `sent`
/// messages; a change in either is what arrived.
#[component]
pub fn ConversationViewport(
    replies: usize,
    sent: usize,
    jump_label: String,
    children: Element,
) -> Element {
    let id = use_hook(|| format!("conversation-{}", VIEWPORTS.fetch_add(1, Ordering::Relaxed)));
    let mut at_end = use_signal(|| true);
    let mut has_new = use_signal(|| false);
    let mut seen = use_signal(|| (replies, sent));
    let region = id.clone();
    use_effect(use_reactive!(|replies, sent| {
        let (seen_replies, seen_sent) = *seen.peek();
        seen.set((replies, sent));
        if sent != seen_sent {
            has_new.set(false);
            script::read_to_end(&region);
        } else if replies != seen_replies {
            if *at_end.peek() {
                script::reply_arrived(&region);
            } else {
                has_new.set(true);
            }
        }
    }));
    let watched = id.clone();
    let jumping = id.clone();
    rsx! {
        div { class: "conversation-viewport",
            div {
                id: "{id}",
                class: "conversation-scroll",
                onmounted: move |_| {
                    let reading = script::watch_reading(&watched);
                    spawn(async move {
                        // The behaviour finished or failed: the region stops hearing where the reader is.
                        while let Ok(end) = reading.next().await {
                            at_end.set(end);
                            if end {
                                has_new.set(false);
                            }
                        }
                    });
                },
                {children}
            }
            if has_new() {
                Button {
                    variant: ButtonVariant::Secondary,
                    class: "jump-to-latest",
                    onclick: move |_| {
                        has_new.set(false);
                        let region = jumping.clone();
                        // Scrolling asked for in code ignores the stylesheet's reduced-motion rule,
                        // so the preference is read here.
                        spawn(async move {
                            let reduced = script::reduced_motion().answer().await;
                            // Unasked, it moves without motion, which suits everyone.
                            script::jump_to_latest(&region, !reduced.unwrap_or(true));
                        });
                    },
                    "{jump_label}"
                }
            }
        }
    }
}

static VIEWPORTS: AtomicUsize = AtomicUsize::new(0);

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
    // Once an arrow that holds focus has stopped going anywhere, focus moves to the other.
    let switch = id.clone();
    use_effect(use_reactive!(|shown, count| {
        let _ = (shown, count);
        script::hand_off(&switch);
    }));
    rsx! {
        div { class: "version-switch", role: "group", aria_label: "{label}",
            Button {
                id: "{id}-previous",
                variant: ButtonVariant::Quiet,
                compact: true,
                icon_only: true,
                aria_label: "{previous_label}",
                disabled: shown == 0,
                onclick: move |_| on_change.call(shown.saturating_sub(1)),
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
                onclick: move |_| on_change.call(shown + 1),
                Icon { name: IconName::ChevronRight }
            }
        }
    }
}

static SWITCHES: AtomicUsize = AtomicUsize::new(0);

/// What has become of a message on its way, given as its `Message`'s `status`, or standing once
/// above a group of messages that share it, such as messages queued together: its icon, its words,
/// such as "Not sent" or "Sends after Coda's reply", and the host's ways on, such as Try again or
/// Check again. When it says the message didn't go, it is `alert`, in red, and announced.
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
                        script::caret_to_end();
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
