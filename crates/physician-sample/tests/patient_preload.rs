//! Opening a patient after Coda read it ahead, in a real browser: what each preload trigger buys and
//! what it costs, and that reading ahead changes nothing the physician can see.
//!
//! `js/patient_reads.js` answers the page's two patient reads a fixed time late, so the gain does
//! not depend on a fast local server, and logs every request. The same pointer paths and keyboard
//! walks replay for each trigger the address can choose (`?preload=click`, `hover`, `predict`).
//! Each open is timed from the click or Enter to "Records as of" on screen, and the log says how
//! many patient reads went out, how many the open reused, and how many no open used.
//!
//! The comparison writes `target/verification/patient-preload/results.md` and `results.json`, tied
//! to the revision. `PRELOAD_REPEATS` sets how often each case runs (3 by default). The paths are
//! scripted: they show each trigger's mechanism and ceiling, not how physicians move.

use std::collections::BTreeMap;
use std::path::PathBuf;
use std::time::Duration;

use serde_json::{Value, json};

mod common;
#[allow(dead_code)]
#[path = "../../mobile/tests/common/devtools.rs"]
mod devtools;
#[allow(dead_code)]
mod required_auth;

use devtools::{Browser, DESKTOP, Key};
use required_auth::{SIGNED_IN, STAGING_EMAIL, spawn_required_server};

const JAKE: &str = "pw-82c44e9b-58c0-4af1-bd21-771bf77e101d";
const DAN: &str = "pw-f5404d89-acde-4bb5-8c42-2fa191766811";
const TANVIR: &str = "pw-67305f4a-7d91-462c-9cc5-1bb30ade34ee";
const TERESA: &str = "pw-d11a35f7-0d8d-466f-a48e-5812df0a4971";

/// The staging account `spawn_required_server` signs in.
const PRACTICE: &str = "roger-synthetic-practice";
const PHYSICIAN: &str = "staging-physician";

const VISIT: &str = ".schedule-visit";
const SEARCH_FIELD: &str = ".search-field input";
const RESULT: &str = ".dialog-results button";

/// How long the pointer rests on a row before the click, and how often it moves on its way.
const REST_MS: u64 = 150;
const FRAME_MS: u64 = 16;
/// The time between two presses of `j`.
const KEY_GAP_MS: u64 = 200;

/// The server's added delay on each patient read, in milliseconds.
const LATENCIES: [u64; 4] = [0, 150, 400, 800];

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Trigger {
    ClickOnly,
    HoverFocus,
    PointerPredicted,
}

impl Trigger {
    const ALL: [Self; 3] = [Self::ClickOnly, Self::HoverFocus, Self::PointerPredicted];

    fn address(self) -> &'static str {
        match self {
            Self::ClickOnly => "click",
            Self::HoverFocus => "hover",
            Self::PointerPredicted => "predict",
        }
    }

    fn reads_ahead(self) -> bool {
        match self {
            Self::ClickOnly => false,
            Self::HoverFocus | Self::PointerPredicted => true,
        }
    }
}

/// One scripted way of reaching a patient, or of coming near one and not opening it.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Path {
    /// From the page straight to a visit in the schedule, then a click.
    Direct,
    /// Down the schedule over three other visits, then a click.
    PassOver,
    /// Find patient: type a name, move to the one result, click.
    Search,
    /// `j` through the schedule to a visit, then Enter.
    Keyboard,
    /// To a visit, a rest, and away again with no open.
    ApproachLeave,
}

impl Path {
    const ALL: [Self; 5] = [
        Self::Direct,
        Self::PassOver,
        Self::Search,
        Self::Keyboard,
        Self::ApproachLeave,
    ];

    fn name(self) -> &'static str {
        match self {
            Self::Direct => "direct to a schedule row",
            Self::PassOver => "past three schedule rows",
            Self::Search => "Find patient result",
            Self::Keyboard => "keyboard j…j, Enter",
            Self::ApproachLeave => "approach, rest, leave",
        }
    }

    /// The patient this path opens, by workspace and name.
    fn opens(self) -> Option<(&'static str, &'static str)> {
        match self {
            Self::Direct | Self::PassOver | Self::Search | Self::Keyboard => {
                Some((TERESA, "Teresa Hayes"))
            }
            Self::ApproachLeave => None,
        }
    }
}

/// What one run measured.
#[derive(Clone, Debug)]
struct Run {
    /// Click or Enter to "Records as of" on screen; none when nothing opened.
    usable_ms: Option<f64>,
    issued: usize,
    reused: usize,
    unused: usize,
    /// How long before the click the opened patient's first read went out.
    head_start_ms: Option<f64>,
}

struct Page {
    browser: Browser,
    base: String,
}

impl Page {
    fn signed_in(base: &str) -> Self {
        let mut browser = Browser::launch();
        browser.resize(DESKTOP);
        browser.page().call_session("Page.enable", json!({}));
        browser.page().call_session(
            "Page.addScriptToEvaluateOnNewDocument",
            json!({ "source": include_str!("js/patient_reads.js") }),
        );
        browser.text_until(&format!("{base}/"), "Sign in to Coda");
        browser.type_into(r#"input[type="email"]"#, STAGING_EMAIL);
        std::thread::sleep(Duration::from_millis(50));
        browser.click("button.control-primary");
        browser.wait_for(SIGNED_IN);
        Self {
            browser,
            base: base.to_owned(),
        }
    }

    fn run(&mut self, name: &str, args: &[Value]) -> Value {
        let mut arguments = vec![json!(name)];
        arguments.extend_from_slice(args);
        self.browser
            .page()
            .call_function(include_str!("js/patient_preload.js"), &arguments)
    }

    /// Loads Today with `trigger` and these server answers, until the schedule is laid out.
    ///
    /// Each load signs in on a fresh browser. Every page Coda shows holds one event stream open,
    /// and Chromium keeps six connections per host, so reloading one browser stalls by the
    /// seventh page; a fresh browser also starts every measurement with nothing cached.
    fn load(&mut self, trigger: Trigger, delay: u64, answers: Value) {
        *self = Self::signed_in(&self.base);
        self.run(
            "setUp",
            &[json!({ "delay": delay, "answers": with_calendar(answers) })],
        );
        let url = format!("{}/?preload={}", self.base, trigger.address());
        self.browser.text_until(&url, "Teresa Hayes");
        self.middle(VISIT, "Teresa Hayes");
        self.run("takeRequests", &[]);
    }

    /// The middle of the first `selector` named `words`, once it is laid out.
    fn middle(&mut self, selector: &str, words: &str) -> (f64, f64) {
        for _ in 0..200 {
            if let Some([x, y]) = self
                .run("middle", &[json!(selector), json!(words)])
                .as_array()
                .map(|point| [point[0].as_f64(), point[1].as_f64()])
                && let (Some(x), Some(y)) = (x, y)
            {
                return (x, y);
            }
            std::thread::sleep(Duration::from_millis(50));
        }
        panic!("{selector} named {words} never appeared");
    }

    fn mouse(&mut self, kind: &str, (x, y): (f64, f64)) {
        let button = if kind == "mouseMoved" { "none" } else { "left" };
        self.browser.page().call_session(
            "Input.dispatchMouseEvent",
            json!({ "type": kind, "x": x, "y": y, "button": button, "clickCount": 1 }),
        );
    }

    /// Moves the pointer from `from` to `to` in `steps` frames, as a hand does: in real time.
    fn glide(&mut self, from: (f64, f64), to: (f64, f64), steps: u32) {
        for step in 1..=steps {
            let t = f64::from(step) / f64::from(steps);
            let at = (from.0 + (to.0 - from.0) * t, from.1 + (to.1 - from.1) * t);
            self.mouse("mouseMoved", at);
            std::thread::sleep(Duration::from_millis(FRAME_MS));
        }
    }

    fn click_at(&mut self, at: (f64, f64)) {
        self.mouse("mousePressed", at);
        self.mouse("mouseReleased", at);
    }

    fn requests(&mut self) -> Vec<Value> {
        self.run("takeRequests", &[])
            .as_array()
            .cloned()
            .unwrap_or_default()
    }

    /// Waits for the open to be usable and returns its times, in the page's clock.
    fn open_times(&mut self) -> (f64, f64) {
        for _ in 0..600 {
            let times = self.run("openTimes", &[]);
            if let (Some(opened), Some(usable)) =
                (times["opened"].as_f64(), times["usable"].as_f64())
            {
                return (opened, usable);
            }
            std::thread::sleep(Duration::from_millis(20));
        }
        panic!("the patient page never became usable");
    }
}

/// The practice calendar with every visit still ahead: the fixture's clock moved to before the first.
fn with_calendar(answers: Value) -> Value {
    let mut day: Value =
        serde_json::from_str(include_str!("../fixtures/practice-day.json")).unwrap();
    day["current_time"] = json!("2026-08-27T12:00:00.000Z");
    let mut all = vec![json!({ "path": "/practice-day", "status": 200, "body": day })];
    all.extend(answers.as_array().cloned().unwrap_or_default());
    Value::from(all)
}

fn walk(page: &mut Page, path: Path, trigger: Trigger, delay: u64) -> Run {
    page.load(trigger, delay, json!([]));
    let away = (640.0, 420.0);
    page.mouse("mouseMoved", away);
    std::thread::sleep(Duration::from_millis(100));
    page.requests();
    if let Some((_, name)) = path.opens() {
        page.run("timeOpen", &[json!(name)]);
    }
    match path {
        Path::Direct => {
            let to = page.middle(VISIT, "Teresa Hayes");
            page.glide(away, to, 20);
            std::thread::sleep(Duration::from_millis(REST_MS));
            page.click_at(to);
        }
        Path::PassOver => {
            let from = page.middle(VISIT, "Jake Smith");
            let to = page.middle(VISIT, "Teresa Hayes");
            page.glide(away, from, 10);
            std::thread::sleep(Duration::from_millis(REST_MS));
            page.glide(from, to, 20);
            std::thread::sleep(Duration::from_millis(REST_MS));
            page.click_at(to);
        }
        Path::Search => {
            let find = page.middle("button", "Find patient");
            page.click_at(find);
            page.middle(SEARCH_FIELD, "");
            page.browser.type_into(SEARCH_FIELD, "Teresa");
            let to = page.middle(RESULT, "Teresa Hayes");
            let from = page.middle(SEARCH_FIELD, "");
            page.mouse("mouseMoved", from);
            page.requests();
            page.run("timeOpen", &[json!("Teresa Hayes")]);
            page.glide(from, to, 12);
            std::thread::sleep(Duration::from_millis(REST_MS));
            page.click_at(to);
        }
        Path::Keyboard => {
            page.browser
                .page()
                .evaluate("document.querySelector('.glass-app').focus(); 'focused'");
            // Jake, Dan, Tanvir, the unnamed visit, then Teresa.
            for _ in 0..5 {
                page.browser.press_key(Key::typing("j", "KeyJ", 74));
                std::thread::sleep(Duration::from_millis(KEY_GAP_MS));
            }
            page.browser.press_key(Key::ENTER);
        }
        Path::ApproachLeave => {
            let to = page.middle(VISIT, "Tanvir Chowdhury");
            page.glide(away, to, 20);
            std::thread::sleep(Duration::from_millis(400));
            page.glide(to, away, 20);
        }
    }
    let times = path.opens().map(|_| page.open_times());
    // Let every read already asked for go out and finish before the log is read.
    std::thread::sleep(Duration::from_millis(delay + 500));
    let reads: Vec<Value> = page
        .requests()
        .into_iter()
        .filter(|request| request["read"] == json!(true))
        .collect();
    for read in &reads {
        assert_eq!(
            read["method"], "GET",
            "a patient read was not a GET: {read}"
        );
    }
    let target = path.opens().map(|(id, _)| id);
    let for_target: Vec<&Value> = reads
        .iter()
        .filter(|read| target.is_some_and(|id| names(read, id)))
        .collect();
    if target.is_some() {
        assert_eq!(
            for_target.len(),
            2,
            "{path:?} {trigger:?} at {delay} ms read the opened patient other than once each: {for_target:?}"
        );
    }
    let opened = times.map(|(opened, _)| opened);
    let reused = for_target
        .iter()
        .filter(|read| opened.is_some_and(|opened| read["at"].as_f64().unwrap() < opened))
        .count();
    let first_target = for_target
        .iter()
        .filter_map(|read| read["at"].as_f64())
        .reduce(f64::min);
    Run {
        usable_ms: times.map(|(opened, usable)| usable - opened),
        issued: reads.len(),
        reused,
        unused: reads.len() - for_target.len(),
        head_start_ms: match (opened, first_target) {
            (Some(opened), Some(first)) if first < opened => Some(opened - first),
            _ => None,
        },
    }
}

/// Whether `read` is one of patient `id`'s two reads.
fn names(read: &Value, id: &str) -> bool {
    let path = read["path"].as_str().unwrap_or_default();
    path == format!("/patient-workspaces/{id}")
        || path == format!("/today?patient_workspace_id={id}")
}

fn median(mut values: Vec<f64>) -> Option<f64> {
    if values.is_empty() {
        return None;
    }
    values.sort_by(f64::total_cmp);
    Some(values[values.len() / 2])
}

fn mean(values: impl Iterator<Item = usize>, count: usize) -> f64 {
    values.sum::<usize>() as f64 / count as f64
}

fn revision() -> String {
    let workspace = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let git = |args: &[&str]| {
        std::process::Command::new("git")
            .current_dir(&workspace)
            .args(args)
            .output()
            .map(|out| String::from_utf8_lossy(&out.stdout).trim().to_owned())
            .unwrap_or_default()
    };
    let head = git(&["rev-parse", "HEAD"]);
    if git(&["status", "--porcelain", "--untracked-files=no"]).is_empty() {
        head
    } else {
        format!("{head} with local changes")
    }
}

fn write_results(results: &BTreeMap<(Path, Trigger, u64), Vec<Run>>, repeats: usize) -> PathBuf {
    let folder =
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../target/verification/patient-preload");
    std::fs::create_dir_all(&folder).expect("the results folder");
    let revision = revision();
    let ms = |value: Option<f64>| value.map_or("–".to_owned(), |ms| format!("{ms:.0}"));
    let mut table = format!(
        "# Patient preload comparison\n\nRevision: `{revision}`\n\n\
         Each case ran {repeats} times; times are medians, read counts are means per run. \
         The pointer rests {REST_MS} ms on a row before clicking and moves at one step per \
         {FRAME_MS} ms; `j` is pressed every {KEY_GAP_MS} ms. Usable is click or Enter to \
         \"Records as of\" on screen. Head start is how long before the click the opened \
         patient's first read went out.\n\n\
         | Path | Trigger | Added read latency (ms) | Click to usable (ms) | Head start (ms) | Reads issued | Reused by the open | Unused |\n\
         |---|---|---:|---:|---:|---:|---:|---:|\n"
    );
    let mut rows = Vec::new();
    for ((path, trigger, delay), runs) in results {
        let usable = median(runs.iter().filter_map(|run| run.usable_ms).collect());
        let head = median(runs.iter().filter_map(|run| run.head_start_ms).collect());
        let count = runs.len();
        let issued = mean(runs.iter().map(|run| run.issued), count);
        let reused = mean(runs.iter().map(|run| run.reused), count);
        let unused = mean(runs.iter().map(|run| run.unused), count);
        table.push_str(&format!(
            "| {} | {} | {delay} | {} | {} | {issued:.1} | {reused:.1} | {unused:.1} |\n",
            path.name(),
            trigger.address(),
            ms(usable),
            ms(head),
        ));
        rows.push(json!({
            "path": format!("{path:?}"),
            "trigger": format!("{trigger:?}"),
            "added_latency_ms": delay,
            "click_to_usable_ms": usable,
            "head_start_ms": head,
            "reads_issued": issued,
            "reads_reused": reused,
            "reads_unused": unused,
            "runs": runs.iter().map(|run| json!({
                "click_to_usable_ms": run.usable_ms,
                "head_start_ms": run.head_start_ms,
                "issued": run.issued,
                "reused": run.reused,
                "unused": run.unused,
            })).collect::<Vec<_>>(),
        }));
    }
    let markdown = folder.join("results.md");
    std::fs::write(&markdown, table).expect("the results table");
    std::fs::write(
        folder.join("results.json"),
        serde_json::to_string_pretty(&json!({
            "revision": revision,
            "repeats": repeats,
            "rest_ms": REST_MS,
            "frame_ms": FRAME_MS,
            "key_gap_ms": KEY_GAP_MS,
            "cases": rows,
        }))
        .unwrap(),
    )
    .expect("the results data");
    markdown
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore = "browser comparison; run explicitly with --ignored"]
async fn each_preload_trigger_is_measured_on_the_same_paths() {
    let server = spawn_required_server().await;
    let base = format!("http://{}", server.address);
    let repeats: usize = std::env::var("PRELOAD_REPEATS")
        .ok()
        .and_then(|value| value.parse().ok())
        .unwrap_or(3);
    let results = tokio::task::spawn_blocking(move || {
        let mut page = Page::signed_in(&base);
        let mut results: BTreeMap<(Path, Trigger, u64), Vec<Run>> = BTreeMap::new();
        for _ in 0..repeats {
            for delay in LATENCIES {
                for trigger in Trigger::ALL {
                    for path in Path::ALL {
                        eprintln!("{path:?} {trigger:?} {delay} ms");
                        let run = walk(&mut page, path, trigger, delay);
                        results.entry((path, trigger, delay)).or_default().push(run);
                    }
                }
            }
        }
        results
    })
    .await
    .expect("the comparison walk");

    let file = write_results(&results, repeats);
    eprintln!("{}", std::fs::read_to_string(&file).unwrap());
    eprintln!("results: {}", file.display());

    for ((path, trigger, delay), runs) in &results {
        for run in runs {
            if !trigger.reads_ahead() {
                assert_eq!(
                    (run.reused, run.unused, run.head_start_ms),
                    (0, 0, None),
                    "click-only read ahead on {path:?} at {delay} ms"
                );
                continue;
            }
            // Resting on the opened row, by pointer or by keyboard, always reads it ahead.
            if path.opens().is_some() {
                assert_eq!(
                    run.reused, 2,
                    "{trigger:?} on {path:?} at {delay} ms: the open did not reuse both reads"
                );
            }
        }
    }
}

/// Reading ahead only reads: two GETs per patient, and a preload the server refuses or answers
/// with no patient leaves the page, the session and the stored drafts as they were. Opening that
/// patient then does what an open does.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore = "browser qualification; run explicitly with --ignored"]
async fn a_preload_changes_nothing_the_physician_can_see() {
    let server = spawn_required_server().await;
    let base = format!("http://{}", server.address);
    tokio::task::spawn_blocking(move || {
        let mut page = Page::signed_in(&base);
        let refused = json!({ "detail": "Sign in again." });
        page.load(
            Trigger::HoverFocus,
            0,
            json!([
                { "path": format!("/patient-workspaces/{JAKE}"), "status": 404, "body": {} },
                { "path": format!("/patient-workspaces/{DAN}"), "status": 401, "body": refused },
            ]),
        );
        for patient in [JAKE, DAN] {
            page.run(
                "keepDraft",
                &[json!(PRACTICE), json!(PHYSICIAN), json!(patient)],
            );
        }
        let drafts = page.run("draftKeys", &[]);
        assert_eq!(drafts.as_array().map(Vec::len), Some(2));

        let away = (640.0, 420.0);
        page.mouse("mouseMoved", away);
        page.requests();
        for name in ["Jake Smith", "Dan Cohen"] {
            let at = page.middle(VISIT, name);
            page.glide(away, at, 10);
            std::thread::sleep(Duration::from_millis(400));
        }
        let dan = page.middle(VISIT, "Dan Cohen");
        page.glide(dan, away, 10);
        std::thread::sleep(Duration::from_millis(500));

        let requests = page.requests();
        let mut patient_requests: Vec<(String, String)> = requests
            .iter()
            .filter(|request| {
                let path = request["path"].as_str().unwrap_or_default();
                path.contains(JAKE) || path.contains(DAN)
            })
            .map(|request| {
                (
                    request["method"].as_str().unwrap_or_default().to_owned(),
                    request["path"].as_str().unwrap_or_default().to_owned(),
                )
            })
            .collect();
        patient_requests.sort();
        let mut expected: Vec<(String, String)> = [JAKE, DAN]
            .iter()
            .flat_map(|id| {
                [
                    ("GET".to_owned(), format!("/patient-workspaces/{id}")),
                    (
                        "GET".to_owned(),
                        format!("/today?patient_workspace_id={id}"),
                    ),
                ]
            })
            .collect();
        expected.sort();
        assert_eq!(
            patient_requests, expected,
            "a preload asked for more than the two reads"
        );
        for request in &requests {
            assert_eq!(
                request["method"], "GET",
                "approaching a row sent a change: {request}"
            );
            let path = request["path"].as_str().unwrap_or_default();
            assert!(
                !path.contains("/chats/") && !path.contains("/physician-state/"),
                "approaching a row read a conversation or a draft: {request}"
            );
        }

        let text = page
            .run("text", &[])
            .as_str()
            .unwrap_or_default()
            .to_owned();
        assert!(
            text.contains(SIGNED_IN)
                && !text.contains("That patient is not available.")
                && !text.contains("Records as of"),
            "a preload changed the page:\n{text}"
        );
        assert_eq!(
            page.run("exists", &[json!(".session-prompt")]),
            json!(false),
            "a refused preload ended the session"
        );
        assert_eq!(
            page.run("draftKeys", &[]),
            drafts,
            "a preload forgot a stored draft"
        );

        // The open still does what an open does: no patient, so its drafts go.
        let jake = page.middle(VISIT, "Jake Smith");
        page.glide(away, jake, 10);
        page.click_at(jake);
        page.browser.wait_for("That patient is not available.");
        let kept = page.run("draftKeys", &[]);
        let kept: Vec<&str> = kept
            .as_array()
            .unwrap()
            .iter()
            .filter_map(Value::as_str)
            .collect();
        assert!(
            kept.len() == 1 && kept[0].contains(DAN),
            "opening the missing patient did not forget only its draft: {kept:?}"
        );
    })
    .await
    .expect("the read-only walk");
}

/// The pointer heading for a row reads it ahead before it arrives, and only with prediction on.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore = "browser qualification; run explicitly with --ignored"]
async fn the_pointer_heading_for_a_row_reads_it_before_arriving() {
    let server = spawn_required_server().await;
    let base = format!("http://{}", server.address);
    tokio::task::spawn_blocking(move || {
        let mut page = Page::signed_in(&base);
        for trigger in [Trigger::HoverFocus, Trigger::PointerPredicted] {
            page.load(trigger, 0, json!([]));
            let away = (640.0, 420.0);
            page.mouse("mouseMoved", away);
            page.requests();
            let to = page.middle(VISIT, "Tanvir Chowdhury");
            // Stop short of the row: the pointer never reaches it.
            let short = (to.0 + (away.0 - to.0) * 0.25, to.1 + (away.1 - to.1) * 0.25);
            page.glide(away, short, 12);
            std::thread::sleep(Duration::from_millis(400));
            let read = page.requests().iter().any(|request| names(request, TANVIR));
            assert_eq!(
                read,
                trigger == Trigger::PointerPredicted,
                "{trigger:?}: heading for a row read it ahead: {read}"
            );
        }
    })
    .await
    .expect("the prediction walk");
}
