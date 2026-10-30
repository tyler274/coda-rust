//! Coda's design, as a catalog of AI-native component families any surface can compose: the
//! tokens (colour, type, space, shape and motion), the icons, and each component's look, motion,
//! gestures, states and accessibility. It holds no product rules. The surface that places a
//! component decides when it shows, what pressing it does, what its server guarantees, every word
//! it says, and how the screen is laid out. `crates/ui-gallery` renders every family.
#![deny(missing_docs)]

use std::sync::atomic::{AtomicUsize, Ordering};

use dioxus::dioxus_core::AttributeValue;
use dioxus::prelude::*;

mod cards;
mod charts;
mod conversation;
mod documents;
mod feedback;
mod icons;
mod live;
mod motion;
mod prose;
mod sources;
mod undo;
pub use cards::{Card, Quote, StepList};
pub use charts::{
    ChartHeader, Comparison, ComparisonBars, MeasurementTile, RangeChoice, RunningChart,
    SeriesChart, SeriesPoint, SeriesShape, Sparkline, Value,
};
pub use conversation::{ContextChip, ConversationViewport, DateSeparator, Suggestion};
pub use documents::{
    FooterActions, Notice, Paper, PaperSection, ProposedChange, SelectionRow, SettingRow, TopBar,
};
pub use feedback::{FeedbackRating, Rating, ToggleChip};
pub use icons::{Icon, IconName};
pub use live::{ListedRow, LiveRows, RowFrame, use_live_rows};
pub use motion::Stylesheet;
pub use prose::{Prose, ProsePart};
pub use sources::{CitationPreview, SourceDetails, SourcePane, SourceReading, SourceText};
pub use undo::{UNDO_MS, UndoNotice};

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
    onclick: Option<EventHandler<MouseEvent>>,
    /// The press, before it takes focus: a control that leaves focus where it is prevents it here.
    onmousedown: Option<EventHandler<MouseEvent>>,
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

/// One message in a thread:
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

/// A field with its label above it: one line, or with
/// `multiline` words that grow with what is written. The host owns its value and hears each
/// change.
#[component]
pub fn TextField(
    label: String,
    value: String,
    on_input: EventHandler<String>,
    #[props(default)] multiline: bool,
    #[props(default)] placeholder: String,
) -> Element {
    rsx! {
        label { class: "text-field",
            span { class: "text-field-label", "{label}" }
            if multiline {
                textarea {
                    class: "field text-field-input",
                    rows: "3",
                    placeholder: "{placeholder}",
                    value: "{value}",
                    oninput: move |event| on_input.call(event.value()),
                }
            } else {
                input {
                    class: "field text-field-input",
                    placeholder: "{placeholder}",
                    value: "{value}",
                    oninput: move |event| on_input.call(event.value()),
                }
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
/// The words grow with what is written, up to seven lines. Keys stay the host's.
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
    children: Element,
) -> Element {
    let held = value.clone();
    rsx! {
        form {
            class: if inline { "composer field is-inline" } else { "composer field" },
            onsubmit: move |event| {
                event.prevent_default();
                let typed = event.get_first("draft").and_then(|value| match value {
                    FormValue::Text(text) => Some(text),
                    FormValue::File(_) => None,
                });
                on_submit.call(submitted_text(typed.as_deref(), &held));
            },
            {context}
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
            div { class: "composer-toolbar",
                if let Some(status) = status {
                    div { class: "composer-context", {status} }
                }
                {children}
            }
            {notices}
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

/// A sheet follows the finger or pointer down from its handle, its header, or its content once
/// that is scrolled to the top. Pulled up, it resists, rubber-banding as iOS does. Let go, it is
/// projected where the flick would carry it, decelerating as a native scroll view does: past half its height it goes, otherwise it springs back. Either way
/// the release carries the flick's speed into the step spring (`--spring-step`: a smooth spring,
/// response 0.35, critically damped). Without motion it lands at once. A drag that began on
/// a control doesn't also press it. `{dialog}` is the sheet's id.
const SHEET_DRAG: &str = "
const sheet = document.getElementById('{dialog}');
const body = sheet && sheet.querySelector('.dialog-body');
if (sheet && body) {
  const stiffness = (2 * Math.PI / 0.35) ** 2, damping = 4 * Math.PI / 0.35;
  // Where a flick at this speed (px/s) would come to rest, at UIScrollView's normal deceleration.
  const projection = 0.998 / (1 - 0.998) / 1000;
  const still = () => matchMedia('(prefers-reduced-motion: reduce)').matches;
  let drag = null, settling = 0;
  const show = (y) => {
    const height = sheet.offsetHeight || 1;
    sheet.style.transform = `translateY(${y}px)`;
    sheet.style.setProperty('--sheet-shown', String(Math.min(1, Math.max(0, 1 - y / height))));
  };
  const rest = () => { sheet.style.removeProperty('transform'); sheet.style.removeProperty('--sheet-shown'); delete sheet.dataset.dragging; };
  const band = (y, height) => y >= 0 ? y : -(1 - 1 / (-y * 0.55 / height + 1)) * height;
  // Where the sheet is drawn now, whether its entry or a spring is moving it.
  const drawn = () => { const t = getComputedStyle(sheet).transform; return t === 'none' ? 0 : new DOMMatrix(t).m42; };
  // Where the hand would be for a sheet already shown at `y`, so taking hold of it never moves it.
  const unband = (y, height) => y >= 0 ? y : -height * (1 / (1 + y / height) - 1) / 0.55;
  const settle = (from, to, speed, done) => {
    cancelAnimationFrame(settling);
    if (still()) { show(to); done(); return; }
    let x = from, v = speed, last = performance.now();
    const step = (now) => {
      let dt = Math.min(0.032, (now - last) / 1000); last = now;
      while (dt > 0) { const h = Math.min(dt, 1 / 240); v += (-stiffness * (x - to) - damping * v) * h; x += v * h; dt -= h; }
      show(x);
      // The spring is critically damped, so it nears its end without passing it.
      const arrived = to > 0 ? x >= to - 0.5 : Math.abs(x - to) < 0.5 && Math.abs(v) < 5;
      if (arrived) { show(to); done(); } else { settling = requestAnimationFrame(step); }
    };
    settling = requestAnimationFrame(step);
  };
  const typing = (node) => node.closest('input, textarea, select, [contenteditable]') !== null;
  sheet.addEventListener('pointerdown', (event) => {
    if (event.button !== 0 || !sheet.open || sheet.classList.contains('is-closing') || typing(event.target)) return;
    const fromContent = body.contains(event.target) && !event.target.closest('.dialog-header');
    drag = { id: event.pointerId, x: event.clientX, y: event.clientY, fromContent, active: false, from: 0, offset: 0, samples: [[event.timeStamp, event.clientY]] };
  });
  sheet.addEventListener('pointermove', (event) => {
    if (!drag || event.pointerId !== drag.id) return;
    const dx = event.clientX - drag.x, dy = event.clientY - drag.y;
    if (!drag.active) {
      if (Math.abs(dy) < 6 || Math.abs(dy) < Math.abs(dx)) return;
      if (drag.fromContent && (body.scrollTop > 0 || dy < 0)) { drag = null; return; }
      drag.active = true;
      // Taken mid-motion, it is held where it is; the movement that started the drag still counts.
      cancelAnimationFrame(settling);
      drag.from = unband(drawn(), sheet.offsetHeight || 1);
      sheet.dataset.dragging = '';
      sheet.setPointerCapture(event.pointerId);
    }
    const height = sheet.offsetHeight || 1;
    drag.offset = band(drag.from + event.clientY - drag.y, height);
    drag.samples.push([event.timeStamp, event.clientY]);
    while (drag.samples.length > 2 && event.timeStamp - drag.samples[0][0] > 100) drag.samples.shift();
    show(drag.offset);
  });
  // A drag is the sheet's, not the page's: on touch the browser would otherwise scroll it away.
  sheet.addEventListener('touchmove', (event) => { if (drag && drag.active) event.preventDefault(); }, { passive: false });
  const release = (event) => {
    if (!drag || event.pointerId !== drag.id) return;
    const ended = drag; drag = null;
    if (!ended.active) return;
    // The click this release would make belongs to the drag; it arrives before any timer runs.
    const swallow = (click) => { click.preventDefault(); click.stopPropagation(); };
    sheet.addEventListener('click', swallow, { capture: true, once: true });
    setTimeout(() => sheet.removeEventListener('click', swallow, { capture: true }), 0);
    // The speed at release, over its last 100ms: a hand held still before letting go has none.
    const recent = [...ended.samples, [event.timeStamp, event.clientY]].filter(([t]) => event.timeStamp - t <= 100);
    const [first, last] = [recent[0], recent[recent.length - 1]];
    const speed = recent.length > 1 && last[0] > first[0] ? (last[1] - first[1]) / ((last[0] - first[0]) / 1000) : 0;
    const height = sheet.offsetHeight || 1;
    const goes = event.type === 'pointerup' && ended.offset > 0 && ended.offset + speed * projection > height / 2;
    if (goes) {
      settle(ended.offset, height, Math.max(speed, 0), () => sheet.close());
    } else {
      settle(ended.offset, 0, ended.offset < 0 ? 0 : speed, rest);
    }
  };
  sheet.addEventListener('pointerup', release);
  sheet.addEventListener('pointercancel', release);
}
";

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
        if sheet {
            document::eval(&SHEET_DRAG.replace("{dialog}", &opening));
        }
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
#[component]
pub fn SourcesButton(
    label: String,
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
            "{label}"
            span { class: "sources-count", "{count}" }
        }
    }
}

/// One row in a list of rows: an optional icon on its tile, a number or a person's avatar, the row's name, an
/// optional detail beneath it, and a chevron, with the whole row one button. Rows group once, a
/// hairline between them, rather than a card around each.
#[component]
pub fn ListRow(
    #[props(default)] icon: Option<IconName>,
    #[props(default)] number: Option<usize>,
    /// The person the row is, by name, drawn as their avatar.
    #[props(default)]
    person: Option<String>,
    title: String,
    detail: Option<String>,
    onclick: EventHandler<MouseEvent>,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
) -> Element {
    let mut class = String::from("navigation-row list-row");
    let rest = host_attributes(&mut class, attributes);
    rsx! {
        button {
            class: "{class}",
            r#type: "button",
            onclick: move |event| onclick.call(event),
            ..rest,
            if let Some(icon) = icon {
                IconTile { name: icon, tinted: true }
            } else if let Some(number) = number {
                span { class: "list-row-number", "{number}" }
            } else if let Some(name) = person {
                Avatar { name }
            }
            span { class: "list-row-text",
                strong { "{title}" }
                if let Some(detail) = detail {
                    small { "{detail}" }
                }
            }
            span { class: "list-row-chevron", aria_hidden: "true", Icon { name: IconName::ChevronDown } }
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

/// A section's title, with the one action that belongs to the whole section beside it.
#[component]
pub fn SectionHeader(
    title: String,
    #[props(default)] action: Option<String>,
    #[props(default)] on_action: Option<EventHandler<MouseEvent>>,
) -> Element {
    rsx! {
        div { class: "section-header",
            h2 { class: "section-title", "{title}" }
            if let (Some(action), Some(on_action)) = (action, on_action) {
                button { class: "section-action", r#type: "button", onclick: move |event| on_action.call(event), "{action}" }
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
pub(crate) fn opens(url: &str) -> bool {
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
