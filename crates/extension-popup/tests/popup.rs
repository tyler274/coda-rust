//! Loads the built extension in Chromium and drives its popup page: verifies its manifest name,
//! optional host permissions before connection, CSP `eval` refusal, storage hygiene (no persisted
//! sign-in tokens), and keyboard menu navigation.

#[allow(dead_code)]
#[path = "../../mobile/tests/common/devtools.rs"]
mod devtools;

use std::path::PathBuf;
use std::process::Command;
use std::time::{Duration, Instant};

use devtools::{Browser, Key};
use serde_json::{Value, json};

#[test]
#[ignore = "browser qualification; run explicitly with --ignored"]
fn the_popup_runs_under_its_own_page_policy() {
    let workspace = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let extension = workspace.join("extension/chrome");
    build_popup(&workspace, &extension);
    let identity: Value = serde_json::from_str(include_str!(
        "../../../extension/sandbox/extension-identity.json"
    ))
    .expect("the extension's identity");
    let id = identity["id"].as_str().expect("the extension's id");

    let mut browser = Browser::launch_with_extension(&extension);
    browser.watch_page_errors();
    let page = format!("chrome-extension://{id}/options.html");
    let deadline = Instant::now() + Duration::from_secs(30);
    while !holds(&mut browser, "document.querySelector('.top-bar') !== null") {
        assert!(Instant::now() < deadline, "{page} never rendered");
        browser.page().navigate(&page);
        std::thread::sleep(Duration::from_millis(500));
    }
    browser.take_page_errors();
    browser.page().navigate(&page);
    wait(&mut browser, "document.querySelector('.top-bar') !== null");

    assert_eq!(
        strictly(
            &mut browser,
            "(() => { try { eval('1'); return false; } catch { return true; } })()",
        ),
        Value::Bool(true),
        "the page should refuse eval, or this proves nothing"
    );

    assert_eq!(
        browser.page().evaluate("chrome.runtime.getManifest().name"),
        Value::String("Coda".to_owned()),
        "extension manifest name must be Coda"
    );

    // Reader host permissions are optional and must not be pre-granted before connecting.
    let pre_granted = browser.page().call_session(
        "Runtime.evaluate",
        json!({
            "expression": "chrome.permissions.contains({ origins: ['https://app.hint.com/*', 'https://api.hint.com/*'] })",
            "awaitPromise": true,
            "returnByValue": true
        }),
    )["result"]["result"]["value"]
        .clone();
    assert_eq!(
        pre_granted,
        Value::Bool(false),
        "optional reader host permissions must not be pre-granted before connecting"
    );

    // Unpaired fresh extension asks to pair with Coda first.
    wait(
        &mut browser,
        "document.body.innerText.includes('Pair with Coda')",
    );

    // Storage hygiene: neither chrome.storage.local nor chrome.storage.session holds any token on disk/start.
    let storage_clean = browser.page().call_session(
        "Runtime.evaluate",
        json!({
            "expression": "(async () => { const local = await chrome.storage.local.get(null); const sess = await chrome.storage.session.get(null); const s = JSON.stringify({local, sess}); return !s.includes('id_token') && !s.includes('refresh_token') && !s.includes('Bearer '); })()",
            "awaitPromise": true,
            "returnByValue": true
        }),
    )["result"]["result"]["value"]
        .clone();
    assert_eq!(storage_clean, Value::Bool(true));

    let at = browser.page().evaluate(
        "(() => { const box = document.querySelector('[popovertarget]').getBoundingClientRect(); \
         return [box.x + box.width / 2, box.y + box.height / 2]; })()",
    );
    let (x, y) = (at[0].clone(), at[1].clone());
    for kind in ["mouseMoved", "mousePressed", "mouseReleased"] {
        browser.page().call_session(
            "Input.dispatchMouseEvent",
            json!({ "type": kind, "x": x, "y": y, "button": "left", "clickCount": 1 }),
        );
    }
    wait(
        &mut browser,
        "document.querySelector('.menu:popover-open') !== null",
    );
    wait(
        &mut browser,
        "document.activeElement.closest('.menu') !== null",
    );
    browser.press_key(Key::ARROW_DOWN);
    wait(
        &mut browser,
        "document.activeElement.matches('[role^=\"menuitem\"]')",
    );
    browser.press_key(Key::ESCAPE);
    wait(
        &mut browser,
        "document.querySelector('.menu:popover-open') === null",
    );
    assert_eq!(browser.take_page_errors(), Vec::<String>::new());
}

fn build_popup(workspace: &PathBuf, extension: &std::path::Path) {
    let run = |command: &mut Command| {
        for (key, _) in std::env::vars_os() {
            let key = key.to_string_lossy().into_owned();
            let kept = matches!(
                key.as_str(),
                "CARGO_HOME" | "CARGO_TARGET_DIR" | "CARGO_NET_OFFLINE"
            );
            if key.starts_with("CARGO_") && !kept {
                command.env_remove(key);
            }
        }
        let status = command.current_dir(workspace).status().expect("the build");
        assert!(status.success(), "{command:?} failed: {status}");
    };
    run(Command::new("cargo").args([
        "build",
        "-p",
        "extension-popup",
        "--target",
        "wasm32-unknown-unknown",
        "--features",
        "web",
    ]));
    let target = std::env::var_os("CARGO_TARGET_DIR")
        .map_or_else(|| workspace.join("target"), PathBuf::from);
    run(Command::new("wasm-bindgen")
        .args(["--target", "web", "--no-typescript", "--out-dir"])
        .arg(extension.join("popup"))
        .arg(target.join("wasm32-unknown-unknown/debug/extension-popup.wasm")));
}

fn holds(browser: &mut Browser, expression: &str) -> bool {
    strictly(browser, expression) == Value::Bool(true)
}

fn strictly(browser: &mut Browser, expression: &str) -> Value {
    browser.page().call_session(
        "Runtime.evaluate",
        json!({ "expression": expression, "returnByValue": true, "allowUnsafeEvalBlockedByCSP": false }),
    )["result"]["result"]["value"]
        .clone()
}

fn wait(browser: &mut Browser, expression: &str) {
    let deadline = Instant::now() + Duration::from_secs(10);
    while !holds(browser, expression) {
        assert!(
            Instant::now() < deadline,
            "timed out waiting for {expression}"
        );
        std::thread::sleep(Duration::from_millis(25));
    }
}
