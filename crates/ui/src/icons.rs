//! The shared icons, drawn on one 24-unit grid at one stroke.

use dioxus::prelude::*;

/// The stroke width every icon draws with.
pub const STROKE: &str = "1.8";

/// The icons in the shared set, each named for what it shows.
#[allow(dead_code, missing_docs)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IconName {
    Alert,
    Archive,
    ArrowLeft,
    ArrowRight,
    Calendar,
    Check,
    ChevronDown,
    Clipboard,
    Copy,
    Expand,
    ExternalLink,
    Flag,
    History,
    Home,
    Keyboard,
    Link,
    Menu,
    Message,
    Microphone,
    Minimize,
    PanelRightClose,
    PanelRightOpen,
    Plus,
    Refresh,
    Repeat,
    Reply,
    Search,
    Sparkle,
    SidebarCollapse,
    SidebarExpand,
    Square,
    ThumbDown,
    ThumbUp,
    Undo,
    User,
    Users,
    X,
    ArrowUp,
    ChevronLeft,
    ChevronRight,
    Book,
    Clock,
    File,
    Heart,
    Moon,
    Settings,
    Share,
    Steps,
    Sun,
    More,
    Pencil,
    Trash,
}

impl IconName {
    /// Every icon in the set.
    pub const ALL: [IconName; 52] = [
        IconName::Alert,
        IconName::Archive,
        IconName::ArrowLeft,
        IconName::ArrowRight,
        IconName::Calendar,
        IconName::Check,
        IconName::ChevronDown,
        IconName::Clipboard,
        IconName::Copy,
        IconName::Expand,
        IconName::ExternalLink,
        IconName::Flag,
        IconName::History,
        IconName::Home,
        IconName::Keyboard,
        IconName::Link,
        IconName::Menu,
        IconName::Message,
        IconName::Microphone,
        IconName::Minimize,
        IconName::PanelRightClose,
        IconName::PanelRightOpen,
        IconName::Plus,
        IconName::Refresh,
        IconName::Repeat,
        IconName::Reply,
        IconName::Search,
        IconName::Sparkle,
        IconName::SidebarCollapse,
        IconName::SidebarExpand,
        IconName::Square,
        IconName::ThumbDown,
        IconName::ThumbUp,
        IconName::Undo,
        IconName::User,
        IconName::Users,
        IconName::X,
        IconName::ArrowUp,
        IconName::ChevronLeft,
        IconName::ChevronRight,
        IconName::Book,
        IconName::Clock,
        IconName::File,
        IconName::Heart,
        IconName::Moon,
        IconName::Settings,
        IconName::Share,
        IconName::Steps,
        IconName::Sun,
        IconName::More,
        IconName::Pencil,
        IconName::Trash,
    ];

    /// This icon's name in lowercase words joined by hyphens, such as `arrow-left`.
    pub fn name(self) -> &'static str {
        match self {
            IconName::Alert => "alert",
            IconName::Archive => "archive",
            IconName::ArrowLeft => "arrow-left",
            IconName::ArrowRight => "arrow-right",
            IconName::Calendar => "calendar",
            IconName::Check => "check",
            IconName::ChevronDown => "chevron-down",
            IconName::Clipboard => "clipboard",
            IconName::Copy => "copy",
            IconName::Expand => "expand",
            IconName::ExternalLink => "external-link",
            IconName::Flag => "flag",
            IconName::History => "history",
            IconName::Home => "home",
            IconName::Keyboard => "keyboard",
            IconName::Link => "link",
            IconName::Menu => "menu",
            IconName::Message => "message",
            IconName::Microphone => "microphone",
            IconName::Minimize => "minimize",
            IconName::PanelRightClose => "panel-right-close",
            IconName::PanelRightOpen => "panel-right-open",
            IconName::Plus => "plus",
            IconName::Refresh => "refresh",
            IconName::Repeat => "repeat",
            IconName::Reply => "reply",
            IconName::Search => "search",
            IconName::Sparkle => "sparkle",
            IconName::SidebarCollapse => "sidebar-collapse",
            IconName::SidebarExpand => "sidebar-expand",
            IconName::Square => "square",
            IconName::ThumbDown => "thumb-down",
            IconName::ThumbUp => "thumb-up",
            IconName::Undo => "undo",
            IconName::User => "user",
            IconName::Users => "users",
            IconName::X => "x",
            IconName::ArrowUp => "arrow-up",
            IconName::ChevronLeft => "chevron-left",
            IconName::ChevronRight => "chevron-right",
            IconName::Book => "book",
            IconName::Clock => "clock",
            IconName::File => "file",
            IconName::Heart => "heart",
            IconName::Moon => "moon",
            IconName::Settings => "settings",
            IconName::Share => "share",
            IconName::Steps => "steps",
            IconName::Sun => "sun",
            IconName::More => "more",
            IconName::Pencil => "pencil",
            IconName::Trash => "trash",
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Mark {
    Path(&'static str),
    Rect(
        &'static str,
        &'static str,
        &'static str,
        &'static str,
        Option<&'static str>,
    ),
    Circle(&'static str, &'static str, &'static str),
}

fn marks(name: IconName) -> &'static [Mark] {
    match name {
        IconName::Alert => &[
            Mark::Path("M12 3 2.8 19h18.4L12 3Z"),
            Mark::Path("M12 9v4"),
            Mark::Path("M12 16.5h.01"),
        ],
        IconName::Archive => &[
            Mark::Rect("3", "4", "18", "5", Some("1")),
            Mark::Path("M5 9v10a1 1 0 0 0 1 1h12a1 1 0 0 0 1-1V9"),
            Mark::Path("M10 13h4"),
        ],
        IconName::ArrowLeft => &[Mark::Path("M19 12H5"), Mark::Path("m12 19-7-7 7-7")],
        IconName::ArrowRight => &[Mark::Path("M5 12h14"), Mark::Path("m12 5 7 7-7 7")],
        IconName::Calendar => &[
            Mark::Rect("3", "5", "18", "16", Some("2")),
            Mark::Path("M16 3v4M8 3v4M3 10h18"),
        ],
        IconName::Check => &[Mark::Path("m5 12 4 4L19 6")],
        IconName::ChevronDown => &[Mark::Path("m6 9 6 6 6-6")],
        IconName::Clipboard => &[
            Mark::Rect("8", "2", "8", "4", Some("1")),
            Mark::Path("M16 4h2a2 2 0 0 1 2 2v14a2 2 0 0 1-2 2H6a2 2 0 0 1-2-2V6a2 2 0 0 1 2-2h2"),
            Mark::Path("M9 12h6M9 16h4"),
        ],
        IconName::Copy => &[
            Mark::Rect("9", "9", "12", "12", Some("2")),
            Mark::Path("M5 15V5a2 2 0 0 1 2-2h10"),
        ],
        IconName::Expand => &[Mark::Path("M14 4h6v6M10 20H4v-6M20 4l-7 7M4 20l7-7")],
        IconName::ExternalLink => &[
            Mark::Path("M15 4h5v5"),
            Mark::Path("m10 14 10-10"),
            Mark::Path("M20 13v5a2 2 0 0 1-2 2H6a2 2 0 0 1-2-2V6a2 2 0 0 1 2-2h5"),
        ],
        IconName::Flag => &[Mark::Path("M5 21V4"), Mark::Path("M5 4h11l-2 4 2 4H5")],
        IconName::History => &[
            Mark::Path("M3 12a9 9 0 1 0 3-6.7L3 8"),
            Mark::Path("M3 3v5h5"),
            Mark::Path("M12 7v5l3 2"),
        ],
        IconName::Home => &[
            Mark::Path("m3 11 9-8 9 8"),
            Mark::Path("M5 10v10h14V10"),
            Mark::Path("M9 20v-6h6v6"),
        ],
        IconName::Keyboard => &[
            Mark::Rect("2", "6", "20", "12", Some("2")),
            Mark::Path("M6 10h.01M10 10h.01M14 10h.01M18 10h.01M7 14h10"),
        ],
        IconName::Link => &[
            Mark::Path("M10 14a5 5 0 0 0 7 0l3-3a5 5 0 0 0-7-7l-1 1"),
            Mark::Path("M14 10a5 5 0 0 0-7 0l-3 3a5 5 0 0 0 7 7l1-1"),
        ],
        IconName::Menu => &[
            Mark::Path("M4 7h16"),
            Mark::Path("M4 12h16"),
            Mark::Path("M4 17h16"),
        ],
        IconName::Message => &[Mark::Path(
            "M21 15a4 4 0 0 1-4 4H8l-5 3V7a4 4 0 0 1 4-4h10a4 4 0 0 1 4 4Z",
        )],
        IconName::Microphone => &[
            Mark::Rect("9", "2", "6", "12", Some("3")),
            Mark::Path("M5 10v2a7 7 0 0 0 14 0v-2M12 19v3M8 22h8"),
        ],
        IconName::Minimize => &[
            Mark::Path("M4 4h16v16H4z"),
            Mark::Path("m8 8 7 7M10 15h5v-5"),
        ],
        IconName::PanelRightClose => &[
            Mark::Rect("3", "4", "18", "16", Some("2")),
            Mark::Path("M15 4v16M9 9l3 3-3 3"),
        ],
        IconName::PanelRightOpen => &[
            Mark::Rect("3", "4", "18", "16", Some("2")),
            Mark::Path("M15 4v16M11 9l-3 3 3 3"),
        ],
        IconName::Plus => &[Mark::Path("M12 5v14"), Mark::Path("M5 12h14")],
        IconName::Refresh => &[
            Mark::Path("M3 12a9 9 0 0 1 15-6.7L21 8"),
            Mark::Path("M21 3v5h-5"),
            Mark::Path("M21 12a9 9 0 0 1-15 6.7L3 16"),
            Mark::Path("M3 21v-5h5"),
        ],
        IconName::Repeat => &[
            Mark::Path("m17 2 4 4-4 4"),
            Mark::Path("M3 11v-1a4 4 0 0 1 4-4h14"),
            Mark::Path("m7 22-4-4 4-4"),
            Mark::Path("M21 13v1a4 4 0 0 1-4 4H3"),
        ],
        IconName::Reply => &[
            Mark::Path("m9 14-5-5 5-5"),
            Mark::Path("M4 9h10a6 6 0 0 1 6 6v5"),
        ],
        IconName::Search => &[Mark::Circle("11", "11", "7"), Mark::Path("m20 20-4-4")],
        IconName::Sparkle => &[
            Mark::Path("M10 3.5 11.6 8.4 16.5 10 11.6 11.6 10 16.5 8.4 11.6 3.5 10 8.4 8.4Z"),
            Mark::Path("M18 14.5 18.8 16.7 21 17.5 18.8 18.3 18 20.5 17.2 18.3 15 17.5 17.2 16.7Z"),
        ],
        IconName::SidebarCollapse => &[
            Mark::Rect("3", "4", "18", "16", Some("2")),
            Mark::Path("M9 4v16M15 9l-3 3 3 3"),
        ],
        IconName::SidebarExpand => &[
            Mark::Rect("3", "4", "18", "16", Some("2")),
            Mark::Path("M9 4v16M12 9l3 3-3 3"),
        ],
        IconName::Square => &[Mark::Rect("5", "5", "14", "14", Some("2"))],
        IconName::ThumbDown => &[Mark::Path(
            "M7 14V4H3v10h4Zm0 0 5 7c2 0 3-1 2-4l-1-3h5a3 3 0 0 0 3-4l-2-6H7",
        )],
        IconName::ThumbUp => &[Mark::Path(
            "M7 10v10H3V10h4Zm0 0 5-7c2 0 3 1 2 4l-1 3h5a3 3 0 0 1 3 4l-2 6H7",
        )],
        IconName::Undo => &[
            Mark::Path("M9 14 4 9l5-5"),
            Mark::Path("M4 9h11a5 5 0 0 1 0 10h-3"),
        ],
        IconName::User => &[
            Mark::Circle("12", "8", "4"),
            Mark::Path("M4 21a8 8 0 0 1 16 0"),
        ],
        IconName::Users => &[
            Mark::Path("M16 21v-2a4 4 0 0 0-4-4H6a4 4 0 0 0-4 4v2"),
            Mark::Circle("9", "7", "4"),
            Mark::Path("M22 21v-2a4 4 0 0 0-3-3.87"),
            Mark::Path("M16 3.13a4 4 0 0 1 0 7.75"),
        ],
        IconName::X => &[Mark::Path("m6 6 12 12"), Mark::Path("m18 6-12 12")],
        IconName::ArrowUp => &[Mark::Path("M12 19V5"), Mark::Path("m5 12 7-7 7 7")],
        IconName::ChevronLeft => &[Mark::Path("m15 6-6 6 6 6")],
        IconName::ChevronRight => &[Mark::Path("m9 6 6 6-6 6")],
        IconName::Book => &[Mark::Path("M4 3h13l3 3v15H4V3Zm4 5h8M8 12h8M8 16h6")],
        IconName::Clock => &[Mark::Path(
            "M21 12a9 9 0 1 1-18 0 9 9 0 0 1 18 0Zm-9-5v6l4 2",
        )],
        IconName::File => &[Mark::Path("M5 3h10l4 4v14H5V3Zm10 0v5h4M8 12h8m-8 4h6")],
        IconName::Heart => &[Mark::Path(
            "M20 5c-3-3-6-1-8 1-2-2-5-4-8-1-5 5 4 11 8 15 4-4 13-10 8-15Z",
        )],
        IconName::Moon => &[Mark::Path("M20 15A9 9 0 0 1 9 3a9 9 0 1 0 11 12Z")],
        IconName::Settings => &[Mark::Path(
            "M12 8a4 4 0 1 1 0 8 4 4 0 0 1 0-8Zm-2-6h4l1 3 3 1 3 2-1 4 1 4-3 2-3 1-1 3h-4l-1-3-3-1-3-2 1-4-1-4 3-2 3-1 1-3Z",
        )],
        IconName::Share => &[Mark::Path("M12 15V2m-5 5 5-5 5 5M5 11v10h14V11")],
        IconName::Steps => &[Mark::Path(
            "M7 3c3 0 3 6 1 8H5c-1-3-1-8 2-8Zm9 6c3 0 3 6 1 8h-3c-1-3-1-8 2-8ZM5 14h3v5H5v-5Zm9 6h3v3h-3v-3",
        )],
        IconName::Sun => &[Mark::Path(
            "M12 3v2m0 14v2M3 12h2m14 0h2M5.6 5.6 7 7m10 10 1.4 1.4M5.6 18.4 7 17m10-10 1.4-1.4M16 12a4 4 0 1 1-8 0 4 4 0 0 1 8 0Z",
        )],
        IconName::More => &[
            Mark::Circle("5", "12", "1"),
            Mark::Circle("12", "12", "1"),
            Mark::Circle("19", "12", "1"),
        ],
        IconName::Pencil => &[
            Mark::Path("M16.5 3.5a2.1 2.1 0 0 1 3 3L8 18l-4 1 1-4Z"),
            Mark::Path("m14.5 5.5 3 3"),
        ],
        IconName::Trash => &[
            Mark::Path("M4 7h16M9 7V4h6v3"),
            Mark::Path("M6 7l1 13h10l1-13"),
            Mark::Path("M10 11v5M14 11v5"),
        ],
    }
}

/// One icon, drawn in the current text colour at the surrounding text's icon size.
#[component]
pub fn Icon(name: IconName) -> Element {
    rsx! {
        svg {
            class: "icon",
            view_box: "0 0 24 24",
            fill: "none",
            stroke: "currentColor",
            stroke_width: "{STROKE}",
            stroke_linecap: "round",
            stroke_linejoin: "round",
            for mark in marks(name) {
                MarkShape { mark: *mark }
            }
        }
    }
}

#[component]
fn MarkShape(mark: Mark) -> Element {
    match mark {
        Mark::Path(d) => rsx! { path { d: "{d}" } },
        Mark::Rect(x, y, width, height, Some(rx)) => rsx! {
            rect { x: "{x}", y: "{y}", width: "{width}", height: "{height}", rx: "{rx}" }
        },
        Mark::Rect(x, y, width, height, None) => rsx! {
            rect { x: "{x}", y: "{y}", width: "{width}", height: "{height}" }
        },
        Mark::Circle(cx, cy, r) => rsx! { circle { cx: "{cx}", cy: "{cy}", r: "{r}" } },
    }
}
