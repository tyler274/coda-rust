//! The rehearsal physician's side of a drill: sign in through IAP as the
//! rehearsal service account, send one chat turn, and wait for its reply.
use std::time::Duration;

use physician_wire::{AnswerState, ChatSnapshotResponse};
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

/// Reads a `GET .../snapshot` answer by its latest exchange. A finished
/// answer names the attempt that published it, which the drill follows into
/// telemetry. An answer Coda couldn't finish names the run Try again would
/// continue, which is the run that failed. Anything else is still writing.
pub(super) fn turn_state(body: Value) -> Result<TurnState, DrillError> {
    let page: ChatSnapshotResponse =
        serde_json::from_value(body).map_err(|source| DrillError::Decode {
            what: "chat snapshot",
            source,
        })?;
    Ok(
        match page
            .snapshot
            .transcript
            .into_iter()
            .last()
            .map(|exchange| exchange.answer)
        {
            Some(AnswerState::Finished {
                text, attempt_id, ..
            }) => TurnState::Answered(Answer {
                text,
                attempt: attempt_id.0,
            }),
            Some(AnswerState::CouldntFinish {
                try_again: Some(target),
            }) => TurnState::Failed(target.expected_run_id),
            Some(
                AnswerState::Working { .. }
                | AnswerState::Stopped { .. }
                | AnswerState::CutOff { .. }
                | AnswerState::CouldntFinish { try_again: None },
            )
            | None => TurnState::Writing,
        },
    )
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

    /// Polls the snapshot until the turn is answered or failed.
    pub(super) fn await_reply(
        &self,
        turn: &Turn,
        attempts: u32,
        pause: Duration,
        watch: &Watch,
    ) -> Result<Answer, DrillError> {
        let path = format!(
            "{}/patient-workspaces/{}/chats/{}/snapshot",
            self.origin, turn.workspace, turn.chat
        );
        let answer = watch.poll(attempts, pause, |_| {
            let body = send_json(self.authorized(self.client.get(&path)), "chat snapshot")?;
            match turn_state(body)? {
                TurnState::Answered(answer) => Ok(Some(answer)),
                TurnState::Failed(run) => Err(DrillError::TurnFailed { run }),
                TurnState::Writing => Ok(None),
            }
        })?;
        answer.ok_or(DrillError::Timeout {
            what: "the rehearsal reply",
            seconds: u64::from(attempts) * pause.as_secs(),
        })
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

    use physician_wire::{
        AttemptId, ChatSnapshot, Exchange, MessageKey, PhysicianMessage, ResponseId,
        SnapshotFeedback, WorkItemId, WorkResumeTarget, WorkStopTarget,
    };

    /// A snapshot of one exchange asking "Hi", answered as `answer`.
    fn snapshot(answer: AnswerState) -> Value {
        serde_json::to_value(ChatSnapshotResponse {
            snapshot: ChatSnapshot {
                transcript: vec![Exchange {
                    work_item_id: WorkItemId("w-1".into()),
                    messages: vec![PhysicianMessage {
                        id: MessageKey("k".into()),
                        text: "Hi".into(),
                    }],
                    steps: physician_wire::ExchangeSteps::Read(vec![]),
                    kept: vec![],
                    answer,
                }],
                feedback: SnapshotFeedback::Loaded,
            },
        })
        .unwrap()
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
    fn a_finished_answer_names_its_attempt_and_couldnt_finish_names_its_run() {
        let state = |answer| turn_state(snapshot(answer)).unwrap();
        assert_eq!(
            state(AnswerState::Finished {
                response_id: ResponseId("a-1:reply".into()),
                attempt_id: AttemptId("a-1".into()),
                text: "Hello".into(),
                feedback: None,
            }),
            TurnState::Answered(Answer {
                text: "Hello".into(),
                attempt: "a-1".into()
            })
        );
        assert_eq!(
            state(AnswerState::Working {
                stop: WorkStopTarget {
                    work_item_id: WorkItemId("w-1".into())
                }
            }),
            TurnState::Writing
        );
        assert_eq!(
            state(AnswerState::CouldntFinish {
                try_again: Some(WorkResumeTarget {
                    work_item_id: WorkItemId("w-1".into()),
                    expected_run_id: "r-2".into(),
                })
            }),
            TurnState::Failed("r-2".into())
        );
        assert!(
            turn_state(json!({ "snapshot": { "conversation": [], "runs": [] } })).is_err(),
            "the older snapshot is refused, not read as still writing"
        );
    }
}
