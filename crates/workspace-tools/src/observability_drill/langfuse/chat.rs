//! The rehearsal physician's side of a drill: sign in through IAP as the
//! rehearsal service account, send one chat turn, and wait for its reply.
use std::time::Duration;

use chrono::{DateTime, Utc};
use reqwest::blocking::{Client, RequestBuilder};
use reqwest::header::PROXY_AUTHORIZATION;
use serde_json::{Value, json};

use super::DrillError;
use super::watch::Watch;

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
            "type": "message.submit",
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

/// An answered turn: the reply and the scoped attempt that wrote it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct Answer {
    pub(super) text: String,
    pub(super) attempt: String,
}

/// Where a turn stands in the chat snapshot.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) enum TurnState {
    Writing,
    Answered(Answer),
    Failed(String),
}

/// Reads a `GET .../snapshot` answer. The snapshot names the run once the
/// turn failed, was held or was answered, and an answered run names the
/// attempt that answered it; a turn counts as answered only when its reply
/// and its `completed` attempt are both there.
pub(super) fn turn_state(body: &Value) -> TurnState {
    let snapshot = &body["snapshot"];
    let latest = |state: &str, id: &str| {
        snapshot["runs"]
            .as_array()
            .into_iter()
            .flatten()
            .find(|run| run["state"] == state)
            .and_then(|run| run[id].as_str())
            .map(str::to_owned)
    };
    if let Some(run) = latest("failed", "run_id") {
        return TurnState::Failed(run);
    }
    let items = snapshot["conversation"].as_array();
    let reply = items
        .and_then(|items| items.last())
        .filter(|item| item["kind"] == "model")
        .and_then(|item| item["text"].as_str());
    match (reply, latest("completed", "attempt_id")) {
        (Some(text), Some(attempt)) => TurnState::Answered(Answer {
            text: text.to_owned(),
            attempt,
        }),
        _ => TurnState::Writing,
    }
}

/// How many model replies the snapshot's conversation holds.
pub(super) fn model_replies(body: &Value) -> usize {
    body["snapshot"]["conversation"]
        .as_array()
        .into_iter()
        .flatten()
        .filter(|item| item["kind"] == "model")
        .count()
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
            rehearsal_sign_in(&client, &origin, &iap),
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

    /// One read of a chat's snapshot.
    pub(super) fn snapshot(&self, workspace: &str, chat: &str) -> Result<Value, DrillError> {
        send_json(self.snapshot_request(workspace, chat), "chat snapshot")
    }

    fn snapshot_request(&self, workspace: &str, chat: &str) -> RequestBuilder {
        let path = format!(
            "{}/patient-workspaces/{workspace}/chats/{chat}/snapshot",
            self.origin
        );
        self.authorized(self.client.get(&path))
    }

    /// Polls the snapshot until the turn is answered or failed, and returns
    /// the answer with the snapshot that held it. A `deadline` ends the wait
    /// once it passes, so slow snapshot requests cannot stretch it.
    pub(super) fn await_reply(
        &self,
        turn: &Turn,
        attempts: u32,
        pause: Duration,
        deadline: Option<DateTime<Utc>>,
        watch: &Watch,
    ) -> Result<(Answer, Value), DrillError> {
        let timeout = || DrillError::Timeout {
            what: "the rehearsal reply",
            seconds: u64::from(attempts) * pause.as_secs(),
        };
        // Time left before `deadline`; `Err` once it has passed.
        let left = || match deadline {
            None => Ok(None),
            Some(deadline) => (deadline - Utc::now())
                .to_std()
                .ok()
                .filter(|left| !left.is_zero())
                .map(Some)
                .ok_or_else(timeout),
        };
        let answer = watch.poll(attempts, pause, |_| {
            let mut request = self.snapshot_request(&turn.workspace, &turn.chat);
            if let Some(left) = left()? {
                request = request.timeout(left);
            }
            let body = send_json(request, "chat snapshot")?;
            // A reply read after the deadline is still too late.
            left()?;
            match turn_state(&body) {
                TurnState::Answered(answer) => Ok(Some((answer, body))),
                TurnState::Failed(run) => Err(DrillError::TurnFailed { run }),
                TurnState::Writing => Ok(None),
            }
        })?;
        answer.ok_or_else(timeout)
    }
}

/// `POST /auth/rehearsal` through IAP. Google's front end answers a POST
/// without `Content-Length` with 411, so the request carries an empty JSON
/// body the route ignores.
fn rehearsal_sign_in(client: &Client, origin: &str, iap: &str) -> RequestBuilder {
    client
        .post(format!("{origin}/auth/rehearsal"))
        .header(PROXY_AUTHORIZATION, format!("Bearer {iap}"))
        .json(&json!({}))
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

#[cfg(test)]
mod tests {
    use super::*;

    fn snapshot(conversation: Value, runs: Value) -> Value {
        json!({ "snapshot": { "conversation": conversation, "runs": runs } })
    }

    #[test]
    fn the_turn_is_a_message_submit_command_on_the_wire() {
        let turn = Turn {
            workspace: "w".into(),
            chat: "c".into(),
            text: "t".into(),
            scenario: "s".into(),
            key: "k".into(),
        };
        let command = turn.command();
        assert_eq!(command["type"], "message.submit");
        assert_eq!(command["idempotency_key"], "k");
        assert_eq!(command["payload"]["scenario"], "s");
    }

    #[test]
    fn the_rehearsal_sign_in_has_a_length_for_google_front_end() {
        let request = rehearsal_sign_in(&Client::new(), "https://physician.example", "jwt")
            .build()
            .unwrap();
        assert_eq!(request.url().path(), "/auth/rehearsal");
        assert_eq!(request.headers()[PROXY_AUTHORIZATION], "Bearer jwt");
        let length = request
            .body()
            .and_then(|body| body.as_bytes())
            .map(<[u8]>::len);
        assert_eq!(length, Some(2));
    }

    #[test]
    fn an_answer_needs_the_reply_and_its_completed_attempt() {
        let asked = json!([{ "kind": "physician", "text": "Hi" }]);
        let answered = json!([
            { "kind": "physician", "text": "Hi" },
            { "kind": "model", "text": "Hello" }
        ]);
        assert_eq!(
            turn_state(&snapshot(
                answered.clone(),
                json!([{ "run_id": "r-1", "state": "completed", "attempt_id": "a-1" }])
            )),
            TurnState::Answered(Answer {
                text: "Hello".into(),
                attempt: "a-1".into()
            })
        );
        assert_eq!(
            turn_state(&snapshot(answered, json!([]))),
            TurnState::Writing
        );
        assert_eq!(
            turn_state(&snapshot(asked.clone(), json!([]))),
            TurnState::Writing
        );
        assert_eq!(
            turn_state(&snapshot(
                asked,
                json!([{ "run_id": "a-2", "state": "failed" }])
            )),
            TurnState::Failed("a-2".into())
        );
    }

    #[test]
    fn replies_count_only_model_items() {
        let conversation = json!([
            { "kind": "physician", "text": "Hi" },
            { "kind": "model", "text": "Hello" },
            { "kind": "model", "text": "Hello again" }
        ]);
        assert_eq!(model_replies(&snapshot(conversation, json!([]))), 2);
        assert_eq!(model_replies(&json!({})), 0);
    }
}
