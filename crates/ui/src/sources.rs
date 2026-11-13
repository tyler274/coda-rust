//! Reading a cited source: a citation previews its source in place on hover, in the source's own
//! words, and opens the source itself beside the reader when asked. Coda's own description of a
//! source stands in only when nothing of the source's own can be shown, set apart as Coda's. The
//! reader is never replaced; on a narrow page the source covers it, with Back.

use std::rc::Rc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Duration;

use dioxus::html::geometry::euclid::Point2D;
use dioxus::html::geometry::{PixelsRect, PixelsSize};
use dioxus::prelude::*;

use crate::script::{self, Bounds, PageHeard, PopoverHeard};
use crate::{
    Button, ButtonVariant, Disclosure, EXIT_MS, Icon, IconName, Placeholder, PlaceholderShape,
    StatePill, StatusLine, Tone, opens,
};

static PREVIEWS: AtomicUsize = AtomicUsize::new(0);

/// How long a preview waits after the pointer and focus leave, so moving from citation to preview
/// keeps it open.
const CLOSE_DELAY_MS: u64 = 160;

/// The room a preview keeps from the window's edges.
const EDGE_PX: f64 = 12.0;

/// The room between a preview and its citation.
const GAP_PX: f64 = 6.0;

/// Where a preview sits: its left edge, the edge nearest its citation, and the height it has.
#[derive(Clone, Copy, Debug, PartialEq)]
struct Placement {
    left: f64,
    /// From the window's top when below the citation, from its bottom when above.
    edge: f64,
    below: bool,
    /// Whether the citation stands nearer the preview's leading edge than its trailing one.
    leading: bool,
    room: f64,
}

/// Places a preview of `width` beside `citation` in a window of `window` size.
/// It goes on the side with more room, chosen once per opening (`side`)
/// so content that loads later never flips it, and never covers the citation: its height is
/// held to that side instead. Above, it grows away from the citation.
fn place(citation: PixelsRect, width: f64, window: (f64, f64), side: Option<bool>) -> Placement {
    let (window_width, window_height) = window;
    let below_room = (window_height - EDGE_PX - citation.max_y() - GAP_PX).max(0.0);
    let above_room = (citation.min_y() - GAP_PX - EDGE_PX).max(0.0);
    let below = side.unwrap_or(below_room >= above_room);
    let left = citation
        .min_x()
        .min(window_width - width - EDGE_PX)
        .max(EDGE_PX);
    Placement {
        left,
        edge: if below {
            citation.max_y() + GAP_PX
        } else {
            window_height - citation.min_y() + GAP_PX
        },
        below,
        leading: (citation.min_x() + citation.max_x()) / 2.0 <= left + width / 2.0,
        room: if below { below_room } else { above_room },
    }
}

impl Placement {
    /// Which side of its control it is on.
    fn side(self) -> &'static str {
        if self.below { "below" } else { "above" }
    }

    /// The corner it grows from, nearest its control.
    fn corner(self) -> &'static str {
        if self.leading { "start" } else { "end" }
    }
}

/// The style of a surface `place` puts: unseen until placed, then where it stands and the height it
/// has. Every property is written each time: a style the renderer merges must never keep a stale
/// one.
fn placed_style(placement: Option<Placement>) -> String {
    match placement {
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
    }
}

impl Bounds {
    fn rect(self) -> PixelsRect {
        PixelsRect::new(
            Point2D::new(self.x, self.y),
            PixelsSize::new(self.width, self.height),
        )
    }
}

/// A popover of the host's content that the control named `control` opens, a `Button` whose `opens`
/// names this `id`, placed as a citation's preview is: on the side of the control with more room,
/// chosen once per opening, never over the control, its height held to that side, and following the
/// control as the page moves. It opens and closes as a `Popover` does; the control going out of
/// sight closes it too. `label` names it.
#[component]
pub(crate) fn PlacedPopover(
    id: String,
    label: String,
    control: String,
    children: Element,
) -> Element {
    let mut placement = use_signal(|| None::<Placement>);
    let popover = id.clone();
    rsx! {
        div {
            id: "{id}",
            class: "popover is-placed",
            "popover": "auto",
            role: "dialog",
            aria_label: label,
            "data-side": placement().map(Placement::side),
            "data-corner": placement().map(Placement::corner),
            style: "{placed_style(placement())}",
            onmounted: move |_| {
                let watch = script::watch_popover(&control, &popover);
                spawn(async move {
                    // The behaviour finished or failed: the popover stops being placed.
                    while let Ok(heard) = watch.next().await {
                        let (standing, side) = match heard {
                            PopoverHeard::Opened(standing) => (standing, None),
                            PopoverHeard::Moved(standing) => (standing, (*placement.peek()).map(|at| at.below)),
                            // Where it stood holds while it settles away; the next opening places it afresh.
                            PopoverHeard::Closed => continue,
                        };
                        placement.set(Some(place(standing.control.rect(), standing.width, standing.window, side)));
                    }
                });
            },
            {children}
        }
    }
}

/// Where the reading layout returns focus once its source closes: the citation that opened it.
#[derive(Clone, Copy)]
struct SourceReturn(Signal<Option<Rc<MountedData>>>);

/// Coda's short, dated description of a source, shown only when nothing of the source's own can
/// be shown, such as an outside document whose content can't be displayed: Coda's words, then the
/// line that marks them as Coda's with the date Coda read the source for its work, not the
/// record's own date, such as "Prepared by Coda · as of May 2, 2024". Both are the host's words;
/// the part sets them apart from the source's own.
#[derive(Clone, Debug, PartialEq)]
pub struct SourceSummary {
    /// Coda's description of the source.
    pub text: String,
    /// The line that marks the description as Coda's, with the date Coda read the source.
    pub prepared: String,
}

/// What a record says about itself, on one quiet line: its kind, when the title doesn't already say
/// it, its date and its own status. A record that says none of these has no line.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct RecordAbout {
    /// What kind of record it is, such as "Discharge summary"; none when the title already says it.
    pub kind: Option<String>,
    /// The record's date, when it has one.
    pub date: Option<String>,
    /// The record's own status, when it has one.
    pub status: Option<RecordStatus>,
}

/// What a citation's preview shows of its source: the source's own words, or, only when nothing of
/// the source's own can be shown, Coda's description of it.
#[derive(Clone, Debug, PartialEq)]
pub enum PreviewContent {
    /// What the record says about itself, then the cited place in its own words, when the citation
    /// names one.
    Cited {
        /// The record's kind, date and status.
        about: RecordAbout,
        /// The cited place; none when the citation names the record as a whole.
        place: Option<CitedPlace>,
    },
    /// Nothing of the source's own can be shown: Coda's description of it, marked as Coda's.
    Described(SourceSummary),
}

/// Where in its source a cited claim rests, as a preview shows it.
#[derive(Clone, Debug, PartialEq)]
pub enum CitedPlace {
    /// The source's own words: passages quoted from its text, each a quoted block, then values of a
    /// structured record, each a row, both in the order given.
    Quoted(Places<String, CitedField>),
    /// A page of a scan that has no text of its own, in the host's words: "Page 2".
    Page(String),
}

/// One value of a structured record, as a preview quotes it: its name, and its value as the record
/// gives it.
#[derive(Clone, Debug, PartialEq)]
pub struct CitedField {
    /// The field's name.
    pub name: String,
    /// Its value, with its unit when it has one.
    pub value: String,
}

/// The places a citation names in its source: passages of its text and fields of a structured
/// record, each in the order given, and at least one in all. A set with neither can't be made.
#[derive(Clone, Debug, PartialEq)]
pub struct Places<P, F> {
    passages: Vec<P>,
    fields: Vec<F>,
}

impl<P, F> Places<P, F> {
    /// The passages and the fields, or none when there is neither.
    pub fn new(passages: Vec<P>, fields: Vec<F>) -> Option<Self> {
        if passages.is_empty() && fields.is_empty() {
            return None;
        }
        Some(Self { passages, fields })
    }

    /// One passage.
    pub fn passage(passage: P) -> Self {
        Self {
            passages: vec![passage],
            fields: Vec::new(),
        }
    }

    /// One field.
    pub fn field(field: F) -> Self {
        Self {
            passages: Vec::new(),
            fields: vec![field],
        }
    }

    /// The passages, in order.
    pub fn passages(&self) -> &[P] {
        &self.passages
    }

    /// The fields, in order.
    pub fn fields(&self) -> &[F] {
        &self.fields
    }
}

/// The source in its own system, opened in a new tab: its address, the link's words, and the
/// hint that it opens in a new tab. An address that can't open as a link is not offered.
#[derive(Clone, Debug, PartialEq)]
pub struct SourceLink {
    /// Where the source opens.
    pub url: String,
    /// The link's words: "Open in Epic".
    pub label: String,
    /// Says it opens in a new tab.
    pub new_tab_hint: Option<String>,
}

/// The way from a citation's preview to its source in the reading pane: the action's words, and
/// what opens the pane.
#[derive(Clone, PartialEq)]
pub struct ExpandSource {
    /// The action's words: "Expand".
    pub label: String,
    /// Opens the same source in the reading pane.
    pub on_expand: EventHandler,
}

/// A citation in prose that previews its source. Pointing at it opens the preview beside it, which
/// stays while the pointer or focus is on either; Escape or Close dismisses it. Under its eyebrow
/// and title, the preview shows its `content`: one quiet line of what the record says about itself,
/// then the cited place, when it names one, in the source's own words, never the whole record; or,
/// only when nothing of the source's own can be shown, Coda's description, marked as Coda's.
/// Choosing the citation opens the source beside the reader when the host gives `expand`, and
/// otherwise toggles the preview, as on touch. The preview's actions open the pane and the
/// `original` record, each shown with its words.
#[component]
pub fn CitationPreview(
    index: usize,
    label: String,
    /// Which source it is, above its label: "Source 2".
    eyebrow: String,
    /// The citation's accessible name, saying that it previews its source.
    citation_label: String,
    /// The preview's close button's accessible name.
    close_label: String,
    content: PreviewContent,
    #[props(default)] expand: Option<ExpandSource>,
    #[props(default)] original: Option<SourceLink>,
) -> Element {
    let original = original.filter(|link| opens(&link.url));
    let on_expand = expand.as_ref().map(|expand| expand.on_expand);
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
    let mut open_pane = move |handler: EventHandler| {
        close(false);
        if let Some(SourceReturn(mut target)) = returns {
            target.set(trigger.peek().clone());
        }
        handler.call(());
    };
    let citation_id = format!("citation-{id}");
    let preview_id = format!("citation-preview-{id}");
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
                "data-source-number": "{index}",
                r#type: "button",
                aria_label: "{citation_label}",
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
                    Some(handler) => open_pane(handler),
                    None if open() => close(false),
                    None => show(),
                },
                "[{index}]"
            }
            if open() {
                PreviewSurface {
                    id: "{preview_id}",
                    label: "{eyebrow}: {label}",
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
                            small { "{eyebrow}" }
                            strong { "{label}" }
                        }
                        Button {
                            variant: ButtonVariant::Quiet,
                            icon_only: true,
                            compact: true,
                            aria_label: "{close_label}",
                            onclick: move |_| close(true),
                            Icon { name: IconName::X }
                        }
                    }
                    span { class: "citation-preview-body",
                        match content {
                            PreviewContent::Cited { about, place } => rsx! {
                                AboutLine { about }
                                if let Some(place) = place {
                                    PlaceView { place }
                                }
                            },
                            PreviewContent::Described(summary) => rsx! { SummaryView { summary } },
                        }
                    }
                    if expand.is_some() || original.is_some() {
                        span { class: "citation-preview-actions",
                            if let Some(ExpandSource { label, on_expand: handler }) = expand {
                                button {
                                    class: "navigation-control",
                                    r#type: "button",
                                    onclick: move |_| open_pane(handler),
                                    Icon { name: IconName::Expand }
                                    "{label}"
                                }
                            }
                            if let Some(link) = original {
                                OriginalLink { link, on_follow: move |_| close(false) }
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
    let watch = use_hook(|| script::watch_page(&citation_id, &id));
    rsx! {
        span {
            id: "{id}",
            class: "citation-preview",
            role: "dialog",
            aria_label: "{label}",
            "data-side": placement().map(Placement::side),
            "data-corner": placement().map(Placement::corner),
            style: "{placed_style(placement())}",
            onpointerenter: move |_| on_enter.call(()),
            onmounted: move |event| {
                let surface = event.data();
                let watch = watch.clone();
                spawn(async move {
                    loop {
                        // The behaviour finished or failed: the preview stops following it.
                        let heard = watch.next().await;
                        let Ok(message) = heard else { break };
                        match message {
                            PageHeard::Window { width, height } => {
                                let Some(trigger) = citation.peek().clone() else { break };
                                let rects = futures_util::future::join(trigger.get_client_rect(), surface.get_client_rect()).await;
                                let (Ok(at), Ok(own)) = rects else { continue };
                                let side = (*placement.peek()).map(|at| at.below);
                                placement.set(Some(place(at, own.width(), (width, height), side)));
                            }
                            PageHeard::Escape { inside } => {
                                on_escape.call(inside);
                                break;
                            }
                            PageHeard::Away => {
                                on_away.call(());
                                break;
                            }
                        }
                    }
                });
            },
            {children}
        }
    }
}

/// Coda's description of a source, in Coda's words, closed by the line that marks it as Coda's.
#[component]
fn SummaryView(summary: SourceSummary) -> Element {
    rsx! {
        span { class: "source-summary",
            span { class: "source-summary-text", "{summary.text}" }
            span { class: "source-summary-prepared",
                Icon { name: IconName::Sparkle }
                "{summary.prepared}"
            }
        }
    }
}

/// Where a claim rests, in the source's own words: each passage quoted in a block of its own, then
/// the fields in one block, a row each with its name and value; or the page of a scan named.
#[component]
fn PlaceView(place: CitedPlace) -> Element {
    match place {
        CitedPlace::Quoted(places) => rsx! {
            for (index, words) in places.passages().iter().enumerate() {
                span { key: "{index}", class: "source-quote", "{words}" }
            }
            if !places.fields().is_empty() {
                span { class: "source-quote",
                    for (index, field) in places.fields().iter().enumerate() {
                        span { key: "{index}", class: "source-quote-field",
                            span { class: "source-quote-name", "{field.name}" }
                            span { class: "source-quote-value", "{field.value}" }
                        }
                    }
                }
            }
        },
        CitedPlace::Page(page) => rsx! {
            span { class: "source-page",
                Icon { name: IconName::File }
                "{page}"
            }
        },
    }
}

/// A link to the source in its own system, in a new tab; following it tells the host.
#[component]
fn OriginalLink(link: SourceLink, #[props(default)] on_follow: Option<EventHandler>) -> Element {
    rsx! {
        a {
            class: "navigation-control source-original",
            href: "{link.url}",
            target: "_blank",
            rel: "noopener noreferrer",
            title: link.new_tab_hint,
            onclick: move |_| {
                if let Some(follow) = on_follow {
                    follow.call(());
                }
            },
            span { "{link.label}" }
            Icon { name: IconName::ExternalLink }
        }
    }
}

/// A page or dialog with one on-demand source beside it. With room for both, the source takes the
/// narrower column; without, it covers the reader with a way back. The reader is never remounted.
/// Only the pane's body scrolls, between its heading and, when the source links to its own system,
/// its foot, each set off by a hairline. The host gives the open source as a `SourcePane`: which
/// source it is, its title, where its reading stands, and the words of its way back and its close
/// button;
/// `on_close` reports once the pane has settled away, and focus returns to the citation that
/// opened it.
#[component]
pub fn SourceReading(
    #[props(default)] source: Option<SourcePane>,
    on_close: EventHandler,
    children: Element,
) -> Element {
    let mut closing = use_signal(|| false);
    let body = use_hook(|| format!("source-pane-{}", PREVIEWS.fetch_add(1, Ordering::Relaxed)));
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
    let source = source.map(|pane| {
        let (about, original) = pinned(&pane.state);
        (pane, about, original)
    });
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
            if let Some((pane, about, original)) = source {
                aside { class: "source-reading-pane", aria_label: "{pane.eyebrow}",
                    div { class: "source-pane-content",
                        div { class: "source-pane-heading",
                            button { class: "navigation-control source-pane-back", r#type: "button", onclick: move |_| dismiss(),
                                Icon { name: IconName::ArrowLeft }
                                "{pane.back}"
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
                                if let Some(about) = about {
                                    AboutLine { about }
                                }
                            }
                            Button {
                                class: "source-pane-close",
                                variant: ButtonVariant::Quiet,
                                icon_only: true,
                                compact: true,
                                aria_label: "{pane.close}",
                                onclick: move |_| dismiss(),
                                Icon { name: IconName::X }
                            }
                        }
                        PaneBody { id: body.clone(), SourceView { state: pane.state } }
                        if let Some(link) = original {
                            div { class: "citation-preview-actions", OriginalLink { link } }
                        }
                    }
                }
            }
            }
        }
    }
}

/// What of a source shown stands outside the pane's scrolling body: what the record says about
/// itself, under the title, and the link to it in its own system, at the pane's foot.
fn pinned(state: &SourceState) -> (Option<RecordAbout>, Option<SourceLink>) {
    match state {
        SourceState::Shown(record) => (
            Some(record.about.clone()),
            record.original.clone().filter(|link| opens(&link.url)),
        ),
        SourceState::Loading(_)
        | SourceState::Failed { .. }
        | SourceState::Unavailable(_)
        | SourceState::Described(_) => (None, None),
    }
}

/// The pane's scrolling body, the surface its source's passages are read in.
#[component]
fn PaneBody(id: String, children: Element) -> Element {
    let body = id.clone();
    use_context_provider(move || PassageSurface(body));
    rsx! { div { id: "{id}", class: "source-pane-body", {children} } }
}

/// The source a `SourceReading` shows: which source, its title, where its reading stands, and the
/// words of its way back and its close button.
#[derive(Clone, PartialEq)]
pub struct SourcePane {
    /// Which source it is, such as "Source 2".
    pub eyebrow: String,
    /// The source's title.
    pub title: String,
    /// Where the source's reading stands.
    pub state: SourceState,
    /// The words of the way back to the reader, where the source covers it.
    pub back: String,
    /// The close button's accessible name.
    pub close: String,
}

/// Where a source's reading in the pane stands. Each state holds only what it shows, so content
/// never stands beside "unavailable", and a highlight never stands beside the notice that the
/// cited place can't be found.
#[derive(Clone, PartialEq)]
pub enum SourceState {
    /// Being read: its shape, with these words for assistive technology.
    Loading(String),
    /// The read failed: what failed, and the retry's words and action.
    Failed {
        /// What failed.
        message: String,
        /// The retry's words.
        retry: String,
        /// Reads the source again.
        on_retry: EventHandler,
    },
    /// It can't be read or shown here: the reason, and no content.
    Unavailable(String),
    /// Nothing of the source's own can be shown: Coda's description of it, marked as Coda's, and
    /// nothing else.
    Described(SourceSummary),
    /// The version Coda read, boxed as it is far larger than the other states.
    Shown(Box<SourceRecord>),
}

/// A source as it reads in the pane: under its title, what it says about itself on one quiet line;
/// under a hairline, scrolling, the record itself, with its cited places highlighted, nothing
/// highlighted when it is cited as a whole, or a notice that its cited place can't be found, and
/// last, collapsed, the raw record; and pinned at the pane's foot under a hairline, a link to it in
/// its own system when there is one.
#[derive(Clone, PartialEq)]
pub struct SourceRecord {
    /// Its kind, date and status.
    pub about: RecordAbout,
    /// The record itself.
    pub body: RecordBody,
    /// The record in its own system, when the source gives a link.
    pub original: Option<SourceLink>,
    /// The record as the source gave it.
    pub raw: RawRecord,
}

/// A record's own status, in its words. One that is inactive, refuted, entered in error or
/// preliminary is `Flagged`, and stands out before the record's statement.
#[derive(Clone, Debug, PartialEq)]
pub enum RecordStatus {
    /// A status that changes nothing about how the record reads: "Final", "Signed".
    Settled(String),
    /// Inactive, refuted, entered in error or preliminary.
    Flagged(String),
}

/// The record as the source gave it, under its disclosure's title: "Technical details".
#[derive(Clone, Debug, PartialEq)]
pub struct RawRecord {
    /// The disclosure's title.
    pub label: String,
    /// The raw record.
    pub text: String,
}

/// Where a citation points in the record the pane shows: the record as a whole, its cited places,
/// or the notice that its cited place can't be found. Only cited places are highlighted, and only a
/// place that can't be found brings a notice.
#[derive(Clone, Debug, PartialEq)]
pub enum CitedAt {
    /// The record as a whole: nothing is highlighted and no notice shows.
    Whole,
    /// The cited passages and fields, each highlighted, a field named by its key. The pane opens at
    /// the first of them in the record's reading order.
    Places(Places<PartRange, String>),
    /// The notice that the cited place can't be found; the record shows without a highlight.
    NotFound(String),
}

/// A cited passage of a record's text: the part it is in, by its place among the record's parts,
/// and its byte range of that part's text. One that isn't a range of its part is left unmarked.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PartRange {
    /// The part, counted from 0 in the record's order.
    pub part: usize,
    /// The byte range of the part's text.
    pub range: (usize, usize),
}

/// The record itself: its own text, part by part, then its fields, group by group, and where its
/// citation points in it. A record may have text, fields or both.
#[derive(Clone, Debug, PartialEq)]
pub struct RecordBody {
    /// The record's own text, in parts, in order.
    pub parts: Vec<TextPart>,
    /// Every field under its name, in groups, in order.
    pub groups: Vec<FieldGroup>,
    /// What a result's reference range is called beside it: "Reference range".
    pub range_label: String,
    /// Where the citation points in the record.
    pub cited: CitedAt,
}

/// One part of a record's own text, under its own label when it has one, such as "Assessment and
/// plan".
#[derive(Clone, Debug, PartialEq)]
pub struct TextPart {
    /// The part's label.
    pub label: Option<String>,
    /// The part's text.
    pub text: String,
}

/// Fields that stand together, under an optional heading such as "Other fields".
#[derive(Clone, Debug, PartialEq)]
pub struct FieldGroup {
    /// The group's heading.
    pub heading: Option<String>,
    /// Its fields, in order.
    pub fields: Vec<RecordField>,
}

/// One field of a structured record: its stable key, such as its path in the record, which a
/// citation names; its name, known or the source's own; and its value.
#[derive(Clone, Debug, PartialEq)]
pub struct RecordField {
    /// The field's stable key.
    pub key: String,
    /// The field's name.
    pub name: String,
    /// Its value.
    pub value: FieldValue,
}

/// A field's value, copied from the record.
#[derive(Clone, Debug, PartialEq)]
pub enum FieldValue {
    /// What the record gives: the value, and its unit, reference range and status when it has them.
    Given {
        /// The value.
        value: String,
        /// Its unit.
        unit: Option<String>,
        /// Its reference range.
        range: Option<String>,
        /// Its status, such as "High".
        status: Option<String>,
    },
    /// The source says this part is absent: the host's words for that, "Not supplied".
    NotSupplied(String),
}

/// What the pane shows for where the source's reading stands.
#[component]
fn SourceView(state: SourceState) -> Element {
    match state {
        SourceState::Loading(words) => rsx! {
            div { class: "source-loading", role: "status",
                p { class: "sr-only", "{words}" }
                Placeholder { shape: PlaceholderShape::Reading }
            }
        },
        SourceState::Failed {
            message,
            retry,
            on_retry,
        } => rsx! {
            StatusLine { message, alert: true, retry, on_retry }
        },
        SourceState::Unavailable(reason) => rsx! {
            p { class: "source-unavailable", "{reason}" }
        },
        SourceState::Described(summary) => rsx! { SummaryView { summary } },
        SourceState::Shown(record) => {
            let SourceRecord { body, raw, .. } = *record;
            rsx! { RecordView { body, raw } }
        }
    }
}

/// A source shown, in the pane's body: the notice when its cited place can't be found; the record,
/// its text parts, each under its label, then its fields, with its cited places marked; its raw
/// form. The pane it is read in opens at the first place marked, and again whenever the record or
/// its places change, never on other renders.
#[component]
fn RecordView(body: RecordBody, raw: RawRecord) -> Element {
    let surface = try_use_context::<PassageSurface>();
    use_effect(use_reactive!(|body| {
        let _ = &body;
        if let Some(PassageSurface(id)) = &surface {
            script::scroll_to_passage(id);
        }
    }));
    let RecordBody {
        parts,
        groups,
        range_label,
        cited,
    } = body;
    let (passages, keys, notice) = match cited {
        CitedAt::Whole => (Vec::new(), Vec::new(), None),
        CitedAt::Places(places) => (places.passages().to_vec(), places.fields().to_vec(), None),
        CitedAt::NotFound(notice) => (Vec::new(), Vec::new(), Some(notice)),
    };
    rsx! {
        div { class: "source-record",
            if let Some(notice) = notice {
                p { class: "source-notice", role: "status",
                    Icon { name: IconName::Search }
                    "{notice}"
                }
            }
            div { class: "source-record-body",
                for (index, part) in parts.into_iter().enumerate() {
                    div { key: "{index}", class: "source-text-part",
                        if let Some(label) = part.label {
                            h3 { class: "source-text-label", "{label}" }
                        }
                        SourceText {
                            text: part.text,
                            passages: passages.iter().filter(|at| at.part == index).map(|at| at.range).collect::<Vec<_>>(),
                        }
                    }
                }
                if !groups.is_empty() {
                    RecordFields { groups, cited: keys, range_label }
                }
            }
            Disclosure { title: raw.label, detail: true,
                pre { class: "source-raw", "{raw.text}" }
            }
        }
    }
}

/// What a record says about itself on one quiet line, a flagged status standing out as a warning;
/// nothing when it says none of it.
#[component]
fn AboutLine(about: RecordAbout) -> Element {
    let RecordAbout { kind, date, status } = about;
    if kind.is_none() && date.is_none() && status.is_none() {
        return rsx! {};
    }
    rsx! {
        span { class: "source-record-about",
            if let Some(kind) = kind {
                span { "{kind}" }
            }
            if let Some(date) = date {
                span { "{date}" }
            }
            match status {
                Some(RecordStatus::Settled(words)) => rsx! { span { "{words}" } },
                Some(RecordStatus::Flagged(words)) => rsx! {
                    span { StatePill { tone: Tone::Warning, icon: IconName::Alert, "{words}" } }
                },
                None => rsx! {},
            }
        }
    }
}

/// Every field of a record under its name, group by group, the cited ones, named by their keys,
/// highlighted. A result keeps its own unit, reference range and status; a part the source says is
/// absent reads in its own quieter words.
#[component]
fn RecordFields(groups: Vec<FieldGroup>, cited: Vec<String>, range_label: String) -> Element {
    rsx! {
        div { class: "record-fields",
            for (index, group) in groups.into_iter().enumerate() {
                section { key: "{index}", class: "record-field-group",
                    if let Some(heading) = group.heading {
                        h3 { class: "record-field-heading", "{heading}" }
                    }
                    dl { class: "record-field-list",
                        for field in group.fields {
                            FieldRow {
                                key: "{field.key}",
                                cited: cited.contains(&field.key),
                                field,
                                range_label: range_label.clone(),
                            }
                        }
                    }
                }
            }
        }
    }
}

/// One field as a row: its name, then its value with its unit, and beneath, its reference range
/// and status. The cited row's value is marked.
#[component]
fn FieldRow(field: RecordField, cited: bool, range_label: String) -> Element {
    let value = match field.value {
        FieldValue::Given {
            value,
            unit,
            range,
            status,
        } => {
            let reading = match unit {
                Some(unit) => format!("{value} {unit}"),
                None => value,
            };
            let range = range.map(|range| format!("{range_label} {range}"));
            let detail: Vec<String> = range.into_iter().chain(status).collect();
            rsx! {
                span { class: "record-field-value", "{reading}" }
                if !detail.is_empty() {
                    span { class: "record-field-detail", {detail.join(" · ")} }
                }
            }
        }
        FieldValue::NotSupplied(words) => rsx! {
            span { class: "record-field-absent", "{words}" }
        },
    };
    rsx! {
        div { class: if cited { "record-field is-cited" } else { "record-field" },
            dt { "{field.name}" }
            dd {
                if cited {
                    mark { {value} }
                } else {
                    {value}
                }
            }
        }
    }
}

/// The reading surface a source's record is read in: the id of the pane's own scrolling body.
#[derive(Clone)]
struct PassageSurface(String);

/// A source's own words with its cited passages marked: its line breaks kept and long words
/// wrapping. `passages` are byte ranges of `text`; one that isn't a range of it is left unmarked.
#[component]
fn SourceText(text: String, passages: Vec<(usize, usize)>) -> Element {
    rsx! {
        span { class: "source-text",
            for (index, (words, cited)) in marked(&text, &passages).into_iter().enumerate() {
                if cited {
                    mark { key: "{index}", "{words}" }
                } else {
                    Fragment { key: "{index}", "{words}" }
                }
            }
        }
    }
}

/// The words in runs, each cited or not, with overlapping and touching passages merged.
fn marked(text: &str, passages: &[(usize, usize)]) -> Vec<(String, bool)> {
    let mut ranges: Vec<(usize, usize)> = passages
        .iter()
        .copied()
        .filter(|(start, end)| {
            start < end
                && *end <= text.len()
                && text.is_char_boundary(*start)
                && text.is_char_boundary(*end)
        })
        .collect();
    ranges.sort();
    let mut merged: Vec<(usize, usize)> = Vec::new();
    for (start, end) in ranges {
        match merged.last_mut() {
            Some(last) if start <= last.1 => last.1 = last.1.max(end),
            _ => merged.push((start, end)),
        }
    }
    let mut runs = Vec::new();
    let mut at = 0;
    for (start, end) in merged {
        if start > at {
            runs.push((text[at..start].to_owned(), false));
        }
        runs.push((text[start..end].to_owned(), true));
        at = end;
    }
    if at < text.len() {
        runs.push((text[at..].to_owned(), false));
    }
    runs
}

#[cfg(test)]
mod tests {
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
        assert!(
            !kept.leading,
            "held left of its citation, it grows from its trailing corner"
        );
        assert!(place(citation(100.0, 100.0), 360.0, (1000.0, 800.0), None).leading);
        let narrow = place(citation(4.0, 100.0), 360.0, (300.0, 800.0), None);
        assert_eq!(narrow.left, 12.0);
    }

    #[test]
    fn cited_passages_merge_and_a_range_outside_the_words_stays_unmarked() {
        let words = "Potassium 5.4 mmol/L. Recheck in one week.";
        assert_eq!(
            marked(words, &[(10, 20), (14, 21), (200, 210), (5, 5)]),
            vec![
                ("Potassium ".to_owned(), false),
                ("5.4 mmol/L.".to_owned(), true),
                (" Recheck in one week.".to_owned(), false),
            ]
        );
        // A range that splits a character is not a range of the words.
        assert_eq!(marked("é", &[(1, 2)]), vec![("é".to_owned(), false)]);
    }

    #[test]
    fn a_set_of_places_has_at_least_one() {
        assert_eq!(Places::<String, String>::new(Vec::new(), Vec::new()), None);
        let fields = vec!["component[3]".to_owned()];
        let places = Places::<String, String>::new(Vec::new(), fields.clone());
        assert_eq!(places.as_ref().map(Places::fields), Some(&fields[..]));
    }
}
