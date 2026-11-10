//! The rehearsal physician's side of a drill: sign in through IAP as the
//! rehearsal service account, send one chat turn, and wait for its reply.
use std::time::Duration;

use reqwest::blocking::{Client, RequestBuilder};
use reqwest::header::PROXY_AUTHORIZATION;
use serde_json::{Value, json};

use super::DrillError;

/// The rehearsal service account (`infra/gcp/rust-environments/rehearsal.tf`).
pub(super) const REHEARSAL_ACCOUNT: &str =
    "physician-rehearsal@coda-rust-staging.iam.gserviceaccount.com";
const TIME_ZONE: &str = "America/New_York";

/// One message the drill sends.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct Turn {
    pub(super) workspace: String,
    pub(super) chat: String,
    pub(super) text: String,
    pub(super) scenario: String,
    pub(super) key: String,
}

impl Turn {
    pub(super) fn command(&self) -> Value {
        json!({
            "kind": "message.submit",
            "idempotency_key": self.key,
            "payload": {
                "patient_workspace_id": self.workspace,
                "chat_id": self.chat,
                "text": self.text,
                "physician_time_zone": TIME_ZONE,
                "scenario": self.scenario,
            },
        })
    }
}

/// Where a turn stands in the chat snapshot.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) enum TurnState {
    Writing,
    Answered(String),
    Failed(String),
}

/// Reads a `GET .../snapshot` answer. A failed run wins over any earlier
/// reply, because the snapshot only names a run once the latest turn failed.
pub(super) fn turn_state(body: &Value) -> TurnState {
    let snapshot = &body["snapshot"];
    let failed = snapshot["runs"]
        .as_array()
        .into_iter()
        .flatten()
        .find(|run| run["state"] == "failed");
    if let Some(run) = failed {
        return TurnState::Failed(run["run_id"].as_str().unwrap_or("unnamed").to_owned());
    }
    let items = snapshot["conversation"].as_array();
    let last = items.and_then(|items| items.last());
    match last {
        Some(item) if item["kind"] == "model" => {
            TurnState::Answered(item["text"].as_str().unwrap_or_default().to_owned())
        }
        _ => TurnState::Writing,
    }
}

/// A signed-in rehearsal physician. IAP admits the signed JWT in
/// `Proxy-Authorization`; the physician session travels in `Authorization`.
pub(super) struct Physician {
    client: Client,
    origin: String,
    iap: String,
    session: String,
}

impl Physician {
    pub(super) fn sign_in(
        client: Client,
        origin: &str,
        access_token: &str,
        now: u64,
    ) -> Result<Self, DrillError> {
        let origin = origin.trim_end_matches('/').to_owned();
        let call = http::signjwt::iap_request(
            http::signjwt::IAM_CREDENTIALS,
            REHEARSAL_ACCOUNT,
            &origin,
            now,
        );
        let answer = send_json(
            client
                .post(&call.url)
                .bearer_auth(access_token)
                .json(&call.body),
            "IAM signJwt for the rehearsal account",
        )?;
        let iap = http::signjwt::signed_jwt(&answer)
            .ok_or(DrillError::Shape {
                what: "IAM signJwt",
                field: "signedJwt",
            })?
            .to_owned();
        let answer = send_json(
            client
                .post(format!("{origin}/auth/rehearsal"))
                .header(PROXY_AUTHORIZATION, format!("Bearer {iap}")),
            "physician rehearsal sign-in",
        )?;
        let session = answer["session"]
            .as_str()
            .filter(|session| !session.is_empty())
            .ok_or(DrillError::Shape {
                what: "physician rehearsal sign-in",
                field: "session",
            })?
            .to_owned();
        Ok(Self {
            client,
            origin,
            iap,
            session,
        })
    }

    fn authorized(&self, request: RequestBuilder) -> RequestBuilder {
        request
            .header(PROXY_AUTHORIZATION, format!("Bearer {}", self.iap))
            .bearer_auth(&self.session)
    }

    /// The first patient on the rehearsal panel.
    pub(super) fn first_workspace(&self) -> Result<String, DrillError> {
        let what = "rehearsal patient list";
        let body = send_json(
            self.authorized(
                self.client
                    .get(format!("{}/patient-workspaces", self.origin)),
            ),
            what,
        )?;
        body["patients"]
            .as_array()
            .and_then(|patients| patients.first())
            .and_then(|patient| patient["patient_workspace_id"].as_str())
            .map(str::to_owned)
            .ok_or(DrillError::Shape {
                what,
                field: "patients[0].patient_workspace_id",
            })
    }

    pub(super) fn submit(&self, turn: &Turn) -> Result<(), DrillError> {
        let body = send_json(
            self.authorized(
                self.client
                    .post(format!("{}/commands", self.origin))
                    .json(&turn.command()),
            ),
            "message.submit",
        )?;
        match body["outcome"].as_str() {
            Some("accepted") => Ok(()),
            other => Err(DrillError::Rejected(
                other.unwrap_or("no outcome").to_owned(),
            )),
        }
    }

    /// Polls the snapshot until the turn is answered or failed.
    pub(super) fn await_reply(
        &self,
        turn: &Turn,
        attempts: u32,
        pause: Duration,
    ) -> Result<String, DrillError> {
        let path = format!(
            "{}/patient-workspaces/{}/chats/{}/snapshot",
            self.origin, turn.workspace, turn.chat
        );
        for _attempt in 0..attempts {
            let body = send_json(self.authorized(self.client.get(&path)), "chat snapshot")?;
            match turn_state(&body) {
                TurnState::Answered(text) => return Ok(text),
                TurnState::Failed(run) => return Err(DrillError::TurnFailed { run }),
                TurnState::Writing => std::thread::sleep(pause),
            }
        }
        Err(DrillError::Timeout {
            what: "the rehearsal reply",
            seconds: u64::from(attempts) * pause.as_secs(),
        })
    }
}

/// Sends a request and reads a JSON answer. Bodies stay out of errors: an
/// answer can carry a session or chat text.
pub(super) fn send_json(request: RequestBuilder, what: &'static str) -> Result<Value, DrillError> {
    let response = request
        .send()
        .map_err(|source| DrillError::Request { what, source })?;
    let status = response.status();
    if !status.is_success() {
        return Err(DrillError::Status {
            what,
            status: status.as_u16(),
        });
    }
    response
        .json()
        .map_err(|source| DrillError::Request { what, source })
}
