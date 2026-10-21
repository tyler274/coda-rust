use super::phones::Phones;
use dioxus::prelude::*;

#[component]
pub fn Home() -> Element {
    rsx! {
        header { class: "hero",
            div { class: "hero-copy",
                h1 { class: "rv", style: "--d:.06s", "Coda handles what happens between visits." }
                p { class: "lede rv", style: "--d:.12s", "A physician-directed AI care team." }
                div { class: "hero-cta rv", style: "--d:.18s",
                    a { class: "btn", href: "mailto:hello@codahealth.ai",
                        "Partner with us "
                        icons::icon_0 {}
                    }
                }
            }
            div {
                class: "frame frame-hero rv",
                "data-art": "mist",
                "data-seed": "7",
                style: "--d:.26s",
                div {
                    class: "stage-wrap",
                    "data-ch": "760",
                    "data-cw": "600",
                    "data-h": "720",
                    "data-w": "1280",
                    div {
                        aria_label: "Illustration: Coda's Today view. Maya Bennett's flagged change opens with its sources.",
                        class: "stage hero-stage",
                        id: "heroStage",
                        div { class: "app a-win",
                            aside { class: "a-side",
                                img {
                                    alt: "",
                                    class: "a-logo",
                                    src: "/brand/coda-health-logo-transparent.webp",
                                }
                                span { class: "a-nav on",
                                    icons::icon_1 {}
                                    "Today"
                                }
                                span { class: "a-nav",
                                    icons::icon_2 {}
                                    "Updates"
                                }
                                span { class: "a-nav",
                                    icons::icon_3 {}
                                    "Find patient"
                                    kbd { "⌘K" }
                                }
                                span { class: "a-sideh", "Recent conversations" }
                                span { class: "a-conv",
                                    icons::icon_4 {}
                                    span {
                                        b { "Creatinine after ibuprofen" }
                                        small { "Jake Smith" }
                                    }
                                }
                                span { class: "a-conv",
                                    icons::icon_5 {}
                                    span {
                                        b { "Diuretic dose before Friday" }
                                        small { "Mary Louis" }
                                    }
                                }
                                span { class: "a-conv",
                                    icons::icon_6 {}
                                    span {
                                        b { "Labs due after travel" }
                                        small { "Elena Park" }
                                    }
                                }
                            }
                            div { class: "a-main",
                                div { class: "a-h",
                                    h3 { "Today" }
                                    span { class: "a-count", "4 need you" }
                                }
                                div { class: "a-cols",
                                    div { class: "a-list a-pre",
                                        div { class: "a-row", "data-k": "maya",
                                            div { class: "a-rh",
                                                i { class: "a-new" }
                                                b { "Maya Bennett" }
                                                span { class: "a-chip a-flag",
                                                    icons::icon_7 {}
                                                    "Flagged"
                                                }
                                                time { "9:12 PM" }
                                            }
                                            p {
                                                "Home blood pressure unchanged two weeks after lisinopril went from 10 to 20 mg."
                                            }
                                            p { class: "a-sub",
                                                "The 20 mg prescription was picked up. Nothing yet shows she started it."
                                            }
                                            div { class: "a-acts",
                                                span { class: "a-btn",
                                                    icons::icon_8 {}
                                                    "Done"
                                                }
                                            }
                                        }
                                        div { class: "a-row",
                                            div { class: "a-rh",
                                                i { class: "a-new" }
                                                b { "Jake Smith" }
                                                span { class: "a-chip a-reply",
                                                    icons::icon_9 {}
                                                    "Drafted a reply"
                                                }
                                                time { "6:05 PM" }
                                            }
                                            p { "“Did my kidney test come back okay?”" }
                                            div { class: "a-acts",
                                                span { class: "a-btn",
                                                    icons::icon_10 {}
                                                    "Review reply"
                                                }
                                            }
                                        }
                                        div { class: "a-row", "data-k": "mary",
                                            div { class: "a-rh",
                                                i { class: "a-new" }
                                                b { "Mary Louis" }
                                                span { class: "a-chip a-flag",
                                                    icons::icon_11 {}
                                                    "Flagged"
                                                }
                                                time { "6:48 AM" }
                                            }
                                            p {
                                                "Weight up 6 pounds in 2 days. Her last echo showed an LVEF of 25%."
                                            }
                                            p { class: "a-sub",
                                                "Review her diuretic dose. Her next visit is Friday."
                                            }
                                            div { class: "a-acts",
                                                span {
                                                    class: "a-btn",
                                                    "data-k": "done",
                                                    icons::icon_12 {}
                                                    "Done"
                                                }
                                            }
                                        }
                                        div { class: "a-row",
                                            div { class: "a-rh",
                                                b { "Elena Park" }
                                                span { class: "a-chip a-work",
                                                    span { class: "a-dots",
                                                        i {}
                                                        i {}
                                                        i {}
                                                    }
                                                    "Reviewing"
                                                }
                                                time { "8:03 AM" }
                                            }
                                            p {
                                                class: "a-step",
                                                "data-steps": "Checking the EHR for new records|Reading the chart|Writing it up",
                                                "Checking the EHR for new records"
                                            }
                                            div { class: "a-acts",
                                                span { class: "a-btn",
                                                    icons::icon_13 {}
                                                    "Stop review"
                                                }
                                            }
                                        }
                                        div { class: "a-also",
                                            "Coda also reviewed 9 patients today"
                                            icons::icon_14 {}
                                        }
                                    }
                                    div { class: "a-sched",
                                        div { class: "a-sh",
                                            icons::icon_15 {}
                                            b { "Schedule" }
                                            span { "5 left" }
                                        }
                                        div { class: "a-visit on",
                                            time { "8:30 AM" }
                                            b { "Jake Smith" }
                                            small { "Briefing ready" }
                                        }
                                        div { class: "a-visit",
                                            time { "9:10 AM" }
                                            b { "Robert Hale" }
                                            small { "Briefing ready" }
                                        }
                                        div { class: "a-visit",
                                            time { "9:50 AM" }
                                            b { "Grace Okafor" }
                                        }
                                        div { class: "a-visit",
                                            time { "10:30 AM" }
                                            b { "Wen Li" }
                                            small { "Briefing ready" }
                                        }
                                        div { class: "a-visit",
                                            time { "11:10 AM" }
                                            b { "Samuel Adeyemi" }
                                        }
                                    }
                                }
                            }
                            div { class: "a-scrim" }
                        }
                        div { class: "app a-reader hero-reader",
                            div { class: "a-rdh",
                                img {
                                    alt: "",
                                    src: "/homepage/assets/people/maya-bennett.jpg",
                                }
                                b { "Maya Bennett" }
                                span { class: "a-chip a-flag",
                                    icons::icon_16 {}
                                    "Flagged"
                                }
                                span { class: "a-ics",
                                    icons::icon_17 {}
                                    icons::icon_18 {}
                                }
                            }
                            h4 { "Home blood pressure unchanged after the dose increase" }
                            p { class: "a-body",
                                "Lisinopril went from 10 mg to 20 mg daily at her August 14 visit "
                                i { class: "a-cite", "data-c": "1", "[1]" }
                                ". Home readings since then average 143/83, close to her average before the change "
                                i { class: "a-cite", "data-c": "2", "[2]" }
                                ". The pharmacy shows the 20 mg prescription picked up on August 16 "
                                i { class: "a-cite", "data-c": "3", "[3]" }
                                ". Nothing in her record yet says whether she started it."
                            }
                            div { class: "chart",
                                div { class: "chart-head",
                                    span { "Home blood pressure" }
                                    span { class: "src",
                                        icons::icon_19 {}
                                        "Apple Health · home cuff"
                                    }
                                }
                                icons::icon_20 {}
                            }
                            div { class: "a-disc",
                                span {
                                    "Basis and limitations "
                                    icons::icon_21 {}
                                }
                                span { class: "a-srcs",
                                    icons::icon_22 {}
                                    "Sources 3"
                                }
                            }
                            div { class: "a-acts",
                                span { class: "a-btn", "data-k": "discuss",
                                    icons::icon_23 {}
                                    "Discuss"
                                }
                                span { class: "a-btn",
                                    icons::icon_24 {}
                                    "Done"
                                }
                            }
                        }
                        div { class: "app a-reader hero-disc",
                            div { class: "a-rdh",
                                img {
                                    alt: "",
                                    src: "/homepage/assets/people/maya-bennett.jpg",
                                }
                                b { "Maya Bennett" }
                                span { class: "a-ics",
                                    icons::icon_25 {}
                                    icons::icon_26 {}
                                }
                            }
                            div { class: "a-anchor",
                                b { "Discussing this review" }
                                small {
                                    "Home blood pressure unchanged after the dose increase · Today, 9:12 PM"
                                }
                            }
                            div { class: "a-msg me", "Could anything else be keeping it up?" }
                            div { class: "a-think",
                                span { class: "a-dots",
                                    i {}
                                    i {}
                                    i {}
                                }
                                span { class: "a-step", "Reading the chart" }
                            }
                            div { class: "a-msg coda",
                                "An urgent care visit on August 9 started a prednisone course for bronchitis "
                                i { class: "a-cite", "data-c": "1", "[1]" }
                                ". It ended August 18, and her home readings since then still average 143/83 "
                                i { class: "a-cite", "data-c": "2", "[2]" }
                                ". So the steroid doesn’t explain it. Whether she started the 20 mg is still the open question."
                            }
                            div { class: "a-compose",
                                span {
                                    class: "a-typed",
                                    "data-ph": "Ask Coda about this patient…",
                                    "Ask Coda about this patient…"
                                }
                                span { class: "a-mic", icons::icon_27 {} }
                                span { class: "a-send",
                                    "Send"
                                    icons::icon_28 {}
                                }
                            }
                        }
                        div { class: "app sat a-srcprev sat-s1",
                            span { class: "a-sl", "Source 1" }
                            b { "Visit note · Dr. Smith" }
                            small { "August 14, 2026" }
                            p {
                                "Increase lisinopril to 20 mg daily. Recheck home blood pressure in two weeks."
                            }
                            span { class: "a-open",
                                "Open original"
                                icons::icon_29 {}
                            }
                        }
                        div { class: "app sat a-srcprev sat-s3",
                            span { class: "a-sl", "Source 3" }
                            b { "Pharmacy record" }
                            small { "Lisinopril 20 mg tablet · 30 days" }
                            p { class: "a-kv",
                                span { "Picked up" }
                                b { "August 16" }
                            }
                            span { class: "a-open",
                                "Open original"
                                icons::icon_30 {}
                            }
                        }
                        div { class: "ptr-x",
                            div { class: "ptr-y", icons::icon_31 {} }
                        }
                    }
                }
            }
        }
        section { class: "sec sec-tight",
            div { class: "wrap",
                div { class: "cred card rv",
                    div { class: "cred-copy",
                        p { class: "two",
                            b { "Built by physicians." }
                            " Founded by a physician at Mass General and an MIT computer scientist."
                        }
                        a { class: "tlink", href: "mailto:hello@codahealth.ai",
                            "Partner with Coda "
                            span { "aria-hidden": "true", "→" }
                        }
                    }
                    div { class: "cred-logos",
                        p { class: "micro", "Our team and clinical network have trained and worked at" }
                        div { class: "logos",
                            img {
                                alt: "Mass General Brigham",
                                src: "/homepage/assets/mass-general-brigham.svg",
                            }
                            img {
                                alt: "Johns Hopkins Medicine",
                                src: "/homepage/assets/johns-hopkins-medicine.png",
                            }
                            img {
                                alt: "Stanford Medicine",
                                src: "/homepage/assets/stanford-medicine.svg",
                            }
                            img {
                                alt: "UCSF Health",
                                src: "/homepage/assets/ucsf-health.svg",
                            }
                            img {
                                alt: "Yale New Haven Health",
                                src: "/homepage/assets/yale-new-haven-health.svg",
                            }
                            img {
                                alt: "NewYork-Presbyterian",
                                src: "/homepage/assets/newyork-presbyterian.svg",
                            }
                            img {
                                alt: "Harvard",
                                src: "/homepage/assets/harvard.svg",
                            }
                            img {
                                alt: "Columbia Vagelos",
                                src: "/homepage/assets/vagelos.svg",
                            }
                            img {
                                alt: "Zucker School of Medicine",
                                src: "/homepage/assets/zucker-school-medicine.png",
                            }
                        }
                    }
                }
            }
        }
        section { class: "sec", id: "physicians",
            div { class: "wrap",
                div { class: "sec-head center rv",
                    span { class: "kicker",
                        b { "Coda" }
                        " for physicians"
                    }
                    h2 { "The work between visits, ready for you." }
                    p {
                        "When a result or message arrives, Coda pulls up the patient, your last plan, and what changed."
                    }
                }
                div { class: "feats",
                    div { class: "flist rv",
                        button { class: "fitem on", "data-f": "0", r#type: "button",
                            span { class: "idx",
                                i {}
                                "01"
                            }
                            h3 { "Today" }
                            p {
                                "What needs you, and why. Everything else Coda reviewed stays out of your way."
                            }
                        }
                        button { class: "fitem", "data-f": "1", r#type: "button",
                            span { class: "idx",
                                i {}
                                "02"
                            }
                            h3 { "Briefing" }
                            p {
                                "Before each visit, what changed since the last one, with a source on every line."
                            }
                        }
                        button { class: "fitem", "data-f": "2", r#type: "button",
                            span { class: "idx",
                                i {}
                                "03"
                            }
                            h3 { "Ask Coda" }
                            p {
                                "Ask about any patient. Coda answers from the chart and shows where each answer came from."
                            }
                        }
                        button { class: "fitem", "data-f": "3", r#type: "button",
                            span { class: "idx",
                                i {}
                                "04"
                            }
                            h3 { "Follow-ups" }
                            p { "Coda keeps checking until the result is back, then brings it to Today." }
                        }
                    }
                    div { class: "fmedia",
                        div {
                            class: "frame fframe rv",
                            "data-art": "sage",
                            "data-seed": "21",
                            div { class: "fstage on", "data-f": "0",
                                div {
                                    class: "stage-wrap",
                                    "data-ch": "620",
                                    "data-cw": "520",
                                    "data-h": "570",
                                    "data-w": "760",
                                    div { class: "stage f-today",
                                        div { class: "app a-card a-todaycard",
                                            div { class: "a-h",
                                                h3 { "Today" }
                                                span { class: "a-count",
                                                    span { class: "a-n", "3" }
                                                    " need you"
                                                }
                                            }
                                            div { class: "a-list",
                                                div {
                                                    class: "a-row",
                                                    "data-k": "maya",
                                                    div { class: "a-rh",
                                                        i { class: "a-new" }
                                                        b { "Maya Bennett" }
                                                        span { class: "a-chip a-flag",
                                                            icons::icon_32 {}
                                                            "Flagged"
                                                        }
                                                        time { "9:12 PM" }
                                                    }
                                                    p {
                                                        "Home blood pressure unchanged two weeks after lisinopril went from 10 to 20 mg."
                                                    }
                                                    p { class: "a-sub",
                                                        "The 20 mg prescription was picked up. Nothing yet shows she started it."
                                                    }
                                                    div { class: "a-acts",
                                                        span { class: "a-btn",
                                                            icons::icon_33 {}
                                                            "Done"
                                                        }
                                                    }
                                                }
                                                div { class: "a-row",
                                                    div { class: "a-rh",
                                                        i { class: "a-new" }
                                                        b { "Jake Smith" }
                                                        span { class: "a-chip a-reply",
                                                            icons::icon_34 {}
                                                            "Drafted a reply"
                                                        }
                                                        time { "6:05 PM" }
                                                    }
                                                    p { "“Did my kidney test come back okay?”" }
                                                    div { class: "a-acts",
                                                        span { class: "a-btn",
                                                            icons::icon_35 {}
                                                            "Review reply"
                                                        }
                                                    }
                                                }
                                                div {
                                                    class: "a-row",
                                                    "data-k": "mary",
                                                    div { class: "a-rh",
                                                        i { class: "a-new" }
                                                        b { "Mary Louis" }
                                                        span { class: "a-chip a-flag",
                                                            icons::icon_36 {}
                                                            "Flagged"
                                                        }
                                                        time { "6:48 AM" }
                                                    }
                                                    p {
                                                        "Weight up 6 pounds in 2 days. Her last echo showed an LVEF of 25%."
                                                    }
                                                    p { class: "a-sub",
                                                        "Review her diuretic dose. Her next visit is Friday."
                                                    }
                                                    div { class: "a-acts",
                                                        span {
                                                            class: "a-btn",
                                                            "data-k": "done",
                                                            icons::icon_37 {}
                                                            "Done"
                                                        }
                                                    }
                                                }
                                                div { class: "a-also",
                                                    "Coda also reviewed 9 patients today"
                                                    icons::icon_38 {}
                                                }
                                                div { class: "a-updates",
                                                    span { class: "a-ul", "Reviewed, nothing for you" }
                                                    div { class: "a-ur",
                                                        b { "Robert Hale" }
                                                        span { "Review" }
                                                        time { "7:40 AM" }
                                                    }
                                                    div { class: "a-ur",
                                                        b { "Grace Okafor" }
                                                        span { "Review" }
                                                        time { "7:12 AM" }
                                                    }
                                                    div { class: "a-ur",
                                                        b { "Wen Li" }
                                                        span { "Briefing" }
                                                        time { "6:55 AM" }
                                                    }
                                                    div { class: "a-ur a-more", "6 more" }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            div { class: "fstage", "data-f": "1",
                                div {
                                    class: "stage-wrap",
                                    "data-ch": "620",
                                    "data-cw": "520",
                                    "data-h": "570",
                                    "data-w": "760",
                                    div { class: "stage f-brief",
                                        div { class: "app a-card a-brief",
                                            div { class: "a-rdh",
                                                img {
                                                    alt: "",
                                                    src: "/homepage/assets/people/mary-louis.jpg",
                                                }
                                                b { "Mary Louis" }
                                                span { class: "a-meta", "Follow-up · Friday, 9:10 AM" }
                                            }
                                            span { class: "a-kind", "Briefing" }
                                            h4 {
                                                "Weight is up, and her diuretic dose hasn’t changed since March"
                                            }
                                            ul { class: "a-lines",
                                                li {
                                                    "Home weight rose 6 pounds in 2 days, to 188.6 lb on September 11. "
                                                    i {
                                                        class: "a-cite",
                                                        "data-c": "1",
                                                        "[1]"
                                                    }
                                                }
                                                li {
                                                    "Her last echocardiogram showed an LVEF of 25%. "
                                                    i {
                                                        class: "a-cite",
                                                        "data-c": "2",
                                                        "[2]"
                                                    }
                                                }
                                                li {
                                                    "Furosemide 40 mg daily, unchanged since March 3. "
                                                    i {
                                                        class: "a-cite",
                                                        "data-c": "3",
                                                        "[3]"
                                                    }
                                                }
                                                li {
                                                    "She wrote that her ankles are more swollen this week. "
                                                    i {
                                                        class: "a-cite",
                                                        "data-c": "4",
                                                        "[4]"
                                                    }
                                                }
                                            }
                                            div { class: "a-disc",
                                                span {
                                                    "Basis and limitations "
                                                    icons::icon_39 {}
                                                }
                                                span { class: "a-srcs",
                                                    icons::icon_40 {}
                                                    "Sources 4"
                                                }
                                            }
                                        }
                                        div { class: "app a-srcprev a-pop",
                                            span { class: "a-sl", "Source 2" }
                                            b { "Echocardiogram report" }
                                            small { "June 3, 2026 · Cardiology" }
                                            p {
                                                "LVEF 25%. Moderate mitral regurgitation. No change from the prior study."
                                            }
                                            span { class: "a-open",
                                                "Open original"
                                                icons::icon_41 {}
                                            }
                                        }
                                    }
                                }
                            }
                            div { class: "fstage", "data-f": "2",
                                div {
                                    class: "stage-wrap",
                                    "data-ch": "620",
                                    "data-cw": "520",
                                    "data-h": "570",
                                    "data-w": "760",
                                    div { class: "stage f-ask",
                                        div { class: "app a-card a-convo",
                                            div { class: "a-ph",
                                                span { class: "a-av", "JS" }
                                                div {
                                                    b { "Jake Smith" }
                                                    small { "58 years · DOB Mar 12, 1968" }
                                                }
                                            }
                                            div { class: "a-anchor",
                                                b { "Discussing this reply" }
                                                small { "Today, 6:05 PM" }
                                            }
                                            div { class: "a-msg me",
                                                "How does his creatinine compare with before?"
                                            }
                                            div { class: "a-think",
                                                span { class: "a-dots",
                                                    i {}
                                                    i {}
                                                    i {}
                                                }
                                                span {
                                                    class: "a-step",
                                                    "data-steps": "Reading the chart|Reviewing earlier work",
                                                    "Reading the chart"
                                                }
                                            }
                                            div { class: "a-msg coda",
                                                "It’s 1.2 today, down from 1.5 on August 20 "
                                                i {
                                                    class: "a-cite",
                                                    "data-c": "1",
                                                    "[1]"
                                                }
                                                ". That rise came after he started ibuprofen for knee pain "
                                                i {
                                                    class: "a-cite",
                                                    "data-c": "2",
                                                    "[2]"
                                                }
                                                ", which he stopped on August 22 "
                                                i {
                                                    class: "a-cite",
                                                    "data-c": "3",
                                                    "[3]"
                                                }
                                                ". Over the past two years it ranged from 1.1 to 1.3 "
                                                i {
                                                    class: "a-cite",
                                                    "data-c": "4",
                                                    "[4]"
                                                }
                                                "."
                                            }
                                            div { class: "a-compose",
                                                span { "Ask Coda about this patient…" }
                                                span { class: "a-mic", icons::icon_42 {} }
                                                span { class: "a-send",
                                                    "Send"
                                                    icons::icon_43 {}
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                            div { class: "fstage", "data-f": "3",
                                div {
                                    class: "stage-wrap",
                                    "data-ch": "620",
                                    "data-cw": "520",
                                    "data-h": "570",
                                    "data-w": "760",
                                    div { class: "stage f-follow",
                                        div { class: "app a-card a-fu",
                                            div { class: "a-sec",
                                                span { class: "a-tile", icons::icon_44 {} }
                                                b { "Follow-ups" }
                                                span { class: "a-c", "1" }
                                            }
                                            div { class: "a-furow",
                                                div { class: "a-rh",
                                                    b { "Elena Park" }
                                                    span { class: "a-chip a-plain",
                                                        icons::icon_45 {}
                                                        "Follow-up"
                                                    }
                                                    time { "Next review Oct 1" }
                                                }
                                                p {
                                                    "Look for her A1c and any messages once she is back from Spain at the end of September."
                                                }
                                                div { class: "a-run",
                                                    span { class: "a-chip a-work",
                                                        span { class: "a-dots",
                                                            i {}
                                                            i {}
                                                            i {}
                                                        }
                                                        "Reviewing"
                                                    }
                                                    span {
                                                        class: "a-step",
                                                        "data-steps": "Checking the EHR for new records|Reading the chart|Writing it up",
                                                        "Checking the EHR for new records"
                                                    }
                                                }
                                                div { class: "a-acts",
                                                    span { class: "a-btn", "Review now" }
                                                    span { class: "a-btn", "Cancel follow-up" }
                                                }
                                            }
                                        }
                                        div { class: "app a-arrive",
                                            span { class: "a-on", "Back on Today" }
                                            div { class: "a-rh",
                                                i { class: "a-new" }
                                                b { "Elena Park" }
                                                span { class: "a-chip a-flag",
                                                    icons::icon_46 {}
                                                    "Flagged"
                                                }
                                                time { "Oct 1" }
                                            }
                                            p { "Back from Spain. A1c 8.1, up from 7.4 in June." }
                                            div { class: "a-acts",
                                                span { class: "a-btn",
                                                    icons::icon_47 {}
                                                    "Done"
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        section { class: "sec", id: "traits",
            div { class: "wrap",
                div { class: "sec-head center rv",
                    h2 { "Quiet until it matters." }
                    p { "Coda works through the day and brings you what needs a physician." }
                }
                div { class: "tiles",
                    div { class: "tile rv", style: "--d:0s",
                        div {
                            class: "frame tframe",
                            "data-art": "sage",
                            "data-seed": "31",
                            div {
                                class: "stage-wrap",
                                "data-h": "300",
                                "data-w": "420",
                                div { class: "stage t-src",
                                    div { class: "app a-card t-card",
                                        p { class: "a-body",
                                            "The pharmacy shows the 20 mg prescription picked up on August 16 "
                                            i { class: "a-cite", "data-c": "3", "[3]" }
                                            "."
                                        }
                                    }
                                    div { class: "app a-srcprev a-pop",
                                        span { class: "a-sl", "Source 3" }
                                        b { "Pharmacy record" }
                                        small { "Lisinopril 20 mg tablet · 30 days" }
                                        p { class: "a-kv",
                                            span { "Picked up" }
                                            b { "August 16" }
                                        }
                                        span { class: "a-open",
                                            "Open original"
                                            icons::icon_48 {}
                                        }
                                    }
                                }
                            }
                        }
                        h3 { "A source on every line" }
                        p {
                            "Every statement links to the record it came from. Open it in place, or in the original."
                        }
                    }
                    div { class: "tile rv", style: "--d:.06s",
                        div {
                            class: "frame tframe",
                            "data-art": "sage",
                            "data-seed": "32",
                            div {
                                class: "stage-wrap",
                                "data-h": "300",
                                "data-w": "420",
                                div { class: "stage t-bg",
                                    div { class: "t-stack",
                                        div { class: "app a-mini",
                                            div { class: "a-rh",
                                                b { "Elena Park" }
                                                span { class: "a-chip a-work",
                                                    span { class: "a-dots",
                                                        i {}
                                                        i {}
                                                        i {}
                                                    }
                                                    "Reviewing"
                                                }
                                            }
                                            p {
                                                class: "a-step",
                                                "data-steps": "Checking the EHR for new records|Reading the chart|Writing it up",
                                                "Checking the EHR for new records"
                                            }
                                        }
                                        div { class: "app a-mini",
                                            div { class: "a-rh",
                                                b { "Robert Hale" }
                                                span { class: "a-chip a-work",
                                                    span { class: "a-dots",
                                                        i {}
                                                        i {}
                                                        i {}
                                                    }
                                                    "Preparing visit"
                                                }
                                            }
                                            p {
                                                class: "a-step",
                                                "data-steps": "Reading the chart|Checking the literature|Writing it up",
                                                "Reading the chart"
                                            }
                                        }
                                        div { class: "app a-mini a-quiet",
                                            "Coda also reviewed 9 patients today"
                                            icons::icon_49 {}
                                        }
                                    }
                                }
                            }
                        }
                        h3 { "Working between visits" }
                        p {
                            "Coda reviews new records, results and messages as they arrive, and shows what it’s doing while it works."
                        }
                    }
                    div { class: "tile rv", style: "--d:.12s",
                        div {
                            class: "frame tframe",
                            "data-art": "sage",
                            "data-seed": "33",
                            div {
                                class: "stage-wrap",
                                "data-h": "300",
                                "data-w": "420",
                                div { class: "stage t-you",
                                    div { class: "t-stack",
                                        div { class: "app a-mini",
                                            div { class: "a-rh",
                                                b { "Jake Smith" }
                                                span { class: "a-chip a-reply",
                                                    icons::icon_50 {}
                                                    "Drafted a reply"
                                                }
                                            }
                                            p { "“Did my kidney test come back okay?”" }
                                            div { class: "a-acts",
                                                span { class: "a-btn",
                                                    icons::icon_51 {}
                                                    "Review reply"
                                                }
                                            }
                                        }
                                        div { class: "app a-mini",
                                            div { class: "a-rh",
                                                b { "Maya Bennett" }
                                                span { class: "a-chip a-flag",
                                                    icons::icon_52 {}
                                                    "Flagged"
                                                }
                                            }
                                            p { "Blood pressure unchanged after the dose increase." }
                                            div { class: "a-acts",
                                                span { class: "a-btn",
                                                    icons::icon_53 {}
                                                    "Discuss"
                                                }
                                                span { class: "a-btn",
                                                    icons::icon_54 {}
                                                    "Done"
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        h3 { "Nothing goes out without you" }
                        p { "Coda flags, prepares and drafts. You decide what happens next." }
                    }
                }
            }
        }
        section { class: "sec", id: "patients",
            div { class: "wrap",
                div { class: "sec-head center rv",
                    div { class: "duet-brand",
                        span { class: "duet-brand-lockup",
                            img {
                                alt: "",
                                class: "duet-brand-mark",
                                height: "400",
                                src: "/brand/duet/mark.svg",
                                width: "400",
                            }
                            img {
                                alt: "Duet",
                                class: "duet-brand-wordmark",
                                height: "170",
                                src: "/brand/duet/wordmark.svg",
                                width: "440",
                            }
                        }
                        span { class: "duet-brand-label", "for patients" }
                    }
                    h2 { "Understand your health between visits." }
                    p { "Track your readings. Prepare for your visit. Share only what you choose." }
                }
                Phones {}
            }
        }
        section { class: "sec", id: "memory",
            div { class: "wrap",
                div { class: "sec-head center rv",
                    span { class: "kicker",
                        b { "Patient" }
                        " memory"
                    }
                    h2 { "Every patient, one current picture." }
                    p {
                        "Records, results, messages and your conversations with Coda come together for each patient. Coda updates the picture as things change, and every line shows where it came from."
                    }
                }
                div {
                    class: "frame frame-wide rv",
                    "data-art": "deep",
                    "data-seed": "18",
                    div {
                        class: "stage-wrap",
                        "data-ch": "860",
                        "data-cw": "600",
                        "data-h": "720",
                        "data-w": "1280",
                        div {
                            aria_label: "Illustration: a patient summary laid out as a table. Each line shows what came before, the weeks since August, and what is true now. As new records arrive, the current value changes, and a 2024 reaction comes forward when a new symptom appears.",
                            class: "stage mem-stage",
                            id: "memStage",
                            div { class: "app mt-card",
                                div { class: "mt-top",
                                    img {
                                        alt: "",
                                        src: "/homepage/assets/people/maya-bennett.jpg",
                                    }
                                    b { "Maya Bennett" }
                                    span { class: "a-meta", "Illustrative patient" }
                                    span { class: "mt-w",
                                        "Summary written "
                                        b { "data-w": "" }
                                    }
                                }
                                div { class: "mt-head",
                                    span { class: "mt-k" }
                                    div { class: "mt-h",
                                        div { class: "mt-in",
                                            span { class: "mt-ht", "2019 to July" }
                                            span { class: "mt-pill mt-hpill" }
                                        }
                                    }
                                    div { class: "mt-l",
                                        div { class: "mt-in",
                                            span {
                                                class: "mt-tk",
                                                "data-d": "2026-08-01",
                                                "Aug"
                                            }
                                            span {
                                                class: "mt-tk",
                                                "data-d": "2026-09-01",
                                                "Sep"
                                            }
                                            span { class: "mt-tk mt-today", "Today" }
                                            span { class: "mt-pill mt-lpill" }
                                        }
                                    }
                                    span { class: "mt-v", "Patient summary" }
                                    span { class: "mt-s" }
                                }
                                div {
                                    class: "mt-row",
                                    "data-base": "Lisinopril 10 mg daily",
                                    "data-bat": "2022-03-10",
                                    "data-bic": "note",
                                    "data-ic": "note",
                                    span { class: "mt-k", "Blood pressure plan" }
                                    div { class: "mt-h",
                                        div { class: "mt-in",
                                            div { class: "mt-clip",
                                                div {
                                                    class: "mt-bar",
                                                    "data-from": "2022-03-10",
                                                    span { "10 mg" }
                                                }
                                            }
                                        }
                                    }
                                    div { class: "mt-l",
                                        div { class: "mt-in",
                                            div { class: "mt-clip",
                                                div {
                                                    class: "mt-bar",
                                                    "data-to": "2026-08-14",
                                                    span { "10 mg" }
                                                }
                                                div {
                                                    class: "mt-bar hi",
                                                    "data-from": "2026-08-14",
                                                    span { "20 mg" }
                                                }
                                            }
                                        }
                                    }
                                    span { class: "mt-v", "Lisinopril 20 mg daily" }
                                    span {
                                        aria_label: "Source: visit note",
                                        class: "mt-s",
                                        icons::icon_55 {}
                                    }
                                }
                                div {
                                    class: "mt-row",
                                    "data-base": "10 mg filled monthly",
                                    "data-bat": "2022-04-10",
                                    "data-bic": "rx",
                                    "data-ic": "rx",
                                    span { class: "mt-k", "Pharmacy" }
                                    div { class: "mt-h",
                                        div { class: "mt-in",
                                            span {
                                                class: "mt-cap",
                                                "data-d": "2022-04-10",
                                                "10 mg filled monthly"
                                            }
                                            div { class: "mt-clip",
                                                i {
                                                    class: "mt-span",
                                                    "data-from": "2022-04-10",
                                                    "data-to": "2026-07-18",
                                                }
                                            }
                                        }
                                    }
                                    div { class: "mt-l",
                                        div { class: "mt-in",
                                            i {
                                                class: "mt-dot",
                                                "data-d": "2026-08-16",
                                            }
                                            span {
                                                class: "mt-lab",
                                                "data-d": "2026-08-16",
                                                "20 mg picked up"
                                            }
                                        }
                                    }
                                    span { class: "mt-v", "20 mg picked up Aug 16" }
                                    span {
                                        aria_label: "Source: pharmacy record",
                                        class: "mt-s",
                                        icons::icon_56 {}
                                    }
                                }
                                div {
                                    class: "mt-row",
                                    "data-base": "No side effects reported",
                                    "data-bat": "2023-02-08",
                                    "data-bic": "msg",
                                    "data-ic": "msg",
                                    span { class: "mt-k", "Reported use" }
                                    div { class: "mt-h",
                                        div { class: "mt-in",
                                            span {
                                                class: "mt-cap",
                                                "data-d": "2023-02-08",
                                                "No side effects reported"
                                            }
                                            i {
                                                class: "mt-dot r",
                                                "data-d": "2023-02-08",
                                            }
                                            i {
                                                class: "mt-dot r",
                                                "data-d": "2023-11-20",
                                            }
                                            i {
                                                class: "mt-dot r",
                                                "data-d": "2024-06-03",
                                            }
                                            i {
                                                class: "mt-dot r",
                                                "data-d": "2025-01-15",
                                            }
                                            i {
                                                class: "mt-dot r",
                                                "data-d": "2025-10-02",
                                            }
                                            i {
                                                class: "mt-dot r",
                                                "data-d": "2026-04-21",
                                            }
                                        }
                                    }
                                    div { class: "mt-l",
                                        div { class: "mt-in",
                                            i {
                                                class: "mt-dot",
                                                "data-d": "2026-08-29",
                                            }
                                            span {
                                                class: "mt-lab",
                                                "data-d": "2026-08-29",
                                                "Started 20 mg"
                                            }
                                        }
                                    }
                                    span { class: "mt-v", "Started 20 mg on Aug 29" }
                                    span {
                                        aria_label: "Source: patient message",
                                        class: "mt-s",
                                        icons::icon_57 {}
                                    }
                                }
                                div {
                                    class: "mt-row",
                                    "data-base": "1.02 in May",
                                    "data-bat": "2026-05-12",
                                    "data-bic": "lab",
                                    "data-ic": "lab",
                                    span { class: "mt-k", "Creatinine" }
                                    div { class: "mt-h",
                                        div { class: "mt-in",
                                            span {
                                                class: "mt-cap",
                                                "data-d": "2020-05-02",
                                                "Between 0.96 and 1.03"
                                            }
                                            i {
                                                class: "mt-dot r",
                                                "data-d": "2020-05-02",
                                            }
                                            i {
                                                class: "mt-dot r",
                                                "data-d": "2021-06-10",
                                            }
                                            i {
                                                class: "mt-dot r",
                                                "data-d": "2022-08-20",
                                            }
                                            i {
                                                class: "mt-dot r",
                                                "data-d": "2023-09-14",
                                            }
                                            i {
                                                class: "mt-dot r",
                                                "data-d": "2024-10-02",
                                            }
                                            i {
                                                class: "mt-dot r",
                                                "data-d": "2025-05-20",
                                            }
                                            i {
                                                class: "mt-dot r",
                                                "data-d": "2026-05-12",
                                            }
                                        }
                                    }
                                    div { class: "mt-l",
                                        div { class: "mt-in",
                                            i {
                                                class: "mt-dot",
                                                "data-d": "2026-09-08",
                                            }
                                            span {
                                                class: "mt-lab rt",
                                                "data-d": "2026-09-08",
                                                "1.4"
                                            }
                                        }
                                    }
                                    span { class: "mt-v", "1.4 on Sep 8, up from 1.02" }
                                    span {
                                        aria_label: "Source: lab result",
                                        class: "mt-s",
                                        icons::icon_58 {}
                                    }
                                }
                                div {
                                    class: "mt-row",
                                    "data-base": "143/83 average",
                                    "data-bat": "2025-03-01",
                                    "data-bic": "pulse",
                                    "data-ic": "pulse",
                                    span { class: "mt-k", "Home blood pressure" }
                                    div { class: "mt-h",
                                        div { class: "mt-in",
                                            span {
                                                class: "mt-cap",
                                                "data-d": "2025-03-01",
                                                "143/83 average"
                                            }
                                            div { class: "mt-clip",
                                                i {
                                                    class: "mt-span",
                                                    "data-from": "2025-03-01",
                                                    "data-to": "2026-07-18",
                                                }
                                            }
                                        }
                                    }
                                    div { class: "mt-l",
                                        div { class: "mt-in",
                                            div { class: "mt-clip",
                                                i {
                                                    class: "mt-span",
                                                    "data-from": "2026-08-15",
                                                }
                                            }
                                            i {
                                                class: "mt-dot",
                                                "data-d": "2026-09-15",
                                            }
                                            span {
                                                class: "mt-lab rt",
                                                "data-d": "2026-09-15",
                                                "131/79 average"
                                            }
                                        }
                                    }
                                    span { class: "mt-v", "131/79 average" }
                                    span {
                                        aria_label: "Source: home readings",
                                        class: "mt-s",
                                        icons::icon_59 {}
                                    }
                                }
                                div {
                                    class: "mt-row",
                                    "data-base": "Amlodipine: ankle swelling, 2024",
                                    "data-bat": "2024-03-12",
                                    "data-bic": "clock",
                                    "data-ic": "msg",
                                    "data-reach": "",
                                    span { class: "mt-k", "Past reactions" }
                                    div { class: "mt-h",
                                        div { class: "mt-in",
                                            i {
                                                class: "mt-dot k24",
                                                "data-d": "2024-03-12",
                                            }
                                            span {
                                                class: "mt-lab rt",
                                                "data-d": "2024-03-12",
                                                "Amlodipine, 2024"
                                            }
                                        }
                                    }
                                    div { class: "mt-l",
                                        div { class: "mt-in",
                                            i {
                                                class: "mt-dot",
                                                "data-d": "2026-08-29",
                                            }
                                            span {
                                                class: "mt-lab",
                                                "data-d": "2026-08-29",
                                                "Dry cough"
                                            }
                                        }
                                    }
                                    span { class: "mt-v",
                                        "Lisinopril: dry cough, Aug 29"
                                        br {}
                                        "Amlodipine: ankle swelling, 2024"
                                    }
                                    span {
                                        aria_label: "Source: patient message and 2024 clinic note",
                                        class: "mt-s",
                                        icons::icon_60 {}
                                    }
                                }
                                i { class: "mt-reach" }
                            }
                        }
                    }
                }
            }
        }
        section { class: "sec sec-close", id: "contact",
            div { class: "wrap",
                div { class: "closer rv", "data-art": "deep", "data-seed": "3",
                    div { class: "closer-in",
                        h2 { "Bring Coda to your practice." }
                        p { "For physicians who want help with the work between visits." }
                        div { class: "closer-cta",
                            a {
                                class: "btn btn-light",
                                href: "mailto:hello@codahealth.ai",
                                "Partner with Coda "
                                icons::icon_61 {}
                            }
                            a {
                                class: "btn-ghost",
                                href: "mailto:hello@codahealth.ai",
                                "hello@codahealth.ai"
                            }
                        }
                    }
                }
            }
        }
    }
}

mod icons {
    use super::*;

    pub(crate) fn icon_0() -> Element {
        rsx! {
            svg { "aria-hidden": "true", view_box: "0 0 24 24",
                path { d: "M5 12h14m-6-6 6 6-6 6" }
            }
        }
    }

    pub(crate) fn icon_1() -> Element {
        rsx! {
            svg { "aria-hidden": "true", view_box: "0 0 24 24",
                path { d: "M4 11.5 12 5l8 6.5V20h-5.5v-5h-5v5H4z" }
            }
        }
    }

    pub(crate) fn icon_2() -> Element {
        rsx! {
            svg { "aria-hidden": "true", view_box: "0 0 24 24",
                path { d: "M5 5h14a1 1 0 0 1 1 1v9a1 1 0 0 1-1 1h-9l-4 3.5V16H5a1 1 0 0 1-1-1V6a1 1 0 0 1 1-1z" }
            }
        }
    }

    pub(crate) fn icon_3() -> Element {
        rsx! {
            svg { "aria-hidden": "true", view_box: "0 0 24 24",
                circle { cx: "11", cy: "11", r: "6" }
                path { d: "m20 20-4.5-4.5" }
            }
        }
    }

    pub(crate) fn icon_4() -> Element {
        rsx! {
            svg { "aria-hidden": "true", view_box: "0 0 24 24",
                path { d: "M5 5h14a1 1 0 0 1 1 1v9a1 1 0 0 1-1 1h-9l-4 3.5V16H5a1 1 0 0 1-1-1V6a1 1 0 0 1 1-1z" }
            }
        }
    }

    pub(crate) fn icon_5() -> Element {
        rsx! {
            svg { "aria-hidden": "true", view_box: "0 0 24 24",
                path { d: "M5 5h14a1 1 0 0 1 1 1v9a1 1 0 0 1-1 1h-9l-4 3.5V16H5a1 1 0 0 1-1-1V6a1 1 0 0 1 1-1z" }
            }
        }
    }

    pub(crate) fn icon_6() -> Element {
        rsx! {
            svg { "aria-hidden": "true", view_box: "0 0 24 24",
                path { d: "M5 5h14a1 1 0 0 1 1 1v9a1 1 0 0 1-1 1h-9l-4 3.5V16H5a1 1 0 0 1-1-1V6a1 1 0 0 1 1-1z" }
            }
        }
    }

    pub(crate) fn icon_7() -> Element {
        rsx! {
            svg { "aria-hidden": "true", view_box: "0 0 24 24",
                path { d: "M6 21V4m0 0h11l-2.5 4 2.5 4H6" }
            }
        }
    }

    pub(crate) fn icon_8() -> Element {
        rsx! {
            svg { "aria-hidden": "true", view_box: "0 0 24 24",
                path { d: "m5 12 4 4L19 6" }
            }
        }
    }

    pub(crate) fn icon_9() -> Element {
        rsx! {
            svg { "aria-hidden": "true", view_box: "0 0 24 24",
                path { d: "M9 7 4 12l5 5M4 12h10a6 6 0 0 1 6 6v1" }
            }
        }
    }

    pub(crate) fn icon_10() -> Element {
        rsx! {
            svg { "aria-hidden": "true", view_box: "0 0 24 24",
                path { d: "M9 7 4 12l5 5M4 12h10a6 6 0 0 1 6 6v1" }
            }
        }
    }

    pub(crate) fn icon_11() -> Element {
        rsx! {
            svg { "aria-hidden": "true", view_box: "0 0 24 24",
                path { d: "M6 21V4m0 0h11l-2.5 4 2.5 4H6" }
            }
        }
    }

    pub(crate) fn icon_12() -> Element {
        rsx! {
            svg { "aria-hidden": "true", view_box: "0 0 24 24",
                path { d: "m5 12 4 4L19 6" }
            }
        }
    }

    pub(crate) fn icon_13() -> Element {
        rsx! {
            svg { "aria-hidden": "true", view_box: "0 0 24 24",
                rect {
                    height: "11",
                    rx: "2",
                    width: "11",
                    x: "6.5",
                    y: "6.5",
                }
            }
        }
    }

    pub(crate) fn icon_14() -> Element {
        rsx! {
            svg { "aria-hidden": "true", view_box: "0 0 24 24",
                path { d: "m9 6 6 6-6 6" }
            }
        }
    }

    pub(crate) fn icon_15() -> Element {
        rsx! {
            svg { "aria-hidden": "true", view_box: "0 0 24 24",
                rect {
                    height: "15",
                    rx: "2",
                    width: "16",
                    x: "4",
                    y: "5",
                }
                path { d: "M8 3v4M16 3v4M4 10h16" }
            }
        }
    }

    pub(crate) fn icon_16() -> Element {
        rsx! {
            svg { "aria-hidden": "true", view_box: "0 0 24 24",
                path { d: "M6 21V4m0 0h11l-2.5 4 2.5 4H6" }
            }
        }
    }

    pub(crate) fn icon_17() -> Element {
        rsx! {
            svg { "aria-hidden": "true", view_box: "0 0 24 24",
                path { d: "M14 4h6v6M10 20H4v-6M20 4l-7 7M4 20l7-7" }
            }
        }
    }

    pub(crate) fn icon_18() -> Element {
        rsx! {
            svg { "aria-hidden": "true", view_box: "0 0 24 24",
                path { d: "M6 6l12 12M18 6 6 18" }
            }
        }
    }

    pub(crate) fn icon_19() -> Element {
        rsx! {
            svg { "aria-hidden": "true", view_box: "0 0 24 24",
                path {
                    d: "M12 19s-6-3.8-7.9-7.1C2.8 9.2 4.1 6.5 6.8 6.5c1.5 0 2.7.8 3.3 1.9h3.8c.6-1.1 1.8-1.9 3.3-1.9 2.7 0 4 2.7 2.7 5.4C18 15.2 12 19 12 19z",
                    fill: "#FF2D55",
                    stroke: "none",
                }
            }
        }
    }

    pub(crate) fn icon_20() -> Element {
        rsx! {
            svg {
                "aria-hidden": "true",
                class: "bp",
                preserve_aspect_ratio: "none",
                view_box: "0 0 640 120",
                line {
                    class: "axis",
                    x1: "0",
                    x2: "640",
                    y1: "100",
                    y2: "100",
                }
                line {
                    class: "grid",
                    x1: "0",
                    x2: "640",
                    y1: "60",
                    y2: "60",
                }
                path {
                    class: "area",
                    d: "M10,46 C60,43 100,48 150,44 S230,42 290,46 S390,41 460,45 S560,42 620,44 L620,100 L10,100 Z",
                }
                path {
                    class: "trend",
                    d: "M10,46 C60,43 100,48 150,44 S230,42 290,46 S390,41 460,45 S560,42 620,44",
                    path_length: "1",
                }
                g { class: "marker",
                    line {
                        x1: "205",
                        x2: "205",
                        y1: "14",
                        y2: "100",
                    }
                    text { x: "212", y: "20", "Rx change" }
                    circle { cx: "620", cy: "44", r: "4.5" }
                }
            }
        }
    }

    pub(crate) fn icon_21() -> Element {
        rsx! {
            svg { "aria-hidden": "true", view_box: "0 0 24 24",
                path { d: "m6 9 6 6 6-6" }
            }
        }
    }

    pub(crate) fn icon_22() -> Element {
        rsx! {
            svg { "aria-hidden": "true", view_box: "0 0 24 24",
                path { d: "m6 9 6 6 6-6" }
            }
        }
    }

    pub(crate) fn icon_23() -> Element {
        rsx! {
            svg { "aria-hidden": "true", view_box: "0 0 24 24",
                path { d: "M5 5h14a1 1 0 0 1 1 1v9a1 1 0 0 1-1 1h-9l-4 3.5V16H5a1 1 0 0 1-1-1V6a1 1 0 0 1 1-1z" }
            }
        }
    }

    pub(crate) fn icon_24() -> Element {
        rsx! {
            svg { "aria-hidden": "true", view_box: "0 0 24 24",
                path { d: "m5 12 4 4L19 6" }
            }
        }
    }

    pub(crate) fn icon_25() -> Element {
        rsx! {
            svg { "aria-hidden": "true", view_box: "0 0 24 24",
                path { d: "M14 4h6v6M10 20H4v-6M20 4l-7 7M4 20l7-7" }
            }
        }
    }

    pub(crate) fn icon_26() -> Element {
        rsx! {
            svg { "aria-hidden": "true", view_box: "0 0 24 24",
                path { d: "M6 6l12 12M18 6 6 18" }
            }
        }
    }

    pub(crate) fn icon_27() -> Element {
        rsx! {
            svg { "aria-hidden": "true", view_box: "0 0 24 24",
                rect {
                    height: "11",
                    rx: "3",
                    width: "6",
                    x: "9",
                    y: "3",
                }
                path { d: "M5 11a7 7 0 0 0 14 0M12 18v3" }
            }
        }
    }

    pub(crate) fn icon_28() -> Element {
        rsx! {
            svg { "aria-hidden": "true", view_box: "0 0 24 24",
                path { d: "M20 5v7a3 3 0 0 1-3 3H6m4-4-4 4 4 4" }
            }
        }
    }

    pub(crate) fn icon_29() -> Element {
        rsx! {
            svg { "aria-hidden": "true", view_box: "0 0 24 24",
                path { d: "M14 4h6v6M20 4l-9 9M18 14v5a1 1 0 0 1-1 1H5a1 1 0 0 1-1-1V7a1 1 0 0 1 1-1h5" }
            }
        }
    }

    pub(crate) fn icon_30() -> Element {
        rsx! {
            svg { "aria-hidden": "true", view_box: "0 0 24 24",
                path { d: "M14 4h6v6M20 4l-9 9M18 14v5a1 1 0 0 1-1 1H5a1 1 0 0 1-1-1V7a1 1 0 0 1 1-1h5" }
            }
        }
    }

    pub(crate) fn icon_31() -> Element {
        rsx! {
            svg { "aria-hidden": "true", class: "pointer", view_box: "0 0 24 24",
                path { d: "M5 3l14 8.2-6.1 1.3 3.6 6.9-2.5 1.3-3.6-6.9L6 18.6z" }
            }
        }
    }

    pub(crate) fn icon_32() -> Element {
        rsx! {
            svg { "aria-hidden": "true", view_box: "0 0 24 24",
                path { d: "M6 21V4m0 0h11l-2.5 4 2.5 4H6" }
            }
        }
    }

    pub(crate) fn icon_33() -> Element {
        rsx! {
            svg { "aria-hidden": "true", view_box: "0 0 24 24",
                path { d: "m5 12 4 4L19 6" }
            }
        }
    }

    pub(crate) fn icon_34() -> Element {
        rsx! {
            svg { "aria-hidden": "true", view_box: "0 0 24 24",
                path { d: "M9 7 4 12l5 5M4 12h10a6 6 0 0 1 6 6v1" }
            }
        }
    }

    pub(crate) fn icon_35() -> Element {
        rsx! {
            svg { "aria-hidden": "true", view_box: "0 0 24 24",
                path { d: "M9 7 4 12l5 5M4 12h10a6 6 0 0 1 6 6v1" }
            }
        }
    }

    pub(crate) fn icon_36() -> Element {
        rsx! {
            svg { "aria-hidden": "true", view_box: "0 0 24 24",
                path { d: "M6 21V4m0 0h11l-2.5 4 2.5 4H6" }
            }
        }
    }

    pub(crate) fn icon_37() -> Element {
        rsx! {
            svg { "aria-hidden": "true", view_box: "0 0 24 24",
                path { d: "m5 12 4 4L19 6" }
            }
        }
    }

    pub(crate) fn icon_38() -> Element {
        rsx! {
            svg { "aria-hidden": "true", view_box: "0 0 24 24",
                path { d: "m9 6 6 6-6 6" }
            }
        }
    }

    pub(crate) fn icon_39() -> Element {
        rsx! {
            svg { "aria-hidden": "true", view_box: "0 0 24 24",
                path { d: "m6 9 6 6 6-6" }
            }
        }
    }

    pub(crate) fn icon_40() -> Element {
        rsx! {
            svg { "aria-hidden": "true", view_box: "0 0 24 24",
                path { d: "m6 9 6 6 6-6" }
            }
        }
    }

    pub(crate) fn icon_41() -> Element {
        rsx! {
            svg { "aria-hidden": "true", view_box: "0 0 24 24",
                path { d: "M14 4h6v6M20 4l-9 9M18 14v5a1 1 0 0 1-1 1H5a1 1 0 0 1-1-1V7a1 1 0 0 1 1-1h5" }
            }
        }
    }

    pub(crate) fn icon_42() -> Element {
        rsx! {
            svg { "aria-hidden": "true", view_box: "0 0 24 24",
                rect {
                    height: "11",
                    rx: "3",
                    width: "6",
                    x: "9",
                    y: "3",
                }
                path { d: "M5 11a7 7 0 0 0 14 0M12 18v3" }
            }
        }
    }

    pub(crate) fn icon_43() -> Element {
        rsx! {
            svg { "aria-hidden": "true", view_box: "0 0 24 24",
                path { d: "M20 5v7a3 3 0 0 1-3 3H6m4-4-4 4 4 4" }
            }
        }
    }

    pub(crate) fn icon_44() -> Element {
        rsx! {
            svg { "aria-hidden": "true", view_box: "0 0 24 24",
                path { d: "M17 2l3 3-3 3M4 11V9a4 4 0 0 1 4-4h12M7 22l-3-3 3-3M20 13v2a4 4 0 0 1-4 4H4" }
            }
        }
    }

    pub(crate) fn icon_45() -> Element {
        rsx! {
            svg { "aria-hidden": "true", view_box: "0 0 24 24",
                path { d: "M17 2l3 3-3 3M4 11V9a4 4 0 0 1 4-4h12M7 22l-3-3 3-3M20 13v2a4 4 0 0 1-4 4H4" }
            }
        }
    }

    pub(crate) fn icon_46() -> Element {
        rsx! {
            svg { "aria-hidden": "true", view_box: "0 0 24 24",
                path { d: "M6 21V4m0 0h11l-2.5 4 2.5 4H6" }
            }
        }
    }

    pub(crate) fn icon_47() -> Element {
        rsx! {
            svg { "aria-hidden": "true", view_box: "0 0 24 24",
                path { d: "m5 12 4 4L19 6" }
            }
        }
    }

    pub(crate) fn icon_48() -> Element {
        rsx! {
            svg { "aria-hidden": "true", view_box: "0 0 24 24",
                path { d: "M14 4h6v6M20 4l-9 9M18 14v5a1 1 0 0 1-1 1H5a1 1 0 0 1-1-1V7a1 1 0 0 1 1-1h5" }
            }
        }
    }

    pub(crate) fn icon_49() -> Element {
        rsx! {
            svg { "aria-hidden": "true", view_box: "0 0 24 24",
                path { d: "m9 6 6 6-6 6" }
            }
        }
    }

    pub(crate) fn icon_50() -> Element {
        rsx! {
            svg { "aria-hidden": "true", view_box: "0 0 24 24",
                path { d: "M9 7 4 12l5 5M4 12h10a6 6 0 0 1 6 6v1" }
            }
        }
    }

    pub(crate) fn icon_51() -> Element {
        rsx! {
            svg { "aria-hidden": "true", view_box: "0 0 24 24",
                path { d: "M9 7 4 12l5 5M4 12h10a6 6 0 0 1 6 6v1" }
            }
        }
    }

    pub(crate) fn icon_52() -> Element {
        rsx! {
            svg { "aria-hidden": "true", view_box: "0 0 24 24",
                path { d: "M6 21V4m0 0h11l-2.5 4 2.5 4H6" }
            }
        }
    }

    pub(crate) fn icon_53() -> Element {
        rsx! {
            svg { "aria-hidden": "true", view_box: "0 0 24 24",
                path { d: "M5 5h14a1 1 0 0 1 1 1v9a1 1 0 0 1-1 1h-9l-4 3.5V16H5a1 1 0 0 1-1-1V6a1 1 0 0 1 1-1z" }
            }
        }
    }

    pub(crate) fn icon_54() -> Element {
        rsx! {
            svg { "aria-hidden": "true", view_box: "0 0 24 24",
                path { d: "m5 12 4 4L19 6" }
            }
        }
    }

    pub(crate) fn icon_55() -> Element {
        rsx! {
            svg { view_box: "0 0 24 24",
                rect {
                    height: "18",
                    rx: "2",
                    width: "14",
                    x: "5",
                    y: "3",
                }
                path { d: "M9 8h6M9 12h6M9 16h4" }
            }
        }
    }

    pub(crate) fn icon_56() -> Element {
        rsx! {
            svg { view_box: "0 0 24 24",
                rect {
                    height: "7",
                    rx: "3.5",
                    transform: "rotate(-35 12 12)",
                    width: "18",
                    x: "3",
                    y: "8.5",
                }
                path { d: "m9.2 8 5.6 8" }
            }
        }
    }

    pub(crate) fn icon_57() -> Element {
        rsx! {
            svg { view_box: "0 0 24 24",
                path { d: "M5 5h14a1 1 0 0 1 1 1v9a1 1 0 0 1-1 1h-9l-4 3.5V16H5a1 1 0 0 1-1-1V6a1 1 0 0 1 1-1z" }
            }
        }
    }

    pub(crate) fn icon_58() -> Element {
        rsx! {
            svg { view_box: "0 0 24 24",
                path { d: "M9 3h6M10 3v6l-5 9a2 2 0 0 0 1.7 3h10.6a2 2 0 0 0 1.7-3l-5-9V3" }
                path { d: "M7.5 15h9" }
            }
        }
    }

    pub(crate) fn icon_59() -> Element {
        rsx! {
            svg { view_box: "0 0 24 24",
                path { d: "M3 12h4l3-7 4 14 3-7h4" }
            }
        }
    }

    pub(crate) fn icon_60() -> Element {
        rsx! {
            svg { view_box: "0 0 24 24",
                path { d: "M5 5h14a1 1 0 0 1 1 1v9a1 1 0 0 1-1 1h-9l-4 3.5V16H5a1 1 0 0 1-1-1V6a1 1 0 0 1 1-1z" }
            }
        }
    }

    pub(crate) fn icon_61() -> Element {
        rsx! {
            svg { "aria-hidden": "true", view_box: "0 0 24 24",
                path { d: "M5 12h14m-6-6 6 6-6 6" }
            }
        }
    }
}
