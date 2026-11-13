//! Code as written, to read and take away: a block of code with what it is and a way to copy it,
//! in the monospace the prose's code uses. The copy control copies any words as written, such as a
//! message's.

use std::time::Duration;

use dioxus::prelude::*;

use crate::{Button, ButtonVariant, Icon, IconName, script};

/// How long a copy says it is done before it offers to copy again.
const COPIED_MS: u64 = 1600;

/// What the copy control is called, and what it says once the words are on the clipboard.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct CopyWords {
    /// The control's name.
    pub copy: String,
    /// Said once the clipboard holds the words.
    pub copied: String,
}

/// A quiet icon control that puts `text` on the clipboard, and says so, its copy mark turned to a
/// check, only once the clipboard has taken it.
#[component]
pub fn CopyButton(text: String, words: CopyWords) -> Element {
    let mut copied = use_signal(|| false);
    rsx! {
        Button {
            variant: ButtonVariant::Quiet,
            compact: true,
            icon_only: true,
            aria_label: if copied() { words.copied.clone() } else { words.copy.clone() },
            onclick: move |_| {
                let text = text.clone();
                // A copy the clipboard refused leaves the copy mark as it is.
                spawn(async move {
                    let copy = copy_text(text).await;
                    if copy.is_ok() {
                        copied.set(true);
                        futures_timer::Delay::new(Duration::from_millis(COPIED_MS)).await;
                        copied.set(false);
                    }
                });
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

/// Why the clipboard didn't take the words, in the browser's own words where it gave any.
#[derive(Debug, thiserror::Error)]
#[error("the clipboard didn't take the words: {0}")]
pub struct CopyFailed(String);

/// Puts `text` on the clipboard, or says why it didn't take, for a host's own copy action to say
/// it's copied only once it is.
pub async fn copy_text(text: String) -> Result<(), CopyFailed> {
    script::copy(&text)
        .answer()
        .await
        .map_err(|why| CopyFailed(why.to_string()))
}

/// Code as written, kept to its lines and scrolled across rather than wrapped, a stop for the keys
/// so they can scroll it too, under a `label` that says what it is, such as its language or file.
/// `name` is what assistive technology calls it, with the `label` after. With `copy`, a
/// `CopyButton` copies it.
#[component]
pub fn CodeBlock(
    code: String,
    name: String,
    label: Option<String>,
    copy: Option<CopyWords>,
) -> Element {
    let heading = label.is_some() || copy.is_some();
    let name = match &label {
        Some(label) => format!("{name}: {label}"),
        None => name,
    };
    rsx! {
        div { class: "code-block",
            if heading {
                div { class: "code-block-heading",
                    if let Some(label) = label {
                        span { class: "code-block-label", "{label}" }
                    }
                    if let Some(words) = copy {
                        CopyButton { text: code.clone(), words }
                    }
                }
            }
            pre { class: "code-block-body", role: "region", aria_label: "{name}", tabindex: "0",
                code { "{code}" }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn html(app: fn() -> Element) -> String {
        let mut dom = VirtualDom::new(app);
        dom.rebuild_in_place();
        dioxus_ssr::render(&dom)
    }

    #[test]
    fn a_code_block_is_a_named_stop_for_the_keys() {
        let html = html(|| rsx! { CodeBlock { code: "1 + 1", name: "Code", label: "json" } });
        assert!(html.contains(r#"role="region""#), "{html}");
        assert!(html.contains(r#"aria-label="Code: json""#), "{html}");
        assert!(html.contains(r#"tabindex="0""#), "{html}");
    }

    #[test]
    fn a_code_block_with_no_label_keeps_its_name() {
        let html = html(|| rsx! { CodeBlock { code: "1 + 1", name: "Code" } });
        assert!(html.contains(r#"aria-label="Code""#), "{html}");
    }
}
