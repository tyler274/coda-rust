//! What the popup says about the connection. The worker owns the connection; this turns its
//! status reply into one state, said once, and the one action that fits, and decides nothing on
//! its own.

use serde::Deserialize;
use ui::{IconName, Task, TaskState, Tone};

/// An instant, in milliseconds since the epoch, as `Date.now` counts it.
#[derive(Debug, Clone, Copy, Deserialize, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct Millis(pub f64);

/// The worker's reply to `connect`, `stop` or `status`, read as far as the popup says anything
/// about it; Copy diagnostics carries the rest as sent. A reply missing what the worker always
/// sends doesn't parse, rather than reading as a state.
#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct Reply {
    ok: bool,
    reason: Option<Refusal>,
    state: Recording,
    run: Option<Run>,
    practice: Option<String>,
    readiness: Option<Readiness>,
}

/// What the worker last recorded, of which the popup reads only the code.
#[derive(Debug, Clone, Copy, Deserialize, PartialEq)]
struct Recording {
    state: Recorded,
}

/// The worker's recorded code, grouped by what the popup says about it. On its own it can't say
/// whether Coda is reading: that also takes the run, which [`Reply::standing`] weighs.
#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq)]
enum Recorded {
    /// Nothing is under way: never connected, checked but not started, or Disconnected.
    #[serde(rename = "not_prepared", alias = "prepared", alias = "stopped")]
    Idle,
    /// The last read went through.
    #[serde(rename = "connected")]
    Connected,
    /// The bridge couldn't be reached; the run goes on and tries again.
    #[serde(rename = "bridge_offline")]
    BridgeOffline,
    /// Hint's sign-in has ended or lapsed, so the physician signs back in.
    #[serde(
        rename = "silent_auth_unavailable",
        alias = "interaction_required",
        alias = "chart_unauthorized"
    )]
    SignedOut,
    /// Anything else that stopped the run.
    #[serde(untagged)]
    Stopped(Stop),
}

/// What stopped a run, other than Hint's sign-in ending.
#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq)]
pub enum Stop {
    /// Another Hint account or practice is signed in.
    #[serde(rename = "auth_identity_changed")]
    IdentityChanged,
    /// Hint refused this account the chart.
    #[serde(rename = "access_denied")]
    AccessDenied,
    /// Renewing failed for a reason that says nothing about the sign-in.
    #[serde(rename = "auth_failed")]
    RenewalFailed,
    /// Hint asked Coda to slow down.
    #[serde(rename = "rate_limited")]
    RateLimited,
    /// A code the popup doesn't single out; diagnostics carry the code itself.
    #[serde(other)]
    Unexpected,
}

/// Why the worker refused an action, among the codes `connect` and `stop` can answer with.
#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq)]
pub enum Refusal {
    /// More or fewer than one Hint tab is open.
    #[serde(rename = "open_exactly_one_sandbox_tab")]
    NotOneTab,
    /// Hint isn't signed in.
    #[serde(rename = "sign_in_required")]
    SignInRequired,
    /// Hint shows another practice.
    #[serde(rename = "wrong_sandbox")]
    WrongSandbox,
    /// A run is already under way.
    #[serde(rename = "stop_before_prepare")]
    AlreadyConnected,
    /// A read kept from the last run couldn't be handed to Coda.
    #[serde(rename = "pending_delivery")]
    PendingDelivery,
    /// A code the popup doesn't single out; diagnostics carry the code itself.
    #[serde(other)]
    Unexpected,
}

/// A bounded reading run.
#[derive(Debug, Clone, Copy, Deserialize, PartialEq)]
struct Run {
    until: Millis,
    stopped: bool,
}

/// How far the physician's Hint is from ready, each step following the one before.
#[derive(Debug, Clone, Copy, Deserialize, PartialEq, PartialOrd)]
pub enum Readiness {
    /// No Hint tab is open.
    #[serde(rename = "hint_closed")]
    HintClosed,
    /// Hint is open, and not signed in.
    #[serde(rename = "signed_out")]
    SignedOut,
    /// Signed in, on a practice Coda doesn't read.
    #[serde(rename = "other_practice")]
    OtherPractice,
    /// Signed in, on the practice Coda reads.
    #[serde(rename = "ready")]
    Ready,
}

/// Where the connection stands at one moment: the worker's code weighed against its run.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Standing {
    /// Not reading, and how far Hint is from ready when the worker could check it.
    NotConnected(Option<Readiness>),
    /// A run is live and reads are going through.
    Reading,
    /// A run is live and waiting on the bridge.
    WaitingForBridge,
    /// The run ran out its time.
    Paused {
        /// When it ended.
        until: Millis,
    },
    /// Hint's sign-in ended the run, and how far Hint is from ready when the worker could check it.
    SignedOut(Option<Readiness>),
    /// Something else ended the run.
    Stopped(Stop),
}

impl Reply {
    /// Why the worker refused the action this answers, if it did. A refusal without a code is
    /// still a refusal.
    pub fn refused(&self) -> Option<Refusal> {
        if self.ok {
            return None;
        }
        Some(self.reason.unwrap_or(Refusal::Unexpected))
    }

    /// Where the connection stands at `now`.
    pub fn standing(&self, now: Millis) -> Standing {
        let phase = match self.run {
            Some(run) if run.stopped => Phase::Over,
            Some(run) if now < run.until => Phase::Live,
            Some(run) => Phase::RanOut(run.until),
            None => Phase::Over,
        };
        match (self.state.state, phase) {
            (Recorded::Connected, Phase::Live) => Standing::Reading,
            (Recorded::BridgeOffline, Phase::Live) => Standing::WaitingForBridge,
            (Recorded::Connected | Recorded::BridgeOffline, Phase::RanOut(until)) => {
                Standing::Paused { until }
            }
            (Recorded::Connected | Recorded::BridgeOffline, Phase::Over) | (Recorded::Idle, _) => {
                Standing::NotConnected(self.readiness)
            }
            (Recorded::SignedOut, _) => Standing::SignedOut(self.readiness),
            (Recorded::Stopped(stop), _) => Standing::Stopped(stop),
        }
    }
}

/// Where the run stands at one moment. One that was stopped, or never started, is over: what
/// stopped it is in the worker's code.
enum Phase {
    Live,
    RanOut(Millis),
    Over,
}

/// What the popup asks of the worker, each by the name `worker.js` lists it under.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Request {
    /// Where things stand.
    Status,
    /// Start reading from the signed-in Hint tab.
    Connect,
    /// Stop reading.
    Stop,
}

impl Request {
    /// Its name in the worker.
    pub fn name(self) -> &'static str {
        match self {
            Self::Status => "status",
            Self::Connect => "connect",
            Self::Stop => "stop",
        }
    }
}

/// What the popup's one action does.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    /// Bring the physician's Hint tab forward, or open Hint.
    OpenHint,
    /// Start reading from the signed-in Hint tab.
    Connect,
}

/// The popup's one action and its words.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Offer {
    /// What it does.
    pub action: Action,
    /// Its words.
    pub label: &'static str,
}

/// The popup's face for one reply.
#[derive(Debug, Clone, PartialEq)]
pub struct Face {
    /// The state's mark.
    pub icon: IconName,
    /// The state, in one line.
    pub title: String,
    /// What to know about it.
    pub detail: String,
    /// The steps to ready, when the worker reported them.
    pub steps: Vec<Task>,
    /// The one action, when the physician has one.
    pub offer: Option<Offer>,
    /// What the worker is doing.
    pub activity: Activity,
}

/// What the worker is doing, as the popup's mark and menu show it.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum Activity {
    /// No run is under way.
    #[default]
    Idle,
    /// A run is under way, waiting on the bridge. Disconnect would end it.
    Waiting,
    /// A run is under way and reading. Disconnect would end it.
    Reading,
}

/// A notice above the state.
#[derive(Debug, Clone, PartialEq)]
pub struct Note {
    /// Its tone.
    pub tone: Tone,
    /// Its title.
    pub title: &'static str,
    /// Its detail.
    pub detail: &'static str,
}

const OPEN_HINT: Offer = Offer {
    action: Action::OpenHint,
    label: "Open Hint",
};
const GO_TO_HINT: Offer = Offer {
    action: Action::OpenHint,
    label: "Go to Hint",
};
const CONNECT: Offer = Offer {
    action: Action::Connect,
    label: "Connect Hint",
};
const RECONNECT: Offer = Offer {
    action: Action::Connect,
    label: "Reconnect",
};
const TRY_AGAIN: Offer = Offer {
    action: Action::Connect,
    label: "Try again",
};

fn state(icon: IconName, title: &str, detail: &str, offer: Option<Offer>) -> Face {
    Face {
        icon,
        title: title.to_owned(),
        detail: detail.to_owned(),
        steps: Vec::new(),
        offer,
        activity: Activity::Idle,
    }
}

/// The steps to ready, each done or still to do.
fn steps(ready: Readiness) -> Vec<Task> {
    [
        ("Open Hint", ready > Readiness::HintClosed),
        ("Sign in", ready > Readiness::SignedOut),
        ("Choose your practice", ready > Readiness::OtherPractice),
    ]
    .into_iter()
    .map(|(label, done)| Task {
        label: label.to_owned(),
        state: if done {
            TaskState::Done
        } else {
            TaskState::Pending
        },
    })
    .collect()
}

/// Not reading: the next step to ready when the worker reported it, or else sign in and connect.
fn not_connected(readiness: Option<Readiness>, practice: Option<&str>) -> Face {
    let Some(ready) = readiness else {
        return state(
            IconName::Link,
            "Connect Hint",
            "Sign into Hint in one tab, then connect. After that you can close Hint.",
            Some(CONNECT),
        );
    };
    let face = match ready {
        Readiness::HintClosed => state(
            IconName::Link,
            "Open Hint to connect",
            "Coda reads Hint in the background, under your own sign-in.",
            Some(OPEN_HINT),
        ),
        Readiness::SignedOut => state(
            IconName::User,
            "Sign into Hint",
            "Use your usual sign-in. Coda never sees your password.",
            Some(GO_TO_HINT),
        ),
        Readiness::OtherPractice => state(
            IconName::Users,
            "Choose your practice",
            "Switch Hint to the practice Coda reads.",
            Some(GO_TO_HINT),
        ),
        Readiness::Ready => {
            let practice = practice.unwrap_or("Your Hint practice");
            state(
                IconName::Link,
                "Ready to connect",
                &format!("{practice}. After connecting you can close Hint."),
                Some(CONNECT),
            )
        }
    };
    Face {
        steps: steps(ready),
        ..face
    }
}

/// Hint's sign-in ended the run: reconnect if the physician is back, or else sign back in.
fn signed_out(readiness: Option<Readiness>) -> Face {
    let face = match readiness {
        Some(Readiness::OtherPractice | Readiness::Ready) => state(
            IconName::Link,
            "Hint is signed in again",
            "Reconnect to keep reading. What Coda already read is kept.",
            Some(RECONNECT),
        ),
        Some(Readiness::HintClosed | Readiness::SignedOut) | None => state(
            IconName::Alert,
            "Sign back into Hint",
            "Coda lost its access to Hint, usually because Hint signed out. What Coda already read is kept.",
            Some(if readiness.is_some() {
                GO_TO_HINT
            } else {
                RECONNECT
            }),
        ),
    };
    Face {
        steps: readiness.map(steps).unwrap_or_default(),
        ..face
    }
}

/// The face for `reply` at `now`; `time` words an instant.
pub fn face(reply: &Reply, now: Millis, time: &dyn Fn(Millis) -> String) -> Face {
    match reply.standing(now) {
        Standing::NotConnected(readiness) => not_connected(readiness, reply.practice.as_deref()),
        Standing::Reading => {
            let detail = match &reply.practice {
                Some(practice) => format!("{practice}. You can close Hint."),
                None => "You can close Hint and keep working.".to_owned(),
            };
            Face {
                activity: Activity::Reading,
                ..state(IconName::Check, "Coda is reading Hint", &detail, None)
            }
        }
        Standing::WaitingForBridge => Face {
            activity: Activity::Waiting,
            ..state(
                IconName::Clock,
                "Waiting for Coda",
                "Coda's connection isn't reachable. Reading picks up on its own when it is.",
                None,
            )
        },
        Standing::Paused { until } => state(
            IconName::Clock,
            "Reading paused",
            &format!(
                "Coda read Hint until {}. Connect to read again.",
                time(until)
            ),
            Some(CONNECT),
        ),
        Standing::SignedOut(readiness) => signed_out(readiness),
        Standing::Stopped(Stop::IdentityChanged) => state(
            IconName::Alert,
            "A different Hint account is signed in",
            "Hint is signed into another account or practice than the one you connected.",
            Some(Offer {
                action: Action::Connect,
                label: "Connect this account",
            }),
        ),
        Standing::Stopped(Stop::AccessDenied) => state(
            IconName::Alert,
            "Hint refused access",
            "Hint didn't allow this account to read the chart.",
            Some(TRY_AGAIN),
        ),
        // Any other refusal while renewing, such as Hint's own server failing, says nothing
        // about whether the physician is still signed in.
        Standing::Stopped(Stop::RenewalFailed) => state(
            IconName::Alert,
            "Coda couldn't renew access to Hint",
            "Hint didn't answer the way Coda expected. What Coda already read is kept.",
            Some(TRY_AGAIN),
        ),
        Standing::Stopped(Stop::RateLimited) => state(
            IconName::Clock,
            "Hint asked Coda to slow down",
            "Try again in a few minutes.",
            Some(TRY_AGAIN),
        ),
        Standing::Stopped(Stop::Unexpected) => state(
            IconName::Alert,
            "Coda stopped reading Hint",
            "Something unexpected happened. Copy diagnostics from the menu to share it.",
            Some(TRY_AGAIN),
        ),
    }
}

/// The notice for an action the worker refused.
pub fn refusal(reason: Refusal) -> Note {
    let (tone, title, detail) = match reason {
        Refusal::NotOneTab => (
            Tone::Warning,
            "Open Hint in one tab",
            "Close the extra Hint tabs, then connect.",
        ),
        Refusal::SignInRequired => (
            Tone::Warning,
            "Sign into Hint first",
            "Sign into Hint in its tab, then connect.",
        ),
        Refusal::WrongSandbox => (
            Tone::Warning,
            "Choose the sandbox practice",
            "Switch Hint to Coda Health Sandbox Practice, then connect.",
        ),
        Refusal::AlreadyConnected => (
            Tone::Neutral,
            "Already connected",
            "Disconnect from the menu to start over.",
        ),
        // Connect delivers a read kept from the last run before starting; this is that failing.
        Refusal::PendingDelivery => (
            Tone::Warning,
            "A read is waiting to reach Coda",
            "Coda kept the last chart section it read but couldn't hand it over. Check Coda's connection is running, then connect again.",
        ),
        Refusal::Unexpected => (
            Tone::Error,
            "Couldn't connect",
            "Hint didn't respond as expected. Copy diagnostics from the menu to share it.",
        ),
    };
    Note {
        tone,
        title,
        detail,
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::*;

    const NOW: Millis = Millis(1_000_000.0);

    fn reply(state: Recorded, run: Option<Run>) -> Reply {
        Reply {
            ok: true,
            reason: None,
            state: Recording { state },
            run,
            practice: None,
            readiness: None,
        }
    }

    fn run(until: f64, stopped: bool) -> Option<Run> {
        Some(Run {
            until: Millis(until),
            stopped,
        })
    }

    fn running() -> Option<Run> {
        run(NOW.0 + 1.0, false)
    }

    fn word(at: Millis) -> String {
        format!("@{}", at.0)
    }

    #[test]
    fn a_live_run_reads_quietly_with_disconnect_in_the_menu() {
        let face = face(&reply(Recorded::Connected, running()), NOW, &word);
        assert_eq!(face.title, "Coda is reading Hint");
        assert_eq!(face.activity, Activity::Reading);
        assert_eq!(face.offer, None);
    }

    #[test]
    fn a_connected_state_without_a_live_run_never_claims_reading() {
        let stopped = run(NOW.0 + 1.0, true);
        let face_stopped = face(&reply(Recorded::Connected, stopped), NOW, &word);
        assert_eq!(face_stopped.activity, Activity::Idle);
        assert_eq!(face_stopped.offer, Some(CONNECT));
        let ended = run(NOW.0 - 1.0, false);
        let paused = face(&reply(Recorded::Connected, ended), NOW, &word);
        assert_eq!(paused.title, "Reading paused");
        assert_eq!(
            paused.detail,
            "Coda read Hint until @999999. Connect to read again."
        );
    }

    #[test]
    fn waiting_on_the_bridge_keeps_disconnect_and_never_claims_reading() {
        let face = face(&reply(Recorded::BridgeOffline, running()), NOW, &word);
        assert_eq!(face.title, "Waiting for Coda");
        assert_eq!(face.activity, Activity::Waiting);
    }

    #[test]
    fn without_readiness_the_popup_claims_no_steps() {
        let face = face(&reply(Recorded::Idle, None), NOW, &word);
        assert_eq!(face.title, "Connect Hint");
        assert!(face.steps.is_empty());
    }

    #[test]
    fn readiness_names_the_next_step_and_its_action() {
        let mut waiting = reply(Recorded::Idle, None);
        waiting.readiness = Some(Readiness::SignedOut);
        let face_waiting = face(&waiting, NOW, &word);
        assert_eq!(face_waiting.title, "Sign into Hint");
        assert_eq!(face_waiting.offer, Some(GO_TO_HINT));
        let states: Vec<_> = face_waiting.steps.iter().map(|task| task.state).collect();
        assert_eq!(
            states,
            [TaskState::Done, TaskState::Pending, TaskState::Pending]
        );
        waiting.readiness = Some(Readiness::Ready);
        assert_eq!(face(&waiting, NOW, &word).offer, Some(CONNECT));
    }

    #[test]
    fn a_hint_sign_out_asks_for_hint_and_keeps_what_was_read() {
        let face = face(&reply(Recorded::SignedOut, running()), NOW, &word);
        assert_eq!(face.title, "Sign back into Hint");
        assert_eq!(face.offer, Some(RECONNECT));
    }

    #[test]
    fn signing_back_into_hint_offers_reconnect() {
        let mut signed_out = reply(Recorded::SignedOut, running());
        signed_out.readiness = Some(Readiness::SignedOut);
        assert_eq!(face(&signed_out, NOW, &word).title, "Sign back into Hint");
        signed_out.readiness = Some(Readiness::Ready);
        let back = face(&signed_out, NOW, &word);
        assert_eq!(back.title, "Hint is signed in again");
        assert_eq!(back.offer, Some(RECONNECT));
    }

    #[test]
    fn a_failed_renewal_never_claims_a_sign_out() {
        let stopped = reply(Recorded::Stopped(Stop::RenewalFailed), None);
        let face = face(&stopped, NOW, &word);
        assert_eq!(face.title, "Coda couldn't renew access to Hint");
        assert_eq!(face.offer, Some(TRY_AGAIN));
    }

    #[test]
    fn a_read_that_could_not_be_handed_over_never_sounds_in_progress() {
        let note = refusal(Refusal::PendingDelivery);
        assert_eq!(note.title, "A read is waiting to reach Coda");
        assert_eq!(note.tone, Tone::Warning);
    }

    #[test]
    fn a_code_the_popup_does_not_single_out_is_an_error_not_a_connection() {
        let state: Recorded = serde_json::from_str(r#""read_schema_changed""#).unwrap();
        assert_eq!(state, Recorded::Stopped(Stop::Unexpected));
        let face = face(&reply(state, None), NOW, &word);
        assert_eq!((face.icon, face.offer), (IconName::Alert, Some(TRY_AGAIN)));
        let reason: Refusal = serde_json::from_str(r#""connector_failed""#).unwrap();
        assert_eq!(reason, Refusal::Unexpected);
        assert_eq!(refusal(reason).title, "Couldn't connect");
    }

    #[test]
    fn each_code_reads_as_its_group() {
        for (code, state) in [
            ("not_prepared", Recorded::Idle),
            ("prepared", Recorded::Idle),
            ("stopped", Recorded::Idle),
            ("connected", Recorded::Connected),
            ("bridge_offline", Recorded::BridgeOffline),
            ("chart_unauthorized", Recorded::SignedOut),
            ("rate_limited", Recorded::Stopped(Stop::RateLimited)),
        ] {
            let read: Recorded = serde_json::from_value(serde_json::json!(code)).unwrap();
            assert_eq!(read, state, "{code}");
        }
    }

    #[test]
    fn a_refusal_without_a_code_is_still_a_refusal() {
        let refused = Reply {
            ok: false,
            ..reply(Recorded::Idle, None)
        };
        assert_eq!(refused.refused(), Some(Refusal::Unexpected));
        assert_eq!(reply(Recorded::Idle, None).refused(), None);
    }

    #[test]
    fn a_reply_missing_what_the_worker_always_sends_does_not_parse() {
        assert!(serde_json::from_str::<Reply>("{}").is_err());
    }

    #[test]
    fn the_worker_reply_parses_as_it_sends_it() {
        let parsed: Reply = serde_json::from_str(
            r#"{"ok":true,"version":"0.3.2","state":{"at":1793456970780,"portal_tab_count":0,
            "state":"connected","token_exp":1793457870,"token_iat":1793456970},
            "run":{"id":"3d9a","started":1793456969948,"until":1793459669948,"stopped":false},
            "pending_delivery":false,"practice":"Coda Health Sandbox Practice",
            "readiness":"ready"}"#,
        )
        .unwrap();
        assert_eq!(parsed.standing(Millis(1793456970780.0)), Standing::Reading);
        assert_eq!(
            parsed.practice.as_deref(),
            Some("Coda Health Sandbox Practice")
        );
        assert_eq!(parsed.readiness, Some(Readiness::Ready));
    }

    #[test]
    fn the_worker_lists_every_request() {
        let worker = include_str!("../../../extension/chrome/worker.js");
        let listed = worker
            .split_once("const actions = {")
            .and_then(|(_, rest)| rest.split_once('}'))
            .expect("worker.js lists its actions")
            .0;
        for request in [Request::Status, Request::Connect, Request::Stop] {
            assert!(
                listed.split(',').any(|name| name.trim() == request.name()),
                "worker.js has no {request:?}"
            );
        }
    }

    /// The codes `worker.js` and `bootstrap.js` can send: what they fault with, record, or report
    /// of Hint. A category they only read from Hint's answers isn't one.
    fn sent_codes() -> BTreeSet<&'static str> {
        let mut codes = BTreeSet::new();
        for source in [
            include_str!("../../../extension/chrome/worker.js"),
            include_str!("../../../extension/chrome/bootstrap.js"),
        ] {
            for sends in [
                "fault(",
                "fail(",
                "setState(",
                "code = ",
                "{state: ",
                "readiness: ",
            ] {
                for (at, _) in source.match_indices(sends) {
                    let sent = source[at..].split([';', ',']).next().unwrap_or_default();
                    codes.extend(sent.split('\'').skip(1).step_by(2));
                }
            }
        }
        codes
    }

    #[test]
    fn every_code_the_popup_names_is_one_the_worker_sends() {
        let sent = sent_codes();
        for code in [
            "chart_unauthorized",
            "read_failed",
            "connector_failed",
            "not_prepared",
        ] {
            assert!(sent.contains(code), "the scan should find {code}: {sent:?}");
        }
        let source = include_str!("face.rs");
        let mut named = 0;
        for names in ["rename = \"", "alias = \""] {
            for (at, _) in source.match_indices(names) {
                let code = source[at + names.len()..]
                    .split('"')
                    .next()
                    .unwrap_or_default();
                assert!(sent.contains(code), "the worker never sends {code}");
                named += 1;
            }
        }
        assert!(
            named > 15,
            "the scan should find the popup's codes: {named}"
        );
    }
}
