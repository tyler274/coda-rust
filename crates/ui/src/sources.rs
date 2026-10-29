//! Reading a cited source, after the React app's citation preview and source reading layout: a
//! citation inspects its source in place on hover, and opens the same content beside the reader
//! when asked. The reader is never replaced; on a narrow page the source covers it, with Back.

use std::rc::Rc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Duration;

use dioxus::prelude::*;

use crate::{Button, ButtonVariant, Icon, IconName};

static PREVIEWS: AtomicUsize = AtomicUsize::new(0);

/// How long a preview waits after the pointer and focus leave, so moving from citation to preview
/// keeps it open.
const CLOSE_DELAY_MS: u64 = 160;

/// How long the source pane takes to settle away: the control motion.
const EXIT_MS: u64 = 140;

/// Where the reading layout returns focus once its source closes: the citation that opened it.
#[derive(Clone, Copy)]
struct SourceReturn(Signal<Option<Rc<MountedData>>>);

/// A citation in prose that previews its source. Pointing at it opens the preview beside it, which
/// stays while the pointer or focus is on either; Escape or Close dismisses it. Choosing it opens
/// the source beside the reader when the host gives `on_expand`, and otherwise toggles the
/// preview, as on touch. The preview's actions are Expand and the way to the original record.
#[component]
pub fn CitationPreview(
    index: usize,
    label: String,
    /// Opens the same source in the reading pane.
    #[props(default)]
    on_expand: Option<EventHandler>,
    /// The original record in its own system, opened in a new tab.
    #[props(default)]
    original: Option<String>,
    /// The system that holds the original, when it has a name.
    #[props(default)]
    original_label: Option<String>,
    children: Element,
) -> Element {
    let id = use_hook(|| PREVIEWS.fetch_add(1, Ordering::Relaxed));
    let mut open = use_signal(|| false);
    let mut hovering = use_signal(|| false);
    let mut focused = use_signal(|| false);
    let mut pending = use_signal(|| 0u64);
    let mut trigger = use_signal(|| None::<Rc<MountedData>>);
    let returns = try_use_context::<SourceReturn>();
    let mut close = move |restore: bool| {
        pending += 1;
        open.set(false);
        if restore && let Some(trigger) = trigger.peek().clone() {
            spawn(async move {
                let _ = trigger.set_focus(true).await;
            });
        }
    };
    let mut leave = move || {
        let started = *pending.peek() + 1;
        pending.set(started);
        spawn(async move {
            futures_timer::Delay::new(Duration::from_millis(CLOSE_DELAY_MS)).await;
            if *pending.peek() == started && !*hovering.peek() && !*focused.peek() {
                open.set(false);
            }
        });
    };
    let mut show = move || {
        pending += 1;
        open.set(true);
    };
    // Opening the pane closes the preview and leaves the citation as the place focus returns to.
    let mut expand = move |expand: EventHandler| {
        close(false);
        if let Some(SourceReturn(mut target)) = returns {
            target.set(trigger.peek().clone());
        }
        expand.call(());
    };
    let anchor = format!("--citation-{id}");
    let preview_id = format!("citation-preview-{id}");
    let original_text = match &original_label {
        Some(name) if !name.trim().is_empty() => format!("Open in {}", name.trim()),
        _ => "Open original".to_owned(),
    };
    rsx! {
        span {
            class: "anchored-popover",
            onpointerenter: move |_| hovering.set(true),
            onpointerleave: move |_| {
                hovering.set(false);
                leave();
            },
            onfocusin: move |_| focused.set(true),
            onfocusout: move |_| {
                focused.set(false);
                leave();
            },
            onkeydown: move |event| {
                if event.key() == Key::Escape && open() {
                    event.stop_propagation();
                    event.prevent_default();
                    close(true);
                }
            },
            button {
                class: "source-trigger citation-chip",
                r#type: "button",
                style: "anchor-name: {anchor}",
                aria_label: "Source {index}: {label}. Preview source.",
                aria_expanded: if open() { "true" } else { "false" },
                aria_haspopup: "dialog",
                aria_controls: "{preview_id}",
                onmounted: move |event| trigger.set(Some(event.data())),
                onpointermove: move |event| {
                    if event.data().pointer_type() != "touch" && !open() {
                        show();
                    }
                },
                onclick: move |_| match on_expand {
                    Some(handler) => expand(handler),
                    None if open() => close(false),
                    None => show(),
                },
                "[{index}]"
            }
            if open() {
                span {
                    id: "{preview_id}",
                    class: "citation-preview",
                    role: "dialog",
                    aria_label: "Source {index}: {label}",
                    style: "position-anchor: {anchor}",
                    onpointerenter: move |_| {
                        hovering.set(true);
                        pending += 1;
                    },
                    span { class: "citation-preview-heading",
                        span {
                            small { "Source {index}" }
                            strong { "{label}" }
                        }
                        Button {
                            variant: ButtonVariant::Quiet,
                            icon_only: true,
                            compact: true,
                            aria_label: "Close source preview",
                            onclick: move |_| close(true),
                            Icon { name: IconName::X }
                        }
                    }
                    span { class: "citation-preview-body", {children} }
                    if on_expand.is_some() || original.is_some() {
                        span { class: "citation-preview-actions",
                            if let Some(handler) = on_expand {
                                button {
                                    class: "navigation-control",
                                    r#type: "button",
                                    onclick: move |_| expand(handler),
                                    Icon { name: IconName::Expand }
                                    "Expand"
                                }
                            }
                            if let Some(href) = original {
                                a {
                                    class: "navigation-control",
                                    href: "{href}",
                                    target: "_blank",
                                    rel: "noopener noreferrer",
                                    title: "Opens in a new tab",
                                    onclick: move |_| close(false),
                                    span { "{original_text}" }
                                    Icon { name: IconName::ExternalLink }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

/// What a source says, shared by its preview and its pane: the source's own metadata, the passages
/// quoted from it, then its content. `unavailable` says plainly when content can't be shown here.
#[component]
pub fn SourceDetails(
    #[props(default)] metadata: Vec<String>,
    #[props(default)] passages: Vec<String>,
    #[props(default)] unavailable: Option<String>,
    children: Element,
) -> Element {
    rsx! {
        span { class: "citation-source-details",
            if !metadata.is_empty() {
                span { class: "citation-source-metadata",
                    for (index, value) in metadata.into_iter().enumerate() {
                        span { key: "{index}", "{value}" }
                    }
                }
            }
            if !passages.is_empty() {
                span { class: "citation-selected-passages", aria_label: "Quoted source passages",
                    for (index, quote) in passages.into_iter().enumerate() {
                        blockquote { key: "{index}", "{quote}" }
                    }
                }
            }
            if let Some(words) = unavailable {
                span { class: "citation-unavailable-view", "{words}" }
            }
            {children}
        }
    }
}

/// A page or dialog with one on-demand source beside it. With room for both, the source takes the
/// narrower column; without, it covers the reader with Back. The reader is never remounted. The
/// host gives the open source's `title`, its `eyebrow` (which source it is) and its `content`;
/// `on_close` reports once the pane has settled away, and focus returns to the citation that
/// opened it.
#[component]
pub fn SourceReading(
    #[props(default)] source: Option<SourcePane>,
    on_close: EventHandler,
    children: Element,
) -> Element {
    let mut closing = use_signal(|| false);
    let returns = use_context_provider(|| SourceReturn(Signal::new(None)));
    let open = source.is_some();
    let mut dismiss = move || {
        if closing() {
            return;
        }
        closing.set(true);
        spawn(async move {
            futures_timer::Delay::new(Duration::from_millis(EXIT_MS)).await;
            closing.set(false);
            on_close.call(());
            let SourceReturn(mut target) = returns;
            if let Some(target) = target.take() {
                let _ = target.set_focus(true).await;
            }
        });
    };
    let mut class = String::from("source-reading-layout");
    if open {
        class.push_str(" has-source");
    }
    if closing() {
        class.push_str(" is-source-closing");
    }
    rsx! {
        div {
            class: "{class}",
            onkeydown: move |event| {
                if event.key() == Key::Escape && open {
                    event.stop_propagation();
                    event.prevent_default();
                    dismiss();
                }
            },
            div { class: "source-reading-frame",
            div { class: "source-reading-main", {children} }
            if let Some(pane) = source {
                aside { class: "source-reading-pane", aria_label: "Source",
                    div { class: "source-pane-content",
                        div { class: "source-pane-heading",
                            button { class: "navigation-control source-pane-back", r#type: "button", onclick: move |_| dismiss(),
                                Icon { name: IconName::ArrowLeft }
                                "Back"
                            }
                            div {
                                small { "{pane.eyebrow}" }
                                h2 {
                                    tabindex: "-1",
                                    onmounted: move |event| {
                                        let data = event.data();
                                        spawn(async move {
                                            let _ = data.set_focus(true).await;
                                        });
                                    },
                                    "{pane.title}"
                                }
                            }
                            Button {
                                class: "source-pane-close",
                                variant: ButtonVariant::Quiet,
                                icon_only: true,
                                compact: true,
                                aria_label: "Close source",
                                onclick: move |_| dismiss(),
                                Icon { name: IconName::X }
                            }
                        }
                        div { class: "source-pane-body", {pane.content} }
                    }
                }
            }
            }
        }
    }
}

/// The source a `SourceReading` shows: which source, its title, and what it says.
#[derive(Clone, PartialEq)]
pub struct SourcePane {
    /// Which source it is, such as "Source 2".
    pub eyebrow: String,
    /// The source's title.
    pub title: String,
    /// What it says, usually a `SourceDetails`.
    pub content: Element,
}
