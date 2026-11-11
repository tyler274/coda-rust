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

    // 3. The answer is lost while the page stays open: the page shows it is
    // confirming and asks again under the same key, with no click.
    let third = "Recheck the magnesium";
    tab.type_composer(third);
    tab.wait_saved();
    let respond_held = serde_json::json!({ "patterns": [{
        "urlPattern": "*/commands",
        "requestStage": "Response"
    }] });
    tab.call("Fetch.enable", respond_held.clone());
    tab.click_named(".composer-toolbar button", "Send");
    let lost = tab.held_request("POST");
    let lost_key = held_body(&lost)["idempotency_key"].clone();
    tab.call(
        "Fetch.failRequest",
        serde_json::json!({ "requestId": lost["requestId"], "errorReason": "Failed" }),
    );
    tab.call(
        "Fetch.enable",
        serde_json::json!({ "patterns": [{ "urlPattern": "*/commands" }] }),
    );
    tab.wait_for_text("Confirming with Coda…", Duration::from_secs(5));
    let reasked = tab.held_request("POST");
    assert_eq!(
        held_body(&reasked)["idempotency_key"],
        lost_key,
        "confirming a lost answer issued a new command"
    );
    tab.save_pictures("command-confirming", ".composer-context");
    tab.release(&reasked);
    tab.call("Fetch.disable", serde_json::json!({}));
    tab.wait_for_text(third, Duration::from_secs(20));
    tab.wait_composer("", Duration::from_secs(10));
    assert_eq!(
        sent_count(&mut tab, third),
        1,
        "the confirmed message did not appear exactly once"
    );
    assert!(
        !tab.dom().contains("Check again"),
        "a confirmed command still offered Check again"
    );

    // 4. Every re-ask is lost too: only then does the page say it couldn't
    // confirm, and Check again asks once more under the same key.
    let fourth = "Add a urine sodium";
    tab.type_composer(fourth);
    tab.wait_saved();
    tab.call("Fetch.enable", respond_held);
    tab.click_named(".composer-toolbar button", "Send");
    let first_try = tab.held_request("POST");
    let lost_key = held_body(&first_try)["idempotency_key"].clone();
    let mut tries = vec![first_try];
    while tries.len() < 4 {
        let lost = tries.last().unwrap().clone();
        tab.call(
            "Fetch.failRequest",
            serde_json::json!({ "requestId": lost["requestId"], "errorReason": "Failed" }),
        );
        assert!(
            !tab.dom().contains("Check again"),
            "Check again showed while Coda was still confirming"
        );
        tries.push(tab.held_request("POST"));
    }
    let last = tries.last().unwrap().clone();
    tab.call(
        "Fetch.failRequest",
        serde_json::json!({ "requestId": last["requestId"], "errorReason": "Failed" }),
    );
    for asked in &tries {
        assert_eq!(
            held_body(asked)["idempotency_key"],
            lost_key,
            "a re-ask changed key"
        );
    }
    tab.call("Fetch.disable", serde_json::json!({}));
    tab.wait_for_text("Coda couldn’t confirm that.", Duration::from_secs(10));
    tab.save_pictures("command-unconfirmed", ".composer-context");
    assert_eq!(
        tab.composer().0,
        fourth,
        "an unconfirmed Send cleared its text"
    );
    tab.call(
        "Fetch.enable",
        serde_json::json!({ "patterns": [{ "urlPattern": "*/commands" }] }),
    );
    tab.click_named("button", "Check again");
    let checked = tab.held_request("POST");
    assert_eq!(
        held_body(&checked)["idempotency_key"],
        lost_key,
        "Check again issued a new command"
    );
    tab.release(&checked);
    tab.call("Fetch.disable", serde_json::json!({}));
    tab.wait_for_text(fourth, Duration::from_secs(20));
    tab.wait_composer("", Duration::from_secs(10));
    assert_eq!(
        sent_count(&mut tab, fourth),
        1,
        "Check again sent the message twice"
    );
}

/// A stale tab's Stop whose answer is lost, on work another tab already
/// stopped, is confirmed on its own after that other tab resumes. The
/// confirmation finds the same accepted no-op, so the resumed run keeps
/// running and is answered.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore = "browser and disposable Postgres qualification; run explicitly with --ignored"]
async fn a_lost_stop_confirmed_after_another_tab_resumes_leaves_the_run_going() {
    let _guard = test_lock().lock().await;
    let Some(database) = scale::Database::create("stop_retry_browser").await else {
        return;
    };
    let site = build_site();
    scale::staging_account(&database.url).await;
    let home = std::env::temp_dir().join(format!("physician-stop-retry-{}", uuid::Uuid::new_v4()));
    fs::create_dir_all(&home).unwrap();
    let valkey = scale::Valkey::start().await;
    let redis = valkey.url();
    let worker_port = scale::free_port();
    let site_port = scale::free_port();
    let emulator = scale::Emulator::start().await;
    emulator.provision(worker_port).await;
    let topic = scale::topic();
    let worker_listen = format!("127.0.0.1:{worker_port}");
    let host = format!("127.0.0.1:{site_port}");
    // Long enough for both Stops, the lost answer and Resume while it runs.
    let stub = scale::ResponsesStub::start(&home, 0, 12_000, "[]").await;
    let mut worker = scale::Process::spawn(
        "stop-retry-worker",
        scale::worker_bin(),
        &[],
        &[
            ("PHYSICIAN_DATABASE_URL", database.url.as_str()),
            ("PHYSICIAN_REDIS_URL", redis.as_str()),
            ("PUBSUB_EMULATOR_HOST", emulator.host.as_str()),
            ("PHYSICIAN_WORKER_LISTEN_ADDR", worker_listen.as_str()),
            ("PHYSICIAN_RESPONSES_STUB_PORT", stub.port.as_str()),
        ],
        &home,
    );
    scale::wait_ready(&mut worker, worker_port).await;
    let mut process = scale::Process::spawn(
        "stop-retry-site",
        scale::site_bin(),
        &[],
        &[
            ("PHYSICIAN_DATABASE_URL", database.url.as_str()),
            ("PHYSICIAN_REDIS_URL", redis.as_str()),
            ("PUBSUB_EMULATOR_HOST", emulator.host.as_str()),
            ("GOOGLE_CLOUD_PROJECT", scale::PROJECT),
            ("PHYSICIAN_CHAT_TOPIC", topic.as_str()),
            ("PHYSICIAN_AUTH", "required"),
            ("PHYSICIAN_AUTH_PROOF_TOKENS", "1"),
            ("PHYSICIAN_SITE_DIR", site.to_str().unwrap()),
            ("PHYSICIAN_LISTEN_ADDR", host.as_str()),
        ],
        &home,
    );
    scale::wait_ready(&mut process, site_port).await;
    let patient = physician_sample::WORKSPACE_IDS[0];
    let chat = format!("chat-stop-retry-{}", uuid::Uuid::new_v4().simple());
    let url = format!("http://{host}/?patient={patient}&chat={chat}");
    let mut a = Tab::open(&url);
    a.wait_for_text("Work email", Duration::from_secs(20));
    a.request_sign_in("physician@codahealth.test");
    a.wait_composer("", Duration::from_secs(20));
    let mut b = Tab::open(&url);
    b.wait_for_text("Work email", Duration::from_secs(20));
    b.request_sign_in("physician@codahealth.test");
    b.wait_composer("", Duration::from_secs(20));

    let writing = "Coda is writing this response.";
    const STOPPED: &str = "Coda stopped this response. Your message is saved.";
    a.type_composer("Hello");
    a.step("sendWithKeyboard", serde_json::json!([]));
    a.wait_for_text(writing, Duration::from_secs(20));
    // Tab B is the page that has not caught up: it opens the running chat.
    b.reload();
    b.wait_for_text(writing, Duration::from_secs(20));

    // Tab B presses Stop; its command is held before it reaches Coda.
    b.call(
        "Fetch.enable",
        serde_json::json!({ "patterns": [
            { "urlPattern": "*/commands", "requestStage": "Request" },
            { "urlPattern": "*/commands", "requestStage": "Response" },
        ] }),
    );
    b.click_named(".conversation-mechanical button", "Stop");
    let stale = b.held_request("POST");
    let key = held_body(&stale)["idempotency_key"].clone();
    assert_eq!(held_body(&stale)["type"], "work.stop");
    // Tab A stops the same work first.
    a.click_named(".conversation-mechanical button", "Stop");
    a.wait_for_text(STOPPED, Duration::from_secs(20));
    // B's Stop reaches Coda, which finds the work already held, and its
    // answer is lost on the way back.
    b.release(&stale);
    let answered = b.held_request("POST");
    assert!(answered["responseStatusCode"].is_number(), "{answered}");
    b.call(
        "Fetch.failRequest",
        serde_json::json!({ "requestId": answered["requestId"], "errorReason": "Failed" }),
    );
    // B confirms on its own under the same key; hold that until A resumes.
    let retry = b.held_request("POST");
    assert_eq!(held_body(&retry)["idempotency_key"], key);
    a.click_named(".conversation-mechanical button", "Resume");
    a.wait_for_text(writing, Duration::from_secs(20));
    b.release(&retry);
    let confirmed = b.held_request("POST");
    b.release(&confirmed);
    b.call("Fetch.disable", serde_json::json!({}));
    b.release_all();

    // The confirmation found the earlier no-op: the resumed run is answered
    // and is never stopped again.
    let started = Instant::now();
    while sent_count(&mut a, scale::FIXED_TEXT) == 0 {
        let page = a.dom();
        assert!(
            !page.contains(STOPPED),
            "the confirmed Stop held the resumed run\n{}\n{}",
            process.tail(),
            worker.tail()
        );
        assert!(
            started.elapsed() < Duration::from_secs(60),
            "the resumed run was never answered\n{}\n{}",
            process.tail(),
            worker.tail()
        );
        std::thread::sleep(Duration::from_millis(200));
    }
    b.wait_for_text(scale::FIXED_TEXT, Duration::from_secs(20));
    assert!(
        !b.dom().contains("Check again"),
        "B's confirmed Stop still offered Check again"
    );
    a.save_pictures("stop-retry-answered", "[aria-label=\"Conversation\"]");
    drop(a);
    drop(b);
    drop(process);
    drop(worker);
    fs::remove_dir_all(home).unwrap();
}

/// How many conversation lines say `text`.
fn sent_count(tab: &mut Tab, text: &str) -> usize {
    tab.step("conversationLines", serde_json::json!([]))
        .as_array()
        .into_iter()
        .flatten()
        .filter(|line| line.as_str().is_some_and(|line| line.contains(text)))
        .count()
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
        snapshot["snapshot"]["conversation"]
            .as_array()
            .unwrap()
            .iter()
            .any(|line| line["text"] == message),
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
            && demographic_dom.contains("54 years")
            && demographic_dom.contains("Female")
            && demographic_dom.contains("00000005")
            && demographic_dom.contains("Assign to panel")
            && !demographic_dom.contains("Records as of")
            && !demographic_dom.contains("This workspace has no chats."),
        "same-clinic non-panel patient should render clean demographic card:\n{demographic_dom}"
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
    _emulator: scale::Emulator,
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
    let emulator = scale::Emulator::start().await;
    let worker_port = scale::free_port();
    emulator.provision(worker_port).await;
    let valkey = scale::Valkey::start().await;
    let redis = valkey.url();
    let stub = scale::ResponsesStub::start_with_reply(&state, 0, 0, "[]", reply).await;
    let listen = format!("127.0.0.1:{worker_port}");
    let mut worker = scale::Process::spawn(
        "worker",
        scale::worker_bin(),
        &[],
        &[
            ("PHYSICIAN_DATABASE_URL", &database.url),
            ("PUBSUB_EMULATOR_HOST", &emulator.host),
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
    let queue =
        physician_sample::queue::PubSubQueue::connect(&scale::topic(), Some(&emulator.host), false)
            .await
            .unwrap();
    let chat = Arc::new(physician_sample::durable::DurableChat::new(
        store,
        Arc::new(queue),
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
        _emulator: emulator,
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
    let server = serve_chat_browser(&site, scale::FIXED_TEXT).await;
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
            "Coda couldn’t respond right now. Your message is saved.",
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
                .settle_chat_attempt(&claim, AttemptFailure::Limit)
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
    browser.wait_for_text("Recheck on Friday", Duration::from_secs(30));
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
    browser.wait_for_text("Recheck on Friday", Duration::from_secs(20));
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
    let topic = scale::topic();
    let publisher = format!("127.0.0.1:{}", scale::free_port());
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
                ("PUBSUB_EMULATOR_HOST", &publisher),
                ("GOOGLE_CLOUD_PROJECT", scale::PROJECT),
                ("PHYSICIAN_CHAT_TOPIC", &topic),
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
    snapshot["snapshot"]["conversation"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|turn| turn["kind"] == "model")
        .map(|turn| (turn["response_id"].clone(), turn["feedback"].clone()))
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
    let topic = scale::topic();
    let publisher = format!("127.0.0.1:{}", scale::free_port());
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
                ("PUBSUB_EMULATOR_HOST", &publisher),
                ("GOOGLE_CLOUD_PROJECT", scale::PROJECT),
                ("PHYSICIAN_CHAT_TOPIC", &topic),
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
    while tab.dom().contains("Sodium was 139 on Monday.") {
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

/// Send next on the hosted site, against a slow fixed worker. Three messages
/// queued behind a running answer show in a second browser, take a removal
/// and an edit there, and the two left get one reply after the answer ends. Two more queued
/// behind the next answer return to the composer on Stop and are never sent.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore = "browser and disposable Postgres qualification; run explicitly with --ignored"]
async fn send_next_queues_on_the_server_and_stop_returns_it_to_the_composer() {
    let _guard = test_lock().lock().await;
    let Some(database) = scale::Database::create("send_next_browser").await else {
        return;
    };
    let site = build_site();
    scale::staging_account(&database.url).await;
    let home = std::env::temp_dir().join(format!("physician-send-next-{}", uuid::Uuid::new_v4()));
    fs::create_dir_all(&home).unwrap();
    let valkey = scale::Valkey::start().await;
    let redis = valkey.url();
    let worker_port = scale::free_port();
    let site_port = scale::free_port();
    let emulator = scale::Emulator::start().await;
    emulator.provision(worker_port).await;
    let topic = scale::topic();
    let worker_listen = format!("127.0.0.1:{worker_port}");
    let host = format!("127.0.0.1:{site_port}");
    // Long enough to queue, edit and remove behind the answer.
    let stub = scale::ResponsesStub::start(&home, 0, 8000, "[]").await;
    let mut worker = scale::Process::spawn(
        "send-next-worker",
        scale::worker_bin(),
        &[],
        &[
            ("PHYSICIAN_DATABASE_URL", database.url.as_str()),
            ("PHYSICIAN_REDIS_URL", redis.as_str()),
            ("PUBSUB_EMULATOR_HOST", emulator.host.as_str()),
            ("PHYSICIAN_WORKER_LISTEN_ADDR", worker_listen.as_str()),
            ("PHYSICIAN_RESPONSES_STUB_PORT", stub.port.as_str()),
        ],
        &home,
    );
    scale::wait_ready(&mut worker, worker_port).await;
    let mut process = scale::Process::spawn(
        "send-next-site",
        scale::site_bin(),
        &[],
        &[
            ("PHYSICIAN_DATABASE_URL", database.url.as_str()),
            ("PHYSICIAN_REDIS_URL", redis.as_str()),
            ("PUBSUB_EMULATOR_HOST", emulator.host.as_str()),
            ("GOOGLE_CLOUD_PROJECT", scale::PROJECT),
            ("PHYSICIAN_CHAT_TOPIC", topic.as_str()),
            ("PHYSICIAN_AUTH", "required"),
            ("PHYSICIAN_AUTH_PROOF_TOKENS", "1"),
            ("PHYSICIAN_SITE_DIR", site.to_str().unwrap()),
            ("PHYSICIAN_LISTEN_ADDR", host.as_str()),
        ],
        &home,
    );
    scale::wait_ready(&mut process, site_port).await;
    let patient = physician_sample::WORKSPACE_IDS[0];
    let chat = format!("chat-send-next-{}", uuid::Uuid::new_v4().simple());
    let url = format!("http://{host}/?patient={patient}&chat={chat}");
    let mut a = Tab::open(&url);
    a.wait_for_text("Work email", Duration::from_secs(20));
    a.request_sign_in("physician@codahealth.test");
    a.wait_composer("", Duration::from_secs(20));
    let mut b = Tab::open(&url);
    b.wait_for_text("Work email", Duration::from_secs(20));
    b.request_sign_in("physician@codahealth.test");
    b.wait_composer("", Duration::from_secs(20));

    a.type_composer("Hello");
    a.step("sendWithKeyboard", serde_json::json!([]));
    wait_send_next(&mut a);
    for text in ["And the potassium?", "Also the creatinine", "Never mind"] {
        a.type_composer(text);
        a.step("sendWithKeyboard", serde_json::json!([]));
        a.wait_composer("", Duration::from_secs(10));
    }
    let shown = wait_queued(&mut a, 3);
    let waiting = |text: &str| serde_json::json!({"text": text, "status": "Waiting for next response", "actions": ["Edit", "Remove"]});
    assert_eq!(
        shown,
        serde_json::json!([
            waiting("And the potassium?"),
            waiting("Also the creatinine"),
            waiting("Never mind"),
        ])
    );
    a.save_pictures("send-next-queued", "[aria-label=\"Conversation\"]");
    // The other browser reads the same queue from Coda, not from this tab.
    assert_eq!(wait_queued(&mut b, 3), shown);
    b.step("clickQueued", serde_json::json!(["Never mind", "Remove"]));
    wait_queued(&mut a, 2);
    b.step(
        "clickQueued",
        serde_json::json!(["And the potassium?", "Edit"]),
    );
    b.browser
        .type_into(".queued-edit textarea", "And the sodium?");
    std::thread::sleep(Duration::from_millis(100));
    b.click_named(".queued-edit button", "Save");
    let edited = wait_queued_text(&mut a, "And the sodium?");
    assert_eq!(
        edited,
        serde_json::json!([waiting("And the sodium?"), waiting("Also the creatinine")])
    );

    // The answer ends; the two queued messages are answered in one reply.
    let started = Instant::now();
    loop {
        let lines = a.step("conversationLines", serde_json::json!([]));
        let texts: Vec<&str> = lines
            .as_array()
            .unwrap()
            .iter()
            .filter_map(|line| line.as_str())
            .collect();
        let replies = texts
            .iter()
            .filter(|text| text.contains(scale::FIXED_TEXT))
            .count();
        if replies == 2 && queued_now(&mut a).as_array().is_some_and(Vec::is_empty) {
            // Both queued messages sit between the first reply and the
            // second, so one reply answers them together.
            let at = |needle: &str| texts.iter().position(|text| text.contains(needle));
            let first_reply = at(scale::FIXED_TEXT).unwrap();
            let last_reply = texts
                .iter()
                .rposition(|text| text.contains(scale::FIXED_TEXT))
                .unwrap();
            for queued in ["And the sodium?", "Also the creatinine"] {
                let index = at(queued).unwrap_or_else(|| panic!("{queued} not sent: {lines}"));
                assert!(first_reply < index && index < last_reply, "{lines}");
            }
            assert!(
                !texts.iter().any(|text| text.contains("Never mind")),
                "a removed message was sent: {lines}"
            );
            break;
        }
        assert!(
            started.elapsed() < Duration::from_secs(60),
            "the queue was never answered: {lines}\n{}\n{}",
            process.tail(),
            worker.tail()
        );
        std::thread::sleep(Duration::from_millis(200));
    }

    // Two more behind the next answer; Stop returns both to the composer.
    a.type_composer("One more");
    a.step("sendWithKeyboard", serde_json::json!([]));
    wait_send_next(&mut a);
    for text in ["Draft one", "Draft two"] {
        a.type_composer(text);
        a.step("sendWithKeyboard", serde_json::json!([]));
        a.wait_composer("", Duration::from_secs(10));
    }
    wait_queued(&mut a, 2);
    a.click_named(".conversation-mechanical button", "Stop");
    a.wait_composer("Draft one\n\nDraft two", Duration::from_secs(20));
    b.wait_composer("Draft one\n\nDraft two", Duration::from_secs(20));
    a.save_pictures("send-next-stopped", ".composer-dock");
    assert!(queued_now(&mut a).as_array().is_some_and(Vec::is_empty));
    let lines = a.step("conversationLines", serde_json::json!([]));
    assert!(
        !lines.to_string().contains("Draft one"),
        "Stop sent a queued message: {lines}"
    );
    drop(a);
    drop(b);
    drop(process);
    drop(worker);
    fs::remove_dir_all(home).unwrap();
}

/// Archive with Undo on the hosted site, and what brings an archived
/// conversation back. Archive waits while Coda is writing; once the answer
/// ends it moves the chat under Archived and offers Undo, which moves it back.
/// Sending, Try again on a failed turn and Resume after Stop each return an
/// archived conversation to the active list.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore = "browser and disposable Postgres qualification; run explicitly with --ignored"]
async fn archive_offers_undo_and_sending_try_again_and_resume_bring_it_back() {
    let _guard = test_lock().lock().await;
    let Some(database) = scale::Database::create("archive_browser").await else {
        return;
    };
    let site = build_site();
    scale::staging_account(&database.url).await;
    let home = std::env::temp_dir().join(format!("physician-archive-{}", uuid::Uuid::new_v4()));
    fs::create_dir_all(&home).unwrap();
    let valkey = scale::Valkey::start().await;
    let redis = valkey.url();
    let worker_port = scale::free_port();
    let site_port = scale::free_port();
    let emulator = scale::Emulator::start().await;
    emulator.provision(worker_port).await;
    let topic = scale::topic();
    let worker_listen = format!("127.0.0.1:{worker_port}");
    let host = format!("127.0.0.1:{site_port}");
    // Long enough to see Archive wait, and to press Stop.
    let stub = scale::ResponsesStub::start(&home, 0, 3000, "[]").await;
    let mut worker = scale::Process::spawn(
        "archive-worker",
        scale::worker_bin(),
        &[],
        &[
            ("PHYSICIAN_DATABASE_URL", database.url.as_str()),
            ("PHYSICIAN_REDIS_URL", redis.as_str()),
            ("PUBSUB_EMULATOR_HOST", emulator.host.as_str()),
            ("PHYSICIAN_WORKER_LISTEN_ADDR", worker_listen.as_str()),
            ("PHYSICIAN_RESPONSES_STUB_PORT", stub.port.as_str()),
            // The staging practice rehearses, so its `[fail:K]` marker fails
            // the turn on purpose.
            ("CODA_HOSTED_PROFILE", "staging-preview"),
            (
                "PHYSICIAN_SIMULATION_PRACTICES",
                physician_sample::STAGING_PRACTICE_ID,
            ),
        ],
        &home,
    );
    scale::wait_ready(&mut worker, worker_port).await;
    let mut process = scale::Process::spawn(
        "archive-site",
        scale::site_bin(),
        &[],
        &[
            ("PHYSICIAN_DATABASE_URL", database.url.as_str()),
            ("PHYSICIAN_REDIS_URL", redis.as_str()),
            ("PUBSUB_EMULATOR_HOST", emulator.host.as_str()),
            ("GOOGLE_CLOUD_PROJECT", scale::PROJECT),
            ("PHYSICIAN_CHAT_TOPIC", topic.as_str()),
            ("PHYSICIAN_AUTH", "required"),
            ("PHYSICIAN_AUTH_PROOF_TOKENS", "1"),
            ("PHYSICIAN_SITE_DIR", site.to_str().unwrap()),
            ("PHYSICIAN_LISTEN_ADDR", host.as_str()),
        ],
        &home,
    );
    scale::wait_ready(&mut process, site_port).await;
    let patient = physician_sample::WORKSPACE_IDS[0];
    let chat = format!("chat-archive-{}", uuid::Uuid::new_v4().simple());
    let url = format!("http://{host}/?patient={patient}&chat={chat}");
    let mut tab = Tab::open(&url);
    tab.wait_for_text("Work email", Duration::from_secs(20));
    tab.request_sign_in("physician@codahealth.test");
    tab.wait_composer("", Duration::from_secs(20));
    let title = "Review the potassium";
    let listed = |state: &serde_json::Value, group: &str| {
        state[group]
            .as_array()
            .is_some_and(|titles| titles.iter().any(|t| t == title))
    };

    // While Coda writes, Archive waits.
    tab.type_composer(title);
    tab.step("sendWithKeyboard", serde_json::json!([]));
    wait_send_next(&mut tab);
    let busy = wait_archive(&mut tab, "Archive never waited for the answer", |state| {
        state["archive"] == "disabled"
    });
    assert!(!listed(&busy, "archived"), "{busy}");
    tab.save_pictures("archive-busy", "[aria-label=\"Conversation\"]");
    wait_reply_count(&mut tab, 1);

    // Archive, then Undo from its toast.
    archive_now(&mut tab);
    let archived = wait_archive(&mut tab, "Archive never moved the chat", |state| {
        listed(state, "archived") && state["toast"].as_str().is_some()
    });
    assert_eq!(
        archived["toast"], "Conversation archived.Undo",
        "{archived}"
    );
    assert!(archived["archivedLine"] == true, "{archived}");
    tab.save_pictures("archive-toast", ".sidebar-content");
    tab.click_named(".toast button", "Undo");
    let undone = wait_archive(&mut tab, "Undo never moved the chat back", |state| {
        listed(state, "active") && !listed(state, "archived")
    });
    assert_eq!(undone["archive"], "enabled", "{undone}");
    tab.save_pictures("archive-undone", ".sidebar-content");

    // An Archive whose answer is lost is asked again by the reload under its
    // own key, and still offers Undo once it is taken.
    tab.call(
        "Fetch.enable",
        serde_json::json!({ "patterns": [{
            "urlPattern": "*/commands",
            "requestStage": "Response"
        }] }),
    );
    archive_now(&mut tab);
    let answered = tab.held_request("POST");
    let lost_key = held_body(&answered)["idempotency_key"].clone();
    assert_eq!(held_body(&answered)["type"], "conversation.archive");
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
        "the reload issued a new command for an Archive whose answer was lost"
    );
    tab.release(&reasked);
    tab.call("Fetch.disable", serde_json::json!({}));
    tab.release_all();
    let recovered = wait_archive(
        &mut tab,
        "a recovered Archive never offered Undo",
        |state| listed(state, "archived") && state["toast"].as_str().is_some(),
    );
    assert_eq!(
        recovered["toast"], "Conversation archived.Undo",
        "{recovered}"
    );
    tab.save_pictures("archive-recovered-toast", ".sidebar-content");
    tab.click_named(".toast button", "Undo");
    wait_archive(
        &mut tab,
        "the recovered Archive's Undo never moved the chat back",
        |state| listed(state, "active") && !listed(state, "archived"),
    );

    // Sending brings it back.
    archive_now(&mut tab);
    wait_archive(
        &mut tab,
        "the second Archive never moved the chat",
        |state| listed(state, "archived"),
    );
    tab.type_composer("And the sodium?");
    tab.step("sendWithKeyboard", serde_json::json!([]));
    wait_archive(&mut tab, "sending never brought the chat back", |state| {
        listed(state, "active") && !listed(state, "archived")
    });
    tab.save_pictures("archive-send-brings-back", ".sidebar-content");
    wait_reply_count(&mut tab, 2);

    // Try again on a failed turn brings it back. The marker fails every
    // delivery the subscription allows, so the turn ends as failed.
    tab.type_composer("Check the creatinine [fail:5]");
    tab.step("sendWithKeyboard", serde_json::json!([]));
    wait_mechanical(
        &mut tab,
        "Try again",
        Duration::from_secs(150),
        &process,
        &worker,
    );
    archive_now(&mut tab);
    wait_archive(&mut tab, "a failed turn never archived", |state| {
        listed(state, "archived")
    });
    tab.save_pictures("archive-failed-archived", ".sidebar-content");
    tab.click_named(".conversation-mechanical button", "Try again");
    wait_archive(&mut tab, "Try again never brought the chat back", |state| {
        listed(state, "active") && !listed(state, "archived")
    });
    tab.save_pictures("archive-try-again-brings-back", ".sidebar-content");

    // Resume after Stop brings it back.
    wait_mechanical(&mut tab, "Stop", Duration::from_secs(30), &process, &worker);
    tab.click_named(".conversation-mechanical button", "Stop");
    wait_mechanical(
        &mut tab,
        "Resume",
        Duration::from_secs(30),
        &process,
        &worker,
    );
    archive_now(&mut tab);
    wait_archive(&mut tab, "a stopped turn never archived", |state| {
        listed(state, "archived")
    });
    tab.save_pictures("archive-stopped-archived", ".sidebar-content");
    tab.click_named(".conversation-mechanical button", "Resume");
    wait_archive(&mut tab, "Resume never brought the chat back", |state| {
        listed(state, "active") && !listed(state, "archived")
    });
    tab.save_pictures("archive-resume-brings-back", ".sidebar-content");
    wait_reply_count(&mut tab, 3);

    drop(tab);
    drop(process);
    drop(worker);
    fs::remove_dir_all(home).unwrap();
}

/// Discuss on the hosted site. Each of Coda's responses offers Discuss and a
/// reply never does. Pressing it while Coda is writing opens a new chat on
/// that response and holds the running answer until Resume. A Discuss whose
/// answer is lost is asked again by the reload under its own key, and still
/// opens its chat.
#[tokio::test(flavor = "multi_thread", worker_threads = 2)]
#[ignore = "browser and disposable Postgres qualification; run explicitly with --ignored"]
async fn discuss_opens_a_new_chat_holds_running_work_and_a_reply_has_none() {
    let _guard = test_lock().lock().await;
    let Some(database) = scale::Database::create("discuss_browser").await else {
        return;
    };
    let site = build_site();
    scale::staging_account(&database.url).await;
    let home = std::env::temp_dir().join(format!("physician-discuss-{}", uuid::Uuid::new_v4()));
    fs::create_dir_all(&home).unwrap();
    let valkey = scale::Valkey::start().await;
    let redis = valkey.url();
    let worker_port = scale::free_port();
    let site_port = scale::free_port();
    let emulator = scale::Emulator::start().await;
    emulator.provision(worker_port).await;
    let topic = scale::topic();
    let worker_listen = format!("127.0.0.1:{worker_port}");
    let host = format!("127.0.0.1:{site_port}");
    // Long enough to press Discuss while Coda writes.
    let stub = scale::ResponsesStub::start(&home, 0, 3000, "[]").await;
    let mut worker = scale::Process::spawn(
        "discuss-worker",
        scale::worker_bin(),
        &[],
        &[
            ("PHYSICIAN_DATABASE_URL", database.url.as_str()),
            ("PHYSICIAN_REDIS_URL", redis.as_str()),
            ("PUBSUB_EMULATOR_HOST", emulator.host.as_str()),
            ("PHYSICIAN_WORKER_LISTEN_ADDR", worker_listen.as_str()),
            ("PHYSICIAN_RESPONSES_STUB_PORT", stub.port.as_str()),
        ],
        &home,
    );
    scale::wait_ready(&mut worker, worker_port).await;
    let patient = physician_sample::WORKSPACE_IDS[0];
    let mut process = scale::Process::spawn(
        "discuss-site",
        scale::site_bin(),
        &[],
        &[
            ("PHYSICIAN_DATABASE_URL", database.url.as_str()),
            ("PHYSICIAN_REDIS_URL", redis.as_str()),
            ("PUBSUB_EMULATOR_HOST", emulator.host.as_str()),
            ("GOOGLE_CLOUD_PROJECT", scale::PROJECT),
            ("PHYSICIAN_CHAT_TOPIC", topic.as_str()),
            ("PHYSICIAN_AUTH", "required"),
            ("PHYSICIAN_AUTH_PROOF_TOKENS", "1"),
            ("PHYSICIAN_SYNTHETIC_REPLY", patient),
            ("PHYSICIAN_SITE_DIR", site.to_str().unwrap()),
            ("PHYSICIAN_LISTEN_ADDR", host.as_str()),
        ],
        &home,
    );
    scale::wait_ready(&mut process, site_port).await;
    let source = format!("chat-discuss-{}", uuid::Uuid::new_v4().simple());
    let url = format!("http://{host}/?patient={patient}&chat={source}");
    let mut tab = Tab::open(&url);
    tab.wait_for_text("Work email", Duration::from_secs(20));
    tab.request_sign_in("physician@codahealth.test");
    tab.wait_composer("", Duration::from_secs(20));
    let title = "Review the potassium";

    // Coda's answer offers Discuss; the reply beside it does not.
    tab.type_composer(title);
    tab.step("sendWithKeyboard", serde_json::json!([]));
    wait_reply_count(&mut tab, 1);
    let offered = wait_discuss(&mut tab, "the answer never offered Discuss", |state| {
        state["replies"] == 1 && state["responses"][0]["discuss"] == true
    });
    assert_eq!(
        offered["replyDiscuss"], 0,
        "a reply offers Discuss: {offered}"
    );
    tab.save_pictures("discuss-offered", "[aria-label=\"Conversation\"]");
    tab.save_pictures("discuss-reply-has-none", "article.reply-review");

    // Discuss while Coda writes the next answer opens a new chat on the first.
    tab.type_composer("And the sodium?");
    tab.step("sendWithKeyboard", serde_json::json!([]));
    wait_send_next(&mut tab);
    tab.step("clickDiscuss", serde_json::json!([0]));
    let opened = wait_discuss(&mut tab, "Discuss never opened its chat", |state| {
        state["chat"] != source.as_str()
            && state["responses"][0]["text"] == scale::FIXED_TEXT
            && state["titles"]
                .as_array()
                .is_some_and(|titles| titles.iter().any(|t| t == scale::FIXED_TEXT))
    });
    let discussion = opened["chat"].as_str().unwrap().to_owned();
    assert_eq!(opened["responses"].as_array().unwrap().len(), 1, "{opened}");
    assert_eq!(tab.step("sendWords", serde_json::json!([])), "Send");
    tab.save_pictures("discuss-opened", "[aria-label=\"Conversation\"]");
    tab.save_pictures("discuss-opened-listed", ".sidebar-content");

    // The source chat's running answer was held, and Resume finishes it.
    tab.click_named(".sidebar-conversation-link", title);
    wait_mechanical(
        &mut tab,
        "Resume",
        Duration::from_secs(30),
        &process,
        &worker,
    );
    tab.save_pictures("discuss-held", "[aria-label=\"Conversation\"]");
    tab.click_named(".conversation-mechanical button", "Resume");
    wait_reply_count(&mut tab, 2);

    // A Discuss whose answer is lost is asked again by the reload under its
    // own key, and still opens the chat it named.
    tab.call(
        "Fetch.enable",
        serde_json::json!({ "patterns": [{
            "urlPattern": "*/commands",
            "requestStage": "Response"
        }] }),
    );
    tab.step("clickDiscuss", serde_json::json!([1]));
    let answered = tab.held_request("POST");
    let lost = held_body(&answered);
    assert_eq!(lost["type"], "discussion.open");
    let named = lost["payload"]["discussion_chat_id"]
        .as_str()
        .unwrap()
        .to_owned();
    assert_ne!(named, discussion, "a later Discuss reused the earlier chat");
    tab.call(
        "Fetch.failRequest",
        serde_json::json!({ "requestId": answered["requestId"], "errorReason": "Failed" }),
    );
    // The reload's workspace read is held until the recovered Discuss has
    // landed, so the chat it opens must wait for the workspace.
    let workspace_read = format!("/patient-workspaces/{patient}");
    tab.call(
        "Fetch.enable",
        serde_json::json!({ "patterns": [
            { "urlPattern": "*/commands" },
            { "urlPattern": format!("*{workspace_read}*") },
        ] }),
    );
    tab.reload();
    let reasked = tab.held_request_keeping("POST", Some(&workspace_read));
    assert_eq!(
        held_body(&reasked)["idempotency_key"],
        lost["idempotency_key"],
        "the reload issued a new command for a Discuss whose answer was lost"
    );
    tab.release(&reasked);
    std::thread::sleep(Duration::from_millis(1500));
    assert!(
        tab.holding(&workspace_read),
        "the workspace read was not held while the recovered Discuss landed"
    );
    tab.release_remaining();
    tab.call("Fetch.disable", serde_json::json!({}));
    let recovered = wait_discuss(
        &mut tab,
        "a recovered Discuss never opened its chat",
        |state| {
            state["chat"] == named.as_str() && state["responses"][0]["text"] == scale::FIXED_TEXT
        },
    );
    assert_eq!(
        recovered["responses"].as_array().unwrap().len(),
        1,
        "{recovered}"
    );
    tab.save_pictures("discuss-recovered", "[aria-label=\"Conversation\"]");

    drop(tab);
    drop(process);
    drop(worker);
    fs::remove_dir_all(home).unwrap();
}

/// Waits until the page's Discuss state satisfies `done`, and returns it.
fn wait_discuss(
    tab: &mut Tab,
    why: &str,
    done: impl Fn(&serde_json::Value) -> bool,
) -> serde_json::Value {
    let started = Instant::now();
    loop {
        let state = tab.step("discussState", serde_json::json!([]));
        if done(&state) {
            return state;
        }
        assert!(
            started.elapsed() < Duration::from_secs(20),
            "{why}: {state}"
        );
        std::thread::sleep(Duration::from_millis(100));
    }
}

/// Presses Archive once it can be pressed.
fn archive_now(tab: &mut Tab) {
    wait_archive(tab, "Archive never became available", |state| {
        state["archive"] == "enabled"
    });
    tab.click("button.conversation-archive");
}

/// Waits until the page's archive state satisfies `done`, and returns it.
fn wait_archive(
    tab: &mut Tab,
    why: &str,
    done: impl Fn(&serde_json::Value) -> bool,
) -> serde_json::Value {
    let started = Instant::now();
    loop {
        let state = tab.step("archiveState", serde_json::json!([]));
        if done(&state) {
            return state;
        }
        assert!(
            started.elapsed() < Duration::from_secs(20),
            "{why}: {state}"
        );
        std::thread::sleep(Duration::from_millis(100));
    }
}

/// Waits until the conversation shows `count` of Coda's fixed replies and nothing running.
fn wait_reply_count(tab: &mut Tab, count: usize) {
    let started = Instant::now();
    loop {
        let lines = tab.step("conversationLines", serde_json::json!([]));
        let replies = lines
            .as_array()
            .unwrap()
            .iter()
            .filter(|line| line.as_str().is_some_and(|t| t.contains(scale::FIXED_TEXT)))
            .count();
        if replies == count && tab.step("sendWords", serde_json::json!([])) == "Send" {
            return;
        }
        assert!(
            started.elapsed() < Duration::from_secs(60),
            "never showed {count} replies: {lines}"
        );
        std::thread::sleep(Duration::from_millis(200));
    }
}

/// Waits until the conversation offers `words` on its mechanical line.
fn wait_mechanical(
    tab: &mut Tab,
    words: &str,
    timeout: Duration,
    site: &scale::Process,
    worker: &scale::Process,
) {
    let started = Instant::now();
    loop {
        let shown = tab.step("mechanicalActions", serde_json::json!([]));
        if shown
            .as_array()
            .is_some_and(|all| all.iter().any(|w| w == words))
        {
            return;
        }
        assert!(
            started.elapsed() < timeout,
            "never offered {words}: {shown}\n{}\n{}",
            site.tail(),
            worker.tail()
        );
        std::thread::sleep(Duration::from_millis(200));
    }
}

/// Waits until Coda is writing in `tab`, so its Send reads Send next.
fn wait_send_next(tab: &mut Tab) {
    let started = Instant::now();
    loop {
        let words = tab.step("sendWords", serde_json::json!([]));
        if words == "Send next" {
            return;
        }
        assert!(
            started.elapsed() < Duration::from_secs(20),
            "Send never became Send next: {words}"
        );
        std::thread::sleep(Duration::from_millis(100));
    }
}

/// Waits until `tab` shows `count` queued messages, and returns them.
fn wait_queued(tab: &mut Tab, count: usize) -> serde_json::Value {
    let started = Instant::now();
    loop {
        let shown = queued_now(tab);
        if shown.as_array().map(Vec::len) == Some(count) {
            return shown;
        }
        assert!(
            started.elapsed() < Duration::from_secs(20),
            "never showed {count} queued messages: {shown}"
        );
        std::thread::sleep(Duration::from_millis(100));
    }
}

/// Waits until `tab` shows a queued message saying `text`, and returns the queue.
fn wait_queued_text(tab: &mut Tab, text: &str) -> serde_json::Value {
    let started = Instant::now();
    loop {
        let shown = queued_now(tab);
        if shown.to_string().contains(text) {
            return shown;
        }
        assert!(
            started.elapsed() < Duration::from_secs(20),
            "never showed queued {text:?}: {shown}"
        );
        std::thread::sleep(Duration::from_millis(100));
    }
}

fn queued_now(tab: &mut Tab) -> serde_json::Value {
    tab.step("queuedLines", serde_json::json!([]))
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
        self.held_request_keeping(method, None)
    }

    /// Whether a read whose path ends with `path` is held now.
    fn holding(&mut self, path: &str) -> bool {
        self.held.extend(self.browser.take_paused_requests());
        self.held.iter().any(|request| ends_at(request, path))
    }

    /// Like `held_request`, but keeps held every read whose path ends with
    /// `keep`, for a later release.
    fn held_request_keeping(&mut self, method: &str, keep: Option<&str>) -> serde_json::Value {
        let started = Instant::now();
        loop {
            self.held.extend(self.browser.take_paused_requests());
            let mut found = None;
            for request in std::mem::take(&mut self.held) {
                let kept = keep.is_some_and(|path| ends_at(&request, path));
                if found.is_none() && request["request"]["method"] == method {
                    found = Some(request);
                } else if request["request"]["method"] == "GET" && !kept {
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

    /// Lets every held request go on, passing over any the page abandoned,
    /// such as a read the document a reload replaced had in flight.
    fn release_remaining(&mut self) {
        self.held.extend(self.browser.take_paused_requests());
        for request in std::mem::take(&mut self.held) {
            let released = self.browser.page().try_call_session(
                "Fetch.continueRequest",
                serde_json::json!({ "requestId": request["requestId"] }),
            );
            if let Err(error) = released {
                assert_eq!(error["message"], "Invalid InterceptionId.", "{error}");
            }
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
/// Whether `request`'s address, without its query, ends with `path`.
fn ends_at(request: &serde_json::Value, path: &str) -> bool {
    request["request"]["url"]
        .as_str()
        .and_then(|url| url.split('?').next())
        .is_some_and(|url| url.ends_with(path))
}

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
