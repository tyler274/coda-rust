//! Shared presentation for the Duet and physician Dioxus apps.
//! The tokens here own the product's colour, type, space, shape and motion roles; product screens stay in each app.
//! Book: `book/src/ui-surface-map.md`.
#![deny(missing_docs)]

use std::sync::atomic::{AtomicUsize, Ordering};

use dioxus::dioxus_core::AttributeValue;
use dioxus::prelude::*;

mod icons;
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

static DIALOGS: AtomicUsize = AtomicUsize::new(0);

/// A modal dialog, after the React app's `TransientDialog` and `DialogHeader`: it opens over the
/// scrim with focus inside it, its title heading the content with Close beside it. Escape, the
/// scrim and Close dismiss it; it settles out before the host hears `on_close`. Focus lands on
/// what the content marks `autofocus`, else the title. The host's class sizes it.
#[component]
pub fn Dialog(
    title: String,
    /// What the dialog belongs to, under the title, when that is not obvious.
    subtitle: Option<String>,
    on_close: EventHandler,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let id = use_hook(|| format!("dialog-{}", DIALOGS.fetch_add(1, Ordering::Relaxed)));
    let mut closing = use_signal(|| false);
    let mut class = String::from(if closing() {
        "dialog is-closing"
    } else {
        "dialog"
    });
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
