//! Agent work: what an agent is doing and has done, shown as it happens. Its steps (thinking,
//! reading, searching, running a tool) stand under one line that says what it did, the plan it
//! works through checks off as it goes, what it asks before acting waits for the person's answer,
//! and what it makes stands as a card that opens it. Every state here is the host's, from the
//! agent's own run; nothing invents progress.

use std::cell::RefCell;
use std::rc::Rc;
use std::sync::atomic::{AtomicUsize, Ordering};

use dioxus::prelude::*;

use crate::{Button, ButtonVariant, Icon, IconName, Spinner, WorkingMark};

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

/// Where one task of a plan stands.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum TaskState {
    /// Not begun.
    #[default]
    Pending,
    /// The one under way.
    Active,
    /// Finished.
    Done,
    /// Passed over.
    Skipped,
}

/// One task in a `TaskList`: its words and where it stands.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Task {
    /// What the task is.
    pub label: String,
    /// Where it stands.
    pub state: TaskState,
}

impl From<&str> for Task {
    fn from(label: &str) -> Self {
        Task {
            label: label.to_owned(),
            state: TaskState::Pending,
        }
    }
}

/// What assistive technology hears before a task that is under way, done or passed over; a task
/// waiting is heard by its words alone.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct TaskWords {
    /// Before the task under way.
    pub active: String,
    /// Before a finished task.
    pub done: String,
    /// Before a task passed over.
    pub skipped: String,
}

/// Tasks in order, one line apart: a saved plan's steps, `numbered`, or a plan an agent works
/// through. A task waiting shows its number, or an open ring; the one under way turns, and is the
/// list's current step; a finished one is checked, its words gone quieter; one passed over is
/// struck through. A check draws in when a task finishes while shown, and is simply there before.
#[component]
pub fn TaskList(
    tasks: Vec<Task>,
    #[props(default)] numbered: bool,
    #[props(default)] words: TaskWords,
) -> Element {
    // Where each task stood when last shown, and whether it finished while shown: only a task that
    // became done here draws its check, and it keeps that mark while it stays done.
    let seen = use_hook(|| Rc::new(RefCell::new(Vec::<(TaskState, bool)>::new())));
    let drawn: Vec<bool> = {
        let mut seen = seen.borrow_mut();
        let first = seen.is_empty();
        let drawn: Vec<bool> = tasks
            .iter()
            .enumerate()
            .map(|(index, task)| match (seen.get(index), task.state) {
                (_, state) if state != TaskState::Done => false,
                (Some(&(TaskState::Done, drawn)), _) => drawn,
                (Some(_), _) => true,
                (None, _) => !first,
            })
            .collect();
        *seen = tasks
            .iter()
            .map(|task| task.state)
            .zip(drawn.iter().copied())
            .collect();
        drawn
    };
    rsx! {
        ol { class: "task-list",
            for (index, task) in tasks.into_iter().enumerate() {
                li {
                    key: "{index}",
                    class: match task.state {
                        TaskState::Pending => "task",
                        TaskState::Active => "task is-active",
                        TaskState::Done if drawn[index] => "task is-done is-new",
                        TaskState::Done => "task is-done",
                        TaskState::Skipped => "task is-skipped",
                    },
                    aria_current: (task.state == TaskState::Active).then_some("step"),
                    span { class: "task-mark", aria_hidden: "true",
                        match task.state {
                            TaskState::Pending if numbered => rsx! { span { class: "task-number", {format!("{:02}", index + 1)} } },
                            TaskState::Pending => rsx! { span { class: "task-ring" } },
                            TaskState::Active => rsx! { Spinner {} },
                            TaskState::Done => rsx! { Icon { name: IconName::Check } },
                            TaskState::Skipped => rsx! { span { class: "task-dash" } },
                        }
                    }
                    span { class: "task-label",
                        match task.state {
                            TaskState::Active if !words.active.is_empty() => rsx! { span { class: "sr-only", "{words.active}: " } },
                            TaskState::Done if !words.done.is_empty() => rsx! { span { class: "sr-only", "{words.done}: " } },
                            TaskState::Skipped if !words.skipped.is_empty() => rsx! { span { class: "sr-only", "{words.skipped}: " } },
                            _ => rsx! {},
                        }
                        "{task.label}"
                    }
                }
            }
        }
    }
}

/// The person's answer to what an agent asked.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Answer {
    /// Go ahead.
    Approved,
    /// Don't.
    Declined,
}

/// What an agent asks before it acts: what it wants to do, a quieter `detail`, and what it would
/// act on as the card's content, such as a command, a change or a message to send. The two answers
/// stand at its foot, approving filled, and red when it removes what the person has; `busy` holds
/// both while an answer is on its way. Answered, the card keeps what was asked and says the
/// `answer` in `answered` words, never what came of it: the action's own result is reported apart.
#[component]
pub fn ApprovalCard(
    title: String,
    detail: Option<String>,
    approve_label: String,
    decline_label: String,
    on_approve: EventHandler<MouseEvent>,
    on_decline: EventHandler<MouseEvent>,
    #[props(default)] danger: bool,
    #[props(default)] busy: bool,
    answer: Option<Answer>,
    #[props(default)] answered: String,
    children: Element,
) -> Element {
    let id = use_hook(|| format!("approval-{}", APPROVALS.fetch_add(1, Ordering::Relaxed)));
    rsx! {
        section {
            class: if answer.is_some() { "approval is-answered" } else { "approval" },
            aria_labelledby: "{id}",
            div { class: "approval-heading",
                strong { id: "{id}", "{title}" }
                if let Some(detail) = detail {
                    span { "{detail}" }
                }
            }
            div { class: "approval-content", {children} }
            match answer {
                None => rsx! {
                    div { class: "approval-actions",
                        Button { compact: true, disabled: busy, onclick: move |event| on_decline.call(event), "{decline_label}" }
                        Button {
                            variant: ButtonVariant::Primary,
                            compact: true,
                            danger,
                            busy,
                            onclick: move |event| on_approve.call(event),
                            "{approve_label}"
                        }
                    }
                },
                Some(answer) => rsx! {
                    p { class: "approval-answer", role: "status",
                        Icon { name: if answer == Answer::Approved { IconName::Check } else { IconName::X } }
                        "{answered}"
                    }
                },
            }
        }
    }
}

static APPROVALS: AtomicUsize = AtomicUsize::new(0);

/// Something an agent made, such as a document, a letter or a table, as a card in the thread: its
/// icon, its title and a quieter `detail`, such as what kind of thing it is, and with `preview` the
/// opening of it, fading at the card's foot. While the agent is still `working` on it, its title
/// shimmers and the working mark stands where the way in would. With `onclick` the whole card opens
/// it, so the card holds no other controls and its preview takes no input.
#[component]
pub fn ArtifactCard(
    icon: IconName,
    title: String,
    detail: Option<String>,
    #[props(default)] working: bool,
    #[props(default)] preview: Option<Element>,
    onclick: Option<EventHandler<MouseEvent>>,
) -> Element {
    let mut class = String::from("card card-object artifact");
    if onclick.is_some() {
        class.push_str(" is-openable");
    }
    if working {
        class.push_str(" is-working");
    }
    rsx! {
        article { class, aria_busy: working.then_some("true"),
            span { class: "artifact-mark", Icon { name: icon } }
            div { class: "artifact-text",
                h3 { class: "artifact-title",
                    if let Some(open) = onclick {
                        button { class: "card-open", r#type: "button", onclick: move |event| open.call(event), "{title}" }
                    } else {
                        "{title}"
                    }
                }
                if let Some(detail) = detail {
                    p { class: "card-detail", "{detail}" }
                }
            }
            if working {
                WorkingMark {}
            } else if onclick.is_some() {
                span { class: "artifact-go", aria_hidden: "true", Icon { name: IconName::ChevronRight } }
            }
            if let Some(preview) = preview {
                div { class: "artifact-preview", "inert": "true", {preview} }
            }
        }
    }
}
