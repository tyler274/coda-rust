//! Code as written, to read and take away: a block of code with what it is and a way to copy it,
//! and a change to code, line by line, what it adds and what it removes. Both stand in one frame,
//! in the monospace the prose's code uses.

use std::time::Duration;

use dioxus::prelude::*;

use crate::{Button, ButtonVariant, Icon, IconName};

/// How long a copy says it is done before it offers to copy again.
const COPIED_MS: u64 = 1600;

/// What the copy control is called, and what it says once the code is on the clipboard.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct CopyWords {
    /// The control's name.
    pub copy: String,
    /// Said once the clipboard holds the code.
    pub copied: String,
}

/// Code as written, kept to its lines and scrolled across rather than wrapped, under a `label` that
/// says what it is, such as its language or file. With `copy`, a control puts the code on the
/// clipboard and says so only once the clipboard has taken it.
#[component]
pub fn CodeBlock(code: String, label: Option<String>, copy: Option<CopyWords>) -> Element {
    let mut copied = use_signal(|| false);
    let heading = label.is_some() || copy.is_some();
    rsx! {
        div { class: "code-block",
            if heading {
                div { class: "code-block-heading",
                    if let Some(label) = label {
                        span { class: "code-block-label", "{label}" }
                    }
                    if let Some(words) = copy {
                        Button {
                            variant: ButtonVariant::Quiet,
                            compact: true,
                            icon_only: true,
                            aria_label: if copied() { words.copied.clone() } else { words.copy.clone() },
                            onclick: {
                                let code = code.clone();
                                move |_| {
                                    let code = code.clone();
                                    spawn(async move {
                                        let write = document::eval(COPY);
                                        if write.send(code).is_err() {
                                            return;
                                        }
                                        if write.join::<bool>().await.unwrap_or(false) {
                                            copied.set(true);
                                            futures_timer::Delay::new(Duration::from_millis(COPIED_MS)).await;
                                            copied.set(false);
                                        }
                                    });
                                }
                            },
                            Icon { name: if copied() { IconName::Check } else { IconName::Copy } }
                        }
                        span { class: "sr-only", role: "status",
                            if copied() {
                                "{words.copied}"
                            }
                        }
                    }
                }
            }
            pre { class: "code-block-body", code { "{code}" } }
        }
    }
}

/// Writes the code it is sent to the clipboard, and answers whether the clipboard took it.
const COPY: &str = r#"
const code = await dioxus.recv();
try { await navigator.clipboard.writeText(code); return true; } catch { return false; }
"#;

/// What a line of a change does.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Change {
    /// Stays as it was, around what changes.
    #[default]
    Kept,
    /// Added.
    Added,
    /// Removed.
    Removed,
}

/// One line of a `Diff`.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct DiffLine {
    /// What the line does.
    pub change: Change,
    /// The line as written.
    pub text: String,
}

/// What assistive technology hears before an added or removed line.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct DiffWords {
    /// Before an added line.
    pub added: String,
    /// Before a removed line.
    pub removed: String,
}

/// A change to code, in its lines: an added line on green after a plus, a removed one on red after
/// a minus, and the lines kept around them plain. Its heading names what changed, such as the file,
/// and counts the lines added and removed.
#[component]
pub fn Diff(
    lines: Vec<DiffLine>,
    label: Option<String>,
    #[props(default)] words: DiffWords,
) -> Element {
    let added = lines
        .iter()
        .filter(|line| line.change == Change::Added)
        .count();
    let removed = lines
        .iter()
        .filter(|line| line.change == Change::Removed)
        .count();
    rsx! {
        div { class: "code-block diff",
            div { class: "code-block-heading",
                if let Some(label) = label {
                    span { class: "code-block-label", "{label}" }
                }
                span { class: "diff-counts",
                    if added > 0 {
                        span { class: "diff-added", "+{added}" }
                    }
                    if removed > 0 {
                        span { class: "diff-removed", "−{removed}" }
                    }
                }
            }
            pre { class: "code-block-body",
                code {
                    for (index, line) in lines.into_iter().enumerate() {
                        match line.change {
                            Change::Added => rsx! {
                                ins { key: "{index}", class: "diff-line",
                                    span { class: "diff-sign", aria_hidden: "true", "+" }
                                    if !words.added.is_empty() {
                                        span { class: "sr-only", "{words.added}: " }
                                    }
                                    "{line.text}"
                                }
                            },
                            Change::Removed => rsx! {
                                del { key: "{index}", class: "diff-line",
                                    span { class: "diff-sign", aria_hidden: "true", "−" }
                                    if !words.removed.is_empty() {
                                        span { class: "sr-only", "{words.removed}: " }
                                    }
                                    "{line.text}"
                                }
                            },
                            Change::Kept => rsx! {
                                span { key: "{index}", class: "diff-line",
                                    span { class: "diff-sign", aria_hidden: "true", " " }
                                    "{line.text}"
                                }
                            },
                        }
                    }
                }
            }
        }
    }
}
