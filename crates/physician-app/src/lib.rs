//! Physician shell: the shipping sidebar, Today, a patient page, and the reader.
//! The React app remains the shipping client. This client renders the same projections.
//! Book: `book/src/physician-site.md`.
#![deny(missing_docs)]
#![allow(
    clippy::map_err_ignore,
    reason = "converting to errors that keep their cause"
)]

mod composer;
mod feedback;
mod local_draft;
mod panels;
mod shell;
mod status;
mod tabs;

use std::sync::atomic::{AtomicU64, Ordering};

use composer::{Composer, ConflictDraft, DraftTarget, Notice, SaveOutcome, SendAnswer};
use dioxus::prelude::*;
use physician_client::{DraftSave, PhysicianClient};
use physician_wire::{
    ChatRunOutcome, ChatSnapshot, HoldReason, PatientWorkspaceResponse, PracticeAccountSummary,
    RecentConversation, ResponseId, SignInResponse, SnapshotFeedback, WhoamiResponse,
    WorkResumeTarget, WorkStopTarget, WorkspaceReply,
};

use panels::Panel;
use shell::{
    CONVERSATION_EMPTY, ChatView, Drawer, KeyInput, NavCommand, PAST_WORK_EMPTY, REPLY_ASK,
    REPLY_CLOSED_NOTICE, REPLY_COPY, REPLY_NO_NEEDED, REPLY_REVISION_STALE, REPLY_TELL_CODA,
    RESUME, ReadView, ReplyReviewAction, STOP, SourceChip, SynopsisPiece, TODAY_LOADING,
    TODAY_QUIET, TODAY_QUIET_DETAIL, TODAY_STALE, TODAY_UNAVAILABLE, TRY_AGAIN, TodayLine,
    TodayView, VisitLine, WORKSPACE_CLOSED, WORKSPACE_EMPTY, WORKSPACE_UNAVAILABLE, answer_to,
    chat_view, chip_name, citation_inspection, connection_lines, demographic_card_view,
    format_calendar_date, format_clock, format_local_date, format_waited, keyboard_command,
    need_you_words, opens_reader, panel_target, past_work_copy, patient_upcoming_visits, read_view,
    records_time, reply_channel_label, reply_review_actions, reply_revision_words,
    reply_urgency_words, row_icon, row_tone, schedule_empty_copy, shows_today_schedule,
    summary_times, synopsis_pieces, today_lines, today_view, upcoming_visits, work_end_reason,
    work_heading,
};
#[cfg(target_arch = "wasm32")]
use shell::{EventRefetch, event_refetch};
use status::{CommandNotice, CommandStage, ReadNotice, ReadStage};
use ui::{
    ActionRow, Author, Button, ButtonVariant, Card, Count, Dialog, DraftConflict, EmptyLine, Icon,
    IconName, LiveDot, NavigationControl, NavigationRow, PageTitle, ProgressBar, SearchField,
    StatePill, StatusLine, TextField, TextKind, Toast, Tone,
};

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
/// The React shell's `new_conversation` command label.
const NEW_CONVERSATION: &str = "New conversation";
const ARCHIVE: &str = "Archive";
const ARCHIVED: &str = "Archived";
const ARCHIVED_TOAST: &str = "Conversation archived.";
const ARCHIVED_LINE: &str = "This conversation is archived. Sending a message brings it back.";
const UNDO: &str = "Undo";
const NOT_SAVED: &str = "Not saved";
const DRAFT_CONFLICT: &str = "A newer version of this draft was saved elsewhere.";
const DRAFT_CONFLICT_HELP: &str =
    "Retry replaces the saved draft with your text. Use newer text discards your edit.";
const CLEAR_CONFLICT_HELP: &str = "Your message was sent. Retry replaces the newer draft with what is left here; Use newer text keeps the newer draft.";
const CLEAR_NOT_SAVED: &str = "Message sent. Coda couldn’t clear the saved draft yet.";
const DRAFT_RETRY: &str = "Retry";
const DRAFT_USE_NEWER: &str = "Use newer text";
const RECOVERY_UNAVAILABLE: &str =
    "This tab can’t keep a reload copy of your draft. Keep the page open until it saves.";
const CONVERSATION_UNAVAILABLE: &str = "Coda couldn’t open this conversation.";
const CONVERSATION_STALE: &str = "Coda couldn’t refresh this conversation. Showing the last one.";
const SEND: &str = "Send";
/// Send while Coda is writing: the message waits for the running answer.
const SEND_NEXT: &str = "Send next";
const QUEUED_WAITING: &str = "Waiting for next response";
const QUEUED_EDIT: &str = "Edit";
const QUEUED_REMOVE: &str = "Remove";
const QUEUED_EDIT_LABEL: &str = "Queued message";
const QUEUED_SAVE: &str = "Save";
const QUEUED_CANCEL: &str = "Cancel";
const DICTATION_UNAVAILABLE: &str =
    "Dictation is unavailable. Check microphone support and server configuration.";
const CONNECTIONS: &str = "Connections";
const SIGN_IN_UNAVAILABLE: &str =
    "Coda couldn’t confirm the request. Check your inbox before trying again.";
const SIGN_IN_INVALID: &str = "That sign-in link is no longer valid. Request a new one.";
const SIGN_IN_UNCONFIRMED: &str =
    "Coda couldn’t confirm sign-in. Open your email link again or request a new one.";
const NO_ACCESS: &str = "This sign-in link no longer gives you access to this practice.";
const ACCESS_REMOVED: &str = "The practice removed your access, so you've been signed out.";
const SESSION_ENDED: &str = "Your session ended. Sign in to continue.";
const SHOW_SESSION_PROMPT: &str = "document.getElementById('session-prompt')?.showModal()";
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

/// The session belongs to this browser, so it lives in localStorage: a new tab,
/// or Coda reopened, opens the workspace while the session lasts. Beside it
/// is the email it was signed in with, so a sign-in link can tell whether it
/// is yours without asking the server, and how many times this browser has
/// signed out, so a tab that catches up late knows a sign-out came between.
#[cfg(target_arch = "wasm32")]
const SESSION_STORAGE_KEY: &str = "coda.physician.session";
#[cfg(target_arch = "wasm32")]
const SESSION_OWNER_KEY: &str = "coda.physician.session.email";
#[cfg(target_arch = "wasm32")]
const SIGN_OUTS_KEY: &str = "coda.physician.sign_outs";

/// Calls `method` on this browser's localStorage. Denied or missing storage is `None`.
#[cfg(target_arch = "wasm32")]
fn local_storage(method: &str, args: &[&str]) -> Option<wasm_bindgen::JsValue> {
    use wasm_bindgen::JsCast;
    let window = web_sys::window()?;
    let storage = js_sys::Reflect::get(&window, &"localStorage".into()).ok()?;
    if storage.is_null() || storage.is_undefined() {
        return None;
    }
    let function = js_sys::Reflect::get(&storage, &method.into())
        .ok()?
        .dyn_into::<js_sys::Function>()
        .ok()?;
    let args: js_sys::Array = args
        .iter()
        .map(|arg| wasm_bindgen::JsValue::from_str(arg))
        .collect();
    function.apply(&storage, &args).ok()
}

#[cfg(target_arch = "wasm32")]
fn stored(key: &str) -> Option<String> {
    local_storage("getItem", &[key])?
        .as_string()
        .filter(|value| !value.trim().is_empty())
}

#[cfg(target_arch = "wasm32")]
fn read_stored_session() -> Option<String> {
    stored(SESSION_STORAGE_KEY)
}

/// Whether this browser still stores `session`.
#[cfg(target_arch = "wasm32")]
fn still_stored(session: &str) -> bool {
    read_stored_session().as_deref() == Some(session)
}

#[cfg(not(target_arch = "wasm32"))]
fn still_stored(_session: &str) -> bool {
    true
}

/// The email the stored session was signed in with.
#[cfg(target_arch = "wasm32")]
fn read_stored_owner() -> Option<String> {
    stored(SESSION_OWNER_KEY)
}

#[cfg(not(target_arch = "wasm32"))]
fn read_stored_owner() -> Option<String> {
    None
}

#[cfg(target_arch = "wasm32")]
fn store_session(session: &str, email: &str) {
    local_storage("setItem", &[SESSION_STORAGE_KEY, session]);
    local_storage("setItem", &[SESSION_OWNER_KEY, email.trim()]);
}

/// Forgets `session` if this browser still stores it; a newer one stays.
#[cfg(target_arch = "wasm32")]
fn forget_stored_session(session: &str) {
    if read_stored_session().as_deref() == Some(session) {
        local_storage("removeItem", &[SESSION_STORAGE_KEY]);
        local_storage("removeItem", &[SESSION_OWNER_KEY]);
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn forget_stored_session(_session: &str) {}

/// How many times this browser has signed out.
#[cfg(target_arch = "wasm32")]
fn read_sign_outs() -> u64 {
    stored(SIGN_OUTS_KEY)
        .and_then(|count| count.parse().ok())
        .unwrap_or(0)
}

#[cfg(not(target_arch = "wasm32"))]
fn read_sign_outs() -> u64 {
    0
}

#[cfg(target_arch = "wasm32")]
fn count_sign_out() {
    let count = (read_sign_outs() + 1).to_string();
    local_storage("setItem", &[SIGN_OUTS_KEY, &count]);
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
) -> dioxus::core::Task {
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
            // Subscribe before rereading: even the first connection can miss
            // changes made after the page's initial snapshot. Events invalidate
            // projections; authoritative reads reconcile every connection gap.
            today_attempt.set(today_attempt() + 1);
            workspace_attempt.set(workspace_attempt() + 1);
            snapshot_attempt.set(snapshot_attempt() + 1);
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
                    &mut today_attempt,
                    &mut workspace_attempt,
                    &mut snapshot_attempt,
                );
            }
            next_delay = retry.next_delay_ms(Some(clock.now() - started_at));
        }
    })
}

#[cfg(target_arch = "wasm32")]
fn apply_event_frames(
    buffer: &mut String,
    selected: &Signal<Option<String>>,
    today_attempt: &mut Signal<u32>,
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
        match event_refetch(&name, &data, open.as_deref()) {
            EventRefetch::Nothing => {}
            EventRefetch::OpenPatient => {
                workspace_attempt.set(workspace_attempt() + 1);
                snapshot_attempt.set(snapshot_attempt() + 1);
            }
            // The server missed events; reread every projection once.
            EventRefetch::Everything => {
                today_attempt.set(today_attempt() + 1);
                workspace_attempt.set(workspace_attempt() + 1);
                snapshot_attempt.set(snapshot_attempt() + 1);
            }
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
/// Every change names the bearer it is about, because a request can finish
/// after this browser has moved on to a newer session.
#[derive(Clone, Copy, PartialEq)]
struct SessionState {
    /// Presents this tab's bearer on every read.
    client: Signal<PhysicianClient>,
    signed_in: Signal<bool>,
    /// The sign-in prompt replaces the workspace when the session expires.
    /// Patient content is cleared before the prompt is shown.
    expired: Signal<bool>,
    whoami_name: Signal<String>,
    whoami: Signal<Option<WhoamiResponse>>,
    /// Whose session this tab holds: the email the browser stored beside it
    /// when the tab took it, or the link's email when the tab signed in.
    owner: Signal<Option<String>>,
    /// Bumped when another physician signs in here: the shell starts over at Today.
    generation: Signal<u32>,
    /// Bumped when this tab takes a newer session of yours in place: the shell
    /// reads with it.
    taken: Signal<u32>,
    /// The browser's sign-out count when this tab last caught up; another
    /// count means a sign-out came between.
    sign_outs_seen: Signal<u64>,
    /// The patient and conversation the page was opened at. The shell opens
    /// there once someone signs in; sign-out and another physician's sign-in
    /// forget them.
    held_link: Signal<(String, String)>,
    /// The mailed link this page was opened at, until this tab is signed in,
    /// by the link or by following another tab.
    sign_in_link: Signal<Option<SignInLink>>,
    /// The practice removed this physician's access; the sign-in page says so.
    removed: Signal<bool>,
}

impl SessionState {
    /// The bearer this tab presents now.
    fn bearer(self) -> Option<String> {
        self.client.peek().session().get()
    }

    /// Back to the sign-in page, forgetting where this tab was.
    fn sign_out(self, ended: Option<&str>) {
        self.forget_place();
        self.no_session(ended);
    }

    /// Back to the sign-in page, still holding the link the page was opened
    /// at. `ended` leaves this tab and the browser, unless a newer session
    /// already replaced it.
    fn no_session(self, ended: Option<&str>) {
        if let Some(ended) = ended {
            self.client.peek().session().clear_if(ended);
            forget_stored_session(ended);
        }
        self.no_session_here();
    }

    /// Drops patient content this tab was showing: the held link, unsent drafts,
    /// and stored panel choices. The shell starts at Today if it is shown again.
    fn drop_patient_content(mut self) {
        panels::forget_all();
        local_draft::forget_everything();
        self.forget_place();
        *self.generation.write() += 1;
    }

    /// Back to the sign-in page in this tab only. A tab that never showed the
    /// workspace keeps the patient link it was opened at.
    fn no_session_here(mut self) {
        panels::forget_all();
        // No old account's unsent text outlives its session in this tab.
        local_draft::forget_everything();
        self.expired.set(false);
        self.removed.set(false);
        self.signed_in.set(false);
        self.whoami_name.set(String::new());
        self.whoami.set(None);
        self.owner.set(None);
    }

    /// Drops the held patient link, from this tab and its address.
    fn forget_place(mut self) {
        self.held_link.set(Default::default());
        write_address(None, None);
    }

    /// Signs this browser out: forgets the session it stores, or this tab's
    /// own when it stores none, counts the sign-out so a tab that catches up
    /// late knows one came between, and tells every other tab. Coda is asked
    /// to end that session, and nothing waits on its answer. This tab forgets
    /// where it was and shows the sign-in page.
    fn end(self) {
        let client = self.client.peek().clone();
        #[cfg(target_arch = "wasm32")]
        let ending = read_stored_session().or_else(|| client.session().get());
        #[cfg(not(target_arch = "wasm32"))]
        let ending = client.session().get();
        #[cfg(target_arch = "wasm32")]
        {
            if let Some(ending) = &ending {
                forget_stored_session(ending);
            }
            count_sign_out();
            let mut seen = self.sign_outs_seen;
            seen.set(read_sign_outs());
            tabs::announce();
        }
        client.session().clear();
        if let Some(ending) = ending.clone() {
            // The root owns the request, so no screen change drops it.
            dioxus::core::spawn_forever(async move {
                let _ = client.sign_out(&ending).await;
            });
        }
        self.sign_out(ending.as_deref());
    }

    /// A read made with `presented` was refused. A refusal of a session this
    /// tab no longer holds is about the past and changes nothing. An ended
    /// session covers a page this tab already showed with the sign-in prompt;
    /// a tab that showed nothing yet was opened without a session, so it shows
    /// the sign-in page and keeps the patient link it was opened at. Either
    /// way the browser keeps storing the ended session: only a sign-out
    /// forgets it, so a tab that later finds nothing stored knows one came.
    /// Removed access signs out and says "No access to this practice".
    fn refused(mut self, refusal: Refusal, shown: bool, presented: Option<String>) {
        if self.bearer() != presented {
            return;
        }
        let presented = presented.as_deref();
        match refusal {
            Refusal::SessionEnded if shown => {
                self.drop_patient_content();
                self.expired.set(true);
            }
            Refusal::SessionEnded => {
                if let Some(presented) = presented {
                    self.client.peek().session().clear_if(presented);
                }
                self.no_session_here();
            }
            Refusal::AccessRemoved => {
                self.sign_out(presented);
                self.removed.set(true);
            }
        }
    }

    /// Follows the session this browser stores. Another tab's news only says
    /// that it changed, so late or reordered news can never move this tab to
    /// an older session. With nothing stored, a signed-in tab signs out,
    /// keeping its place. A signed-out tab, which a workspace still opening
    /// with no session is too, takes the stored session and keeps the patient
    /// link it was opened at. Your own tab takes it where it was, reading with
    /// it at once. Only a sign-out that came between makes a tab forget where
    /// it was: a waiting tab opens at Today, and a signed-in one, opening or
    /// not, like a tab that was another physician's, starts over at Today.
    #[cfg(target_arch = "wasm32")]
    fn catch_up(mut self) {
        let client = self.client.peek().clone();
        let signed_in = *self.signed_in.peek();
        let sign_outs = read_sign_outs();
        let missed_a_sign_out = sign_outs != *self.sign_outs_seen.peek();
        self.sign_outs_seen.set(sign_outs);
        if missed_a_sign_out {
            self.forget_place();
        }
        let Some(stored) = read_stored_session() else {
            client.session().clear();
            if signed_in {
                self.no_session_here();
            }
            return;
        };
        if signed_in
            && !missed_a_sign_out
            && client.session().get().as_deref() == Some(stored.as_str())
        {
            return;
        }
        let opening_signed_out = client.session().get().is_none();
        client.session().set(stored);
        let owner = read_stored_owner();
        if !signed_in {
            self.sign_in_link.set(None);
            self.owner.set(owner);
            self.signed_in.set(true);
            return;
        }
        if opening_signed_out && !missed_a_sign_out {
            // The workspace reads again with the session it took.
            self.owner.set(owner);
            *self.taken.write() += 1;
            return;
        }
        let yours = owner.as_ref().is_some_and(|owner| {
            self.owner
                .peek()
                .as_ref()
                .is_some_and(|mine| mine.eq_ignore_ascii_case(owner))
        });
        if yours && !missed_a_sign_out {
            // The session is yours again. Patient content was cleared when it
            // ended, so the workspace opens at Today. A newer session of yours
            // re-reads in place.
            if *self.expired.peek() {
                self.expired.set(false);
            } else {
                *self.taken.write() += 1;
            }
            return;
        }
        // Someone this tab can't name as you, or you after a sign-out: the tab
        // starts at Today and nothing from before carries over.
        self.drop_patient_content();
        self.whoami_name.set(String::new());
        self.whoami.set(None);
        self.owner.set(owner);
        self.expired.set(false);
    }
}

/// Signing out ends this session and reaches this browser's other Coda tabs.
fn sign_out_everywhere(session: SessionState) {
    session.end();
}

#[component]
pub fn PhysicianRoot(
    base_url: String,
    open_patient: String,
    /// A conversation of `open_patient` to open first; empty opens the first listed.
    #[props(default)]
    open_chat: String,
    sign_in: Option<SignInLink>,
) -> Element {
    // A mailed link goes through the sign-in screen even when this browser
    // already has a session. Otherwise the workspace opens, and its first
    // `/today` read handles the auth gate.
    let mut signed_in = use_signal(|| sign_in.is_none());
    let mut whoami_name = use_signal(String::new);
    let mut whoami = use_signal(|| None::<WhoamiResponse>);
    #[cfg(target_arch = "wasm32")]
    let owner = use_signal(|| read_stored_session().and_then(|_| read_stored_owner()));
    #[cfg(not(target_arch = "wasm32"))]
    let owner = use_signal(|| None::<String>);
    let expired = use_signal(|| false);
    let generation = use_signal(|| 0u32);
    let taken = use_signal(|| 0u32);
    let sign_outs_seen = use_signal(read_sign_outs);
    let held_link = use_signal(|| (open_patient.clone(), open_chat.clone()));
    let mut removed = use_signal(|| false);
    let mut sign_in_link = use_signal(|| sign_in);
    // The bearer session rides the client, so one client is built once.
    let client = use_signal(|| {
        let client = PhysicianClient::new(base_url.clone());
        #[cfg(target_arch = "wasm32")]
        if let Some(session) = read_stored_session() {
            client.session().set(session);
        }
        client
    });
    let session = SessionState {
        client,
        signed_in,
        expired,
        whoami_name,
        whoami,
        owner,
        generation,
        taken,
        sign_outs_seen,
        held_link,
        sign_in_link,
        removed,
    };
    // Other tabs' sign-in news arrives outside the Dioxus runtime, so it
    // re-enters this scope before touching the session.
    #[cfg(target_arch = "wasm32")]
    use_hook(move || {
        let runtime = dioxus::core::Runtime::current();
        let scope = dioxus::core::current_scope_id();
        std::rc::Rc::new(tabs::listen(move || {
            runtime.in_scope(scope, || session.catch_up());
        }))
    });
    if !signed_in() {
        rsx! {
            ui::Stylesheet { app: SHELL_STYLES }
            SignInScreen {
                client,
                session,
                link: sign_in_link(),
                on_signed_in: move |who: Option<WhoamiResponse>| {
                    sign_in_link.set(None);
                    removed.set(false);
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
    } else if expired() {
        // The prompt replaces the workspace. PhysicianShell is not mounted, so
        // no patient view stays in the page.
        let mut expired = expired;
        rsx! {
            ui::Stylesheet { app: SHELL_STYLES }
            dialog {
                id: "session-prompt",
                class: "session-prompt",
                aria_label: "Sign in to continue",
                "closedby": "none",
                onmounted: move |_| {
                    document::eval(SHOW_SESSION_PROMPT);
                },
                SignInScreen {
                    client,
                    session,
                    link: None,
                    resume_email: whoami.read().as_ref().map(|account| account.email.clone()),
                    on_signed_in: move |_| expired.set(false),
                    on_sign_out: move |_| sign_out_everywhere(session),
                }
            }
        }
    } else {
        // A different physician's sign-in starts a fresh shell at Today. A
        // keyed list replaces the shell when the key changes; a lone keyed
        // component is diffed in place and keeps its state.
        let shell = generation();
        let (open_patient, open_chat) = held_link();
        rsx! {
            for shell in std::iter::once(shell) {
                PhysicianShell {
                    key: "{shell}",
                    base_url: base_url.clone(),
                    open_patient: open_patient.clone(),
                    open_chat: open_chat.clone(),
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
    open_chat: String,
    client: Signal<PhysicianClient>,
    session: SessionState,
) -> Element {
    // The address's conversation belongs to the address's patient only.
    let mut requested_chat = use_signal(|| {
        (!open_patient.is_empty() && !open_chat.is_empty())
            .then(|| (open_patient.clone(), open_chat.clone()))
    });
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
    let mut drawer = use_signal(|| Drawer::Closed);
    let mut drawer_menu = use_signal(|| None::<std::rc::Rc<MountedData>>);
    let mut drawer_close = use_signal(|| None::<std::rc::Rc<MountedData>>);
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
    // Focus goes into the drawer as it opens, and back to the menu when it's dismissed, or once a
    // dialog opened from it closes, since that dialog returns focus to the hidden row it came from.
    use_effect(move || {
        let dialog_open = search_open() || shortcuts_open() || connections_open();
        let control = match drawer() {
            Drawer::Open => drawer_close.peek().clone(),
            Drawer::Dismissed if !dialog_open => {
                drawer.set(Drawer::Closed);
                drawer_menu.peek().clone()
            }
            Drawer::Closed | Drawer::Dismissed => None,
        };
        if let Some(control) = control {
            spawn(async move {
                let _ = control.set_focus(true).await;
            });
        }
    });
    // Draft text and the Save mark, per patient workspace and chat.
    let mut composer = use_signal(Composer::default);
    let mut reading = use_signal(|| None::<OpenReading>);
    let mut focus = use_signal(|| -1i32);
    let mut open_chat = use_signal(|| None::<String>);
    let mut moved = use_signal(|| false);
    let mut editing = use_signal(|| false);
    let mut query = use_signal(String::new);
    let mut today_attempt = use_signal(|| 0u32);
    let mut workspace_attempt = use_signal(|| 0u32);
    let mut snapshot_attempt = use_signal(|| 0u32);
    let mut feedback_desk = feedback::use_feedback(client, session, shown, selected, open_chat);
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
    // Whose tab this is, for what a reload of it may ask again.
    let command_scope = use_memo(move || {
        let account = whoami.read();
        let account = account.as_ref()?;
        local_draft::Scope::new(
            client.read().base(),
            &account.practice_id,
            &account.physician_id,
        )
    });
    let reply_slot = use_command_slot("reply", command_scope);
    // A Send's command is kept with its draft, and re-asked from there.
    let send_slot = use_unkept_command_slot();
    let resume_slot = use_command_slot("resume", command_scope);
    let chat_resume_slot = use_command_slot("chat-resume", command_scope);
    let chat_stop_slot = use_command_slot("chat-stop", command_scope);
    let queued_slot = use_command_slot("queued", command_scope);
    // The queued message being edited, and its words so far.
    let mut queued_editing = use_signal(|| None::<(String, String)>);
    let work_slot = use_command_slot("work", command_scope);
    let archive_slot = use_command_slot("archive", command_scope);
    let restore_slot = use_command_slot("restore", command_scope);
    // The Undo toast for the latest Archive, keyed by its count: its patient and chat.
    let mut archived_toast = use_signal(|| None::<(u32, String, String)>);
    let mut archives = use_signal(|| 0u32);
    // An accepted Archive, clicked or recovered after a reload, offers Undo.
    let mut offer_undo = move |workspace: String, chat: String| {
        *workspace_attempt.write() += 1;
        let shown = archives() + 1;
        archives.set(shown);
        archived_toast.set(Some((shown, workspace, chat)));
    };
    // Commands a reload interrupted are asked again under their own keys
    // once this physician is known; every read follows whatever they changed.
    #[cfg(target_arch = "wasm32")]
    {
        let mut asked = use_signal(|| false);
        use_effect(move || {
            if command_scope.read().is_none() || *asked.peek() {
                return;
            }
            asked.set(true);
            let settled = Callback::new(move |()| {
                *today_attempt.write() += 1;
                *workspace_attempt.write() += 1;
                *snapshot_attempt.write() += 1;
            });
            for slot in [
                reply_slot,
                resume_slot,
                chat_resume_slot,
                chat_stop_slot,
                queued_slot,
                work_slot,
                restore_slot,
            ] {
                slot.ask_kept(client.peek().clone(), settled);
            }
            if let Some(command) = archive_slot.kept_command() {
                let target = command.archive_target();
                let accepted = Callback::new(move |()| {
                    settled.call(());
                    if let Some((workspace, chat)) = target.clone() {
                        offer_undo(workspace, chat);
                    }
                });
                archive_slot.send(client.peek().clone(), command, accepted, Some(settled));
            }
        });
    }
    use_effect(move || {
        let open = workspace_cache.read().clone();
        let showing = place();
        let allowed =
            showing == Place::Workspace && open.as_ref().and_then(past_work_copy).is_some();
        if !allowed && past_work_open() {
            past_work_open.set(false);
        }
    });
    // Taking a newer session of yours from another tab re-reads the page in
    // place, and the stream restarts on the session now held. The prompt
    // unmounts the shell, so signing in from it opens a fresh one at Today.
    let taken = session.taken;
    let mut was_expired = use_signal(|| false);
    let mut read_with = use_signal(|| *taken.peek());
    use_effect(move || {
        let now_expired = expired();
        let now_taken = taken();
        if (*was_expired.peek() && !now_expired) || *read_with.peek() != now_taken {
            *today_attempt.write() += 1;
            *workspace_attempt.write() += 1;
            *snapshot_attempt.write() += 1;
        }
        was_expired.set(now_expired);
        read_with.set(now_taken);
    });
    #[cfg(target_arch = "wasm32")]
    let mut stream = use_signal(|| None::<dioxus::core::Task>);
    #[cfg(target_arch = "wasm32")]
    use_effect(move || {
        let _taken = taken();
        if let Some(task) = stream.write().take() {
            task.cancel();
        }
        if expired() {
            return;
        }
        let session = client.read().session().get();
        let task = follow_workspace_events(
            &base_url,
            session,
            today_attempt,
            workspace_attempt,
            snapshot_attempt,
            selected,
        );
        stream.set(Some(task));
    });
    let _whoami_load = use_resource(move || {
        let _attempt = today_attempt();
        let client = client.read().clone();
        async move {
            if let Ok(account) = client.whoami().await {
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
        let presented = client.session().get();
        async move {
            match client.today().await {
                Ok(response) => {
                    shown.set(true);
                    Ok(response)
                }
                Err(error) => {
                    if let Some(refused) = refusal(&error) {
                        session.refused(refused, *shown.peek(), presented);
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
        let presented = client.session().get();
        let open_id = selected();
        async move {
            let answer = match open_id.clone() {
                Some(id) => match client.workspace(&id).await {
                    Ok(open) => {
                        if open.is_clinic_demographic() {
                            forget_workspace_drafts(composer, &client, whoami, &id);
                        }
                        shown.set(true);
                        gone.set(None);
                        Ok(Some(open))
                    }
                    // A covered page keeps what it showed; signing out leaves
                    // the shell, so nothing of it stays either way.
                    Err(error) => {
                        if let Some(refused) = refusal(&error) {
                            session.refused(refused, *shown.peek(), presented);
                        } else if matches!(
                            error,
                            physician_client::HttpError::Status { status: 404, .. }
                        ) {
                            workspace_cache.set(None);
                            // No access (or no patient): its unsent text is
                            // not kept for a later reload or account.
                            forget_workspace_drafts(composer, &client, whoami, &id);
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
    // Worker completion announces `workspace_changed`, and a reconnect resyncs.
    // Should both be missed while a reply revision runs, reread the workspace
    // on a bounded schedule. Only the newest schedule may fire.
    let mut revision_poll = use_signal(|| (0u64, 0u32));
    use_effect(move || {
        let running = workspace_cache.read().as_ref().is_some_and(|open| {
            open.replies
                .iter()
                .any(|reply| reply.revision_run.as_ref().is_some_and(|run| run.running()))
        });
        let (generation, polls) = *revision_poll.peek();
        let generation = generation + 1;
        if !running {
            revision_poll.set((generation, 0));
            return;
        }
        if polls >= REVISION_POLL_LIMIT {
            revision_poll.set((generation, polls));
            return;
        }
        revision_poll.set((generation, polls + 1));
        spawn(async move {
            sleep_ms(REVISION_POLL_MS).await;
            if revision_poll.peek().0 == generation {
                *workspace_attempt.write() += 1;
            }
        });
    });
    use_effect(move || {
        let current = selected();
        let cached = workspace_cache.read().clone();
        let patient = if place() == Place::Workspace {
            current.as_deref()
        } else {
            None
        };
        let requested = requested_chat
            .read()
            .as_ref()
            .filter(|(patient, _)| Some(patient.as_str()) == current.as_deref())
            .map(|(_, chat)| chat.clone());
        let same = cached
            .as_ref()
            .is_some_and(|open| Some(open.patient_workspace_id.as_str()) == current.as_deref());
        if !same {
            if open_chat.read().is_some() {
                open_chat.set(None);
            }
            // Keep an initial deep link while its patient loads, but never
            // pair the previous patient's conversation with a new selection.
            write_address(patient, requested.as_deref());
            return;
        }
        if open_chat.read().is_none()
            && let Some(chat_id) = requested.or_else(|| {
                cached.and_then(|open| open.chats.first().map(|chat| chat.chat_id.clone()))
            })
        {
            open_chat.set(Some(chat_id));
            requested_chat.set(None);
        }
        write_address(patient, open_chat.read().as_deref());
    });
    // The draft the composer shows now: the open patient's chat, or `draft`
    // before a chat is open. Nothing outside a patient page.
    let draft_target = move || {
        let workspace = if place() == Place::Workspace {
            selected()
        } else {
            None
        };
        workspace.map(|workspace| DraftTarget::new(workspace, open_chat().as_deref()))
    };
    // The draft target left behind saves now instead of waiting for its
    // debounce.
    let mut left_target = use_signal(|| None::<DraftTarget>);
    use_effect(move || {
        let target = draft_target();
        let previous = left_target.peek().clone();
        if previous != target {
            left_target.set(target.clone());
            if let Some(previous) = previous {
                autosave(composer, client, previous);
            }
        }
        composer.write().open(target.as_ref());
    });
    // Opening a target reads its saved draft and the revision a save names.
    // A failed read changes nothing: it never stands for "no draft".
    // A snapshot refresh rereads it too: Stop returns queued messages to
    // this draft, in this tab or another.
    let _draft_read = use_resource(move || {
        let _attempt = snapshot_attempt();
        let target = draft_target();
        let reader = client.read().pinned();
        async move {
            let Some(target) = target else { return };
            if let Ok(saved) = reader.read_draft(&target.workspace, &target.chat).await {
                composer
                    .write()
                    .loaded(&target, saved.revision, &saved.text);
                autosave_after(composer, client, target, AUTOSAVE_DEBOUNCE_MS);
            }
        }
    });
    // Same-tab recovery: once this physician's identity is known and the
    // workspace was read for them, a reloaded tab puts back its unsent text.
    #[cfg(target_arch = "wasm32")]
    let writer = use_hook(writer_id);
    #[cfg(target_arch = "wasm32")]
    let mut restored = use_signal(std::collections::HashSet::<DraftTarget>::new);
    #[cfg_attr(not(target_arch = "wasm32"), allow(unused_mut))]
    let mut recovery_unavailable = use_signal(|| false);
    #[cfg(target_arch = "wasm32")]
    {
        use_effect(move || {
            let Some(target) = draft_target() else { return };
            let Some(account) = whoami.read().clone() else {
                return;
            };
            let open = workspace_cache.read().clone();
            let validated = open.as_ref().is_some_and(|open| {
                open.patient_workspace_id == target.workspace && !open.is_clinic_demographic()
            });
            if open.as_ref().is_some_and(|open| {
                open.patient_workspace_id == target.workspace && open.is_clinic_demographic()
            }) {
                // whoami may arrive after the demographic-only workspace.
                // Purge again once the storage scope is known, never restore.
                forget_workspace_drafts(composer, &client.peek(), whoami, &target.workspace);
                return;
            }
            if !validated || restored.peek().contains(&target) {
                return;
            }
            restored.write().insert(target.clone());
            let base = client.peek().base().to_owned();
            let Some(scope) =
                local_draft::Scope::new(&base, &account.practice_id, &account.physician_id)
            else {
                return;
            };
            let Some(storage) = local_draft::SessionStorage::open() else {
                return;
            };
            let Some(record) = local_draft::read(&storage, &scope, &target) else {
                return;
            };
            if composer.write().restore(&target, record) {
                // A Send whose answer was lost before the reload: ask again
                // with the same command and key, never a new one.
                let pending = composer.peek().pending_send(&target);
                if let Some(pending) = pending {
                    dispatch_send(
                        send_slot,
                        client,
                        composer,
                        snapshot_attempt,
                        target.clone(),
                        pending,
                    );
                }
                autosave_after(composer, client, target, AUTOSAVE_DEBOUNCE_MS);
            }
        });
    }
    #[cfg(target_arch = "wasm32")]
    {
        let written = use_hook(|| {
            std::rc::Rc::new(std::cell::RefCell::new(std::collections::HashMap::<
                DraftTarget,
                Option<composer::Recovery>,
            >::new()))
        });
        use_effect(move || {
            let held = composer.read();
            let restored = restored.read();
            let Some(account) = whoami.read().clone() else {
                return;
            };
            let base = client.peek().base().to_owned();
            let Some(scope) =
                local_draft::Scope::new(&base, &account.practice_id, &account.physician_id)
            else {
                return;
            };
            let storage = local_draft::SessionStorage::open();
            let mut failed = false;
            for target in restored.iter() {
                let record = held.recovery(target);
                if written.borrow().get(target) == Some(&record) {
                    continue;
                }
                let kept = match &storage {
                    Some(storage) => local_draft::write(
                        storage,
                        &scope,
                        target,
                        &writer,
                        record.as_ref(),
                        js_sys::Date::now(),
                    )
                    .is_ok(),
                    None => record.is_none(),
                };
                if kept {
                    written.borrow_mut().insert(target.clone(), record);
                } else {
                    failed = true;
                }
            }
            if *recovery_unavailable.peek() != failed {
                recovery_unavailable.set(failed);
            }
        });
    }
    // Send, from the button or the keyboard: one snapshot of the target,
    // text, edit generation and command, which Check again reuses.
    let mut send_message = move || {
        let target = draft_target();
        let text = composer.peek().text(target.as_ref());
        let command = message_command(
            selected(),
            open_chat(),
            workspace_cache
                .read()
                .as_ref()
                .and_then(|open| open.time_zone.clone()),
            text.clone(),
        );
        let (Some(target), Ok(command)) = (target, command.clone()) else {
            send_slot.run_or_refuse(client.read().clone(), command, || {});
            return;
        };
        if send_slot.sending() {
            return;
        }
        // A Send on this target whose answer never arrived is asked about
        // again with its own key; a new command would risk sending twice.
        let unanswered = composer.peek().pending_send(&target);
        if let Some(pending) = unanswered {
            dispatch_send(
                send_slot,
                client,
                composer,
                snapshot_attempt,
                target,
                pending,
            );
            return;
        }
        if text.trim().is_empty() {
            send_slot.run_or_refuse(
                client.read().clone(),
                Err(MESSAGE_REFUSED.to_owned()),
                || {},
            );
            return;
        }
        let pending = composer
            .write()
            .begin_send(&target, command.key(), &text, command.0.clone());
        if let Some(pending) = pending {
            dispatch_send(
                send_slot,
                client,
                composer,
                snapshot_attempt,
                target,
                pending,
            );
        }
    };
    let shown_target = draft_target();
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
    let practice_state = use_read_view(practice);
    let updates_state = use_read_view(updates);
    let schedule = practice_state.stage();
    let day = practice_state.content();
    let server_now = day.map(|day| day.current_time.clone()).unwrap_or_default();
    let practice_zone = day
        .map(|day| day.time_zone.clone())
        .unwrap_or_else(|| "America/New_York".to_owned());
    let practice_date = format_local_date(&server_now, &practice_zone);
    let visits = day.map(upcoming_visits).unwrap_or_default();
    let show_schedule = schedule_unanswered(schedule) || day.is_some_and(shows_today_schedule);
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
        (Some(Err(())), Some(Ok(_))) => ReadStage::Stale,
        (_, Some(Ok(_))) => ReadStage::Showing,
        (_, Some(Err(()))) => ReadStage::Unavailable,
        (_, None) => ReadStage::Loading,
    };
    let chats = chat_view(
        selected.read().as_deref(),
        workspace_current.as_ref().map(|result| match result {
            Ok(open) => Ok(*open),
            Err(_) => Err(()),
        }),
        open_chat.read().as_deref(),
    );
    // Whether the open conversation is archived, once the list says.
    let open_archived = match (&chats, open_chat.read().as_deref()) {
        (ChatView::Chats(lines), Some(id)) => lines
            .iter()
            .find(|line| line.id == id)
            .map(|line| line.archived),
        _ => None,
    };
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
                    .map(|page| conversation_lines(&page.snapshot))
                    .map_err(|_| ()),
                _ => Ok(Vec::new()),
            };
            (asked, answer)
        }
    });
    // A failed reread keeps this conversation's last lines, shown as stale, so
    // its turns and any open details dialog stay; one conversation's lines
    // never stand in for another's.
    let mut last_conversation = use_signal(|| None);
    use_effect(move || {
        if let Some((asked, Ok(lines))) = snapshot.read().as_ref() {
            if let (Some(workspace), Some(chat)) = asked.clone() {
                let reads = lines.iter().filter_map(|line| match &line.response {
                    Some((id, feedback::Read::Loaded(Some(saved)))) => Some((id, saved)),
                    _ => None,
                });
                feedback_desk
                    .write()
                    .remember(&feedback::ChatKey { workspace, chat }, reads);
            }
            last_conversation.set(Some((asked.clone(), lines.clone())));
        }
    });
    let asking = (selected(), open_chat());
    let conversation = read_view(
        answer_to(&asking, last_conversation.read().as_ref()).cloned(),
        answer_to(&asking, snapshot.read().as_ref()).cloned(),
    );
    // New conversation: a fresh id the server keeps once a message is sent.
    // The address names it, so a reload before the reply still reopens it.
    let mut start_chat = move || {
        if selected().is_none() {
            return;
        }
        // The new chat is its own draft target, so its composer starts empty.
        open_chat.set(Some(new_chat_id()));
    };
    let drawer_open = drawer() == Drawer::Open;
    let shell_class = format!(
        "glass-app{}{}{}",
        if app_collapsed() { " is-collapsed" } else { "" },
        if drawer_open { " has-open-sidebar" } else { "" },
        if sidebar_collapsed() {
            " has-collapsed-sidebar"
        } else {
            ""
        }
    );
    rsx! {
        ui::Stylesheet { app: SHELL_STYLES }
        if let Some((key, workspace, chat)) = archived_toast() {
            Toast {
                key: "{key}",
                words: ARCHIVED_TOAST,
                action: UNDO.to_owned(),
                on_action: move |_| {
                    // Undo is its own command, under a new key.
                    let command = archive_command(ArchiveAction::Restore, &workspace, &chat);
                    restore_slot.run(client.read().clone(), command, move || {
                        *workspace_attempt.write() += 1;
                    });
                },
                on_gone: move |_| {
                    if archived_toast.peek().as_ref().map(|(standing, ..)| *standing) == Some(key) {
                        archived_toast.set(None);
                    }
                },
            }
        }
        if app_collapsed() {
            Button {
                class: "coda-restore",
                aria_label: "Open Coda",
                onclick: move |_| app_collapsed.set(false),
                img { class: "coda-logo", src: "./assets/coda-health-logo-transparent.png", alt: "" }
            }
        }
        div {
            class: "{shell_class}",
            aria_hidden: if app_collapsed() { "true" } else { "false" },
            // Tab stays on the drawer's own controls while it is open.
            tabindex: if drawer_open { "-1" } else { "0" },
            onkeydown: move |event| {
                let key = event.key().to_string();
                // The page behind an open drawer answers no keys; Escape closes the drawer.
                if drawer() == Drawer::Open {
                    if key == "Escape" {
                        drawer.set(Drawer::Dismissed);
                    }
                    return;
                }
                if key == " " && event.modifiers().ctrl() && event.modifiers().shift() {
                    event.prevent_default();
                    let client = client.read().clone();
                    spawn(async move {
                        let _ = client.dictate().await;
                        composer.write().say(DICTATION_UNAVAILABLE);
                    });
                    return;
                }
                if key == "Enter" && (event.modifiers().meta() || event.modifiers().ctrl()) {
                    event.prevent_default();
                    send_message();
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
                        Button {
                            variant: ButtonVariant::Quiet,
                            class: "sidebar-brand",
                            aria_label: "Coda Health",
                            onclick: move |_| {
                                place.set(Place::Today);
                                drawer.set(Drawer::Dismissed);
                            },
                            img { class: "coda-logo", src: "./assets/coda-health-logo-transparent.png", alt: "Coda Health" }
                        }
                        Button {
                            variant: ButtonVariant::Quiet,
                            icon_only: true,
                            class: "sidebar-collapse",
                            aria_label: "Collapse navigation",
                            onclick: move |_| remember(Panel::Sidebar, true),
                            Icon { name: IconName::SidebarCollapse }
                        }
                        Button {
                            variant: ButtonVariant::Quiet,
                            icon_only: true,
                            class: "sidebar-expand",
                            aria_label: "Expand navigation",
                            onclick: move |_| remember(Panel::Sidebar, false),
                            Icon { name: IconName::SidebarExpand }
                        }
                        Button {
                            variant: ButtonVariant::Quiet,
                            icon_only: true,
                            class: "sidebar-close",
                            aria_label: "Close navigation",
                            onmounted: move |event: MountedEvent| drawer_close.set(Some(event.data())),
                            onclick: move |_| drawer.set(Drawer::Dismissed),
                            Icon { name: IconName::X }
                        }
                    }
                    nav { class: "sidebar-primary selection-host", aria_label: "Primary",
                        NavigationRow {
                            class: "sidebar-nav-item",
                            aria_label: "Open Today",
                            aria_current: (place() == Place::Today).then_some("page"),
                            onclick: move |_| {
                                place.set(Place::Today);
                                drawer.set(Drawer::Dismissed);
                            },
                            if place() == Place::Today {
                                Icon { name: IconName::Home }
                            } else {
                                Icon { name: IconName::ArrowLeft }
                            }
                            strong { class: "item-text", "Today" }
                        }
                        NavigationRow {
                            class: "sidebar-nav-item",
                            aria_current: (place() == Place::Updates).then_some("page"),
                            onclick: move |_| {
                                place.set(Place::Updates);
                                drawer.set(Drawer::Dismissed);
                            },
                            Icon { name: IconName::Message }
                            strong { class: "item-text", "Updates" }
                        }
                        NavigationRow {
                            class: "sidebar-nav-item",
                            aria_label: "Find patient",
                            onclick: move |_| {
                                search_open.set(true);
                                drawer.set(Drawer::Dismissed);
                            },
                            Icon { name: IconName::Search }
                            strong { class: "item-text", "Find patient" }
                            ui::KeyHint { key_label: "⌘ K" }
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
                                if !matches!(chats, ChatView::Closed | ChatView::Pending | ChatView::Unavailable) {
                                    Button {
                                        compact: true,
                                        class: "sidebar-new-conversation",
                                        onclick: move |_| {
                                            start_chat();
                                            drawer.set(Drawer::Dismissed);
                                        },
                                        Icon { name: IconName::Plus }
                                        "{NEW_CONVERSATION}"
                                    }
                                }
                                div { class: "sidebar-section-heading", h2 { "Conversations" } }
                                match chats.clone() {
                                    ChatView::Empty | ChatView::Closed => rsx! { p { class: "sidebar-empty", "{NO_CONVERSATIONS}" } },
                                    ChatView::Pending => rsx! {},
                                    ChatView::Unavailable if patient_gone => rsx! {},
                                    ChatView::Unavailable => rsx! { p { class: "sidebar-loading", "{WORKSPACE_UNAVAILABLE}" } },
                                    ChatView::Chats(lines) => {
                                        let (archived, active): (Vec<_>, Vec<_>) =
                                            lines.into_iter().partition(|line| line.archived);
                                        rsx! {
                                            for line in active {
                                                NavigationRow {
                                                    class: "sidebar-conversation-link",
                                                    onclick: {
                                                        let id = line.id.clone();
                                                        move |_| {
                                                            open_chat.set(Some(id.clone()));
                                                            drawer.set(Drawer::Dismissed);
                                                        }
                                                    },
                                                    Icon { name: IconName::Message }
                                                    "{line.title}"
                                                }
                                            }
                                            if !archived.is_empty() {
                                                div { class: "sidebar-section-heading", h2 { "{ARCHIVED}" } }
                                                for line in archived {
                                                    NavigationRow {
                                                        class: "sidebar-conversation-link sidebar-archived-link",
                                                        onclick: {
                                                            let id = line.id.clone();
                                                            move |_| {
                                                                open_chat.set(Some(id.clone()));
                                                                drawer.set(Drawer::Dismissed);
                                                            }
                                                        },
                                                        Icon { name: IconName::Archive }
                                                        "{line.title}"
                                                    }
                                                }
                                            }
                                        }
                                    }
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
                            Button {
                                variant: ButtonVariant::Quiet,
                                onclick: move |_| {
                                    connections_open.set(true);
                                    drawer.set(Drawer::Dismissed);
                                },
                                Icon { name: IconName::Users }
                                "{CONNECTIONS}"
                            }
                        }
                        Button {
                            variant: ButtonVariant::Quiet,
                            onclick: move |_| {
                                shortcuts_open.set(true);
                                drawer.set(Drawer::Dismissed);
                            },
                            Icon { name: IconName::Keyboard }
                            "Keyboard shortcuts"
                        }
                        Button {
                            variant: ButtonVariant::Quiet,
                            aria_label: "Sign out of Coda",
                            onclick: move |_| sign_out_everywhere(session),
                            Icon { name: IconName::Users }
                            "Sign out"
                        }
                    }
                }
            }
            // The stylesheet gives it the window's size wherever the sidebar is a drawer, seen or
            // not, and no size where the sidebar stands beside the page. So having no size means
            // the window grew past the drawer, which leaves nothing to keep the page inert behind.
            div {
                class: "glass-sidebar-backdrop",
                onclick: move |_| drawer.set(Drawer::Dismissed),
                onresize: move |event: ResizeEvent| {
                    let gone = event.get_border_box_size().is_ok_and(|size| size.width == 0.0);
                    if gone && *drawer.peek() == Drawer::Open {
                        drawer.set(Drawer::Closed);
                    }
                },
            }
            main { class: "glass-stage",
                "inert": drawer_open.then_some("true"),
                header { class: "glass-toolbar",
                    Button {
                        variant: ButtonVariant::Quiet,
                        icon_only: true,
                        class: "stage-menu",
                        aria_label: "Open navigation",
                        aria_expanded: if drawer_open { "true" } else { "false" },
                        onmounted: move |event: MountedEvent| drawer_menu.set(Some(event.data())),
                        onclick: move |_| drawer.set(Drawer::Open),
                        Icon { name: IconName::Menu }
                    }
                    Button {
                        variant: ButtonVariant::Quiet,
                        class: "stage-minimize",
                        onclick: move |_| app_collapsed.set(true),
                        "Collapse Coda"
                    }
                }
                match place() {
                    Place::Today => rsx! {
                        TodaySurface {
                            state: today_state.clone(),
                            connections: connections.clone(),
                            visits: visits.clone(),
                            show_schedule,
                            schedule,
                            on_schedule_retry: move |_| *today_attempt.write() += 1,
                            schedule_empty: schedule_empty.clone(),
                            schedule_collapsed: schedule_collapsed(),
                            on_schedule_collapsed: move |collapsed| remember(Panel::Schedule, collapsed),
                            server_now: server_now.clone(),
                            practice_date: practice_date.clone(),
                            need_you,
                            focused: shown_focus,
                            on_retry: move |_| today_attempt.set(today_attempt() + 1),
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
                            draft: composer.read().text(shown_target.as_ref()),
                            notice: notice_words(
                                composer.read().notice(shown_target.as_ref()),
                                composer.read().clearing(shown_target.as_ref()),
                            ),
                            conflict: composer.read().conflict(shown_target.as_ref()),
                            clearing: composer.read().clearing(shown_target.as_ref()),
                            recovery_unavailable: recovery_unavailable(),
                            on_conflict_retry: move |_| {
                                let target = draft_target();
                                composer.write().retry(target.as_ref());
                                if let Some(target) = target {
                                    composer.write().retry_failed(&target);
                                    autosave(composer, client, target);
                                }
                            },
                            on_conflict_use_newer: move |_| {
                                composer.write().take_newer(draft_target().as_ref());
                            },
                            on_draft_blur: move |_| {
                                if let Some(target) = draft_target() {
                                    autosave(composer, client, target);
                                }
                            },
                            is_admin: whoami.read().as_ref().is_some_and(|account| account.admin),
                            practice_accounts: whoami
                                .read()
                                .as_ref()
                                .map(|account| account.practice_accounts.clone())
                                .unwrap_or_default(),
                            target_physician_id: whoami
                                .read()
                                .as_ref()
                                .map(|account| panel_target(account, &target_physician()).to_owned())
                                .unwrap_or_default(),
                            panel_busy: panel_busy(),
                            panel_notice: panel_notice(),
                            on_select_physician: move |value: String| target_physician.set(value),
                            on_assign_panel: move |_| {
                                let Some(patient_id) = selected() else { return };
                                let Some(physician_id) = whoami
                                    .read()
                                    .as_ref()
                                    .map(|account| panel_target(account, &target_physician()).to_owned())
                                else {
                                    return;
                                };
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
                                let Some(physician_id) = whoami
                                    .read()
                                    .as_ref()
                                    .map(|account| panel_target(account, &target_physician()).to_owned())
                                else {
                                    return;
                                };
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
                            work_busy: work_slot.unanswered() || resume_slot.unanswered(),
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
                            on_draft: move |value: String| {
                                let target = draft_target();
                                composer.write().edit(target.as_ref(), value);
                                if let Some(target) = target {
                                    autosave_after(composer, client, target, AUTOSAVE_DEBOUNCE_MS);
                                }
                            },
                            conversation: conversation.clone(),
                            on_conversation_retry: move |_| *snapshot_attempt.write() += 1,
                            conversation_resume_busy: chat_resume_slot.unanswered(),
                            conversation_resume_notice: chat_resume_slot.notice(client),
                            on_conversation_resume: move |target: WorkResumeTarget| {
                                if let Some(command) =
                                    chat_resume_command(selected(), open_chat(), &target)
                                {
                                    chat_resume_slot.run(client.read().clone(), command, move || {
                                        *snapshot_attempt.write() += 1;
                                        // Resume and Try again bring an archived conversation back.
                                        *workspace_attempt.write() += 1;
                                    });
                                }
                            },
                            conversation_stop_busy: chat_stop_slot.unanswered(),
                            conversation_stop_notice: chat_stop_slot.notice(client),
                            on_conversation_stop: move |target: WorkStopTarget| {
                                if let Some(command) =
                                    chat_stop_command(selected(), open_chat(), &target)
                                {
                                    chat_stop_slot.run(client.read().clone(), command, move || {
                                        *snapshot_attempt.write() += 1;
                                    });
                                }
                            },
                            queued_editing: queued_editing(),
                            queued_busy: queued_slot.unanswered(),
                            queued_notice: queued_slot.notice(client),
                            on_queued_edit: move |(id, text): (String, String)| queued_editing.set(Some((id, text))),
                            on_queued_edit_cancel: move |_| queued_editing.set(None),
                            on_queued_edit_save: move |_| {
                                let Some((id, text)) = queued_editing() else { return };
                                if let Some(command) =
                                    queued_edit_command(selected(), open_chat(), &id, &text)
                                {
                                    queued_slot.run(client.read().clone(), command, move || {
                                        // Words typed after Save stay open to save.
                                        if queued_editing.peek().as_ref() == Some(&(id.clone(), text.clone())) {
                                            queued_editing.set(None);
                                        }
                                        *snapshot_attempt.write() += 1;
                                    });
                                }
                            },
                            on_queued_withdraw: move |id: String| {
                                if let Some(command) =
                                    queued_withdraw_command(selected(), open_chat(), &id)
                                {
                                    queued_slot.run(client.read().clone(), command, move || {
                                        *snapshot_attempt.write() += 1;
                                    });
                                }
                            },
                            on_open_chat: move |id: String| open_chat.set(Some(id)),
                            open_archived,
                            archive_busy: archive_slot.unanswered() || restore_slot.unanswered(),
                            archive_notice: rsx! {
                                {archive_slot.notice(client)}
                                {restore_slot.notice(client)}
                            },
                            on_archive: move |_| {
                                let (Some(workspace), Some(chat)) = (selected(), open_chat()) else {
                                    return;
                                };
                                let command = archive_command(ArchiveAction::Archive, &workspace, &chat);
                                archive_slot.run(client.read().clone(), command, move || {
                                    offer_undo(workspace.clone(), chat.clone());
                                });
                            },
                            on_new_chat: move |_| start_chat(),
                            on_send: move |_| send_message(),
                            on_dictate: move |_| {
                                let client = client.read().clone();
                                spawn(async move {
                                    let _ = client.dictate().await;
                                    composer.write().say(DICTATION_UNAVAILABLE);
                                });
                            },
                            on_editing: move |value: bool| editing.set(value),
                            conversation_open: open_chat.read().is_some(),
                            feedback_chat: selected().zip(open_chat()).map(|(workspace, chat)| {
                                feedback::ChatKey { workspace, chat }
                            }),
                            past_work: open_workspace.as_ref().is_some_and(|open| past_work_copy(open).is_some()),
                            reply_notice: reply_notice(),
                            reply_busy: reply_slot.unanswered(),
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
                            on_resume_reply: move |target: WorkResumeTarget| {
                                let Some(command) = chat_resume_command(selected(), Some(target.work_item_id.clone()), &target) else { return };
                                reply_notice.set(String::new());
                                reply_slot.run_answered(
                                    client.read().clone(),
                                    command,
                                    Callback::new(move |()| *workspace_attempt.write() += 1),
                                    Callback::new(move |()| {
                                        reply_notice.set(REPLY_REVISION_STALE.to_owned());
                                        *workspace_attempt.write() += 1;
                                    }),
                                );
                            },
                            on_revise_reply: move |(reply_id, revision, instruction): (String, u64, String)| {
                                let Some(ws_id) = selected() else { return };
                                let zone = workspace_cache
                                    .read()
                                    .as_ref()
                                    .and_then(|open| open.time_zone.clone())
                                    .unwrap_or_default();
                                reply_notice.set(String::new());
                                // The command names the revision the physician
                                // was shown; a refusal rereads the workspace so
                                // the current draft replaces the stale one.
                                reply_slot.run_answered(
                                    client.read().clone(),
                                    Command::new(
                                        "reply.revise",
                                        serde_json::json!({
                                            "patient_workspace_id": ws_id,
                                            "reply_work_id": reply_id,
                                            "expected_draft_revision": revision,
                                            "instruction": instruction,
                                            "physician_time_zone": zone,
                                        }),
                                    ),
                                    Callback::new(move |()| *workspace_attempt.write() += 1),
                                    Callback::new(move |()| {
                                        reply_notice.set(REPLY_REVISION_STALE.to_owned());
                                        *workspace_attempt.write() += 1;
                                    }),
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
                on_close: move |_| {
                    search_open.set(false);
                    editing.set(false);
                },
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
const COMMAND_CONFIRMING: &str = "Confirming with Coda…";
const COMMAND_UNCONFIRMED: &str = "Coda couldn’t confirm that.";
const CHECK_AGAIN: &str = "Check again";
const COMMAND_UNKEPT: &str =
    "This tab can’t keep a reload copy of that. Keep the page open until Coda confirms it.";
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

    /// The idempotency key the server recognizes this command by.
    fn key(&self) -> String {
        self.0
            .get("idempotency_key")
            .and_then(serde_json::Value::as_str)
            .unwrap_or_default()
            .to_owned()
    }

    /// The patient and chat an Archive names, so its Undo can name them too.
    #[cfg(any(target_arch = "wasm32", test))]
    fn archive_target(&self) -> Option<(String, String)> {
        if self.0.get("type")?.as_str()? != ArchiveAction::Archive.wire() {
            return None;
        }
        let payload = self.0.get("payload")?;
        let field = |name: &str| payload.get(name)?.as_str().map(str::to_owned);
        Some((field("patient_workspace_id")?, field("chat_id")?))
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

/// The waits before each automatic re-ask of a command whose answer was lost.
/// Every re-ask is the same request under the same key, so the server takes it
/// at most once.
const CONFIRM_BACKOFF_MS: [u32; 3] = [1_000, 2_000, 4_000];

/// Asks until an answer arrives. While none does, `confirming` runs and the
/// same request is asked again after each wait in [`CONFIRM_BACKOFF_MS`];
/// the answer is `Unconfirmed` only once every re-ask went unanswered too.
async fn ask_until_answered(
    mut ask: impl AsyncFnMut() -> Answer,
    mut wait: impl AsyncFnMut(u32),
    mut confirming: impl FnMut(),
) -> Answer {
    let mut answer = ask().await;
    for ms in CONFIRM_BACKOFF_MS {
        if answer != Answer::Unconfirmed {
            break;
        }
        confirming();
        wait(ms).await;
        answer = ask().await;
    }
    answer
}

/// One command at a time on one item: what is in flight, and what Check again resends.
#[derive(Clone, Copy, PartialEq)]
struct CommandSlot {
    stage: Signal<CommandStage>,
    /// The unconfirmed command, what runs once it is taken, and what runs if
    /// it is refused.
    last: Signal<Option<(Command, Callback, Option<Callback>)>>,
    /// Where an unanswered command is kept across a reload of this tab: the
    /// slot's name and the signed-in physician's scope. `None` for a slot
    /// whose command another owner keeps (Send's lives with its draft).
    kept: Option<(&'static str, Memo<Option<local_draft::Scope>>)>,
    /// The unanswered command could not be kept for a reload, so a reload
    /// would lose it; the notice says so until it is answered.
    unkept: Signal<bool>,
}

/// A schedule rail shows while its day is loading or failed: only an answered day
/// can say there is no calendar or no visit to show.
fn schedule_unanswered(schedule: ReadStage) -> bool {
    matches!(schedule, ReadStage::Loading | ReadStage::Unavailable)
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

/// A slot whose unanswered command is kept under `name` for this physician, so
/// a reload asks it again under its own key.
fn use_command_slot(name: &'static str, scope: Memo<Option<local_draft::Scope>>) -> CommandSlot {
    CommandSlot {
        stage: use_signal(|| CommandStage::Idle),
        last: use_signal(|| None),
        kept: Some((name, scope)),
        unkept: use_signal(|| false),
    }
}

/// A slot whose command another owner keeps across a reload.
fn use_unkept_command_slot() -> CommandSlot {
    CommandSlot {
        stage: use_signal(|| CommandStage::Idle),
        last: use_signal(|| None),
        kept: None,
        unkept: use_signal(|| false),
    }
}

impl CommandSlot {
    /// Sent or being confirmed, and not yet answered.
    fn sending(&self) -> bool {
        matches!(
            *self.stage.read(),
            CommandStage::Sending | CommandStage::Confirming
        )
    }

    /// Sent, and not yet known to be taken or refused. Another command must
    /// wait, so Check again can only resend this one.
    fn unanswered(&self) -> bool {
        matches!(
            *self.stage.read(),
            CommandStage::Sending | CommandStage::Confirming | CommandStage::Unconfirmed
        )
    }

    /// Keeps `command` for a reload of this tab, or forgets it when `None`.
    /// Without storage or a known physician nothing is kept: the command is
    /// still confirmed while the page stays open, and the notice says a reload
    /// would lose it.
    fn keep(mut self, command: Option<&Command>) {
        #[cfg(target_arch = "wasm32")]
        if let Some((name, scope)) = self.kept {
            let kept = scope.peek().as_ref().is_some_and(|scope| {
                local_draft::SessionStorage::open().is_some_and(|storage| {
                    local_draft::write_command(
                        &storage,
                        scope,
                        name,
                        command.map(|command| &command.0),
                    )
                    .is_ok()
                })
            });
            let unkept = command.is_some() && !kept;
            if *self.unkept.peek() != unkept {
                self.unkept.set(unkept);
            }
        }
        #[cfg(not(target_arch = "wasm32"))]
        let _ = (command, &mut self.unkept);
    }

    /// Asks again, under its own key, the command a reload interrupted.
    /// `settled` runs on any answer, since what the command was for went with
    /// the old page.
    #[cfg(target_arch = "wasm32")]
    fn ask_kept(self, client: PhysicianClient, settled: Callback) {
        if let Some(command) = self.kept_command() {
            self.send(client, command, settled, Some(settled));
        }
    }

    /// The command a reload interrupted, if this slot kept one.
    #[cfg(target_arch = "wasm32")]
    fn kept_command(&self) -> Option<Command> {
        let (name, scope) = self.kept?;
        let scope = scope.peek().clone()?;
        let storage = local_draft::SessionStorage::open()?;
        local_draft::read_command(&storage, &scope, name).map(Command)
    }

    /// Sends `command` unless one is already in flight; `then` runs once it is taken.
    fn run(self, client: PhysicianClient, command: Command, mut then: impl FnMut() + 'static) {
        self.send(client, command, Callback::new(move |()| then()), None);
    }

    /// Like `run`; `refused` runs when the server answers and does not take it.
    fn run_answered(
        self,
        client: PhysicianClient,
        command: Command,
        then: Callback,
        refused: Callback,
    ) {
        self.send(client, command, then, Some(refused));
    }

    fn send(
        mut self,
        client: PhysicianClient,
        command: Command,
        then: Callback,
        refused: Option<Callback>,
    ) {
        if matches!(
            *self.stage.peek(),
            CommandStage::Sending | CommandStage::Confirming
        ) {
            return;
        }
        // A slot that keeps its command holds the unconfirmed one until it is
        // answered: a new command would take its place and its reload copy, so
        // it could never be confirmed. Send keeps each target's own.
        let replaces_unconfirmed = *self.stage.peek() == CommandStage::Unconfirmed
            && self
                .last
                .peek()
                .as_ref()
                .is_some_and(|(last, ..)| last.key() != command.key());
        if self.kept.is_some() && replaces_unconfirmed {
            return;
        }
        self.last.set(Some((command.clone(), then, refused)));
        self.keep(Some(&command));
        self.stage.set(CommandStage::Sending);
        spawn(async move {
            let mut stage = self.stage;
            let answered = ask_until_answered(
                async || answer(&client.command(&command.0).await),
                async |ms| sleep_ms(ms).await,
                || stage.set(CommandStage::Confirming),
            )
            .await;
            match answered {
                Answer::Accepted => {
                    self.stage.set(CommandStage::Idle);
                    self.last.set(None);
                    self.keep(None);
                    then.call(());
                }
                Answer::Refused(reason) => {
                    self.stage.set(CommandStage::Refused(reason));
                    self.last.set(None);
                    self.keep(None);
                    if let Some(refused) = refused {
                        refused.call(());
                    }
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
            Err(reason) if !self.unanswered() => self.stage.set(CommandStage::Refused(reason)),
            Err(_) => {}
        }
    }

    /// Resends the unconfirmed command as the same request, with its own follow-up.
    fn check_again(self, client: PhysicianClient) {
        // Read and let go of `last` before `send` writes it again.
        let last = self.last.peek().clone();
        if let Some((command, then, refused)) = last {
            self.send(client, command, then, refused);
        }
    }

    /// A line beside the command's controls.
    fn notice(self, client: Signal<PhysicianClient>) -> Element {
        rsx! {
            CommandNotice {
                stage: self.stage.read().clone(),
                sending: COMMAND_SENDING,
                confirming: COMMAND_CONFIRMING,
                unconfirmed: COMMAND_UNCONFIRMED,
                check_again: CHECK_AGAIN,
                on_check_again: move |_| self.check_again(client.peek().clone()),
            }
            if self.unanswered() && *self.unkept.read() {
                p { class: "quiet-line", role: "status", "{COMMAND_UNKEPT}" }
            }
        }
    }
}

#[cfg(test)]
mod confirm_tests {
    use std::cell::RefCell;
    use std::future::Future;
    use std::pin::pin;
    use std::task::{Context, Poll, Waker};

    use super::{Answer, CONFIRM_BACKOFF_MS, ask_until_answered};

    /// Runs `future`, whose fakes never wait, to its end.
    fn run<T>(future: impl Future<Output = T>) -> T {
        let mut future = pin!(future);
        let mut context = Context::from_waker(Waker::noop());
        loop {
            if let Poll::Ready(value) = future.as_mut().poll(&mut context) {
                return value;
            }
        }
    }

    /// What one command's asking did: each request, wait and confirming
    /// notice in order, and how it was answered in the end.
    fn asked(answers: Vec<Answer>) -> (Vec<String>, Answer) {
        let answers = RefCell::new(answers.into_iter());
        let seen = RefCell::new(Vec::new());
        let answer = run(ask_until_answered(
            async || {
                seen.borrow_mut().push("ask".to_owned());
                answers.borrow_mut().next().expect("asked once too often")
            },
            async |ms| seen.borrow_mut().push(format!("wait {ms}")),
            || seen.borrow_mut().push("confirming".to_owned()),
        ));
        (seen.into_inner(), answer)
    }

    #[test]
    fn an_answered_command_is_asked_once() {
        assert_eq!(
            asked(vec![Answer::Accepted]),
            (vec!["ask".into()], Answer::Accepted)
        );
        assert_eq!(
            asked(vec![Answer::Refused("No.".into())]),
            (vec!["ask".into()], Answer::Refused("No.".into()))
        );
    }

    #[test]
    fn a_lost_answer_is_confirmed_by_asking_again_after_a_wait() {
        let (seen, answer) = asked(vec![
            Answer::Unconfirmed,
            Answer::Unconfirmed,
            Answer::Accepted,
        ]);
        assert_eq!(
            seen,
            [
                "ask",
                "confirming",
                "wait 1000",
                "ask",
                "confirming",
                "wait 2000",
                "ask"
            ]
        );
        assert_eq!(answer, Answer::Accepted);
        let (_, answer) = asked(vec![Answer::Unconfirmed, Answer::Refused("Gone.".into())]);
        assert_eq!(
            answer,
            Answer::Refused("Gone.".into()),
            "a re-ask's refusal still counts"
        );
    }

    #[test]
    fn it_is_unconfirmed_only_once_every_re_ask_went_unanswered() {
        let lost = (0..=CONFIRM_BACKOFF_MS.len()).map(|_| Answer::Unconfirmed);
        let (seen, answer) = asked(lost.collect());
        assert_eq!(answer, Answer::Unconfirmed);
        assert_eq!(
            seen.iter().filter(|step| *step == "ask").count(),
            CONFIRM_BACKOFF_MS.len() + 1
        );
        assert_eq!(
            seen.last().map(String::as_str),
            Some("ask"),
            "no wait after the last ask"
        );
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

/// The sign-in page, or with `resume_email` the prompt in place of a workspace whose
/// session ended: it mails the same address, and signing in opens Today.
#[component]
fn SignInScreen(
    client: Signal<PhysicianClient>,
    session: SessionState,
    link: Option<SignInLink>,
    on_signed_in: EventHandler<Option<WhoamiResponse>>,
    resume_email: Option<String>,
    on_sign_out: Option<EventHandler<()>>,
) -> Element {
    let resuming = resume_email.is_some();
    let mut removed = session.removed;
    let mut email = use_signal(|| {
        link.as_ref()
            .map(|link| link.email.clone())
            .or_else(|| resume_email.clone())
            .unwrap_or_default()
    });
    let mut stage = use_signal(|| {
        if link.is_some() {
            SignInStage::SigningIn
        } else if *removed.peek() {
            SignInStage::NoAccess
        } else {
            SignInStage::Email
        }
    });

    let mut verify_link = move |link: SignInLink| {
        let client = client.peek().clone();
        stage.set(SignInStage::SigningIn);
        spawn(async move {
            match client.verify(&link.email, &link.code).await {
                Ok(verified) => {
                    #[cfg(target_arch = "wasm32")]
                    {
                        store_session(&verified.session, &link.email);
                        let mut owner = session.owner;
                        owner.set(Some(link.email.trim().to_owned()));
                        let mut seen = session.sign_outs_seen;
                        seen.set(read_sign_outs());
                        tabs::announce();
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
                        _ => SIGN_IN_UNCONFIRMED,
                    };
                    stage.set(SignInStage::Recovery(message.to_owned()));
                }
            }
        });
    };
    // This effect has no reactive reads: one page load redeems its mailed link
    // once. Ordinary rerenders and request failures never retry the token.
    // Coda says whether the session this browser stores is still signed in.
    // Your own link changes nothing while your session stands, and signs you
    // back in once it has ended; when Coda can't say which, your session
    // stays and the link is not used. Another physician still signed in is
    // signed out before the link is redeemed; an ended session needs no
    // sign-out. When Coda can't say for another physician, nobody is signed
    // out: the page says Coda couldn't confirm sign-in, and the link stays
    // unused for another try. Either way the session must still be the one
    // this browser stores when Coda answers; otherwise the link is redeemed.
    use_effect(move || {
        if let Some(link) = link.clone() {
            let held = client.peek().clone();
            let yours = read_stored_owner()
                .is_some_and(|owner| owner.eq_ignore_ascii_case(link.email.trim()));
            let checked = held.session().get();
            spawn(async move {
                if let Some(checked) = checked {
                    match held.whoami().await {
                        Ok(account)
                            if account.email.eq_ignore_ascii_case(link.email.trim())
                                && still_stored(&checked) =>
                        {
                            on_signed_in.call(Some(account));
                            return;
                        }
                        Ok(account) if account.email.eq_ignore_ascii_case(link.email.trim()) => {}
                        Ok(_) if still_stored(&checked) => session.end(),
                        Ok(_) => {}
                        Err(failure) if refusal(&failure).is_some() => {}
                        Err(_) if !still_stored(&checked) => {}
                        Err(_) if yours => {
                            on_signed_in.call(None);
                            return;
                        }
                        Err(_) => {
                            stage.set(SignInStage::Recovery(SIGN_IN_UNCONFIRMED.to_owned()));
                            return;
                        }
                    }
                }
                verify_link(link);
            });
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
                        stage.set(SignInStage::Recovery(text));
                    }
                },
                Err(_) => {
                    stage.set(SignInStage::Recovery(SIGN_IN_UNAVAILABLE.to_owned()));
                }
            }
        });
    };
    let working = matches!(stage(), SignInStage::Requesting | SignInStage::SigningIn);
    let ways_on = move || {
        rsx! {
            ActionRow {
                Button {
                    onclick: move |_| request_token(()),
                    "Send a new link"
                }
                if !resuming {
                    Button {
                        variant: ButtonVariant::Quiet,
                        onclick: move |_| stage.set(SignInStage::Email),
                        "Use a different email"
                    }
                }
            }
        }
    };
    rsx! {
        div { class: "sign-in-screen",
            section {
                class: "sign-in-column",
                aria_label: "Sign in to Coda",
                img {
                    class: "coda-logo",
                    src: "./assets/coda-health-logo-transparent.png",
                    alt: "Coda Health",
                }
                Card {
                    div { class: "sign-in-content", aria_live: "polite", aria_busy: working,
                        PageTitle { text: stage().heading(resuming) }
                        match stage() {
                            SignInStage::Email if resuming => rsx! {
                                p { class: "sign-in-sub", "{SESSION_ENDED}" }
                                p { class: "sign-in-sub sign-in-email", "{email}" }
                                div { class: "sign-in-form",
                                    Button {
                                        variant: ButtonVariant::Primary,
                                        disabled: email.read().trim().is_empty(),
                                        onclick: move |_| request_token(()),
                                        "Email me a sign-in link"
                                    }
                                    p { class: "sign-in-sub", "Open it to sign in again." }
                                }
                            },
                            SignInStage::NoAccess => rsx! {
                                p { class: "sign-in-sub",
                                    if removed() { "{ACCESS_REMOVED}" } else { "{NO_ACCESS}" }
                                }
                                ActionRow {
                                    Button {
                                        onclick: move |_| {
                                            removed.set(false);
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
                                    TextField {
                                        label: "Work email",
                                        kind: TextKind::Email,
                                        placeholder: "you@practice.com",
                                        hint: "We’ll email you a link. No password needed.",
                                        value: email(),
                                        on_input: move |words| email.set(words),
                                        onkeydown: move |event: KeyboardEvent| {
                                            if event.key() == Key::Enter && !email.peek().trim().is_empty() {
                                                request_token(());
                                            }
                                        },
                                    }
                                    Button {
                                        variant: ButtonVariant::Primary,
                                        disabled: email.read().trim().is_empty(),
                                        onclick: move |_| request_token(()),
                                        "Send sign-in link"
                                    }
                                }
                            },
                            SignInStage::Requesting => rsx! {
                                p { class: "sign-in-sub", role: "status", "This may take a moment." }
                                ProgressBar { label: "Sending your link", bare: true }
                            },
                            SignInStage::SigningIn => rsx! {
                                p { class: "sign-in-sub", role: "status", "Opening your Coda workspace." }
                                ProgressBar { label: "Signing you in", bare: true }
                            },
                            SignInStage::CheckEmail => rsx! {
                                p { class: "sign-in-sub", "{LINK_ON_THE_WAY}" }
                                p { class: "sign-in-sub sign-in-email", "{email}" }
                                p { class: "sign-in-sub",
                                    if resuming {
                                        "Choose Sign in in the email to sign in again."
                                    } else {
                                        "Choose Sign in in the email to open your workspace."
                                    }
                                }
                                {ways_on()}
                            },
                            SignInStage::Recovery(message) => rsx! {
                                ui::Notice { tone: Tone::Error, icon: IconName::Alert, title: message }
                                {ways_on()}
                            },
                        }
                        if let Some(sign_out) = on_sign_out {
                            ActionRow {
                                Button {
                                    variant: ButtonVariant::Quiet,
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
}

#[derive(Clone, PartialEq, Eq)]
enum SignInStage {
    Email,
    Requesting,
    CheckEmail,
    SigningIn,
    /// What went wrong, with the ways on from it.
    Recovery(String),
    /// A held link whose account no longer has access to this practice.
    NoAccess,
}

impl SignInStage {
    /// What the page is called at this stage; `resuming` when it covers a page whose session ended.
    fn heading(&self, resuming: bool) -> &'static str {
        match self {
            Self::CheckEmail => "Check your email",
            Self::Requesting => "Sending your link…",
            Self::SigningIn => "Signing you in…",
            Self::Recovery(_) => "Let’s get you signed in",
            Self::NoAccess => "No access to this practice",
            Self::Email if resuming => "Sign in to continue",
            Self::Email => "Sign in to Coda",
        }
    }
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
    connections: Vec<String>,
    visits: Vec<VisitLine>,
    show_schedule: bool,
    schedule: ReadStage,
    on_schedule_retry: EventHandler,
    schedule_empty: String,
    schedule_collapsed: bool,
    on_schedule_collapsed: EventHandler<bool>,
    server_now: String,
    practice_date: Option<String>,
    need_you: usize,
    focused: Option<i32>,
    on_retry: EventHandler,
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
                if let Some(date) = practice_date {
                    span { class: "today-date", "{date}" }
                }
                if need_you > 0 {
                    Count { value: need_you, words }
                }
            }
            div { class: if scheduled { "today-layout has-schedule" } else { "today-layout" },
                section { class: "today-main", aria_label: "Needs you",
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
                    NavigationRow {
                        class: if focused == Some(index) { "today-row-open is-keyboard-focus" } else { "today-row-open" },
                        onclick: {
                            let row = row.clone();
                            move |_| on_open.call(row.clone())
                        },
                        span { class: "today-row-line",
                            span { class: "today-row-heading",
                                LiveDot {}
                                strong { class: "today-row-who", "{row.patient}" }
                                if row.now {
                                    span { class: "urgency-now", "{NEEDS_YOU_NOW}" }
                                }
                                StatePill { tone: row_tone(&row.kind), icon: row_icon(&row.kind), "{row.did}" }
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
                            Button {
                                compact: true,
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
    read: ReadStage,
    on_retry: EventHandler,
    empty: String,
    collapsed: bool,
    on_collapsed: EventHandler<bool>,
    name_the_visit: bool,
    focused: Option<usize>,
    on_open: EventHandler<VisitLine>,
) -> Element {
    let count = visits.len();
    rsx! {
        ui::RailCard {
            class: if collapsed { "schedule-rail is-collapsed" } else { "schedule-rail" },
            aria_label: "{title}",
            icon: IconName::Calendar,
            title,
            count,
            count_words: "left",
            collapsed,
            on_collapsed: move |folded| on_collapsed.call(folded),
            expand_label: "Show {title}",
            collapse_label: "Collapse {title}",
            ReadNotice {
                stage: read,
                loading: SCHEDULE_LOADING,
                stale: SCHEDULE_STALE,
                unavailable: SCHEDULE_UNAVAILABLE,
                retry: TRY_AGAIN,
                on_retry,
            }
            if visits.is_empty() && read == ReadStage::Showing {
                p { class: "schedule-rail-empty", "{empty}" }
            }
            for (index, visit) in visits.iter().enumerate() {
                NavigationRow {
                    class: if focused == Some(index) {
                        "schedule-visit is-next is-keyboard-focus"
                    } else if index == 0 {
                        "schedule-visit is-next"
                    } else {
                        "schedule-visit"
                    },
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
    read: ReadStage,
    on_go_today: EventHandler,
    name: String,
    workspace: Option<PatientWorkspaceResponse>,
    rows: Vec<TodayLine>,
    visits: Vec<VisitLine>,
    schedule_collapsed: bool,
    on_schedule_collapsed: EventHandler<bool>,
    schedule: ReadStage,
    on_schedule_retry: EventHandler,
    focused: Option<i32>,
    chats: ChatView,
    server_now: String,
    draft: String,
    notice: String,
    /// The newer saved draft a conflict is waiting on.
    conflict: Option<ConflictDraft>,
    /// A sent message's text is still waiting to be cleared from the saved draft.
    clearing: bool,
    /// This tab cannot keep a reload copy of unsent text.
    recovery_unavailable: bool,
    on_conflict_retry: EventHandler,
    on_conflict_use_newer: EventHandler,
    on_draft_blur: EventHandler,
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
    on_read: EventHandler<(String, String)>,
    on_open_source: EventHandler<SourceChip>,
    on_draft: EventHandler<String>,
    on_send: EventHandler,
    on_dictate: EventHandler,
    on_editing: EventHandler<bool>,
    conversation: ReadView<Vec<ConversationLine>>,
    on_conversation_retry: EventHandler,
    conversation_resume_busy: bool,
    conversation_resume_notice: Element,
    on_conversation_resume: EventHandler<WorkResumeTarget>,
    conversation_stop_busy: bool,
    conversation_stop_notice: Element,
    on_conversation_stop: EventHandler<WorkStopTarget>,
    /// The queued message being edited, and its words so far.
    queued_editing: Option<(String, String)>,
    queued_busy: bool,
    queued_notice: Element,
    on_queued_edit: EventHandler<(String, String)>,
    on_queued_edit_cancel: EventHandler,
    on_queued_edit_save: EventHandler,
    on_queued_withdraw: EventHandler<String>,
    on_open_chat: EventHandler<String>,
    on_new_chat: EventHandler,
    /// The open conversation is archived; `None` until the list names it.
    open_archived: Option<bool>,
    archive_busy: bool,
    archive_notice: Element,
    on_archive: EventHandler,
    conversation_open: bool,
    /// The open chat, which its responses' feedback belongs to.
    feedback_chat: Option<feedback::ChatKey>,
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
    on_revise_reply: EventHandler<(String, u64, String)>,
    on_resume_reply: EventHandler<WorkResumeTarget>,
) -> Element {
    let running = conversation
        .content()
        .is_some_and(|lines| conversation_running(lines));
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
    let records = workspace
        .as_ref()
        .and_then(|open| records_time(open, &server_now));
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
    if gone {
        // Nothing of the patient shows, not even the name.
        return rsx! {
            div { class: "reading-workspace patient-page",
                StatusLine { message: PATIENT_GONE, alert: true }
                NavigationControl { onclick: move |_| on_go_today.call(()), "{GO_TO_TODAY}" }
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
                            Button {
                                compact: true,
                                class: "past-work-button",
                                onclick: move |_| on_open_past_work.call(()),
                                "Past work"
                            }
                        }
                        if is_admin && workspace.is_some() && !clinic_demographic {
                            Button {
                                compact: true,
                                danger: true,
                                class: "remove-panel-button",
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
                    if let Some(records) = records {
                        time { datetime: "{records.instant}", "{records.label} {records.words}" }
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
                Card {
                    aria_label: "Clinic demographic record",
                    mark: rsx! { StatePill { "Clinic record · Not on your panel" } },
                    div { class: "clinic-demographic-header",
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
                                ui::Select {
                                    label: "Physician panel",
                                    placeholder: "Choose a physician",
                                    selected: practice_accounts
                                        .iter()
                                        .position(|account| account.physician_id == target_physician_id),
                                    options: practice_accounts
                                        .iter()
                                        .map(|account| ui::Choice {
                                            label: format!("{} ({})", account.display_name, account.email),
                                            ..ui::Choice::default()
                                        })
                                        .collect::<Vec<_>>(),
                                    on_select: {
                                        let accounts = practice_accounts.clone();
                                        move |index: usize| {
                                            if let Some(account) = accounts.get(index) {
                                                on_select_physician.call(account.physician_id.clone());
                                            }
                                        }
                                    },
                                }
                            }
                            Button {
                                variant: ButtonVariant::Primary,
                                class: "assign-panel-button",
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
                            on_resume_reply,
                        }
                    }
                    for superseded in superseded_replies {
                        Card {
                            tinted: true,
                            "data-reply-state": "superseded_by_correction",
                            mark: rsx! { StatePill { "Superseded by corrected labs" } },
                            title: superseded.headline,
                            detail: superseded.superseded_reason,
                        }
                    }
                }
            }
            if let Some(run) = work {
                if let Some(heading) = work_heading(&run) {
                    section { class: "patient-section", aria_label: "Current work",
                        h2 { "Current work" }
                        NavigationRow {
                            class: "today-row-open",
                            onclick: {
                                let heading = heading.clone();
                                let preparation = visits
                                    .first()
                                    .and_then(|visit| visit.preparation.clone());
                                let ended_cause = run.ended_cause.clone();
                                move |_| {
                                    let body = preparation
                                        .clone()
                                        .or_else(|| work_end_reason(ended_cause.as_deref()).map(str::to_owned))
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
            if !active_plan_items.is_empty() {
                section { class: "patient-section", aria_label: "Active plan",
                    h2 { "Active plan" }
                    for item in active_plan_items {
                        Card {
                            "data-plan-item": "{item.item_id}",
                            title: item.title,
                            detail: item.detail,
                        }
                    }
                }
            }
            if !undecided_recommendations.is_empty() {
                section { class: "patient-section", aria_label: "Undecided medication recommendations",
                    h2 { "Undecided medication recommendations" }
                    for rec in undecided_recommendations {
                        Card {
                            "data-recommendation-id": "{rec.recommendation_id}",
                            title: format!("{} — Held (undecided)", rec.medication),
                            detail: rec.rationale,
                            footer: format!("Source: {}", rec.source_label),
                            ActionRow {
                                Button {
                                    compact: true,
                                    "data-decide-recommendation": "{rec.recommendation_id}",
                                    disabled: work_busy,
                                    onclick: {
                                        let id = rec.recommendation_id.clone();
                                        move |_| on_decide_recommendation.call(id.clone())
                                    },
                                    "Approve into plan"
                                }
                            }
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
            section { class: "patient-section", aria_label: "Conversation",
                h2 { "Conversation" }
                match chats {
                    ChatView::Closed => rsx! { EmptyLine { message: WORKSPACE_CLOSED.to_owned() } },
                    ChatView::Pending => rsx! {},
                    // The page's own read line says the patient couldn't open.
                    ChatView::Unavailable => rsx! {},
                    ChatView::Empty => rsx! {
                        p { class: "sidebar-empty", "{WORKSPACE_EMPTY}" }
                        Button {
                            compact: true,
                            onclick: move |_| on_new_chat.call(()),
                            Icon { name: IconName::Plus }
                            "{NEW_CONVERSATION}"
                        }
                    },
                    ChatView::Chats(lines) => rsx! {
                        for line in lines.iter().filter(|line| !line.archived).cloned() {
                            NavigationRow {
                                class: "sidebar-conversation-link",
                                onclick: {
                                    let id = line.id.clone();
                                    move |_| on_open_chat.call(id.clone())
                                },
                                Icon { name: IconName::Message }
                                "{line.title}"
                            }
                        }
                        if conversation_open {
                            if open_archived == Some(true) {
                                p { class: "quiet-line conversation-archived", role: "status", "{ARCHIVED_LINE}" }
                            } else if open_archived == Some(false) {
                                ActionRow {
                                    Button {
                                        compact: true,
                                        class: "conversation-archive",
                                        disabled: archive_busy || running,
                                        onclick: move |_| on_archive.call(()),
                                        Icon { name: IconName::Archive }
                                        "{ARCHIVE}"
                                    }
                                }
                            }
                            {archive_notice}
                        }
                        if conversation_open {
                            ReadNotice {
                                stage: conversation.stage(),
                                loading: CONVERSATION_LOADING,
                                stale: CONVERSATION_STALE,
                                unavailable: CONVERSATION_UNAVAILABLE,
                                retry: TRY_AGAIN,
                                on_retry: on_conversation_retry,
                            }
                        }
                        if conversation_open && conversation.content().is_some_and(Vec::is_empty) {
                            h3 { class: "conversation-empty", "{CONVERSATION_EMPTY}" }
                        }
                        for line in conversation.content().cloned().unwrap_or_default() {
                            if let Some(id) = line.queued {
                                QueuedLine {
                                    id,
                                    text: line.text,
                                    editing: queued_editing.clone(),
                                    busy: queued_busy,
                                    on_edit: on_queued_edit,
                                    on_cancel: on_queued_edit_cancel,
                                    on_save: on_queued_edit_save,
                                    on_withdraw: on_queued_withdraw,
                                }
                            } else if let Some(target) = line.stop {
                                div { class: "conversation-mechanical", role: "status",
                                    p { "{line.text}" }
                                    Button {
                                        compact: true,
                                        disabled: conversation_stop_busy,
                                        onclick: move |_| on_conversation_stop.call(target.clone()),
                                        "{STOP}"
                                    }
                                }
                            } else if let Some(target) = line.resume {
                                div { class: "conversation-mechanical", role: "status",
                                    p { "{line.text}" }
                                    Button {
                                        compact: true,
                                        disabled: conversation_resume_busy,
                                        onclick: move |_| on_conversation_resume.call(target.clone()),
                                        if line.held { "{RESUME}" } else { "{TRY_AGAIN}" }
                                    }
                                }
                            } else if line.author == Some(Author::Person) {
                                ui::Message { author: Author::Person, by: "You",
                                    LiteralWords { text: line.text }
                                }
                            } else if line.author == Some(Author::Assistant) {
                                ui::Message { author: Author::Assistant, by: "Coda",
                                    actions: feedback_chat.clone().zip(line.response).map(|(chat, (response, read))| rsx! {
                                        feedback::ResponseRating { chat, response, read }
                                    }),
                                    MessageWords { text: line.text }
                                }
                            } else {
                                MessageWords { text: line.text }
                            }
                        }
                        {conversation_resume_notice}
                        {conversation_stop_notice}
                        {queued_notice}
                        div { class: "composer-dock",
                            ui::Composer {
                                label: "Message",
                                placeholder: "Write a message",
                                value: draft.clone(),
                                on_input: move |text| on_draft.call(text),
                                on_submit: move |_| on_send.call(()),
                                on_editing: move |editing| {
                                    on_editing.call(editing);
                                    if !editing {
                                        on_draft_blur.call(());
                                    }
                                },
                                status: (!notice.is_empty()).then(|| rsx! {
                                    span { role: "status", "{notice}" }
                                    if notice == NOT_SAVED || notice == CLEAR_NOT_SAVED {
                                        Button { compact: true, onclick: move |_| on_conflict_retry.call(()), "{DRAFT_RETRY}" }
                                    }
                                }),
                                notices: conflict.map(|newer| rsx! {
                                    DraftConflict {
                                        message: DRAFT_CONFLICT,
                                        saved: newer.text,
                                        "data-revision": "{newer.revision}",
                                        p { class: "quiet-line",
                                            if clearing { "{CLEAR_CONFLICT_HELP}" } else { "{DRAFT_CONFLICT_HELP}" }
                                        }
                                        ActionRow {
                                            Button { compact: true, onclick: move |_| on_conflict_retry.call(()), "{DRAFT_RETRY}" }
                                            Button { compact: true, onclick: move |_| on_conflict_use_newer.call(()), "{DRAFT_USE_NEWER}" }
                                        }
                                    }
                                }),
                                Button { compact: true, onclick: move |_| on_dictate.call(()), "Dictate" }
                                Button { variant: ButtonVariant::Primary, compact: true, submit: true,
                                    if running { "{SEND_NEXT}" } else { "{SEND}" }
                                }
                            }
                        }
                        if recovery_unavailable {
                            p { class: "quiet-line composer-recovery", role: "status", "{RECOVERY_UNAVAILABLE}" }
                        }
                        {send_notice}
                    },
                }
            }
            }
            }
            if !visits.is_empty() || schedule_unanswered(schedule) {
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

/// A Send next message waiting for the running answer: an outlined bubble
/// with Edit and Remove until its turn starts.
#[component]
fn QueuedLine(
    id: String,
    text: String,
    /// The queued message being edited, and its words so far.
    editing: Option<(String, String)>,
    busy: bool,
    on_edit: EventHandler<(String, String)>,
    on_cancel: EventHandler,
    on_save: EventHandler,
    on_withdraw: EventHandler<String>,
) -> Element {
    let words = editing
        .filter(|(editing, _)| *editing == id)
        .map(|(_, words)| words);
    if let Some(words) = words {
        return rsx! {
            div { class: "queued-edit", "data-queued-id": "{id}",
                TextField {
                    label: QUEUED_EDIT_LABEL,
                    kind: TextKind::Multiline,
                    value: words,
                    on_input: {
                        let id = id.clone();
                        move |words: String| on_edit.call((id.clone(), words))
                    },
                }
                ActionRow {
                    Button { compact: true, disabled: busy, onclick: move |_| on_save.call(()), "{QUEUED_SAVE}" }
                    Button { compact: true, variant: ButtonVariant::Quiet, onclick: move |_| on_cancel.call(()), "{QUEUED_CANCEL}" }
                }
            }
        };
    }
    rsx! {
        ui::Message { author: Author::Person, by: "You", state: ui::MessageState::Queued,
            "data-queued-id": "{id}",
            status: rsx! {
                ui::MessageStatus { words: QUEUED_WAITING, icon: IconName::Clock,
                    Button {
                        variant: ButtonVariant::Quiet,
                        compact: true,
                        disabled: busy,
                        onclick: {
                            let (id, text) = (id.clone(), text.clone());
                            move |_| on_edit.call((id.clone(), text.clone()))
                        },
                        "{QUEUED_EDIT}"
                    }
                    Button {
                        variant: ButtonVariant::Quiet,
                        compact: true,
                        disabled: busy,
                        onclick: {
                            let id = id.clone();
                            move |_| on_withdraw.call(id.clone())
                        },
                        "{QUEUED_REMOVE}"
                    }
                }
            },
            LiteralWords { text }
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
        Dialog { close_label: "Close", title: "Find patient", on_close,
                div { class: "patient-search",
                    SearchField {
                        label: "Search your panel",
                        placeholder: "Name, DOB or phone",
                        clear_label: "Clear search",
                        value: query.clone(),
                        autofocus: true,
                        onfocus: move |_| on_editing.call(true),
                        onblur: move |_| on_editing.call(false),
                        on_input: move |words| on_query.call(words),
                        onkeydown: move |event: KeyboardEvent| {
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
                        EmptyLine { message: SEARCH_NONE }
                    }
                    if !results.is_empty() {
                        ui::RowGroup { class: "dialog-results",
                            for (index, patient) in results.iter().enumerate() {
                                li {
                                    ui::ListRow {
                                        class: if focused == Some(index as i32) { "is-keyboard-focus" } else { "" },
                                        title: patient.patient_display_name.clone(),
                                        detail: format!(
                                            "DOB {}",
                                            patient
                                                .birth_date
                                                .as_deref()
                                                .map(format_calendar_date)
                                                .unwrap_or_else(|| "not recorded".to_owned())
                                        ),
                                        onclick: {
                                            let id = patient.patient_workspace_id.clone();
                                            move |_| on_open.call(id.clone())
                                        },
                                    }
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
        Dialog { close_label: "Close", title: CONNECTIONS, on_close,
                if failed {
                    p { class: "dialog-note", "{TODAY_UNAVAILABLE}" }
                } else if lines.is_empty() {
                    p { class: "dialog-note", "Nothing needs you right now." }
                } else {
                    for line in lines {
                        p { class: "today-connection", "{line}" }
                    }
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
    let label = chip_name(&chip);
    let index = chip.index;
    if chip.open {
        rsx! {
            ui::CitationChip { index, label, onclick: move |_| on_open.call(chip.clone()) }
        }
    } else {
        rsx! {
            ui::CitationChip { index, label }
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
        "reader citation-inspection"
    } else {
        "reader"
    };
    let title = reading.title.clone();
    let eyebrow = reading.eyebrow.clone();
    let provenance = reading.provenance.clone();
    let unavailable = reading.unavailable.clone();
    let link_label = reading.link_label.clone();
    let link_uri = reading.link_uri.clone();
    rsx! {
        Dialog { close_label: "Close", class: "{section_class}", title, subtitle: eyebrow, on_close,
                div { class: "reader-content",
                    if quiet {
                        p { class: "dialog-note", "Nothing is open to read." }
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
                }
        }
    }
}

#[component]
fn PastWorkDialog(patient: String, on_close: EventHandler) -> Element {
    rsx! {
        Dialog {
            close_label: "Close",
            class: "past-work-dialog",
            title: "Past work",
            subtitle: (!patient.is_empty()).then_some(patient),
            on_close,
            p { class: "past-work-empty", "{PAST_WORK_EMPTY}" }
        }
    }
}

/// What Coda says in a chat snapshot, drawn richly. This doesn't fetch a source or run a model.
#[component]
fn MessageWords(text: String) -> Element {
    rsx! {
        ui::Prose {
            message: true,
            names: ui::ProseNames { code: "Code".to_owned(), table: "Table".to_owned() },
            parts: vec![ui::ProsePart::Words(text)],
        }
    }
}

/// Words that read exactly as written: a physician's own, which aren't Markdown, and a reply draft,
/// which goes to a patient as plain text, so the physician reviews what Copy sends.
#[component]
fn LiteralWords(text: String) -> Element {
    rsx! {
        p { class: "literal-words", "{text}" }
    }
}

#[component]
fn ReplyReviewCard(
    reply: WorkspaceReply,
    busy: bool,
    on_editing: EventHandler<bool>,
    on_close_reply: EventHandler<String>,
    on_revise_reply: EventHandler<(String, u64, String)>,
    on_resume_reply: EventHandler<WorkResumeTarget>,
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
    // A revision names the exact revision on screen, never a fresh read.
    let shown_revision = reply.draft_revision;
    let run_status = reply
        .revision_run
        .as_ref()
        .map(|run| run.status.clone())
        .unwrap_or_default();
    let resume_target = reply
        .revision_run
        .as_ref()
        .and_then(|run| run.resume_target.clone());
    let run_words = reply_revision_words(&reply);
    let can_copy = actions.contains(&ReplyReviewAction::Copy);
    let can_close = actions.contains(&ReplyReviewAction::NoReplyNeeded);
    let can_revise = actions.contains(&ReplyReviewAction::Revise) && shown_revision.is_some();
    let revision_attr = shown_revision.map(|r| r.to_string()).unwrap_or_default();
    let label = format!("Replying in {channel}");
    let mut ask = move |reply_id: String| {
        let asked = instruction.read().trim().to_owned();
        let Some(revision) = shown_revision else {
            return;
        };
        if asked.is_empty() || busy {
            return;
        }
        on_revise_reply.call((reply_id, revision, asked));
        instruction.set(String::new());
    };
    rsx! {
        Card {
            class: "reply-review",
            "data-reply-state": "{state}",
            "data-draft-revision": "{revision_attr}",
            "data-revision-status": "{run_status}",
            aria_label: "{label}",
            mark: rsx! { StatePill { tone: Tone::Warning, "{urgency}" } },
            title: headline,
            detail: label.clone(),
            LiteralWords { text: draft.clone() }
            if let Some(words) = run_words {
                p {
                    class: "composer-notice",
                    role: "status",
                    "data-reply-revision": "{run_status}",
                    "{words}"
                }
            }
            if let Some(target) = resume_target {
                Button {
                    compact: true,
                    "data-reply-action": "try-again",
                    disabled: busy,
                    onclick: move |_| on_resume_reply.call(target.clone()),
                    "Try again"
                }
            }
            if can_copy || can_close {
                div { class: "reply-review-actions",
                    if can_copy {
                        Button {
                            variant: ButtonVariant::Primary,
                            "data-reply-action": "copy",
                            disabled: busy,
                            onclick: move |_| {
                                let draft = draft.clone();
                                spawn(async move {
                                    let copy = ui::copy_text(draft).await;
                                    copied.set(match copy {
                                        Ok(()) => "Copied".to_owned(),
                                        Err(_) => "Couldn’t copy".to_owned(),
                                    });
                                });
                            },
                            "{REPLY_COPY}"
                        }
                    }
                    if can_close {
                        Button {
                            compact: true,
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
                    ui::CompactField {
                        class: "reply-review-instruction",
                        label: REPLY_TELL_CODA,
                        disabled: busy,
                        value: instruction(),
                        onfocus: move |_| on_editing.call(true),
                        onblur: move |_| on_editing.call(false),
                        on_input: move |words| instruction.set(words),
                        onkeydown: {
                            let reply_id = reply_id.clone();
                            move |event: KeyboardEvent| {
                                if event.key() != Key::Enter {
                                    return;
                                }
                                event.prevent_default();
                                ask(reply_id.clone());
                            }
                        },
                    }
                    if !instruction.read().trim().is_empty() && !busy {
                        Button {
                            compact: true,
                            "data-reply-action": "revise",
                            aria_label: "{REPLY_ASK}",
                            onclick: {
                                let reply_id = reply_id.clone();
                                move |_| ask(reply_id.clone())
                            },
                            "{REPLY_ASK}"
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn ShortcutsDialog(on_close: EventHandler) -> Element {
    rsx! {
        Dialog { close_label: "Close", title: "Keyboard shortcuts", on_close,
                div { class: "shortcut", span { "Search your panel" } kbd { "/  ⌘ K" } }
                div { class: "shortcut", span { "Move between patients or conversations" } kbd { "J  K" } }
                div { class: "shortcut", span { "Open the focused item" } kbd { "Enter" } }
                div { class: "shortcut", span { "Update the response in progress" } kbd { "⌘ Enter" } }
                div { class: "shortcut", span { "Close the current preview or dialog" } kbd { "Esc" } }
                div { class: "shortcut", span { "Hold to record; release to transcribe" } kbd { "Ctrl+Shift+Space" } }
                p { class: "dialog-note", "Navigation shortcuts pause while you are typing. Dictation works in text fields while this tab has focus. Esc cancels dictation." }
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

/// How the server answered a draft save. Only a validated conflict body
/// names a newer revision; a bare or malformed 409 (a read-only projection,
/// an authority refusal) is a failure and never authorizes an overwrite.
fn save_outcome(answer: Result<DraftSave, physician_client::HttpError>) -> SaveOutcome {
    match answer {
        Ok(DraftSave::Saved(saved)) => SaveOutcome::Saved(saved.revision),
        Ok(DraftSave::Conflict(current)) => SaveOutcome::Conflict(current.revision, current.text),
        Err(_) => SaveOutcome::Failed,
    }
}

/// The words beside the composer for its notice. A failed clear after an
/// accepted Send says the message went; only the draft cleanup is pending.
fn notice_words(notice: Notice, clearing: bool) -> String {
    match notice {
        Notice::None | Notice::Conflict => String::new(),
        Notice::NotSaved if clearing => CLEAR_NOT_SAVED.to_owned(),
        Notice::NotSaved => NOT_SAVED.to_owned(),
        Notice::Said(words) => words.to_owned(),
    }
}

#[cfg(test)]
mod draft_notice_tests {
    use super::*;

    #[test]
    fn routine_draft_saves_are_silent_but_failures_remain_visible() {
        for clearing in [false, true] {
            for notice in [Notice::None, Notice::Conflict] {
                assert_eq!(notice_words(notice, clearing), "");
            }
        }
        assert_eq!(notice_words(Notice::NotSaved, false), NOT_SAVED);
        assert_eq!(notice_words(Notice::NotSaved, true), CLEAR_NOT_SAVED);
        assert_eq!(
            notice_words(Notice::Said(DICTATION_UNAVAILABLE), false),
            DICTATION_UNAVAILABLE
        );
    }
}

/// How long typing must pause before the composer saves.
const AUTOSAVE_DEBOUNCE_MS: u32 = 800;
/// While a reply revision runs, the workspace is reread this often in case
/// its completion notification was missed...
const REVISION_POLL_MS: u32 = 3_000;
/// ...at most this many times per run; events and resync still apply after.
const REVISION_POLL_LIMIT: u32 = 40;
/// Failed saves retry on their own this many times, slower each time; after
/// that the next edit tries again. Never a tight loop.
const AUTOSAVE_RETRIES: u32 = 5;

/// The wait before the slow retry of a save that failed `failures` times in
/// a row, or `None` once automatic retries are spent.
fn autosave_retry_ms(failures: u32) -> Option<u32> {
    (1..=AUTOSAVE_RETRIES)
        .contains(&failures)
        .then(|| 2_000u32 << (failures - 1))
}

/// Clinical access can be withdrawn with a successful demographic-only read,
/// not just a 404. Drop memory and same-tab recovery together.
fn forget_workspace_drafts(
    mut composer: Signal<Composer>,
    client: &PhysicianClient,
    whoami: Signal<Option<WhoamiResponse>>,
    workspace: &str,
) {
    composer.write().forget_workspace(workspace);
    #[cfg(target_arch = "wasm32")]
    if let Some(account) = whoami.peek().as_ref()
        && let Some(scope) =
            local_draft::Scope::new(client.base(), &account.practice_id, &account.physician_id)
        && let Some(storage) = local_draft::SessionStorage::open()
    {
        local_draft::forget_workspace(&storage, &scope, workspace);
    }
    #[cfg(not(target_arch = "wasm32"))]
    let _ = (client, whoami);
}

/// Waits `ms` milliseconds without blocking the page.
#[cfg(target_arch = "wasm32")]
async fn sleep_ms(ms: u32) {
    let Some(window) = web_sys::window() else {
        return;
    };
    let wait = js_sys::Promise::new(&mut |resolve, _reject| {
        let _ = window.set_timeout_with_callback_and_timeout_and_arguments_0(
            &resolve,
            ms.min(i32::MAX as u32) as i32,
        );
    });
    let _ = wasm_bindgen_futures::JsFuture::from(wait).await;
}

/// Waits `ms` milliseconds on a helper thread (native builds).
#[cfg(not(target_arch = "wasm32"))]
async fn sleep_ms(ms: u32) {
    use std::sync::{Arc, Mutex};
    use std::task::{Poll, Waker};
    let state: Arc<Mutex<(bool, Option<Waker>)>> = Arc::default();
    let timer = Arc::clone(&state);
    std::thread::spawn(move || {
        std::thread::sleep(std::time::Duration::from_millis(u64::from(ms)));
        let mut state = timer.lock().expect("timer");
        state.0 = true;
        if let Some(waker) = state.1.take() {
            waker.wake();
        }
    });
    std::future::poll_fn(move |context| {
        let mut state = state.lock().expect("timer");
        if state.0 {
            Poll::Ready(())
        } else {
            state.1 = Some(context.waker().clone());
            Poll::Pending
        }
    })
    .await;
}

/// Saves `target` once it has gone `ms` without an edit. A later edit makes
/// this timer obsolete (it then does nothing); the shell's scope owns the
/// task, so leaving the signed-in shell cancels it.
fn autosave_after(
    composer: Signal<Composer>,
    client: Signal<PhysicianClient>,
    target: DraftTarget,
    ms: u32,
) {
    let generation = composer.peek().generation(&target);
    spawn(async move {
        sleep_ms(ms).await;
        if composer.peek().generation(&target) == generation {
            autosave(composer, client, target);
        }
    });
}

/// Starts `target`'s save now when it has unsaved text and nothing stops it
/// (a running save, a conflict, a failure waiting on its retry). The request
/// keeps the session it began with. When it ends, edits made meanwhile get a
/// trailing save, and a failure retries slowly a few times.
fn autosave(composer: Signal<Composer>, client: Signal<PhysicianClient>, target: DraftTarget) {
    let mut held = composer;
    let Some(attempt) = held.write().begin(&target) else {
        return;
    };
    let saver = client.peek().pinned();
    spawn(async move {
        let outcome = save_outcome(
            saver
                .save_draft(
                    &attempt.target.workspace,
                    &attempt.target.chat,
                    attempt.expected_revision,
                    &attempt.text,
                )
                .await,
        );
        held.write().finish(&attempt, outcome);
        let failing = composer.peek().failing(&target);
        if composer.peek().wants_save(&target) {
            autosave_after(composer, client, target, AUTOSAVE_DEBOUNCE_MS);
        } else if let Some(failures) = failing
            && let Some(wait) = autosave_retry_ms(failures)
        {
            spawn(async move {
                sleep_ms(wait).await;
                if composer.peek().failing(&target) == Some(failures) {
                    held.write().retry_failed(&target);
                    autosave(composer, client, target);
                }
            });
        }
    });
}

/// Sends a captured message. Acceptance removes only the submitted text and
/// queues its durable clear; refusal keeps the text as an ordinary draft; no
/// answer leaves the Send pending, and Check again (or a reload) asks again
/// with the same command and key.
fn dispatch_send(
    slot: CommandSlot,
    client: Signal<PhysicianClient>,
    composer: Signal<Composer>,
    mut snapshot_attempt: Signal<u32>,
    target: DraftTarget,
    pending: composer::PendingSend,
) {
    let key = pending.key.clone();
    let accepted = {
        let (target, key) = (target.clone(), key.clone());
        Callback::new(move |()| {
            let mut held = composer;
            held.write().sent(&target, &key, SendAnswer::Accepted);
            *snapshot_attempt.write() += 1;
            autosave(composer, client, target.clone());
        })
    };
    let refused = Callback::new(move |()| {
        let mut held = composer;
        held.write().sent(&target, &key, SendAnswer::Refused);
    });
    slot.run_answered(
        client.peek().pinned(),
        Command(pending.command),
        accepted,
        refused,
    );
}

/// A per-document id: records this document writes are its own.
#[cfg(target_arch = "wasm32")]
fn writer_id() -> String {
    static NEXT: AtomicU64 = AtomicU64::new(1);
    let n = NEXT.fetch_add(1, Ordering::Relaxed);
    format!("doc-{}-{n}", now_millis())
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

/// One line of the open conversation. A turn Coda couldn't finish carries the
/// work and run its Try again resumes. A running turn carries Stop, and a
/// stopped turn carries Resume.
#[derive(Debug, Clone, PartialEq)]
struct ConversationLine {
    text: String,
    /// Who wrote it, when the server says: the physician or Coda. A turn that
    /// doesn't say shows without a speaker rather than as either.
    author: Option<Author>,
    resume: Option<WorkResumeTarget>,
    stop: Option<WorkStopTarget>,
    /// This resume is an intentional Stop. A failed turn's resume is Try again.
    held: bool,
    /// On one of Coda's published responses: its response id, which
    /// feedback names, and the physician's feedback on it as the snapshot
    /// read it.
    response: Option<(ResponseId, feedback::Read)>,
    /// A Send next message waiting for the running answer, named by the id
    /// its Edit and Remove send. Only the physician's own words are queued.
    queued: Option<String>,
}

/// The lines a chat snapshot shows, in order. Only a mechanical line keeps a
/// resume target; turns with no text are skipped.
fn conversation_lines(snapshot: &ChatSnapshot) -> Vec<ConversationLine> {
    snapshot
        .conversation
        .iter()
        .filter_map(|turn| {
            let text = turn.body()?.to_owned();
            let mechanical = turn.kind.as_deref() == Some("mechanical");
            let stop = turn.stop_target.clone().filter(|_| mechanical);
            let resume = turn
                .resume_target
                .clone()
                .filter(|_| mechanical && stop.is_none());
            let held = resume.as_ref().is_some_and(|target| {
                snapshot.runs.iter().any(|run| {
                    run.run_id == target.expected_run_id
                        && run.outcome
                            == ChatRunOutcome::Held {
                                hold_reason: HoldReason::Stopped,
                            }
                })
            });
            let author = match turn.kind.as_deref() {
                Some("physician") => Some(Author::Person),
                Some("model") => Some(Author::Assistant),
                _ => None,
            };
            let response = turn
                .response_id
                .clone()
                .filter(|_| author == Some(Author::Assistant))
                .map(|id| {
                    let read = match snapshot.feedback {
                        SnapshotFeedback::Loaded => feedback::Read::Loaded(turn.feedback.clone()),
                        SnapshotFeedback::Unavailable => feedback::Read::Unavailable,
                    };
                    (id, read)
                });
            Some(ConversationLine {
                text,
                author,
                resume,
                stop,
                held,
                response,
                queued: None,
            })
        })
        .chain(snapshot.queued.iter().map(|queued| ConversationLine {
            text: queued.text.clone(),
            author: Some(Author::Person),
            resume: None,
            stop: None,
            held: false,
            response: None,
            queued: Some(queued.message_id.clone()),
        }))
        .collect()
}

/// Archive or its Undo on one chat.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ArchiveAction {
    /// `conversation.archive`: the chat leaves the list.
    Archive,
    /// `conversation.restore`: Undo brings it back.
    Restore,
}

impl ArchiveAction {
    /// The command type the server names this action by.
    fn wire(self) -> &'static str {
        match self {
            Self::Archive => "conversation.archive",
            Self::Restore => "conversation.restore",
        }
    }
}

/// Archive or its Undo on one chat. Each press is its own command, so an
/// Archive asked again after a lost answer never undoes a later Undo.
fn archive_command(action: ArchiveAction, workspace_id: &str, chat_id: &str) -> Command {
    Command::new(
        action.wire(),
        serde_json::json!({
            "patient_workspace_id": workspace_id,
            "chat_id": chat_id,
        }),
    )
}

/// Whether Coda is writing in this conversation, so Send waits as Send next.
fn conversation_running(lines: &[ConversationLine]) -> bool {
    lines.iter().any(|line| line.stop.is_some())
}

/// Edit on a queued message. Each save is its own command; Check again
/// reuses it, so a lost answer never applies a second, different edit.
fn queued_edit_command(
    workspace_id: Option<String>,
    chat_id: Option<String>,
    message_id: &str,
    text: &str,
) -> Option<Command> {
    let (workspace_id, chat_id) = (workspace_id?, chat_id?);
    if text.trim().is_empty() {
        return None;
    }
    Some(Command::new(
        "message.edit",
        serde_json::json!({
            "patient_workspace_id": workspace_id,
            "chat_id": chat_id,
            "message_id": message_id,
            "text": text,
        }),
    ))
}

/// Remove on a queued message. Its key names the message: removing it again
/// is the same request.
fn queued_withdraw_command(
    workspace_id: Option<String>,
    chat_id: Option<String>,
    message_id: &str,
) -> Option<Command> {
    let (workspace_id, chat_id) = (workspace_id?, chat_id?);
    Some(Command::keyed(
        "message.withdraw",
        format!("withdraw-{message_id}"),
        serde_json::json!({
            "patient_workspace_id": workspace_id,
            "chat_id": chat_id,
            "message_id": message_id,
        }),
    ))
}

/// Try again on chat or reply-revision work. Like the Today row, its key
/// names the run, so pressing again for the same run is the same request.
fn chat_resume_command(
    workspace_id: Option<String>,
    chat_id: Option<String>,
    target: &WorkResumeTarget,
) -> Option<Command> {
    let (workspace_id, chat_id) = (workspace_id?, chat_id?);
    Some(Command::keyed(
        "work.resume",
        format!("resume-{}-{}", target.work_item_id, target.expected_run_id),
        serde_json::json!({
            "patient_workspace_id": workspace_id,
            "chat_id": chat_id,
            "work_item_id": target.work_item_id,
            "expected_run_id": target.expected_run_id,
        }),
    ))
}

/// Stop on the chat turn Coda is writing. Resume keeps the work's identity,
/// so each press is its own command; a key that named the work would make a
/// second Stop after Resume a replay of the first, which leaves the run going.
fn chat_stop_command(
    workspace_id: Option<String>,
    chat_id: Option<String>,
    target: &WorkStopTarget,
) -> Option<Command> {
    let (workspace_id, chat_id) = (workspace_id?, chat_id?);
    Some(Command::new(
        "work.stop",
        serde_json::json!({
            "patient_workspace_id": workspace_id,
            "chat_id": chat_id,
            "work_item_id": target.work_item_id,
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
    format!("submit-{}-{n}", now_millis())
}

/// A conversation id the page picks itself, built like [`idempotency_key`].
/// The server keeps a chat once its first message is accepted.
fn new_chat_id() -> String {
    static NEXT: AtomicU64 = AtomicU64::new(1);
    let n = NEXT.fetch_add(1, Ordering::Relaxed);
    format!("chat-{}-{n}", now_millis())
}

/// Puts the open patient and conversation in the address, without a history
/// entry, so a reload reopens the same chat.
#[cfg(target_arch = "wasm32")]
fn write_address(patient: Option<&str>, chat: Option<&str>) {
    if let Some(window) = web_sys::window()
        && let Ok(href) = window.location().href()
        && let Ok(url) = web_sys::Url::new(&href)
    {
        match patient {
            Some(patient) => url.search_params().set("patient", patient),
            None => url.search_params().delete("patient"),
        }
        match chat.filter(|_| patient.is_some()) {
            Some(chat) => url.search_params().set("chat", chat),
            None => url.search_params().delete("chat"),
        }
        if let Ok(history) = window.history() {
            let _ =
                history.replace_state_with_url(&wasm_bindgen::JsValue::NULL, "", Some(&url.href()));
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn write_address(_patient: Option<&str>, _chat: Option<&str>) {}

/// Milliseconds since the epoch. `std::time::SystemTime::now` panics on
/// wasm32-unknown-unknown, so the browser build asks the page's clock.
#[cfg(target_arch = "wasm32")]
fn now_millis() -> u128 {
    js_sys::Date::now() as u128
}

#[cfg(not(target_arch = "wasm32"))]
fn now_millis() -> u128 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_millis())
        .unwrap_or(0)
}

#[cfg(test)]
mod conversation_line_tests {
    use super::{
        ArchiveAction, ConversationLine, archive_command, chat_resume_command, chat_stop_command,
        conversation_lines, conversation_running, message_command, new_chat_id,
        queued_edit_command, queued_withdraw_command,
    };
    use crate::feedback;
    use crate::shell::STOPPED;
    use physician_wire::{
        ChatRunOutcome, ChatSnapshotResponse, HoldReason, ResponseFeedback, ResponseId,
        WorkResumeTarget, WorkStopTarget,
    };
    use ui::Author;

    #[test]
    fn queued_messages_follow_the_conversation_with_edit_and_remove() {
        let page: ChatSnapshotResponse = serde_json::from_value(serde_json::json!({
            "snapshot": {
                "conversation": [
                    { "kind": "physician", "text": "Hello" },
                    { "kind": "mechanical", "text": "Coda is writing this response.",
                      "stop_target": { "work_item_id": "work-1" } },
                ],
                "runs": [],
                "queued": [
                    { "message_id": "submit-1", "text": "And the potassium?" },
                    { "message_id": "submit-2", "text": "Also the creatinine" },
                ],
            }
        }))
        .unwrap();
        let lines = conversation_lines(&page.snapshot);
        assert!(conversation_running(&lines));
        let queued: Vec<_> = lines
            .iter()
            .filter_map(|line| Some((line.queued.as_deref()?, line.text.as_str(), line.author)))
            .collect();
        assert_eq!(
            queued,
            [
                ("submit-1", "And the potassium?", Some(Author::Person)),
                ("submit-2", "Also the creatinine", Some(Author::Person)),
            ]
        );
        // Queued lines come after the running answer, never before it.
        assert!(lines[1].stop.is_some() && lines[2].queued.is_some());
        let done: ChatSnapshotResponse = serde_json::from_value(serde_json::json!({
            "snapshot": { "conversation": [{ "kind": "physician", "text": "Hello" }], "runs": [] }
        }))
        .unwrap();
        assert!(!conversation_running(&conversation_lines(&done.snapshot)));
    }

    #[test]
    fn edit_and_remove_name_the_queued_message() {
        let (ws, chat) = (Some("pw-1".to_owned()), Some("chat-1".to_owned()));
        let edit = queued_edit_command(ws.clone(), chat.clone(), "submit-1", "New").unwrap();
        assert_eq!(edit.0["type"], "message.edit");
        assert_eq!(edit.0["payload"]["message_id"], "submit-1");
        assert_eq!(edit.0["payload"]["text"], "New");
        let again = queued_edit_command(ws.clone(), chat.clone(), "submit-1", "New").unwrap();
        assert_ne!(edit.key(), again.key(), "each save is its own edit");
        assert!(queued_edit_command(ws.clone(), chat.clone(), "submit-1", "  ").is_none());
        let remove = queued_withdraw_command(ws.clone(), chat.clone(), "submit-1").unwrap();
        assert_eq!(remove.0["type"], "message.withdraw");
        assert_eq!(remove.0["payload"]["message_id"], "submit-1");
        assert!(remove.0["payload"].get("text").is_none());
        let twice = queued_withdraw_command(ws, chat.clone(), "submit-1").unwrap();
        assert_eq!(
            remove.key(),
            twice.key(),
            "Remove again is the same request"
        );
        assert!(queued_withdraw_command(None, chat, "submit-1").is_none());
        // Send asks for Send next; the server queues it only while Coda writes.
        let send = message_command(
            Some("pw-1".into()),
            Some("chat-1".into()),
            Some("America/New_York".into()),
            "Hi".into(),
        )
        .unwrap();
        assert_eq!(send.0["payload"]["intent"], "queue");
    }

    #[test]
    fn new_conversation_ids_are_distinct_chat_ids() {
        let (first, second) = (new_chat_id(), new_chat_id());
        assert!(first.starts_with("chat-") && second.starts_with("chat-"));
        assert_ne!(first, second);
    }

    fn target() -> WorkResumeTarget {
        WorkResumeTarget {
            work_item_id: "work-1".into(),
            expected_run_id: "run-1".into(),
        }
    }

    #[test]
    fn a_couldnt_finish_line_keeps_the_run_its_try_again_resumes() {
        let page: ChatSnapshotResponse = serde_json::from_value(serde_json::json!({
            "snapshot": {
                "conversation": [
                    { "kind": "physician", "text": "Hello" },
                    { "kind": "model", "text": "",
                      "resume_target": { "work_item_id": "x", "expected_run_id": "y" } },
                    { "kind": "mechanical",
                      "text": "Coda couldn’t finish this response. Your message is saved.",
                      "resume_target": { "work_item_id": "work-1", "expected_run_id": "run-1" } },
                ],
                "runs": [{ "run_id": "run-1", "state": "failed" }],
            }
        }))
        .unwrap();
        assert_eq!(
            conversation_lines(&page.snapshot),
            vec![
                ConversationLine {
                    text: "Hello".into(),
                    author: Some(Author::Person),
                    resume: None,
                    stop: None,
                    held: false,
                    response: None,
                    queued: None,
                },
                ConversationLine {
                    text: "Coda couldn’t finish this response. Your message is saved.".into(),
                    author: None,
                    resume: Some(target()),
                    stop: None,
                    held: false,
                    response: None,
                    queued: None,
                },
            ]
        );
    }

    #[test]
    fn only_a_turn_that_names_its_writer_gets_a_speaker() {
        let page: ChatSnapshotResponse = serde_json::from_value(serde_json::json!({
            "snapshot": { "runs": [], "conversation": [
                { "kind": "physician", "text": "Start metoprolol?" },
                { "kind": "model", "text": "Her heart rate allows it." },
                { "text": "Recorded work status." },
            ] }
        }))
        .unwrap();
        let authors: Vec<_> = conversation_lines(&page.snapshot)
            .into_iter()
            .map(|line| line.author)
            .collect();
        assert_eq!(
            authors,
            vec![Some(Author::Person), Some(Author::Assistant), None]
        );
    }

    #[test]
    fn only_codas_responses_carry_the_response_id_feedback_names() {
        let page: ChatSnapshotResponse = serde_json::from_value(serde_json::json!({
            "snapshot": { "runs": [], "conversation": [
                { "kind": "physician", "text": "Start metoprolol?", "response_id": "r-0" },
                { "kind": "model", "text": "Her heart rate allows it.", "response_id": "r-1",
                  "feedback": { "revision": 2, "rating": "up" } },
                { "kind": "model", "text": "From an older server.",
                  "feedback": { "revision": 1, "rating": "down" } },
                { "text": "Recorded in local simulation.", "response_id": "r-3" },
            ] }
        }))
        .unwrap();
        let named: Vec<_> = conversation_lines(&page.snapshot)
            .into_iter()
            .map(|line| line.response.map(|(id, read)| (id.0, read)))
            .collect();
        // Feedback rides only on a response that names its response id.
        assert_eq!(
            named,
            vec![
                None,
                Some((
                    "r-1".to_owned(),
                    feedback::Read::Loaded(Some(ResponseFeedback {
                        revision: 2,
                        rating: physician_wire::FeedbackRating::Up,
                        details: None,
                    }))
                )),
                None,
                None
            ]
        );
    }

    #[test]
    fn unavailable_feedback_keeps_every_line_and_marks_each_response() {
        let page: ChatSnapshotResponse = serde_json::from_value(serde_json::json!({
            "snapshot": { "runs": [], "feedback": "unavailable", "conversation": [
                { "kind": "physician", "text": "Start metoprolol?" },
                { "kind": "model", "text": "Her heart rate allows it.", "response_id": "r-1" },
            ] }
        }))
        .unwrap();
        let lines = conversation_lines(&page.snapshot);
        assert_eq!(lines.len(), 2);
        assert_eq!(lines[1].text, "Her heart rate allows it.");
        assert_eq!(lines[0].response, None);
        assert_eq!(
            lines[1].response,
            Some((ResponseId("r-1".into()), feedback::Read::Unavailable))
        );
    }

    #[test]
    fn only_a_mechanical_line_offers_try_again() {
        let page: ChatSnapshotResponse = serde_json::from_value(serde_json::json!({
            "snapshot": { "runs": [], "conversation": [
                { "kind": "model", "text": "Done",
                  "resume_target": { "work_item_id": "work-1", "expected_run_id": "run-1" } },
                { "text": "Older server line" },
            ] }
        }))
        .unwrap();
        assert!(
            conversation_lines(&page.snapshot)
                .iter()
                .all(|line| line.resume.is_none() && line.stop.is_none())
        );
    }

    #[test]
    fn a_writing_line_offers_stop_and_a_stopped_line_offers_resume() {
        let writing: ChatSnapshotResponse = serde_json::from_value(serde_json::json!({
            "snapshot": { "runs": [], "conversation": [
                { "kind": "physician", "text": "Hello" },
                { "kind": "mechanical", "text": "Coda is writing this response.",
                  "stop_target": { "work_item_id": "work-1" } },
            ] }
        }))
        .unwrap();
        let lines = conversation_lines(&writing.snapshot);
        assert_eq!(
            lines[1].stop,
            Some(WorkStopTarget {
                work_item_id: "work-1".into(),
            })
        );
        assert!(lines[1].resume.is_none());
        let stopped: ChatSnapshotResponse = serde_json::from_value(serde_json::json!({
            "snapshot": {
                "conversation": [
                    { "kind": "mechanical",
                      "text": "Coda stopped this response. Your message is saved.",
                      "resume_target": { "work_item_id": "work-1", "expected_run_id": "run-1" } },
                ],
                "runs": [{ "run_id": "run-1", "state": "held", "hold_reason": "stopped" }],
            }
        }))
        .unwrap();
        let lines = conversation_lines(&stopped.snapshot);
        assert_eq!(lines[0].text, STOPPED);
        assert_eq!(lines[0].resume, Some(target()));
        assert!(lines[0].held);
        assert!(lines[0].stop.is_none());
        let mut limited = stopped.snapshot;
        limited.runs[0].outcome = ChatRunOutcome::Held {
            hold_reason: HoldReason::Limit,
        };
        let limited = conversation_lines(&limited);
        assert_eq!(limited[0].resume, Some(target()));
        assert!(
            !limited[0].held,
            "a limit offers Try again, never Stop's Resume"
        );
    }

    #[test]
    fn a_hold_reason_this_app_does_not_know_reads_like_a_limit() {
        let held: ChatSnapshotResponse = serde_json::from_value(serde_json::json!({
            "snapshot": {
                "conversation": [
                    { "kind": "mechanical",
                      "text": "Coda paused this response. Choose Try again to continue.",
                      "resume_target": { "work_item_id": "work-1", "expected_run_id": "run-1" } },
                ],
                "runs": [{ "run_id": "run-1", "state": "held", "hold_reason": "a_reason_from_a_newer_site" }],
            }
        }))
        .unwrap();
        let lines = conversation_lines(&held.snapshot);
        assert_eq!(
            lines[0].text,
            "Coda paused this response. Choose Try again to continue."
        );
        assert_eq!(lines[0].resume, Some(target()));
        assert!(
            !lines[0].held,
            "an unknown hold offers Try again, never Stop's Resume"
        );
        assert!(lines[0].stop.is_none());
    }

    #[test]
    fn a_recovered_archive_names_the_chat_its_undo_restores() {
        let archive = archive_command(ArchiveAction::Archive, "pw-1", "chat-1");
        assert_eq!(archive.0["type"], "conversation.archive");
        assert_eq!(
            archive.archive_target(),
            Some(("pw-1".to_owned(), "chat-1".to_owned()))
        );
        let undo = archive_command(ArchiveAction::Restore, "pw-1", "chat-1");
        assert_eq!(undo.0["type"], "conversation.restore");
        assert_eq!(undo.archive_target(), None);
    }

    #[test]
    fn each_stop_press_is_its_own_command_for_the_named_work() {
        let target = WorkStopTarget {
            work_item_id: "work-1".into(),
        };
        let command =
            chat_stop_command(Some("pw-1".into()), Some("chat-1".into()), &target).unwrap();
        assert_eq!(command.0["type"], "work.stop");
        assert_eq!(
            command.0["payload"],
            serde_json::json!({
                "patient_workspace_id": "pw-1",
                "chat_id": "chat-1",
                "work_item_id": "work-1",
            })
        );
        // Resume keeps the work's identity, so a Stop after Resume must not
        // reuse the earlier Stop's key: the server would answer it as that
        // earlier, already-taken Stop and leave the resumed turn running.
        let again = chat_stop_command(Some("pw-1".into()), Some("chat-1".into()), &target).unwrap();
        assert_ne!(command.0["idempotency_key"], again.0["idempotency_key"]);
        assert_eq!(command.0["payload"], again.0["payload"]);
        assert!(chat_stop_command(None, Some("chat-1".into()), &target).is_none());
        assert!(chat_stop_command(Some("pw-1".into()), None, &target).is_none());
    }

    #[test]
    fn try_again_is_one_keyed_resume_of_the_named_run() {
        let command =
            chat_resume_command(Some("pw-1".into()), Some("chat-1".into()), &target()).unwrap();
        assert_eq!(
            command.0,
            serde_json::json!({
                "type": "work.resume",
                "idempotency_key": "resume-work-1-run-1",
                "payload": {
                    "patient_workspace_id": "pw-1",
                    "chat_id": "chat-1",
                    "work_item_id": "work-1",
                    "expected_run_id": "run-1",
                },
            })
        );
        assert!(chat_resume_command(None, Some("chat-1".into()), &target()).is_none());
        assert!(chat_resume_command(Some("pw-1".into()), None, &target()).is_none());
    }
}
