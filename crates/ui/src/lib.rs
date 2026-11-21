//! Coda's design, as a catalog of AI-native component families any surface can compose: the
//! tokens (colour, type, space, shape and motion), the icons, and each component's look, motion,
//! gestures, states and accessibility. It holds no product rules. The surface that places a
//! component decides when it shows, what pressing it does, what its server guarantees, every word
//! it says, and how the screen is laid out. `crates/ui-gallery` renders every family. Its browser
//! behaviour runs from `assets/behaviour.js` as a module, never through `eval`, so it works on a
//! page that refuses `eval`, such as a Chrome extension's.
#![deny(missing_docs)]

use std::sync::atomic::{AtomicUsize, Ordering};

use dioxus::dioxus_core::AttributeValue;
use dioxus::html::FileData;
use dioxus::prelude::*;

mod agent;
mod attachments;
mod cards;
mod charts;
mod code;
mod conversation;
mod documents;
mod feedback;
mod icons;
mod inputs;
mod live;
mod motion;
mod overlays;
mod prose;
mod script;
mod sources;
mod stack;
mod tables;
mod toast;
pub use agent::{
    Answer, AnswerEnding, AnswerSources, ApprovalCard, ApprovalState, ArtifactCard, Basis,
    EndAction, Ending, IndexedSource, Orb, OrbState, SourcesPopover, StepState, Task, TaskList,
    TaskState, TaskWords, WorkStep, WorkTrace,
};
pub use attachments::{AttachButton, Attachment, AttachmentState, Attachments};
pub use cards::{Card, RailCard};
pub use charts::{
    ChartHeader, Comparison, ComparisonBars, MeasurementTile, RunningChart, SeriesChart,
    SeriesPoint, SeriesShape, Sparkline, Value,
};
pub use code::{CodeBlock, CopyButton, CopyFailed, CopyWords, copy_text};
pub use conversation::{
    ChatColumn, ChatHeader, ContextChip, ConversationViewport, DateSeparator, Dictation,
    HeaderFact, MessageActions, MessageEditor, MessageQueue, MessageStatus, QueuePlace, Suggestion,
    VersionSwitch,
};
pub use documents::{FooterActions, Notice, Paper, PaperSection, TopBar};
pub use feedback::{FeedbackRating, Rating, ToggleChip};
pub use icons::{Icon, IconName};
pub use inputs::{
    Checkbox, Choice, Combobox, CompactField, DateField, DateKind, RadioGroup, Segmented, Select,
    TextField, TextKind,
};
pub use live::{ListedRow, LiveRows, RowFrame, use_live_rows};
pub use motion::Stylesheet;
pub use overlays::{ContextMenu, Menu, MenuItem, MenuSection, Tooltip};
pub use prose::{Prose, ProseNames, ProsePart};
pub use script::{
    Appearance, Asked, DictationAudio, DictationHeard, DictationTrigger, Listening, Unheard,
    cancel_dictation, dismiss_open_popovers, read_appearance, set_appearance, start_dictation,
    stop_dictation, watch_cover_shortcut,
};
pub use sources::{
    CitationPreview, CitedAt, CitedField, CitedPlace, ExpandSource, FieldGroup, FieldValue,
    PartRange, Places, PreviewContent, RawRecord, RecordAbout, RecordBody, RecordField,
    RecordStatus, SourceLink, SourcePane, SourceReading, SourceRecord, SourceState, SourceSummary,
    TextPart,
};
pub use stack::{
    Layer, Nav, Opened, Screen, ScreenFrame, ScreenStack, Serial, Standing, use_screen_stack,
};
pub use tables::{Column, SortOrder, Table, TableRow};
pub use toast::{TOAST_MS, Toast};

/// How long a surface takes to settle away before it is removed: `--motion-control` in
/// `tokens.css`, which owns it. Removal waits on a timer rather than the animation's end because
/// reduced motion turns the animation off.
pub(crate) const EXIT_MS: u64 = 140;

/// The crate's tokens and components, drawn by `Stylesheet`.
const STYLESHEET: &str = concat!(
    include_str!("../assets/tokens.css"),
    include_str!("../assets/components.css"),
);

/// A screen's heading: its title, what the screen belongs to on the
/// line above (the day, a collection), and its state or source on the line beneath. A record's
/// heading leads with its icon. A screen with no top bar sets its own `actions` above the title,
/// at the trailing edge.
#[component]
pub fn PageTitle(
    text: String,
    #[props(default)] eyebrow: Option<String>,
    #[props(default)] detail: Option<String>,
    #[props(default)] icon: Option<IconName>,
    #[props(default)] actions: Option<Element>,
) -> Element {
    if eyebrow.is_none() && detail.is_none() && icon.is_none() && actions.is_none() {
        return rsx! { h1 { class: "page-title", "{text}" } };
    }
    rsx! {
        header { class: "page-heading",
            if let Some(actions) = actions {
                div { class: "page-heading-actions", {actions} }
            }
            div { class: "page-heading-main",
                if let Some(name) = icon {
                    IconTile { name }
                }
                div { class: "page-heading-text",
                    if let Some(eyebrow) = eyebrow {
                        p { class: "page-heading-eyebrow", "{eyebrow}" }
                    }
                    h1 { class: "page-title", "{text}" }
                    if let Some(detail) = detail {
                        p { class: "card-detail", "{detail}" }
                    }
                }
            }
        }
    }
}

/// What a card holds, named above its title in small capitals: "Original record".
#[component]
fn Overline(text: String) -> Element {
    rsx! { p { class: "overline", "{text}" } }
}

/// An empty section: one plain line, with no box or mark.
/// What comes next on the screen carries the action.
#[component]
pub fn EmptyLine(message: String) -> Element {
    rsx! { p { class: "empty", "{message}" } }
}

/// An empty or caught-up place: its mark, one line, what to
/// know, and up to two quiet ways on, centred. The second is the quieter way out when the first
/// may never succeed.
#[component]
pub fn EmptyState(
    icon: IconName,
    title: String,
    #[props(default)] detail: String,
    /// The way on's words; with `on_action`, it is offered, as it is only with both.
    #[props(default)]
    action: String,
    on_action: Option<EventHandler>,
    /// The quieter way out's words; with `on_secondary`, it is offered, as it is only with both.
    #[props(default)]
    secondary: String,
    on_secondary: Option<EventHandler>,
) -> Element {
    rsx! {
        div { class: "empty-state",
            span { class: "empty-state-mark", aria_hidden: "true", Icon { name: icon } }
            p { class: "empty-state-title", "{title}" }
            if !detail.is_empty() {
                p { class: "empty-state-detail", "{detail}" }
            }
            if let Some(handler) = offered(&action, on_action) {
                Button {
                    variant: ButtonVariant::Quiet,
                    class: "empty-state-action",
                    onclick: move |_| handler.call(()),
                    "{action}"
                }
            }
            if let Some(handler) = offered(&secondary, on_secondary) {
                Button {
                    variant: ButtonVariant::Quiet,
                    class: "empty-state-secondary",
                    onclick: move |_| handler.call(()),
                    "{secondary}"
                }
            }
        }
    }
}

/// The one status line: what failed or what is under way, in
/// one muted line, with the one Try again under the words after a failure. While pending it draws
/// the shape of what is coming, after the shared delay so a fast read never flashes, and keeps its
/// words for assistive technology. A failure the person was waiting on announces itself.
#[component]
pub fn StatusLine(
    message: String,
    #[props(default)] alert: bool,
    #[props(default)] pending: bool,
    /// The retry's words; with `on_retry`, the line offers it, as it does only with both.
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
            if let Some(handler) = offered(&retry, on_retry) {
                TryAgain { label: retry, on_retry: handler }
            }
        }
    }
}

/// An action is offered only with both its words and its handler, so no control goes unnamed.
fn offered(words: &str, handler: Option<EventHandler>) -> Option<EventHandler> {
    handler.filter(|_| !words.trim().is_empty())
}

/// The one recovery action after a failure: one verb, one shape, one size, under the line that
/// says what failed. While the retry runs, `running` names it for assistive technology and the
/// control waits as a busy `Button` does.
#[component]
pub fn TryAgain(
    label: String,
    #[props(default)] disabled: bool,
    /// The retry is running, and this is what it is called meanwhile: "Trying again".
    #[props(default)]
    running: Option<String>,
    on_retry: EventHandler,
) -> Element {
    let busy = running.is_some();
    rsx! {
        Button { compact: true, disabled, busy, pending_label: running, onclick: move |_| on_retry.call(()), "{label}" }
    }
}

/// The shape of what is loading.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum PlaceholderShape {
    /// A line of words.
    #[default]
    Line,
    /// List items: a mark and a name.
    Items,
    /// Rows: a name, then words.
    Rows,
    /// Reading: a title, then prose.
    Reading,
}

/// What is loading, drawn as its shape: bars in the text's own measure with
/// a slow sheen passing over them. Decoration only; the host's words stay with assistive
/// technology.
#[component]
pub fn Placeholder(#[props(default)] shape: PlaceholderShape) -> Element {
    let bar = |kind: &str| rsx! { span { class: "placeholder-bar {kind}" } };
    match shape {
        PlaceholderShape::Line => rsx! {
            span { class: "placeholder is-line", aria_hidden: "true", {bar("")} }
        },
        PlaceholderShape::Items => rsx! {
            span { class: "placeholder is-items", aria_hidden: "true",
                for index in 0..3 {
                    span { key: "{index}", class: "placeholder-item", {bar("is-mark")} {bar("")} }
                }
            }
        },
        PlaceholderShape::Rows => rsx! {
            span { class: "placeholder is-rows", aria_hidden: "true",
                for index in 0..3 {
                    span { key: "{index}", class: "placeholder-row",
                        {bar("is-title")} {bar("")} {bar("is-short")}
                    }
                }
            }
        },
        PlaceholderShape::Reading => rsx! {
            span { class: "placeholder is-reading", aria_hidden: "true",
                {bar("is-title")} {bar("")} {bar("")} {bar("")} {bar("is-short")}
            }
        },
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
    /// Flagged.
    Warning,
    /// What failed or conflicts.
    Error,
}

/// Where a thing stands, on its tone, led by its kind's icon. The host names the
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

/// The one working mark: three live dots lighting in turn, shown only while work actually runs; in
/// a reply's place, it says a reply is coming. The words beside it, or the host's status, say what
/// runs.
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

/// Running work: the working mark beside the verb that
/// says what runs, and under it, while there is one, the step under way. It announces itself
/// unless the host says it shouldn't.
#[component]
pub fn WorkingPresence(
    label: String,
    /// The smaller form, for a toolbar or a row.
    #[props(default)]
    compact: bool,
    /// The verb's hover title and accessible description, such as when the work started.
    #[props(default)]
    hint: String,
    /// The step under way.
    #[props(default)]
    activity: String,
    #[props(default = true)] announce: bool,
) -> Element {
    rsx! {
        div {
            class: if compact { "working-presence is-compact" } else { "working-presence" },
            role: if announce { "status" },
            WorkingMark {}
            span { class: "working-presence-verb", title: if !hint.is_empty() { "{hint}" }, "{label}" }
            if !activity.is_empty() {
                span { class: "run-activity", "{activity}" }
            }
        }
    }
}

/// A count that changes quietly: the number and
/// its words fade into a new value together, once, so they never move apart. The first value is
/// simply there.
#[component]
pub fn Count(value: usize, #[props(default)] words: String) -> Element {
    // The value last drawn, and how many times it has changed since.
    let seen = use_hook(|| std::rc::Rc::new(std::cell::Cell::new((value, 0u64))));
    let (last, changes) = seen.get();
    let changes = if last == value { changes } else { changes + 1 };
    seen.set((value, changes));
    // Each change takes the other of two identical animations, so the next change runs again.
    let class = match changes {
        0 => "count",
        odd if odd % 2 == 1 => "count is-changed",
        _ => "count is-changed-again",
    };
    rsx! {
        span { class,
            span { class: "count-value", "{value}" }
            if !words.is_empty() {
                " {words}"
            }
        }
    }
}

/// A key that does what the control does, shown beside the
/// control's words where there is a keyboard, hidden from assistive technology, which hears the
/// control's own shortcut.
#[component]
pub fn KeyHint(key_label: String) -> Element {
    rsx! { kbd { class: "key-hint", aria_hidden: "true", "{key_label}" } }
}

/// A draft saved elsewhere from the same revision:
/// what happened, the saved draft's words as written, and then the host's way to choose.
#[component]
pub fn DraftConflict(
    message: String,
    /// What the saved words are called, above them.
    #[props(default)]
    label: String,
    saved: String,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut class = String::from("draft-conflict");
    let rest = host_attributes(&mut class, attributes);
    rsx! {
        div { class: "{class}", role: "alert", ..rest,
            p { "{message}" }
            if !label.is_empty() {
                p { class: "draft-conflict-label", "{label}" }
            }
            pre { "{saved}" }
            {children}
        }
    }
}

/// Waiting on an action the person took: a turning ring where the
/// control's icon was, at icon size. Running work keeps the working mark; content still loading
/// keeps its placeholder.
#[component]
pub fn Spinner() -> Element {
    rsx! {
        svg {
            class: "icon spinner",
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "2.2",
            stroke_linecap: "round",
            "aria-hidden": "true",
            circle { class: "spinner-track", cx: "12", cy: "12", r: "9" }
            path { d: "M21 12a9 9 0 0 0-9-9" }
        }
    }
}

/// How far something has got, which only the host knows: `value` runs from 0 to 1, and without one,
/// or when it isn't a number, the bar moves without claiming an amount. `label` names it and
/// `reading` says how far in the host's words, such as "3 of 12 notes"; both stand above the bar,
/// and are only spoken when it is `bare`, for a place that already says what it is.
#[component]
pub fn ProgressBar(
    label: String,
    value: Option<f64>,
    reading: Option<String>,
    #[props(default)] bare: bool,
) -> Element {
    let filled = filled_percent(value);
    let bar = rsx! {
        div {
            class: if filled.is_some() { "progress" } else { "progress is-unknown" },
            role: "progressbar",
            aria_label: "{label}",
            aria_valuemin: "0",
            aria_valuemax: "100",
            aria_valuenow: filled.map(|percent| percent.to_string()),
            aria_valuetext: reading.clone(),
            style: filled.map(|percent| format!("--progress: {percent}%")),
        }
    };
    if bare {
        return bar;
    }
    rsx! {
        div { class: "progress-labelled",
            div { class: "field-heading", aria_hidden: "true",
                span { "{label}" }
                if let Some(reading) = reading {
                    span { class: "field-reading", "{reading}" }
                }
            }
            {bar}
        }
    }
}

/// How much of a bar is filled: never all of it before the work is all done, and no amount at all
/// from a value that isn't a number, as a share of nothing isn't.
fn filled_percent(value: Option<f64>) -> Option<f64> {
    value
        .filter(|done| done.is_finite())
        .map(|done| (done.clamp(0.0, 1.0) * 100.0).floor())
}

/// How large an avatar is drawn.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum AvatarSize {
    /// In a list row.
    #[default]
    Small,
    /// Beside a current person in navigation.
    Medium,
    /// At the head of a person's page.
    Large,
}

/// A person's initials in a circle. Decoration only: the
/// person's name belongs to the words beside it.
#[component]
pub fn Avatar(name: String, #[props(default)] size: AvatarSize) -> Element {
    let class = match size {
        AvatarSize::Small => "avatar",
        AvatarSize::Medium => "avatar avatar-md",
        AvatarSize::Large => "avatar avatar-lg",
    };
    rsx! { span { class, aria_hidden: "true", "{initials(&name)}" } }
}

/// The first letters of a name's first two words.
fn initials(name: &str) -> String {
    name.split_whitespace()
        .take(2)
        .filter_map(|word| word.chars().next())
        .flat_map(char::to_uppercase)
        .collect()
}

/// A leading icon on its tile: neutral on a hairline, as a card header or a record's heading leads,
/// or `tinted` in the data colour, as a row leads.
#[component]
pub fn IconTile(name: IconName, #[props(default)] tinted: bool) -> Element {
    rsx! {
        span { class: if tinted { "icon-tile is-tinted" } else { "icon-tile" }, aria_hidden: "true", Icon { name } }
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

/// A command: one shape, a variant, and the shared
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
    /// Waiting on the action it took: the spinner turns where its icon was, and it takes no input.
    #[props(default)]
    busy: bool,
    /// What it is called while busy, so the wait is announced.
    #[props(default)]
    pending_label: Option<String>,
    /// Submits its form instead of acting on its own.
    #[props(default)]
    submit: bool,
    /// The id of the `Menu` it opens and closes, which stands beside it.
    #[props(default)]
    opens: Option<String>,
    onclick: Option<EventHandler<MouseEvent>>,
    /// The press, before it takes focus: a control that leaves focus where it is prevents it here.
    onmousedown: Option<EventHandler<MouseEvent>>,
    /// The control reaching the page, for a caller that later moves focus to it.
    onmounted: Option<EventHandler<MountedEvent>>,
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
    let pending = pending_label.filter(|_| busy);
    rsx! {
        button {
            class: "{class}",
            r#type: if submit { "submit" } else { "button" },
            disabled: disabled || busy,
            aria_busy: if busy { "true" },
            aria_label: pending,
            popovertarget: opens,
            onclick: move |event| {
                if let Some(handler) = &onclick {
                    handler.call(event);
                }
            },
            onmousedown: move |event| {
                if let Some(handler) = &onmousedown {
                    handler.call(event);
                }
            },
            onmounted: move |event| {
                if let Some(handler) = &onmounted {
                    handler.call(event);
                }
            },
            ..rest,
            if busy {
                Spinner {}
            }
            {children}
        }
    }
}

/// In-place navigation: a quiet link, separate from
/// commands, that opens another place or goes back. It reads in body text and underlines on hover;
/// a surface may colour it once at its root (`--navigation-color`).
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

/// A row that opens something: the whole row is one button
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
    /// The assistant: its words read as plain prose.
    Assistant,
}

/// Where the person's message stands on its way.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum MessageState {
    /// Delivered.
    #[default]
    Sent,
    /// On its way.
    Sending,
    /// Waiting its turn, such as until the reply under way is done.
    Queued,
    /// Didn't go.
    Failed,
}

/// One message in a thread:
/// the person's words in a light bubble at the far edge, the assistant's as plain prose. Who
/// wrote it stays with assistive technology. A message rises in as it appears. On its way it
/// stands a little quieter; waiting its turn, its bubble is outlined, not filled; when it didn't
/// go, its outline is red. Its `status`, a `MessageStatus`, and its `actions`, a `MessageActions`
/// row, stand close under it, at the bubble's edge for the person's words; waiting its turn,
/// they stand in one row inside its bubble, the status first and the actions at the far edge.
#[component]
pub fn Message(
    author: Author,
    /// Who wrote it, for assistive technology.
    #[props(default)]
    by: String,
    #[props(default)] state: MessageState,
    #[props(default)] status: Option<Element>,
    #[props(default)] actions: Option<Element>,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut class = String::from(match author {
        Author::Person => "message message-person",
        Author::Assistant => "message message-assistant",
    });
    class.push_str(match state {
        MessageState::Sent => "",
        MessageState::Sending => " is-sending",
        MessageState::Queued => " is-queued",
        MessageState::Failed => " is-failed",
    });
    let rest = host_attributes(&mut class, attributes);
    let queued = state == MessageState::Queued;
    let row = queued && (status.is_some() || actions.is_some());
    rsx! {
        div { class: if author == Author::Person { "message-turn is-person" } else { "message-turn" },
            article { class: "{class}", ..rest,
                if !by.is_empty() {
                    span { class: "sr-only", "{by}" }
                }
                {children}
                if row {
                    div { class: "message-queued-row", {status.clone()} {actions.clone()} }
                }
            }
            if !queued {
                {status}
                {actions}
            }
        }
    }
}

/// The one search field: the search mark, then the words on
/// body large, and, while there are words, the field's own clear in place of the browser's, which
/// empties the field and keeps focus in it. Its name and words are the host's; so are its keys.
#[component]
pub fn SearchField(
    /// The field's accessible name.
    label: String,
    value: String,
    on_input: EventHandler<String>,
    #[props(default)] placeholder: String,
    /// The clear control's accessible name.
    clear_label: String,
    #[props(default)] autofocus: bool,
    onkeydown: Option<EventHandler<KeyboardEvent>>,
    onfocus: Option<EventHandler<FocusEvent>>,
    onblur: Option<EventHandler<FocusEvent>>,
) -> Element {
    let mut input = use_signal(|| None::<std::rc::Rc<MountedData>>);
    rsx! {
        div { class: "search-field field",
            Icon { name: IconName::Search }
            input {
                r#type: "search",
                aria_label: "{label}",
                placeholder: "{placeholder}",
                value: "{value}",
                autofocus,
                onmounted: move |event| input.set(Some(event.data())),
                oninput: move |event| on_input.call(event.value()),
                onkeydown: move |event| {
                    if let Some(handler) = onkeydown {
                        handler.call(event);
                    }
                },
                onfocus: move |event| {
                    if let Some(handler) = onfocus {
                        handler.call(event);
                    }
                },
                onblur: move |event| {
                    if let Some(handler) = onblur {
                        handler.call(event);
                    }
                },
            }
            if !value.is_empty() {
                Button {
                    variant: ButtonVariant::Quiet,
                    icon_only: true,
                    compact: true,
                    aria_label: "{clear_label}",
                    // Pressing it leaves focus in the field, so clearing never ends the search.
                    onmousedown: move |event: MouseEvent| event.prevent_default(),
                    onclick: move |_| {
                        on_input.call(String::new());
                        if let Some(field) = input.peek().clone() {
                            spawn(async move {
                                let _ = field.set_focus(true).await;
                            });
                        }
                    },
                    Icon { name: IconName::X }
                }
            }
        }
    }
}

/// The composer: the one raised text field, holding the words and, under
/// them, a toolbar of the host's context and actions; a host's submit control sends. `inline`
/// puts the actions beside the words instead, at their last line.
/// The words grow with what is written, up to seven lines. Keys stay the host's. With `on_files`,
/// files dragged over it are taken: the field says `drop_label` while they are over it, and the
/// files dropped go to `on_files`. Words dragged in drop into the field as they always do. While the person speaks, a `Dictation` given as `dictation`
/// stands in the words' place; the words wait, kept, until it is gone.
#[component]
pub fn Composer(
    /// The words' accessible name.
    label: String,
    #[props(default)] placeholder: String,
    value: String,
    on_input: EventHandler<String>,
    /// The words as written, when the host's submit control is pressed.
    on_submit: EventHandler<String>,
    /// Key press inside the composer textarea, before default handling.
    #[props(default)]
    on_key_down: Option<EventHandler<KeyboardEvent>>,
    /// Whether the words have focus.
    on_editing: Option<EventHandler<bool>>,
    /// What the person brought in, held above the words: a `ContextChip`.
    #[props(default)]
    context: Option<Element>,
    /// The actions sit beside the words rather than under them.
    #[props(default)]
    inline: bool,
    /// What is happening, at the toolbar's leading edge: working presence, the draft's state, or
    /// guidance.
    #[props(default)]
    status: Option<Element>,
    /// What needs the person under the toolbar, inside the field: a `DraftConflict`, or what
    /// failed.
    #[props(default)]
    notices: Option<Element>,
    on_files: Option<EventHandler<Vec<FileData>>>,
    /// What the field says while files are dragged over it.
    #[props(default)]
    drop_label: String,
    #[props(default)] dictation: Option<Element>,
    children: Element,
) -> Element {
    let held = value.clone();
    let id = use_hook(|| format!("composer-{}", COMPOSERS.fetch_add(1, Ordering::Relaxed)));
    let takes_files = on_files.is_some();
    let dropping = id.clone();
    use_effect(use_reactive!(|takes_files| {
        if takes_files {
            script::file_drags(&dropping);
        }
    }));
    let mut class = String::from(if inline {
        "composer field is-inline"
    } else {
        "composer field"
    });
    if dictation.is_some() {
        class.push_str(" is-dictating");
    }
    rsx! {
        form {
            id,
            class,
            ondrop: move |event| {
                if let Some(take) = on_files {
                    let files = event.data_transfer().files();
                    if !files.is_empty() {
                        take.call(files);
                    }
                }
            },
            onsubmit: move |event| {
                event.prevent_default();
                let typed = event.get_first("draft").and_then(|value| match value {
                    FormValue::Text(text) => Some(text),
                    FormValue::File(_) => None,
                });
                on_submit.call(submitted_text(typed.as_deref(), &held));
            },
            {context}
            {dictation}
            textarea {
                class: "composer-input",
                name: "draft",
                rows: "1",
                aria_label: "{label}",
                placeholder: "{placeholder}",
                value: "{value}",
                oninput: move |event| on_input.call(event.value()),
                onkeydown: move |event| {
                    if let Some(handler) = &on_key_down {
                        handler.call(event);
                    }
                },
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
            div { class: "composer-toolbar",
                if let Some(status) = status {
                    div { class: "composer-context", {status} }
                }
                {children}
            }
            {notices}
            if takes_files {
                div { class: "composer-drop", aria_hidden: "true", "{drop_label}" }
            }
        }
    }
}

static COMPOSERS: AtomicUsize = AtomicUsize::new(0);

/// The words a send uses. The field wins when it has something in it: on Android the controlled
/// value can drop keystrokes while the person still sees them.
fn submitted_text(field: Option<&str>, value: &str) -> String {
    match field {
        Some(text) if !text.trim().is_empty() => text.to_owned(),
        _ => value.to_owned(),
    }
}

static DISCLOSURES: AtomicUsize = AtomicUsize::new(0);

/// A section that opens in place: its title is one
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
    let toggle = rsx! {
        button {
            class: "disclosure-toggle",
            r#type: "button",
            aria_expanded: "{open}",
            aria_controls: "{id}",
            onclick: move |_| open.toggle(),
            strong { "{title}" }
            Icon { name: IconName::ChevronDown }
        }
    };
    rsx! {
        section { class: "{class}", "data-state": if open() { "open" } else { "closed" },
            // A detail is a line within a section, not a heading of its own.
            if detail {
                div { class: "disclosure-heading", {toggle} }
            } else {
                h2 { class: "disclosure-heading", {toggle} }
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

/// The life of a modal dialog `id` the browser owns: it opens modally once mounted, with focus on
/// what its content marks `autofocus`, else the first match of `focus`; its exit motion
/// (`dialog-exit`, on `.is-closing`) ends by closing it; and whatever closed it, `on_close` hears
/// it once.
pub(crate) fn use_modal(id: String, focus: &'static str, on_close: EventHandler) {
    use_effect(move || {
        let opened = script::modal(&id, focus);
        spawn(async move {
            let closed = opened.answer().await;
            if matches!(closed, Ok(true)) {
                on_close.call(());
            }
        });
    });
}

/// A modal dialog: it opens over the
/// scrim with focus inside it, its title heading the content with Close beside it. Escape, the
/// scrim and its close button (named by `close_label`) dismiss it; it settles out before the host
/// hears `on_close`. Focus lands on
/// what the content marks `autofocus`, else the title. The host's class sizes it. A sheet is the
/// same dialog along the bottom edge, and one sheet can
/// open over another.
#[component]
pub fn Dialog(
    title: String,
    /// What the dialog belongs to, under the title, when that is not obvious.
    subtitle: Option<String>,
    #[props(default)] sheet: bool,
    close_label: String,
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
    use_modal(id.clone(), "h2", on_close);
    let dragging = id.clone();
    use_effect(move || {
        if sheet {
            script::sheet_drag(&dragging);
        }
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
                        aria_label: "{close_label}",
                        onclick: move |_| dismiss(),
                        Icon { name: IconName::X }
                    }
                }
                {children}
            }
        }
    }
}

/// A citation in prose: its number in a small pill. It opens
/// its source when the host can; otherwise it stays a mark. `label` names the source for
/// assistive technology. A citation whose source can't be shown is muted, with `unavailable`'s
/// words after its number.
#[component]
pub fn CitationChip(
    index: usize,
    label: String,
    onclick: Option<EventHandler<MouseEvent>>,
    #[props(default)] unavailable: String,
) -> Element {
    if !unavailable.is_empty() {
        return rsx! {
            span { class: "source-trigger citation-chip citation-chip-unavailable", aria_label: "{label}",
                "[{index}]"
                span { class: "citation-unavailable-label", "{unavailable}" }
            }
        };
    }
    match onclick {
        Some(open) => rsx! {
            button {
                class: "source-trigger citation-chip",
                r#type: "button",
                "data-source-number": "{index}",
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

/// The one Sources control under a message: a compact pill with its words and how many sources the message has.
/// It opens the popover named by `opens`, which stands beside it, or acts through `onclick`.
#[component]
pub fn SourcesButton(
    label: String,
    count: usize,
    /// The id of the popover it opens and closes, which stands beside it.
    #[props(default)]
    opens: Option<String>,
    onclick: Option<EventHandler<MouseEvent>>,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    let mut class = String::from("source-trigger sources-button");
    let rest = host_attributes(&mut class, attributes);
    rsx! {
        button {
            class: "{class}",
            r#type: "button",
            aria_haspopup: "dialog",
            popovertarget: opens,
            onclick: move |event| {
                if let Some(handler) = &onclick {
                    handler.call(event);
                }
            },
            ..rest,
            Icon { name: IconName::ChevronDown }
            "{label}"
            span { class: "sources-count", "{count}" }
        }
    }
}

/// One row in a list of rows: an optional icon on its tile, a number or a person's avatar, the row's name, an
/// optional detail beneath it, and, when it opens something, a chevron, with the whole row one button.
/// A row with no `onclick` only states its thing: no chevron, no press and no stop for the keyboard.
/// A row that opens something cuts its title after two lines, as opening it shows the whole; a row
/// that opens nothing shows its whole title. Its state, when it has one, leads the detail in bold.
/// Rows group once, a hairline between them, rather than a card around each.
#[component]
pub fn ListRow(
    #[props(default)] icon: Option<IconName>,
    #[props(default)] number: Option<usize>,
    /// The person the row is, by name, drawn as their avatar.
    #[props(default)]
    person: Option<String>,
    title: String,
    detail: Option<String>,
    /// Where the row's thing stands, in the host's words: "Sent".
    #[props(default)]
    state: Option<String>,
    /// What the row opens; without it, the row is not a control.
    onclick: Option<EventHandler<MouseEvent>>,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    let mut class = String::from("list-row");
    let rest = host_attributes(&mut class, attributes);
    let lead = if let Some(icon) = icon {
        rsx! { IconTile { name: icon, tinted: true } }
    } else if let Some(number) = number {
        rsx! { span { class: "list-row-number", "{number}" } }
    } else if let Some(name) = person {
        rsx! { Avatar { name } }
    } else {
        rsx! {}
    };
    let text = row_text(title, detail, state);
    let Some(open) = onclick else {
        return rsx! { div { class: "{class}", ..rest, {lead} {text} } };
    };
    rsx! {
        button {
            class: "navigation-row {class}",
            r#type: "button",
            onclick: move |event| open.call(event),
            ..rest,
            {lead}
            {text}
            span { class: "list-row-chevron", aria_hidden: "true", Icon { name: IconName::ChevronDown } }
        }
    }
}

/// A row's words: its name, and beneath it the detail, led by its state in bold when it has one.
fn row_text(title: String, detail: Option<String>, state: Option<String>) -> Element {
    rsx! {
        span { class: "list-row-text",
            strong { "{title}" }
            if let Some(state) = state {
                small {
                    span { class: "list-row-state", "{state}" }
                    if let Some(detail) = detail {
                        " · {detail}"
                    }
                }
            } else if let Some(detail) = detail {
                small { "{detail}" }
            }
        }
    }
}

/// Where an on/off row stands: the host's last answer, and whether a press waits on the next.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ToggleState {
    /// Off.
    Off,
    /// Off, pressed to turn on, with the host's answer not back yet.
    TurningOn,
    /// On, pressed to turn off, with the host's answer not back yet.
    TurningOff,
    /// On.
    On,
}

/// A row that is on or off, among list rows: its icon on its tile, its name and an optional detail,
/// the whole row one button that says whether it is pressed. Off, it shows the icon as any row does
/// and nothing at its trailing edge, since it opens nothing; on, its tile fills with the brand
/// colour and a check stands where a chevron would. Turning either way, it still says the last
/// answer as pressed or not, and that it is busy; its tile fills at half strength, a spinner stands
/// where the check would, and it still takes a press unless the host makes it `disabled`. The title
/// keeps its width in every state. Where transitions run, each change fades and the spinner waits a
/// moment first, so a quick answer to a row pressed while settled shows no spinner; under reduced
/// motion it still waits, but stands still. Its title is never cut. The words are the host's for each state.
#[component]
pub fn ToggleRow(
    icon: IconName,
    title: String,
    detail: Option<String>,
    state: ToggleState,
    #[props(default)] disabled: bool,
    onclick: EventHandler<MouseEvent>,
) -> Element {
    let (pressed, busy) = match state {
        ToggleState::Off => ("false", false),
        ToggleState::TurningOn => ("false", true),
        ToggleState::TurningOff => ("true", true),
        ToggleState::On => ("true", false),
    };
    rsx! {
        button {
            class: "navigation-row list-row toggle-row",
            r#type: "button",
            aria_pressed: pressed,
            aria_busy: if busy { "true" },
            disabled,
            onclick: move |event| onclick.call(event),
            IconTile { name: icon, tinted: true }
            {row_text(title, detail, None)}
            span { class: "toggle-row-end", aria_hidden: "true",
                Spinner {}
                Icon { name: IconName::Check }
            }
        }
    }
}

/// A group of rows on one white surface, a hairline between them. Its children are `li` rows.
#[component]
pub fn RowGroup(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    let mut class = String::from("row-list row-group");
    let rest = host_attributes(&mut class, attributes);
    rsx! { ul { class: "{class}", ..rest, {children} } }
}

/// A section's title, with the one action that belongs to the whole section beside it: its words,
/// or, with `action_icon`, that icon alone on a full touch target, the words then its spoken name.
#[component]
pub fn SectionHeader(
    title: String,
    #[props(default)] action: Option<String>,
    #[props(default)] on_action: Option<EventHandler<MouseEvent>>,
    /// Draws the action as this icon alone; `action` names it for assistive technology.
    #[props(default)]
    action_icon: Option<IconName>,
) -> Element {
    rsx! {
        div { class: "section-header",
            h2 { class: "section-title", "{title}" }
            if let (Some(action), Some(on_action)) = (action, on_action) {
                if let Some(name) = action_icon {
                    button {
                        class: "section-action is-icon",
                        r#type: "button",
                        aria_label: "{action}",
                        onclick: move |event| on_action.call(event),
                        Icon { name }
                    }
                } else {
                    button { class: "section-action", r#type: "button", onclick: move |event| on_action.call(event), "{action}" }
                }
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

/// Whether an address the host gives may open as a link: a web, mail or phone address, or a place
/// on the same page. Any other scheme, `javascript:` among them, never becomes a link.
pub fn opens(url: &str) -> bool {
    let url = url.trim();
    match url.split_once(':') {
        Some((scheme, _)) if !scheme.contains(['/', '?', '#']) => {
            matches!(
                scheme.to_ascii_lowercase().as_str(),
                "http" | "https" | "mailto" | "tel"
            )
        }
        _ => !url.is_empty(),
    }
}

/// Actions that sit together: one row, one gap, one centre line, wrapping where the width runs out.
/// A host places the row; it never sets the row's own layout.
#[component]
pub fn ActionRow(children: Element) -> Element {
    rsx! { div { class: "action-row", {children} } }
}

/// The app's sections, the current one marked, named as a whole by `label`. With `icons`, one per
/// label, each section shows its icon above its label, as a phone's tab bar does.
#[component]
pub fn TabBar(
    label: String,
    labels: Vec<String>,
    selected: usize,
    on_select: EventHandler<usize>,
    #[props(default)] icons: Vec<IconName>,
) -> Element {
    let with_icons = icons.len() == labels.len();
    rsx! {
        nav { class: if with_icons { "tabs has-icons selection-host" } else { "tabs selection-host" }, aria_label: "{label}",
            for (index, label) in labels.iter().enumerate() {
                button {
                    key: "{label}",
                    r#type: "button",
                    aria_current: if index == selected { "page" } else { "false" },
                    onclick: move |_| on_select.call(index),
                    if with_icons {
                        Icon { name: icons[index] }
                    }
                    "{label}"
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_bar_is_full_only_when_the_work_is_and_claims_nothing_from_no_number() {
        assert_eq!(filled_percent(Some(0.996)), Some(99.0));
        assert_eq!(filled_percent(Some(1.0)), Some(100.0));
        assert_eq!(filled_percent(Some(1.4)), Some(100.0));
        assert_eq!(filled_percent(Some(-0.2)), Some(0.0));
        assert_eq!(filled_percent(Some(f64::NAN)), None);
        assert_eq!(filled_percent(Some(f64::INFINITY)), None);
        assert_eq!(filled_percent(None), None);
    }

    #[test]
    fn only_web_mail_phone_and_same_page_addresses_open() {
        for url in [
            "https://example.org/a",
            "HTTP://example.org",
            "mailto:a@b.org",
            "tel:+15550100",
            "#notes",
            "/records/1",
            "page?x=a:b",
        ] {
            assert!(opens(url), "{url}");
        }
        for url in [
            "javascript:alert(1)",
            " JavaScript:alert(1)",
            "data:text/html,x",
            "vbscript:x",
            "file:///etc/passwd",
            "",
        ] {
            assert!(!opens(url), "{url}");
        }
    }

    #[test]
    fn a_send_uses_the_words_the_person_sees() {
        let draft = "How did I sleep?";
        assert_eq!(submitted_text(Some("Hello"), ""), "Hello");
        assert_eq!(submitted_text(Some("  "), draft), draft);
        assert_eq!(submitted_text(None, draft), draft);
    }

    #[test]
    fn an_avatar_shows_the_first_letters_of_the_first_two_words() {
        assert_eq!(initials("maria  de la Cruz"), "MD");
        assert_eq!(initials("Ólafur"), "Ó");
        assert_eq!(initials("  "), "");
    }

    #[test]
    fn a_state_leads_a_rows_detail_and_a_row_without_one_keeps_its_plain_detail() {
        fn app() -> Element {
            rsx! {
                ListRow { title: "Visit note", detail: "Northside Clinic · October 15", onclick: move |_| {} }
                ListRow { title: "November 4", state: "Sent", detail: "2 records, 1 note", onclick: move |_| {} }
                ListRow { title: "November 12", state: "Sent", onclick: move |_| {} }
            }
        }
        let mut dom = VirtualDom::new(app);
        dom.rebuild_in_place();
        let html = dioxus_ssr::render(&dom);
        assert!(
            html.contains("<small>Northside Clinic · October 15</small>"),
            "{html}"
        );
        assert!(
            html.contains(
                "<small><span class=\"list-row-state\">Sent</span> · 2 records, 1 note</small>"
            ),
            "{html}"
        );
        assert!(
            html.contains("<small><span class=\"list-row-state\">Sent</span></small>"),
            "{html}"
        );
    }

    #[test]
    fn a_toggle_row_says_its_last_answer_and_that_it_waits_while_turning() {
        fn off() -> Element {
            rsx! { ToggleRow { icon: IconName::Flag, title: "Mark", state: ToggleState::Off, onclick: move |_| {} } }
        }
        fn turning_on() -> Element {
            rsx! { ToggleRow { icon: IconName::Flag, title: "Mark", state: ToggleState::TurningOn, onclick: move |_| {} } }
        }
        fn turning_off() -> Element {
            rsx! { ToggleRow { icon: IconName::Flag, title: "Marked", state: ToggleState::TurningOff, onclick: move |_| {} } }
        }
        fn on() -> Element {
            rsx! { ToggleRow { icon: IconName::Flag, title: "Marked", state: ToggleState::On, onclick: move |_| {} } }
        }
        let render = |app: fn() -> Element| {
            let mut dom = VirtualDom::new(app);
            dom.rebuild_in_place();
            dioxus_ssr::render(&dom)
        };
        for (app, pressed, busy) in [
            (off as fn() -> Element, "false", false),
            (turning_on, "false", true),
            (turning_off, "true", true),
            (on, "true", false),
        ] {
            let html = render(app);
            assert!(
                html.contains(&format!("aria-pressed=\"{pressed}\"")),
                "{html}"
            );
            assert!(!html.contains("mixed"), "{html}");
            assert_eq!(html.contains("aria-busy=\"true\""), busy, "{html}");
            assert!(
                !html.contains("disabled"),
                "turning leaves the press to the host: {html}"
            );
            assert!(!html.contains("list-row-chevron"), "{html}");
        }
    }

    #[test]
    fn a_row_that_opens_nothing_is_no_control_and_has_no_chevron() {
        fn app() -> Element {
            rsx! {
                ListRow { title: "Visit notes", detail: "3 from this clinic", onclick: move |_| {} }
                ListRow { title: "Dr. Maya Okafor", detail: "Your physician" }
            }
        }
        let mut dom = VirtualDom::new(app);
        dom.rebuild_in_place();
        let html = dioxus_ssr::render(&dom);
        let end = html.find("</button>").expect("the row that opens") + "</button>".len();
        let (opens, states) = html.split_at(end);
        assert!(
            opens.starts_with("<button") && opens.contains("list-row-chevron"),
            "{opens}"
        );
        assert!(states.starts_with("<div class=\"list-row\">"), "{states}");
        assert!(!states.contains("list-row-chevron"), "{states}");
    }

    #[test]
    fn a_sections_icon_action_is_named_by_its_words() {
        fn app() -> Element {
            rsx! {
                SectionHeader { title: "Records", action: "See all", on_action: move |_| {} }
                SectionHeader { title: "Notes", action: "Write a note", action_icon: IconName::Plus, on_action: move |_| {} }
            }
        }
        let mut dom = VirtualDom::new(app);
        dom.rebuild_in_place();
        let html = dioxus_ssr::render(&dom);
        assert!(html.contains(">See all</button>"), "{html}");
        assert!(html.contains("aria-label=\"Write a note\""), "{html}");
        assert!(!html.contains(">Write a note<"), "{html}");
    }

    #[test]
    fn a_way_on_without_its_words_is_not_drawn() {
        fn app() -> Element {
            rsx! {
                EmptyState { icon: IconName::File, title: "Gone", on_action: move |_| {}, on_secondary: move |_| {} }
                StatusLine { message: "Couldn’t refresh.", on_retry: move |_| {} }
                EmptyState { icon: IconName::File, title: "Gone", action: "Try again", on_action: move |_| {} }
            }
        }
        let mut dom = VirtualDom::new(app);
        dom.rebuild_in_place();
        let html = dioxus_ssr::render(&dom);
        assert_eq!(html.matches("<button").count(), 1, "{html}");
        assert!(html.contains("Try again"));
    }
}
