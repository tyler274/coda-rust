//! Agent work: what an agent is doing and has done, shown as it happens. Its steps (thinking,
//! reading, searching, running a tool) stand under one line that says what it did, the plan it
//! works through checks off as it goes, what it asks before acting waits for the person's answer,
//! and what it makes stands as a card that opens it. Every state here is the host's, from the
//! agent's own run; nothing invents progress.

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::atomic::{AtomicUsize, Ordering};

use dioxus::prelude::*;

use crate::{Button, ButtonVariant, Icon, IconName, Spinner, WorkingMark, script};

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
    // Folding away when the work ends brings focus inside the steps back to the line.
    let mut chosen = use_signal(|| None::<bool>);
    let region = id.clone();
    use_effect(use_reactive!(|working| {
        chosen.set(None);
        if !working {
            script::refocus_line(&region);
        }
    }));
    let open = chosen().unwrap_or(working);
    rsx! {
        section {
            class: if working { "work-trace is-working" } else { "work-trace" },
            "data-state": if open { "open" } else { "closed" },
            aria_busy: working.then_some("true"),
            button {
                id: "{id}-line",
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
    let seen = use_hook(|| Rc::new(RefCell::new(HashMap::new())));
    let marks = finished(&mut seen.borrow_mut(), &tasks);
    rsx! {
        ol { class: "task-list",
            for (index, (task, (key, drawn))) in tasks.into_iter().zip(marks).enumerate() {
                li {
                    key: "{key}",
                    class: match task.state {
                        TaskState::Pending => "task",
                        TaskState::Active => "task is-active",
                        TaskState::Done if drawn => "task is-done is-new",
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

/// Each task's key and whether its check draws in, given where each stood when last shown, which
/// it then updates. A task is known by its words; tasks that share their words can't be told
/// apart, so theirs never draw. Only a task that became done while shown draws its check, and it
/// keeps that mark while it stays done; a task not shown before arrives as it stands.
fn finished(seen: &mut HashMap<String, (TaskState, bool)>, tasks: &[Task]) -> Vec<(String, bool)> {
    let mut counts = HashMap::new();
    for task in tasks {
        *counts.entry(task.label.as_str()).or_insert(0) += 1;
    }
    let mut placed = HashMap::new();
    let marks: Vec<(String, bool)> = tasks
        .iter()
        .map(|task| {
            let place = placed.entry(task.label.as_str()).or_insert(0);
            *place += 1;
            let drawn = match (seen.get(&task.label), task.state) {
                (_, state) if state != TaskState::Done || counts[task.label.as_str()] > 1 => false,
                (Some(&(TaskState::Done, drawn)), _) => drawn,
                (Some(_), _) => true,
                (None, _) => false,
            };
            (format!("{}#{place}", task.label), drawn)
        })
        .collect();
    *seen = marks
        .iter()
        .zip(tasks)
        .filter(|(_, task)| counts[task.label.as_str()] == 1)
        .map(|((_, drawn), task)| (task.label.clone(), (task.state, *drawn)))
        .collect();
    marks
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

/// Ambient state of Coda's collapsed companion [`Orb`].
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum OrbState {
    /// Quiet ambient presence at rest.
    #[default]
    Idle,
    /// One or more background reviews are actively running.
    Working,
    /// An urgent concern awaits opening.
    Urgent,
}

/// Coda's collapsed companion mark: a circular control carrying Coda's icon and ambient motion
/// that respects `prefers-reduced-motion`.
#[component]
pub fn Orb(
    aria_label: String,
    #[props(default)] state: OrbState,
    #[props(default)] expanded: Option<bool>,
    onclick: EventHandler<MouseEvent>,
) -> Element {
    let class = match state {
        OrbState::Idle => "coda-orb",
        OrbState::Working => "coda-orb has-orb-work",
        OrbState::Urgent => "coda-orb has-orb-urgent",
    };
    rsx! {
        button {
            class,
            r#type: "button",
            aria_label: "{aria_label}",
            aria_expanded: expanded.map(|open| if open { "true" } else { "false" }),
            onclick: move |event| onclick.call(event),
            span { class: "coda-orb-aura", aria_hidden: "true" }
            span { class: "coda-orb-core", aria_hidden: "true",
                Icon { name: IconName::Sparkle }
            }
            if state == OrbState::Working {
                span { class: "coda-orb-working", aria_hidden: "true", WorkingMark {} }
            } else if state == OrbState::Urgent {
                span { class: "coda-orb-urgent-dot", aria_hidden: "true" }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn task(label: &str, state: TaskState) -> Task {
        Task {
            label: label.into(),
            state,
        }
    }

    fn drawn(seen: &mut HashMap<String, (TaskState, bool)>, tasks: &[Task]) -> Vec<bool> {
        finished(seen, tasks)
            .into_iter()
            .map(|(_, drawn)| drawn)
            .collect()
    }

    #[test]
    fn only_a_task_that_finishes_while_shown_draws_its_check() {
        let mut seen = HashMap::new();
        let first = [
            task("Find", TaskState::Done),
            task("Gather", TaskState::Active),
        ];
        assert_eq!(drawn(&mut seen, &first), [false, false]);
        // Gather finishes; a task already done arrives at the end; both stay as they are after.
        let next = [
            task("Find", TaskState::Done),
            task("Gather", TaskState::Done),
            task("Ask", TaskState::Done),
        ];
        assert_eq!(drawn(&mut seen, &next), [false, true, false]);
        assert_eq!(drawn(&mut seen, &next), [false, true, false]);
        // Moved into another's place, a done task is still the one it was.
        let moved = [
            task("Ask", TaskState::Done),
            task("Find", TaskState::Done),
            task("Gather", TaskState::Done),
        ];
        assert_eq!(drawn(&mut seen, &moved), [false, false, true]);
    }

    #[test]
    fn tasks_that_share_their_words_never_draw() {
        let mut seen = HashMap::new();
        let both = [
            task("Check", TaskState::Active),
            task("Check", TaskState::Done),
        ];
        assert_eq!(drawn(&mut seen, &both), [false, false]);
        // The active one leaves: the one left was done all along.
        assert_eq!(drawn(&mut seen, &[task("Check", TaskState::Done)]), [false]);
        let again = [
            task("Check", TaskState::Done),
            task("Check", TaskState::Active),
        ];
        assert_eq!(drawn(&mut seen, &again), [false, false]);
        assert_eq!(
            drawn(
                &mut seen,
                &[
                    task("Check", TaskState::Done),
                    task("Check", TaskState::Done)
                ]
            ),
            [false, false]
        );
    }
}
