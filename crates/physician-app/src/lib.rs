//! Physician shell: the shipping sidebar, Today, a patient page, and the reader.
//! The React app remains the shipping client. This client renders the same projections.
//! Book: `book/src/physician-site.md`.
#![deny(missing_docs)]

mod icons;
mod panels;
mod shell;
mod tabs;

use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

use dioxus::prelude::*;
use icons::{Icon, IconName};
use physician_client::PhysicianClient;
use physician_wire::{
    PatientWorkspaceResponse, PracticeAccountSummary, RecentConversation, SignInResponse,
    SimulationStatus, WhoamiResponse, WorkspaceReply,
};

use panels::Panel;
#[cfg(target_arch = "wasm32")]
use shell::workspace_event_rereads;
use shell::{
    CONVERSATION_EMPTY, ChatView, Inline, KeyInput, MessageBlock, NavCommand, PAST_WORK_EMPTY,
    REPLY_ASK, REPLY_CLOSED_NOTICE, REPLY_COPY, REPLY_NO_NEEDED, REPLY_TELL_CODA, ReadView,
    ReplyReviewAction, SourceChip, SynopsisPiece, TODAY_LOADING, TODAY_QUIET, TODAY_QUIET_DETAIL,
    TODAY_STALE, TODAY_UNAVAILABLE, TRY_AGAIN, TodayLine, TodayView, VisitLine, WORKSPACE_CLOSED,
    WORKSPACE_EMPTY, WORKSPACE_UNAVAILABLE, answer_to, chat_view, chip_name, citation_inspection,
    connection_lines, demographic_card_view, format_calendar_date, format_clock, format_message,
    format_waited, keyboard_command, need_you_words, opens_reader, past_work_copy,
    patient_upcoming_visits, read_view, reply_channel_label, reply_review_actions,
    reply_urgency_words, schedule_empty_copy, shows_today_schedule, summary_times, synopsis_pieces,
    today_lines, today_view, upcoming_visits, work_end_reason, work_heading,
};
use ui::{CommandNotice, CommandStage, EmptyState, ErrorState, ReadNotice, TextButton};

const SHELL_STYLES: &str = include_str!("../assets/shell.css");
const UPDATES_LOADING: &str = "Loading updates…";
const UPDATES_EMPTY: &str = "Nothing from Coda yet today.";
const UPDATES_EMPTY_DETAIL: &str = "What Coda does for your patients today will appear here.";
const UPDATES_UNAVAILABLE: &str = "Coda couldn’t load Updates.";
const UPDATES_STALE: &str = "Coda couldn’t refresh Updates. Showing the last list.";
const SCHEDULE_LOADING: &str = "Loading the schedule…";
const SCHEDULE_UNAVAILABLE: &str = "Coda couldn’t load the schedule.";
const SCHEDULE_STALE: &str = "Coda couldn’t refresh the schedule. Showing the last one.";
const SEARCH_LOADING: &str = "Searching…";
const SEARCH_UNAVAILABLE: &str = "Coda couldn’t search patients.";
const SEARCH_NONE: &str = "No patients match.";
const CONVERSATION_LOADING: &str = "Opening the conversation…";
const PATIENT_LOADING: &str = "Opening the patient…";
const PATIENT_UNAVAILABLE: &str = "Coda couldn’t open this patient.";
const PATIENT_STALE: &str = "Coda couldn’t refresh this patient. Showing the last view.";
const PATIENT_GONE: &str = "That patient is not available.";
const GO_TO_TODAY: &str = "Go to Today";
const NEEDS_YOU_NOW: &str = "Needs you now";
const RECENT_NONE: &str = "Conversations with Coda will appear here.";
const RECENT_UNAVAILABLE: &str = "Coda couldn’t load recent conversations.";
const NO_CONVERSATIONS: &str = "No conversations yet.";
const NOT_SAVED: &str = "Not saved";
const CONVERSATION_UNAVAILABLE: &str = "Coda couldn’t open this conversation.";
const DICTATION_UNAVAILABLE: &str =
    "Dictation is unavailable. Check microphone support and server configuration.";
const CONNECTIONS: &str = "Connections";
const SIGN_IN_UNAVAILABLE: &str =
    "Coda couldn’t confirm the request. Check your inbox before trying again.";
const SIGN_IN_INVALID: &str = "That sign-in link is no longer valid. Request a new one.";
const NO_ACCESS: &str = "This sign-in link no longer gives you access to this practice.";
const SESSION_ENDED: &str = "Your session ended. Coda kept your place.";
/// The server's indistinguishable sign-in body. Proof mode uses it when the
/// address has no account. Hosted mode uses it after mailing, and when the
/// send is missing or fails, and never returns the code.
const LINK_ON_THE_WAY: &str = "If that address has an account, a sign-in link is on its way.";

#[derive(Clone, Copy, PartialEq, Eq)]
enum Place {
    Today,
    Updates,
    Workspace,
}

#[cfg(target_arch = "wasm32")]
const SESSION_STORAGE_KEY: &str = "coda.physician.session";

#[cfg(target_arch = "wasm32")]
fn read_stored_session() -> Option<String> {
    use wasm_bindgen::JsCast;
    let window = web_sys::window()?;
    let storage = js_sys::Reflect::get(&window, &"sessionStorage".into()).ok()?;
    if storage.is_null() || storage.is_undefined() {
        return None;
    }
    let get_item = js_sys::Reflect::get(&storage, &"getItem".into())
        .ok()?
        .dyn_into::<js_sys::Function>()
        .ok()?;
    get_item
        .call1(&storage, &SESSION_STORAGE_KEY.into())
        .ok()?
        .as_string()
        .filter(|value| !value.trim().is_empty())
}

#[cfg(target_arch = "wasm32")]
fn write_stored_session(session: Option<&str>) {
    use wasm_bindgen::JsCast;
    let Some(window) = web_sys::window() else {
        return;
    };
    let Ok(storage) = js_sys::Reflect::get(&window, &"sessionStorage".into()) else {
        return;
    };
    if storage.is_null() || storage.is_undefined() {
        return;
    }
    match session {
        Some(token) if !token.trim().is_empty() => {
            if let Ok(set_item) = js_sys::Reflect::get(&storage, &"setItem".into())
                && let Ok(func) = set_item.dyn_into::<js_sys::Function>()
            {
                let _ = func.call2(&storage, &SESSION_STORAGE_KEY.into(), &token.into());
            }
        }
        _ => {
            if let Ok(remove_item) = js_sys::Reflect::get(&storage, &"removeItem".into())
                && let Ok(func) = remove_item.dyn_into::<js_sys::Function>()
            {
                let _ = func.call1(&storage, &SESSION_STORAGE_KEY.into());
            }
        }
    }
}

// Count consecutive failed/short streams, not successful Cloud Run rotations.
// A valid SSE response sustained for at least 30 seconds restores all five
// retries. Even a healthy rotation waits one second; immediate EOF cannot spin.
#[cfg(any(target_arch = "wasm32", test))]
#[derive(Default)]
struct StreamRetryBudget {
    failures: u32,
}

#[cfg(any(target_arch = "wasm32", test))]
impl StreamRetryBudget {
    // None denotes a failed connection/HTTP response, regardless of how long
    // that request took. Some(elapsed_ms) measures only an accepted SSE body.
    fn next_delay_ms(&mut self, elapsed_ms: Option<f64>) -> Option<i32> {
        if elapsed_ms.is_some_and(|elapsed| elapsed >= 30_000.0) {
            self.failures = 0;
            return Some(1_000);
        }
        self.failures = self.failures.saturating_add(1);
        (self.failures <= 5).then(|| 1_000 << (self.failures - 1))
    }
}

#[cfg(test)]
mod stream_retry_tests {
    use super::StreamRetryBudget;

    #[test]
    fn immediate_eof_and_failures_back_off_and_exhaust_the_budget() {
        let mut retry = StreamRetryBudget::default();
        for (elapsed, expected) in [
            (Some(0.0), Some(1_000)),
            (None, Some(2_000)),
            (Some(29_999.0), Some(4_000)),
            (None, Some(8_000)),
            (Some(0.0), Some(16_000)),
            (None, None),
            (Some(0.0), None),
        ] {
            assert_eq!(retry.next_delay_ms(elapsed), expected);
        }
    }

    #[test]
    fn sustained_stream_resets_both_backoff_and_entire_retry_budget() {
        let mut retry = StreamRetryBudget::default();
        for _ in 0..5 {
            assert!(retry.next_delay_ms(None).is_some());
        }
        assert_eq!(retry.next_delay_ms(Some(30_000.0)), Some(1_000));
        for delay in [1_000, 2_000, 4_000, 8_000, 16_000] {
            assert_eq!(retry.next_delay_ms(Some(0.0)), Some(delay));
        }
        assert_eq!(retry.next_delay_ms(None), None);
    }

    #[test]
    fn cloud_run_rotations_do_not_exhaust_the_budget() {
        let mut retry = StreamRetryBudget::default();
        // Simulate a full day of five-minute responses without browser waits.
        for _ in 0..288 {
            assert_eq!(retry.next_delay_ms(Some(300_000.0)), Some(1_000));
        }
        assert_eq!(retry.next_delay_ms(Some(0.0)), Some(1_000));
        assert_eq!(retry.next_delay_ms(None), Some(2_000));
    }
}

#[cfg(target_arch = "wasm32")]
fn follow_workspace_events(
    base_url: &str,
    session: Option<String>,
    mut today_attempt: Signal<u32>,
    mut workspace_attempt: Signal<u32>,
    mut snapshot_attempt: Signal<u32>,
    selected: Signal<Option<String>>,
) {
    let url = format!(
        "{}/patient-workspaces/events?after=0",
        base_url.trim_end_matches('/')
    );
    use wasm_bindgen::JsCast;
    // Dioxus owns this task: leaving the signed-in shell cancels it. Aborting
    // the fetch on drop also releases a pending stream on logout/unmount.
    spawn(async move {
        let Some(window) = web_sys::window() else {
            return;
        };
        let Ok(controller) = web_sys::AbortController::new() else {
            return;
        };
        struct AbortOnDrop(web_sys::AbortController);
        impl Drop for AbortOnDrop {
            fn drop(&mut self) {
                self.0.abort();
            }
        }
        let _abort = AbortOnDrop(controller.clone());
        let Some(clock) = window.performance() else {
            return;
        };
        let mut retry = StreamRetryBudget::default();
        let mut next_delay = Some(0);
        let mut reconnecting = false;
        while let Some(delay) = next_delay {
            if delay > 0 {
                let wait = js_sys::Promise::new(&mut |resolve, reject| {
                    if let Err(error) = window
                        .set_timeout_with_callback_and_timeout_and_arguments_0(&resolve, delay)
                    {
                        let _ = reject.call1(&wasm_bindgen::JsValue::NULL, &error);
                    }
                });
                if wasm_bindgen_futures::JsFuture::from(wait).await.is_err() {
                    return;
                }
            }
            let init = web_sys::RequestInit::new();
            init.set_signal(Some(&controller.signal()));
            init.set_method("GET");
            if let Some(token) = session.as_deref().filter(|token| !token.is_empty())
                && let Ok(headers) = web_sys::Headers::new()
            {
                let _ = headers.set("Authorization", &format!("Bearer {token}"));
                init.set_headers(&headers);
            }
            let Ok(value) =
                wasm_bindgen_futures::JsFuture::from(window.fetch_with_str_and_init(&url, &init))
                    .await
            else {
                next_delay = retry.next_delay_ms(None);
                reconnecting = true;
                continue;
            };
            let Ok(response) = value.dyn_into::<web_sys::Response>() else {
                return;
            };
            if !response.ok() {
                // Never retry an authorization refusal. Re-read through the owner
                // so a revoked session returns to the sign-in screen.
                today_attempt.set(today_attempt() + 1);
                workspace_attempt.set(workspace_attempt() + 1);
                if response.status() == 429 || response.status() >= 500 {
                    next_delay = retry.next_delay_ms(None);
                    reconnecting = true;
                    continue;
                }
                return;
            }
            let event_stream = response
                .headers()
                .get("content-type")
                .ok()
                .flatten()
                .is_some_and(|kind| kind.contains("text/event-stream"));
            if !event_stream {
                return;
            }
            let Some(body) = response.body() else {
                return;
            };
            if reconnecting {
                // These events invalidate projections; they are not a second
                // ledger. Reconcile the gap via authoritative reads, never by
                // replaying commands or assuming every server supports cursors.
                today_attempt.set(today_attempt() + 1);
                workspace_attempt.set(workspace_attempt() + 1);
                snapshot_attempt.set(snapshot_attempt() + 1);
            }
            let started_at = clock.now();
            let reader = body.get_reader();
            let mut buffer = String::new();
            loop {
                let Ok(read) = js_sys::Reflect::get(&reader, &"read".into()) else {
                    return;
                };
                let Ok(read) = read.dyn_into::<js_sys::Function>() else {
                    return;
                };
                let Ok(promise) = read.call0(&reader) else {
                    return;
                };
                let Ok(promise) = promise.dyn_into::<js_sys::Promise>() else {
                    return;
                };
                let Ok(chunk) = wasm_bindgen_futures::JsFuture::from(promise).await else {
                    today_attempt.set(today_attempt() + 1);
                    workspace_attempt.set(workspace_attempt() + 1);
                    snapshot_attempt.set(snapshot_attempt() + 1);
                    break;
                };
                let done = js_sys::Reflect::get(&chunk, &"done".into())
                    .ok()
                    .and_then(|value| value.as_bool())
                    .unwrap_or(true);
                if done {
                    today_attempt.set(today_attempt() + 1);
                    workspace_attempt.set(workspace_attempt() + 1);
                    snapshot_attempt.set(snapshot_attempt() + 1);
                    break;
                }
                let Some(bytes) = js_sys::Reflect::get(&chunk, &"value".into())
                    .ok()
                    .map(|value| js_sys::Uint8Array::new(&value).to_vec())
                else {
                    return;
                };
                buffer.push_str(&String::from_utf8_lossy(&bytes));
                apply_event_frames(
                    &mut buffer,
                    &selected,
                    &mut workspace_attempt,
                    &mut snapshot_attempt,
                );
            }
            next_delay = retry.next_delay_ms(Some(clock.now() - started_at));
            reconnecting = true;
        }
    });
}

#[cfg(target_arch = "wasm32")]
fn apply_event_frames(
    buffer: &mut String,
    selected: &Signal<Option<String>>,
    workspace_attempt: &mut Signal<u32>,
    snapshot_attempt: &mut Signal<u32>,
) {
    while let Some(at) = buffer.find("\n\n") {
        let frame: String = buffer.drain(..at + 2).collect();
        let mut name = "message".to_owned();
        let mut data = String::new();
        for line in frame.lines() {
            if let Some(value) = line.strip_prefix("event:") {
                name = value.trim().to_owned();
            } else if let Some(value) = line.strip_prefix("data:") {
                if !data.is_empty() {
                    data.push('\n');
                }
                data.push_str(value.trim_start());
            }
        }
        let open = selected.read().clone();
        if workspace_event_rereads(&name, &data, open.as_deref()) {
            workspace_attempt.set(workspace_attempt() + 1);
            snapshot_attempt.set(snapshot_attempt() + 1);
        }
    }
}

/// A mailed token that is redeemed when its link opens.
#[derive(Clone, PartialEq)]
pub struct SignInLink {
    /// The account the email was sent to.
    pub email: String,
    /// The single-use token; the server remains its only verifier.
    pub code: String,
}

/// Why the server refused a signed-in read: the `reason` beside its 401.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Refusal {
    /// The session ended or expired.
    SessionEnded,
    /// The practice removed this physician's access.
    AccessRemoved,
}

fn refusal(error: &physician_client::HttpError) -> Option<Refusal> {
    let physician_client::HttpError::Status { status: 401, body } = error else {
        return None;
    };
    let removed = serde_json::from_str::<serde_json::Value>(body)
        .ok()
        .is_some_and(|value| {
            value.get("reason").and_then(|r| r.as_str()) == Some("access_removed")
        });
    Some(if removed {
        Refusal::AccessRemoved
    } else {
        Refusal::SessionEnded
    })
}

/// The tab's sign-in state, owned by [`PhysicianRoot`] and changed only here.
#[derive(Clone, Copy, PartialEq)]
struct SessionState {
    signed_in: Signal<bool>,
    /// The sign-in prompt covers the open page (physician.mmd: Session expired,
    /// sign in to continue). Nothing underneath changes.
    expired: Signal<bool>,
    whoami_name: Signal<String>,
    whoami: Signal<Option<WhoamiResponse>>,
    /// Bumped when another physician signs in here: the shell starts over at Today.
    generation: Signal<u32>,
}

impl SessionState {
    /// Back to the sign-in page.
    fn sign_out(mut self) {
        #[cfg(target_arch = "wasm32")]
        write_stored_session(None);
        self.expired.set(false);
        self.signed_in.set(false);
        self.whoami_name.set(String::new());
        self.whoami.set(None);
    }

    /// A refused read. An ended session covers a page this tab already showed
    /// with the sign-in prompt; a tab that showed nothing yet was reopened after
    /// the session ended, so it shows the sign-in page, as removed access does.
    fn refused(mut self, refusal: Refusal, shown: bool) {
        match refusal {
            Refusal::SessionEnded if shown => {
                #[cfg(target_arch = "wasm32")]
                write_stored_session(None);
                self.expired.set(true);
            }
            _ => self.sign_out(),
        }
    }

    /// Another tab's sign-in news.
    #[cfg(target_arch = "wasm32")]
    fn hear(mut self, news: tabs::TabNews, client: PhysicianClient) {
        if !*self.signed_in.peek() {
            // A signed-out tab picks up this browser's sign-in, as sign-out
            // reaches every tab. It keeps what it was opening, so the tab
            // that opened a Coda link lands on its patient when the emailed
            // link is opened elsewhere (sign-in.mmd, Patient link).
            if let tabs::TabNews::SignedIn { session, .. } = news {
                client.session().set(session.clone());
                write_stored_session(Some(&session));
                self.signed_in.set(true);
            }
            return;
        }
        match news {
            tabs::TabNews::SignedOut => {
                spawn(async move {
                    let _ = client.sign_out().await;
                    self.sign_out();
                });
            }
            tabs::TabNews::SignedIn {
                physician_id,
                session,
            } => {
                let mine = self
                    .whoami
                    .peek()
                    .as_ref()
                    .map(|account| account.physician_id.clone());
                if mine.as_deref() == Some(physician_id.as_str()) {
                    // Your own link: a covered page picks up where it was;
                    // an open one changes nothing.
                    if *self.expired.peek() {
                        client.session().set(session.clone());
                        write_stored_session(Some(&session));
                        self.expired.set(false);
                    }
                    return;
                }
                if mine.is_none() {
                    return;
                }
                // Another physician's link: this session ends and theirs
                // starts at Today; nothing of yours carries over.
                spawn(async move {
                    let _ = client.sign_out().await;
                    client.session().set(session.clone());
                    write_stored_session(Some(&session));
                    self.whoami_name.set(String::new());
                    self.whoami.set(None);
                    self.expired.set(false);
                    *self.generation.write() += 1;
                });
            }
        }
    }
}

/// Signing out ends this session and reaches this browser's other Coda tabs.
fn sign_out_everywhere(client: PhysicianClient, session: SessionState) {
    spawn(async move {
        let _ = client.sign_out().await;
        #[cfg(target_arch = "wasm32")]
        tabs::announce(&tabs::TabNews::SignedOut);
        session.sign_out();
    });
}

#[component]
pub fn PhysicianRoot(
    base_url: String,
    open_patient: String,
    sign_in: Option<SignInLink>,
) -> Element {
    // A mailed link is redeemed even when this browser already has a session.
    // Otherwise the first `/today` read handles the auth gate.
    let mut signed_in = use_signal(|| sign_in.is_none());
    let mut whoami_name = use_signal(String::new);
    let mut whoami = use_signal(|| None::<WhoamiResponse>);
    let expired = use_signal(|| false);
    let generation = use_signal(|| 0u32);
    let session = SessionState {
        signed_in,
        expired,
        whoami_name,
        whoami,
        generation,
    };
    let mut pending_sign_in = use_signal(|| sign_in);
    // The bearer session rides the client, so one client is built once.
    let client = use_signal(|| {
        let client = PhysicianClient::new(base_url.clone());
        #[cfg(target_arch = "wasm32")]
        if let Some(session) = read_stored_session() {
            client.session().set(session);
        }
        client
    });
    // Other tabs' sign-in news arrives outside the Dioxus runtime, so it
    // re-enters this scope before touching the session.
    #[cfg(target_arch = "wasm32")]
    use_hook(move || {
        let runtime = dioxus::core::Runtime::current();
        let scope = dioxus::core::current_scope_id();
        std::rc::Rc::new(tabs::listen(move |news| {
            runtime.in_scope(scope, || session.hear(news, client.peek().clone()));
        }))
    });
    #[cfg(target_arch = "wasm32")]
    use_effect(move || {
        if pending_sign_in.peek().is_some() {
            return;
        }
        let client = client.read().clone();
        spawn(async move {
            if let Err(error) = client.today().await
                && refusal(&error).is_some()
            {
                session.sign_out();
            }
        });
    });
    if !signed_in() {
        rsx! {
            style { "{ui::STYLESHEET}{SHELL_STYLES}" }
            SignInScreen {
                client,
                link: pending_sign_in(),
                on_signed_in: move |who: Option<WhoamiResponse>| {
                    pending_sign_in.set(None);
                    let name = who
                        .as_ref()
                        .map(|account| account.display_name.clone())
                        .unwrap_or_default();
                    whoami_name.set(name);
                    whoami.set(who);
                    signed_in.set(true);
                },
            }
        }
    } else {
        // A different physician's sign-in starts a fresh shell at Today. A
        // keyed list replaces the shell when the key changes; a lone keyed
        // component is diffed in place and keeps its state.
        let shell = generation();
        rsx! {
            for shell in std::iter::once(shell) {
                PhysicianShell {
                    key: "{shell}",
                    base_url: base_url.clone(),
                    open_patient: if shell == 0 { open_patient.clone() } else { String::new() },
                    client,
                    session,
                }
            }
        }
    }
}

#[component]
fn PhysicianShell(
    base_url: String,
    open_patient: String,
    client: Signal<PhysicianClient>,
    session: SessionState,
) -> Element {
    let mut whoami_name = session.whoami_name;
    let mut whoami = session.whoami;
    let expired = session.expired;
    // Whether this tab has shown the practice yet; see SessionState::refused.
    let mut shown = use_signal(|| false);
    let mut place = use_signal(|| {
        if open_patient.is_empty() {
            Place::Today
        } else {
            Place::Workspace
        }
    });
    let mut selected = use_signal(|| {
        if open_patient.is_empty() {
            None
        } else {
            Some(open_patient)
        }
    });
    let mut sidebar_collapsed = use_signal(|| false);
    let mut app_collapsed = use_signal(|| false);
    let mut schedule_collapsed = use_signal(|| false);
    // The physician's remembered choices arrive with who they are.
    use_effect(move || {
        if let Some(account) = whoami.read().as_ref() {
            sidebar_collapsed.set(panels::collapsed(&account.physician_id, Panel::Sidebar));
            schedule_collapsed.set(panels::collapsed(&account.physician_id, Panel::Schedule));
        }
    });
    let mut remember = move |panel: Panel, collapsed: bool| {
        match panel {
            Panel::Sidebar => sidebar_collapsed.set(collapsed),
            Panel::Schedule => schedule_collapsed.set(collapsed),
        }
        if let Some(account) = whoami.peek().as_ref() {
            panels::remember(&account.physician_id, panel, collapsed);
        }
    };
    let mut search_open = use_signal(|| false);
    let mut shortcuts_open = use_signal(|| false);
    let mut connections_open = use_signal(|| false);
    let mut draft = use_signal(String::new);
    let mut notice = use_signal(String::new);
    let mut reading = use_signal(|| None::<OpenReading>);
    let mut focus = use_signal(|| -1i32);
    let mut open_chat = use_signal(|| None::<String>);
    let mut moved = use_signal(|| false);
    let mut editing = use_signal(|| false);
    let mut query = use_signal(String::new);
    let mut today_attempt = use_signal(|| 0u32);
    let mut workspace_attempt = use_signal(|| 0u32);
    let mut snapshot_attempt = use_signal(|| 0u32);
    let mut search_attempt = use_signal(|| 0u32);
    let mut today_cache = use_signal(|| None::<Vec<TodayLine>>);
    let mut workspace_cache = use_signal(|| None::<PatientWorkspaceResponse>);
    // The patient the server said isn't there for this physician (a 404 also
    // answers for no access, so presence never leaks).
    let mut gone = use_signal(|| None::<String>);
    let mut target_physician = use_signal(String::new);
    let mut panel_busy = use_signal(|| false);
    let mut panel_notice = use_signal(String::new);
    let mut past_work_open = use_signal(|| false);
    let mut reply_notice = use_signal(String::new);
    let reply_slot = use_command_slot();
    let send_slot = use_command_slot();
    let resume_slot = use_command_slot();
    let work_slot = use_command_slot();
    use_effect(move || {
        let open = workspace_cache.read().clone();
        let showing = place();
        let allowed =
            showing == Place::Workspace && open.as_ref().and_then(past_work_copy).is_some();
        if !allowed && past_work_open() {
            past_work_open.set(false);
        }
    });
    // Signing in again under the prompt re-reads the page in place; the stream
    // restarts on the new session (an ended one stops on its refusal).
    let mut was_expired = use_signal(|| false);
    use_effect(move || {
        let now_expired = expired();
        if *was_expired.peek() && !now_expired {
            *today_attempt.write() += 1;
            *workspace_attempt.write() += 1;
            *snapshot_attempt.write() += 1;
        }
        was_expired.set(now_expired);
    });
    #[cfg(target_arch = "wasm32")]
    use_effect(move || {
        if expired() {
            return;
        }
        let session = client.read().session().get();
        follow_workspace_events(
            &base_url,
            session,
            today_attempt,
            workspace_attempt,
            snapshot_attempt,
            selected,
        );
    });
    let _whoami_load = use_resource(move || {
        let _attempt = today_attempt();
        let client = client.read().clone();
        async move {
            if let Ok(account) = client.whoami().await {
                if target_physician.read().is_empty() {
                    target_physician.set(account.physician_id.clone());
                }
                whoami_name.set(account.display_name.clone());
                whoami.set(Some(account));
            }
        }
    });
    let health = use_resource(move || {
        let client = client.read().clone();
        async move { client.health().await.map_err(|_| ()) }
    });
    let today = use_resource(move || {
        let _attempt = today_attempt();
        let client = client.read().clone();
        async move {
            match client.today().await {
                Ok(response) => {
                    shown.set(true);
                    Ok(response)
                }
                Err(error) => {
                    if let Some(refused) = refusal(&error) {
                        session.refused(refused, *shown.peek());
                    }
                    Err(())
                }
            }
        }
    });
    let patients = use_resource(move || {
        let _attempt = today_attempt();
        let client = client.read().clone();
        async move { client.workspaces().await.map_err(|_| ()) }
    });
    let practice = use_resource(move || {
        let _attempt = today_attempt();
        let client = client.read().clone();
        async move { client.practice_day().await.map_err(|_| ()) }
    });
    let recent = use_resource(move || {
        let _attempt = today_attempt();
        let client = client.read().clone();
        async move { client.recent_conversations().await.map_err(|_| ()) }
    });
    let updates = use_resource(move || {
        let _attempt = today_attempt();
        let client = client.read().clone();
        async move { client.updates().await.map(|page| page.rows).map_err(|_| ()) }
    });
    let found = use_resource(move || {
        let _attempt = today_attempt();
        let _search_attempt = search_attempt();
        let query = query();
        let client = client.read().clone();
        async move {
            let answer = if query.trim().is_empty() {
                Ok(Vec::new())
            } else {
                client
                    .search(&query)
                    .await
                    .map(|page| page.patients)
                    .map_err(|_| ())
            };
            (query, answer)
        }
    });
    let patient_today = use_resource(move || {
        let _today_attempt = today_attempt();
        let _workspace_attempt = workspace_attempt();
        let client = client.read().clone();
        let open_id = selected();
        async move {
            let answer = match &open_id {
                Some(id) => client.today_for(id).await.map(Some).map_err(|_| ()),
                None => Ok(None),
            };
            (open_id, answer)
        }
    });
    let workspace = use_resource(move || {
        let _attempt = workspace_attempt();
        let client = client.read().clone();
        let open_id = selected();
        async move {
            let answer = match open_id.clone() {
                Some(id) => match client.workspace(&id).await {
                    Ok(open) => {
                        shown.set(true);
                        gone.set(None);
                        Ok(Some(open))
                    }
                    // A covered page keeps what it showed; signing out leaves
                    // the shell, so nothing of it stays either way.
                    Err(error) => {
                        if let Some(refused) = refusal(&error) {
                            session.refused(refused, *shown.peek());
                        } else if matches!(
                            error,
                            physician_client::HttpError::Status { status: 404, .. }
                        ) {
                            workspace_cache.set(None);
                            gone.set(Some(id));
                        }
                        Err(())
                    }
                },
                None => Ok(None),
            };
            (open_id, answer)
        }
    });
    use_effect(move || {
        if let Some(Ok(list)) = today.read().as_ref() {
            today_cache.set(Some(today_lines(list)));
        }
    });
    use_effect(move || {
        if let Some(Ok(Some(open))) = answer_to(&*selected.read(), workspace.read().as_ref()) {
            workspace_cache.set(Some(open.clone()));
        }
    });
    use_effect(move || {
        let current = selected();
        let cached = workspace_cache.read().clone();
        let same = cached
            .as_ref()
            .is_some_and(|open| Some(open.patient_workspace_id.as_str()) == current.as_deref());
        if !same {
            if open_chat.read().is_some() {
                open_chat.set(None);
            }
            return;
        }
        if open_chat.read().is_none()
            && let Some(chat_id) =
                cached.and_then(|open| open.chats.first().map(|chat| chat.chat_id.clone()))
        {
            open_chat.set(Some(chat_id));
        }
    });
    let today_read = today.read();
    let today_cache_read = today_cache.read();
    let today_result = today_read
        .as_ref()
        .map(|result| result.as_ref().map_err(|_| ()));
    let today_state = today_view(today_cache_read.as_deref(), today_result);
    let connection_accounts = health
        .read()
        .as_ref()
        .and_then(|result| result.as_ref().ok())
        .and_then(|health| health.capabilities.as_ref())
        .is_some_and(|capabilities| capabilities.connection_accounts);
    let connections = today_read
        .as_ref()
        .and_then(|result| result.as_ref().ok())
        .map(|list| connection_lines(&list.connections))
        .unwrap_or_default();
    let simulation_status = today_read
        .as_ref()
        .and_then(|result| result.as_ref().ok())
        .and_then(|list| list.simulation.clone());
    let practice_state = use_read_view(practice);
    let updates_state = use_read_view(updates);
    let schedule = practice_state.stage();
    let day = practice_state.content();
    let server_now = day.map(|day| day.current_time.clone()).unwrap_or_default();
    let practice_zone = day
        .map(|day| day.time_zone.clone())
        .unwrap_or_else(|| "America/New_York".to_owned());
    let visits = day.map(upcoming_visits).unwrap_or_default();
    // The rail shows while the day loads; only an answered day can say there is no calendar.
    let show_schedule = matches!(
        schedule,
        ui::ReadStage::Loading | ui::ReadStage::Unavailable
    ) || day.is_some_and(shows_today_schedule);
    let schedule_empty = day.map(schedule_empty_copy).unwrap_or("").to_owned();
    let need_you = today_state.content().map_or(0, Vec::len);
    let cached_workspace = workspace_cache.read().clone();
    let workspace_read = workspace.read();
    let workspace_answer = answer_to(&*selected.read(), workspace_read.as_ref());
    let workspace_current = match workspace_answer {
        Some(Ok(Some(open))) => Some(Ok(open)),
        Some(Ok(None)) => None,
        Some(Err(())) => cached_workspace
            .as_ref()
            .filter(|open| selected.read().as_deref() == Some(open.patient_workspace_id.as_str()))
            .map(Ok)
            .or(Some(Err(()))),
        None => cached_workspace
            .as_ref()
            .filter(|open| selected.read().as_deref() == Some(open.patient_workspace_id.as_str()))
            .map(Ok),
    };
    let patient_gone = selected.read().is_some() && *gone.read() == *selected.read();
    let patient_read = match (workspace_answer, &workspace_current) {
        (Some(Err(())), Some(Ok(_))) => ui::ReadStage::Stale,
        (_, Some(Ok(_))) => ui::ReadStage::Showing,
        (_, Some(Err(()))) => ui::ReadStage::Unavailable,
        (_, None) => ui::ReadStage::Loading,
    };
    let chats = chat_view(
        selected.read().as_deref(),
        workspace_current.as_ref().map(|result| match result {
            Ok(open) => Ok(*open),
            Err(_) => Err(()),
        }),
    );
    let patient_list = patients
        .read()
        .as_ref()
        .and_then(|result| result.as_ref().ok())
        .map(|list| list.patients.clone())
        .unwrap_or_default();
    let open_name = cached_workspace
        .as_ref()
        .filter(|open| selected.read().as_deref() == Some(open.patient_workspace_id.as_str()))
        .map(|open| open.display_name.clone())
        .or_else(|| {
            patient_list
                .iter()
                .find(|patient| {
                    Some(patient.patient_workspace_id.as_str()) == selected.read().as_deref()
                })
                .map(|patient| patient.display_name.clone())
        });
    let open_workspace = cached_workspace
        .as_ref()
        .filter(|open| selected.read().as_deref() == Some(open.patient_workspace_id.as_str()))
        .cloned();
    let patient_rows = answer_to(&*selected.read(), patient_today.read().as_ref())
        .and_then(|result| result.as_ref().ok())
        .and_then(|list| list.as_ref())
        .map(today_lines)
        .unwrap_or_default();
    let patient_visits = day
        .and_then(|day| {
            selected
                .read()
                .as_deref()
                .map(|id| patient_upcoming_visits(day, id))
        })
        .unwrap_or_default();
    let recent_read = recent.read();
    // A search answers one query, so an earlier query's rows never stand in for it.
    let search_state = read_view(None, answer_to(&query(), found.read().as_ref()).cloned());
    let search_rows = search_state.content().cloned().unwrap_or_default();
    let modal =
        shortcuts_open() || connections_open() || past_work_open() || reading.read().is_some();
    let shown_focus = if moved() { Some(focus()) } else { None };
    let snapshot = use_resource(move || {
        let _attempt = snapshot_attempt();
        let asked = (selected(), open_chat());
        let client = client.read().clone();
        async move {
            let answer = match &asked {
                (Some(workspace_id), Some(chat_id)) => client
                    .chat_snapshot(workspace_id, chat_id)
                    .await
                    .map(|page| {
                        page.snapshot
                            .conversation
                            .iter()
                            .filter_map(|turn| turn.body().map(str::to_owned))
                            .collect::<Vec<_>>()
                    })
                    .map_err(|_| ()),
                _ => Ok(Vec::new()),
            };
            (asked, answer)
        }
    });
    // One conversation's lines never stand in for another's.
    let conversation = read_view(
        None,
        answer_to(&(selected(), open_chat()), snapshot.read().as_ref()).cloned(),
    );
    let shell_class = format!(
        "glass-app{}{}",
        if app_collapsed() { " is-collapsed" } else { "" },
        if sidebar_collapsed() {
            " has-collapsed-sidebar"
        } else {
            ""
        }
    );
    rsx! {
        style { "{ui::STYLESHEET}{SHELL_STYLES}" }
        if app_collapsed() {
            button {
                class: "coda-restore",
                r#type: "button",
                aria_label: "Open Coda",
                onclick: move |_| app_collapsed.set(false),
                img { class: "coda-logo", src: "./assets/coda-health-logo-transparent.png", alt: "" }
            }
        }
        if expired() {
            // Covers and hides the page; nothing underneath changes.
            div {
                class: "session-prompt",
                role: "dialog",
                aria_modal: "true",
                aria_label: "Sign in to continue",
                SignInScreen {
                    client,
                    link: None,
                    resume_email: whoami.read().as_ref().map(|account| account.email.clone()),
                    on_signed_in: move |_| {
                        let mut expired = expired;
                        expired.set(false);
                    },
                    on_sign_out: move |_| sign_out_everywhere(client.read().clone(), session),
                }
            }
        }
        div {
            class: "{shell_class}",
            aria_hidden: if app_collapsed() || expired() { "true" } else { "false" },
            "inert": expired().then_some("true"),
            tabindex: "0",
            onkeydown: move |event| {
                let key = event.key().to_string();
                if key == "Escape" {
                    event.prevent_default();
                    if search_open() {
                        search_open.set(false);
                        editing.set(false);
                    } else if shortcuts_open() {
                        shortcuts_open.set(false);
                    } else if connections_open() {
                        connections_open.set(false);
                    } else if past_work_open() {
                        past_work_open.set(false);
                    } else if reading.read().is_some() {
                        reading.set(None);
                    }
                    return;
                }
                if key == " " && event.modifiers().ctrl() && event.modifiers().shift() {
                    event.prevent_default();
                    let client = client.read().clone();
                    spawn(async move {
                        let _ = client.dictate().await;
                        notice.set(DICTATION_UNAVAILABLE.to_owned());
                    });
                    return;
                }
                if key == "Enter" && (event.modifiers().meta() || event.modifiers().ctrl()) {
                    event.prevent_default();
                    send_slot.run_or_refuse(
                        client.read().clone(),
                        message_command(
                            selected(),
                            open_chat(),
                            workspace_cache
                                .read()
                                .as_ref()
                                .and_then(|open| open.time_zone.clone()),
                            draft(),
                        ),
                        move || *snapshot_attempt.write() += 1,
                    );
                    return;
                }
                let rail = if place() == Place::Workspace {
                    patient_visits.clone()
                } else {
                    visits.clone()
                };
                let targets = focus_targets(
                    search_open(),
                    place(),
                    &today_state,
                    &rail,
                    &search_rows,
                );
                let Some(command) = keyboard_command(&KeyInput {
                    key,
                    meta: event.modifiers().meta(),
                    ctrl: event.modifiers().ctrl(),
                    alt: event.modifiers().alt(),
                    shift: event.modifiers().shift(),
                    editing: editing(),
                    modal,
                    item_focused: moved(),
                }) else {
                    return;
                };
                event.prevent_default();
                match command {
                    NavCommand::Search => search_open.set(true),
                    NavCommand::Shortcuts => shortcuts_open.set(true),
                    NavCommand::Next => {
                        moved.set(true);
                        let last = targets.len().saturating_sub(1) as i32;
                        let next = if focus() < 0 { 0 } else { focus() + 1 };
                        focus.set(next.min(last));
                    }
                    NavCommand::Previous => {
                        moved.set(true);
                        let last = targets.len().saturating_sub(1) as i32;
                        let next = if focus() < 0 { last } else { focus() - 1 };
                        focus.set(next.max(0));
                    }
                    NavCommand::Activate => {
                        let Some(target) = targets.get(focus().max(0) as usize) else { return };
                        open_target(target.clone(), &mut selected, &mut place, &mut search_open, &mut reading, &mut workspace_attempt);
                    }
                }
            },
            aside { class: "glass-sidebar",
                div { class: "sidebar-content",
                    div { class: "sidebar-brand-row",
                        button {
                            class: "sidebar-brand",
                            r#type: "button",
                            aria_label: "Coda Health",
                            onclick: move |_| place.set(Place::Today),
                            img { class: "coda-logo", src: "./assets/coda-health-logo-transparent.png", alt: "Coda Health" }
                        }
                        button {
                            class: "icon-button sidebar-collapse",
                            r#type: "button",
                            aria_label: "Collapse navigation",
                            onclick: move |_| remember(Panel::Sidebar, true),
                            Icon { name: IconName::SidebarCollapse }
                        }
                        button {
                            class: "icon-button sidebar-expand",
                            r#type: "button",
                            aria_label: "Expand navigation",
                            onclick: move |_| remember(Panel::Sidebar, false),
                            Icon { name: IconName::SidebarExpand }
                        }
                    }
                    nav { class: "sidebar-primary", aria_label: "Primary",
                        button {
                            class: "sidebar-nav-item",
                            r#type: "button",
                            aria_label: "Open Today",
                            aria_current: if place() == Place::Today { "page" } else { "" },
                            onclick: move |_| {
                                place.set(Place::Today);
                            },
                            if place() == Place::Today {
                                Icon { name: IconName::Home }
                            } else {
                                Icon { name: IconName::ArrowLeft }
                            }
                            strong { class: "item-text", "Today" }
                        }
                        button {
                            class: "sidebar-nav-item",
                            r#type: "button",
                            aria_current: if place() == Place::Updates { "page" } else { "" },
                            onclick: move |_| place.set(Place::Updates),
                            Icon { name: IconName::Message }
                            strong { class: "item-text", "Updates" }
                        }
                        button {
                            class: "sidebar-nav-item",
                            r#type: "button",
                            aria_label: "Find patient",
                            onclick: move |_| search_open.set(true),
                            Icon { name: IconName::Search }
                            strong { class: "item-text", "Find patient" }
                            kbd { class: "key-hint", aria_hidden: "true", "⌘ K" }
                        }
                    }
                    div { class: "sidebar-rule" }
                    if place() == Place::Workspace {
                        section { class: "sidebar-patient-context", aria_label: "Patient navigation",
                            div { class: "sidebar-current-patient",
                                "{open_name.clone().unwrap_or_default()}"
                                Icon { name: IconName::ArrowRight }
                            }
                            if open_workspace.as_ref().is_some_and(PatientWorkspaceResponse::is_clinic_demographic) {
                                div { class: "sidebar-section-heading", h2 { "Clinic record" } }
                                p { class: "sidebar-empty", "Not on your panel" }
                            } else {
                                div { class: "sidebar-section-heading", h2 { "Conversations" } }
                                match chats.clone() {
                                    ChatView::Empty | ChatView::Closed => rsx! { p { class: "sidebar-empty", "{NO_CONVERSATIONS}" } },
                                    ChatView::Pending => rsx! {},
                                    ChatView::Unavailable if patient_gone => rsx! {},
                                    ChatView::Unavailable => rsx! { p { class: "sidebar-loading", "{WORKSPACE_UNAVAILABLE}" } },
                                    ChatView::Chats(lines) => rsx! {
                                        for line in lines {
                                            button {
                                                class: "sidebar-conversation-link",
                                                r#type: "button",
                                                onclick: {
                                                    let id = line.id.clone();
                                                    move |_| open_chat.set(Some(id.clone()))
                                                },
                                                Icon { name: IconName::Message }
                                                "{line.title}"
                                            }
                                        }
                                    },
                                }
                            }
                        }
                    } else {
                        section { class: "sidebar-recent", aria_labelledby: "sidebar-recent-heading",
                            div { class: "sidebar-section-heading", h2 { id: "sidebar-recent-heading", "Recent conversations" } }
                            match recent_read.as_ref() {
                                None => rsx! {},
                                Some(Err(())) => rsx! { p { class: "sidebar-loading", "{RECENT_UNAVAILABLE}" } },
                                Some(Ok(list)) if list.conversations.is_empty() => rsx! { p { class: "sidebar-empty", "{RECENT_NONE}" } },
                                Some(Ok(list)) => rsx! {
                                    for conversation in list.conversations.clone() {
                                        RecentRow { conversation }
                                    }
                                },
                            }
                        }
                    }
                    div { class: "sidebar-footer",
                        if !whoami_name().is_empty() {
                            span { class: "sidebar-physician", "{whoami_name}" }
                        }
                        if connection_accounts {
                            button {
                                r#type: "button",
                                onclick: move |_| connections_open.set(true),
                                Icon { name: IconName::Users }
                                "{CONNECTIONS}"
                            }
                        }
                        button {
                            r#type: "button",
                            onclick: move |_| shortcuts_open.set(true),
                            Icon { name: IconName::Keyboard }
                            "Keyboard shortcuts"
                        }
                        button {
                            r#type: "button",
                            aria_label: "Sign out of Coda",
                            onclick: move |_| sign_out_everywhere(client.read().clone(), session),
                            Icon { name: IconName::Users }
                            "Sign out"
                        }
                    }
                }
            }
            main { class: "glass-stage",
                button {
                    class: "stage-minimize",
                    r#type: "button",
                    onclick: move |_| app_collapsed.set(true),
                    "Collapse Coda"
                }
                match place() {
                    Place::Today => rsx! {
                        TodaySurface {
                            state: today_state.clone(),
                            simulation: simulation_status.clone(),
                            connections: connections.clone(),
                            visits: visits.clone(),
                            show_schedule,
                            schedule,
                            on_schedule_retry: move |_| *today_attempt.write() += 1,
                            schedule_empty: schedule_empty.clone(),
                            schedule_collapsed: schedule_collapsed(),
                            on_schedule_collapsed: move |collapsed| remember(Panel::Schedule, collapsed),
                            server_now: server_now.clone(),
                            need_you,
                            focused: shown_focus,
                            on_retry: move |_| today_attempt.set(today_attempt() + 1),
                            on_run_scenario: move |scenario: String| {
                                let client = client.read().clone();
                                spawn(async move {
                                    let _ = client
                                        .command(&serde_json::json!({
                                            "type": "simulation.run",
                                            "payload": { "scenario": scenario },
                                        }))
                                        .await;
                                    today_attempt.set(today_attempt() + 1);
                                    workspace_attempt.set(workspace_attempt() + 1);
                                });
                            },
                            on_open_patient: move |id: String| {
                                selected.set(Some(id));
                                place.set(Place::Workspace);
                                search_open.set(false);
                                workspace_attempt.set(workspace_attempt() + 1);
                            },
                            on_open_row: move |line: TodayLine| {
                                let reading_text = if opens_reader(&line.kind) {
                                    Some(prose_reading(line.patient.clone(), line.reason.clone()))
                                } else {
                                    None
                                };
                                reading.set(reading_text);
                                selected.set(Some(line.workspace_id));
                                place.set(Place::Workspace);
                                search_open.set(false);
                                workspace_attempt.set(workspace_attempt() + 1);
                            },
                            on_open_visit: move |visit: VisitLine| {
                                let Some(id) = visit.workspace_id.clone() else { return };
                                selected.set(Some(id));
                                place.set(Place::Workspace);
                                search_open.set(false);
                                workspace_attempt.set(workspace_attempt() + 1);
                            },
                            on_resume: move |line: TodayLine| {
                                if let Some(command) = resume_command(&line, &practice_zone) {
                                    resume_slot.run(client.read().clone(), command, move || *today_attempt.write() += 1);
                                }
                            },
                            resume_notice: resume_slot.notice(client),
                        }
                    },
                    Place::Updates => rsx! { UpdatesSurface { state: updates_state.clone(), on_retry: move |_| *today_attempt.write() += 1 } },
                    Place::Workspace => rsx! {
                        PatientSurface {
                            gone: patient_gone,
                            read: patient_read,
                            on_go_today: move |_| {
                                selected.set(None);
                                place.set(Place::Today);
                            },
                            name: open_name.clone().unwrap_or_default(),
                            workspace: open_workspace.clone(),
                            rows: patient_rows.clone(),
                            visits: patient_visits.clone(),
                            schedule_collapsed: schedule_collapsed(),
                            on_schedule_collapsed: move |collapsed| remember(Panel::Schedule, collapsed),
                            schedule,
                            on_schedule_retry: move |_| *today_attempt.write() += 1,
                            focused: shown_focus,
                            chats: chats.clone(),
                            server_now: server_now.clone(),
                            draft: draft(),
                            notice: notice(),
                            is_admin: whoami.read().as_ref().is_some_and(|account| account.admin),
                            practice_accounts: whoami
                                .read()
                                .as_ref()
                                .map(|account| account.practice_accounts.clone())
                                .unwrap_or_default(),
                            target_physician_id: {
                                let chosen = target_physician();
                                if !chosen.is_empty() {
                                    chosen
                                } else {
                                    whoami
                                        .read()
                                        .as_ref()
                                        .map(|account| account.physician_id.clone())
                                        .unwrap_or_default()
                                }
                            },
                            panel_busy: panel_busy(),
                            panel_notice: panel_notice(),
                            on_select_physician: move |value: String| target_physician.set(value),
                            on_assign_panel: move |_| {
                                let Some(patient_id) = selected() else { return };
                                let physician_id = {
                                    let chosen = target_physician();
                                    if !chosen.is_empty() {
                                        chosen
                                    } else {
                                        whoami
                                            .read()
                                            .as_ref()
                                            .map(|account| account.physician_id.clone())
                                            .unwrap_or_default()
                                    }
                                };
                                if physician_id.is_empty() {
                                    return;
                                }
                                let client = client.read().clone();
                                spawn(async move {
                                    panel_busy.set(true);
                                    panel_notice.set(String::new());
                                    match client.assign_panel(&physician_id, &patient_id, true).await {
                                        Ok(_) => {
                                            workspace_attempt.set(workspace_attempt() + 1);
                                            today_attempt.set(today_attempt() + 1);
                                        }
                                        Err(_) => {
                                            panel_notice.set("Coda couldn’t update panel assignment.".to_owned());
                                        }
                                    }
                                    panel_busy.set(false);
                                });
                            },
                            on_remove_panel: move |_| {
                                let Some(patient_id) = selected() else { return };
                                let physician_id = {
                                    let chosen = target_physician();
                                    if !chosen.is_empty() {
                                        chosen
                                    } else {
                                        whoami
                                            .read()
                                            .as_ref()
                                            .map(|account| account.physician_id.clone())
                                            .unwrap_or_default()
                                    }
                                };
                                if physician_id.is_empty() {
                                    return;
                                }
                                let client = client.read().clone();
                                spawn(async move {
                                    panel_busy.set(true);
                                    panel_notice.set(String::new());
                                    match client.remove_panel(&physician_id, &patient_id).await {
                                        Ok(_) => {
                                            workspace_attempt.set(workspace_attempt() + 1);
                                            today_attempt.set(today_attempt() + 1);
                                        }
                                        Err(_) => {
                                            panel_notice.set("Coda couldn’t update panel assignment.".to_owned());
                                        }
                                    }
                                    panel_busy.set(false);
                                });
                            },
                            on_retry: move |_| workspace_attempt.set(workspace_attempt() + 1),
                            on_decide_recommendation: move |rec_id: String| {
                                let Some(ws_id) = selected() else { return };
                                work_slot.run(
                                    client.read().clone(),
                                    Command::new(
                                        "plan.decide_recommendation",
                                        serde_json::json!({ "patient_workspace_id": ws_id, "recommendation_id": rec_id }),
                                    ),
                                    move || {
                                        *workspace_attempt.write() += 1;
                                        *today_attempt.write() += 1;
                                    },
                                );
                            },
                            on_admit_corrected_labs: move |_| {
                                let Some(ws_id) = selected() else { return };
                                work_slot.run(
                                    client.read().clone(),
                                    Command::new(
                                        "simulation.admit_corrected_labs",
                                        serde_json::json!({ "patient_workspace_id": ws_id }),
                                    ),
                                    move || {
                                        *workspace_attempt.write() += 1;
                                        *today_attempt.write() += 1;
                                    },
                                );
                            },
                            work_busy: work_slot.sending() || resume_slot.sending(),
                            work_notice: work_slot.notice(client),
                            resume_notice: resume_slot.notice(client),
                            on_resume: {
                                let zone = workspace_cache
                                    .read()
                                    .as_ref()
                                    .and_then(|open| open.time_zone.clone())
                                    .unwrap_or_else(|| practice_zone.clone());
                                move |line: TodayLine| {
                                    if let Some(command) = resume_command(&line, &zone) {
                                        resume_slot.run(client.read().clone(), command, move || {
                                            *workspace_attempt.write() += 1;
                                            *today_attempt.write() += 1;
                                        });
                                    }
                                }
                            },
                            on_read: move |(title, body): (String, String)| {
                                reading.set(Some(prose_reading(title, body)));
                            },
                            on_open_source: move |chip: SourceChip| {
                                reading.set(Some(source_reading(chip)));
                            },
                            on_draft: move |value: String| draft.set(value),
                            conversation: conversation.clone(),
                            on_conversation_retry: move |_| *snapshot_attempt.write() += 1,
                            on_open_chat: move |id: String| open_chat.set(Some(id)),
                            on_save: move |_| {
                                let Some(id) = selected() else { return };
                                let client = client.read().clone();
                                let text = draft();
                                let chat_id = open_chat();
                                spawn(async move {
                                    let saved = if let Some(chat_id) = chat_id {
                                        match client.read_draft(&id, &chat_id).await {
                                            Ok(existing) => {
                                                client
                                                    .save_draft(&id, &chat_id, existing.revision, &text)
                                                    .await
                                            }
                                            Err(error) => Err(error),
                                        }
                                    } else {
                                        client.save_draft(&id, "draft", 0, &text).await
                                    };
                                    notice.set(match saved {
                                        Ok(()) => String::new(),
                                        Err(_) => NOT_SAVED.to_owned(),
                                    });
                                });
                            },
                            on_send: move |_| {
                                send_slot.run_or_refuse(
                                    client.read().clone(),
                                    message_command(
                                        selected(),
                                        open_chat(),
                                        workspace_cache
                                            .read()
                                            .as_ref()
                                            .and_then(|open| open.time_zone.clone()),
                                        draft(),
                                    ),
                                    move || *snapshot_attempt.write() += 1,
                                );
                            },
                            on_dictate: move |_| {
                                let client = client.read().clone();
                                spawn(async move {
                                    let _ = client.dictate().await;
                                    notice.set(DICTATION_UNAVAILABLE.to_owned());
                                });
                            },
                            on_editing: move |value: bool| editing.set(value),
                            conversation_open: open_chat.read().is_some(),
                            past_work: open_workspace.as_ref().is_some_and(|open| past_work_copy(open).is_some()),
                            reply_notice: reply_notice(),
                            reply_busy: reply_slot.sending(),
                            reply_command_notice: reply_slot.notice(client),
                            send_notice: send_slot.notice(client),
                            on_open_past_work: move |_| past_work_open.set(true),
                            on_close_reply: move |reply_id: String| {
                                let Some(ws_id) = selected() else { return };
                                reply_notice.set(String::new());
                                reply_slot.run(
                                    client.read().clone(),
                                    Command::new(
                                        "reply.close",
                                        serde_json::json!({ "patient_workspace_id": ws_id, "reply_work_id": reply_id }),
                                    ),
                                    move || {
                                        *workspace_attempt.write() += 1;
                                        *today_attempt.write() += 1;
                                        reply_notice.set(REPLY_CLOSED_NOTICE.to_owned());
                                    },
                                );
                            },
                            on_revise_reply: move |(reply_id, instruction): (String, String)| {
                                let Some(ws_id) = selected() else { return };
                                let zone = workspace_cache
                                    .read()
                                    .as_ref()
                                    .and_then(|open| open.time_zone.clone())
                                    .unwrap_or_default();
                                reply_notice.set(String::new());
                                reply_slot.run(
                                    client.read().clone(),
                                    Command::new(
                                        "reply.revise",
                                        serde_json::json!({
                                            "patient_workspace_id": ws_id,
                                            "reply_work_id": reply_id,
                                            "instruction": instruction,
                                            "physician_time_zone": zone,
                                        }),
                                    ),
                                    move || *workspace_attempt.write() += 1,
                                );
                            },
                        }
                    },
                }
            }
        }
        if search_open() {
            SearchDialog {
                query: query(),
                state: search_state.clone(),
                on_retry: move |_| *search_attempt.write() += 1,
                on_query: move |value: String| query.set(value),
                on_close: move |_| search_open.set(false),
                on_open: move |id: String| {
                    selected.set(Some(id));
                    place.set(Place::Workspace);
                    search_open.set(false);
                    editing.set(false);
                    workspace_attempt.set(workspace_attempt() + 1);
                },
                focused: if search_open() { shown_focus } else { None },
                on_editing: move |value: bool| editing.set(value),
            }
        }
        if shortcuts_open() {
            ShortcutsDialog { on_close: move |_| shortcuts_open.set(false) }
        }
        if connections_open() {
            ConnectionsDialog {
                lines: connections.clone(),
                failed: matches!(today_state, ReadView::Unavailable),
                on_close: move |_| connections_open.set(false),
            }
        }
        if let Some(open) = reading() {
            ReaderDialog {
                reading: open,
                on_close: move |_| reading.set(None),
            }
        }
        if past_work_open() && place() == Place::Workspace {
            PastWorkDialog {
                patient: open_name.clone().unwrap_or_default(),
                on_close: move |_| past_work_open.set(false),
            }
        }
    }
}

const COMMAND_REFUSED: &str = "Coda didn’t take that.";
const COMMAND_SENDING: &str = "Sending…";
const COMMAND_UNCONFIRMED: &str = "Coda couldn’t confirm that.";
const CHECK_AGAIN: &str = "Check again";
const MESSAGE_REFUSED: &str = "Coda couldn’t accept that message.";

fn command_rejection(value: &serde_json::Value) -> Option<String> {
    if value.get("outcome").and_then(serde_json::Value::as_str) == Some("rejected") {
        Some(
            value
                .get("message")
                .and_then(serde_json::Value::as_str)
                .unwrap_or(COMMAND_REFUSED)
                .to_owned(),
        )
    } else {
        None
    }
}

/// One physician command. Its key stays with it, so Check again resends the
/// same request and the server takes it at most once.
#[derive(Clone, Debug, PartialEq)]
struct Command(serde_json::Value);

impl Command {
    fn new(kind: &str, payload: serde_json::Value) -> Self {
        Self::keyed(kind, idempotency_key(), payload)
    }

    fn keyed(kind: &str, key: String, payload: serde_json::Value) -> Self {
        Self(serde_json::json!({ "type": kind, "idempotency_key": key, "payload": payload }))
    }
}

/// How a command was answered.
#[derive(Debug, PartialEq)]
enum Answer {
    Accepted,
    Refused(String),
    /// No answer arrived, so whether it took is unknown.
    Unconfirmed,
}

fn answer(result: &Result<serde_json::Value, physician_client::HttpError>) -> Answer {
    match result {
        Ok(value) => command_rejection(value).map_or(Answer::Accepted, Answer::Refused),
        // The server answered and said no: a refusal, with its reason when given.
        Err(physician_client::HttpError::Status { status, body })
            if (400..500).contains(status) && !matches!(status, 408 | 429) =>
        {
            let value = serde_json::from_str::<serde_json::Value>(body).unwrap_or_default();
            Answer::Refused(
                command_rejection(&value)
                    .or_else(|| {
                        value
                            .get("detail")
                            .and_then(serde_json::Value::as_str)
                            .map(str::to_owned)
                    })
                    .unwrap_or_else(|| COMMAND_REFUSED.to_owned()),
            )
        }
        Err(_) => Answer::Unconfirmed,
    }
}

/// One command at a time on one item: what is in flight, and what Check again resends.
#[derive(Clone, Copy, PartialEq)]
struct CommandSlot {
    stage: Signal<CommandStage>,
    /// The unconfirmed command and what runs once it is taken.
    last: Signal<Option<(Command, Callback)>>,
}

/// One read as the page shows it. A failed refresh keeps the last answer.
fn use_read_view<T: Clone + 'static>(resource: Resource<Result<T, ()>>) -> ReadView<T> {
    let mut last = use_signal(|| None::<T>);
    use_effect(move || {
        if let Some(Ok(value)) = resource.read().as_ref() {
            last.set(Some(value.clone()));
        }
    });
    read_view(last.read().clone(), resource.read().clone())
}

fn use_command_slot() -> CommandSlot {
    CommandSlot {
        stage: use_signal(|| CommandStage::Idle),
        last: use_signal(|| None),
    }
}

impl CommandSlot {
    fn sending(&self) -> bool {
        *self.stage.read() == CommandStage::Sending
    }

    /// Sends `command` unless one is already in flight; `then` runs once it is taken.
    fn run(self, client: PhysicianClient, command: Command, mut then: impl FnMut() + 'static) {
        self.send(client, command, Callback::new(move |()| then()));
    }

    fn send(mut self, client: PhysicianClient, command: Command, then: Callback) {
        if *self.stage.peek() == CommandStage::Sending {
            return;
        }
        self.last.set(Some((command.clone(), then)));
        self.stage.set(CommandStage::Sending);
        spawn(async move {
            match answer(&client.command(&command.0).await) {
                Answer::Accepted => {
                    self.stage.set(CommandStage::Idle);
                    self.last.set(None);
                    then.call(());
                }
                Answer::Refused(reason) => {
                    self.stage.set(CommandStage::Refused(reason));
                    self.last.set(None);
                }
                Answer::Unconfirmed => self.stage.set(CommandStage::Unconfirmed),
            }
        });
    }

    /// Like `run`, for a command that could not be put together: its reason shows as a refusal.
    fn run_or_refuse(
        mut self,
        client: PhysicianClient,
        command: Result<Command, String>,
        then: impl FnMut() + 'static,
    ) {
        match command {
            Ok(command) => self.run(client, command, then),
            Err(reason) if !self.sending() => self.stage.set(CommandStage::Refused(reason)),
            Err(_) => {}
        }
    }

    /// Resends the unconfirmed command as the same request, with its own follow-up.
    fn check_again(self, client: PhysicianClient) {
        if let Some((command, then)) = self.last.peek().clone() {
            self.send(client, command, then);
        }
    }

    /// A line beside the command's controls.
    fn notice(self, client: Signal<PhysicianClient>) -> Element {
        rsx! {
            CommandNotice {
                stage: self.stage.read().clone(),
                sending: COMMAND_SENDING,
                unconfirmed: COMMAND_UNCONFIRMED,
                check_again: CHECK_AGAIN,
                on_check_again: move |_| self.check_again(client.peek().clone()),
            }
        }
    }
}

#[cfg(test)]
mod command_answer_tests {
    use super::{Answer, COMMAND_REFUSED, answer};
    use physician_client::HttpError;

    #[test]
    fn a_command_is_taken_refused_with_its_reason_or_unconfirmed() {
        assert_eq!(
            answer(&Ok(serde_json::json!({ "outcome": "accepted" }))),
            Answer::Accepted
        );
        assert_eq!(
            answer(&Ok(
                serde_json::json!({ "outcome": "rejected", "message": "Already closed." })
            )),
            Answer::Refused("Already closed.".into())
        );
        let status = |status: u16, body: &str| HttpError::Status {
            status,
            body: body.into(),
        };
        assert_eq!(
            answer(&Err(status(404, r#"{"detail":"Not found."}"#))),
            Answer::Refused("Not found.".into())
        );
        assert_eq!(
            answer(&Err(status(409, ""))),
            Answer::Refused(COMMAND_REFUSED.into())
        );
        // No answer, or one that says nothing about the command: unknown.
        for unknown in [status(503, ""), status(429, ""), status(408, "")] {
            assert_eq!(answer(&Err(unknown)), Answer::Unconfirmed);
        }
        assert_eq!(
            answer(&Err(HttpError::Transport("reset".into()))),
            Answer::Unconfirmed
        );
    }
}

/// The sign-in page, or with `resume_email` the prompt covering a page whose
/// session ended: it mails the same address, and signing in picks up there.
#[component]
fn SignInScreen(
    client: Signal<PhysicianClient>,
    link: Option<SignInLink>,
    on_signed_in: EventHandler<Option<WhoamiResponse>>,
    resume_email: Option<String>,
    on_sign_out: Option<EventHandler<()>>,
) -> Element {
    let resuming = resume_email.is_some();
    let mut email = use_signal(|| {
        link.as_ref()
            .map(|link| link.email.clone())
            .or_else(|| resume_email.clone())
            .unwrap_or_default()
    });
    let mut stage = use_signal(|| {
        if link.is_some() {
            SignInStage::SigningIn
        } else {
            SignInStage::Email
        }
    });
    let mut error = use_signal(String::new);

    let mut verify_link = move |link: SignInLink| {
        let client = client.peek().clone();
        stage.set(SignInStage::SigningIn);
        error.set(String::new());
        spawn(async move {
            match client.verify(&link.email, &link.code).await {
                Ok(verified) => {
                    #[cfg(target_arch = "wasm32")]
                    {
                        write_stored_session(Some(&verified.session));
                        tabs::announce(&tabs::TabNews::SignedIn {
                            physician_id: verified.physician_id.clone(),
                            session: verified.session.clone(),
                        });
                    }
                    #[cfg(not(target_arch = "wasm32"))]
                    let _ = verified;
                    on_signed_in.call(client.whoami().await.ok());
                }
                Err(physician_client::HttpError::Status { status: 403, .. }) => {
                    stage.set(SignInStage::NoAccess);
                }
                Err(failure) => {
                    let message = match failure {
                        physician_client::HttpError::Status { status: 401, .. } => SIGN_IN_INVALID,
                        _ => {
                            "Coda couldn’t confirm sign-in. Open your email link again or request a new one."
                        }
                    };
                    error.set(message.to_owned());
                    stage.set(SignInStage::Recovery);
                }
            }
        });
    };
    // This effect has no reactive reads: one page load redeems its mailed link
    // once. Ordinary rerenders and request failures never retry the token.
    use_effect(move || {
        if let Some(link) = link.clone() {
            verify_link(link);
        }
    });

    let mut request_token = move |_: ()| {
        if matches!(
            *stage.peek(),
            SignInStage::Requesting | SignInStage::SigningIn
        ) {
            return;
        }
        let client = client.peek().clone();
        let email_value = email.peek().trim().to_owned();
        stage.set(SignInStage::Requesting);
        error.set(String::new());
        spawn(async move {
            match client.begin_sign_in(&email_value).await {
                Ok(answer) => match sign_in_followup(&answer) {
                    SignInFollowup::Issued(code) => {
                        // Local proof delivery returns the token directly;
                        // redeem it through the same path without a code UI.
                        verify_link(SignInLink {
                            email: email_value,
                            code,
                        });
                    }
                    SignInFollowup::Mailed => {
                        // Hosted delivery and unknown addresses share this
                        // confirmation. Only the emailed link can sign in.
                        stage.set(SignInStage::CheckEmail);
                    }
                    SignInFollowup::Failed(text) => {
                        error.set(text);
                        stage.set(SignInStage::Recovery);
                    }
                },
                Err(_) => {
                    error.set(SIGN_IN_UNAVAILABLE.to_owned());
                    stage.set(SignInStage::Recovery);
                }
            }
        });
    };
    let working = matches!(stage(), SignInStage::Requesting | SignInStage::SigningIn);
    rsx! {
        div { class: "sign-in-screen",
            section {
                class: "sign-in-card",
                aria_label: "Sign in to Coda",
                img {
                    class: "sign-in-logo",
                    src: "./assets/coda-health-logo-transparent.png",
                    alt: "Coda Health",
                    width: "152",
                }
                div { class: "sign-in-content", aria_live: "polite", aria_busy: working,
                    h1 {
                        match stage() {
                            SignInStage::CheckEmail => "Check your email",
                            SignInStage::Requesting => "Sending your link…",
                            SignInStage::SigningIn => "Signing you in…",
                            SignInStage::Recovery => "Let’s get you signed in",
                            SignInStage::NoAccess => "No access to this practice",
                            SignInStage::Email if resuming => "Sign in to continue",
                            SignInStage::Email => "Sign in to Coda",
                        }
                    }
                    match stage() {
                        SignInStage::Email if resuming => rsx! {
                            p { class: "sign-in-sub", "{SESSION_ENDED}" }
                            p { class: "sign-in-email", "{email}" }
                            div { class: "sign-in-form",
                                button {
                                    class: "sign-in-primary",
                                    r#type: "button",
                                    disabled: email.read().trim().is_empty(),
                                    onclick: move |_| request_token(()),
                                    "Email me a sign-in link"
                                }
                                p { class: "sign-in-hint", "Open it to pick up where you were." }
                            }
                        },
                        SignInStage::NoAccess => rsx! {
                            p { class: "sign-in-sub", "{NO_ACCESS}" }
                            div { class: "sign-in-actions",
                                button {
                                    class: "sign-in-text-action",
                                    r#type: "button",
                                    onclick: move |_| {
                                        email.set(String::new());
                                        stage.set(SignInStage::Email);
                                    },
                                    "Use a different email"
                                }
                            }
                        },
                        SignInStage::Email => rsx! {
                            p { class: "sign-in-sub", "Your practice, with you." }
                            div { class: "sign-in-form",
                                label { "Work email"
                                    input {
                                        class: "sign-in-input",
                                        r#type: "email",
                                        autocomplete: "email",
                                        placeholder: "you@practice.com",
                                        value: "{email}",
                                        oninput: move |event| email.set(event.value()),
                                        onkeydown: move |event| {
                                            if event.key().to_string() == "Enter" && !email.peek().trim().is_empty() {
                                                request_token(());
                                            }
                                        },
                                    }
                                }
                                button {
                                    class: "sign-in-primary",
                                    r#type: "button",
                                    disabled: email.read().trim().is_empty(),
                                    onclick: move |_| request_token(()),
                                    "Send sign-in link"
                                }
                                p { class: "sign-in-hint", "We’ll email you a link. No password needed." }
                            }
                        },
                        SignInStage::Requesting | SignInStage::SigningIn => rsx! {
                            p { class: "sign-in-sub", role: "status",
                                if stage() == SignInStage::Requesting {
                                    "This may take a moment."
                                } else {
                                    "Opening your Coda workspace."
                                }
                            }
                            div { class: "sign-in-progress", aria_hidden: "true", span {} }
                        },
                        SignInStage::CheckEmail | SignInStage::Recovery => rsx! {
                            if stage() == SignInStage::CheckEmail {
                                p { class: "sign-in-sub", "{LINK_ON_THE_WAY}" }
                                p { class: "sign-in-email", "{email}" }
                                p { class: "sign-in-sub",
                                    if resuming {
                                        "Choose Sign in in the email to pick up where you were."
                                    } else {
                                        "Choose Sign in in the email to open your workspace."
                                    }
                                }
                            }
                            if !error().is_empty() {
                                p { class: "sign-in-error", role: "alert", "{error}" }
                            }
                            div { class: "sign-in-actions",
                                button {
                                    class: "sign-in-text-action",
                                    r#type: "button",
                                    onclick: move |_| request_token(()),
                                    "Send a new link"
                                }
                                if !resuming {
                                    button {
                                        class: "sign-in-text-action sign-in-change",
                                        r#type: "button",
                                        onclick: move |_| {
                                            error.set(String::new());
                                            stage.set(SignInStage::Email);
                                        },
                                        "Use a different email"
                                    }
                                }
                            }
                        },
                    }
                    if let Some(sign_out) = on_sign_out {
                        div { class: "sign-in-actions",
                            button {
                                class: "sign-in-text-action sign-in-change",
                                r#type: "button",
                                onclick: move |_| sign_out.call(()),
                                "Sign out"
                            }
                        }
                    }
                }
            }
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum SignInStage {
    Email,
    Requesting,
    CheckEmail,
    SigningIn,
    Recovery,
    /// A held link whose account no longer has access to this practice.
    NoAccess,
}

#[derive(Debug, PartialEq, Eq)]
enum SignInFollowup {
    /// Proof mode returned a token to redeem directly.
    Issued(String),
    /// The link is on its way; the person continues from their email.
    Mailed,
    /// The response is not a sign-in the screen can continue.
    Failed(String),
}

fn sign_in_followup(answer: &SignInResponse) -> SignInFollowup {
    if answer.status == "token_issued" {
        return SignInFollowup::Issued(answer.token.clone().unwrap_or_default());
    }
    if answer.message.as_deref() == Some(LINK_ON_THE_WAY) {
        return SignInFollowup::Mailed;
    }
    SignInFollowup::Failed(
        answer
            .message
            .clone()
            .filter(|text| !text.is_empty())
            .unwrap_or_else(|| SIGN_IN_UNAVAILABLE.to_owned()),
    )
}

#[cfg(test)]
mod sign_in_followup_tests {
    use super::{LINK_ON_THE_WAY, SIGN_IN_UNAVAILABLE, SignInFollowup, sign_in_followup};
    use physician_wire::SignInResponse;

    fn answer(status: &str, token: Option<&str>, message: Option<&str>) -> SignInResponse {
        SignInResponse {
            status: status.to_owned(),
            token: token.map(str::to_owned),
            message: message.map(str::to_owned),
        }
    }

    #[test]
    fn proof_delivery_provides_the_token_for_redemption() {
        assert_eq!(
            sign_in_followup(&answer("token_issued", Some("codas-1"), None)),
            SignInFollowup::Issued("codas-1".into())
        );
    }

    #[test]
    fn a_hosted_link_waits_for_email() {
        assert_eq!(
            sign_in_followup(&answer("unavailable", None, Some(LINK_ON_THE_WAY))),
            SignInFollowup::Mailed
        );
    }

    #[test]
    fn a_hosted_response_is_not_token_issued() {
        let followup = sign_in_followup(&answer("unavailable", None, Some(LINK_ON_THE_WAY)));
        assert!(!matches!(followup, SignInFollowup::Issued(_)));
    }

    #[test]
    fn another_message_stays_a_failure() {
        assert_eq!(
            sign_in_followup(&answer("unavailable", None, Some("nope"))),
            SignInFollowup::Failed("nope".into())
        );
        assert_eq!(
            sign_in_followup(&answer("unavailable", None, None)),
            SignInFollowup::Failed(SIGN_IN_UNAVAILABLE.into())
        );
    }
}

#[component]
fn RecentRow(conversation: RecentConversation) -> Element {
    let title = conversation
        .title
        .clone()
        .unwrap_or_else(|| "Untitled".to_owned());
    rsx! {
        div { class: "sidebar-recent-link",
            Icon { name: IconName::Message }
            span { class: "item-text",
                span { class: "sidebar-recent-title", "{title}" }
                if let Some(patient) = conversation.patient_display_name {
                    span { class: "sidebar-recent-patient", "{patient}" }
                }
            }
        }
    }
}

#[component]
fn TodaySurface(
    state: TodayView,
    simulation: Option<SimulationStatus>,
    connections: Vec<String>,
    visits: Vec<VisitLine>,
    show_schedule: bool,
    schedule: ui::ReadStage,
    on_schedule_retry: EventHandler,
    schedule_empty: String,
    schedule_collapsed: bool,
    on_schedule_collapsed: EventHandler<bool>,
    server_now: String,
    need_you: usize,
    focused: Option<i32>,
    on_retry: EventHandler,
    on_run_scenario: EventHandler<String>,
    on_open_patient: EventHandler<String>,
    on_open_row: EventHandler<TodayLine>,
    on_open_visit: EventHandler<VisitLine>,
    on_resume: EventHandler<TodayLine>,
    resume_notice: Element,
) -> Element {
    let words = need_you_words(need_you);
    let scheduled = show_schedule;
    let connections_quiet = connections.is_empty();
    let row_len = state.content().map_or(0, |rows| rows.len() as i32);
    let row_focus = focused.and_then(|index| {
        if index >= 0 && index < row_len {
            Some(index as usize)
        } else {
            None
        }
    });
    let visit_focus = focused.and_then(|index| {
        let visit_index = index - row_len;
        if visit_index >= 0 {
            Some(visit_index as usize)
        } else {
            None
        }
    });
    rsx! {
        div { class: "reading-workspace",
            header { class: "today-heading",
                h1 { "Today" }
                if need_you > 0 {
                    span { class: "count", span { class: "count-value", "{need_you}" } " {words}" }
                }
            }
            div { class: if scheduled { "today-layout has-schedule" } else { "today-layout" },
                section { class: "today-main", aria_label: "Needs you",
                    if let Some(sim) = simulation {
                        SimulationPanel {
                            sim,
                            on_run_scenario,
                            on_open_patient,
                        }
                    }
                    for line in connections {
                        p { class: "today-connection", "{line}" }
                    }
                    ReadNotice {
                        stage: state.stage(),
                        loading: TODAY_LOADING,
                        stale: TODAY_STALE,
                        unavailable: TODAY_UNAVAILABLE,
                        retry: TRY_AGAIN,
                        on_retry,
                    }
                    {resume_notice}
                    match state.content().cloned() {
                        Some(rows) if rows.is_empty() && connections_quiet => rsx! {
                            div { class: "today-list today-quiet",
                                strong { "{TODAY_QUIET}" }
                                span { "{TODAY_QUIET_DETAIL}" }
                            }
                        },
                        Some(rows) if !rows.is_empty() => rsx! {
                            TodayRows { rows, server_now: server_now.clone(), focused: row_focus, on_open: on_open_row, on_resume }
                        },
                        _ => rsx! {},
                    }
                }
                if show_schedule {
                    ScheduleRail {
                        visits: visits.clone(),
                        title: "Schedule",
                        read: schedule,
                        on_retry: on_schedule_retry,
                        empty: schedule_empty.clone(),
                        collapsed: schedule_collapsed,
                        on_collapsed: on_schedule_collapsed,
                        name_the_visit: false,
                        focused: visit_focus,
                        on_open: on_open_visit,
                    }
                }
            }
        }
    }
}

#[component]
fn SimulationPanel(
    sim: SimulationStatus,
    on_run_scenario: EventHandler<String>,
    on_open_patient: EventHandler<String>,
) -> Element {
    let metric = &sim.investor_metric;
    let agg = &sim.aggregate_metric;
    let items_per_100 = format!("{:.1}", metric.items_per_100_patients_per_week);
    let agg_items_per_100 = format!("{:.1}", agg.items_per_100_patients_per_week);
    let active_description = sim
        .scenarios
        .iter()
        .find(|scenario| scenario.key == sim.active_scenario_key)
        .map(|scenario| scenario.description.clone())
        .unwrap_or_default();
    rsx! {
        section { class: "simulation-panel", aria_label: "Local practice simulation",
            div { class: "simulation-header",
                h2 { "Simulated practice walk — {sim.active_scenario_label}" }
                span { class: "simulation-clock", "{sim.current_time}" }
            }
            if !active_description.is_empty() {
                p { class: "simulation-description", "{active_description}" }
            }
            div { class: "investor-metric-card", aria_label: "Investor metric",
                span { class: "investor-metric-label", "Items the physician touched per 100 patients per week" }
                strong { class: "investor-metric-value", "{items_per_100}" }
                span { class: "investor-metric-detail",
                    "{metric.physician_touched_items} physician touches across {metric.panel_size} patients ({metric.ambient_handled_items} handled ambiently)"
                }
                span { class: "investor-metric-aggregate",
                    "Across {agg.runs_count} simulation runs: {agg_items_per_100} items per 100 patients per week"
                }
            }
            div { class: "simulation-scenarios", aria_label: "Simulation scenarios",
                for scenario in sim.scenarios.iter() {
                    button {
                        class: if scenario.key == sim.active_scenario_key {
                            "control-secondary is-active-scenario"
                        } else {
                            "control-secondary"
                        },
                        r#type: "button",
                        "data-scenario-key": "{scenario.key}",
                        onclick: {
                            let key = scenario.key.clone();
                            move |_| on_run_scenario.call(key.clone())
                        },
                        "{scenario.label}"
                    }
                }
            }
            div { class: "simulation-panel-patients", aria_label: "Synthetic panel patients",
                for patient in sim.panel_patients.iter() {
                    button {
                        class: "control-secondary",
                        r#type: "button",
                        "data-panel-patient": "{patient.patient_workspace_id}",
                        onclick: {
                            let id = patient.patient_workspace_id.clone();
                            move |_| on_open_patient.call(id.clone())
                        },
                        "{patient.display_name} (MRN {patient.mrn})"
                    }
                }
            }
        }
    }
}

#[component]
fn TodayRows(
    rows: Vec<TodayLine>,
    server_now: String,
    focused: Option<usize>,
    on_open: EventHandler<TodayLine>,
    on_resume: EventHandler<TodayLine>,
) -> Element {
    rsx! {
        div { class: "today-list",
            for (index, row) in rows.iter().enumerate() {
                article { class: "today-row",
                    button {
                        class: if focused == Some(index) { "today-row-open is-keyboard-focus" } else { "today-row-open" },
                        r#type: "button",
                        onclick: {
                            let row = row.clone();
                            move |_| on_open.call(row.clone())
                        },
                        span { class: "today-row-line",
                            span { class: "today-row-heading",
                                span { class: "live-dot", aria_hidden: "true" }
                                strong { class: "today-row-who", "{row.patient}" }
                                if row.now {
                                    span { class: "urgency-now", "{NEEDS_YOU_NOW}" }
                                }
                                span { class: "state-pill",
                                    Icon { name: IconName::Alert }
                                    "{row.did}"
                                }
                            }
                            if let Some(waited) = format_waited(&row.waiting_since, &server_now) {
                                time { class: "today-row-waited", datetime: "{row.waiting_since}", "{waited}" }
                            }
                        }
                        if !row.reason.is_empty() {
                            span { class: "today-row-reason", "{row.reason}" }
                        }
                    }
                    if row.resume_work_item_id.is_some() {
                        div { class: "today-row-action",
                            button {
                                class: "control-secondary",
                                r#type: "button",
                                onclick: {
                                    let row = row.clone();
                                    move |_| on_resume.call(row.clone())
                                },
                                "{TRY_AGAIN}"
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn ScheduleRail(
    visits: Vec<VisitLine>,
    title: &'static str,
    read: ui::ReadStage,
    on_retry: EventHandler,
    empty: String,
    collapsed: bool,
    on_collapsed: EventHandler<bool>,
    name_the_visit: bool,
    focused: Option<usize>,
    on_open: EventHandler<VisitLine>,
) -> Element {
    let count = visits.len();
    let rail_class = if collapsed {
        "schedule-rail is-collapsed"
    } else {
        "schedule-rail"
    };
    rsx! {
        aside { class: "{rail_class}", aria_label: "{title}",
            if collapsed {
                button {
                    class: "schedule-rail-expand",
                    r#type: "button",
                    aria_expanded: "false",
                    aria_label: "Show {title}",
                    onclick: move |_| on_collapsed.call(false),
                    span { class: "icon-tile", aria_hidden: "true",
                        Icon { name: IconName::Calendar }
                    }
                    if count > 0 {
                        span { class: "count", span { class: "count-value", "{count}" } }
                    }
                }
            }
            div { class: "schedule-rail-body",
            header { class: "card-header",
                span { class: "icon-tile", aria_hidden: "true",
                    Icon { name: IconName::Calendar }
                }
                div { class: "card-title",
                    h2 { "{title}" }
                    if count > 0 {
                        span { class: "count", span { class: "count-value", "{count}" } " left" }
                    }
                }
                button {
                    class: "icon-button schedule-rail-collapse",
                    r#type: "button",
                    aria_expanded: "true",
                    aria_label: "Collapse {title}",
                    onclick: move |_| on_collapsed.call(true),
                    Icon { name: IconName::PanelRightClose }
                }
            }
            ReadNotice {
                stage: read,
                loading: SCHEDULE_LOADING,
                stale: SCHEDULE_STALE,
                unavailable: SCHEDULE_UNAVAILABLE,
                retry: TRY_AGAIN,
                on_retry,
            }
            if visits.is_empty() && read == ui::ReadStage::Showing {
                p { class: "schedule-rail-empty", "{empty}" }
            }
            for (index, visit) in visits.iter().enumerate() {
                button {
                    class: if focused == Some(index) {
                        "schedule-visit is-next is-keyboard-focus"
                    } else if index == 0 {
                        "schedule-visit is-next"
                    } else {
                        "schedule-visit"
                    },
                    r#type: "button",
                    onclick: {
                        let visit = visit.clone();
                        move |_| on_open.call(visit.clone())
                    },
                    aria_label: "Open {visit.patient}",
                    if let Some(clock) = format_clock(&visit.starts_at, &visit.time_zone) {
                        time { datetime: "{visit.starts_at}", "{clock}" }
                    }
                    strong { if name_the_visit { "{visit.label}" } else { "{visit.patient}" } }
                    if name_the_visit {
                        if let Some(cue) = &visit.cue {
                            span { class: "schedule-visit-preparation", "{cue}" }
                        }
                    }
                }
            }
            }
        }
    }
}

#[component]
fn UpdatesSurface(state: ReadView<Vec<serde_json::Value>>, on_retry: EventHandler) -> Element {
    rsx! {
        div { class: "reading-workspace",
            header { class: "today-heading", h1 { "Updates" } }
            ReadNotice {
                stage: state.stage(),
                loading: UPDATES_LOADING,
                stale: UPDATES_STALE,
                unavailable: UPDATES_UNAVAILABLE,
                retry: TRY_AGAIN,
                on_retry,
            }
            match state.content().cloned() {
                None => rsx! {},
                Some(rows) if rows.is_empty() => rsx! {
                    div { class: "updates-empty",
                        h2 { "{UPDATES_EMPTY}" }
                        p { "{UPDATES_EMPTY_DETAIL}" }
                    }
                },
                Some(rows) => rsx! {
                    div { class: "today-list",
                        for row in rows {
                            article { class: "today-row",
                                strong { class: "today-row-who", "{row.get(\"patient_display_name\").and_then(|value| value.as_str()).unwrap_or(\"Update\")}" }
                            }
                        }
                    }
                },
            }
        }
    }
}

#[component]
fn PatientSurface(
    gone: bool,
    read: ui::ReadStage,
    on_go_today: EventHandler,
    name: String,
    workspace: Option<PatientWorkspaceResponse>,
    rows: Vec<TodayLine>,
    visits: Vec<VisitLine>,
    schedule_collapsed: bool,
    on_schedule_collapsed: EventHandler<bool>,
    schedule: ui::ReadStage,
    on_schedule_retry: EventHandler,
    focused: Option<i32>,
    chats: ChatView,
    server_now: String,
    draft: String,
    notice: String,
    is_admin: bool,
    practice_accounts: Vec<PracticeAccountSummary>,
    target_physician_id: String,
    panel_busy: bool,
    panel_notice: String,
    on_select_physician: EventHandler<String>,
    on_assign_panel: EventHandler,
    on_remove_panel: EventHandler,
    on_retry: EventHandler,
    on_decide_recommendation: EventHandler<String>,
    on_admit_corrected_labs: EventHandler,
    on_read: EventHandler<(String, String)>,
    on_open_source: EventHandler<SourceChip>,
    on_draft: EventHandler<String>,
    on_save: EventHandler,
    on_send: EventHandler,
    on_dictate: EventHandler,
    on_editing: EventHandler<bool>,
    conversation: ReadView<Vec<String>>,
    on_conversation_retry: EventHandler,
    on_open_chat: EventHandler<String>,
    conversation_open: bool,
    past_work: bool,
    reply_notice: String,
    reply_busy: bool,
    reply_command_notice: Element,
    send_notice: Element,
    work_busy: bool,
    work_notice: Element,
    resume_notice: Element,
    on_resume: EventHandler<TodayLine>,
    on_open_past_work: EventHandler,
    on_close_reply: EventHandler<String>,
    on_revise_reply: EventHandler<(String, String)>,
) -> Element {
    let clinic_demographic = workspace
        .as_ref()
        .is_some_and(PatientWorkspaceResponse::is_clinic_demographic);
    let demographic = workspace.as_ref().map(demographic_card_view);
    let segments = workspace
        .as_ref()
        .and_then(|open| open.patient_synopsis.as_ref())
        .map(|synopsis| synopsis.segments.clone())
        .unwrap_or_default();
    let work = workspace
        .as_ref()
        .and_then(|open| open.automation_run.clone());
    let follow_ups = workspace
        .as_ref()
        .map(|open| open.active_follow_ups.clone())
        .unwrap_or_default();
    let simulation = workspace.as_ref().and_then(|open| open.simulation.clone());
    let replies = workspace
        .as_ref()
        .map(|open| open.replies.clone())
        .unwrap_or_default();
    let superseded_replies = workspace
        .as_ref()
        .map(|open| open.superseded_replies.clone())
        .unwrap_or_default();
    let active_plan_items = workspace
        .as_ref()
        .map(|open| open.active_plan_items.clone())
        .unwrap_or_default();
    let undecided_recommendations = workspace
        .as_ref()
        .map(|open| open.undecided_recommendations.clone())
        .unwrap_or_default();
    let shared_records = workspace
        .as_ref()
        .map(|open| open.duet_records.clone())
        .unwrap_or_default();
    let doctor_plan = workspace.as_ref().and_then(|open| open.doctor_plan.clone());
    let dob = demographic
        .as_ref()
        .and_then(|card| card.birth_date.clone());
    let gender = demographic
        .as_ref()
        .and_then(|card| card.sex_or_gender.clone());
    let age = demographic.as_ref().and_then(|card| card.age.clone());
    let mrn = demographic.as_ref().and_then(|card| card.mrn.clone());
    let record_state = demographic
        .as_ref()
        .and_then(|card| card.record_state.clone());
    let times = workspace
        .as_ref()
        .map(|open| summary_times(open, &server_now))
        .unwrap_or_default();
    let visit_focus = focused.and_then(|index| {
        if index >= 0 && (index as usize) < visits.len() {
            Some(index as usize)
        } else {
            None
        }
    });
    let scheduled = !visits.is_empty();
    let heading_name = demographic
        .as_ref()
        .map(|card| card.display_name.clone())
        .filter(|value| !value.is_empty())
        .unwrap_or_else(|| name.clone());
    let is_simulated = simulation.is_some();
    if gone {
        // Nothing of the patient shows, not even the name.
        return rsx! {
            div { class: "reading-workspace patient-page",
                ErrorState { message: PATIENT_GONE }
                TextButton { label: GO_TO_TODAY, quiet: true, on_press: on_go_today }
            }
        };
    }
    rsx! {
        div { class: "reading-workspace patient-page",
            header { class: "patient-identity",
                div { class: "patient-identity-top",
                    h1 { class: "workspace-title", "{heading_name}" }
                    div { class: "patient-identity-actions",
                        if past_work && !clinic_demographic {
                            button {
                                class: "control-secondary past-work-button",
                                r#type: "button",
                                onclick: move |_| on_open_past_work.call(()),
                                "Past work"
                            }
                        }
                        if is_admin && workspace.is_some() && !clinic_demographic {
                            button {
                                class: "control-secondary remove-panel-button",
                                r#type: "button",
                                disabled: panel_busy,
                                onclick: move |_| on_remove_panel.call(()),
                                if panel_busy { "Updating…" } else { "Remove from panel" }
                            }
                        }
                    }
                }
                div { class: "patient-facts",
                    if let Some(age) = age.clone() {
                        span { "Age {age}" }
                    }
                    if let Some(dob) = dob.clone() {
                        span { "DOB {dob}" }
                    }
                    if let Some(gender) = gender.clone() {
                        span { "Gender {gender}" }
                    }
                    if let Some(mrn) = mrn.clone() {
                        span { "MRN {mrn}" }
                    }
                }
                if !clinic_demographic && !panel_notice.is_empty() {
                    span { class: "composer-notice", "{panel_notice}" }
                }
            }
            div { class: if scheduled { "today-layout has-schedule" } else { "today-layout" },
            div { class: "today-main",
            ReadNotice {
                stage: read,
                loading: PATIENT_LOADING,
                stale: PATIENT_STALE,
                unavailable: PATIENT_UNAVAILABLE,
                retry: TRY_AGAIN,
                on_retry,
            }
            if clinic_demographic {
                section {
                    class: "patient-section clinic-demographic-card",
                    aria_label: "Clinic demographic record",
                    div { class: "clinic-demographic-header",
                        span { class: "clinic-demographic-badge", "Clinic record · Not on your panel" }
                        h2 { "Patient demographics" }
                        p { class: "clinic-demographic-copy",
                            "Basic clinic demographics are visible to physicians in this practice. Assign this patient to a physician panel to open clinical summaries, records, and conversations."
                        }
                    }
                    dl { class: "clinic-demographic-grid",
                        div { class: "clinic-demographic-field",
                            dt { "Full name" }
                            dd { "{heading_name}" }
                        }
                        if let Some(dob) = dob {
                            div { class: "clinic-demographic-field",
                                dt { "Date of birth" }
                                dd { "{dob}" }
                            }
                        }
                        if let Some(age) = age {
                            div { class: "clinic-demographic-field",
                                dt { "Age" }
                                dd { "{age}" }
                            }
                        }
                        if let Some(gender) = gender {
                            div { class: "clinic-demographic-field",
                                dt { "Sex or gender" }
                                dd { "{gender}" }
                            }
                        }
                        if let Some(mrn) = mrn {
                            div { class: "clinic-demographic-field",
                                dt { "Medical record number" }
                                dd { "{mrn}" }
                            }
                        }
                        if let Some(state) = record_state {
                            div { class: "clinic-demographic-field",
                                dt { "Record status" }
                                dd { "{state}" }
                            }
                        }
                    }
                    if is_admin {
                        div {
                            class: "panel-assignment-controls",
                            aria_label: "Panel assignment",
                            if practice_accounts.len() > 1 {
                                label { class: "panel-physician-label",
                                    "Physician panel"
                                    select {
                                        class: "panel-physician-select",
                                        aria_label: "Physician panel",
                                        value: "{target_physician_id}",
                                        onchange: move |event| on_select_physician.call(event.value()),
                                        for account in practice_accounts {
                                            option {
                                                value: "{account.physician_id}",
                                                "{account.display_name} ({account.email})"
                                            }
                                        }
                                    }
                                }
                            }
                            button {
                                class: "sign-in-primary assign-panel-button",
                                r#type: "button",
                                disabled: panel_busy,
                                onclick: move |_| on_assign_panel.call(()),
                                if panel_busy { "Assigning…" } else { "Assign to panel" }
                            }
                            if !panel_notice.is_empty() {
                                span { class: "composer-notice", "{panel_notice}" }
                            }
                        }
                    }
                }
            } else {
            if let Some(sim) = &simulation {
                div { class: "investor-metric-card", aria_label: "Investor metric",
                    span { class: "investor-metric-label", "Items the physician touched per 100 patients per week" }
                    strong { class: "investor-metric-value", "{sim.investor_metric.items_per_100_patients_per_week:.1}" }
                    span { class: "investor-metric-detail",
                        "Scenario: {sim.active_scenario_label} · {sim.investor_metric.physician_touched_items} touches across {sim.investor_metric.panel_size} patients"
                    }
                    if !replies.is_empty() {
                        button {
                            class: "control-secondary",
                            r#type: "button",
                            "data-action": "admit-corrected-labs",
                            disabled: work_busy,
                            onclick: move |_| on_admit_corrected_labs.call(()),
                            "Admit corrected BMP"
                        }
                    }
                }
            }
            {work_notice}
            {resume_notice}
            if !rows.is_empty() {
                TodayRows {
                    rows,
                    server_now: server_now.clone(),
                    focused: None,
                    on_open: move |line: TodayLine| {
                        if opens_reader(&line.kind) {
                            on_read.call((line.patient.clone(), line.reason.clone()));
                        }
                    },
                    on_resume,
                }
            }
            if !replies.is_empty() || !superseded_replies.is_empty() {
                section { class: "patient-section", aria_label: "Patient replies",
                    h2 { "Patient replies" }
                    if !reply_notice.is_empty() {
                        p { class: "composer-notice", role: "status", "{reply_notice}" }
                    }
                    {reply_command_notice}
                    for reply in replies {
                        ReplyReviewCard {
                            reply: reply.clone(),
                            busy: reply_busy,
                            on_editing,
                            on_close_reply,
                            on_revise_reply,
                        }
                    }
                    for superseded in superseded_replies {
                        article {
                            class: "simulation-card is-superseded",
                            "data-reply-state": "superseded_by_correction",
                            div { class: "simulation-card-header",
                                strong { "{superseded.headline}" }
                                span { class: "state-pill", "Superseded by corrected labs" }
                            }
                            if let Some(reason) = superseded.superseded_reason {
                                p { class: "patient-synopsis-meta", "{reason}" }
                            }
                        }
                    }
                }
            }
            if let Some(run) = work {
                if let Some(heading) = work_heading(&run) {
                    section { class: "patient-section", aria_label: "Current work",
                        h2 { "Current work" }
                        button {
                            class: "today-row-open",
                            r#type: "button",
                            onclick: {
                                let heading = heading.clone();
                                let preparation = visits
                                    .first()
                                    .and_then(|visit| visit.preparation.clone());
                                move |_| {
                                    let body = preparation
                                        .clone()
                                        .or_else(|| work_end_reason(run.ended_cause.as_deref()).map(str::to_owned))
                                        .unwrap_or_default();
                                    on_read.call((heading.clone(), body));
                                }
                            },
                            strong { "{heading}" }
                            if let Some(reason) = work_end_reason(run.ended_cause.as_deref()) {
                                span { class: "today-row-reason", "{reason}" }
                            }
                        }
                    }
                }
            }
            if !segments.is_empty() {
                section { class: "patient-section", aria_label: "Patient summary",
                    h2 { "Patient summary" }
                    if !times.is_empty() {
                        p { class: "patient-synopsis-meta",
                            for (index, mark) in times.iter().enumerate() {
                                if index > 0 {
                                    " · "
                                }
                                time { datetime: "{mark.instant}", "{mark.label} {mark.words}" }
                            }
                        }
                    }
                    div { class: "synopsis",
                        for piece in synopsis_pieces(&segments) {
                            match piece {
                                SynopsisPiece::Text(text) => rsx! { span { "{text}" } },
                                SynopsisPiece::Chips(chips) => rsx! {
                                    for chip in chips {
                                        CitationChip { chip: chip.clone(), on_open: on_open_source }
                                    }
                                },
                            }
                        }
                    }
                }
            }
            if is_simulated || !active_plan_items.is_empty() {
                section { class: "patient-section", aria_label: "Active plan",
                    h2 { "Active plan" }
                    for item in active_plan_items {
                        article {
                            class: "simulation-card",
                            "data-plan-item": "{item.item_id}",
                            strong { "{item.title}" }
                            p { class: "patient-synopsis-meta", "{item.detail}" }
                        }
                    }
                }
            }
            if is_simulated || !undecided_recommendations.is_empty() {
                section { class: "patient-section", aria_label: "Undecided medication recommendations",
                    h2 { "Undecided medication recommendations" }
                    if undecided_recommendations.is_empty() {
                        p { class: "patient-synopsis-meta", "No undecided medication recommendations." }
                    }
                    for rec in undecided_recommendations {
                        article {
                            class: "simulation-card",
                            "data-recommendation-id": "{rec.recommendation_id}",
                            div { class: "simulation-card-header",
                                strong { "{rec.medication} — Held (undecided)" }
                                button {
                                    class: "control-secondary",
                                    r#type: "button",
                                    "data-decide-recommendation": "{rec.recommendation_id}",
                                    disabled: work_busy,
                                    onclick: {
                                        let id = rec.recommendation_id.clone();
                                        move |_| on_decide_recommendation.call(id.clone())
                                    },
                                    "Approve into plan"
                                }
                            }
                            p { class: "patient-synopsis-meta", "{rec.rationale}" }
                            p { class: "patient-synopsis-meta", "Source: {rec.source_label}" }
                        }
                    }
                }
            }
            if !follow_ups.is_empty() {
                section { class: "patient-section", aria_label: "Follow-ups",
                    h2 { "Follow-ups" }
                    for follow_up in follow_ups {
                        p { "{follow_up.description.clone().unwrap_or_default()}" }
                    }
                }
            }
            if !shared_records.is_empty() {
                section { class: "patient-section", aria_label: "Patient-shared records",
                    h2 { "Patient-shared records" }
                    for record in shared_records {
                        article { class: "today-row",
                            strong { "{record.title}" }
                            p { "{record.summary}" }
                            span { class: "today-row-reason", "{record.provenance}" }
                        }
                    }
                }
            }
            if let Some(plan) = doctor_plan {
                section { class: "patient-section", aria_label: "Doctor's plan",
                    h2 { "Doctor's plan" }
                    p { class: "patient-synopsis-meta",
                        if plan.currently_approved {
                            "Approved v{plan.approved_version} · shared as your doctor's plan"
                        } else if plan.approved_version == 0 {
                            "Draft · full first version requires exact-text approval"
                        } else {
                            "Edited · prior approval invalidated until batched diffs are approved"
                        }
                    }
                    for item in plan.items {
                        article { class: "today-row",
                            strong { "{item.title}" }
                            p { "{item.body}" }
                            if item.needs_approval {
                                span { class: "today-row-reason", "Needs exact-text approval" }
                            } else {
                                span { class: "today-row-reason", "Approved" }
                            }
                        }
                    }
                }
            }
            section { class: "patient-section", aria_label: "Conversation",
                h2 { "Conversation" }
                match chats {
                    ChatView::Closed => rsx! { EmptyState { message: WORKSPACE_CLOSED.to_owned() } },
                    ChatView::Pending => rsx! {},
                    // The page's own read line says the patient couldn't open.
                    ChatView::Unavailable => rsx! {},
                    ChatView::Empty => rsx! { p { class: "sidebar-empty", "{WORKSPACE_EMPTY}" } },
                    ChatView::Chats(lines) => rsx! {
                        for line in lines {
                            button {
                                class: "sidebar-conversation-link",
                                r#type: "button",
                                onclick: {
                                    let id = line.id.clone();
                                    move |_| on_open_chat.call(id.clone())
                                },
                                Icon { name: IconName::Message }
                                "{line.title}"
                            }
                        }
                        if conversation_open {
                            ReadNotice {
                                stage: conversation.stage(),
                                loading: CONVERSATION_LOADING,
                                stale: "",
                                unavailable: CONVERSATION_UNAVAILABLE,
                                retry: TRY_AGAIN,
                                on_retry: on_conversation_retry,
                            }
                        }
                        if conversation_open && conversation.content().is_some_and(Vec::is_empty) {
                            h3 { class: "conversation-empty", "{CONVERSATION_EMPTY}" }
                        }
                        for message in conversation.content().cloned().unwrap_or_default() {
                            FormattedMessage { text: message }
                        }
                        textarea {
                            class: "composer",
                            placeholder: "Write a message",
                            value: "{draft}",
                            onfocus: move |_| on_editing.call(true),
                            onblur: move |_| on_editing.call(false),
                            oninput: move |event| on_draft.call(event.value()),
                        }
                        div { class: "composer-actions",
                            button { class: "control-secondary", r#type: "button", onclick: move |_| on_save.call(()), "Save" }
                            button { class: "control-secondary", r#type: "button", onclick: move |_| on_send.call(()), "Send" }
                            button { class: "control-secondary", r#type: "button", onclick: move |_| on_dictate.call(()), "Dictate" }
                            if !notice.is_empty() {
                                span { class: "composer-notice", "{notice}" }
                            }
                        }
                        {send_notice}
                    },
                }
            }
            }
            }
            if !visits.is_empty() || schedule == ui::ReadStage::Unavailable {
                ScheduleRail {
                    visits: visits.clone(),
                    title: "Upcoming visits",
                    read: schedule,
                    on_retry: on_schedule_retry,
                    empty: String::new(),
                    collapsed: schedule_collapsed,
                    on_collapsed: on_schedule_collapsed,
                    name_the_visit: true,
                    focused: visit_focus,
                    on_open: move |visit: VisitLine| {
                        let clock = format_clock(&visit.starts_at, &visit.time_zone).unwrap_or_default();
                        on_read.call((
                            format!("{} {clock}", visit.label),
                            visit.preparation.clone().unwrap_or_default(),
                        ));
                    },
                }
            }
        }
        }
    }
}

#[component]
fn SearchDialog(
    query: String,
    state: ReadView<Vec<physician_wire::PatientSearchRow>>,
    on_retry: EventHandler,
    on_query: EventHandler<String>,
    on_close: EventHandler,
    on_open: EventHandler<String>,
    on_editing: EventHandler<bool>,
    focused: Option<i32>,
) -> Element {
    let results = state.content().cloned().unwrap_or_default();
    let first = results
        .first()
        .map(|patient| patient.patient_workspace_id.clone());
    let none = !query.trim().is_empty() && state.content().is_some_and(Vec::is_empty);
    rsx! {
        div { class: "dialog-backdrop", onclick: move |_| on_close.call(()),
            section { class: "dialog", aria_label: "Find patient", onclick: move |event| event.stop_propagation(),
                h2 { "Find patient" }
                input {
                    class: "dialog-search",
                    r#type: "search",
                    placeholder: "Name, DOB or phone",
                    value: "{query}",
                    autofocus: true,
                    onfocus: move |_| on_editing.call(true),
                    onblur: move |_| on_editing.call(false),
                    oninput: move |event| on_query.call(event.value()),
                    onkeydown: move |event| {
                        if event.key().to_string() == "Enter" {
                            event.stop_propagation();
                            if let Some(id) = first.clone() {
                                on_open.call(id);
                            }
                        }
                    },
                }
                ReadNotice {
                    stage: state.stage(),
                    loading: SEARCH_LOADING,
                    stale: "",
                    unavailable: SEARCH_UNAVAILABLE,
                    retry: TRY_AGAIN,
                    on_retry,
                }
                if none {
                    EmptyState { message: SEARCH_NONE }
                }
                ul { class: "dialog-results",
                    for (index, patient) in results.iter().enumerate() {
                        li {
                            button {
                                class: if focused == Some(index as i32) { "is-keyboard-focus" } else { "" },
                                r#type: "button",
                                onclick: {
                                    let id = patient.patient_workspace_id.clone();
                                    move |_| on_open.call(id.clone())
                                },
                                span { "{patient.patient_display_name}" }
                                span { class: "search-dob", "DOB {patient.birth_date.as_deref().map(format_calendar_date).unwrap_or_else(|| \"not recorded\".to_owned())}" }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn ConnectionsDialog(lines: Vec<String>, failed: bool, on_close: EventHandler) -> Element {
    rsx! {
        div { class: "dialog-backdrop", onclick: move |_| on_close.call(()),
            section { class: "dialog", aria_label: "Connections", onclick: move |event| event.stop_propagation(),
                h2 { "{CONNECTIONS}" }
                if failed {
                    p { "{TODAY_UNAVAILABLE}" }
                } else if lines.is_empty() {
                    p { "Nothing needs you right now." }
                } else {
                    for line in lines {
                        p { class: "today-connection", "{line}" }
                    }
                }
                button { class: "control-secondary", r#type: "button", onclick: move |_| on_close.call(()), "Close" }
            }
        }
    }
}

#[derive(Clone, PartialEq, Eq)]
struct OpenReading {
    eyebrow: Option<String>,
    title: String,
    paragraphs: Vec<String>,
    provenance: Option<String>,
    unavailable: Option<String>,
    link_label: Option<String>,
    link_uri: Option<String>,
}

fn prose_reading(title: String, body: String) -> OpenReading {
    OpenReading {
        eyebrow: None,
        title,
        paragraphs: if body.is_empty() {
            Vec::new()
        } else {
            vec![body]
        },
        provenance: None,
        unavailable: None,
        link_label: None,
        link_uri: None,
    }
}

fn source_reading(chip: SourceChip) -> OpenReading {
    let inspection = citation_inspection(&chip);
    OpenReading {
        eyebrow: Some(inspection.eyebrow),
        title: inspection.title,
        paragraphs: inspection.paragraphs,
        provenance: inspection.provenance,
        unavailable: inspection.unavailable.map(str::to_owned),
        link_label: inspection.open_label,
        link_uri: inspection.open_uri,
    }
}

#[component]
fn CitationChip(chip: SourceChip, on_open: EventHandler<SourceChip>) -> Element {
    let name = chip_name(&chip);
    let index = chip.index;
    if chip.open {
        rsx! {
            button {
                class: "citation-chip",
                r#type: "button",
                aria_label: "{name}",
                onclick: move |_| on_open.call(chip.clone()),
                "[{index}]"
            }
        }
    } else {
        rsx! {
            span { class: "citation-chip", aria_label: "{name}", "[{index}]" }
        }
    }
}

#[component]
fn ReaderDialog(reading: OpenReading, on_close: EventHandler) -> Element {
    let quiet = reading.paragraphs.is_empty()
        && reading.provenance.is_none()
        && reading.link_uri.is_none()
        && reading.unavailable.is_none();
    let citation = reading.eyebrow.is_some();
    let section_class = if citation {
        "dialog reader citation-inspection"
    } else {
        "dialog reader"
    };
    let title = reading.title.clone();
    let eyebrow = reading.eyebrow.clone();
    let provenance = reading.provenance.clone();
    let unavailable = reading.unavailable.clone();
    let link_label = reading.link_label.clone();
    let link_uri = reading.link_uri.clone();
    let label = if citation { "Citation" } else { "Reader" };
    rsx! {
        div { class: "dialog-backdrop", onclick: move |_| on_close.call(()),
            section { class: "{section_class}", aria_label: "{label}", onclick: move |event| event.stop_propagation(),
                if let Some(eyebrow) = eyebrow {
                    span { class: "citation-eyebrow", "{eyebrow}" }
                }
                h2 { "{title}" }
                if quiet {
                    p { "Nothing is open to read." }
                }
                if let Some(unavailable) = unavailable {
                    p { class: "citation-unavailable", "{unavailable}" }
                }
                for paragraph in reading.paragraphs {
                    p { class: "reader-body", "{paragraph}" }
                }
                if let Some(provenance) = provenance {
                    p { class: "reader-provenance", "{provenance}" }
                }
                if let (Some(link_label), Some(link_uri)) = (link_label, link_uri) {
                    a { class: "citation-open", href: "{link_uri}", target: "_blank", rel: "noreferrer", "{link_label}" }
                }
                button { class: "control-secondary", r#type: "button", onclick: move |_| on_close.call(()), "Close" }
            }
        }
    }
}

#[component]
fn PastWorkDialog(patient: String, on_close: EventHandler) -> Element {
    rsx! {
        div { class: "dialog-backdrop", onclick: move |_| on_close.call(()),
            section {
                class: "dialog past-work-dialog",
                aria_label: "Past work",
                onclick: move |event| event.stop_propagation(),
                h2 { id: "past-work-title", "Past work" }
                if !patient.is_empty() {
                    p { class: "patient-synopsis-meta", "{patient}" }
                }
                p { class: "past-work-empty", "{PAST_WORK_EMPTY}" }
                button { class: "control-secondary", r#type: "button", onclick: move |_| on_close.call(()), "Close" }
            }
        }
    }
}

#[component]
fn FormattedMessage(text: String) -> Element {
    let blocks = format_message(&text);
    rsx! {
        div { class: "formatted-message",
            for block in blocks {
                match block {
                    MessageBlock::Paragraph(inlines) => rsx! {
                        p { class: "formatted-paragraph synopsis",
                            FormattedInlines { inlines }
                        }
                    },
                    MessageBlock::List(items) => rsx! {
                        ul { class: "formatted-list",
                            for item in items {
                                li { FormattedInlines { inlines: item } }
                            }
                        }
                    },
                }
            }
        }
    }
}

#[component]
fn FormattedInlines(inlines: Vec<Inline>) -> Element {
    rsx! {
        for inline in inlines {
            match inline {
                Inline::Text(text) => rsx! { span { "{text}" } },
                Inline::Strong(text) => rsx! { strong { "{text}" } },
                Inline::Emphasis(text) => rsx! { em { "{text}" } },
                Inline::Break => rsx! { br {} },
            }
        }
    }
}

#[component]
fn ReplyReviewCard(
    reply: WorkspaceReply,
    busy: bool,
    on_editing: EventHandler<bool>,
    on_close_reply: EventHandler<String>,
    on_revise_reply: EventHandler<(String, String)>,
) -> Element {
    let mut instruction = use_signal(String::new);
    let mut copied = use_signal(String::new);
    let actions = reply_review_actions(&reply);
    let channel = reply_channel_label(&reply.channel);
    let headline = reply.headline.clone();
    let state = reply.state.clone();
    let urgency = reply_urgency_words(&reply.urgency);
    let draft = reply.draft_text.clone();
    let reply_id = reply.reply_work_id.clone();
    let can_copy = actions.contains(&ReplyReviewAction::Copy);
    let can_close = actions.contains(&ReplyReviewAction::NoReplyNeeded);
    let can_revise = actions.contains(&ReplyReviewAction::Revise);
    let label = format!("Replying in {channel}");
    rsx! {
        article {
            class: "simulation-card reply-review",
            "data-reply-state": "{state}",
            aria_label: "{label}",
            div { class: "simulation-card-header",
                strong { "{headline}" }
                span { class: "state-pill", "{urgency}" }
            }
            p { class: "patient-synopsis-meta", "{label}" }
            FormattedMessage { text: draft.clone() }
            if can_copy || can_close {
                div { class: "reply-review-actions",
                    if can_copy {
                        button {
                            class: "sign-in-primary",
                            r#type: "button",
                            "data-reply-action": "copy",
                            disabled: busy,
                            onclick: move |_| {
                                copied.set(if copy_text(&draft) {
                                    "Copied".to_owned()
                                } else {
                                    "Couldn’t copy".to_owned()
                                });
                            },
                            "{REPLY_COPY}"
                        }
                    }
                    if can_close {
                        button {
                            class: "control-secondary",
                            r#type: "button",
                            "data-reply-action": "no-reply-needed",
                            disabled: busy,
                            onclick: {
                                let reply_id = reply_id.clone();
                                move |_| on_close_reply.call(reply_id.clone())
                            },
                            "{REPLY_NO_NEEDED}"
                        }
                    }
                    if !copied.read().is_empty() {
                        span { class: "composer-notice", role: "status", "{copied}" }
                    }
                }
            }
            if can_revise {
                div { class: "reply-review-ask",
                    input {
                        class: "reply-review-instruction",
                        r#type: "text",
                        placeholder: "{REPLY_TELL_CODA}",
                        aria_label: "{REPLY_TELL_CODA}",
                        disabled: busy,
                        value: "{instruction}",
                        onfocus: move |_| on_editing.call(true),
                        onblur: move |_| on_editing.call(false),
                        oninput: move |event| instruction.set(event.value()),
                        onkeydown: {
                            let reply_id = reply_id.clone();
                            move |event| {
                                if event.key().to_string() != "Enter" {
                                    return;
                                }
                                event.prevent_default();
                                let asked = instruction.read().trim().to_owned();
                                if asked.is_empty() || busy {
                                    return;
                                }
                                on_revise_reply.call((reply_id.clone(), asked));
                            }
                        },
                    }
                    if !instruction.read().trim().is_empty() && !busy {
                        button {
                            class: "control-secondary",
                            r#type: "button",
                            "data-reply-action": "revise",
                            aria_label: "{REPLY_ASK}",
                            onclick: {
                                let reply_id = reply_id.clone();
                                move |_| {
                                    let asked = instruction.read().trim().to_owned();
                                    if !asked.is_empty() {
                                        on_revise_reply.call((reply_id.clone(), asked));
                                    }
                                }
                            },
                            "{REPLY_ASK}"
                        }
                    }
                }
            }
        }
    }
}

fn copy_text(text: &str) -> bool {
    #[cfg(target_arch = "wasm32")]
    {
        copy_text_wasm(text)
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = text;
        false
    }
}

#[cfg(target_arch = "wasm32")]
fn copy_text_wasm(text: &str) -> bool {
    use wasm_bindgen::JsCast;
    let Some(window) = web_sys::window() else {
        return false;
    };
    let Ok(navigator) = js_sys::Reflect::get(&window, &"navigator".into()) else {
        return false;
    };
    let Ok(clipboard) = js_sys::Reflect::get(&navigator, &"clipboard".into()) else {
        return false;
    };
    if clipboard.is_null() || clipboard.is_undefined() {
        return false;
    }
    let Ok(write) = js_sys::Reflect::get(&clipboard, &"writeText".into()) else {
        return false;
    };
    let Ok(write) = write.dyn_into::<js_sys::Function>() else {
        return false;
    };
    write.call1(&clipboard, &text.into()).is_ok()
}

#[component]
fn ShortcutsDialog(on_close: EventHandler) -> Element {
    rsx! {
        div { class: "dialog-backdrop", onclick: move |_| on_close.call(()),
            section { class: "dialog", aria_label: "Keyboard shortcuts", onclick: move |event| event.stop_propagation(),
                h2 { "Keyboard shortcuts" }
                div { class: "shortcut", span { "Search your panel" } kbd { "/  ⌘ K" } }
                div { class: "shortcut", span { "Move between patients or conversations" } kbd { "J  K" } }
                div { class: "shortcut", span { "Open the focused item" } kbd { "Enter" } }
                div { class: "shortcut", span { "Update the response in progress" } kbd { "⌘ Enter" } }
                div { class: "shortcut", span { "Close the current preview or dialog" } kbd { "Esc" } }
                div { class: "shortcut", span { "Hold to record; release to transcribe" } kbd { "Ctrl+Shift+Space" } }
                p { "Navigation shortcuts pause while you are typing. Dictation works in text fields while this tab has focus. Esc cancels dictation." }
                button { class: "control-secondary", r#type: "button", onclick: move |_| on_close.call(()), "Close" }
            }
        }
    }
}

#[derive(Clone)]
enum FocusTarget {
    Row(TodayLine),
    Visit(VisitLine),
    Patient(String),
}

fn focus_targets(
    search_open: bool,
    place: Place,
    today: &TodayView,
    visits: &[VisitLine],
    search_rows: &[physician_wire::PatientSearchRow],
) -> Vec<FocusTarget> {
    if search_open {
        return search_rows
            .iter()
            .map(|row| FocusTarget::Patient(row.patient_workspace_id.clone()))
            .collect();
    }
    let mut targets = Vec::new();
    if place == Place::Workspace {
        targets.extend(visits.iter().cloned().map(FocusTarget::Visit));
        return targets;
    }
    if place == Place::Today {
        if let Some(rows) = today.content() {
            targets.extend(rows.iter().cloned().map(FocusTarget::Row));
        }
        targets.extend(visits.iter().cloned().map(FocusTarget::Visit));
    }
    targets
}

fn open_target(
    target: FocusTarget,
    selected: &mut Signal<Option<String>>,
    place: &mut Signal<Place>,
    search_open: &mut Signal<bool>,
    reading: &mut Signal<Option<OpenReading>>,
    workspace_attempt: &mut Signal<u32>,
) {
    match target {
        FocusTarget::Patient(id) => {
            selected.set(Some(id));
            place.set(Place::Workspace);
            search_open.set(false);
            workspace_attempt.set(workspace_attempt() + 1);
        }
        FocusTarget::Row(line) => {
            if opens_reader(&line.kind) {
                reading.set(Some(prose_reading(
                    line.patient.clone(),
                    line.reason.clone(),
                )));
            }
            selected.set(Some(line.workspace_id));
            place.set(Place::Workspace);
            search_open.set(false);
            workspace_attempt.set(workspace_attempt() + 1);
        }
        FocusTarget::Visit(visit) => {
            let Some(id) = visit.workspace_id.clone() else {
                return;
            };
            let clock = format_clock(&visit.starts_at, &visit.time_zone).unwrap_or_default();
            reading.set(Some(prose_reading(
                format!("{} {clock}", visit.patient),
                visit.preparation.clone().unwrap_or_default(),
            )));
            selected.set(Some(id));
            place.set(Place::Workspace);
            search_open.set(false);
            workspace_attempt.set(workspace_attempt() + 1);
        }
    }
}

/// The message the physician wrote, as one command. Nothing goes without a
/// patient, a conversation and the practice's time zone.
fn message_command(
    workspace_id: Option<String>,
    chat_id: Option<String>,
    time_zone: Option<String>,
    text: String,
) -> Result<Command, String> {
    let (Some(workspace_id), Some(chat_id), Some(time_zone)) = (workspace_id, chat_id, time_zone)
    else {
        return Err(MESSAGE_REFUSED.to_owned());
    };
    Ok(Command::new(
        "message.submit",
        serde_json::json!({
            "patient_workspace_id": workspace_id,
            "chat_id": chat_id,
            "text": text,
            "physician_time_zone": time_zone,
            "intent": "queue",
        }),
    ))
}

/// Try again on one piece of work. Its key names the run it resumes, so
/// pressing again for the same run is the same request.
fn resume_command(line: &TodayLine, time_zone: &str) -> Option<Command> {
    let (work, run) = (
        line.resume_work_item_id.as_ref()?,
        line.resume_run_id.as_ref()?,
    );
    Some(Command::keyed(
        "work.resume",
        format!("resume-{work}-{run}"),
        serde_json::json!({
            "patient_workspace_id": line.workspace_id,
            "work_item_id": work,
            "expected_run_id": run,
            "physician_time_zone": time_zone,
        }),
    ))
}

fn idempotency_key() -> String {
    static NEXT: AtomicU64 = AtomicU64::new(1);
    let n = NEXT.fetch_add(1, Ordering::Relaxed);
    let millis = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_millis())
        .unwrap_or(0);
    format!("submit-{millis}-{n}")
}
