//! What a person fills in: words, choices, amounts and dates. Each field stands under its label,
//! with a hint or what is wrong beneath it. Where the platform has a good control of its own, this
//! is that control dressed in the library's look, so a phone still offers its own picker and the
//! keys work as they do everywhere.

use std::sync::atomic::{AtomicUsize, Ordering};

use dioxus::prelude::*;

use crate::{Icon, IconName};

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
    hint: Option<String>,
    error: Option<String>,
    children: Element,
) -> Element {
    rsx! {
        div { class: "text-field",
            label { class: "text-field-label", r#for: "{id}", "{label}" }
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

/// A field with its label above it: one line, or with `multiline` words that grow with what is
/// written. The host owns its value and hears each change. A `hint` stands beneath it; an `error`
/// says what is wrong in its place and outlines the field in red.
#[component]
pub fn TextField(
    label: String,
    value: String,
    on_input: EventHandler<String>,
    #[props(default)] multiline: bool,
    #[props(default)] placeholder: String,
    hint: Option<String>,
    error: Option<String>,
) -> Element {
    let id = use_hook(field_id);
    let note = note_id(&id, &hint, &error);
    let invalid = error.is_some().then_some("true");
    rsx! {
        FieldFrame { id: id.clone(), label, hint, error,
            if multiline {
                textarea {
                    id: "{id}",
                    class: "field text-field-input",
                    rows: "3",
                    placeholder: "{placeholder}",
                    value: "{value}",
                    aria_describedby: note,
                    aria_invalid: invalid,
                    oninput: move |event| on_input.call(event.value()),
                }
            } else {
                input {
                    id: "{id}",
                    class: "field text-field-input",
                    placeholder: "{placeholder}",
                    value: "{value}",
                    aria_describedby: note,
                    aria_invalid: invalid,
                    oninput: move |event| on_input.call(event.value()),
                }
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
        document::eval(&format!(
            "const box = document.getElementById('{box_id}'); if (box) box.indeterminate = {mixed};"
        ));
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

/// One of the choices in a `RadioGroup`.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Choice {
    /// Its words.
    pub label: String,
    /// What choosing it means, more quietly.
    pub detail: Option<String>,
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

/// One of several choices that each need saying in full, as rows under a `label`, none chosen until
/// the person chooses. The arrow keys move through them. For a few short words, `Segmented`.
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

/// One of the choices in a `Segmented`: its short word, what it is spoken as when that word alone
/// would not say it, and an optional icon before it.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Segment {
    /// Its short word.
    pub label: String,
    /// The word it is spoken as, when the short one wouldn't do.
    pub spoken: Option<String>,
    /// Before its word.
    pub icon: Option<IconName>,
}

impl From<&str> for Segment {
    fn from(label: &str) -> Self {
        Segment {
            label: label.to_owned(),
            ..Segment::default()
        }
    }
}

impl From<(&str, &str)> for Segment {
    fn from((label, spoken): (&str, &str)) -> Self {
        Segment {
            label: label.to_owned(),
            spoken: Some(spoken.to_owned()),
            icon: None,
        }
    }
}

impl From<(IconName, &str)> for Segment {
    fn from((icon, label): (IconName, &str)) -> Self {
        Segment {
            label: label.to_owned(),
            spoken: None,
            icon: Some(icon),
        }
    }
}

/// One of a few short choices side by side, always one chosen, as a row of equal pills: the chosen
/// one in the accent's words on its soft fill, which slides to the next one chosen. `label` names
/// the choice; the arrow keys move through it.
#[component]
pub fn Segmented(
    label: String,
    options: Vec<Segment>,
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
                        aria_label: option.spoken,
                        checked: index == selected,
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
