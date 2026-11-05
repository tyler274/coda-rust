//! Every `ui` family, rendered with the library's own stylesheet so each component's look, states
//! and motion can be seen and replayed on its own. Its examples are fictional and scripted.
//!
//! From the dev shell, this builds it and serves it at <http://127.0.0.1:8080/>:
//!
//! ```sh
//! scripts/ui-gallery.sh
//! ```
//!
//! Each family opens at its own address, such as `/#charts`. **Slow motion** plays every animation
//! at quarter speed, and **Replay entries** remounts the sections so their entry motion runs again.
//!
//! `cargo test -p ui-gallery --test sections -- --ignored` opens every family in Chromium at
//! desktop and phone sizes.

#[cfg(feature = "web")]
use dioxus::prelude::*;
#[cfg(feature = "web")]
use std::time::Duration;
#[cfg(feature = "web")]
use ui::{
    ActionRow, Answer, ApprovalCard, ArtifactCard, AttachButton, Attachment, AttachmentState,
    Attachments, Author, Avatar, AvatarSize, Button, ButtonVariant, Card, Change, ChartHeader,
    Checkbox, Choice, CitationChip, CitationPreview, CodeBlock, Column, Combobox, Command,
    CommandPalette, CompactField, Comparison, ComparisonBars, Composer, ContextChip, ContextMenu,
    ConversationViewport, CopyButton, CopyWords, Count, DateField, DateKind, DateSeparator, Dialog,
    Dictation, Diff, DiffLine, DiffWords, Disclosure, DraftConflict, EmptyLine, EmptyState,
    FeedbackRating, FooterActions, Icon, IconName, IconTile, KeyHint, ListRow, LiveDot,
    MeasurementTile, Menu, MenuItem, MenuSection, Message, MessageActions, MessageEditor,
    MessageState, MessageStatus, NavigationControl, NavigationRow, Notice, PageTitle, Paper,
    PaperSection, Placeholder, PlaceholderShape, Popover, ProgressBar, ProposedChange, Prose,
    ProseNames, ProsePart, Quote, RadioGroup, RailCard, Rating, RowFrame, RowGroup, RunningChart,
    SearchField, SectionHeader, Segmented, Select, SeriesChart, SeriesPoint, SeriesShape,
    SettingRow, Slider, SortOrder, SourceDetails, SourcePane, SourceReading, SourceText,
    SourcesButton, Sparkline, Spinner, StatePill, StatusLine, StepState, Suggestion, TabBar, Table,
    TableRow, Task, TaskList, TaskState, TaskWords, TextField, TextKind, Toast, ToggleChip, Tone,
    Tooltip, TopBar, TryAgain, Value, VersionSwitch, WorkStep, WorkTrace, WorkingMark,
    WorkingPresence, use_live_rows,
};
#[cfg(feature = "web")]
use wasm_bindgen::{JsCast, closure::Closure};

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

/// One family's place in the gallery: its link, its heading and what it shows.
#[cfg(feature = "web")]
struct Family {
    slug: &'static str,
    title: &'static str,
    note: &'static str,
    body: fn() -> Element,
}

#[cfg(feature = "web")]
const FAMILIES: [Family; 16] = [
    Family {
        slug: "type",
        title: "Type",
        note: "One scale for every app, on the system sans. A role is a whole setting; hosts name one and compose none of their own.",
        body: || {
            rsx! {
                TypeDemo {}
            }
        },
    },
    Family {
        slug: "colour",
        title: "Colour",
        note: "Roles in pairs, a fill and what reads on it. Hosts read roles, never tones; dark mode swaps the tones beneath.",
        body: || {
            rsx! {
                ColourDemo {}
            }
        },
    },
    Family {
        slug: "icons",
        title: "Icons",
        note: "One stroke set in the text's colour, each icon named for what it shows.",
        body: || {
            rsx! {
                IconDemo {}
                Part { title: "Icon tiles", note: "An icon leading what it belongs to: neutral on a hairline, as a card header or a record's heading leads, or tinted in the data colour, as a row leads.",
                    div { class: "gallery-row",
                        span { class: "gallery-label", IconTile { name: IconName::Calendar } "Neutral" }
                        span { class: "gallery-label", IconTile { name: IconName::Heart, tinted: true } "Tinted" }
                    }
                }
            }
        },
    },
    Family {
        slug: "buttons-and-navigation",
        title: "Buttons and navigation",
        note: "",
        body: || {
            rsx! {
                Part { title: "Buttons", note: "One shape in three variants, with the shared hover, press, focus and touch target.",
                    ButtonDemo {}
                }
                Part { title: "Tabs", note: "The app's sections, the current one marked; with icons, as a phone's tab bar does.",
                    TabsDemo {}
                }
            }
        },
    },
    Family {
        slug: "inputs",
        title: "Inputs",
        note: "A few short choices stand side by side, the fill sliding to the one chosen. A longer choice is a real box or ring: the check draws across, the dot springs out, and a box over a list shows when only some of it is chosen. A list opens the platform's own picker; a long one is found by typing, the list narrowing as the words come. An amount moves along a track and says itself; a date or a time opens the platform's own picker. A hint stands under a field, and what is wrong takes its place with the field outlined in red. The text field carries focus on its outline. The search field leads with its mark, which darkens while it has focus, and clears itself without losing focus. A labelled field names what it holds; with several lines, it grows with what is written.",
        body: || {
            rsx! {
                InputsDemo {}
            }
        },
    },
    Family {
        slug: "rows",
        title: "Rows",
        note: "",
        body: || {
            rsx! {
                Part { title: "Rows and sections", note: "Related rows group once, a hairline between them, rather than a card around each. A row leads with an icon, a number or a person's avatar, in three sizes. A section's one action sits beside its title.",
                    RowsDemo {}
                }
                Part { title: "Live lists", note: "A row that joins a list already shown opens and settles from just below; a row the list drops stays in its place as it collapses, then goes. A count fades into its new value, number and words together. A row the list moves up rises over the others, lifted; the selection is one fill that slides to the row chosen and travels with it. With a keyboard, all of it simply arrives.",
                    LiveDemo {}
                }
                Part { title: "Tables", note: "Things of one kind read across their columns, figures lined up on the trailing edge, in a frame that scrolls on its own where it doesn't fit. A heading asks for a sort and the host answers with the rows in that order. A row can open as a whole.",
                    TableDemo {}
                }
                Part { title: "Disclosure", note: "Rows open and close on a height transition; the chevron turns.",
                    Disclosure { title: "What Duet used", default_open: true,
                        p { class: "gallery-copy", "Sleep note from Tuesday, and seven nights of sleep from Apple Health." }
                    }
                    Disclosure { title: "Basis and limitations", detail: true,
                        p { class: "gallery-copy", "Coverage is seven of the last ten nights." }
                    }
                }
            }
        },
    },
    Family {
        slug: "cards",
        title: "Cards",
        note: "A card holds one object and names it: an observation and its evidence, saved work, an original record with the person's later context. One can open as a whole, or lead on from its footer.",
        body: || {
            rsx! {
                CardDemo {}
            }
        },
    },
    Family {
        slug: "pages",
        title: "Pages",
        note: "",
        body: || {
            rsx! {
                Part { title: "Screen headings", note: "A screen's title, what it belongs to above it, and its state or source beneath. A record leads with its icon; a screen with no top bar keeps its actions above the title.",
                    HeadingsDemo {}
                }
                Part { title: "Pages and forms", note: "A pushed screen's bar, what actually happened said once, a document page, choices and settings as rows, and actions pinned to the foot.",
                    PagesDemo {}
                }
                Part { title: "Reviewing a proposed update", note: "Section by section: what changed, the proposed words, what was saved before, and the sources each rests on. A section left as it was says so.",
                    ReviewDemo {}
                }
            }
        },
    },
    Family {
        slug: "charts",
        title: "Charts",
        note: "",
        body: || {
            rsx! {
                Part { title: "Measurements and charts", note: "Data draws in its own chart green, yellower than the action's, and colour never says whether a value is normal. Bars grow on the step spring; lines draw in, then their latest point.",
                    ChartDemo {}
                }
                Part { title: "Series", note: "A series across its buckets: amounts as bars from zero, levels as a line on their own range. The latest is drawn full and the rest quieter, a bucket still filling between them; a dashed line in the action green marks what it is compared with. Tap a mark to pin it, again or on empty plot to let go; the arrow keys move it. The page says what is pinned.",
                    SeriesDemo {}
                }
                Part { title: "Series readout", note: "Over its chart, a series says what its figure is, the figure, and the span it covers, each on one line so the chart never moves. Pinning a mark turns the readout to that bucket, its words in the accent. The span is a row of pills; the chosen one's fill slides to the next. Back and forward page the span, quiet where there is nowhere to go.",
                    ReadoutDemo {}
                }
                Part { title: "Running total", note: "A day so far against its usual pace: the running total in the data colour, ending in a dot where the day so far ends, and the usual total as a quieter dashed curve over a soft wash across the whole day. Both rise smoothly and never double back. A tile draws the two curves alone.",
                    RunningDemo {}
                }
            }
        },
    },
    Family {
        slug: "prose",
        title: "Prose",
        note: "An answer's words as Markdown: headings on the title roles, lists, emphasis, quotes, code and tables that scroll on their own, with citations where they were written.",
        body: || {
            rsx! {
                ProseDemo {}
            }
        },
    },
    Family {
        slug: "conversation",
        title: "Conversation",
        note: "",
        body: || {
            rsx! {
                Part { title: "Conversation frame", note: "Who the conversation is with, the day, a way to begin, a receipt that leads to a thing, and what the person brought in, held above the words.",
                    FrameDemo {}
                }
                Part { title: "The thread", note: "Scripted. Send rises on the send spring; the typing mark holds the reply's place; the reply arrives on the arrival spring.",
                    ConversationDemo {}
                }
                Part { title: "Composer states", note: "What is happening leads the toolbar: work running, beside its verb, or the draft's state. The actions keep the far edge, a key hint beside the words where there is a keyboard. A draft saved elsewhere opens under the toolbar with the saved words as written.",
                    ComposerStatesDemo {}
                }
                Part { title: "Conversation states", note: "What can be done with a message: under a reply, copy, ask again, rate, and move between its versions; under the person's words, copy and edit. On pointer devices the row shows while the message is hovered or focused; under the latest reply it always shows. Edited in place, the question keeps its earlier version. On its way a message stands quieter, waiting its turn it is outlined, and when it didn't go it says so in red with a way to try again. Files the person adds stand as tiles, an image as its picture; in the composer each uploads, filling its foot, or says why it didn't. Choose files with the plus, or drop them on the composer. Speaking, the composer listens in the words' place, the voice drawn as it comes; Done transcribes into the words, Cancel drops it.",
                    ThreadStatesDemo {}
                    DeliveryDemo {}
                    AttachmentsDemo {}
                    DictationDemo {}
                }
                Part { title: "Following a conversation", note: "At the end, each new reply is followed; scrolled back, the reading stays put and Jump to new response appears. What the person sends always brings them to the end.",
                    ViewportDemo {}
                }
            }
        },
    },
    Family {
        slug: "agent-work",
        title: "Agent work",
        note: "What an agent is doing, as it happens: one line for the turn, its steps beneath while it works, each step's words shimmering while it runs. Done, the steps fold under the line, which says what was done; a step with more opens to show it, such as the reasoning or a tool's result. Its plan checks off as it goes, each check drawing in. Before it acts it asks: answered, the card keeps the question and says the answer, and what came of it is reported apart. Its words fade in as they stream, and what it makes stands as a card that opens it, its title shimmering while it is made. Code stands in its own frame, to copy, and a change to it line by line. Every state comes from the run.",
        body: || {
            rsx! {
                AgentWorkDemo {}
                ApprovalDemo {}
                ReplyDemo {}
                CodeDemo {}
            }
        },
    },
    Family {
        slug: "citations-and-sources",
        title: "Citations and sources",
        note: "Pointing at a citation previews its source beside it; choosing it opens the source beside the reader, or over it with Back where there isn't room. A citation without a source stays a mark; one whose source can't be shown is muted and says so. A source opens at the passage it was cited for, marked.",
        body: || {
            rsx! {
                CitationDemo {}
            }
        },
    },
    Family {
        slug: "overlays",
        title: "Overlays",
        note: "",
        body: || {
            rsx! {
                Part { title: "Sheets and dialogs", note: "A sheet rises on the step spring and can open over another; a dialog rises the same way. Escape, the scrim or Close dismisses the top one. Drag a sheet down by its header, or by its content from the top: past half its height or flicked, it goes; otherwise it springs back.",
                    OverlayDemo {}
                }
                Part { title: "Menus, popovers and tooltips", note: "A menu or popover opens beside its control, under it from its leading edge, flipping where there isn't room, and arrives from that corner on the arrival spring. The arrow keys, a letter or the pointer move through a menu's choices; choosing one, Escape or a press elsewhere closes it, focus back on the control. A message's menu also opens at a right-click, from the menu key, or, held on touch, lifts the message over the blurred page with its menu beneath. A tooltip names an icon's control once the pointer rests on it, at once from the keys, and never on touch.",
                    MenuDemo {}
                }
                Part { title: "Command palette", note: "Every command, found by typing: each word narrows the list, groups keep together, and the first left is active. The arrow keys or the pointer move it; Enter or a press runs it. Press ⌘K, or Ctrl K, while this section is open.",
                    PaletteDemo {}
                }
            }
        },
    },
    Family {
        slug: "status",
        title: "Status",
        note: "",
        body: || {
            rsx! {
                Part { title: "Feedback and toasts", note: "The rating pair shows while its response is hovered or focused, and always on touch; reasons are toggle chips. A toast says what just happened at the window's foot, with what can follow, such as Undo; one stands at a time, and it pauses while pointed at.",
                    FeedbackDemo {}
                }
                Part { title: "Working and loading", note: "The working mark while work runs, the live dot, a placeholder in the shape of what loads (a line, list items, rows or reading), and notices for reads and commands.",
                    LoadingDemo {}
                }
                Part { title: "Waiting on an action", note: "A turning ring at icon size, on its own or where a busy button's icon was. Running work keeps the working mark; content still loading keeps its placeholder.",
                    div { class: "gallery-row",
                        span { class: "gallery-label", Spinner {} "Sending" }
                        Button { busy: true, pending_label: "Saving", Icon { name: IconName::Check } "Save" }
                    }
                }
                Part { title: "Trying again", note: "Scripted. The one recovery action after a failure, under the line that says what failed. It takes no input while the retry runs.",
                    TryAgainDemo {}
                }
                Part { title: "Progress", note: "Scripted. How far something has got, in the host's words. Not knowing how far, the bar moves and claims no amount. Bare, it stands where something else says what it is, such as a file's tile or a table's cell.",
                    ProgressDemo {}
                }
                Part { title: "Empty", note: "An empty section is one plain line. An empty or caught-up place has its mark, one line, what to know, and up to two quiet ways on: the second is the way out when the first may never succeed.",
                    EmptyDemo {}
                }
                Part { title: "State pills", note: "What a state chip's colour says.",
                    div { class: "gallery-row",
                        StatePill { "Neutral" }
                        StatePill { tone: Tone::Primary, icon: IconName::Check, "Saved" }
                        StatePill { tone: Tone::Warning, icon: IconName::Flag, "Needs you" }
                        StatePill { tone: Tone::Error, icon: IconName::Alert, "Couldn’t finish" }
                    }
                }
            }
        },
    },
    Family {
        slug: "duets-theme",
        title: "Duet's theme",
        note: "The same pieces under theme-duet: prose at body large, each action a full-width bar, quiet actions and links in green, cards on a hairline, and the person's bubble tailed toward them.",
        body: || {
            rsx! {
                ThemeDemo {}
            }
        },
    },
];

#[cfg(feature = "web")]
#[component]
fn Gallery() -> Element {
    let mut slow = use_signal(|| false);
    let mut replay = use_signal(|| 0u32);
    let mut linked = use_signal(linked_slug);
    use_hook(|| {
        document::eval(SLOW_MOTION);
        follow_the_link(move || linked.set(linked_slug()));
    });
    let family = FAMILIES
        .iter()
        .find(|family| family.slug == linked())
        .unwrap_or(&FAMILIES[0]);
    rsx! {
        ui::Stylesheet { app: GALLERY_STYLES }
        div { class: "gallery",
            header { class: "gallery-header",
                div {
                    h1 { class: "page-title", "crates/ui" }
                    p { class: "gallery-lede",
                        "Every family in the library, with its states. Entry motion replays when a section remounts."
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
            nav { class: "gallery-families", aria_label: "Families",
                for each in FAMILIES.iter() {
                    a {
                        key: "{each.slug}",
                        class: "gallery-family",
                        href: "#{each.slug}",
                        aria_current: if each.slug == family.slug { "page" },
                        "{each.title}"
                    }
                }
            }
            // A key only counts among a list's items, so the section is a list of one.
            for generation in std::iter::once(format!("{}-{}", family.slug, replay())) {
                div { key: "{generation}", class: "gallery-sections",
                    Section { title: family.title, note: family.note, {(family.body)()} }
                }
            }
        }
    }
}

/// The family the address names, from the part after `#`.
#[cfg(feature = "web")]
fn linked_slug() -> String {
    web_sys::window()
        .and_then(|window| window.location().hash().ok())
        .unwrap_or_default()
        .trim_start_matches('#')
        .to_owned()
}

/// Runs `changed` whenever the address names another family, by a link or by going back.
#[cfg(feature = "web")]
fn follow_the_link(changed: impl FnMut() + 'static) {
    let listener = Closure::<dyn FnMut()>::new(changed);
    if let Some(window) = web_sys::window() {
        let _ = window
            .add_event_listener_with_callback("hashchange", listener.as_ref().unchecked_ref());
    }
    // The gallery is the whole page, so its listener lasts as long as the page.
    listener.forget();
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
    ("label-sm", "Overline", "Original record"),
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
                        dd { class: "overline", "{sample}" }
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
    let mut rail_folded = use_signal(|| false);
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
                TaskList {
                    numbered: true,
                    tasks: vec![
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
                Card { tinted: true, overline: "Your later context", footer: "Added October 21",
                    Quote { text: "I now walk four mornings a week." }
                }
            }
            RailCard {
                icon: IconName::Calendar,
                title: "Upcoming visits",
                count: 2,
                count_words: "left",
                collapsed: rail_folded(),
                on_collapsed: move |folded| rail_folded.set(folded),
                expand_label: "Show upcoming visits",
                collapse_label: "Fold upcoming visits",
                "data-gallery": "rail-card",
                p { class: "card-detail", "What the card holds stands under its heading." }
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
                        words: format!("October {}, {hours:.1} hours of sleep", 14 + night),
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
fn Section(title: String, #[props(default)] note: String, children: Element) -> Element {
    rsx! {
        section { class: "gallery-section card",
            h2 { class: "gallery-title", "{title}" }
            if !note.is_empty() {
                p { class: "gallery-note", "{note}" }
            }
            div { class: "gallery-stage", {children} }
        }
    }
}

/// One piece of a family, under its own name and note.
#[cfg(feature = "web")]
#[component]
fn Part(title: String, note: String, children: Element) -> Element {
    rsx! {
        div { class: "gallery-part",
            h3 { class: "gallery-part-title", "{title}" }
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
                    StatePill { "Updated" }
                    StatePill { "Through Oct 21" }
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
                    li { Checkbox { label: "Recorded sleep", detail: "Apple Health · October 14–21", checked: sleep(), on_change: move |on| sleep.set(on) } }
                    li { Checkbox { label: "Visit note", detail: "Northside Clinic · October 15", checked: visit(), on_change: move |on| visit.set(on) } }
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
                trailing: rsx! {
                    Button {
                        variant: ButtonVariant::Quiet,
                        icon_only: true,
                        opens: "gallery-sprout-menu",
                        aria_label: "Conversation actions",
                        aria_haspopup: "menu",
                        Icon { name: IconName::More }
                    }
                    Menu { id: "gallery-sprout-menu", label: "Conversation actions",
                        MenuSection {
                            MenuItem { icon: IconName::Share, onclick: move |_| {}, "Share" }
                            MenuItem { icon: IconName::Archive, onclick: move |_| {}, "Archive" }
                        }
                    }
                },
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
    ("Visit tomorrow, 9:30 AM", "Follow-up · Primary care"),
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
        Some(68.0),
        Some(72.0),
        Some(66.0),
        None,
        Some(74.0),
        Some(70.0),
        Some(69.0),
    ];
    // Each day's lowest and highest around its average; Wednesday kept only its average.
    let spread = [
        Some((54.0, 88.0)),
        Some((55.0, 96.0)),
        Some((53.0, 84.0)),
        None,
        None,
        Some((52.0, 101.0)),
        Some((54.0, 90.0)),
    ];
    let mut pinned_steps = use_signal(|| None::<usize>);
    let mut pinned_rest = use_signal(|| None::<usize>);
    let mut pinned_month = use_signal(|| None::<usize>);
    let month_points: Vec<SeriesPoint> = (0..30)
        .map(|date| {
            let steps = 5200.0 + ((date * 37 % 11) as f64) * 520.0;
            SeriesPoint {
                value: Some(steps),
                axis: if date % 7 == 0 {
                    format!("Oct {}", date + 1)
                } else {
                    String::new()
                },
                words: format!("October {}, {} steps", date + 1, steps as u32),
                ..SeriesPoint::default()
            }
        })
        .collect();
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
            low: spread[index].map(|(low, _)| low),
            high: spread[index].map(|(_, high)| high),
            axis: day(index),
            words: value.map_or_else(String::new, |bpm| {
                format!("{}, averaging {bpm} beats per minute", WEEK[index])
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
            p { class: "gallery-note", "{readout(pinned_rest(), &rest_points, \"Heart rate · this week, each day's range\")}" }
            SeriesChart {
                shape: SeriesShape::Line,
                points: rest_points.clone(),
                label: "Heart rate each day this week, averaging 66 to 74 beats per minute, nothing on Tuesday",
                selected: pinned_rest(),
                on_select: move |next| pinned_rest.set(next),
            }
            p { class: "gallery-note", "{readout(pinned_month(), &month_points, \"Steps · October\")}" }
            SeriesChart {
                points: month_points.clone(),
                label: "Steps each day in October, from 5,200 to 10,400",
                emphasized: Some((0, 29)),
                selected: pinned_month(),
                on_select: move |next| pinned_month.set(next),
            }
        }
    }
}

#[cfg(feature = "web")]
#[component]
fn RunningDemo() -> Element {
    // The usual running total at each hour's start, and today's to 2 PM.
    let usual = [
        0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 150.0, 600.0, 1400.0, 2200.0, 2900.0, 3500.0, 4300.0, 5000.0,
        5600.0, 6200.0, 6900.0, 7500.0, 8100.0, 8600.0, 8900.0, 9100.0, 9200.0, 9250.0, 9300.0,
    ];
    let today = [
        0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 80.0, 300.0, 900.0, 1800.0, 2700.0, 3300.0, 3900.0, 4600.0,
        5100.0,
    ];
    let place = |values: &[f64]| -> Vec<(f64, f64)> {
        values
            .iter()
            .enumerate()
            .map(|(hour, value)| (hour as f64 / 24.0, *value))
            .collect()
    };
    let marks: Vec<(f64, String)> = ["12 AM", "6 AM", "12 PM", "6 PM", ""]
        .iter()
        .enumerate()
        .map(|(index, word)| (index as f64 / 4.0, (*word).to_owned()))
        .collect();
    rsx! {
        div { class: "gallery-series",
            p { class: "gallery-note", "Steps · today so far" }
            RunningChart {
                total: place(&today),
                usual: place(&usual),
                marks,
                end_word: "today",
                label: "Steps today, 5,100 by 2 PM, against a usual 5,600 by then",
            }
            div { class: "gallery-tile",
                RunningChart {
                    total: place(&today),
                    usual: place(&usual),
                    label: "Steps today, 5,100 by 2 PM, against a usual 5,600 by then",
                    axes: false,
                }
            }
        }
    }
}

#[cfg(feature = "web")]
#[component]
fn ReadoutDemo() -> Element {
    let mut span = use_signal(|| 0usize);
    let mut pinned = use_signal(|| None::<usize>);
    let week = [6200.0, 8400.0, 7100.0, 9800.0, 5400.0, 10200.0, 4300.0];
    let points: Vec<SeriesPoint> = if span() == 0 {
        week.iter()
            .enumerate()
            .map(|(index, value)| SeriesPoint {
                value: Some(*value),
                axis: WEEK[index][..1].to_owned(),
                words: format!("{}, October {}", WEEK[index], 24 + index),
                ..SeriesPoint::default()
            })
            .collect()
    } else {
        (0..30)
            .map(|date| SeriesPoint {
                value: Some(5200.0 + ((date * 37 % 11) as f64) * 520.0),
                axis: if date % 7 == 0 {
                    format!("Oct {}", date + 1)
                } else {
                    String::new()
                },
                words: format!("October {}", date + 1),
                ..SeriesPoint::default()
            })
            .collect()
    };
    let figure = |value: f64| {
        let whole = value.round() as u64;
        if whole >= 1000 {
            format!("{},{:03}", whole / 1000, whole % 1000)
        } else {
            whole.to_string()
        }
    };
    let average = points.iter().filter_map(|point| point.value).sum::<f64>() / points.len() as f64;
    let (label, value, caption) = match pinned().and_then(|index| points.get(index)) {
        Some(point) => (
            "Total".to_owned(),
            figure(point.value.unwrap_or_default()),
            point.words.clone(),
        ),
        None => (
            "Daily average".to_owned(),
            figure(average),
            if span() == 0 {
                "Oct 24 – 30, 2026 · today so far".to_owned()
            } else {
                "October 2026".to_owned()
            },
        ),
    };
    rsx! {
        div { class: "gallery-readout",
            Segmented {
                label: "Time range",
                options: vec![("W", "Week").into(), ("M", "Month").into()],
                selected: span(),
                on_select: move |next| {
                    span.set(next);
                    pinned.set(None);
                },
            }
            div { class: "gallery-readout-card",
                ChartHeader {
                    label,
                    value: vec![(value, String::new())],
                    caption,
                    pinned: pinned().is_some(),
                    previous: if span() == 0 { "Previous week" } else { "Previous month" },
                    next: if span() == 0 { "Next week" } else { "Next month" },
                    on_previous: move |_| {},
                }
                SeriesChart {
                    points: points.clone(),
                    label: if span() == 0 { "Steps each day this week" } else { "Steps each day in October" },
                    emphasized: if span() == 0 { None } else { Some((0, 29)) },
                    selected: pinned(),
                    on_select: move |next| pinned.set(next),
                }
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
#[component]
fn TryAgainDemo() -> Element {
    let mut retrying = use_signal(|| false);
    let retry = move |_| {
        spawn(async move {
            retrying.set(true);
            futures_timer::Delay::new(Duration::from_millis(1200)).await;
            retrying.set(false);
        });
    };
    rsx! {
        div { class: "gallery-stack",
            p { class: "gallery-copy", "Couldn’t refresh the visit list." }
            div { class: "gallery-row",
                TryAgain { label: "Try again", disabled: retrying(), on_retry: retry }
                TryAgain { label: "Try again", disabled: true, on_retry: move |_| {} }
            }
        }
    }
}

#[cfg(feature = "web")]
#[component]
fn ProgressDemo() -> Element {
    const NOTES: u32 = 12;
    let mut read = use_signal(|| 5);
    let mut importing = use_signal(|| false);
    let import = move |_| {
        spawn(async move {
            importing.set(true);
            for step in 0..=NOTES {
                read.set(step);
                futures_timer::Delay::new(Duration::from_millis(260)).await;
            }
            importing.set(false);
        });
    };
    rsx! {
        div { class: "gallery-stack",
            ProgressBar {
                label: "Importing notes",
                value: f64::from(read()) / f64::from(NOTES),
                reading: "{read()} of {NOTES} notes",
            }
            ProgressBar { label: "Reading the chart" }
            div { class: "gallery-row",
                Button { compact: true, disabled: importing(), onclick: import, "Import again" }
            }
        }
    }
}

#[cfg(feature = "web")]
// What each clinic's week came to: name, visits, minutes waited, the share of charts imported.
const CLINICS: [(&str, u32, u32, f64); 4] = [
    ("North clinic", 142, 12, 1.0),
    ("Riverside", 96, 21, 0.72),
    ("Eastgate", 118, 8, 0.4),
    ("Harbor", 64, 16, 0.15),
];

#[cfg(feature = "web")]
#[component]
fn TableDemo() -> Element {
    let mut sorted = use_signal(|| (1, SortOrder::Descending));
    let mut chosen = use_signal(|| None::<&'static str>);
    let (column, order) = sorted();
    let mut clinics = CLINICS.to_vec();
    clinics.sort_by(|a, b| match column {
        0 => a.0.cmp(b.0),
        1 => a.1.cmp(&b.1),
        _ => a.2.cmp(&b.2),
    });
    if order == SortOrder::Descending {
        clinics.reverse();
    }
    let rows = clinics
        .into_iter()
        .map(|(name, visits, waited, imported)| TableRow {
            key: name.into(),
            cells: vec![
                rsx! { "{name}" },
                rsx! { "{visits}" },
                rsx! { "{waited} min" },
                rsx! {
                    ProgressBar {
                        label: "Charts imported at {name}",
                        value: imported,
                        bare: true,
                    }
                },
            ],
            onclick: Some(EventHandler::new(move |_| chosen.set(Some(name)))),
        })
        .collect();
    rsx! {
        div { class: "gallery-stack",
            Table {
                label: "Clinics this week",
                columns: vec![
                    Column { sortable: true, .."Clinic".into() },
                    Column { numeric: true, sortable: true, .."Visits".into() },
                    Column { numeric: true, sortable: true, .."Waited".into() },
                    "Charts imported".into(),
                ],
                rows,
                sorted: sorted(),
                on_sort: move |next| {
                    let flipped = match order {
                        SortOrder::Ascending => SortOrder::Descending,
                        SortOrder::Descending => SortOrder::Ascending,
                    };
                    sorted.set((next, if next == column { flipped } else { SortOrder::Ascending }));
                },
            }
            if let Some(name) = chosen() {
                p { class: "gallery-note", "Chose {name}." }
            }
        }
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

/// The prose specimen.
#[cfg(feature = "web")]
const PROSE_BEFORE: &str = "## Recorded measurements\n\nPotassium was **5.4 mmol/L** on Tuesday";
#[cfg(feature = "web")]
const PROSE_AFTER: &str = ", up from 4.9 in September. Confirm the current reading with the patient.\n\n- Lisinopril 10 mg daily\n- Spironolactone 25 mg, *held* since October 2\n\n> Feels well, with no palpitations.\n\n| Date | Potassium |\n| --- | --- |\n| September 12 | 4.9 |\n| [October 28](https://example.org/results) | 5.4 |\n\nRecheck with a `BMP` next week.";
/// Prose in a message: compact, keeping its line breaks.
#[cfg(feature = "web")]
const PROSE_MESSAGE: &str =
    "Potassium is **5.4** today.\nRecheck on Friday.\n\n- Hold the supplement\n- Repeat the panel";

#[cfg(feature = "web")]
#[component]
fn ProseDemo() -> Element {
    rsx! {
        div { class: "gallery-prose",
            Prose {
                names: prose_names(),
                parts: vec![
                    ProsePart::Words(PROSE_BEFORE.to_owned()),
                    ProsePart::Inline(rsx! { CitationChip { index: 1, label: "Source 1: Basic metabolic panel" } }),
                    ProsePart::Words(PROSE_AFTER.to_owned()),
                ],
            }
            Message { author: Author::Assistant, by: "Coda",
                Prose {
                    message: true,
                    names: prose_names(),
                    parts: vec![ProsePart::Words(PROSE_MESSAGE.to_owned())],
                }
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
                TaskList { numbered: true, tasks: vec!["Put my phone away at 10:30 pm.".into(), "Keep my usual wake-up time.".into()] }
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
    // The one toast standing, keyed by its count: an archive's, which can be undone, or a copy's.
    let mut toast = use_signal(|| None::<(u32, bool)>);
    let mut shown = use_signal(|| 0u32);
    let mut restored = use_signal(|| false);
    rsx! {
        Message { author: Author::Assistant, by: "Coda",
            p { "{COPIED_REPLY}" }
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
                    shown.set(shown() + 1);
                    toast.set(Some((shown(), true)));
                },
                "Archive conversation"
            }
            Button {
                onclick: move |_| {
                    spawn(async move {
                        let copy = ui::copy_text(COPIED_REPLY.to_owned()).await;
                        if copy.is_ok() {
                            shown.set(shown() + 1);
                            toast.set(Some((shown(), false)));
                        }
                    });
                },
                Icon { name: IconName::Copy }
                "Copy reply"
            }
            if restored() {
                span { class: "gallery-note", "Restored." }
            }
        }
        if let Some((key, archive)) = toast() {
            Toast {
                key: "{key}",
                icon: (!archive).then_some(IconName::Check),
                words: if archive { "Conversation archived." } else { "Reply copied." },
                action: archive.then(|| "Undo".to_owned()),
                on_action: move |_| restored.set(true),
                on_gone: move |_| {
                    if toast().map(|(standing, _)| standing) == Some(key) {
                        toast.set(None);
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

#[cfg(feature = "web")]
#[component]
fn MenuDemo() -> Element {
    let mut model = use_signal(|| 0usize);
    let mut sources = use_signal(|| true);
    let mut chosen = use_signal(|| None::<&'static str>);
    let models = [
        ("Most capable", "For the hardest questions"),
        ("Balanced", "Fast and thorough"),
        ("Fastest", "For quick answers"),
    ];
    rsx! {
        div { class: "gallery-row",
            Tooltip { text: "More actions",
                Button {
                    variant: ButtonVariant::Quiet,
                    icon_only: true,
                    opens: "gallery-reply-menu",
                    aria_label: "More actions",
                    aria_haspopup: "menu",
                    Icon { name: IconName::More }
                }
            }
            Menu { id: "gallery-reply-menu", label: "Reply actions",
                MenuSection {
                    MenuItem { icon: IconName::Copy, shortcut: "⌘C", onclick: move |_| chosen.set(Some("Copy")), "Copy" }
                    MenuItem { icon: IconName::Pencil, onclick: move |_| chosen.set(Some("Edit")), "Edit" }
                    MenuItem { icon: IconName::Refresh, onclick: move |_| chosen.set(Some("Try again")), "Try again" }
                    MenuItem { icon: IconName::Share, disabled: true, onclick: move |_| {}, "Share" }
                }
                MenuSection {
                    MenuItem { icon: IconName::Trash, danger: true, onclick: move |_| chosen.set(Some("Delete")), "Delete" }
                }
            }
            Button {
                variant: ButtonVariant::Secondary,
                compact: true,
                opens: "gallery-model-menu",
                aria_haspopup: "menu",
                "{models[model()].0}"
                Icon { name: IconName::ChevronDown }
            }
            Menu { id: "gallery-model-menu", label: "Model",
                MenuSection { label: "Model",
                    for (index, (name, detail)) in models.into_iter().enumerate() {
                        MenuItem { key: "{name}", detail, checked: model() == index, radio: true,
                            onclick: move |_| model.set(index), "{name}" }
                    }
                }
                MenuSection {
                    MenuItem { icon: IconName::Book, checked: sources(), onclick: move |_| sources.set(!sources()), "Show sources" }
                }
            }
            Button {
                variant: ButtonVariant::Quiet,
                compact: true,
                opens: "gallery-popover",
                Icon { name: IconName::Users }
                "Who can see this"
            }
            Popover { id: "gallery-popover", label: "Who can see this",
                h3 { "Only you" }
                p { "This conversation is private. Sharing it makes a copy anyone with the link can read." }
                Button { compact: true, autofocus: true, onclick: move |_| chosen.set(Some("Copy link")),
                    Icon { name: IconName::Link }
                    "Copy link"
                }
            }
        }
        div { class: "gallery-row gallery-menu-tools",
            for (name, icon, key) in [
                ("Add files", IconName::Plus, None),
                ("Dictate", IconName::Microphone, None),
                ("Search", IconName::Search, Some(("⌘K", "Meta+K"))),
                ("New conversation", IconName::Message, Some(("⇧⌘O", "Shift+Meta+O"))),
            ] {
                Tooltip { key: "{name}", text: name, shortcut: key.map(|(shown, _)| shown.to_owned()),
                    Button { variant: ButtonVariant::Quiet, icon_only: true, aria_label: name, aria_keyshortcuts: key.map(|(_, heard)| heard), Icon { name: icon } }
                }
            }
        }
        div { class: "gallery-thread gallery-menu-thread",
            ContextMenu { menu: "gallery-own-menu", tabindex: "0",
                Message { author: Author::Person, by: "You", "Why was my sleep shorter this week?" }
            }
            ContextMenu { menu: "gallery-reply-menu", tabindex: "0",
                Message { author: Author::Assistant, by: "Coda",
                    p { "You averaged 6.1 hours, about 40 minutes under your usual. Most of the gap is on the three nights you went to bed after midnight." }
                }
            }
            Menu { id: "gallery-own-menu", label: "Message actions",
                MenuSection {
                    MenuItem { icon: IconName::Copy, shortcut: "⌘C", onclick: move |_| chosen.set(Some("Copy")), "Copy" }
                    MenuItem { icon: IconName::Pencil, onclick: move |_| chosen.set(Some("Edit")), "Edit" }
                }
                MenuSection {
                    MenuItem { icon: IconName::Trash, danger: true, onclick: move |_| chosen.set(Some("Delete")), "Delete" }
                }
            }
        }
        if let Some(words) = chosen() {
            p { class: "gallery-note", "Chose {words}." }
        }
    }
}

#[cfg(feature = "web")]
#[component]
fn PaletteDemo() -> Element {
    let mut open = use_signal(|| false);
    let mut ran = use_signal(|| None::<String>);
    use_hook(move || {
        let mut keys = document::eval(
            "document.addEventListener('keydown', (event) => {
              if ((event.metaKey || event.ctrlKey) && event.key.toLowerCase() === 'k') { event.preventDefault(); dioxus.send(true); }
            });",
        );
        spawn(async move {
            while keys.recv::<bool>().await.is_ok() {
                open.set(true);
            }
        });
    });
    let command =
        |group: &str, label: &str, icon: IconName, detail: Option<&str>, shortcut: Option<&str>| {
            Command {
                group: group.into(),
                label: label.into(),
                icon: Some(icon),
                detail: detail.map(Into::into),
                shortcut: shortcut.map(Into::into),
            }
        };
    let commands = vec![
        command(
            "Suggested",
            "New conversation",
            IconName::Message,
            None,
            Some("⇧⌘O"),
        ),
        command(
            "Suggested",
            "Search conversations",
            IconName::Search,
            None,
            Some("⌘F"),
        ),
        command(
            "This conversation",
            "Copy last reply",
            IconName::Copy,
            None,
            Some("⇧⌘C"),
        ),
        command(
            "This conversation",
            "Try again",
            IconName::Refresh,
            Some("Ask for a new reply"),
            None,
        ),
        command(
            "This conversation",
            "Share",
            IconName::Share,
            Some("Anyone with the link"),
            None,
        ),
        command(
            "This conversation",
            "Archive",
            IconName::Archive,
            None,
            None,
        ),
        command(
            "Settings",
            "Dark appearance",
            IconName::Moon,
            Some("Follow the system otherwise"),
            None,
        ),
        command("Settings", "Settings", IconName::Settings, None, Some("⌘,")),
    ];
    let names: Vec<String> = commands
        .iter()
        .map(|command| command.label.clone())
        .collect();
    rsx! {
        div { class: "gallery-row",
            Button { onclick: move |_| open.set(true),
                Icon { name: IconName::Search }
                "Search commands"
                KeyHint { key_label: "⌘K" }
            }
            if let Some(name) = ran() {
                span { class: "gallery-note", "Chose {name}." }
            }
        }
        if open() {
            CommandPalette {
                label: "Commands",
                placeholder: "Type a command",
                commands,
                empty: "No commands match.",
                on_choose: move |index: usize| ran.set(Some(names[index].clone())),
                on_close: move |_| open.set(false),
            }
        }
    }
}

/// A scripted run: each step runs, then is done, and the trace folds once the last is.
#[cfg(feature = "web")]
const RUN: [(IconName, &str, &str, &str); 4] = [
    (IconName::Sparkle, "Thinking", "Thought for 4s", ""),
    (
        IconName::File,
        "Reading the discharge summary",
        "Read the discharge summary",
        "2 pages",
    ),
    (
        IconName::Search,
        "Searching for potassium results",
        "Searched for potassium results",
        "6 found",
    ),
    (
        IconName::File,
        "Reading the cardiology note",
        "Read the cardiology note",
        "October 2",
    ),
];

/// The run's plan: a task finishes with each step after the first, and the last is passed over.
#[cfg(feature = "web")]
const PLAN: [&str; 4] = [
    "Find her baseline potassium",
    "Gather results since the dose change",
    "Check the dose in the cardiology note",
    "Ask the outside records",
];

#[cfg(feature = "web")]
#[component]
fn AgentWorkDemo() -> Element {
    // How far the run has come: steps before it are done, the one at it runs.
    let mut reached = use_signal(|| RUN.len());
    let mut runs = use_signal(|| 0u32);
    let replay = move |_| {
        let run = runs() + 1;
        runs.set(run);
        reached.set(0);
        spawn(async move {
            for at in 1..=RUN.len() {
                futures_timer::Delay::new(Duration::from_millis(1400)).await;
                if runs() != run {
                    return;
                }
                reached.set(at);
            }
        });
    };
    let working = reached() < RUN.len();
    rsx! {
        div { class: "gallery-agent",
            WorkTrace {
                label: if working { RUN[reached()].1 } else { "Read 2 notes and searched the record" },
                working,
                detail: (!working).then(|| "14s".to_owned()),
                for (index, (icon, running, done, detail)) in RUN.into_iter().enumerate().take(reached() + 1).take(RUN.len()) {
                    WorkStep {
                        key: "{index}",
                        icon,
                        label: if index < reached() { done } else { running },
                        state: if index < reached() { StepState::Done } else { StepState::Running },
                        detail: (index < reached() && !detail.is_empty()).then(|| detail.to_owned()),
                        body: (index == 0).then(|| rsx! {
                            p { "The question is whether her potassium is still rising after the spironolactone dose went up. I need the discharge summary for the baseline, the recent results, and the cardiology note for the dose." }
                        }),
                    }
                }
            }
            TaskList {
                words: TaskWords { active: "In progress".into(), done: "Done".into(), skipped: "Skipped".into() },
                tasks: PLAN.iter().enumerate().map(|(index, label)| Task {
                    label: (*label).into(),
                    state: match index.cmp(&reached()) {
                        _ if !working && index == PLAN.len() - 1 => TaskState::Skipped,
                        std::cmp::Ordering::Less => TaskState::Done,
                        std::cmp::Ordering::Equal => TaskState::Active,
                        std::cmp::Ordering::Greater => TaskState::Pending,
                    },
                }).collect::<Vec<_>>(),
            }
            Button { compact: true, disabled: working, onclick: replay,
                Icon { name: IconName::Refresh }
                "Replay the run"
            }
            WorkTrace { label: "Checked 3 sources, one didn't answer", detail: "9s",
                WorkStep { icon: IconName::File, label: "Read the visit note", state: StepState::Done, detail: "Northside Clinic" }
                WorkStep { icon: IconName::ExternalLink, label: "Couldn't reach the outside records", state: StepState::Failed, detail: "The connection timed out",
                    body: rsx! { p { "Asked the regional exchange for results since September; no answer after 8 seconds." } },
                }
                WorkStep { icon: IconName::Search, label: "Stopped searching older notes", state: StepState::Stopped, detail: "You stopped it" }
            }
        }
    }
}

#[cfg(feature = "web")]
#[component]
fn ApprovalDemo() -> Element {
    let mut answer = use_signal(|| None::<Answer>);
    let mut busy = use_signal(|| false);
    // What came of the answer, reported apart from it: None until sending starts.
    let mut sent = use_signal(|| None::<bool>);
    let mut removal = use_signal(|| None::<Answer>);
    let mut give = move |given: Answer| {
        busy.set(true);
        spawn(async move {
            futures_timer::Delay::new(Duration::from_millis(700)).await;
            busy.set(false);
            answer.set(Some(given));
            if given == Answer::Approved {
                sent.set(Some(false));
                futures_timer::Delay::new(Duration::from_millis(1600)).await;
                sent.set(Some(true));
            }
        });
    };
    rsx! {
        div { class: "gallery-agent",
            ApprovalCard {
                title: "Send this summary to her cardiologist?",
                detail: "Through the practice's secure messages",
                approve_label: "Send",
                decline_label: "Not now",
                busy: busy(),
                answer: answer(),
                answered: if answer() == Some(Answer::Approved) { "You approved sending it." } else { "You said not now." },
                on_approve: move |_| give(Answer::Approved),
                on_decline: move |_| give(Answer::Declined),
                Quote { text: "Potassium rose from 4.9 to 5.4 since the spironolactone dose went to 25 mg on October 2. No repeat level since. Worth a recheck this week?" }
            }
            if let Some(done) = sent() {
                WorkTrace { label: if done { "Sent the summary" } else { "Sending the summary" }, working: !done,
                    WorkStep { icon: IconName::Message,
                        label: if done { "Sent to her cardiologist" } else { "Sending to her cardiologist" },
                        state: if done { StepState::Done } else { StepState::Running },
                        detail: done.then(|| "Secure messages".to_owned()),
                    }
                }
            }
            ApprovalCard {
                title: "Remove the old care plan?",
                detail: "Replaced by the plan saved October 21",
                approve_label: "Remove",
                decline_label: "Keep it",
                danger: true,
                answer: removal(),
                answered: if removal() == Some(Answer::Approved) { "You approved removing it." } else { "You kept it." },
                on_approve: move |_| removal.set(Some(Answer::Approved)),
                on_decline: move |_| removal.set(Some(Answer::Declined)),
            }
            Button { compact: true, variant: ButtonVariant::Quiet,
                onclick: move |_| {
                    answer.set(None);
                    sent.set(None);
                    removal.set(None);
                },
                Icon { name: IconName::Refresh }
                "Ask again"
            }
        }
    }
}

/// A scripted reply, streamed in a few words at a time, that ends in the letter it wrote.
#[cfg(feature = "web")]
const REPLY: &str = "Her potassium has risen from **4.9 to 5.4** since the spironolactone dose went to 25 mg on October 2, and there is no repeat level since.\n\nI drafted a letter to her cardiologist asking whether to recheck this week. The results came from this request:\n\n```http\nGET /Observation?patient=4471&code=2823-3&date=ge2026-09-01\n```";

#[cfg(feature = "web")]
fn copy_words() -> CopyWords {
    CopyWords {
        copy: "Copy".into(),
        copied: "Copied".into(),
    }
}

#[cfg(feature = "web")]
fn prose_names() -> ProseNames {
    ProseNames {
        code: "Code".into(),
        table: "Table".into(),
    }
}

#[cfg(feature = "web")]
#[component]
fn ReplyDemo() -> Element {
    // How much of the reply has arrived, in bytes.
    let mut shown = use_signal(|| REPLY.len());
    let mut runs = use_signal(|| 0u32);
    let replay = move |_| {
        let run = runs() + 1;
        runs.set(run);
        shown.set(0);
        spawn(async move {
            let ends: Vec<usize> = REPLY
                .split_inclusive(char::is_whitespace)
                .scan(0, |at, word| {
                    *at += word.len();
                    Some(*at)
                })
                .collect();
            for end in ends.chunks(2).filter_map(|pair| pair.last().copied()) {
                futures_timer::Delay::new(Duration::from_millis(90)).await;
                if runs() != run {
                    return;
                }
                shown.set(end);
            }
        });
    };
    let streaming = shown() < REPLY.len();
    rsx! {
        div { class: "gallery-agent",
            Prose { streaming, copy: copy_words(), names: prose_names(), parts: vec![ProsePart::Words(REPLY[..shown()].to_owned())] }
            ArtifactCard {
                icon: IconName::File,
                title: "Letter to her cardiologist",
                detail: if streaming { "Writing" } else { "Letter · 1 page" },
                working: streaming,
                onclick: move |_| {},
                preview: rsx! {
                    p { "Dear colleague," }
                    p { "Her potassium has risen from 4.9 to 5.4 mmol/L since the spironolactone dose went to 25 mg on October 2. Would you like a repeat level this week, or a change in the dose?" }
                },
            }
            Button { compact: true, disabled: streaming, onclick: replay,
                Icon { name: IconName::Refresh }
                "Replay the reply"
            }
        }
    }
}

#[cfg(feature = "web")]
fn line(change: Change, text: &str) -> DiffLine {
    DiffLine {
        change,
        text: text.into(),
    }
}

/// A result as a tool returned it.
#[cfg(feature = "web")]
const RESULT: &str = "{\n  \"code\": \"2823-3\",\n  \"display\": \"Potassium\",\n  \"value\": 5.4,\n  \"unit\": \"mmol/L\",\n  \"effective\": \"2026-10-28\",\n  \"note\": \"Drawn four hours after the morning dose; repeat within one week if the reading stays above 5.0 mmol/L.\"\n}";

#[cfg(feature = "web")]
#[component]
fn CodeDemo() -> Element {
    rsx! {
        div { class: "gallery-agent",
            CodeBlock {
                name: "Code",
                label: "json",
                copy: copy_words(),
                code: RESULT,
            }
            Diff {
                label: "care-plan.md",
                words: DiffWords { added: "Added".into(), removed: "Removed".into() },
                lines: vec![
                    line(Change::Kept, "## Medications"),
                    line(Change::Kept, "- Lisinopril 10 mg daily"),
                    line(Change::Removed, "- Spironolactone 25 mg daily"),
                    line(Change::Added, "- Spironolactone 12.5 mg daily, until potassium is rechecked"),
                    line(Change::Added, "- Repeat basic metabolic panel within 1 week"),
                    line(Change::Kept, "- Metoprolol succinate 50 mg daily"),
                ],
            }
        }
    }
}

/// A reply asked three times, in the order it was asked.
#[cfg(feature = "web")]
const VERSIONS: [&str; 3] = [
    "Her potassium was 5.4 on Tuesday, up from 4.9 in September.",
    "Potassium rose from 4.9 in September to 5.4 on Tuesday, since the spironolactone dose went up.",
    "5.4 mmol/L on Tuesday; it was 4.9 in September. No repeat level since.",
];

/// The reply to the question once it is edited.
#[cfg(feature = "web")]
const EDITED_REPLY: &str = "Her last potassium was 5.4 mmol/L, on Tuesday's basic metabolic panel.";

#[cfg(feature = "web")]
#[component]
fn ThreadStatesDemo() -> Element {
    let mut version = use_signal(|| VERSIONS.len() - 1);
    let mut rating = use_signal(|| None::<Rating>);
    // The question as asked and as edited, which of them is shown, and the edit under way.
    let mut questions = use_signal(|| vec!["What was her last potassium?".to_owned()]);
    let mut asked = use_signal(|| 0usize);
    let mut draft = use_signal(|| None::<String>);
    let mut replying = use_signal(|| false);
    let question = questions()[asked()].clone();
    let reply = if asked() == 0 {
        VERSIONS[version()]
    } else {
        EDITED_REPLY
    };
    rsx! {
        div { class: "gallery-thread",
            if let Some(words) = draft() {
                MessageEditor {
                    label: "Edit your message",
                    value: words,
                    on_input: move |text| draft.set(Some(text)),
                    on_save: move |text: String| {
                        questions.write().push(text);
                        asked.set(questions.len() - 1);
                        draft.set(None);
                        replying.set(true);
                        spawn(async move {
                            futures_timer::Delay::new(Duration::from_millis(1200)).await;
                            replying.set(false);
                        });
                    },
                    on_cancel: move |_| draft.set(None),
                    save_label: "Save and ask",
                    cancel_label: "Cancel",
                    note: "Coda answers the edited question; the first answer stays one version back.",
                }
            } else {
                Message { author: Author::Person, by: "You",
                    actions: rsx! {
                        MessageActions { label: "Your message",
                            CopyButton { text: question.clone(), words: copy_words() }
                            Button {
                                variant: ButtonVariant::Quiet,
                                compact: true,
                                icon_only: true,
                                aria_label: "Edit",
                                disabled: replying(),
                                onclick: {
                                    let question = question.clone();
                                    move |_| draft.set(Some(question.clone()))
                                },
                                Icon { name: IconName::Pencil }
                            }
                            if questions().len() > 1 {
                                VersionSwitch {
                                    at: asked(),
                                    count: questions().len(),
                                    label: format!("Question {} of {}", asked() + 1, questions().len()),
                                    previous_label: "Previous question",
                                    next_label: "Next question",
                                    on_change: move |at| asked.set(at),
                                }
                            }
                        }
                    },
                    p { "{question}" }
                }
            }
            if replying() {
                Message { author: Author::Assistant,
                    span { class: "sr-only", role: "status", "Coda is responding" }
                    WorkingMark {}
                }
            } else if draft().is_none() {
                Message { author: Author::Assistant, by: "Coda",
                    actions: rsx! {
                        MessageActions { label: "Response", shown: true,
                            CopyButton { text: reply, words: copy_words() }
                            Button { variant: ButtonVariant::Quiet, compact: true, icon_only: true, aria_label: "Ask again", Icon { name: IconName::Refresh } }
                            FeedbackRating {
                                selected: rating(),
                                label: "Rate this response",
                                up_label: "Good response",
                                down_label: "Poor response",
                                on_rate: move |choice| rating.set(Some(choice)),
                            }
                            if asked() == 0 {
                                VersionSwitch {
                                    at: version(),
                                    count: VERSIONS.len(),
                                    label: format!("Response {} of {}", version() + 1, VERSIONS.len()),
                                    previous_label: "Previous response",
                                    next_label: "Next response",
                                    on_change: move |at| version.set(at),
                                }
                            }
                        }
                    },
                    p { "{reply}" }
                }
            }
        }
    }
}

/// Where a message stands in the delivery demo.
#[cfg(feature = "web")]
#[derive(Clone, Copy, PartialEq)]
enum Delivery {
    Failed,
    Sending,
    Sent,
}

#[cfg(feature = "web")]
#[component]
fn DeliveryDemo() -> Element {
    let mut delivery = use_signal(|| Delivery::Failed);
    let mut queued = use_signal(|| true);
    rsx! {
        div { class: "gallery-thread",
            Message { author: Author::Person, by: "You", p { "Summarize her last visit." } }
            Message { author: Author::Assistant,
                span { class: "sr-only", role: "status", "Coda is responding" }
                WorkingMark {}
            }
            if queued() {
                Message { author: Author::Person, by: "You", state: MessageState::Queued,
                    status: rsx! {
                        MessageStatus { words: "Sends after Coda's reply", icon: IconName::Clock,
                            Button { variant: ButtonVariant::Quiet, compact: true, onclick: move |_| queued.set(false), "Cancel" }
                        }
                    },
                    p { "And when was her last ECG?" }
                }
            }
            Message { author: Author::Person, by: "You",
                state: match delivery() {
                    Delivery::Failed => MessageState::Failed,
                    Delivery::Sending => MessageState::Sending,
                    Delivery::Sent => MessageState::Sent,
                },
                status: match delivery() {
                    Delivery::Failed => Some(rsx! {
                        MessageStatus { words: "Not sent. No connection.", icon: IconName::Alert, alert: true,
                            Button {
                                variant: ButtonVariant::Quiet,
                                compact: true,
                                onclick: move |_| {
                                    delivery.set(Delivery::Sending);
                                    spawn(async move {
                                        futures_timer::Delay::new(Duration::from_millis(1200)).await;
                                        delivery.set(Delivery::Sent);
                                    });
                                },
                                "Try again"
                            }
                        }
                    }),
                    Delivery::Sending => Some(rsx! { MessageStatus { words: "Sending" } }),
                    Delivery::Sent => None,
                },
                p { "Also check her magnesium." }
            }
            Button {
                variant: ButtonVariant::Quiet,
                compact: true,
                onclick: move |_| {
                    delivery.set(Delivery::Failed);
                    queued.set(true);
                },
                Icon { name: IconName::Refresh }
                "Show again"
            }
        }
    }
}

/// A strip of rhythm, standing in for a photo the person sent.
#[cfg(feature = "web")]
const STRIP: &str = "data:image/svg+xml;utf8,<svg xmlns='http://www.w3.org/2000/svg' viewBox='0 0 64 64'><rect width='64' height='64' fill='%23f7efe9'/><path d='M0 16h64M0 32h64M0 48h64M16 0v64M32 0v64M48 0v64' stroke='%23ecd3c6' stroke-width='.6'/><path d='M0 38h12l3-6 3 6h4l3-24 4 34 3-10h8l4-6 4 6h16' fill='none' stroke='%23a3341f' stroke-width='1.6' stroke-linejoin='round'/></svg>";

/// One file in the demo composer, as it uploads.
#[cfg(feature = "web")]
#[derive(Clone, PartialEq)]
struct Added {
    key: u32,
    name: String,
    detail: String,
    image: Option<&'static str>,
    state: AttachmentState,
    progress: f64,
}

#[cfg(feature = "web")]
fn size(bytes: u64) -> String {
    match bytes {
        0..1_000 => format!("{bytes} B"),
        1_000..1_000_000 => format!("{} KB", bytes / 1_000),
        _ => format!("{:.1} MB", bytes as f64 / 1_000_000.0),
    }
}

#[cfg(feature = "web")]
#[component]
fn AttachmentsDemo() -> Element {
    let mut added = use_signal(|| {
        vec![
            Added {
                key: 0,
                name: "Echo report.pdf".into(),
                detail: "PDF · 1.2 MB".into(),
                image: None,
                state: AttachmentState::Uploading,
                progress: 0.0,
            },
            Added {
                key: 1,
                name: "Holter export.zip".into(),
                detail: "Over the 25 MB limit".into(),
                image: None,
                state: AttachmentState::Failed,
                progress: 0.0,
            },
            Added {
                key: 2,
                name: "Rhythm strip.png".into(),
                detail: "Couldn't read this image".into(),
                image: Some(STRIP),
                state: AttachmentState::Failed,
                progress: 0.0,
            },
        ]
    });
    let mut next = use_signal(|| 3u32);
    let mut draft = use_signal(String::new);
    let mut opened = use_signal(|| None::<String>);
    // Fills a file's foot a step at a time, then leaves it ready.
    let upload = move |key: u32| {
        spawn(async move {
            for step in 1..=10 {
                futures_timer::Delay::new(Duration::from_millis(260)).await;
                let mut files = added.write();
                let Some(file) = files.iter_mut().find(|file| file.key == key) else {
                    return;
                };
                file.progress = f64::from(step) / 10.0;
                if step == 10 {
                    file.state = AttachmentState::Ready;
                }
            }
        });
    };
    use_hook(move || upload(0));
    let take = move |files: Vec<dioxus::html::FileData>| {
        for file in files {
            let key = next();
            next.set(key + 1);
            added.write().push(Added {
                key,
                name: file.name(),
                detail: size(file.size()),
                image: None,
                state: AttachmentState::Uploading,
                progress: 0.0,
            });
            upload(key);
        }
    };
    rsx! {
        div { class: "gallery-thread",
            Message { author: Author::Person, by: "You",
                Attachments {
                    Attachment { name: "Rhythm strip, this morning", image: STRIP, onclick: move |_| opened.set(Some("Rhythm strip, this morning".into())) }
                    Attachment { name: "Discharge summary.pdf", detail: "PDF · 240 KB", onclick: move |_| opened.set(Some("Discharge summary.pdf".into())) }
                }
                p { "Can you read these and tell me if anything changed?" }
            }
            if let Some(name) = opened() {
                p { class: "gallery-note", "Chose {name}." }
            }
        }
        Composer {
            label: "Message",
            placeholder: "Write a message",
            value: draft(),
            on_input: move |text| draft.set(text),
            on_submit: move |_| {},
            on_files: take,
            drop_label: "Drop files to attach",
            context: rsx! {
                if !added().is_empty() {
                    Attachments {
                        for file in added() {
                            Attachment {
                                key: "{file.key}",
                                name: file.name.clone(),
                                detail: file.detail.clone(),
                                image: file.image.map(str::to_owned),
                                state: file.state,
                                progress: file.progress,
                                remove_label: format!("Remove {}", file.name),
                                on_remove: move |_| added.write().retain(|kept| kept.key != file.key),
                            }
                        }
                    }
                }
            },
            AttachButton { label: "Attach files", multiple: true, on_files: take }
            Button { variant: ButtonVariant::Primary, compact: true, submit: true, disabled: added().iter().any(|file| file.state == AttachmentState::Uploading), "Send" }
        }
    }
}

/// What the scripted dictation transcribes to.
#[cfg(feature = "web")]
const SPOKEN: &str = "Ask her cardiologist whether to recheck the potassium this week.";

#[cfg(feature = "web")]
#[component]
fn DictationDemo() -> Element {
    let mut draft = use_signal(String::new);
    // Each listening is a run; its levels, tenths of a second, and whether it is transcribing.
    let mut listening = use_signal(|| None::<u32>);
    let mut levels = use_signal(Vec::<f32>::new);
    let mut tenths = use_signal(|| 0u32);
    let mut transcribing = use_signal(|| false);
    let start = move |_| {
        let run = listening().map_or(0, |run| run + 1);
        listening.set(Some(run));
        levels.set(Vec::new());
        tenths.set(0);
        transcribing.set(false);
        spawn(async move {
            // A voice's loudness, scripted: syllables rising and falling, with pauses between words.
            let mut seed = 7u32;
            loop {
                futures_timer::Delay::new(Duration::from_millis(100)).await;
                if listening() != Some(run) || transcribing() {
                    return;
                }
                seed = seed.wrapping_mul(1_103_515_245).wrapping_add(12_345);
                let tick = tenths() + 1;
                let noise = (seed >> 16) as f32 / 65_535.0;
                let syllable = ((tick as f32) * 1.7).sin().abs();
                let pause = if tick % 11 < 2 { 0.08 } else { 1.0 };
                levels
                    .write()
                    .push((0.15 + 0.75 * syllable * noise.sqrt()) * pause);
                tenths.set(tick);
            }
        });
    };
    let done = move |_| {
        transcribing.set(true);
        spawn(async move {
            futures_timer::Delay::new(Duration::from_millis(1100)).await;
            if transcribing() {
                draft.set(SPOKEN.to_owned());
                transcribing.set(false);
                listening.set(None);
            }
        });
    };
    let seconds = tenths() / 10;
    rsx! {
        Composer {
            label: "Message",
            placeholder: "Write a message, or speak",
            value: draft(),
            on_input: move |text| draft.set(text),
            on_submit: move |_| {},
            dictation: listening().is_some().then(|| rsx! {
                Dictation {
                    label: if transcribing() { "Transcribing" } else { "Listening" },
                    levels: levels(),
                    elapsed: format!("{}:{:02}", seconds / 60, seconds % 60),
                    transcribing: transcribing(),
                    cancel_label: "Cancel dictation",
                    done_label: "Done",
                    on_cancel: move |_| {
                        transcribing.set(false);
                        listening.set(None);
                    },
                    on_done: done,
                }
            }),
            if listening().is_none() {
                Button { variant: ButtonVariant::Quiet, compact: true, icon_only: true, aria_label: "Dictate", onclick: start, Icon { name: IconName::Microphone } }
                Button { variant: ButtonVariant::Primary, compact: true, submit: true, disabled: draft().trim().is_empty(), "Send" }
            }
        }
    }
}

#[cfg(feature = "web")]
const COPIED_REPLY: &str = "Her potassium was 5.4 on Tuesday, up from 4.9 in September.";

#[cfg(feature = "web")]
const VISIT_TYPES: [&str; 4] = ["Follow-up", "New patient", "Annual physical", "Telehealth"];

#[cfg(feature = "web")]
const MEDICATIONS: [(&str, &str); 8] = [
    ("Lisinopril", "Tablet, 10 mg"),
    ("Losartan", "Tablet, 50 mg"),
    ("Metoprolol succinate", "Tablet, 50 mg"),
    ("Metformin", "Tablet, 500 mg"),
    ("Spironolactone", "Tablet, 25 mg"),
    ("Atorvastatin", "Tablet, 40 mg"),
    ("Apixaban", "Tablet, 5 mg"),
    ("Furosemide", "Tablet, 40 mg"),
];

/// Where each medication matching the words stands in the list.
#[cfg(feature = "web")]
fn matching_medications(words: &str) -> Vec<usize> {
    let words = words.trim().to_lowercase();
    (0..MEDICATIONS.len())
        .filter(|index| MEDICATIONS[*index].0.to_lowercase().contains(&words))
        .collect()
}

#[cfg(feature = "web")]
const RESULTS: [&str; 3] = ["Labs", "Imaging", "Visit notes"];

#[cfg(feature = "web")]
#[component]
fn InputsDemo() -> Element {
    let mut sharing = use_signal(|| [true, false, true]);
    let mut reach = use_signal(|| Some(0usize));
    let mut on = use_signal(|| true);
    let mut instruction = use_signal(String::new);
    let mut email = use_signal(|| "dana@northside".to_owned());
    let mut phone = use_signal(String::new);
    let mut span = use_signal(|| 1usize);
    let mut look = use_signal(|| 0usize);
    let mut search = use_signal(|| "Delgado".to_string());
    let mut discuss = use_signal(|| {
        "I’d like to understand why I’m so tired in the afternoon. It’s getting in the way of work."
            .to_string()
    });
    let mut pain = use_signal(|| 4.0);
    let mut day = use_signal(|| "2026-09-10".to_owned());
    let mut time = use_signal(String::new);
    let mut visit = use_signal(|| None::<usize>);
    let mut medicine = use_signal(String::new);
    let mut chosen_medicine = use_signal(|| None::<usize>);
    let mut found = use_signal(|| matching_medications(""));
    let mut searching = use_signal(|| false);
    let chosen = sharing().iter().filter(|on| **on).count();
    rsx! {
        div { class: "gallery-inputs",
            Segmented {
                label: "Span",
                options: vec![
                    "Day".into(),
                    "Week".into(),
                    "Month".into(),
                    Choice { disabled: true, .."Year".into() },
                ],
                selected: span(),
                on_select: move |index| span.set(index),
            }
            Segmented {
                label: "Appearance",
                options: vec![(IconName::Settings, "Auto").into(), (IconName::Sun, "Light").into(), (IconName::Moon, "Dark").into()],
                selected: look(),
                on_select: move |index| look.set(index),
            }
            div { class: "row-list",
                Checkbox {
                    label: "Everything since September",
                    detail: format!("{chosen} of {} kinds of record", RESULTS.len()),
                    checked: chosen == RESULTS.len(),
                    mixed: chosen > 0 && chosen < RESULTS.len(),
                    on_change: move |all: bool| sharing.set([all; 3]),
                }
                for (index, label) in RESULTS.into_iter().enumerate() {
                    Checkbox {
                        key: "{label}",
                        label,
                        checked: sharing()[index],
                        on_change: move |on: bool| sharing.write()[index] = on,
                    }
                }
                Checkbox { label: "Billing records", detail: "Not shared with this clinic", checked: false, disabled: true, on_change: move |_| {} }
            }
            RadioGroup {
                label: "How should we reach her?",
                selected: reach(),
                on_select: move |index| reach.set(Some(index)),
                options: vec![
                    ("By text", "Fastest, for anything routine").into(),
                    ("By phone", "For results that need a conversation").into(),
                    Choice { disabled: true, ..("By letter", "No address on file").into() },
                ],
            }
            div { class: "row-list",
                SettingRow { title: "Share with her care team", detail: "Clinicians at Northside Clinic", on: on(), on_change: move |value| on.set(value) }
                SettingRow { title: "Share with research", detail: "Set by the practice", on: false, disabled: true, on_change: move |_| {} }
            }
            Select {
                label: "Visit type",
                placeholder: "Choose a visit type",
                options: VISIT_TYPES.map(Choice::from).to_vec(),
                selected: visit(),
                on_select: move |index| visit.set(Some(index)),
            }
            Combobox {
                label: "Medication",
                placeholder: "Search medications",
                query: medicine(),
                options: found().iter().map(|index| {
                    let (name, form) = MEDICATIONS[*index];
                    Choice { disabled: name == "Apixaban", ..(name, form).into() }
                }).collect::<Vec<_>>(),
                selected: found().iter().position(|index| chosen_medicine() == Some(*index)),
                loading: searching(),
                empty: "No medication by that name.",
                hint: "Search by name.",
                on_query: move |words: String| {
                    medicine.set(words.clone());
                    chosen_medicine.set(None);
                    searching.set(true);
                    spawn(async move {
                        futures_timer::Delay::new(Duration::from_millis(240)).await;
                        if medicine() == words {
                            found.set(matching_medications(&words));
                            searching.set(false);
                        }
                    });
                },
                on_choose: move |at: usize| {
                    let index = found()[at];
                    chosen_medicine.set(Some(index));
                    medicine.set(MEDICATIONS[index].0.to_owned());
                },
            }
            SearchField {
                label: "Search your panel",
                placeholder: "Name, DOB or phone",
                clear_label: "Clear search",
                value: search(),
                on_input: move |words| search.set(words),
            }
            Slider {
                label: "Pain today",
                value: pain(),
                min: 0.0,
                max: 10.0,
                reading: format!("{} of 10", pain()),
                hint: "0 is none, 10 the worst you can imagine.",
                on_change: move |amount| pain.set(amount),
            }
            div { class: "gallery-when",
                DateField {
                    label: "Follow-up",
                    value: day(),
                    min: "2026-08-27",
                    on_change: move |when| day.set(when),
                }
                DateField {
                    label: "Time",
                    kind: DateKind::Time,
                    value: time(),
                    on_change: move |when| time.set(when),
                }
            }
            TextField {
                label: "What I want to discuss",
                kind: TextKind::Multiline,
                value: discuss(),
                on_input: move |words| discuss.set(words),
            }
            TextField {
                label: "Mobile number",
                value: phone(),
                on_input: move |words| phone.set(words),
                hint: "For reminders only.",
            }
            TextField {
                label: "Email",
                kind: TextKind::Email,
                value: email(),
                on_input: move |words| email.set(words),
                error: (!email().contains('.')).then(|| "That address looks incomplete.".to_owned()),
            }
            CompactField {
                label: "Tell Coda what to change",
                value: instruction(),
                on_input: move |words| instruction.set(words),
            }
        }
    }
}
