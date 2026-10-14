//! Compares the live React practice with the Rust sample page.
//!
//! Both documents have to show the same patient facts. A closed React port, an
//! empty document, or a missing chip fails the test.

use std::env;
use std::path::PathBuf;
use std::process::{Child, Command};
use std::thread;
use std::time::{Duration, Instant};

#[path = "../../mobile/tests/common/devtools.rs"]
mod devtools;

use devtools::{Page, connect, targets};

const MARY: &str = "pw-39e51534-6ce4-43cf-b5f8-7c1ca6252d8e";

#[tokio::test]
async fn the_live_react_practice_matches_the_rust_page() {
    assert!(
        std::net::TcpStream::connect("127.0.0.1:8787").is_ok(),
        "the React practice is not serving on 127.0.0.1:8787"
    );
    let site = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../target/physician-web");
    assert!(
        site.join("physician-app.js").is_file() && site.join("physician-app_bg.wasm").is_file(),
        "the Rust physician page has not been built at {}",
        site.display()
    );
    // SAFETY: this test process is the only one that reads the variable.
    unsafe { env::set_var("PHYSICIAN_SITE_DIR", &site) };
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    tokio::spawn(async move { physician_sample::serve(listener).await });
    wait_until_ready(address).await;

    let mut browser = Browser::launch();
    let react_today = browser.text_until("http://127.0.0.1:8787/", "Today");
    let react_mary = browser.text_until(
        &format!("http://127.0.0.1:8787/patients/{MARY}"),
        "Patient summary",
    );
    let rust_today = browser.text_until(&format!("http://{address}/"), "3:30 PM");
    let rust_mary = browser.text_until(
        &format!("http://{address}/?patient={MARY}"),
        "Patient summary",
    );

    assert_today("React", &react_today);
    assert_today("Rust", &rust_today);
    assert!(
        rust_today.contains("3:30 PM") && rust_today.contains("Mary Louis"),
        "Rust Today lost the 3:30 PM visit:\n{rust_today}"
    );
    assert!(
        react_today.contains("Mary Louis") || react_today.contains("No visits remaining"),
        "React Today showed neither Mary Louis nor the later-clock visit list:\n{react_today}"
    );
    assert_patient("React", &react_mary);
    assert_patient("Rust", &rust_mary);
}

fn assert_today(name: &str, text: &str) {
    assert!(
        text.contains("Today")
            && text.contains("Find patient")
            && text.contains("Coda hasn’t read Gmail yet.")
            && text.contains("Coda hasn’t read Spruce yet.")
            && !text.contains("Nothing needs you right now."),
        "{name} Today did not match the practice:\n{text}"
    );
}

fn assert_patient(name: &str, text: &str) {
    assert!(
        text.contains("Patient summary")
            && text.contains("Age 54 years")
            && text.contains("MRN 00000005")
            && text.contains("Gender Female")
            && text.contains("[1]")
            && !text.contains("[[patient_source")
            && !text.contains("Write a message")
            && !text.contains("Visit preparation failed. No new briefing was completed."),
        "{name} workspace did not match Mary Louis:\n{text}"
    );
}

struct Browser {
    child: Child,
    page: Page,
}

impl Browser {
    fn launch() -> Self {
        let port = 9333;
        let child = Command::new(chromium())
            .args([
                "--headless=new",
                "--disable-gpu",
                "--no-sandbox",
                &format!("--remote-debugging-port={port}"),
                "about:blank",
            ])
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .spawn()
            .expect("chromium");
        let started = Instant::now();
        let target = loop {
            if started.elapsed() > Duration::from_secs(15) {
                panic!("chromium did not open a DevTools page");
            }
            if let Ok(pages) = targets(port)
                && let Some(target) = pages.into_iter().next()
            {
                break target;
            }
            thread::sleep(Duration::from_millis(100));
        };
        let page = connect(port, &target.socket_path).expect("chromium DevTools");
        Self { child, page }
    }

    fn text_until(&mut self, url: &str, needle: &str) -> String {
        self.page.navigate(url);
        let started = Instant::now();
        let mut last = String::new();
        while started.elapsed() < Duration::from_secs(20) {
            if let Some(text) = self
                .page
                .evaluate("document.body ? document.body.innerText : \"\"")
                .as_str()
            {
                last = text.to_owned();
                if last.contains(needle) {
                    return last;
                }
            }
            thread::sleep(Duration::from_millis(250));
        }
        panic!("{url} never showed {needle}:\n{last}");
    }
}

impl Drop for Browser {
    fn drop(&mut self) {
        let _ = self.child.kill();
    }
}

fn chromium() -> PathBuf {
    if let Some(path) = env::var_os("CHROMIUM") {
        return PathBuf::from(path);
    }
    for name in [
        "chromium",
        "chromium-browser",
        "google-chrome",
        "google-chrome-stable",
    ] {
        if let Some(path) = find_on_path(name) {
            return path;
        }
    }
    panic!("chromium is required to validate the practice against React");
}

fn find_on_path(name: &str) -> Option<PathBuf> {
    let paths = env::var_os("PATH")?;
    env::split_paths(&paths)
        .map(|dir| dir.join(name))
        .find(|path| path.is_file())
}

async fn wait_until_ready(address: std::net::SocketAddr) {
    let started = Instant::now();
    loop {
        if started.elapsed() > Duration::from_secs(10) {
            panic!("the Rust practice did not accept connections on {address}");
        }
        if tokio::net::TcpStream::connect(address).await.is_ok() {
            return;
        }
        tokio::time::sleep(Duration::from_millis(50)).await;
    }
}
