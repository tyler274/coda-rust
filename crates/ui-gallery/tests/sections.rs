//! Opens every gallery section in Chromium through the documented command.
//!
//! The sections come from the page's own links, so a new family is covered the
//! moment the gallery shows it. A section fails on a panic, a console error,
//! sideways page overflow, or a heading or body that never appears.

// The shared client also carries helpers for journeys this test doesn't walk.
#[allow(dead_code)]
#[path = "../../mobile/tests/common/devtools.rs"]
mod devtools;

use std::net::TcpStream;
use std::path::PathBuf;
use std::process::{Child, Command};
use std::sync::{Mutex, MutexGuard};
use std::time::{Duration, Instant};

use devtools::{Browser, DESKTOP, PHONE, free_port};
use serde_json::Value;

/// How long a section's entry motion and first scripted steps get to run.
const SETTLE: Duration = Duration::from_secs(2);

/// Each start rewrites the one built bundle, so a test holds this while it serves it.
static BUNDLE: Mutex<()> = Mutex::new(());

#[test]
#[ignore = "browser qualification; run explicitly with --ignored"]
fn every_section_opens_at_desktop_and_phone_sizes() {
    let gallery = Gallery::start();
    let mut browser = Browser::launch();
    browser.watch_page_errors();
    let sections = sections(&mut browser, &gallery.url);

    let mut failures: Vec<String> = browser
        .take_page_errors()
        .into_iter()
        .map(|problem| format!("first load: {problem}"))
        .collect();
    for (name, size) in [("desktop", DESKTOP), ("phone", PHONE)] {
        browser.resize(size);
        for (slug, title) in &sections {
            let problems = open_section(&mut browser, &gallery.url, slug, title, size);
            println!(
                "{name} {}x{} #{slug}: {}",
                size.0,
                size.1,
                if problems.is_empty() { "ok" } else { "FAILED" }
            );
            failures.extend(
                problems
                    .into_iter()
                    .map(|problem| format!("{name} #{slug}: {problem}")),
            );
        }
    }
    println!(
        "opened {} sections at 2 sizes, {} failures",
        sections.len(),
        failures.len()
    );
    assert!(
        failures.is_empty(),
        "gallery sections failed:\n{}",
        failures.join("\n")
    );
}

/// A link swaps the section in place, and going back returns to the last one.
#[test]
#[ignore = "browser qualification; run explicitly with --ignored"]
fn links_and_going_back_change_the_section() {
    let gallery = Gallery::start();
    let mut browser = Browser::launch();
    browser.watch_page_errors();
    browser.text_until(&format!("{}#icons", gallery.url), "Icons");
    wait_for_heading(&mut browser, "Icons").expect("the linked section");

    browser.page().evaluate("window.sameDocument = true; 0");
    browser.click("a[href='#prose']");
    wait_for_heading(&mut browser, "Prose").expect("the section a link opens");
    let current = browser
        .page()
        .evaluate("document.querySelector('.gallery-family[aria-current]').textContent");
    assert_eq!(current.as_str(), Some("Prose"), "the link is not marked");

    browser.page().evaluate("history.back(); 0");
    wait_for_heading(&mut browser, "Icons").expect("the section before it");
    let kept = browser.page().evaluate("window.sameDocument === true");
    assert_eq!(
        kept.as_bool(),
        Some(true),
        "changing section reloaded the page"
    );
    assert_eq!(browser.take_page_errors(), Vec::<String>::new());
}

/// Replay entries throws the section away: its nodes and what was done to them go, and its entry
/// motion runs again.
#[test]
#[ignore = "browser qualification; run explicitly with --ignored"]
fn replay_entries_remounts_the_section() {
    let gallery = Gallery::start();
    let mut browser = Browser::launch();
    browser.watch_page_errors();

    browser.text_until(&format!("{}#inputs", gallery.url), "Inputs");
    wait_for_heading(&mut browser, "Inputs").expect("the section to replay");
    let checked =
        "document.querySelectorAll('.gallery-section input[type=checkbox]:checked').length";
    let first_state = browser.page().evaluate(checked);
    let ticked = browser.page().evaluate(
        "(() => { window.before = document.querySelector('.gallery-section'); \
         const box = document.querySelector('.gallery-section input[type=checkbox]:not(:checked)'); \
         box.click(); window.boxBefore = box; return box.checked; })()",
    );
    assert_eq!(ticked.as_bool(), Some(true), "the checkbox didn't tick");

    press_replay(&mut browser);
    wait_until_replaced(&mut browser);
    let reset = browser.page().evaluate(
        "(() => { const boxes = document.querySelectorAll('.gallery-section input[type=checkbox]'); \
         return !window.boxBefore.isConnected && boxes.length > 0; })()",
    );
    assert_eq!(
        reset.as_bool(),
        Some(true),
        "the checkbox survived the replay"
    );
    assert_eq!(
        browser.page().evaluate(checked),
        first_state,
        "the replayed section didn't return to its first state"
    );

    // Charts draw in on entry, so a replay there starts animations of its own.
    browser.click("a[href='#charts']");
    wait_for_heading(&mut browser, "Charts").expect("the section to replay");
    std::thread::sleep(SETTLE);
    browser.page().evaluate(
        "window.before = document.querySelector('.gallery-section'); window.entries = 0; \
         document.addEventListener('animationstart', () => { window.entries += 1; }, true); 0",
    );
    press_replay(&mut browser);
    wait_until_replaced(&mut browser);
    std::thread::sleep(SETTLE);
    let entries = browser.page().evaluate("window.entries");
    assert!(
        entries.as_u64().is_some_and(|count| count > 0),
        "the replay started no entry motion: {entries}"
    );
    assert_eq!(browser.take_page_errors(), Vec::<String>::new());
}

/// Slow motion plays what is running at quarter speed, and turning it off restores it.
#[test]
#[ignore = "browser qualification; run explicitly with --ignored"]
fn slow_motion_changes_the_pace_of_running_animations() {
    let gallery = Gallery::start();
    let mut browser = Browser::launch();
    browser.text_until(&format!("{}#status", gallery.url), "Status");
    wait_for_heading(&mut browser, "Status").expect("a section with running animations");
    let rates = "(() => { const all = document.getAnimations().map((a) => a.playbackRate); \
                 return [all.length, Math.min(...all), Math.max(...all)]; })()";
    let press = "document.querySelector('.gallery-header button[aria-pressed]').click(); 0";

    browser.page().evaluate(press);
    wait_for_value(&mut browser, rates, |rates| {
        rates[1] == 0.25 && rates[2] == 0.25
    })
    .expect("every animation at quarter speed");
    browser.page().evaluate(press);
    wait_for_value(&mut browser, rates, |rates| {
        rates[1] == 1.0 && rates[2] == 1.0
    })
    .expect("every animation back at full speed");
}

/// The components the rest of the gallery leaves out stand under their own names, in their states.
#[test]
#[ignore = "browser qualification; run explicitly with --ignored"]
fn tiles_spinners_and_retries_show_their_states() {
    let gallery = Gallery::start();
    let mut browser = Browser::launch();
    browser.watch_page_errors();

    browser.text_until(&format!("{}#icons", gallery.url), "Icons");
    wait_for_heading(&mut browser, "Icons").expect("the section with the tiles");
    assert_eq!(
        in_part(&mut browser, "Icon tiles", ".icon-tile:not(.is-tinted) svg"),
        1,
        "the neutral tile"
    );
    assert_eq!(
        in_part(&mut browser, "Icon tiles", ".icon-tile.is-tinted svg"),
        1,
        "the tinted tile"
    );

    browser.click("a[href='#status']");
    wait_for_heading(&mut browser, "Status").expect("the section with the spinner");
    assert_eq!(
        in_part(&mut browser, "Waiting on an action", ".spinner"),
        2,
        "the spinner bare and in its busy button"
    );
    assert_eq!(
        in_part(
            &mut browser,
            "Waiting on an action",
            "button[aria-busy=true] .spinner"
        ),
        1,
        "the spinner in its busy button"
    );

    assert_eq!(
        in_part(&mut browser, "Trying again", "button:not([disabled])"),
        1,
        "the retry that can be pressed"
    );
    assert_eq!(
        in_part(&mut browser, "Trying again", "button[disabled]"),
        1,
        "the retry that can't yet"
    );

    // Pressing one holds it while the retry runs, then offers it again.
    let enabled = "(() => { const part = Array.from(document.querySelectorAll('.gallery-part')) \
                   .find((part) => part.querySelector('.gallery-part-title').textContent === 'Trying again'); \
                   return [part.querySelectorAll('button[disabled]').length]; })()";
    browser.page().evaluate(
        "Array.from(document.querySelectorAll('.gallery-part')) \
         .find((part) => part.querySelector('.gallery-part-title').textContent === 'Trying again') \
         .querySelector('button:not([disabled])').click(); 0",
    );
    wait_for_value(&mut browser, enabled, |held| held[0] == 2.0)
        .expect("the pressed retry held while it runs");
    wait_for_value(&mut browser, enabled, |held| held[0] == 1.0).expect("the retry offered again");
    assert_eq!(browser.take_page_errors(), Vec::<String>::new());
}

/// An address that names no section opens the first rather than a blank page.
#[test]
#[ignore = "browser qualification; run explicitly with --ignored"]
fn an_unknown_address_opens_the_first_section() {
    let gallery = Gallery::start();
    let mut browser = Browser::launch();
    let (_, first) = sections(&mut browser, &gallery.url).remove(0);
    browser.page().navigate("about:blank");
    wait_until_blank(&mut browser);
    browser
        .page()
        .navigate(&format!("{}#no-such-family", gallery.url));
    wait_for_heading(&mut browser, &first).expect("the first section");
}

/// Presses Replay entries in the page's header.
fn press_replay(browser: &mut Browser) {
    browser.click(".gallery-header .control-primary");
}

/// Waits for `window.before`, the section marked before a replay, to leave the page.
fn wait_until_replaced(browser: &mut Browser) {
    let deadline = Instant::now() + Duration::from_secs(5);
    while Instant::now() < deadline {
        let replaced = browser
            .page()
            .evaluate("!window.before.isConnected && !!document.querySelector('.gallery-section')");
        if replaced.as_bool() == Some(true) {
            return;
        }
        std::thread::sleep(Duration::from_millis(25));
    }
    panic!("Replay entries left the section's nodes in place");
}

/// Waits until `expression`, a list of numbers, satisfies `done`.
fn wait_for_value(
    browser: &mut Browser,
    expression: &str,
    done: impl Fn(&[f64]) -> bool,
) -> Result<(), String> {
    let deadline = Instant::now() + Duration::from_secs(10);
    let mut seen = Value::Null;
    while Instant::now() < deadline {
        seen = browser.page().evaluate(expression);
        let numbers: Vec<f64> = seen
            .as_array()
            .map(|items| items.iter().filter_map(Value::as_f64).collect())
            .unwrap_or_default();
        if done(&numbers) {
            return Ok(());
        }
        std::thread::sleep(Duration::from_millis(25));
    }
    Err(format!("last saw {seen}"))
}

/// How many of `selector` the part titled `title` holds.
fn in_part(browser: &mut Browser, title: &str, selector: &str) -> u64 {
    browser
        .page()
        .evaluate(&format!(
            "(() => {{ const part = Array.from(document.querySelectorAll('.gallery-part')) \
             .find((part) => part.querySelector('.gallery-part-title').textContent === {title:?}); \
             return part ? part.querySelectorAll({selector:?}).length : 0; }})()"
        ))
        .as_u64()
        .unwrap_or_default()
}

/// The gallery's links, as `(slug, title)`.
fn sections(browser: &mut Browser, url: &str) -> Vec<(String, String)> {
    browser.text_until(url, "crates/ui");
    let links = browser.page().evaluate(
        "Array.from(document.querySelectorAll('.gallery-family'))\
         .map((link) => [link.getAttribute('href').slice(1), link.textContent])",
    );
    let sections: Vec<(String, String)> = links
        .as_array()
        .expect("a list of links")
        .iter()
        .map(|link| {
            let text = |index: usize| link[index].as_str().expect("link text").to_owned();
            (text(0), text(1))
        })
        .collect();
    assert!(
        sections.len() > 1,
        "the gallery listed no sections: {links}"
    );
    sections
}

/// Opens one section as a fresh page load and returns what is wrong with it.
fn open_section(
    browser: &mut Browser,
    url: &str,
    slug: &str,
    title: &str,
    size: (u32, u32),
) -> Vec<String> {
    // A navigation that changes only the fragment would keep the page, so leave it first.
    browser.page().navigate("about:blank");
    wait_until_blank(browser);
    browser.take_page_errors();
    browser.page().navigate(&format!("{url}#{slug}"));

    let mut problems = Vec::new();
    if let Err(problem) = wait_for_heading(browser, title) {
        problems.push(problem);
    } else {
        std::thread::sleep(SETTLE);
        problems.extend(wrong_size(browser, size));
        problems.extend(blank_body(browser));
        problems.extend(sideways_overflow(browser));
    }
    problems.extend(browser.take_page_errors());
    problems
}

fn wait_until_blank(browser: &mut Browser) {
    let deadline = Instant::now() + Duration::from_secs(10);
    while Instant::now() < deadline {
        let location = browser.page().evaluate("location.href");
        if location.as_str() == Some("about:blank") {
            return;
        }
        std::thread::sleep(Duration::from_millis(25));
    }
    panic!("the page never left the gallery");
}

/// Waits for the section's own heading.
fn wait_for_heading(browser: &mut Browser, title: &str) -> Result<(), String> {
    let deadline = Instant::now() + Duration::from_secs(30);
    let mut shown = Value::Null;
    while Instant::now() < deadline {
        shown = browser.page().evaluate(
            "(() => { const heading = document.querySelector('.gallery-title'); \
             return heading ? heading.textContent : ''; })()",
        );
        if shown.as_str() == Some(title) {
            return Ok(());
        }
        std::thread::sleep(Duration::from_millis(50));
    }
    Err(format!(
        "never showed its heading {title:?}, showing {shown}"
    ))
}

/// The page was not laid out at the size under test, so its overflow check would mean nothing.
fn wrong_size(browser: &mut Browser, size: (u32, u32)) -> Option<String> {
    let width = browser.page().evaluate("window.innerWidth");
    (width.as_u64() != Some(u64::from(size.0)))
        .then(|| format!("the viewport is {width}px wide, not {}px", size.0))
}

fn blank_body(browser: &mut Browser) -> Option<String> {
    let blank = browser.page().evaluate(
        "(() => { const stages = Array.from(document.querySelectorAll('.gallery-stage')); \
         return !stages.length || stages.some((stage) => !stage.childElementCount); })()",
    );
    (blank.as_bool() != Some(false)).then(|| "rendered nothing under a heading".to_owned())
}

/// The page scrolls sideways, with the elements reaching furthest past the edge.
fn sideways_overflow(browser: &mut Browser) -> Option<String> {
    let found = browser.page().evaluate(
        "(() => { const page = document.documentElement; \
         const over = page.scrollWidth - page.clientWidth; \
         if (over <= 0) return null; \
         const widest = Array.from(document.querySelectorAll('body *')) \
           .map((node) => [node.getBoundingClientRect().right - page.clientWidth, node]) \
           .filter(([past]) => past > 0).sort((a, b) => b[0] - a[0]).slice(0, 3) \
           .map(([past, node]) => `${node.tagName.toLowerCase()}.${node.getAttribute('class') ?? ''} +${Math.round(past)}px`); \
         return `the page overflows sideways by ${over}px: ${widest.join(', ')}`; })()",
    );
    found.as_str().map(str::to_owned)
}

/// The documented command, serving on a port of its own.
struct Gallery {
    server: Child,
    url: String,
    _bundle: MutexGuard<'static, ()>,
}

impl Gallery {
    fn start() -> Self {
        let bundle = BUNDLE
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        let workspace = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
        let port = free_port().expect("a free local port for the gallery");
        let mut command = Command::new(workspace.join("scripts/ui-gallery.sh"));
        // Cargo's own variables for this test would change the inner build's fingerprints.
        for (key, _) in std::env::vars_os() {
            let key = key.to_string_lossy().into_owned();
            let cargo_home = matches!(
                key.as_str(),
                "CARGO_HOME" | "CARGO_TARGET_DIR" | "CARGO_NET_OFFLINE"
            );
            if key == "LD_LIBRARY_PATH" || (key.starts_with("CARGO_") && !cargo_home) {
                command.env_remove(key);
            }
        }
        let mut server = command
            .env("UI_GALLERY_LISTEN_ADDR", format!("127.0.0.1:{port}"))
            .spawn()
            .expect("scripts/ui-gallery.sh");
        // Builds the gallery first, so this allows for a cold WebAssembly build.
        let deadline = Instant::now() + Duration::from_secs(1200);
        while TcpStream::connect(("127.0.0.1", port)).is_err() {
            if let Some(status) = server.try_wait().expect("the gallery command") {
                panic!("scripts/ui-gallery.sh exited before serving: {status}");
            }
            if Instant::now() > deadline {
                let _ = server.kill();
                let _ = server.wait();
                panic!("scripts/ui-gallery.sh never served port {port}");
            }
            std::thread::sleep(Duration::from_millis(100));
        }
        Self {
            server,
            url: format!("http://127.0.0.1:{port}/"),
            _bundle: bundle,
        }
    }
}

impl Drop for Gallery {
    fn drop(&mut self) {
        let _ = self.server.kill();
        let _ = self.server.wait();
    }
}
