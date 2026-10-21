use dioxus::prelude::*;

#[component]
pub fn Phones() -> Element {
    rsx! {
        div { class: "frame frame-wide rv", "data-art": "dawn", "data-seed": "5",
            div {
                class: "stage-wrap",
                "data-ch": "820",
                "data-cw": "640",
                "data-h": "760",
                "data-w": "1280",
                div { class: "stage duet-stage", "data-play": "duet",
                    div { class: "phone p1",
                        div { class: "scr scr-chat",
                            div { class: "ph-status",
                                span { "9:41" }
                                span { class: "ph-ic",
                                    icons::icon_0 {}
                                    i { class: "batt" }
                                }
                            }
                            i { class: "island" }
                            div { class: "ph-who",
                                img { alt: "", src: "/homepage/assets/sprout.webp" }
                                b { "Sprout" }
                            }
                            div { class: "chat",
                                div { class: "cm me", "Can you help me get ready for Tuesday’s visit?" }
                                div { class: "cm du",
                                    "Sure. Your home blood pressure has stayed close to your recent average. What would you like your doctor to know?"
                                }
                                div { class: "cm me",
                                    "I’m still on lisinopril 10 mg. I picked up the 20 mg but haven’t started it."
                                }
                                div { class: "cm du",
                                    "I added that to For my doctor, in your words. You can edit the page before you share it."
                                }
                            }
                            div { class: "ph-attach",
                                span { class: "ic", icons::icon_1 {} }
                                div {
                                    b { "For my doctor" }
                                    small { "Your saved page" }
                                }
                                icons::icon_2 {}
                            }
                            div { class: "ph-compose",
                                span { "Message Sprout" }
                                i { icons::icon_3 {} }
                            }
                            nav { class: "ph-tabs",
                                span { class: "on",
                                    icons::icon_4 {}
                                    "Duet"
                                }
                                span {
                                    icons::icon_5 {}
                                    "Today"
                                }
                                span {
                                    icons::icon_6 {}
                                    "You"
                                }
                                span {
                                    icons::icon_7 {}
                                    "Shared"
                                }
                            }
                        }
                    }
                    div { class: "phone p2",
                        div { class: "scr scr-today",
                            div { class: "ph-status",
                                span { "9:41" }
                                span { class: "ph-ic",
                                    icons::icon_8 {}
                                    i { class: "batt" }
                                }
                            }
                            i { class: "island" }
                            div { class: "ph-dateRow",
                                span { "Thursday, October 8" }
                                i { "Edit" }
                            }
                            b { class: "ph-large", "Good morning, Maya" }
                            div { class: "ph-moment",
                                div { class: "ph-mh",
                                    img {
                                        alt: "",
                                        src: "/homepage/assets/sprout.webp",
                                    }
                                    span { "From Tuesday" }
                                    icons::icon_9 {}
                                }
                                b { "Home blood pressure stayed close to your recent average" }
                                div { class: "ph-pair",
                                    div {
                                        b {
                                            "143/83 "
                                            small { "mmHg" }
                                        }
                                        small { "7-day average" }
                                    }
                                    div {
                                        b {
                                            "12 "
                                            small { "readings" }
                                        }
                                        small { "Home cuff" }
                                    }
                                }
                                p { "Recorded readings were similar to the prior two weeks." }
                            }
                            div { class: "ph-stat",
                                div { class: "ph-sh",
                                    span {
                                        icons::icon_10 {}
                                        "Blood pressure"
                                    }
                                    small {
                                        "Today "
                                        icons::icon_11 {}
                                    }
                                }
                                b {
                                    "142/82 "
                                    small { "mmHg" }
                                }
                                icons::icon_12 {}
                            }
                            div { class: "ph-stat ph-sleep",
                                div { class: "ph-sh",
                                    span {
                                        icons::icon_13 {}
                                        "Sleep"
                                    }
                                    small {
                                        "Today "
                                        icons::icon_14 {}
                                    }
                                }
                                b {
                                    "7 "
                                    small { "hr" }
                                    " 24 "
                                    small { "min" }
                                }
                                div { class: "bars",
                                    i { style: "height:78%" }
                                    i { style: "height:84%" }
                                    i { style: "height:70%" }
                                    i { style: "height:88%" }
                                    i { style: "height:82%" }
                                    i { style: "height:76%" }
                                    i { class: "last", style: "height:86%" }
                                }
                            }
                            nav { class: "ph-tabs",
                                span {
                                    icons::icon_15 {}
                                    "Duet"
                                }
                                span { class: "on",
                                    icons::icon_16 {}
                                    "Today"
                                }
                                span {
                                    icons::icon_17 {}
                                    "You"
                                }
                                span {
                                    icons::icon_18 {}
                                    "Shared"
                                }
                            }
                        }
                    }
                    div { class: "phone p3",
                        div { class: "scr scr-doc",
                            div { class: "ph-status",
                                span { "9:41" }
                                span { class: "ph-ic",
                                    icons::icon_19 {}
                                    i { class: "batt" }
                                }
                            }
                            i { class: "island" }
                            div { class: "sheet",
                                i { class: "grab" }
                                div { class: "sheet-h",
                                    b { "For my doctor" }
                                    span { class: "xbtn", icons::icon_20 {} }
                                }
                                div { class: "sheet-body",
                                    div { class: "doc-intro",
                                        p {
                                            "Your words and the information you choose, on one page for your visit."
                                        }
                                        a { "Edit page" }
                                    }
                                    span { class: "doc-lbl", "Page preview" }
                                    div { class: "paper",
                                        b { class: "paper-h", "For my doctor" }
                                        div { class: "paper-who",
                                            b { "Maya Bennett" }
                                            span { "Prepared Oct 8, 2026" }
                                        }
                                        div { class: "ps",
                                            small { "What I want to discuss" }
                                            p {
                                                "I’m still taking lisinopril 10 mg and haven’t started the 20 mg. I’d like to review my readings."
                                            }
                                        }
                                        div { class: "ps",
                                            small { "My questions" }
                                            p { "1. Should I start the higher dose now?" }
                                        }
                                        div { class: "ps",
                                            small { "From my records" }
                                            p {
                                                b { "Home blood pressure" }
                                                br {}
                                                "143/83 mmHg, 7-day average · Apple Health"
                                            }
                                        }
                                        div { class: "ps",
                                            small { "What I take (confirmed by me)" }
                                            p {
                                                b { "Lisinopril 10 mg" }
                                                br {}
                                                "Once daily"
                                            }
                                        }
                                        p { class: "paper-foot",
                                            "Prepared by me with help from Duet, an AI app, and reviewed by me"
                                        }
                                    }
                                }
                                div { class: "share-btn",
                                    icons::icon_21 {}
                                    "Share page"
                                }
                            }
                        }
                    }
                }
            }
        }
        div { class: "duet-labels",
            div { class: "rv",
                span { class: "idx",
                    i {}
                    "01"
                }
                h3 { "Talk with Duet" }
            }
            div { class: "rv", style: "--d:.06s",
                span { class: "idx",
                    i {}
                    "02"
                }
                h3 { "Today" }
            }
            div { class: "rv", style: "--d:.12s",
                span { class: "idx",
                    i {}
                    "03"
                }
                h3 { "For my doctor" }
            }
        }
    }
}

mod icons {
    use super::*;

    pub(crate) fn icon_0() -> Element {
        rsx! {
            svg { "aria-hidden": "true", view_box: "0 0 18 12",
                path { d: "M9 11.2 7 9.1a2.8 2.8 0 0 1 4 0zM4.6 6.8a6.2 6.2 0 0 1 8.8 0l-1.3 1.3a4.4 4.4 0 0 0-6.2 0zM2.2 4.4a9.6 9.6 0 0 1 13.6 0l-1.3 1.3a7.8 7.8 0 0 0-11 0z" }
            }
        }
    }

    pub(crate) fn icon_1() -> Element {
        rsx! {
            svg { "aria-hidden": "true", view_box: "0 0 24 24",
                rect {
                    height: "13",
                    rx: "3",
                    width: "18",
                    x: "3",
                    y: "7",
                }
                path { d: "M9 7V5.5A1.5 1.5 0 0 1 10.5 4h3A1.5 1.5 0 0 1 15 5.5V7M12 10.5v6M9 13.5h6" }
            }
        }
    }

    pub(crate) fn icon_2() -> Element {
        rsx! {
            svg { "aria-hidden": "true", class: "chev", view_box: "0 0 24 24",
                path { d: "m9 5 7 7-7 7" }
            }
        }
    }

    pub(crate) fn icon_3() -> Element {
        rsx! {
            svg { "aria-hidden": "true", view_box: "0 0 24 24",
                path { d: "M12 19V5M6 11l6-6 6 6" }
            }
        }
    }

    pub(crate) fn icon_4() -> Element {
        rsx! {
            svg { "aria-hidden": "true", view_box: "0 0 24 24",
                path { d: "M3 5.5A2.5 2.5 0 0 1 5.5 3h8A2.5 2.5 0 0 1 16 5.5v5a2.5 2.5 0 0 1-2.5 2.5H8l-3.5 3v-3H5.5A2.5 2.5 0 0 1 3 10.5z" }
                path { d: "M18 8.2h.5A2.5 2.5 0 0 1 21 10.7v4.8a2.5 2.5 0 0 1-2.5 2.5h-.5v3l-3.5-3H11a2.5 2.5 0 0 1-2.4-1.8" }
            }
        }
    }

    pub(crate) fn icon_5() -> Element {
        rsx! {
            svg { "aria-hidden": "true", view_box: "0 0 24 24",
                circle { cx: "12", cy: "12", r: "4.2" }
                path { d: "M12 2.5v2.2M12 19.3v2.2M2.5 12h2.2M19.3 12h2.2M5.3 5.3l1.6 1.6M17.1 17.1l1.6 1.6M5.3 18.7l1.6-1.6M17.1 6.9l1.6-1.6" }
            }
        }
    }

    pub(crate) fn icon_6() -> Element {
        rsx! {
            svg { "aria-hidden": "true", view_box: "0 0 24 24",
                circle { cx: "12", cy: "12", r: "9.5" }
                circle {
                    class: "f",
                    cx: "12",
                    cy: "9.6",
                    r: "3.3",
                }
                path {
                    class: "f",
                    d: "M5.8 18.4c1.3-2.3 3.6-3.6 6.2-3.6s4.9 1.3 6.2 3.6A8.6 8.6 0 0 1 12 21a8.6 8.6 0 0 1-6.2-2.6z",
                }
            }
        }
    }

    pub(crate) fn icon_7() -> Element {
        rsx! {
            svg { "aria-hidden": "true", view_box: "0 0 24 24",
                circle {
                    class: "f",
                    cx: "8.5",
                    cy: "8",
                    r: "3.2",
                }
                circle {
                    class: "f",
                    cx: "16.5",
                    cy: "8.5",
                    r: "2.7",
                }
                path {
                    class: "f",
                    d: "M2.5 18.5c.4-3.3 2.9-5.4 6-5.4s5.6 2.1 6 5.4zM15.2 18.5c-.2-1.9-.9-3.4-2-4.5a5 5 0 0 1 3.3-1c2.7 0 4.6 1.9 5 5.5z",
                }
            }
        }
    }

    pub(crate) fn icon_8() -> Element {
        rsx! {
            svg { "aria-hidden": "true", view_box: "0 0 18 12",
                path { d: "M9 11.2 7 9.1a2.8 2.8 0 0 1 4 0zM4.6 6.8a6.2 6.2 0 0 1 8.8 0l-1.3 1.3a4.4 4.4 0 0 0-6.2 0zM2.2 4.4a9.6 9.6 0 0 1 13.6 0l-1.3 1.3a7.8 7.8 0 0 0-11 0z" }
            }
        }
    }

    pub(crate) fn icon_9() -> Element {
        rsx! {
            svg { "aria-hidden": "true", class: "chev", view_box: "0 0 24 24",
                path { d: "m9 5 7 7-7 7" }
            }
        }
    }

    pub(crate) fn icon_10() -> Element {
        rsx! {
            svg { "aria-hidden": "true", view_box: "0 0 24 24",
                path { d: "M12 20s-7-4.4-9-8.3C1.6 8.6 3.4 5 6.8 5c2 0 3.4 1.1 4.2 2.4h2c.8-1.3 2.2-2.4 4.2-2.4 3.4 0 5.2 3.6 3.8 6.7C19 15.6 12 20 12 20z" }
            }
        }
    }

    pub(crate) fn icon_11() -> Element {
        rsx! {
            svg { "aria-hidden": "true", class: "chev", view_box: "0 0 24 24",
                path { d: "m9 5 7 7-7 7" }
            }
        }
    }

    pub(crate) fn icon_12() -> Element {
        rsx! {
            svg { "aria-hidden": "true", class: "ph-chart", view_box: "0 0 260 74",
                path { class: "grid", d: "M0 14H236M0 40H236M0 66H236" }
                text { x: "242", y: "17", "150" }
                text { x: "242", y: "43", "135" }
                text { x: "242", y: "69", "120" }
                path { class: "avg", d: "M0 26.1H236" }
                text { class: "avgt", x: "0", y: "21.1", "avg" }
                path {
                    class: "bpl",
                    d: "M28 20.9L61 33.1L94 15.7L127 29.6L160 24.4L193 34.8L226 27.9",
                    path_length: "1",
                }
                g { class: "dots",
                    circle { cx: "28", cy: "20.9", r: "2.6" }
                    circle { cx: "61", cy: "33.1", r: "2.6" }
                    circle { cx: "94", cy: "15.7", r: "2.6" }
                    circle { cx: "127", cy: "29.6", r: "2.6" }
                    circle { cx: "160", cy: "24.4", r: "2.6" }
                    circle { cx: "193", cy: "34.8", r: "2.6" }
                    circle {
                        class: "last",
                        cx: "226",
                        cy: "27.9",
                        r: "4",
                    }
                }
            }
        }
    }

    pub(crate) fn icon_13() -> Element {
        rsx! {
            svg { "aria-hidden": "true", view_box: "0 0 24 24",
                path { d: "M20 14.5A8 8 0 0 1 9.5 4a8 8 0 1 0 10.5 10.5z" }
            }
        }
    }

    pub(crate) fn icon_14() -> Element {
        rsx! {
            svg { "aria-hidden": "true", class: "chev", view_box: "0 0 24 24",
                path { d: "m9 5 7 7-7 7" }
            }
        }
    }

    pub(crate) fn icon_15() -> Element {
        rsx! {
            svg { "aria-hidden": "true", view_box: "0 0 24 24",
                path { d: "M3 5.5A2.5 2.5 0 0 1 5.5 3h8A2.5 2.5 0 0 1 16 5.5v5a2.5 2.5 0 0 1-2.5 2.5H8l-3.5 3v-3H5.5A2.5 2.5 0 0 1 3 10.5z" }
                path { d: "M18 8.2h.5A2.5 2.5 0 0 1 21 10.7v4.8a2.5 2.5 0 0 1-2.5 2.5h-.5v3l-3.5-3H11a2.5 2.5 0 0 1-2.4-1.8" }
            }
        }
    }

    pub(crate) fn icon_16() -> Element {
        rsx! {
            svg { "aria-hidden": "true", view_box: "0 0 24 24",
                circle { cx: "12", cy: "12", r: "4.2" }
                path { d: "M12 2.5v2.2M12 19.3v2.2M2.5 12h2.2M19.3 12h2.2M5.3 5.3l1.6 1.6M17.1 17.1l1.6 1.6M5.3 18.7l1.6-1.6M17.1 6.9l1.6-1.6" }
            }
        }
    }

    pub(crate) fn icon_17() -> Element {
        rsx! {
            svg { "aria-hidden": "true", view_box: "0 0 24 24",
                circle { cx: "12", cy: "12", r: "9.5" }
                circle {
                    class: "f",
                    cx: "12",
                    cy: "9.6",
                    r: "3.3",
                }
                path {
                    class: "f",
                    d: "M5.8 18.4c1.3-2.3 3.6-3.6 6.2-3.6s4.9 1.3 6.2 3.6A8.6 8.6 0 0 1 12 21a8.6 8.6 0 0 1-6.2-2.6z",
                }
            }
        }
    }

    pub(crate) fn icon_18() -> Element {
        rsx! {
            svg { "aria-hidden": "true", view_box: "0 0 24 24",
                circle {
                    class: "f",
                    cx: "8.5",
                    cy: "8",
                    r: "3.2",
                }
                circle {
                    class: "f",
                    cx: "16.5",
                    cy: "8.5",
                    r: "2.7",
                }
                path {
                    class: "f",
                    d: "M2.5 18.5c.4-3.3 2.9-5.4 6-5.4s5.6 2.1 6 5.4zM15.2 18.5c-.2-1.9-.9-3.4-2-4.5a5 5 0 0 1 3.3-1c2.7 0 4.6 1.9 5 5.5z",
                }
            }
        }
    }

    pub(crate) fn icon_19() -> Element {
        rsx! {
            svg { "aria-hidden": "true", view_box: "0 0 18 12",
                path { d: "M9 11.2 7 9.1a2.8 2.8 0 0 1 4 0zM4.6 6.8a6.2 6.2 0 0 1 8.8 0l-1.3 1.3a4.4 4.4 0 0 0-6.2 0zM2.2 4.4a9.6 9.6 0 0 1 13.6 0l-1.3 1.3a7.8 7.8 0 0 0-11 0z" }
            }
        }
    }

    pub(crate) fn icon_20() -> Element {
        rsx! {
            svg { "aria-hidden": "true", view_box: "0 0 24 24",
                path { d: "M7 7l10 10M17 7 7 17" }
            }
        }
    }

    pub(crate) fn icon_21() -> Element {
        rsx! {
            svg { "aria-hidden": "true", view_box: "0 0 24 24",
                path { d: "M12 15V3M8 7l4-4 4 4M7 11H6a2 2 0 0 0-2 2v6a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2v-6a2 2 0 0 0-2-2h-1" }
            }
        }
    }
}
