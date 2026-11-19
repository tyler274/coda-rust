//! What the popup says about the connection. The connector worker owns the connection; this turns
//! its status reply into one state, said once, and the one action that fits, and decides nothing
//! on its own.

use serde::{Deserialize, Serialize};
use serde_json::Value;
use ui::{IconName, Task, TaskState, Tone};

/// The worker's reply to `pair`, `unpair`, `connect`, `stop` or `status`, read as far as the popup
/// says anything about it; Copy diagnostics carries the rest as sent. A reply missing what the
/// worker always sends doesn't parse, rather than reading as a state.
#[derive(Debug, Clone, Deserialize, PartialEq)]
pub struct Reply {
    ok: bool,
    reason: Option<Refusal>,
    paired: bool,
    endpoint: Option<String>,
    source_label: String,
    entry_url: String,
    tab_pattern: Option<String>,
    hosts: Vec<String>,
    state: Recording,
    run: Option<Run>,
    practice: Option<String>,
    readiness: Option<Readiness>,
}

/// What the worker last recorded, of which the popup reads only the code.
#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq)]
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
    /// Reads are going through.
    #[serde(rename = "connected")]
    Connected,
    /// Chords couldn't be reached; the run stays live and tries again.
    #[serde(rename = "bridge_offline")]
    BridgeOffline,
    /// The EHR sign-in has ended or lapsed; reading resumes once signed back in.
    #[serde(
        rename = "silent_auth_unavailable",
        alias = "interaction_required",
        alias = "unauthorized"
    )]
    SignedOut,
    /// Anything else that stopped the run.
    #[serde(untagged)]
    Stopped(Stop),
}

/// What stopped a run, other than the EHR sign-in ending.
#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq)]
pub enum Stop {
    /// Another EHR account or practice is signed in.
    #[serde(rename = "auth_identity_changed")]
    IdentityChanged,
    /// The EHR refused this account access.
    #[serde(rename = "access_denied")]
    AccessDenied,
    /// Renewing failed for a reason that says nothing about the sign-in.
    #[serde(rename = "auth_failed")]
    RenewalFailed,
    /// Token renewal was rate-limited.
    #[serde(rename = "rate_limited")]
    RateLimited,
    /// Host permissions for the reader were revoked.
    #[serde(rename = "permission_required")]
    PermissionRevoked,
    /// A code the popup doesn't single out; diagnostics carry the code itself.
    #[serde(other)]
    Unexpected,
}

/// Why the worker refused an action, among the codes `pair`, `unpair`, `connect` and `stop` answer with.
#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq)]
pub enum Refusal {
    /// The extension isn't paired with Chords yet.
    #[serde(rename = "unpaired")]
    Unpaired,
    /// The Chords address isn't a valid URL.
    #[serde(rename = "invalid_endpoint")]
    InvalidEndpoint,
    /// Chords refused the pairing request or code.
    #[serde(rename = "pair_refused")]
    PairRefused,
    /// Chrome host permission was declined.
    #[serde(rename = "permission_required")]
    PermissionRequired,
    /// More or fewer than one EHR tab is open.
    #[serde(rename = "open_one_source_tab")]
    NotOneTab,
    /// The EHR isn't signed in.
    #[serde(rename = "sign_in_required")]
    SignInRequired,
    /// The EHR shows an unrecognized or unauthorized practice.
    #[serde(rename = "other_practice")]
    WrongPractice,
    /// A run is already under way.
    #[serde(rename = "stop_before_prepare", alias = "already_running")]
    AlreadyConnected,
    /// A read kept from the last run couldn't be handed to Coda.
    #[serde(rename = "pending_delivery")]
    PendingDelivery,
    /// A code the popup doesn't single out; diagnostics carry the code itself.
    #[serde(other)]
    Unexpected,
}

/// A continuous background reading run.
#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq)]
struct Run {
    stopped: bool,
    #[serde(default)]
    awaiting_auth: bool,
}

/// How far the physician's EHR tab is from ready, each step following the one before.
#[derive(Debug, Clone, Copy, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum Readiness {
    /// No EHR tab is open.
    #[serde(rename = "closed")]
    Closed,
    /// The EHR is open, and not signed in.
    #[serde(rename = "signed_out")]
    SignedOut,
    /// Signed in, on a practice Coda doesn't read.
    #[serde(rename = "other_practice")]
    OtherPractice,
    /// Signed in, on the practice Coda reads.
    #[serde(rename = "ready")]
    Ready,
}

/// Where the connection stands: pairing first, then the worker's code weighed against its run.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Standing {
    /// Not yet paired with Chords.
    Unpaired,
    /// Paired and not reading, with how far the EHR is from ready when host permissions allow checking it.
    NotConnected(Option<Readiness>),
    /// A run is live and reads are going through.
    Reading,
    /// A run is live and waiting on Chords.
    WaitingForBridge,
    /// The EHR sign-in lapsed; reading resumes once signed back in.
    SignedOut(Option<Readiness>),
    /// Something else ended the run.
    Stopped(Stop),
}

impl Reply {
    /// Why the worker refused the action this answers, if it did.
    pub fn refused(&self) -> Option<Refusal> {
        if self.ok {
            return None;
        }
        Some(self.reason.unwrap_or(Refusal::Unexpected))
    }

    /// Whether the extension is paired with Chords.
    pub fn is_paired(&self) -> bool {
        self.paired
    }

    /// Display name of the active EHR reader.
    pub fn source_label(&self) -> &str {
        &self.source_label
    }

    /// Entry URL of the active EHR reader.
    pub fn entry_url(&self) -> &str {
        &self.entry_url
    }

    /// Tab match pattern of the active EHR reader, if any.
    pub fn tab_pattern(&self) -> Option<&str> {
        self.tab_pattern.as_deref()
    }

    /// Host patterns Chrome requests when connecting the active EHR reader.
    pub fn hosts(&self) -> &[String] {
        &self.hosts
    }

    /// Paired Chords endpoint, when paired.
    pub fn endpoint(&self) -> Option<&str> {
        self.endpoint.as_deref()
    }

    /// Where the connection stands.
    pub fn standing(&self) -> Standing {
        if !self.paired {
            return Standing::Unpaired;
        }
        let live = self
            .run
            .is_some_and(|run| !run.stopped && !run.awaiting_auth);
        match (self.state.state, live) {
            (Recorded::Connected, true) => Standing::Reading,
            (Recorded::BridgeOffline, true) => Standing::WaitingForBridge,
            (Recorded::Connected | Recorded::BridgeOffline, false) | (Recorded::Idle, _) => {
                Standing::NotConnected(self.readiness)
            }
            (Recorded::SignedOut, _) => Standing::SignedOut(self.readiness),
            (Recorded::Stopped(stop), _) => Standing::Stopped(stop),
        }
    }
}

/// What the popup asks of the worker, each by the name `connector.js` lists it under.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(tag = "action")]
pub enum Request {
    /// Where things stand.
    #[serde(rename = "status")]
    Status,
    /// Pair the extension with Chords.
    #[serde(rename = "pair")]
    Pair {
        /// Chords base URL.
        endpoint: String,
        /// Optional one-time pairing code.
        #[serde(skip_serializing_if = "Option::is_none")]
        code: Option<String>,
    },
    /// Remove the current Chords pairing while idle.
    #[serde(rename = "unpair")]
    Unpair,
    /// Start reading from the signed-in EHR tab.
    #[serde(rename = "connect")]
    Connect,
    /// Stop reading and clear the in-memory session.
    #[serde(rename = "stop")]
    Stop,
}

impl Request {
    /// Its action name in the worker.
    pub fn name(&self) -> &'static str {
        match self {
            Self::Status => "status",
            Self::Pair { .. } => "pair",
            Self::Unpair => "unpair",
            Self::Connect => "connect",
            Self::Stop => "stop",
        }
    }

    /// JSON message sent to the worker.
    pub fn to_wire(&self) -> Value {
        serde_json::to_value(self).expect("popup request serializes")
    }
}

/// What the popup's one primary action does.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    /// Pair the extension with Chords.
    Pair,
    /// Bring the physician's EHR tab forward, or open the EHR.
    OpenSource,
    /// Request host permissions if needed and start reading from the signed-in EHR tab.
    Connect,
}

/// The popup's one action and its words.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Offer {
    /// What it does.
    pub action: Action,
    /// Its words.
    pub label: String,
}

/// The popup's face for one reply.
#[derive(Debug, Clone, PartialEq)]
pub struct Face {
    /// Eyebrow naming the surface or source.
    pub eyebrow: String,
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
    /// A run is under way, waiting on Chords. Disconnect would end it.
    Waiting,
    /// A run is under way and reading. Disconnect would end it.
    Reading,
}

/// A notice above the state.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Note {
    /// Its tone.
    pub tone: Tone,
    /// Its title.
    pub title: &'static str,
    /// Its detail.
    pub detail: String,
}

fn offer(action: Action, label: impl Into<String>) -> Offer {
    Offer {
        action,
        label: label.into(),
    }
}

fn state(
    eyebrow: &str,
    icon: IconName,
    title: impl Into<String>,
    detail: impl Into<String>,
    offer: Option<Offer>,
) -> Face {
    Face {
        eyebrow: eyebrow.to_owned(),
        icon,
        title: title.into(),
        detail: detail.into(),
        steps: Vec::new(),
        offer,
        activity: Activity::Idle,
    }
}

/// The steps to ready, each done or still to do.
fn steps(source: &str, ready: Readiness) -> Vec<Task> {
    [
        (format!("Open {source}"), ready > Readiness::Closed),
        ("Sign in".to_owned(), ready > Readiness::SignedOut),
        (
            "Choose your practice".to_owned(),
            ready > Readiness::OtherPractice,
        ),
    ]
    .into_iter()
    .map(|(label, done)| Task {
        label,
        state: if done {
            TaskState::Done
        } else {
            TaskState::Pending
        },
    })
    .collect()
}

/// Not reading: the next step to ready when host permissions allowed checking it, or else sign in and connect.
fn not_connected(source: &str, readiness: Option<Readiness>, practice: Option<&str>) -> Face {
    let Some(ready) = readiness else {
        return state(
            source,
            IconName::Link,
            format!("Connect {source}"),
            format!(
                "Sign into {source} in one tab, then connect. Chrome will ask permission for {source}'s hosts, and after connecting you can close {source}."
            ),
            Some(offer(Action::Connect, format!("Connect {source}"))),
        );
    };
    let built = match ready {
        Readiness::Closed => state(
            source,
            IconName::Link,
            format!("Open {source} to connect"),
            format!("Coda reads {source} in the background, under your own sign-in."),
            Some(offer(Action::OpenSource, format!("Open {source}"))),
        ),
        Readiness::SignedOut => state(
            source,
            IconName::User,
            format!("Sign into {source}"),
            "Use your usual sign-in. Coda never sees your password.".to_owned(),
            Some(offer(Action::OpenSource, format!("Go to {source}"))),
        ),
        Readiness::OtherPractice => state(
            source,
            IconName::Users,
            "Choose your practice",
            format!("Switch {source} to the practice Coda reads."),
            Some(offer(Action::OpenSource, format!("Go to {source}"))),
        ),
        Readiness::Ready => {
            let fallback = format!("Your {source} practice");
            let practice = practice.unwrap_or(&fallback);
            state(
                source,
                IconName::Link,
                "Ready to connect",
                format!("{practice}. After connecting you can close {source}."),
                Some(offer(Action::Connect, format!("Connect {source}"))),
            )
        }
    };
    Face {
        steps: steps(source, ready),
        ..built
    }
}

/// Sign-in ended or paused the run: reconnect or catch up once signed back in.
fn signed_out(source: &str, readiness: Option<Readiness>) -> Face {
    let built = match readiness {
        Some(Readiness::OtherPractice | Readiness::Ready) => state(
            source,
            IconName::Link,
            format!("{source} is signed in again"),
            "Reconnect to resume right away, or wait a moment for background catch-up. What Coda already read is kept.",
            Some(offer(Action::Connect, "Reconnect")),
        ),
        Some(Readiness::Closed | Readiness::SignedOut) | None => state(
            source,
            IconName::Alert,
            format!("Sign back into {source}"),
            format!(
                "Coda lost its access to {source}, usually because {source} signed out. Sign back in and Coda catches up where it left off."
            ),
            Some(if readiness.is_some() {
                offer(Action::OpenSource, format!("Go to {source}"))
            } else {
                offer(Action::Connect, "Reconnect")
            }),
        ),
    };
    Face {
        steps: readiness.map(|r| steps(source, r)).unwrap_or_default(),
        ..built
    }
}

/// The face for `reply`.
pub fn face(reply: &Reply) -> Face {
    let source = reply.source_label.as_str();
    match reply.standing() {
        Standing::Unpaired => state(
            "Chords",
            IconName::Link,
            "Pair with Coda",
            "Enter your Chords address to pair this browser. Sign-in tokens stay in Chrome's memory.",
            Some(offer(Action::Pair, "Pair extension")),
        ),
        Standing::NotConnected(readiness) => {
            not_connected(source, readiness, reply.practice.as_deref())
        }
        Standing::Reading => {
            let detail = match &reply.practice {
                Some(practice) => format!("{practice}. You can close {source}."),
                None => format!("You can close {source} and keep working."),
            };
            Face {
                activity: Activity::Reading,
                ..state(
                    source,
                    IconName::Check,
                    format!("Coda is reading {source}"),
                    detail,
                    None,
                )
            }
        }
        Standing::WaitingForBridge => Face {
            activity: Activity::Waiting,
            ..state(
                source,
                IconName::Clock,
                "Waiting for Coda",
                "Coda's connection isn't reachable. Reading picks up on its own when it is.",
                None,
            )
        },
        Standing::SignedOut(readiness) => signed_out(source, readiness),
        Standing::Stopped(Stop::IdentityChanged) => state(
            source,
            IconName::Alert,
            format!("A different {source} account is signed in"),
            format!(
                "{source} is signed into another account or practice than the one you connected."
            ),
            Some(offer(Action::Connect, "Connect this account")),
        ),
        Standing::Stopped(Stop::AccessDenied) => state(
            source,
            IconName::Alert,
            format!("{source} refused access"),
            format!("{source} didn't allow this account to read the chart."),
            Some(offer(Action::Connect, "Try again")),
        ),
        Standing::Stopped(Stop::RenewalFailed) => state(
            source,
            IconName::Alert,
            format!("Coda couldn't renew access to {source}"),
            format!(
                "{source} didn't answer the way Coda expected. What Coda already read is kept."
            ),
            Some(offer(Action::Connect, "Try again")),
        ),
        Standing::Stopped(Stop::RateLimited) => state(
            source,
            IconName::Clock,
            format!("{source} asked Coda to slow down"),
            "Try again in a few minutes.",
            Some(offer(Action::Connect, "Try again")),
        ),
        Standing::Stopped(Stop::PermissionRevoked) => state(
            source,
            IconName::Alert,
            format!("Allow access to {source}"),
            format!(
                "Chrome no longer grants access to {source}'s hosts. Reconnect to allow them again."
            ),
            Some(offer(Action::Connect, format!("Connect {source}"))),
        ),
        Standing::Stopped(Stop::Unexpected) => state(
            source,
            IconName::Alert,
            format!("Coda stopped reading {source}"),
            "Something unexpected happened. Copy diagnostics from the menu to share it.",
            Some(offer(Action::Connect, "Try again")),
        ),
    }
}

/// Origin match pattern (`scheme://host[:port]/*`) for requesting Chrome host permission on Pair.
pub fn endpoint_origin_pattern(endpoint: &str) -> Option<String> {
    let trimmed = endpoint.trim();
    let (scheme, rest) = trimmed
        .strip_prefix("https://")
        .map(|r| ("https://", r))
        .or_else(|| trimmed.strip_prefix("http://").map(|r| ("http://", r)))?;
    let host_port = rest.split('/').next().unwrap_or_default();
    if host_port.is_empty()
        || host_port.contains('@')
        || host_port.contains('?')
        || host_port.contains('#')
    {
        return None;
    }
    Some(format!("{scheme}{host_port}/*"))
}

/// The notice for an action the worker refused.
pub fn refusal(reason: Refusal, source: &str) -> Note {
    let (tone, title, detail) = match reason {
        Refusal::Unpaired => (
            Tone::Warning,
            "Pair with Coda first",
            "Enter your Chords address to pair this extension before connecting.".to_owned(),
        ),
        Refusal::InvalidEndpoint => (
            Tone::Warning,
            "Check the Chords address",
            "Enter an https:// or http:// address without extra query text.".to_owned(),
        ),
        Refusal::PairRefused => (
            Tone::Error,
            "Couldn't pair with Coda",
            "Chords refused the pairing request. Check the address and pairing code, then try again."
                .to_owned(),
        ),
        Refusal::PermissionRequired => (
            Tone::Warning,
            "Allow site access in Chrome",
            format!("Chrome needs host access for {source} and Coda so Coda can read in the background."),
        ),
        Refusal::NotOneTab => (
            Tone::Warning,
            "Open one EHR tab",
            format!("Keep {source} open in a single tab, then connect."),
        ),
        Refusal::SignInRequired => (
            Tone::Warning,
            "Sign in first",
            format!("Sign into {source} in its tab, then connect."),
        ),
        Refusal::WrongPractice => (
            Tone::Warning,
            "Choose your practice",
            format!("Switch {source} to your practice, then connect."),
        ),
        Refusal::AlreadyConnected => (
            Tone::Neutral,
            "Already connected",
            "Disconnect from the menu to start over.".to_owned(),
        ),
        Refusal::PendingDelivery => (
            Tone::Warning,
            "A read is waiting to reach Coda",
            "Coda kept the last read in its outbox but couldn't hand it over yet. Check Coda's connection is reachable, then try again."
                .to_owned(),
        ),
        Refusal::Unexpected => (
            Tone::Error,
            "Couldn't complete that action",
            format!("{source} didn't respond as expected. Copy diagnostics from the menu to share it."),
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

    fn reply(state: Recorded, run: Option<Run>) -> Reply {
        Reply {
            ok: true,
            reason: None,
            paired: true,
            endpoint: Some("https://chords.example".to_owned()),
            source_label: "Hint".to_owned(),
            entry_url: "https://app.hint.com/".to_owned(),
            tab_pattern: Some("https://app.hint.com/*".to_owned()),
            hosts: vec!["https://app.hint.com/*".to_owned()],
            state: Recording { state },
            run,
            practice: None,
            readiness: None,
        }
    }

    fn running() -> Option<Run> {
        Some(Run {
            stopped: false,
            awaiting_auth: false,
        })
    }

    #[test]
    fn unpaired_reply_asks_to_pair_first() {
        let mut unpaired = reply(Recorded::Idle, None);
        unpaired.paired = false;
        let shown = face(&unpaired);
        assert_eq!(shown.title, "Pair with Coda");
        assert_eq!(shown.offer, Some(offer(Action::Pair, "Pair extension")));
    }

    #[test]
    fn a_live_run_reads_quietly_with_disconnect_in_the_menu() {
        let shown = face(&reply(Recorded::Connected, running()));
        assert_eq!(shown.title, "Coda is reading Hint");
        assert_eq!(shown.activity, Activity::Reading);
        assert_eq!(shown.offer, None);
    }

    #[test]
    fn a_connected_state_without_a_live_run_never_claims_reading() {
        let stopped = Some(Run {
            stopped: true,
            awaiting_auth: false,
        });
        let shown = face(&reply(Recorded::Connected, stopped));
        assert_eq!(shown.activity, Activity::Idle);
        assert_eq!(shown.offer, Some(offer(Action::Connect, "Connect Hint")));
    }

    #[test]
    fn waiting_on_the_bridge_keeps_disconnect_and_never_claims_reading() {
        let shown = face(&reply(Recorded::BridgeOffline, running()));
        assert_eq!(shown.title, "Waiting for Coda");
        assert_eq!(shown.activity, Activity::Waiting);
    }

    #[test]
    fn without_readiness_the_popup_claims_no_steps() {
        let shown = face(&reply(Recorded::Idle, None));
        assert_eq!(shown.title, "Connect Hint");
        assert!(shown.steps.is_empty());
    }

    #[test]
    fn readiness_names_the_next_step_and_its_action() {
        let mut waiting = reply(Recorded::Idle, None);
        waiting.readiness = Some(Readiness::SignedOut);
        let shown = face(&waiting);
        assert_eq!(shown.title, "Sign into Hint");
        assert_eq!(shown.offer, Some(offer(Action::OpenSource, "Go to Hint")));
        let states: Vec<_> = shown.steps.iter().map(|task| task.state).collect();
        assert_eq!(
            states,
            [TaskState::Done, TaskState::Pending, TaskState::Pending]
        );
        waiting.readiness = Some(Readiness::Ready);
        assert_eq!(
            face(&waiting).offer,
            Some(offer(Action::Connect, "Connect Hint"))
        );
    }

    #[test]
    fn sign_out_and_return_offer_reconnect_and_keep_prior_reads() {
        let mut lapsed = reply(
            Recorded::SignedOut,
            Some(Run {
                stopped: false,
                awaiting_auth: true,
            }),
        );
        assert_eq!(face(&lapsed).title, "Sign back into Hint");
        lapsed.readiness = Some(Readiness::Ready);
        let back = face(&lapsed);
        assert_eq!(back.title, "Hint is signed in again");
        assert_eq!(back.offer, Some(offer(Action::Connect, "Reconnect")));
    }

    #[test]
    fn endpoint_origin_pattern_validates_urls() {
        assert_eq!(
            endpoint_origin_pattern("https://chords.coda.example/api"),
            Some("https://chords.coda.example/*".to_owned())
        );
        assert_eq!(
            endpoint_origin_pattern("http://127.0.0.1:51383"),
            Some("http://127.0.0.1:51383/*".to_owned())
        );
        assert_eq!(endpoint_origin_pattern("ftp://bad"), None);
        assert_eq!(endpoint_origin_pattern("https://user:pass@bad"), None);
    }

    #[test]
    fn the_worker_reply_parses_as_it_sends_it() {
        let parsed: Reply = serde_json::from_str(
            r#"{"ok":true,"version":"0.4.0","paired":true,"endpoint":"http://127.0.0.1:51383",
            "source":"hint","source_label":"Hint","entry_url":"https://app.hint.com/",
            "tab_pattern":"https://app.hint.com/*","hosts":["https://app.hint.com/*"],
            "state":{"at":1793456970780,"portal_tab_count":0,"state":"connected","token_exp":1793457870,"token_iat":1793456970},
            "run":{"id":"3d9a","source":"hint","started":1793456969948,"stopped":false,"awaiting_auth":false},
            "pending_delivery":false,"practice":"Northwind Direct Primary Care",
            "permissions_granted":true,"readiness":"ready"}"#,
        )
        .unwrap();
        assert_eq!(parsed.standing(), Standing::Reading);
        assert_eq!(
            parsed.practice.as_deref(),
            Some("Northwind Direct Primary Care")
        );
        assert_eq!(parsed.readiness, Some(Readiness::Ready));
    }

    #[test]
    fn the_worker_lists_every_request() {
        let connector = include_str!("../../../extension/chrome/connector.js");
        let listed = connector
            .split_once("const actions = {")
            .and_then(|(_, rest)| rest.split_once('}'))
            .expect("connector.js lists its actions")
            .0;
        for request in [
            Request::Status,
            Request::Pair {
                endpoint: "https://chords.example".to_owned(),
                code: None,
            },
            Request::Unpair,
            Request::Connect,
            Request::Stop,
        ] {
            assert!(
                listed.split(',').any(|name| name.trim() == request.name()),
                "connector.js has no {request:?}"
            );
        }
    }

    fn sent_codes() -> BTreeSet<&'static str> {
        let mut codes = BTreeSet::new();
        for source in [
            include_str!("../../../extension/chrome/connector.js"),
            include_str!("../../../extension/chrome/readers/hint.js"),
            include_str!("../../../extension/chrome/readers/hint-page.js"),
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
            "unauthorized",
            "read_failed",
            "connector_failed",
            "not_prepared",
            "permission_required",
            "unpaired",
        ] {
            assert!(sent.contains(code), "the scan should find {code}: {sent:?}");
        }
        let source = include_str!("face.rs");
        let allowed_request_tags = ["status", "pair", "unpair", "connect", "stop"];
        let mut named = 0;
        for names in ["rename = \"", "alias = \""] {
            for (at, _) in source.match_indices(names) {
                let code = source[at + names.len()..]
                    .split('"')
                    .next()
                    .unwrap_or_default();
                if allowed_request_tags.contains(&code) {
                    continue;
                }
                assert!(sent.contains(code), "the worker never sends {code}");
                named += 1;
            }
        }
        assert!(
            named >= 18,
            "the scan should find the popup's codes: {named}"
        );
    }

    #[test]
    fn connector_files_name_no_ehr_host_patient_or_chart_part() {
        let face_prod = include_str!("face.rs")
            .split("#[cfg(test)]")
            .next()
            .unwrap_or_default();
        for (name, content) in [
            (
                "manifest.json",
                include_str!("../../../extension/chrome/manifest.json"),
            ),
            (
                "options.html",
                include_str!("../../../extension/chrome/options.html"),
            ),
            (
                "popup.js",
                include_str!("../../../extension/chrome/popup.js"),
            ),
            (
                "connector.js",
                include_str!("../../../extension/chrome/connector.js"),
            ),
            (
                "worker.js",
                include_str!("../../../extension/chrome/worker.js"),
            ),
            ("chrome.js", include_str!("../assets/chrome.js")),
            ("lib.rs", include_str!("lib.rs")),
            ("main.rs", include_str!("main.rs")),
            ("face.rs (production)", face_prod),
        ] {
            let lower = content.to_ascii_lowercase();
            for forbidden in [
                "hint",
                "frontegg",
                "127.0.0.1",
                "localhost",
                "hint.com",
                ".com/",
                ".com\"",
                ".com'",
                ".health",
                "sbx-pat",
                "diagnoses",
                "allergies",
                "medications",
                "vitals",
                "vaccines",
                "surgeries",
            ] {
                assert!(
                    !lower.contains(forbidden),
                    "{name} must not contain `{forbidden}`"
                );
            }
        }
    }
}
