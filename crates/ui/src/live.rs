//! A live list's rows, after the React app's `ExitFrame` (Dev, 2026-10-02: whatever shows or leaves
//! moves like liquid, never a pop). A row that joins a list already shown opens and settles from
//! just below; a row the list drops stays where it stood, as last listed, until its frame has
//! collapsed; a row brought back while leaving returns from where its exit reached. What a list
//! first answers with is simply there.

use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;
use std::sync::atomic::{AtomicUsize, Ordering};

use dioxus::prelude::*;

/// A row as a live list shows it.
#[derive(Clone, PartialEq)]
pub struct ListedRow<T> {
    /// The row's identity in its list.
    pub key: String,
    /// The host's row.
    pub row: T,
    /// Listed now; false while it leaves.
    pub present: bool,
    /// Listed since the list was last shown, after it had answered: it moves in rather than
    /// simply being there.
    pub arriving: bool,
}

/// The rows a live list shows, each leaving row still in its place until its frame reports it has
/// exited.
pub struct LiveRows<T: 'static> {
    /// The rows to draw, in order, each in a `RowFrame`.
    pub listed: Vec<ListedRow<T>>,
    /// Report that a leaving row's frame has collapsed, by its key.
    pub on_exited: Callback<String>,
}

struct Shown<T> {
    rows: Vec<ListedRow<T>>,
    answered: bool,
    exited: HashSet<String>,
}

/// The one entry and exit owner for a live list. `answered` says whether the list's read has
/// answered: rows move only once the list was shown after an answer, so a first answer simply
/// shows.
pub fn use_live_rows<T: Clone + 'static>(
    rows: Vec<T>,
    key_of: impl Fn(&T) -> String,
    answered: bool,
) -> LiveRows<T> {
    let shown = use_hook(|| {
        Rc::new(RefCell::new(Shown {
            rows: Vec::new(),
            answered: false,
            exited: HashSet::new(),
        }))
    });
    let mut exits = use_signal(|| 0u64);
    // A row's exit renders the list again, so the row it reports is let go.
    let _ = exits();
    let mut state = shown.borrow_mut();
    let before: HashSet<String> = state.rows.iter().map(|entry| entry.key.clone()).collect();
    let mut listed: Vec<ListedRow<T>> = rows
        .into_iter()
        .map(|row| {
            let key = key_of(&row);
            let arriving = answered && state.answered && !before.contains(&key);
            ListedRow {
                key,
                row,
                present: true,
                arriving,
            }
        })
        .collect();
    let current: HashSet<String> = listed.iter().map(|entry| entry.key.clone()).collect();
    for (index, entry) in state.rows.iter().enumerate() {
        if current.contains(&entry.key) || state.exited.contains(&entry.key) {
            continue;
        }
        // A leaving row keeps its place: after the nearest row above it that is still shown.
        let at = state.rows[..index]
            .iter()
            .rev()
            .find_map(|earlier| listed.iter().position(|kept| kept.key == earlier.key))
            .map_or(0, |position| position + 1);
        listed.insert(
            at,
            ListedRow {
                present: false,
                arriving: false,
                ..entry.clone()
            },
        );
    }
    state.rows = listed.clone();
    state.answered = answered;
    // An exit is remembered only while its row still shows as leaving.
    let leaving: HashSet<&str> = listed
        .iter()
        .filter(|entry| !entry.present)
        .map(|entry| entry.key.as_str())
        .collect();
    state.exited.retain(|key| leaving.contains(key.as_str()));
    drop(state);
    let on_exited = use_callback(move |key: String| {
        shown.borrow_mut().exited.insert(key);
        exits += 1;
    });
    LiveRows { listed, on_exited }
}

/// Reports once a frame's collapse has finished, or at once with none to run: under reduced
/// motion. A collapse cancelled by the row coming back reports nothing. `{id}` is the frame's id.
const WATCH_EXIT: &str = r#"
requestAnimationFrame(() => {
  const frame = document.getElementById("{id}");
  const collapsing = frame ? frame.getAnimations().filter((animation) => animation.transitionProperty === "grid-template-rows") : [];
  if (collapsing.length === 0) { dioxus.send(true); return; }
  Promise.allSettled(collapsing.map((animation) => animation.finished))
    .then((settled) => dioxus.send(settled.every((result) => result.status === "fulfilled")));
});
"#;

static FRAMES: AtomicUsize = AtomicUsize::new(0);

/// A live list's row: arriving, it opens and settles from just below with the entry timing;
/// leaving, it collapses and fades with the shorter timing, takes no input and is hidden from
/// assistive technology, then reports that it has exited. Spacing between rows belongs inside the
/// frames, so a leaving row takes its own along.
#[component]
pub fn RowFrame(
    present: bool,
    #[props(default)] arriving: bool,
    on_exited: EventHandler,
    children: Element,
) -> Element {
    let id = use_hook(|| format!("row-frame-{}", FRAMES.fetch_add(1, Ordering::Relaxed)));
    let watched = id.clone();
    use_effect(use_reactive!(|present| {
        if !present {
            let mut exit = document::eval(&WATCH_EXIT.replace("{id}", &watched));
            spawn(async move {
                if let Ok(true) = exit.recv::<bool>().await {
                    on_exited.call(());
                }
            });
        }
    }));
    let mut class = String::from("row-frame");
    if !present {
        class.push_str(" is-exiting");
    }
    if arriving {
        class.push_str(" is-arriving");
    }
    rsx! {
        li {
            id: "{id}",
            class: "{class}",
            "inert": (!present).then_some("true"),
            aria_hidden: (!present).then_some("true"),
            div { class: "row-frame-content", {children} }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static LIST: std::sync::Mutex<(Vec<&'static str>, bool)> =
        std::sync::Mutex::new((Vec::new(), false));
    static SEEN: std::sync::Mutex<Vec<Vec<(String, bool, bool)>>> =
        std::sync::Mutex::new(Vec::new());

    #[test]
    fn a_first_answer_shows_a_joining_row_arrives_and_a_dropped_row_stays_in_place() {
        fn app() -> Element {
            let (rows, answered) = LIST.lock().unwrap().clone();
            let live = use_live_rows(rows, |row| row.to_string(), answered);
            SEEN.lock().unwrap().push(
                live.listed
                    .iter()
                    .map(|entry| (entry.key.clone(), entry.present, entry.arriving))
                    .collect(),
            );
            rsx! {}
        }
        let row = |key: &str, present, arriving| (key.to_owned(), present, arriving);
        *LIST.lock().unwrap() = (vec!["a", "b", "c"], true);
        let mut dom = VirtualDom::new(app);
        dom.rebuild_in_place();
        *LIST.lock().unwrap() = (vec!["a", "c", "d"], true);
        dom.mark_dirty(ScopeId::APP);
        dom.render_immediate(&mut dioxus::dioxus_core::NoOpMutations);
        let seen = SEEN.lock().unwrap().clone();
        assert_eq!(
            seen[0],
            vec![
                row("a", true, false),
                row("b", true, false),
                row("c", true, false)
            ]
        );
        assert_eq!(
            seen[1],
            vec![
                row("a", true, false),
                row("b", false, false),
                row("c", true, false),
                row("d", true, true)
            ]
        );
    }

    #[test]
    fn only_a_leaving_row_is_out_of_reach() {
        fn app() -> Element {
            rsx! {
                RowFrame { present: true, arriving: false, on_exited: move |_| {}, "Kept" }
                RowFrame { present: false, arriving: false, on_exited: move |_| {}, "Leaving" }
            }
        }
        let mut dom = VirtualDom::new(app);
        dom.rebuild_in_place();
        let html = dioxus_ssr::render(&dom);
        let (kept, leaving) = html.split_once("Kept").unwrap();
        assert!(
            !kept.contains("inert") && !kept.contains("aria-hidden"),
            "{html}"
        );
        assert!(
            leaving.contains("inert=\"true\"") && leaving.contains("aria-hidden=\"true\""),
            "{html}"
        );
    }
}
