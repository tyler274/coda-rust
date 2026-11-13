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

use crate::{
    Button, ButtonVariant, Icon, IconName, ListRow, SourcesButton, Spinner, StatusLine,
    WorkingMark, script, sources::PlacedPopover,
};

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
/// in red; stopped before it finished, its mark turns to the stop square and its words go quiet.
/// With a `body`, such as the reasoning itself or a tool's input and output, the row is a
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
    let mark = match state {
        StepState::Failed => IconName::Alert,
        StepState::Stopped => IconName::Square,
        StepState::Running | StepState::Done => icon,
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
/// While `working` the line also carries the working mark. The steps start folded under the line;
/// pressing the line opens or closes them, and that choice holds while the work runs and after it
/// ends: nothing folds them on its own.
#[component]
pub fn WorkTrace(
    label: String,
    #[props(default)] working: bool,
    detail: Option<String>,
    children: Element,
) -> Element {
    let id = use_hook(|| format!("work-trace-{}", TRACES.fetch_add(1, Ordering::Relaxed)));
    // Opened and closed only by the person.
    let mut open = use_signal(|| false);
    let open_now = open();
    rsx! {
        section {
            class: if working { "work-trace is-working" } else { "work-trace" },
            "data-state": if open_now { "open" } else { "closed" },
            aria_busy: working.then_some("true"),
            button {
                id: "{id}-line",
                class: "work-trace-line",
                r#type: "button",
                aria_expanded: "{open_now}",
                aria_controls: "{id}",
                onclick: move |_| open.toggle(),
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
                class: if open_now { "disclosure-region is-open" } else { "disclosure-region" },
                aria_hidden: if open_now { "false" } else { "true" },
                "inert": (!open_now).then_some("true"),
                div { class: "disclosure-clip",
                    ol { class: "work-steps", {children} }
                }
            }
        }
    }
}

static TRACES: AtomicUsize = AtomicUsize::new(0);

/// One source in a finished answer's Sources list: its number, as the answer's citations show it,
/// its title, and a quieter detail, such as its date.
#[derive(Clone, Debug, PartialEq)]
pub struct IndexedSource {
    /// Its number, as the answer's citations show it.
    pub number: usize,
    /// Its title: "Discharge summary".
    pub title: String,
    /// A quieter detail, such as its date.
    pub detail: Option<String>,
}

/// A finished answer's Sources control: its words and the sources the answer cites, each once, in
/// the order of their numbers. How many there are is what the control counts.
#[derive(Clone, Debug, PartialEq)]
pub struct AnswerSources {
    /// The control's words, and the list's name: "Sources".
    pub label: String,
    /// The sources the answer cites.
    pub sources: Vec<IndexedSource>,
}

/// Coda's account of what a finished answer rests on and what could change it: its title and its
/// words, which may carry citation marks with the answer's numbers.
#[derive(Clone, PartialEq)]
pub struct Basis {
    /// The title: "Basis and limitations".
    pub title: String,
    /// What the answer rests on and what could change it.
    pub words: Element,
}

/// A way on after an answer ended: its words and what it does.
#[derive(Clone, PartialEq)]
pub struct EndAction {
    /// The action's words: "Resume", "Try again".
    pub label: String,
    /// What pressing it does.
    pub on_press: EventHandler,
}

/// How an answer ended, each ending holding only what it shows. Only a finished answer has words
/// with citation marks, Sources when it cites any, Basis and limitations, and a rating; only a
/// stopped one offers Resume; only one cut off or not finished offers Try again, and only while it
/// can be tried again.
#[derive(Clone, PartialEq)]
pub enum Ending {
    /// Coda completed the answer.
    Finished {
        /// The answer's words, with their citation marks.
        words: Element,
        /// The Sources control; none when the answer cites no source, and then no control shows.
        sources: Option<AnswerSources>,
        /// Basis and limitations, collapsed, ahead of Sources.
        basis: Basis,
        /// The host's rating of the answer, a `FeedbackRating`.
        rating: Element,
    },
    /// The person stopped the work: the stopped mark's words, and Resume until the person sends
    /// something else.
    Stopped {
        /// The stopped mark's words: "Stopped".
        mark: String,
        /// Resume, while the work waits for it.
        resume: Option<EndAction>,
    },
    /// Coda reached a limit before it finished: the line that says so, and Try again while it can
    /// be tried again. What Coda had written before the limit is the host's to show, as answer
    /// words through `Prose` before the ending, as it shows them while Coda works again, once it
    /// finishes and when stopped.
    CutOff {
        /// The line that says the answer was cut off.
        message: String,
        /// Try again; none once the answer can't be tried again, such as when a later message
        /// replaced it.
        retry: Option<EndAction>,
    },
    /// Coda couldn't finish: the line that says so, and Try again while it can be tried again.
    Failed {
        /// The line that says Coda couldn't finish.
        message: String,
        /// Try again; none once the answer can't be tried again.
        retry: Option<EndAction>,
    },
}

/// An answer as it ended, rising in as it arrives. Finished, its words, then one row: Basis and
/// limitations as quiet words with a chevron, collapsed; the Sources pill with its count when the
/// answer cites any; and right after them the rating, which shows as a rating does under any
/// message: while the answer is hovered or focused, and always on touch. Basis opens in place
/// beneath the row as plain text. Sources opens a small list of the cited sources beside it,
/// numbered as in the words, on the side with more room and never over the pill; choosing one
/// closes the list and chooses that source's first citation in the answer, brought into view, so
/// the source opens as that citation opens it.
/// Stopped, the stopped mark with Resume beside it. Cut off or not finished, one line, with Try
/// again under it while the answer can be tried again; words Coda kept before a limit are the
/// host's, shown before the ending. A line that ends the wait announces itself.
#[component]
pub fn AnswerEnding(ending: Ending) -> Element {
    let id = use_hook(|| format!("answer-basis-{}", BASES.fetch_add(1, Ordering::Relaxed)));
    let mut open = use_signal(|| false);
    match ending {
        Ending::Finished {
            words,
            sources,
            basis,
            rating,
        } => {
            let answer = format!("{id}-answer");
            let list = format!("{id}-sources");
            let control = format!("{id}-sources-control");
            rsx! {
                div { id: "{answer}", class: "answer-ending",
                    div { class: "answer-words", {words} }
                    div { class: "answer-ending-row",
                        button {
                            class: "answer-control",
                            r#type: "button",
                            aria_expanded: "{open}",
                            aria_controls: "{id}",
                            onclick: move |_| open.toggle(),
                            "{basis.title}"
                            Icon { name: IconName::ChevronDown }
                        }
                        if let Some(AnswerSources { label, sources }) = sources {
                            SourcesButton { id: control.clone(), label: label.clone(), count: sources.len(), opens: list.clone() }
                            PlacedPopover { id: list.clone(), label, control,
                                ol { class: "row-list sources-index",
                                    for source in sources {
                                        li { key: "{source.number}",
                                            ListRow {
                                                number: source.number,
                                                title: source.title,
                                                detail: source.detail,
                                                onclick: {
                                                    let (answer, list) = (answer.clone(), list.clone());
                                                    move |_| {
                                                        script::toggle_popover(&list, false);
                                                        script::open_citation(&answer, source.number);
                                                    }
                                                },
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        {rating}
                    }
                    div {
                        id: "{id}",
                        class: if open() { "disclosure-region is-open" } else { "disclosure-region" },
                        aria_hidden: if open() { "false" } else { "true" },
                        "inert": (!open()).then_some("true"),
                        div { class: "disclosure-clip",
                            div { class: "answer-basis", {basis.words} }
                        }
                    }
                }
            }
        }
        Ending::Stopped { mark, resume } => rsx! {
            div { class: "answer-ending answer-stopped",
                span { class: "answer-stopped-mark",
                    Icon { name: IconName::Square }
                    "{mark}"
                }
                if let Some(EndAction { label, on_press }) = resume {
                    Button { compact: true, onclick: move |_| on_press.call(()), "{label}" }
                }
            }
        },
        Ending::CutOff { message, retry } | Ending::Failed { message, retry } => {
            let (label, on_retry) = match retry {
                Some(EndAction { label, on_press }) => (label, Some(on_press)),
                None => (String::new(), None),
            };
            rsx! {
                div { class: "answer-ending",
                    StatusLine { message, alert: true, retry: label, on_retry }
                }
            }
        }
    }
}

static BASES: AtomicUsize = AtomicUsize::new(0);

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
    fn an_ending_that_cannot_be_tried_again_shows_its_line_and_no_button() {
        fn app() -> Element {
            rsx! {
                AnswerEnding {
                    ending: Ending::CutOff {
                        message: "Coda reached a limit before finishing.".into(),
                        retry: None,
                    },
                }
                AnswerEnding {
                    ending: Ending::Failed {
                        message: "Coda couldn’t finish.".into(),
                        retry: None,
                    },
                }
            }
        }
        let mut dom = VirtualDom::new(app);
        dom.rebuild_in_place();
        let html = dioxus_ssr::render(&dom);
        assert!(
            html.contains("Coda reached a limit before finishing."),
            "{html}"
        );
        assert!(html.contains("Coda couldn’t finish."), "{html}");
        assert_eq!(html.matches(r#"role="alert""#).count(), 2, "{html}");
        assert!(!html.contains("<button"), "{html}");
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
