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
