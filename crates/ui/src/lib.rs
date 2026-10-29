//! Shared presentation for the Duet and physician Dioxus apps.
//! Tokens come from the physician stylesheet; product screens stay in each app.
//! Book: `book/src/ui-surface-map.md`.
#![deny(missing_docs)]

use dioxus::dioxus_core::AttributeValue;
use dioxus::prelude::*;

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

#[component]
pub fn ErrorState(message: String) -> Element {
    rsx! { p { class: "error", role: "alert", "{message}" } }
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
        ReadStage::Loading => rsx! { p { class: "empty", role: "status", "{loading}" } },
        ReadStage::Showing => rsx! {},
        ReadStage::Stale => rsx! {
            p { class: "quiet-line", role: "status",
                "{stale} "
                button { class: "inline-action", r#type: "button", onclick: move |_| on_retry.call(()), "{retry}" }
            }
        },
        ReadStage::Unavailable => rsx! {
            ErrorState { message: unavailable }
            Button { compact: true, onclick: move |_| on_retry.call(()), "{retry}" }
        },
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
        CommandStage::Sending => rsx! { p { class: "quiet-line", role: "status", "{sending}" } },
        CommandStage::Unconfirmed => rsx! {
            p { class: "quiet-line", role: "status",
                "{unconfirmed} "
                button { class: "inline-action", r#type: "button", onclick: move |_| on_check_again.call(()), "{check_again}" }
            }
        },
        CommandStage::Refused(reason) => {
            rsx! { p { class: "quiet-line", role: "alert", "{reason}" } }
        }
    }
}

#[component]
pub fn Card(children: Element) -> Element {
    rsx! { section { class: "card", div { class: "card-body", {children} } } }
}

#[component]
pub fn Row(name: String, detail: String) -> Element {
    rsx! {
        div { class: "row",
            div { class: "row-name", "{name}" }
            div { class: "row-detail", "{detail}" }
        }
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
    // A host's class joins the control's own; every other attribute passes through.
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
