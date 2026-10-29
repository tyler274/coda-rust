//! Shared presentation for the Duet and physician Dioxus apps.
//! The tokens here own the product's colour, type, space, shape and motion roles; product screens stay in each app.
//! Book: `book/src/ui-surface-map.md`.
#![deny(missing_docs)]

use std::sync::atomic::{AtomicUsize, Ordering};

use dioxus::dioxus_core::AttributeValue;
use dioxus::prelude::*;

mod cards;
mod charts;
mod icons;
pub use cards::{Card, LaterContext, Quote, StepList};
pub use charts::{Comparison, ComparisonBars, MeasurementTile, Sparkline, TrendChart, Value};
pub use icons::{Icon, IconName};

/// The `STYLESHEET` constant.
pub const STYLESHEET: &str = concat!(
    include_str!("../assets/tokens.css"),
    include_str!("../assets/components.css"),
);

#[component]
pub fn PageTitle(text: String) -> Element {
    rsx! { h1 { class: "page-title", "{text}" } }
}

/// What a card holds, named above its title in small capitals: "What Duet noticed", "Original record".
#[component]
pub fn Overline(text: String) -> Element {
    rsx! { p { class: "overline", "{text}" } }
}

#[component]
pub fn EmptyState(message: String) -> Element {
    rsx! { p { class: "empty", "{message}" } }
}

/// Something that failed and has no retry of its own: the one status line, announced.
#[component]
pub fn ErrorState(message: String) -> Element {
    rsx! { StatusLine { message, alert: true } }
}

/// The one status line, after the React app's `StatusLine`: what failed or what is under way, in
/// one muted line, with the one Try again under the words after a failure. While pending it draws
/// the shape of what is coming, after the shared delay so a fast read never flashes, and keeps its
/// words for assistive technology. A failure the person was waiting on announces itself.
#[component]
pub fn StatusLine(
    message: String,
    #[props(default)] alert: bool,
    #[props(default)] pending: bool,
    /// The retry's words; with `on_retry`, the line offers it.
    #[props(default)]
    retry: String,
    on_retry: Option<EventHandler>,
) -> Element {
    let class = if pending {
        "status-line pending-status"
    } else {
        "status-line"
    };
    rsx! {
        div { class: "{class}", role: if alert { "alert" } else { "status" },
            if pending {
                p { class: "sr-only", "{message}" }
                Placeholder {}
            } else {
                p { "{message}" }
            }
            if let Some(handler) = on_retry {
                TryAgain { label: retry, on_retry: handler }
            }
        }
    }
}

/// The one recovery action after a failure: one verb, one shape, one size, under the line that
/// says what failed.
#[component]
pub fn TryAgain(
    label: String,
    #[props(default)] disabled: bool,
    on_retry: EventHandler,
) -> Element {
    rsx! {
        Button { compact: true, disabled, onclick: move |_| on_retry.call(()), "{label}" }
    }
}

/// What is loading, drawn as its shape (Material's skeleton): a bar in the text's own measure with
/// a slow sheen passing over it. Decoration only; the host's words stay with assistive technology.
#[component]
pub fn Placeholder() -> Element {
    rsx! {
        span { class: "placeholder", aria_hidden: "true", span { class: "placeholder-bar" } }
    }
}

/// What a state chip's colour says: neutral, ready or done, flagged, or failed.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Tone {
    /// Nothing to call out.
    #[default]
    Neutral,
    /// Ready or done.
    Primary,
    /// What Coda flags.
    Warning,
    /// What failed or conflicts.
    Error,
}

/// What Coda did, or where a thing stands, on its tone, led by its kind's icon. The host names the
/// tone and icon from structured state, never from the words.
#[component]
pub fn StatePill(
    #[props(default)] tone: Tone,
    icon: Option<IconName>,
    children: Element,
) -> Element {
    let class = match tone {
        Tone::Neutral => "state-pill tone-neutral",
        Tone::Primary => "state-pill tone-primary",
        Tone::Warning => "state-pill tone-warning",
        Tone::Error => "state-pill tone-error",
    };
    rsx! {
        span { class: "{class}",
            if let Some(name) = icon {
                Icon { name }
            }
            {children}
        }
    }
}

/// The one live mark: a pulsing dot for something live, new or needing the person now.
#[component]
pub fn LiveDot() -> Element {
    rsx! { span { class: "live-dot", aria_hidden: "true" } }
}

/// The one working mark, after the React app's `WorkingMark`: three live dots lighting in turn,
/// shown only while work actually runs. Duet's typing indicator is this mark in a reply's place.
/// The words beside it, or the host's status, say what runs.
#[component]
pub fn WorkingMark() -> Element {
    rsx! {
        span { class: "working-dots", aria_hidden: "true",
            LiveDot {}
            LiveDot {}
            LiveDot {}
        }
    }
}

/// Where one read stands, as the page shows it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ReadStage {
    /// The first read has not answered yet.
    Loading,
    /// The read answered and its content shows.
    Showing,
    /// A refresh failed: the last answer stays, under one quiet line with Retry.
    Stale,
    /// The first read failed: nothing to show, with Retry.
    Unavailable,
}

/// A read's own line beside its content: loading, one quiet line with Retry
/// over the last answer, or couldn't load with Retry. Showing adds nothing.
#[component]
pub fn ReadNotice(
    stage: ReadStage,
    loading: String,
    stale: String,
    unavailable: String,
    retry: String,
    on_retry: EventHandler,
) -> Element {
    match stage {
        ReadStage::Loading => rsx! { StatusLine { message: loading, pending: true } },
        ReadStage::Showing => rsx! {},
        ReadStage::Stale => rsx! { StatusLine { message: stale, retry, on_retry } },
        ReadStage::Unavailable => {
            rsx! { StatusLine { message: unavailable, alert: true, retry, on_retry } }
        }
    }
}

/// Where one command stands after it was pressed.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CommandStage {
    /// Nothing is in flight.
    Idle,
    /// Sent and not yet answered; other commands on the same item wait.
    Sending,
    /// The answer never arrived, so whether it took is unknown. Check again
    /// resends the same request, which the server recognizes.
    Unconfirmed,
    /// The server answered and did not take it, saying why.
    Refused(String),
}

/// A command's own line: sending, couldn't confirm with Check again, or why
/// it was refused. Idle adds nothing.
#[component]
pub fn CommandNotice(
    stage: CommandStage,
    sending: String,
    unconfirmed: String,
    check_again: String,
    on_check_again: EventHandler,
) -> Element {
    match stage {
        CommandStage::Idle => rsx! {},
        CommandStage::Sending => rsx! { StatusLine { message: sending } },
        CommandStage::Unconfirmed => rsx! {
            StatusLine { message: unconfirmed, retry: check_again, on_retry: on_check_again }
        },
        CommandStage::Refused(reason) => rsx! { StatusLine { message: reason, alert: true } },
    }
}

/// A card header's leading icon on its neutral tile.
#[component]
pub fn IconTile(name: IconName) -> Element {
    rsx! {
        span { class: "icon-tile", aria_hidden: "true", Icon { name } }
    }
}

/// How much a button stands out.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ButtonVariant {
    /// The one filled green action on a surface.
    Primary,
    /// White on a hairline: every other command.
    #[default]
    Secondary,
    /// Text alone, for the least emphasis.
    Quiet,
}

/// A command, after the React physician app's `Control`: one shape, a variant, and the shared
/// hover, press, focus and touch target. A host class places it; it never restyles it.
#[component]
pub fn Button(
    #[props(default)] variant: ButtonVariant,
    /// Removes or disconnects what the person has: red.
    #[props(default)]
    danger: bool,
    /// The shorter control, for rows of actions.
    #[props(default)]
    compact: bool,
    /// The icon is the whole label, so the caller names it with `aria_label`.
    #[props(default)]
    icon_only: bool,
    #[props(default)] disabled: bool,
    /// Submits its form instead of acting on its own.
    #[props(default)]
    submit: bool,
    onclick: Option<EventHandler<MouseEvent>>,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut class = String::from(match variant {
        ButtonVariant::Primary => "control control-primary",
        ButtonVariant::Secondary => "control control-secondary",
        ButtonVariant::Quiet => "control control-quiet",
    });
    for (on, name) in [
        (danger, " control-danger"),
        (compact, " control-compact"),
        (icon_only, " control-icon"),
    ] {
        if on {
            class.push_str(name);
        }
    }
    let rest = host_attributes(&mut class, attributes);
    rsx! {
        button {
            class: "{class}",
            r#type: if submit { "submit" } else { "button" },
            disabled: disabled,
            onclick: move |event| {
                if let Some(handler) = &onclick {
                    handler.call(event);
                }
            },
            ..rest,
            {children}
        }
    }
}

/// In-place navigation, after the React app's `NavigationControl`: a quiet link, separate from
/// commands, that opens another place or goes back. It reads in body text and underlines on hover;
/// a surface may colour it once at its root (`--navigation-color`), as Duet does.
#[component]
pub fn NavigationControl(
    #[props(default)] disabled: bool,
    onclick: Option<EventHandler<MouseEvent>>,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut class = String::from("navigation-control");
    let rest = host_attributes(&mut class, attributes);
    rsx! {
        button {
            class: "{class}",
            r#type: "button",
            disabled: disabled,
            onclick: move |event| {
                if let Some(handler) = &onclick {
                    handler.call(event);
                }
            },
            ..rest,
            {children}
        }
    }
}

/// A row that opens something, after the React app's `NavigationRow`: the whole row is one button
/// with the shared state layer and an inward focus ring. The host's class lays the row out.
#[component]
pub fn NavigationRow(
    onclick: Option<EventHandler<MouseEvent>>,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut class = String::from("navigation-row");
    let rest = host_attributes(&mut class, attributes);
    rsx! {
        button {
            class: "{class}",
            r#type: "button",
            onclick: move |event| {
                if let Some(handler) = &onclick {
                    handler.call(event);
                }
            },
            ..rest,
            {children}
        }
    }
}

/// Who wrote a message in a thread.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Author {
    /// The person using the app: their words stand in a bubble at the far edge.
    Person,
    /// Coda or Duet: its words read as plain prose.
    Assistant,
}

/// One message in a thread, after the React app's conversation and the October 21 design review:
/// the person's words in a light bubble at the far edge, the assistant's as plain prose. Who
/// wrote it stays with assistive technology. A message rises in as it appears.
#[component]
pub fn Message(
    author: Author,
    /// Who wrote it, for assistive technology.
    #[props(default)]
    by: String,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut class = String::from(match author {
        Author::Person => "message message-person",
        Author::Assistant => "message message-assistant",
    });
    let rest = host_attributes(&mut class, attributes);
    rsx! {
        article { class: "{class}", ..rest,
            if !by.is_empty() {
                span { class: "sr-only", "{by}" }
            }
            {children}
        }
    }
}

/// The composer, after the React app's: the one raised text field, holding the words and, under
/// them, a toolbar of the host's context and actions; a host's submit control sends. The words
/// grow with what is written, up to seven lines. Keys stay the host's.
#[component]
pub fn Composer(
    /// The words' accessible name.
    label: String,
    #[props(default)] placeholder: String,
    value: String,
    on_input: EventHandler<String>,
    /// The words as written, when the host's submit control is pressed.
    on_submit: EventHandler<String>,
    /// Whether the words have focus.
    on_editing: Option<EventHandler<bool>>,
    children: Element,
) -> Element {
    let held = value.clone();
    rsx! {
        form {
            class: "composer field",
            onsubmit: move |event| {
                event.prevent_default();
                let typed = event.get_first("draft").and_then(|value| match value {
                    FormValue::Text(text) => Some(text),
                    FormValue::File(_) => None,
                });
                on_submit.call(submitted_text(typed.as_deref(), &held));
            },
            textarea {
                class: "composer-input",
                name: "draft",
                rows: "1",
                aria_label: "{label}",
                placeholder: "{placeholder}",
                value: "{value}",
                oninput: move |event| on_input.call(event.value()),
                onfocus: move |_| {
                    if let Some(handler) = &on_editing {
                        handler.call(true);
                    }
                },
                onblur: move |_| {
                    if let Some(handler) = &on_editing {
                        handler.call(false);
                    }
                },
            }
            div { class: "composer-toolbar", {children} }
        }
    }
}

/// The words a send uses. The field wins when it has something in it: on Android the controlled
/// value can drop keystrokes while the person still sees them.
fn submitted_text(field: Option<&str>, value: &str) -> String {
    match field {
        Some(text) if !text.trim().is_empty() => text.to_owned(),
        _ => value.to_owned(),
    }
}

static DISCLOSURES: AtomicUsize = AtomicUsize::new(0);

/// A section that opens in place, after the React app's `ProductDisclosure`: its title is one
/// toggle with a chevron, and its content stays mounted, out of reach while closed, growing open
/// and shut. A detail reads as a quiet line rather than a section's title.
#[component]
pub fn Disclosure(
    title: String,
    #[props(default)] detail: bool,
    #[props(default)] default_open: bool,
    children: Element,
) -> Element {
    let id = use_hook(|| format!("disclosure-{}", DISCLOSURES.fetch_add(1, Ordering::Relaxed)));
    let mut open = use_signal(|| default_open);
    let class = if detail {
        "disclosure is-detail"
    } else {
        "disclosure"
    };
    rsx! {
        section { class: "{class}", "data-state": if open() { "open" } else { "closed" },
            h2 { class: "disclosure-heading",
                button {
                    class: "disclosure-toggle",
                    r#type: "button",
                    aria_expanded: "{open}",
                    aria_controls: "{id}",
                    onclick: move |_| open.toggle(),
                    strong { "{title}" }
                    Icon { name: IconName::ChevronDown }
                }
            }
            div {
                id: "{id}",
                class: if open() { "disclosure-region is-open" } else { "disclosure-region" },
                aria_hidden: if open() { "false" } else { "true" },
                "inert": (!open()).then_some("true"),
                div { class: "disclosure-clip",
                    div { class: "disclosure-body", {children} }
                }
            }
        }
    }
}

static DIALOGS: AtomicUsize = AtomicUsize::new(0);

/// A modal dialog, after the React app's `TransientDialog` and `DialogHeader`: it opens over the
/// scrim with focus inside it, its title heading the content with Close beside it. Escape, the
/// scrim and Close dismiss it; it settles out before the host hears `on_close`. Focus lands on
/// what the content marks `autofocus`, else the title. The host's class sizes it. A sheet is the
/// same dialog along the bottom edge, after the October 21 design review, and one sheet can
/// open over another.
#[component]
pub fn Dialog(
    title: String,
    /// What the dialog belongs to, under the title, when that is not obvious.
    subtitle: Option<String>,
    #[props(default)] sheet: bool,
    on_close: EventHandler,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let id = use_hook(|| format!("dialog-{}", DIALOGS.fetch_add(1, Ordering::Relaxed)));
    let mut closing = use_signal(|| false);
    let mut class = String::from("dialog");
    if sheet {
        class.push_str(" dialog-sheet");
    }
    if closing() {
        class.push_str(" is-closing");
    }
    let rest = host_attributes(&mut class, attributes);
    let opening = id.clone();
    // The browser owns the dialog's life: it opens modally here, the exit motion ends by closing
    // it, and whatever closed it, the host hears it once.
    use_effect(move || {
        let mut opened = document::eval(&format!(
            "const d = document.getElementById('{opening}');
            if (d && !d.open) {{
              d.addEventListener('animationend', (e) => {{
                if (e.target === d && e.animationName === 'dialog-exit') d.close();
              }});
              d.addEventListener('close', () => dioxus.send(true), {{ once: true }});
              d.showModal();
              (d.querySelector('[autofocus]') ?? d.querySelector('h2')).focus({{ preventScroll: true }});
            }}"
        ));
        spawn(async move {
            if opened.recv::<bool>().await.is_ok() {
                on_close.call(());
            }
        });
    });
    let mut dismiss = move || {
        if !closing() {
            closing.set(true);
        }
    };
    rsx! {
        dialog {
            id: "{id}",
            class: "{class}",
            aria_labelledby: "{id}-title",
            onkeydown: move |event| {
                // The dialog answers its own Escape, through the browser's close request below.
                if event.key() == Key::Escape {
                    event.stop_propagation();
                }
            },
            oncancel: move |event| {
                event.prevent_default();
                dismiss();
            },
            onclick: move |_| dismiss(),
            ..rest,
            div { class: "dialog-body", onclick: move |event| event.stop_propagation(),
                header { class: "dialog-header",
                    div {
                        h2 { id: "{id}-title", tabindex: "-1", "{title}" }
                        if let Some(subtitle) = subtitle {
                            p { "{subtitle}" }
                        }
                    }
                    Button {
                        variant: ButtonVariant::Quiet,
                        icon_only: true,
                        aria_label: "Close",
                        onclick: move |_| dismiss(),
                        Icon { name: IconName::X }
                    }
                }
                {children}
            }
        }
    }
}

/// A citation in prose, after the React app's citation chip: its number in a small pill. It opens
/// its source when the host can; otherwise it stays a mark. `label` names the source for
/// assistive technology.
#[component]
pub fn CitationChip(
    index: usize,
    label: String,
    onclick: Option<EventHandler<MouseEvent>>,
) -> Element {
    match onclick {
        Some(open) => rsx! {
            button {
                class: "source-trigger citation-chip",
                r#type: "button",
                aria_label: "{label}",
                onclick: move |event| open.call(event),
                "[{index}]"
            }
        },
        None => rsx! {
            span { class: "source-trigger citation-chip", aria_label: "{label}", "[{index}]" }
        },
    }
}

/// The one Sources control under a message, after the React app's sources index and the SwiftUI
/// Sources pill: a compact pill with how many sources the message has.
#[component]
pub fn SourcesButton(
    count: usize,
    onclick: EventHandler<MouseEvent>,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    let mut class = String::from("source-trigger sources-button");
    let rest = host_attributes(&mut class, attributes);
    rsx! {
        button {
            class: "{class}",
            r#type: "button",
            aria_haspopup: "dialog",
            onclick: move |event| onclick.call(event),
            ..rest,
            Icon { name: IconName::ChevronDown }
            "Sources"
            span { class: "sources-count", "{count}" }
        }
    }
}

/// One source in a Sources list, after the React app's sources index: its number when the
/// message numbers its citations, its title, a quiet detail, and an arrow. The whole row is one
/// navigation row that opens it.
#[component]
pub fn SourceRow(
    number: Option<usize>,
    title: String,
    detail: Option<String>,
    onclick: EventHandler<MouseEvent>,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    let mut class = String::from("navigation-row source-entry");
    let rest = host_attributes(&mut class, attributes);
    rsx! {
        button {
            class: "{class}",
            r#type: "button",
            onclick: move |event| onclick.call(event),
            ..rest,
            if let Some(number) = number {
                span { class: "source-entry-number", "{number}" }
            }
            span { class: "source-entry-text",
                strong { "{title}" }
                if let Some(detail) = detail {
                    small { "{detail}" }
                }
            }
            Icon { name: IconName::ArrowRight }
        }
    }
}

/// A host's class joins the component's own; every other attribute passes through.
fn host_attributes(class: &mut String, attributes: Vec<Attribute>) -> Vec<Attribute> {
    let mut rest = Vec::with_capacity(attributes.len());
    for attribute in attributes {
        match (&attribute.value, attribute.name) {
            (AttributeValue::Text(host), "class") if !host.is_empty() => {
                class.push(' ');
                class.push_str(host);
            }
            (_, "class") => {}
            _ => rest.push(attribute),
        }
    }
    rest
}

/// Actions that sit together: one row, one gap, one centre line, wrapping where the width runs out.
/// A host places the row; it never sets the row's own layout.
#[component]
pub fn ActionRow(children: Element) -> Element {
    rsx! { div { class: "action-row", {children} } }
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

#[cfg(test)]
mod tests {
    use super::submitted_text;

    #[test]
    fn a_send_uses_the_words_the_person_sees() {
        let draft = "How did I sleep?";
        assert_eq!(submitted_text(Some("Hello"), ""), "Hello");
        assert_eq!(submitted_text(Some("  "), draft), draft);
        assert_eq!(submitted_text(None, draft), draft);
    }
}
