//! Fenced reply-revision execution. The draft row owns all evidence and text;
//! no chat row, effect tool, or projection is an authority for this work.
use super::WorkerAuthority;
use super::attempt::{self, Claimed, Failure, Outcome, Push, Tally};
use super::{Hosted, Worker, absent_claim, announce};
use async_trait::async_trait;
use axum::http::StatusCode;
use harness::engine::recipe::Finish;
use harness::engine::{self, Cancellation, Cause, Event, Invocation, Product, Turn};
use local_product::physician_sources::{self, read_sources};
use local_product::runtime::Store;
use local_product::runtime::delivery::Delivery;
use local_product::runtime::reply::{
    RevisionClaim, RevisionCommit, requests_finish, revised_text, revision_feedback,
};
use serde_json::{Value, json};
use telemetry::genai::{self, RunContext};
use tracing::Instrument;

const JOB: &str = "physician_reply_revision";

/// Handles one revision delivery inside its own `physician.worker.attempt`.
pub(super) async fn deliver(worker: &Worker, push: &Push) -> StatusCode {
    let span = attempt::span(push, JOB);
    handle(worker, &push.delivery, &span)
        .instrument(span.clone())
        .await
}

async fn handle(worker: &Worker, delivery: &Delivery, span: &tracing::Span) -> StatusCode {
    let claim = match worker
        .store
        .claim_revision(delivery, super::LEASE_SECONDS)
        .await
    {
        Ok(Some(claim)) => claim,
        Ok(None) => {
            attempt::finish(span, JOB, None, Outcome::Absent, None);
            return absent_claim(worker, delivery).await;
        }
        Err(_) => {
            attempt::finish(span, JOB, None, Outcome::ClaimError, Some(Failure::Claim));
            return StatusCode::INTERNAL_SERVER_ERROR;
        }
    };
    let (origin, scenario_id) = worker.turn_origin(
        &claim.scope.practice,
        claim.accepted.input.scenario.as_deref(),
    );
    let generation = genai::generation_span(
        &RunContext {
            run_id: claim.attempt.clone(),
            job: JOB,
            recipe_version: genai::recipe_version(),
            origin,
            scenario_id,
            session_id: Some(claim.scope.chat.clone()),
        },
        "openai",
        &worker.model.roles.background.adapter,
    );
    genai::record_input(&generation, &claim.accepted.input.instruction);
    let mut owner = RevisionOwner {
        store: worker.store.clone(),
        authority: worker.model.authority(worker.store.clone()),
        claim,
        usage: Tally::default(),
    };
    let cancel = Cancellation::default();
    let claimed = owner.claim.clone();
    let mut failure = Failure::Revision;
    let run = async {
        // Fail closed: an unreadable draft must not reset the call budget.
        let draft = worker
            .store
            .draft(&owner.claim.scope)
            .await
            .map_err(|error| engine::Failure::Host(error.into()))?
            .ok_or_else(|| engine::Failure::Host("revision draft unavailable".into()))?;
        // The revision's scope chat names its reply work; it has no recorded
        // predecessor attempt, so none is reported.
        let earlier = draft
            .attempts
            .iter()
            .filter(|saved| {
                saved.input_id == owner.claim.accepted.input.key && saved.id != owner.claim.attempt
            })
            .count();
        attempt::record_claim(
            span,
            &Claimed {
                run_id: &owner.claim.attempt,
                work_id: Some(&owner.claim.scope.chat),
                predecessor_run_id: None,
                input_revision: None,
                attempt_number: u32::try_from(earlier).unwrap_or(u32::MAX).saturating_add(1),
            },
        );
        let messages = worker
            .store
            .revision_message_sources(&owner.claim.scope, &draft)
            .await
            .map_err(|error| engine::Failure::Host(error.into()))?;
        let records = worker
            .store
            .revision_records(&owner.claim.scope, &draft)
            .await
            .map_err(|error| engine::Failure::Host(error.into()))?;
        let work = owner
            .claim
            .binding
            .request(context(&owner.claim).items(&owner.claim.sources))
            .freeze()?
            .recover(
                &owner.claim.attempt,
                engine::recovery::Records {
                    events: records.iter().map(Into::into).collect(),
                    allowance_after: draft.allowance_after,
                },
                engine::recovery::Messages {
                    sources: &messages,
                    partial: None,
                },
                engine::recovery::Inputs {
                    accepted: Vec::new(),
                    // Revisions freeze a chart snapshot and retain each refresh
                    // with the attempt. Recovery must replay that saved refresh.
                    // Duet instead checks live evidence/workspace authority at
                    // tools and publication; it has no accepted chart snapshot.
                    source_feedback: draft
                        .attempts
                        .last()
                        .and_then(|attempt| attempt.refreshes.last())
                        .map(|refresh| refresh.result(&owner.claim.sources)),
                },
            )?;
        drop(draft);
        let mut provider = worker
            .model
            .provider(worker.quota.clone())
            .await
            .map_err(|e| {
                failure = Failure::ProviderInit;
                engine::Failure::Provider(e)
            })?;
        work.run(
            &mut provider,
            &mut owner,
            engine::RunLimits::default(),
            &cancel,
        )
        .await
    };
    let hosted = super::while_owned(
        claimed.lease,
        super::STALL,
        &cancel,
        || worker.store.renew_revision_claim(&claimed),
        run.instrument(generation.clone()),
    )
    .await;
    if let Some((input, output)) = owner.usage.known() {
        genai::record_usage(&generation, input, output);
    }
    let result = match hosted {
        Hosted::Returned(result) => result,
        Hosted::Lost => return super::lost(span, JOB, &owner.claim.attempt),
    };
    let run_id = Some(owner.claim.attempt.as_str());
    let status = match result {
        Ok(response) => {
            // Completion has already atomically compared its retained exchange
            // and committed under the real session's publication locks.
            if let Ok(text) = revised_text(&response) {
                genai::record_output(&generation, &text);
            }
            attempt::finish(span, JOB, run_id, Outcome::Committed, None);
            StatusCode::NO_CONTENT
        }
        Err(error) => {
            let reason = engine::AttemptFailure::from_error(&error, &cancel);
            cancel.cancel();
            use local_product::runtime::delivery::AttemptSettlement;
            let settlement = worker
                .store
                .settle_revision_attempt(&owner.claim, reason)
                .await;
            match settlement {
                Err(_) => {
                    attempt::retry_not_persisted(span, JOB, &owner.claim.attempt);
                    attempt::finish(span, JOB, run_id, Outcome::Retry, Some(failure));
                    StatusCode::INTERNAL_SERVER_ERROR
                }
                Ok(AttemptSettlement::Applied) if reason.counts_as_failure() => {
                    genai::record_error(&generation, "physician reply revision failed");
                    attempt::finish(span, JOB, run_id, Outcome::Retry, Some(failure));
                    StatusCode::INTERNAL_SERVER_ERROR
                }
                Ok(AttemptSettlement::Applied | AttemptSettlement::Superseded) => {
                    attempt::finish(span, JOB, run_id, Outcome::Settled, None);
                    StatusCode::NO_CONTENT
                }
            }
        }
    };
    announce(worker, owner.claim.scope.workspace.clone()).await;
    status
}

fn context(claim: &RevisionClaim) -> local_product::runtime::reply::RevisionContext {
    local_product::runtime::reply::RevisionContext {
        saved_draft: claim.base_text.clone(),
        instruction: claim.accepted.input.message(),
        expected_draft_revision: claim.accepted.input.expected_revision,
        physician_time_zone: claim.accepted.input.timezone.clone(),
    }
}

struct RevisionOwner {
    store: Store,
    claim: RevisionClaim,
    authority: WorkerAuthority,
    /// Provider-reported usage of this attempt's retained returns.
    usage: Tally,
}

#[async_trait]
impl Product for RevisionOwner {
    async fn message_sources(
        &mut self,
    ) -> Result<Option<Vec<harness::engine::ProductMessage>>, harness::engine::Cause> {
        let draft = self
            .store
            .draft(&self.claim.scope)
            .await?
            .ok_or("revision draft unavailable")?;
        Ok(Some(
            self.store
                .revision_message_sources(&self.claim.scope, &draft)
                .await?,
        ))
    }

    async fn admit_binding(
        &mut self,
        recipe: &engine::recipe::RequestBinding,
    ) -> Result<(), Cause> {
        self.assert_current().await?;
        self.store
            .admit_revision_binding(&self.claim, recipe)
            .await?;
        Ok(())
    }

    async fn window_seed(&mut self) -> Result<engine::WindowSeed, Cause> {
        self.assert_current().await?;
        let draft = self
            .store
            .draft(&self.claim.scope)
            .await?
            .ok_or("revision draft disappeared")?;
        let note_references = local_product::runtime::working_notes::references(
            draft.attempts.iter().flat_map(|attempt| {
                attempt
                    .events
                    .iter()
                    .map(|event| (attempt.input_id.as_str(), event))
            }),
        )?;
        Ok(engine::WindowSeed {
            items: context(&self.claim).items(&self.claim.sources),
            note_references,
        })
    }
    fn read_only(&self, name: &str) -> bool {
        local_product::physician_chat::read_only(name)
    }

    fn read_result(&mut self, id: &str) -> harness::engine::recipe::ToolFuture<Option<String>> {
        let store = self.store.clone();
        let synthetic_only = self.authority.synthetic_only;
        let claim = self.claim.clone();
        let id = id.to_owned();
        Box::pin(async move {
            let mut authority = WorkerAuthority {
                inner: super::RuntimeAuthority,
                store: store.clone(),
                synthetic_only,
            };
            store
                .authorize_revision(&claim.scope, &mut authority)
                .await?;
            store.assert_revision_current(&claim).await?;
            let draft = store
                .draft(&claim.scope)
                .await?
                .ok_or("revision draft unavailable")?;
            let records = store.revision_records(&claim.scope, &draft).await?;
            let messages = store.revision_message_sources(&claim.scope, &draft).await?;
            engine::results::resolve(records.iter(), &id, &messages).map_err(Into::into)
        })
    }

    fn terminal(&self, response: &Turn) -> bool {
        // Mixed terminal/effect responses also route to completion, which
        // rejects them as a whole, without executing any of their calls.
        requests_finish(response)
    }

    async fn assert_current(&mut self) -> Result<(), Cause> {
        // The revision supplies its own accepted session reference to the
        // same read-only auth owner used by chat.
        self.store
            .authorize_revision(&self.claim.scope, &mut self.authority)
            .await?;
        self.store.assert_revision_current(&self.claim).await
    }

    async fn persist(&mut self, event: Event) -> Result<(), Cause> {
        self.usage.event(&event);
        self.store
            .record_revision(&self.claim, event)
            .await
            .map_err(local_product::runtime::persistence_failure)
    }

    fn execute(
        &mut self,
        call: &Invocation,
        arguments: Value,
    ) -> harness::engine::recipe::ToolFuture {
        let store = self.store.clone();
        let synthetic_only = self.authority.synthetic_only;
        let claim = self.claim.clone();
        let call = call.clone();
        Box::pin(async move {
            let mut authority = WorkerAuthority {
                inner: super::RuntimeAuthority,
                store: store.clone(),
                synthetic_only,
            };
            store
                .authorize_revision(&claim.scope, &mut authority)
                .await?;
            store.assert_revision_current(&claim).await?;
            let draft = store
                .draft(&claim.scope)
                .await?
                .ok_or("revision draft unavailable")?;
            let work_records = store.revision_records(&claim.scope, &draft).await?;
            let call = Invocation {
                id: call.id.clone(),
                name: call.name.clone(),
                arguments,
            };
            let records = draft.attempts.iter().flat_map(|attempt| {
                attempt
                    .events
                    .iter()
                    .map(|event| (attempt.input_id.as_str(), event))
            });
            if let Some(outcome) = local_product::runtime::working_notes::execute(
                &claim.accepted.input.key,
                &call,
                records,
            )? {
                return Ok(outcome);
            }
            if engine::history::is_tool(&call.name) {
                let messages = store.revision_messages(&claim.scope, &draft).await?;
                return Ok(
                    match engine::history::execute(
                        &call.name,
                        &call.arguments,
                        &messages,
                        work_records.iter(),
                    ) {
                        Ok(page) => engine::ToolOutcome::History(
                            page.map_metadata(|metadata| json!(metadata)),
                        ),
                        Err(reason) => engine::ToolOutcome::refused(reason),
                    },
                );
            }
            if call.name == harness::skills::SKILL_VIEW {
                return Ok(claim.skills.execute(&call.name, &call.arguments));
            }
            read_sources(&claim.sources, &claim.scope.workspace, &call)
        })
    }

    /// A finish on an older reading of the chart publishes nothing. The
    /// attempt re-pins and the model gets what changed, reads it, and
    /// finishes again under the revision's same call budget.
    async fn finish(&mut self, response: &Turn) -> Result<Finish, Cause> {
        if let Err(reason) = revised_text(response) {
            return Ok(revision_feedback(reason));
        }
        match self
            .store
            .commit_revision(&self.claim, response, &mut self.authority)
            .await?
        {
            RevisionCommit::Committed => Ok(Finish::Committed),
            RevisionCommit::SourcesMoved { refresh, sources } => {
                self.claim.sources = sources;
                self.claim.sources_version = refresh.to_version;
                Ok(Finish::Continue(refresh.result(&self.claim.sources)))
            }
            RevisionCommit::SourcesUnread(references) => Ok(Finish::Continue(
                physician_sources::unread_result(&references),
            )),
        }
    }

    async fn finish_partial(
        &mut self,
        response: &Turn,
        _backstops: &[engine::LimitBackstop],
    ) -> Result<(), Cause> {
        self.store.hold_revision(&self.claim, response).await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use harness::engine::Item;
    use harness::engine::ToolOutcome;
    use harness::engine::recipe::Usage;
    use local_product::physician_sources::READ_TOOL;
    use local_product::runtime::reply::FINISH_TOOL;

    #[test]
    fn exact_reads_are_pinned_workspace_scoped_and_refuse_effects() {
        let source = physician_sources::SourceRevision {
            workspace: "w".into(),
            source_id: "ehr".into(),
            candidate_id: "pinned".into(),
            revision: "v1".into(),
            record_type: "note".into(),
            dates: vec![],
            content: "hello".into(),
            sha256: "2cf24dba5fb0a30e26e83b2ac5b9e29e1b161e5c1fa7425e73043362938b9824".into(),
            readable_parts: vec![],
            provenance: json!({"owner": "test-source"}),
            native: None,
        };
        let mut call = Invocation {
            id: "read".into(),
            name: READ_TOOL.into(),
            arguments: json!({"references":["pinned", "missing"], "max_result_bytes":4096}),
        };
        let sources = vec![source];
        let outcome = read_sources(&sources, "w", &call).unwrap();
        assert!(matches!(&outcome, ToolOutcome::Returned(_)));
        let value: Value = serde_json::from_str(outcome.text().unwrap()).unwrap();
        assert_eq!(value["results"][0]["content"], "hello");
        assert_eq!(value["results"][0]["revision"], "v1");
        assert_eq!(value["results"][1]["outcome"], "unavailable");
        let elsewhere = read_sources(&sources, "other", &call).unwrap();
        let value: Value = serde_json::from_str(elsewhere.text().unwrap()).unwrap();
        assert_eq!(value["results"][0]["outcome"], "unavailable");
        call.arguments["workspace"] = json!("other");
        let invalid = read_sources(&sources, "w", &call).unwrap();
        let ToolOutcome::Refused(invalid) = invalid else {
            panic!("extra workspace argument must be refused")
        };
        assert!(!invalid.reason().is_empty());
        call.name = "send_message".into();
        let unoffered = read_sources(&sources, "w", &call).unwrap();
        let ToolOutcome::Refused(unoffered) = unoffered else {
            panic!("unoffered action must be refused")
        };
        assert!(!unoffered.reason().is_empty());
    }

    #[test]
    fn completion_requires_exact_terminal_not_plain_text_or_mixed_calls() {
        let response = Turn {
            receipt: Value::Null,
            usage: Usage::default(),
            items: vec![Item::from(Invocation {
                name: FINISH_TOOL.into(),
                id: "finish".into(),
                arguments: json!({"text":"Revised reply","review_basis":"Read sources"}),
            })],
        };
        assert!(revised_text(&response).is_ok());
        let call = response.calls().next().unwrap().parsed().unwrap();
        let check = |calls: Vec<Invocation<Value>>| {
            let mut candidate = response.clone();
            candidate.items = vec![Item::assistant("A bare assistant message cannot publish")];
            candidate.items.extend(calls.into_iter().map(Item::from));
            assert!(revised_text(&candidate).is_err());
        };
        check(vec![]);
        check(vec![
            call.clone(),
            Invocation {
                id: "effect".into(),
                name: "send_message".into(),
                arguments: json!({}),
            },
        ]);
        let mut missing_basis = call.clone();
        missing_basis
            .arguments
            .as_object_mut()
            .unwrap()
            .remove("review_basis");
        check(vec![missing_basis]);
        let mut blank = call;
        blank.arguments["text"] = json!("   ");
        check(vec![blank]);
    }
}
