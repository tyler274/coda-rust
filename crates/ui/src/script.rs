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

use serde::Deserialize;
use serde::de::DeserializeOwned;
use serde_json::{Value, json};

/// Installs, once per page, which way the person is driving it and the slide of selection fills.
pub(crate) fn motion() {
    fire("motion", json!([]));
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

/// Scrolls element `id` just into view.
pub(crate) fn scroll_into_view(id: &str) {
    fire("scrollIntoView", json!([id]));
}

/// Focuses element `id`.
pub(crate) fn focus_element(id: &str) {
    fire("focusElement", json!([id]));
}

/// Brings focus inside region `id` back to its line.
pub(crate) fn refocus_line(id: &str) {
    fire("refocusLine", json!([id]));
}

/// Puts the caret at the end of the focused field.
pub(crate) fn caret_to_end() {
    fire("caretToEnd", json!([]));
}

/// Watches frame `id`'s collapse: true once it has finished, false if the row came back.
pub(crate) fn watch_exit(id: &str) -> Asked<bool> {
    ask("watchExit", json!([id]))
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

/// Why a behaviour has nothing for its caller.
#[derive(Debug)]
pub(crate) enum Unheard {
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
pub(crate) struct Asked<T> {
    name: &'static str,
    running: transport::Running,
    answer: PhantomData<T>,
}

impl<T: DeserializeOwned> Asked<T> {
    /// The behaviour's answer, or why there is none, which also goes to the console. It waits only
    /// on the behaviour: `assets/behaviour.js` settles every one, unless its promise never does.
    pub(crate) async fn answer(self) -> Result<T, Unheard> {
        hear(self.name, &self.running).await
    }
}

/// A behaviour that keeps reporting. Dropping the last copy stops it.
#[derive(Clone)]
pub(crate) struct Listening<T> {
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
    pub(crate) async fn next(&self) -> Result<T, Unheard> {
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
    use std::rc::Rc;

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

    #[derive(Clone)]
    pub(super) struct Running(Rc<Inner>);

    struct Inner {
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
            Self(Rc::new(Inner {
                handle,
                heard: RefCell::new(heard),
                _report: report,
            }))
        }

        // The sender goes only with the receiver, so this waits rather than ending: a behaviour says
        // why it has nothing through its report.
        pub(super) async fn recv(&self) -> Result<String, Unheard> {
            std::future::poll_fn(|cx| self.0.heard.borrow_mut().poll_next_unpin(cx))
                .await
                .ok_or_else(|| Unheard::Failed("the report channel closed".to_owned()))
        }

        pub(super) fn stop(&self) {
            stop_behaviour(self.0.handle);
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

    #[derive(Clone)]
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
}
