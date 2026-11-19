//! The conversation as the physician reads it: one exchange per piece of
//! accepted work, with the messages that started or steered it, the steps
//! Coda took and how its answer stands. Every ending comes from the work's
//! own run history, so an answer keeps its mark after a later message
//! replaces its work.
use std::collections::HashMap;

use harness::engine::results::ToolResult;
use harness::engine::runs::{HoldReason, RunHistory, RunOutcome};
use harness::engine::{EventKind, Invocation};
use physician_wire::{
    AnswerState, AttemptId, Exchange, ExchangeSteps, MessageKey, PhysicianMessage,
    ResponseFeedback, Step, WorkItemId, WorkResumeTarget, WorkStopTarget,
};

use super::{
    AssistantKind, Attempt, AttemptStatus, Conversation, Message, MessageRole, ResponseId, SavedAt,
    reply_id,
};
use crate::physician_chat::{CallEnding, StepError};

/// A saved conversation that does not read as a transcript.
#[derive(Debug, thiserror::Error)]
pub enum TranscriptError {
    /// One of Coda's answers is saved before any physician message.
    #[error("an answer is saved before any physician message")]
    AnswerBeforeMessage,
    /// A physician message names work with no run.
    #[error("accepted work has no run")]
    MissingRun,
    /// Work that a later message replaced has no ending.
    #[error("replaced work has not ended")]
    ReplacedWorkOpen,
    /// Completed work has no published answer from its last attempt.
    #[error("completed work has no published answer")]
    MissingAnswer,
    /// Chat work never ends cancelled.
    #[error("chat work ended cancelled")]
    Cancelled,
}

/// One work's messages and Coda's saved answers, in order.
struct Gathered<'a> {
    work: &'a str,
    messages: Vec<PhysicianMessage>,
    /// Each answer's saved id, how it was saved, and the message it was saved as.
    answers: Vec<(&'a str, &'a AssistantKind, &'a Message)>,
}

/// When the server saved a message, as the wire carries it.
fn wire_time(saved_at: SavedAt) -> physician_wire::SavedAt {
    physician_wire::SavedAt(saved_at.seconds())
}

impl Conversation {
    /// The physician's messages queued behind active/held/failed work, in order.
    pub fn queued_messages(&self) -> Vec<physician_wire::QueuedMessageWire> {
        self.queued
            .iter()
            .map(|msg| physician_wire::QueuedMessageWire {
                id: MessageKey(msg.id.clone()),
                text: msg.text.clone(),
                saved_at: wire_time(msg.saved_at),
            })
            .collect()
    }

    /// The exchanges of this conversation, in the order the physician started
    /// them. `feedback` gives the physician's saved feedback on a published
    /// answer. Only the current work offers Stop, Resume or Try again.
    pub fn transcript(
        &self,
        feedback: impl Fn(&ResponseId) -> Option<ResponseFeedback>,
    ) -> Result<Vec<Exchange>, TranscriptError> {
        let mut gathered: Vec<Gathered<'_>> = Vec::new();
        for message in &self.messages {
            match &message.role {
                MessageRole::User { id, work, .. } => {
                    let message = PhysicianMessage {
                        id: MessageKey(id.clone()),
                        text: message.text.clone(),
                        saved_at: wire_time(message.saved_at),
                    };
                    match gathered.last_mut().filter(|exchange| exchange.work == work) {
                        Some(exchange) => exchange.messages.push(message),
                        None => gathered.push(Gathered {
                            work,
                            messages: vec![message],
                            answers: Vec::new(),
                        }),
                    }
                }
                MessageRole::Assistant { id, kind } => gathered
                    .last_mut()
                    .ok_or(TranscriptError::AnswerBeforeMessage)?
                    .answers
                    .push((id, kind, message)),
            }
        }
        gathered
            .into_iter()
            .map(|exchange| self.exchange(exchange, &feedback))
            .collect()
    }

    fn exchange(
        &self,
        gathered: Gathered<'_>,
        feedback: &impl Fn(&ResponseId) -> Option<ResponseFeedback>,
    ) -> Result<Exchange, TranscriptError> {
        let work = gathered.work;
        let run = self
            .runs
            .get(work)
            .and_then(RunHistory::current)
            .ok_or(TranscriptError::MissingRun)?;
        let attempts: Vec<&Attempt> = self
            .attempts
            .iter()
            .filter(|attempt| attempt.work == work)
            .collect();
        let current = work == self.work;
        let continuation = || {
            current.then(|| WorkResumeTarget {
                work_item_id: WorkItemId(work.to_owned()),
                expected_run_id: run.id.to_owned(),
            })
        };
        let answer = match run.outcome {
            None if current => AnswerState::Working {
                stop: WorkStopTarget {
                    work_item_id: WorkItemId(work.to_owned()),
                },
                writing: writing(&attempts),
            },
            None => return Err(TranscriptError::ReplacedWorkOpen),
            Some(RunOutcome::Completed) => {
                // The published answer is the one the run's last attempt saved.
                // Its text holds no labels.
                let attempt = attempts.last().ok_or(TranscriptError::MissingAnswer)?;
                let id = reply_id(&attempt.id);
                let message = gathered
                    .answers
                    .iter()
                    .find(|(saved, kind, _)| {
                        *saved == id && matches!(kind, AssistantKind::Finished(_))
                    })
                    .map(|(_, _, message)| *message)
                    .ok_or(TranscriptError::MissingAnswer)?;
                let id = ResponseId(id);
                AnswerState::Finished {
                    feedback: feedback(&id),
                    response_id: physician_wire::ResponseId(id.0),
                    attempt_id: AttemptId(attempt.id.clone()),
                    finished_at: wire_time(message.saved_at),
                    text: message.text.clone(),
                    segments: Vec::new(),
                    basis: None,
                    newer_records: false,
                }
            }
            Some(RunOutcome::Held {
                reason: HoldReason::Stopped,
            }) => AnswerState::Stopped {
                resume: continuation(),
            },
            Some(RunOutcome::Held {
                reason: HoldReason::Limit,
            }) => AnswerState::CutOff {
                try_again: continuation(),
            },
            Some(
                RunOutcome::Held {
                    reason: HoldReason::ProviderBlocked,
                }
                | RunOutcome::Failed,
            ) => AnswerState::CouldntFinish {
                try_again: continuation(),
            },
            Some(RunOutcome::Cancelled) => return Err(TranscriptError::Cancelled),
        };
        // Try again continues after what a limit kept, so every kept part
        // stays with the exchange.
        let kept = gathered
            .answers
            .iter()
            .filter(|(_, kind, _)| matches!(kind, AssistantKind::Kept))
            .map(|(_, _, message)| message.text.clone())
            .collect();
        Ok(Exchange {
            work_item_id: WorkItemId(work.to_owned()),
            messages: gathered.messages,
            steps: exchange_steps(work, &attempts),
            kept,
            answer,
        })
    }
}

/// The steps of one work, or unavailable when one of its saved calls cannot
/// be read as a step. The rest of the exchange still shows, and a partial
/// list would read as everything Coda did, so none are shown.
fn exchange_steps(work: &str, attempts: &[&Attempt]) -> ExchangeSteps {
    match steps(attempts) {
        Ok(steps) => ExchangeSteps::Read(steps),
        Err(error) => {
            tracing::error!(
                work,
                error = &error as &dyn std::error::Error,
                "a saved chat call does not read as a step; its exchange shows no steps"
            );
            ExchangeSteps::Unavailable
        }
    }
}

/// Whether Coda is writing the running work's answer: the work's latest
/// attempt still runs, and [`writing_after`] says so over that attempt's
/// saved events.
fn writing(attempts: &[&Attempt]) -> bool {
    let Some(attempt) = attempts.last() else {
        return false;
    };
    let writing = attempt
        .events
        .iter()
        .fold(false, |writing, event| writing_after(writing, &event.kind));
    writing && attempt.status == AttemptStatus::Active
}

/// Whether Coda is writing the answer after `event`, when it was `writing`
/// before it. Writing begins when a request begins the finish. It ends when
/// the next request starts, or when the request fails, is retried, is blocked
/// or its context is refused. The finish's return does not end it: its answer
/// is saved next, which ends the work, and a refused finish ends it when the
/// next request starts. The snapshot and the listener that tells an open page
/// both step by this rule.
pub(super) fn writing_after(writing: bool, event: &EventKind) -> bool {
    match event {
        EventKind::CallBegan { name } => writing || crate::physician_chat::hands_in_answer(name),
        EventKind::ProviderStarted { .. }
        | EventKind::ProviderFailed { .. }
        | EventKind::TransportRetry { .. }
        | EventKind::ProviderBlocked { .. }
        | EventKind::ContextRejected => false,
        EventKind::ProviderReturned(_)
        | EventKind::ProviderObservation { .. }
        | EventKind::ToolStarted(_)
        | EventKind::ToolReturned { .. }
        | EventKind::Candidate { .. }
        | EventKind::WindowOpened { .. } => writing,
    }
}

/// The steps of one work, in the order its attempts started the calls. A
/// call's result may be saved by a later attempt than the one that started
/// it; the latest saved result is the one the work reads. A call with no
/// saved result is running while its attempt runs, stopped when Stop ended
/// that attempt, and otherwise interrupted.
fn steps(attempts: &[&Attempt]) -> Result<Vec<Step>, StepError> {
    let mut started: Vec<(&Attempt, &Invocation)> = Vec::new();
    let mut returned: HashMap<&str, &ToolResult> = HashMap::new();
    for attempt in attempts {
        for event in &attempt.events {
            if let EventKind::ToolStarted(call) = &event.kind {
                if !started.iter().any(|(_, seen)| seen.id == call.id) {
                    started.push((attempt, call));
                }
            } else if let EventKind::ToolReturned { call_id, result } = &event.kind {
                returned.insert(call_id, result);
            }
        }
    }
    let mut steps = Vec::new();
    for (attempt, call) in started {
        let ending = match returned.get(call.id.as_str()) {
            Some(result) => CallEnding::Returned(result),
            None => match attempt.status {
                AttemptStatus::Active => CallEnding::Running,
                AttemptStatus::Cancelled => CallEnding::Stopped,
                AttemptStatus::Interrupted
                | AttemptStatus::Failed
                | AttemptStatus::Held
                | AttemptStatus::Completed => CallEnding::Interrupted,
            },
        };
        if let Some(step) = crate::physician_chat::step(call, ending)? {
            steps.push(step);
        }
    }
    Ok(steps)
}

#[cfg(test)]
mod tests {
    use harness::engine::results::ToolResult;
    use harness::engine::runs::{HoldReason, RunHistory, RunOutcome};
    use harness::engine::{Event, EventKind, Invocation};
    use physician_wire::{AnswerState, ExchangeSteps, Found, Step, StepState, WorkResumeTarget};

    use super::super::{
        AssistantKind, Attempt, AttemptStatus, Conversation, Message, MessageRole, SavedAt,
        WorkStatus, reply_id,
    };

    /// When [`ended`] saved its physician message and its answer.
    const ASKED: i64 = 1_794_600_000;
    const ANSWERED: i64 = ASKED + 42;

    /// The physician's `text` on `work`, saved `at` the database clock's seconds.
    fn ask(conversation: &mut Conversation, work: &str, text: &str, at: i64) {
        conversation.messages.push(Message {
            role: MessageRole::User {
                id: format!("key-{text}"),
                work: work.into(),
                revision: 1,
            },
            text: text.into(),
            saved_at: SavedAt(at),
        });
    }

    /// Coda's `text` from `attempt`, kept by a limit when `partial`, saved `at` the database
    /// clock's seconds.
    fn answer(conversation: &mut Conversation, attempt: &str, partial: bool, text: &str, at: i64) {
        let kind = if partial {
            AssistantKind::Kept
        } else {
            AssistantKind::Finished(
                serde_json::from_value(serde_json::json!({
                    "marks": [],
                    "basis": {"text": "Fixture basis.", "marks": []},
                    "descriptions": []
                }))
                .unwrap(),
            )
        };
        conversation.messages.push(Message {
            role: MessageRole::Assistant {
                id: reply_id(attempt),
                kind,
            },
            text: text.into(),
            saved_at: SavedAt(at),
        });
    }

    fn attempt(work: &str, id: &str, status: AttemptStatus, events: Vec<EventKind>) -> Attempt {
        Attempt {
            id: id.into(),
            predecessor: None,
            work: work.into(),
            revision: 1,
            status,
            expires: 0,
            events: events
                .into_iter()
                .zip(1..)
                .map(|(kind, sequence)| Event {
                    attempt_id: id.into(),
                    sequence,
                    ordinal: 0,
                    kind,
                })
                .collect(),
        }
    }

    /// A conversation whose work `work-1` ran once as `attempt-1` and ended
    /// with `outcome`.
    fn ended(outcome: RunOutcome) -> Conversation {
        let mut conversation = Conversation::new(
            serde_json::from_value(serde_json::json!("fixture-package")).unwrap(),
        );
        ask(&mut conversation, "work-1", "Hello", ASKED);
        let mut runs = RunHistory::default();
        runs.start("run-1".into()).unwrap();
        runs.end(outcome).unwrap();
        conversation.runs.insert("work-1".into(), runs);
        conversation.work = "work-1".into();
        conversation.status = outcome.into();
        conversation.attempts = vec![attempt(
            "work-1",
            "attempt-1",
            AttemptStatus::Completed,
            vec![],
        )];
        match outcome {
            RunOutcome::Completed => {
                answer(&mut conversation, "attempt-1", false, "Done.", ANSWERED);
            }
            RunOutcome::Held {
                reason: HoldReason::Limit,
            } => answer(
                &mut conversation,
                "attempt-1",
                true,
                "The saved part.",
                ANSWERED,
            ),
            RunOutcome::Held { .. } | RunOutcome::Failed | RunOutcome::Cancelled => {}
        }
        conversation
    }

    /// Every ending reads the same whether its work is current or replaced;
    /// only the current work offers Resume or Try again, naming its run.
    #[test]
    fn a_replaced_answer_keeps_its_ending_without_resume_or_try_again() {
        let held = |reason| RunOutcome::Held { reason };
        for outcome in [
            RunOutcome::Completed,
            held(HoldReason::Stopped),
            held(HoldReason::Limit),
            held(HoldReason::ProviderBlocked),
            RunOutcome::Failed,
        ] {
            let current = ended(outcome);
            let mut replaced = current.clone();
            ask(&mut replaced, "work-2", "Something else", ANSWERED + 60);
            let mut runs = RunHistory::default();
            runs.start("run-2".into()).unwrap();
            replaced.runs.insert("work-2".into(), runs);
            replaced.work = "work-2".into();
            replaced.status = WorkStatus::Pending;
            let expected = |continuation: Option<WorkResumeTarget>| match outcome {
                RunOutcome::Completed => AnswerState::Finished {
                    response_id: physician_wire::ResponseId(reply_id("attempt-1")),
                    attempt_id: physician_wire::AttemptId("attempt-1".into()),
                    finished_at: physician_wire::SavedAt(ANSWERED),
                    text: "Done.".into(),
                    feedback: None,
                    segments: Vec::new(),
                    basis: None,
                    newer_records: false,
                },
                RunOutcome::Held {
                    reason: HoldReason::Stopped,
                } => AnswerState::Stopped {
                    resume: continuation,
                },
                RunOutcome::Held {
                    reason: HoldReason::Limit,
                } => AnswerState::CutOff {
                    try_again: continuation,
                },
                RunOutcome::Held {
                    reason: HoldReason::ProviderBlocked,
                }
                | RunOutcome::Failed => AnswerState::CouldntFinish {
                    try_again: continuation,
                },
                RunOutcome::Cancelled => unreachable!("not in the list"),
            };
            let target = WorkResumeTarget {
                work_item_id: physician_wire::WorkItemId("work-1".into()),
                expected_run_id: "run-1".into(),
            };
            let continues = !matches!(outcome, RunOutcome::Completed);
            let now = current.transcript(|_| None).unwrap();
            assert_eq!(
                now[0].answer,
                expected(continues.then_some(target)),
                "{outcome:?} as the current work"
            );
            let kept = match outcome {
                RunOutcome::Held {
                    reason: HoldReason::Limit,
                } => vec!["The saved part.".to_owned()],
                RunOutcome::Completed
                | RunOutcome::Held { .. }
                | RunOutcome::Failed
                | RunOutcome::Cancelled => vec![],
            };
            assert_eq!(now[0].kept, kept, "{outcome:?} as the current work");
            let later = replaced.transcript(|_| None).unwrap();
            assert_eq!(later.len(), 2, "{outcome:?}");
            assert_eq!(
                later[0].answer,
                expected(None),
                "{outcome:?} after a new message"
            );
            assert_eq!(later[0].kept, kept, "{outcome:?} after a new message");
            assert!(
                matches!(later[1].answer, AnswerState::Working { .. }),
                "{outcome:?}"
            );
        }
    }

    /// Try again after a cut-off answer starts a new run that continues after
    /// the kept text. If that run is cut off too with nothing kept, the first
    /// run's kept text still shows, and stays when a third run finishes.
    #[test]
    fn kept_text_stays_with_its_exchange_across_try_again() {
        let limit = RunOutcome::Held {
            reason: HoldReason::Limit,
        };
        let mut conversation = ended(limit);
        let runs = conversation.runs.get_mut("work-1").unwrap();
        runs.start("run-2".into()).unwrap();
        runs.end(limit).unwrap();
        conversation
            .attempts
            .push(attempt("work-1", "attempt-2", AttemptStatus::Held, vec![]));
        let transcript = conversation.transcript(|_| None).unwrap();
        assert_eq!(transcript[0].kept, ["The saved part."]);
        assert_eq!(
            transcript[0].answer,
            AnswerState::CutOff {
                try_again: Some(WorkResumeTarget {
                    work_item_id: physician_wire::WorkItemId("work-1".into()),
                    expected_run_id: "run-2".into(),
                }),
            }
        );

        let runs = conversation.runs.get_mut("work-1").unwrap();
        runs.start("run-3".into()).unwrap();
        runs.end(RunOutcome::Completed).unwrap();
        conversation.attempts.push(attempt(
            "work-1",
            "attempt-3",
            AttemptStatus::Completed,
            vec![],
        ));
        answer(
            &mut conversation,
            "attempt-3",
            false,
            "The rest.",
            ANSWERED + 120,
        );
        let transcript = conversation.transcript(|_| None).unwrap();
        assert_eq!(transcript[0].kept, ["The saved part."]);
        assert_eq!(
            transcript[0].answer,
            AnswerState::Finished {
                response_id: physician_wire::ResponseId(reply_id("attempt-3")),
                attempt_id: physician_wire::AttemptId("attempt-3".into()),
                finished_at: physician_wire::SavedAt(ANSWERED + 120),
                text: "The rest.".into(),
                feedback: None,
                segments: Vec::new(),
                basis: None,
                newer_records: false,
            }
        );
    }

    /// A call without a saved result is running only while its attempt runs.
    /// Stop makes it stopped; any other end of its attempt, such as a
    /// restart, leaves how it ended unknown. A result a later attempt saved
    /// still ends the call.
    #[test]
    fn a_call_without_a_result_reads_by_how_its_attempt_ended() {
        let found = |id: &str| EventKind::ToolReturned {
            call_id: id.into(),
            result: ToolResult::Original(r#"{"results":[{}, {}],"next":null}"#.into()),
        };
        let mut conversation = ended(RunOutcome::Completed);
        conversation.attempts = vec![
            attempt(
                "work-1",
                "attempt-0",
                AttemptStatus::Interrupted,
                vec![search("lost"), search("recovered")],
            ),
            attempt(
                "work-1",
                "attempt-1",
                AttemptStatus::Cancelled,
                vec![found("recovered"), search("stopped")],
            ),
            attempt(
                "work-1",
                "attempt-2",
                AttemptStatus::Active,
                vec![search("running")],
            ),
        ];
        conversation.messages.truncate(1);
        let mut runs = RunHistory::default();
        runs.start("run-1".into()).unwrap();
        conversation.runs.insert("work-1".into(), runs);
        conversation.status = WorkStatus::Active;
        let transcript = conversation.transcript(|_| None).unwrap();
        let states: Vec<_> = read(&transcript[0].steps)
            .iter()
            .map(|step| match step {
                Step::SearchedChart { state, .. } => state.clone(),
                Step::ReadRecords { .. }
                | Step::ReadConversation { .. }
                | Step::SavedMemoryNote { .. }
                | Step::EditedMemory { .. } => {
                    panic!("only searches started: {step:?}")
                }
            })
            .collect();
        assert_eq!(
            states,
            [
                StepState::Interrupted,
                StepState::Returned {
                    result: Found {
                        records: 2,
                        more: false
                    }
                },
                StepState::Stopped,
                StepState::Running,
            ]
        );
    }

    /// One saved event, as `attempt-1` saves it.
    fn saved(kind: EventKind) -> Event {
        Event {
            attempt_id: "attempt-1".into(),
            sequence: 1,
            ordinal: 0,
            kind,
        }
    }

    /// What the snapshot says of writing the answer when `attempt-1` has `status` and saved
    /// `events`, after an earlier attempt that began the finish was interrupted.
    fn snapshot_writing(status: AttemptStatus, events: &[EventKind]) -> bool {
        let mut conversation = ended(RunOutcome::Completed);
        conversation.messages.truncate(1);
        let mut runs = RunHistory::default();
        runs.start("run-1".into()).unwrap();
        conversation.runs.insert("work-1".into(), runs);
        conversation.status = WorkStatus::Active;
        conversation.attempts = vec![
            attempt(
                "work-1",
                "attempt-0",
                AttemptStatus::Interrupted,
                vec![
                    writing_request(),
                    EventKind::CallBegan {
                        name: crate::physician_sources::citations::FINISH_TOOL.into(),
                    },
                ],
            ),
            attempt("work-1", "attempt-1", status, events.to_vec()),
        ];
        match &conversation.transcript(|_| None).unwrap()[0].answer {
            AnswerState::Working { writing, .. } => *writing,
            other => panic!("a working answer: {other:?}"),
        }
    }

    fn writing_request() -> EventKind {
        EventKind::ProviderStarted {
            dispatched_at_unix_ms: 1,
            context: Default::default(),
            request: harness::engine::SavedRequest {
                added: vec![],
                items: vec![],
            },
        }
    }

    /// The event lists both readers of writing the answer are held to: the snapshot, and the
    /// listener that tells an open page. Each row is its case, its attempt's status, its saved
    /// events, and whether the snapshot then says Coda is writing.
    fn writing_cases() -> Vec<(&'static str, AttemptStatus, Vec<EventKind>, bool)> {
        let started = writing_request;
        let began = |name: &str| EventKind::CallBegan { name: name.into() };
        let finish = crate::physician_sources::citations::FINISH_TOOL;
        let search = crate::physician_sources::SEARCH_TOOL;
        let returned = || {
            EventKind::ProviderReturned(harness::engine::Turn {
                items: vec![],
                receipt: serde_json::Value::Null,
                usage: Default::default(),
            })
        };
        let failed = || EventKind::ProviderFailed {
            reason: "the stream broke".into(),
        };
        let active = AttemptStatus::Active;
        vec![
            ("nothing sent yet", active, vec![], false),
            (
                "a search began",
                active,
                vec![started(), began(search)],
                false,
            ),
            (
                "the finish began",
                active,
                vec![started(), began(finish)],
                true,
            ),
            (
                "the finish was announced twice in one request",
                active,
                vec![started(), began(finish), began(finish)],
                true,
            ),
            (
                "a search, then the finish, began",
                active,
                vec![started(), began(search), began(finish)],
                true,
            ),
            (
                "the finish returned and its answer is not saved yet",
                active,
                vec![started(), began(finish), returned()],
                true,
            ),
            (
                "the finish was refused and Coda is asked again",
                active,
                vec![started(), began(finish), returned(), started()],
                false,
            ),
            (
                "the finish began again after a refusal",
                active,
                vec![
                    started(),
                    began(finish),
                    returned(),
                    started(),
                    began(finish),
                ],
                true,
            ),
            (
                "the request failed",
                active,
                vec![started(), began(finish), failed()],
                false,
            ),
            (
                "the attempt ended",
                AttemptStatus::Interrupted,
                vec![started(), began(finish)],
                false,
            ),
        ]
    }

    /// The working answer says Coda is writing it from the moment the latest request began the
    /// finish, through that request's return, until the next request starts or the request
    /// fails, and only while its attempt runs. A call that is not the finish never makes it
    /// writing. A refused finish ends it when the next request starts; a saved answer ends the
    /// work.
    #[test]
    fn the_working_answer_is_writing_from_the_finish_until_the_next_request() {
        for (case, status, events, expected) in writing_cases() {
            assert_eq!(snapshot_writing(status, &events), expected, "{case}");
        }
    }

    /// An open page is told of writing the answer exactly when the snapshot's value changes: over
    /// the same event lists, each saved event tells the listener if and only if the snapshot read
    /// after it says otherwise than the snapshot read before it.
    #[test]
    fn the_page_is_told_exactly_when_the_snapshot_says_otherwise_of_writing() {
        for (case, _, events, _) in writing_cases() {
            let mut shown = super::super::ShownSteps::new(());
            for (at, kind) in events.iter().enumerate() {
                let before = snapshot_writing(AttemptStatus::Active, &events[..at]);
                let after = snapshot_writing(AttemptStatus::Active, &events[..=at]);
                assert_eq!(
                    shown.shown(&saved(kind.clone())),
                    before != after,
                    "{case}, event {at}"
                );
            }
        }
    }

    /// Each physician message carries when the server saved it, and a finished answer when the
    /// server saved that answer: its own message's time, never the time of a part a limit kept
    /// before it or of the message that asked. An ending that is not a finished answer carries no
    /// time.
    #[test]
    fn the_snapshot_carries_when_each_message_and_finished_answer_were_saved() {
        let conversation = two_finished_exchanges(vec![]);
        let transcript = conversation.transcript(|_| None).unwrap();
        let times: Vec<_> = transcript
            .iter()
            .map(|exchange| {
                let AnswerState::Finished { finished_at, .. } = &exchange.answer else {
                    panic!("a finished answer: {:?}", exchange.answer)
                };
                let asked: Vec<_> = exchange
                    .messages
                    .iter()
                    .map(|message| message.saved_at)
                    .collect();
                (asked, *finished_at)
            })
            .collect();
        let at = physician_wire::SavedAt;
        assert_eq!(
            times,
            [
                (vec![at(ASKED)], at(ANSWERED)),
                (vec![at(ANSWERED + 60)], at(ANSWERED + 90)),
            ]
        );
        assert_eq!(
            serde_json::to_value(&transcript[1].messages[0]).unwrap()["saved_at"],
            serde_json::json!(ANSWERED + 60),
            "the wire carries the time as a number of seconds"
        );

        let stopped = ended(RunOutcome::Held {
            reason: HoldReason::Stopped,
        })
        .transcript(|_| None)
        .unwrap();
        assert_eq!(stopped[0].messages[0].saved_at, at(ASKED));
        let ending = serde_json::to_value(&stopped[0].answer).unwrap();
        assert_eq!(
            ending,
            serde_json::json!({"state": "stopped", "resume": {
                "work_item_id": "work-1", "expected_run_id": "run-1"}}),
            "a stopped answer carries no time"
        );
    }

    fn read(steps: &ExchangeSteps) -> &[Step] {
        match steps {
            ExchangeSteps::Read(steps) => steps,
            ExchangeSteps::Unavailable => panic!("the steps were unavailable"),
        }
    }

    fn search(id: &str) -> EventKind {
        EventKind::ToolStarted(Invocation {
            id: id.into(),
            name: crate::physician_sources::SEARCH_TOOL.into(),
            arguments: r#"{"query":"sodium","record_type":null,"after":null}"#.into(),
        })
    }

    /// `work-1` kept a cut-off part and finished with `unreadable` saved in
    /// its attempt, and a later `work-2` finished after one readable search.
    fn two_finished_exchanges(unreadable: Vec<EventKind>) -> Conversation {
        let mut conversation = ended(RunOutcome::Completed);
        conversation.attempts[0] =
            attempt("work-1", "attempt-1", AttemptStatus::Completed, unreadable);
        answer(
            &mut conversation,
            "attempt-0",
            true,
            "Kept before the end.",
            ANSWERED - 20,
        );
        ask(&mut conversation, "work-2", "And potassium?", ANSWERED + 60);
        let mut runs = RunHistory::default();
        runs.start("run-2".into()).unwrap();
        runs.end(RunOutcome::Completed).unwrap();
        conversation.runs.insert("work-2".into(), runs);
        conversation.work = "work-2".into();
        conversation.attempts.push(attempt(
            "work-2",
            "attempt-2",
            AttemptStatus::Completed,
            vec![
                search("found"),
                EventKind::ToolReturned {
                    call_id: "found".into(),
                    result: ToolResult::Original(r#"{"results":[{}],"next":null}"#.into()),
                },
            ],
        ));
        answer(
            &mut conversation,
            "attempt-2",
            false,
            "Potassium is 4.1.",
            ANSWERED + 90,
        );
        conversation
    }

    /// A saved call that cannot be read as a step marks only its own
    /// exchange's steps unavailable. The exchange keeps its messages, kept
    /// text and answer, and its neighbour still shows its steps.
    #[test]
    fn an_unreadable_call_marks_only_its_exchange_steps_unavailable() {
        let unknown_tool = vec![EventKind::ToolStarted(Invocation {
            id: "gone".into(),
            name: "a_tool_chat_never_offered".into(),
            arguments: "{}".into(),
        })];
        let not_its_receipt = vec![
            search("mangled"),
            EventKind::ToolReturned {
                call_id: "mangled".into(),
                result: ToolResult::Original("not a search receipt".into()),
            },
        ];
        for (case, unreadable) in [
            ("unknown tool", unknown_tool),
            ("result is not the receipt", not_its_receipt),
        ] {
            let conversation = two_finished_exchanges(unreadable);
            let transcript = conversation
                .transcript(|_| None)
                .unwrap_or_else(|error| panic!("{case}: the transcript failed: {error}"));
            let first = &transcript[0];
            assert_eq!(first.steps, ExchangeSteps::Unavailable, "{case}");
            assert_eq!(first.messages[0].text, "Hello", "{case}");
            assert_eq!(first.kept, ["Kept before the end."], "{case}");
            assert_eq!(
                first.answer,
                AnswerState::Finished {
                    response_id: physician_wire::ResponseId(reply_id("attempt-1")),
                    attempt_id: physician_wire::AttemptId("attempt-1".into()),
                    finished_at: physician_wire::SavedAt(ANSWERED),
                    text: "Done.".into(),
                    feedback: None,
                    segments: Vec::new(),
                    basis: None,
                    newer_records: false,
                },
                "{case}"
            );
            assert_eq!(
                read(&transcript[1].steps),
                [Step::SearchedChart {
                    query: "sodium".into(),
                    group: None,
                    page: physician_wire::SearchPage::First,
                    state: StepState::Returned {
                        result: Found {
                            records: 1,
                            more: false
                        }
                    },
                }],
                "{case}"
            );
        }
    }
}
