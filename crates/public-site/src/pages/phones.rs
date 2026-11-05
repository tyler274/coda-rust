use dioxus::prelude::*;

/// The Duet section, shared by the homepage and /duet: one story told across three
/// phones, drawn from the October 21 Duet mockups (ENG-168, ENG-169). Duet does the
/// noticing and the following up; Maya says yes. Today shows what Duet noticed about
/// her home blood pressure; in her conversation Duet offers a morning Check-in, which
/// she accepts; on Monday the Check-in returns by itself and Duet prepares an update to
/// her page for her doctor; she reviews it, saves it, previews it, and the story ends
/// on the page a clinician receives.
///
/// Each screen is laid out at the mockups' native 390-point width and scaled into the
/// phone, so its sizes are the mockups' own. It is styled by /css/duet.css and played
/// by /homepage/duet.js. The markup is the story's
/// last frame, which reduced-motion and no-script visitors see; duet.js starts each
/// pass from the opening frame.
#[component]
pub fn Phones() -> Element {
    rsx! {
        div {
            div { class: "frame frame-wide rv", "data-art": "dawn", "data-seed": "5",
                div {
                    class: "stage-wrap",
                    "data-ch": "820",
                    "data-cw": "640",
                    "data-h": "760",
                    "data-w": "1280",
                    div {
                        class: "stage duet-story",
                        "data-day": "mon",
                        "data-step": "preview",
                        Today {}
                        Chat {}
                        Doctor {}
                        Clinician {}
                    }
                }
            }
            div { class: "duet-labels",
                div { class: "rv",
                    span { class: "idx", i {} "01" }
                    h3 { "Duet notices" }
                }
                div { class: "rv", style: "--d:.06s",
                    span { class: "idx", i {} "02" }
                    h3 { "Duet follows up" }
                }
                div { class: "rv", style: "--d:.12s",
                    span { class: "idx", i {} "03" }
                    h3 { "You choose what to share" }
                }
            }
        }
    }
}

/// Maya's page, which For my doctor shows and her clinician receives in the same words.
const NAME: &str = "Maya Bennett";
const BORN: &str = "March 14, 1968";
const CONCERN: &str = "I want to know whether I should start the higher dose of lisinopril.";
const MEDICINE: &str = "Maya takes lisinopril 10 mg each morning. She picked up a 20 mg prescription on October 1 and has not started it.";
const READINGS: &str = "Her home cuff readings averaged 141/82 mmHg across October 6–12 (7 mornings), after 144/84 mmHg across September 22–October 5 (12 days).";
const QUESTION: &str = "Should I start the 20 mg dose now, or keep recording at home first?";
const SOURCES: [(&str, &str); 2] = [
    ("Maya’s words", "Patient note · October 1, 2026"),
    (
        "Home blood pressure",
        "Apple Health · Home cuff · September 22–October 12, 2026",
    ),
];
/// The Check-in Duet offers and Maya accepts, as the conversation and Today show it.
const CHECK_IN: &str = "Morning blood pressure";

#[component]
fn Status() -> Element {
    rsx! {
        div { class: "ds-status",
            span { "9:41" }
            i { class: "ds-island" }
            span { "▮▮▮ ◒ ▰" }
        }
    }
}

#[component]
fn Tabs(on: &'static str) -> Element {
    rsx! {
        nav { class: "ds-tabs",
            for (label , symbol) in [("Duet", "chat"), ("Today", "sun"), ("You", "you")] {
                span { class: if on == label { "ds-tab ds-on" } else { "ds-tab" },
                    Icon { name: symbol }
                    span { "{label}" }
                }
            }
        }
    }
}

/// A navigation bar: back, title and a trailing Edit.
#[component]
fn Bar(title: &'static str) -> Element {
    rsx! {
        div { class: "ds-bar",
            span { class: "ds-bar-side", Icon { name: "back" } }
            span { class: "ds-bar-title", "{title}" }
            span { class: "ds-bar-side ds-bar-edit", "Edit" }
        }
    }
}

#[component]
fn Row(symbol: &'static str, title: &'static str, children: Element) -> Element {
    rsx! {
        div { class: "ds-row",
            Icon { name: symbol, class: "ds-row-icon" }
            span { class: "ds-copy",
                strong { "{title}" }
                small { {children} }
            }
            span { class: "ds-chevron", "›" }
        }
    }
}

/// Phone 1: Today, where the weekly review's insight appears without being asked
/// for, and the care Duet keeps for Maya (her Check-in, her page) waits.
#[component]
fn Today() -> Element {
    rsx! {
        div { class: "phone p1",
            div { class: "scr ds-scr",
                div { class: "ds-app",
                    Status {}
                    div { class: "ds-bar",
                        span { class: "ds-bar-date",
                            span { class: "ds-thu", "Thursday, October 8" }
                            span { class: "ds-mon", "Monday, October 12" }
                        }
                        span { class: "ds-bar-side ds-bar-edit", "Edit" }
                    }
                    div { class: "ds-scroll",
                        h1 { class: "ds-hero",
                            "Good morning,"
                            br {}
                            "Maya"
                        }
                        div { class: "ds-card ds-insight",
                            div { class: "ds-card-h",
                                img {
                                    class: "ds-face",
                                    alt: "",
                                    src: "/homepage/assets/sprout.webp",
                                }
                                "What Duet noticed"
                            }
                            div { class: "ds-card-title",
                                "Home blood pressure stayed close to your recent average"
                            }
                            div { class: "ds-compare",
                                div { class: "ds-compare-cap",
                                    span { "This week" }
                                    span { "143/83" }
                                }
                                div { class: "ds-compare-line", style: "width:93%" }
                                div { class: "ds-compare-cap",
                                    span { "Previous 2 weeks" }
                                    span { "144/84" }
                                }
                                div { class: "ds-compare-line ds-pale" }
                            }
                            div { class: "ds-insight-foot",
                                Icon { name: "heart" }
                                "9 readings · Apple Health"
                                span { class: "ds-chevron", "↗" }
                            }
                        }
                        div { class: "ds-coming",
                            div {
                                h2 { class: "ds-section", "Coming up" }
                                div { class: "ds-card ds-flat",
                                    Row { symbol: "clock", title: CHECK_IN, "Every morning · until Tuesday" }
                                }
                            }
                        }
                        h2 { class: "ds-section", "For my doctor" }
                        div { class: "ds-card ds-flat",
                            Row { symbol: "file", title: "My page for my doctor",
                                span { class: "ds-fd-saved", "Saved · ready to preview" }
                                span { class: "ds-fd-update", "An update is ready to review" }
                            }
                        }
                        h2 { class: "ds-section", "Your measurements" }
                        div { class: "ds-tiles",
                            div { class: "ds-card",
                                div { class: "ds-card-h",
                                    Icon { name: "heart" }
                                    "Blood pressure"
                                }
                                div { class: "ds-measure",
                                    "141/82"
                                    small { "mmHg" }
                                }
                                p { class: "ds-caption", "This morning · Home cuff" }
                                Spark { d: "M2 14 20 10 38 16 56 11 74 18 92 12 110 16 128 13", y: "13" }
                            }
                            div { class: "ds-card",
                                div { class: "ds-card-h",
                                    Icon { name: "steps" }
                                    "Steps"
                                }
                                div { class: "ds-measure", "4,280" }
                                p { class: "ds-caption", "Today · Apple Health" }
                                Spark { d: "M2 28 20 22 38 27 56 13 74 17 92 9 110 14 128 4", y: "4" }
                            }
                        }
                    }
                    Tabs { on: "Today" }
                    i { class: "ds-home" }
                }
            }
        }
    }
}

#[component]
fn Spark(d: &'static str, y: &'static str) -> Element {
    rsx! {
        svg {
            class: "ds-spark",
            view_box: "0 0 130 37",
            preserve_aspect_ratio: "none",
            "aria-hidden": "true",
            path {
                d,
                fill: "none",
                stroke: "currentColor",
                stroke_width: "2.2",
                stroke_linejoin: "round",
            }
            circle {
                cx: "128",
                cy: y,
                r: "3",
                fill: "currentColor",
            }
        }
    }
}

/// Phone 2: the conversation with Maya's companion. Duet offers a Check-in, which she
/// accepts in place; on Monday the Check-in returns on its own, and Duet prepares an
/// update to her page for her doctor.
#[component]
fn Chat() -> Element {
    rsx! {
        div { class: "phone p2",
            div { class: "scr ds-scr",
                div { class: "ds-app",
                    Status {}
                    div { class: "ds-chat-h",
                        img {
                            class: "ds-face",
                            alt: "",
                            src: "/homepage/assets/sprout.webp",
                        }
                        span { "Sprout" }
                    }
                    div { class: "ds-chat",
                        div { class: "ds-thread",
                            div { class: "ds-date", "Thursday, October 8" }
                            div { class: "ds-msg ds-bubble ds-offer ds-accepted",
                                p {
                                    "Your home readings this week averaged 143/83, close to 144/84 the two weeks before. With your visit on Tuesday, a reading each morning would give your doctor a steady week."
                                }
                                span { class: "ds-action ds-accept", "Check in each morning" }
                                p { class: "ds-caption ds-added", "Check-in added." }
                            }
                            div { class: "ds-msg ds-object",
                                div { class: "ds-receipt",
                                    Icon { name: "clock" }
                                    span {
                                        strong { "{CHECK_IN}" }
                                        small { "Check-in every morning until Tuesday · open current item" }
                                    }
                                    span { class: "ds-chevron", "›" }
                                }
                            }
                            div { class: "ds-msg ds-date", "Monday, October 12" }
                            div { class: "ds-msg ds-bubble",
                                p {
                                    "Good morning. Today’s reading is in, 7 mornings this week. Want me to add them to your page for your doctor?"
                                }
                            }
                            div { class: "ds-msg ds-bubble ds-user",
                                p { "Yes, add them" }
                            }
                            div { class: "ds-msg ds-bubble",
                                p { "Here’s the update. Your saved page stays as it is until you save it." }
                                span { class: "ds-object ds-review-link", "Review proposed update →" }
                            }
                            div { class: "ds-typing", i {} i {} i {} }
                        }
                    }
                    div { class: "ds-compose-wrap",
                        div { class: "ds-composer",
                            span { class: "ds-input", "Message Sprout" }
                            span { class: "ds-send", "↑" }
                        }
                    }
                    Tabs { on: "Duet" }
                    i { class: "ds-home" }
                }
            }
        }
    }
}

/// Phone 3: For my doctor, Maya's continuing page. The stage's `data-step` is where
/// it stands: rest (her page as she saved it), ready (Duet's update waits for her),
/// review (what changed, not saved), saved (saved, not shared) or preview. The markup
/// is the last step.
#[component]
fn Doctor() -> Element {
    rsx! {
        div { class: "phone p3",
            div { class: "scr ds-scr",
                div { class: "ds-app",
                    Status {}
                    div { class: "ds-views",
                        div { class: "ds-view ds-v-page",
                            Bar { title: "For my doctor" }
                            div { class: "ds-scroll",
                                p { class: "ds-caption ds-kicker", "Your continuing page" }
                                h1 { class: "ds-detail-title", "For my doctor" }
                                div { class: "ds-fold ds-fold-saved",
                                    div {
                                        div { class: "ds-saved-receipt",
                                            Icon { name: "check" }
                                            div {
                                                strong { "Page saved" }
                                                small { "This update has not been shared." }
                                            }
                                        }
                                    }
                                }
                                div { class: "ds-fold ds-fold-update",
                                    div {
                                        div { class: "ds-update",
                                            div { class: "ds-card-h",
                                                img {
                                                    class: "ds-face",
                                                    alt: "",
                                                    src: "/homepage/assets/sprout.webp",
                                                }
                                                "An update is ready"
                                            }
                                            h2 {
                                                "This week’s readings."
                                                br {}
                                                "Ready for Tuesday."
                                            }
                                            p { "Seven mornings of home readings are ready to add to your page." }
                                            p { class: "ds-caption", "Prepared October 12 · Not saved" }
                                            span { class: "ds-action ds-review", "Review update" }
                                        }
                                    }
                                }
                                div { class: "ds-saved-label",
                                    span { "Saved page" }
                                    span {
                                        span { class: "ds-oct1", "October 1" }
                                        span { class: "ds-oct12", "October 12" }
                                    }
                                }
                                div { class: "ds-peek",
                                    p { class: "ds-doc-label", "What I want to discuss" }
                                    blockquote { "{CONCERN}" }
                                    div { class: "ds-peek-foot",
                                        Icon { name: "file" }
                                        "Blood pressure"
                                        span { class: "ds-text-link", "View page →" }
                                    }
                                }
                                span { class: "ds-text-link", "Ask Duet about this page" }
                            }
                            div { class: "ds-footer ds-footer-preview",
                                span { class: "ds-action ds-to-preview", "Preview before sharing" }
                            }
                        }
                        div { class: "ds-view ds-v-review",
                            Bar { title: "Review update" }
                            div { class: "ds-scroll ds-white",
                                h1 { class: "ds-detail-title", "Here’s what changed." }
                                p { class: "ds-caption", "Proposed by Duet · Not saved" }
                                section { class: "ds-change",
                                    div { class: "ds-change-h",
                                        span { class: "ds-tag", "Through Oct 12" }
                                        h2 { "Home blood pressure" }
                                    }
                                    p { "{READINGS}" }
                                    span { class: "ds-source",
                                        Icon { name: "heart" }
                                        "Apple Health · Home cuff · Oct 6–12 ↗"
                                    }
                                }
                                section { class: "ds-unchanged",
                                    p { class: "ds-doc-label", "Your words · unchanged" }
                                    blockquote { "{CONCERN}" }
                                    p { class: "ds-summary", "▸ Your question · unchanged" }
                                }
                            }
                            div { class: "ds-footer",
                                span { class: "ds-action ds-save", "Save updated page" }
                                p { class: "ds-after", "Saving does not share this page." }
                            }
                        }
                        div { class: "ds-view ds-v-preview",
                            Bar { title: "Preview" }
                            div { class: "ds-scroll ds-paper-bg",
                                p { class: "ds-preview-cap", "Your saved page · October 12" }
                                Document {}
                            }
                            div { class: "ds-footer",
                                span { class: "ds-action ds-open",
                                    "Open clinician preview"
                                    Icon { name: "share" }
                                }
                                p { class: "ds-after", "Nothing is shared by opening this preview." }
                            }
                        }
                    }
                    i { class: "ds-home" }
                }
            }
        }
    }
}

/// Maya's saved page as her clinician receives it, word for word what she reviewed.
#[component]
fn Document() -> Element {
    rsx! {
        article { class: "ds-doc",
            header {
                p { class: "ds-doc-label", "For my doctor" }
                h2 { "{NAME}" }
                p { class: "ds-doc-meta", "Date of birth · {BORN}" }
            }
            section {
                h3 { "What I want to discuss" }
                blockquote { "{CONCERN}" }
            }
            section {
                h3 { "Blood pressure" }
                p {
                    "{MEDICINE} "
                    span { class: "ds-cite", "[1]" }
                }
                p {
                    "{READINGS} "
                    span { class: "ds-cite", "[2]" }
                }
            }
            section {
                h3 { "My questions" }
                p { "{QUESTION}" }
            }
            section { class: "ds-sources",
                h3 { "Sources included" }
                for (n , (title , detail)) in SOURCES.iter().enumerate() {
                    div { class: "ds-src",
                        span { class: "ds-src-n", "{n + 1}" }
                        span {
                            "{title}"
                            small { "{detail}" }
                        }
                        i { "+" }
                    }
                }
            }
            div { class: "ds-doc-foot",
                p { "Prepared with Duet · Reviewed by Maya" }
                p { "October 12, 2026 · 8:14 am" }
            }
        }
    }
}

/// The ending: the clinician page, in a browser window, slides up into the frame.
#[component]
fn Clinician() -> Element {
    rsx! {
        div { class: "ds-clin",
            div { class: "ds-browser",
                span { "• • •" }
                "For my doctor · {NAME}"
            }
            div { class: "ds-clin-body", Document {} }
        }
    }
}

/// The mockups' line icons, on a 24-point grid.
#[component]
fn Icon(name: &'static str, #[props(default = "ds-icon")] class: &'static str) -> Element {
    let d = match name {
        "chat" => "M5 4h14a2 2 0 0 1 2 2v10a2 2 0 0 1-2 2H9l-5 4v-4H3V6a2 2 0 0 1 2-2Z",
        "sun" => {
            "M12 3v2m0 14v2M3 12h2m14 0h2M5.6 5.6 7 7m10 10 1.4 1.4M5.6 18.4 7 17m10-10 1.4-1.4M16 12a4 4 0 1 1-8 0 4 4 0 0 1 8 0Z"
        }
        "you" => "M20 21a8 8 0 0 0-16 0m12-14a4 4 0 1 1-8 0 4 4 0 0 1 8 0Z",
        "clock" => "M21 12a9 9 0 1 1-18 0 9 9 0 0 1 18 0Zm-9-5v6l4 2",
        "steps" => {
            "M7 3c3 0 3 6 1 8H5c-1-3-1-8 2-8Zm9 6c3 0 3 6 1 8h-3c-1-3-1-8 2-8ZM5 14h3v5H5v-5Zm9 6h3v3h-3v-3"
        }
        "heart" => "M20 5c-3-3-6-1-8 1-2-2-5-4-8-1-5 5 4 11 8 15 4-4 13-10 8-15Z",
        "check" => "m5 12 4 4L20 5",
        "back" => "m14 5-7 7 7 7",
        "share" => "M12 15V2m-5 5 5-5 5 5M5 11v10h14V11",
        _ => "M5 3h10l4 4v14H5V3Zm10 0v5h4M8 12h8m-8 4h6",
    };
    rsx! {
        svg {
            class,
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            "aria-hidden": "true",
            path { d }
        }
    }
}
