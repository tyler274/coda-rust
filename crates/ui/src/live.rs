//! A live list's rows, which move like liquid as they show or leave, never popping. A row that joins a list already shown opens and settles from
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
    /// The id for the element whose children are the rows' frames: carrying it, the list slides
    /// each row it reorders from where it stood.
    pub list: String,
}

struct Shown<T> {
    rows: Vec<ListedRow<T>>,
    /// Whether each row's own source had answered when the list was last shown, asked of a row now.
    ready: Rc<dyn Fn(&T) -> bool>,
    exited: HashSet<String>,
}

/// The one entry and exit owner for a live list. `ready` says whether a row's own source has
/// answered: a list drawn from several reads moves each source's rows only once that source had
/// answered when the list was last shown, so a source's first answer simply shows, and a read that
/// has not answered holds back no other source's rows.
/// `ready` is kept and asked again at the next render, so it captures this render's readiness by
/// value rather than reading a signal then.
pub fn use_live_rows<T: Clone + 'static>(
    rows: Vec<T>,
    key_of: impl Fn(&T) -> String,
    ready: impl Fn(&T) -> bool + 'static,
) -> LiveRows<T> {
    let shown = use_hook(|| {
        Rc::new(RefCell::new(Shown {
            rows: Vec::new(),
            ready: Rc::new(|_: &T| false) as Rc<dyn Fn(&T) -> bool>,
            exited: HashSet::new(),
        }))
    });
    let list = use_hook(|| format!("live-list-{}", LISTS.fetch_add(1, Ordering::Relaxed)));
    let mut exits = use_signal(|| 0u64);
    // A row's exit renders the list again, so the row it reports is let go.
    let _ = exits();
    let mut state = shown.borrow_mut();
    let before: HashSet<String> = state.rows.iter().map(|entry| entry.key.clone()).collect();
    let mut listed: Vec<ListedRow<T>> = rows
        .into_iter()
        .map(|row| {
            let key = key_of(&row);
            let arriving = ready(&row) && (state.ready)(&row) && !before.contains(&key);
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
    state.ready = Rc::new(ready);
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
    let keys: Vec<String> = listed.iter().map(|entry| entry.key.clone()).collect();
    let reordered = list.clone();
    // The rows' places are taken after every render of the list, so a reorder slides from where
    // each row last stood, whatever changed its height since.
    let renders = use_hook(|| Rc::new(std::cell::Cell::new(0u64)));
    renders.set(renders.get() + 1);
    let render = renders.get();
    use_effect(use_reactive!(|keys, render| {
        let _ = render;
        let reorder = document::eval(&REORDER.replace("{id}", &reordered));
        let _ = reorder.send(keys);
    }));
    LiveRows {
        listed,
        on_exited,
        list,
    }
}

/// A list the server reorders: each row it keeps slides
/// from where it stood to where it now stands, its current item carrying the selection fill. A row that rises past others is lifted while it moves, so it passes over them rather than
/// through them. Rows that only join or leave move nothing here; their own frames open and close.
/// The list's frames are its children, in the order of the keys sent. `{id}` is the list's id.
const REORDER: &str = r#"
const keys = await dioxus.recv();
const list = document.getElementById("{id}");
if (list !== null) {
  const frames = [...list.children].filter((child) => child.classList.contains("row-frame"));
  const tops = new Map(keys.map((key, index) => [key, frames[index]?.offsetTop ?? 0]));
  const before = list.codaShownRows ?? { keys: [], tops: new Map() };
  list.codaShownRows = { keys, tops };
  const motion = window.codaMotion;
  const kept = keys.filter((key) => before.tops.has(key));
  if (motion !== undefined && kept.join("\n") !== before.keys.filter((key) => tops.has(key)).join("\n")) {
    const selected = motion.currentItem(list);
    keys.forEach((key, index) => {
      const from = before.tops.get(key);
      const to = tops.get(key);
      const frame = frames[index];
      if (from === undefined || frame === undefined || from === to) return;
      const shift = [{ translate: `0 ${from - to}px` }, {}];
      const moving = motion.animate(frame, shift, "step", "emphasized");
      if (moving !== undefined && from > to) {
        frame.classList.add("is-lifted");
        moving.finished.catch(() => undefined).then(() => frame.classList.remove("is-lifted"));
      }
      // The current item carries the fill itself while it moves, so the two never part.
      if (moving !== undefined && selected !== null && frame.contains(selected)) {
        selected.classList.add("is-carrying-selection");
        moving.finished.catch(() => undefined).then(() => selected.classList.remove("is-carrying-selection"));
      }
    });
  }
}
"#;

static LISTS: AtomicUsize = AtomicUsize::new(0);

/// Reports once a frame's collapse has finished, or at once with none to run: under reduced
/// motion. A collapse cancelled by the row coming back reports nothing; one cancelled while the
/// row still leaves is watched again. `{id}` is the frame's id.
const WATCH_EXIT: &str = r#"
const watch = () => requestAnimationFrame(() => {
  const frame = document.getElementById("{id}");
  const collapsing = frame ? frame.getAnimations().filter((animation) => animation.transitionProperty === "grid-template-rows") : [];
  if (collapsing.length === 0) { dioxus.send(true); return; }
  Promise.allSettled(collapsing.map((animation) => animation.finished)).then((settled) => {
    if (settled.every((result) => result.status === "fulfilled")) dioxus.send(true);
    // A collapse cancelled while the row still leaves (motion became immediate) is watched again.
    else if (frame.classList.contains("is-exiting")) watch();
    else dioxus.send(false);
  });
});
watch();
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
    // Each change of `present` supersedes the exit watched before it: a row brought back while
    // leaving never reports the exit it left.
    let watch = use_hook(|| Rc::new(std::cell::Cell::new(0u64)));
    use_effect(use_reactive!(|present| {
        let this = watch.get() + 1;
        watch.set(this);
        if !present {
            let mut exit = document::eval(&WATCH_EXIT.replace("{id}", &watched));
            let watch = watch.clone();
            spawn(async move {
                if let Ok(true) = exit.recv::<bool>().await
                    && watch.get() == this
                {
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
            let live = use_live_rows(rows, |row| row.to_string(), move |_| answered);
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

    static SOURCES: std::sync::Mutex<(Vec<&'static str>, bool, bool)> =
        std::sync::Mutex::new((Vec::new(), false, false));
    static FROM_SOURCES: std::sync::Mutex<Vec<Vec<(String, bool)>>> =
        std::sync::Mutex::new(Vec::new());

    #[test]
    fn each_source_moves_its_rows_only_after_its_own_first_answer() {
        // Rows from source a and source b, each named by its source's letter.
        fn app() -> Element {
            let (rows, a, b) = SOURCES.lock().unwrap().clone();
            let live = use_live_rows(
                rows,
                |row| row.to_string(),
                move |row: &&str| {
                    if row.starts_with('a') { a } else { b }
                },
            );
            FROM_SOURCES.lock().unwrap().push(
                live.listed
                    .iter()
                    .map(|entry| (entry.key.clone(), entry.arriving))
                    .collect(),
            );
            rsx! {}
        }
        *SOURCES.lock().unwrap() = (vec!["a1"], true, false);
        let mut dom = VirtualDom::new(app);
        dom.rebuild_in_place();
        // a lists a second row while b answers for the first time.
        *SOURCES.lock().unwrap() = (vec!["a1", "a2", "b1"], true, true);
        dom.mark_dirty(ScopeId::APP);
        dom.render_immediate(&mut dioxus::dioxus_core::NoOpMutations);
        let seen = FROM_SOURCES.lock().unwrap().clone();
        let row = |key: &str, arriving| (key.to_owned(), arriving);
        assert_eq!(
            seen[1],
            vec![row("a1", false), row("a2", true), row("b1", false)]
        );
    }
}
