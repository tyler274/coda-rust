//! Every stage of the physician sign-in page, walked in a real browser the way a person reaches it,
//! at desktop and phone sizes in light and dark.
//!
//! The sample server signs in by itself, so `js/sign_in_states.js` scripts the answers it never
//! gives: a mailed link, Coda out of reach, no access. With `SIGN_IN_SHOTS` set to a folder, each
//! stage is also saved there as a screenshot, so two revisions can be compared.

use std::time::{Duration, Instant};

use base64::Engine;
use serde_json::{Value, json};

mod common;
#[allow(dead_code)]
#[path = "../../mobile/tests/common/devtools.rs"]
mod devtools;
mod required_auth;

use devtools::{Browser, DESKTOP, Key, PHONE};
use required_auth::{END_THE_SESSION, SIGNED_IN, STAGING_EMAIL, spawn_required_server};

const EMAIL_FIELD: &str = r#"input[type="email"]"#;

/// What Coda says when a link is on its way, or when there's no such account.
const LINK_ON_THE_WAY: &str = "If that address has an account, a sign-in link is on its way.";

/// A mailed link's answer to a sign-in request.
fn mailed(hold: bool) -> Value {
    json!({ "path": "/auth/sign-in", "status": 200, "hold": hold,
            "body": { "status": "unavailable", "message": LINK_ON_THE_WAY } })
}

struct Walk {
    browser: Browser,
    size: &'static str,
    theme: &'static str,
    shots: Option<std::path::PathBuf>,
}

impl Walk {
    fn launch(size: (&'static str, (u32, u32)), theme: &'static str) -> Self {
        let mut browser = Browser::launch();
        browser.resize(size.1);
        browser.page().call_session("Page.enable", json!({}));
        browser.page().call_session(
            "Page.addScriptToEvaluateOnNewDocument",
            json!({ "source": include_str!("js/sign_in_states.js") }),
        );
        Self {
            browser,
            size: size.0,
            theme,
            shots: std::env::var_os("SIGN_IN_SHOTS").map(Into::into),
        }
    }

    /// Runs `name` from `js/sign_in_walk.js` with `args`.
    fn run(&mut self, name: &str, args: &[Value]) -> Value {
        let mut arguments = vec![json!(name)];
        arguments.extend_from_slice(args);
        self.browser
            .page()
            .call_function(include_str!("js/sign_in_walk.js"), &arguments)
    }

    /// Loads `url` until it says `needle`, in this walk's theme. The app has no theme setting of its
    /// own: dark is the library's `data-theme`.
    fn open(&mut self, url: &str, needle: &str) {
        self.browser.text_until(url, needle);
        let theme = self.theme;
        self.run("theme", &[json!(theme)]);
    }

    /// Scripts the next answer to a request.
    fn answer(&mut self, step: Value) {
        self.run("answer", &[step]);
    }

    /// Presses the button with these words once it can be pressed.
    fn press(&mut self, words: &str) {
        let deadline = Instant::now() + Duration::from_secs(10);
        while self.run("press", &[json!(words)]) != json!(true) {
            assert!(Instant::now() < deadline, "no enabled button says {words}");
            std::thread::sleep(Duration::from_millis(50));
        }
    }

    fn field_value(&mut self) -> Value {
        self.run("field", &[json!(EMAIL_FIELD)])["value"].clone()
    }

    /// Holds the stage on screen: no sideways scroll, and a screenshot once it has arrived.
    fn stage(&mut self, name: &str) {
        let overflow = self.run("sidewaysScroll", &[]);
        assert!(
            overflow.as_f64().is_some_and(|by| by <= 0.0),
            "{name} scrolls sideways by {overflow} at {} {}",
            self.size,
            self.theme
        );
        let Some(folder) = self.shots.clone() else {
            return;
        };
        std::thread::sleep(Duration::from_millis(500));
        let shot = self
            .browser
            .page()
            .call_session("Page.captureScreenshot", json!({ "format": "png" }));
        let data = shot
            .pointer("/result/data")
            .and_then(Value::as_str)
            .expect("a screenshot");
        let bytes = base64::engine::general_purpose::STANDARD
            .decode(data)
            .expect("a screenshot's bytes");
        std::fs::create_dir_all(&folder).expect("the screenshot folder");
        let file = folder.join(format!("{}-{}-{name}.png", self.size, self.theme));
        std::fs::write(file, bytes).expect("the screenshot");
    }
}

fn walk_every_stage(base: &str, size: (&'static str, (u32, u32)), theme: &'static str) {
    let mut walk = Walk::launch(size, theme);

    // The email stage: nothing to send until there's an address, by button or by Enter.
    walk.open(base, "Sign in to Coda");
    walk.stage("01-email-empty");
    assert_eq!(
        walk.run("disabled", &[json!("Send sign-in link")]),
        json!(true),
        "an empty address could be sent"
    );
    assert_eq!(
        walk.run("field", &[json!(EMAIL_FIELD)])["autocomplete"],
        json!("email"),
        "the browser can't offer an address"
    );
    walk.run("focus", &[json!(EMAIL_FIELD)]);
    walk.browser.press_key(Key::ENTER);
    std::thread::sleep(Duration::from_millis(200));
    let text = walk.browser.body_text().unwrap_or_default();
    assert!(
        text.contains("Sign in to Coda"),
        "Enter sent an empty address:\n{text}"
    );
    walk.browser.type_into(EMAIL_FIELD, STAGING_EMAIL);
    walk.stage("02-email-filled");

    // Loading, then the mailed link's confirmation.
    walk.answer(mailed(true));
    walk.press("Send sign-in link");
    walk.browser.wait_for("Sending your link…");
    walk.stage("03-requesting");
    walk.run("release", &[]);
    let text = walk.browser.wait_for("Check your email");
    assert!(
        text.contains(LINK_ON_THE_WAY) && text.contains(STAGING_EMAIL),
        "the confirmation lost its words:\n{text}"
    );
    walk.stage("04-check-email");

    // Error: Coda out of reach, said aloud. The address survives the way back.
    walk.answer(json!({ "path": "/auth/sign-in", "status": 500, "body": {} }));
    walk.press("Send a new link");
    walk.browser.wait_for("Let’s get you signed in");
    let alert = walk.run("alert", &[]);
    assert!(
        alert
            .as_str()
            .is_some_and(|words| words.contains("Coda couldn’t")),
        "the failure wasn't announced: {alert}"
    );
    walk.stage("05-recovery");
    walk.press("Use a different email");
    walk.browser.wait_for("Sign in to Coda");
    assert_eq!(walk.field_value(), json!(STAGING_EMAIL));

    // Signing in, on a link that no longer gives access. The way back clears the address.
    walk.answer(json!({ "path": "/auth/verify", "status": 403, "hold": true, "body": {} }));
    walk.press("Send sign-in link");
    walk.browser.wait_for("Signing you in…");
    walk.stage("06-signing-in");
    walk.run("release", &[]);
    walk.browser.wait_for("no longer gives you access");
    walk.stage("07-no-access");
    walk.press("Use a different email");
    walk.browser.wait_for("Sign in to Coda");
    assert_eq!(walk.field_value(), json!(""));

    // Enter sends from the field, as the button does.
    walk.browser.type_into(EMAIL_FIELD, STAGING_EMAIL);
    walk.run("focus", &[json!(EMAIL_FIELD)]);
    walk.browser.press_key(Key::ENTER);
    walk.browser.wait_for(SIGNED_IN);

    // The session ends: the prompt covers the page, then mails the same address, and offers no
    // other one.
    walk.browser.page().evaluate(END_THE_SESSION);
    let text = walk.browser.wait_for("Sign in to continue");
    assert!(
        text.contains(STAGING_EMAIL),
        "the prompt lost the address:\n{text}"
    );
    walk.stage("08-session-prompt");
    walk.answer(mailed(false));
    walk.press("Email me a sign-in link");
    let text = walk.browser.wait_for("Check your email");
    assert!(
        !text.contains("Use a different email"),
        "the prompt offered another address:\n{text}"
    );
    walk.stage("09-prompt-check-email");
    walk.press("Send a new link");
    let deadline = Instant::now() + Duration::from_secs(30);
    while walk.run("promptUp", &[]) == json!(true) {
        assert!(
            Instant::now() < deadline,
            "signing in again never lifted the prompt"
        );
        std::thread::sleep(Duration::from_millis(100));
    }

    // The practice removes this physician's access: every read presenting the session is refused.
    let refused =
        json!({ "detail": "The practice removed your access.", "reason": "access_removed" });
    let removed: Vec<Value> = [
        "/auth/whoami",
        "/today",
        "/patient-workspaces",
        "/practice-day",
        "/recent-conversations",
        "/updates",
    ]
    .map(|path| json!({ "path": path, "status": 401, "keep": true, "body": refused }))
    .into();
    walk.run("answerNextLoad", &[Value::from(removed)]);
    walk.open(base, "The practice removed your access");
    walk.stage("10-access-removed");
    walk.press("Use a different email");
    walk.browser.wait_for("Sign in to Coda");
    walk.run("forgetAnswers", &[]);

    // Recovery: a used link says so, and a new one signs in.
    walk.open(
        &format!("{base}/?email={STAGING_EMAIL}&code=not-a-real-code"),
        "no longer valid",
    );
    walk.stage("11-link-no-longer-valid");
    walk.press("Send a new link");
    walk.browser.wait_for(SIGNED_IN);
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore = "browser qualification; run explicitly with --ignored"]
async fn every_sign_in_stage_keeps_its_words_and_its_way_on() {
    let server = spawn_required_server().await;
    let base = format!("http://{}", server.address);
    for size in [("desktop", DESKTOP), ("phone", PHONE)] {
        for theme in ["light", "dark"] {
            walk_every_stage(&base, size, theme);
        }
    }
}
