//! Loads the physician page in Chromium and fails when the document is empty.
//! Dioxus mounts into `#main`. A missing element leaves the visible page blank.

use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::{Child, Command, Stdio};
use std::sync::{Arc, OnceLock};
use std::time::{Duration, Instant};

use physician_auth::{AuthStore, Membership, MemoryAuthStore, PhysicianIdentity};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::sync::Mutex;

mod common;

static TEST_MUTEX: OnceLock<Mutex<()>> = OnceLock::new();
static BUILT_SITE: OnceLock<PathBuf> = OnceLock::new();

fn test_lock() -> &'static Mutex<()> {
    TEST_MUTEX.get_or_init(|| Mutex::new(()))
}

#[tokio::test]
#[ignore = "browser qualification; run explicitly with --ignored"]
async fn the_recorded_practice_is_visible() {
    let _guard = test_lock().lock().await;
    let site = build_site();
    // A temporary physician session and offline replay: never the operator's
    // XDG session, never Codex or OpenAI.
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
            && dom.contains("Coda hasn’t read Gmail yet.")
            && dom.contains("Coda hasn’t read Spruce yet.")
            && dom.contains("3:30 PM")
            && dom.contains("Mary Louis")
            && dom.contains("m3 11 9-8 9 8")
            && dom.contains("viewBox=\"0 0 24 24\"")
            && dom.contains("stroke-width=\"1.8\"")
            && dom.contains("box-sizing: border-box")
            && !dom.contains("Nothing needs you right now.")
            && !dom.contains("Connections"),
        "Today did not match the React shell:\n{dom}"
    );
    let workspace = dump_dom(
        &chrome,
        &format!("http://{address}/?patient=pw-39e51534-6ce4-43cf-b5f8-7c1ca6252d8e"),
    )
    .await;
    assert!(
        workspace.contains("Patient summary")
            && workspace.contains("Age 54 years")
            && workspace.contains("MRN 00000005")
            && workspace.contains("Gender Female")
            && workspace.contains("[1]")
            && workspace.contains("Source 2: Discharge Summary. Preview source.")
            && workspace.contains("Upcoming visits")
            && workspace.contains("3:30 PM")
            && workspace.contains("Follow-up visit")
            && workspace.contains("Preparation failed")
            && workspace.contains("Summary written")
            && workspace.contains("Today, 8:00 AM")
            && workspace.contains("Records as of")
            && workspace.contains("Today, 12:57 PM")
            && !workspace.contains("[[patient_source")
            && !workspace.contains("Write a message")
            && !workspace.contains("Visit preparation failed. No new briefing was completed."),
        "the workspace did not match the React patient page:\n{workspace}"
    );
    // Exercise the legacy preview ingress through the running server, then
    // verify what the physician sees in the compiled client. Submitted source
    // and approval claims must not become patient-origin attribution.
    let patient = "pw-39e51534-6ce4-43cf-b5f8-7c1ca6252d8e";
    let http = reqwest::Client::new();
    let workspace_url = format!("http://{address}/patient-workspaces/{patient}");
    let invitation: serde_json::Value = http
        .post(format!("{workspace_url}/invite"))
        .send()
        .await
        .unwrap()
        .error_for_status()
        .unwrap()
        .json()
        .await
        .unwrap();
    http.post(format!("http://{address}/patient-invitations/accept"))
        .json(&serde_json::json!({"token": invitation["token"], "duet_account_id": "synthetic-preview-account"}))
        .send().await.unwrap().error_for_status().unwrap();
    http.put(format!("{workspace_url}/consent"))
        .json(&serde_json::json!({"duet_account_id": "synthetic-preview-account", "consent_version": "v1"}))
        .send().await.unwrap().error_for_status().unwrap();
    http.post(format!("{workspace_url}/duet-sync"))
        .json(&serde_json::json!({"items": [{
            "record_id": "synthetic-preview", "revision_cursor": "revision-1",
            "content_hash": "synthetic-content", "source_identity": "synthetic-source",
            "origin_label": "via the patient's Duet", "patient_approved_note": true,
            "category": "clinical_note", "title": "Preview source attribution",
            "summary": "Synthetic content entered through the physician preview.",
            "recorded_at": "2026-10-25"
        }]}))
        .send()
        .await
        .unwrap()
        .error_for_status()
        .unwrap();
    let preview = dump_dom(&chrome, &format!("http://{address}/?patient={patient}")).await;
    assert!(
        preview.contains("Preview records") && preview.contains("Physician-entered preview"),
        "{preview}"
    );
    assert!(
        !preview.contains("Patient-shared records") && !preview.contains("via the patient's Duet"),
        "{preview}"
    );

    // `?chat=` opens a conversation the recorded workspace does not list.
    let chat = dump_dom(
        &chrome,
        &format!(
            "http://{address}/?patient=pw-39e51534-6ce4-43cf-b5f8-7c1ca6252d8e&chat=chat-from-address"
        ),
    )
    .await;
    assert!(
        chat.contains("Patient summary")
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
#[tokio::test]
#[ignore = "browser qualification; run explicitly with --ignored"]
async fn a_new_conversation_is_listed_after_a_reload_without_its_address() {
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
    let patient = "pw-39e51534-6ce4-43cf-b5f8-7c1ca6252d8e";
    let workspace_url = format!("http://{address}/?patient={patient}");
    let chrome = chromium();
    let mut browser = CdpBrowser::launch(&chrome, &workspace_url).await;
    browser
        .wait_for_text("This workspace has no chats.", Duration::from_secs(20))
        .await;

    // The empty state and the sidebar both offer it; use the sidebar's.
    let clicked = browser
        .eval("(() => { const b = document.querySelector('button.sidebar-new-conversation'); if (!b) return false; b.click(); return true; })()")
        .await;
    assert_eq!(
        clicked.pointer("/result/result/value"),
        Some(&serde_json::json!(true)),
        "no New conversation button in the sidebar: {}",
        browser.dom().await
    );
    browser
        .wait_for_text(
            "What would you like Coda to review?",
            Duration::from_secs(10),
        )
        .await;
    let address_chat = browser
        .eval("new URL(location.href).searchParams.get('chat')")
        .await;
    let chat = address_chat
        .pointer("/result/result/value")
        .and_then(|value| value.as_str())
        .unwrap_or_default()
        .to_owned();
    assert!(
        chat.starts_with("chat-"),
        "the address names no chat: {address_chat}"
    );

    let message = "Review the discharge summary before the follow-up visit";
    // There is no Save button: typing saves itself once the physician
    // pauses, confirms beside the composer, and an edit clears the mark.
    let notice =
        r#"document.querySelector('.composer-toolbar .composer-notice')?.textContent ?? ''"#;
    let buttons = browser
        .eval("[...document.querySelectorAll('.composer-toolbar button')].map(b => b.textContent.trim())")
        .await;
    assert_eq!(
        buttons.pointer("/result/result/value"),
        Some(&serde_json::json!(["Dictate", "Send"])),
        "the composer still offers a manual Save: {buttons}"
    );
    browser.type_composer(message).await;
    assert_eq!(
        browser.settled_mark().await,
        "Saved",
        "autosave gave no confirmation"
    );
    let edited = browser
        .eval(&format!(
            r#"new Promise(resolve => {{
            const box = document.querySelector('textarea.composer-input');
            Object.getOwnPropertyDescriptor(HTMLTextAreaElement.prototype, 'value').set.call(box, box.value + ' ');
            box.dispatchEvent(new Event('input', {{bubbles: true}}));
            setTimeout(() => resolve({notice}), 200);
        }})"#
        ))
        .await;
    assert_eq!(
        edited.pointer("/result/result/value"),
        Some(&serde_json::json!("")),
        "an edit left the Saved mark: {edited}"
    );
    browser
        .eval(&format!(
            r#"new Promise(resolve => {{
            const box = document.querySelector('textarea.composer-input');
            Object.getOwnPropertyDescriptor(HTMLTextAreaElement.prototype, 'value').set.call(box, {});
            box.dispatchEvent(new Event('input', {{bubbles: true}}));
            setTimeout(() => {{
                const send = [...document.querySelectorAll('.composer-toolbar button')].find(b => b.textContent.trim() === 'Send');
                send.click();
                resolve(true);
            }}, 100);
        }})"#,
            serde_json::to_string(message).unwrap()
        ))
        .await;
    // The inline session saves the turn before calling the (empty) replay.
    browser
        .wait_for_text(message, Duration::from_secs(15))
        .await;

    browser.navigate(&workspace_url).await;
    let html = browser
        .wait_for_text(message, Duration::from_secs(20))
        .await;
    assert!(
        !html.contains("This workspace has no chats.")
            && html.contains("sidebar-conversation-link"),
        "the reloaded workspace did not list the new chat:\n{html}"
    );
    let reopened = browser
        .eval("new URL(location.href).searchParams.get('chat')")
        .await;
    assert_eq!(
        reopened.pointer("/result/result/value"),
        Some(&serde_json::json!(chat)),
        "the address must reflect the conversation selected after a workspace-only reload: {reopened}"
    );

    // A second new conversation never shows the listed chat's lines, not even
    // for the frame before its own (empty) read answers. Every render after the
    // click is checked until the empty state; the click itself changes no DOM,
    // so the page before it (still showing the old chat) is not counted.
    let stale = browser
        .eval(&format!(
            r#"new Promise(resolve => {{
            const old = {};
            const region = () => document.querySelector('[aria-label="Conversation"]');
            const lines = () => [...(region()?.querySelectorAll('.formatted-message') ?? [])].map(m => m.textContent);
            let seen = false;
            const check = () => {{
                if (lines().some(t => t.includes(old))) seen = true;
                if (region()?.querySelector('.conversation-empty')) {{ observer.disconnect(); resolve({{seen, done: true}}); }}
            }};
            const observer = new MutationObserver(check);
            observer.observe(document.body, {{subtree: true, childList: true, characterData: true}});
            document.querySelector('button.sidebar-new-conversation').click();
            setTimeout(() => {{ observer.disconnect(); resolve({{seen, done: false}}); }}, 10000);
        }})"#,
            serde_json::to_string(message).unwrap()
        ))
        .await;
    assert_eq!(
        stale.pointer("/result/result/value"),
        Some(&serde_json::json!({"seen": false, "done": true})),
        "a new conversation showed the previous chat's lines: {stale}"
    );
    browser
        .eval("document.querySelector('button.sidebar-conversation-link').click()")
        .await;
    browser
        .wait_for_text(message, Duration::from_secs(10))
        .await;
    let selected = browser
        .eval("new URL(location.href).searchParams.get('chat')")
        .await;
    assert_eq!(
        selected.pointer("/result/result/value"),
        Some(&serde_json::json!(chat)),
        "selecting the saved conversation left the new conversation in the address: {selected}"
    );
    let previous_document = browser.eval("performance.timeOrigin").await;
    browser.reload().await;
    let reloaded_document = browser.eval("performance.timeOrigin").await;
    assert_ne!(
        reloaded_document.pointer("/result/result/value"),
        previous_document.pointer("/result/result/value"),
        "the conversation must be reopened in a new document, not read from the old page"
    );
    browser
        .wait_for_text(message, Duration::from_secs(20))
        .await;
    browser
        .eval("document.querySelector('button[aria-label=\"Open Today\"]').click()")
        .await;
    browser
        .wait_for_text("Coda hasn’t read Gmail yet.", Duration::from_secs(10))
        .await;
    let address = browser.eval("[new URL(location.href).searchParams.get('patient'), new URL(location.href).searchParams.get('chat')]").await;
    assert_eq!(
        address.pointer("/result/result/value"),
        Some(&serde_json::json!([null, null])),
        "Today retained a patient conversation in its address: {address}"
    );
    browser
        .eval("document.querySelector('button[aria-label=\"Find patient\"]').click()")
        .await;
    browser.search_for("Jake").await;
    browser.wait_for_text("DOB", Duration::from_secs(10)).await;
    browser
        .eval("document.querySelector('.dialog-results button').click()")
        .await;
    browser
        .wait_for_text("Patient summary", Duration::from_secs(10))
        .await;
    let address = browser.eval("[new URL(location.href).searchParams.get('patient'), new URL(location.href).searchParams.get('chat')]").await;
    assert_eq!(
        address.pointer("/result/result/value"),
        Some(&serde_json::json!([
            "pw-82c44e9b-58c0-4af1-bd21-771bf77e101d",
            null
        ])),
        "changing patient retained the previous patient's conversation: {address}"
    );
    browser.reload().await;
    browser
        .wait_for_text("Patient summary", Duration::from_secs(20))
        .await;
}

/// Two tabs on one chat with debounced autosave: a tab whose save names an
/// older revision is fenced (nothing written, its text kept, autosave paused)
/// and offered Retry or Use newer text. Use newer text discards the edit;
/// Retry names exactly the revision shown, so a draft that moved again shows
/// another conflict instead of being overwritten. Typing during a long save
/// gets a trailing save, and a save still running when the physician opens
/// another chat never marks that chat.
#[tokio::test]
#[ignore = "browser qualification; run explicitly with --ignored"]
async fn a_stale_tab_is_not_saved_and_a_save_never_marks_another_chat() {
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
    let patient = "pw-39e51534-6ce4-43cf-b5f8-7c1ca6252d8e";
    let chat = "chat-draft-fence";
    let chat_url = format!("http://{address}/?patient={patient}&chat={chat}");
    let server_draft = || async {
        reqwest::get(format!(
            "http://{address}/physician-state/drafts/{patient}/{chat}"
        ))
        .await
        .unwrap()
        .json::<serde_json::Value>()
        .await
        .unwrap()
    };
    let chrome = chromium();
    let mut tab_a = CdpBrowser::launch(&chrome, &chat_url).await;
    let mut tab_b = CdpBrowser::launch(&chrome, &chat_url).await;
    for tab in [&mut tab_a, &mut tab_b] {
        tab.wait_for_text(
            "What would you like Coda to review?",
            Duration::from_secs(20),
        )
        .await;
    }

    // Both tabs read the empty draft (revision 0). A's autosave lands first.
    tab_a.type_composer("Older text from tab A").await;
    assert_eq!(tab_a.settled_mark().await, "Saved");
    assert_eq!(
        server_draft().await,
        serde_json::json!({"revision": 1, "text": "Older text from tab A"})
    );
    // B still names revision 0: its autosave is refused, nothing is written,
    // B keeps its text and is shown the newer draft with both choices.
    tab_b.type_composer("Newer text from tab B").await;
    let conflict = tab_b.wait_conflict().await;
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
        tab_b.composer().await.0,
        "Newer text from tab B",
        "a refused save dropped the stale tab's text"
    );
    // Autosave stays paused while the choice waits: more typing writes nothing.
    tab_b
        .type_composer("Newer text from tab B, still mine")
        .await;
    tokio::time::sleep(Duration::from_millis(1_500)).await;
    assert_eq!(server_draft().await["revision"], 1);

    // Use newer text discards B's edit and shows A's saved draft.
    tab_b.click_conflict("Use newer text").await;
    tab_b
        .wait_composer("Older text from tab A", Duration::from_secs(5))
        .await;
    assert!(tab_b.conflict().await.is_none());
    tokio::time::sleep(Duration::from_millis(1_500)).await;
    assert_eq!(
        server_draft().await,
        serde_json::json!({"revision": 1, "text": "Older text from tab A"}),
        "Use newer text wrote to the server"
    );

    // B saves over revision 1; A, still at revision 1, now conflicts.
    tab_b.type_composer("Newer text from tab B").await;
    assert_eq!(tab_b.settled_mark().await, "Saved");
    tab_a.type_composer("Tab A insists").await;
    assert_eq!(
        tab_a.wait_conflict().await,
        (2, "Newer text from tab B".to_owned())
    );
    // The draft moves again before A chooses: Retry names the revision A was
    // shown (2), so it is refused and the newest draft is shown instead.
    tab_b.type_composer("Tab B moved again").await;
    assert_eq!(tab_b.settled_mark().await, "Saved");
    assert_eq!(server_draft().await["revision"], 3);
    tab_a.click_conflict("Retry").await;
    let started = Instant::now();
    loop {
        if tab_a.conflict().await == Some((3, "Tab B moved again".to_owned())) {
            break;
        }
        assert!(
            started.elapsed() < Duration::from_secs(10),
            "Retry over a draft that moved again showed no new conflict: {:?}",
            tab_a.conflict().await
        );
        tokio::time::sleep(Duration::from_millis(60)).await;
    }
    assert_eq!(
        server_draft().await,
        serde_json::json!({"revision": 3, "text": "Tab B moved again"}),
        "Retry overwrote a revision the physician was never shown"
    );
    // Retry over the revision now shown replaces it with A's text.
    tab_a.click_conflict("Retry").await;
    assert_eq!(tab_a.settled_mark().await, "Saved");
    assert_eq!(
        server_draft().await,
        serde_json::json!({"revision": 4, "text": "Tab A insists"})
    );
    drop(tab_b);

    // Reopened, the chat shows its saved draft.
    tab_a.navigate(&chat_url).await;
    tab_a
        .wait_composer("Tab A insists", Duration::from_secs(20))
        .await;

    // Typing during a long save still gets a trailing save of the newer text.
    tab_a
        .call(
            "Fetch.enable",
            serde_json::json!({ "patterns": [{ "urlPattern": "*/physician-state/drafts/*" }] }),
        )
        .await;
    tab_a.type_composer("Edited during").await;
    let put = tab_a.held_request("PUT").await;
    assert_eq!(tab_a.composer().await.1, "Saving…");
    tab_a.type_composer("Edited during a long save").await;
    tab_a.release(&put).await;
    let trailing = tab_a.held_request("PUT").await;
    let body = CdpBrowser::held_body(&trailing);
    assert_eq!(
        body,
        serde_json::json!({"expected_revision": 5, "text": "Edited during a long save"}),
        "the trailing save did not name the revision the first save produced"
    );
    tab_a.release(&trailing).await;
    let started = Instant::now();
    while server_draft().await["revision"] != 6 {
        assert!(
            started.elapsed() < Duration::from_secs(10),
            "the trailing save never landed"
        );
        tab_a.release_all().await;
        tokio::time::sleep(Duration::from_millis(30)).await;
    }
    assert_eq!(
        server_draft().await,
        serde_json::json!({"revision": 6, "text": "Edited during a long save"})
    );

    // Hold the next save in flight, open another chat, then let it land.
    tab_a.type_composer("Edited after reopening").await;
    let put = tab_a.held_request("PUT").await;
    assert_eq!(
        tab_a.composer().await.1,
        "Saving…",
        "a running save showed no Saving mark"
    );
    tab_a
        .eval("document.querySelector('button.sidebar-new-conversation').click()")
        .await;
    let started = Instant::now();
    while new_chat(&mut tab_a).await == chat {
        assert!(
            started.elapsed() < Duration::from_secs(10),
            "New conversation did not open"
        );
        tokio::time::sleep(Duration::from_millis(30)).await;
    }
    assert_eq!(
        tab_a.composer().await,
        (String::new(), String::new()),
        "the new chat showed the other chat's draft or its Saving mark"
    );
    tab_a.release(&put).await;
    // The save lands on the server for its own chat ...
    let started = Instant::now();
    while server_draft().await["revision"] != 7 {
        assert!(
            started.elapsed() < Duration::from_secs(10),
            "the held save never landed"
        );
        tab_a.release_all().await;
        tokio::time::sleep(Duration::from_millis(30)).await;
    }
    assert_eq!(
        server_draft().await,
        serde_json::json!({"revision": 7, "text": "Edited after reopening"})
    );
    // ... and its answer never marks the chat now shown.
    for _ in 0..10 {
        tab_a.release_all().await;
        assert_eq!(
            tab_a.composer().await,
            (String::new(), String::new()),
            "another chat's save marked this chat"
        );
        tokio::time::sleep(Duration::from_millis(50)).await;
    }
    tab_a.call("Fetch.disable", serde_json::json!({})).await;
}

/// Same-tab recovery through `sessionStorage`, signed in (a page restores
/// only under a known physician): text typed just before a reload, text
/// whose save was still on its way, and text whose save landed but whose
/// answer was lost all come back without a duplicate write; a stale local
/// baseline asks for an explicit choice; sign-out forgets every record; and
/// denied storage keeps the text in memory and says reload recovery is off.
#[tokio::test]
#[ignore = "browser qualification; run explicitly with --ignored"]
async fn a_reloaded_tab_recovers_unsent_text_and_sign_out_forgets_it() {
    let _guard = test_lock().lock().await;
    let site = build_site();
    let server = common::serve_isolated(|| {
        // SAFETY: tests in this binary hold `test_lock()` before mutating
        // process env, and `serve_isolated` holds its start lock here.
        unsafe {
            env::set_var("PHYSICIAN_SITE_DIR", &site);
            env::set_var("PHYSICIAN_AUTH", "required");
            env::remove_var("PHYSICIAN_AUTH_PROOF_TOKENS");
            env::remove_var("PHYSICIAN_UPSTREAM");
            env::remove_var("PHYSICIAN_DATABASE_URL");
        }
    })
    .await;
    let address = server.address;
    let patient = "pw-39e51534-6ce4-43cf-b5f8-7c1ca6252d8e";
    let chat = "chat-recovery";
    let chat_url = format!("http://{address}/?patient={patient}&chat={chat}");
    let mut tab = CdpBrowser::launch(&chromium(), &format!("http://{address}/")).await;
    tab.wait_for_text("Work email", Duration::from_secs(20))
        .await;
    tab.request_sign_in("physician@codahealth.test").await;
    tab.wait_for_text("Mary Louis", Duration::from_secs(20))
        .await;
    let bearer = tab
        .eval("sessionStorage.getItem('coda.physician.session')")
        .await
        .pointer("/result/result/value")
        .and_then(|value| value.as_str())
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
    let open_chat = async |tab: &mut CdpBrowser| {
        tab.wait_for_text(
            "What would you like Coda to review?",
            Duration::from_secs(20),
        )
        .await;
    };
    tab.navigate(&chat_url).await;
    open_chat(&mut tab).await;

    // 1. Reloaded before the debounce: the text is only in this tab's
    // storage, comes back, and then saves.
    tab.type_composer("Typed just before the reload").await;
    assert_eq!(server_draft().await["revision"], 0);
    assert_eq!(
        tab.recovery_keys().await.len(),
        1,
        "the unsent text was not kept for a reload"
    );
    tab.reload().await;
    tab.wait_composer("Typed just before the reload", Duration::from_secs(20))
        .await;
    assert_eq!(tab.settled_mark().await, "Saved");
    assert_eq!(
        server_draft().await,
        serde_json::json!({"revision": 1, "text": "Typed just before the reload"})
    );

    // 2. Reloaded while the save was held before the server: it comes back
    // and saves over the revision this tab last saw.
    tab.call(
        "Fetch.enable",
        serde_json::json!({ "patterns": [{ "urlPattern": "*/physician-state/drafts/*" }] }),
    )
    .await;
    tab.type_composer("Typed while a save was on its way").await;
    let _held = tab.held_request("PUT").await;
    tab.reload().await;
    tab.release_all().await;
    tab.call("Fetch.disable", serde_json::json!({})).await;
    tab.release_all().await;
    tab.wait_composer("Typed while a save was on its way", Duration::from_secs(20))
        .await;
    assert_eq!(tab.settled_mark().await, "Saved");
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
    )
    .await;
    tab.type_composer("Saved but never answered").await;
    let answer = tab.held_request("PUT").await;
    assert_eq!(
        server_draft().await,
        serde_json::json!({"revision": 3, "text": "Saved but never answered"})
    );
    tab.call(
        "Fetch.failRequest",
        serde_json::json!({ "requestId": answer["requestId"], "errorReason": "Failed" }),
    )
    .await;
    assert_eq!(tab.settled_mark().await, "Not saved");
    tab.call("Fetch.disable", serde_json::json!({})).await;
    tab.release_all().await;
    tab.reload().await;
    tab.wait_composer("Saved but never answered", Duration::from_secs(20))
        .await;
    tokio::time::sleep(Duration::from_millis(2_500)).await;
    assert!(
        tab.conflict().await.is_none(),
        "this tab's own lost save showed as a conflict"
    );
    assert_eq!(
        server_draft().await,
        serde_json::json!({"revision": 3, "text": "Saved but never answered"}),
        "the recovered text was written twice"
    );
    assert!(
        tab.recovery_keys().await.is_empty(),
        "a record outlived the text it kept"
    );

    // 4. Another writer moved the draft before this tab's text was saved:
    // after the reload the physician chooses; nothing is overwritten.
    tab.call(
        "Fetch.enable",
        serde_json::json!({ "patterns": [{ "urlPattern": "*/physician-state/drafts/*" }] }),
    )
    .await;
    tab.type_composer("Local edit nobody saved").await;
    let _held = tab.held_request("PUT").await;
    let moved = http
        .put(&draft_url)
        .bearer_auth(&bearer)
        .json(&serde_json::json!({"expected_revision": 3, "text": "Written elsewhere"}))
        .send()
        .await
        .unwrap();
    assert!(moved.status().is_success());
    tab.reload().await;
    tab.call("Fetch.disable", serde_json::json!({})).await;
    tab.release_all().await;
    assert_eq!(
        tab.wait_conflict().await,
        (4, "Written elsewhere".to_owned())
    );
    assert_eq!(tab.composer().await.0, "Local edit nobody saved");
    assert_eq!(server_draft().await["revision"], 4);
    tab.click_conflict("Use newer text").await;
    tab.wait_composer("Written elsewhere", Duration::from_secs(5))
        .await;
    assert!(tab.recovery_keys().await.is_empty());

    // 5. Sign-out forgets every record before anything else can read it.
    tab.type_composer("Unsent at sign-out").await;
    assert_eq!(tab.recovery_keys().await.len(), 1);
    tab.eval("document.querySelector('button[aria-label=\"Sign out of Coda\"]').click()")
        .await;
    tab.wait_for_text("Sign in to Coda", Duration::from_secs(10))
        .await;
    assert!(
        tab.recovery_keys().await.is_empty(),
        "sign-out left unsent text in this tab's storage"
    );
    drop(tab);

    // 6. Denied storage: the text stays on screen and saves; the page says
    // a reload cannot bring it back.
    let mut denied = CdpBrowser::launch(&chromium(), "about:blank").await;
    denied
        .call(
            "Page.addScriptToEvaluateOnNewDocument",
            serde_json::json!({"source": r#"(() => {
        const set = Storage.prototype.setItem;
        Storage.prototype.setItem = function (key, value) {
            if (String(key).startsWith('coda.physician.draft.')) {
                throw new DOMException('quota', 'QuotaExceededError');
            }
            return set.call(this, key, value);
        };
    })()"#}),
        )
        .await;
    denied.navigate(&format!("http://{address}/")).await;
    denied
        .wait_for_text("Work email", Duration::from_secs(20))
        .await;
    denied.request_sign_in("physician@codahealth.test").await;
    denied
        .wait_for_text("Mary Louis", Duration::from_secs(20))
        .await;
    denied.navigate(&chat_url).await;
    denied
        .wait_composer("Written elsewhere", Duration::from_secs(20))
        .await;
    denied.type_composer("Kept only in memory").await;
    denied
        .wait_for_text(
            "This tab can’t keep a reload copy of your draft.",
            Duration::from_secs(5),
        )
        .await;
    assert_eq!(denied.composer().await.0, "Kept only in memory");
    assert_eq!(denied.settled_mark().await, "Saved");
    drop(denied);

    // 7. Same-clinic panel removal answers 200 with demographics, not 404.
    // Recovery must be purged even on this successful workspace read.
    let mut removed = CdpBrowser::launch(&chromium(), &format!("http://{address}/")).await;
    removed
        .wait_for_text("Work email", Duration::from_secs(20))
        .await;
    removed.request_sign_in("physician@codahealth.test").await;
    removed
        .wait_for_text("Mary Louis", Duration::from_secs(20))
        .await;
    removed.navigate(&chat_url).await;
    removed
        .wait_composer("Kept only in memory", Duration::from_secs(20))
        .await;
    removed
        .type_composer("Must not survive panel removal")
        .await;
    assert_eq!(removed.recovery_keys().await.len(), 1);
    let removed_bearer = removed
        .eval("sessionStorage.getItem('coda.physician.session')")
        .await
        .pointer("/result/result/value")
        .and_then(|value| value.as_str())
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
    removed.reload().await;
    removed
        .wait_for_text("Clinic record · Not on your panel", Duration::from_secs(20))
        .await;
    assert!(
        removed.recovery_keys().await.is_empty(),
        "demographic-only access retained clinical recovery"
    );
}

/// Send from the keyboard (Ctrl+Enter) with text typed while the command is
/// still unanswered: acceptance removes only the submitted message, keeps
/// the later text, and the durable draft follows. A Send whose answer is lost
/// across a reload is asked about again with the same command key, never a
/// new one, and the message appears once.
#[tokio::test]
#[ignore = "browser qualification; run explicitly with --ignored"]
async fn keyboard_send_clears_only_the_sent_text_and_a_reload_reasks_the_same_command() {
    let _guard = test_lock().lock().await;
    let site = build_site();
    let server = common::serve_isolated(|| {
        // SAFETY: tests in this binary hold `test_lock()` before mutating
        // process env, and `serve_isolated` holds its start lock here.
        unsafe {
            env::set_var("PHYSICIAN_SITE_DIR", &site);
            env::set_var("PHYSICIAN_AUTH", "required");
            env::remove_var("PHYSICIAN_AUTH_PROOF_TOKENS");
            env::remove_var("PHYSICIAN_UPSTREAM");
            env::remove_var("PHYSICIAN_DATABASE_URL");
        }
    })
    .await;
    let address = server.address;
    let patient = "pw-39e51534-6ce4-43cf-b5f8-7c1ca6252d8e";
    let chat = "chat-keyboard-send";
    let chat_url = format!("http://{address}/?patient={patient}&chat={chat}");
    let mut tab = CdpBrowser::launch(&chromium(), &format!("http://{address}/")).await;
    tab.wait_for_text("Work email", Duration::from_secs(20))
        .await;
    tab.request_sign_in("physician@codahealth.test").await;
    tab.wait_for_text("Mary Louis", Duration::from_secs(20))
        .await;
    let bearer = tab
        .eval("sessionStorage.getItem('coda.physician.session')")
        .await
        .pointer("/result/result/value")
        .and_then(|value| value.as_str())
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
    let ctrl_enter = r#"(() => {
        const box = document.querySelector('textarea.composer-input');
        box.focus();
        box.dispatchEvent(new KeyboardEvent('keydown', {key: 'Enter', ctrlKey: true, bubbles: true}));
        return true;
    })()"#;
    tab.navigate(&chat_url).await;
    tab.wait_for_text(
        "What would you like Coda to review?",
        Duration::from_secs(20),
    )
    .await;

    // 1. Typed, autosaved, then sent from the keyboard while the command is
    // held; more typing lands before the answer.
    let first = "Check the potassium trend";
    tab.type_composer(first).await;
    assert_eq!(tab.settled_mark().await, "Saved");
    assert_eq!(server_draft().await["text"], first);
    tab.call(
        "Fetch.enable",
        serde_json::json!({ "patterns": [{ "urlPattern": "*/commands" }] }),
    )
    .await;
    tab.eval(ctrl_enter).await;
    let held = tab.held_request("POST").await;
    let key = CdpBrowser::held_body(&held)["idempotency_key"].clone();
    assert_eq!(
        CdpBrowser::held_body(&held)["payload"]["text"],
        first,
        "keyboard Send did not submit the composer text"
    );
    tab.type_composer(&format!("{first} and the sodium")).await;
    tab.release(&held).await;
    tab.call("Fetch.disable", serde_json::json!({})).await;
    tab.wait_composer("and the sodium", Duration::from_secs(10))
        .await;
    tab.wait_for_text(first, Duration::from_secs(10)).await;
    assert_eq!(tab.settled_mark().await, "Saved");
    assert_eq!(
        server_draft().await["text"],
        "and the sodium",
        "the durable draft still held the sent message"
    );

    // 2. Sent, but the answer is lost and the page reloads: the reload asks
    // again with the same key; the message lands once and clears.
    let second = "Order a repeat BMP";
    tab.type_composer(second).await;
    assert_eq!(tab.settled_mark().await, "Saved");
    tab.call(
        "Fetch.enable",
        serde_json::json!({ "patterns": [{
            "urlPattern": "*/commands",
            "requestStage": "Response"
        }] }),
    )
    .await;
    tab.eval(
        "[...document.querySelectorAll('.composer-toolbar button')].find(b => b.textContent.trim() === 'Send').click()",
    )
    .await;
    let answered = tab.held_request("POST").await;
    let lost_key = CdpBrowser::held_body(&answered)["idempotency_key"].clone();
    assert_ne!(lost_key, key);
    tab.call(
        "Fetch.failRequest",
        serde_json::json!({ "requestId": answered["requestId"], "errorReason": "Failed" }),
    )
    .await;
    tab.call(
        "Fetch.enable",
        serde_json::json!({ "patterns": [{ "urlPattern": "*/commands" }] }),
    )
    .await;
    tab.reload().await;
    let reasked = tab.held_request("POST").await;
    assert_eq!(
        CdpBrowser::held_body(&reasked)["idempotency_key"],
        lost_key,
        "the reload issued a new command for a Send whose answer was lost"
    );
    tab.release(&reasked).await;
    tab.call("Fetch.disable", serde_json::json!({})).await;
    tab.release_all().await;
    tab.wait_for_text(second, Duration::from_secs(20)).await;
    tab.wait_composer("", Duration::from_secs(10)).await;
    let lines = tab
        .eval(&format!(
            "[...document.querySelectorAll('[aria-label=\"Conversation\"] .formatted-message')].filter(m => m.textContent.includes({})).length",
            serde_json::to_string(second).unwrap()
        ))
        .await;
    assert_eq!(
        lines.pointer("/result/result/value"),
        Some(&serde_json::json!(1)),
        "the re-asked message did not appear exactly once"
    );
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

async fn new_chat(browser: &mut CdpBrowser) -> String {
    browser
        .eval("new URL(location.href).searchParams.get('chat') ?? ''")
        .await
        .pointer("/result/result/value")
        .and_then(|value| value.as_str())
        .unwrap_or_default()
        .to_owned()
}

#[tokio::test]
#[ignore = "browser qualification; run explicitly with --ignored"]
async fn the_first_event_connection_reads_changes_after_the_initial_snapshot() {
    let _guard = test_lock().lock().await;
    let site = build_site();
    let server = common::serve_isolated(|| {
        // SAFETY: this binary holds test_lock and serve_isolated's start lock.
        unsafe {
            env::set_var("PHYSICIAN_SITE_DIR", &site);
            env::remove_var("PHYSICIAN_AUTH");
            env::remove_var("PHYSICIAN_AUTH_PROOF_TOKENS");
            env::remove_var("PHYSICIAN_UPSTREAM");
        }
    })
    .await;
    let base = format!("http://{}", server.address);
    let patient = "pw-39e51534-6ce4-43cf-b5f8-7c1ca6252d8e";
    let chat = "chat-before-events";
    let mut browser = CdpBrowser::launch(&chromium(), "about:blank").await;
    // Hold the actual first event request before it reaches the server. The
    // initial snapshot can complete, then another client saves a turn while
    // this page has no subscription. No synthetic event or refresh is sent.
    browser
        .call(
            "Page.addScriptToEvaluateOnNewDocument",
            serde_json::json!({"source": r#"(() => {
        const original = window.fetch.bind(window);
        window.__eventRequests = 0;
        window.fetch = (input, init) => {
            const url = input instanceof Request ? input.url : String(input);
            if (url.includes('/patient-workspaces/events')) {
                window.__eventRequests++;
                if (window.__eventRequests === 1) {
                    return new Promise(resolve => {
                        window.__connectEvents = () => resolve(original(input, init));
                    });
                }
            }
            return original(input, init);
        };
    })()"#}),
        )
        .await;
    browser
        .navigate(&format!("{base}/?patient={patient}&chat={chat}"))
        .await;
    browser
        .wait_for_text(
            "What would you like Coda to review?",
            Duration::from_secs(20),
        )
        .await;
    let message = "Review the update accepted before this page subscribed";
    let client = reqwest::Client::new();
    let accepted: serde_json::Value = client.post(format!("{base}/commands"))
        .json(&serde_json::json!({"type":"message.submit","idempotency_key":"before-events",
            "payload":{"patient_workspace_id":patient,"chat_id":chat,"text":message,"physician_time_zone":"UTC"}}))
        .send().await.unwrap().json().await.unwrap();
    assert_eq!(accepted["outcome"], "accepted", "{accepted}");
    let snapshot: serde_json::Value = client
        .get(format!(
            "{base}/patient-workspaces/{patient}/chats/{chat}/snapshot"
        ))
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
        !browser.dom().await.contains(message),
        "the page must still hold the earlier snapshot"
    );
    let connected = browser.eval("(() => { if (!window.__connectEvents) return false; window.__connectEvents(); return true; })()").await;
    assert_eq!(
        connected.pointer("/result/result/value"),
        Some(&serde_json::json!(true))
    );
    browser
        .wait_for_text(message, Duration::from_secs(10))
        .await;
    let count = browser.eval("window.__eventRequests").await;
    assert_eq!(
        count.pointer("/result/result/value"),
        Some(&serde_json::json!(1)),
        "the first connection must reconcile without waiting for reconnect"
    );
}

#[tokio::test]
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
    let app = physician_sample::recorded().layer(axum::middleware::from_fn(
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
        .nest("/override", physician_sample::recorded())
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
    let chrome = chromium();
    let mut browser = CdpBrowser::launch(&chrome, &format!("http://{address}/physician/")).await;
    browser
        .wait_for_text("Mary Louis", Duration::from_secs(15))
        .await;
    let assets = browser
        .eval(
            r#"(async () => {
        const logo = document.querySelector('img.coda-logo');
        await logo.decode();
        const icon = document.querySelector('link[rel="icon"]');
        const response = await fetch(icon.href);
        return logo.naturalWidth > 0 && new URL(logo.src).pathname.startsWith('/physician/assets/')
            && new URL(icon.href).pathname === '/physician/favicon.svg' && response.ok
            && (await response.text()).includes('<svg');
    })()"#,
        )
        .await;
    assert_eq!(
        assets.pointer("/result/result/value"),
        Some(&serde_json::json!(true)),
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
    browser
        .navigate(&format!(
            "http://{address}/physician/?api=http%3A%2F%2F{address}%2Foverride"
        ))
        .await;
    browser
        .wait_for_text("Mary Louis", Duration::from_secs(15))
        .await;
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
        physician_proxy: public_site::proxy::ProxyConfig::new(
            &format!("http://{upstream}"),
            &format!("http://{public}"),
            None,
            false,
            false,
        )
        .unwrap(),
        ..Default::default()
    };
    public_site::server::router_with_options(
        public_site::prerender(&public_site::Config {
            listen_addr: public,
        }),
        runtime,
        [],
        public_site::server::Readiness::ready(),
    )
}

#[tokio::test]
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
    let app = physician_sample::authed_router(physician_sample::recorded()).await;
    let upstream = tokio::spawn(async move { axum::serve(listener, app).await.unwrap() });
    wait_until_ready(upstream_address).await;
    let proxy = tokio::spawn(async move {
        axum::serve(public_listener, public_proxy(upstream_address, address))
            .await
            .unwrap()
    });

    let mut browser =
        CdpBrowser::launch(&chromium(), &format!("http://{address}/physician/")).await;
    browser
        .wait_for_text("Work email", Duration::from_secs(15))
        .await;
    browser.request_sign_in(email).await;
    let waiting = browser
        .wait_for_text("Check your email", Duration::from_secs(10))
        .await;
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
    browser
        .navigate(&format!(
            "data:text/html;base64,{}",
            base64::engine::general_purpose::STANDARD.encode(html)
        ))
        .await;
    browser
        .wait_for_text("This link expires", Duration::from_secs(10))
        .await;
    browser.eval("document.querySelector('a').click()").await;
    let signed_in = browser
        .wait_for_text("Staging Physician", Duration::from_secs(15))
        .await;
    assert!(
        !signed_in.contains("Sign-in code"),
        "the email link must sign in without a code field or another confirmation"
    );
    let location = browser.eval("location.href").await;
    assert_eq!(
        location["result"]["result"]["value"].as_str(),
        Some(format!("http://{address}/physician/").as_str()),
        "the email and token must leave the address bar before redemption"
    );
    browser.call("Page.reload", serde_json::json!({})).await;
    browser
        .wait_for_text("Staging Physician", Duration::from_secs(10))
        .await;
    // A newly mailed link must be redeemed even when a browser session exists.
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
    browser.navigate(next_link).await;
    browser
        .wait_for_text("Staging Physician", Duration::from_secs(10))
        .await;
    browser
        .eval("document.querySelector('[aria-label=\"Sign out of Coda\"]').click()")
        .await;
    browser
        .wait_for_text("Sign in to Coda", Duration::from_secs(10))
        .await;

    // The same message cannot establish another session after its first use.
    browser.navigate(next_link).await;
    let invalid = browser
        .wait_for_text("That sign-in link", Duration::from_secs(10))
        .await;
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

#[tokio::test]
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
    store.register_patient("practice-clinic-a", mary).await;
    let admin_identity = PhysicianIdentity {
        practice_id: "practice-clinic-a".to_owned(),
        physician_id: "phys-admin".to_owned(),
    };
    store
        .assign_panel(&admin_identity, "phys-colleague", mary, Membership::Allowed)
        .await
        .unwrap();
    // Jake belongs to a completely different practice (practice-clinic-b).
    store.register_patient("practice-clinic-b", jake).await;

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

    let chrome = chromium();
    let mut browser = CdpBrowser::launch(&chrome, &format!("http://{address}/physician/")).await;

    // 1. Unauthenticated visit renders SignInScreen (never Today or clinical content).
    let sign_in_dom = browser
        .wait_for_text("Sign in to Coda", Duration::from_secs(15))
        .await;
    assert!(
        sign_in_dom.contains("Work email")
            && sign_in_dom.contains("Send sign-in link")
            && !sign_in_dom.contains("Mary Louis"),
        "expected SignInScreen before authentication:\n{sign_in_dom}"
    );

    // 2. Local proof delivery signs in through the same redemption path.
    browser.request_sign_in("admin@clinic-a.test").await;
    let today_dom = browser
        .wait_for_text("Dr. Clinic Admin", Duration::from_secs(10))
        .await;
    assert!(
        today_dom.contains("Today") && !today_dom.contains("Mary Louis"),
        "Mary Louis is not on Dr. Clinic Admin's panel yet and must not appear on Today:\n{today_dom}"
    );

    // Reload through the real proxy: sessionStorage must restore the bearer
    // session rather than rely on the prior in-memory client.
    browser.call("Page.reload", serde_json::json!({})).await;
    tokio::time::sleep(Duration::from_millis(250)).await;
    browser
        .wait_for_text("Dr. Clinic Admin", Duration::from_secs(10))
        .await;
    let session = browser
        .eval("sessionStorage.getItem('coda.physician.session')")
        .await;
    let token = session
        .pointer("/result/result/value")
        .and_then(|value| value.as_str())
        .expect("stored bearer session")
        .to_owned();
    assert!(!token.is_empty());

    // 3. Cross-practice patient (Jake in practice-clinic-b) returns 404: not available, with zero data leak.
    browser
        .navigate(&format!("http://{address}/physician/?patient={jake}"))
        .await;
    let cross_practice_dom = browser
        .wait_for_text("That patient is not available.", Duration::from_secs(10))
        .await;
    assert!(
        cross_practice_dom.contains("Go to Today")
            && !cross_practice_dom.contains("Coda couldn’t load what needs you for this patient.")
            && !cross_practice_dom.contains("Jake")
            && !cross_practice_dom.contains("Patient summary")
            && !cross_practice_dom.contains("Patient demographics"),
        "cross-practice patient must not leak demographics or clinical data:\n{cross_practice_dom}"
    );

    // 4. Same-clinic non-panel patient (Mary in practice-clinic-a) renders clean Clinic Demographic Card.
    browser
        .navigate(&format!("http://{address}/physician/?patient={mary}"))
        .await;
    let demographic_dom = browser
        .wait_for_text("Clinic record · Not on your panel", Duration::from_secs(10))
        .await;
    assert!(
        demographic_dom.contains("Patient demographics")
            && demographic_dom.contains("Mary Louis")
            && demographic_dom.contains("Mar 18, 1972")
            && demographic_dom.contains("54 years")
            && demographic_dom.contains("Female")
            && demographic_dom.contains("00000005")
            && demographic_dom.contains("Assign to panel")
            && !demographic_dom.contains("Patient summary")
            && !demographic_dom.contains("Discharge Summary")
            && !demographic_dom.contains("This workspace has no chats."),
        "same-clinic non-panel patient should render clean demographic card:\n{demographic_dom}"
    );

    // Verify phone viewport (390x844) and prefers-reduced-motion on the demographic card.
    browser.set_viewport(390, 844, true).await;
    let mobile_dom = browser.dom().await;
    assert!(
        mobile_dom.contains("Patient demographics") && mobile_dom.contains("Assign to panel"),
        "demographic card should remain intact on phone viewport:\n{mobile_dom}"
    );
    browser.set_viewport(1280, 800, false).await;

    // 5. Admin clicks "Assign to panel" -> PUT /auth/panels/... unlocks full clinical workspace live.
    browser
        .eval(
            r#"(() => {
                const btn = document.querySelector('button.assign-panel-button');
                if (btn) btn.click();
                return Boolean(btn);
            })()"#,
        )
        .await;
    let unlocked_dom = browser
        .wait_for_text("Patient summary", Duration::from_secs(10))
        .await;
    assert!(
        unlocked_dom.contains("Mary Louis")
            && unlocked_dom.contains("Source 2: Discharge Summary. Preview source.")
            && unlocked_dom.contains("Remove from panel")
            && !unlocked_dom.contains("Clinic record · Not on your panel"),
        "assigning patient to panel must unlock full clinical workspace live:\n{unlocked_dom}"
    );

    // 6. Admin clicks "Remove from panel" -> DELETE /auth/panels/... returns live to demographic card.
    browser
        .eval(
            r#"(() => {
                const btn = document.querySelector('button.remove-panel-button');
                if (btn) btn.click();
                return Boolean(btn);
            })()"#,
        )
        .await;
    let relocked_dom = browser
        .wait_for_text("Clinic record · Not on your panel", Duration::from_secs(10))
        .await;
    assert!(
        relocked_dom.contains("Patient demographics") && !relocked_dom.contains("Patient summary"),
        "removing patient from panel must lock clinical workspace back to demographic card:\n{relocked_dom}"
    );

    // Sign-out crosses the proxy, revokes the real server session, and clears
    // browser persistence. Reload must remain at the gate.
    browser
        .eval("document.querySelector('[aria-label=\"Sign out of Coda\"]').click()")
        .await;
    browser
        .wait_for_text("Sign in to Coda", Duration::from_secs(10))
        .await;
    let stored = browser
        .eval("sessionStorage.getItem('coda.physician.session')")
        .await;
    assert_eq!(
        stored.pointer("/result/result/value"),
        Some(&serde_json::Value::Null)
    );
    let response = reqwest::Client::new()
        .get(format!("http://{address}/physician/today"))
        .bearer_auth(&token)
        .send()
        .await
        .unwrap();
    assert_eq!(response.status(), reqwest::StatusCode::UNAUTHORIZED);
    browser.call("Page.reload", serde_json::json!({})).await;
    tokio::time::sleep(Duration::from_millis(250)).await;
    browser
        .wait_for_text("Sign in to Coda", Duration::from_secs(10))
        .await;
    browser.sign_in_again().await;
    browser
        .wait_for_text("Dr. Clinic Admin", Duration::from_secs(10))
        .await;

    // 7. Deactivating the physician account terminates SSE through the proxy and returns the browser to SignInScreen.
    assert!(store.deactivate("admin@clinic-a.test").await);
    let revoked_dom = browser
        .wait_for_text("Sign in to Coda", Duration::from_secs(10))
        .await;
    assert!(
        revoked_dom.contains("Work email") && !revoked_dom.contains("Mary Louis"),
        "revoking session/account must terminate SSE and return to SignInScreen:\n{revoked_dom}"
    );

    proxy.abort();
    upstream.abort();
    unsafe {
        env::remove_var("PHYSICIAN_AUTH");
        env::remove_var("PHYSICIAN_AUTH_PROOF_TOKENS");
    }
}

#[tokio::test]
#[ignore = "browser qualification; run explicitly with --ignored"]
async fn the_local_simulation_panel_scenarios_and_investor_metric_render_and_interact() {
    let _guard = test_lock().lock().await;
    let site = build_site();
    unsafe {
        env::set_var("PHYSICIAN_SITE_DIR", &site);
    }
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    tokio::spawn(async move {
        axum::serve(listener, physician_sample::simulation_router())
            .await
            .unwrap();
    });
    wait_until_ready(address).await;

    let chrome = chromium();
    let today_dom = dump_dom(&chrome, &format!("http://{address}/")).await;
    assert!(
        today_dom.contains("Simulated practice walk — Connected care walk")
            && today_dom.contains("Items the physician touched per 100 patients per week")
            && today_dom.contains("20.0")
            && today_dom.contains("1 physician touches across 5 patients")
            && today_dom.contains("Critical lab arrival (ED reply)")
            && today_dom.contains("Lab correction supersedes ED reply")
            && today_dom.contains("Undecided medication guard")
            && today_dom.contains("Jake Smith (MRN 00000001)")
            && today_dom.contains("Tanvir Chowdhury (MRN 00000003)")
            && today_dom.contains("Dan Cohen (MRN 00000002)")
            && today_dom.contains("Teresa Hayes (MRN 00000004)")
            && today_dom.contains("Mary Louis (MRN 00000005)")
            && !today_dom.contains("Urgent potassium 6.3 mmol/L"),
        "Simulated Today did not render panel, scenarios, and investor metric:\n{today_dom}"
    );

    let jake_url = format!("http://{address}/?patient=pw-82c44e9b-58c0-4af1-bd21-771bf77e101d");
    let jake_dom = dump_dom(&chrome, &jake_url).await;
    assert!(
        jake_dom.contains("Jake Smith")
            && jake_dom.contains("Patient replies")
            && jake_dom.contains("Superseded by corrected labs")
            && jake_dom.contains("prior K+ 6.3 mmol/L entered in error")
            && jake_dom.contains("Active plan")
            && jake_dom.contains("Record home blood-pressure log for 1 week")
            && jake_dom.contains("Undecided medication recommendations")
            && jake_dom.contains("Losartan 50 mg daily — Held (undecided)")
            && jake_dom.contains("Approve into plan")
            && !jake_dom.contains("data-plan-item=\"plan-rec-jake-losartan-50\""),
        "Jake workspace did not guard undecided Losartan or show superseded ED reply:\n{jake_dom}"
    );

    // Switch to `lab-arrives-ed-reply` (pre-correction) and verify the urgent ED reply is active on Today.
    let http = reqwest::Client::new();
    let resp = http
        .post(format!("http://{address}/commands"))
        .json(&serde_json::json!({
            "type": "simulation.run",
            "payload": { "scenario": "lab-arrives-ed-reply" }
        }))
        .send()
        .await
        .expect("run lab-arrives-ed-reply");
    assert!(resp.status().is_success());

    let pre_correction_today = dump_dom(&chrome, &format!("http://{address}/")).await;
    assert!(
        pre_correction_today.contains("Urgent potassium 6.3 mmol/L — advise ED evaluation")
            && pre_correction_today.contains("Critical lab arrival (ED reply)"),
        "Pre-correction scenario did not surface urgent ED reply on Today:\n{pre_correction_today}"
    );

    // Admit corrected labs and approve undecided Losartan, then verify the UI reflects both transitions and updates the investor metric.
    let resp = http
        .post(format!("http://{address}/commands"))
        .json(&serde_json::json!({
            "type": "simulation.admit_corrected_labs",
            "payload": { "patient_workspace_id": "pw-82c44e9b-58c0-4af1-bd21-771bf77e101d" }
        }))
        .send()
        .await
        .expect("admit corrected labs");
    assert!(resp.status().is_success());

    let resp = http
        .post(format!("http://{address}/commands"))
        .json(&serde_json::json!({
            "type": "plan.decide_recommendation",
            "payload": {
                "patient_workspace_id": "pw-82c44e9b-58c0-4af1-bd21-771bf77e101d",
                "recommendation_id": "rec-jake-losartan-50"
            }
        }))
        .send()
        .await
        .expect("decide recommendation");
    assert!(resp.status().is_success());

    let jake_after_dom = dump_dom(&chrome, &jake_url).await;
    assert!(
        jake_after_dom.contains("Superseded by corrected labs")
            && jake_after_dom.contains("data-plan-item=\"plan-rec-jake-losartan-50\"")
            && jake_after_dom.contains("No undecided medication recommendations.")
            && jake_after_dom.contains("40.0"),
        "Jake workspace after lab correction + recommendation approval did not update:\n{jake_after_dom}"
    );
}

#[tokio::test]
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

    let chrome = chromium();
    let mut browser = CdpBrowser::launch(&chrome, "about:blank").await;
    // Requests whose URL matches `__hold` wait in the page until `__release()`,
    // so the test sees what shows while an answer is still on its way.
    browser
        .call(
            "Page.addScriptToEvaluateOnNewDocument",
            serde_json::json!({ "source": r#"(() => {
                const fetch = window.fetch.bind(window);
                window.__hold = '/practice-day';
                window.__held = [];
                window.fetch = (input, init) => {
                    const answer = fetch(input, init);
                    const url = typeof input === 'string' ? input : input.url;
                    if (window.__hold && new RegExp(window.__hold).test(url)) {
                        return new Promise(resolve => window.__held.push(() => resolve(answer)));
                    }
                    return answer;
                };
                window.__release = () => {
                    window.__hold = null;
                    window.__held.splice(0).forEach(release => release());
                };
            })()"# }),
        )
        .await;
    browser.navigate(&format!("http://{address}/")).await;

    // The schedule rail shows its own loading line; it never waits hidden.
    browser
        .wait_for_text("Loading the schedule…", Duration::from_secs(15))
        .await;
    browser.eval("window.__release()").await;
    browser
        .wait_for_text("3:30 PM", Duration::from_secs(10))
        .await;

    browser
        .eval("document.querySelector('button[aria-label=\"Find patient\"]').click()")
        .await;
    browser.search_for("Mary").await;
    browser.wait_for_text("DOB", Duration::from_secs(10)).await;
    browser
        .eval("document.querySelector('.dialog-results button').click()")
        .await;
    browser
        .wait_for_text("MRN 00000005", Duration::from_secs(10))
        .await;

    // A new query never shows the last query's rows or a false "No patients match."
    browser
        .eval("document.querySelector('button[aria-label=\"Find patient\"]').click()")
        .await;
    browser.eval("window.__hold = '/patient-roster'").await;
    browser.search_for("Tanvir").await;
    browser
        .wait_for_text("Searching…", Duration::from_secs(10))
        .await;
    let searching = browser.dialog_text().await;
    assert!(
        !searching.contains("Mary Louis") && !searching.contains("No patients match."),
        "a search still on its way showed another answer:\n{searching}"
    );
    browser.eval("window.__release()").await;
    browser
        .wait_for_text("Tanvir Chowdhury", Duration::from_secs(10))
        .await;

    // Switching patients never shows the last patient while the next one opens.
    browser
        .eval("window.__hold = 'pw-67305f4a-7d91-462c-9cc5-1bb30ade34ee'")
        .await;
    browser
        .eval("document.querySelector('.dialog-results button').click()")
        .await;
    let opening = browser
        .wait_for_text("Opening the patient…", Duration::from_secs(10))
        .await;
    assert!(
        !opening.contains("MRN 00000005"),
        "the next patient's page still showed the last patient:\n{opening}"
    );
    browser.eval("window.__release()").await;
    browser
        .wait_for_text("MRN 00000003", Duration::from_secs(10))
        .await;

    // A patient link opened directly shows the patient's rail loading too.
    browser
        .navigate(&format!(
            "http://{address}/?patient=pw-39e51534-6ce4-43cf-b5f8-7c1ca6252d8e"
        ))
        .await;
    browser
        .wait_for_text("Loading the schedule…", Duration::from_secs(15))
        .await;
    browser.eval("window.__release()").await;
    browser
        .wait_for_text("Follow-up visit", Duration::from_secs(10))
        .await;
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
            let target = env::temp_dir().join("coda-physician-wasm");
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

struct CdpBrowser {
    child: Child,
    user_data_dir: PathBuf,
    stream: TcpStream,
    next_id: u64,
    /// `Fetch.requestPaused` events not yet handled, oldest first.
    paused: Vec<serde_json::Value>,
}

impl CdpBrowser {
    async fn launch(chrome: &std::path::Path, initial_url: &str) -> Self {
        // One profile per browser, even with two open at once.
        static NEXT_PROFILE: std::sync::atomic::AtomicUsize =
            std::sync::atomic::AtomicUsize::new(0);
        let user_data_dir = env::temp_dir().join(format!(
            "coda-cdp-profile-{}-{}",
            std::process::id(),
            NEXT_PROFILE.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
        ));
        let _ = fs::remove_dir_all(&user_data_dir);
        fs::create_dir_all(&user_data_dir).unwrap();
        let mut child = Command::new(chrome)
            .env_remove("LD_LIBRARY_PATH")
            .args([
                "--headless=new",
                "--disable-gpu",
                "--no-sandbox",
                "--remote-debugging-port=0",
                "--window-size=1280,800",
                &format!("--user-data-dir={}", user_data_dir.display()),
                initial_url,
            ])
            .stdout(Stdio::null())
            .stderr(Stdio::piped())
            .spawn()
            .expect("spawn headless chromium");

        let stderr = child.stderr.take().expect("chromium stderr");
        let (tx, rx) = tokio::sync::oneshot::channel();
        std::thread::spawn(move || {
            use std::io::BufRead;
            let mut tx = Some(tx);
            let reader = std::io::BufReader::new(stderr);
            for line in reader.lines().map_while(Result::ok) {
                if let Some((_, url)) = line.split_once("DevTools listening on ")
                    && let Some(sender) = tx.take()
                {
                    let _ = sender.send(url.trim().to_owned());
                }
            }
        });
        let ws_browser_url = tokio::time::timeout(Duration::from_secs(15), rx)
            .await
            .expect("chromium devtools startup timeout")
            .expect("DevTools listening URL");
        let host_port = ws_browser_url
            .strip_prefix("ws://")
            .and_then(|rest| rest.split('/').next())
            .expect("ws host:port");

        let page_ws_url = Self::find_page_ws_url(host_port).await;
        let stream = Self::connect_ws(&page_ws_url).await;
        let mut browser = Self {
            child,
            user_data_dir,
            stream,
            next_id: 1,
            paused: Vec::new(),
        };
        browser.call("Page.enable", serde_json::json!({})).await;
        browser.call("Runtime.enable", serde_json::json!({})).await;
        browser
    }

    async fn find_page_ws_url(host_port: &str) -> String {
        let client = reqwest::Client::new();
        let list_url = format!("http://{host_port}/json/list");
        let started = Instant::now();
        loop {
            if let Ok(resp) = client.get(&list_url).send().await
                && let Ok(targets) = resp.json::<Vec<serde_json::Value>>().await
                && let Some(ws) = targets.iter().find_map(|target| {
                    (target.get("type").and_then(|t| t.as_str()) == Some("page"))
                        .then(|| target.get("webSocketDebuggerUrl").and_then(|u| u.as_str()))
                        .flatten()
                })
            {
                return ws.to_owned();
            }
            assert!(
                started.elapsed() <= Duration::from_secs(10),
                "timed out waiting for page target on {list_url}"
            );
            tokio::time::sleep(Duration::from_millis(50)).await;
        }
    }

    async fn connect_ws(ws_url: &str) -> TcpStream {
        let without_scheme = ws_url.strip_prefix("ws://").expect("ws:// scheme");
        let (host_port, path) = without_scheme.split_once('/').expect("ws path");
        let mut stream = TcpStream::connect(host_port).await.expect("connect cdp ws");
        let request = format!(
            "GET /{path} HTTP/1.1\r\n\
             Host: {host_port}\r\n\
             Upgrade: websocket\r\n\
             Connection: Upgrade\r\n\
             Sec-WebSocket-Key: dGhlIHNhbXBsZSBub25jZQ==\r\n\
             Sec-WebSocket-Version: 13\r\n\r\n"
        );
        stream.write_all(request.as_bytes()).await.unwrap();
        let mut header_buf = Vec::new();
        let mut byte = [0u8; 1];
        while !header_buf.ends_with(b"\r\n\r\n") {
            stream.read_exact(&mut byte).await.unwrap();
            header_buf.push(byte[0]);
        }
        let header_str = String::from_utf8_lossy(&header_buf);
        assert!(
            header_str.starts_with("HTTP/1.1 101"),
            "unexpected websocket handshake response:\n{header_str}"
        );
        stream
    }

    async fn call(&mut self, method: &str, params: serde_json::Value) -> serde_json::Value {
        let id = self.next_id;
        self.next_id += 1;
        let payload = serde_json::json!({
            "id": id,
            "method": method,
            "params": params,
        })
        .to_string();
        self.send_frame(payload.as_bytes()).await;
        loop {
            let text = self.read_frame().await;
            let Ok(message) = serde_json::from_str::<serde_json::Value>(&text) else {
                continue;
            };
            if message.get("id").and_then(|v| v.as_u64()) == Some(id) {
                return message;
            }
            if message.get("method").and_then(|m| m.as_str()) == Some("Fetch.requestPaused") {
                self.paused.push(message["params"].clone());
            }
        }
    }

    /// Lets every held draft read go on, and returns the first held request
    /// with `method`, waiting up to ten seconds for it.
    async fn held_request(&mut self, method: &str) -> serde_json::Value {
        let started = Instant::now();
        loop {
            // A cheap call drains pending events into `paused`.
            self.eval("0").await;
            let mut found = None;
            for request in std::mem::take(&mut self.paused) {
                if found.is_none() && request["request"]["method"] == method {
                    found = Some(request);
                } else if request["request"]["method"] == "GET" {
                    self.release(&request).await;
                } else {
                    self.paused.push(request);
                }
            }
            if let Some(request) = found {
                return request;
            }
            assert!(
                started.elapsed() < Duration::from_secs(10),
                "no {method} request was held"
            );
            tokio::time::sleep(Duration::from_millis(30)).await;
        }
    }

    /// Lets one held request go on to the server.
    async fn release(&mut self, request: &serde_json::Value) {
        self.call(
            "Fetch.continueRequest",
            serde_json::json!({ "requestId": request["requestId"] }),
        )
        .await;
    }

    /// Types `text` into the composer as the physician would.
    async fn type_composer(&mut self, text: &str) {
        self.eval(&format!(
            r#"new Promise(resolve => {{
            const box = document.querySelector('textarea.composer-input');
            Object.getOwnPropertyDescriptor(HTMLTextAreaElement.prototype, 'value').set.call(box, {});
            box.dispatchEvent(new Event('input', {{bubbles: true}}));
            setTimeout(() => resolve(true), 100);
        }})"#,
            serde_json::to_string(text).unwrap()
        ))
        .await;
    }

    /// The composer's text, and the mark beside it.
    async fn composer(&mut self) -> (String, String) {
        let value = self
            .eval("[document.querySelector('textarea.composer-input')?.value ?? null, document.querySelector('.composer-toolbar .composer-notice')?.textContent ?? '']")
            .await;
        let pair = &value["result"]["result"]["value"];
        (
            pair[0].as_str().unwrap_or("<no composer>").to_owned(),
            pair[1].as_str().unwrap_or_default().to_owned(),
        )
    }

    /// Waits for the autosave after typing to settle, and returns its mark:
    /// Saved or Not saved. There is no Save button to click.
    async fn settled_mark(&mut self) -> String {
        let started = Instant::now();
        loop {
            let (_, mark) = self.composer().await;
            if mark == "Saved" || mark == "Not saved" {
                return mark;
            }
            assert!(
                started.elapsed() < Duration::from_secs(10),
                "autosave settled on no mark: {mark:?}\n{}",
                self.dom().await
            );
            tokio::time::sleep(Duration::from_millis(50)).await;
        }
    }

    /// Waits until the composer shows `text`.
    async fn wait_composer(&mut self, text: &str, timeout: Duration) {
        let started = Instant::now();
        loop {
            let (shown, _) = self.composer().await;
            if shown == text {
                return;
            }
            assert!(
                started.elapsed() < timeout,
                "the composer never showed {text:?}; it shows {shown:?}"
            );
            tokio::time::sleep(Duration::from_millis(60)).await;
        }
    }

    /// The conflict the composer shows: the newer revision and its text.
    async fn conflict(&mut self) -> Option<(u64, String)> {
        let value = self
            .eval("(() => { const c = document.querySelector('.composer-conflict'); return c ? [Number(c.dataset.revision), c.querySelector('.composer-conflict-text')?.textContent ?? ''] : null; })()")
            .await;
        let pair = value.pointer("/result/result/value")?.as_array()?.clone();
        Some((pair[0].as_u64()?, pair[1].as_str()?.to_owned()))
    }

    /// Waits for the composer to show a conflict, and returns it.
    async fn wait_conflict(&mut self) -> (u64, String) {
        let started = Instant::now();
        loop {
            if let Some(conflict) = self.conflict().await {
                return conflict;
            }
            assert!(
                started.elapsed() < Duration::from_secs(10),
                "no conflict showed: {:?}",
                self.composer().await
            );
            tokio::time::sleep(Duration::from_millis(60)).await;
        }
    }

    /// Clicks the conflict's `label` button (Retry or Use newer text).
    async fn click_conflict(&mut self, label: &str) {
        let clicked = self
            .eval(&format!(
                "(() => {{ const b = [...document.querySelectorAll('.composer-conflict button')].find(b => b.textContent.trim() === {}); if (!b) return false; b.click(); return true; }})()",
                serde_json::to_string(label).unwrap()
            ))
            .await;
        assert_eq!(
            clicked.pointer("/result/result/value"),
            Some(&serde_json::json!(true)),
            "no {label} button in the conflict"
        );
    }

    /// This tab's same-tab draft recovery keys.
    async fn recovery_keys(&mut self) -> Vec<String> {
        self.eval("Object.keys(sessionStorage).filter(k => k.startsWith('coda.physician.draft.'))")
            .await
            .pointer("/result/result/value")
            .and_then(|value| value.as_array())
            .map(|keys| {
                keys.iter()
                    .filter_map(|key| key.as_str().map(str::to_owned))
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Lets every held request go on.
    async fn release_all(&mut self) {
        self.eval("0").await;
        for request in std::mem::take(&mut self.paused) {
            self.release(&request).await;
        }
    }

    /// The JSON body of a held request.
    fn held_body(request: &serde_json::Value) -> serde_json::Value {
        use base64::Engine;
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

    async fn send_frame(&mut self, payload: &[u8]) {
        let mut frame = Vec::with_capacity(14 + payload.len());
        frame.push(0x81); // FIN + text frame
        if payload.len() < 126 {
            frame.push(0x80 | (payload.len() as u8));
        } else if payload.len() <= u16::MAX as usize {
            frame.push(0x80 | 126);
            frame.extend_from_slice(&(payload.len() as u16).to_be_bytes());
        } else {
            frame.push(0x80 | 127);
            frame.extend_from_slice(&(payload.len() as u64).to_be_bytes());
        }
        // Masking key of all zeroes keeps payload bytes unchanged while satisfying RFC 6455.
        frame.extend_from_slice(&[0, 0, 0, 0]);
        frame.extend_from_slice(payload);
        self.stream.write_all(&frame).await.unwrap();
    }

    async fn read_frame(&mut self) -> String {
        loop {
            let mut head = [0u8; 2];
            self.stream.read_exact(&mut head).await.unwrap();
            let opcode = head[0] & 0x0f;
            let masked = (head[1] & 0x80) != 0;
            let mut len = (head[1] & 0x7f) as usize;
            if len == 126 {
                let mut ext = [0u8; 2];
                self.stream.read_exact(&mut ext).await.unwrap();
                len = u16::from_be_bytes(ext) as usize;
            } else if len == 127 {
                let mut ext = [0u8; 8];
                self.stream.read_exact(&mut ext).await.unwrap();
                len = u64::from_be_bytes(ext) as usize;
            }
            let mut mask = [0u8; 4];
            if masked {
                self.stream.read_exact(&mut mask).await.unwrap();
            }
            let mut data = vec![0u8; len];
            self.stream.read_exact(&mut data).await.unwrap();
            if masked {
                for (i, byte) in data.iter_mut().enumerate() {
                    *byte ^= mask[i % 4];
                }
            }
            if opcode == 0x9 {
                // Reply to ping with pong.
                let mut pong = vec![0x8a, 0x80, 0, 0, 0, 0];
                pong[1] |= data.len() as u8;
                pong.extend_from_slice(&data);
                let _ = self.stream.write_all(&pong).await;
                continue;
            }
            if opcode == 0x1 {
                return String::from_utf8_lossy(&data).into_owned();
            }
        }
    }

    async fn eval(&mut self, expression: &str) -> serde_json::Value {
        self.call(
            "Runtime.evaluate",
            serde_json::json!({
                "expression": expression,
                "returnByValue": true,
                "awaitPromise": true,
            }),
        )
        .await
    }

    async fn request_sign_in(&mut self, email: &str) {
        self.eval(&format!(r#"new Promise(resolve => {{
            const input = document.querySelector('input.sign-in-input[type="email"]');
            Object.getOwnPropertyDescriptor(HTMLInputElement.prototype, 'value').set.call(input, {});
            input.dispatchEvent(new Event('input', {{bubbles: true}}));
            setTimeout(() => {{ document.querySelector('button.sign-in-primary').click(); resolve(true); }}, 50);
        }})"#, serde_json::to_string(email).unwrap())).await;
    }

    async fn search_for(&mut self, query: &str) {
        self.eval(&format!(r#"(() => {{
            const input = document.querySelector('.search-field input');
            Object.getOwnPropertyDescriptor(HTMLInputElement.prototype, 'value').set.call(input, {});
            input.dispatchEvent(new Event('input', {{bubbles: true}}));
        }})()"#, serde_json::to_string(query).unwrap())).await;
    }

    async fn dialog_text(&mut self) -> String {
        self.eval("document.querySelector('dialog.dialog')?.innerText ?? ''")
            .await
            .pointer("/result/result/value")
            .and_then(|v| v.as_str())
            .unwrap_or_default()
            .to_owned()
    }

    async fn sign_in_again(&mut self) {
        self.request_sign_in("admin@clinic-a.test").await;
    }

    async fn dom(&mut self) -> String {
        let response = self
            .eval("document.documentElement ? document.documentElement.outerHTML : ''")
            .await;
        response
            .pointer("/result/result/value")
            .and_then(|v| v.as_str())
            .unwrap_or_default()
            .to_owned()
    }

    async fn wait_for_text(&mut self, expected: &str, timeout: Duration) -> String {
        let started = Instant::now();
        loop {
            // Read visible text and its HTML in one browser turn. Hidden text or
            // a CSS class cannot satisfy the wait, nor can we return older HTML.
            let snapshot = self
                .eval("({ text: document.body ? document.body.innerText : '', html: document.documentElement ? document.documentElement.outerHTML : '' })")
                .await;
            let value = &snapshot["result"]["result"]["value"];
            let html = value["html"].as_str().unwrap_or_default();
            if value["text"]
                .as_str()
                .is_some_and(|text| text.contains(expected))
            {
                return html.to_owned();
            }
            if started.elapsed() > timeout {
                panic!("timed out waiting for {expected:?} in DOM:\n{html}");
            }
            tokio::time::sleep(Duration::from_millis(60)).await;
        }
    }

    async fn navigate(&mut self, url: &str) {
        self.call("Page.navigate", serde_json::json!({ "url": url }))
            .await;
    }

    async fn reload(&mut self) {
        let previous = self.document_loader().await;
        self.call("Page.reload", serde_json::json!({})).await;
        let started = Instant::now();
        // The same conversation text is visible before and after a reload.
        // Wait for a different document before text waits or clicks can run;
        // otherwise they can act on the page the browser is about to discard.
        while self.document_loader().await == previous {
            assert!(
                started.elapsed() < Duration::from_secs(10),
                "reload did not replace the document"
            );
            tokio::time::sleep(Duration::from_millis(20)).await;
        }
    }

    async fn document_loader(&mut self) -> String {
        self.call("Page.getFrameTree", serde_json::json!({}))
            .await
            .pointer("/result/frameTree/frame/loaderId")
            .and_then(|id| id.as_str())
            .expect("main frame document loader")
            .to_owned()
    }

    async fn set_viewport(&mut self, width: u32, height: u32, reduced_motion: bool) {
        self.call(
            "Emulation.setDeviceMetricsOverride",
            serde_json::json!({
                "width": width,
                "height": height,
                "deviceScaleFactor": 1,
                "mobile": width < 600,
            }),
        )
        .await;
        self.call(
            "Emulation.setEmulatedMedia",
            serde_json::json!({
                "features": [{
                    "name": "prefers-reduced-motion",
                    "value": if reduced_motion { "reduce" } else { "no-preference" },
                }]
            }),
        )
        .await;
    }
}

impl Drop for CdpBrowser {
    fn drop(&mut self) {
        let _ = self.child.kill();
        let _ = self.child.wait();
        let _ = fs::remove_dir_all(&self.user_data_dir);
    }
}
