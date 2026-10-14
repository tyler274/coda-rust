//! The shared Chromium proof helpers launch and drive a real page.

#[path = "common/devtools.rs"]
mod devtools;

use devtools::{Browser, DESKTOP, PHONE};

/// A browser boots on its own port and reports the body text.
#[test]
fn a_browser_boots_and_reads_a_page() {
    let served = serve_one_page();
    let mut browser = Browser::launch();
    let text = browser.text_until(&format!("http://{}", served), "proof page");
    assert!(
        text.contains("proof page"),
        "the page did not render:\n{text}"
    );
    assert_ne!(browser.port(), 0, "the browser booted on port 0");
}

/// Two browsers get two different ports, so proofs run in parallel.
#[test]
fn two_browsers_take_two_ports() {
    let first = Browser::launch();
    let second = Browser::launch();
    assert_ne!(
        first.port(),
        second.port(),
        "two browsers shared one DevTools port"
    );
}

/// Clicking and typing reach the page: the input takes the text and the
/// button's click flips the flag the page shows.
#[test]
fn click_and_type_reach_the_page() {
    let served = serve_one_page();
    let mut browser = Browser::launch();
    browser.text_until(&format!("http://{}", served), "proof page");
    browser.type_into("#proof-input", "typed-value");
    browser.click("#proof-button");
    let text = browser.wait_for("clicked: typed-value");
    assert!(text.contains("clicked: typed-value"), "{text}");
}

/// The viewport helpers: the emulation reports the phone metrics and the
/// reduced-motion media query flips.
#[test]
fn viewports_resize_and_motion_emulates() {
    let _ = serve_one_page();
    let mut browser = Browser::launch();
    browser.resize(PHONE);
    let phone = browser
        .page()
        .evaluate("window.innerWidth + 'x' + window.innerHeight");
    assert_eq!(
        phone.as_str(),
        Some("390x844"),
        "the phone emulation did not take: {phone}"
    );
    browser.resize(DESKTOP);
    let desktop = browser
        .page()
        .evaluate("window.innerWidth + 'x' + window.innerHeight");
    assert_eq!(
        desktop.as_str(),
        Some("1280x800"),
        "the desktop emulation did not take: {desktop}"
    );
    browser.emulate_reduced_motion();
    assert!(
        browser.prefers_reduced_motion(),
        "the emulated reduced-motion preference did not reach the page"
    );
}

/// Serves one static page with the inputs the helpers drive.
fn serve_one_page() -> std::net::SocketAddr {
    let listener = std::net::TcpListener::bind("127.0.0.1:0").expect("listener");
    let address = listener.local_addr().expect("address");
    std::thread::spawn(move || {
        let page = concat!(
            "<!doctype html><html><body><h1>proof page</h1>",
            "<input id=\"proof-input\" oninput=\"window.proofValue=this.value\">",
            "<button id=\"proof-button\" onclick=\"document.body.insertAdjacentHTML(",
            "'beforeend','<p>clicked: '+(window.proofValue||'')+'</p>')\">go</button>",
            "</body></html>"
        );
        for stream in listener.incoming().flatten() {
            let mut stream = stream;
            let response = format!(
                "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                page.len(),
                page
            );
            let _ = std::io::Write::write_all(&mut stream, response.as_bytes());
        }
    });
    address
}
