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
    AnswerState, Exchange, ExchangeSteps, PhysicianMessage, ResponseFeedback, Step,
    WorkResumeTarget, WorkStopTarget,
};

use super::{Attempt, AttemptStatus, Conversation, MessageRole, ResponseId, reply_id};
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
    /// Each answer's saved id, whether it is partial, and its text.
    answers: Vec<(&'a str, bool, &'a str)>,
}

impl Conversation {
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
                        id: id.clone(),
                        text: message.text.clone(),
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
                MessageRole::Assistant { id, partial } => gathered
                    .last_mut()
                    .ok_or(TranscriptError::AnswerBeforeMessage)?
                    .answers
                    .push((id, *partial, &message.text)),
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
                work_item_id: work.to_owned(),
                expected_run_id: run.id.to_owned(),
            })
        };
        let answer = match run.outcome {
            None if current => AnswerState::Working {
                stop: WorkStopTarget {
                    work_item_id: work.to_owned(),
                },
            },
            None => return Err(TranscriptError::ReplacedWorkOpen),
            Some(RunOutcome::Completed) => {
                // The published answer is the one the run's last attempt saved.
                let attempt = attempts.last().ok_or(TranscriptError::MissingAnswer)?;
                let id = reply_id(&attempt.id);
                let text = gathered
                    .answers
                    .iter()
                    .find(|(saved, partial, _)| *saved == id && !partial)
                    .map(|(_, _, text)| *text)
                    .ok_or(TranscriptError::MissingAnswer)?;
                let id = ResponseId(id);
                AnswerState::Finished {
                    feedback: feedback(&id),
                    response_id: physician_wire::ResponseId(id.0),
                    attempt_id: attempt.id.clone(),
                    text: text.to_owned(),
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
            .filter(|(_, partial, _)| *partial)
            .map(|(_, _, text)| (*text).to_owned())
            .collect();
        Ok(Exchange {
            work_item_id: work.to_owned(),
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
        Attempt, AttemptStatus, Conversation, Message, MessageRole, WorkStatus, reply_id,
    };

    fn ask(conversation: &mut Conversation, work: &str, text: &str) {
        conversation.messages.push(Message {
            role: MessageRole::User {
                id: format!("key-{text}"),
                work: work.into(),
                revision: 1,
            },
            text: text.into(),
        });
    }

    fn answer(conversation: &mut Conversation, attempt: &str, partial: bool, text: &str) {
        conversation.messages.push(Message {
            role: MessageRole::Assistant {
                id: reply_id(attempt),
                partial,
            },
            text: text.into(),
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
        ask(&mut conversation, "work-1", "Hello");
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
            RunOutcome::Completed => answer(&mut conversation, "attempt-1", false, "Done."),
            RunOutcome::Held {
                reason: HoldReason::Limit,
            } => answer(&mut conversation, "attempt-1", true, "The saved part."),
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
            ask(&mut replaced, "work-2", "Something else");
            let mut runs = RunHistory::default();
            runs.start("run-2".into()).unwrap();
            replaced.runs.insert("work-2".into(), runs);
            replaced.work = "work-2".into();
            replaced.status = WorkStatus::Pending;
            let expected = |continuation: Option<WorkResumeTarget>| match outcome {
                RunOutcome::Completed => AnswerState::Finished {
                    response_id: physician_wire::ResponseId(reply_id("attempt-1")),
                    attempt_id: "attempt-1".into(),
                    text: "Done.".into(),
                    feedback: None,
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
                work_item_id: "work-1".into(),
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
                    work_item_id: "work-1".into(),
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
        answer(&mut conversation, "attempt-3", false, "The rest.");
        let transcript = conversation.transcript(|_| None).unwrap();
        assert_eq!(transcript[0].kept, ["The saved part."]);
        assert_eq!(
            transcript[0].answer,
            AnswerState::Finished {
                response_id: physician_wire::ResponseId(reply_id("attempt-3")),
                attempt_id: "attempt-3".into(),
                text: "The rest.".into(),
                feedback: None,
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
                Step::ReadRecords { .. } | Step::ReadConversation { .. } => {
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
        answer(&mut conversation, "attempt-0", true, "Kept before the end.");
        ask(&mut conversation, "work-2", "And potassium?");
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
        answer(&mut conversation, "attempt-2", false, "Potassium is 4.1.");
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
                    attempt_id: "attempt-1".into(),
                    text: "Done.".into(),
                    feedback: None,
                },
                "{case}"
            );
            assert_eq!(
                read(&transcript[1].steps),
                [Step::SearchedChart {
                    query: "sodium".into(),
                    record_type: None,
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
