//! What a person fills in: words, choices, amounts and dates. Each field stands under its label,
//! with a hint or what is wrong beneath it. Where the platform has a good control of its own, this
//! is that control dressed in the library's look, so a phone still offers its own picker and the
//! keys work as they do everywhere.

use std::sync::atomic::{AtomicUsize, Ordering};

use dioxus::prelude::*;

use crate::overlays::OptionRow;
use crate::{Icon, IconName, Spinner, script};

static FIELDS: AtomicUsize = AtomicUsize::new(0);

/// A field's id, unique on the page.
fn field_id() -> String {
    format!("field-{}", FIELDS.fetch_add(1, Ordering::Relaxed))
}

/// The id of the words under a field, when it has any.
fn note_id(id: &str, hint: &Option<String>, error: &Option<String>) -> Option<String> {
    (hint.is_some() || error.is_some()).then(|| format!("{id}-note"))
}

/// A control under its label, with beneath it what is wrong, announced, or else a hint.
#[component]
fn FieldFrame(
    id: String,
    label: String,
    reading: Option<String>,
    hint: Option<String>,
    error: Option<String>,
    children: Element,
) -> Element {
    rsx! {
        div { class: "text-field",
            div { class: "field-heading",
                label { class: "text-field-label", r#for: "{id}", "{label}" }
                if let Some(reading) = reading {
                    span { class: "field-reading", aria_hidden: "true", "{reading}" }
                }
            }
            {children}
            if let Some(error) = error {
                p { id: "{id}-note", class: "field-note is-error", role: "alert",
                    Icon { name: IconName::Alert }
                    "{error}"
                }
            } else if let Some(hint) = hint {
                p { id: "{id}-note", class: "field-note", "{hint}" }
            }
        }
    }
}

/// What a `TextField` takes, which decides its control and what the platform offers to fill it.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum TextKind {
    /// One line of words.
    #[default]
    Line,
    /// Words that grow with what is written.
    Multiline,
    /// One address: a phone offers its email keys, and the browser the person's own addresses.
    Email,
}

/// A field with its label above it, taking what its `kind` says. The host owns its value and hears
/// each change and, with `onkeydown`, each key. A `hint` stands beneath it; an `error` says what is
/// wrong in its place and outlines the field in red.
#[component]
pub fn TextField(
    label: String,
    value: String,
    on_input: EventHandler<String>,
    #[props(default)] kind: TextKind,
    #[props(default)] placeholder: String,
    hint: Option<String>,
    error: Option<String>,
    onkeydown: Option<EventHandler<KeyboardEvent>>,
) -> Element {
    let id = use_hook(field_id);
    let note = note_id(&id, &hint, &error);
    let invalid = error.is_some().then_some("true");
    let email = kind == TextKind::Email;
    let key = move |event: KeyboardEvent| {
        if let Some(handler) = onkeydown {
            handler.call(event);
        }
    };
    rsx! {
        FieldFrame { id: id.clone(), label, hint, error,
            if kind == TextKind::Multiline {
                textarea {
                    id: "{id}",
                    class: "field text-field-input",
                    rows: "3",
                    placeholder: "{placeholder}",
                    value: "{value}",
                    aria_describedby: note,
                    aria_invalid: invalid,
                    oninput: move |event| on_input.call(event.value()),
                    onkeydown: key,
                }
            } else {
                input {
                    id: "{id}",
                    class: "field text-field-input",
                    r#type: email.then_some("email"),
                    autocomplete: email.then_some("email"),
                    placeholder: "{placeholder}",
                    value: "{value}",
                    aria_describedby: note,
                    aria_invalid: invalid,
                    oninput: move |event| on_input.call(event.value()),
                    onkeydown: key,
                }
            }
        }
    }
}

/// One line set among other controls: a pill at their height, whose `label` is what the empty
/// field shows and what assistive technology calls it, so the two can't differ. It has no hint or
/// error, having nowhere to say them. `class` places it in its host.
#[component]
pub fn CompactField(
    label: String,
    value: String,
    on_input: EventHandler<String>,
    #[props(default)] disabled: bool,
    #[props(default)] class: String,
    onkeydown: Option<EventHandler<KeyboardEvent>>,
    onfocus: Option<EventHandler<FocusEvent>>,
    onblur: Option<EventHandler<FocusEvent>>,
) -> Element {
    let id = use_hook(field_id);
    rsx! {
        div { class: "compact-field {class}",
            label { class: "sr-only", r#for: "{id}", "{label}" }
            input {
                id: "{id}",
                class: "field compact-field-input",
                placeholder: "{label}",
                value: "{value}",
                disabled,
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
        }
    }
}

/// One choice that is on or off: a checkbox, its words and a quieter detail on what choosing it
/// includes, the whole row pressing it. `mixed` says some of what it stands for is chosen, as for a
/// box over a list; pressing it asks for all.
#[component]
pub fn Checkbox(
    label: String,
    detail: Option<String>,
    checked: bool,
    #[props(default)] mixed: bool,
    #[props(default)] disabled: bool,
    on_change: EventHandler<bool>,
) -> Element {
    let id = use_hook(field_id);
    // Only a property holds this state; the markup has no attribute for it.
    let box_id = id.clone();
    use_effect(use_reactive!(|mixed| {
        script::indeterminate(&box_id, mixed);
    }));
    rsx! {
        label { class: "selection-row",
            input {
                id: "{id}",
                r#type: "checkbox",
                checked,
                disabled,
                onchange: move |event| on_change.call(event.checked()),
            }
            span { class: "list-row-text",
                strong { "{label}" }
                if let Some(detail) = detail {
                    small { "{detail}" }
                }
            }
        }
    }
}

/// One of a set of choices, for a `RadioGroup`, `Segmented`, `Select` or `Combobox`: its words, what
/// it means, more quietly, an icon before it, and whether it is shown but not to be chosen. A
/// `Combobox` shows all of it; the others say what they have no room for.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Choice {
    /// Its words.
    pub label: String,
    /// What choosing it means, more quietly.
    pub detail: Option<String>,
    /// Before its words.
    pub icon: Option<IconName>,
    /// Shown, but not to be chosen.
    pub disabled: bool,
}

impl From<&str> for Choice {
    fn from(label: &str) -> Self {
        Choice {
            label: label.to_owned(),
            ..Choice::default()
        }
    }
}

impl From<(&str, &str)> for Choice {
    fn from((label, detail): (&str, &str)) -> Self {
        Choice {
            detail: Some(detail.to_owned()),
            ..label.into()
        }
    }
}

impl From<(IconName, &str)> for Choice {
    fn from((icon, label): (IconName, &str)) -> Self {
        Choice {
            icon: Some(icon),
            ..label.into()
        }
    }
}

/// One of several choices that each need saying in full, as rows under a `label`, none chosen until
/// the person chooses. The arrow keys move through them. Rows carry no icon. For a few short words,
/// `Segmented`.
#[component]
pub fn RadioGroup(
    label: String,
    options: Vec<Choice>,
    selected: Option<usize>,
    on_select: EventHandler<usize>,
) -> Element {
    let id = use_hook(field_id);
    rsx! {
        fieldset { class: "radio-group",
            legend { class: "text-field-label", "{label}" }
            for (index, option) in options.into_iter().enumerate() {
                label { key: "{index}", class: "selection-row",
                    input {
                        r#type: "radio",
                        name: "{id}",
                        checked: selected == Some(index),
                        disabled: option.disabled,
                        onchange: move |_| on_select.call(index),
                    }
                    span { class: "list-row-text",
                        strong { "{option.label}" }
                        if let Some(detail) = option.detail {
                            small { "{detail}" }
                        }
                    }
                }
            }
        }
    }
}

/// One of a few short choices side by side, always one chosen, as a row of equal pills: the chosen
/// one in the accent's words on its soft fill, which slides to the next one chosen. `label` names
/// the choice; the arrow keys move through it. A pill has no room for a choice's `detail`, so that
/// is what it is spoken as, where its short word alone wouldn't say it.
#[component]
pub fn Segmented(
    label: String,
    options: Vec<Choice>,
    selected: usize,
    on_select: EventHandler<usize>,
) -> Element {
    let id = use_hook(field_id);
    rsx! {
        div { class: "segmented selection-host", role: "radiogroup", aria_label: "{label}",
            for (index, option) in options.into_iter().enumerate() {
                label {
                    key: "{index}",
                    class: "segmented-option",
                    aria_current: (index == selected).then_some("true"),
                    input {
                        class: "sr-only",
                        r#type: "radio",
                        name: "{id}",
                        aria_label: option.detail,
                        checked: index == selected,
                        disabled: option.disabled,
                        onchange: move |_| on_select.call(index),
                    }
                    if let Some(name) = option.icon {
                        Icon { name }
                    }
                    "{option.label}"
                }
            }
        }
    }
}

/// One of a list of choices, from the platform's own picker, so a phone offers its wheel or sheet.
/// With none `selected` yet, `placeholder` stands in the field. The picker shows each choice's
/// words alone.
#[component]
pub fn Select(
    label: String,
    options: Vec<Choice>,
    selected: Option<usize>,
    on_select: EventHandler<usize>,
    #[props(default)] placeholder: String,
    #[props(default)] disabled: bool,
    hint: Option<String>,
    error: Option<String>,
) -> Element {
    let id = use_hook(field_id);
    let note = note_id(&id, &hint, &error);
    rsx! {
        FieldFrame { id: id.clone(), label, hint, error: error.clone(),
            div { class: "select",
                select {
                    id: "{id}",
                    class: if selected.is_none() { "field text-field-input is-placeholder" } else { "field text-field-input" },
                    disabled,
                    aria_describedby: note,
                    aria_invalid: error.is_some().then_some("true"),
                    onchange: move |event| {
                        if let Ok(index) = event.value().parse() {
                            on_select.call(index);
                        }
                    },
                    if selected.is_none() {
                        option { value: "", disabled: true, hidden: true, selected: true, "{placeholder}" }
                    }
                    for (index, choice) in options.into_iter().enumerate() {
                        option {
                            key: "{index}",
                            value: "{index}",
                            selected: selected == Some(index),
                            disabled: choice.disabled,
                            "{choice.label}"
                        }
                    }
                }
                Icon { name: IconName::ChevronDown }
            }
        }
    }
}

/// The choice to move to from `from`, forward or back, past any that are disabled, wrapping round.
fn step_choice(options: &[Choice], from: Option<usize>, forward: bool) -> Option<usize> {
    let count = options.len();
    (1..=count)
        .map(|distance| match (from, forward) {
            (Some(from), true) => (from + distance) % count,
            (Some(from), false) => (from + count - distance % count) % count,
            (None, true) => distance - 1,
            (None, false) => count - distance,
        })
        .find(|&index| !options[index].disabled)
}

/// One choice, found by typing, for a list too long to read: the field holds the host's `query`,
/// and the host narrows `options` to it, from memory or by asking, `loading` while it does. The
/// arrow keys or the pointer move through what is listed; Enter or a press chooses, once any
/// search is done, and Escape or leaving the field closes the list. `selected` is marked; `empty`
/// is said when nothing is found.
/// After a choice the host sets the `query` to what it wants the field to say.
#[component]
pub fn Combobox(
    label: String,
    query: String,
    options: Vec<Choice>,
    on_query: EventHandler<String>,
    on_choose: EventHandler<usize>,
    empty: String,
    selected: Option<usize>,
    #[props(default)] placeholder: String,
    #[props(default)] loading: bool,
    hint: Option<String>,
    error: Option<String>,
) -> Element {
    let id = use_hook(field_id);
    let note = note_id(&id, &hint, &error);
    let mut open = use_signal(|| false);
    let mut active = use_signal(|| None::<usize>);
    let popup = format!("{id}-popup");
    let list = format!("{id}-list");
    let shown = popup.clone();
    use_effect(move || {
        let show = open();
        script::toggle_popover(&shown, show);
    });
    // The active choice stands for a place in this list; a new list starts again from its first.
    use_effect(use_reactive!(|options| {
        let _ = options;
        active.set(None);
    }));
    let current = active()
        .filter(|&index| options.get(index).is_some_and(|option| !option.disabled))
        .or_else(|| step_choice(&options, None, true));
    let keyed = options.clone();
    let moving = id.clone();
    let mut choose = move |index: usize| {
        // Mid-search the list is the last search's, not what was asked for.
        if !loading {
            on_choose.call(index);
            open.set(false);
        }
    };
    rsx! {
        FieldFrame { id: id.clone(), label: label.clone(), hint, error: error.clone(),
            div { class: "search-field field combobox", style: "anchor-name: --{id}",
                Icon { name: IconName::Search }
                input {
                    id: "{id}",
                    r#type: "text",
                    role: "combobox",
                    aria_expanded: "{open}",
                    aria_controls: "{list}",
                    aria_autocomplete: "list",
                    aria_activedescendant: current.filter(|_| open()).map(|index| format!("{id}-{index}")),
                    aria_describedby: note,
                    aria_invalid: error.is_some().then_some("true"),
                    aria_busy: loading.then_some("true"),
                    autocomplete: "off",
                    spellcheck: "false",
                    placeholder: "{placeholder}",
                    value: "{query}",
                    oninput: move |event| {
                        on_query.call(event.value());
                        active.set(None);
                        open.set(true);
                    },
                    onclick: move |_| open.set(true),
                    onblur: move |_| open.set(false),
                    onkeydown: move |event| {
                        // Keys that confirm composed text are the text's, not the list's.
                        if event.is_composing() {
                            return;
                        }
                        match event.key() {
                            Key::ArrowDown | Key::ArrowUp => {
                                event.prevent_default();
                                if !open() {
                                    open.set(true);
                                    return;
                                }
                                let next = step_choice(&keyed, current, event.key() == Key::ArrowDown);
                                active.set(next);
                                if let Some(next) = next {
                                    script::scroll_into_view(&format!("{moving}-{next}"));
                                }
                            }
                            Key::Enter if open() => {
                                event.prevent_default();
                                if let Some(index) = current {
                                    choose(index);
                                }
                            }
                            Key::Escape if open() => {
                                event.prevent_default();
                                event.stop_propagation();
                                open.set(false);
                            }
                            _ => {}
                        }
                    },
                }
                if loading {
                    Spinner {}
                }
            }
            div {
                id: "{popup}",
                class: "popover combobox-popup",
                "popover": "manual",
                style: "position-anchor: --{id}",
                // A press in the list leaves focus in the field, so the press still lands.
                onmousedown: move |event| event.prevent_default(),
                div { id: "{list}", role: "listbox", aria_label: "{label}",
                    for (index, option) in options.iter().enumerate() {
                        OptionRow {
                            key: "{index}",
                            id: "{id}-{index}",
                            label: option.label.clone(),
                            detail: option.detail.clone(),
                            icon: option.icon,
                            active: current == Some(index),
                            disabled: option.disabled,
                            on_point: move |_| active.set(Some(index)),
                            on_choose: move |_| choose(index),
                            if selected == Some(index) {
                                span { class: "option-chosen", Icon { name: IconName::Check } }
                            }
                        }
                    }
                }
                if options.is_empty() && !loading {
                    p { class: "combobox-empty", role: "status", "{empty}" }
                }
            }
        }
    }
}

/// An amount along a track from `min` to `max`, moved by `step`, by finger, pointer or arrow key.
/// `reading` says it in words, such as "4 of 10", beside the label and to assistive technology.
#[component]
pub fn Slider(
    label: String,
    value: f64,
    min: f64,
    max: f64,
    #[props(default = 1.0)] step: f64,
    reading: String,
    on_change: EventHandler<f64>,
    #[props(default)] disabled: bool,
    hint: Option<String>,
) -> Element {
    let id = use_hook(field_id);
    let note = note_id(&id, &hint, &None);
    let filled = if max > min {
        ((value - min) / (max - min) * 100.0).clamp(0.0, 100.0)
    } else {
        0.0
    };
    rsx! {
        FieldFrame { id: id.clone(), label, reading: reading.clone(), hint,
            input {
                id: "{id}",
                class: "slider",
                r#type: "range",
                // The range stands before the value, or the browser clamps the value to its own.
                min: "{min}",
                max: "{max}",
                step: "{step}",
                value: "{value}",
                disabled,
                style: "--filled: {filled}%",
                aria_valuetext: "{reading}",
                aria_describedby: note,
                oninput: move |event| {
                    if let Ok(value) = event.value().parse() {
                        on_change.call(value);
                    }
                },
            }
        }
    }
}

/// What a `DateField` asks for.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum DateKind {
    /// A day.
    #[default]
    Date,
    /// A time of day.
    Time,
    /// A day and a time.
    DateTime,
}

/// A date, a time or both, from the platform's own picker, in the person's own format on screen.
/// `value` is the platform's plain form, such as `2026-08-27` or `14:30`, and empty until it is
/// whole; `min` and `max`, in the same form, bound what the picker offers. What is wrong with a
/// date is the host's to say, in `error`.
#[component]
pub fn DateField(
    label: String,
    value: String,
    on_change: EventHandler<String>,
    #[props(default)] kind: DateKind,
    min: Option<String>,
    max: Option<String>,
    #[props(default)] disabled: bool,
    hint: Option<String>,
    error: Option<String>,
) -> Element {
    let id = use_hook(field_id);
    let note = note_id(&id, &hint, &error);
    rsx! {
        FieldFrame { id: id.clone(), label, hint, error: error.clone(),
            input {
                id: "{id}",
                class: "field text-field-input date-field",
                r#type: match kind {
                    DateKind::Date => "date",
                    DateKind::Time => "time",
                    DateKind::DateTime => "datetime-local",
                },
                min,
                max,
                value: "{value}",
                disabled,
                aria_describedby: note,
                aria_invalid: error.is_some().then_some("true"),
                oninput: move |event| on_change.call(event.value()),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn options() -> Vec<Choice> {
        vec![
            "a".into(),
            Choice {
                label: "b".into(),
                disabled: true,
                ..Choice::default()
            },
            "c".into(),
        ]
    }

    #[test]
    fn stepping_passes_disabled_choices_and_wraps() {
        let options = options();
        assert_eq!(step_choice(&options, None, true), Some(0));
        assert_eq!(step_choice(&options, None, false), Some(2));
        assert_eq!(step_choice(&options, Some(0), true), Some(2));
        assert_eq!(step_choice(&options, Some(2), true), Some(0));
        assert_eq!(step_choice(&options, Some(0), false), Some(2));
        assert_eq!(step_choice(&[], None, true), None);
    }
}
