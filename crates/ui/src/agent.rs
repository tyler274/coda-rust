//! Agent work: what an agent is doing and has done, shown as it happens. Its steps (thinking,
//! reading, searching, running a tool) stand under one line that says what it did. Every state
//! here is the host's, from the agent's own run; nothing invents progress.

use std::sync::atomic::{AtomicUsize, Ordering};

use dioxus::prelude::*;

use crate::{Icon, IconName, WorkingMark};

/// Where one step of an agent's work stands.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum StepState {
    /// Under way.
    #[default]
    Running,
    /// Finished.
    Done,
    /// Ended without finishing.
    Failed,
    /// Stopped before it finished, by the person or the agent.
    Stopped,
}

/// One step of an agent's work, as a row: its icon, its words, and a quieter detail, such as what
/// it found or how long it took. Running, its words shimmer; failed, its mark turns to the alert
/// in red. With a `body`, such as the reasoning itself or a tool's input and output, the row is a
/// toggle that opens to show it.
#[component]
pub fn WorkStep(
    icon: IconName,
    label: String,
    #[props(default)] state: StepState,
    detail: Option<String>,
    #[props(default)] body: Option<Element>,
) -> Element {
    let id = use_hook(|| format!("work-step-{}", STEPS.fetch_add(1, Ordering::Relaxed)));
    let mut open = use_signal(|| false);
    let class = match state {
        StepState::Running => "work-step is-running",
        StepState::Done => "work-step",
        StepState::Failed => "work-step is-failed",
        StepState::Stopped => "work-step is-stopped",
    };
    let mark = if state == StepState::Failed {
        IconName::Alert
    } else {
        icon
    };
    let line = rsx! {
        span { class: "work-step-mark", Icon { name: mark } }
        span { class: "work-step-text",
            span { class: "work-step-label", "{label}" }
            if let Some(detail) = detail {
                span { class: "work-step-detail", "{detail}" }
            }
        }
    };
    rsx! {
        li { class, "data-state": if open() { "open" } else { "closed" },
            if let Some(body) = body {
                button {
                    class: "work-step-line",
                    r#type: "button",
                    aria_expanded: "{open}",
                    aria_controls: "{id}",
                    onclick: move |_| open.toggle(),
                    {line}
                    Icon { name: IconName::ChevronDown }
                }
                div {
                    id: "{id}",
                    class: if open() { "disclosure-region is-open" } else { "disclosure-region" },
                    aria_hidden: if open() { "false" } else { "true" },
                    "inert": (!open()).then_some("true"),
                    div { class: "disclosure-clip",
                        div { class: "work-step-body", {body} }
                    }
                }
            } else {
                div { class: "work-step-line", {line} }
            }
        }
    }
}

static STEPS: AtomicUsize = AtomicUsize::new(0);

/// An agent's work on one turn: its `WorkStep`s under one line that says what it is doing or did,
/// such as "Read 3 notes and searched twice", with a quieter `detail`, such as how long it took.
/// While `working`, the line carries the working mark and the steps stand open; once done they
/// fold away under it. Pressing the line opens or closes them until the work next starts or ends.
#[component]
pub fn WorkTrace(
    label: String,
    #[props(default)] working: bool,
    detail: Option<String>,
    children: Element,
) -> Element {
    let id = use_hook(|| format!("work-trace-{}", TRACES.fetch_add(1, Ordering::Relaxed)));
    // The person's own choice holds until the work starts or ends; then the steps follow the work.
    let mut chosen = use_signal(|| None::<bool>);
    use_effect(use_reactive!(|working| {
        let _ = working;
        chosen.set(None);
    }));
    let open = chosen().unwrap_or(working);
    rsx! {
        section {
            class: if working { "work-trace is-working" } else { "work-trace" },
            "data-state": if open { "open" } else { "closed" },
            aria_busy: working.then_some("true"),
            button {
                class: "work-trace-line",
                r#type: "button",
                aria_expanded: "{open}",
                aria_controls: "{id}",
                onclick: move |_| chosen.set(Some(!open)),
                if working {
                    WorkingMark {}
                }
                span { class: "work-trace-label", "{label}" }
                if let Some(detail) = detail {
                    span { class: "work-trace-detail", "{detail}" }
                }
                Icon { name: IconName::ChevronDown }
            }
            div {
                id: "{id}",
                class: if open { "disclosure-region is-open" } else { "disclosure-region" },
                aria_hidden: if open { "false" } else { "true" },
                "inert": (!open).then_some("true"),
                div { class: "disclosure-clip",
                    ol { class: "work-steps", {children} }
                }
            }
        }
    }
}

static TRACES: AtomicUsize = AtomicUsize::new(0);
