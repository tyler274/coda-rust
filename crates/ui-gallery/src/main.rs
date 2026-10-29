//! Every shared `ui` component, rendered by the crate itself with its own stylesheet, so its
//! look and motion can be seen and replayed apart from either app. The conversation is scripted.

#[cfg(feature = "web")]
use dioxus::prelude::*;
#[cfg(feature = "web")]
use std::time::Duration;
#[cfg(feature = "web")]
use ui::{
    ActionRow, Author, Button, ButtonVariant, Card, CitationChip, CommandNotice, CommandStage,
    Comparison, ComparisonBars, Composer, Dialog, Disclosure, Icon, IconName, LaterContext,
    ListRow, LiveDot, MeasurementTile, Message, NavigationControl, NavigationRow, Overline,
    Placeholder, Quote, ReadNotice, ReadStage, RowGroup, SectionHeader, SourcesButton, Sparkline,
    StatePill, StatusLine, StepList, TabBar, Tone, TrendChart, Value, WorkingMark,
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
        style { "{ui::STYLESHEET}{GALLERY_STYLES}" }
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
                Section { title: "Cards", note: "A card holds one object and names it: an observation and its evidence, saved work, an original record with the person's later context.",
                    CardDemo {}
                }
                Section { title: "Measurements and charts", note: "Data draws in a quieter green than the action, and colour never says whether a value is normal. Bars grow on the step spring; lines draw in, then their latest point.",
                    ChartDemo {}
                }
                Section { title: "Conversation", note: "Scripted. Send rises on the send spring; the typing mark holds the reply's place; the reply arrives on the arrival spring.",
                    ConversationDemo {}
                }
                Section { title: "Sheets and dialogs", note: "A sheet rises on the step spring and can open over another; a dialog rises the same way. Escape, the scrim or Close dismisses the top one.",
                    OverlayDemo {}
                }
                Section { title: "Working and loading", note: "The working mark while work runs, the live dot, the placeholder sheen while content loads, and notices for reads and commands.",
                    LoadingDemo {}
                }
                Section { title: "Citations and sources", note: "A citation in prose opens its source; a static one stays a mark.",
                    CitationDemo {}
                }
                Section { title: "Rows and sections", note: "Related rows group once, a hairline between them, rather than a card around each. A section's one action sits beside its title.",
                    RowsDemo {}
                }
                Section { title: "Buttons and navigation", note: "One shape in three variants, with the shared hover, press, focus and touch target.",
                    ButtonDemo {}
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
                Section { title: "Fields and tabs", note: "The text field carries focus on its outline.",
                    input { class: "field", aria_label: "Name", placeholder: "Name" }
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
fn CardDemo() -> Element {
    rsx! {
        div { class: "gallery-cards",
            Card {
                overline: "What Duet noticed",
                title: "Sleep was shorter than your recent average",
                footer: "October 14–21 · Apple Health",
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
            TrendChart {
                points: SLEEP_HOURS.to_vec(),
                label: "Sleep each night, October 14 to 21: from 7.1 hours to 6.3 last night",
                labels: vec!["Oct 14".into(), "Oct 21".into()],
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
                        SourcesButton { count: 2, onclick: move |_| sources_open.set(true) }
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
            Dialog { sheet: true, title: "Sources", on_close: move |_| sources_open.set(false),
                ol { class: "row-list",
                    li { ListRow { title: "Sleep note", detail: "Tuesday", onclick: move |_| source_open.set(true) } }
                    li { ListRow { title: "Sleep, Apple Health", detail: "Last 7 nights", onclick: move |_| source_open.set(true) } }
                }
            }
        }
        if source_open() {
            Dialog { sheet: true, title: "Sleep note", on_close: move |_| source_open.set(false),
                p { class: "gallery-copy", "About seven hours, from the sleep note." }
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
            Dialog { title: "Past work", subtitle: "Mary Louis", on_close: move |_| dialog.set(false),
                p { class: "gallery-copy", "A dialog rises near the top of the window over the scrim, with focus on its title." }
            }
        }
        if sheet() {
            Dialog { sheet: true, title: "Sources", on_close: move |_| sheet.set(false),
                p { class: "gallery-copy", "A sheet runs along the bottom edge." }
                Button { onclick: move |_| over.set(true), "Open another over it" }
            }
        }
        if over() {
            Dialog { sheet: true, title: "Sleep note", on_close: move |_| over.set(false),
                p { class: "gallery-copy", "Closing this returns to the sheet beneath." }
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
        div { class: "gallery-placeholder", Placeholder {} Placeholder {} }
        StatusLine { message: "Reading records…", pending: true }
        ReadNotice {
            stage: ReadStage::Stale,
            loading: "Loading…",
            stale: "Couldn’t refresh. Showing what was read at 9:41.",
            unavailable: "Couldn’t load.",
            retry: "Try again",
            on_retry: move |_| {},
        }
        CommandNotice {
            stage: CommandStage::Unconfirmed,
            sending: "Sending…",
            unconfirmed: "Couldn’t confirm it was saved.",
            check_again: "Check again",
            on_check_again: move |_| {},
        }
    }
}

#[cfg(feature = "web")]
#[component]
fn CitationDemo() -> Element {
    let mut open = use_signal(|| None::<usize>);
    rsx! {
        p { class: "gallery-copy",
            "Following an April admission with decompensated heart failure, she remained NYHA II–III with mild congestion."
            CitationChip { index: 1, label: "Source 1: Discharge summary", onclick: move |_| open.set(Some(1)) }
            CitationChip { index: 2, label: "Source 2: Cardiology note", onclick: move |_| open.set(Some(2)) }
            " Genetic testing was not yet back."
            CitationChip { index: 3, label: "Source 3: unavailable" }
        }
        if let Some(index) = open() {
            Dialog { title: if index == 1 { "Discharge summary" } else { "Cardiology note" }, subtitle: "Read Today, 8:00 AM", on_close: move |_| open.set(None),
                p { class: "gallery-copy", "The passage Coda cited for this claim." }
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

#[cfg(feature = "web")]
#[component]
fn TabsDemo() -> Element {
    let mut selected = use_signal(|| 0usize);
    let mut phone = use_signal(|| 1usize);
    rsx! {
        TabBar {
            labels: vec!["Duet".into(), "Today".into(), "You".into()],
            selected: selected(),
            on_select: move |index| selected.set(index),
        }
        div { class: "gallery-phone-tabs",
            TabBar {
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
            SectionHeader { title: "Sources, numbered" }
            ol { class: "row-list",
                li { ListRow { number: 1, title: "Discharge summary", detail: "Read Today, 8:00 AM", onclick: move |_| {} } }
                li { ListRow { number: 2, title: "Cardiology note", onclick: move |_| {} } }
            }
        }
    }
}
