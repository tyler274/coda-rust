//! The shared icons, for the physician app and Duet alike. The paths are the React `Icon` component.

use dioxus::prelude::*;

/// The stroke width every icon draws with.
pub const STROKE: &str = "1.8";

/// The icons in the shared set: the React app's, each named as it names them, and Duet's own from the October 21
/// mockups (arrow up through sun below), which the React app doesn't draw.
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
    Book,
    Clock,
    File,
    Heart,
    Moon,
    Settings,
    Share,
    Steps,
    Sun,
}

impl IconName {
    /// Every icon in the set: the React app's, then Duet's own.
    pub const ALL: [IconName; 47] = [
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
        IconName::Book,
        IconName::Clock,
        IconName::File,
        IconName::Heart,
        IconName::Moon,
        IconName::Settings,
        IconName::Share,
        IconName::Steps,
        IconName::Sun,
    ];

    /// The React app's name for this icon, or the mockups' for Duet's own.
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
            IconName::Book => "book",
            IconName::Clock => "clock",
            IconName::File => "file",
            IconName::Heart => "heart",
            IconName::Moon => "moon",
            IconName::Settings => "settings",
            IconName::Share => "share",
            IconName::Steps => "steps",
            IconName::Sun => "sun",
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

#[cfg(test)]
pub fn icon_paths(name: IconName) -> &'static [&'static str] {
    const ALERT: &[&str] = &["M12 3 2.8 19h18.4L12 3Z", "M12 9v4", "M12 16.5h.01"];
    const ARCHIVE: &[&str] = &["M5 9v10a1 1 0 0 0 1 1h12a1 1 0 0 0 1-1V9", "M10 13h4"];
    const ARROW_LEFT: &[&str] = &["M19 12H5", "m12 19-7-7 7-7"];
    const ARROW_RIGHT: &[&str] = &["M5 12h14", "m12 5 7 7-7 7"];
    const CALENDAR: &[&str] = &["M16 3v4M8 3v4M3 10h18"];
    const CHECK: &[&str] = &["m5 12 4 4L19 6"];
    const CHEVRON: &[&str] = &["m6 9 6 6 6-6"];
    const CLIPBOARD: &[&str] = &[
        "M16 4h2a2 2 0 0 1 2 2v14a2 2 0 0 1-2 2H6a2 2 0 0 1-2-2V6a2 2 0 0 1 2-2h2",
        "M9 12h6M9 16h4",
    ];
    const COPY: &[&str] = &["M5 15V5a2 2 0 0 1 2-2h10"];
    const EXPAND: &[&str] = &["M14 4h6v6M10 20H4v-6M20 4l-7 7M4 20l7-7"];
    const EXTERNAL: &[&str] = &[
        "M15 4h5v5",
        "m10 14 10-10",
        "M20 13v5a2 2 0 0 1-2 2H6a2 2 0 0 1-2-2V6a2 2 0 0 1 2-2h5",
    ];
    const FLAG: &[&str] = &["M5 21V4", "M5 4h11l-2 4 2 4H5"];
    const HISTORY: &[&str] = &["M3 12a9 9 0 1 0 3-6.7L3 8", "M3 3v5h5", "M12 7v5l3 2"];
    const HOME: &[&str] = &["m3 11 9-8 9 8", "M5 10v10h14V10", "M9 20v-6h6v6"];
    const KEYBOARD: &[&str] = &["M6 10h.01M10 10h.01M14 10h.01M18 10h.01M7 14h10"];
    const LINK: &[&str] = &[
        "M10 14a5 5 0 0 0 7 0l3-3a5 5 0 0 0-7-7l-1 1",
        "M14 10a5 5 0 0 0-7 0l-3 3a5 5 0 0 0 7 7l1-1",
    ];
    const MENU: &[&str] = &["M4 7h16", "M4 12h16", "M4 17h16"];
    const MESSAGE: &[&str] = &["M21 15a4 4 0 0 1-4 4H8l-5 3V7a4 4 0 0 1 4-4h10a4 4 0 0 1 4 4Z"];
    const MICROPHONE: &[&str] = &["M5 10v2a7 7 0 0 0 14 0v-2M12 19v3M8 22h8"];
    const MINIMIZE: &[&str] = &["M4 4h16v16H4z", "m8 8 7 7M10 15h5v-5"];
    const PANEL_CLOSE: &[&str] = &["M15 4v16M9 9l3 3-3 3"];
    const PANEL_OPEN: &[&str] = &["M15 4v16M11 9l-3 3 3 3"];
    const PLUS: &[&str] = &["M12 5v14", "M5 12h14"];
    const REFRESH: &[&str] = &[
        "M3 12a9 9 0 0 1 15-6.7L21 8",
        "M21 3v5h-5",
        "M21 12a9 9 0 0 1-15 6.7L3 16",
        "M3 21v-5h5",
    ];
    const REPEAT: &[&str] = &[
        "m17 2 4 4-4 4",
        "M3 11v-1a4 4 0 0 1 4-4h14",
        "m7 22-4-4 4-4",
        "M21 13v1a4 4 0 0 1-4 4H3",
    ];
    const REPLY: &[&str] = &["m9 14-5-5 5-5", "M4 9h10a6 6 0 0 1 6 6v5"];
    const SEARCH: &[&str] = &["m20 20-4-4"];
    const SPARKLE: &[&str] = &[
        "M10 3.5 11.6 8.4 16.5 10 11.6 11.6 10 16.5 8.4 11.6 3.5 10 8.4 8.4Z",
        "M18 14.5 18.8 16.7 21 17.5 18.8 18.3 18 20.5 17.2 18.3 15 17.5 17.2 16.7Z",
    ];
    const COLLAPSE: &[&str] = &["M9 4v16M15 9l-3 3 3 3"];
    const EXPAND_SIDEBAR: &[&str] = &["M9 4v16M12 9l3 3-3 3"];
    const THUMB_DOWN: &[&str] =
        &["M7 14V4H3v10h4Zm0 0 5 7c2 0 3-1 2-4l-1-3h5a3 3 0 0 0 3-4l-2-6H7"];
    const THUMB_UP: &[&str] = &["M7 10v10H3V10h4Zm0 0 5-7c2 0 3 1 2 4l-1 3h5a3 3 0 0 1 3 4l-2 6H7"];
    const UNDO: &[&str] = &["M9 14 4 9l5-5", "M4 9h11a5 5 0 0 1 0 10h-3"];
    const USER: &[&str] = &["M4 21a8 8 0 0 1 16 0"];
    const USERS: &[&str] = &[
        "M16 21v-2a4 4 0 0 0-4-4H6a4 4 0 0 0-4 4v2",
        "M22 21v-2a4 4 0 0 0-3-3.87",
        "M16 3.13a4 4 0 0 1 0 7.75",
    ];
    const X: &[&str] = &["m6 6 12 12", "m18 6-12 12"];
    match name {
        IconName::Alert => ALERT,
        IconName::Archive => ARCHIVE,
        IconName::ArrowLeft => ARROW_LEFT,
        IconName::ArrowRight => ARROW_RIGHT,
        IconName::Calendar => CALENDAR,
        IconName::Check => CHECK,
        IconName::ChevronDown => CHEVRON,
        IconName::Clipboard => CLIPBOARD,
        IconName::Copy => COPY,
        IconName::Expand => EXPAND,
        IconName::ExternalLink => EXTERNAL,
        IconName::Flag => FLAG,
        IconName::History => HISTORY,
        IconName::Home => HOME,
        IconName::Keyboard => KEYBOARD,
        IconName::Link => LINK,
        IconName::Menu => MENU,
        IconName::Message => MESSAGE,
        IconName::Microphone => MICROPHONE,
        IconName::Minimize => MINIMIZE,
        IconName::PanelRightClose => PANEL_CLOSE,
        IconName::PanelRightOpen => PANEL_OPEN,
        IconName::Plus => PLUS,
        IconName::Refresh => REFRESH,
        IconName::Repeat => REPEAT,
        IconName::Reply => REPLY,
        IconName::Search => SEARCH,
        IconName::Sparkle => SPARKLE,
        IconName::SidebarCollapse => COLLAPSE,
        IconName::SidebarExpand => EXPAND_SIDEBAR,
        IconName::Square => &[],
        IconName::ThumbDown => THUMB_DOWN,
        IconName::ThumbUp => THUMB_UP,
        IconName::Undo => UNDO,
        IconName::User => USER,
        IconName::Users => USERS,
        IconName::X => X,
        IconName::ArrowUp
        | IconName::Book
        | IconName::Clock
        | IconName::File
        | IconName::Heart
        | IconName::Moon
        | IconName::Settings
        | IconName::Share
        | IconName::Steps
        | IconName::Sun => &[],
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_react_icon_keeps_its_paths() {
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../submodules/coda-agents/web/src/components/Icon.tsx");
        let source = match std::fs::read_to_string(&path) {
            Ok(source) => source,
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => return,
            Err(error) => panic!("{}: {error}", path.display()),
        };
        let names = react_names(&source);
        let react: Vec<IconName> = IconName::ALL
            .into_iter()
            .filter(|icon| names.contains(&icon.name()))
            .collect();
        assert_eq!(react.len(), names.len(), "an icon React draws is missing");
        let mut rendered_paths = 0;
        let mut rendered_rects = 0;
        let mut rendered_circles = 0;
        for icon in react.iter().copied() {
            let drawn: Vec<&str> = marks(icon)
                .iter()
                .filter_map(|mark| match mark {
                    Mark::Path(path) => Some(*path),
                    Mark::Rect(_, _, _, _, _) => {
                        rendered_rects += 1;
                        None
                    }
                    Mark::Circle(_, _, _) => {
                        rendered_circles += 1;
                        None
                    }
                })
                .collect();
            assert_eq!(drawn.as_slice(), icon_paths(icon), "{}", icon.name());
            rendered_paths += drawn.len();
            for path in icon_paths(icon) {
                assert!(
                    source.contains(&format!("d=\"{path}\"")),
                    "{} dropped {path}",
                    icon.name()
                );
            }
        }
        for path in quoted(&source, "d=\"") {
            assert!(
                react.iter().any(|icon| icon_paths(*icon).contains(&path)),
                "path from React is not in the catalog: {path}"
            );
        }
        assert_eq!(source.matches("d=\"").count(), rendered_paths);
        assert_eq!(source.matches("<rect").count(), rendered_rects);
        assert_eq!(source.matches("<circle").count(), rendered_circles);
        assert_eq!(STROKE, "1.8");
        assert!(
            icon_paths(IconName::SidebarExpand)
                .iter()
                .any(|path| path.contains("M12 9l3 3-3 3"))
        );
    }

    fn react_names(source: &str) -> Vec<&str> {
        let start = source.find("export type IconName").expect("type");
        let end = source.find("export function Icon").expect("function");
        let mut names = Vec::new();
        for line in source[start..end].lines() {
            let line = line.trim().trim_start_matches('|').trim();
            let name = line.trim_matches(|ch: char| ch == '"' || ch == ';');
            if !name.is_empty() && !name.contains(' ') && name != "export type IconName =" {
                names.push(name);
            }
        }
        names
    }

    fn quoted<'a>(source: &'a str, prefix: &str) -> Vec<&'a str> {
        let mut found = Vec::new();
        let mut rest = source;
        while let Some(start) = rest.find(prefix) {
            let after = &rest[start + prefix.len()..];
            let end = after.find('"').expect("close");
            found.push(&after[..end]);
            rest = &after[end + 1..];
        }
        found
    }
}
