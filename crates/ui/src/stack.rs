//! A phone app's tabs and the screens open over each, as one stack. `Nav` says which tab shows and
//! which screens are open over every tab; `use_screen_stack` draws it as layers that each keep their
//! own scroll place, mirrors its depth into the web view's history so the platform's own back
//! arrives as a step there, and plays each screen's entry and exit; each layer is drawn in a
//! `ScreenFrame`. Opening a screen and going back move on a press, a tab change arrives at once,
//! and going back finds the earlier screen as it was.

use std::cell::Cell;
use std::rc::Rc;

use dioxus::prelude::*;

use crate::live::{self, use_live_rows};
use crate::script;

/// A screen's identity among everything its nav has opened. Two visits to the same record are two
/// screens, each with its own serial; the nav never gives one out twice.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Serial(u64);

/// A screen open over a tab, with the serial its nav gave it as it opened.
#[derive(Clone, Debug, PartialEq)]
pub struct Opened<S> {
    serial: Serial,
    screen: S,
}

impl<S> Opened<S> {
    /// The screen's identity for drawing.
    pub fn serial(&self) -> Serial {
        self.serial
    }

    /// The host's screen.
    pub fn screen(&self) -> &S {
        &self.screen
    }
}

/// One tab and the screens open over its first screen, bottom to top. The first screen is the
/// tab's own, so it is never among them and nothing can close it.
#[derive(Clone, Debug, PartialEq)]
struct TabStack<T, S> {
    tab: T,
    open: Vec<Opened<S>>,
}

/// Which of a phone app's tabs shows, and the screens open over each tab: `T` is the host's tab,
/// `S` its screen. It is built from its tabs, so it always holds one and always shows one of them;
/// each tab keeps its screens while another shows.
#[derive(Clone, Debug, PartialEq)]
pub struct Nav<T, S> {
    /// Never empty: built with a first tab, and no tab is ever removed.
    tabs: Vec<TabStack<T, S>>,
    /// The tab that shows, always an index into `tabs`.
    shown: usize,
    /// The serial the next opened screen takes.
    next: u64,
}

impl<T: Copy + PartialEq, S> Nav<T, S> {
    /// A nav over `first` and then `others`, showing `first` with no screen open over any tab. A
    /// tab named twice is a host's mistake: a debug build says so, and a release build holds it once.
    pub fn new(first: T, others: impl IntoIterator<Item = T>) -> Self {
        let mut tabs = vec![TabStack {
            tab: first,
            open: Vec::new(),
        }];
        for tab in others {
            let unheld = tabs.iter().all(|held| held.tab != tab);
            debug_assert!(unheld, "a nav's tabs are each named once");
            if unheld {
                tabs.push(TabStack {
                    tab,
                    open: Vec::new(),
                });
            }
        }
        Self {
            tabs,
            shown: 0,
            next: 0,
        }
    }

    /// Opens `screen` over the tab that shows.
    pub fn open(&mut self, screen: S) {
        let serial = Serial(self.next);
        self.next += 1;
        self.tabs[self.shown].open.push(Opened { serial, screen });
    }

    /// Closes the top screen of the tab that shows and returns it; on the tab's first screen, does
    /// nothing and returns `None`.
    pub fn back(&mut self) -> Option<S> {
        self.tabs[self.shown].open.pop().map(|opened| opened.screen)
    }

    /// Shows `tab`, which finds its screens as it left them; selecting the tab that already shows
    /// returns it to its first screen. A tab the nav does not hold is a host's mistake: a debug
    /// build says so, and a release build changes nothing.
    pub fn select(&mut self, tab: T) {
        let Some(index) = self.tabs.iter().position(|held| held.tab == tab) else {
            debug_assert!(false, "selected a tab the nav does not hold");
            return;
        };
        if index == self.shown {
            self.tabs[index].open.clear();
        }
        self.shown = index;
    }

    /// The tab that shows.
    pub fn tab(&self) -> T {
        self.tabs[self.shown].tab
    }

    /// The screens open over the tab that shows, bottom to top.
    pub fn screens(&self) -> &[Opened<S>] {
        &self.tabs[self.shown].open
    }

    /// Every tab, in order, with the screens open over it, bottom to top.
    pub fn tabs(&self) -> impl Iterator<Item = (T, &[Opened<S>])> {
        self.tabs
            .iter()
            .map(|held| (held.tab, held.open.as_slice()))
    }

    /// How many screens are open over the tab that shows.
    pub fn depth(&self) -> usize {
        self.screens().len()
    }
}

/// What a layer of the stack is.
#[derive(Clone, Debug, PartialEq)]
pub enum Screen<S> {
    /// Its tab's own first screen.
    First,
    /// A screen opened over its tab.
    Open(S),
}

/// Where a layer stands.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Standing {
    /// It shows and takes input.
    Top,
    /// Covered by another screen of the tab that shows.
    Under,
    /// It belongs to a tab that does not show: unseen, its layout and scroll place kept.
    Away,
    /// Closed, and still drawn until its exit has played.
    Leaving,
}

/// One layer of the stack, to draw in a `ScreenFrame`.
#[derive(Clone, PartialEq)]
pub struct Layer<T, S> {
    /// Its identity: its tab's first screen, or the serial of the screen opened there.
    pub key: String,
    /// The tab it belongs to.
    pub tab: T,
    /// What it is.
    pub screen: Screen<S>,
    /// Where it stands.
    pub standing: Standing,
    /// Opened since the stack was last drawn: it moves in rather than simply being there.
    pub arriving: bool,
}

/// The layers a screen stack draws, each leaving layer still in its place until its frame reports
/// that its exit has played.
pub struct ScreenStack<T: 'static, S: 'static> {
    /// Every tab's first screen and the screens open over it, tab by tab and bottom to top, each in
    /// a `ScreenFrame`: a later layer stands over an earlier one.
    pub layers: Vec<Layer<T, S>>,
    /// Report that a leaving layer's exit has played, by its key.
    pub on_exited: Callback<String>,
}

#[derive(Clone, PartialEq)]
struct Row<T, S> {
    key: String,
    tab: T,
    screen: Screen<S>,
}

/// The one owner of a phone app's screens on the page: it draws `nav` as layers and plays their
/// entries and exits, and mirrors `nav`'s depth into the web view's history, so the platform's own
/// back, such as Android's back key and gesture or a browser's Back, goes back one screen. At the
/// first screen that back finds no step and leaves the app. The tab bar stands outside the layers.
pub fn use_screen_stack<T, S>(nav: Signal<Nav<T, S>>) -> ScreenStack<T, S>
where
    T: Copy + PartialEq + 'static,
    S: Clone + PartialEq + 'static,
{
    let (shown, rows) = {
        let nav = nav.read();
        let rows: Vec<Row<T, S>> = nav
            .tabs()
            .enumerate()
            .flat_map(|(index, (tab, open))| {
                std::iter::once(Row {
                    key: format!("tab-{index}"),
                    tab,
                    screen: Screen::First,
                })
                .chain(open.iter().map(move |opened| Row {
                    key: format!("screen-{}", opened.serial.0),
                    tab,
                    screen: Screen::Open(opened.screen.clone()),
                }))
            })
            .collect();
        (nav.tab(), rows)
    };
    let live = use_live_rows(rows, |row| row.key.clone(), |_| true);
    let on_exited = live.on_exited;
    let top = live
        .listed
        .iter()
        .rposition(|entry| entry.present && entry.row.tab == shown);
    let mut layers = Vec::with_capacity(live.listed.len());
    // A screen closed on a tab that no longer shows has no exit to play where anyone sees it.
    let mut let_go = Vec::new();
    for (index, entry) in live.listed.into_iter().enumerate() {
        let standing = match (entry.present, entry.row.tab == shown) {
            (false, true) => Standing::Leaving,
            (false, false) => {
                let_go.push(entry.key);
                continue;
            }
            (true, false) => Standing::Away,
            (true, true) if Some(index) == top => Standing::Top,
            (true, true) => Standing::Under,
        };
        layers.push(Layer {
            key: entry.key,
            tab: entry.row.tab,
            screen: entry.row.screen,
            standing,
            arriving: entry.arriving,
        });
    }
    use_effect(use_reactive!(|let_go| {
        for key in let_go {
            on_exited.call(key);
        }
    }));
    mirror_history(nav);
    ScreenStack { layers, on_exited }
}

/// Keeps the web view's history exactly `nav`'s depth above the entry the app stands on, and
/// answers each step the person takes there by the one rule, `heard`.
fn mirror_history<T, S>(mut nav: Signal<Nav<T, S>>)
where
    T: Copy + PartialEq + 'static,
    S: 'static,
{
    let depth = use_memo(move || nav.read().depth());
    use_effect(move || script::set_depth(depth()));
    use_hook(move || {
        let steps = script::watch_depth();
        spawn(async move {
            // A watch that fails has said why in the console; history then stays as the nav last set it.
            while let Ok(reported) = steps.next().await {
                let wanted = nav.peek().depth();
                match heard(reported, wanted) {
                    Heard::Back(levels) => {
                        let mut nav = nav.write();
                        for _ in 0..levels {
                            nav.back();
                        }
                    }
                    Heard::InStep => {}
                    Heard::Forward => script::set_depth(wanted),
                }
            }
        });
    });
}

/// What the person's step in the web view's history means against the depth the nav wants.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Heard {
    /// The person went back this many levels: close as many screens.
    Back(usize),
    /// History already stands at the depth the nav wants, because the nav moved first: an in-app
    /// Back, say, and then the person's back landing before the depth was set. Nothing to do.
    InStep,
    /// The person went forward: set the depth again, stepping back over it.
    Forward,
}

/// The one rule that compares the depth history reports with the depth the nav wants.
fn heard(reported: usize, wanted: usize) -> Heard {
    match reported.cmp(&wanted) {
        std::cmp::Ordering::Less => Heard::Back(wanted - reported),
        std::cmp::Ordering::Equal => Heard::InStep,
        std::cmp::Ordering::Greater => Heard::Forward,
    }
}

/// One layer of a screen stack: the host's `bar`, such as its `TopBar`, stays at the layer's top
/// and moves with it, and beneath it the host's content scrolls in the layer's own scroll area,
/// kept while the layer is covered or its tab is away. Frames fill the box the host gives them,
/// which clips them, a later frame standing over an earlier one. Only the top layer takes input;
/// the rest are inert and hidden from assistive technology. An arriving screen slides in, and a
/// leaving one slides away, then reports that it has exited. On an iPhone, and in a browser, the
/// new screen comes in from the trailing edge over the old one, which moves back by 0.3 of the
/// width and darkens; on Android both move by 96 px as the new one fades in and the old one fades
/// out at the start. Going back is the exact mirror. A tab change arrives at once. Under reduced
/// motion nothing slides: on an iPhone the screens arrive at once and only the darkening still
/// fades, and on Android the two screens cross-fade in place, a closed one going once its fade
/// ends. A screen just opened, or one whose screen above has closed, takes focus without scrolling
/// anything; a tab change leaves focus on the tab chosen, and on the stack's first draw nothing
/// takes it.
#[component]
pub fn ScreenFrame(
    standing: Standing,
    #[props(default)] arriving: bool,
    on_exited: EventHandler,
    #[props(default)] bar: Option<Element>,
    children: Element,
) -> Element {
    let id = live::use_exit_watch(
        "screen-frame",
        standing == Standing::Leaving,
        script::Exit::Slide,
        on_exited,
    );
    // Where the layer stood when last drawn; none before its first draw.
    let stood = use_hook(|| Rc::new(Cell::new(None::<Standing>)));
    let focused = id.clone();
    use_effect(use_reactive!(|standing, arriving| {
        let before = stood.replace(Some(standing));
        let takes_focus = match (before, standing) {
            (None, Standing::Top) => arriving,
            (Some(Standing::Under), Standing::Top) => true,
            _ => false,
        };
        if takes_focus {
            script::focus_element(&focused, script::Reveal::Still);
        }
    }));
    let mut class = String::from(match standing {
        Standing::Top => "screen-frame is-top",
        Standing::Under => "screen-frame is-under",
        Standing::Away => "screen-frame is-away",
        Standing::Leaving => "screen-frame is-exiting",
    });
    if arriving {
        class.push_str(" is-arriving");
    }
    let out_of_reach = standing != Standing::Top;
    rsx! {
        div {
            id: "{id}",
            class: "{class}",
            tabindex: "-1",
            "inert": out_of_reach.then_some("true"),
            aria_hidden: out_of_reach.then_some("true"),
            {bar}
            div { class: "screen-frame-body", {children} }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone, Copy, Debug, PartialEq)]
    enum Tab {
        Records,
        You,
    }

    fn nav() -> Nav<Tab, &'static str> {
        Nav::new(Tab::Records, [Tab::You])
    }

    fn shown(nav: &Nav<Tab, &'static str>) -> Vec<&'static str> {
        nav.screens()
            .iter()
            .map(|opened| *opened.screen())
            .collect()
    }

    #[test]
    fn going_back_closes_the_screen_just_opened() {
        let mut nav = nav();
        nav.open("visit note");
        nav.open("original record");
        assert_eq!(nav.back(), Some("original record"));
        assert_eq!(shown(&nav), ["visit note"]);
    }

    #[test]
    fn back_on_a_first_screen_does_nothing() {
        let mut nav = nav();
        let before = nav.clone();
        assert_eq!(nav.back(), None);
        assert_eq!(nav, before);
    }

    #[test]
    fn a_tab_keeps_its_screens_while_another_shows() {
        let mut nav = nav();
        nav.open("visit note");
        nav.select(Tab::You);
        assert_eq!((nav.tab(), shown(&nav)), (Tab::You, vec![]));
        nav.open("reminders");
        nav.select(Tab::Records);
        assert_eq!((nav.tab(), shown(&nav)), (Tab::Records, vec!["visit note"]));
        let tabs: Vec<(Tab, usize)> = nav.tabs().map(|(tab, open)| (tab, open.len())).collect();
        assert_eq!(tabs, [(Tab::Records, 1), (Tab::You, 1)]);
    }

    #[test]
    fn selecting_the_tab_that_shows_returns_it_to_its_first_screen() {
        let mut nav = nav();
        nav.open("visit note");
        nav.open("original record");
        nav.select(Tab::Records);
        assert_eq!((nav.tab(), nav.depth()), (Tab::Records, 0));
    }

    #[test]
    fn the_depth_is_the_shown_tabs() {
        let mut nav = nav();
        nav.open("visit note");
        nav.open("original record");
        assert_eq!(nav.depth(), 2);
        nav.select(Tab::You);
        assert_eq!(nav.depth(), 0);
        nav.open("reminders");
        assert_eq!(nav.depth(), 1);
        nav.select(Tab::Records);
        assert_eq!(nav.depth(), 2);
    }

    #[test]
    fn serials_never_repeat() {
        let mut nav = nav();
        let mut seen = Vec::new();
        for _ in 0..3 {
            nav.open("visit note");
            seen.push(nav.screens()[0].serial());
            nav.back();
        }
        nav.open("visit note");
        nav.open("visit note");
        seen.extend(nav.screens().iter().map(Opened::serial));
        nav.select(Tab::You);
        nav.open("visit note");
        seen.push(nav.screens()[0].serial());
        let distinct: std::collections::BTreeSet<Serial> = seen.iter().copied().collect();
        assert_eq!(distinct.len(), seen.len(), "{seen:?}");
    }

    #[test]
    fn history_lower_than_the_nav_closes_one_screen_per_level() {
        assert_eq!(heard(1, 2), Heard::Back(1));
        assert_eq!(heard(0, 3), Heard::Back(3));
    }

    #[test]
    fn history_equal_to_the_nav_is_already_in_step() {
        assert_eq!(heard(0, 0), Heard::InStep);
        assert_eq!(heard(2, 2), Heard::InStep);
    }

    #[test]
    fn history_higher_than_the_nav_is_stepped_back_over() {
        assert_eq!(heard(1, 0), Heard::Forward);
        assert_eq!(heard(4, 1), Heard::Forward);
    }

    #[test]
    fn only_the_top_layer_is_in_reach() {
        fn app() -> Element {
            rsx! {
                for (standing, words) in [
                    (Standing::Under, "Under"),
                    (Standing::Away, "Away"),
                    (Standing::Leaving, "Leaving"),
                    (Standing::Top, "Top"),
                ] {
                    ScreenFrame { key: "{words}", standing, on_exited: move |_| {}, "{words}" }
                }
            }
        }
        let mut dom = VirtualDom::new(app);
        dom.rebuild_in_place();
        let html = dioxus_ssr::render(&dom);
        let frames: Vec<&str> = html.split("<div id=").skip(1).collect();
        assert_eq!(frames.len(), 4, "{html}");
        for frame in &frames[..3] {
            assert!(
                frame.contains("inert=\"true\"") && frame.contains("aria-hidden=\"true\""),
                "{frame}"
            );
        }
        assert!(
            frames[3].contains("Top")
                && !frames[3].contains("inert")
                && !frames[3].contains("aria-hidden"),
            "{}",
            frames[3]
        );
    }
}
