//! Reading a cited source, after the React app's citation preview and source reading layout: a
//! citation inspects its source in place on hover, and opens the same content beside the reader
//! when asked. The reader is never replaced; on a narrow page the source covers it, with Back.

use std::rc::Rc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Duration;

use dioxus::html::geometry::PixelsRect;
use dioxus::prelude::*;

use crate::{Button, ButtonVariant, EXIT_MS, Icon, IconName};

static PREVIEWS: AtomicUsize = AtomicUsize::new(0);

/// How long a preview waits after the pointer and focus leave, so moving from citation to preview
/// keeps it open.
const CLOSE_DELAY_MS: u64 = 160;

/// The room a preview keeps from the window's edges.
const EDGE_PX: f64 = 12.0;

/// The room between a preview and its citation.
const GAP_PX: f64 = 6.0;

/// What the open preview hears from the page while it is open, until told to stop: the window's
/// size now and on every scroll or resize; Escape, taken first so a surrounding dialog keeps it,
/// with whether focus was inside the preview; and the citation going out of sight, scroll areas
/// included. Every Dioxus renderer runs it the same way, so the preview depends on no one engine's
/// CSS. `{citation}` and `{preview}` are the two elements' ids.
const WATCH_PAGE: &str = "
const root = document.documentElement;
const citation = document.getElementById('{citation}');
const report = () => dioxus.send([0, root.clientWidth, root.clientHeight]);
const escape = (event) => {
  if (event.key !== 'Escape') return;
  event.preventDefault();
  event.stopPropagation();
  // Looked up now: this script starts before the preview is in the page.
  const preview = document.getElementById('{preview}');
  dioxus.send([1, preview !== null && preview.contains(document.activeElement) ? 1 : 0, 0]);
};
const sight = new IntersectionObserver(([entry]) => { if (entry !== undefined && !entry.isIntersecting) dioxus.send([2, 0, 0]); });
if (citation !== null) sight.observe(citation);
addEventListener('scroll', report, true);
addEventListener('resize', report);
document.addEventListener('keydown', escape, true);
report();
await dioxus.recv();
sight.disconnect();
removeEventListener('scroll', report, true);
removeEventListener('resize', report);
document.removeEventListener('keydown', escape, true);
";

/// One thing `WATCH_PAGE` heard.
#[derive(Clone, Copy, Debug, PartialEq)]
enum Heard {
    /// The window, as wide and tall as this.
    Window(f64, f64),
    /// Escape, and whether focus was inside the preview.
    Escape(bool),
    /// The citation is out of sight.
    Away,
}

impl Heard {
    fn read([kind, a, b]: [f64; 3]) -> Option<Self> {
        match kind as u8 {
            0 => Some(Self::Window(a, b)),
            1 => Some(Self::Escape(a != 0.0)),
            2 => Some(Self::Away),
            _ => None,
        }
    }
}

/// Where a preview sits: its left edge, the edge nearest its citation, and the height it has.
#[derive(Clone, Copy, Debug, PartialEq)]
struct Placement {
    left: f64,
    /// From the window's top when below the citation, from its bottom when above.
    edge: f64,
    below: bool,
    room: f64,
}

/// Places a preview of `width` beside `citation` in a window of `window` size, after the React
/// app's anchored popover. It goes on the side with more room, chosen once per opening (`side`)
/// so content that loads later never flips it, and never covers the citation: its height is
/// held to that side instead. Above, it grows away from the citation.
fn place(citation: PixelsRect, width: f64, window: (f64, f64), side: Option<bool>) -> Placement {
    let (window_width, window_height) = window;
    let below_room = (window_height - EDGE_PX - citation.max_y() - GAP_PX).max(0.0);
    let above_room = (citation.min_y() - GAP_PX - EDGE_PX).max(0.0);
    let below = side.unwrap_or(below_room >= above_room);
    Placement {
        left: citation
            .min_x()
            .min(window_width - width - EDGE_PX)
            .max(EDGE_PX),
        edge: if below {
            citation.max_y() + GAP_PX
        } else {
            window_height - citation.min_y() + GAP_PX
        },
        below,
        room: if below { below_room } else { above_room },
    }
}

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
    let citation_id = format!("citation-{id}");
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
            button {
                id: "{citation_id}",
                class: "source-trigger citation-chip",
                r#type: "button",
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
                PreviewSurface {
                    id: "{preview_id}",
                    label: "Source {index}: {label}",
                    citation: trigger,
                    citation_id: "{citation_id}",
                    on_enter: move |_| {
                        hovering.set(true);
                        pending += 1;
                    },
                    on_away: move |_| close(false),
                    on_escape: close,
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

/// The preview's surface, placed beside its citation while it is open and following it as the
/// page scrolls. It stays unseen until first placed, so it enters already on its side. Escape
/// closes it wherever focus is, returning focus to the citation only from inside the preview;
/// the citation going out of sight closes it too.
#[component]
fn PreviewSurface(
    id: String,
    label: String,
    citation: Signal<Option<Rc<MountedData>>>,
    citation_id: String,
    on_enter: EventHandler,
    on_away: EventHandler,
    on_escape: EventHandler<bool>,
    children: Element,
) -> Element {
    let mut placement = use_signal(|| None::<Placement>);
    let watch = use_hook(|| {
        document::eval(
            &WATCH_PAGE
                .replace("{citation}", &citation_id)
                .replace("{preview}", &id),
        )
    });
    use_drop(move || {
        let _ = watch.send(());
    });
    // Every property is written each time: a style the renderer merges must never keep a stale one.
    let style = match placement() {
        None => "visibility: hidden; left: 0px; top: 0px; bottom: auto".to_owned(),
        Some(at) => {
            let (top, bottom) = if at.below {
                (format!("{}px", at.edge), "auto".to_owned())
            } else {
                ("auto".to_owned(), format!("{}px", at.edge))
            };
            format!(
                "visibility: visible; left: {}px; top: {top}; bottom: {bottom}; --popover-available-height: {}px",
                at.left, at.room
            )
        }
    };
    rsx! {
        span {
            id: "{id}",
            class: "citation-preview",
            role: "dialog",
            aria_label: "{label}",
            "data-side": placement().map(|at| if at.below { "below" } else { "above" }),
            style: "{style}",
            onpointerenter: move |_| on_enter.call(()),
            onmounted: move |event| {
                let surface = event.data();
                let mut watch = watch;
                spawn(async move {
                    while let Ok(message) = watch.recv::<[f64; 3]>().await {
                        match Heard::read(message) {
                            Some(Heard::Window(width, height)) => {
                                let Some(trigger) = citation.peek().clone() else { break };
                                let (Ok(at), Ok(own)) = (trigger.get_client_rect().await, surface.get_client_rect().await) else {
                                    continue;
                                };
                                let side = (*placement.peek()).map(|at| at.below);
                                placement.set(Some(place(at, own.width(), (width, height), side)));
                            }
                            Some(Heard::Escape(inside)) => {
                                on_escape.call(inside);
                                break;
                            }
                            Some(Heard::Away) => {
                                on_away.call(());
                                break;
                            }
                            None => {}
                        }
                    }
                });
            },
            {children}
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

#[cfg(test)]
mod tests {
    use dioxus::html::geometry::PixelsSize;
    use dioxus::html::geometry::euclid::Point2D;

    use super::*;

    fn citation(x: f64, y: f64) -> PixelsRect {
        PixelsRect::new(Point2D::new(x, y), PixelsSize::new(24.0, 20.0))
    }

    #[test]
    fn opens_on_the_roomier_side_without_covering_the_citation() {
        let high = place(citation(100.0, 100.0), 360.0, (1000.0, 800.0), None);
        assert!(high.below);
        assert_eq!(high.edge, 126.0);
        assert_eq!(high.room, 800.0 - 12.0 - 120.0 - 6.0);
        let low = place(citation(100.0, 700.0), 360.0, (1000.0, 800.0), None);
        assert!(!low.below);
        assert_eq!(low.edge, 800.0 - 700.0 + 6.0);
        assert_eq!(low.room, 700.0 - 6.0 - 12.0);
    }

    #[test]
    fn keeps_its_side_once_chosen_and_stays_inside_the_window() {
        let kept = place(citation(900.0, 700.0), 360.0, (1000.0, 800.0), Some(true));
        assert!(kept.below);
        assert_eq!(kept.left, 1000.0 - 360.0 - 12.0);
        let narrow = place(citation(4.0, 100.0), 360.0, (300.0, 800.0), None);
        assert_eq!(narrow.left, 12.0);
    }

    #[test]
    fn reads_what_the_page_reports() {
        assert_eq!(
            Heard::read([0.0, 1280.0, 800.0]),
            Some(Heard::Window(1280.0, 800.0))
        );
        assert_eq!(Heard::read([1.0, 1.0, 0.0]), Some(Heard::Escape(true)));
        assert_eq!(Heard::read([1.0, 0.0, 0.0]), Some(Heard::Escape(false)));
        assert_eq!(Heard::read([2.0, 0.0, 0.0]), Some(Heard::Away));
        assert_eq!(Heard::read([9.0, 0.0, 0.0]), None);
    }
}
