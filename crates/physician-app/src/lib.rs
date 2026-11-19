//! Physician shell: the shipping sidebar, Today, a patient page, and the reader.
//! The React app remains the shipping client. This client renders the same projections.
//! Book: `book/src/physician-site.md`.
#![deny(missing_docs)]
#![allow(
    clippy::map_err_ignore,
    reason = "converting to errors that keep their cause"
)]

mod access_notice;
mod composer;
mod feedback;
mod follow_ups;
mod invitations;
mod local_command;
mod local_draft;
mod matching_reply;
mod panels;
mod patient_reviews_panel;
mod schedule_briefing;
mod shell;
mod source_setup;
mod status;
mod tabs;
mod today_updates;

use std::sync::atomic::{AtomicU64, Ordering};

use composer::{Composer, ConflictDraft, DraftTarget, Notice, SaveOutcome, SendAnswer};
use dioxus::prelude::*;
use physician_client::{DraftSave, PhysicianClient};
use physician_wire::briefings::BriefingSummaryItemWire;
use physician_wire::{
    ActiveReviewRun, AnswerState, ChatSnapshot, HistoryWorkOutcome, PastWorkItem, PastWorkTarget,
    PatientWorkspaceResponse, PracticeAccountSummary, RecentConversation, ResponseId,
    SignInResponse, SnapshotFeedback, SubmitDelivery, WhoamiResponse, WorkResumeTarget,
    WorkStopTarget, WorkspaceReply,
};

use panels::Panel;
use schedule_briefing::{
    BriefingWorkspaceView, OpenBriefingTarget, PatientBriefingsSection, RailContext,
    SchedulePaneFreshness, ScheduleRailPane, ScheduleVisitActivation, ScheduleVisitRow,
    browser_time_zone, history_standing_summary, schedule_pane_freshness, schedule_rows,
};
use shell::{
    AppendedRosterPages, CONVERSATION_EMPTY, ChatView, ConnectedSourceStanding, Drawer, KeyInput,
    NavCommand, PAST_WORK_EMPTY, ProjectedStepKind, ProjectedSteps, REPLY_ASK, REPLY_CLOSED_NOTICE,
    REPLY_COPY, REPLY_NO_NEEDED, REPLY_REVISION_STALE, REPLY_TELL_CODA, RESUME, ReadView,
    ReplyReviewAction, RosterContinuation, STOP, SourceChip, SynopsisPiece, TODAY_LOADING,
    TODAY_QUIET, TODAY_QUIET_DETAIL, TODAY_STALE, TODAY_UNAVAILABLE, TRY_AGAIN, TodayLine,
    TodayView, WORKSPACE_CLOSED, WORKSPACE_EMPTY, WORKSPACE_UNAVAILABLE, WorkspaceRecordsState,
    active_now_concern, answer_to, chat_view, chip_name, citation_inspection, connection_lines,
    demographic_card_view, format_context_time, format_local_date, format_waited, keyboard_command,
    message_pieces, need_you_words, panel_target, past_work_copy, project_steps, read_view,
    reply_channel_label, reply_review_actions, reply_revision_words, reply_urgency_words,
    resolve_roster_view, row_icon, row_tone, schedule_empty_copy, search_row_detail,
    shows_today_schedule, source_lines, summary_times, synopsis_pieces, today_lines, today_view,
    work_end_reason, work_heading, workspace_records_state,
};
#[cfg(target_arch = "wasm32")]
use shell::{EventRefetch, event_refetch};
use status::{CommandNotice, CommandStage, ReadNotice, ReadStage};
use ui::{
    ActionRow, Author, Button, ButtonVariant, Card, Count, DateField, DateKind, Dialog,
    DraftConflict, EmptyLine, Icon, IconName, LiveDot, NavigationControl, NavigationRow, Orb,
    OrbState, PageTitle, ProgressBar, SearchField, StatePill, StatusLine, TextField, TextKind,
    Tone, WorkingMark,
};

const SHELL_STYLES: &str = include_str!("../assets/shell.css");
const UPDATES_EMPTY: &str = "Nothing from Coda yet today.";
const PAST_WORK_LOADING: &str = "Loading past work…";
const PAST_WORK_UNAVAILABLE: &str = "Coda couldn’t load past work.";
const PAST_WORK_STALE: &str = "Coda couldn’t refresh past work. Showing the last list.";
const ANSWER_NEWER_RECORDS: &str = "Newer records arrived after this answer.";
const UPDATE_ANSWER_PROMPT: &str = "Update this answer with the latest records.";
pub(crate) const SCHEDULE_LOADING: &str = "Loading the schedule…";
pub(crate) const SCHEDULE_UNAVAILABLE: &str = "Coda couldn’t load the schedule.";
const SEARCH_LOADING: &str = "Searching…";
const SEARCH_UNAVAILABLE: &str = "Coda couldn’t search patients.";
const SEARCH_NONE: &str = "No patients match.";
const ROSTER_LOADING: &str = "Loading patients…";
const ROSTER_UNAVAILABLE: &str = "Coda couldn’t load patients.";
const ROSTER_MORE_LOADING: &str = "Loading more patients…";
const ROSTER_MORE_UNAVAILABLE: &str = "Coda couldn’t load more patients.";
const ROSTER_EMPTY: &str = "No patients on the roster yet.";
const ROSTER_SCROLL_THRESHOLD_PX: f64 = 96.0;
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
    MessagesToMatch,
    Filtered,
    Workspace,
}

#[cfg(target_arch = "wasm32")]
fn initial_place_from_address(open_patient: &str) -> Place {
    if !open_patient.is_empty() {
        return Place::Workspace;
    }
    if let Some(window) = web_sys::window()
        && let Ok(href) = window.location().href()
        && let Ok(url) = web_sys::Url::new(&href)
    {
        match url.search_params().get("view").as_deref() {
            Some("matching") => return Place::MessagesToMatch,
            Some("filtered") => return Place::Filtered,
            Some("updates") => return Place::Updates,
            _ => {}
        }
    }
    Place::Today
}

#[cfg(not(target_arch = "wasm32"))]
fn initial_place_from_address(open_patient: &str) -> Place {
    if open_patient.is_empty() {
        Place::Today
    } else {
        Place::Workspace
    }
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
// A failure worth retrying is always retried: the wait doubles from one second
// to sixteen and stays there. A valid SSE response sustained for at least 30
// seconds starts the count over. Even a healthy rotation waits one second;
// immediate EOF cannot spin.
#[cfg(any(target_arch = "wasm32", test))]
#[derive(Default)]
struct StreamBackoff {
    failures: u32,
}

#[cfg(any(target_arch = "wasm32", test))]
impl StreamBackoff {
    // None denotes a failed connection/HTTP response, regardless of how long
    // that request took. Some(elapsed_ms) measures only an accepted SSE body.
    fn next_delay_ms(&mut self, elapsed_ms: Option<f64>) -> i32 {
        if elapsed_ms.is_some_and(|elapsed| elapsed >= 30_000.0) {
            self.failures = 0;
            return 1_000;
        }
        self.failures = self.failures.saturating_add(1);
        1_000 << (self.failures.min(5) - 1)
    }
}

/// How long the change stream may go without any bytes before the page takes the connection as
/// dropped: twice the server's keep-alive interval (axum's default, 15 seconds). SignalR documents
/// the same rule for its client timeout: "The recommended value is a number at least double the
/// server's keep-alive interval ... to allow time for pings to arrive", and its default is 30
/// seconds.
#[cfg(any(target_arch = "wasm32", test))]
const STREAM_SILENCE_MS: f64 = 30_000.0;

/// How much longer the change stream may stay silent when bytes last arrived at `last_bytes_at`
/// and it is `now`, in the page clock's milliseconds; none once it has been silent for
/// [`STREAM_SILENCE_MS`], and the connection is taken as dropped. Any bytes count, a keep-alive
/// comment too.
#[cfg(any(target_arch = "wasm32", test))]
fn silence_left_ms(last_bytes_at: f64, now: f64) -> Option<f64> {
    let left = STREAM_SILENCE_MS - (now - last_bytes_at);
    (left > 0.0).then_some(left)
}

#[cfg(test)]
mod stream_retry_tests {
    use super::{StreamBackoff, silence_left_ms};

    /// A stream is not silent just under 30 seconds after its last bytes, and is at 30 seconds.
    #[test]
    fn a_stream_is_silent_30_seconds_after_its_last_bytes() {
        let last = 120_000.0;
        assert_eq!(silence_left_ms(last, last), Some(30_000.0));
        assert_eq!(silence_left_ms(last, last + 29_999.0), Some(1.0));
        assert_eq!(silence_left_ms(last, last + 30_000.0), None);
    }

    #[test]
    fn immediate_eof_and_failures_back_off_to_the_longest_delay() {
        let mut retry = StreamBackoff::default();
        for (elapsed, expected) in [
            (Some(0.0), 1_000),
            (None, 2_000),
            (Some(29_999.0), 4_000),
            (None, 8_000),
            (Some(0.0), 16_000),
        ] {
            assert_eq!(retry.next_delay_ms(elapsed), expected);
        }
    }

    #[test]
    fn sustained_stream_starts_the_backoff_over() {
        let mut retry = StreamBackoff::default();
        for _ in 0..7 {
            retry.next_delay_ms(None);
        }
        assert_eq!(retry.next_delay_ms(Some(30_000.0)), 1_000);
        for delay in [1_000, 2_000, 4_000, 8_000, 16_000, 16_000] {
            assert_eq!(retry.next_delay_ms(Some(0.0)), delay);
        }
    }

    #[test]
    fn the_sixth_and_later_failures_still_retry_at_the_longest_delay() {
        let mut retry = StreamBackoff::default();
        for _ in 0..5 {
            retry.next_delay_ms(None);
        }
        for elapsed in [None, Some(0.0), None, Some(29_999.0)] {
            assert_eq!(retry.next_delay_ms(elapsed), 16_000);
        }
    }

    #[test]
    fn cloud_run_rotations_never_build_up_the_wait() {
        let mut retry = StreamBackoff::default();
        // Simulate a full day of five-minute responses without browser waits.
        for _ in 0..288 {
            assert_eq!(retry.next_delay_ms(Some(300_000.0)), 1_000);
        }
        assert_eq!(retry.next_delay_ms(Some(0.0)), 1_000);
        assert_eq!(retry.next_delay_ms(None), 2_000);
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
        let mut retry = StreamBackoff::default();
        let mut delay = 0;
        loop {
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
                delay = retry.next_delay_ms(None);
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
                    delay = retry.next_delay_ms(None);
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
            let mut last_bytes_at = started_at;
            // A read the silence timer woke before, still on its way.
            let mut on_its_way: Option<js_sys::Promise> = None;
            loop {
                let promise = match on_its_way.take() {
                    Some(promise) => promise,
                    None => {
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
                        promise
                    }
                };
                let Some(left) = silence_left_ms(last_bytes_at, clock.now()) else {
                    // Silent too long: the connection is taken as dropped, and let go.
                    if let Ok(cancel) = js_sys::Reflect::get(&reader, &"cancel".into())
                        && let Ok(cancel) = cancel.dyn_into::<js_sys::Function>()
                    {
                        let _ = cancel.call0(&reader);
                    }
                    break;
                };
                // The silence timer resolves with nothing; a read always resolves with an object.
                let timer = js_sys::Promise::new(&mut |resolve, reject| {
                    if let Err(error) = window
                        .set_timeout_with_callback_and_timeout_and_arguments_0(
                            &resolve,
                            left.ceil() as i32,
                        )
                    {
                        let _ = reject.call1(&wasm_bindgen::JsValue::NULL, &error);
                    }
                });
                let first = js_sys::Promise::race(&js_sys::Array::of2(&promise, &timer));
                let Ok(chunk) = wasm_bindgen_futures::JsFuture::from(first).await else {
                    break;
                };
                if chunk.is_undefined() {
                    on_its_way = Some(promise);
                    continue;
                }
                let done = js_sys::Reflect::get(&chunk, &"done".into())
                    .ok()
                    .and_then(|value| value.as_bool())
                    .unwrap_or(true);
                if done {
                    break;
                }
                let Some(bytes) = js_sys::Reflect::get(&chunk, &"value".into())
                    .ok()
                    .map(|value| js_sys::Uint8Array::new(&value).to_vec())
                else {
                    return;
                };
                last_bytes_at = clock.now();
                buffer.push_str(&String::from_utf8_lossy(&bytes));
                apply_event_frames(
                    &mut buffer,
                    &selected,
                    &mut today_attempt,
                    &mut workspace_attempt,
                    &mut snapshot_attempt,
                );
            }
            // The stream ended: it closed, its read failed, or it went silent. Read everything
            // again, then connect again after the backoff.
            today_attempt.set(today_attempt() + 1);
            workspace_attempt.set(workspace_attempt() + 1);
            snapshot_attempt.set(snapshot_attempt() + 1);
            delay = retry.next_delay_ms(Some(clock.now() - started_at));
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
            EventRefetch::Today => {
                today_attempt.set(today_attempt() + 1);
            }
            EventRefetch::OpenPatient => {
                workspace_attempt.set(workspace_attempt() + 1);
                snapshot_attempt.set(snapshot_attempt() + 1);
            }
            EventRefetch::TodayAndOpenPatient => {
                today_attempt.set(today_attempt() + 1);
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
pub(crate) enum Refusal {
    /// The session ended or expired.
    SessionEnded,
    /// The practice removed this physician's access.
    AccessRemoved,
}

pub(crate) fn refusal(error: &physician_client::HttpError) -> Option<Refusal> {
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
pub(crate) struct SessionState {
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
        local_command::forget_everything();
        self.forget_place();
        *self.generation.write() += 1;
    }

    /// Back to the sign-in page in this tab only. A tab that never showed the
    /// workspace keeps the patient link it was opened at.
    fn no_session_here(mut self) {
        panels::forget_all();
        // No old account's unsent text outlives its session in this tab.
        local_draft::forget_everything();
        local_command::forget_everything();
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
    pub(crate) fn refused(mut self, refusal: Refusal, shown: bool, presented: Option<String>) {
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
    /// Optional browser SQLite database URL powering the static product demo.
    #[props(default)]
    demo_db: Option<String>,
) -> Element {
    // A mailed link goes through the sign-in screen even when this browser
    // already has a session. Otherwise the workspace opens, and its first
    // `/today` read handles the auth gate.
    #[cfg(target_arch = "wasm32")]
    let demo_active = demo_db.is_some();
    #[cfg(not(target_arch = "wasm32"))]
    let demo_active = false;
    let mut signed_in = use_signal(|| demo_active || sign_in.is_none());
    let mut whoami_name = use_signal(|| {
        if demo_active {
            "Dr. Hannah Alvarez".to_owned()
        } else {
            String::new()
        }
    });
    let mut whoami = use_signal(|| None::<WhoamiResponse>);
    #[cfg(target_arch = "wasm32")]
    let owner = use_signal(|| {
        if demo_active {
            None
        } else {
            read_stored_session().and_then(|_| read_stored_owner())
        }
    });
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
        #[cfg(target_arch = "wasm32")]
        if let Some(ref db_url) = demo_db {
            return PhysicianClient::demo_sqlite(db_url.clone());
        }
        #[cfg(not(target_arch = "wasm32"))]
        let _ = &demo_db;
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
    let mut place = use_signal({
        let initial = initial_place_from_address(&open_patient);
        move || initial
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
    // The physician's remembered choices arrive with who they are: seed
    // immediately from this browser's cache, then converge with server truth
    // (`GET /physician-state/panels`) so preferences survive sign-out and
    // follow the physician across browsers (`P4.Rule.PanelChoiceRemembered`).
    let mut panel_prefs_generation = use_signal(|| 0_u64);
    use_effect(move || {
        let next_gen = *panel_prefs_generation.peek() + 1;
        panel_prefs_generation.set(next_gen);
        if let Some(account) = whoami.read().as_ref() {
            sidebar_collapsed.set(panels::collapsed(&account.physician_id, Panel::Sidebar));
            schedule_collapsed.set(panels::collapsed(&account.physician_id, Panel::Schedule));
            let physician_id = account.physician_id.clone();
            let api = client.read().clone();
            spawn(async move {
                if let Ok(saved) = api.panel_preferences().await {
                    if *panel_prefs_generation.peek() != next_gen {
                        return;
                    }
                    sidebar_collapsed.set(saved.sidebar_collapsed);
                    schedule_collapsed.set(saved.schedule_collapsed);
                    panels::remember(&physician_id, Panel::Sidebar, saved.sidebar_collapsed);
                    panels::remember(&physician_id, Panel::Schedule, saved.schedule_collapsed);
                }
            });
        }
    });
    let mut remember = move |panel: Panel, collapsed: bool| {
        let next_gen = *panel_prefs_generation.peek() + 1;
        panel_prefs_generation.set(next_gen);
        match panel {
            Panel::Sidebar => sidebar_collapsed.set(collapsed),
            Panel::Schedule => schedule_collapsed.set(collapsed),
        }
        let next = physician_wire::briefings::PanelPreferencesWire {
            sidebar_collapsed: *sidebar_collapsed.peek(),
            schedule_collapsed: *schedule_collapsed.peek(),
        };
        if let Some(account) = whoami.peek().as_ref() {
            panels::remember(&account.physician_id, panel, collapsed);
            let api = client.read().clone();
            spawn(async move {
                let _ = api.save_panel_preferences(&next).await;
            });
        }
    };
    let mut search_open = use_signal(|| false);
    let mut shortcuts_open = use_signal(|| false);
    let mut connections_open = use_signal(|| false);
    let mut connections_source =
        use_signal(|| None::<physician_wire::source_setup::SupportedSourceWire>);
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
    let mut open_briefing = use_signal(|| None::<OpenBriefingTarget>);
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
    let mut roster_pages = use_signal(|| None::<AppendedRosterPages>);
    let mut open_search_dialog = move || {
        query.set(String::new());
        roster_pages.set(None);
        focus.set(-1);
        moved.set(false);
        *search_attempt.write() += 1;
        search_open.set(true);
    };
    let send_refresh = SendRefresh {
        snapshot: snapshot_attempt,
        workspace: workspace_attempt,
        today: today_attempt,
    };
    let mut today_cache = use_signal(|| None::<Vec<TodayLine>>);
    let mut today_meta_cache = use_signal(today_updates::TodayMeta::default);
    let mut workspace_cache = use_signal(|| None::<PatientWorkspaceResponse>);
    // The patient the server said isn't there for this physician (a 404 also
    // answers for no access, so presence never leaks).
    let mut gone = use_signal(|| None::<String>);
    let mut target_physician = use_signal(String::new);
    let mut panel_busy = use_signal(|| false);
    let mut panel_notice = use_signal(String::new);
    let mut past_work_open = use_signal(|| false);
    let mut past_work_attempt = use_signal(|| 0u32);
    let mut reply_notice = use_signal(String::new);
    let reply_slot = use_command_slot();
    let send_slot = use_command_slot();
    let resume_slot = use_command_slot();
    let chat_resume_slot = use_persisted_command_slot(
        "chat-resume",
        client,
        whoami,
        selected,
        open_chat,
        move |_| {
            *snapshot_attempt.write() += 1;
            *workspace_attempt.write() += 1;
        },
    );
    let chat_stop_slot = use_persisted_command_slot(
        "chat-stop",
        client,
        whoami,
        selected,
        open_chat,
        move |value| {
            restore_returned_queue_from_response(&value, None, composer, client);
            *snapshot_attempt.write() += 1;
            *workspace_attempt.write() += 1;
        },
    );
    let queue_slot = use_persisted_command_slot(
        "chat-queue",
        client,
        whoami,
        selected,
        open_chat,
        move |_| {
            *snapshot_attempt.write() += 1;
            *workspace_attempt.write() += 1;
        },
    );
    let work_slot = use_command_slot();
    let workspace_briefings_res = use_resource(move || {
        let _today = today_attempt();
        let _workspace = workspace_attempt();
        let _past_work = past_work_attempt();
        let open_id = if place() == Place::Workspace {
            selected()
        } else {
            None
        };
        let api = client.read().clone();
        async move {
            let answer = match &open_id {
                Some(id) => api
                    .workspace_briefings(id)
                    .await
                    .map(|resp| resp.briefings)
                    .map_err(|_| ()),
                None => Ok(Vec::new()),
            };
            (open_id, answer)
        }
    });
    use_effect(move || {
        let open = workspace_cache.read().clone();
        let showing = place();
        let has_briefings = workspace_briefings_res
            .read()
            .as_ref()
            .is_some_and(|(asked, res)| {
                *asked == selected() && res.as_ref().is_ok_and(|items| !items.is_empty())
            });
        let allowed = showing == Place::Workspace
            && open.as_ref().is_some_and(|ws| !ws.is_clinic_demographic())
            && (open.as_ref().and_then(past_work_copy).is_some() || has_briefings);
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
        if expired() || client.read().is_demo() {
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
        async move { client.updates().await.map_err(|_| ()) }
    });
    let found = use_resource(move || {
        let _attempt = today_attempt();
        let search_gen = search_attempt();
        let open = search_open();
        let query = query();
        let client = client.read().clone();
        async move {
            let answer = if !open {
                None
            } else {
                Some(client.search_page(&query, None).await.map_err(|_| ()))
            };
            ((open, query, search_gen), answer)
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
                            if selected.peek().as_deref() == Some(id.as_str()) {
                                reading.set(None);
                                open_briefing.set(None);
                                past_work_open.set(false);
                            }
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
                            physician_client::HttpError::Status {
                                status: 403 | 404,
                                ..
                            }
                        ) {
                            workspace_cache.set(None);
                            // No access (or no patient): its unsent text is
                            // not kept for a later reload or account.
                            forget_workspace_drafts(composer, &client, whoami, &id);
                            if selected.peek().as_deref() == Some(id.as_str()) {
                                reading.set(None);
                                open_briefing.set(None);
                                past_work_open.set(false);
                            }
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
            today_meta_cache.set(today_updates::TodayMeta::from_response(list));
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
    let _draft_read = use_resource(move || {
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
                        send_refresh,
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
    let practice_zone_now = move || {
        today
            .peek()
            .as_ref()
            .and_then(|res| res.as_ref().ok())
            .and_then(|list| list.time_zone.clone())
            .or_else(|| {
                practice
                    .peek()
                    .as_ref()
                    .and_then(|res| res.as_ref().ok())
                    .map(|d| d.time_zone.clone())
            })
            .or_else(|| {
                workspace_cache
                    .peek()
                    .as_ref()
                    .and_then(|open| open.time_zone.clone())
            })
            .unwrap_or_else(|| "UTC".to_owned())
    };
    // Send (or Send next), from the button or the keyboard: one snapshot of the
    // target, text, edit generation and command, which Check again reuses.
    let mut send_message_with_delivery = move |delivery: SubmitDelivery| {
        let target = draft_target();
        let text = composer.peek().text(target.as_ref());
        let tz = browser_time_zone(&practice_zone_now());
        let command = message_command_with_delivery(
            selected(),
            open_chat(),
            Some(tz),
            text.clone(),
            delivery,
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
            dispatch_send(send_slot, client, composer, send_refresh, target, pending);
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
            dispatch_send(send_slot, client, composer, send_refresh, target, pending);
        }
    };
    let send_update_answer = move || {
        if send_slot.sending() {
            return;
        }
        let tz = browser_time_zone(&practice_zone_now());
        let command = update_answer_command(selected(), open_chat(), Some(tz));
        let mut snapshot_attempt = snapshot_attempt;
        let mut workspace_attempt = workspace_attempt;
        send_slot.run_or_refuse(client.read().clone(), command, move || {
            *snapshot_attempt.write() += 1;
            *workspace_attempt.write() += 1;
        });
    };
    let shown_target = draft_target();
    let today_read = today.read();
    let today_cache_read = today_cache.read();
    let today_result = today_read
        .as_ref()
        .map(|result| result.as_ref().map_err(|_| ()));
    let today_state = today_view(today_cache_read.as_deref(), today_result);
    let today_meta = today_read
        .as_ref()
        .and_then(|result| result.as_ref().ok())
        .map(today_updates::TodayMeta::from_response)
        .unwrap_or_else(|| today_meta_cache.read().clone());
    let (connection_accounts, practice_sources) = health
        .read()
        .as_ref()
        .and_then(|result| result.as_ref().ok())
        .and_then(|health| health.capabilities.as_ref())
        .map_or((false, false), |capabilities| {
            (
                capabilities.connection_accounts,
                capabilities.practice_sources,
            )
        });
    let connections = today_read
        .as_ref()
        .and_then(|result| result.as_ref().ok())
        .map(|list| connection_lines(&list.connections))
        .unwrap_or_default();
    let source_connections = today_read
        .as_ref()
        .and_then(|result| result.as_ref().ok())
        .map(|list| source_setup::structured_today_connections(&list.connections))
        .unwrap_or_default();
    let today_zone = today_read
        .as_ref()
        .and_then(|result| result.as_ref().ok())
        .and_then(|list| list.time_zone.clone());
    let practice_state = use_read_view(practice);
    let updates_state = use_read_view(updates);
    let schedule = practice_state.stage();
    let day = practice_state.content();
    let server_now = day.map(|day| day.current_time.clone()).unwrap_or_default();
    let practice_zone = day
        .map(|day| day.time_zone.clone())
        .or(today_zone)
        .unwrap_or_else(|| "America/New_York".to_owned());
    let practice_date = format_local_date(&server_now, &practice_zone);
    let browser_tz = browser_time_zone(&practice_zone);
    let today_schedule_rows = day.map(|day| schedule_rows(day, None)).unwrap_or_default();
    let schedule_freshness = schedule_pane_freshness(day, schedule, &browser_tz);
    let should_refresh_schedule_source =
        day.is_some_and(|day| day.source_state.freshness.as_deref() == Some("refresh_failed"));
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
    let patient_schedule_rows = day
        .and_then(|day| {
            selected
                .read()
                .as_deref()
                .map(|id| schedule_rows(day, Some(id)))
        })
        .unwrap_or_default();
    let show_patient_schedule =
        show_schedule && (!patient_schedule_rows.is_empty() || schedule_unanswered(schedule));
    let patient_briefings = answer_to(&selected(), workspace_briefings_res.read().as_ref())
        .and_then(|res| res.as_ref().ok())
        .cloned()
        .unwrap_or_default();
    let recent_read = recent.read();
    // Each open or query read is keyed by `(open, query, search_attempt)`, so
    // an earlier query's rows never stand in while a new query is in flight.
    let found_read = found.read();
    let roster_first = answer_to(
        &(search_open(), query(), search_attempt()),
        found_read.as_ref(),
    )
    .and_then(Option::as_ref)
    .map(|res| res.as_ref().map_err(|_| ()));
    let roster_view = resolve_roster_view(
        &query(),
        search_attempt(),
        roster_first,
        roster_pages.read().as_ref(),
    );
    let search_state = roster_view.rows;
    let roster_continuation = roster_view.continuation;
    let search_rows = search_state.content().cloned().unwrap_or_default();
    let mut load_more_roster = move |allow_retry: bool| {
        let current_query = query();
        let current_attempt = search_attempt();
        if !search_open() || !current_query.trim().is_empty() {
            return;
        }
        let first_ok = found
            .peek()
            .as_ref()
            .filter(|(asked, _)| *asked == (true, current_query.clone(), current_attempt))
            .and_then(|(_, res)| res.as_ref())
            .and_then(|res| res.as_ref().ok())
            .cloned();
        let Some(first_page) = first_ok else {
            return;
        };
        let existing = roster_pages
            .peek()
            .clone()
            .filter(|extra| extra.query == current_query && extra.attempt == current_attempt);
        let (extra_patients, continuation) = match existing {
            Some(extra) => (extra.extra_patients, extra.continuation),
            None => {
                let cont = match first_page
                    .next_after
                    .as_deref()
                    .map(str::trim)
                    .filter(|cursor| !cursor.is_empty())
                {
                    Some(cursor) => RosterContinuation::HasMore {
                        cursor: cursor.to_owned(),
                    },
                    None => RosterContinuation::Complete,
                };
                (Vec::new(), cont)
            }
        };
        let cursor = match continuation {
            RosterContinuation::HasMore { cursor } => cursor,
            RosterContinuation::FailedMore { cursor } if allow_retry => cursor,
            RosterContinuation::Complete
            | RosterContinuation::LoadingMore { .. }
            | RosterContinuation::FailedMore { .. } => return,
        };
        roster_pages.set(Some(AppendedRosterPages {
            query: current_query.clone(),
            attempt: current_attempt,
            extra_patients: extra_patients.clone(),
            continuation: RosterContinuation::LoadingMore {
                cursor: cursor.clone(),
            },
        }));
        let api = client.read().clone();
        spawn(async move {
            let next = api.search_page(&current_query, Some(&cursor)).await;
            if !*search_open.peek()
                || *query.peek() != current_query
                || *search_attempt.peek() != current_attempt
            {
                return;
            }
            let mut combined = extra_patients;
            let next_cont = match next {
                Ok(page) => {
                    combined.extend(page.patients);
                    match page
                        .next_after
                        .as_deref()
                        .map(str::trim)
                        .filter(|c| !c.is_empty())
                    {
                        Some(next_cursor) => RosterContinuation::HasMore {
                            cursor: next_cursor.to_owned(),
                        },
                        None => RosterContinuation::Complete,
                    }
                }
                Err(_) => RosterContinuation::FailedMore { cursor },
            };
            roster_pages.set(Some(AppendedRosterPages {
                query: current_query,
                attempt: current_attempt,
                extra_patients: combined,
                continuation: next_cont,
            }));
        });
    };
    let modal =
        shortcuts_open() || connections_open() || past_work_open() || reading.read().is_some();
    let shown_focus = if moved() { Some(focus()) } else { None };
    let mut opened_now_items = use_signal(std::collections::HashSet::<(String, String)>::new);
    let mut active_evidence =
        use_signal(|| None::<physician_wire::today_updates::ItemWorkEvidenceWire>);
    let mut open_item_failure = use_signal(|| None::<today_updates::FailedItemOpen>);
    let mut opening_item_busy = use_signal(|| false);
    let mut done_state = use_signal(today_updates::DoneActionState::default);
    let mut cannot_restart_prompt = use_signal(|| None::<TodayCannotRestartPrompt>);
    let on_panel_workspace = use_memo(move || {
        if place() != Place::Workspace {
            return None;
        }
        let current = selected()?;
        let cached = workspace_cache.read();
        let open = cached.as_ref()?;
        (open.patient_workspace_id == current && !open.is_clinic_demographic()).then_some(current)
    });
    let mark_patient_gone = Callback::new(move |gone_ws: String| {
        workspace_cache.set(None);
        forget_workspace_drafts(composer, &client.peek(), whoami, &gone_ws);
        reading.set(None);
        open_briefing.set(None);
        past_work_open.set(false);
        gone.set(Some(gone_ws));
        *today_attempt.write() += 1;
    });
    let shell_reviews_context = patient_reviews_panel::PatientReviewsContext {
        client,
        session,
        shown,
        workspace_attempt,
        today_attempt,
    };
    let spruce_replies_read = matching_reply::use_patient_spruce_replies(
        on_panel_workspace,
        shell_reviews_context,
        mark_patient_gone,
    );
    let follow_ups_read = follow_ups::use_patient_follow_ups(
        on_panel_workspace,
        shell_reviews_context,
        mark_patient_gone,
    );
    let snapshot = use_resource(move || {
        let _attempt = snapshot_attempt();
        let asked = (selected(), open_chat());
        let client = client.read().clone();
        async move {
            let answer = match &asked {
                (Some(workspace_id), Some(chat_id)) => {
                    match client.chat_snapshot(workspace_id, chat_id).await {
                        Ok(page) => Ok((
                            conversation_lines(&page.snapshot),
                            page.snapshot.queued.clone(),
                        )),
                        Err(error) => {
                            if matches!(
                                error,
                                physician_client::HttpError::Status {
                                    status: 403 | 404,
                                    ..
                                }
                            ) {
                                *workspace_attempt.write() += 1;
                                *today_attempt.write() += 1;
                            }
                            Err(())
                        }
                    }
                }
                _ => Ok((Vec::new(), Vec::new())),
            };
            (asked, answer)
        }
    });
    let past_work_items = use_resource(move || {
        let _today = today_attempt();
        let _workspace = workspace_attempt();
        let _attempt = past_work_attempt();
        let open_id = if past_work_open() { selected() } else { None };
        let client = client.read().clone();
        async move {
            let answer = match &open_id {
                Some(id) => match client.past_work(id).await {
                    Ok(page) => Ok(page.items),
                    Err(error) => {
                        if matches!(
                            error,
                            physician_client::HttpError::Status {
                                status: 403 | 404,
                                ..
                            }
                        ) {
                            past_work_open.set(false);
                            *workspace_attempt.write() += 1;
                            *today_attempt.write() += 1;
                        }
                        Err(())
                    }
                },
                None => Ok(Vec::new()),
            };
            (open_id, answer)
        }
    });
    let mut last_past_work = use_signal(|| None);
    use_effect(move || {
        if let Some((Some(id), Ok(items))) = past_work_items.read().as_ref() {
            last_past_work.set(Some((Some(id.clone()), items.clone())));
        }
    });
    use_effect(move || {
        let current_id = selected();
        let gone_id = gone();
        let cached = workspace_cache.read();
        if clinical_dialogs_blocked(
            place(),
            current_id.as_deref(),
            gone_id.as_deref(),
            cached.as_ref(),
        ) {
            if reading.peek().is_some() {
                reading.set(None);
            }
            if open_briefing.peek().is_some() {
                open_briefing.set(None);
            }
            if *past_work_open.peek() {
                past_work_open.set(false);
            }
            if last_past_work
                .peek()
                .as_ref()
                .is_some_and(|(cached_id, _)| *cached_id == current_id)
            {
                last_past_work.set(None);
            }
            if active_evidence
                .peek()
                .as_ref()
                .is_some_and(|ev| Some(ev.patient_workspace_id.as_str()) == current_id.as_deref())
            {
                active_evidence.set(None);
            }
        }
    });
    let past_work_state = read_view(
        answer_to(&selected(), last_past_work.read().as_ref()).cloned(),
        answer_to(&selected(), past_work_items.read().as_ref()).cloned(),
    );
    // A failed reread keeps this conversation's last lines, shown as stale, so
    // its turns and any open details dialog stay; one conversation's lines
    // never stand in for another's.
    let mut last_conversation = use_signal(|| None);
    let mut last_queued = use_signal(|| None);
    use_effect(move || {
        if let Some((asked, Ok((lines, queued_items)))) = snapshot.read().as_ref() {
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
            last_queued.set(Some((asked.clone(), queued_items.clone())));
        }
    });
    let asking = (selected(), open_chat());
    let conversation = read_view(
        answer_to(&asking, last_conversation.read().as_ref()).cloned(),
        answer_to(&asking, snapshot.read().as_ref()).map(|res| match res {
            Ok((lines, _)) => Ok(lines.clone()),
            Err(()) => Err(()),
        }),
    );
    let queued = answer_to(&asking, snapshot.read().as_ref())
        .and_then(|res| res.as_ref().ok())
        .map(|(_, queued)| queued.clone())
        .or_else(|| answer_to(&asking, last_queued.read().as_ref()).cloned())
        .unwrap_or_default();
    let active_now = today_state.content().and_then(|rows| {
        let opened = opened_now_items.read();
        let remaining: Vec<TodayLine> = rows
            .iter()
            .filter(|line| !opened.contains(&(line.item_id.clone(), line.version_id.clone())))
            .cloned()
            .collect();
        active_now_concern(&remaining).cloned()
    });
    let active_runs = updates_state
        .content()
        .map(|page| page.active_runs.clone())
        .unwrap_or_default();
    let update_preview_rows = updates_state
        .content()
        .map(|page| page.rows.clone())
        .unwrap_or_default();
    let mut open_item_target = move |target: today_updates::OpenItemTarget| {
        if target.item_id.starts_with("spruce-match:")
            || target.item_id.starts_with("spruce-unreadable:")
        {
            open_item_failure.set(None);
            reading.set(None);
            open_briefing.set(None);
            place.set(Place::MessagesToMatch);
            app_collapsed.set(false);
            search_open.set(false);
            drawer.set(Drawer::Dismissed);
            *today_attempt.write() += 1;
            return;
        }
        if target.item_id.starts_with("spruce-reply:")
            || target.item_id.starts_with("follow-up-cannot-restart:")
            || target.item_id.starts_with("follow-up-update:")
        {
            open_item_failure.set(None);
            reading.set(None);
            open_briefing.set(None);
            selected.set(Some(target.workspace_id));
            place.set(Place::Workspace);
            app_collapsed.set(false);
            search_open.set(false);
            drawer.set(Drawer::Dismissed);
            *workspace_attempt.write() += 1;
            *today_attempt.write() += 1;
            return;
        }
        if *opening_item_busy.peek() {
            return;
        }
        let client = client.read().clone();
        let presented = client.session().get();
        opening_item_busy.set(true);
        spawn(async move {
            let outcome = client
                .open_today_update_item(
                    &target.workspace_id,
                    &target.item_id,
                    &target.idempotency_key,
                )
                .await;
            opening_item_busy.set(false);
            match outcome {
                Ok(evidence) => {
                    opened_now_items
                        .write()
                        .insert((evidence.item_id.clone(), evidence.version_id.clone()));
                    open_item_failure.set(None);
                    reading.set(None);
                    let workspace_id = evidence.patient_workspace_id.clone();
                    let linked_chat = evidence.linked_chat_id.clone();
                    active_evidence.set(Some(evidence));
                    if let Some(chat_id) = linked_chat {
                        requested_chat.set(Some((workspace_id.clone(), chat_id.clone())));
                        open_chat.set(Some(chat_id));
                        *snapshot_attempt.write() += 1;
                    }
                    open_briefing.set(None);
                    selected.set(Some(workspace_id));
                    place.set(Place::Workspace);
                    app_collapsed.set(false);
                    search_open.set(false);
                    drawer.set(Drawer::Dismissed);
                    *today_attempt.write() += 1;
                    *workspace_attempt.write() += 1;
                }
                Err(error) => {
                    if *app_collapsed.peek() {
                        app_collapsed.set(false);
                    }
                    if let Some(refused) = refusal(&error) {
                        session.refused(refused, *shown.peek(), presented);
                    } else {
                        let kind = today_updates::ItemOpenFailureKind::from_http_error(&error);
                        let unavailable =
                            kind == today_updates::ItemOpenFailureKind::UnavailableOnPanel;
                        open_item_failure
                            .set(Some(today_updates::build_item_open_failure(target, kind)));
                        if unavailable {
                            *today_attempt.write() += 1;
                            *workspace_attempt.write() += 1;
                        }
                    }
                }
            }
        });
    };
    let mut retry_schedule = move || {
        if should_refresh_schedule_source {
            let api = client.read().clone();
            spawn(async move {
                let _ = api.refresh_practice_day().await;
                *today_attempt.write() += 1;
                *workspace_attempt.write() += 1;
            });
        } else {
            *today_attempt.write() += 1;
        }
    };
    let mut activate_schedule_visit = move |row: ScheduleVisitRow| match row.activation() {
        ScheduleVisitActivation::OpenBriefing {
            workspace_id,
            briefing_id,
        } => {
            reading.set(None);
            selected.set(Some(workspace_id.clone()));
            open_briefing.set(Some(OpenBriefingTarget {
                workspace_id,
                briefing_id,
            }));
            place.set(Place::Workspace);
            app_collapsed.set(false);
            search_open.set(false);
            drawer.set(Drawer::Dismissed);
            workspace_attempt.set(workspace_attempt() + 1);
        }
        ScheduleVisitActivation::OpenPatient { workspace_id } => {
            open_briefing.set(None);
            selected.set(Some(workspace_id));
            place.set(Place::Workspace);
            app_collapsed.set(false);
            search_open.set(false);
            drawer.set(Drawer::Dismissed);
            workspace_attempt.set(workspace_attempt() + 1);
        }
        ScheduleVisitActivation::None => {}
    };
    let mut open_today_line = move |line: TodayLine| {
        opened_now_items
            .write()
            .insert((line.item_id.clone(), line.version_id.clone()));
        if line.kind == "unmatched_message" || line.kind == "unreadable_message" {
            open_item_failure.set(None);
            reading.set(None);
            open_briefing.set(None);
            place.set(Place::MessagesToMatch);
            app_collapsed.set(false);
            search_open.set(false);
            drawer.set(Drawer::Dismissed);
            *today_attempt.write() += 1;
            return;
        }
        if line.kind == "drafted_reply"
            || line.item_id.starts_with("spruce-reply:")
            || line.item_id.starts_with("follow-up-cannot-restart:")
            || line.item_id.starts_with("unhandled-followup:")
        {
            open_item_failure.set(None);
            reading.set(None);
            open_briefing.set(None);
            selected.set(Some(line.workspace_id));
            place.set(Place::Workspace);
            app_collapsed.set(false);
            search_open.set(false);
            drawer.set(Drawer::Dismissed);
            *workspace_attempt.write() += 1;
            *today_attempt.write() += 1;
            return;
        }
        let label = if line.title.trim().is_empty() {
            line.patient.clone()
        } else {
            format!("{} — {}", line.patient, line.title)
        };
        open_item_target(today_updates::OpenItemTarget {
            workspace_id: line.workspace_id,
            item_id: line.item_id,
            label,
            idempotency_key: idempotency_key(),
        });
    };
    let mut mark_item_done =
        move |(workspace_id, item_id, version_id): (String, String, String)| {
            if let Some((ws_from_key, follow_up_id)) = parse_unhandled_followup_key(&item_id) {
                let target_ws = if workspace_id.trim().is_empty() {
                    ws_from_key
                } else {
                    workspace_id
                };
                cannot_restart_prompt.set(Some(TodayCannotRestartPrompt {
                    workspace_id: target_ws,
                    follow_up_id,
                    new_date: String::new(),
                    note: String::new(),
                    submitting: false,
                    error: None,
                }));
                return;
            }
            if done_state.peek().is_clearing(&item_id) {
                return;
            }
            let client = client.read().clone();
            let presented = client.session().get();
            let key = idempotency_key();
            done_state.set(today_updates::DoneActionState::Clearing {
                item_id: item_id.clone(),
            });
            spawn(async move {
                match client
                    .mark_today_item_done(&workspace_id, &item_id, &version_id, &key)
                    .await
                {
                    Ok(resp)
                        if resp.outcome
                            == physician_wire::today_updates::TodayDoneOutcomeWire::Accepted =>
                    {
                        done_state.set(today_updates::DoneActionState::Idle);
                        if active_evidence
                            .peek()
                            .as_ref()
                            .is_some_and(|ev| ev.item_id == item_id)
                        {
                            active_evidence.set(None);
                        }
                        *today_attempt.write() += 1;
                        *workspace_attempt.write() += 1;
                    }
                    Ok(resp) => {
                        done_state.set(today_updates::DoneActionState::Failed {
                            item_id,
                            message: resp
                                .message
                                .unwrap_or_else(|| "Coda couldn’t mark that done.".to_owned()),
                        });
                    }
                    Err(error) => {
                        if let Some(refused) = refusal(&error) {
                            done_state.set(today_updates::DoneActionState::Idle);
                            session.refused(refused, *shown.peek(), presented);
                        } else {
                            done_state.set(today_updates::DoneActionState::Failed {
                                item_id,
                                message: "Coda couldn’t mark that done.".to_owned(),
                            });
                            if matches!(
                                error,
                                physician_client::HttpError::Status {
                                    status: 403 | 404,
                                    ..
                                }
                            ) {
                                *today_attempt.write() += 1;
                                *workspace_attempt.write() += 1;
                            }
                        }
                    }
                }
            });
        };
    // New conversation: a fresh id the server keeps once a message is sent.
    // The address names it, so a reload before the reply still reopens it.
    let mut start_chat = move || {
        if selected().is_none() {
            return;
        }
        open_briefing.set(None);
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
        if app_collapsed() {
            CodaOrbDock {
                urgent: active_now.clone(),
                active_runs: active_runs.clone(),
                update_rows: update_preview_rows,
                on_open_coda: move |_| app_collapsed.set(false),
                on_open_updates: move |_| {
                    place.set(Place::Updates);
                    app_collapsed.set(false);
                },
                on_open_urgent: move |line: TodayLine| open_today_line(line),
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
                let rail = if place() == Place::Workspace {
                    patient_schedule_rows.clone()
                } else {
                    today_schedule_rows.clone()
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
                    NavCommand::Search => open_search_dialog(),
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
                        let Some(target) = targets.get(focus().max(0) as usize).cloned() else {
                            return;
                        };
                        match target {
                            FocusTarget::Row(line) => open_today_line(line),
                            FocusTarget::Visit(row) => activate_schedule_visit(row),
                            FocusTarget::Patient(id) => {
                                open_briefing.set(None);
                                selected.set(Some(id));
                                place.set(Place::Workspace);
                                search_open.set(false);
                                workspace_attempt.set(workspace_attempt() + 1);
                            }
                        }
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
                                open_briefing.set(None);
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
                                open_briefing.set(None);
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
                                open_briefing.set(None);
                                place.set(Place::Updates);
                                drawer.set(Drawer::Dismissed);
                            },
                            Icon { name: IconName::Message }
                            strong { class: "item-text", "Updates" }
                        }
                        NavigationRow {
                            class: "sidebar-nav-item",
                            aria_label: "Messages to match",
                            aria_current: (place() == Place::MessagesToMatch || place() == Place::Filtered).then_some("page"),
                            "data-nav-messages-to-match": "true",
                            onclick: move |_| {
                                open_briefing.set(None);
                                place.set(Place::MessagesToMatch);
                                drawer.set(Drawer::Dismissed);
                            },
                            Icon { name: IconName::Link }
                            strong { class: "item-text", "Messages to match" }
                            if today_meta.messages_to_match_count > 0 {
                                span {
                                    class: "mr-nav-badge",
                                    "data-messages-to-match-count": "{today_meta.messages_to_match_count}",
                                    "{today_meta.messages_to_match_count}"
                                }
                            }
                        }
                        NavigationRow {
                            class: "sidebar-nav-item",
                            aria_label: "Find patient",
                            onclick: move |_| {
                                open_search_dialog();
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
                                    ChatView::Chats(lines) => rsx! {
                                        for line in lines {
                                            NavigationRow {
                                                class: "sidebar-conversation-link",
                                                onclick: {
                                                    let id = line.id.clone();
                                                    move |_| {
                                                        open_briefing.set(None);
                                                        open_chat.set(Some(id.clone()));
                                                        drawer.set(Drawer::Dismissed);
                                                    }
                                                },
                                                Icon { name: IconName::Message }
                                                span { class: "chat-row-copy",
                                                    span { class: "sidebar-recent-title", "{line.title}" }
                                                    if let Some(summary) = distinct_history_summary(line.summary.as_deref(), &line.title, line.outcome) {
                                                        span { class: "chat-row-summary", "{summary}" }
                                                    }
                                                    if let Some(outcome) = line.outcome {
                                                        span { class: "chat-row-meta",
                                                            StatePill { tone: history_outcome_tone(outcome), "{outcome.label()}" }
                                                        }
                                                    }
                                                }
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
                                Some(Ok(list)) if list.conversations.is_empty() && !list.may_be_missing_someone => {
                                    rsx! { p { class: "sidebar-empty", "{RECENT_NONE}" } }
                                }
                                Some(Ok(list)) => rsx! {
                                    access_notice::MayBeMissingSomeoneNotice {
                                        visible: list.may_be_missing_someone,
                                    }
                                    for conversation in list.conversations.clone() {
                                        RecentRow {
                                            conversation,
                                            on_open: move |(workspace_id, chat_id): (String, String)| {
                                                open_briefing.set(None);
                                                requested_chat.set(Some((
                                                    workspace_id.clone(),
                                                    chat_id.clone(),
                                                )));
                                                selected.set(Some(workspace_id));
                                                open_chat.set(Some(chat_id));
                                                place.set(Place::Workspace);
                                                drawer.set(Drawer::Dismissed);
                                                workspace_attempt.set(workspace_attempt() + 1);
                                                snapshot_attempt.set(snapshot_attempt() + 1);
                                            },
                                        }
                                    }
                                },
                            }
                        }
                    }
                    div { class: "sidebar-footer",
                        if !whoami_name().is_empty() {
                            span { class: "sidebar-physician", "{whoami_name}" }
                        }
                        if client.read().is_demo() {
                            StatePill { "SQLite demo" }
                            Button {
                                variant: ButtonVariant::Quiet,
                                aria_label: "Reset demo database",
                                onclick: move |_| {
                                    let demo_client = client.read().clone();
                                    spawn(async move {
                                        if demo_client.demo_reset().await.is_ok() {
                                            *today_attempt.write() += 1;
                                            *workspace_attempt.write() += 1;
                                            *snapshot_attempt.write() += 1;
                                            reading.set(None);
                                            drawer.set(Drawer::Dismissed);
                                        }
                                    });
                                },
                                "Reset demo"
                            }
                        }
                        if connection_accounts || practice_sources {
                            Button {
                                variant: ButtonVariant::Quiet,
                                onclick: move |_| {
                                    connections_source.set(None);
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
                        if !client.read().is_demo() {
                            invitations::ColleagueInviteTrigger {
                                client,
                                session,
                                shown,
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
                if let Some(concern) = active_now.clone() {
                    NowBanner {
                        concern,
                        on_open: move |line: TodayLine| open_today_line(line),
                    }
                }
                if let Some(prompt) = cannot_restart_prompt() {
                    TodayCannotRestartPromptCard {
                        prompt: prompt.clone(),
                        default_date: practice_date
                            .clone()
                            .filter(|d| d.len() == 10)
                            .unwrap_or_else(|| "2026-12-01".to_owned()),
                        on_change_date: move |val: String| {
                            if let Some(cur) = cannot_restart_prompt.write().as_mut() {
                                cur.new_date = val;
                                cur.error = None;
                            }
                        },
                        on_change_note: move |val: String| {
                            if let Some(cur) = cannot_restart_prompt.write().as_mut() {
                                cur.note = val;
                                cur.error = None;
                            }
                        },
                        on_dismiss: move |_| cannot_restart_prompt.set(None),
                        on_resolve: move |resolution: physician_wire::follow_ups::CannotRestartResolutionWire| {
                            let Some(cur) = cannot_restart_prompt.peek().clone() else {
                                return;
                            };
                            if cur.submitting {
                                return;
                            }
                            if matches!(
                                &resolution,
                                physician_wire::follow_ups::CannotRestartResolutionWire::NewDate { due_date, .. }
                                if due_date.trim().is_empty()
                            ) {
                                if let Some(state) = cannot_restart_prompt.write().as_mut() {
                                    state.error = Some("Choose a new review date first.".to_owned());
                                }
                                return;
                            }
                            if let Some(state) = cannot_restart_prompt.write().as_mut() {
                                state.submitting = true;
                                state.error = None;
                            }
                            let client_val = client.read().clone();
                            let presented = client_val.session().get();
                            let key = idempotency_key();
                            let ws = cur.workspace_id.clone();
                            let fid = cur.follow_up_id.clone();
                            spawn(async move {
                                match client_val
                                    .resolve_cannot_restart_follow_up(&ws, &fid, resolution, &key)
                                    .await
                                {
                                    Ok(_) => {
                                        cannot_restart_prompt.set(None);
                                        *today_attempt.write() += 1;
                                        *workspace_attempt.write() += 1;
                                    }
                                    Err(err) => {
                                        if let Some(refused) = refusal(&err) {
                                            cannot_restart_prompt.set(None);
                                            session.refused(refused, *shown.peek(), presented);
                                        } else if let Some(state) = cannot_restart_prompt.write().as_mut() {
                                            state.submitting = false;
                                            state.error = Some(
                                                "Not saved. Coda couldn’t resolve that follow-up right now."
                                                    .to_owned(),
                                            );
                                        }
                                    }
                                }
                            });
                        },
                    }
                }
                match place() {
                    Place::Today => rsx! {
                        TodaySurface {
                            state: today_state.clone(),
                            meta: today_meta.clone(),
                            connections: connections.clone(),
                            source_connections: source_connections.clone(),
                            on_open_source: move |target_src: Option<physician_wire::source_setup::SupportedSourceWire>| {
                                connections_source.set(target_src);
                                connections_open.set(true);
                            },
                            schedule_rows: today_schedule_rows.clone(),
                            show_schedule,
                            schedule,
                            schedule_may_be_missing_someone: day.is_some_and(|d| d.may_be_missing_someone),
                            schedule_freshness: schedule_freshness.clone(),
                            practice_zone: practice_zone.clone(),
                            browser_tz: browser_tz.clone(),
                            on_schedule_retry: move |_| retry_schedule(),
                            schedule_empty: schedule_empty.clone(),
                            schedule_collapsed: schedule_collapsed(),
                            on_schedule_collapsed: move |collapsed| remember(Panel::Schedule, collapsed),
                            server_now: server_now.clone(),
                            practice_date: practice_date.clone(),
                            need_you,
                            focused: shown_focus,
                            open_failure: open_item_failure(),
                            opening_busy: opening_item_busy(),
                            done_state: done_state(),
                            on_retry: move |_| today_attempt.set(today_attempt() + 1),
                            on_retry_open: move |target: today_updates::OpenItemTarget| open_item_target(target),
                            on_dismiss_open_failure: move |_| open_item_failure.set(None),
                            on_dismiss_done_failure: move |_| done_state.set(today_updates::DoneActionState::Idle),
                            on_open_updates: move |_| place.set(Place::Updates),
                            on_open_patient: move |id: String| {
                                open_briefing.set(None);
                                selected.set(Some(id));
                                place.set(Place::Workspace);
                                search_open.set(false);
                                workspace_attempt.set(workspace_attempt() + 1);
                            },
                            on_open_row: move |line: TodayLine| open_today_line(line),
                            on_done_row: move |line: TodayLine| {
                                mark_item_done((line.workspace_id, line.item_id, line.version_id));
                            },
                            on_activate_visit: move |row: ScheduleVisitRow| activate_schedule_visit(row),
                            on_resume: {
                                let zone = browser_tz.clone();
                                move |line: TodayLine| {
                                    if let Some(command) = resume_command(&line, &zone) {
                                        resume_slot.run(client.read().clone(), command, move || *today_attempt.write() += 1);
                                    }
                                }
                            },
                            resume_notice: resume_slot.notice(client),
                        }
                    },
                    Place::Updates => rsx! {
                        today_updates::UpdatesMainSurface {
                            state: updates_state.clone(),
                            open_failure: open_item_failure(),
                            opening_busy: opening_item_busy(),
                            on_retry: move |_| *today_attempt.write() += 1,
                            on_retry_open: move |target: today_updates::OpenItemTarget| open_item_target(target),
                            on_dismiss_open_failure: move |_| open_item_failure.set(None),
                            on_open_patient: move |id: String| {
                                open_briefing.set(None);
                                selected.set(Some(id));
                                place.set(Place::Workspace);
                                workspace_attempt.set(workspace_attempt() + 1);
                            },
                            on_open_item: move |target: today_updates::OpenItemTarget| open_item_target(target),
                        }
                    },
                    Place::MessagesToMatch => rsx! {
                        matching_reply::MessagesToMatchSurface {
                            client,
                            session,
                            shown,
                            today_attempt,
                            on_open_filtered: move |_| {
                                open_briefing.set(None);
                                place.set(Place::Filtered);
                            },
                            on_open_patient: move |id: String| {
                                open_briefing.set(None);
                                selected.set(Some(id));
                                place.set(Place::Workspace);
                                workspace_attempt.set(workspace_attempt() + 1);
                            },
                        }
                    },
                    Place::Filtered => rsx! {
                        matching_reply::FilteredMessagesSurface {
                            client,
                            session,
                            shown,
                            today_attempt,
                            on_back_to_matching: move |_| {
                                open_briefing.set(None);
                                place.set(Place::MessagesToMatch);
                            },
                            on_open_patient: move |id: String| {
                                open_briefing.set(None);
                                selected.set(Some(id));
                                place.set(Place::Workspace);
                                workspace_attempt.set(workspace_attempt() + 1);
                            },
                        }
                    },
                    Place::Workspace => {
                        if let Some(target) = open_briefing().filter(|t| {
                            selected.read().as_deref() == Some(t.workspace_id.as_str())
                                && !clinical_dialogs_blocked(
                                    place(),
                                    selected.read().as_deref(),
                                    gone.read().as_deref(),
                                    cached_workspace.as_ref(),
                                )
                        }) {
                            rsx! {
                                BriefingWorkspaceView {
                                    client,
                                    target,
                                    browser_tz: browser_tz.clone(),
                                    server_now: server_now.clone(),
                                    refresh_tick: workspace_attempt(),
                                    on_back_today: move |_| {
                                        open_briefing.set(None);
                                        place.set(Place::Today);
                                    },
                                    on_open_patient_page: move |_| {
                                        open_briefing.set(None);
                                    },
                                    on_open_source: move |chip: SourceChip| {
                                        reading.set(Some(source_reading(chip)));
                                    },
                                }
                            }
                        } else {
                            rsx! {
                        PatientSurface {
                            gone: patient_gone,
                            read: patient_read,
                            reviews_context: shell_reviews_context,
                            spruce_replies_read,
                            follow_ups_read,
                            on_go_today: move |_| {
                                open_briefing.set(None);
                                selected.set(None);
                                place.set(Place::Today);
                            },
                            workspace_id: selected().unwrap_or_default(),
                            name: open_name.clone().unwrap_or_default(),
                            workspace: open_workspace.clone(),
                            rows: patient_rows.clone(),
                            schedule_rows: patient_schedule_rows.clone(),
                            show_schedule: show_patient_schedule,
                            schedule_freshness: schedule_freshness.clone(),
                            browser_tz: browser_tz.clone(),
                            schedule_collapsed: schedule_collapsed(),
                            on_schedule_collapsed: move |collapsed| remember(Panel::Schedule, collapsed),
                            schedule,
                            on_schedule_retry: move |_| retry_schedule(),
                            on_activate_visit: move |row: ScheduleVisitRow| activate_schedule_visit(row),
                            briefings: patient_briefings.clone(),
                            on_open_briefing: move |target: OpenBriefingTarget| {
                                open_briefing.set(Some(target));
                            },
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
                            work_busy: work_slot.sending() || resume_slot.sending(),
                            work_notice: work_slot.notice(client),
                            resume_notice: resume_slot.notice(client),
                            on_resume: {
                                let zone = browser_tz.clone();
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
                            on_open_demo_source: client.read().is_demo().then(|| {
                                Callback::new(move |record_id: String| {
                                    let Some(ws_id) = selected() else { return };
                                    let demo_client = client.read().clone();
                                    spawn(async move {
                                        if let Ok(rec) =
                                            demo_client.demo_source_preview(&ws_id, &record_id).await
                                        {
                                            let chip = SourceChip {
                                                index: 1,
                                                label: rec.title,
                                                open: true,
                                                preview: rec.body,
                                                as_of: rec.recorded_at,
                                                open_label: Some(rec.source_label),
                                                open_uri: None,
                                                origin: physician_wire::CitationOrigin::SourceRecord,
                                            };
                                            reading.set(Some(source_reading(chip)));
                                        }
                                    });
                                })
                            }),
                            on_draft: move |value: String| {
                                let target = draft_target();
                                composer.write().edit(target.as_ref(), value);
                                if let Some(target) = target {
                                    autosave_after(composer, client, target, AUTOSAVE_DEBOUNCE_MS);
                                }
                            },
                            conversation: conversation.clone(),
                            queued: queued.clone(),
                            queue_busy: queue_slot.sending(),
                            queue_notice: queue_slot.notice(client),
                            on_conversation_retry: move |_| *snapshot_attempt.write() += 1,
                            conversation_resume_busy: chat_resume_slot.sending(),
                            conversation_resume_notice: chat_resume_slot.notice(client),
                            on_conversation_resume: move |target: WorkResumeTarget| {
                                if let Some(command) =
                                    chat_resume_command(selected(), open_chat(), &target)
                                {
                                    chat_resume_slot.run(client.read().clone(), command, move || {
                                        *snapshot_attempt.write() += 1;
                                    });
                                }
                            },
                            conversation_stop_busy: chat_stop_slot.sending(),
                            conversation_stop_notice: chat_stop_slot.notice(client),
                            on_conversation_stop: move |target: WorkStopTarget| {
                                if let Some(command) =
                                    chat_stop_command(selected(), open_chat(), &target)
                                {
                                    let fallback = command_target(&command);
                                    chat_stop_slot.run_with_response(
                                        client.read().clone(),
                                        command,
                                        move |value| {
                                            restore_returned_queue_from_response(
                                                &value,
                                                fallback.clone(),
                                                composer,
                                                client,
                                            );
                                            *snapshot_attempt.write() += 1;
                                        },
                                    );
                                }
                            },
                            on_open_chat: move |id: String| open_chat.set(Some(id)),
                            on_new_chat: move |_| start_chat(),
                            on_send: move |_| send_message_with_delivery(SubmitDelivery::SendNow),
                            on_send_next: move |_| send_message_with_delivery(SubmitDelivery::SendNext),
                            on_queue_edit: move |(message_id, text): (String, String)| {
                                if let Some(command) =
                                    queue_edit_command(selected(), open_chat(), message_id, text)
                                {
                                    queue_slot.run(client.read().clone(), command, move || {
                                        *snapshot_attempt.write() += 1;
                                    });
                                }
                            },
                            on_queue_withdraw: move |message_id: String| {
                                if let Some(command) =
                                    queue_withdraw_command(selected(), open_chat(), message_id)
                                {
                                    queue_slot.run(client.read().clone(), command, move || {
                                        *snapshot_attempt.write() += 1;
                                    });
                                }
                            },
                            on_queue_send_now: {
                                let zone = browser_tz.clone();
                                move |message_id: String| {
                                    if let Some(command) = queue_send_now_command(
                                        selected(),
                                        open_chat(),
                                        message_id,
                                        zone.clone(),
                                    ) {
                                        queue_slot.run(client.read().clone(), command, move || {
                                            *snapshot_attempt.write() += 1;
                                            *workspace_attempt.write() += 1;
                                        });
                                    }
                                }
                            },
                            on_update_answer: move |_| send_update_answer(),
                            on_open_patient_row: move |line: TodayLine| open_today_line(line),
                            on_done_patient_row: move |line: TodayLine| {
                                mark_item_done((line.workspace_id, line.item_id, line.version_id));
                            },
                            active_evidence: active_evidence()
                                .filter(|ev| Some(ev.patient_workspace_id.as_str()) == selected.read().as_deref()),
                            on_close_evidence: move |_| active_evidence.set(None),
                            on_done_evidence: move |(ws, id, ver): (String, String, String)| {
                                mark_item_done((ws, id, ver));
                            },
                            open_failure: open_item_failure(),
                            opening_busy: opening_item_busy(),
                            done_state: done_state(),
                            on_retry_open: move |target: today_updates::OpenItemTarget| open_item_target(target),
                            on_dismiss_open_failure: move |_| open_item_failure.set(None),
                            on_dismiss_done_failure: move |_| done_state.set(today_updates::DoneActionState::Idle),
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
                            past_work: open_workspace.as_ref().is_some_and(|open| past_work_copy(open).is_some())
                                || !patient_briefings.is_empty(),
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
                            on_resume_reply: move |target: WorkResumeTarget| {
                                let Some(command) = chat_resume_command(selected(), Some(target.work_item_id.0.clone()), &target) else { return };
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
                            on_revise_reply: {
                                let zone = browser_tz.clone();
                                move |(reply_id, revision, instruction): (String, u64, String)| {
                                    let Some(ws_id) = selected() else { return };
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
                                }
                            },
                            on_open_patient: move |id: String| {
                                open_briefing.set(None);
                                selected.set(Some(id));
                                place.set(Place::Workspace);
                                workspace_attempt.set(workspace_attempt() + 1);
                            },
                            on_patient_gone: move |gone_ws: String| {
                                mark_patient_gone.call(gone_ws);
                            },
                            on_open_discuss_chat: move |chat_id: String| {
                                if let Some(ws_id) = selected() {
                                    requested_chat.set(Some((ws_id, chat_id.clone())));
                                }
                                open_chat.set(Some(chat_id));
                                *workspace_attempt.write() += 1;
                                *snapshot_attempt.write() += 1;
                            },
                            invite_control: rsx! {
                                invitations::PatientInvitePanel {
                                    client,
                                    session,
                                    shown,
                                    workspace_id: selected().unwrap_or_default(),
                                    default_display_name: open_name.clone().unwrap_or_default(),
                                }
                            },
                        }
                            }
                        }
                    },
                }
            }
        }
        if search_open() {
            SearchDialog {
                query: query(),
                state: search_state.clone(),
                continuation: roster_continuation.clone(),
                may_be_missing_someone: roster_first
                    .and_then(|res| res.ok())
                    .is_some_and(|page| page.may_be_missing_someone),
                on_retry: move |_| {
                    roster_pages.set(None);
                    *search_attempt.write() += 1;
                },
                on_scroll_more: move |_| load_more_roster(false),
                on_retry_more: move |_| load_more_roster(true),
                on_query: move |value: String| {
                    roster_pages.set(None);
                    focus.set(-1);
                    moved.set(false);
                    query.set(value);
                },
                on_close: move |_| {
                    search_open.set(false);
                    editing.set(false);
                },
                on_open: move |id: String| {
                    open_briefing.set(None);
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
            if practice_sources {
                source_setup::PracticeSourcesDialog {
                    client,
                    session,
                    shown: shown(),
                    practice_zone: practice_zone.clone(),
                    server_now: server_now.clone(),
                    initial_source: connections_source(),
                    on_changed: move |_| {
                        today_attempt.set(today_attempt() + 1);
                        workspace_attempt.set(workspace_attempt() + 1);
                    },
                    on_close: move |_| {
                        connections_open.set(false);
                        connections_source.set(None);
                    },
                }
            } else {
                ConnectionsDialog {
                    lines: connections.clone(),
                    failed: matches!(today_state, ReadView::Unavailable),
                    on_close: move |_| connections_open.set(false),
                }
            }
        }
        if let Some(open) = reading().filter(|_| {
            !clinical_dialogs_blocked(
                place(),
                selected.read().as_deref(),
                gone.read().as_deref(),
                cached_workspace.as_ref(),
            )
        }) {
            ReaderDialog {
                reading: open,
                on_close: move |_| reading.set(None),
            }
        }
        if past_work_open()
            && place() == Place::Workspace
            && !clinical_dialogs_blocked(
                place(),
                selected.read().as_deref(),
                gone.read().as_deref(),
                cached_workspace.as_ref(),
            )
        {
            PastWorkDialog {
                workspace_id: selected().unwrap_or_default(),
                patient: open_name.clone().unwrap_or_default(),
                state: past_work_state.clone(),
                briefings: patient_briefings.clone(),
                browser_tz: browser_tz.clone(),
                time_zone: browser_tz.clone(),
                server_now: server_now.clone(),
                on_retry: move |_| *past_work_attempt.write() += 1,
                on_open_briefing: move |target: OpenBriefingTarget| {
                    past_work_open.set(false);
                    open_briefing.set(Some(target));
                },
                on_open_item: move |item: PastWorkItem| {
                    match item.target {
                        PastWorkTarget::Conversation { chat_id } => {
                            open_chat.set(Some(chat_id));
                            past_work_open.set(false);
                            snapshot_attempt.set(snapshot_attempt() + 1);
                        }
                        PastWorkTarget::ReplyDraft { .. } => {
                            let Some(workspace_id) = selected() else {
                                past_work_open.set(false);
                                return;
                            };
                            let item_id = item.id.clone();
                            let client = client.read().clone();
                            let presented = client.session().get();
                            past_work_open.set(false);
                            spawn(async move {
                                match client.past_work(&workspace_id).await {
                                    Ok(page) => {
                                        if selected.peek().as_deref()
                                            != Some(workspace_id.as_str())
                                        {
                                            return;
                                        }
                                        if let Some(fresh) = page
                                            .items
                                            .into_iter()
                                            .find(|entry| entry.id == item_id)
                                        {
                                            reading.set(Some(prose_reading(
                                                fresh.title,
                                                fresh.summary,
                                            )));
                                        } else {
                                            reading.set(None);
                                            *past_work_attempt.write() += 1;
                                            *workspace_attempt.write() += 1;
                                        }
                                    }
                                    Err(error) => {
                                        reading.set(None);
                                        if let Some(refused) = refusal(&error) {
                                            session.refused(refused, *shown.peek(), presented);
                                        } else if matches!(
                                            error,
                                            physician_client::HttpError::Status {
                                                status: 403 | 404,
                                                ..
                                            }
                                        ) {
                                            last_past_work.set(None);
                                            *workspace_attempt.write() += 1;
                                            *today_attempt.write() += 1;
                                        }
                                    }
                                }
                            });
                        }
                    }
                },
                on_close: move |_| past_work_open.set(false),
            }
        }
    }
}

/// Whether the open patient workspace currently forbids clinical dialogs
/// (`ReaderDialog` and `PastWorkDialog`) because panel access was removed or
/// the workspace is unavailable to this physician.
fn clinical_dialogs_blocked(
    place: Place,
    selected_id: Option<&str>,
    gone_id: Option<&str>,
    cached_workspace: Option<&PatientWorkspaceResponse>,
) -> bool {
    if place != Place::Workspace {
        return false;
    }
    let Some(id) = selected_id else {
        return false;
    };
    if gone_id == Some(id) {
        return true;
    }
    cached_workspace
        .filter(|ws| ws.patient_workspace_id == id)
        .is_some_and(PatientWorkspaceResponse::is_clinic_demographic)
}

const COMMAND_REFUSED: &str = "Coda didn’t take that.";
const COMMAND_SENDING: &str = "Sending…";
const COMMAND_UNCONFIRMED: &str = "Coda couldn’t confirm that.";
pub(crate) const CHECK_AGAIN: &str = "Check again";
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
}

/// How a command was answered.
#[derive(Debug, PartialEq)]
enum Answer {
    Accepted(serde_json::Value),
    Refused(String),
    /// No answer arrived, so whether it took is unknown.
    Unconfirmed,
}

fn answer(result: &Result<serde_json::Value, physician_client::HttpError>) -> Answer {
    match result {
        Ok(value) => command_rejection(value)
            .map_or_else(|| Answer::Accepted(value.clone()), Answer::Refused),
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

/// Extracts `(patient_workspace_id, chat_id)` from a chat command payload so
/// persistence and response recovery stay bound to the dispatched target even
/// if the physician navigates elsewhere while the request is in flight.
fn command_target(command: &Command) -> Option<DraftTarget> {
    let payload = command.0.get("payload")?;
    let workspace = payload
        .get("patient_workspace_id")
        .and_then(serde_json::Value::as_str)
        .filter(|s| !s.is_empty())?
        .to_owned();
    let chat = payload
        .get("chat_id")
        .and_then(serde_json::Value::as_str)
        .unwrap_or_default()
        .to_owned();
    Some(DraftTarget { workspace, chat })
}

/// Optional same-tab persistence binding for chat commands so `Pending` /
/// `Unconfirmed` commands survive a page reload (`Chat Command 8`, `Chat Rule 13`).
#[derive(Clone, Copy, PartialEq)]
struct SlotPersistence {
    slot_id: &'static str,
    client: Signal<PhysicianClient>,
    whoami: Signal<Option<WhoamiResponse>>,
    workspace: Signal<Option<String>>,
    chat: Signal<Option<String>>,
}

impl SlotPersistence {
    #[cfg(target_arch = "wasm32")]
    fn save(self, command: &Command, stage: local_command::PersistedCommandStage) {
        let Some(account) = self.whoami.peek().clone() else {
            return;
        };
        let Some(target) = command_target(command) else {
            return;
        };
        let Some(scope) = local_draft::Scope::new(
            self.client.peek().base(),
            &account.practice_id,
            &account.physician_id,
        ) else {
            return;
        };
        if let Some(storage) = local_draft::SessionStorage::open() {
            local_command::write(
                &storage,
                &scope,
                &local_command::PersistedCommand {
                    slot: self.slot_id.to_owned(),
                    workspace: target.workspace,
                    chat: target.chat,
                    command: command.0.clone(),
                    stage,
                },
            );
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn save(self, _command: &Command, _stage: local_command::PersistedCommandStage) {}

    #[cfg(target_arch = "wasm32")]
    fn clear(self, command: &Command) {
        let Some(account) = self.whoami.peek().clone() else {
            return;
        };
        let Some(target) = command_target(command) else {
            return;
        };
        let Some(scope) = local_draft::Scope::new(
            self.client.peek().base(),
            &account.practice_id,
            &account.physician_id,
        ) else {
            return;
        };
        if let Some(storage) = local_draft::SessionStorage::open() {
            local_command::remove(
                &storage,
                &scope,
                self.slot_id,
                &target.workspace,
                &target.chat,
            );
        }
    }

    #[cfg(not(target_arch = "wasm32"))]
    fn clear(self, _command: &Command) {}

    fn matches_live_target(self, target: Option<&DraftTarget>) -> bool {
        let Some(target) = target else {
            return true;
        };
        self.workspace.peek().as_deref() == Some(target.workspace.as_str())
            && self.chat.peek().as_deref().unwrap_or("") == target.chat.as_str()
    }
}

type StoredSlotCommand = (Command, Callback<serde_json::Value>, Option<Callback>);

/// One command at a time on one item: what is in flight, and what Check again resends.
#[derive(Clone, Copy, PartialEq)]
struct CommandSlot {
    stage: Signal<CommandStage>,
    /// The unconfirmed command, what runs once it is taken, and what runs if
    /// it is refused.
    last: Signal<Option<StoredSlotCommand>>,
    generation: Signal<u64>,
    persistence: Option<SlotPersistence>,
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

fn use_command_slot() -> CommandSlot {
    CommandSlot {
        stage: use_signal(|| CommandStage::Idle),
        last: use_signal(|| None),
        generation: use_signal(|| 0),
        persistence: None,
    }
}

fn use_persisted_command_slot(
    slot_id: &'static str,
    client: Signal<PhysicianClient>,
    whoami: Signal<Option<WhoamiResponse>>,
    workspace: Signal<Option<String>>,
    chat: Signal<Option<String>>,
    on_restored_accepted: impl FnMut(serde_json::Value) + 'static,
) -> CommandSlot {
    let slot = CommandSlot {
        stage: use_signal(|| CommandStage::Idle),
        last: use_signal(|| None),
        generation: use_signal(|| 0),
        persistence: Some(SlotPersistence {
            slot_id,
            client,
            whoami,
            workspace,
            chat,
        }),
    };
    let on_accepted = Callback::new(on_restored_accepted);
    let mut slot_stage = slot.stage;
    let mut slot_last = slot.last;
    let mut slot_gen = slot.generation;
    let mut active_target = use_signal(|| None::<(Option<String>, Option<String>)>);
    #[cfg(target_arch = "wasm32")]
    let mut restored_for = use_signal(|| None::<String>);
    use_effect(move || {
        let ws_now = workspace.read().clone();
        let chat_now = chat.read().clone();
        let current = (ws_now.clone(), chat_now.clone());
        if active_target.peek().as_ref() != Some(&current) {
            active_target.set(Some(current));
            *slot_gen.write() += 1;
            slot_stage.set(CommandStage::Idle);
            slot_last.set(None);
            #[cfg(target_arch = "wasm32")]
            restored_for.set(None);
        }
        #[cfg(target_arch = "wasm32")]
        {
            let Some(account) = whoami.read().clone() else {
                return;
            };
            let Some(ws) = ws_now else {
                return;
            };
            let chat_id = chat_now.unwrap_or_default();
            let base = client.read().base().to_owned();
            let scope_key = format!(
                "{slot_id}|{}|{}|{ws}|{chat_id}",
                account.practice_id, account.physician_id
            );
            if restored_for.peek().as_deref() == Some(scope_key.as_str()) {
                return;
            }
            restored_for.set(Some(scope_key));
            let Some(scope) =
                local_draft::Scope::new(&base, &account.practice_id, &account.physician_id)
            else {
                return;
            };
            let Some(storage) = local_draft::SessionStorage::open() else {
                return;
            };
            let Some(saved) = local_command::read(&storage, &scope, slot_id, &ws, &chat_id) else {
                return;
            };
            if *slot_stage.peek() != CommandStage::Idle {
                return;
            }
            let cmd = Command(saved.command);
            match saved.stage {
                local_command::PersistedCommandStage::Pending => {
                    slot.send(client.peek().clone(), cmd, on_accepted, None);
                }
                local_command::PersistedCommandStage::Unconfirmed => {
                    slot_last.set(Some((cmd, on_accepted, None)));
                    slot_stage.set(CommandStage::Unconfirmed);
                }
            }
        }
        #[cfg(not(target_arch = "wasm32"))]
        let _ = (whoami, client, on_accepted);
    });
    slot
}

const COMMAND_AUTO_RETRY_DELAY_MS: u32 = 500;

/// Sends `command` once, automatically asking again once on `Answer::Unconfirmed`
/// before settling into `Unconfirmed` (`Chat Command 4/6`).
async fn dispatch_with_auto_retry(client: &PhysicianClient, command: &serde_json::Value) -> Answer {
    match answer(&client.command(command).await) {
        Answer::Unconfirmed => {
            sleep_ms(COMMAND_AUTO_RETRY_DELAY_MS).await;
            answer(&client.command(command).await)
        }
        settled => settled,
    }
}

impl CommandSlot {
    fn sending(&self) -> bool {
        *self.stage.read() == CommandStage::Sending
    }

    /// Sends `command` unless one is already in flight; `then` runs once it is taken.
    fn run(self, client: PhysicianClient, command: Command, mut then: impl FnMut() + 'static) {
        self.run_with_response(client, command, move |_| then());
    }

    /// Like `run`, passing the server's accepted JSON payload to `then`.
    fn run_with_response(
        self,
        client: PhysicianClient,
        command: Command,
        then: impl FnMut(serde_json::Value) + 'static,
    ) {
        self.send(client, command, Callback::new(then), None);
    }

    /// Like `run`; `refused` runs when the server answers and does not take it.
    fn run_answered(
        self,
        client: PhysicianClient,
        command: Command,
        then: Callback,
        refused: Callback,
    ) {
        self.send(
            client,
            command,
            Callback::new(move |_| then.call(())),
            Some(refused),
        );
    }

    fn send(
        mut self,
        client: PhysicianClient,
        command: Command,
        then: Callback<serde_json::Value>,
        refused: Option<Callback>,
    ) {
        if *self.stage.peek() == CommandStage::Sending {
            return;
        }
        let start_gen = *self.generation.peek();
        let target = command_target(&command);
        self.last.set(Some((command.clone(), then, refused)));
        self.stage.set(CommandStage::Sending);
        if let Some(p) = self.persistence {
            p.save(&command, local_command::PersistedCommandStage::Pending);
        }
        spawn(async move {
            let answer = dispatch_with_auto_retry(&client, &command.0).await;
            let still_current = *self.generation.peek() == start_gen
                && self
                    .persistence
                    .is_none_or(|p| p.matches_live_target(target.as_ref()));
            match answer {
                Answer::Accepted(value) => {
                    if let Some(p) = self.persistence {
                        p.clear(&command);
                    }
                    if still_current {
                        self.stage.set(CommandStage::Idle);
                        self.last.set(None);
                    }
                    then.call(value);
                }
                Answer::Refused(reason) => {
                    if let Some(p) = self.persistence {
                        p.clear(&command);
                    }
                    if still_current {
                        self.stage.set(CommandStage::Refused(reason));
                        self.last.set(None);
                        if let Some(refused) = refused {
                            refused.call(());
                        }
                    }
                }
                Answer::Unconfirmed => {
                    if let Some(p) = self.persistence {
                        p.save(&command, local_command::PersistedCommandStage::Unconfirmed);
                    }
                    if still_current {
                        self.stage.set(CommandStage::Unconfirmed);
                    }
                }
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
        if let Some((command, then, refused)) = self.last.peek().clone() {
            self.send(client, command, then, refused);
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
            Answer::Accepted(serde_json::json!({ "outcome": "accepted" }))
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

fn history_outcome_tone(outcome: HistoryWorkOutcome) -> Tone {
    match outcome {
        HistoryWorkOutcome::Answered => Tone::Neutral,
        HistoryWorkOutcome::Running | HistoryWorkOutcome::DraftReady => Tone::Primary,
        HistoryWorkOutcome::Held => Tone::Warning,
        HistoryWorkOutcome::Failed | HistoryWorkOutcome::RevisionFailed => Tone::Error,
    }
}

fn distinct_history_summary(
    summary: Option<&str>,
    title: &str,
    outcome: Option<HistoryWorkOutcome>,
) -> Option<String> {
    let trimmed = summary?.trim();
    if trimmed.is_empty()
        || trimmed == title.trim()
        || outcome.is_some_and(|status| trimmed == status.label())
    {
        return None;
    }
    Some(trimmed.to_owned())
}

#[component]
fn RecentRow(conversation: RecentConversation, on_open: EventHandler<(String, String)>) -> Element {
    let title = conversation
        .title
        .clone()
        .unwrap_or_else(|| "Untitled".to_owned());
    let workspace_id = conversation.patient_workspace_id.clone();
    let chat_id = conversation.chat_id.clone();
    let summary = distinct_history_summary(
        conversation.summary.as_deref(),
        &title,
        conversation.outcome,
    );
    rsx! {
        NavigationRow {
            class: "sidebar-recent-link",
            onclick: move |_| {
                if let (Some(workspace_id), Some(chat_id)) =
                    (workspace_id.clone(), chat_id.clone())
                {
                    on_open.call((workspace_id, chat_id));
                }
            },
            Icon { name: IconName::Message }
            span { class: "item-text chat-row-copy",
                span { class: "sidebar-recent-title", "{title}" }
                if conversation.patient_display_name.is_some() || conversation.outcome.is_some() {
                    span { class: "chat-row-meta",
                        if let Some(patient) = conversation.patient_display_name {
                            span { class: "sidebar-recent-patient", "{patient}" }
                        }
                        if let Some(outcome) = conversation.outcome {
                            StatePill { tone: history_outcome_tone(outcome), "{outcome.label()}" }
                        }
                    }
                }
                if let Some(summary) = summary {
                    span { class: "chat-row-summary", "{summary}" }
                }
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct TodayCannotRestartPrompt {
    workspace_id: String,
    follow_up_id: String,
    new_date: String,
    note: String,
    submitting: bool,
    error: Option<String>,
}

fn parse_unhandled_followup_key(item_id: &str) -> Option<(String, String)> {
    let rest = item_id.strip_prefix("unhandled-followup:")?;
    let (ws, fid) = rest.split_once(':')?;
    let ws = ws.trim();
    let fid = fid.trim();
    (!ws.is_empty() && !fid.is_empty()).then(|| (ws.to_owned(), fid.to_owned()))
}

#[component]
fn TodayCannotRestartPromptCard(
    prompt: TodayCannotRestartPrompt,
    default_date: String,
    on_change_date: EventHandler<String>,
    on_change_note: EventHandler<String>,
    on_dismiss: EventHandler,
    on_resolve: EventHandler<physician_wire::follow_ups::CannotRestartResolutionWire>,
) -> Element {
    let fid = prompt.follow_up_id.clone();
    let busy = prompt.submitting;
    let new_date = if prompt.new_date.trim().is_empty() {
        default_date
    } else {
        prompt.new_date.clone()
    };
    let note = prompt.note.clone();
    rsx! {
        section {
            class: "today-cannot-restart-prompt",
            role: "region",
            aria_label: "Resolve follow-up that could not restart",
            "data-today-cannot-restart-prompt": "{fid}",
            Card {
                ui::Notice {
                    tone: Tone::Warning,
                    icon: Some(IconName::Alert),
                    title: "Choose how to resolve this follow-up.".to_owned(),
                    detail: Some("Automated review couldn’t restart. Choose Complete, Set a new date, or Cancel follow-up.".to_owned()),
                }
                if let Some(err) = &prompt.error {
                    ui::Notice {
                        tone: Tone::Error,
                        icon: Some(IconName::Alert),
                        title: err.clone(),
                    }
                }
                div { class: "fu-form-stack",
                    div { "data-today-cr-date": "{fid}",
                        DateField {
                            label: "New review date".to_owned(),
                            kind: DateKind::Date,
                            value: new_date.clone(),
                            disabled: busy,
                            on_change: move |val| on_change_date.call(val),
                        }
                    }
                    div { "data-today-cr-note": "{fid}",
                        TextField {
                            label: "Resolution note (optional)".to_owned(),
                            placeholder: "Optional note for completion, reschedule, or cancellation".to_owned(),
                            value: note.clone(),
                            disabled: busy,
                            on_input: move |val| on_change_note.call(val),
                            onkeydown: move |ev: KeyboardEvent| ev.stop_propagation(),
                        }
                    }
                    ActionRow {
                        Button {
                            compact: true,
                            variant: ButtonVariant::Primary,
                            busy,
                            "data-today-cr-complete": "{fid}",
                            onclick: {
                                let note = note.clone();
                                move |_| {
                                    on_resolve.call(
                                        physician_wire::follow_ups::CannotRestartResolutionWire::Complete {
                                            resolution: note.trim().to_owned(),
                                        },
                                    );
                                }
                            },
                            "Complete"
                        }
                        Button {
                            compact: true,
                            variant: ButtonVariant::Secondary,
                            disabled: busy,
                            "data-today-cr-new-date": "{fid}",
                            onclick: {
                                let new_date = new_date.clone();
                                let note = note.clone();
                                move |_| {
                                    on_resolve.call(
                                        physician_wire::follow_ups::CannotRestartResolutionWire::NewDate {
                                            due_date: new_date.trim().to_owned(),
                                            note: (!note.trim().is_empty()).then(|| note.trim().to_owned()),
                                        },
                                    );
                                }
                            },
                            "Set a new date"
                        }
                        Button {
                            compact: true,
                            variant: ButtonVariant::Quiet,
                            danger: true,
                            disabled: busy,
                            "data-today-cr-cancel": "{fid}",
                            onclick: {
                                let note = note.clone();
                                move |_| {
                                    on_resolve.call(
                                        physician_wire::follow_ups::CannotRestartResolutionWire::Cancel {
                                            reason: note.trim().to_owned(),
                                        },
                                    );
                                }
                            },
                            "Cancel follow-up"
                        }
                        Button {
                            compact: true,
                            variant: ButtonVariant::Quiet,
                            disabled: busy,
                            "data-today-cr-dismiss": "{fid}",
                            onclick: move |_| on_dismiss.call(()),
                            "Dismiss"
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn NowBanner(concern: TodayLine, on_open: EventHandler<TodayLine>) -> Element {
    let open_line = concern.clone();
    rsx! {
        section {
            class: "now-banner",
            role: "region",
            aria_label: "Urgent patient concern",
            div { class: "now-banner-body",
                LiveDot {}
                StatePill { tone: Tone::Error, icon: IconName::Alert, "Now" }
                strong { class: "now-banner-who", "{concern.patient}" }
                if !concern.reason.is_empty() {
                    span { class: "now-banner-reason", "{concern.reason}" }
                }
            }
            Button {
                compact: true,
                variant: ButtonVariant::Primary,
                onclick: move |_| on_open.call(open_line.clone()),
                "Open"
            }
        }
    }
}

#[component]
fn CodaOrbDock(
    urgent: Option<TodayLine>,
    active_runs: Vec<ActiveReviewRun>,
    update_rows: Vec<serde_json::Value>,
    on_open_coda: EventHandler,
    on_open_updates: EventHandler,
    on_open_urgent: EventHandler<TodayLine>,
) -> Element {
    let mut preview_open = use_signal(|| false);
    let orb_state = if urgent.is_some() {
        OrbState::Urgent
    } else if !active_runs.is_empty() {
        OrbState::Working
    } else {
        OrbState::Idle
    };
    rsx! {
        div {
            class: "orb-dock",
            onpointerenter: move |_| preview_open.set(true),
            onpointerleave: move |_| preview_open.set(false),
            onfocusin: move |_| preview_open.set(true),
            onkeydown: move |event: KeyboardEvent| {
                if event.key() == Key::Escape {
                    preview_open.set(false);
                }
            },
            div { class: "orb-dock-bar",
                Orb {
                    aria_label: "Open Coda",
                    state: orb_state,
                    expanded: Some(preview_open()),
                    onclick: move |_| on_open_coda.call(()),
                }
                if let Some(concern) = urgent {
                    div {
                        class: "orb-now-pill",
                        role: "region",
                        aria_label: "Urgent patient concern",
                        LiveDot {}
                        StatePill { tone: Tone::Error, icon: IconName::Alert, "Now" }
                        span { class: "orb-now-copy",
                            strong { "{concern.patient}" }
                            if !concern.reason.is_empty() {
                                span { "{concern.reason}" }
                            }
                        }
                        Button {
                            compact: true,
                            variant: ButtonVariant::Primary,
                            onclick: move |_| on_open_urgent.call(concern.clone()),
                            "Open"
                        }
                    }
                }
            }
            if preview_open() {
                section {
                    class: "orb-preview",
                    aria_label: "Updates preview",
                    header { class: "orb-preview-head",
                        strong { "Updates" }
                    }
                    if !active_runs.is_empty() {
                        div { class: "orb-preview-section",
                            for run in active_runs {
                                div { class: "orb-preview-run",
                                    WorkingMark {}
                                    span { class: "orb-preview-item-copy",
                                        strong { "{run.patient_display_name}" }
                                        span { "{run.headline}" }
                                    }
                                }
                            }
                        }
                    }
                    div { class: "orb-preview-section",
                        if update_rows.is_empty() {
                            p { class: "orb-preview-empty", "{UPDATES_EMPTY}" }
                        } else {
                            for row in update_rows.iter().take(4) {
                                div { class: "orb-preview-item",
                                    strong {
                                        "{row.get(\"patient_display_name\").and_then(|value| value.as_str()).unwrap_or(\"Update\")}"
                                    }
                                    if let Some(detail) = row
                                        .get("headline")
                                        .or_else(|| row.get("summary"))
                                        .or_else(|| row.get("reason"))
                                        .and_then(|value| value.as_str())
                                        .filter(|text| !text.is_empty())
                                    {
                                        span { "{detail}" }
                                    }
                                }
                            }
                        }
                    }
                    div { class: "orb-preview-actions",
                        Button {
                            compact: true,
                            onclick: move |_| {
                                preview_open.set(false);
                                on_open_coda.call(());
                            },
                            "Open Coda"
                        }
                        Button {
                            compact: true,
                            variant: ButtonVariant::Primary,
                            onclick: move |_| {
                                preview_open.set(false);
                                on_open_updates.call(());
                            },
                            "All updates"
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn TodaySurface(
    state: TodayView,
    meta: today_updates::TodayMeta,
    connections: Vec<String>,
    source_connections: Vec<physician_wire::TodayConnection>,
    on_open_source: EventHandler<Option<physician_wire::source_setup::SupportedSourceWire>>,
    schedule_rows: Vec<ScheduleVisitRow>,
    show_schedule: bool,
    schedule: ReadStage,
    #[props(default)] schedule_may_be_missing_someone: bool,
    schedule_freshness: SchedulePaneFreshness,
    practice_zone: String,
    browser_tz: String,
    on_schedule_retry: EventHandler,
    schedule_empty: String,
    schedule_collapsed: bool,
    on_schedule_collapsed: EventHandler<bool>,
    server_now: String,
    practice_date: Option<String>,
    need_you: usize,
    focused: Option<i32>,
    open_failure: Option<today_updates::FailedItemOpen>,
    opening_busy: bool,
    done_state: today_updates::DoneActionState,
    on_retry: EventHandler,
    on_retry_open: EventHandler<today_updates::OpenItemTarget>,
    on_dismiss_open_failure: EventHandler,
    on_dismiss_done_failure: EventHandler,
    on_open_updates: EventHandler,
    on_open_patient: EventHandler<String>,
    on_open_row: EventHandler<TodayLine>,
    on_done_row: EventHandler<TodayLine>,
    on_activate_visit: EventHandler<ScheduleVisitRow>,
    on_resume: EventHandler<TodayLine>,
    resume_notice: Element,
) -> Element {
    let _ = on_open_patient;
    let words = need_you_words(need_you);
    let _ = &on_open_patient;
    let scheduled = show_schedule;
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
    let show_quiet = state.content().is_some_and(|rows| {
        today_updates::today_shows_nothing_needs_you(rows, &connections, &source_connections, &meta)
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
                    source_setup::TodaySourceConnections {
                        connections: source_connections,
                        practice_zone: practice_zone.clone(),
                        server_now: server_now.clone(),
                        on_open_source,
                    }
                    for line in connections {
                        p { class: "today-connection", "{line}" }
                    }
                    today_updates::TodayPracticeOutages {
                        outages: meta.practice_outages.clone(),
                    }
                    ReadNotice {
                        stage: state.stage(),
                        loading: TODAY_LOADING,
                        stale: TODAY_STALE,
                        unavailable: TODAY_UNAVAILABLE,
                        retry: TRY_AGAIN,
                        on_retry,
                    }
                    today_updates::TodayPartialFailureBanner {
                        surface: "Today",
                        failed_parts: meta.failed_parts.clone(),
                        incomplete: meta.incomplete,
                        on_retry,
                    }
                    access_notice::MayBeMissingSomeoneNotice {
                        visible: meta.may_be_missing_someone,
                    }
                    today_updates::ItemOpenFailureBanner {
                        failure: open_failure,
                        opening_busy,
                        on_retry: on_retry_open,
                        on_dismiss: on_dismiss_open_failure,
                    }
                    today_updates::DoneFailureBanner {
                        state: done_state.clone(),
                        on_dismiss: on_dismiss_done_failure,
                    }
                    {resume_notice}
                    match state.content().cloned() {
                        Some(_) if show_quiet => rsx! {
                            div { class: "today-list today-quiet",
                                strong { "{TODAY_QUIET}" }
                                span { "{TODAY_QUIET_DETAIL}" }
                            }
                        },
                        Some(rows) if !rows.is_empty() => rsx! {
                            TodayRows {
                                rows,
                                server_now: server_now.clone(),
                                focused: row_focus,
                                done_state,
                                on_open: on_open_row,
                                on_done: on_done_row,
                                on_resume,
                            }
                        },
                        _ => rsx! {},
                    }
                    today_updates::TodayAlsoReviewedBar {
                        also_reviewed: meta.also_reviewed,
                        on_open_updates,
                    }
                }
                if show_schedule {
                    ScheduleRailPane {
                        rows: schedule_rows,
                        context: RailContext::PracticeToday,
                        read: schedule,
                        freshness: schedule_freshness,
                        browser_tz,
                        on_retry: on_schedule_retry,
                        empty: schedule_empty.clone(),
                        collapsed: schedule_collapsed,
                        on_collapsed: on_schedule_collapsed,
                        focused: visit_focus,
                        on_activate: on_activate_visit,
                        may_be_missing_someone: schedule_may_be_missing_someone,
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
    done_state: today_updates::DoneActionState,
    on_open: EventHandler<TodayLine>,
    on_done: EventHandler<TodayLine>,
    on_resume: EventHandler<TodayLine>,
) -> Element {
    rsx! {
        div { class: "today-list",
            for (index, row) in rows.iter().enumerate() {
                article {
                    class: "today-row",
                    "data-today-item": "{row.item_id}",
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
                                if let Some(what_changed) = row.standing.changed_summary() {
                                    span {
                                        class: "tu-changed-badge",
                                        "data-changed-since-seen": "true",
                                        "Updated · {what_changed}"
                                    }
                                }
                            }
                            if let Some(waited) = format_waited(&row.waiting_since, &server_now) {
                                time { class: "today-row-waited", datetime: "{row.waiting_since}", "{waited}" }
                            }
                        }
                        if !row.reason.is_empty() && row.reason.trim() != row.did.trim() {
                            span { class: "today-row-reason", "{row.reason}" }
                        }
                    }
                    if row.resume_work_item_id.is_some()
                        || row.can_mark_done
                        || row.kind == "drafted_reply"
                    {
                        div { class: "today-row-action",
                            if row.kind == "drafted_reply" {
                                Button {
                                    compact: true,
                                    variant: ButtonVariant::Secondary,
                                    "data-today-review-reply": "{row.item_id}",
                                    onclick: {
                                        let row = row.clone();
                                        move |_| on_open.call(row.clone())
                                    },
                                    "Review reply"
                                }
                            }
                            if row.resume_work_item_id.is_some() {
                                Button {
                                    compact: true,
                                    onclick: {
                                        let row = row.clone();
                                        move |_| on_resume.call(row.clone())
                                    },
                                    "{TRY_AGAIN}"
                                }
                            }
                            if row.can_mark_done {
                                {
                                    let clearing = done_state.is_clearing(&row.item_id);
                                    let row_for_done = row.clone();
                                    rsx! {
                                        Button {
                                            compact: true,
                                            disabled: clearing,
                                            "data-today-done": "{row.item_id}",
                                            onclick: move |_| on_done.call(row_for_done.clone()),
                                            if clearing { "Saving…" } else { "Done" }
                                        }
                                    }
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
fn PatientSurface(
    gone: bool,
    read: ReadStage,
    reviews_context: patient_reviews_panel::PatientReviewsContext,
    spruce_replies_read: matching_reply::PatientSpruceRepliesRead,
    follow_ups_read: follow_ups::PatientFollowUpsRead,
    on_go_today: EventHandler,
    workspace_id: String,
    name: String,
    workspace: Option<PatientWorkspaceResponse>,
    rows: Vec<TodayLine>,
    schedule_rows: Vec<ScheduleVisitRow>,
    show_schedule: bool,
    schedule_freshness: SchedulePaneFreshness,
    browser_tz: String,
    schedule_collapsed: bool,
    on_schedule_collapsed: EventHandler<bool>,
    schedule: ReadStage,
    on_schedule_retry: EventHandler,
    on_activate_visit: EventHandler<ScheduleVisitRow>,
    briefings: Vec<BriefingSummaryItemWire>,
    on_open_briefing: EventHandler<OpenBriefingTarget>,
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
    #[props(default)] on_open_demo_source: Option<EventHandler<String>>,
    on_draft: EventHandler<String>,
    on_send: EventHandler,
    on_send_next: EventHandler,
    on_dictate: EventHandler,
    on_editing: EventHandler<bool>,
    conversation: ReadView<Vec<ConversationLine>>,
    queued: Vec<physician_wire::QueuedMessageWire>,
    queue_busy: bool,
    queue_notice: Element,
    on_queue_edit: EventHandler<(String, String)>,
    on_queue_withdraw: EventHandler<String>,
    on_queue_send_now: EventHandler<String>,
    on_conversation_retry: EventHandler,
    conversation_resume_busy: bool,
    conversation_resume_notice: Element,
    on_conversation_resume: EventHandler<WorkResumeTarget>,
    conversation_stop_busy: bool,
    conversation_stop_notice: Element,
    on_conversation_stop: EventHandler<WorkStopTarget>,
    on_open_chat: EventHandler<String>,
    on_new_chat: EventHandler,
    on_update_answer: EventHandler,
    on_open_patient_row: EventHandler<TodayLine>,
    on_done_patient_row: EventHandler<TodayLine>,
    active_evidence: Option<physician_wire::today_updates::ItemWorkEvidenceWire>,
    on_close_evidence: EventHandler,
    on_done_evidence: EventHandler<(String, String, String)>,
    open_failure: Option<today_updates::FailedItemOpen>,
    opening_busy: bool,
    done_state: today_updates::DoneActionState,
    on_retry_open: EventHandler<today_updates::OpenItemTarget>,
    on_dismiss_open_failure: EventHandler,
    on_dismiss_done_failure: EventHandler,
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
    on_open_patient: EventHandler<String>,
    on_patient_gone: EventHandler<String>,
    on_open_discuss_chat: EventHandler<String>,
    invite_control: Element,
) -> Element {
    let mut editing_queue = use_signal(|| None::<(String, String)>);
    let is_working = conversation
        .content()
        .is_some_and(|lines| lines.iter().any(|line| line.working));
    let clinic_demographic = workspace
        .as_ref()
        .is_some_and(PatientWorkspaceResponse::is_clinic_demographic);
    let demographic = workspace.as_ref().map(demographic_card_view);
    let segments = workspace
        .as_ref()
        .and_then(|open| open.patient_synopsis.as_ref())
        .map(|synopsis| synopsis.segments.clone())
        .unwrap_or_default();
    let synopsis_text = workspace
        .as_ref()
        .and_then(|open| open.patient_synopsis.as_ref())
        .and_then(|synopsis| synopsis.authored_text.clone())
        .filter(|text| !text.trim().is_empty());
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
    let records_state = workspace
        .as_ref()
        .map(|open| workspace_records_state(open, &server_now))
        .unwrap_or(WorkspaceRecordsState::Absent);
    let connected_source_lines = workspace
        .as_ref()
        .map(|open| source_lines(open, &server_now))
        .unwrap_or_default();
    let visit_focus = focused.and_then(|index| {
        if index >= 0 && (index as usize) < schedule_rows.len() {
            Some(index as usize)
        } else {
            None
        }
    });
    let scheduled = show_schedule;
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
                        {invite_control}
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
                    match records_state {
                        WorkspaceRecordsState::Current(records) => rsx! {
                            time { datetime: "{records.instant}", "{records.label} {records.words}" }
                        },
                        WorkspaceRecordsState::RefreshFailedWithSnapshot(records) => rsx! {
                            span { class: "records-stale-status",
                                "{records.label} "
                                time { datetime: "{records.instant}", "{records.words}" }
                                ", couldn't update"
                            }
                        },
                        WorkspaceRecordsState::Hydrating => rsx! {
                            span { "Connecting records…" }
                        },
                        WorkspaceRecordsState::Unavailable(message) => rsx! {
                            span { class: "records-stale-status", "{message}" }
                        },
                        WorkspaceRecordsState::Absent => rsx! {},
                    }
                }
                if !clinic_demographic && !connected_source_lines.is_empty() {
                    div { class: "patient-sources", aria_label: "Record sources",
                        for source in connected_source_lines {
                            match source.standing {
                                ConnectedSourceStanding::Usable { last_good } => rsx! {
                                    span { class: "patient-source-status", "{source.label} · As of {last_good}" }
                                },
                                ConnectedSourceStanding::RefreshFailed { last_good: Some(last_good) } => rsx! {
                                    span { class: "patient-source-status is-stale-source",
                                        "{source.label} · As of {last_good}, couldn't update"
                                    }
                                },
                                ConnectedSourceStanding::RefreshFailed { last_good: None } => rsx! {
                                    span { class: "patient-source-status is-stale-source",
                                        "{source.label} · Couldn't update"
                                    }
                                },
                                ConnectedSourceStanding::Connecting => rsx! {
                                    span { class: "patient-source-status", "{source.label} · Connecting…" }
                                },
                                ConnectedSourceStanding::AwaitingFirstRead => rsx! {
                                    span { class: "patient-source-status", "{source.label} · Waiting for first read" }
                                },
                            }
                        }
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
            today_updates::ItemOpenFailureBanner {
                failure: open_failure,
                opening_busy,
                on_retry: on_retry_open,
                on_dismiss: on_dismiss_open_failure,
            }
            today_updates::DoneFailureBanner {
                state: done_state.clone(),
                on_dismiss: on_dismiss_done_failure,
            }
            {work_notice}
            {resume_notice}
            if !rows.is_empty() {
                {
                    let rows_for_view = rows.clone();
                    let done_for_rows = done_state.clone();
                    rsx! {
                        section {
                            class: "patient-section",
                            aria_label: "Needs you for this patient",
                            "data-patient-today-rows": "true",
                            h2 { "Needs you" }
                            TodayRows {
                                rows: rows_for_view,
                                server_now: server_now.clone(),
                                focused: None,
                                done_state: done_for_rows,
                                on_open: move |line: TodayLine| on_open_patient_row.call(line),
                                on_done: move |line: TodayLine| on_done_patient_row.call(line),
                                on_resume,
                            }
                        }
                    }
                }
            }
            if let Some(evidence) = active_evidence {
                {
                    let can_done = rows
                        .iter()
                        .any(|row| row.item_id == evidence.item_id && row.can_mark_done);
                    let done_busy = done_state.is_clearing(&evidence.item_id);
                    let ws_for_done = evidence.patient_workspace_id.clone();
                    let id_for_done = evidence.item_id.clone();
                    let ver_for_done = evidence.version_id.clone();
                    rsx! {
                        today_updates::OpenedWorkEvidencePanel {
                            evidence,
                            can_mark_done: can_done,
                            done_busy,
                            on_mark_done: move |_| {
                                on_done_evidence.call((
                                    ws_for_done.clone(),
                                    id_for_done.clone(),
                                    ver_for_done.clone(),
                                ));
                            },
                            on_open_source,
                            on_close: on_close_evidence,
                        }
                    }
                }
            }
            if let Some(open) = workspace.as_ref().filter(|_| !clinic_demographic) {
                matching_reply::PatientSpruceRepliesSection {
                    workspace_id: open.patient_workspace_id.clone(),
                    patient_name: heading_name.clone(),
                    browser_tz: browser_tz.clone(),
                    read_state: spruce_replies_read,
                    client: reviews_context.client,
                    session: reviews_context.session,
                    shown: reviews_context.shown,
                    workspace_attempt: reviews_context.workspace_attempt,
                    today_attempt: reviews_context.today_attempt,
                    on_open_source,
                    on_open_basis: on_read,
                    on_open_patient,
                    on_patient_gone,
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
                                let preparation = schedule_rows
                                    .first()
                                    .and_then(ScheduleVisitRow::preparation_detail);
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
            if let Some(open) = workspace.as_ref().filter(|_| !clinic_demographic) {
                patient_reviews_panel::PatientReviewsPanel {
                    workspace_id: open.patient_workspace_id.clone(),
                    server_now: server_now.clone(),
                    time_zone: browser_tz.clone(),
                    context: reviews_context,
                    on_open_source,
                }
            }
            if !briefings.is_empty() && !workspace_id.is_empty() {
                PatientBriefingsSection {
                    workspace_id: workspace_id.clone(),
                    briefings,
                    browser_tz: browser_tz.clone(),
                    server_now: server_now.clone(),
                    on_open_briefing,
                }
            }
            if !segments.is_empty() || synopsis_text.is_some() {
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
                        if !segments.is_empty() {
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
                        } else if let Some(text) = synopsis_text {
                            span { "{text}" }
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
            if let Some(open) = workspace.as_ref().filter(|_| !clinic_demographic) {
                follow_ups::PatientFollowUpsSection {
                    workspace_id: open.patient_workspace_id.clone(),
                    browser_tz: browser_tz.clone(),
                    legacy_follow_ups: follow_ups,
                    read_state: follow_ups_read,
                    client: reviews_context.client,
                    session: reviews_context.session,
                    shown: reviews_context.shown,
                    workspace_attempt: reviews_context.workspace_attempt,
                    today_attempt: reviews_context.today_attempt,
                    on_open_discuss_chat,
                    on_patient_gone,
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
                        for line in lines {
                            NavigationRow {
                                class: "sidebar-conversation-link",
                                onclick: {
                                    let id = line.id.clone();
                                    move |_| on_open_chat.call(id.clone())
                                },
                                Icon { name: IconName::Message }
                                span { class: "chat-row-copy",
                                    span { class: "sidebar-recent-title", "{line.title}" }
                                    if let Some(summary) = distinct_history_summary(line.summary.as_deref(), &line.title, line.outcome) {
                                        span { class: "chat-row-summary", "{summary}" }
                                    }
                                }
                                if let Some(outcome) = line.outcome {
                                    StatePill { tone: history_outcome_tone(outcome), "{outcome.label()}" }
                                }
                            }
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
                        for (line_idx, line) in conversation.content().cloned().unwrap_or_default().into_iter().enumerate() {
                            if line.author == Some(Author::Person) {
                                ui::Message { key: "person-{line_idx}", author: Author::Person, by: "You",
                                    LiteralWords { text: line.text }
                                }
                            } else if !line.work_item_id.is_empty() && (line.stop.is_some() || line.resume.is_some() || line.author == Some(Author::Assistant)) {
                                {
                                    let has_trace = !line.steps.steps.is_empty() || line.steps.unavailable;
                                    let trace_label = line.steps.line_label.clone().unwrap_or_else(|| {
                                        if line.working { "Working".to_owned() } else { "Steps".to_owned() }
                                    });
                                    let steps_unavailable = line.steps.unavailable;
                                    let steps_items = line.steps.steps;
                                    rsx! {
                                        div { key: "work-{line.work_item_id}", class: "conversation-turn-end",
                                            if has_trace {
                                                ui::WorkTrace {
                                                    label: trace_label,
                                                    working: line.working,
                                                    detail: None,
                                                    if steps_unavailable && steps_items.is_empty() {
                                                        ui::WorkStep {
                                                            icon: IconName::Alert,
                                                            label: "Coda couldn’t load the steps for this turn.".to_owned(),
                                                            state: ui::StepState::Failed,
                                                            detail: None,
                                                        }
                                                    } else {
                                                        for (step_idx, step) in steps_items.into_iter().enumerate() {
                                                            ui::WorkStep {
                                                                key: "{step_idx}",
                                                                icon: step_icon(step.kind),
                                                                label: step.label,
                                                                state: step.state,
                                                                detail: step.detail,
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                            if let Some(target) = line.stop {
                                                div { class: "conversation-mechanical", role: "status",
                                                    if !has_trace {
                                                        ui::WorkingMark {}
                                                    }
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
                                            } else if line.author == Some(Author::Assistant) {
                                                ui::Message { author: Author::Assistant, by: "Coda",
                                                    actions: feedback_chat.clone().zip(line.response).map(|(chat, (response, read))| rsx! {
                                                        feedback::ResponseRating { chat, response, read }
                                                    }),
                                                    if line.freshness == AnswerFreshness::NewerRecordsArrived {
                                                        div {
                                                            class: "answer-update-banner",
                                                            role: "status",
                                                            "data-answer-freshness": "newer-records",
                                                            span { "{ANSWER_NEWER_RECORDS}" }
                                                            Button {
                                                                compact: true,
                                                                onclick: move |_| on_update_answer.call(()),
                                                                "Update"
                                                            }
                                                        }
                                                    }
                                                    if !line.body_pieces.is_empty() {
                                                        CitedMessage { pieces: line.body_pieces, on_open_source }
                                                    } else {
                                                        MessageWords { text: line.text }
                                                    }
                                                    if !line.basis_pieces.is_empty() {
                                                        AnswerBasis { pieces: line.basis_pieces, on_open_source }
                                                    }
                                                    if !line.sources.is_empty()
                                                        && let Some(open_demo_source) = on_open_demo_source
                                                    {
                                                        div { class: "message-sources",
                                                            for (idx, src) in line.sources.iter().enumerate() {
                                                                {
                                                                    let record_id = src.record_id.clone();
                                                                    let label = src.title.clone();
                                                                    rsx! {
                                                                        ui::CitationChip {
                                                                            index: idx + 1,
                                                                            label,
                                                                            onclick: move |_| open_demo_source.call(record_id.clone()),
                                                                        }
                                                                    }
                                                                }
                                                            }
                                                        }
                                                    }
                                                }
                                            } else {
                                                MessageWords { text: line.text }
                                            }
                                        }
                                    }
                                }
                            } else if let Some(target) = line.stop {
                                div { key: "stop-{line_idx}", class: "conversation-mechanical", role: "status",
                                    ui::WorkingMark {}
                                    p { "{line.text}" }
                                    Button {
                                        compact: true,
                                        disabled: conversation_stop_busy,
                                        onclick: move |_| on_conversation_stop.call(target.clone()),
                                        "{STOP}"
                                    }
                                }
                            } else if let Some(target) = line.resume {
                                div { key: "resume-{line_idx}", class: "conversation-mechanical", role: "status",
                                    p { "{line.text}" }
                                    Button {
                                        compact: true,
                                        disabled: conversation_resume_busy,
                                        onclick: move |_| on_conversation_resume.call(target.clone()),
                                        if line.held { "{RESUME}" } else { "{TRY_AGAIN}" }
                                    }
                                }
                            } else if line.author == Some(Author::Assistant) {
                                ui::Message { key: "kept-{line_idx}", author: Author::Assistant, by: "Coda",
                                    MessageWords { text: line.text }
                                }
                            } else {
                                MessageWords { text: line.text }
                            }
                        }
                        {conversation_resume_notice}
                        {conversation_stop_notice}
                        {queue_notice}
                        if !queued.is_empty() {
                            ui::MessageQueue {
                                ui::MessageStatus {
                                    words: "Sends after Coda's reply".to_owned(),
                                    icon: Some(IconName::Clock),
                                }
                                for item in queued {
                                    if let Some((_, draft_edit)) = editing_queue.read().as_ref().filter(|(id, _)| id == &item.id.0).cloned() {
                                        {
                                            let edit_id = item.id.0.clone();
                                            let save_id = item.id.0.clone();
                                            rsx! {
                                                ui::MessageEditor {
                                                    key: "{item.id.0}",
                                                    label: "Edit queued message".to_owned(),
                                                    value: draft_edit,
                                                    on_input: move |next| {
                                                        editing_queue.set(Some((edit_id.clone(), next)));
                                                    },
                                                    on_save: move |next| {
                                                        editing_queue.set(None);
                                                        on_queue_edit.call((save_id.clone(), next));
                                                    },
                                                    on_cancel: move |_| editing_queue.set(None),
                                                    save_label: "Update".to_owned(),
                                                    cancel_label: "Cancel".to_owned(),
                                                    note: None,
                                                }
                                            }
                                        }
                                    } else {
                                        {
                                            let send_now_id = item.id.0.clone();
                                            let edit_id = item.id.0.clone();
                                            let edit_text = item.text.clone();
                                            let withdraw_id = item.id.0.clone();
                                            rsx! {
                                                ui::Message {
                                                    key: "{item.id.0}",
                                                    author: Author::Person,
                                                    by: "You",
                                                    state: ui::MessageState::Queued,
                                                    actions: Some(rsx! {
                                                        ui::MessageActions {
                                                            label: "Queued message actions".to_owned(),
                                                            Button {
                                                                compact: true,
                                                                disabled: queue_busy,
                                                                onclick: move |_| on_queue_send_now.call(send_now_id.clone()),
                                                                "Send now"
                                                            }
                                                            Button {
                                                                compact: true,
                                                                disabled: queue_busy,
                                                                onclick: move |_| {
                                                                    editing_queue.set(Some((edit_id.clone(), edit_text.clone())));
                                                                },
                                                                "Edit"
                                                            }
                                                            Button {
                                                                compact: true,
                                                                disabled: queue_busy,
                                                                onclick: move |_| on_queue_withdraw.call(withdraw_id.clone()),
                                                                "Delete"
                                                            }
                                                        }
                                                    }),
                                                    LiteralWords { text: item.text }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        div { class: "composer-dock",
                            ui::Composer {
                                label: "Message",
                                placeholder: "Write a message",
                                value: draft.clone(),
                                on_input: move |text| on_draft.call(text),
                                on_submit: move |_| on_send.call(()),
                                on_key_down: move |event: KeyboardEvent| {
                                    if event.key() != Key::Enter || event.is_composing() || event.modifiers().shift() {
                                        return;
                                    }
                                    event.prevent_default();
                                    event.stop_propagation();
                                    if (event.modifiers().meta() || event.modifiers().ctrl()) && is_working {
                                        on_send_next.call(());
                                    } else {
                                        on_send.call(());
                                    }
                                },
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
                                if is_working {
                                    Button {
                                        compact: true,
                                        onclick: move |_| on_send_next.call(()),
                                        "Send next"
                                    }
                                    Button { variant: ButtonVariant::Primary, compact: true, submit: true, "Send now" }
                                } else {
                                    Button { variant: ButtonVariant::Primary, compact: true, submit: true, "Send" }
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
            if show_schedule {
                ScheduleRailPane {
                    rows: schedule_rows,
                    context: RailContext::PatientUpcoming,
                    read: schedule,
                    freshness: schedule_freshness,
                    browser_tz,
                    on_retry: on_schedule_retry,
                    empty: String::new(),
                    collapsed: schedule_collapsed,
                    on_collapsed: on_schedule_collapsed,
                    focused: visit_focus,
                    on_activate: on_activate_visit,
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
    continuation: RosterContinuation,
    #[props(default)] may_be_missing_someone: bool,
    on_retry: EventHandler,
    on_scroll_more: EventHandler,
    on_retry_more: EventHandler,
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
    let is_blank_query = query.trim().is_empty();
    let empty_message = if state.content().is_some_and(Vec::is_empty) && !may_be_missing_someone {
        Some(if is_blank_query {
            ROSTER_EMPTY
        } else {
            SEARCH_NONE
        })
    } else {
        None
    };
    let loading_copy = if is_blank_query {
        ROSTER_LOADING
    } else {
        SEARCH_LOADING
    };
    let unavailable_copy = if is_blank_query {
        ROSTER_UNAVAILABLE
    } else {
        SEARCH_UNAVAILABLE
    };
    let more_stage = match continuation {
        RosterContinuation::Complete | RosterContinuation::HasMore { .. } => ReadStage::Showing,
        RosterContinuation::LoadingMore { .. } => ReadStage::Loading,
        RosterContinuation::FailedMore { .. } => ReadStage::Unavailable,
    };
    rsx! {
        Dialog { close_label: "Close", title: "Find patient", on_close,
                div { class: "patient-search",
                    SearchField {
                        label: "Search your panel",
                        placeholder: "Name, DOB or MRN",
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
                        loading: loading_copy,
                        stale: "",
                        unavailable: unavailable_copy,
                        retry: TRY_AGAIN,
                        on_retry,
                    }
                    access_notice::MayBeMissingSomeoneNotice {
                        visible: may_be_missing_someone,
                    }
                    if let Some(empty_copy) = empty_message {
                        EmptyLine { message: empty_copy }
                    }
                    if !results.is_empty() {
                        div {
                            class: "patient-search-scroll",
                            onscroll: move |event| {
                                let data = event.data();
                                let from_end = f64::from(data.scroll_height())
                                    - f64::from(data.client_height())
                                    - data.scroll_top();
                                if from_end <= ROSTER_SCROLL_THRESHOLD_PX {
                                    on_scroll_more.call(());
                                }
                            },
                            ui::RowGroup { class: "dialog-results",
                                for (index, patient) in results.iter().enumerate() {
                                    li {
                                        ui::ListRow {
                                            class: if focused == Some(index as i32) { "is-keyboard-focus" } else { "" },
                                            title: patient.patient_display_name.clone(),
                                            detail: search_row_detail(
                                                patient.birth_date.as_deref(),
                                                patient.mrn.as_deref(),
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
                        ReadNotice {
                            stage: more_stage,
                            loading: ROSTER_MORE_LOADING,
                            stale: "",
                            unavailable: ROSTER_MORE_UNAVAILABLE,
                            retry: TRY_AGAIN,
                            on_retry: on_retry_more,
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
pub(crate) fn CitationChip(chip: SourceChip, on_open: EventHandler<SourceChip>) -> Element {
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
fn PastWorkDialog(
    workspace_id: String,
    patient: String,
    state: ReadView<Vec<PastWorkItem>>,
    briefings: Vec<BriefingSummaryItemWire>,
    browser_tz: String,
    time_zone: String,
    server_now: String,
    on_retry: EventHandler,
    on_open_briefing: EventHandler<OpenBriefingTarget>,
    on_open_item: EventHandler<PastWorkItem>,
    on_close: EventHandler,
) -> Element {
    rsx! {
        Dialog {
            close_label: "Close",
            class: "past-work-dialog",
            title: "Past work",
            subtitle: (!patient.is_empty()).then_some(patient),
            on_close,
            ReadNotice {
                stage: state.stage(),
                loading: PAST_WORK_LOADING,
                stale: PAST_WORK_STALE,
                unavailable: PAST_WORK_UNAVAILABLE,
                retry: TRY_AGAIN,
                on_retry,
            }
            match state.content().cloned() {
                None if briefings.is_empty() => rsx! {},
                Some(items) if items.is_empty() && briefings.is_empty() => rsx! {
                    p { class: "past-work-empty", "{PAST_WORK_EMPTY}" }
                },
                maybe_items => {
                    let items = maybe_items.unwrap_or_default();
                    rsx! {
                        ui::RowGroup { class: "past-work-items",
                            for briefing in briefings {
                                li {
                                    ui::ListRow {
                                        icon: Some(IconName::Calendar),
                                        title: format!("{} · Visit briefing", briefing.visit_label),
                                        detail: Some(past_work_briefing_detail(&briefing, &server_now, &browser_tz)),
                                        onclick: {
                                            let ws = workspace_id.clone();
                                            let bid = briefing.briefing_id.clone();
                                            move |_| {
                                                on_open_briefing.call(OpenBriefingTarget {
                                                    workspace_id: ws.clone(),
                                                    briefing_id: bid.clone(),
                                                });
                                            }
                                        },
                                    }
                                }
                            }
                            for item in items {
                                li {
                                    ui::ListRow {
                                        icon: Some(IconName::Message),
                                        title: item.title.clone(),
                                        detail: past_work_item_detail(&item, &time_zone, &server_now),
                                        onclick: {
                                            let item = item.clone();
                                            move |_| on_open_item.call(item.clone())
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
}

fn past_work_briefing_detail(
    briefing: &BriefingSummaryItemWire,
    server_now: &str,
    browser_tz: &str,
) -> String {
    let (badge, detail) = history_standing_summary(briefing, server_now, browser_tz);
    match badge {
        Some((label, _)) => format!("{label} · {detail}"),
        None => detail,
    }
}

fn past_work_item_detail(item: &PastWorkItem, time_zone: &str, server_now: &str) -> String {
    let mut parts = vec![item.outcome.label().to_owned()];
    if let Some(summary) =
        distinct_history_summary(Some(&item.summary), &item.title, Some(item.outcome))
    {
        parts.push(summary);
    }
    if let Some(when) = format_context_time(&item.updated_at, time_zone, server_now) {
        parts.push(when);
    }
    parts.join(" · ")
}

#[component]
pub(crate) fn CitedMessage(
    pieces: Vec<SynopsisPiece>,
    on_open_source: EventHandler<SourceChip>,
) -> Element {
    let parts = pieces
        .into_iter()
        .map(|piece| match piece {
            SynopsisPiece::Text(text) => ui::ProsePart::Words(text),
            SynopsisPiece::Chips(chips) => ui::ProsePart::Inline(rsx! {
                for chip in chips {
                    CitationChip { chip: chip.clone(), on_open: on_open_source }
                }
            }),
        })
        .collect();
    rsx! {
        ui::Prose {
            message: true,
            names: ui::ProseNames { code: "Code".to_owned(), table: "Table".to_owned() },
            parts,
        }
    }
}

#[component]
pub(crate) fn AnswerBasis(
    pieces: Vec<SynopsisPiece>,
    on_open_source: EventHandler<SourceChip>,
) -> Element {
    rsx! {
        div { class: "basis-disclosure",
            ui::Disclosure {
                title: "Basis and limitations",
                detail: true,
                CitedMessage { pieces, on_open_source }
            }
        }
    }
}

/// What Coda says in a chat snapshot, drawn richly. This doesn't fetch a source or run a model.
#[component]
pub(crate) fn MessageWords(text: String) -> Element {
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
                div { class: "shortcut", span { "Send message (or Send now while Coda works)" } kbd { "Enter" } }
                div { class: "shortcut", span { "New line in message" } kbd { "Shift + Enter" } }
                div { class: "shortcut", span { "Send next after Coda's reply" } kbd { "⌘ Enter / Ctrl + Enter" } }
                div { class: "shortcut", span { "Close the current preview or dialog" } kbd { "Esc" } }
                div { class: "shortcut", span { "Hold to record; release to transcribe" } kbd { "Ctrl+Shift+Space" } }
                p { class: "dialog-note", "Navigation shortcuts pause while you are typing. Dictation works in text fields while this tab has focus." }
        }
    }
}

#[derive(Clone)]
enum FocusTarget {
    Row(TodayLine),
    Visit(ScheduleVisitRow),
    Patient(String),
}

fn focus_targets(
    search_open: bool,
    place: Place,
    today: &TodayView,
    visits: &[ScheduleVisitRow],
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
        local_command::forget_workspace(&storage, &scope, workspace);
    }
    #[cfg(not(target_arch = "wasm32"))]
    let _ = (client, whoami);
}

/// Waits `ms` milliseconds without blocking the page.
#[cfg(target_arch = "wasm32")]
pub(crate) async fn sleep_ms(ms: u32) {
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
pub(crate) async fn sleep_ms(ms: u32) {
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

#[derive(Clone, Copy)]
struct SendRefresh {
    snapshot: Signal<u32>,
    workspace: Signal<u32>,
    today: Signal<u32>,
}

impl SendRefresh {
    fn bump(&mut self) {
        *self.snapshot.write() += 1;
        *self.workspace.write() += 1;
        *self.today.write() += 1;
    }
}

/// Sends a captured message. Acceptance removes only the submitted text and
/// queues its durable clear; refusal keeps the text as an ordinary draft; no
/// answer leaves the Send pending, and Check again (or a reload) asks again
/// with the same command and key.
fn dispatch_send(
    slot: CommandSlot,
    client: Signal<PhysicianClient>,
    composer: Signal<Composer>,
    mut refresh: SendRefresh,
    target: DraftTarget,
    pending: composer::PendingSend,
) {
    let key = pending.key.clone();
    let accepted = {
        let (target, key) = (target.clone(), key.clone());
        Callback::new(move |()| {
            let mut held = composer;
            held.write().sent(&target, &key, SendAnswer::Accepted);
            refresh.bump();
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

/// Maps a projected step kind to its UI icon.
fn step_icon(kind: ProjectedStepKind) -> IconName {
    match kind {
        ProjectedStepKind::Search => IconName::Search,
        ProjectedStepKind::Read => IconName::File,
        ProjectedStepKind::Conversation => IconName::Message,
    }
}

/// The message the physician wrote, as one command with its delivery mode.
/// Nothing goes without a patient, a conversation and a time zone.
fn message_command_with_delivery(
    workspace_id: Option<String>,
    chat_id: Option<String>,
    time_zone: Option<String>,
    text: String,
    delivery: SubmitDelivery,
) -> Result<Command, String> {
    let (Some(workspace_id), Some(chat_id), Some(time_zone)) = (workspace_id, chat_id, time_zone)
    else {
        return Err(MESSAGE_REFUSED.to_owned());
    };
    let mut payload = serde_json::json!({
        "patient_workspace_id": workspace_id,
        "chat_id": chat_id,
        "text": text,
        "physician_time_zone": time_zone,
    });
    if delivery == SubmitDelivery::SendNext
        && let Some(obj) = payload.as_object_mut()
    {
        obj.insert("delivery".to_owned(), serde_json::json!("send_next"));
    }
    Ok(Command::new("message.submit", payload))
}

/// Builds the "Update this answer" message command directly without altering
/// any unsent draft held by the composer.
fn update_answer_command(
    workspace_id: Option<String>,
    chat_id: Option<String>,
    time_zone: Option<String>,
) -> Result<Command, String> {
    message_command_with_delivery(
        workspace_id,
        chat_id,
        time_zone,
        UPDATE_ANSWER_PROMPT.to_owned(),
        SubmitDelivery::SendNow,
    )
}

/// Extracts queued message texts returned to the composer when Coda stops.
fn returned_queue_texts(value: &serde_json::Value) -> Vec<String> {
    value
        .get("returned_messages")
        .and_then(serde_json::Value::as_array)
        .map(|items| {
            items
                .iter()
                .filter_map(serde_json::Value::as_str)
                .map(str::to_owned)
                .collect()
        })
        .unwrap_or_default()
}

/// Extracts `(patient_workspace_id, chat_id)` from an accepted `work.stop`
/// payload so queue restoration targets the patient conversation that stopped.
fn returned_queue_target(value: &serde_json::Value) -> Option<DraftTarget> {
    let workspace = value
        .get("patient_workspace_id")
        .and_then(serde_json::Value::as_str)
        .filter(|s| !s.is_empty())?
        .to_owned();
    let chat = value
        .get("chat_id")
        .and_then(serde_json::Value::as_str)
        .filter(|s| !s.is_empty())?
        .to_owned();
    Some(DraftTarget { workspace, chat })
}

fn restore_returned_queue_from_response(
    value: &serde_json::Value,
    fallback_target: Option<DraftTarget>,
    mut composer: Signal<Composer>,
    client: Signal<PhysicianClient>,
) {
    let returned = returned_queue_texts(value);
    if returned.is_empty() {
        return;
    }
    let Some(target) = returned_queue_target(value).or(fallback_target) else {
        return;
    };
    composer.write().restore_returned_queue(&target, &returned);
    autosave(composer, client, target);
}

fn queue_edit_command(
    workspace_id: Option<String>,
    chat_id: Option<String>,
    message_id: String,
    text: String,
) -> Option<Command> {
    let (Some(workspace_id), Some(chat_id)) = (workspace_id, chat_id) else {
        return None;
    };
    Some(Command::new(
        "message.queue_edit",
        serde_json::json!({
            "patient_workspace_id": workspace_id,
            "chat_id": chat_id,
            "message_id": message_id,
            "text": text,
        }),
    ))
}

fn queue_withdraw_command(
    workspace_id: Option<String>,
    chat_id: Option<String>,
    message_id: String,
) -> Option<Command> {
    let (Some(workspace_id), Some(chat_id)) = (workspace_id, chat_id) else {
        return None;
    };
    Some(Command::new(
        "message.withdraw",
        serde_json::json!({
            "patient_workspace_id": workspace_id,
            "chat_id": chat_id,
            "message_id": message_id,
        }),
    ))
}

fn queue_send_now_command(
    workspace_id: Option<String>,
    chat_id: Option<String>,
    message_id: String,
    time_zone: String,
) -> Option<Command> {
    let (Some(workspace_id), Some(chat_id)) = (workspace_id, chat_id) else {
        return None;
    };
    Some(Command::new(
        "message.queue_send_now",
        serde_json::json!({
            "patient_workspace_id": workspace_id,
            "chat_id": chat_id,
            "message_id": message_id,
            "physician_time_zone": time_zone,
        }),
    ))
}

/// Whether newer clinical records arrived after Coda published an answer.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum AnswerFreshness {
    Current,
    NewerRecordsArrived,
}

/// One line of the open conversation. A turn Coda couldn't finish carries the
/// work and run its Try again resumes. A running turn carries Stop, and a
/// stopped turn carries Resume.
#[derive(Debug, Clone, PartialEq, Eq)]
struct DemoLineSource {
    record_id: String,
    title: String,
}

#[derive(Debug, Clone, PartialEq)]
struct ConversationLine {
    text: String,
    body_pieces: Vec<SynopsisPiece>,
    basis_pieces: Vec<SynopsisPiece>,
    freshness: AnswerFreshness,
    /// Who wrote it, when the server says: the physician or Coda. A turn that
    /// doesn't say shows without a speaker rather than as either.
    author: Option<Author>,
    resume: Option<WorkResumeTarget>,
    stop: Option<WorkStopTarget>,
    /// Stable exchange identifier on the turn's ending line, so `WorkTrace` keeps
    /// its open/closed state when `Working` finishes.
    work_item_id: String,
    /// Whether this turn is currently in progress.
    working: bool,
    /// Physician-readable steps of this turn.
    steps: ProjectedSteps,
    /// This resume is an intentional Stop. A failed turn's resume is Try again.
    held: bool,
    /// On one of Coda's published responses: its response id, which
    /// feedback names, and the physician's feedback on it as the snapshot
    /// read it.
    response: Option<(ResponseId, feedback::Read)>,
    /// Chart records read for this response, for interactive demo citation opening.
    sources: Vec<DemoLineSource>,
}

/// The line while Coda works on an answer.
const WRITING: &str = "Coda is writing this response.";
/// The line when Coda's recovery is used up or the model service refused the work.
const COULDNT_FINISH: &str = "Coda couldn’t finish this response. Your message is saved.";
/// The line after a partial answer cut off at a limit.
const PARTIAL: &str = "Partial response.";
/// The line when a limit cut off an answer before any of it was kept.
const LIMIT: &str = "Coda reached its limit for this response.";
/// Follows a cut-off line only where Try again is offered.
const TRY_AGAIN_PROMISE: &str = "Choose Try again to continue.";

fn extract_exchange_sources(steps: &physician_wire::ExchangeSteps) -> Vec<DemoLineSource> {
    let physician_wire::ExchangeSteps::Read(step_list) = steps else {
        return Vec::new();
    };
    let mut out = Vec::new();
    for step in step_list {
        if let physician_wire::Step::ReadRecords {
            state: physician_wire::StepState::Returned { result },
            ..
        } = step
        {
            for rec in result {
                if let physician_wire::RecordRead::Whole { version, about } = rec {
                    out.push(DemoLineSource {
                        record_id: version.candidate_id.clone(),
                        title: about
                            .title
                            .clone()
                            .unwrap_or_else(|| "Chart record".to_owned()),
                    });
                }
            }
        }
    }
    out
}

/// The lines a chat snapshot shows, in order: each exchange's messages, the
/// text a limit kept, then its answer or the line that says how it ended.
fn conversation_lines(snapshot: &ChatSnapshot) -> Vec<ConversationLine> {
    let line = |text: &str, author| ConversationLine {
        text: text.to_owned(),
        body_pieces: Vec::new(),
        basis_pieces: Vec::new(),
        freshness: AnswerFreshness::Current,
        author,
        resume: None,
        stop: None,
        work_item_id: String::new(),
        working: false,
        steps: ProjectedSteps::default(),
        held: false,
        response: None,
        sources: Vec::new(),
    };
    let mut lines = Vec::new();
    for exchange in &snapshot.transcript {
        let exchange_sources = extract_exchange_sources(&exchange.steps);
        let working = matches!(exchange.answer, AnswerState::Working { .. });
        let steps = project_steps(&exchange.steps, working);
        let work_item_id = exchange.work_item_id.0.clone();
        lines.extend(
            exchange
                .messages
                .iter()
                .map(|message| line(&message.text, Some(Author::Person))),
        );
        lines.extend(
            exchange
                .kept
                .iter()
                .map(|kept| line(kept, Some(Author::Assistant))),
        );
        match &exchange.answer {
            AnswerState::Working { stop, .. } => lines.push(ConversationLine {
                stop: Some(stop.clone()),
                work_item_id,
                working: true,
                steps,
                ..line(WRITING, None)
            }),
            AnswerState::Finished {
                response_id,
                text,
                feedback: saved,
                segments,
                basis,
                newer_records,
                ..
            } => {
                let read = match snapshot.feedback {
                    SnapshotFeedback::Loaded => feedback::Read::Loaded(saved.clone()),
                    SnapshotFeedback::Unavailable => feedback::Read::Unavailable,
                };
                let body_pieces = if segments.is_empty() {
                    Vec::new()
                } else {
                    message_pieces(segments, &snapshot.sources)
                };
                let basis_pieces = basis
                    .as_deref()
                    .filter(|basis| !basis.is_empty())
                    .map(|basis| message_pieces(basis, &snapshot.sources))
                    .unwrap_or_default();
                let freshness = if *newer_records {
                    AnswerFreshness::NewerRecordsArrived
                } else {
                    AnswerFreshness::Current
                };
                lines.push(ConversationLine {
                    body_pieces,
                    basis_pieces,
                    freshness,
                    work_item_id,
                    steps,
                    response: Some((response_id.clone(), read)),
                    sources: exchange_sources,
                    ..line(text, Some(Author::Assistant))
                });
            }
            AnswerState::Stopped { resume } => lines.push(ConversationLine {
                resume: resume.clone(),
                work_item_id,
                steps,
                held: true,
                ..line(shell::STOPPED, None)
            }),
            AnswerState::CutOff { try_again } => {
                let ending = if exchange.kept.is_empty() {
                    LIMIT
                } else {
                    PARTIAL
                };
                let text = match try_again {
                    Some(_) => format!("{ending} {TRY_AGAIN_PROMISE}"),
                    None => ending.to_owned(),
                };
                lines.push(ConversationLine {
                    resume: try_again.clone(),
                    work_item_id,
                    steps,
                    ..line(&text, None)
                });
            }
            AnswerState::CouldntFinish { try_again } => lines.push(ConversationLine {
                resume: try_again.clone(),
                work_item_id,
                steps,
                ..line(COULDNT_FINISH, None)
            }),
        }
    }
    lines
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
        format!(
            "resume-{}-{}",
            target.work_item_id.as_str(),
            target.expected_run_id
        ),
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

pub(crate) fn idempotency_key() -> String {
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
/// entry, so a reload reopens the same chat and keeps any selected review on
/// that patient.
#[cfg(target_arch = "wasm32")]
fn write_address(patient: Option<&str>, chat: Option<&str>) {
    if let Some(window) = web_sys::window()
        && let Ok(href) = window.location().href()
        && let Ok(url) = web_sys::Url::new(&href)
    {
        let same_patient =
            patient.is_some() && url.search_params().get("patient").as_deref() == patient;
        match patient {
            Some(patient) => url.search_params().set("patient", patient),
            None => url.search_params().delete("patient"),
        }
        match chat.filter(|_| patient.is_some()) {
            Some(chat) => url.search_params().set("chat", chat),
            None => url.search_params().delete("chat"),
        }
        if !same_patient {
            url.search_params().delete("review");
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
        AnswerFreshness, COULDNT_FINISH, ConversationLine, Place, UPDATE_ANSWER_PROMPT, WRITING,
        chat_resume_command, chat_stop_command, clinical_dialogs_blocked, conversation_lines,
        new_chat_id, update_answer_command,
    };
    use crate::feedback;
    use crate::shell::{STOPPED, SynopsisPiece};
    use physician_wire::{
        AnswerState, AttemptId, ChatSnapshot, ChatSnapshotResponse, Exchange, FeedbackRating,
        MessageKey, PatientWorkspaceResponse, PhysicianMessage, ResponseFeedback, ResponseId,
        SnapshotFeedback, WorkItemId, WorkResumeTarget, WorkStopTarget,
    };
    use ui::Author;

    #[test]
    fn new_conversation_ids_are_distinct_chat_ids() {
        let (first, second) = (new_chat_id(), new_chat_id());
        assert!(first.starts_with("chat-") && second.starts_with("chat-"));
        assert_ne!(first, second);
    }

    fn target() -> WorkResumeTarget {
        WorkResumeTarget {
            work_item_id: WorkItemId("work-1".into()),
            expected_run_id: "run-1".into(),
        }
    }

    /// One exchange asking "Hello", with `kept` text, answered as `answer`.
    fn snapshot(answer: AnswerState, kept: &[&str], feedback: SnapshotFeedback) -> ChatSnapshot {
        ChatSnapshot {
            transcript: vec![Exchange {
                work_item_id: WorkItemId("work-1".into()),
                messages: vec![PhysicianMessage {
                    id: MessageKey("key-1".into()),
                    text: "Hello".into(),
                    saved_at: physician_wire::SavedAt(1_794_600_000),
                }],
                steps: physician_wire::ExchangeSteps::Read(vec![]),
                kept: kept.iter().map(|text| (*text).to_owned()).collect(),
                answer,
            }],
            queued: Vec::new(),
            feedback,
            sources: std::collections::BTreeMap::new(),
        }
    }

    fn line(text: &str, author: Option<Author>) -> ConversationLine {
        ConversationLine {
            text: text.into(),
            body_pieces: Vec::new(),
            basis_pieces: Vec::new(),
            freshness: AnswerFreshness::Current,
            author,
            resume: None,
            stop: None,
            work_item_id: String::new(),
            working: false,
            steps: Default::default(),
            held: false,
            response: None,
            sources: Vec::new(),
        }
    }

    fn ending_line(text: &str, author: Option<Author>) -> ConversationLine {
        ConversationLine {
            work_item_id: "work-1".into(),
            ..line(text, author)
        }
    }

    /// Each ending draws the line the page drew before the typed transcript:
    /// Stop while working, Resume when stopped, Try again when cut off or
    /// unfinished. Text a limit kept comes first in every state, and the
    /// cut-off line says whether any was kept and promises Try again only
    /// where it is offered.
    #[test]
    fn each_answer_state_draws_its_line_and_action() {
        let ask = line("Hello", Some(Author::Person));
        let kept = line("The saved part.", Some(Author::Assistant));
        let finished = AnswerState::Finished {
            response_id: ResponseId("r-1".into()),
            attempt_id: AttemptId("attempt-1".into()),
            finished_at: physician_wire::SavedAt(1_794_600_060),
            text: "The rest.".into(),
            feedback: None,
            segments: Vec::new(),
            basis: None,
            newer_records: false,
        };
        let cases: [(AnswerState, &[&str], Vec<ConversationLine>); 9] = [
            (
                AnswerState::Working {
                    stop: WorkStopTarget {
                        work_item_id: WorkItemId("work-1".into()),
                    },
                    writing: false,
                },
                &["The saved part."],
                vec![
                    kept.clone(),
                    ConversationLine {
                        stop: Some(WorkStopTarget {
                            work_item_id: WorkItemId("work-1".into()),
                        }),
                        working: true,
                        ..ending_line(WRITING, None)
                    },
                ],
            ),
            (
                finished,
                &["The saved part."],
                vec![
                    kept.clone(),
                    ConversationLine {
                        response: Some((ResponseId("r-1".into()), feedback::Read::Loaded(None))),
                        ..ending_line("The rest.", Some(Author::Assistant))
                    },
                ],
            ),
            (
                AnswerState::Stopped {
                    resume: Some(target()),
                },
                &[],
                vec![ConversationLine {
                    resume: Some(target()),
                    held: true,
                    ..ending_line(STOPPED, None)
                }],
            ),
            (
                AnswerState::CutOff {
                    try_again: Some(target()),
                },
                &["The saved part."],
                vec![
                    kept.clone(),
                    ConversationLine {
                        resume: Some(target()),
                        ..ending_line("Partial response. Choose Try again to continue.", None)
                    },
                ],
            ),
            (
                AnswerState::CutOff {
                    try_again: Some(target()),
                },
                &[],
                vec![ConversationLine {
                    resume: Some(target()),
                    ..ending_line(
                        "Coda reached its limit for this response. Choose Try again to continue.",
                        None,
                    )
                }],
            ),
            (
                AnswerState::CutOff { try_again: None },
                &["The saved part."],
                vec![kept.clone(), ending_line("Partial response.", None)],
            ),
            (
                AnswerState::CutOff { try_again: None },
                &[],
                vec![ending_line(
                    "Coda reached its limit for this response.",
                    None,
                )],
            ),
            (
                AnswerState::CouldntFinish {
                    try_again: Some(target()),
                },
                &[],
                vec![ConversationLine {
                    resume: Some(target()),
                    ..ending_line(COULDNT_FINISH, None)
                }],
            ),
            (
                AnswerState::CouldntFinish { try_again: None },
                &[],
                vec![ending_line(COULDNT_FINISH, None)],
            ),
        ];
        for (answer, kept, ending) in cases {
            let drawn =
                conversation_lines(&snapshot(answer.clone(), kept, SnapshotFeedback::Loaded));
            let expected: Vec<_> = std::iter::once(ask.clone()).chain(ending).collect();
            assert_eq!(drawn, expected, "{answer:?} with {kept:?}");
        }
    }

    #[test]
    fn reopened_answer_carries_citations_basis_and_newer_records_marker() {
        let page: ChatSnapshotResponse = serde_json::from_value(serde_json::json!({
            "snapshot": {
                "feedback": "loaded",
                "sources": {
                    "s1": {
                        "source_key": "s1",
                        "number": 1,
                        "source_label": "Practice EHR",
                        "title": "BMP panel",
                        "quote": "Potassium 4.4 mmol/L",
                        "body": "Basic metabolic panel: Potassium 4.4 mmol/L."
                    }
                },
                "transcript": [
                    {
                        "work_item_id": "w1",
                        "messages": [],
                        "steps": { "state": "read", "steps": [] },
                        "kept": [],
                        "answer": {
                            "state": "finished",
                            "response_id": "a1:reply",
                            "attempt_id": "a1",
                            "finished_at": 1794600060,
                            "text": "Potassium is 4.4 mmol/L.",
                            "feedback": null,
                            "segments": [
                                { "kind": "text", "text": "Potassium is 4.4 mmol/L" },
                                {
                                    "kind": "citation",
                                    "source_key": "s1",
                                    "number": 1,
                                    "label": "BMP panel",
                                    "quote": "Potassium 4.4 mmol/L"
                                },
                                { "kind": "text", "text": "." }
                            ],
                            "basis": [
                                { "kind": "text", "text": "Rests on the outpatient BMP" },
                                {
                                    "kind": "citation",
                                    "source_key": "s1",
                                    "number": 1,
                                    "label": "BMP panel",
                                    "quote": "Potassium 4.4 mmol/L"
                                },
                                { "kind": "text", "text": "; no inpatient labs are on file." }
                            ],
                            "newer_records": true,
                            "finished_at": 1_794_600_060
                        }
                    }
                ]
            }
        }))
        .unwrap();
        let lines = conversation_lines(&page.snapshot);
        assert_eq!(lines.len(), 1);
        assert_eq!(lines[0].text, "Potassium is 4.4 mmol/L.");
        assert_eq!(lines[0].freshness, AnswerFreshness::NewerRecordsArrived);
        assert!(matches!(
            lines[0].body_pieces.as_slice(),
            [
                SynopsisPiece::Text(_),
                SynopsisPiece::Chips(_),
                SynopsisPiece::Text(_)
            ]
        ));
        assert!(matches!(
            lines[0].basis_pieces.as_slice(),
            [
                SynopsisPiece::Text(_),
                SynopsisPiece::Chips(_),
                SynopsisPiece::Text(_)
            ]
        ));
    }

    #[test]
    fn a_finished_answer_carries_its_response_id_and_feedback_as_read() {
        let finished = AnswerState::Finished {
            response_id: ResponseId("r-1".into()),
            attempt_id: AttemptId("attempt-1".into()),
            finished_at: physician_wire::SavedAt(1_794_600_060),
            text: "Her heart rate allows it.".into(),
            feedback: Some(ResponseFeedback {
                revision: 2,
                rating: FeedbackRating::Up,
                details: None,
            }),
            segments: Vec::new(),
            basis: None,
            newer_records: false,
        };
        let loaded = conversation_lines(&snapshot(finished.clone(), &[], SnapshotFeedback::Loaded));
        assert_eq!(loaded[1].author, Some(Author::Assistant));
        assert_eq!(loaded[1].text, "Her heart rate allows it.");
        assert_eq!(
            loaded[1].response,
            Some((
                ResponseId("r-1".into()),
                feedback::Read::Loaded(Some(ResponseFeedback {
                    revision: 2,
                    rating: FeedbackRating::Up,
                    details: None,
                }))
            ))
        );
        let unavailable =
            conversation_lines(&snapshot(finished, &[], SnapshotFeedback::Unavailable));
        assert_eq!(
            unavailable[1].response,
            Some((ResponseId("r-1".into()), feedback::Read::Unavailable))
        );
        assert_eq!(loaded[0].response, None);
    }

    #[test]
    fn each_stop_press_is_its_own_command_for_the_named_work() {
        let target = WorkStopTarget {
            work_item_id: WorkItemId("work-1".into()),
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

    #[test]
    fn clinical_dialogs_close_when_workspace_leaves_panel_or_disappears() {
        let on_panel: PatientWorkspaceResponse = serde_json::from_value(serde_json::json!({
            "patient_workspace_id": "pw-1",
            "display_name": "Tanvir Chowdhury",
            "membership": "allowed",
        }))
        .unwrap();
        let off_panel: PatientWorkspaceResponse = serde_json::from_value(serde_json::json!({
            "patient_workspace_id": "pw-1",
            "display_name": "Tanvir Chowdhury",
            "membership": "clinic_demographic",
        }))
        .unwrap();

        assert!(!clinical_dialogs_blocked(
            Place::Workspace,
            Some("pw-1"),
            None,
            None,
        ));
        assert!(!clinical_dialogs_blocked(
            Place::Workspace,
            Some("pw-1"),
            None,
            Some(&on_panel),
        ));
        assert!(clinical_dialogs_blocked(
            Place::Workspace,
            Some("pw-1"),
            None,
            Some(&off_panel),
        ));
        assert!(clinical_dialogs_blocked(
            Place::Workspace,
            Some("pw-1"),
            Some("pw-1"),
            Some(&on_panel),
        ));
        assert!(!clinical_dialogs_blocked(
            Place::Today,
            Some("pw-1"),
            None,
            Some(&off_panel),
        ));
    }

    #[test]
    fn update_answer_command_uses_update_prompt_without_changing_draft_inputs() {
        let cmd = update_answer_command(
            Some("pw-1".into()),
            Some("chat-1".into()),
            Some("America/Los_Angeles".into()),
        )
        .unwrap();
        assert_eq!(cmd.0["type"], "message.submit");
        assert_eq!(cmd.0["payload"]["text"], UPDATE_ANSWER_PROMPT);
        assert!(
            update_answer_command(
                Some("pw-1".into()),
                None,
                Some("America/Los_Angeles".into())
            )
            .is_err()
        );
    }

    #[test]
    fn queue_and_delivery_commands_carry_expected_payloads() {
        let now_cmd = super::message_command_with_delivery(
            Some("pw-1".into()),
            Some("chat-1".into()),
            Some("America/Chicago".into()),
            "Check sodium".into(),
            physician_wire::SubmitDelivery::SendNow,
        )
        .unwrap();
        assert_eq!(now_cmd.0["type"], "message.submit");
        assert!(now_cmd.0["payload"].get("delivery").is_none());

        let next_cmd = super::message_command_with_delivery(
            Some("pw-1".into()),
            Some("chat-1".into()),
            Some("America/Chicago".into()),
            "Then check potassium".into(),
            physician_wire::SubmitDelivery::SendNext,
        )
        .unwrap();
        assert_eq!(next_cmd.0["payload"]["delivery"], "send_next");
        assert_eq!(
            next_cmd.0["payload"]["physician_time_zone"],
            "America/Chicago"
        );

        let edit = super::queue_edit_command(
            Some("pw-1".into()),
            Some("chat-1".into()),
            "q-1".into(),
            "Edited".into(),
        )
        .unwrap();
        assert_eq!(edit.0["type"], "message.queue_edit");
        assert_eq!(edit.0["payload"]["message_id"], "q-1");
        assert_eq!(edit.0["payload"]["text"], "Edited");

        let withdraw =
            super::queue_withdraw_command(Some("pw-1".into()), Some("chat-1".into()), "q-1".into())
                .unwrap();
        assert_eq!(withdraw.0["type"], "message.withdraw");
        assert_eq!(withdraw.0["payload"]["message_id"], "q-1");

        let promote = super::queue_send_now_command(
            Some("pw-1".into()),
            Some("chat-1".into()),
            "q-1".into(),
            "America/New_York".into(),
        )
        .unwrap();
        assert_eq!(promote.0["type"], "message.queue_send_now");
        assert_eq!(
            promote.0["payload"]["physician_time_zone"],
            "America/New_York"
        );

        let returned = super::returned_queue_texts(&serde_json::json!({
            "outcome": "accepted",
            "returned_messages": ["First queued", "Second queued"]
        }));
        assert_eq!(returned, vec!["First queued", "Second queued"]);

        assert_eq!(
            super::command_target(&promote),
            Some(super::DraftTarget {
                workspace: "pw-1".into(),
                chat: "chat-1".into(),
            })
        );
        assert_eq!(
            super::returned_queue_target(&serde_json::json!({
                "outcome": "accepted",
                "patient_workspace_id": "pw-a",
                "chat_id": "chat-a",
                "returned_messages": ["Patient A draft"]
            })),
            Some(super::DraftTarget {
                workspace: "pw-a".into(),
                chat: "chat-a".into(),
            })
        );
        assert_eq!(
            super::returned_queue_target(&serde_json::json!({
                "outcome": "accepted",
                "returned_messages": ["Missing target"]
            })),
            None
        );
    }
}
