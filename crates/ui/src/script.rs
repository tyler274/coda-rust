//! How the crate runs its browser behaviour, the functions in `assets/behaviour.js`. A browser
//! build imports that file as a module, so the crate asks nothing of `eval`, which a page such as a
//! Chrome extension's refuses. A native webview, which has no such page rule, imports the same
//! module once per page through `assets/native_load.js`, and runs each behaviour through
//! `assets/native.js` over the document's script channel.
//!
//! Each behaviour has one function here, and what it returns says what its caller can do: nothing,
//! for one that only acts; an [`Asked`], for one that answers once; a [`Listening`], for one that
//! keeps reporting until stopped.

use std::fmt;
use std::marker::PhantomData;
use std::rc::Rc;

use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

/// Marks the page with the phone `platform` it runs on, if any, and installs, once per page, which
/// way the person is driving it and the slide of selection fills.
pub(crate) fn motion(platform: Option<Platform>) {
    fire("motion", json!([platform]));
}

/// The phone platform a build runs on, which decides the form of its motion.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub(crate) enum Platform {
    /// An iPhone or iPad.
    Ios,
    /// An Android phone or tablet.
    Android,
}

impl Platform {
    /// The platform this build is compiled for; none for a browser or a desktop.
    pub(crate) fn current() -> Option<Self> {
        if cfg!(target_os = "ios") {
            Some(Self::Ios)
        } else if cfg!(target_os = "android") {
            Some(Self::Android)
        } else {
            None
        }
    }
}

/// Installs, once per page, every menu's keys, its focus on opening and its closing on a choice.
pub(crate) fn menu_keys() {
    fire("menuKeys", json!([]));
}

/// Installs, once per page, every tooltip's showing and leaving.
pub(crate) fn tooltips() {
    fire("tooltips", json!([]));
}

/// Gives context menu host `id` its ways in.
pub(crate) fn context_host(id: &str) {
    fire("contextHost", json!([id]));
}

/// Ends context menu host `id`: what is pending never lands, and what it opened closes.
pub(crate) fn dispose_context_host(id: &str) {
    fire("disposeContextHost", json!([id]));
}

/// Slides the rows of list `id`, in the order of `keys`, from where they stood.
pub(crate) fn reorder(id: &str, keys: &[String]) {
    fire("reorder", json!([id, keys]));
}

/// Opens reading surface `body` at the first passage marked in it.
pub(crate) fn scroll_to_passage(body: &str) {
    fire("scrollToPassage", json!([body]));
}

/// Brings the reader of conversation `scroller` to its end.
pub(crate) fn read_to_end(scroller: &str) {
    fire("readToEnd", json!([scroller]));
}

/// Takes the reader of conversation `scroller` to the top of its newest reply, moving there
/// `smoothly` or at once, and focuses that reply.
pub(crate) fn jump_to_latest(scroller: &str, smoothly: bool) {
    fire("jumpToLatest", json!([scroller, smoothly]));
}

/// Shows the reply that just arrived in conversation `scroller` from its top, when its reader
/// follows the end and the end would take that top out of sight.
pub(crate) fn reply_arrived(scroller: &str) {
    fire("replyArrived", json!([scroller]));
}

/// Has composer `id` take only drags that carry files.
pub(crate) fn file_drags(id: &str) {
    fire("fileDrags", json!([id]));
}

/// Has sheet `dialog` follow the finger or pointer down.
pub(crate) fn sheet_drag(dialog: &str) {
    fire("sheetDrag", json!([dialog]));
}

/// Moves focus off an arrow of switch `id` that has just been disabled.
pub(crate) fn hand_off(id: &str) {
    fire("handOff", json!([id]));
}

/// Sets checkbox `id`'s mixed state, which only a property holds.
pub(crate) fn indeterminate(id: &str, mixed: bool) {
    fire("indeterminate", json!([id, mixed]));
}

/// Shows or hides popover `id`.
pub(crate) fn toggle_popover(id: &str, show: bool) {
    fire("togglePopover", json!([id, show]));
}

/// Brings the first citation numbered `number` in answer `answer` into view and chooses it.
pub(crate) fn open_citation(answer: &str, number: usize) {
    fire("openCitation", json!([answer, number]));
}

/// Scrolls element `id` just into view.
pub(crate) fn scroll_into_view(id: &str) {
    fire("scrollIntoView", json!([id]));
}

/// Focuses element `id`, bringing it into view or not as `reveal` says.
pub(crate) fn focus_element(id: &str, reveal: Reveal) {
    fire("focusElement", json!([id, reveal]));
}

/// Whether moving focus to an element may scroll to bring it into view.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub(crate) enum Reveal {
    /// The browser scrolls it into view, as focus does by default.
    Scroll,
    /// Nothing scrolls.
    Still,
}

/// Puts the caret at the end of the focused field.
pub(crate) fn caret_to_end() {
    fire("caretToEnd", json!([]));
}

/// Makes the page's history hold exactly `depth` entries above the one the app stands on with no
/// screen open. A step back it takes lands later; a depth set while one travels is reached once it
/// lands.
pub(crate) fn set_depth(depth: usize) {
    fire("setDepth", json!([depth]));
}

/// Watches frame `id`'s `exit`: true once it has finished, false if the frame came back.
pub(crate) fn watch_exit(id: &str, exit: Exit) -> Asked<bool> {
    ask("watchExit", json!([id, exit.properties()]))
}

/// A frame's exit, which its watch waits on.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Exit {
    /// A live list's row collapsing.
    Collapse,
    /// A screen sliding away, or fading where it does not slide.
    Slide,
}

impl Exit {
    /// Every property whose transition makes up the exit.
    fn properties(self) -> &'static [&'static str] {
        match self {
            Self::Collapse => &["grid-template-rows"],
            Self::Slide => &["transform", "opacity"],
        }
    }
}

/// Opens dialog `id` modally, focusing its `[autofocus]` or else `focus`: true once it has closed,
/// false if it couldn't open.
pub(crate) fn modal(id: &str, focus: &str) -> Asked<bool> {
    ask("modal", json!([id, focus]))
}

/// Writes `text` to the clipboard, failing with the browser's refusal.
pub(crate) fn copy(text: &str) -> Asked<()> {
    ask("copy", json!([text]))
}

/// Whether the person asks for reduced motion.
pub(crate) fn reduced_motion() -> Asked<bool> {
    ask("reducedMotion", json!([]))
}

/// What the page tells the open preview `preview` of citation `citation`, until dropped.
pub(crate) fn watch_page(citation: &str, preview: &str) -> Listening<PageHeard> {
    listen("watchPage", json!([citation, preview]))
}

/// What popover `popover`, opened by control `control`, needs to be placed: as it opens and as the
/// page moves while it is open, and that it closed, until dropped. The control going out of sight
/// closes it.
pub(crate) fn watch_popover(control: &str, popover: &str) -> Listening<PopoverHeard> {
    listen("watchPopover", json!([control, popover]))
}

/// The depth of the history entry the page shows, each time the person steps through history, until
/// dropped.
pub(crate) fn watch_depth() -> Listening<usize> {
    listen("watchDepth", json!([]))
}

/// Holds the reader's place in conversation `scroller` while its size or content changes, and
/// reports whether the reader is at its end, now and on each change, until dropped.
pub(crate) fn watch_reading(scroller: &str) -> Listening<bool> {
    listen("watchReading", json!([scroller]))
}

/// How a dictation capture session was initiated.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DictationTrigger {
    /// Started via the toolbar button; ends only on explicit Finish or Cancel.
    ClickButton,
    /// Started via hold-shortcut (`Ctrl+Shift+Space`); ends when a held key releases.
    HoldShortcut,
}

/// Starts capturing microphone audio for dictation, streaming start/level/release/error events until stopped.
pub fn start_dictation(trigger: DictationTrigger) -> Listening<DictationHeard> {
    let stop_on_key_release = matches!(trigger, DictationTrigger::HoldShortcut);
    listen("watchDictation", json!([stop_on_key_release]))
}

/// Stops active dictation capture and returns encoded audio for transcription.
pub fn stop_dictation() -> Asked<DictationAudio> {
    ask("stopDictation", json!([]))
}

/// Cancels any active dictation capture and releases the microphone.
pub fn cancel_dictation() {
    fire("cancelDictation", json!([]));
}

/// The page appearance chosen for the surface (`light` or `dark`).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Appearance {
    /// Light neutrals on the bright canvas.
    #[default]
    Light,
    /// Dark neutrals on `data-theme="dark"`.
    Dark,
}

impl Appearance {
    /// Switches between light and dark appearance.
    pub fn toggle(self) -> Self {
        match self {
            Self::Light => Self::Dark,
            Self::Dark => Self::Light,
        }
    }

    /// Whether the appearance is currently dark.
    pub fn is_dark(self) -> bool {
        matches!(self, Self::Dark)
    }
}

/// Reads the active appearance (`light` or `dark`) from `storage_key`, `data-theme`, or system preference.
pub fn read_appearance(storage_key: &str) -> Asked<Appearance> {
    ask("readAppearance", json!([storage_key]))
}

/// Sets `data-theme` on the root element and persists `appearance` under `storage_key`.
pub fn set_appearance(storage_key: &str, appearance: Appearance) {
    fire("setAppearance", json!([storage_key, appearance]));
}

/// Closes every open top-layer popover (lifted context-menu rows, menus, tooltips, citation popovers).
pub fn dismiss_open_popovers() {
    fire("dismissOpenPopovers", json!([]));
}

/// Listens in the window capture phase for `⌘⇧L` / `Ctrl+Shift+L`, reporting each press until dropped.
pub fn watch_cover_shortcut() -> Listening<bool> {
    listen("watchCoverShortcut", json!([]))
}

/// Event streamed while dictation listens on the browser microphone.
#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(tag = "kind", rename_all = "lowercase")]
pub enum DictationHeard {
    /// Microphone capture started cleanly.
    Started,
    /// Normalized RMS voice loudness sample in `0.0..=1.0`.
    Level {
        /// Loudness sample.
        level: f32,
    },
    /// Physician released a held push-to-talk shortcut key.
    Released,
    /// Recording could not start or access the microphone.
    Error {
        /// Human-readable error detail.
        message: String,
    },
}

/// Encoded audio bytes captured when dictation finishes listening.
#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
pub struct DictationAudio {
    /// Base64-encoded audio stream.
    pub audio_base64: String,
    /// MediaRecorder MIME type.
    pub mime_type: String,
}

/// One thing `watchPage` reports.
#[derive(Clone, Copy, Debug, Deserialize, PartialEq)]
#[serde(tag = "kind", rename_all = "lowercase")]
pub(crate) enum PageHeard {
    /// The window, as wide and tall as this.
    Window {
        /// Its width.
        width: f64,
        /// Its height.
        height: f64,
    },
    /// Escape, and whether focus was inside the preview.
    Escape {
        /// Whether focus was inside the preview.
        inside: bool,
    },
    /// The citation is out of sight.
    Away,
}

/// One thing `watchPopover` reports.
#[derive(Clone, Copy, Debug, Deserialize, PartialEq)]
#[serde(tag = "kind", rename_all = "lowercase")]
pub(crate) enum PopoverHeard {
    /// It opened, standing so; each opening chooses its side afresh.
    Opened(Standing),
    /// The page scrolled or the window resized while it is open.
    Moved(Standing),
    /// It closed.
    Closed,
}

/// Where an open popover stands: its control's box in the window, its own width, and the window's
/// width and height.
#[derive(Clone, Copy, Debug, Deserialize, PartialEq)]
pub(crate) struct Standing {
    /// The control's box.
    pub(crate) control: Bounds,
    /// The popover's own width.
    pub(crate) width: f64,
    /// The window's width and height.
    pub(crate) window: (f64, f64),
}

/// A box in the window: its top-left corner and its size.
#[derive(Clone, Copy, Debug, Deserialize, PartialEq)]
pub(crate) struct Bounds {
    /// Its left edge.
    pub(crate) x: f64,
    /// Its top edge.
    pub(crate) y: f64,
    /// Its width.
    pub(crate) width: f64,
    /// Its height.
    pub(crate) height: f64,
}

/// Why a behaviour has nothing for its caller.
#[derive(Debug)]
pub enum Unheard {
    /// The behaviour threw or its promise rejected, or the module it lives in wouldn't load.
    Failed(String),
    /// It reported something its caller can't read: `assets/behaviour.js` and this file disagree.
    Unreadable(serde_json::Error),
    /// The native webview's script channel failed.
    #[cfg(not(target_arch = "wasm32"))]
    Channel(dioxus::document::EvalError),
}

impl fmt::Display for Unheard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Failed(why) => f.write_str(why),
            Self::Unreadable(error) => write!(f, "the behaviour's report can't be read: {error}"),
            #[cfg(not(target_arch = "wasm32"))]
            Self::Channel(error) => write!(f, "the script channel failed: {error}"),
        }
    }
}

/// A behaviour that answers once. It starts when asked; [`Asked::answer`] waits for what it says.
pub struct Asked<T> {
    name: &'static str,
    running: transport::Running,
    answer: PhantomData<T>,
}

impl<T: DeserializeOwned> Asked<T> {
    /// The behaviour's answer, or why there is none, which also goes to the console. It waits only
    /// on the behaviour: `assets/behaviour.js` settles every one, unless its promise never does.
    pub async fn answer(self) -> Result<T, Unheard> {
        hear(self.name, &self.running).await
    }
}

/// A behaviour that keeps reporting. Dropping the last copy stops it.
#[derive(Clone)]
pub struct Listening<T> {
    name: &'static str,
    running: Rc<Stopping>,
    report: PhantomData<T>,
}

struct Stopping(transport::Running);

impl Drop for Stopping {
    fn drop(&mut self) {
        self.0.stop();
    }
}

impl<T: DeserializeOwned> Listening<T> {
    /// The next thing the behaviour reports, or why there is none, which also goes to the console.
    pub async fn next(&self) -> Result<T, Unheard> {
        hear(self.name, &self.running.0).await
    }
}

/// What every report is on its way to Rust, whichever path carried it.
#[derive(Deserialize)]
#[serde(rename_all = "lowercase")]
enum Report<T> {
    Ok(T),
    Err(String),
}

fn read<T: DeserializeOwned>(report: &str) -> Result<T, Unheard> {
    match serde_json::from_str(report).map_err(Unheard::Unreadable)? {
        Report::Ok(value) => Ok(value),
        Report::Err(why) => Err(Unheard::Failed(why)),
    }
}

async fn hear<T: DeserializeOwned>(
    name: &'static str,
    running: &transport::Running,
) -> Result<T, Unheard> {
    let heard = running.recv().await.and_then(|report| read(&report));
    if let Err(error) = &heard {
        transport::complain(&format!("coda ui: {name}: {error}"));
    }
    heard
}

fn fire(name: &'static str, args: Value) {
    transport::fire(name, &args.to_string());
}

fn ask<T>(name: &'static str, args: Value) -> Asked<T> {
    Asked {
        name,
        running: transport::ask(name, &args.to_string()),
        answer: PhantomData,
    }
}

fn listen<T>(name: &'static str, args: Value) -> Listening<T> {
    Listening {
        name,
        running: Rc::new(Stopping(transport::listen(name, &args.to_string()))),
        report: PhantomData,
    }
}

/// Makes the module ready on this page, for a native webview, whose first use it must come before.
/// A failure to load is said here, once. A browser build imports the module with the page.
pub(crate) fn load() {
    transport::load();
}

#[cfg(target_arch = "wasm32")]
mod transport {
    use std::cell::RefCell;

    use futures_channel::mpsc::{UnboundedReceiver, unbounded};
    use futures_util::StreamExt;
    use wasm_bindgen::prelude::*;

    use super::Unheard;

    #[wasm_bindgen(module = "/assets/behaviour.js")]
    extern "C" {
        #[wasm_bindgen(js_name = fire)]
        fn fire_behaviour(name: &str, args: &str);
        #[wasm_bindgen(js_name = ask)]
        fn ask_behaviour(name: &str, args: &str, report: &Closure<dyn FnMut(String)>);
        #[wasm_bindgen(js_name = listen)]
        fn listen_behaviour(name: &str, args: &str, report: &Closure<dyn FnMut(String)>) -> u32;
        #[wasm_bindgen(js_name = stop)]
        fn stop_behaviour(handle: u32);
    }

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace = console, js_name = error)]
        pub(super) fn complain(message: &str);
    }

    pub(super) fn load() {}

    pub(super) fn fire(name: &str, args: &str) {
        fire_behaviour(name, args);
    }

    pub(super) fn ask(name: &str, args: &str) -> Running {
        Running::start(|report| {
            ask_behaviour(name, args, report);
            0
        })
    }

    pub(super) fn listen(name: &str, args: &str) -> Running {
        Running::start(|report| listen_behaviour(name, args, report))
    }

    pub(super) struct Running {
        handle: u32,
        heard: RefCell<UnboundedReceiver<String>>,
        // Held so the behaviour can report for as long as anyone listens.
        _report: Closure<dyn FnMut(String)>,
    }

    impl Running {
        fn start(call: impl FnOnce(&Closure<dyn FnMut(String)>) -> u32) -> Self {
            let (tell, heard) = unbounded();
            let report = Closure::new(move |value: String| {
                // The receiver goes only with this closure, so a report always has a listener.
                let _ = tell.unbounded_send(value);
            });
            let handle = call(&report);
            Self {
                handle,
                heard: RefCell::new(heard),
                _report: report,
            }
        }

        // The sender goes only with the receiver, so this waits rather than ending: a behaviour says
        // why it has nothing through its report.
        pub(super) async fn recv(&self) -> Result<String, Unheard> {
            std::future::poll_fn(|cx| self.heard.borrow_mut().poll_next_unpin(cx))
                .await
                .ok_or_else(|| Unheard::Failed("the report channel closed".to_owned()))
        }

        pub(super) fn stop(&self) {
            stop_behaviour(self.handle);
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
mod transport {
    use dioxus::document::{self, Eval};
    use dioxus::prelude::spawn;

    use super::Unheard;

    const LOAD: &str = include_str!("../assets/native_load.js");
    const NATIVE: &str = include_str!("../assets/native.js");
    const BEHAVIOUR: &str = include_str!("../assets/behaviour.js");

    pub(super) fn complain(message: &str) {
        eprintln!("{message}");
    }

    pub(super) fn load() {
        let eval = document::eval(LOAD);
        if let Err(error) = eval.send(BEHAVIOUR) {
            complain(&format!(
                "coda ui: the module's source couldn't be sent: {error}"
            ));
        }
        spawn(async move {
            if let Err(error) = eval.join::<()>().await {
                complain(&format!("coda ui: the module didn't load: {error}"));
            }
        });
    }

    pub(super) fn fire(name: &str, args: &str) {
        Running::start("fire", name, args);
    }

    pub(super) fn ask(name: &str, args: &str) -> Running {
        Running::start("ask", name, args)
    }

    pub(super) fn listen(name: &str, args: &str) -> Running {
        Running::start("listen", name, args)
    }

    pub(super) struct Running(Eval);

    impl Running {
        fn start(kind: &str, name: &str, args: &str) -> Self {
            let eval = document::eval(NATIVE);
            if let Err(error) = eval.send((kind, name, args)) {
                complain(&format!(
                    "coda ui: {name}: the script channel failed: {error}"
                ));
            }
            Self(eval)
        }

        pub(super) async fn recv(&self) -> Result<String, Unheard> {
            let mut eval = self.0;
            eval.recv().await.map_err(Unheard::Channel)
        }

        pub(super) fn stop(&self) {
            // A behaviour that has already finished has nothing to stop.
            let _ = self.0.send(());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_report_is_its_value_or_why_there_is_none() {
        assert!(matches!(read::<()>(r#"{"ok":null}"#), Ok(())));
        assert!(matches!(read::<bool>(r#"{"ok":true}"#), Ok(true)));
        assert!(matches!(
            read::<()>(r#"{"err":"NotAllowedError: denied"}"#),
            Err(Unheard::Failed(why)) if why == "NotAllowedError: denied"
        ));
    }

    #[test]
    fn a_report_the_caller_cant_read_says_so() {
        assert!(matches!(
            read::<bool>(r#"{"ok":"yes"}"#),
            Err(Unheard::Unreadable(_))
        ));
        assert!(matches!(read::<bool>("true"), Err(Unheard::Unreadable(_))));
    }

    #[test]
    fn reads_what_the_page_reports() {
        let heard = |report: &str| read::<PageHeard>(report).unwrap();
        assert_eq!(
            heard(r#"{"ok":{"kind":"window","width":1280,"height":800}}"#),
            PageHeard::Window {
                width: 1280.0,
                height: 800.0
            }
        );
        assert_eq!(
            heard(r#"{"ok":{"kind":"escape","inside":true}}"#),
            PageHeard::Escape { inside: true }
        );
        assert_eq!(heard(r#"{"ok":{"kind":"away"}}"#), PageHeard::Away);
        assert!(matches!(
            read::<PageHeard>(r#"{"ok":{"kind":"elsewhere"}}"#),
            Err(Unheard::Unreadable(_))
        ));
    }

    #[test]
    fn reads_what_a_popover_hears() {
        let heard = |report: &str| read::<PopoverHeard>(report).unwrap();
        let standing = Standing {
            control: Bounds {
                x: 300.0,
                y: 640.5,
                width: 96.0,
                height: 28.0,
            },
            width: 360.0,
            window: (1280.0, 800.0),
        };
        assert_eq!(
            heard(
                r#"{"ok":{"kind":"opened","control":{"x":300,"y":640.5,"width":96,"height":28},"width":360,"window":[1280,800]}}"#
            ),
            PopoverHeard::Opened(standing)
        );
        assert_eq!(
            heard(
                r#"{"ok":{"kind":"moved","control":{"x":300,"y":640.5,"width":96,"height":28},"width":360,"window":[1280,800]}}"#
            ),
            PopoverHeard::Moved(standing)
        );
        assert_eq!(heard(r#"{"ok":{"kind":"closed"}}"#), PopoverHeard::Closed);
    }
}
