//! Loads the physician page in Chromium and fails when the document is empty.
//! Dioxus mounts into `#main`. A missing element leaves the visible page blank.

use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::sync::{Arc, OnceLock};
use std::time::{Duration, Instant};

use axum::response::IntoResponse;
use base64::Engine as _;
use physician_auth::{AuthStore, Membership, MemoryAuthStore, PhysicianIdentity};
use tokio::sync::Mutex;

mod common;
#[allow(dead_code)]
#[path = "../../mobile/tests/common/devtools.rs"]
mod devtools;
#[path = "scale/mod.rs"]
mod scale;

use devtools::{Browser, Key};

static TEST_MUTEX: OnceLock<Mutex<()>> = OnceLock::new();
static BUILT_SITE: OnceLock<PathBuf> = OnceLock::new();

fn test_lock() -> &'static Mutex<()> {
    TEST_MUTEX.get_or_init(|| Mutex::new(()))
}

#[tokio::test]
#[ignore = "browser qualification; run explicitly with --ignored"]
async fn the_practice_and_its_patients_are_visible() {
    let _guard = test_lock().lock().await;
    let site = build_site();
    // Static fixtures have no model transport or operator state.
    let server = common::serve_isolated(|| {
        // SAFETY: tests in this binary hold `test_lock()` before mutating
        // process env, and `serve_isolated` holds its start lock here.
        unsafe {
            env::set_var("PHYSICIAN_SITE_DIR", &site);
            env::remove_var("PHYSICIAN_AUTH");
            env::remove_var("PHYSICIAN_AUTH_PROOF_TOKENS");
            env::remove_var("PHYSICIAN_UPSTREAM");
        }
    })
    .await;
    let address = server.address;

    let chrome = chromium();
    let dom = dump_dom(&chrome, &format!("http://{address}/")).await;
    assert!(
        dom.contains("Today")
            && dom.contains("Aug 27, 2026")
            && dom.contains("Nothing needs you right now.")
            && dom.contains("m3 11 9-8 9 8")
            && dom.contains("viewBox=\"0 0 24 24\"")
            && dom.contains("stroke-width=\"1.8\"")
            && dom.contains("box-sizing: border-box")
            && !dom.contains("Connections"),
        "Today did not match the React shell:\n{dom}"
    );
    let workspace = dump_dom(
        &chrome,
        &format!("http://{address}/?patient=pw-39e51534-6ce4-43cf-b5f8-7c1ca6252d8e"),
    )
    .await;
    assert!(
        workspace.contains("Mary Louis")
            && workspace.contains("Age 54 years")
            && workspace.contains("MRN 00000005")
            && workspace.contains("Gender Female")
            && workspace.contains("Records as of")
            && workspace.contains("Today, 12:00 PM")
            && !workspace.contains("Patient summary")
            && !workspace.contains("Write a message"),
        "the workspace did not show the patient:\n{workspace}"
    );
    // `?chat=` opens a conversation the workspace does not list.
    let chat = dump_dom(
        &chrome,
        &format!(
            "http://{address}/?patient=pw-39e51534-6ce4-43cf-b5f8-7c1ca6252d8e&chat=chat-from-address"
        ),
    )
    .await;
    assert!(
        chat.contains("Records as of")
            && chat.contains("What would you like Coda to review?")
            && chat.contains("Write a message")
            && !chat.contains("This workspace has no chats."),
        "?chat= did not open the conversation:\n{chat}"
    );
    let logo = tokio::task::spawn_blocking(move || read_logo(address))
        .await
        .expect("logo read");
    let head = String::from_utf8_lossy(&logo[..logo.len().min(80)]);
    assert!(
        head.contains("200") && logo.windows(8).any(|window| window == b"\x89PNG\r\n\x1a\n"),
        "the reference logo was not a PNG:\n{head}"
    );
}

/// New conversation, send, then a reload without `?chat=`: the workspace
/// lists the chat the physician started, titled by its first message.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore = "browser qualification; run explicitly with --ignored"]
async fn a_new_conversation_is_listed_after_a_reload_without_its_address() {
    let _guard = test_lock().lock().await;
    let site = build_site();
    let server = serve_chat_browser(&site, scale::FIXED_TEXT).await;
    let address = server.address;
    let patient = "pw-39e51534-6ce4-43cf-b5f8-7c1ca6252d8e";
    let workspace_url = format!("http://{address}/?patient={patient}");
    let mut browser = Tab::open(&workspace_url);
    browser.wait_for_text("Work email", Duration::from_secs(20));
    browser.request_sign_in("physician@codahealth.test");
    browser.wait_for_text("This workspace has no chats.", Duration::from_secs(20));

    // The empty state and the sidebar both offer it; use the sidebar's.
    browser.click("button.sidebar-new-conversation");
    browser.wait_for_text(
        "What would you like Coda to review?",
        Duration::from_secs(10),
    );
    let address_chat = browser.eval("new URL(location.href).searchParams.get('chat')");
    let chat = address_chat.as_str().unwrap_or_default().to_owned();
    assert!(
        chat.starts_with("chat-"),
        "the address names no chat: {address_chat}"
    );

    let message = "Review the discharge summary before the follow-up visit";
    let buttons = browser.eval(
        "[...document.querySelectorAll('.composer-toolbar button')].map(b => b.textContent.trim())",
    );
    assert_eq!(
        buttons,
        serde_json::json!(["Dictate", "Send"]),
        "the composer still offers a manual Save: {buttons}"
    );
    browser.type_composer(message);
    browser.wait_saved();
    browser.save_pictures("draft-save-status", ".composer-context");
    assert_eq!(
        browser.composer().1,
        "",
        "successful draft saves must be silent"
    );
    let (typed, _) = browser.composer();
    browser.type_composer(&format!("{typed} "));
    std::thread::sleep(Duration::from_millis(100));
    let (_, edited) = browser.composer();
    assert_eq!(edited, "", "an edit left the Saved mark: {edited}");
    browser.type_composer(message);
    browser.click_named(".composer-toolbar button", "Send");
    // Acceptance saves the turn before the worker calls the provider.
    browser.wait_for_text(message, Duration::from_secs(15));

    browser.navigate(&workspace_url);
    let html = browser.wait_for_text(message, Duration::from_secs(20));
    assert!(
        !html.contains("This workspace has no chats.")
            && html.contains("sidebar-conversation-link"),
        "the reloaded workspace did not list the new chat:\n{html}"
    );
    let reopened = browser.eval("new URL(location.href).searchParams.get('chat')");
    assert_eq!(
        reopened,
        serde_json::json!(chat),
        "the address must reflect the conversation selected after a workspace-only reload: {reopened}"
    );

    // A second new conversation never shows the listed chat's lines, not even
    // for the frame before its own (empty) read answers. Every render after the
    // click is checked until the empty state; the click itself changes no DOM,
    // so the page before it (still showing the old chat) is not counted.
    let stale = browser.step("newConversationShowing", serde_json::json!([message]));
    assert_eq!(
        stale,
        serde_json::json!({"seen": false, "done": true}),
        "a new conversation showed the previous chat's lines: {stale}"
    );
    browser.click("button.sidebar-conversation-link");
    browser.wait_for_text(message, Duration::from_secs(10));
    let selected = browser.eval("new URL(location.href).searchParams.get('chat')");
    assert_eq!(
        selected,
        serde_json::json!(chat),
        "selecting the saved conversation left the new conversation in the address: {selected}"
    );
    let previous_document = browser.eval("performance.timeOrigin");
    browser.reload();
    let reloaded_document = browser.eval("performance.timeOrigin");
    assert_ne!(
        reloaded_document, previous_document,
        "the conversation must be reopened in a new document, not read from the old page"
    );
    browser.wait_for_text(message, Duration::from_secs(20));
    browser.click("button[aria-label=\"Open Today\"]");
    browser.wait_for_text("Nothing needs you right now.", Duration::from_secs(10));
    let address = browser.eval("[new URL(location.href).searchParams.get('patient'), new URL(location.href).searchParams.get('chat')]");
    assert_eq!(
        address,
        serde_json::json!([null, null]),
        "Today retained a patient conversation in its address: {address}"
    );
    browser.click("button[aria-label=\"Find patient\"]");
    browser.search_for("Jake");
    browser.wait_for_text("DOB", Duration::from_secs(10));
    browser.click(".dialog-results button");
    browser.wait_for_text("Records as of", Duration::from_secs(10));
    let address = browser.eval("[new URL(location.href).searchParams.get('patient'), new URL(location.href).searchParams.get('chat')]");
    assert_eq!(
        address,
        serde_json::json!(["pw-82c44e9b-58c0-4af1-bd21-771bf77e101d", null]),
        "changing patient retained the previous patient's conversation: {address}"
    );
    browser.reload();
    browser.wait_for_text("Records as of", Duration::from_secs(20));
}

/// Two tabs on one chat with debounced autosave: a tab whose save names an
/// older revision is fenced (nothing written, its text kept, autosave paused)
/// and offered Retry or Use newer text. Use newer text discards the edit;
/// Retry names exactly the revision shown, so a draft that moved again shows
/// another conflict instead of being overwritten. Typing during a long save
/// gets a trailing save, and a save still running when the physician opens
/// another chat never marks that chat.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore = "browser qualification; run explicitly with --ignored"]
async fn a_stale_tab_is_not_saved_and_a_save_never_marks_another_chat() {
    let _guard = test_lock().lock().await;
    let site = build_site();
    let server = serve_chat_browser(&site, scale::FIXED_TEXT).await;
    let address = server.address;
    let patient = "pw-39e51534-6ce4-43cf-b5f8-7c1ca6252d8e";
    let chat = "chat-draft-fence";
    let chat_url = format!("http://{address}/?patient={patient}&chat={chat}");
    let bearer = scale::sign_in(&address.to_string()).await;
    let http = reqwest::Client::new();
    let server_draft = || async {
        http.get(format!(
            "http://{address}/physician-state/drafts/{patient}/{chat}"
        ))
        .bearer_auth(&bearer)
        .send()
        .await
        .unwrap()
        .json::<serde_json::Value>()
        .await
        .unwrap()
    };
    let mut tab_a = Tab::open(&chat_url);
    let mut tab_b = Tab::open(&chat_url);
    for tab in [&mut tab_a, &mut tab_b] {
        tab.wait_for_text("Work email", Duration::from_secs(20));
        tab.request_sign_in("physician@codahealth.test");
        tab.wait_for_text(
            "What would you like Coda to review?",
            Duration::from_secs(20),
        );
    }

    // Both tabs read the empty draft (revision 0). A's autosave lands first.
    tab_a.type_composer("Older text from tab A");
    tab_a.wait_saved();
    assert_eq!(
        server_draft().await,
        serde_json::json!({"revision": 1, "text": "Older text from tab A"})
    );
    // B still names revision 0: its autosave is refused, nothing is written,
    // B keeps its text and is shown the newer draft with both choices.
    tab_b.type_composer("Newer text from tab B");
    let conflict = tab_b.wait_conflict();
    assert_eq!(
        conflict,
        (1, "Older text from tab A".to_owned()),
        "the stale tab showed no conflict naming the newer draft"
    );
    assert_eq!(
        server_draft().await,
        serde_json::json!({"revision": 1, "text": "Older text from tab A"}),
        "a refused save wrote the stale text"
    );
    assert_eq!(
        tab_b.composer().0,
        "Newer text from tab B",
        "a refused save dropped the stale tab's text"
    );
    // Autosave stays paused while the choice waits: more typing writes nothing.
    tab_b.type_composer("Newer text from tab B, still mine");
    tokio::time::sleep(Duration::from_millis(1_500)).await;
    assert_eq!(server_draft().await["revision"], 1);

    // Use newer text discards B's edit and shows A's saved draft.
    tab_b.click_conflict("Use newer text");
    tab_b.wait_composer("Older text from tab A", Duration::from_secs(5));
    assert!(tab_b.conflict().is_none());
    tokio::time::sleep(Duration::from_millis(1_500)).await;
    assert_eq!(
        server_draft().await,
        serde_json::json!({"revision": 1, "text": "Older text from tab A"}),
        "Use newer text wrote to the server"
    );

    // B saves over revision 1; A, still at revision 1, now conflicts.
    tab_b.type_composer("Newer text from tab B");
    tab_b.wait_saved();
    tab_a.type_composer("Tab A insists");
    assert_eq!(
        tab_a.wait_conflict(),
        (2, "Newer text from tab B".to_owned())
    );
    // The draft moves again before A chooses: Retry names the revision A was
    // shown (2), so it is refused and the newest draft is shown instead.
    tab_b.type_composer("Tab B moved again");
    tab_b.wait_saved();
    assert_eq!(server_draft().await["revision"], 3);
    tab_a.click_conflict("Retry");
    let started = Instant::now();
    loop {
        if tab_a.conflict() == Some((3, "Tab B moved again".to_owned())) {
            break;
        }
        assert!(
            started.elapsed() < Duration::from_secs(10),
            "Retry over a draft that moved again showed no new conflict: {:?}",
            tab_a.conflict()
        );
        tokio::time::sleep(Duration::from_millis(60)).await;
    }
    assert_eq!(
        server_draft().await,
        serde_json::json!({"revision": 3, "text": "Tab B moved again"}),
        "Retry overwrote a revision the physician was never shown"
    );
    // Retry over the revision now shown replaces it with A's text.
    tab_a.click_conflict("Retry");
    tab_a.wait_saved();
    assert_eq!(
        server_draft().await,
        serde_json::json!({"revision": 4, "text": "Tab A insists"})
    );
    drop(tab_b);

    // Reopened, the chat shows its saved draft.
    tab_a.navigate(&chat_url);
    tab_a.wait_composer("Tab A insists", Duration::from_secs(20));

    // Typing during a long save still gets a trailing save of the newer text.
    tab_a.call(
        "Fetch.enable",
        serde_json::json!({ "patterns": [{ "urlPattern": "*/physician-state/drafts/*" }] }),
    );
    tab_a.type_composer("Edited during");
    let put = tab_a.held_request("PUT");
    assert_eq!(
        tab_a.composer().1,
        "",
        "in-flight draft saves must be silent"
    );
    tab_a.type_composer("Edited during a long save");
    tab_a.release(&put);
    let trailing = tab_a.held_request("PUT");
    let body = held_body(&trailing);
    assert_eq!(
        body,
        serde_json::json!({"expected_revision": 5, "text": "Edited during a long save"}),
        "the trailing save did not name the revision the first save produced"
    );
    tab_a.release(&trailing);
    let started = Instant::now();
    while server_draft().await["revision"] != 6 {
        assert!(
            started.elapsed() < Duration::from_secs(10),
            "the trailing save never landed"
        );
        tab_a.release_all();
        tokio::time::sleep(Duration::from_millis(30)).await;
    }
    assert_eq!(
        server_draft().await,
        serde_json::json!({"revision": 6, "text": "Edited during a long save"})
    );

    // Hold the next save in flight, open another chat, then let it land.
    tab_a.type_composer("Edited after reopening");
    let put = tab_a.held_request("PUT");
    assert_eq!(
        tab_a.composer().1,
        "",
        "in-flight draft saves must be silent"
    );
    tab_a.click("button.sidebar-new-conversation");
    let started = Instant::now();
    while new_chat(&mut tab_a) == chat {
        assert!(
            started.elapsed() < Duration::from_secs(10),
            "New conversation did not open"
        );
        tokio::time::sleep(Duration::from_millis(30)).await;
    }
    assert_eq!(
        tab_a.composer(),
        (String::new(), String::new()),
        "the new chat showed the other chat's draft or its Saving mark"
    );
    tab_a.release(&put);
    // The save lands on the server for its own chat ...
    let started = Instant::now();
    while server_draft().await["revision"] != 7 {
        assert!(
            started.elapsed() < Duration::from_secs(10),
            "the held save never landed"
        );
        tab_a.release_all();
        tokio::time::sleep(Duration::from_millis(30)).await;
    }
    assert_eq!(
        server_draft().await,
        serde_json::json!({"revision": 7, "text": "Edited after reopening"})
    );
    // ... and its answer never marks the chat now shown.
    for _ in 0..10 {
        tab_a.release_all();
        assert_eq!(
            tab_a.composer(),
            (String::new(), String::new()),
            "another chat's save marked this chat"
        );
        tokio::time::sleep(Duration::from_millis(50)).await;
    }
    tab_a.call("Fetch.disable", serde_json::json!({}));
}

/// Same-tab recovery through `sessionStorage`, signed in (a page restores
/// only under a known physician): text typed just before a reload, text
/// whose save was still on its way, and text whose save landed but whose
/// answer was lost all come back without a duplicate write; a stale local
/// baseline asks for an explicit choice; sign-out forgets every record; and
/// denied storage keeps the text in memory and says reload recovery is off.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore = "browser qualification; run explicitly with --ignored"]
async fn a_reloaded_tab_recovers_unsent_text_and_sign_out_forgets_it() {
    let _guard = test_lock().lock().await;
    let site = build_site();
    let server = serve_chat_browser(&site, scale::FIXED_TEXT).await;
    let address = server.address;
    let patient = "pw-39e51534-6ce4-43cf-b5f8-7c1ca6252d8e";
    let chat = "chat-recovery";
    let chat_url = format!("http://{address}/?patient={patient}&chat={chat}");
    let mut tab = Tab::open(&format!("http://{address}/"));
    tab.wait_for_text("Work email", Duration::from_secs(20));
    tab.request_sign_in("physician@codahealth.test");
    tab.wait_for_text("Nothing needs you right now.", Duration::from_secs(20));
    let bearer = tab
        .eval("localStorage.getItem('coda.physician.session')")
        .as_str()
        .expect("signed-in bearer session")
        .to_owned();
    let http = reqwest::Client::new();
    let draft_url = format!("http://{address}/physician-state/drafts/{patient}/{chat}");
    let server_draft = || async {
        http.get(&draft_url)
            .bearer_auth(&bearer)
            .send()
            .await
            .unwrap()
            .json::<serde_json::Value>()
            .await
            .unwrap()
    };
    tab.navigate(&chat_url);
    tab.wait_for_text(
        "What would you like Coda to review?",
        Duration::from_secs(20),
    );

    // 1. Reloaded before the debounce: the text is only in this tab's
    // storage, comes back, and then saves.
    tab.type_composer("Typed just before the reload");
    assert_eq!(server_draft().await["revision"], 0);
    assert_eq!(
        tab.recovery_keys().len(),
        1,
        "the unsent text was not kept for a reload"
    );
    tab.reload();
    tab.wait_composer("Typed just before the reload", Duration::from_secs(20));
    tab.wait_saved();
    assert_eq!(
        server_draft().await,
        serde_json::json!({"revision": 1, "text": "Typed just before the reload"})
    );

    // 2. Reloaded while the save was held before the server: it comes back
    // and saves over the revision this tab last saw.
    tab.call(
        "Fetch.enable",
        serde_json::json!({ "patterns": [{ "urlPattern": "*/physician-state/drafts/*" }] }),
    );
    tab.type_composer("Typed while a save was on its way");
    let _held = tab.held_request("PUT");
    tab.reload();
    tab.release_all();
    tab.call("Fetch.disable", serde_json::json!({}));
    tab.release_all();
    tab.wait_composer("Typed while a save was on its way", Duration::from_secs(20));
    tab.wait_saved();
    assert_eq!(
        server_draft().await,
        serde_json::json!({"revision": 2, "text": "Typed while a save was on its way"})
    );

    // 3. The save lands but its answer is lost: after the reload the
    // server's matching text is taken as the baseline; no conflict, no
    // second write.
    tab.call(
        "Fetch.enable",
        serde_json::json!({ "patterns": [{
            "urlPattern": "*/physician-state/drafts/*",
            "requestStage": "Response"
        }] }),
    );
    tab.type_composer("Saved but never answered");
    let answer = tab.held_request("PUT");
    assert_eq!(
        server_draft().await,
        serde_json::json!({"revision": 3, "text": "Saved but never answered"})
    );
    tab.call(
        "Fetch.failRequest",
        serde_json::json!({ "requestId": answer["requestId"], "errorReason": "Failed" }),
    );
    tab.wait_not_saved();
    assert_eq!(tab.composer().0, "Saved but never answered");
    tab.wait_for_text("Retry", Duration::from_secs(5));
    tab.save_pictures("draft-not-saved", ".composer-context");
    tab.call("Fetch.disable", serde_json::json!({}));
    tab.release_all();
    tab.click_named(".composer-context button", "Retry");
    tab.wait_saved();
    tab.reload();
    tab.wait_composer("Saved but never answered", Duration::from_secs(20));
    tokio::time::sleep(Duration::from_millis(2_500)).await;
    assert!(
        tab.conflict().is_none(),
        "this tab's own lost save showed as a conflict"
    );
    assert_eq!(
        server_draft().await,
        serde_json::json!({"revision": 3, "text": "Saved but never answered"}),
        "the recovered text was written twice"
    );
    assert!(
        tab.recovery_keys().is_empty(),
        "a record outlived the text it kept"
    );

    // 4. Another writer moved the draft before this tab's text was saved:
    // after the reload the physician chooses; nothing is overwritten.
    tab.call(
        "Fetch.enable",
        serde_json::json!({ "patterns": [{ "urlPattern": "*/physician-state/drafts/*" }] }),
    );
    tab.type_composer("Local edit nobody saved");
    let _held = tab.held_request("PUT");
    let moved = http
        .put(&draft_url)
        .bearer_auth(&bearer)
        .json(&serde_json::json!({"expected_revision": 3, "text": "Written elsewhere"}))
        .send()
        .await
        .unwrap();
    assert!(moved.status().is_success());
    tab.reload();
    tab.call("Fetch.disable", serde_json::json!({}));
    tab.release_all();
    assert_eq!(tab.wait_conflict(), (4, "Written elsewhere".to_owned()));
    assert_eq!(tab.composer().0, "Local edit nobody saved");
    assert_eq!(server_draft().await["revision"], 4);
    tab.click_conflict("Use newer text");
    tab.wait_composer("Written elsewhere", Duration::from_secs(5));
    assert!(tab.recovery_keys().is_empty());

    // 5. Sign-out forgets every record before anything else can read it.
    tab.type_composer("Unsent at sign-out");
    assert_eq!(tab.recovery_keys().len(), 1);
    tab.click("button[aria-label=\"Sign out of Coda\"]");
    tab.wait_for_text("Sign in to Coda", Duration::from_secs(10));
    assert!(
        tab.recovery_keys().is_empty(),
        "sign-out left unsent text in this tab's storage"
    );
    drop(tab);

    // 6. Denied storage: the text stays on screen and saves; the page says
    // a reload cannot bring it back.
    let mut denied = Tab::open("about:blank");
    denied.call(
        "Page.addScriptToEvaluateOnNewDocument",
        serde_json::json!({"source": include_str!("js/deny_draft_storage.js")}),
    );
    denied.navigate(&format!("http://{address}/"));
    denied.wait_for_text("Work email", Duration::from_secs(20));
    denied.request_sign_in("physician@codahealth.test");
    denied.wait_for_text("Nothing needs you right now.", Duration::from_secs(20));
    denied.navigate(&chat_url);
    denied.wait_composer("Written elsewhere", Duration::from_secs(20));
    denied.type_composer("Kept only in memory");
    denied.wait_for_text(
        "This tab can’t keep a reload copy of your draft.",
        Duration::from_secs(5),
    );
    assert_eq!(denied.composer().0, "Kept only in memory");
    denied.wait_saved();
    drop(denied);

    // 7. Same-clinic panel removal answers 200 with demographics, not 404.
    // Recovery must be purged even on this successful workspace read.
    let mut removed = Tab::open(&format!("http://{address}/"));
    removed.wait_for_text("Work email", Duration::from_secs(20));
    removed.request_sign_in("physician@codahealth.test");
    removed.wait_for_text("Nothing needs you right now.", Duration::from_secs(20));
    removed.navigate(&chat_url);
    removed.wait_composer("Kept only in memory", Duration::from_secs(20));
    removed.type_composer("Must not survive panel removal");
    assert_eq!(removed.recovery_keys().len(), 1);
    let removed_bearer = removed
        .eval("localStorage.getItem('coda.physician.session')")
        .as_str()
        .unwrap()
        .to_owned();
    let revoked = http
        .delete(format!(
            "http://{address}/auth/panels/staging-physician/{patient}"
        ))
        .bearer_auth(&removed_bearer)
        .send()
        .await
        .unwrap();
    assert!(revoked.status().is_success());
    removed.reload();
    removed.wait_for_text("Clinic record · Not on your panel", Duration::from_secs(20));
    assert!(
        removed.recovery_keys().is_empty(),
        "demographic-only access retained clinical recovery"
    );
}

/// Send from the keyboard (Ctrl+Enter) with text typed while the command is
/// still unanswered: acceptance removes only the submitted message, keeps
/// the later text, and the durable draft follows. A Send whose answer is lost
/// across a reload is asked about again with the same command key, never a
/// new one, and the message appears once.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore = "browser qualification; run explicitly with --ignored"]
async fn keyboard_send_clears_only_the_sent_text_and_a_reload_reasks_the_same_command() {
    let _guard = test_lock().lock().await;
    let site = build_site();
    let server = serve_chat_browser(&site, scale::FIXED_TEXT).await;
    let address = server.address;
    let patient = "pw-39e51534-6ce4-43cf-b5f8-7c1ca6252d8e";
    let chat = "chat-keyboard-send";
    let chat_url = format!("http://{address}/?patient={patient}&chat={chat}");
    let mut tab = Tab::open(&format!("http://{address}/"));
    tab.wait_for_text("Work email", Duration::from_secs(20));
    tab.request_sign_in("physician@codahealth.test");
    tab.wait_for_text("Nothing needs you right now.", Duration::from_secs(20));
    let bearer = tab
        .eval("localStorage.getItem('coda.physician.session')")
        .as_str()
        .expect("signed-in bearer session")
        .to_owned();
    let http = reqwest::Client::new();
    let draft_url = format!("http://{address}/physician-state/drafts/{patient}/{chat}");
    let server_draft = || async {
        http.get(&draft_url)
            .bearer_auth(&bearer)
            .send()
            .await
            .unwrap()
            .json::<serde_json::Value>()
            .await
            .unwrap()
    };
    tab.navigate(&chat_url);
    tab.wait_for_text(
        "What would you like Coda to review?",
        Duration::from_secs(20),
    );

    // 1. Typed, autosaved, then sent from the keyboard while the command is
    // held; more typing lands before the answer.
    let first = "Check the potassium trend";
    tab.type_composer(first);
    tab.wait_saved();
    assert_eq!(server_draft().await["text"], first);
    tab.call(
        "Fetch.enable",
        serde_json::json!({ "patterns": [{ "urlPattern": "*/commands" }] }),
    );
    tab.step("sendWithKeyboard", serde_json::json!([]));
    let held = tab.held_request("POST");
    let key = held_body(&held)["idempotency_key"].clone();
    assert_eq!(
        held_body(&held)["payload"]["text"],
        first,
        "keyboard Send did not submit the composer text"
    );
    tab.type_composer(&format!("{first} and the sodium"));
    tab.release(&held);
    tab.call("Fetch.disable", serde_json::json!({}));
    tab.wait_composer("and the sodium", Duration::from_secs(10));
    tab.wait_for_text(first, Duration::from_secs(10));
    tab.wait_saved();
    assert_eq!(
        server_draft().await["text"],
        "and the sodium",
        "the durable draft still held the sent message"
    );

    tab.type_composer("Keep this rejected message");
    tab.wait_saved();
    tab.call(
        "Fetch.enable",
        serde_json::json!({ "patterns": [{ "urlPattern": "*/commands" }] }),
    );
    tab.click_named(".composer-toolbar button", "Send");
    let rejected = tab.held_request("POST");
    tab.fulfill_json(&rejected, r#"{"outcome":"rejected"}"#);
    tab.call("Fetch.disable", serde_json::json!({}));
    tab.wait_for_text("Coda didn’t take that.", Duration::from_secs(10));
    assert_eq!(tab.composer().0, "Keep this rejected message");
    assert_eq!(server_draft().await["text"], "Keep this rejected message");
    tab.save_pictures("draft-failed-send", ".composer-context");

    // 2. Sent, but the answer is lost and the page reloads: the reload asks
    // again with the same key; the message lands once and clears.
    let second = "Order a repeat BMP";
    tab.type_composer(second);
    tab.wait_saved();
    tab.call(
        "Fetch.enable",
        serde_json::json!({ "patterns": [{
            "urlPattern": "*/commands",
            "requestStage": "Response"
        }] }),
    );
    tab.click_named(".composer-toolbar button", "Send");
    let answered = tab.held_request("POST");
    let lost_key = held_body(&answered)["idempotency_key"].clone();
    assert_ne!(lost_key, key);
    tab.call(
        "Fetch.failRequest",
        serde_json::json!({ "requestId": answered["requestId"], "errorReason": "Failed" }),
    );
    tab.call(
        "Fetch.enable",
        serde_json::json!({ "patterns": [{ "urlPattern": "*/commands" }] }),
    );
    tab.reload();
    let reasked = tab.held_request("POST");
    assert_eq!(
        held_body(&reasked)["idempotency_key"],
        lost_key,
        "the reload issued a new command for a Send whose answer was lost"
    );
    tab.release(&reasked);
    tab.call("Fetch.disable", serde_json::json!({}));
    tab.release_all();
    tab.wait_for_text(second, Duration::from_secs(20));
    tab.wait_composer("", Duration::from_secs(10));
    let lines = tab.step("conversationLines", serde_json::json!([]));
    let sent = lines
        .as_array()
        .into_iter()
        .flatten()
        .filter(|line| line.as_str().is_some_and(|text| text.contains(second)))
        .count();
    assert_eq!(sent, 1, "the re-asked message did not appear exactly once");
    let started = Instant::now();
    while server_draft().await["text"] != "" {
        assert!(
            started.elapsed() < Duration::from_secs(10),
            "the durable draft kept the accepted message: {}",
            server_draft().await
        );
        tokio::time::sleep(Duration::from_millis(100)).await;
    }
}

#[derive(Clone, Copy, Debug)]
enum Theme {
    Light,
    Dark,
}

impl Theme {
    const fn attribute(self) -> &'static str {
        match self {
            Self::Light => "light",
            Self::Dark => "dark",
        }
    }
}

/// A window the sidebar has to fit, and how many recent conversations it lists.
#[derive(Clone, Copy, Debug)]
struct SidebarCase {
    name: &'static str,
    width: u32,
    height: u32,
    recents: Recents,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Recents {
    /// The sample's empty list.
    None,
    /// More rows than any window can show, so the list has to give up space.
    Overflowing,
}

const OVERFLOWING_RECENTS: usize = 60;

const SIDEBAR_CASES: [SidebarCase; 3] = [
    SidebarCase {
        name: "desktop",
        width: 1440,
        height: 900,
        recents: Recents::None,
    },
    SidebarCase {
        name: "phone",
        width: 390,
        height: 844,
        recents: Recents::None,
    },
    SidebarCase {
        name: "short",
        width: 1440,
        height: 600,
        recents: Recents::Overflowing,
    },
];

/// An element's edges in the window.
#[derive(Debug, serde::Deserialize)]
struct Edges {
    left: f64,
    right: f64,
    top: f64,
    bottom: f64,
}

/// What `sidebar_layout.js` reads from the page.
#[derive(Debug, serde::Deserialize)]
#[expect(
    clippy::struct_excessive_bools,
    reason = "each bool is one independent measurement the page reports"
)]
struct SidebarLayout {
    window_width: f64,
    window_height: f64,
    sign_out: Edges,
    sign_out_visible: bool,
    recent_rows: usize,
    recent_visible_height: f64,
    recent_scrolls_down: bool,
    recent_scrolls_sideways: bool,
}

impl SidebarLayout {
    fn sign_out_on_screen(&self) -> bool {
        let Edges {
            left,
            right,
            top,
            bottom,
        } = self.sign_out;
        self.sign_out_visible
            && left >= 0.0
            && top >= 0.0
            && left < right
            && top < bottom
            && right <= self.window_width
            && bottom <= self.window_height
    }

    /// Down, and never sideways.
    fn recent_list_scrolls(&self) -> bool {
        self.recent_visible_height > 0.0
            && self.recent_scrolls_down
            && !self.recent_scrolls_sideways
    }
}

/// The recent list's response, read back through the type the app reads it with.
fn overflowing_recents() -> String {
    let conversations: Vec<_> = (1..=OVERFLOWING_RECENTS)
        .map(|n| {
            serde_json::json!({
                "patient_workspace_id": format!("pw-synthetic-{n}"),
                "title": format!("Synthetic conversation {n}"),
                "patient_display_name": format!("Synthetic patient {n}"),
            })
        })
        .collect();
    let body = serde_json::json!({ "conversations": conversations }).to_string();
    let response: physician_wire::RecentConversationsResponse =
        serde_json::from_str(&body).expect("the wire type reads the recent list");
    assert_eq!(response.conversations.len(), OVERFLOWING_RECENTS);
    body
}

/// Sign out stays on screen, and a long recent list scrolls instead of pushing it off.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore = "browser qualification; run explicitly with --ignored"]
async fn sign_out_stays_on_screen_and_a_long_recent_list_scrolls() {
    let _guard = test_lock().lock().await;
    let site = build_site();
    let server = common::serve_isolated(|| {
        // SAFETY: tests in this binary hold `test_lock()` before mutating
        // process env, and `serve_isolated` holds its start lock here.
        unsafe {
            env::set_var("PHYSICIAN_SITE_DIR", &site);
            env::remove_var("PHYSICIAN_AUTH");
            env::remove_var("PHYSICIAN_AUTH_PROOF_TOKENS");
            env::remove_var("PHYSICIAN_UPSTREAM");
        }
    })
    .await;
    let url = format!("http://{}/", server.address);

    let mut cut_off = Vec::new();
    for case in SIDEBAR_CASES {
        let mut browser = Tab::open("about:blank");
        browser.set_viewport(case.width, case.height, true);
        match case.recents {
            Recents::None => {
                browser.navigate(&url);
                browser.wait_for_recent_rows(0, None);
            }
            Recents::Overflowing => {
                browser
                    .call(
                        "Fetch.enable",
                        serde_json::json!({ "patterns": [{ "urlPattern": "*/recent-conversations*" }] }),
                    );
                browser.navigate(&url);
                let recents = overflowing_recents();
                browser.wait_for_recent_rows(OVERFLOWING_RECENTS, Some(&recents));
            }
        }
        // Below 900px the sidebar is a drawer, and Sign out is in it once it is open.
        if case.width < 900 {
            open_the_drawer(&mut browser);
        }
        for theme in [Theme::Light, Theme::Dark] {
            let layout = browser
                .sidebar_layout(theme)
                .expect("the sidebar stays rendered");
            browser.save_picture(&format!("{}-{}", case.name, theme.attribute()));
            if !layout.sign_out_on_screen() {
                cut_off.push((case.name, theme, layout));
                continue;
            }
            if case.recents == Recents::Overflowing {
                assert!(
                    layout.recent_list_scrolls(),
                    "the recent list does not scroll at {case:?}: {layout:?}"
                );
            }
        }
    }
    assert!(cut_off.is_empty(), "Sign out is cut off: {cut_off:#?}");
}

fn new_chat(browser: &mut Tab) -> String {
    browser
        .eval("new URL(location.href).searchParams.get('chat') ?? ''")
        .as_str()
        .unwrap_or_default()
        .to_owned()
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore = "browser qualification; run explicitly with --ignored"]
async fn the_first_event_connection_reads_changes_after_the_initial_snapshot() {
    let _guard = test_lock().lock().await;
    let site = build_site();
    let server = serve_chat_browser(&site, scale::FIXED_TEXT).await;
    let base = format!("http://{}", server.address);
    let patient = "pw-39e51534-6ce4-43cf-b5f8-7c1ca6252d8e";
    let chat = "chat-before-events";
    let mut browser = Tab::open(&base);
    browser.wait_for_text("Work email", Duration::from_secs(20));
    browser.request_sign_in("physician@codahealth.test");
    browser.wait_for_text("Nothing needs you right now.", Duration::from_secs(20));
    let bearer = browser
        .eval("localStorage.getItem('coda.physician.session')")
        .as_str()
        .unwrap()
        .to_owned();
    // Hold the actual first event request before it reaches the server. The
    // initial snapshot can complete, then another client saves a turn while
    // this page has no subscription. No synthetic event or refresh is sent.
    browser.call(
        "Page.addScriptToEvaluateOnNewDocument",
        serde_json::json!({"source": include_str!("js/hold_first_event_request.js")}),
    );
    browser.navigate(&format!("{base}/?patient={patient}&chat={chat}"));
    browser.wait_for_text(
        "What would you like Coda to review?",
        Duration::from_secs(20),
    );
    let message = "Review the update accepted before this page subscribed";
    let client = reqwest::Client::new();
    let accepted: serde_json::Value = client.post(format!("{base}/commands")).bearer_auth(&bearer)
        .json(&serde_json::json!({"type":"message.submit","idempotency_key":"before-events",
            "payload":{"patient_workspace_id":patient,"chat_id":chat,"text":message,"physician_time_zone":"UTC"}}))
        .send().await.unwrap().json().await.unwrap();
    assert_eq!(accepted["outcome"], "accepted", "{accepted}");
    let snapshot: serde_json::Value = client
        .get(format!(
            "{base}/patient-workspaces/{patient}/chats/{chat}/snapshot"
        ))
        .bearer_auth(&bearer)
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();
    assert!(
        scale::chat_texts(&snapshot["snapshot"])
            .iter()
            .any(|text| text == message),
        "{snapshot}"
    );
    assert!(
        !browser.dom().contains(message),
        "the page must still hold the earlier snapshot"
    );
    let connected = browser.eval("window.__connectEvents()");
    assert_eq!(connected, serde_json::json!(true));
    browser.wait_for_text(message, Duration::from_secs(10));
    let count = browser.eval("window.__eventRequests");
    assert_eq!(
        count,
        serde_json::json!(1),
        "the first connection must reconcile without waiting for reconnect"
    );
}

/// The change stream of a signed-in page that is not a chat, quiet and then a connection that
/// dies without closing. Where nothing changes for longer than 30 seconds, Today keeps its first
/// change stream: the keep-alive comments pass the sign-in guard and reach it. Then the page's
/// change stream stops handing the page any bytes, keep-alives too, and stays open. Today reads
/// on no timer, so no Today read leaves the page meanwhile. Once nothing has reached it for 30
/// seconds, the page gives that stream up, reads Today again and connects again.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore = "browser and disposable Postgres qualification; run explicitly with --ignored"]
async fn a_quiet_page_keeps_its_change_stream_and_a_silent_one_is_dropped_and_read_again() {
    let _guard = test_lock().lock().await;
    let site = build_site();
    let server = serve_chat_browser(&site, scale::FIXED_TEXT).await;
    let base = format!("http://{}", server.address);
    let mut tab = Tab::open(&base);
    tab.wait_for_text("Work email", Duration::from_secs(20));
    tab.request_sign_in("physician@codahealth.test");
    tab.wait_for_text("Nothing needs you right now.", Duration::from_secs(20));
    tab.call(
        "Page.addScriptToEvaluateOnNewDocument",
        serde_json::json!({"source": include_str!("js/silence_event_stream.js")}),
    );
    tab.reload();
    tab.wait_for_text("Nothing needs you right now.", Duration::from_secs(20));

    // Quiet past the page's 30-second silence limit by one server keep-alive interval (the
    // server's own `live::KEEP_ALIVE_EVERY`, 15 seconds): a page that heard no keep-alive would
    // have given its stream up.
    tokio::time::sleep(Duration::from_secs(30 + 15)).await;
    let quiet = tab.step("eventStreamHeard", serde_json::json!([]));
    assert!(
        quiet["streams"] == 1
            && quiet["sinceBytes"].as_f64().is_some_and(|ms| ms < 30_000.0)
            && quiet["lastChunk"]
                .as_str()
                .is_some_and(|chunk| chunk.starts_with(':')),
        "a quiet signed-in page lost its change stream or heard no keep-alive: {quiet}"
    );

    // From here on the browser holds each Today read the page starts before it leaves for the
    // server.
    tab.call(
        "Fetch.enable",
        serde_json::json!({ "patterns": [{ "urlPattern": "*/today" }] }),
    );
    assert_eq!(
        tab.eval("window.__silenceEvents()"),
        serde_json::json!(1),
        "the page holds more than one change stream"
    );
    // The page gives the stream up 30 seconds after its last bytes at the latest, and those came
    // before the silence.
    let silenced = Instant::now();
    let stream = loop {
        let reads = tab.browser.take_paused_requests();
        let stream = tab.step("silencedEventStream", serde_json::json!([]));
        if !stream["droppedAt"].is_null() {
            tab.held.extend(reads);
            break stream;
        }
        assert!(
            reads.is_empty(),
            "the page read Today again before it gave the silent stream up: {reads:?}"
        );
        assert!(
            silenced.elapsed() < Duration::from_secs(30 + 15),
            "the page never gave the silent stream up: {stream}"
        );
        std::thread::sleep(Duration::from_millis(100));
    };
    let at = |name: &str| stream[name].as_f64().unwrap_or(f64::NAN);
    assert!(
        at("droppedAt") - at("lastBytes") >= 30_000.0,
        "the page gave the stream up before 30 seconds without bytes: {stream}"
    );
    eprintln!(
        "the stream was silent {:.1} s before the page gave it up",
        (at("droppedAt") - at("silencedAt")) / 1000.0
    );

    let read = tab.held_request("GET");
    assert!(
        read["request"]["url"]
            .as_str()
            .is_some_and(|url| url.ends_with("/today")),
        "the page did not read Today again after it gave the stream up: {read}"
    );
    tab.release(&read);
    let reconnected = Instant::now();
    while tab.step("eventStreams", serde_json::json!([])) != serde_json::json!(2) {
        assert!(
            reconnected.elapsed() < Duration::from_secs(20),
            "the page never connected again"
        );
        std::thread::sleep(Duration::from_millis(100));
    }
    tab.release_all();
    tab.call("Fetch.disable", serde_json::json!({}));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore = "browser qualification; run explicitly with --ignored"]
async fn the_same_bundle_runs_under_a_stripped_physician_prefix_and_reconnects() {
    use std::sync::atomic::{AtomicUsize, Ordering};

    let _guard = test_lock().lock().await;
    let site = build_site();
    // SAFETY: all site tests hold test_lock while changing environment.
    unsafe {
        env::set_var("PHYSICIAN_SITE_DIR", &site);
    }
    let streams = Arc::new(AtomicUsize::new(0));
    let requests = Arc::new(std::sync::Mutex::new(Vec::<String>::new()));
    let seen = requests.clone();
    let stream_count = streams.clone();
    let app = physician_sample::fixture_site().layer(axum::middleware::from_fn(
        move |request: axum::extract::Request, next: axum::middleware::Next| {
            let streams = stream_count.clone();
            async move {
                use axum::response::IntoResponse;
                if request.uri().path() != "/patient-workspaces/events" {
                    return next.run(request).await;
                }
                let attempt = streams.fetch_add(1, Ordering::SeqCst);
                // Two finite Cloud Run responses, then an authority refusal.
                // The browser must reconnect EOF but never retry the refusal.
                let status = if attempt >= 2 {
                    axum::http::StatusCode::FORBIDDEN
                } else {
                    axum::http::StatusCode::OK
                };
                (status, [("content-type", "text/event-stream")], ": end\n\n").into_response()
            }
        },
    ));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let upstream_address = listener.local_addr().unwrap();
    let upstream = tokio::spawn(async move { axum::serve(listener, app).await.unwrap() });
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    let app = public_proxy(upstream_address, address)
        // Override-only fixture; /physician itself is the real public-site transport.
        .nest("/override", physician_sample::fixture_site())
        .layer(axum::middleware::from_fn(
            move |request: axum::extract::Request, next: axum::middleware::Next| {
                let seen = seen.clone();
                async move {
                    seen.lock().unwrap().push(request.uri().path().to_owned());
                    next.run(request).await
                }
            },
        ));
    let server = tokio::spawn(async move { axum::serve(listener, app).await.unwrap() });
    let mut browser = Tab::open(&format!("http://{address}/physician/"));
    browser.wait_for_text("Nothing needs you right now.", Duration::from_secs(15));
    let assets = browser.step("prefixAssetsLoad", serde_json::json!([]));
    assert_eq!(
        assets,
        serde_json::json!(true),
        "prefix assets must load: {assets}"
    );
    let deadline = Instant::now() + Duration::from_secs(10);
    while streams.load(Ordering::SeqCst) < 3 {
        assert!(Instant::now() < deadline, "finite SSE did not reconnect");
        tokio::time::sleep(Duration::from_millis(50)).await;
    }
    tokio::time::sleep(Duration::from_secs(5)).await;
    assert_eq!(
        streams.load(Ordering::SeqCst),
        3,
        "authorization refusal must stop retries"
    );
    let paths = requests.lock().unwrap().clone();
    assert!(paths.iter().any(|path| path == "/physician/today"));
    assert!(
        paths.iter().all(|path| path.starts_with("/physician/")),
        "escaped mount: {paths:?}"
    );

    // Explicit query API still wins over the page prefix, with the same wasm.
    browser.navigate(&format!(
        "http://{address}/physician/?api=http%3A%2F%2F{address}%2Foverride"
    ));
    browser.wait_for_text("Nothing needs you right now.", Duration::from_secs(15));
    let deadline = Instant::now() + Duration::from_secs(10);
    while !requests
        .lock()
        .unwrap()
        .iter()
        .any(|path| path == "/override/today")
    {
        assert!(Instant::now() < deadline, "query override was not used");
        tokio::time::sleep(Duration::from_millis(50)).await;
    }
    server.abort();
    upstream.abort();
}

fn public_proxy(upstream: std::net::SocketAddr, public: std::net::SocketAddr) -> axum::Router {
    let runtime = public_site::runtime::RuntimeConfig {
        environment: "local".into(),
        physician_proxy: Some(
            public_site::proxy::ProxyConfig::new(
                &format!("http://{upstream}"),
                &format!("http://{public}"),
                None,
                false,
                false,
            )
            .unwrap(),
        ),
        ..Default::default()
    };
    public_site::server::router_with_options(
        public_site::prerender(&public_site::Config {
            listen_addr: public,
            physician_sign_in: true,
            ..Default::default()
        })
        .unwrap(),
        runtime,
        [],
        public_site::server::Readiness::ready(),
    )
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore = "browser qualification; run explicitly with --ignored"]
async fn mailed_sign_in_link_round_trips_through_the_public_mount() {
    use base64::Engine;

    fn mail_body(raw: &str, mime: &str) -> String {
        let bytes = base64::engine::general_purpose::URL_SAFE_NO_PAD
            .decode(raw)
            .unwrap();
        let message = mailparse::parse_mail(&bytes).expect("valid MIME email");
        message
            .subparts
            .iter()
            .find(|part| part.ctype.mimetype == mime)
            .expect("MIME alternative")
            .get_body()
            .expect("decoded text")
    }

    let _guard = test_lock().lock().await;
    let site = build_site();
    let fake = gmail::testing::spawn(false).await;
    let public_listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = public_listener.local_addr().unwrap();
    let endpoints = fake.endpoints();
    let mail_env = [
        ("PHYSICIAN_GMAIL_FROM", "signin@coda.test".to_owned()),
        (
            "PHYSICIAN_PUBLIC_ORIGIN",
            format!("http://{address}/physician"),
        ),
        ("PHYSICIAN_GMAIL_CLIENT_ID", "client".to_owned()),
        ("PHYSICIAN_GMAIL_CLIENT_SECRET", "secret".to_owned()),
        ("PHYSICIAN_GMAIL_REFRESH_TOKEN", "refresh".to_owned()),
        ("PHYSICIAN_GMAIL_SEND_URL", endpoints.send),
        ("PHYSICIAN_GMAIL_TOKEN_URL", endpoints.token),
    ];
    unsafe {
        env::set_var("PHYSICIAN_SITE_DIR", &site);
        env::set_var("PHYSICIAN_AUTH", "required");
        env::remove_var("PHYSICIAN_AUTH_PROOF_TOKENS");
        env::remove_var("PHYSICIAN_UPSTREAM");
        env::remove_var("PHYSICIAN_DATABASE_URL");
        for (key, value) in &mail_env {
            env::set_var(key, value);
        }
    }
    let email = "physician@codahealth.test";
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let upstream_address = listener.local_addr().unwrap();
    let app = physician_sample::authed_router(physician_sample::fixture_site()).await;
    let upstream = tokio::spawn(async move { axum::serve(listener, app).await.unwrap() });
    wait_until_ready(upstream_address).await;
    let proxy = tokio::spawn(async move {
        axum::serve(public_listener, public_proxy(upstream_address, address))
            .await
            .unwrap()
    });

    let mut browser = Tab::open(&format!("http://{address}/physician/"));
    browser.wait_for_text("Work email", Duration::from_secs(15));
    browser.request_sign_in(email);
    let waiting = browser.wait_for_text("Check your email", Duration::from_secs(10));
    assert!(
        !waiting.contains("Sign-in code") && !waiting.contains("codas-"),
        "requesting an email must not show a code field or expose the token"
    );
    let html = {
        let sent = fake.sent.lock().unwrap();
        assert_eq!(sent.len(), 1);
        mail_body(sent[0]["raw"].as_str().unwrap(), "text/html")
    };
    // Render the actual delivered HTML and click its button. Nothing in the
    // test submits the token or clicks a second confirmation in the app.
    browser.navigate(&format!(
        "data:text/html;base64,{}",
        base64::engine::general_purpose::STANDARD.encode(html)
    ));
    browser.wait_for_text("This link expires", Duration::from_secs(10));
    browser.click("a");
    let signed_in = browser.wait_for_text("Staging Physician", Duration::from_secs(15));
    assert!(
        !signed_in.contains("Sign-in code"),
        "the email link must sign in without a code field or another confirmation"
    );
    let location = browser.eval("location.href");
    assert_eq!(
        location.as_str(),
        Some(format!("http://{address}/physician/").as_str()),
        "the email and token must leave the address bar before redemption"
    );
    browser.reload();
    browser.wait_for_text("Staging Physician", Duration::from_secs(10));
    // A newly mailed link signs in once, after sign-out. (Opened while you are
    // signed in, your own link keeps your session; sign_in_model covers that.)
    browser.click("[aria-label=\"Sign out of Coda\"]");
    browser.wait_for_text("Sign in to Coda", Duration::from_secs(10));
    reqwest::Client::new()
        .post(format!("http://{address}/physician/auth/sign-in"))
        .header("Origin", format!("http://{address}"))
        .json(&serde_json::json!({"email": email}))
        .send()
        .await
        .unwrap();
    let next_message = {
        let sent = fake.sent.lock().unwrap();
        assert_eq!(sent.len(), 2);
        mail_body(sent[1]["raw"].as_str().unwrap(), "text/plain")
    };
    let next_link = next_message
        .lines()
        .find(|line| line.starts_with("http://"))
        .expect("second mailed link");
    browser.navigate(next_link);
    browser.wait_for_text("Staging Physician", Duration::from_secs(10));
    browser.click("[aria-label=\"Sign out of Coda\"]");
    browser.wait_for_text("Sign in to Coda", Duration::from_secs(10));

    // The same message cannot establish another session after its first use.
    browser.navigate(next_link);
    let invalid = browser.wait_for_text("That sign-in link", Duration::from_secs(10));
    assert!(invalid.contains("Send a new link") && !invalid.contains("Sign-in code"));
    assert_eq!(
        fake.sent.lock().unwrap().len(),
        2,
        "opening the link must not request another email"
    );
    proxy.abort();
    upstream.abort();
    unsafe {
        env::remove_var("PHYSICIAN_AUTH");
        for (key, _) in mail_env {
            env::remove_var(key);
        }
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore = "browser qualification; run explicitly with --ignored"]
async fn authenticated_practice_isolation_clinic_demographics_and_panel_assignment() {
    let _guard = test_lock().lock().await;
    let site = build_site();
    // SAFETY: tests in this binary hold `test_lock()` before mutating process env.
    unsafe {
        env::set_var("PHYSICIAN_SITE_DIR", &site);
        env::set_var("PHYSICIAN_AUTH", "required");
        env::set_var("PHYSICIAN_AUTH_PROOF_TOKENS", "1");
        env::remove_var("PHYSICIAN_UPSTREAM");
        env::remove_var("CODA_LOCAL_MODEL_EXECUTION");
        env::remove_var("CODA_ALLOW_MODEL_CALLS");
    }

    let mary = "pw-39e51534-6ce4-43cf-b5f8-7c1ca6252d8e";
    let jake = "pw-82c44e9b-58c0-4af1-bd21-771bf77e101d";

    let store = Arc::new(MemoryAuthStore::default());
    store.seed_account(
        "practice-clinic-a",
        "phys-admin",
        "admin@clinic-a.test",
        "Dr. Clinic Admin",
        true,
    );
    store.seed_account(
        "practice-clinic-a",
        "phys-colleague",
        "colleague@clinic-a.test",
        "Dr. Colleague",
        false,
    );
    // Mary belongs to practice-clinic-a (initially on Dr. Colleague's panel, not Dr. Clinic Admin's).
    store
        .register_patient("practice-clinic-a", mary)
        .await
        .unwrap();
    let admin_identity = PhysicianIdentity {
        practice_id: "practice-clinic-a".to_owned(),
        physician_id: "phys-admin".to_owned(),
    };
    store
        .assign_panel(&admin_identity, "phys-colleague", mary, Membership::Allowed)
        .await
        .unwrap();
    // Jake belongs to a completely different practice (practice-clinic-b).
    store
        .register_patient("practice-clinic-b", jake)
        .await
        .unwrap();

    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let upstream_address = listener.local_addr().unwrap();
    let server_store: Arc<dyn AuthStore> = store.clone();
    let upstream =
        tokio::spawn(
            async move { physician_sample::serve_with_store(listener, server_store).await },
        );
    wait_until_ready(upstream_address).await;
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    let app = public_proxy(upstream_address, address);
    let proxy = tokio::spawn(async move { axum::serve(listener, app).await.unwrap() });

    let mut browser = Tab::open(&format!("http://{address}/physician/"));

    // 1. Unauthenticated visit renders SignInScreen (never Today or clinical content).
    let sign_in_dom = browser.wait_for_text("Sign in to Coda", Duration::from_secs(15));
    assert!(
        sign_in_dom.contains("Work email")
            && sign_in_dom.contains("Send sign-in link")
            && !sign_in_dom.contains("Mary Louis"),
        "expected SignInScreen before authentication:\n{sign_in_dom}"
    );

    // 2. Local proof delivery signs in through the same redemption path.
    browser.request_sign_in("admin@clinic-a.test");
    let today_dom = browser.wait_for_text("Dr. Clinic Admin", Duration::from_secs(10));
    assert!(
        today_dom.contains("Today") && !today_dom.contains("Mary Louis"),
        "Mary Louis is not on Dr. Clinic Admin's panel yet and must not appear on Today:\n{today_dom}"
    );

    // Reload through the real proxy: localStorage must restore the bearer
    // session rather than rely on the prior in-memory client.
    browser.call("Page.reload", serde_json::json!({}));
    tokio::time::sleep(Duration::from_millis(250)).await;
    browser.wait_for_text("Dr. Clinic Admin", Duration::from_secs(10));
    let session = browser.eval("localStorage.getItem('coda.physician.session')");
    let token = session.as_str().expect("stored bearer session").to_owned();
    assert!(!token.is_empty());

    // 3. Cross-practice patient (Jake in practice-clinic-b) returns 404: not available, with zero data leak.
    browser.navigate(&format!("http://{address}/physician/?patient={jake}"));
    let cross_practice_dom =
        browser.wait_for_text("That patient is not available.", Duration::from_secs(10));
    assert!(
        cross_practice_dom.contains("Go to Today")
            && !cross_practice_dom.contains("Coda couldn’t load what needs you for this patient.")
            && !cross_practice_dom.contains("Jake")
            && !cross_practice_dom.contains("Records as of")
            && !cross_practice_dom.contains("Patient demographics"),
        "cross-practice patient must not leak demographics or clinical data:\n{cross_practice_dom}"
    );

    // 4. Same-clinic non-panel patient (Mary in practice-clinic-a) renders clean Clinic Demographic Card.
    browser.navigate(&format!("http://{address}/physician/?patient={mary}"));
    let demographic_dom =
        browser.wait_for_text("Clinic record · Not on your panel", Duration::from_secs(10));
    assert!(
        demographic_dom.contains("Patient demographics")
            && demographic_dom.contains("Mary Louis")
            && demographic_dom.contains("Mar 18, 1972")
            && !demographic_dom.contains("54 years")
            && !demographic_dom.contains("Female")
            && demographic_dom.contains("00000005")
            && demographic_dom.contains("Assign to panel")
            && !demographic_dom.contains("Records as of")
            && !demographic_dom.contains("This workspace has no chats."),
        "same-clinic non-panel patient should render only name, DOB, and MRN:\n{demographic_dom}"
    );

    // Verify phone viewport (390x844) and prefers-reduced-motion on the demographic card.
    browser.set_viewport(390, 844, true);
    let mobile_dom = browser.dom();
    assert!(
        mobile_dom.contains("Patient demographics") && mobile_dom.contains("Assign to panel"),
        "demographic card should remain intact on phone viewport:\n{mobile_dom}"
    );
    browser.set_viewport(1280, 800, false);

    // 5. Admin clicks "Assign to panel" -> PUT /auth/panels/... unlocks full clinical workspace live.
    browser.click("button.assign-panel-button");
    let unlocked_dom = browser.wait_for_text("Records as of", Duration::from_secs(10));
    assert!(
        unlocked_dom.contains("Mary Louis")
            && unlocked_dom.contains("Remove from panel")
            && !unlocked_dom.contains("Clinic record · Not on your panel"),
        "assigning patient to panel must unlock full clinical workspace live:\n{unlocked_dom}"
    );

    // 6. Admin clicks "Remove from panel" -> DELETE /auth/panels/... returns live to demographic card.
    browser.click("button.remove-panel-button");
    let relocked_dom =
        browser.wait_for_text("Clinic record · Not on your panel", Duration::from_secs(10));
    assert!(
        relocked_dom.contains("Patient demographics") && !relocked_dom.contains("Records as of"),
        "removing patient from panel must lock clinical workspace back to demographic card:\n{relocked_dom}"
    );

    // Sign-out crosses the proxy, revokes the real server session, and clears
    // browser persistence. Reload must remain at the gate.
    browser.click("[aria-label=\"Sign out of Coda\"]");
    browser.wait_for_text("Sign in to Coda", Duration::from_secs(10));
    let stored = browser.eval("localStorage.getItem('coda.physician.session')");
    assert_eq!(stored, serde_json::Value::Null);
    let response = reqwest::Client::new()
        .get(format!("http://{address}/physician/today"))
        .bearer_auth(&token)
        .send()
        .await
        .unwrap();
    assert_eq!(response.status(), reqwest::StatusCode::UNAUTHORIZED);
    browser.call("Page.reload", serde_json::json!({}));
    tokio::time::sleep(Duration::from_millis(250)).await;
    browser.wait_for_text("Sign in to Coda", Duration::from_secs(10));
    browser.sign_in_again();
    browser.wait_for_text("Dr. Clinic Admin", Duration::from_secs(10));

    // 7. A non-admin physician sees no panel controls, on a patient on their
    // panel or on a same-clinic patient outside it. Their name on the page means
    // the account has loaded, so absent controls aren't just a page still loading.
    let mut colleague = Tab::open(&format!("http://{address}/physician/"));
    colleague.wait_for_text("Sign in to Coda", Duration::from_secs(15));
    colleague.request_sign_in("colleague@clinic-a.test");
    colleague.wait_for_text("Dr. Colleague", Duration::from_secs(10));
    colleague.navigate(&format!("http://{address}/physician/?patient={mary}"));
    let on_panel_dom = colleague.wait_for_text("Records as of", Duration::from_secs(10));
    assert!(
        on_panel_dom.contains("Dr. Colleague")
            && !on_panel_dom.contains("Assign to panel")
            && !on_panel_dom.contains("Remove from panel"),
        "a non-admin must see no panel controls on their own patient:\n{on_panel_dom}"
    );
    store
        .assign_panel(&admin_identity, "phys-colleague", mary, Membership::Denied)
        .await
        .unwrap();
    colleague.navigate(&format!("http://{address}/physician/?patient={mary}"));
    let outside_panel_dom =
        colleague.wait_for_text("Clinic record · Not on your panel", Duration::from_secs(10));
    assert!(
        outside_panel_dom.contains("Dr. Colleague")
            && !outside_panel_dom.contains("Assign to panel")
            && !outside_panel_dom.contains("Remove from panel"),
        "a non-admin must see no panel controls outside their panel:\n{outside_panel_dom}"
    );
    drop(colleague);

    // 8. Deactivating the physician account terminates SSE through the proxy, signs the browser out and says why.
    assert!(store.deactivate("admin@clinic-a.test").await);
    let revoked_dom = browser.wait_for_text("No access to this practice", Duration::from_secs(10));
    assert!(
        revoked_dom.contains("Use a different email") && !revoked_dom.contains("Mary Louis"),
        "removing access must terminate SSE and show No access:\n{revoked_dom}"
    );

    proxy.abort();
    upstream.abort();
    unsafe {
        env::remove_var("PHYSICIAN_AUTH");
        env::remove_var("PHYSICIAN_AUTH_PROOF_TOKENS");
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore = "browser qualification; run explicitly with --ignored"]
async fn a_read_asked_a_new_question_shows_loading_until_that_answer_lands() {
    let _guard = test_lock().lock().await;
    let site = build_site();
    let server = common::serve_isolated(|| {
        // SAFETY: tests in this binary hold `test_lock()` before mutating
        // process env, and `serve_isolated` holds its start lock here.
        unsafe {
            env::set_var("PHYSICIAN_SITE_DIR", &site);
            env::remove_var("PHYSICIAN_AUTH");
            env::remove_var("PHYSICIAN_AUTH_PROOF_TOKENS");
            env::remove_var("PHYSICIAN_UPSTREAM");
        }
    })
    .await;
    let address = server.address;

    let mut browser = Tab::open("about:blank");
    // Requests whose URL matches `__hold` wait in the page until `__release()`,
    // so the test sees what shows while an answer is still on its way.
    browser.call(
        "Page.addScriptToEvaluateOnNewDocument",
        serde_json::json!({ "source": include_str!("js/hold_matching_requests.js") }),
    );
    browser.navigate(&format!("http://{address}/"));

    // The schedule rail shows its own loading line; it never waits hidden.
    browser.wait_for_text("Loading the schedule…", Duration::from_secs(15));
    browser.eval("window.__release()");
    browser.wait_for_text("Aug 27, 2026", Duration::from_secs(10));

    browser.click("button[aria-label=\"Find patient\"]");
    browser.search_for("Mary");
    browser.wait_for_text("DOB", Duration::from_secs(10));
    browser.click(".dialog-results button");
    browser.wait_for_text("MRN 00000005", Duration::from_secs(10));

    // A new query never shows the last query's rows or a false "No patients match."
    browser.click("button[aria-label=\"Find patient\"]");
    browser.eval("window.__hold = '/patient-roster'");
    browser.search_for("Tanvir");
    browser.wait_for_text("Searching…", Duration::from_secs(10));
    let searching = browser.dialog_text();
    assert!(
        !searching.contains("Mary Louis") && !searching.contains("No patients match."),
        "a search still on its way showed another answer:\n{searching}"
    );
    browser.eval("window.__release()");
    browser.wait_for_text("Tanvir Chowdhury", Duration::from_secs(10));

    // Switching patients never shows the last patient while the next one opens.
    browser.eval("window.__hold = 'pw-67305f4a-7d91-462c-9cc5-1bb30ade34ee'");
    browser.click(".dialog-results button");
    let opening = browser.wait_for_text("Opening the patient…", Duration::from_secs(10));
    assert!(
        !opening.contains("MRN 00000005"),
        "the next patient's page still showed the last patient:\n{opening}"
    );
    browser.eval("window.__release()");
    browser.wait_for_text("MRN 00000003", Duration::from_secs(10));

    // A patient link opened directly shows the patient's rail loading too.
    browser.navigate(&format!(
        "http://{address}/?patient=pw-39e51534-6ce4-43cf-b5f8-7c1ca6252d8e"
    ));
    browser.wait_for_text("Loading the schedule…", Duration::from_secs(15));
    browser.eval("window.__release()");
    browser.wait_for_text("Records as of", Duration::from_secs(10));
}

/// A reply using everything a physician's reply can hold: a kept line break, a list, words in angle
/// brackets, a citation marker the page doesn't draw yet, and code and a table too wide for a phone.
const RICH_REPLY: &str = "Potassium is **5.4** today.\nRecheck on Friday.\n\n- Hold the supplement\n- Repeat the panel\n\nThe order reads <clinic-note> and cites [[patient_source|a|b]].\n\n```sql\nselect patient, potassium, collected_at, reported_at, ordering_physician, performing_laboratory from basic_metabolic_panel;\n```\n\n| Collected | Potassium | Reference range | Performing laboratory | Ordering physician |\n| --- | --- | --- | --- | --- |\n| Tuesday morning | 5.4 mmol/L | 3.5 to 5.0 mmol/L | Synthetic Regional Laboratory | Synthetic Physician |\n";

/// What the next conversation read meets, so the page's loading and error states can be reached.
#[derive(Clone, Copy, PartialEq)]
enum SnapshotGate {
    Open,
    Fails,
    Held,
}

static SNAPSHOT: std::sync::Mutex<SnapshotGate> = std::sync::Mutex::new(SnapshotGate::Open);

fn snapshot_gate() -> SnapshotGate {
    *SNAPSHOT.lock().unwrap()
}

fn set_snapshot_gate(gate: SnapshotGate) {
    *SNAPSHOT.lock().unwrap() = gate;
}

async fn gate_snapshot(
    request: axum::extract::Request,
    next: axum::middleware::Next,
) -> axum::response::Response {
    if request.uri().path().ends_with("/snapshot") {
        while snapshot_gate() == SnapshotGate::Held {
            tokio::time::sleep(Duration::from_millis(50)).await;
        }
        if snapshot_gate() == SnapshotGate::Fails {
            return axum::http::StatusCode::INTERNAL_SERVER_ERROR.into_response();
        }
    }
    next.run(request).await
}

/// Adds a saved reply to the fixture workspace to exercise rendering and revision controls.
async fn with_rich_draft(
    request: axum::extract::Request,
    next: axum::middleware::Next,
) -> axum::response::Response {
    with_draft(request, next, RICH_REPLY).await
}

/// A draft whose first line is a threshold, which Markdown would read as a quote.
const THRESHOLD_DRAFT: &str = "> 180 mg/dL: call the office.\n>= 200 mg/dL: go to the ED.";

async fn with_threshold_draft(
    request: axum::extract::Request,
    next: axum::middleware::Next,
) -> axum::response::Response {
    with_draft(request, next, THRESHOLD_DRAFT).await
}

async fn with_draft(
    request: axum::extract::Request,
    next: axum::middleware::Next,
    draft: &str,
) -> axum::response::Response {
    let path = request.uri().path().to_owned();
    let response = next.run(request).await;
    if path != format!("/patient-workspaces/{REVIEW_PATIENT}") {
        return response;
    }
    let (mut parts, body) = response.into_parts();
    let bytes = axum::body::to_bytes(body, usize::MAX).await.unwrap();
    let mut workspace: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
    workspace["replies"] = serde_json::json!([{
        "reply_work_id": "reply-rich",
        "patient_workspace_id": REVIEW_PATIENT,
        "patient_display_name": "Jake Smith",
        "channel": "spruce",
        "headline": "Synthetic draft to review",
        "draft_text": draft,
        "urgency": "urgent_ed",
        "created_at": "2026-08-27T12:05:00Z",
        "state": "ready",
        "draft_revision": 1,
    }]);
    parts.headers.remove(axum::http::header::CONTENT_LENGTH);
    axum::response::Response::from_parts(parts, axum::body::Body::from(workspace.to_string()))
}

const REVIEW_PATIENT: &str = "pw-82c44e9b-58c0-4af1-bd21-771bf77e101d";

const CONVERSATION_PERSON: &str = "[aria-label=\"Conversation\"] .message-person";
const CONVERSATION_REPLY: &str = "[aria-label=\"Conversation\"] .message-assistant";
const REVIEW_FIELD: &str = ".reply-review-ask input";

fn assert_rich(reply: &serde_json::Value, body_font: &str) {
    let text = reply["text"].as_str().unwrap_or_default();
    assert_eq!(reply["prose"], true, "not drawn by Prose: {reply}");
    assert_eq!(reply["code"], true, "the code block is missing: {reply}");
    assert_eq!(reply["table"], true, "the table is missing: {reply}");
    assert_eq!(reply["list"], true, "the list is missing: {reply}");
    assert!(
        text.contains("today.\nRecheck on Friday."),
        "the line break was lost: {text}"
    );
    assert!(
        text.contains("<clinic-note>"),
        "words in angle brackets were dropped: {text}"
    );
    assert!(
        text.contains("[[patient_source|a|b]]"),
        "the citation marker changed: {text}"
    );
    assert!(
        reply["font"]
            .as_str()
            .unwrap_or_default()
            .contains(body_font),
        "the words changed size: {reply}"
    );
}

struct ChatBrowserServer {
    address: std::net::SocketAddr,
    worker: scale::Process,
    site_task: tokio::task::JoinHandle<()>,
    _stub: scale::ResponsesStub,
    _valkey: scale::Valkey,
    _database: scale::Database,
}

impl Drop for ChatBrowserServer {
    fn drop(&mut self) {
        self.worker.kill_now();
        self.site_task.abort();
    }
}

async fn serve_chat_browser(site: &std::path::Path, reply: &str) -> ChatBrowserServer {
    serve_chat_browser_with_stub(site, 0, "[]", reply).await
}

async fn serve_chat_browser_with_stub(
    site: &std::path::Path,
    delay_ms: u64,
    rounds: &str,
    reply: &str,
) -> ChatBrowserServer {
    let state = env::temp_dir().join(format!("coda-chat-browser-{}", uuid::Uuid::new_v4()));
    let _ = fs::remove_dir_all(&state);
    fs::create_dir_all(&state).unwrap();
    // SAFETY: tests in this binary hold `test_lock()` before mutating process env.
    unsafe {
        env::set_var("PHYSICIAN_SITE_DIR", site);
        for key in [
            "PHYSICIAN_AUTH",
            "PHYSICIAN_AUTH_PROOF_TOKENS",
            "PHYSICIAN_UPSTREAM",
        ] {
            env::remove_var(key);
        }
    }
    let database = scale::Database::create("browser_chat")
        .await
        .expect("browser proof database");
    scale::staging_account(&database.url).await;
    let store = local_product::runtime::Store::connect(&database.url)
        .await
        .unwrap();
    let worker_port = scale::free_port();
    let valkey = scale::Valkey::start().await;
    let redis = valkey.url();
    let stub = scale::ResponsesStub::start_with_reply(&state, 0, delay_ms, rounds, reply).await;
    let listen = format!("127.0.0.1:{worker_port}");
    let mut worker = scale::Process::spawn(
        "worker",
        scale::worker_bin(),
        &[],
        &[
            ("PHYSICIAN_DATABASE_URL", &database.url),
            ("PHYSICIAN_WORKER_LISTEN_ADDR", &listen),
            ("PHYSICIAN_RESPONSES_STUB_PORT", &stub.port),
            ("PHYSICIAN_REDIS_URL", &redis),
        ],
        &state,
    );
    scale::wait_ready(&mut worker, worker_port).await;
    let bus: Arc<dyn physician_bus::Bus> = Arc::new(
        physician_bus::RedisBus::connect(&redis, None)
            .await
            .unwrap(),
    );
    let chat = Arc::new(physician_sample::durable::DurableChat::new(
        store,
        local_product::physician_chat::PhysicianSkills::configured(None, false).unwrap(),
        &scale::models::roles(),
    ));
    let auth = physician_auth::PostgresAuthStore::connect_without_migrations(&database.url)
        .await
        .unwrap();
    let front = front_door::PostgresFrontDoorStore::connect_without_migrations(&database.url)
        .await
        .unwrap();
    set_snapshot_gate(SnapshotGate::Open);
    let app = physician_sample::authed_router_on(
        physician_sample::durable_site(chat, bus),
        Arc::new(auth),
        Arc::new(front),
    )
    .await
    .layer(axum::middleware::from_fn(gate_snapshot));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    let site_task = tokio::spawn(async move { axum::serve(listener, app).await.unwrap() });
    wait_until_ready(address).await;

    ChatBrowserServer {
        address,
        worker,
        site_task,
        _stub: stub,
        _valkey: valkey,
        _database: database,
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore = "browser qualification; run explicitly with --ignored"]
async fn held_limits_and_provider_blocks_render_without_a_partial_answer() {
    use harness::engine::{AttemptFailure, Event, EventKind};
    use local_product::runtime::{Input, Scope, Store};
    let _guard = test_lock().lock().await;
    let site = build_site();
    let mut server = serve_chat_browser(&site, scale::FIXED_TEXT).await;
    // This journey claims and ends each turn by hand. A running worker would
    // claim the accepted turn first, so it stops before any turn exists.
    server.worker.kill_now();
    let base = format!("http://{}", server.address);
    let store = Store::connect(&server._database.url).await.unwrap();
    let mut browser = Tab::open(&base);
    browser.wait_for_text("Work email", Duration::from_secs(20));
    browser.request_sign_in("physician@codahealth.test");
    browser.wait_for_text("Nothing needs you right now.", Duration::from_secs(20));
    let bearer = browser
        .eval("localStorage.getItem('coda.physician.session')")
        .as_str()
        .unwrap()
        .to_owned();
    let patient = physician_sample::WORKSPACE_IDS[0];
    for (blocked, expected, picture) in [
        (
            false,
            "Coda reached its limit for this response. Choose Try again to continue.",
            "held-limit-no-partial",
        ),
        (
            true,
            "Coda couldn’t finish this response. Your message is saved.",
            "held-provider-blocked",
        ),
    ] {
        let scope = Scope {
            practice: physician_sample::STAGING_PRACTICE_ID.into(),
            physician: "staging-physician".into(),
            workspace: patient.into(),
            chat: uuid::Uuid::new_v4().to_string(),
        };
        store
            .accept_authorized(
                &scope,
                &Input {
                    key: format!("held-input-{picture}"),
                    text: "Review this synthetic question.".into(),
                    timezone: "UTC".into(),
                    scenario: None,
                },
                physician_auth::PostgresAuthStore::session_reference(&bearer),
                &local_product::physician_chat::PhysicianSkills::configured(None, false).unwrap(),
                &scale::models::chat_recipe(),
                None,
            )
            .await
            .unwrap();
        let claim = store.claim(&scope, 60).await.unwrap().unwrap();
        if blocked {
            store
                .persist(
                    &claim,
                    Event {
                        attempt_id: claim.attempt.clone(),
                        sequence: 1,
                        ordinal: 1,
                        kind: EventKind::ProviderBlocked {
                            reason: "raw-provider-diagnostic-739".into(),
                        },
                    },
                )
                .await
                .unwrap();
        } else {
            store
                .settle_chat_attempt(
                    &claim,
                    AttemptFailure::Limit,
                    local_product::runtime::delivery::Backoff::IMMEDIATE,
                )
                .await
                .unwrap();
        }
        assert_eq!(store.conversation(&scope).await.unwrap().messages.len(), 1);
        browser.navigate(&format!("{base}/?patient={patient}&chat={}", scope.chat));
        browser.wait_for_text(expected, Duration::from_secs(20));
        assert_eq!(
            browser.step(
                "readText",
                serde_json::json!([".conversation-mechanical button"])
            ),
            serde_json::json!("Try again")
        );
        let dom = browser.dom();
        assert!(!dom.contains("Partial response") && !dom.contains("raw-provider-diagnostic-739"));
        browser.save_picture(picture);
    }
}

/// In a real turn, a reply draws its code and table, and each scrolls by key where it can't fit.
/// The conversation's empty, loading, error and recovery states are walked around it.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore = "browser qualification; run explicitly with --ignored"]
async fn a_reply_draws_code_and_a_table_that_scroll_by_key() {
    let _guard = test_lock().lock().await;
    let site = build_site();
    let server = serve_chat_browser(&site, RICH_REPLY).await;
    let address = server.address;

    let patient = "pw-39e51534-6ce4-43cf-b5f8-7c1ca6252d8e";
    let mut browser = Tab::open(&format!("http://{address}/?patient={patient}"));
    browser.wait_for_text("Work email", Duration::from_secs(20));
    browser.request_sign_in("physician@codahealth.test");
    browser.wait_for_text("This workspace has no chats.", Duration::from_secs(20));
    browser.step(
        "click",
        serde_json::json!(["button.sidebar-new-conversation"]),
    );
    browser.wait_for_text(
        "What would you like Coda to review?",
        Duration::from_secs(10),
    );
    browser.save_pictures("thread-empty", ".conversation-empty");

    browser.type_composer("Please review **Friday**\nand the panel");
    browser.step(
        "clickNamed",
        serde_json::json!([".composer-toolbar button", "Send"]),
    );
    browser.wait_for_text("Synthetic Regional Laboratory", Duration::from_secs(30));
    browser.save_pictures("thread-reply", ".message-assistant");

    // Reached by reopening the conversation, as the page itself would.
    set_snapshot_gate(SnapshotGate::Held);
    browser.reload();
    browser.wait_for_text("Opening the conversation…", Duration::from_secs(20));
    browser.save_pictures("thread-loading", ".composer");
    set_snapshot_gate(SnapshotGate::Fails);
    browser.wait_for_text(
        "Coda couldn’t open this conversation.",
        Duration::from_secs(20),
    );
    browser.save_pictures("thread-error", ".composer");
    set_snapshot_gate(SnapshotGate::Open);
    browser.step(
        "clickNamed",
        serde_json::json!(["[aria-label=\"Conversation\"] button", "Try again"]),
    );
    browser.wait_for_text("Synthetic Regional Laboratory", Duration::from_secs(20));
    browser.save_pictures("thread-recovered", ".message-assistant");

    // Physicians don't write Markdown, so their words show as they wrote them.
    let person = browser.step(
        "readText",
        serde_json::json!([format!("{CONVERSATION_PERSON} p")]),
    );
    assert_eq!(
        person.as_str(),
        Some("Please review **Friday**\nand the panel"),
        "the physician's own words read as Markdown"
    );
    let reply = browser.step("readReply", serde_json::json!([CONVERSATION_REPLY]));
    assert_rich(&reply, "15px / 22px");

    browser.set_viewport(390, 844, true);
    // A table too wide for a phone scrolls; it doesn't split its words to fit.
    let lines = browser.step(
        "headerLines",
        serde_json::json!([
            format!("{CONVERSATION_REPLY} .prose-table-scroll"),
            "Potassium"
        ]),
    );
    assert_eq!(lines, 1, "a table header split across lines: {lines}");
    for part in [".code-block-body", ".prose-table-scroll"] {
        let target = format!("{CONVERSATION_REPLY} {part}");
        browser.tab_to(&target);
        browser.press_key(Key::ARROW_RIGHT);
        browser.wait_for_scroll(&target, |left| left > 0.0);
        browser.press_key(Key::ARROW_LEFT);
        browser.wait_for_scroll(&target, |left| left == 0.0);
    }
}

/// A threshold at the start of a line reads as written, so the physician reviews what Copy sends.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore = "browser qualification; run explicitly with --ignored"]
async fn a_draft_keeps_a_threshold_that_markdown_would_read_as_a_quote() {
    let _guard = test_lock().lock().await;
    let site = build_site();
    // SAFETY: tests in this binary hold `test_lock()` before mutating process env.
    unsafe {
        env::set_var("PHYSICIAN_SITE_DIR", &site);
    }
    let app =
        physician_sample::fixture_site().layer(axum::middleware::from_fn(with_threshold_draft));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    tokio::spawn(async move { axum::serve(listener, app).await.unwrap() });
    wait_until_ready(address).await;

    let mut browser = Tab::open(&format!("http://{address}/?patient={REVIEW_PATIENT}"));
    browser.wait_for_text("Synthetic draft to review", Duration::from_secs(20));
    let shown = browser.step("readText", serde_json::json!(["article.reply-review"]));
    assert!(
        shown
            .as_str()
            .is_some_and(|words| words.contains(THRESHOLD_DRAFT)),
        "the review shows other words than the draft holds: {shown}"
    );
    let copied = browser.step(
        "copied",
        serde_json::json!(["[data-reply-action=\"copy\"]"]),
    );
    assert_eq!(copied.as_str(), Some(THRESHOLD_DRAFT));
}

/// A reply draft goes to a patient as plain text, so the review shows exactly what Copy sends, and
/// the instruction to revise it takes keys as it did.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore = "browser qualification; run explicitly with --ignored"]
async fn a_reply_draft_reads_as_written_and_copies_its_own_words() {
    let _guard = test_lock().lock().await;
    let site = build_site();
    // SAFETY: tests in this binary hold `test_lock()` before mutating process env.
    unsafe {
        env::set_var("PHYSICIAN_SITE_DIR", &site);
    }
    let app = physician_sample::fixture_site().layer(axum::middleware::from_fn(with_rich_draft));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    tokio::spawn(async move { axum::serve(listener, app).await.unwrap() });
    wait_until_ready(address).await;

    let mut browser = Tab::open(&format!("http://{address}/?patient={REVIEW_PATIENT}"));
    browser.wait_for_text("Synthetic draft to review", Duration::from_secs(20));
    browser.save_pictures("review-draft", "article.reply-review");
    browser.save_pictures("patient-summary-citations", "div.synopsis");

    let copied = browser.step(
        "copied",
        serde_json::json!(["[data-reply-action=\"copy\"]"]),
    );
    assert_eq!(
        copied.as_str(),
        Some(RICH_REPLY),
        "Copy no longer sends the draft's own words"
    );
    browser.wait_for_text("Copied", Duration::from_secs(5));

    let field = browser.step("field", serde_json::json!([REVIEW_FIELD]));
    assert_eq!(
        field["labelContained"], true,
        "a hidden label reaches past its field and can scroll the page"
    );
    assert_eq!(
        field["name"], field["placeholder"],
        "the instruction is not named by what it asks: {field}"
    );
    // Stacked on a phone, the field takes only its own height.
    browser.set_viewport(390, 844, true);
    let stacked = browser.step("field", serde_json::json!([REVIEW_FIELD]));
    browser.set_viewport(1440, 900, true);
    assert_eq!(
        stacked["hostHeight"], stacked["inputHeight"],
        "the instruction field holds empty space on a phone: {stacked}"
    );
    browser.tab_to(REVIEW_FIELD);
    browser.call(
        "Input.insertText",
        serde_json::json!({ "text": "Make it shorter" }),
    );
    browser.wait_for_text("Ask Coda", Duration::from_secs(5));
    browser.save_pictures("review-instruction", ".reply-review-ask");
    browser.press_key(Key::ENTER);
    let cleared = browser.step("valueSoon", serde_json::json!([REVIEW_FIELD]));
    assert_eq!(
        cleared.as_str(),
        Some(""),
        "Enter did not ask for the revision"
    );

    let shown = browser.step("readText", serde_json::json!(["article.reply-review"]));
    assert!(
        shown
            .as_str()
            .is_some_and(|words| words.contains(RICH_REPLY.trim_end())),
        "the review shows other words than Copy sends: {shown}"
    );
}

/// Reads the shell until `settled` holds; the page answers a press a moment later.
fn shell_when(
    browser: &mut Tab,
    what: &str,
    settled: impl Fn(&serde_json::Value) -> bool,
) -> serde_json::Value {
    let deadline = Instant::now() + Duration::from_secs(5);
    loop {
        let shell = browser.step("readShell", serde_json::json!([]));
        if settled(&shell) {
            return shell;
        }
        assert!(Instant::now() < deadline, "{what}: {shell}");
        std::thread::sleep(Duration::from_millis(50));
    }
}

fn open_the_drawer(browser: &mut Tab) {
    browser.click("button.stage-menu");
    let open = shell_when(
        browser,
        "the drawer did not open with focus inside",
        |shell| {
            shell["sidebar"] == true
                && shell["sidebarLeft"] == 0
                && shell["stageInert"] == true
                && shell["backdrop"] == true
                && shell["focus"] == "Close navigation"
        },
    );
    assert_eq!(open["close"], true, "the drawer has no close: {open}");
    assert_eq!(open["labels"], true, "the drawer is not a rail: {open}");
    assert_eq!(
        open["collapse"], false,
        "the drawer offers the rail: {open}"
    );
    assert_eq!(open["expand"], false, "the drawer offers the rail: {open}");
}

fn drawer_closed(shell: &serde_json::Value) -> bool {
    shell["sidebar"] == false && shell["stageInert"] == false && shell["backdrop"] == false
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore = "browser qualification; run explicitly with --ignored"]
async fn the_sidebar_is_a_closed_drawer_on_a_phone_and_a_rail_beside_the_page_otherwise() {
    let _guard = test_lock().lock().await;
    let site = build_site();
    // SAFETY: tests in this binary hold `test_lock()` before mutating process env.
    unsafe {
        env::set_var("PHYSICIAN_SITE_DIR", &site);
    }
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    let app = physician_sample::fixture_site();
    tokio::spawn(async move { axum::serve(listener, app).await.unwrap() });
    wait_until_ready(address).await;

    let mut browser = Tab::open(&format!("http://{address}/"));
    browser.wait_for_text("Today", Duration::from_secs(20));

    // A phone starts on the whole page, with the sidebar out of sight, focus and reach.
    browser.set_viewport(390, 844, true);
    browser.reload();
    browser.wait_for_text("Today", Duration::from_secs(20));
    let start = shell_when(&mut browser, "a phone did not start closed", drawer_closed);
    assert_eq!(
        start["stageLeft"], 0,
        "the page is not at the edge: {start}"
    );
    assert_eq!(start["stageWidth"], 390, "the page is not whole: {start}");
    assert_eq!(
        start["pageWidth"], 390,
        "the page scrolls sideways: {start}"
    );
    assert_eq!(start["menu"], true, "a phone has no way in: {start}");
    browser.save_picture("drawer-closed-phone-light");

    // Every way out closes it; its own controls hand focus back to the menu.
    open_the_drawer(&mut browser);
    browser.save_picture("drawer-open-phone-light");
    // Tab walks the drawer's own controls round to its first, never stopping on the shell.
    let mut came_round = false;
    for _ in 0..60 {
        browser.press_key(Key::TAB);
        let on_shell = browser.step("hasFocus", serde_json::json!([".glass-app"]));
        assert_ne!(on_shell, true, "Tab stopped on the shell behind the drawer");
        let on_close = browser.step("hasFocus", serde_json::json!(["button.sidebar-close"]));
        if on_close == true {
            came_round = true;
            break;
        }
    }
    assert!(came_round, "Tab never came back round the drawer");
    browser.press_key(Key::ESCAPE);
    shell_when(&mut browser, "Escape left the drawer open", |shell| {
        drawer_closed(shell) && shell["focus"] == "Open navigation"
    });
    open_the_drawer(&mut browser);
    browser.click(".glass-sidebar-backdrop");
    shell_when(&mut browser, "the backdrop left the drawer open", |shell| {
        drawer_closed(shell) && shell["focus"] == "Open navigation"
    });
    open_the_drawer(&mut browser);
    browser.click("button.sidebar-close");
    shell_when(&mut browser, "Close left the drawer open", |shell| {
        drawer_closed(shell) && shell["focus"] == "Open navigation"
    });
    // A dialog opened from it hands focus back to the menu when it closes.
    open_the_drawer(&mut browser);
    browser.click_named("button", "Keyboard shortcuts");
    browser.wait_for_text("Keyboard shortcuts", Duration::from_secs(5));
    shell_when(
        &mut browser,
        "the dialog left the drawer open",
        drawer_closed,
    );
    std::thread::sleep(Duration::from_millis(300));
    browser.press_key(Key::ESCAPE);
    shell_when(&mut browser, "the dialog lost the page's focus", |shell| {
        drawer_closed(shell) && shell["focus"] == "Open navigation"
    });
    // Going to the page already open still closes it, as going elsewhere does.
    for place in ["Today", "Updates"] {
        open_the_drawer(&mut browser);
        browser.click_named(".sidebar-nav-item", place);
        shell_when(
            &mut browser,
            "going somewhere left the drawer open",
            drawer_closed,
        );
    }

    // A window grown past the drawer has the sidebar beside the page and nothing left inert.
    open_the_drawer(&mut browser);
    browser.set_viewport(900, 800, true);
    let beside = shell_when(
        &mut browser,
        "growing left the drawer over the page",
        |shell| {
            shell["sidebar"] == true && shell["stageInert"] == false && shell["backdrop"] == false
        },
    );
    assert_eq!(
        beside["stageLeft"], 244,
        "the page isn't beside it: {beside}"
    );
    assert_eq!(beside["sidebarLeft"], 0, "the sidebar moved: {beside}");
    assert_eq!(
        beside["menu"], false,
        "a wide window has the menu: {beside}"
    );
    assert_eq!(
        beside["close"], false,
        "a wide window has the close: {beside}"
    );
    assert_eq!(
        beside["collapse"], true,
        "a wide window lost collapse: {beside}"
    );
    browser.set_viewport(899, 800, true);
    shell_when(&mut browser, "899px is not a drawer", drawer_closed);

    // The rail still collapses and expands beside the page.
    browser.set_viewport(1280, 800, true);
    browser.click("button.sidebar-collapse");
    let rail = |shell: &serde_json::Value| {
        shell["sidebarWidth"] == 56 && shell["stageLeft"] == 56 && shell["expand"] == true
    };
    shell_when(&mut browser, "collapse did not make a rail", rail);

    // A phone ignores the rail, and the drawer opens whole, without forgetting it.
    browser.set_viewport(390, 844, true);
    let collapsed = shell_when(&mut browser, "the rail stayed on a phone", drawer_closed);
    assert_eq!(
        collapsed["stageWidth"], 390,
        "the page is not whole: {collapsed}"
    );
    assert_eq!(
        collapsed["expand"], false,
        "a phone offers expand: {collapsed}"
    );
    open_the_drawer(&mut browser);
    browser.press_key(Key::ESCAPE);
    shell_when(&mut browser, "Escape left the drawer open", drawer_closed);
    browser.set_viewport(1280, 800, true);
    shell_when(&mut browser, "the phone forgot the rail", rail);
    browser.click("button.sidebar-expand");
    shell_when(
        &mut browser,
        "expand did not bring the sidebar back",
        |shell| {
            shell["sidebarWidth"] == 244 && shell["stageLeft"] == 244 && shell["collapse"] == true
        },
    );
    browser.save_picture("sidebar-desktop-light");
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore = "browser and disposable Postgres qualification; run explicitly with --ignored"]
async fn drafts_survive_site_restart_in_independent_browser_sessions() {
    let _guard = test_lock().lock().await;
    let Some(database) = scale::Database::create("draft_browser").await else {
        return;
    };
    let site = build_site();
    scale::staging_account(&database.url).await;
    let home =
        std::env::temp_dir().join(format!("physician-draft-browser-{}", uuid::Uuid::new_v4()));
    fs::create_dir_all(&home).unwrap();
    let mut sites = Vec::new();
    let mut hosts = Vec::new();
    for name in ["draft-a", "draft-b"] {
        let port = scale::free_port();
        let host = format!("127.0.0.1:{port}");
        let mut process = scale::Process::spawn(
            name,
            scale::site_bin(),
            &[],
            &[
                ("PHYSICIAN_DATABASE_URL", &database.url),
                scale::DURABLE,
                ("PHYSICIAN_AUTH", "required"),
                ("PHYSICIAN_AUTH_PROOF_TOKENS", "1"),
                ("PHYSICIAN_SITE_DIR", site.to_str().unwrap()),
                ("PHYSICIAN_LISTEN_ADDR", &host),
            ],
            &home,
        );
        scale::wait_ready(&mut process, port).await;
        sites.push(process);
        hosts.push(host);
    }
    let patient = physician_sample::WORKSPACE_IDS[0];
    let chat = "chat-durable-browser";
    let urls: Vec<_> = hosts
        .iter()
        .map(|host| format!("http://{host}/?patient={patient}&chat={chat}"))
        .collect();
    let mut a = Tab::open(&urls[0]);
    a.wait_for_text("Work email", Duration::from_secs(20));
    a.request_sign_in("physician@codahealth.test");
    a.wait_composer("", Duration::from_secs(20));
    a.type_composer("Draft from the first browser");
    a.wait_saved();
    a.save_pictures("durable-draft-a", ".composer-context");
    let mut b = Tab::open(&urls[1]);
    b.wait_for_text("Work email", Duration::from_secs(20));
    b.request_sign_in("physician@codahealth.test");
    b.wait_composer("Draft from the first browser", Duration::from_secs(20));
    assert!(
        b.recovery_keys().is_empty(),
        "fresh browser inherited local draft storage"
    );
    b.type_composer("Draft from the second browser");
    b.wait_saved();
    a.type_composer("Unsaved conflicting text from the first browser");
    assert_eq!(
        a.wait_conflict(),
        (2, "Draft from the second browser".into())
    );
    a.save_pictures("durable-draft-conflict", ".draft-conflict");
    a.click_conflict("Use newer text");
    a.wait_composer("Draft from the second browser", Duration::from_secs(10));
    drop(a);
    drop(b);
    for (process, host) in sites.iter_mut().zip(&hosts) {
        process.restart();
        scale::wait_ready(process, host.rsplit_once(':').unwrap().1.parse().unwrap()).await;
    }
    let session = scale::sign_in(&hosts[0]).await;
    let path = format!("/physician-state/drafts/{patient}/{chat}");
    let readback = scale::get_json(&hosts[1], &path, &session).await;
    assert_eq!(
        readback,
        (
            200,
            serde_json::json!({"revision": 2, "text": "Draft from the second browser"})
        )
    );
    println!(
        "Both site processes restarted; durable draft readback: {}",
        readback.1
    );
    let mut fresh = Tab::open(&urls[0]);
    fresh.wait_for_text("Work email", Duration::from_secs(20));
    fresh.request_sign_in("physician@codahealth.test");
    fresh.wait_composer("Draft from the second browser", Duration::from_secs(20));
    assert!(fresh.recovery_keys().is_empty());
    assert_eq!(fresh.composer().1, "");
    fresh.save_pictures("durable-draft-after-restart", ".composer-context");
    drop(fresh);
    drop(sites);
    fs::remove_dir_all(home).unwrap();
}

/// Each of Coda's responses in the chat snapshot as `(response_id, feedback)`.
async fn snapshot_feedback(
    host: &str,
    session: &str,
    patient: &str,
    chat: &str,
) -> Vec<(serde_json::Value, serde_json::Value)> {
    let (status, snapshot) = scale::get_json(
        host,
        &format!("/patient-workspaces/{patient}/chats/{chat}/snapshot"),
        session,
    )
    .await;
    assert_eq!(status, 200, "{snapshot}");
    scale::chat(&snapshot["snapshot"])
        .transcript
        .into_iter()
        .map(|exchange| match exchange.answer {
            physician_wire::AnswerState::Finished {
                response_id,
                feedback,
                ..
            } => (serde_json::json!(response_id), serde_json::json!(feedback)),
            other => panic!("an unfinished answer: {other:?}"),
        })
        .collect()
}

/// Reads each response's feedback controls until `settled` holds.
fn ratings_when(
    tab: &mut Tab,
    what: &str,
    settled: impl Fn(&[serde_json::Value]) -> bool,
) -> Vec<serde_json::Value> {
    let deadline = Instant::now() + Duration::from_secs(10);
    loop {
        let ratings = tab.step("ratings", serde_json::json!([]));
        let ratings = ratings.as_array().cloned().unwrap_or_default();
        if settled(&ratings) {
            return ratings;
        }
        assert!(Instant::now() < deadline, "{what}: {ratings:?}");
        std::thread::sleep(Duration::from_millis(60));
    }
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore = "browser and disposable Postgres qualification; run explicitly with --ignored"]
async fn feedback_stays_on_its_response_across_site_restart() {
    let _guard = test_lock().lock().await;
    let Some(database) = scale::Database::create("feedback_browser").await else {
        return;
    };
    let site = build_site();
    scale::staging_account(&database.url).await;
    let patient = physician_sample::WORKSPACE_IDS[0];
    let chat = "chat-feedback-browser";
    let store = local_product::runtime::Store::connect(&database.url)
        .await
        .expect("runtime store");
    let responses = scale::answered_chat(
        &store,
        patient,
        chat,
        &[
            ("Latest potassium?", "Potassium was 4.1 on Monday."),
            ("And sodium?", "Sodium was 139 on Monday."),
        ],
    )
    .await;
    let [first, second] = [responses[0].as_str(), responses[1].as_str()];
    let home = std::env::temp_dir().join(format!(
        "physician-feedback-browser-{}",
        uuid::Uuid::new_v4()
    ));
    fs::create_dir_all(&home).unwrap();
    let mut sites = Vec::new();
    let mut hosts = Vec::new();
    for name in ["feedback-a", "feedback-b"] {
        let port = scale::free_port();
        let host = format!("127.0.0.1:{port}");
        let mut process = scale::Process::spawn(
            name,
            scale::site_bin(),
            &[],
            &[
                ("PHYSICIAN_DATABASE_URL", &database.url),
                scale::DURABLE,
                ("PHYSICIAN_AUTH", "required"),
                ("PHYSICIAN_AUTH_PROOF_TOKENS", "1"),
                ("PHYSICIAN_SITE_DIR", site.to_str().unwrap()),
                ("PHYSICIAN_LISTEN_ADDR", &host),
            ],
            &home,
        );
        scale::wait_ready(&mut process, port).await;
        sites.push(process);
        hosts.push(host);
    }
    let urls: Vec<_> = hosts
        .iter()
        .map(|host| format!("http://{host}/?patient={patient}&chat={chat}"))
        .collect();
    let mut tab = Tab::open(&urls[0]);
    tab.wait_for_text("Work email", Duration::from_secs(20));
    tab.request_sign_in("physician@codahealth.test");
    tab.wait_for_text("Sodium was 139 on Monday.", Duration::from_secs(20));
    ratings_when(&mut tab, "both responses offer unrated thumbs", |ratings| {
        ratings.len() == 2
            && ratings
                .iter()
                .all(|rating| rating["enabled"] == true && rating["pressed"].is_null())
    });
    tab.save_pictures("feedback-unrated", "[aria-label=\"Conversation\"]");

    tab.step("clickInRating", serde_json::json!([0, "Good response"]));
    ratings_when(&mut tab, "the first response saved as good", |ratings| {
        ratings[0]["pressed"] == "Good response" && ratings[0]["details"] == "Add details"
    });

    tab.call(
        "Fetch.enable",
        serde_json::json!({ "patterns": [{ "urlPattern": "*/physician-state/feedback/*" }] }),
    );
    tab.step("clickInRating", serde_json::json!([1, "Poor response"]));
    let save = tab.held_request("PUT");
    tab.call(
        "Fetch.failRequest",
        serde_json::json!({ "requestId": save["requestId"], "errorReason": "Failed" }),
    );
    ratings_when(&mut tab, "the failed save says so", |ratings| {
        ratings[1]["notice"] == "Couldn’t save" && ratings[1]["pressed"].is_null()
    });
    tab.save_pictures("feedback-not-saved", "[aria-label=\"Conversation\"]");
    // A save that answers after the physician left the chat still settles it.
    tab.step("clickInRating", serde_json::json!([1, "Poor response"]));
    let retry = tab.held_request("PUT");
    tab.click_named(".sidebar-nav-item", "Today");
    let left = Instant::now();
    while tab
        .step("readText", serde_json::json!(["main.glass-stage"]))
        .as_str()
        .is_some_and(|stage| stage.contains("Sodium was 139 on Monday."))
    {
        assert!(
            left.elapsed() < Duration::from_secs(5),
            "Today left the chat on screen"
        );
        std::thread::sleep(Duration::from_millis(60));
    }
    tab.release(&retry);
    tab.call("Fetch.disable", serde_json::json!({}));
    tab.release_all();
    tab.click(r#"button[aria-label="Find patient"]"#);
    tab.browser.type_into(".search-field input", "Tanvir");
    tab.wait_for_text("DOB", Duration::from_secs(5));
    tab.click(".dialog-results button");
    tab.wait_for_text("Sodium was 139 on Monday.", Duration::from_secs(10));
    ratings_when(&mut tab, "the retried save landed", |ratings| {
        ratings[1]["pressed"] == "Poor response"
            && ratings[1]["notice"].is_null()
            && ratings[1]["details"] == "Add details"
    });

    tab.step("clickInRating", serde_json::json!([1, "Add details"]));
    tab.wait_for_text(
        "What would have made this response better?",
        Duration::from_secs(5),
    );
    tab.click("dialog.feedback-details button[aria-label=\"Close\"]");
    let closed = Instant::now();
    while !tab.dialog_text().is_empty() {
        assert!(
            closed.elapsed() < Duration::from_secs(5),
            "Close left the dialog open"
        );
        std::thread::sleep(Duration::from_millis(60));
    }
    ratings_when(&mut tab, "closing the dialog kept the rating", |ratings| {
        ratings[1]["pressed"] == "Poor response" && ratings[1]["details"] == "Add details"
    });
    // A details save that answers after its dialog closed still lands.
    tab.step("clickInRating", serde_json::json!([1, "Add details"]));
    tab.wait_for_text(
        "What would have made this response better?",
        Duration::from_secs(5),
    );
    tab.click_named("dialog.feedback-details .toggle-chip", "Not useful");
    tab.call(
        "Fetch.enable",
        serde_json::json!({ "patterns": [{ "urlPattern": "*/physician-state/feedback/*" }] }),
    );
    tab.click_named("dialog.feedback-details button", "Save details");
    let held = tab.held_request("PUT");
    tab.click("dialog.feedback-details button[aria-label=\"Close\"]");
    let closed = Instant::now();
    while !tab.dialog_text().is_empty() {
        assert!(
            closed.elapsed() < Duration::from_secs(5),
            "Close left the dialog open while its save was on its way"
        );
        std::thread::sleep(Duration::from_millis(60));
    }
    tab.release(&held);
    tab.call("Fetch.disable", serde_json::json!({}));
    tab.release_all();
    ratings_when(
        &mut tab,
        "the details saved after the dialog closed",
        |ratings| {
            ratings[1]["pressed"] == "Poor response"
                && ratings[1]["details"] == "Edit details"
                && ratings[1]["notice"].is_null()
        },
    );

    tab.step("clickInRating", serde_json::json!([0, "Add details"]));
    tab.wait_for_text(
        "What would have made this response better?",
        Duration::from_secs(5),
    );
    tab.click_named("dialog.feedback-details .toggle-chip", "Missing context");
    tab.browser
        .type_into("dialog.feedback-details textarea", "It left out the trend");
    // Keys pressed in the details dialog are its own, never the page's shortcuts.
    const DETAILS: &str = "dialog.feedback-details textarea";
    tab.step("focusEnd", serde_json::json!([DETAILS]));
    tab.press_key(Key::typing("?", "Slash", 191));
    assert_eq!(
        tab.step("valueSoon", serde_json::json!([DETAILS])),
        "It left out the trend?",
        "a key typed into the details went missing"
    );
    tab.step(
        "keyOn",
        serde_json::json!(["dialog.feedback-details button[aria-label=\"Close\"]", "?"]),
    );
    std::thread::sleep(Duration::from_millis(150));
    assert_eq!(
        tab.step("dialogsSaying", serde_json::json!(["Keyboard shortcuts"])),
        0,
        "a key pressed in the details dialog opened the page's shortcuts"
    );
    // A conversation reread that fails keeps the conversation, marked stale,
    // and the open dialog with its unsaved words.
    sites[0].kill_now();
    tab.wait_for_text(
        "Coda couldn’t refresh this conversation.",
        Duration::from_secs(20),
    );
    ratings_when(
        &mut tab,
        "the stale conversation keeps its ratings",
        |ratings| ratings.len() == 2 && ratings[0]["pressed"] == "Good response",
    );
    assert_eq!(
        tab.step("valueSoon", serde_json::json!([DETAILS])),
        "It left out the trend?",
        "a failed conversation reread lost the dialog's words"
    );
    sites[0].restart();
    scale::wait_ready(
        &mut sites[0],
        hosts[0].rsplit_once(':').unwrap().1.parse().unwrap(),
    )
    .await;
    let back = Instant::now();
    while tab
        .dom()
        .contains("Coda couldn’t refresh this conversation.")
    {
        assert!(
            back.elapsed() < Duration::from_secs(30),
            "the conversation stayed stale after the site came back"
        );
        std::thread::sleep(Duration::from_millis(100));
    }
    assert_eq!(
        tab.step("valueSoon", serde_json::json!([DETAILS])),
        "It left out the trend?",
        "the reread after recovery lost the dialog's words"
    );
    tab.save_pictures("feedback-details", "dialog.feedback-details");
    tab.click_named("dialog.feedback-details button", "Save details");
    ratings_when(&mut tab, "the details saved", |ratings| {
        ratings[0]["pressed"] == "Good response" && ratings[0]["details"] == "Edit details"
    });
    // A save refused for an ended session, while every read still answers,
    // covers the page with the sign-in prompt and clears the patient.
    tab.call(
        "Fetch.enable",
        serde_json::json!({ "patterns": [{ "urlPattern": "*/physician-state/feedback/*" }] }),
    );
    tab.step("clickInRating", serde_json::json!([0, "Poor response"]));
    let refused = tab.held_request("PUT");
    tab.call(
        "Fetch.fulfillRequest",
        serde_json::json!({ "requestId": refused["requestId"], "responseCode": 401 }),
    );
    tab.call("Fetch.disable", serde_json::json!({}));
    tab.release_all();
    let ended = tab.wait_for_text(
        "Your session ended. Sign in to continue.",
        Duration::from_secs(10),
    );
    assert!(
        !ended.contains("Sodium was 139 on Monday."),
        "a refused feedback save left the patient on screen:\n{ended}"
    );
    drop(tab);

    for (process, host) in sites.iter_mut().zip(&hosts) {
        process.restart();
        scale::wait_ready(process, host.rsplit_once(':').unwrap().1.parse().unwrap()).await;
    }
    let session = scale::sign_in(&hosts[0]).await;
    let saved = snapshot_feedback(&hosts[1], &session, patient, chat).await;
    // The first response: rated, then details. The second: a failed save,
    // the retry, then details.
    assert_eq!(
        saved,
        [
            (
                serde_json::json!(first),
                serde_json::json!({
                    "revision": 2,
                    "rating": "up",
                    "details": {"reasons": ["missing_context"], "text": "It left out the trend?"},
                })
            ),
            (
                serde_json::json!(second),
                serde_json::json!({
                    "revision": 2,
                    "rating": "down",
                    "details": {"reasons": ["not_useful"], "text": ""},
                })
            ),
        ]
    );
    println!("Both site processes restarted; snapshot feedback: {saved:?}");
    let mut fresh = Tab::open(&urls[1]);
    fresh.wait_for_text("Work email", Duration::from_secs(20));
    fresh.request_sign_in("physician@codahealth.test");
    fresh.wait_for_text("Sodium was 139 on Monday.", Duration::from_secs(20));
    ratings_when(
        &mut fresh,
        "each rating stayed on its response",
        |ratings| {
            ratings.len() == 2
                && ratings[0]["pressed"] == "Good response"
                && ratings[0]["details"] == "Edit details"
                && ratings[1]["pressed"] == "Poor response"
                && ratings[1]["details"] == "Edit details"
        },
    );
    fresh.save_pictures("feedback-after-restart", "[aria-label=\"Conversation\"]");

    // On a phone the controls and the details dialog fit the screen and work.
    // A phone opens with the sidebar closed, so load the chat afresh at its size.
    fresh.set_viewport(390, 844, true);
    fresh.reload();
    fresh.wait_for_text("Sodium was 139 on Monday.", Duration::from_secs(20));
    ratings_when(&mut fresh, "the phone shows the saved ratings", |ratings| {
        ratings.len() == 2 && ratings[0]["pressed"] == "Good response"
    });
    const CONTROLS: &str = ".message-actions[aria-label=\"Response actions\"]";
    assert_eq!(
        fresh.step("outsideViewport", serde_json::json!([CONTROLS])),
        serde_json::json!([]),
        "a response's feedback controls run off a phone's screen"
    );
    let shell = fresh.step("readShell", serde_json::json!([]));
    assert_eq!(
        shell["pageWidth"], 390,
        "the page is wider than a phone: {shell}"
    );
    fresh.save_picture("feedback-phone-controls");
    fresh.step("clickInRating", serde_json::json!([0, "Poor response"]));
    ratings_when(&mut fresh, "the phone rating saved", |ratings| {
        ratings[0]["pressed"] == "Poor response"
            && ratings[0]["enabled"] == true
            && ratings[0]["details"] == "Edit details"
    });
    fresh.step("clickInRating", serde_json::json!([0, "Edit details"]));
    fresh.wait_for_text(
        "What would have made this response better?",
        Duration::from_secs(5),
    );
    assert_eq!(
        fresh.step(
            "outsideViewport",
            serde_json::json!(["dialog.feedback-details"])
        ),
        serde_json::json!([]),
        "the details dialog runs off a phone's screen"
    );
    assert_eq!(
        fresh.step(
            "outsideViewport",
            serde_json::json!([
                "dialog.feedback-details button.control-primary, dialog.feedback-details textarea"
            ]),
        ),
        serde_json::json!([]),
        "the dialog's words and Save sit off a phone's screen"
    );
    fresh.save_picture("feedback-phone-details");
    fresh.click("dialog.feedback-details button[aria-label=\"Close\"]");
    let closed = Instant::now();
    while !fresh.dialog_text().is_empty() {
        assert!(
            closed.elapsed() < Duration::from_secs(5),
            "Close left the phone dialog open"
        );
        std::thread::sleep(Duration::from_millis(60));
    }
    let phone = snapshot_feedback(&hosts[0], &session, patient, chat).await;
    let rerated = &phone[0].1;
    assert_eq!(phone[0].0, first, "{phone:?}");
    assert_eq!(
        (&rerated["rating"], &rerated["revision"]),
        (&serde_json::json!("down"), &serde_json::json!(3))
    );
    assert_eq!(
        rerated["details"]["text"], "It left out the trend?",
        "a new rating on a phone kept the saved details"
    );

    // What a reread found stays when a later one can't read feedback: a newer
    // revision saved on another device and an open dialog's unsaved words.
    fresh.set_viewport(1280, 800, true);
    let http = reqwest::Client::new();
    let reread = || {
        http.put(format!(
            "http://{}/auth/panels/staging-physician/{patient}",
            hosts[1]
        ))
        .bearer_auth(&session)
        .send()
    };
    let elsewhere = serde_json::json!({
        "rating": "up",
        "details": {"reasons": ["hard_to_follow"], "text": "Saved on another device."},
    });
    let (status, body) = scale::exchange(
        &hosts[0],
        "PUT",
        &format!("/physician-state/feedback/{patient}/{chat}/{first}"),
        &[
            ("content-type", "application/json"),
            ("authorization", &format!("Bearer {session}")),
        ],
        Some(&serde_json::to_vec(&elsewhere).unwrap()),
    )
    .await
    .unwrap();
    assert_eq!(status, 200, "{body}");
    // A panel change announces the patient, so the open chat is read again.
    assert!(reread().await.unwrap().status().is_success());
    ratings_when(
        &mut fresh,
        "the other device's rating was read",
        |ratings| ratings[0]["pressed"] == "Good response" && ratings[0]["notice"].is_null(),
    );
    fresh.step("clickInRating", serde_json::json!([0, "Edit details"]));
    fresh.wait_for_text(
        "What would have made this response better?",
        Duration::from_secs(5),
    );
    assert_eq!(
        fresh.step("valueSoon", serde_json::json!([DETAILS])),
        "Saved on another device."
    );
    fresh
        .browser
        .type_into(DETAILS, "Not saved yet: the trend matters");
    scale::corrupt_feedback_details_on(&database.url, second).await;
    assert!(reread().await.unwrap().status().is_success());
    let failed = Instant::now();
    while !sites[1]
        .tail()
        .contains("physician chat snapshot feedback read failed")
    {
        assert!(
            failed.elapsed() < Duration::from_secs(10),
            "the reread never found feedback unavailable"
        );
        std::thread::sleep(Duration::from_millis(60));
    }
    std::thread::sleep(Duration::from_millis(300));
    ratings_when(
        &mut fresh,
        "unavailable feedback kept what earlier reads found",
        |ratings| {
            ratings.len() == 2
                && ratings[0]["pressed"] == "Good response"
                && ratings[1]["pressed"] == "Poor response"
                && ratings
                    .iter()
                    .all(|rating| rating["enabled"] == true && rating["notice"].is_null())
        },
    );
    assert_eq!(
        fresh.step("valueSoon", serde_json::json!([DETAILS])),
        "Not saved yet: the trend matters",
        "an unavailable reread closed the dialog and lost its words"
    );
    fresh.save_pictures("feedback-kept-when-unavailable", "dialog.feedback-details");
    fresh.click("dialog.feedback-details button[aria-label=\"Close\"]");
    // A thumb then keeps the newest details read, not an older local save's.
    fresh.step("clickInRating", serde_json::json!([0, "Poor response"]));
    ratings_when(&mut fresh, "the thumb saved over the read", |ratings| {
        ratings[0]["pressed"] == "Poor response" && ratings[0]["notice"].is_null()
    });
    // The page shows the pressed thumb while its save is in flight, so wait for the row.
    let pressed = Instant::now();
    let rows = loop {
        let rows = scale::feedback_rows(&database.url).await;
        let landed = rows
            .iter()
            .any(|row| row["response"] == first && row["revision"] == 5);
        if landed || pressed.elapsed() > Duration::from_secs(10) {
            break rows;
        }
        tokio::time::sleep(Duration::from_millis(60)).await;
    };
    let row = rows.iter().find(|row| row["response"] == first);
    assert_eq!(
        row,
        Some(&serde_json::json!({
            "response": first,
            "revision": 5,
            "rating": "down",
            "details": elsewhere["details"],
        })),
        "{rows:?}"
    );

    // Feedback the site can't read leaves the chat usable: the conversation
    // shows, current, and each response's thumbs are disabled and say why.
    scale::corrupt_feedback_details(&database.url).await;
    fresh.set_viewport(1280, 800, true);
    fresh.reload();
    fresh.wait_for_text("Sodium was 139 on Monday.", Duration::from_secs(20));
    ratings_when(
        &mut fresh,
        "unreadable feedback disables each response's thumbs",
        |ratings| {
            ratings.len() == 2
                && ratings.iter().all(|rating| {
                    rating["enabled"] == false
                        && rating["pressed"].is_null()
                        && rating["details"].is_null()
                        && rating["notice"] == "Couldn’t load your rating"
                })
        },
    );
    let page = fresh.dom();
    assert!(
        page.contains("Potassium was 4.1 on Monday.")
            && !page.contains("Coda couldn’t refresh this conversation."),
        "unreadable feedback kept the conversation from showing as current"
    );
    fresh.type_composer("Still writing to Coda");
    fresh.wait_saved();
    fresh.save_pictures("feedback-unavailable", "[aria-label=\"Conversation\"]");
    drop(fresh);
    drop(sites);
    fs::remove_dir_all(home).unwrap();
}

/// Deterministic Find patient ranking, clinic-demographic wire & DOM isolation,
/// live panel removal, per-source standing & failed refresh wording, and
/// `SEARCH_UNAVAILABLE` failure across desktop, phone (`<900px`), and
/// `prefers-reduced-motion`.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore = "browser qualification; run explicitly with --ignored"]
async fn find_patient_and_patient_page_enforce_practice_panel_and_source_states() {
    use front_door::MemoryFrontDoorStore;
    use local_product::physician_sources::{MedicalRecordNumber, SourcePatient};
    use physician_auth::PracticeClock;
    use physician_sample::patients::{
        MemoryPatients, PatientDirectory, PatientSourceRecord, SourceDeliveryState,
    };

    let _guard = test_lock().lock().await;
    let site = build_site();
    unsafe {
        env::set_var("PHYSICIAN_SITE_DIR", &site);
        env::set_var("PHYSICIAN_AUTH", "required");
        env::set_var("PHYSICIAN_AUTH_PROOF_TOKENS", "1");
        env::remove_var("PHYSICIAN_UPSTREAM");
    }

    let auth_store = Arc::new(MemoryAuthStore::new());
    auth_store.seed_practice("practice-clinic-a", PracticeClock::Fixed(1_787_846_400));
    auth_store.seed_practice("practice-clinic-b", PracticeClock::Fixed(1_787_846_400));
    auth_store.seed_account(
        "practice-clinic-a",
        "phys-admin",
        "admin@clinic-a.codahealth.test",
        "Dr. Clinic Admin",
        true,
    );
    let admin_identity = PhysicianIdentity {
        practice_id: "practice-clinic-a".to_owned(),
        physician_id: "phys-admin".to_owned(),
    };

    let tanvir = physician_sample::WORKSPACE_IDS[0];
    let jake_other_practice = physician_sample::WORKSPACE_IDS[1];
    let mary_off_panel = physician_sample::WORKSPACE_IDS[4];
    let clara_partial = "pw-partial-identity";
    let rank_sub = "pw-rank-sub";
    let rank_prefix = "pw-rank-prefix";
    let rank_exact = "pw-rank-exact";

    for workspace in [
        tanvir,
        mary_off_panel,
        clara_partial,
        rank_sub,
        rank_prefix,
        rank_exact,
    ] {
        auth_store
            .register_patient("practice-clinic-a", workspace)
            .await
            .unwrap();
    }
    auth_store
        .register_patient("practice-clinic-b", jake_other_practice)
        .await
        .unwrap();

    for workspace in [tanvir, clara_partial, rank_sub, rank_prefix, rank_exact] {
        auth_store
            .assign_panel(
                &admin_identity,
                "phys-admin",
                workspace,
                Membership::Allowed,
            )
            .await
            .unwrap();
    }

    let make_record = |id: &str,
                       source_id: &str,
                       source_label: &str,
                       name: &str,
                       dob: Option<&str>,
                       gender: Option<&str>,
                       mrn: Option<&str>,
                       delivery: SourceDeliveryState| PatientSourceRecord {
        source_id: source_id.to_owned(),
        source_label: source_label.to_owned(),
        patient: SourcePatient {
            source_patient_id: id.to_owned(),
            display_name: name.to_owned(),
            birth_date: dob.map(str::to_owned),
            sex_or_gender: gender.map(str::to_owned),
            medical_record_numbers: mrn
                .into_iter()
                .map(|value| MedicalRecordNumber {
                    system: "Practice MRN".to_owned(),
                    value: value.to_owned(),
                })
                .collect(),
        },
        delivery,
    };

    let custom_patients = Arc::new(MemoryPatients::default());
    // Tanvir: on panel, NO Duet account (front_door stays empty), saved summary,
    // one usable source + one unpulled source (must NOT show as usable!).
    custom_patients.set_source_records(
        tanvir,
        vec![
            make_record(
                tanvir,
                "hint-health",
                "Hint Health",
                "Tanvir Chowdhury",
                Some("1964-11-02"),
                Some("Male"),
                Some("00000003"),
                SourceDeliveryState::Usable {
                    pulled_at: 1_787_846_400,
                },
            ),
            make_record(
                tanvir,
                "quest-labs",
                "Quest Diagnostics",
                "Tanvir Chowdhury",
                Some("1964-11-02"),
                Some("Male"),
                Some("00000003"),
                SourceDeliveryState::NeverPulled,
            ),
        ],
    );
    custom_patients.set_synopsis(
        tanvir,
        serde_json::json!({
            "version_id": "syn-tanvir-1",
            "authored_text": "Stage 3 chronic kidney disease; home blood pressure stable this month."
        }),
    );

    // Mary Louis: same-practice, initially off panel (`ClinicDemographic`), with saved
    // synopsis that must NEVER leak until assigned to panel.
    custom_patients.set_source_records(
        mary_off_panel,
        vec![make_record(
            mary_off_panel,
            "fixture-chart",
            "Fixture chart",
            "Mary Louis",
            Some("1972-03-18"),
            Some("Female"),
            Some("00000005"),
            SourceDeliveryState::Usable {
                pulled_at: 1_787_846_400,
            },
        )],
    );
    custom_patients.set_synopsis(
        mary_off_panel,
        serde_json::json!({
            "version_id": "syn-mary-1",
            "authored_text": "Persistent atrial fibrillation on apixaban; rate controlled."
        }),
    );

    // Clara Jenks: missing DOB and missing MRN stay missing (never fabricated).
    custom_patients.set_source_records(
        clara_partial,
        vec![make_record(
            clara_partial,
            "hint-health",
            "Hint Health",
            "Clara Jenks",
            None,
            None,
            None,
            SourceDeliveryState::Usable {
                pulled_at: 1_787_846_400,
            },
        )],
    );

    // Deterministic search ranking fixtures for query `"marlo"`.
    custom_patients.set_source_records(
        rank_sub,
        vec![make_record(
            rank_sub,
            "hint-health",
            "Hint Health",
            "Elena Marlowe",
            Some("1981-04-10"),
            Some("Female"),
            Some("880100"),
            SourceDeliveryState::Usable {
                pulled_at: 1_787_846_400,
            },
        )],
    );
    custom_patients.set_source_records(
        rank_prefix,
        vec![make_record(
            rank_prefix,
            "hint-health",
            "Hint Health",
            "Marlo Vance",
            Some("1979-09-14"),
            Some("Male"),
            Some("880200"),
            SourceDeliveryState::Usable {
                pulled_at: 1_787_846_400,
            },
        )],
    );
    custom_patients.set_source_records(
        rank_exact,
        vec![make_record(
            rank_exact,
            "hint-health",
            "Hint Health",
            "Devon Brooks",
            Some("1990-01-22"),
            Some("Male"),
            Some("MARLO"),
            SourceDeliveryState::Usable {
                pulled_at: 1_787_846_400,
            },
        )],
    );

    // Other-practice patient (must never appear in Practice A search or open).
    custom_patients.set_source_records(
        jake_other_practice,
        vec![make_record(
            jake_other_practice,
            "hint-health",
            "Hint Health",
            "Jake Smith",
            Some("1988-05-12"),
            Some("Male"),
            Some("00000001"),
            SourceDeliveryState::Usable {
                pulled_at: 1_787_846_400,
            },
        )],
    );

    let directory_ext: Arc<dyn PatientDirectory> = custom_patients.clone();
    let base_router = physician_sample::fixture_site_with_feed(physician_sample::SampleFeed::new())
        .layer(axum::Extension(directory_ext));
    let front_door = Arc::new(MemoryFrontDoorStore::new());
    let app = physician_sample::authed_router_on(base_router, auth_store.clone(), front_door).await;
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    let server_task = tokio::spawn(async move { axum::serve(listener, app).await.unwrap() });
    wait_until_ready(address).await;

    let mut browser = Tab::open(&format!("http://{address}/"));
    browser.set_viewport(1280, 800, false);
    browser.wait_for_text("Sign in to Coda", Duration::from_secs(20));
    browser.request_sign_in("admin@clinic-a.codahealth.test");
    browser.wait_for_text("Dr. Clinic Admin", Duration::from_secs(15));

    let bearer = browser
        .eval("localStorage.getItem('coda.physician.session')")
        .as_str()
        .expect("bearer session")
        .to_owned();
    let http = reqwest::Client::new();

    // AC1: Deterministic search order across MRN, DOB, and display name; other-practice never appears.
    browser.click("button[aria-label=\"Find patient\"]");
    browser.search_for("marlo");
    browser.wait_for_text("Devon Brooks", Duration::from_secs(10));
    let result_labels = browser.eval(
        "[...document.querySelectorAll('.dialog-results button')].map(b => b.innerText.replace(/\\s+/g, ' ').trim())",
    );
    let labels: Vec<String> = result_labels
        .as_array()
        .unwrap()
        .iter()
        .map(|v| v.as_str().unwrap().to_owned())
        .collect();
    assert_eq!(
        labels.len(),
        3,
        "expected 3 deterministic matches: {labels:?}"
    );
    assert!(
        labels[0].contains("Devon Brooks") && labels[0].contains("MRN MARLO"),
        "exact MRN match must rank first: {labels:?}"
    );
    assert!(
        labels[1].contains("Marlo Vance") && labels[1].contains("MRN 880200"),
        "full-name prefix must rank second: {labels:?}"
    );
    assert!(
        labels[2].contains("Elena Marlowe") && labels[2].contains("MRN 880100"),
        "token/substring match must rank third: {labels:?}"
    );
    std::thread::sleep(Duration::from_millis(250));
    browser.save_picture("find-patient-ranked-desktop");

    // Cross-practice patient Jake Smith never appears in search.
    browser.search_for("Jake");
    browser.wait_for_text("No patients match.", Duration::from_secs(10));
    assert!(!browser.dialog_text().contains("Jake Smith"));

    // Open non-Duet patient Tanvir Chowdhury via search by MRN (`00000003`).
    browser.search_for("00000003");
    browser.wait_for_text("Tanvir Chowdhury", Duration::from_secs(10));
    browser.click(".dialog-results button");
    let tanvir_dom = browser.wait_for_text(
        "Stage 3 chronic kidney disease; home blood pressure stable this month.",
        Duration::from_secs(10),
    );
    // Non-Duet patient opens cleanly; supplied DOB + MRN labels show;
    // saved summary renders; Hint Health shows usable while Quest Diagnostics
    // (no saved successful read) shows Waiting for first read, NEVER usable.
    assert!(
        tanvir_dom.contains("DOB Nov 2, 1964")
            && tanvir_dom.contains("MRN 00000003")
            && tanvir_dom.contains("Patient summary")
            && tanvir_dom.contains("Hint Health · As of Today, 12:00 PM")
            && tanvir_dom.contains("Quest Diagnostics · Waiting for first read")
            && !tanvir_dom.contains("Quest Diagnostics · As of"),
        "unexpected patient page identity/summary/source state:\n{tanvir_dom}"
    );

    // Simulate failed refresh on Hint Health keeping last good read time (11:00 AM EDT).
    custom_patients.set_source_records(
        tanvir,
        vec![
            make_record(
                tanvir,
                "hint-health",
                "Hint Health",
                "Tanvir Chowdhury",
                Some("1964-11-02"),
                Some("Male"),
                Some("00000003"),
                SourceDeliveryState::RefreshFailed {
                    last_good_pulled_at: Some(1_787_842_800),
                },
            ),
            make_record(
                tanvir,
                "quest-labs",
                "Quest Diagnostics",
                "Tanvir Chowdhury",
                Some("1964-11-02"),
                Some("Male"),
                Some("00000003"),
                SourceDeliveryState::NeverPulled,
            ),
        ],
    );
    browser.reload();
    let stale_dom = browser.wait_for_text(
        "Records as of Today, 11:00 AM, couldn't update",
        Duration::from_secs(15),
    );
    assert!(
        stale_dom.contains("Hint Health · As of Today, 11:00 AM, couldn't update")
            && stale_dom.contains("Quest Diagnostics · Waiting for first read"),
        "failed refresh must preserve last good read time and say couldn't update:\n{stale_dom}"
    );
    browser.save_picture("patient-stale-source-desktop");

    // Phone viewport (<900px) + prefers-reduced-motion check with zero horizontal overflow.
    browser.set_viewport(390, 844, true);
    browser.wait_for_text(
        "Records as of Today, 11:00 AM, couldn't update",
        Duration::from_secs(10),
    );
    let phone_shell = browser.step("readShell", serde_json::json!([]));
    assert_eq!(
        phone_shell["pageWidth"], 390,
        "patient page overflowed phone viewport: {phone_shell}"
    );
    browser.save_picture("patient-stale-source-phone");
    browser.set_viewport(1280, 800, false);

    // Missing DOB and MRN stay missing on Clara Jenks.
    browser.navigate(&format!("http://{address}/?patient={clara_partial}"));
    let clara_dom = browser.wait_for_text("Clara Jenks", Duration::from_secs(10));
    let facts_text = browser
        .eval("document.querySelector('.patient-facts')?.innerText ?? ''")
        .as_str()
        .unwrap_or_default()
        .to_owned();
    assert!(
        !facts_text.contains("DOB") && !facts_text.contains("MRN") && !facts_text.contains("Age"),
        "missing DOB and MRN must stay missing:\n{facts_text}\n{clara_dom}"
    );

    // Same-practice patient outside physician's panel (`mary_off_panel`) exposes
    // ONLY name, DOB, and MRN in both wire JSON and screen DOM.
    let wire_off_panel: serde_json::Value = http
        .get(format!(
            "http://{address}/patient-workspaces/{mary_off_panel}"
        ))
        .bearer_auth(&bearer)
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();
    assert_eq!(wire_off_panel["membership"], "clinic_demographic");
    assert_eq!(wire_off_panel["display_name"], "Mary Louis");
    assert!(wire_off_panel["patient_synopsis"].is_null());
    assert_eq!(wire_off_panel["connected_sources"], serde_json::json!([]));
    assert!(wire_off_panel["identity"].get("sex_or_gender").is_none());
    assert!(wire_off_panel["identity"].get("age_display").is_none());

    browser.navigate(&format!("http://{address}/?patient={mary_off_panel}"));
    let off_panel_dom =
        browser.wait_for_text("Clinic record · Not on your panel", Duration::from_secs(10));
    assert!(
        off_panel_dom.contains("Mary Louis")
            && off_panel_dom.contains("Mar 18, 1972")
            && off_panel_dom.contains("00000005")
            && !off_panel_dom.contains("54 years")
            && !off_panel_dom.contains("Female")
            && !off_panel_dom.contains("Records as of")
            && !off_panel_dom.contains("Persistent atrial fibrillation"),
        "outside-panel view leaked clinical or extra fields:\n{off_panel_dom}"
    );
    browser.save_picture("patient-outside-panel-restricted");

    // Assign Mary to panel -> clinical summary unlocks; remove panel access
    // while open -> clinical content closes immediately.
    browser.click("button.assign-panel-button");
    let mary_on_panel_dom = browser.wait_for_text(
        "Persistent atrial fibrillation on apixaban; rate controlled.",
        Duration::from_secs(10),
    );
    assert!(mary_on_panel_dom.contains("Records as of"));
    browser.click("button.remove-panel-button");
    let closed_live_dom =
        browser.wait_for_text("Clinic record · Not on your panel", Duration::from_secs(10));
    assert!(
        !closed_live_dom.contains("Persistent atrial fibrillation")
            && !closed_live_dom.contains("Records as of"),
        "clinical content did not close immediately when panel access was removed:\n{closed_live_dom}"
    );
    browser.save_picture("patient-panel-removed-live");

    // Failed lookup shows SEARCH_UNAVAILABLE failure, NEVER `"No patients match."`.
    custom_patients.set_unavailable(true);
    browser.click("button[aria-label=\"Find patient\"]");
    browser.search_for("Mary");
    browser.wait_for_text("Coda couldn’t search patients.", Duration::from_secs(10));
    let failed_dialog = browser.dialog_text();
    assert!(
        failed_dialog.contains("Try again") && !failed_dialog.contains("No patients match."),
        "failed patient search must never read as empty:\n{failed_dialog}"
    );
    std::thread::sleep(Duration::from_millis(250));
    browser.save_picture("find-patient-search-unavailable");

    server_task.abort();
    unsafe {
        env::remove_var("PHYSICIAN_AUTH");
        env::remove_var("PHYSICIAN_AUTH_PROOF_TOKENS");
    }
}

/// Durable Postgres conversation & work history across server restart:
/// exact saved answer, Basis, and source citation readback, content-distinguished
/// lists, newer-records marker on old answers without altering saved text, and
/// post-load access revocation refusal across desktop and phone.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore = "browser and disposable Postgres qualification; run explicitly with --ignored"]
async fn past_conversations_and_work_reopen_across_restart_mark_newer_records_and_enforce_access() {
    use harness::engine::AttemptFailure;
    use local_product::physician_sources::{
        MedicalRecordNumber, READ_TOOL, ReadablePart, RecordDate, SEARCH_TOOL, SourcePatient,
        SourceRevision,
    };
    use local_product::runtime::sources::SourcePull;
    use local_product::runtime::{Input, PhysicianHost, Scope, ShownSteps, Store, engine};
    use physician_auth::PostgresAuthStore;

    struct CitedAnswerProvider {
        round: usize,
        candidate_id: String,
    }

    impl engine::recipe::Provider for CitedAnswerProvider {
        async fn exchange(
            &mut self,
            _: &engine::recipe::Request,
            _: &engine::Cancellation,
            _: &engine::RunLimits,
            _: &mut dyn engine::recipe::Observer,
        ) -> Result<engine::Turn, engine::Cause> {
            let items = if self.round == 0 {
                self.round += 1;
                vec![
                    engine::Item::Call(engine::Invocation {
                        id: "call-search-1".to_owned(),
                        name: SEARCH_TOOL.to_owned(),
                        arguments: serde_json::json!({
                            "query": "potassium",
                            "record_type": null,
                            "after": null,
                        })
                        .to_string(),
                    }),
                    engine::Item::Call(engine::Invocation {
                        id: "call-read-1".to_owned(),
                        name: READ_TOOL.to_owned(),
                        arguments: serde_json::json!({
                            "references": [{
                                "candidate_id": self.candidate_id,
                                "revision": "rev-1",
                                "cursor": null,
                            }],
                            "max_result_bytes": 16_384,
                        })
                        .to_string(),
                    }),
                ]
            } else {
                vec![engine::Item::Call(engine::Invocation {
                    id: "call-finish-1".to_owned(),
                    name: local_product::physician_sources::citations::FINISH_TOOL.to_owned(),
                    arguments: serde_json::json!({
                        "answer": "Potassium is 5.4 mmol/L on Tuesday's panel [S1].",
                        "quotes": [{"label": "S1", "quote": "Potassium 5.4 mmol/L"}],
                        "descriptions": [],
                        "basis": "Based on Regional Lab Basic metabolic panel [S1]; hemolysis status was not reported.",
                        "cites_none": false,
                    })
                    .to_string(),
                })]
            };
            Ok(engine::Turn {
                items,
                receipt: serde_json::Value::Null,
                usage: Default::default(),
            })
        }
    }

    struct TestOpenAuthority;

    #[local_product::runtime::async_trait]
    impl local_product::runtime::Authority for TestOpenAuthority {
        async fn check(
            &mut self,
            _: &local_product::runtime::PublicationTransaction,
            _: &Scope,
            _: Option<&[u8]>,
        ) -> Result<(), String> {
            Ok(())
        }

        async fn lock_publication(
            &mut self,
            _: &local_product::runtime::PublicationTransaction,
            _: &Scope,
            _: Option<&[u8]>,
        ) -> Result<(), String> {
            Ok(())
        }
    }

    let _guard = test_lock().lock().await;
    let database = scale::Database::create("history_reopen_browser")
        .await
        .expect("browser proof database");
    let site = build_site();
    unsafe {
        env::set_var("PHYSICIAN_SITE_DIR", &site);
        env::set_var("PHYSICIAN_AUTH", "required");
        env::set_var("PHYSICIAN_AUTH_PROOF_TOKENS", "1");
        env::remove_var("PHYSICIAN_UPSTREAM");
    }
    scale::staging_account(&database.url).await;

    let patient_a = physician_sample::WORKSPACE_IDS[0];
    let patient_b = physician_sample::WORKSPACE_IDS[1];
    let store = Store::connect(&database.url).await.expect("runtime store");

    // Seed real retained clinical record v1 on patient_a so tool reads produce real receipts.
    let raw_lab_v1 = r#"{"test":"Basic metabolic panel","potassium_mmol_l":5.4}"#;
    let pull_v1 = SourcePull {
        source_id: "fixture-chart".to_owned(),
        source_label: "Regional Lab".to_owned(),
        patient: SourcePatient {
            source_patient_id: patient_a.to_owned(),
            display_name: "Tanvir Chowdhury".to_owned(),
            birth_date: Some("1964-11-02".to_owned()),
            sex_or_gender: Some("Male".to_owned()),
            medical_record_numbers: vec![MedicalRecordNumber {
                system: "Practice MRN".to_owned(),
                value: "00000003".to_owned(),
            }],
        },
        pulled_at: Some(1_787_846_400),
    };
    let rev_v1 = SourceRevision {
        workspace: patient_a.to_owned(),
        source_id: "fixture-chart".to_owned(),
        candidate_id: "bmp-2026-08-27".to_owned(),
        revision: "rev-1".to_owned(),
        record_type: "Laboratory result".to_owned(),
        dates: vec![RecordDate {
            name: "Collected".to_owned(),
            value: "2026-08-27".to_owned(),
        }],
        content: raw_lab_v1.to_owned(),
        sha256: local_product::sha256_hex(raw_lab_v1.as_bytes()),
        readable_parts: vec![ReadablePart {
            label: "Basic metabolic panel".to_owned(),
            text: "Potassium 5.4 mmol/L (reference 3.5-5.0 mmol/L), collected Tuesday morning."
                .to_owned(),
        }],
        provenance: serde_json::json!({
            "source_label": "Regional Lab",
            "title": "Basic metabolic panel",
        }),
        envelope: None,
    };
    store
        .record_chart(patient_a, &[rev_v1], &pull_v1)
        .await
        .expect("record initial chart");

    let skills = local_product::physician_chat::PhysicianSkills::configured(None, false).unwrap();
    let chat_cited = "chat-potassium-cited";
    let scope_cited = Scope {
        practice: physician_sample::STAGING_PRACTICE_ID.into(),
        physician: "staging-physician".into(),
        workspace: patient_a.into(),
        chat: chat_cited.into(),
    };
    store
        .accept_authorized(
            &scope_cited,
            &Input {
                key: "turn-potassium-1".into(),
                text: "What was Tuesday's potassium and its basis?".into(),
                timezone: "America/New_York".into(),
                scenario: None,
            },
            vec![7; 32],
            &skills,
            &scale::models::chat_recipe(),
            None,
        )
        .await
        .unwrap();
    let claim = store.claim(&scope_cited, 60).await.unwrap().unwrap();
    PhysicianHost {
        store: store.clone(),
        claim,
        authority: TestOpenAuthority,
        shown_steps: ShownSteps::new(()),
    }
    .run(
        &mut CitedAnswerProvider {
            round: 0,
            candidate_id: "bmp-2026-08-27".to_owned(),
        },
        engine::RunLimits::default(),
        &engine::Cancellation::default(),
    )
    .await
    .expect("complete cited turn");

    // Seed a second conversation on patient_a that ended Held so Past work &
    // Conversations distinguish items by question, summary, and outcome!
    let chat_held = "chat-renal-dosing-held";
    let scope_held = Scope {
        practice: physician_sample::STAGING_PRACTICE_ID.into(),
        physician: "staging-physician".into(),
        workspace: patient_a.into(),
        chat: chat_held.into(),
    };
    store
        .accept_authorized(
            &scope_held,
            &Input {
                key: "turn-renal-held-1".into(),
                text: "Check lisinopril renal dosing options".into(),
                timezone: "America/New_York".into(),
                scenario: None,
            },
            vec![7; 32],
            &skills,
            &scale::models::chat_recipe(),
            None,
        )
        .await
        .unwrap();
    let held_claim = store.claim(&scope_held, 60).await.unwrap().unwrap();
    store
        .settle_chat_attempt(
            &held_claim,
            AttemptFailure::Limit,
            local_product::runtime::delivery::Backoff::IMMEDIATE,
        )
        .await
        .unwrap();

    // Seed a saved reply draft on patient_a so Past work includes `PastWorkTarget::ReplyDraft`
    // alongside `Conversation` items for both reopen and revoked-access verification!
    let scope_reply = Scope {
        practice: physician_sample::STAGING_PRACTICE_ID.into(),
        physician: "staging-physician".into(),
        workspace: patient_a.into(),
        chat: "reply-k-followup".into(),
    };
    store
        .open_draft_authorized(
            &scope_reply,
            "Recheck serum potassium Friday morning at Regional Lab.",
            &[7; 32],
            &mut TestOpenAuthority,
        )
        .await
        .unwrap()
        .expect("seed saved reply draft");

    // Also seed a conversation on patient_b so `/recent-conversations` spans multiple workspaces.
    scale::answered_chat(
        &store,
        patient_b,
        "chat-jake-anticoag",
        &[(
            "Summarize anticoagulation hold window",
            "Hold apixaban 48 hours prior to high-bleed-risk procedure.",
        )],
    )
    .await;

    let valkey = scale::Valkey::start().await;
    let redis_url = valkey.url();

    let spawn_durable_instance = |db_url: String, redis: String| async move {
        let runtime_store = Store::connect(&db_url).await.unwrap();
        let bus: Arc<dyn physician_bus::Bus> = Arc::new(
            physician_bus::RedisBus::connect(&redis, None)
                .await
                .unwrap(),
        );
        let chat = Arc::new(physician_sample::durable::DurableChat::new(
            runtime_store,
            local_product::physician_chat::PhysicianSkills::configured(None, false).unwrap(),
            &scale::models::roles(),
        ));
        let auth = PostgresAuthStore::connect_without_migrations(&db_url)
            .await
            .unwrap();
        let front = front_door::PostgresFrontDoorStore::connect_without_migrations(&db_url)
            .await
            .unwrap();
        let app = physician_sample::authed_router_on(
            physician_sample::durable_site(chat, bus),
            Arc::new(auth),
            Arc::new(front),
        )
        .await;
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        let handle = tokio::spawn(async move { axum::serve(listener, app).await.unwrap() });
        wait_until_ready(addr).await;
        (addr, handle)
    };

    // Start server instance 1, verify initial reachability, then drop it to simulate server restart.
    let (addr_1, server_1) = spawn_durable_instance(database.url.clone(), redis_url.clone()).await;
    let session_1 = scale::sign_in(&addr_1.to_string()).await;
    let (status_1, _) =
        scale::get_json(&addr_1.to_string(), "/recent-conversations", &session_1).await;
    assert_eq!(status_1, 200);
    server_1.abort();

    // Start fresh server instance 2 on the same Postgres database ("after server restart").
    let (addr_2, server_2) = spawn_durable_instance(database.url.clone(), redis_url.clone()).await;
    let host_2 = addr_2.to_string();
    let session_2 = scale::sign_in(&host_2).await;

    // AC1 & AC3: Compare HTTP query results directly against Store saved records.
    let store_readback = store
        .reopen_conversation(&scope_cited)
        .await
        .unwrap()
        .expect("saved conversation in Postgres");
    let (snap_status, snap_wire) = scale::get_json(
        &host_2,
        &format!("/patient-workspaces/{patient_a}/chats/{chat_cited}/snapshot"),
        &session_2,
    )
    .await;
    assert_eq!(snap_status, 200, "{snap_wire}");
    let assistant_wire = snap_wire["snapshot"]["transcript"]
        .as_array()
        .unwrap()
        .iter()
        .filter_map(|exchange| exchange.get("answer"))
        .find(|answer| answer["state"] == "finished")
        .expect("finished assistant answer in snapshot");
    assert!(!assistant_wire["newer_records"].as_bool().unwrap_or(false));
    assert!(
        assistant_wire["basis"].is_array()
            && !assistant_wire["basis"].as_array().unwrap().is_empty(),
        "reopened snapshot must carry Basis segments: {assistant_wire}"
    );
    assert_eq!(
        snap_wire["snapshot"]["sources"].as_object().unwrap().len(),
        store_readback.sources.len(),
        "wire sources must match saved store receipts"
    );

    let (rec_status, rec_wire) =
        scale::get_json(&host_2, "/recent-conversations", &session_2).await;
    assert_eq!(rec_status, 200, "{rec_wire}");
    let rec_items = rec_wire["conversations"].as_array().unwrap();
    assert_eq!(
        rec_items.len(),
        3,
        "expected 3 durable recent conversations across workspaces: {rec_wire}"
    );
    let (pw_status, pw_wire) = scale::get_json(
        &host_2,
        &format!("/patient-workspaces/{patient_a}/past-work"),
        &session_2,
    )
    .await;
    assert_eq!(pw_status, 200, "{pw_wire}");
    assert_eq!(pw_wire["items"].as_array().unwrap().len(), 3);
    println!(
        "Restarted site durable history verified: recent_count={}, past_work_count={}, sources_count={}",
        rec_items.len(),
        pw_wire["items"].as_array().unwrap().len(),
        store_readback.sources.len()
    );

    // Browser verification on fresh server instance 2:
    let mut browser = Tab::open(&format!("http://{host_2}/"));
    browser.set_viewport(1280, 800, false);
    browser.wait_for_text("Work email", Duration::from_secs(20));
    browser.request_sign_in("physician@codahealth.test");
    let today_dom = browser.wait_for_text(
        "What was Tuesday's potassium and its basis?",
        Duration::from_secs(20),
    );
    // AC3: Recent conversations list distinguishes items by question, summary, and outcome!
    assert!(
        today_dom.contains("Potassium is **5.4 mmol/L** on Tuesday's panel")
            || today_dom.contains("Potassium is 5.4 mmol/L")
            || today_dom.contains("5.4 mmol/L"),
        "recent conversation row must show answer summary:\n{today_dom}"
    );
    assert!(
        today_dom.contains("Check lisinopril renal dosing options")
            && today_dom.contains("Paused")
            && today_dom.contains("Answered"),
        "recent conversations must distinguish items by content and outcome:\n{today_dom}"
    );
    browser.save_picture("history-recent-conversations");

    // Click the cited conversation in Recent conversations to reopen it!
    browser.eval(
        "([...document.querySelectorAll('.sidebar-recent button')].find(b => b.innerText.includes(\"What was Tuesday's potassium\"))?.click(), true)",
    );
    browser.wait_for_text("Basis and limitations", Duration::from_secs(15));
    // Open Past work dialog on the patient page and verify content distinction + ReplyDraft reopen.
    browser.click_named("button", "Past work");
    browser.wait_for_text(
        "Check lisinopril renal dosing options",
        Duration::from_secs(10),
    );
    let past_work_dialog = browser.dialog_text();
    assert!(
        past_work_dialog.contains("What was Tuesday's potassium and its basis?")
            && past_work_dialog.contains("Patient reply draft")
            && past_work_dialog.contains("Answered")
            && past_work_dialog.contains("Paused")
            && past_work_dialog.contains("Reply ready"),
        "Past work dialog must distinguish conversations and reply drafts by content and outcome:\n{past_work_dialog}"
    );
    std::thread::sleep(Duration::from_millis(250));
    browser.save_picture("history-past-work-dialog");
    browser.eval(
        "([...document.querySelectorAll('dialog.past-work-dialog button')].find(b => b.innerText.includes('Patient reply draft'))?.click(), true)",
    );
    let reply_reader_deadline = Instant::now() + Duration::from_secs(10);
    while browser
        .eval("document.querySelector('dialog.reader[open]')?.innerText ?? ''")
        .as_str()
        .is_none_or(|t| !t.contains("Recheck serum potassium Friday morning at Regional Lab."))
    {
        assert!(
            Instant::now() < reply_reader_deadline,
            "ReaderDialog did not open with saved reply draft"
        );
        std::thread::sleep(Duration::from_millis(40));
    }
    browser.click("dialog.reader[open] button[aria-label=\"Close\"]");
    let close_deadline = Instant::now() + Duration::from_secs(5);
    while !browser.dialog_text().is_empty() {
        assert!(
            Instant::now() < close_deadline,
            "ReaderDialog did not finish closing"
        );
        std::thread::sleep(Duration::from_millis(40));
    }

    // Expand Basis and open citation [1] in the citation side panel; compare against saved record!
    browser.click_named("button.answer-control", "Basis and limitations");
    browser.wait_for_text(
        "Based on Regional Lab Basic metabolic panel",
        Duration::from_secs(10),
    );
    browser.click("button.citation-chip");
    browser.wait_for_text(
        "Potassium 5.4 mmol/L (reference 3.5-5.0 mmol/L), collected Tuesday morning.",
        Duration::from_secs(10),
    );
    let citation_heading = browser
        .eval("document.querySelector('.source-reading-pane .source-pane-heading h2')?.innerText ?? ''")
        .as_str()
        .unwrap_or_default()
        .to_owned();
    let readable_body = browser
        .eval("document.querySelector('.source-reading-pane .source-record-body')?.innerText ?? ''")
        .as_str()
        .unwrap_or_default()
        .to_owned();
    let raw_disclosure = browser.eval(
        "({ state: document.querySelector('.source-reading-pane .disclosure')?.getAttribute('data-state') ?? '', height: document.querySelector('.source-reading-pane .disclosure-region')?.getBoundingClientRect().height ?? -1 })",
    );
    assert_eq!(
        readable_body
            .matches("Potassium 5.4 mmol/L (reference 3.5-5.0 mmol/L), collected Tuesday morning.")
            .count(),
        1,
        "citation side panel must render the cited passage once without duplication:\n{readable_body}"
    );
    assert!(
        citation_heading == "Basic metabolic panel"
            && !readable_body.contains("Basic metabolic panel"),
        "citation side panel must not repeat the record title inside readable parts: heading={citation_heading:?}, body:\n{readable_body}"
    );
    assert!(
        !readable_body.contains('{')
            && raw_disclosure["state"] == serde_json::json!("closed")
            && raw_disclosure["height"] == serde_json::json!(0),
        "citation side panel must keep raw JSON collapsed under Technical details ({raw_disclosure}):\n{readable_body}"
    );
    std::thread::sleep(Duration::from_millis(250));
    browser.save_picture("history-reopened-answer-basis-citation");
    browser.click(".source-reading-pane button[aria-label=\"Close\"]");
    let cite_close_deadline = Instant::now() + Duration::from_secs(5);
    while browser.eval("document.querySelector('.source-reading-pane') !== null")
        == serde_json::json!(true)
    {
        assert!(
            Instant::now() < cite_close_deadline,
            "citation side panel did not finish closing"
        );
        std::thread::sleep(Duration::from_millis(40));
    }

    // Record a newer source revision in Postgres after the answer's records version.
    // The old answer must stay unchanged AND show the newer-records marker!
    let raw_lab_v2 = r#"{"test":"Repeat metabolic panel","potassium_mmol_l":4.6}"#;
    let pull_v2 = SourcePull {
        pulled_at: Some(1_787_850_000),
        ..pull_v1
    };
    let rev_v2 = SourceRevision {
        workspace: patient_a.to_owned(),
        source_id: "fixture-chart".to_owned(),
        candidate_id: "bmp-2026-08-27-repeat".to_owned(),
        revision: "rev-2".to_owned(),
        record_type: "Laboratory result".to_owned(),
        dates: vec![RecordDate {
            name: "Collected".to_owned(),
            value: "2026-08-27".to_owned(),
        }],
        content: raw_lab_v2.to_owned(),
        sha256: local_product::sha256_hex(raw_lab_v2.as_bytes()),
        readable_parts: vec![ReadablePart {
            label: "Repeat metabolic panel".to_owned(),
            text: "Potassium 4.6 mmol/L (repeat specimen).".to_owned(),
        }],
        provenance: serde_json::json!({
            "source_label": "Regional Lab",
            "title": "Repeat metabolic panel",
        }),
        envelope: None,
    };
    store
        .record_chart(patient_a, &[rev_v2], &pull_v2)
        .await
        .expect("record newer chart");

    browser.reload();
    let newer_dom = browser.wait_for_text(
        "Newer records arrived after this answer.",
        Duration::from_secs(15),
    );
    assert!(
        newer_dom.contains("5.4 mmol/L") && newer_dom.contains("Update"),
        "old answer must stay unchanged and show the Update marker:\n{newer_dom}"
    );
    browser.save_picture("history-newer-records-marker");

    // Phone viewport (<900px) verification of reopened conversation + newer-records marker.
    browser.set_viewport(390, 844, true);
    browser.wait_for_text(
        "Newer records arrived after this answer.",
        Duration::from_secs(10),
    );
    let phone_shell = browser.step("readShell", serde_json::json!([]));
    assert_eq!(phone_shell["pageWidth"], 390, "{phone_shell}");
    browser.set_viewport(1280, 800, false);

    // AC4 (part 1): Load Past work dialog on patient_a, remove panel access, then click
    // `PastWorkTarget::ReplyDraft` from the already-loaded dialog -> server returns 404,
    // ReaderDialog never opens, PastWorkDialog closes, and clinical content closes immediately!
    browser.click_named("button", "Past work");
    browser.wait_for_text("Patient reply draft", Duration::from_secs(10));
    let auth_pg = PostgresAuthStore::connect_without_migrations(&database.url)
        .await
        .unwrap();
    let staging_identity = PhysicianIdentity {
        practice_id: physician_sample::STAGING_PRACTICE_ID.to_owned(),
        physician_id: "staging-physician".to_owned(),
    };
    auth_pg
        .set_membership(&staging_identity, patient_a, Membership::Denied)
        .await
        .unwrap();
    let (revoked_pw_status, _) = scale::get_json(
        &host_2,
        &format!("/patient-workspaces/{patient_a}/past-work"),
        &session_2,
    )
    .await;
    assert_eq!(
        revoked_pw_status, 404,
        "revoked panel access must refuse past-work read with 404"
    );
    browser.eval(
        "([...document.querySelectorAll('dialog.past-work-dialog button')].find(b => b.innerText.includes('Patient reply draft'))?.click(), true)",
    );
    let blocked_draft_dom =
        browser.wait_for_text("Clinic record · Not on your panel", Duration::from_secs(15));
    assert!(
        browser.dialog_text().is_empty()
            && !blocked_draft_dom.contains("Recheck serum potassium Friday morning"),
        "revoked access must block ReplyDraft reopen and close dialogs:\ndialog={}\ndom={blocked_draft_dom}",
        browser.dialog_text()
    );

    // Restore access briefly so Today loads Recent conversations, then remove panel access
    // again and attempt to reopen the conversation from the loaded Recent list (AC4 part 2)!
    auth_pg
        .set_membership(&staging_identity, patient_a, Membership::Allowed)
        .await
        .unwrap();
    browser.navigate(&format!("http://{host_2}/"));
    browser.wait_for_text(
        "What was Tuesday's potassium and its basis?",
        Duration::from_secs(15),
    );
    auth_pg
        .set_membership(&staging_identity, patient_a, Membership::Denied)
        .await
        .unwrap();

    let (revoked_snap_status, _) = scale::get_json(
        &host_2,
        &format!("/patient-workspaces/{patient_a}/chats/{chat_cited}/snapshot"),
        &session_2,
    )
    .await;
    assert_eq!(
        revoked_snap_status, 404,
        "revoked panel access must refuse conversation reopen with 404"
    );

    browser.eval(
        "([...document.querySelectorAll('.sidebar-recent button')].find(b => b.innerText.includes(\"What was Tuesday's potassium\"))?.click(), true)",
    );
    let blocked_dom =
        browser.wait_for_text("Clinic record · Not on your panel", Duration::from_secs(15));
    assert!(
        !blocked_dom.contains("5.4 mmol/L")
            && !blocked_dom.contains("Based on Regional Lab Basic metabolic panel"),
        "revoked access after list load must block reopening saved clinical answers:\n{blocked_dom}"
    );
    browser.save_picture("history-access-removed-blocked");

    // Verify phone (<900px) also blocks reopening cleanly.
    browser.set_viewport(390, 844, true);
    assert!(browser.dom().contains("Clinic record · Not on your panel"));

    drop(browser);
    server_2.abort();
    unsafe {
        env::remove_var("PHYSICIAN_AUTH");
        env::remove_var("PHYSICIAN_AUTH_PROOF_TOKENS");
    }
}

/// Collapsed orb icon, motion / `prefers-reduced-motion` handling, hover/focus
/// Updates preview (active reviews + recent items + controls), and persistent
/// cross-page `Now` banner until opened once, on desktop and phone (`<900px`).
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore = "browser qualification; run explicitly with --ignored"]
async fn collapsed_orb_and_persistent_now_banner_across_pages_and_motion_modes() {
    let _guard = test_lock().lock().await;
    let site = build_site();
    unsafe {
        env::set_var("PHYSICIAN_SITE_DIR", &site);
        env::remove_var("PHYSICIAN_AUTH");
        env::remove_var("PHYSICIAN_AUTH_PROOF_TOKENS");
        env::remove_var("PHYSICIAN_UPSTREAM");
    }

    let patient_a = physician_sample::WORKSPACE_IDS[0]; // Tanvir Chowdhury
    let patient_b = physician_sample::WORKSPACE_IDS[4]; // Mary Louis

    let feed = physician_sample::SampleFeed::new();
    feed.set_today_rows(vec![
        serde_json::json!({
            "item_id": "concern-now-tanvir-k",
            "version_id": "v1",
            "waiting_since": "2026-08-27T15:45:00Z",
            "kind": "concern",
            "patient_workspace_id": patient_a,
            "patient_display_name": "Tanvir Chowdhury",
            "urgency": "now",
            "reason": "Critical potassium 6.2 mmol/L from Regional Lab",
            "seen": false,
        }),
        serde_json::json!({
            "item_id": "concern-today-mary-bp",
            "version_id": "v1",
            "waiting_since": "2026-08-27T15:30:00Z",
            "kind": "concern",
            "patient_workspace_id": patient_b,
            "patient_display_name": "Mary Louis",
            "urgency": "today",
            "reason": "Home blood pressure log ready for review",
            "seen": false,
        }),
    ]);
    feed.set_updates(
        vec![serde_json::json!({
            "run_id": "run-review-tanvir-1",
            "patient_workspace_id": patient_a,
            "patient_display_name": "Tanvir Chowdhury",
            "headline": "Coda is reviewing repeat nephrology labs",
            "started_at": "2026-08-27T15:58:00Z",
        })],
        vec![serde_json::json!({
            "item_id": "update-mary-discharge",
            "patient_workspace_id": patient_b,
            "patient_display_name": "Mary Louis",
            "headline": "Cardiology clinic note imported",
        })],
    );

    let app = physician_sample::fixture_site_with_feed(Arc::clone(&feed));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    let server_task = tokio::spawn(async move { axum::serve(listener, app).await.unwrap() });
    wait_until_ready(address).await;

    // 1. Desktop Today: persistent NowBanner shows at top naming patient + concern + Open control.
    let mut browser = Tab::open(&format!("http://{address}/"));
    browser.set_viewport(1280, 800, false);
    let today_dom = browser.wait_for_text(
        "Critical potassium 6.2 mmol/L from Regional Lab",
        Duration::from_secs(20),
    );
    assert!(
        today_dom.contains("now-banner") && today_dom.contains("Tanvir Chowdhury"),
        "Today must render persistent NowBanner:\n{today_dom}"
    );
    browser.save_picture("now-banner-today-desktop");

    // Phone Today (<900px): NowBanner fits cleanly with zero sideways scroll.
    browser.set_viewport(390, 844, false);
    std::thread::sleep(Duration::from_millis(350));
    browser.wait_for_text(
        "Critical potassium 6.2 mmol/L from Regional Lab",
        Duration::from_secs(10),
    );
    let phone_today = browser.step("readShell", serde_json::json!([]));
    assert_eq!(phone_today["pageWidth"], 390, "{phone_today}");
    browser.save_picture("now-banner-today-phone");
    browser.set_viewport(1280, 800, false);

    // Navigate to ANOTHER patient's page (Mary Louis) -> NowBanner for Tanvir stays at top!
    browser.navigate(&format!("http://{address}/?patient={patient_b}"));
    let other_patient_dom = browser.wait_for_text("Mary Louis", Duration::from_secs(15));
    let banner_text = browser
        .eval("document.querySelector('.now-banner')?.innerText ?? ''")
        .as_str()
        .unwrap_or_default()
        .to_owned();
    assert!(
        banner_text.contains("Tanvir Chowdhury")
            && banner_text.contains("Critical potassium 6.2 mmol/L from Regional Lab")
            && banner_text.contains("Open"),
        "NowBanner must persist at the top of another patient's page:\n{banner_text}\n{other_patient_dom}"
    );
    browser.save_picture("now-banner-other-patient");

    // Collapse Coda -> orb renders with icon, breathing aura animation under
    // `no-preference`, honors `prefers-reduced-motion: reduce`, AND shows the Now concern pill!
    browser.click("button.stage-minimize");
    browser.wait_for_text(
        "Critical potassium 6.2 mmol/L from Regional Lab",
        Duration::from_secs(10),
    );
    let orb_motion = browser.eval(
        "(() => { const aura = document.querySelector('.coda-orb-aura'); const svg = document.querySelector('button.coda-orb svg'); return { hasOrb: Boolean(svg), auraAnim: aura ? getComputedStyle(aura).animationName : null }; })()",
    );
    assert_eq!(orb_motion["hasOrb"], serde_json::json!(true));
    assert_ne!(
        orb_motion["auraAnim"],
        serde_json::json!("none"),
        "orb aura should animate when motion is allowed: {orb_motion}"
    );
    let orb_pill = browser
        .eval("document.querySelector('.orb-now-pill')?.innerText ?? ''")
        .as_str()
        .unwrap_or_default()
        .to_owned();
    assert!(
        orb_pill.contains("Tanvir Chowdhury")
            && orb_pill.contains("Critical potassium 6.2 mmol/L from Regional Lab"),
        "collapsed orb must surface the Now concern:\n{orb_pill}"
    );
    browser.save_picture("orb-collapsed-with-now");

    // Verify `prefers-reduced-motion: reduce` disables orb animations!
    browser.set_viewport(1280, 800, true);
    let reduced_motion = browser.eval(
        "(() => { const orb = document.querySelector('button.coda-orb'); const aura = document.querySelector('.coda-orb-aura'); return { orbAnim: orb ? getComputedStyle(orb).animationName : null, auraAnim: aura ? getComputedStyle(aura).animationName : null }; })()",
    );
    assert_eq!(
        reduced_motion,
        serde_json::json!({ "orbAnim": "none", "auraAnim": "none" }),
        "prefers-reduced-motion: reduce must disable orb animations: {reduced_motion}"
    );
    browser.save_picture("orb-reduced-motion");

    // Hover/focus the collapsed orb -> previews Updates (active reviews + new items +
    // controls to open Coda and All updates), on both desktop and phone (<900px)!
    browser.eval(
        "Boolean(document.querySelector('.orb-dock')?.dispatchEvent(new FocusEvent('focusin', { bubbles: true })))",
    );
    browser.wait_for_text(
        "Coda is reviewing repeat nephrology labs",
        Duration::from_secs(10),
    );
    let preview_text = browser
        .eval("document.querySelector('.orb-preview')?.innerText ?? ''")
        .as_str()
        .unwrap_or_default()
        .to_owned();
    assert!(
        preview_text.contains("Updates")
            && preview_text.contains("Coda is reviewing repeat nephrology labs")
            && preview_text.contains("Cardiology clinic note imported")
            && preview_text.contains("Open Coda")
            && preview_text.contains("All updates"),
        "orb preview missing active review, recent item, or action buttons:\n{preview_text}"
    );
    browser.save_picture("orb-updates-preview-desktop");

    browser.set_viewport(390, 844, true);
    let phone_orb_overflow = browser
        .eval("document.scrollingElement.scrollWidth - document.scrollingElement.clientWidth");
    assert!(
        phone_orb_overflow.as_f64().is_some_and(|by| by <= 0.0),
        "orb preview overflowed phone viewport by {phone_orb_overflow}"
    );
    browser.save_picture("orb-updates-preview-phone");
    browser.set_viewport(1280, 800, false);

    // Click "All updates" in the orb preview -> expands Coda to Updates surface, NowBanner still at top!
    browser.click_named(".orb-preview button", "All updates");
    browser.wait_for_text("Collapse Coda", Duration::from_secs(10));
    assert!(
        browser
            .eval("Boolean(document.querySelector('.now-banner'))")
            .as_bool()
            == Some(true),
        "NowBanner must stay visible on Updates until opened once"
    );

    // Click "Open" on NowBanner -> opens Tanvir Chowdhury, sends `item.open`, and clears NowBanner!
    browser.click_named(".now-banner button", "Open");
    browser.wait_for_text("MRN 00000003", Duration::from_secs(10));
    let cleared_deadline = Instant::now() + Duration::from_secs(10);
    while browser
        .eval("Boolean(document.querySelector('.now-banner'))")
        .as_bool()
        == Some(true)
    {
        assert!(
            Instant::now() < cleared_deadline,
            "NowBanner did not clear after opening the concern once"
        );
        std::thread::sleep(Duration::from_millis(60));
    }
    std::thread::sleep(Duration::from_millis(250));
    browser.save_picture("now-banner-cleared-after-open");

    // Collapsing again after opening confirms `.orb-now-pill` is also gone.
    browser.click("button.stage-minimize");
    let orb_deadline = Instant::now() + Duration::from_secs(5);
    while browser
        .eval("Boolean(document.querySelector('button.coda-orb[aria-label=\"Open Coda\"]'))")
        .as_bool()
        != Some(true)
    {
        assert!(Instant::now() < orb_deadline, "orb did not render");
        std::thread::sleep(Duration::from_millis(40));
    }
    assert_eq!(
        browser.eval("Boolean(document.querySelector('.orb-now-pill'))"),
        serde_json::json!(false)
    );

    server_task.abort();
}

async fn dump_dom(chrome: &std::path::Path, url: &str) -> String {
    let child = Command::new(chrome)
        .env_remove("LD_LIBRARY_PATH")
        .args([
            "--headless=new",
            "--disable-gpu",
            "--no-sandbox",
            "--virtual-time-budget=15000",
            "--timeout=20000",
            "--dump-dom",
            url,
        ])
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .expect("chromium");
    let pid = child.id();
    let wait = tokio::task::spawn_blocking(move || child.wait_with_output());
    let output = match tokio::time::timeout(Duration::from_secs(40), wait).await {
        Ok(Ok(output)) => output.expect("chromium"),
        _ => {
            let _ = Command::new("kill").arg(pid.to_string()).status();
            panic!("chromium did not finish");
        }
    };
    let dom = String::from_utf8_lossy(&output.stdout);
    assert!(
        output.status.success(),
        "chromium exited {:?}\n{dom}\n{}",
        output.status.code(),
        String::from_utf8_lossy(&output.stderr)
    );
    dom.into_owned()
}

fn read_logo(address: std::net::SocketAddr) -> Vec<u8> {
    use std::io::{Read, Write};
    let mut stream = std::net::TcpStream::connect(address).expect("logo connection");
    stream
        .write_all(
            b"GET /assets/coda-health-logo-transparent.png HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\n\r\n",
        )
        .unwrap();
    let mut body = Vec::new();
    stream.read_to_end(&mut body).unwrap();
    body
}

fn build_site() -> PathBuf {
    BUILT_SITE
        .get_or_init(|| {
            let workspace = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
            let target = workspace.join("target/coda-physician-wasm");
            let wasm = target.join("wasm32-unknown-unknown/debug/physician-app.wasm");
            let mut cargo = Command::new(env::var_os("CARGO").unwrap_or_else(|| "cargo".into()));
            cargo_without_parent_package(&mut cargo);
            let status = cargo
                .current_dir(&workspace)
                .env("CARGO_TARGET_DIR", &target)
                .env_remove("PHYSICIAN_API_URL")
                .args([
                    "build",
                    "-p",
                    "physician-app",
                    "--target",
                    "wasm32-unknown-unknown",
                    "--features",
                    "web",
                ])
                .status()
                .expect("cargo build");
            assert!(status.success(), "physician-app wasm build failed");
            let site = workspace.join("target/physician-web");
            fs::create_dir_all(&site).unwrap();
            let status = Command::new(wasm_bindgen())
                .args([
                    "--target",
                    "web",
                    "--out-dir",
                    site.to_str().unwrap(),
                    wasm.to_str().unwrap(),
                ])
                .status()
                .expect("wasm-bindgen");
            assert!(status.success(), "wasm-bindgen failed");
            site
        })
        .clone()
}

fn cargo_without_parent_package(command: &mut Command) {
    command.env_clear();
    for key in [
        "HOME",
        "USER",
        "RUSTUP_HOME",
        "CARGO_HOME",
        "RUSTUP_TOOLCHAIN",
        "TMPDIR",
    ] {
        if let Some(value) = env::var_os(key) {
            command.env(key, value);
        }
    }
    if let Some(paths) = env::var_os("PATH") {
        let filtered = env::join_paths(
            env::split_paths(&paths)
                .filter(|dir| !dir.to_string_lossy().contains(".local/share/swiftly")),
        )
        .expect("join PATH");
        command.env("PATH", filtered);
    }
}

fn wasm_bindgen() -> PathBuf {
    if let Some(path) = env::var_os("WASM_BINDGEN") {
        return PathBuf::from(path);
    }
    if let Some(path) = find_on_path("wasm-bindgen") {
        return path;
    }
    let local = env::var_os("HOME").map(|home| PathBuf::from(home).join(".local/bin/wasm-bindgen"));
    if let Some(path) = local.filter(|path| path.is_file()) {
        return path;
    }
    panic!("wasm-bindgen 0.2.129 is required to build the physician page");
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
    panic!("chromium is required to validate the physician page");
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
            panic!("physician site did not accept connections on {address}");
        }
        if std::net::TcpStream::connect(address).is_ok() {
            return;
        }
        tokio::time::sleep(Duration::from_millis(50)).await;
    }
}

/// One physician page in its own headless Chromium, driven through the shared
/// DevTools client, and the requests it holds before they reach the server.
struct Tab {
    browser: Browser,
    /// Requests paused by `Fetch.enable` and not yet let go, oldest first.
    held: Vec<serde_json::Value>,
}

impl Tab {
    fn open(url: &str) -> Self {
        let mut browser = Browser::launch();
        browser
            .page()
            .call_session("Page.enable", serde_json::json!({}));
        browser
            .page()
            .call_session("Runtime.enable", serde_json::json!({}));
        browser.page().navigate(url);
        Self {
            browser,
            held: Vec::new(),
        }
    }

    fn eval(&mut self, expression: &str) -> serde_json::Value {
        self.browser.page().evaluate(expression)
    }

    fn call(&mut self, method: &str, params: serde_json::Value) -> serde_json::Value {
        self.browser.page().call_session(method, params)
    }

    /// Runs one step of `js/site.js` with `args`, a JSON array, as real
    /// arguments, and returns what it answers. A step that throws fails the test.
    fn step(&mut self, name: &str, args: serde_json::Value) -> serde_json::Value {
        let mut arguments = vec![serde_json::json!(name)];
        arguments.extend(args.as_array().expect("a step's arguments").iter().cloned());
        self.browser
            .page()
            .call_function(include_str!("js/site.js"), &arguments)
    }

    /// Puts the page in `theme` and reads the sidebar's layout, once it has rendered.
    fn sidebar_layout(&mut self, theme: Theme) -> Option<SidebarLayout> {
        let answer = self.browser.page().call_function(
            include_str!("js/sidebar_layout.js"),
            &[serde_json::json!(theme.attribute())],
        );
        serde_json::from_value(answer.clone())
            .unwrap_or_else(|error| panic!("sidebar_layout.js answered {answer}: {error}"))
    }

    fn navigate(&mut self, url: &str) {
        self.browser.page().navigate(url);
    }

    fn click(&mut self, selector: &str) {
        self.browser.click(selector);
    }

    /// Clicks the first `selector` whose words are exactly `words`.
    fn click_named(&mut self, selector: &str, words: &str) {
        self.step("clickNamed", serde_json::json!([selector, words]));
    }

    /// One real key press.
    fn press_key(&mut self, key: Key) {
        self.browser.press_key(key);
    }

    fn dom(&mut self) -> String {
        self.browser.html()
    }

    /// Lets every held draft read go on, and returns the first held request
    /// with `method`, waiting up to ten seconds for it.
    fn held_request(&mut self, method: &str) -> serde_json::Value {
        let started = Instant::now();
        loop {
            self.held.extend(self.browser.take_paused_requests());
            let mut found = None;
            for request in std::mem::take(&mut self.held) {
                if found.is_none() && request["request"]["method"] == method {
                    found = Some(request);
                } else if request["request"]["method"] == "GET" {
                    self.release(&request);
                } else {
                    self.held.push(request);
                }
            }
            if let Some(request) = found {
                return request;
            }
            assert!(
                started.elapsed() < Duration::from_secs(10),
                "no {method} request was held"
            );
            std::thread::sleep(Duration::from_millis(30));
        }
    }

    /// Lets one held request go on to the server.
    fn release(&mut self, request: &serde_json::Value) {
        self.call(
            "Fetch.continueRequest",
            serde_json::json!({ "requestId": request["requestId"] }),
        );
    }

    /// Lets every held request go on.
    fn release_all(&mut self) {
        self.held.extend(self.browser.take_paused_requests());
        for request in std::mem::take(&mut self.held) {
            self.release(&request);
        }
    }

    /// Types `text` into the composer as the physician would.
    fn type_composer(&mut self, text: &str) {
        self.browser.type_into("textarea.composer-input", text);
        std::thread::sleep(Duration::from_millis(100));
    }

    /// The composer's text, and the mark beside it.
    fn composer(&mut self) -> (String, String) {
        let pair = self.step("composer", serde_json::json!([]));
        (
            pair[0].as_str().unwrap_or("<no composer>").to_owned(),
            pair[1].as_str().unwrap_or_default().to_owned(),
        )
    }

    fn wait_saved(&mut self) {
        let started = Instant::now();
        loop {
            let (text, mark) = self.composer();
            assert_eq!(mark, "", "successful draft saves must be silent");
            if self.step("savedDraft", serde_json::json!([]))["text"] == text {
                return;
            }
            assert!(
                started.elapsed() < Duration::from_secs(10),
                "draft did not save"
            );
            std::thread::sleep(Duration::from_millis(50));
        }
    }

    fn wait_not_saved(&mut self) {
        self.wait_for_text("Not saved", Duration::from_secs(10));
        assert_eq!(self.composer().1, "Not saved");
    }

    /// Waits until the composer shows `text`.
    fn wait_composer(&mut self, text: &str, timeout: Duration) {
        let started = Instant::now();
        loop {
            let (shown, _) = self.composer();
            if shown == text {
                return;
            }
            assert!(
                started.elapsed() < timeout,
                "the composer never showed {text:?}; it shows {shown:?}"
            );
            std::thread::sleep(Duration::from_millis(60));
        }
    }

    /// The conflict the composer shows: the newer revision and its text.
    fn conflict(&mut self) -> Option<(u64, String)> {
        let pair = self.step("conflict", serde_json::json!([]));
        Some((pair.get(0)?.as_u64()?, pair.get(1)?.as_str()?.to_owned()))
    }

    /// Waits for the composer to show a conflict, and returns it.
    fn wait_conflict(&mut self) -> (u64, String) {
        let started = Instant::now();
        loop {
            if let Some(conflict) = self.conflict() {
                return conflict;
            }
            assert!(
                started.elapsed() < Duration::from_secs(10),
                "no conflict showed: {:?}",
                self.composer()
            );
            std::thread::sleep(Duration::from_millis(60));
        }
    }

    /// Clicks the conflict's `label` button (Retry or Use newer text).
    fn click_conflict(&mut self, label: &str) {
        self.click_named(".draft-conflict button", label);
    }

    /// This tab's same-tab draft recovery keys.
    fn recovery_keys(&mut self) -> Vec<String> {
        self.step("draftRecoveryKeys", serde_json::json!([]))
            .as_array()
            .map(|keys| {
                keys.iter()
                    .filter_map(|key| key.as_str().map(str::to_owned))
                    .collect()
            })
            .unwrap_or_default()
    }

    fn request_sign_in(&mut self, email: &str) {
        self.browser.type_into("input[type=\"email\"]", email);
        std::thread::sleep(Duration::from_millis(50));
        self.click("button.control-primary");
    }

    fn search_for(&mut self, query: &str) {
        self.browser.type_into(".search-field input", query);
    }

    fn dialog_text(&mut self) -> String {
        self.step("dialogText", serde_json::json!([]))
            .as_str()
            .unwrap_or_default()
            .to_owned()
    }

    fn sign_in_again(&mut self) {
        self.request_sign_in("admin@clinic-a.test");
    }

    fn wait_for_text(&mut self, expected: &str, timeout: Duration) -> String {
        let started = Instant::now();
        loop {
            // Read visible text and its HTML in one browser turn. Hidden text or
            // a CSS class cannot satisfy the wait, nor can we return older HTML.
            let snapshot = self.eval(
                "({text: document.body?.innerText ?? '', html: document.documentElement?.outerHTML ?? ''})",
            );
            let html = snapshot["html"].as_str().unwrap_or_default();
            if snapshot["text"]
                .as_str()
                .is_some_and(|text| text.contains(expected))
            {
                return html.to_owned();
            }
            if started.elapsed() > timeout {
                panic!("timed out waiting for {expected:?} in DOM:\n{html}");
            }
            std::thread::sleep(Duration::from_millis(60));
        }
    }

    fn reload(&mut self) {
        let previous = self.document_loader();
        self.call("Page.reload", serde_json::json!({}));
        let started = Instant::now();
        // The same conversation text is visible before and after a reload.
        // Wait for a different document before text waits or clicks can run;
        // otherwise they can act on the page the browser is about to discard.
        while self.document_loader() == previous {
            assert!(
                started.elapsed() < Duration::from_secs(10),
                "reload did not replace the document"
            );
            std::thread::sleep(Duration::from_millis(20));
        }
    }

    fn document_loader(&mut self) -> String {
        self.call("Page.getFrameTree", serde_json::json!({}))
            .pointer("/result/frameTree/frame/loaderId")
            .and_then(|id| id.as_str())
            .expect("main frame document loader")
            .to_owned()
    }

    /// Answers one held request with `body` instead of the server.
    /// Answers a held request with `body`. The page may have dropped the
    /// request for a newer one while it was held; Chrome then no longer knows
    /// it, and there is nothing left to answer.
    fn fulfill_json(&mut self, request: &serde_json::Value, body: &str) {
        let answer = self.browser.page().try_call_session(
            "Fetch.fulfillRequest",
            serde_json::json!({
                "requestId": request["requestId"],
                "responseCode": 200,
                "responseHeaders": [{ "name": "content-type", "value": "application/json" }],
                "body": base64::engine::general_purpose::STANDARD.encode(body),
            }),
        );
        if let Err(error) = answer
            && error["message"] != "Invalid InterceptionId."
        {
            panic!("Fetch.fulfillRequest failed: {error}");
        }
    }

    /// Waits for the sidebar to list `rows` recent conversations, answering every
    /// held request with `recents` meanwhile: the app reads the list again as its
    /// session settles.
    fn wait_for_recent_rows(&mut self, rows: usize, recents: Option<&str>) {
        let started = Instant::now();
        loop {
            let layout = self.sidebar_layout(Theme::Light);
            if let Some(recents) = recents {
                self.held.extend(self.browser.take_paused_requests());
                for request in std::mem::take(&mut self.held) {
                    self.fulfill_json(&request, recents);
                }
            }
            match layout {
                Some(layout) if layout.recent_rows == rows => return,
                seen => assert!(
                    started.elapsed() < Duration::from_secs(20),
                    "the sidebar did not list {rows} recent conversations: {seen:?}"
                ),
            }
            std::thread::sleep(Duration::from_millis(50));
        }
    }

    /// The page as it stands, as PNG bytes.
    fn screenshot(&mut self) -> Vec<u8> {
        let shot = self.call(
            "Page.captureScreenshot",
            serde_json::json!({ "format": "png" }),
        );
        let encoded = shot["result"]["data"].as_str().expect("a screenshot");
        base64::engine::general_purpose::STANDARD
            .decode(encoded)
            .expect("a screenshot in base64")
    }

    /// Saves the page as it stands, when `PHYSICIAN_PROOF_SHOTS` names a folder for it.
    fn save_picture(&mut self, name: &str) {
        let Some(folder) = env::var_os("PHYSICIAN_PROOF_SHOTS") else {
            return;
        };
        let png = self.screenshot();
        let folder = PathBuf::from(folder);
        fs::create_dir_all(&folder).expect("the screenshot folder");
        fs::write(folder.join(format!("{name}.png")), png).expect("the screenshot");
    }

    /// With `PHYSICIAN_PROOF_SHOTS` set, saves `state` scrolled to `anchor` at desktop and phone
    /// sizes in light and dark, then returns the page to desktop light. The phone shots collapse
    /// the sidebar first, which otherwise covers most of a phone's width.
    fn save_pictures(&mut self, state: &str, anchor: &str) {
        if env::var_os("PHYSICIAN_PROOF_SHOTS").is_none() {
            return;
        }
        for (size, width, height) in [("desktop", 1440, 900), ("phone", 390, 844)] {
            self.set_viewport(width, height, true);
            for theme in ["light", "dark"] {
                self.step("setTheme", serde_json::json!([theme]));
                self.step("scrollToStart", serde_json::json!([anchor]));
                std::thread::sleep(Duration::from_millis(400));
                self.save_picture(&format!("{state}-{size}-{theme}"));
            }
        }
        self.set_viewport(1440, 900, true);
        self.step("setTheme", serde_json::json!(["light"]));
    }

    /// Presses Tab until the element `selector` names has focus.
    fn tab_to(&mut self, selector: &str) {
        for _ in 0..120 {
            self.press_key(Key::TAB);
            if self.step("hasFocus", serde_json::json!([selector])) == true {
                return;
            }
        }
        panic!("Tab never reached {selector}");
    }

    /// Waits out the browser's smooth scroll for `selector`'s scroll position to answer `arrived`.
    fn wait_for_scroll(&mut self, selector: &str, arrived: impl Fn(f64) -> bool) {
        let deadline = Instant::now() + Duration::from_secs(5);
        loop {
            let left = self.step("scrollLeft", serde_json::json!([selector]));
            if left.as_f64().is_some_and(&arrived) {
                return;
            }
            assert!(
                Instant::now() < deadline,
                "{selector} never reached the scroll position it should: {left}"
            );
            std::thread::sleep(Duration::from_millis(50));
        }
    }

    fn set_viewport(&mut self, width: u32, height: u32, reduced_motion: bool) {
        self.call(
            "Emulation.setDeviceMetricsOverride",
            serde_json::json!({
                "width": width,
                "height": height,
                "deviceScaleFactor": 1,
                "mobile": width < 600,
            }),
        );
        self.call(
            "Emulation.setEmulatedMedia",
            serde_json::json!({
                "features": [{
                    "name": "prefers-reduced-motion",
                    "value": if reduced_motion { "reduce" } else { "no-preference" },
                }]
            }),
        );
    }
}

/// The JSON body of a held request.
fn held_body(request: &serde_json::Value) -> serde_json::Value {
    let text = match request["request"]["postData"].as_str() {
        Some(text) => text.to_owned(),
        None => request["request"]["postDataEntries"]
            .as_array()
            .into_iter()
            .flatten()
            .filter_map(|entry| entry["bytes"].as_str())
            .filter_map(|bytes| base64::engine::general_purpose::STANDARD.decode(bytes).ok())
            .map(|bytes| String::from_utf8_lossy(&bytes).into_owned())
            .collect(),
    };
    serde_json::from_str(&text).unwrap_or(serde_json::Value::Null)
}

/// Real first-source path (`load_synthetic_practice_with_understanding` ->
/// `PracticeJobHost` -> `GET /patient-workspaces/{id}` -> `physician-app` in
/// Chromium): shows in-progress work before completion, then renders the saved
/// summary, material gaps, and interactive citation evidence while keeping
/// Coda's private `OperatingIndex` completely absent from wire responses and DOM.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore = "browser qualification; run explicitly with --ignored"]
async fn first_source_path_renders_saved_patient_summary_and_evidence_without_private_operating_index()
 {
    use local_product::initial_understanding::{
        HealthSystemStanding, IndexedAnchor, KnownHealthSystem, OperatingIndex,
        SAVE_UNDERSTANDING_TOOL, SaveUnderstandingArgs, SummaryClaimInput, job_key,
    };
    use local_product::physician_sources::{READ_TOOL, SEARCH_TOOL, workspace_id};
    use local_product::runtime::concerns::ExactRecordReference;
    use local_product::runtime::engine;
    use local_product::runtime::jobs::{PracticeJobHost, PracticeScope};

    let _guard = test_lock().lock().await;
    let database = scale::Database::create("first_source_browser")
        .await
        .expect("browser proof database");
    scale::staging_account(&database.url).await;
    let site = build_site();

    // 1. Build a synthetic FHIR patient bank on disk and run the real first-source loader!
    let bank_dir = env::temp_dir().join(format!("coda-first-source-bank-{}", uuid::Uuid::new_v4()));
    let _ = fs::remove_dir_all(&bank_dir);
    let chart_dir = bank_dir.join("patients/SYN-FIRST-01");
    fs::create_dir_all(&chart_dir).unwrap();

    let patient_ndjson = serde_json::json!({
        "resourceType": "Patient",
        "id": "SYN-FIRST-01",
        "meta": { "lastUpdated": "2026-08-25T10:00:00Z" },
        "identifier": [{ "system": "urn:oid:2.16.840.1.113883.19.5", "value": "00091402" }],
        "name": [{ "family": "Navarro", "given": ["Sofia"] }],
        "gender": "female",
        "birthDate": "1968-05-19"
    });
    let note_text = "CONSULTATION NOTE (2026-08-20): Persistent atrial fibrillation anticoagulated with apixaban 5 mg twice daily. Prior ablation operative report from Harborview Electrophysiology Center has not yet been received.";
    let note_ndjson = serde_json::json!({
        "resourceType": "DocumentReference",
        "id": "consult-afib-2026",
        "date": "2026-08-20T14:30:00Z",
        "content": [{
            "attachment": {
                "contentType": "text/plain; charset=utf-8",
                "data": base64::engine::general_purpose::STANDARD.encode(note_text.as_bytes()),
            }
        }]
    });
    let lab_ndjson = serde_json::json!({
        "resourceType": "Observation",
        "id": "creatinine-2026-08",
        "effectiveDateTime": "2026-08-22T09:15:00Z",
        "code": { "text": "Serum Creatinine" },
        "valueQuantity": { "value": 1.12, "unit": "mg/dL" }
    });

    fs::write(
        chart_dir.join("Patient.ndjson"),
        format!("{}\n", serde_json::to_string(&patient_ndjson).unwrap()),
    )
    .unwrap();
    fs::write(
        chart_dir.join("DocumentReference.ndjson"),
        format!("{}\n", serde_json::to_string(&note_ndjson).unwrap()),
    )
    .unwrap();
    fs::write(
        chart_dir.join("Observation.ndjson"),
        format!("{}\n", serde_json::to_string(&lab_ndjson).unwrap()),
    )
    .unwrap();

    let store = local_product::runtime::Store::connect(&database.url)
        .await
        .unwrap();
    let auth = physician_auth::PostgresAuthStore::connect_without_migrations(&database.url)
        .await
        .unwrap();
    let skills = local_product::physician_chat::PhysicianSkills::configured(None, false).unwrap();
    let roles = scale::models::roles();

    let load_summary = physician_sample::chart_load::load_synthetic_practice_with_understanding(
        &bank_dir, &auth, &store, &roles, &skills,
    )
    .await
    .expect("first-source synthetic practice load succeeds");
    assert_eq!(load_summary.patients, 1);
    assert_eq!(load_summary.records, 3);
    assert!(load_summary.failed.is_empty(), "{:?}", load_summary.failed);

    let db_url = database.url.clone();
    let server = common::serve_isolated(|| {
        // SAFETY: `test_lock()` and `serve_isolated`'s start lock are both held.
        unsafe {
            env::set_var("PHYSICIAN_SITE_DIR", &site);
            env::set_var("PHYSICIAN_DATABASE_URL", &db_url);
            env::set_var("PHYSICIAN_AUTH", "required");
            env::set_var("PHYSICIAN_AUTH_PROOF_TOKENS", "1");
            env::remove_var("PHYSICIAN_UPSTREAM");
        }
    })
    .await;
    let base = format!("http://{}", server.address);

    let workspace = workspace_id(physician_sample::synthetic_chart::SOURCE_ID, "SYN-FIRST-01");
    let job_scope = PracticeScope {
        practice: physician_sample::STAGING_PRACTICE_ID.to_owned(),
        workspace: workspace.clone(),
        job_id: job_key(&workspace, 1),
    };
    let pending_job = store
        .practice_job(&job_scope)
        .await
        .unwrap()
        .expect("first-source load started InitialUnderstanding practice job");
    assert_eq!(pending_job.sources_version, 1);

    // 2. Sign in via Chromium and verify in-progress `Building initial patient summary` appears BEFORE completion!
    let mut browser = Tab::open(&base);
    browser.wait_for_text("Work email", Duration::from_secs(20));
    browser.request_sign_in("physician@codahealth.test");
    browser.wait_for_text("Nothing needs you right now.", Duration::from_secs(20));
    let bearer = browser
        .eval("localStorage.getItem('coda.physician.session')")
        .as_str()
        .expect("session bearer token")
        .to_owned();

    let patient_url = format!("{base}/?patient={workspace}");
    browser.navigate(&patient_url);
    browser.wait_for_text("Sofia Navarro", Duration::from_secs(20));
    browser.wait_for_text("Building initial patient summary", Duration::from_secs(10));
    std::thread::sleep(Duration::from_millis(300));
    browser.save_picture("ac5-patient-summary-building");

    // 3. Execute the real `PracticeJobHost` on the triggered job, saving both a sentinel-bearing
    //    private `OperatingIndex` and a cited `PatientSummary` with material gaps!
    let claim = store
        .claim_practice_job(&job_scope, 60, &auth)
        .await
        .unwrap()
        .expect("claim triggered initial understanding job");

    struct FirstSourceBrowserProvider {
        turn: usize,
        note_rev: Option<String>,
        lab_rev: Option<String>,
    }
    impl engine::recipe::Provider for FirstSourceBrowserProvider {
        async fn exchange(
            &mut self,
            request: &engine::recipe::Request,
            _: &engine::Cancellation,
            _: &engine::RunLimits,
            _: &mut dyn engine::recipe::Observer,
        ) -> Result<engine::Turn, engine::Cause> {
            self.turn += 1;
            match self.turn {
                1 => Ok(engine::Turn {
                    items: vec![engine::Item::Call(engine::Invocation {
                        id: "browse-chart".into(),
                        name: SEARCH_TOOL.into(),
                        arguments: serde_json::json!({
                            "query": "",
                            "record_type": null,
                            "after": null
                        })
                        .to_string(),
                    })],
                    receipt: serde_json::json!({"turn": 1}),
                    usage: Default::default(),
                }),
                2 => {
                    let search_json: serde_json::Value = request
                        .items
                        .iter()
                        .find_map(|item| match item {
                            engine::Item::MaterializedResult { call_id, output }
                                if call_id == "browse-chart" =>
                            {
                                serde_json::from_str(output).ok()
                            }
                            _ => None,
                        })
                        .expect("search results materialized");
                    let results = search_json["results"].as_array().expect("results array");
                    let find_rev = |cand: &str| -> String {
                        results
                            .iter()
                            .find(|r| r["reference"]["candidate_id"] == cand)
                            .and_then(|r| r["reference"]["revision"].as_str())
                            .unwrap_or_else(|| panic!("missing candidate {cand} in {results:?}"))
                            .to_owned()
                    };
                    let note_rev = find_rev("synthetic-charts/DocumentReference/consult-afib-2026");
                    let lab_rev = find_rev("synthetic-charts/Observation/creatinine-2026-08");
                    self.note_rev = Some(note_rev.clone());
                    self.lab_rev = Some(lab_rev.clone());

                    Ok(engine::Turn {
                        items: vec![engine::Item::Call(engine::Invocation {
                            id: "read-sources".into(),
                            name: READ_TOOL.into(),
                            arguments: serde_json::json!({
                                "references": [
                                    {
                                        "candidate_id": "synthetic-charts/DocumentReference/consult-afib-2026",
                                        "revision": note_rev,
                                        "cursor": null
                                    },
                                    {
                                        "candidate_id": "synthetic-charts/Observation/creatinine-2026-08",
                                        "revision": lab_rev,
                                        "cursor": null
                                    }
                                ],
                                "max_result_bytes": 32_768
                            })
                            .to_string(),
                        })],
                        receipt: serde_json::json!({"turn": 2}),
                        usage: Default::default(),
                    })
                }
                3 => {
                    let note_rev = self.note_rev.clone().unwrap();
                    let lab_rev = self.lab_rev.clone().unwrap();
                    let args = SaveUnderstandingArgs {
                        expected_revision: Some(0),
                        operating_index: OperatingIndex {
                            working_overview: "PRIVATE_INDEX_ORIENTATION_7741: Persistent AFib on apixaban; Harborview EP ablation report missing.".into(),
                            known_health_systems: vec![
                                KnownHealthSystem {
                                    name: "Synthetic chart".into(),
                                    standing: HealthSystemStanding::Connected {
                                        source_id: physician_sample::synthetic_chart::SOURCE_ID.into(),
                                    },
                                    note: "PRIVATE_CONNECTED_NOTE_7741".into(),
                                },
                                KnownHealthSystem {
                                    name: "PRIVATE_OUTSIDE_EP_SYSTEM_7741".into(),
                                    standing: HealthSystemStanding::MentionedInRecords,
                                    note: "PRIVATE_EP_ABLATION_NOTE_7741".into(),
                                },
                            ],
                            open_threads: vec![
                                "PRIVATE_OPERATING_GAP_7741".into(),
                                "PRIVATE_ANCHOR_REASON_7741".into(),
                                "PRIVATE_MED_VERIFICATION_CUE_7741".into(),
                            ],
                            anchors: vec![
                                IndexedAnchor {
                                    reference: ExactRecordReference {
                                        candidate_id: "synthetic-charts/DocumentReference/consult-afib-2026".into(),
                                        revision: note_rev.clone(),
                                    },
                                    source_id: physician_sample::synthetic_chart::SOURCE_ID.into(),
                                    topic: "PRIVATE_ANCHOR_AFIB_7741".into(),
                                },
                            ],
                        },
                        claims: vec![
                            SummaryClaimInput {
                                text: "Persistent atrial fibrillation anticoagulated with apixaban 5 mg twice daily".into(),
                                citations: vec![ExactRecordReference {
                                    candidate_id: "synthetic-charts/DocumentReference/consult-afib-2026".into(),
                                    revision: note_rev,
                                }],
                            },
                            SummaryClaimInput {
                                text: "Serum creatinine was 1.12 mg/dL on 2026-08-22".into(),
                                citations: vec![ExactRecordReference {
                                    candidate_id: "synthetic-charts/Observation/creatinine-2026-08".into(),
                                    revision: lab_rev,
                                }],
                            },
                        ],
                        material_gaps: vec![
                            "Harborview Electrophysiology Center ablation operative report not yet received".into(),
                        ],
                        scope_note: None,
                    };
                    Ok(engine::Turn {
                        items: vec![engine::Item::Call(engine::Invocation {
                            id: "save-understanding".into(),
                            name: SAVE_UNDERSTANDING_TOOL.into(),
                            arguments: serde_json::to_string(&args).unwrap(),
                        })],
                        receipt: serde_json::json!({"turn": 3}),
                        usage: Default::default(),
                    })
                }
                4 => Ok(engine::Turn {
                    items: vec![engine::Item::assistant(
                        "Initial patient understanding saved.",
                    )],
                    receipt: serde_json::json!({"turn": 4}),
                    usage: Default::default(),
                }),
                other => panic!("unexpected turn {other}"),
            }
        }
    }

    let mut host = PracticeJobHost {
        store: store.clone(),
        claim,
        authority: &auth,
    };
    host.run(
        &mut FirstSourceBrowserProvider {
            turn: 0,
            note_rev: None,
            lab_rev: None,
        },
        engine::RunLimits::default(),
        &engine::Cancellation::default(),
    )
    .await
    .expect("first-source initial understanding run completes");

    // 4. Inspect raw HTTP wire response `GET /patient-workspaces/{workspace}`:
    let http = reqwest::Client::new();
    let ws_resp = http
        .get(format!("{base}/patient-workspaces/{workspace}"))
        .bearer_auth(&bearer)
        .send()
        .await
        .expect("fetch patient workspace wire JSON");
    assert_eq!(ws_resp.status(), reqwest::StatusCode::OK);
    let raw_wire = ws_resp.text().await.unwrap();
    let wire_json: serde_json::Value = serde_json::from_str(&raw_wire).unwrap();

    assert!(
        wire_json["automation_run"].is_null(),
        "completed initial understanding must clear automation_run: {raw_wire}"
    );
    assert_eq!(
        wire_json["patient_synopsis"]["version_id"],
        "understanding-v1"
    );
    assert!(
        wire_json["patient_synopsis"]["authored_text"]
            .as_str()
            .unwrap_or_default()
            .contains(
                "Harborview Electrophysiology Center ablation operative report not yet received"
            )
    );
    for forbidden in [
        "operating_index",
        "PRIVATE_INDEX_ORIENTATION_7741",
        "PRIVATE_CONNECTED_NOTE_7741",
        "PRIVATE_OUTSIDE_EP_SYSTEM_7741",
        "PRIVATE_EP_ABLATION_NOTE_7741",
        "PRIVATE_ANCHOR_AFIB_7741",
        "PRIVATE_ANCHOR_REASON_7741",
        "PRIVATE_OPERATING_GAP_7741",
        "PRIVATE_MED_VERIFICATION_CUE_7741",
    ] {
        assert!(
            !raw_wire.contains(forbidden),
            "wire workspace response leaked private operating index token {forbidden}:\n{raw_wire}"
        );
    }

    // 5. Reload the patient page in Chromium: verify summary, material gaps, and citation inspection!
    browser.navigate(&patient_url);
    browser.wait_for_text("Patient summary", Duration::from_secs(20));
    browser.wait_for_text(
        "Harborview Electrophysiology Center ablation operative report not yet received",
        Duration::from_secs(10),
    );
    std::thread::sleep(Duration::from_millis(300));
    browser.save_picture("ac5-patient-summary-loaded");

    let dom_summary = browser
        .eval("document.documentElement.outerHTML")
        .as_str()
        .unwrap_or_default()
        .to_owned();
    assert!(
        !dom_summary.contains("Building initial patient summary"),
        "completed run banner remained on DOM:\n{dom_summary}"
    );
    assert!(
        dom_summary.contains("Persistent atrial fibrillation anticoagulated with apixaban 5 mg twice daily")
            && dom_summary.contains("Material gaps: Harborview Electrophysiology Center ablation operative report not yet received"),
        "patient summary prose or material gaps missing in DOM:\n{dom_summary}"
    );

    // Click citation chip 1 inside Patient summary -> side-panel source inspection opens!
    browser.click("section[aria-label=\"Patient summary\"] button[aria-label*=\"Source 1:\"]");
    browser.wait_for_text("2026-08-20", Duration::from_secs(10));
    browser.wait_for_text(
        "CONSULTATION NOTE (2026-08-20): Persistent atrial fibrillation",
        Duration::from_secs(10),
    );
    let source_pane_text = browser
        .eval("document.querySelector('.source-reading-pane')?.innerText ?? ''")
        .as_str()
        .unwrap_or_default()
        .to_owned();
    assert!(
        !source_pane_text.contains("Prepared by Coda"),
        "native source record must not say Prepared by Coda:\n{source_pane_text}"
    );
    std::thread::sleep(Duration::from_millis(300));
    browser.save_picture("ac5-patient-summary-citation-open");

    let dom_after_open = browser
        .eval("document.documentElement.outerHTML")
        .as_str()
        .unwrap_or_default()
        .to_owned();
    for forbidden in [
        "operating_index",
        "PRIVATE_INDEX_ORIENTATION_7741",
        "PRIVATE_CONNECTED_NOTE_7741",
        "PRIVATE_OUTSIDE_EP_SYSTEM_7741",
        "PRIVATE_EP_ABLATION_NOTE_7741",
        "PRIVATE_ANCHOR_AFIB_7741",
        "PRIVATE_ANCHOR_REASON_7741",
        "PRIVATE_OPERATING_GAP_7741",
        "PRIVATE_MED_VERIFICATION_CUE_7741",
    ] {
        assert!(
            !dom_after_open.contains(forbidden),
            "Chromium DOM leaked private operating index token {forbidden}:\n{dom_after_open}"
        );
    }

    drop(browser);
    let _ = fs::remove_dir_all(&bank_dir);
    unsafe {
        env::remove_var("PHYSICIAN_DATABASE_URL");
        env::remove_var("PHYSICIAN_AUTH");
        env::remove_var("PHYSICIAN_AUTH_PROOF_TOKENS");
    }
}

/// End-to-end browser verification of composer enter/shift-enter/ctrl-enter keys,
/// server-backed MessageQueue (Send next, inline Edit/Update, Delete), WorkTrace
/// steps without tool names staying open across Working -> Finished and across
/// reload, Stop returning queued items to the composer, CommandSlot single
/// automatic retry on lost receipt, and browser IANA timezone forwarding.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore = "browser qualification; run explicitly with --ignored"]
async fn p1_composer_queue_stop_steps_retry_and_provenance() {
    let _guard = test_lock().lock().await;
    let site = build_site();
    let rounds = serde_json::json!([
        [{
            "name": "search_patient_information",
            "arguments": {"query": "sodium", "record_type": null, "after": null}
        }]
    ])
    .to_string();
    let server = serve_chat_browser_with_stub(&site, 5_000, &rounds, scale::FIXED_TEXT).await;
    let address = server.address;
    let patient = "pw-39e51534-6ce4-43cf-b5f8-7c1ca6252d8e";
    let chat = "chat-p1-composer-queue";
    let chat_url = format!("http://{address}/?patient={patient}&chat={chat}");

    let mut tab = Tab::open(&format!("http://{address}/"));
    tab.wait_for_text("Work email", Duration::from_secs(20));
    tab.request_sign_in("physician@codahealth.test");
    tab.wait_for_text("Nothing needs you right now.", Duration::from_secs(20));
    tab.navigate(&chat_url);
    tab.wait_for_text(
        "What would you like Coda to review?",
        Duration::from_secs(20),
    );

    // 1. Shortcuts dialog matches shipped shortcuts (no false dictation Esc text).
    tab.click_named("button", "Keyboard shortcuts");
    let shortcuts_dom = tab.wait_for_text("Send next after Coda's reply", Duration::from_secs(10));
    assert!(
        !shortcuts_dom.contains("Esc cancels dictation"),
        "stale dictation Esc text remained in ShortcutsDialog:\n{shortcuts_dom}"
    );
    tab.click("dialog button[aria-label=\"Close\"]");
    let dialog_deadline = Instant::now() + Duration::from_secs(5);
    while tab.eval("Boolean(document.querySelector('dialog'))") == serde_json::json!(true) {
        assert!(
            Instant::now() < dialog_deadline,
            "ShortcutsDialog did not close"
        );
        tokio::time::sleep(Duration::from_millis(25)).await;
    }

    let send_enter = |tab: &mut Tab, modifiers: i64, with_text: bool| {
        tab.step("focusEnd", serde_json::json!(["textarea.composer-input"]));
        assert_eq!(
            tab.step("hasFocus", serde_json::json!(["textarea.composer-input"])),
            serde_json::json!(true),
            "composer did not take focus before key dispatch"
        );
        let mut down = serde_json::json!({
            "type": "keyDown",
            "key": "Enter",
            "code": "Enter",
            "windowsVirtualKeyCode": 13,
            "nativeVirtualKeyCode": 13,
            "modifiers": modifiers
        });
        if with_text {
            down["text"] = serde_json::json!("\r");
            down["unmodifiedText"] = serde_json::json!("\r");
        }
        tab.call("Input.dispatchKeyEvent", down);
        tab.call(
            "Input.dispatchKeyEvent",
            serde_json::json!({
                "type": "keyUp",
                "key": "Enter",
                "code": "Enter",
                "windowsVirtualKeyCode": 13,
                "nativeVirtualKeyCode": 13,
                "modifiers": modifiers
            }),
        );
    };

    // 2. Shift+Enter inserts a newline without sending; bare Enter sends.
    tab.type_composer("First line");
    send_enter(&mut tab, 8, true);
    tab.call(
        "Input.insertText",
        serde_json::json!({ "text": "Second line" }),
    );
    tab.eval("(() => { const b = document.querySelector('textarea.composer-input'); b.dispatchEvent(new Event('input', { bubbles: true })); return true; })()");
    let composer_now = tab.composer().0;
    assert!(
        composer_now.contains("First line\nSecond line")
            || composer_now.contains("First line\rSecond line"),
        "Shift+Enter did not add a newline without sending: {composer_now:?}"
    );
    send_enter(&mut tab, 0, false);

    // 3. While working: WorkingMark + WorkTrace + Send next / Send now toolbar buttons.
    tab.wait_for_text("Send next", Duration::from_secs(15));
    let working_buttons = tab.eval(
        "[...document.querySelectorAll('.composer-toolbar button')].map(b => b.textContent.trim())",
    );
    assert_eq!(
        working_buttons,
        serde_json::json!(["Dictate", "Send next", "Send now"]),
        "unexpected working composer buttons: {working_buttons}"
    );
    assert_eq!(
        tab.eval("Boolean(document.querySelector('.working-dots'))"),
        serde_json::json!(true),
        "WorkingMark pulsing dots missing while working"
    );

    // WorkTrace appears with physician-readable steps and no raw tool names; open it before finish!
    tab.wait_for_text("patient chart", Duration::from_secs(15));
    tab.click(".work-trace-line");
    assert_eq!(
        tab.eval("document.querySelector('.work-trace')?.getAttribute('data-state')"),
        serde_json::json!("open"),
        "WorkTrace did not open on click while working"
    );
    let working_dom = tab
        .eval("document.documentElement.outerHTML")
        .as_str()
        .unwrap_or_default()
        .to_owned();
    for internal in [
        "search_patient_information",
        "read_patient_sources",
        "finish_answer",
    ] {
        assert!(
            !working_dom.contains(internal),
            "raw tool name {internal} leaked into DOM:\n{working_dom}"
        );
    }

    // Queue 2 follow-ups (one via Ctrl+Enter, one via Send next button), edit the first, delete the second.
    tab.type_composer("Queued alpha");
    send_enter(&mut tab, 2, false);
    tab.wait_for_text("Sends after Coda's reply", Duration::from_secs(10));
    tab.wait_for_text("Queued alpha", Duration::from_secs(10));

    tab.type_composer("Queued beta");
    tab.click_named(".composer-toolbar button", "Send next");
    tab.wait_for_text("Queued beta", Duration::from_secs(10));
    tab.save_picture("p1-working-trace-and-queue");

    tab.click_named(".message-queue button", "Edit");
    tab.wait_for_text("Update", Duration::from_secs(5));
    tab.eval(
        "(() => { const box = document.querySelector('textarea.message-editor-input'); const d = Object.getOwnPropertyDescriptor(HTMLTextAreaElement.prototype, 'value'); d.set.call(box, 'Queued alpha edited'); box.dispatchEvent(new Event('input', { bubbles: true })); return true; })()",
    );
    tab.click_named(".message-queue button", "Update");
    tab.wait_for_text("Queued alpha edited", Duration::from_secs(10));

    assert_eq!(
        tab.eval(
            "(() => { const msgs = [...document.querySelectorAll('.message-queue .message')]; const btns = msgs[1] ? [...msgs[1].querySelectorAll('button')] : []; const del = btns.find(b => b.textContent.trim() === 'Delete'); del?.click(); return Boolean(del); })()",
        ),
        serde_json::json!(true),
        "Delete button missing on second queued item"
    );
    let start_del = Instant::now();
    while start_del.elapsed() < Duration::from_secs(10) {
        let html = tab
            .eval("document.querySelector('.message-queue')?.innerText ?? ''")
            .as_str()
            .unwrap_or_default()
            .to_owned();
        if !html.contains("Queued beta") {
            break;
        }
        tokio::time::sleep(Duration::from_millis(50)).await;
    }

    // 4. Both Turn 1 and drained queued Turn 2 finish; Turn 1 WorkTrace stays OPEN across Working -> Finished!
    tab.wait_for_text(scale::FIXED_TEXT, Duration::from_secs(30));
    let trace_deadline = Instant::now() + Duration::from_secs(30);
    loop {
        let count_finished =
            tab.eval("document.querySelectorAll('[aria-label=\"Conversation\"] .message').length");
        if count_finished.as_u64().unwrap_or(0) >= 4
            && tab.eval("document.querySelectorAll('.message-queue').length")
                == serde_json::json!(0)
        {
            break;
        }
        assert!(
            Instant::now() < trace_deadline,
            "queued turn did not drain and finish cleanly"
        );
        tokio::time::sleep(Duration::from_millis(100)).await;
    }
    assert_eq!(
        tab.eval("document.querySelector('.work-trace')?.getAttribute('data-state')"),
        serde_json::json!("open"),
        "WorkTrace did not stay open across Working -> Finished transition"
    );
    tab.save_picture("p1-trace-still-open-after-finish");

    // 5. Reopen conversation across reload -> WorkTrace still renders saved steps in start order.
    tab.reload();
    tab.wait_for_text(scale::FIXED_TEXT, Duration::from_secs(20));
    tab.wait_for_text("Queued alpha edited", Duration::from_secs(15));
    assert!(
        tab.eval("document.querySelectorAll('.work-trace').length")
            .as_u64()
            .unwrap_or(0)
            >= 1,
        "WorkTrace missing after conversation reopen"
    );

    // 6. Stop during active work returns queued messages directly into the composer.
    tab.type_composer("Stop target turn");
    send_enter(&mut tab, 0, false);
    tab.wait_for_text("Stop target turn", Duration::from_secs(10));
    tab.wait_for_text("Send next", Duration::from_secs(10));
    tab.type_composer("Return this on Stop");
    send_enter(&mut tab, 2, false);
    tab.wait_for_text("Return this on Stop", Duration::from_secs(10));
    tab.click_named(".conversation-mechanical button", "Stop");
    tab.wait_composer("Return this on Stop", Duration::from_secs(15));
    tab.save_picture("p1-stop-returned-queue");
    tab.type_composer("");

    // 7. Auto-retry ONCE with the same idempotency_key when response is dropped, plus browser timezone.
    let expected_tz = tab
        .eval("Intl.DateTimeFormat().resolvedOptions().timeZone")
        .as_str()
        .unwrap_or("UTC")
        .to_owned();
    tab.call(
        "Fetch.enable",
        serde_json::json!({ "patterns": [{ "urlPattern": "*/commands", "requestStage": "Response" }] }),
    );
    tab.type_composer("Auto retry once message");
    send_enter(&mut tab, 0, false);
    let dropped_req = tab.held_request("POST");
    let first_key = held_body(&dropped_req)["idempotency_key"].clone();
    assert_eq!(
        held_body(&dropped_req)["payload"]["physician_time_zone"]
            .as_str()
            .unwrap_or_default(),
        expected_tz,
        "command did not carry browser IANA timezone"
    );
    tab.call(
        "Fetch.failRequest",
        serde_json::json!({ "requestId": dropped_req["requestId"], "errorReason": "Failed" }),
    );
    tab.call(
        "Fetch.enable",
        serde_json::json!({ "patterns": [{ "urlPattern": "*/commands", "requestStage": "Request" }] }),
    );
    let retry_req = tab.held_request("POST");
    assert_eq!(
        held_body(&retry_req)["idempotency_key"],
        first_key,
        "automatic retry did not preserve idempotency_key"
    );
    tab.release(&retry_req);
    tab.call("Fetch.disable", serde_json::json!({}));
    tab.release_all();
    tab.wait_for_text("Auto retry once message", Duration::from_secs(15));
    tab.wait_for_text(scale::FIXED_TEXT, Duration::from_secs(20));

    // 8. Cross-patient Stop isolation: while Patient A's `work.stop` is in flight,
    // navigate in-app (no reload) to another patient and ensure neither Patient A's
    // returned queue text nor any Stop slot notice bleeds into Patient B.
    tab.type_composer("Stop cross patient turn");
    send_enter(&mut tab, 0, false);
    tab.wait_for_text("Stop cross patient turn", Duration::from_secs(10));
    tab.wait_for_text("Send next", Duration::from_secs(10));
    tab.type_composer("Patient A private queued note");
    send_enter(&mut tab, 2, false);
    tab.wait_for_text("Patient A private queued note", Duration::from_secs(10));

    tab.call(
        "Fetch.enable",
        serde_json::json!({ "patterns": [{ "urlPattern": "*/commands", "requestStage": "Request" }] }),
    );
    tab.click_named(".conversation-mechanical button", "Stop");
    let held_stop = tab.held_request("POST");

    // In-app switch to another patient via Find patient while Stop is in flight.
    assert_eq!(
        tab.eval("(() => { const b = [...document.querySelectorAll('button')].find(x => x.textContent.includes('Find patient')); b?.click(); return Boolean(b); })()"),
        serde_json::json!(true),
        "Find patient button not found"
    );
    let search_deadline = Instant::now() + Duration::from_secs(10);
    while tab
        .eval("document.querySelectorAll('.dialog-results button').length")
        .as_u64()
        .unwrap_or(0)
        < 2
    {
        assert!(
            Instant::now() < search_deadline,
            "Find patient did not list 2 patients"
        );
        tokio::time::sleep(Duration::from_millis(50)).await;
    }
    assert_eq!(
        tab.eval("(() => { const rows = document.querySelectorAll('.dialog-results button'); const other = [...rows].find(b => !b.innerText.includes('Mary Louis')); other?.click(); return Boolean(other); })()"),
        serde_json::json!(true),
        "could not open a second patient via Find patient"
    );
    let patient_b_deadline = Instant::now() + Duration::from_secs(10);
    while {
        let title = tab
            .eval("document.querySelector('h1.workspace-title')?.innerText ?? ''")
            .as_str()
            .unwrap_or("")
            .trim()
            .to_owned();
        title.is_empty() || title == "Mary Louis"
    } {
        assert!(
            Instant::now() < patient_b_deadline,
            "did not switch to second patient"
        );
        tokio::time::sleep(Duration::from_millis(50)).await;
    }
    tab.wait_for_text("New conversation", Duration::from_secs(10));
    assert_eq!(
        tab.eval("(() => { const b = [...document.querySelectorAll('button')].find(x => x.textContent.includes('New conversation')); b?.click(); return Boolean(b); })()"),
        serde_json::json!(true),
        "New conversation button not found on second patient page"
    );
    tab.wait_composer("", Duration::from_secs(10));

    // Release Patient A's Stop while viewing Patient B.
    tab.release(&held_stop);
    tab.call("Fetch.disable", serde_json::json!({}));
    tab.release_all();
    tokio::time::sleep(Duration::from_millis(600)).await;

    assert_eq!(
        tab.composer().0,
        "",
        "Patient A returned queue leaked into Patient B composer"
    );
    let patient_b_dom = tab
        .eval("document.documentElement.innerText")
        .as_str()
        .unwrap_or_default()
        .to_owned();
    assert!(
        !patient_b_dom.contains("Patient A private queued note"),
        "Patient A queued PHI appeared on Patient B page:\n{patient_b_dom}"
    );
    assert!(
        !patient_b_dom.contains("Check again"),
        "Patient A slot notice leaked onto Patient B page:\n{patient_b_dom}"
    );
}

#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore = "browser and disposable Postgres qualification; run explicitly with --ignored"]
async fn p1_citations_panel_archive_discuss_dictation_and_finder() {
    use local_product::physician_sources::{
        MedicalRecordNumber, READ_TOOL, ReadablePart, RecordDate, SEARCH_TOOL, SourcePatient,
        SourceRevision,
    };
    use local_product::runtime::sources::SourcePull;
    use local_product::runtime::{Input, PhysicianHost, Scope, ShownSteps, Store, engine};
    use physician_auth::PostgresAuthStore;

    enum TurnBehavior {
        Uncited {
            text: &'static str,
        },
        TwoSources {
            candidate_a: String,
            candidate_b: String,
            round: usize,
        },
    }

    impl engine::recipe::Provider for TurnBehavior {
        async fn exchange(
            &mut self,
            _: &engine::recipe::Request,
            _: &engine::Cancellation,
            _: &engine::RunLimits,
            _: &mut dyn engine::recipe::Observer,
        ) -> Result<engine::Turn, engine::Cause> {
            let items = match self {
                TurnBehavior::Uncited { text } => vec![engine::Item::Call(engine::Invocation {
                    id: "call-finish-uncited".to_owned(),
                    name: local_product::physician_sources::citations::FINISH_TOOL.to_owned(),
                    arguments: serde_json::json!({
                        "answer": text,
                        "quotes": [],
                        "descriptions": [],
                        "basis": "Based on intake history.",
                        "cites_none": true,
                    })
                    .to_string(),
                })],
                TurnBehavior::TwoSources {
                    candidate_a,
                    candidate_b,
                    round,
                } => {
                    if *round == 0 {
                        *round += 1;
                        vec![
                            engine::Item::Call(engine::Invocation {
                                id: "call-search-two".to_owned(),
                                name: SEARCH_TOOL.to_owned(),
                                arguments: serde_json::json!({
                                    "query": "potassium echocardiogram",
                                    "record_type": null,
                                    "after": null,
                                })
                                .to_string(),
                            }),
                            engine::Item::Call(engine::Invocation {
                                id: "call-read-two".to_owned(),
                                name: READ_TOOL.to_owned(),
                                arguments: serde_json::json!({
                                    "references": [
                                        {"candidate_id": candidate_a, "revision": "rev-1", "cursor": null},
                                        {"candidate_id": candidate_b, "revision": "rev-2", "cursor": null},
                                    ],
                                    "max_result_bytes": 16_384,
                                })
                                .to_string(),
                            }),
                        ]
                    } else {
                        vec![engine::Item::Call(engine::Invocation {
                            id: "call-finish-two".to_owned(),
                            name: local_product::physician_sources::citations::FINISH_TOOL
                                .to_owned(),
                            arguments: serde_json::json!({
                                "answer": "Potassium is 5.4 mmol/L [S1] and LVEF is 55% [S2].",
                                "quotes": [
                                    {"label": "S1", "quote": "Potassium 5.4 mmol/L"},
                                    {"label": "S2", "quote": "Left ventricular ejection fraction 55%"},
                                ],
                                "descriptions": [],
                                "basis": "Based on Regional Lab BMP [S1] and preliminary Echocardiogram [S2].",
                                "cites_none": false,
                            })
                            .to_string(),
                        })]
                    }
                }
            };
            Ok(engine::Turn {
                items,
                receipt: serde_json::Value::Null,
                usage: Default::default(),
            })
        }
    }

    struct TestOpenAuthority;

    #[local_product::runtime::async_trait]
    impl local_product::runtime::Authority for TestOpenAuthority {
        async fn check(
            &mut self,
            _: &local_product::runtime::PublicationTransaction,
            _: &Scope,
            _: Option<&[u8]>,
        ) -> Result<(), String> {
            Ok(())
        }

        async fn lock_publication(
            &mut self,
            _: &local_product::runtime::PublicationTransaction,
            _: &Scope,
            _: Option<&[u8]>,
        ) -> Result<(), String> {
            Ok(())
        }
    }

    let _guard = test_lock().lock().await;
    let database = scale::Database::create("p1_b2_shell_browser")
        .await
        .expect("browser database");
    let site = build_site();
    unsafe {
        env::set_var("PHYSICIAN_SITE_DIR", &site);
        env::set_var("PHYSICIAN_AUTH", "required");
        env::set_var("PHYSICIAN_AUTH_PROOF_TOKENS", "1");
        env::remove_var("PHYSICIAN_UPSTREAM");
    }
    scale::staging_account(&database.url).await;

    let patient_a = physician_sample::WORKSPACE_IDS[0];
    let patient_b = physician_sample::WORKSPACE_IDS[1];
    let store = Store::connect(&database.url).await.expect("runtime store");

    let raw_lab = r#"{"test":"Basic metabolic panel","potassium_mmol_l":5.4,"status":"final"}"#;
    let raw_echo = r#"{"study":"Echocardiogram","lvef_percent":55,"status":"preliminary"}"#;
    let pull = SourcePull {
        source_id: "fixture-chart".to_owned(),
        source_label: "Regional Lab".to_owned(),
        patient: SourcePatient {
            source_patient_id: patient_a.to_owned(),
            display_name: "Tanvir Chowdhury".to_owned(),
            birth_date: Some("1964-11-02".to_owned()),
            sex_or_gender: Some("Male".to_owned()),
            medical_record_numbers: vec![MedicalRecordNumber {
                system: "Practice MRN".to_owned(),
                value: "00000003".to_owned(),
            }],
        },
        pulled_at: Some(1_787_846_400),
    };
    let rev_lab = SourceRevision {
        workspace: patient_a.to_owned(),
        source_id: "fixture-chart".to_owned(),
        candidate_id: "bmp-cite-v1".to_owned(),
        revision: "rev-1".to_owned(),
        record_type: "Laboratory result".to_owned(),
        dates: vec![RecordDate {
            name: "Collected".to_owned(),
            value: "2026-08-27".to_owned(),
        }],
        content: raw_lab.to_owned(),
        sha256: local_product::sha256_hex(raw_lab.as_bytes()),
        readable_parts: vec![ReadablePart {
            label: "Basic metabolic panel".to_owned(),
            text: "Potassium 5.4 mmol/L (reference 3.5-5.0 mmol/L), collected Tuesday morning."
                .to_owned(),
        }],
        provenance: serde_json::json!({
            "source_label": "Regional Lab",
            "title": "Basic metabolic panel",
            "status": "final",
        }),
        envelope: None,
    };
    let rev_echo = SourceRevision {
        workspace: patient_a.to_owned(),
        source_id: "fixture-chart".to_owned(),
        candidate_id: "echo-flagged-v1".to_owned(),
        revision: "rev-2".to_owned(),
        record_type: "Echocardiogram".to_owned(),
        dates: vec![RecordDate {
            name: "Recorded".to_owned(),
            value: "2026-08-26".to_owned(),
        }],
        content: raw_echo.to_owned(),
        sha256: local_product::sha256_hex(raw_echo.as_bytes()),
        readable_parts: vec![ReadablePart {
            label: "Findings".to_owned(),
            text: "Left ventricular ejection fraction 55%, mild concentric LVH.".to_owned(),
        }],
        provenance: serde_json::json!({
            "source_label": "Regional Lab",
            "title": "Echocardiogram",
            "status": "preliminary",
        }),
        envelope: None,
    };
    store
        .record_chart(patient_a, &[rev_lab, rev_echo], &pull)
        .await
        .expect("record clinical fixtures");

    let skills = local_product::physician_chat::PhysicianSkills::configured(None, false).unwrap();
    let chat_cited = "chat-p1b2-cited";
    let scope_cited = Scope {
        practice: physician_sample::STAGING_PRACTICE_ID.into(),
        physician: "staging-physician".into(),
        workspace: patient_a.into(),
        chat: chat_cited.into(),
    };
    // Turn 1: uncited (verifies Sources pill hides when answer cites zero sources).
    store
        .accept_authorized(
            &scope_cited,
            &Input {
                key: "turn-uncited-1".into(),
                text: "Any recent vitals check?".into(),
                timezone: "America/New_York".into(),
                scenario: None,
            },
            vec![7; 32],
            &skills,
            &scale::models::chat_recipe(),
            None,
        )
        .await
        .unwrap();
    let claim_1 = store.claim(&scope_cited, 60).await.unwrap().unwrap();
    PhysicianHost {
        store: store.clone(),
        claim: claim_1,
        authority: TestOpenAuthority,
        shown_steps: ShownSteps::new(()),
    }
    .run(
        &mut TurnBehavior::Uncited {
            text: "Routine vital signs are within expected limits.",
        },
        engine::RunLimits::default(),
        &engine::Cancellation::default(),
    )
    .await
    .unwrap();

    // Turn 2: cited across 2 sources (matched BMP + flagged preliminary Echocardiogram w/ unmatched quote).
    store
        .accept_authorized(
            &scope_cited,
            &Input {
                key: "turn-cited-2".into(),
                text: "Review potassium and echo results.".into(),
                timezone: "America/New_York".into(),
                scenario: None,
            },
            vec![7; 32],
            &skills,
            &scale::models::chat_recipe(),
            None,
        )
        .await
        .unwrap();
    let claim_2 = store.claim(&scope_cited, 60).await.unwrap().unwrap();
    PhysicianHost {
        store: store.clone(),
        claim: claim_2,
        authority: TestOpenAuthority,
        shown_steps: ShownSteps::new(()),
    }
    .run(
        &mut TurnBehavior::TwoSources {
            candidate_a: "bmp-cite-v1".to_owned(),
            candidate_b: "echo-flagged-v1".to_owned(),
            round: 0,
        },
        engine::RunLimits::default(),
        &engine::Cancellation::default(),
    )
    .await
    .unwrap();

    // Seed idle conversation on patient_b for AC2 Archive + Undo test.
    let chat_archive = "chat-p1b2-archive";
    let scope_archive = Scope {
        practice: physician_sample::STAGING_PRACTICE_ID.into(),
        physician: "staging-physician".into(),
        workspace: patient_b.into(),
        chat: chat_archive.into(),
    };
    store
        .accept_authorized(
            &scope_archive,
            &Input {
                key: "turn-archive-1".into(),
                text: "Archive candidate renal follow-up".into(),
                timezone: "America/New_York".into(),
                scenario: None,
            },
            vec![7; 32],
            &skills,
            &scale::models::chat_recipe(),
            None,
        )
        .await
        .unwrap();
    let claim_arch = store.claim(&scope_archive, 60).await.unwrap().unwrap();
    PhysicianHost {
        store: store.clone(),
        claim: claim_arch,
        authority: TestOpenAuthority,
        shown_steps: ShownSteps::new(()),
    }
    .run(
        &mut TurnBehavior::Uncited {
            text: "Creatinine remains stable at 1.1 mg/dL.",
        },
        engine::RunLimits::default(),
        &engine::Cancellation::default(),
    )
    .await
    .unwrap();

    let valkey = scale::Valkey::start().await;
    let redis_url = valkey.url();
    let bus: Arc<dyn physician_bus::Bus> = Arc::new(
        physician_bus::RedisBus::connect(&redis_url, None)
            .await
            .unwrap(),
    );
    let chat = Arc::new(physician_sample::durable::DurableChat::new(
        store,
        skills,
        &scale::models::roles(),
    ));
    let auth = Arc::new(
        PostgresAuthStore::connect_without_migrations(&database.url)
            .await
            .unwrap(),
    );
    let front = Arc::new(
        front_door::PostgresFrontDoorStore::connect_without_migrations(&database.url)
            .await
            .unwrap(),
    );
    let app = physician_sample::authed_router_on(
        physician_sample::durable_site(chat, bus.clone()),
        auth.clone(),
        front,
    )
    .await;
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    let server = tokio::spawn(async move { axum::serve(listener, app).await.unwrap() });
    wait_until_ready(addr).await;

    let base_url = format!("http://{addr}/");
    let mut tab = Tab::open(&format!("{base_url}?patient={patient_a}&chat={chat_cited}"));
    tab.set_viewport(1280, 800, false);
    tab.wait_for_text("Work email", Duration::from_secs(20));
    tab.request_sign_in("physician@codahealth.test");
    tab.wait_for_text("Potassium is 5.4 mmol/L", Duration::from_secs(20));
    tab.wait_for_text("Basis and limitations", Duration::from_secs(20));
    tab.wait_for_text(
        "Routine vital signs are within expected limits.",
        Duration::from_secs(10),
    );

    // AC1: Only cited turns show the `Sources` pill beside collapsed `Basis and limitations`.
    let sources_btn_count = tab.eval("document.querySelectorAll('button.sources-button').length");
    assert_eq!(
        sources_btn_count,
        serde_json::json!(1),
        "expected Sources pill strictly on cited answer turn (hidden on uncited turn); stage={}",
        tab.eval("document.querySelector('.glass-stage')?.innerText ?? ''")
    );
    assert!(
        tab.eval("document.querySelectorAll('.answer-ending').length")
            .as_u64()
            == Some(2)
    );

    // AC1: Hover citation [1] shows CitationPreview popover (`role="dialog"`) with quote (`Citations Code 6, 8`).
    let rect = tab.eval(
        "(() => { const b = document.querySelector('button.citation-chip'); b.scrollIntoView({ block: 'center', inline: 'center' }); const r = b.getBoundingClientRect(); return { x: r.left + r.width / 2, y: r.top + r.height / 2 }; })()",
    );
    tab.call(
        "Input.dispatchMouseEvent",
        serde_json::json!({
            "type": "mouseMoved",
            "pointerType": "mouse",
            "x": rect["x"].as_f64().unwrap_or(0.0),
            "y": rect["y"].as_f64().unwrap_or(0.0),
        }),
    );
    tab.wait_for_text("Potassium 5.4 mmol/L", Duration::from_secs(10));
    let preview_visible =
        tab.eval("document.querySelector('.citation-preview[role=\"dialog\"]')?.innerText ?? ''");
    assert!(
        preview_visible
            .as_str()
            .unwrap_or_default()
            .contains("Potassium 5.4 mmol/L"),
        "hover CitationPreview did not show quote: {preview_visible}"
    );
    tokio::time::sleep(Duration::from_millis(240)).await;
    tab.save_picture("p1b2-hover-preview-and-sources");

    // AC1: Click `Sources` popover -> click first source -> force 503 then retry -> verify `<mark>`, collapsed Technical details, desktop side-by-side layout.
    tab.click("button.sources-button");
    tab.wait_for_text("Echocardiogram", Duration::from_secs(10));
    tab.call(
        "Fetch.enable",
        serde_json::json!({ "patterns": [{ "urlPattern": "*/sources/*", "requestStage": "Request" }] }),
    );
    assert_eq!(
        tab.eval("(() => { const b = document.querySelector('ol.sources-index button'); b?.click(); return Boolean(b); })()"),
        serde_json::json!(true),
        "first row button missing in Sources popover"
    );
    let failed_source_req = tab.held_request("GET");
    tab.call(
        "Fetch.fulfillRequest",
        serde_json::json!({
            "requestId": failed_source_req["requestId"],
            "responseCode": 503,
            "responseHeaders": [{"name": "content-type", "value": "text/plain"}],
            "body": base64::engine::general_purpose::STANDARD.encode("source temporarily unavailable"),
        }),
    );
    tab.call("Fetch.disable", serde_json::json!({}));
    tab.release_all();
    tab.wait_for_text("Try again", Duration::from_secs(10));
    tab.click_named(".source-reading-pane button", "Try again");
    tab.wait_for_text(
        "Potassium 5.4 mmol/L (reference 3.5-5.0 mmol/L), collected Tuesday morning.",
        Duration::from_secs(10),
    );
    assert_eq!(
        tab.eval("document.querySelector('.source-reading-pane mark')?.textContent ?? ''"),
        serde_json::json!("Potassium 5.4 mmol/L"),
        "passage highlight mark missing inside open source pane"
    );
    let pane_collapsed_json = tab
        .eval("document.querySelector('.source-reading-pane')?.innerText ?? ''")
        .as_str()
        .unwrap_or_default()
        .to_owned();
    let raw_disclosure_state = tab.eval(
        "({ state: document.querySelector('.source-reading-pane .disclosure')?.getAttribute('data-state') ?? '', height: document.querySelector('.source-reading-pane .disclosure-region')?.getBoundingClientRect().height ?? -1, opacity: getComputedStyle(document.querySelector('.source-reading-pane .disclosure-region')).opacity })",
    );
    assert!(
        raw_disclosure_state["state"] == serde_json::json!("closed")
            && raw_disclosure_state["height"] == serde_json::json!(0)
            && raw_disclosure_state["opacity"] == serde_json::json!("0")
            && pane_collapsed_json.contains("Technical details"),
        "raw JSON must stay behind collapsed Technical details disclosure ({raw_disclosure_state}):\n{pane_collapsed_json}"
    );
    assert!(
        tab.eval("document.querySelector('.glass-stage')?.innerText.includes('Potassium is 5.4 mmol/L') ?? false")
            == serde_json::json!(true),
        "desktop side panel must keep open conversation visible beside it"
    );
    tab.save_picture("p1b2-citations-panel-desktop");

    // AC1: Stale response drop (`[1]` held in flight while `[2]` completes first -> `[2]` stays!).
    tab.click(".source-reading-pane button[aria-label=\"Close\"]");
    let close_wait = Instant::now() + Duration::from_secs(5);
    while tab.eval("Boolean(document.querySelector('.source-reading-pane'))")
        == serde_json::json!(true)
    {
        assert!(Instant::now() < close_wait, "source panel did not close");
        tokio::time::sleep(Duration::from_millis(30)).await;
    }
    tab.call(
        "Fetch.enable",
        serde_json::json!({ "patterns": [{ "urlPattern": "*/sources/*", "requestStage": "Request" }] }),
    );
    assert_eq!(
        tab.eval("(() => { const b = document.querySelectorAll('button.citation-chip')[0]; b?.click(); return Boolean(b); })()"),
        serde_json::json!(true)
    );
    let held_s1 = tab.held_request("GET");
    assert_eq!(
        tab.eval("(() => { const b = document.querySelectorAll('button.citation-chip')[1]; b?.click(); return Boolean(b); })()"),
        serde_json::json!(true)
    );
    let held_s2 = tab.held_request("GET");
    let modified_s2 = serde_json::json!({
        "source": {
            "source_key": "turn-cited-2:echo-flagged-v1:rev-2",
            "number": 2,
            "source_label": "Regional Lab",
            "title": "Echocardiogram",
            "record_type": "Echocardiogram",
            "recorded_at": "2026-08-26",
            "record_status": "preliminary",
            "quote": "Left ventricular ejection fraction 55%",
            "body": "Amended note: systolic assessment pending final sign-off.",
            "readable_parts": [{
                "label": "Findings",
                "text": "Amended note: systolic assessment pending final sign-off."
            }],
            "fields": [],
            "raw_content": raw_echo,
            "external_url": null,
            "origin": "source_record"
        }
    });
    tab.call(
        "Fetch.fulfillRequest",
        serde_json::json!({
            "requestId": held_s2["requestId"],
            "responseCode": 200,
            "responseHeaders": [{"name": "content-type", "value": "application/json"}],
            "body": base64::engine::general_purpose::STANDARD.encode(modified_s2.to_string()),
        }),
    );
    tab.wait_for_text("preliminary", Duration::from_secs(10));
    tab.wait_for_text(
        "Cited passage could not be matched",
        Duration::from_secs(10),
    );
    tab.release(&held_s1);
    tab.call("Fetch.disable", serde_json::json!({}));
    tab.release_all();
    tokio::time::sleep(Duration::from_millis(350)).await;
    let after_stale = tab
        .eval("document.querySelector('.source-reading-pane')?.innerText ?? ''")
        .as_str()
        .unwrap_or_default()
        .to_owned();
    assert!(
        after_stale.contains("Echocardiogram")
            && after_stale.contains("preliminary")
            && after_stale.contains("Cited passage could not be matched"),
        "stale delayed response overwrote newer citation panel:\n{after_stale}"
    );

    // AC1: Phone viewport (<900px) renders Back control cleanly, then closes panel.
    tab.set_viewport(390, 844, true);
    tokio::time::sleep(Duration::from_millis(250)).await;
    tab.wait_for_text("Back", Duration::from_secs(5));
    tab.save_picture("p1b2-citations-panel-phone");
    tab.click(".source-pane-back");
    let phone_close = Instant::now() + Duration::from_secs(5);
    while tab.eval("Boolean(document.querySelector('.source-reading-pane'))")
        == serde_json::json!(true)
    {
        assert!(
            Instant::now() < phone_close,
            "phone source panel Back failed"
        );
        tokio::time::sleep(Duration::from_millis(30)).await;
    }
    tab.set_viewport(1280, 800, false);

    // AC1: Cross-patient SPA navigation resets open source panel so Patient A's PHI never bleeds onto Patient B.
    assert_eq!(
        tab.eval("(() => { const b = document.querySelectorAll('button.citation-chip')[0]; b?.click(); return Boolean(b); })()"),
        serde_json::json!(true)
    );
    tab.wait_for_text(
        "Potassium 5.4 mmol/L (reference 3.5-5.0 mmol/L)",
        Duration::from_secs(10),
    );
    assert_eq!(
        tab.eval("(() => { const b = [...document.querySelectorAll('button')].find(x => x.textContent.includes('Find patient')); b?.click(); return Boolean(b); })()"),
        serde_json::json!(true),
        "Find patient button not found for cross-patient switch"
    );
    tab.wait_for_text("Jake Smith", Duration::from_secs(10));
    assert_eq!(
        tab.eval("(() => { const b = [...document.querySelectorAll('dialog button')].find(x => x.innerText.includes('Jake Smith')); b?.click(); return Boolean(b); })()"),
        serde_json::json!(true),
        "Jake Smith row missing in Find patient dialog"
    );
    tab.wait_for_text("Archive candidate renal follow-up", Duration::from_secs(15));
    assert_eq!(
        tab.eval("Boolean(document.querySelector('.source-reading-pane'))"),
        serde_json::json!(false),
        "Patient A source reading pane remained visible after SPA navigation to Patient B"
    );

    // AC1: Removing panel access while citation side panel is open transitions it to Unavailable.
    tab.navigate(&format!("{base_url}?patient={patient_a}&chat={chat_cited}"));
    let chip_deadline = Instant::now() + Duration::from_secs(20);
    while tab.eval("Boolean(document.querySelector('button.citation-chip'))")
        != serde_json::json!(true)
    {
        assert!(
            Instant::now() < chip_deadline,
            "citation chip did not render after returning to patient_a"
        );
        tokio::time::sleep(Duration::from_millis(40)).await;
    }
    assert_eq!(
        tab.eval("(() => { const b = document.querySelectorAll('button.citation-chip')[0]; b?.click(); return Boolean(b); })()"),
        serde_json::json!(true)
    );
    tab.wait_for_text(
        "Potassium 5.4 mmol/L (reference 3.5-5.0 mmol/L)",
        Duration::from_secs(10),
    );
    let staging_identity = PhysicianIdentity {
        practice_id: physician_sample::STAGING_PRACTICE_ID.to_owned(),
        physician_id: "staging-physician".to_owned(),
    };
    auth.set_membership(&staging_identity, patient_a, Membership::Denied)
        .await
        .expect("remove patient_a panel assignment");
    bus.publish(physician_bus::Event::WorkspaceChanged {
        workspace_id: patient_a.to_owned(),
    })
    .await
    .expect("publish WorkspaceChanged");
    tab.wait_for_text(
        "Panel access is required to view this patient’s records.",
        Duration::from_secs(15),
    );
    tab.save_picture("p1b2-citations-panel-revoked");

    // AC2: Archive idle conversation + Undo banner + reload persistence.
    tab.navigate(&format!(
        "{base_url}?patient={patient_b}&chat={chat_archive}"
    ));
    tab.wait_for_text(
        "Creatinine remains stable at 1.1 mg/dL.",
        Duration::from_secs(20),
    );
    let arch_deadline = Instant::now() + Duration::from_secs(15);
    while tab.eval("Boolean(document.querySelector('button[data-archive-conversation=\"true\"]'))")
        != serde_json::json!(true)
    {
        assert!(
            Instant::now() < arch_deadline,
            "Archive button never rendered"
        );
        tokio::time::sleep(Duration::from_millis(40)).await;
    }
    tab.click("button[data-archive-conversation=\"true\"]");
    tab.wait_for_text("Undo", Duration::from_secs(10));
    let sidebar_hide_deadline = Instant::now() + Duration::from_secs(10);
    let sidebar_after_archive = loop {
        let text = tab
            .eval("document.querySelector('.sidebar-patient-context')?.innerText ?? ''")
            .as_str()
            .unwrap_or_default()
            .to_owned();
        if !text.is_empty() && !text.contains("Archive candidate renal follow-up") {
            break text;
        }
        assert!(
            Instant::now() < sidebar_hide_deadline,
            "archived chat remained in patient sidebar context:\n{text}"
        );
        tokio::time::sleep(Duration::from_millis(40)).await;
    };
    assert!(
        !sidebar_after_archive.contains("Archive candidate renal follow-up"),
        "archived chat remained in patient sidebar context:\n{sidebar_after_archive}"
    );
    tab.save_picture("p1b2-archive-undo-banner");
    tab.click_named("button", "Undo");
    tab.wait_for_text("Archive candidate renal follow-up", Duration::from_secs(10));

    // Re-archive and reload -> stays hidden after reload.
    let rearch_deadline = Instant::now() + Duration::from_secs(15);
    while tab.eval("Boolean(document.querySelector('button[data-archive-conversation=\"true\"]:not(:disabled)'))")
        != serde_json::json!(true)
    {
        assert!(Instant::now() < rearch_deadline, "Archive button not ready after Undo");
        tokio::time::sleep(Duration::from_millis(40)).await;
    }
    tab.click("button[data-archive-conversation=\"true\"]");
    tab.wait_for_text("Undo", Duration::from_secs(10));
    tab.navigate(&format!("{base_url}?patient={patient_b}"));
    tab.wait_for_text("Past work", Duration::from_secs(20));
    tokio::time::sleep(Duration::from_millis(300)).await;
    let sidebar_after_reload = tab
        .eval("document.querySelector('.sidebar-patient-context')?.innerText ?? ''")
        .as_str()
        .unwrap_or_default()
        .to_owned();
    assert!(
        !sidebar_after_reload.is_empty()
            && !sidebar_after_reload.contains("Archive candidate renal follow-up"),
        "archived chat reappeared in patient sidebar context after reload:\n{sidebar_after_reload}"
    );

    // AC5: Shared Find patient component shows KeywordFallback notice on broad keyword fallback.
    assert_eq!(
        tab.eval("(() => { const b = [...document.querySelectorAll('button')].find(x => x.textContent.includes('Find patient')); b?.click(); return Boolean(b); })()"),
        serde_json::json!(true),
        "Find patient button not found"
    );
    tab.wait_for_text("Dan Cohen", Duration::from_secs(10));
    tab.call(
        "Fetch.enable",
        serde_json::json!({ "patterns": [{ "urlPattern": "*/patient-roster*", "requestStage": "Request" }] }),
    );
    tab.search_for("fallbackcheck");
    let held_search = tab.held_request("GET");
    let fallback_body = serde_json::json!({
        "patients": [{
            "patient_workspace_id": patient_b,
            "patient_display_name": "Jake Smith",
            "birth_date": "1950-03-12",
            "mrn": "00000001",
            "duet_linked": false,
            "sharing_consented": false,
            "membership": "physician_panel"
        }],
        "next_after": null,
        "match_mode": "keyword_fallback"
    });
    tab.call(
        "Fetch.fulfillRequest",
        serde_json::json!({
            "requestId": held_search["requestId"],
            "responseCode": 200,
            "responseHeaders": [{"name": "content-type", "value": "application/json"}],
            "body": base64::engine::general_purpose::STANDARD.encode(fallback_body.to_string()),
        }),
    );
    tab.call("Fetch.disable", serde_json::json!({}));
    tab.release_all();
    tab.wait_for_text(
        "No exact or prefix match — showing keyword matches.",
        Duration::from_secs(10),
    );
    tab.save_picture("p1b2-find-patient-fallback");

    drop(tab);
    server.abort();

    // AC3 (`Discuss`) & AC4 (`Dictation`: Ctrl+Shift+Space hold/release + Dictate click + 503 failure notice).
    let chat_server = serve_chat_browser_with_stub(&site, 5_000, "[]", scale::FIXED_TEXT).await;
    let chat_base = format!("http://{}/", chat_server.address);
    let mut tab2 = Tab::open("about:blank");
    tab2.browser.watch_page_errors();
    tab2.set_viewport(1280, 800, false);
    tab2.call(
        "Page.addScriptToEvaluateOnNewDocument",
        serde_json::json!({
            "source": r#"
(() => {
  const bytes = new TextEncoder().encode("TEXT:Repeat basic metabolic panel in the morning.");
  class FakeRecorder {
    constructor(stream) {
      this.stream = stream;
      this.mimeType = 'audio/webm';
      this.state = 'inactive';
      this.ondataavailable = null;
      this.listeners = {};
    }
    addEventListener(type, fn) { (this.listeners[type] ||= []).push(fn); }
    start() {
      this.state = 'recording';
    }
    stop() {
      this.state = 'inactive';
      if (this.ondataavailable) {
        this.ondataavailable({ data: new Blob([bytes], { type: 'audio/webm' }) });
      }
      (this.listeners['stop'] || []).forEach(fn => fn());
    }
  }
  window.MediaRecorder = FakeRecorder;
  window.AudioContext = class FakeAudioContext {
    createMediaStreamSource() { return { connect() {} }; }
    createAnalyser() {
      return {
        fftSize: 256,
        frequencyBinCount: 16,
        getByteTimeDomainData(buf) { for (let i = 0; i < buf.length; i++) buf[i] = 160; },
      };
    }
    close() { return Promise.resolve(); }
  };
  Object.defineProperty(navigator, 'mediaDevices', {
    configurable: true,
    value: {
      getUserMedia: async () => ({ getTracks: () => [{ stop() {} }] }),
    },
  });
})();
"#
        }),
    );
    tab2.navigate(&chat_base);
    tab2.wait_for_text("Work email", Duration::from_secs(20));
    tab2.request_sign_in("physician@codahealth.test");
    tab2.wait_for_text("Nothing needs you right now.", Duration::from_secs(20));
    tab2.navigate(&format!(
        "{chat_base}?patient={patient_b}&chat=chat-p1b2-discuss-dictate"
    ));
    tab2.wait_for_text(
        "What would you like Coda to review?",
        Duration::from_secs(20),
    );

    // AC4: Hold Ctrl+Shift+Space (CDP modifiers: Ctrl=2 | Shift=8 = 10) while focused in composer,
    // wait for active dictation wave, release -> inserts into composer without sending!
    tab2.step("focusEnd", serde_json::json!(["textarea.composer-input"]));
    tab2.call(
        "Input.dispatchKeyEvent",
        serde_json::json!({
            "type": "keyDown",
            "key": " ",
            "code": "Space",
            "windowsVirtualKeyCode": 32,
            "nativeVirtualKeyCode": 32,
            "modifiers": 10
        }),
    );
    tab2.wait_for_text("Listening", Duration::from_secs(10));
    tokio::time::sleep(Duration::from_millis(120)).await;
    tab2.call(
        "Input.dispatchKeyEvent",
        serde_json::json!({
            "type": "keyUp",
            "key": " ",
            "code": "Space",
            "windowsVirtualKeyCode": 32,
            "nativeVirtualKeyCode": 32,
            "modifiers": 10
        }),
    );
    let started_wait = Instant::now();
    while tab2.composer().0 != "Repeat basic metabolic panel in the morning." {
        if started_wait.elapsed() >= Duration::from_secs(10) {
            let html = tab2.eval("document.querySelector('.composer')?.outerHTML ?? ''");
            let errors = tab2.browser.take_page_errors();
            panic!(
                "dictation did not insert transcript; composer={:?}, errors={errors:?}, html={html}",
                tab2.composer()
            );
        }
        tokio::time::sleep(Duration::from_millis(50)).await;
    }
    tab2.save_picture("p1b2-dictation-transcribed");

    // AC4: Failure on POST /dictation renders honest unavailability notice and keeps existing composer text.
    tab2.call(
        "Fetch.enable",
        serde_json::json!({ "patterns": [{ "urlPattern": "*/dictation", "requestStage": "Request" }] }),
    );
    tab2.click_named(".composer button", "Dictate");
    tab2.wait_for_text("Listening", Duration::from_secs(10));
    tab2.click(".dictation button[aria-label=\"Finish dictation\"]");
    let held_dictation = tab2.held_request("POST");
    tab2.call(
        "Fetch.fulfillRequest",
        serde_json::json!({
            "requestId": held_dictation["requestId"],
            "responseCode": 503,
            "responseHeaders": [{"name": "content-type", "value": "text/plain"}],
            "body": base64::engine::general_purpose::STANDARD.encode("unavailable"),
        }),
    );
    tab2.call("Fetch.disable", serde_json::json!({}));
    tab2.release_all();
    tab2.wait_for_text("Dictation is unavailable", Duration::from_secs(10));
    assert_eq!(
        tab2.composer().0,
        "Repeat basic metabolic panel in the morning.",
        "failed dictation must preserve existing composer draft"
    );

    // AC4 cancel-during-transcribing race: clicking Cancel dictation while POST /dictation is in flight
    // must discard any late transcript resolution without mutating the composer.
    tab2.call(
        "Fetch.enable",
        serde_json::json!({ "patterns": [{ "urlPattern": "*/dictation", "requestStage": "Request" }] }),
    );
    tab2.click_named(".composer button", "Dictate");
    tab2.wait_for_text("Listening", Duration::from_secs(10));
    tab2.click(".dictation button[aria-label=\"Finish dictation\"]");
    let held_cancel_dictation = tab2.held_request("POST");
    tab2.wait_for_text("Transcribing", Duration::from_secs(10));
    tab2.click(".dictation button[aria-label=\"Cancel dictation\"]");
    let cancel_wait = Instant::now() + Duration::from_secs(5);
    while tab2.eval("Boolean(document.querySelector('.dictation'))") == serde_json::json!(true) {
        assert!(Instant::now() < cancel_wait, "dictation bar did not cancel");
        tokio::time::sleep(Duration::from_millis(30)).await;
    }
    // Cancelling during Transcribing aborts the in-flight Task / fetch and clears the dictation bar.
    let _ = held_cancel_dictation;
    tab2.call("Fetch.disable", serde_json::json!({}));
    tab2.release_all();
    tokio::time::sleep(Duration::from_millis(200)).await;
    assert_eq!(
        tab2.composer().0,
        "Repeat basic metabolic panel in the morning.",
        "cancelled in-flight dictation must never insert transcript into composer"
    );

    // AC3: Start active turn + queue follow-up -> verify Stopped::Refused on wrong work_item_id ->
    // click Discuss on working bar -> stops active turn & restores queued item to composer.
    tab2.type_composer("Active run before discuss");
    tab2.click_named(".composer-toolbar button", "Send");
    tab2.wait_for_text("Send next", Duration::from_secs(15));
    tab2.type_composer("Queued item restored on discuss");
    tab2.click_named(".composer-toolbar button", "Send next");
    tab2.wait_for_text("Queued item restored on discuss", Duration::from_secs(10));
    let refused_eval = tab2.call(
        "Runtime.evaluate",
        serde_json::json!({
            "awaitPromise": true,
            "returnByValue": true,
            "expression": format!(
                r#"(async () => {{
                    const token = localStorage.getItem('coda.physician.session') || '';
                    const r = await fetch('/commands', {{
                        method: 'POST',
                        headers: {{
                            'content-type': 'application/json',
                            'authorization': 'Bearer ' + token
                        }},
                        body: JSON.stringify({{
                            type: 'discussion.open',
                            idempotency_key: 'discuss-wrong-work-id-1',
                            payload: {{
                                patient_workspace_id: '{patient_b}',
                                chat_id: 'chat-p1b2-discuss-dictate',
                                work_item_id: 'wrong-work-id'
                            }}
                        }})
                    }});
                    return r.status + ':' + (await r.text());
                }})()"#
            ),
        }),
    );
    let refused_text = refused_eval["result"]["result"]["value"]
        .as_str()
        .or_else(|| refused_eval["result"]["value"].as_str())
        .unwrap_or_default();
    assert!(
        refused_text.contains("\"outcome\":\"rejected\""),
        "discussion.open must reject mismatched work_item_id while running: {refused_eval}"
    );
    tab2.click_named("button", "Discuss");
    tab2.wait_composer("Queued item restored on discuss", Duration::from_secs(15));
    tab2.wait_for_text("Coda stopped this response.", Duration::from_secs(10));
    tab2.save_picture("p1b2-discuss-stopped-restored");

    // K1.6 & K1.7: Discuss on finished response opens a new chat, repeat returns same chat,
    // and deliberate Discuss after sending in it opens a fresh chat.
    tab2.type_composer("Finished turn for discuss topic");
    tab2.click_named(".composer-toolbar button", "Send");
    tab2.wait_for_text(scale::FIXED_TEXT, Duration::from_secs(20));
    let discuss_ready = Instant::now() + Duration::from_secs(15);
    while tab2.eval("Boolean(document.querySelector('.answer-ending button[data-discuss-response=\"true\"]:not(:disabled)'))")
        != serde_json::json!(true)
    {
        assert!(
            Instant::now() < discuss_ready,
            "Finished Discuss button never rendered; stage={}",
            tab2.eval("document.querySelector('.glass-stage')?.innerHTML ?? ''")
        );
        tokio::time::sleep(Duration::from_millis(40)).await;
    }
    tab2.click(".answer-ending button[data-discuss-response=\"true\"]");
    let d1_wait = Instant::now() + Duration::from_secs(10);
    let d1_url = loop {
        let href = tab2
            .eval("window.location.href")
            .as_str()
            .unwrap_or_default()
            .to_owned();
        if href.contains("chat=") && !href.contains("chat=chat-p1b2-discuss-dictate") {
            break href;
        }
        assert!(
            Instant::now() < d1_wait,
            "Discuss did not open a new chat: {href}; notices={}; buttons={}; errors={:?}",
            tab2.eval("[...document.querySelectorAll('.notice, [role=\"status\"]')].map(e => e.textContent)"),
            tab2.eval("[...document.querySelectorAll('button[data-discuss-response]')].map(b => ({disabled: b.disabled, text: b.textContent}))"),
            tab2.browser.take_page_errors()
        );
        tokio::time::sleep(Duration::from_millis(40)).await;
    };
    tab2.wait_for_text("Active run before discuss", Duration::from_secs(15));
    assert_eq!(
        tab2.eval("(() => { const b = [...document.querySelectorAll('.sidebar-conversation-link')].find(x => x.innerText.includes('Active run before discuss')); b?.click(); return Boolean(b); })()"),
        serde_json::json!(true),
        "could not click back to source conversation in sidebar"
    );
    tab2.wait_for_text(scale::FIXED_TEXT, Duration::from_secs(15));
    let discuss_repeat_ready = Instant::now() + Duration::from_secs(10);
    while tab2.eval("Boolean(document.querySelector('.answer-ending button[data-discuss-response=\"true\"]:not(:disabled)'))")
        != serde_json::json!(true)
    {
        assert!(
            Instant::now() < discuss_repeat_ready,
            "Finished Discuss button missing after returning to chat; stage={}",
            tab2.eval("document.querySelector('.glass-stage')?.innerHTML ?? ''")
        );
        tokio::time::sleep(Duration::from_millis(40)).await;
    }
    tab2.click(".answer-ending button[data-discuss-response=\"true\"]");
    let d1_repeat_wait = Instant::now() + Duration::from_secs(10);
    loop {
        let href = tab2
            .eval("window.location.href")
            .as_str()
            .unwrap_or_default()
            .to_owned();
        if href == d1_url {
            break;
        }
        assert!(
            Instant::now() < d1_repeat_wait,
            "Repeating Discuss did not return same unused chat ({d1_url} vs {href})"
        );
        tokio::time::sleep(Duration::from_millis(40)).await;
    }
    tab2.type_composer("Discuss follow-up turn 1");
    tab2.click_named(".composer-toolbar button", "Send");
    tab2.wait_for_text("Discuss follow-up turn 1", Duration::from_secs(15));
    assert_eq!(
        tab2.eval("(() => { const b = [...document.querySelectorAll('.sidebar-conversation-link')].find(x => x.innerText.includes('Active run before discuss')); b?.click(); return Boolean(b); })()"),
        serde_json::json!(true),
        "could not click back to source conversation in sidebar after sending"
    );
    tab2.wait_for_text(scale::FIXED_TEXT, Duration::from_secs(15));
    let discuss_d2_ready = Instant::now() + Duration::from_secs(10);
    while tab2.eval("Boolean(document.querySelector('.answer-ending button[data-discuss-response=\"true\"]:not(:disabled)'))")
        != serde_json::json!(true)
    {
        assert!(
            Instant::now() < discuss_d2_ready,
            "Finished Discuss button missing before d2; stage={}",
            tab2.eval("document.querySelector('.glass-stage')?.innerHTML ?? ''")
        );
        tokio::time::sleep(Duration::from_millis(40)).await;
    }
    tab2.click(".answer-ending button[data-discuss-response=\"true\"]");
    let d2_wait = Instant::now() + Duration::from_secs(10);
    loop {
        let href = tab2
            .eval("window.location.href")
            .as_str()
            .unwrap_or_default()
            .to_owned();
        if href.contains("chat=")
            && !href.contains("chat=chat-p1b2-discuss-dictate")
            && href != d1_url
        {
            break;
        }
        assert!(
            Instant::now() < d2_wait,
            "Deliberate later Discuss did not open fresh chat (still {href}, d1={d1_url})"
        );
        tokio::time::sleep(Duration::from_millis(40)).await;
    }
}
