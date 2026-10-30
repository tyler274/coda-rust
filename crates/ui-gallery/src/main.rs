//! Every shared `ui` component, rendered by the crate itself with its own stylesheet, so its
//! look and motion can be seen and replayed apart from either app. The conversation is scripted.

#[cfg(feature = "web")]
use dioxus::prelude::*;
#[cfg(feature = "web")]
use std::time::Duration;
#[cfg(feature = "web")]
use ui::{
    ActionRow, Author, Avatar, AvatarSize, Button, ButtonVariant, Card, CitationChip,
    CitationPreview, Comparison, ComparisonBars, Composer, ContextChip, ConversationViewport,
    Count, DateSeparator, Dialog, Disclosure, DraftConflict, EmptyLine, EmptyState, FeedbackRating,
    FooterActions, Icon, IconName, KeyHint, LaterContext, ListRow, LiveDot, MeasurementTile,
    Message, NavigationControl, NavigationRow, Notice, Overline, PageTitle, Paper, PaperSection,
    Placeholder, PlaceholderShape, ProposedChange, Prose, ProsePart, Quote, Rating, RowFrame,
    RowGroup, SearchField, SectionHeader, SelectionRow, SeriesChart, SeriesPoint, SeriesShape,
    SettingRow, SourceDetails, SourcePane, SourceReading, SourceText, SourcesButton, Sparkline,
    StatePill, StatusLine, StepList, Suggestion, TabBar, Tag, TextField, ToggleChip, Tone, TopBar,
    UndoNotice, Value, WorkingMark, WorkingPresence, use_live_rows,
};

fn main() {
    #[cfg(feature = "web")]
    dioxus::launch(Gallery);
    #[cfg(not(feature = "web"))]
    {
        eprintln!("Build ui-gallery with --features web.");
        std::process::exit(1);
    }
}

#[cfg(feature = "web")]
const GALLERY_STYLES: &str = include_str!("../web/gallery.css");

/// Slows every animation and transition that runs on the page by the factor the page sets.
#[cfg(feature = "web")]
const SLOW_MOTION: &str = "
window.galleryPace = 1;
const pace = () => document.getAnimations().forEach((a) => { a.playbackRate = 1 / window.galleryPace; });
document.addEventListener('transitionrun', pace, true);
document.addEventListener('animationstart', pace, true);
window.gallerySetPace = (factor) => { window.galleryPace = factor; pace(); };
";

#[cfg(feature = "web")]
#[component]
fn Gallery() -> Element {
    let mut slow = use_signal(|| false);
    let mut replay = use_signal(|| 0u32);
    use_hook(|| {
        document::eval(SLOW_MOTION);
    });
    rsx! {
        ui::Stylesheet { app: GALLERY_STYLES }
        div { class: "gallery",
            header { class: "gallery-header",
                div {
                    h1 { class: "page-title", "crates/ui" }
                    p { class: "gallery-lede",
                        "Each piece below is the shared component and stylesheet both apps use. Entry motion replays when a section remounts."
                    }
                }
                ActionRow {
                    Button {
                        aria_pressed: if slow() { "true" } else { "false" },
                        onclick: move |_| {
                            let next = !slow();
                            slow.set(next);
                            document::eval(&format!("window.gallerySetPace({})", if next { 4 } else { 1 }));
                        },
                        if slow() { "Slow motion: on (4×)" } else { "Slow motion: off" }
                    }
                    Button { variant: ButtonVariant::Primary, onclick: move |_| replay.set(replay() + 1), "Replay entries" }
                }
            }
            div { key: "{replay}", class: "gallery-sections",
                Section { title: "Type", note: "One scale for every app, on the system sans. A role is a whole setting; hosts name one and compose none of their own.",
                    TypeDemo {}
                }
                Section { title: "Colour", note: "Roles in pairs, a fill and what reads on it. Hosts read roles, never tones; dark mode swaps the tones beneath.",
                    ColourDemo {}
                }
                Section { title: "Icons", note: "One stroke set in the text's colour: the React app's icons, then Duet's own from the October 21 mockups.",
                    IconDemo {}
                }
                Section { title: "Cards", note: "A card holds one object and names it: an observation and its evidence, saved work, an original record with the person's later context. One can open as a whole, or lead on from its footer.",
                    CardDemo {}
                }
                Section { title: "Measurements and charts", note: "Data draws in a quieter green than the action, and colour never says whether a value is normal. Bars grow on the step spring; lines draw in, then their latest point.",
                    ChartDemo {}
                }
                Section { title: "Series", note: "A series across its buckets: amounts as bars from zero, levels as a line on their own range. The latest is drawn full and the rest quieter, a bucket still filling between them; a dashed line in the action green marks what it is compared with. Tap a mark to pin it, again or on empty plot to let go; the arrow keys move it. The page says what is pinned.",
                    SeriesDemo {}
                }
                Section { title: "Screen headings", note: "A screen's title, what it belongs to above it, and its state or source beneath. A record leads with its icon; a screen with no top bar keeps its actions above the title.",
                    HeadingsDemo {}
                }
                Section { title: "Reviewing a proposed update", note: "Section by section: what changed, the proposed words, what was saved before, and the sources each rests on. A section left as it was says so.",
                    ReviewDemo {}
                }
                Section { title: "Pages and forms", note: "A pushed screen's bar, what actually happened said once, a document page, choices and settings as rows, and actions pinned to the foot.",
                    PagesDemo {}
                }
                Section { title: "Conversation frame", note: "Who the conversation is with, the day, a way to begin, a receipt that leads to a thing, and what the person brought in, held above the words.",
                    FrameDemo {}
                }
                Section { title: "Conversation", note: "Scripted. Send rises on the send spring; the typing mark holds the reply's place; the reply arrives on the arrival spring.",
                    ConversationDemo {}
                }
                Section { title: "Composer states", note: "What is happening leads the toolbar: work running, beside its verb, or the draft's state. The actions keep the far edge, a key hint beside the words where there is a keyboard. A draft saved elsewhere opens under the toolbar with the saved words as written.",
                    ComposerStatesDemo {}
                }
                Section { title: "Prose", note: "An answer's words as Markdown: headings on the title roles, lists, emphasis, quotes, code and tables that scroll on their own, with citations where they were written.",
                    ProseDemo {}
                }
                Section { title: "Following a conversation", note: "At the end, each new reply is followed; scrolled back, the reading stays put and Jump to new response appears. What the person sends always brings them to the end.",
                    ViewportDemo {}
                }
                Section { title: "Feedback and undo", note: "The rating pair shows while its response is hovered or focused, and always on touch; reasons are toggle chips. Undo floats at the window's foot and pauses while pointed at.",
                    FeedbackDemo {}
                }
                Section { title: "Sheets and dialogs", note: "A sheet rises on the step spring and can open over another; a dialog rises the same way. Escape, the scrim or Close dismisses the top one. Drag a sheet down by its header, or by its content from the top: past half its height or flicked, it goes; otherwise it springs back.",
                    OverlayDemo {}
                }
                Section { title: "Working and loading", note: "The working mark while work runs, the live dot, a placeholder in the shape of what loads (a line, list items, rows or reading), and notices for reads and commands.",
                    LoadingDemo {}
                }
                Section { title: "Live lists", note: "A row that joins a list already shown opens and settles from just below; a row the list drops stays in its place as it collapses, then goes. A count fades into its new value, number and words together. A row the list moves up rises over the others, lifted; the selection is one fill that slides to the row chosen and travels with it. With a keyboard, all of it simply arrives.",
                    LiveDemo {}
                }
                Section { title: "Empty", note: "An empty section is one plain line. An empty or caught-up place has its mark, one line, what to know, and up to two quiet ways on: the second is the way out when the first may never succeed.",
                    EmptyDemo {}
                }
                Section { title: "Citations and sources", note: "Pointing at a citation previews its source beside it; choosing it opens the source beside the reader, or over it with Back where there isn't room. A citation without a source stays a mark; one whose source can't be shown is muted and says so. A source opens at the passage it was cited for, marked.",
                    CitationDemo {}
                }
                Section { title: "Rows and sections", note: "Related rows group once, a hairline between them, rather than a card around each. A row leads with an icon, a number or a person's avatar, in three sizes. A section's one action sits beside its title.",
                    RowsDemo {}
                }
                Section { title: "Buttons and navigation", note: "One shape in three variants, with the shared hover, press, focus and touch target.",
                    ButtonDemo {}
                }
                Section { title: "Duet's theme", note: "The same pieces under theme-duet: prose at body large, each action a full-width bar, quiet actions and links in green, cards on a hairline, and the person's bubble tailed toward them.",
                    ThemeDemo {}
                }
                Section { title: "Disclosure", note: "Rows open and close on a height transition; the chevron turns.",
                    Disclosure { title: "What Duet used", default_open: true,
                        p { class: "gallery-copy", "Sleep note from Tuesday, and seven nights of sleep from Apple Health." }
                    }
                    Disclosure { title: "Basis and limitations", detail: true,
                        p { class: "gallery-copy", "Coverage is seven of the last ten nights." }
                    }
                }
                Section { title: "State pills", note: "What a state chip's colour says.",
                    div { class: "gallery-row",
                        StatePill { "Neutral" }
                        StatePill { tone: Tone::Primary, icon: IconName::Check, "Saved" }
                        StatePill { tone: Tone::Warning, icon: IconName::Flag, "Needs you" }
                        StatePill { tone: Tone::Error, icon: IconName::Alert, "Couldn’t finish" }
                    }
                }
                Section { title: "Fields and tabs", note: "The text field carries focus on its outline. The search field leads with its mark, which darkens while it has focus, and clears itself without losing focus. A labelled field names what it holds; with several lines, it grows with what is written.",
                    FieldsDemo {}
                    TabsDemo {}
                }
            }
        }
    }
}

#[cfg(feature = "web")]
const TYPE_ROLES: [(&str, &str, &str); 14] = [
    ("headline-md", "Headline", "Today"),
    (
        "title-lg",
        "Title large",
        "Sleep was shorter than your recent average",
    ),
    ("title-md", "Title medium", "Your measurements"),
    ("title-sm", "Title small", "Visit note"),
    (
        "body-lg",
        "Body large",
        "A short walk after dinner keeps the evening steady.",
    ),
    ("body-md", "Body medium", "An update is ready to review."),
    ("body-sm", "Body small", "Northside Clinic · October 15"),
    ("label-lg", "Label large", "Save plan"),
    ("label-md", "Label medium", "Needs you"),
    ("label-sm", "Label small", "Today"),
    ("value-lg", "Value large", "6 hr 18 min"),
    ("value-md", "Value medium", "4,280"),
    ("code", "Code", "PHYSICIAN_APP_URL"),
    ("label-sm", "Overline", "What Duet noticed"),
];

#[cfg(feature = "web")]
#[component]
fn TypeDemo() -> Element {
    rsx! {
        dl { class: "gallery-specimens",
            for (role, name, sample) in TYPE_ROLES {
                div { class: "gallery-specimen",
                    dt { class: "gallery-note", "{name} · --type-{role}" }
                    if name == "Overline" {
                        dd { Overline { text: sample } }
                    } else {
                        dd { style: "font: var(--type-{role})", "{sample}" }
                    }
                }
            }
        }
    }
}

#[cfg(feature = "web")]
const COLOUR_ROLES: [(&str, &str); 16] = [
    ("surface", "ink"),
    ("surface-lowest", "ink"),
    ("surface-low", "ink"),
    ("surface-high", "ink"),
    ("surface-highest", "ink"),
    ("primary", "on-primary"),
    ("primary-container", "on-primary-container"),
    ("highlight", "ink"),
    ("warning-container", "on-warning-container"),
    ("error-container", "on-error-container"),
    ("error", "on-error"),
    ("tertiary", "on-primary"),
    ("ink", "surface-lowest"),
    ("body", "surface-lowest"),
    ("muted", "surface-lowest"),
    ("outline", "ink"),
];

#[cfg(feature = "web")]
#[component]
fn ColourDemo() -> Element {
    rsx! {
        div { class: "gallery-swatches",
            for (fill, on) in COLOUR_ROLES {
                div { class: "gallery-swatch", style: "background: var(--{fill}); color: var(--{on})",
                    strong { "{fill}" }
                    span { "on {on}" }
                }
            }
        }
    }
}

#[cfg(feature = "web")]
#[component]
fn IconDemo() -> Element {
    rsx! {
        ul { class: "gallery-icons",
            for icon in IconName::ALL {
                li { Icon { name: icon } span { "{icon.name()}" } }
            }
        }
    }
}

#[cfg(feature = "web")]
#[component]
fn CardDemo() -> Element {
    rsx! {
        div { class: "gallery-cards",
            Card {
                overline: "What Duet noticed",
                title: "Sleep was shorter than your recent average",
                footer: "42 min less · Apple Health",
                footer_icon: IconName::Heart,
                onclick: move |_| {},
                ComparisonBars {
                    rows: vec![
                        Comparison { label: "Last night".into(), value: "6 hr 18 min".into(), fraction: 0.9 },
                        Comparison { label: "Previous 7 nights".into(), value: "7 hr".into(), fraction: 1.0 },
                    ],
                }
            }
            Card {
                tinted: true,
                overline: "Your saved plan",
                title: "Evening wind-down",
                footer: "Saved in chat · October 21",
                footer_icon: IconName::Message,
                on_footer: move |_| {},
                StepList {
                    steps: vec![
                        "Put my phone away at 10:30 pm.".into(),
                        "Keep my usual wake-up time.".into(),
                        "Try this through Sunday.".into(),
                    ],
                }
            }
            Card {
                overline: "Original record",
                title: "Visit note",
                detail: "Northside Clinic · October 15",
                Quote { text: "Walks twice a week." }
                LaterContext { overline: "Your later context", text: "I now walk four mornings a week.", added: "Added October 21" }
            }
        }
    }
}

#[cfg(feature = "web")]
const SLEEP_HOURS: [f64; 8] = [7.1, 7.4, 7.2, 6.9, 7.0, 7.3, 6.8, 6.3];

#[cfg(feature = "web")]
fn parts(parts: &[(&str, &str)]) -> Vec<(String, String)> {
    parts
        .iter()
        .map(|(figure, unit)| (figure.to_string(), unit.to_string()))
        .collect()
}

#[cfg(feature = "web")]
#[component]
fn ChartDemo() -> Element {
    rsx! {
        div { class: "gallery-tiles",
            MeasurementTile {
                icon: IconName::Moon,
                label: "Sleep",
                value: parts(&[("6", "hr"), ("18", "min")]),
                detail: "Last night · Apple Health",
                points: SLEEP_HOURS.to_vec(),
                trend: "Sleep over the last eight nights, from 7.1 to 6.3 hours",
                onclick: move |_| {},
            }
            MeasurementTile {
                icon: IconName::Steps,
                label: "Steps",
                value: parts(&[("4,280", "")]),
                detail: "Today · Apple Health",
                points: vec![3100.0, 3600.0, 3300.0, 4000.0, 3800.0, 4300.0, 4100.0, 4280.0],
                trend: "Steps over the last eight days, from 3,100 to 4,280",
            }
        }
        div { class: "gallery-measure",
            p { class: "gallery-note", "Sleep · Last night" }
            Value { parts: parts(&[("6", "hr"), ("18", "min")]), large: true }
            SeriesChart {
                shape: SeriesShape::Line,
                points: SLEEP_HOURS
                    .iter()
                    .enumerate()
                    .map(|(night, hours)| SeriesPoint {
                        value: Some(*hours),
                        axis: match night {
                            0 => "Oct 14".into(),
                            7 => "Oct 21".into(),
                            _ => String::new(),
                        },
                        ..SeriesPoint::default()
                    })
                    .collect::<Vec<_>>(),
                label: "Sleep each night, October 14 to 21: from 7.1 hours to 6.3 last night",
                emphasized: Some((0, 7)),
            }
        }
        div { class: "gallery-row",
            span { class: "gallery-label", "Sparkline alone" }
            span { class: "gallery-spark", Sparkline { points: vec![2.0, 3.0, 2.5, 4.0, 3.5, 5.0], label: "Rising over six days" } }
        }
    }
}

#[cfg(feature = "web")]
#[component]
fn Section(title: String, note: String, children: Element) -> Element {
    rsx! {
        section { class: "gallery-section card",
            h2 { class: "gallery-title", "{title}" }
            p { class: "gallery-note", "{note}" }
            div { class: "gallery-stage", {children} }
        }
    }
}

#[cfg(feature = "web")]
#[derive(Clone, PartialEq)]
struct Line {
    person: bool,
    text: String,
    sources: bool,
}

#[cfg(feature = "web")]
#[component]
fn ConversationDemo() -> Element {
    let mut lines = use_signal(|| {
        vec![
            Line {
                person: true,
                text: "How did I sleep?".into(),
                sources: false,
            },
            Line {
                person: false,
                text: "About seven hours, close to your usual week.".into(),
                sources: true,
            },
        ]
    });
    let mut draft = use_signal(String::new);
    let mut typing = use_signal(|| false);
    let mut sources_open = use_signal(|| false);
    let mut source_open = use_signal(|| false);
    let send = move |text: String| {
        let text = if text.trim().is_empty() {
            "Was that more than last week?".to_owned()
        } else {
            text
        };
        lines.write().push(Line {
            person: true,
            text,
            sources: false,
        });
        draft.set(String::new());
        typing.set(true);
        spawn(async move {
            futures_timer::Delay::new(Duration::from_millis(1800)).await;
            typing.set(false);
            lines.write().push(Line {
                person: false,
                text: "A little more: six nights over seven hours, against four the week before."
                    .into(),
                sources: true,
            });
        });
    };
    rsx! {
        div { class: "gallery-thread",
            for (index, line) in lines().into_iter().enumerate() {
                Message {
                    key: "{index}",
                    author: if line.person { Author::Person } else { Author::Assistant },
                    by: if line.person { "You" } else { "Duet" },
                    p { "{line.text}" }
                    if line.sources {
                        SourcesButton { label: "Sources", count: 2, onclick: move |_| sources_open.set(true) }
                    }
                }
            }
            if typing() {
                Message { author: Author::Assistant,
                    span { class: "sr-only", role: "status", "Duet is responding" }
                    WorkingMark {}
                }
            }
        }
        Composer {
            label: "Message",
            placeholder: "Write a message, or just press Send",
            value: draft(),
            on_input: move |text| draft.set(text),
            on_submit: send,
            Button { variant: ButtonVariant::Primary, compact: true, submit: true, disabled: typing(), "Send" }
        }
        if sources_open() {
            Dialog { close_label: "Close", sheet: true, title: "Sources", on_close: move |_| sources_open.set(false),
                ol { class: "row-list",
                    li { ListRow { title: "Sleep note", detail: "Tuesday", onclick: move |_| source_open.set(true) } }
                    li { ListRow { title: "Sleep, Apple Health", detail: "Last 7 nights", onclick: move |_| source_open.set(true) } }
                }
            }
        }
        if source_open() {
            Dialog { close_label: "Close", sheet: true, title: "Sleep note", on_close: move |_| source_open.set(false),
                p { class: "gallery-copy", "About seven hours, from the sleep note." }
            }
        }
    }
}

#[cfg(feature = "web")]
#[component]
fn FieldsDemo() -> Element {
    let mut title = use_signal(|| "Questions for my visit".to_string());
    let mut discuss = use_signal(|| {
        "I’d like to understand why I’m so tired in the afternoon. It’s getting in the way of work."
            .to_string()
    });
    let mut search = use_signal(|| "Delgado".to_string());
    rsx! {
        div { class: "gallery-fields",
            SearchField {
                label: "Search your panel",
                placeholder: "Name, DOB or phone",
                clear_label: "Clear search",
                value: search(),
                on_input: move |words| search.set(words),
            }
            TextField { label: "Title", value: title(), on_input: move |words| title.set(words) }
            TextField { label: "What I want to discuss", multiline: true, value: discuss(), on_input: move |words| discuss.set(words) }
        }
    }
}

#[cfg(feature = "web")]
#[component]
fn ReviewDemo() -> Element {
    rsx! {
        div { class: "gallery-phone",
            TopBar { title: "Review update", back: "Back", on_back: move |_| {}, action: "Edit", on_action: move |_| {} }
            div { class: "gallery-phone-thread",
                PageTitle { text: "Here’s what changed.", detail: "Proposed by Duet · Not saved" }
                div {
                    ProposedChange {
                        title: "What you’ve tried",
                        tag: "Updated",
                        previous: ("Previously saved".to_owned(), "Since October 16, Alex has stopped drinking coffee after noon.".to_owned()),
                        sources: rsx! {
                            NavigationControl { Icon { name: IconName::File } "Your note · October 21" }
                        },
                        p { "Since October 16, Alex has stopped drinking coffee after noon. On October 21, Alex reported that the afternoon tiredness had not improved." }
                    }
                    ProposedChange {
                        title: "Recorded sleep",
                        tag: "Through Oct 21",
                        sources: rsx! {
                            NavigationControl { Icon { name: IconName::Heart } "Apple Health · October 14–21" }
                        },
                        p { "Apple Watch recorded 6 hr 18 min on October 20–21, after an average of 7 hr per night across October 14–20 (7 nights)." }
                    }
                    ProposedChange { title: "What I want to discuss", unchanged: "Unchanged",
                        p { "I’d like to understand why I’m so tired in the afternoon. It’s getting in the way of work." }
                    }
                }
            }
            FooterActions { note: "Saving does not send this page.",
                Button { variant: ButtonVariant::Primary, "Save updated page" }
            }
        }
    }
}

#[cfg(feature = "web")]
#[component]
fn HeadingsDemo() -> Element {
    rsx! {
        div { class: "gallery-headings",
            PageTitle { text: "Good morning, Alex", eyebrow: "Wednesday, October 21" }
            PageTitle {
                text: "You",
                actions: rsx! {
                    Button { variant: ButtonVariant::Quiet, icon_only: true, aria_label: "Settings", Icon { name: IconName::Settings } }
                },
            }
            PageTitle { text: "For my doctor", eyebrow: "Your continuing page", detail: "Saved October 21 · Not sent" }
            PageTitle { text: "Visit note", icon: IconName::File, detail: "Northside Clinic · October 15, 2026" }
            PageTitle { text: "Here’s what changed.", detail: "Proposed by Duet · Not saved" }
        }
    }
}

#[cfg(feature = "web")]
#[component]
fn PagesDemo() -> Element {
    let mut sleep = use_signal(|| true);
    let mut visit = use_signal(|| false);
    let mut reminder = use_signal(|| true);
    rsx! {
        div { class: "gallery-phone",
            TopBar { title: "Review update", back: "Back", on_back: move |_| {}, action: "Edit", on_action: move |_| {} }
            div { class: "gallery-phone-thread",
                Notice { tone: Tone::Primary, icon: IconName::Check, title: "Page saved", detail: "This update has not been shared." }
                div { class: "gallery-row",
                    Tag { text: "Updated" }
                    Tag { text: "Through Oct 21" }
                }
                Paper { title: "For my doctor", detail: "Your continuing page · saved October 21",
                    PaperSection { heading: "What I want to discuss",
                        p { "I’d like to understand why I’m so tired in the afternoon. It’s getting in the way of work." }
                    }
                    PaperSection { heading: "What I’ve tried",
                        p { "Since October 16 I’ve stopped drinking coffee after noon." }
                    }
                }
                ul { class: "row-list",
                    li { SelectionRow { title: "Recorded sleep", detail: "Apple Health · October 14–21", checked: sleep(), on_change: move |on| sleep.set(on) } }
                    li { SelectionRow { title: "Visit note", detail: "Northside Clinic · October 15", checked: visit(), on_change: move |on| visit.set(on) } }
                    li { SettingRow { title: "Remind me", detail: "Daily · 10:30 pm", on: reminder(), on_change: move |on| reminder.set(on) } }
                }
                Notice { tone: Tone::Error, icon: IconName::Alert, title: "Your changes weren’t saved", detail: "Your draft is still here.",
                    Button { compact: true, "Try saving again" }
                }
                Notice { tone: Tone::Warning, title: "Only three nights recorded", detail: "These records don’t cover the whole week." }
                Notice { title: "Existing care stays useful", detail: "Your saved plan and upcoming check-in are here when you need them." }
            }
            FooterActions { note: "Saving does not share this page.",
                Button { variant: ButtonVariant::Primary, "Save updated page" }
            }
        }
    }
}

#[cfg(feature = "web")]
#[component]
fn FrameDemo() -> Element {
    let mut attached = use_signal(|| true);
    let mut draft = use_signal(String::new);
    rsx! {
        div { class: "gallery-phone",
            TopBar {
                title: "Sprout",
                mark: rsx! { span { class: "gallery-companion", aria_hidden: "true" } },
                back: "Back",
                on_back: move |_| {},
            }
            div { class: "gallery-phone-thread",
                DateSeparator { date: "Wednesday, October 21" }
                Message { author: Author::Assistant, p { "What would you like to work through?" } }
                Suggestion { text: "Help me keep a simple wind-down plan for this week.", onclick: move |_| {} }
                Message { author: Author::Assistant,
                    p { "Saved. You can change it any time from Today." }
                    RowGroup {
                        li { ListRow { icon: IconName::Check, title: "Evening wind-down", detail: "Plan saved · open the current version", onclick: move |_| {} } }
                    }
                }
            }
            Composer {
                label: "Message Sprout",
                inline: true,
                placeholder: "Message Sprout",
                value: draft(),
                on_input: move |text| draft.set(text),
                on_submit: move |_| draft.set(String::new()),
                context: rsx! {
                    if attached() {
                        ContextChip {
                            icon: IconName::File,
                            text: "About: Sleep · October 14–21 · Apple Health",
                            remove_label: "Remove Sleep from this message",
                            on_remove: move |_| attached.set(false),
                        }
                    }
                },
                Button { variant: ButtonVariant::Primary, compact: true, icon_only: true, submit: true, aria_label: "Send", Icon { name: IconName::ArrowUp } }
            }
        }
    }
}

#[cfg(feature = "web")]
#[component]
fn OverlayDemo() -> Element {
    let mut dialog = use_signal(|| false);
    let mut sheet = use_signal(|| false);
    let mut over = use_signal(|| false);
    rsx! {
        ActionRow {
            Button { onclick: move |_| dialog.set(true), "Open a dialog" }
            Button { onclick: move |_| sheet.set(true), "Open a sheet" }
        }
        if dialog() {
            Dialog { close_label: "Close", title: "Past work", subtitle: "Mary Louis", on_close: move |_| dialog.set(false),
                p { class: "gallery-copy", "A dialog rises near the top of the window over the scrim, with focus on its title." }
            }
        }
        if sheet() {
            Dialog { close_label: "Close", sheet: true, title: "Sources", on_close: move |_| sheet.set(false),
                p { class: "gallery-copy", "A sheet runs along the bottom edge." }
                Button { onclick: move |_| over.set(true), "Open another over it" }
            }
        }
        if over() {
            Dialog { close_label: "Close", sheet: true, title: "Sleep note", on_close: move |_| over.set(false),
                p { class: "gallery-copy", "Closing this returns to the sheet beneath." }
            }
        }
    }
}

#[cfg(feature = "web")]
#[component]
fn ComposerStatesDemo() -> Element {
    let mut next = use_signal(|| "Also check her potassium trend.".to_string());
    let mut mine = use_signal(|| "Summarize the cardiology note for her visit.".to_string());
    rsx! {
        div { class: "gallery-composers",
            Composer {
                label: "Message Coda",
                value: next(),
                on_input: move |text| next.set(text),
                on_submit: move |_| {},
                status: rsx! { WorkingPresence { label: "Reviewing the chart", compact: true, announce: false, hint: "Started 9:41 AM" } },
                Button { variant: ButtonVariant::Quiet, "Stop" }
                Button { "Update response" }
                Button { variant: ButtonVariant::Primary, submit: true, "Send next", KeyHint { key_label: "↵" } }
            }
            Composer {
                label: "Message Coda",
                value: mine(),
                on_input: move |text| mine.set(text),
                on_submit: move |_| {},
                status: rsx! { span { "Draft changed in another window" } },
                notices: rsx! {
                    DraftConflict {
                        message: "Your text is still in the composer. Choose which draft to keep.",
                        label: "Saved draft",
                        saved: "Summarize the cardiology note and the latest echo for her visit.",
                        ActionRow {
                            Button { "Keep my text" }
                            Button { "Use saved text" }
                        }
                    }
                },
                Button { variant: ButtonVariant::Primary, submit: true, disabled: true, "Send", KeyHint { key_label: "↵" } }
            }
            WorkingPresence { label: "Reviewing the chart", activity: "Reading the cardiology note", announce: false }
        }
    }
}

#[cfg(feature = "web")]
const LIVE: [(&str, &str); 5] = [
    (
        "Potassium 5.4 mmol/L",
        "Basic metabolic panel · Today, 8:00 AM",
    ),
    (
        "Refill request: lisinopril",
        "Patient message · Today, 7:42 AM",
    ),
    ("Echo report signed", "Cardiology · Yesterday"),
    ("Visit tomorrow, 9:30 AM", "Follow-up · Dr. Osei"),
    ("Prior authorization approved", "Insurance · Monday"),
];

#[cfg(feature = "web")]
#[component]
fn LiveDemo() -> Element {
    let mut shown = use_signal(|| vec![0usize, 1, 2]);
    let mut selected = use_signal(|| Some(0usize));
    let live = use_live_rows(shown(), |index| index.to_string(), |_| true);
    let on_exited = live.on_exited;
    rsx! {
        div { class: "gallery-rows",
            div { class: "title-with-count",
                h3 { class: "section-title", "Needs you" }
                Count { value: shown().len(), words: "to review" }
            }
            RowGroup { id: live.list, class: "selection-host",
                for entry in live.listed {
                    RowFrame {
                        key: "{entry.key}",
                        present: entry.present,
                        arriving: entry.arriving,
                        on_exited: {
                            let key = entry.key.clone();
                            move |_| on_exited.call(key.clone())
                        },
                        ListRow {
                            icon: IconName::File,
                            title: LIVE[entry.row].0,
                            detail: LIVE[entry.row].1,
                            aria_current: (selected() == Some(entry.row)).then_some("true"),
                            onclick: move |_| selected.set(Some(entry.row)),
                        }
                    }
                }
            }
            ActionRow {
                Button {
                    disabled: shown().len() == LIVE.len(),
                    onclick: move |_| {
                        let next = (0..LIVE.len()).find(|index| !shown().contains(index));
                        if let Some(next) = next {
                            let at = 1.min(shown().len());
                            shown.write().insert(at, next);
                        }
                    },
                    "Add a row"
                }
                Button {
                    disabled: shown().is_empty(),
                    onclick: move |_| {
                        let at = shown().len() / 2;
                        shown.write().remove(at);
                    },
                    "Remove a row"
                }
                Button {
                    disabled: shown().len() < 2,
                    onclick: move |_| {
                        let last = shown.write().pop();
                        if let Some(last) = last {
                            shown.write().insert(0, last);
                        }
                    },
                    "Move the last to the top"
                }
            }
        }
    }
}

#[cfg(feature = "web")]
const WEEK: [&str; 7] = [
    "Saturday",
    "Sunday",
    "Monday",
    "Tuesday",
    "Wednesday",
    "Thursday",
    "Friday",
];

#[cfg(feature = "web")]
#[component]
fn SeriesDemo() -> Element {
    let steps = [6200.0, 8400.0, 7100.0, 9800.0, 5400.0, 10200.0, 4300.0];
    let resting = [
        Some(62.0),
        Some(64.0),
        Some(61.0),
        None,
        Some(66.0),
        Some(60.0),
        Some(62.0),
    ];
    let mut pinned_steps = use_signal(|| None::<usize>);
    let mut pinned_rest = use_signal(|| None::<usize>);
    let day = |index: usize| {
        WEEK[index]
            .chars()
            .next()
            .map(String::from)
            .unwrap_or_default()
    };
    let step_points: Vec<SeriesPoint> = steps
        .iter()
        .enumerate()
        .map(|(index, value)| SeriesPoint {
            value: Some(*value),
            axis: day(index),
            words: format!("{}, {} steps", WEEK[index], *value as u32),
            ..SeriesPoint::default()
        })
        .collect();
    let rest_points: Vec<SeriesPoint> = resting
        .iter()
        .enumerate()
        .map(|(index, value)| SeriesPoint {
            value: *value,
            axis: day(index),
            words: value.map_or_else(String::new, |bpm| {
                format!("{}, {bpm} beats per minute", WEEK[index])
            }),
            ..SeriesPoint::default()
        })
        .collect();
    let readout = |pinned: Option<usize>, points: &[SeriesPoint], idle: &str| {
        pinned
            .and_then(|index| points.get(index))
            .map_or_else(|| idle.to_owned(), |point| point.words.clone())
    };
    rsx! {
        div { class: "gallery-series",
            p { class: "gallery-note", "{readout(pinned_steps(), &step_points, \"Steps · this week, today so far\")}" }
            SeriesChart {
                points: step_points.clone(),
                label: "Steps each day this week, averaging 7,343; 4,300 today so far",
                reference: 7343.0,
                reference_label: "avg",
                selected: pinned_steps(),
                on_select: move |next| pinned_steps.set(next),
            }
            p { class: "gallery-note", "{readout(pinned_rest(), &rest_points, \"Resting heart rate · this week\")}" }
            SeriesChart {
                shape: SeriesShape::Line,
                points: rest_points.clone(),
                label: "Resting heart rate each day this week, 60 to 66 beats per minute, nothing on Tuesday",
                selected: pinned_rest(),
                on_select: move |next| pinned_rest.set(next),
            }
        }
    }
}

#[cfg(feature = "web")]
#[component]
fn EmptyDemo() -> Element {
    rsx! {
        div { class: "gallery-empty",
            EmptyLine { message: "Nothing shared yet." }
            EmptyState {
                icon: IconName::File,
                title: "This record is no longer available",
                detail: "It may have been removed or replaced by a correction.",
            }
            EmptyState {
                icon: IconName::Refresh,
                title: "Couldn’t check your account",
                detail: "The connection was lost.",
                action: "Try again",
                on_action: move |_| {},
                secondary: "Sign out",
                on_secondary: move |_| {},
            }
        }
    }
}

#[cfg(feature = "web")]
#[component]
fn LoadingDemo() -> Element {
    rsx! {
        div { class: "gallery-row",
            span { class: "gallery-label", WorkingMark {} "Preparing the visit" }
            span { class: "gallery-label", LiveDot {} "New" }
        }
        div { class: "gallery-placeholder",
            Placeholder {}
            Placeholder { shape: PlaceholderShape::Items }
            Placeholder { shape: PlaceholderShape::Rows }
            Placeholder { shape: PlaceholderShape::Reading }
        }
        StatusLine { message: "Reading records…", pending: true }
        StatusLine { message: "Couldn’t refresh. Showing what was read at 9:41.", retry: "Try again", on_retry: move |_| {} }
        StatusLine { message: "Couldn’t confirm it was saved.", retry: "Check again", on_retry: move |_| {} }
    }
}

#[cfg(feature = "web")]
#[derive(Clone, Copy, PartialEq)]
struct Cited {
    index: usize,
    title: &'static str,
    metadata: [&'static str; 3],
    passage: &'static str,
    content: &'static str,
}

#[cfg(feature = "web")]
const CITED: [Cited; 2] = [
    Cited {
        index: 1,
        title: "Discharge summary",
        metadata: ["Hospital course", "April 18", "Final"],
        passage: "Admitted with decompensated heart failure; diuresed 6 kg.",
        content: "Hospital course.\nPresented to the emergency department with three days of worsening dyspnea, orthopnea and leg swelling. Chest radiograph showed pulmonary vascular congestion and small bilateral effusions. BNP was elevated on arrival. Admitted with decompensated heart failure; diuresed 6 kg. Renal function stayed stable through diuresis. Echocardiogram showed an ejection fraction of 35 percent. Discharged on furosemide 40 mg daily with follow-up in cardiology clinic within two weeks.",
    },
    Cited {
        index: 2,
        title: "Cardiology note",
        metadata: ["Assessment", "October 2", "Signed"],
        passage: "NYHA II–III, mild congestion on exam.",
        content: "Assessment. NYHA II–III, mild congestion on exam. Continue spironolactone 25 mg daily. Repeat potassium in one week.",
    },
];

#[cfg(feature = "web")]
#[component]
fn CitedDetails(cited: Cited) -> Element {
    // A source's words arrive a moment after it opens, as a read does; its placeholder holds their place.
    let mut read = use_signal(|| false);
    use_hook(|| {
        spawn(async move {
            futures_timer::Delay::new(Duration::from_millis(600)).await;
            read.set(true);
        })
    });
    if !read() {
        return rsx! { Placeholder { shape: PlaceholderShape::Reading } };
    }
    rsx! {
        SourceDetails {
            metadata: cited.metadata.iter().map(|value| value.to_string()).collect::<Vec<_>>(),
            passages: vec![cited.passage.to_owned()],
            passages_label: "Quoted source passages",
            p { class: "gallery-copy",
                SourceText {
                    text: cited.content,
                    passages: cited
                        .content
                        .find(cited.passage)
                        .map(|start| (start, start + cited.passage.len()))
                        .into_iter()
                        .collect::<Vec<_>>(),
                }
            }
        }
    }
}

#[cfg(feature = "web")]
#[component]
fn CitationDemo() -> Element {
    let mut open = use_signal(|| None::<Cited>);
    let source = open().map(|cited| SourcePane {
        eyebrow: format!("Source {}", cited.index),
        title: cited.title.to_owned(),
        content: rsx! { CitedDetails { cited } },
        back: "Back".to_owned(),
        close: "Close source".to_owned(),
    });
    rsx! {
        div { class: "gallery-reading",
            SourceReading { source, on_close: move |_| open.set(None),
                div { class: "gallery-reader",
                    p { class: "gallery-copy",
                        "Following an April admission with decompensated heart failure, she remained NYHA II–III with mild congestion."
                        CitationPreview {
                            index: 1,
                            label: CITED[0].title,
                            eyebrow: "Source 1",
                            citation_label: "Source 1: {CITED[0].title}. Preview source.",
                            close_label: "Close source preview",
                            on_expand: move |_| open.set(Some(CITED[0])),
                            expand_label: "Expand",
                            original: "https://example.org/records/discharge",
                            original_label: "Open in Epic",
                            new_tab_hint: "Opens in a new tab",
                            CitedDetails { cited: CITED[0] }
                        }
                        CitationPreview {
                            index: 2,
                            label: CITED[1].title,
                            eyebrow: "Source 2",
                            citation_label: "Source 2: {CITED[1].title}. Preview source.",
                            close_label: "Close source preview",
                            on_expand: move |_| open.set(Some(CITED[1])),
                            expand_label: "Expand",
                            CitedDetails { cited: CITED[1] }
                        }
                        " Genetic testing was not yet back."
                        CitationChip { index: 3, label: "Source 3 unavailable", unavailable: "unavailable" }
                    }
                }
            }
        }
    }
}

#[cfg(feature = "web")]
#[component]
fn ButtonDemo() -> Element {
    rsx! {
        div { class: "gallery-row",
            Button { variant: ButtonVariant::Primary, "Save plan" }
            Button { "Change time" }
            Button { variant: ButtonVariant::Quiet, "Not now" }
            Button { danger: true, "Remove" }
            Button { disabled: true, "Disabled" }
            Button { compact: true, "Compact" }
            BusyDemo {}
            Button { variant: ButtonVariant::Quiet, icon_only: true, aria_label: "Close", Icon { name: IconName::X } }
        }
        div { class: "gallery-row",
            NavigationControl { "Open original" }
        }
        NavigationRow { class: "gallery-nav-row",
            strong { "Evening walk" }
            span { class: "gallery-note", "Every evening" }
            Icon { name: IconName::ArrowRight }
        }
    }
}

/// The prose specimen, shared with the React side-by-side.
#[cfg(feature = "web")]
const PROSE_BEFORE: &str = "## Recorded measurements\n\nPotassium was **5.4 mmol/L** on Tuesday";
#[cfg(feature = "web")]
const PROSE_AFTER: &str = ", up from 4.9 in September. Confirm the current reading with the patient.\n\n- Lisinopril 10 mg daily\n- Spironolactone 25 mg, *held* since October 2\n\n> Feels well, with no palpitations.\n\n| Date | Potassium |\n| --- | --- |\n| September 12 | 4.9 |\n| October 28 | 5.4 |\n\nRecheck with a `BMP` next week.";

#[cfg(feature = "web")]
#[component]
fn ProseDemo() -> Element {
    rsx! {
        div { class: "gallery-prose",
            Prose {
                parts: vec![
                    ProsePart::Words(PROSE_BEFORE.to_owned()),
                    ProsePart::Inline(rsx! { CitationChip { index: 1, label: "Source 1: Basic metabolic panel" } }),
                    ProsePart::Words(PROSE_AFTER.to_owned()),
                ],
            }
        }
    }
}

#[cfg(feature = "web")]
#[component]
fn ThemeDemo() -> Element {
    rsx! {
        div { class: "theme-duet gallery-theme",
            Message { author: Author::Person, by: "You", p { "Can we keep it simple this week?" } }
            Message { author: Author::Assistant, by: "Duet", p { "Yes: phone away at 10:30, and the same wake-up time." } }
            Card { overline: "Your saved plan", title: "Evening wind-down", footer: "Saved in chat · October 21",
                StepList { steps: vec!["Put my phone away at 10:30 pm.".into(), "Keep my usual wake-up time.".into()] }
            }
            NavigationControl { "Open original" }
            Button { variant: ButtonVariant::Primary, "Save plan" }
            Button { variant: ButtonVariant::Quiet, "Not now" }
        }
    }
}

#[cfg(feature = "web")]
#[component]
fn TabsDemo() -> Element {
    let mut selected = use_signal(|| 0usize);
    let mut phone = use_signal(|| 1usize);
    rsx! {
        TabBar {
            label: "Sections",
            labels: vec!["Duet".into(), "Today".into(), "You".into()],
            selected: selected(),
            on_select: move |index| selected.set(index),
        }
        div { class: "gallery-phone-tabs",
            TabBar {
                label: "Sections",
                labels: vec!["Duet".into(), "Today".into(), "You".into()],
                icons: vec![IconName::Message, IconName::Sun, IconName::User],
                selected: phone(),
                on_select: move |index| phone.set(index),
            }
        }
    }
}

#[cfg(feature = "web")]
#[component]
fn RowsDemo() -> Element {
    rsx! {
        div { class: "gallery-rows",
            SectionHeader { title: "For my doctor" }
            RowGroup {
                li { ListRow { icon: IconName::File, title: "My page for my doctor", detail: "An update is ready to review", onclick: move |_| {} } }
            }
            SectionHeader { title: "Records", action: "See all", on_action: move |_| {} }
            RowGroup {
                li { ListRow { icon: IconName::File, title: "Visit note", detail: "Northside Clinic · October 15", onclick: move |_| {} } }
                li { ListRow { icon: IconName::Heart, title: "Blood pressure", detail: "Home cuff · October 20", onclick: move |_| {} } }
                li { ListRow { icon: IconName::Book, title: "Questions for my visit", detail: "Your note · October 21", onclick: move |_| {} } }
            }
            SectionHeader { title: "People" }
            RowGroup {
                li { ListRow { person: "Maria Delgado", title: "Maria Delgado", detail: "DOB March 4, 1961", onclick: move |_| {} } }
                li { ListRow { person: "James O’Neill", title: "James O’Neill", detail: "DOB not recorded", onclick: move |_| {} } }
            }
            div { class: "gallery-row",
                Avatar { name: "Maria Delgado" }
                Avatar { name: "Maria Delgado", size: AvatarSize::Medium }
                Avatar { name: "Maria Delgado", size: AvatarSize::Large }
            }
            SectionHeader { title: "Sources, numbered" }
            ol { class: "row-list",
                li { ListRow { number: 1, title: "Discharge summary", detail: "Read Today, 8:00 AM", onclick: move |_| {} } }
                li { ListRow { number: 2, title: "Cardiology note", onclick: move |_| {} } }
            }
        }
    }
}

#[cfg(feature = "web")]
#[component]
fn BusyDemo() -> Element {
    let mut busy = use_signal(|| false);
    rsx! {
        Button {
            busy: busy(),
            pending_label: "Saving plan",
            onclick: move |_| {
                busy.set(true);
                spawn(async move {
                    futures_timer::Delay::new(Duration::from_millis(1600)).await;
                    busy.set(false);
                });
            },
            Icon { name: IconName::Check }
            "Save plan"
        }
        Button { variant: ButtonVariant::Primary, busy: true, pending_label: "Sending", Icon { name: IconName::ArrowUp } "Send" }
    }
}

#[cfg(feature = "web")]
const REPLIES: [&str; 4] = [
    "Her potassium was 5.4 on Tuesday, up from 4.9 in September.",
    "The cardiology note from October 2 kept spironolactone at 25 mg daily.",
    "No repeat level is on file since the dose change.",
    "Her creatinine held at 1.3 across the same draws.",
];

#[cfg(feature = "web")]
#[component]
fn ViewportDemo() -> Element {
    let mut lines = use_signal(|| {
        (0..6)
            .map(|index| Line {
                person: index % 2 == 0,
                text: if index % 2 == 0 {
                    "What changed since her last visit?".to_owned()
                } else {
                    REPLIES[index / 2 % REPLIES.len()].to_owned()
                },
                sources: false,
            })
            .collect::<Vec<_>>()
    });
    let replies = lines.read().iter().filter(|line| !line.person).count();
    let sent = lines.read().iter().filter(|line| line.person).count();
    rsx! {
        div { class: "gallery-viewport",
            ConversationViewport { replies, sent, jump_label: "Jump to new response",
                div { class: "gallery-thread",
                    for (index, line) in lines().into_iter().enumerate() {
                        Message {
                            key: "{index}",
                            author: if line.person { Author::Person } else { Author::Assistant },
                            by: if line.person { "You" } else { "Coda" },
                            p { "{line.text}" }
                        }
                    }
                }
            }
        }
        ActionRow {
            Button {
                onclick: move |_| {
                    let text = REPLIES[replies % REPLIES.len()].to_owned();
                    lines.write().push(Line { person: false, text, sources: false });
                },
                "A reply arrives"
            }
            Button {
                variant: ButtonVariant::Primary,
                onclick: move |_| {
                    lines.write().push(Line { person: true, text: "And her blood pressure?".into(), sources: false });
                },
                "Send a message"
            }
        }
    }
}

#[cfg(feature = "web")]
const REASONS: [&str; 4] = [
    "Not accurate",
    "Missing information",
    "Wrong source",
    "Hard to follow",
];

#[cfg(feature = "web")]
#[component]
fn FeedbackDemo() -> Element {
    let mut rating = use_signal(|| None::<Rating>);
    let mut saving = use_signal(|| false);
    let mut asking = use_signal(|| false);
    let mut reasons = use_signal(Vec::<usize>::new);
    let mut sending = use_signal(|| false);
    let mut undo = use_signal(|| None::<u32>);
    let mut archived = use_signal(|| 0u32);
    let mut restored = use_signal(|| false);
    rsx! {
        Message { author: Author::Assistant, by: "Coda",
            p { "Her potassium was 5.4 on Tuesday, up from 4.9 in September." }
            FeedbackRating {
                selected: rating(),
                label: "Rate this response",
                up_label: "Good response",
                down_label: "Poor response",
                busy: saving(),
                on_rate: move |choice| {
                    rating.set(Some(choice));
                    saving.set(true);
                    spawn(async move {
                        futures_timer::Delay::new(Duration::from_millis(900)).await;
                        saving.set(false);
                        if choice == Rating::Down {
                            asking.set(true);
                        }
                    });
                },
            }
        }
        div { class: "gallery-row",
            Button {
                onclick: move |_| {
                    restored.set(false);
                    archived.set(archived() + 1);
                    undo.set(Some(archived()));
                },
                "Archive conversation"
            }
            if restored() {
                span { class: "gallery-note", "Restored." }
            }
        }
        if let Some(key) = undo() {
            UndoNotice {
                key: "{key}",
                words: "Conversation archived.",
                undo_label: "Undo",
                on_undo: move |_| restored.set(true),
                on_gone: move |_| {
                    if undo() == Some(key) {
                        undo.set(None);
                    }
                },
            }
        }
        if asking() {
            Dialog { close_label: "Close", title: "Share feedback", subtitle: "What was wrong with this response?", on_close: move |_| asking.set(false),
                div { class: "gallery-row",
                    for (index, reason) in REASONS.into_iter().enumerate() {
                        ToggleChip {
                            key: "{reason}",
                            pressed: reasons.read().contains(&index),
                            disabled: sending(),
                            onclick: move |_| {
                                let mut chosen = reasons.write();
                                if let Some(at) = chosen.iter().position(|&held| held == index) {
                                    chosen.remove(at);
                                } else {
                                    chosen.push(index);
                                }
                            },
                            "{reason}"
                        }
                    }
                }
                ActionRow {
                    Button { variant: ButtonVariant::Quiet, disabled: sending(), onclick: move |_| asking.set(false), "Skip" }
                    Button {
                        variant: ButtonVariant::Primary,
                        busy: sending(),
                        pending_label: "Sending feedback",
                        onclick: move |_| {
                            sending.set(true);
                            spawn(async move {
                                futures_timer::Delay::new(Duration::from_millis(1200)).await;
                                sending.set(false);
                                asking.set(false);
                            });
                        },
                        "Send feedback"
                    }
                }
            }
        }
    }
}
