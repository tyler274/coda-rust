//! Fenced reply-revision execution. The draft row owns all evidence and text;
//! no chat row, effect tool, or projection is an authority for this work.
use super::WorkerAuthority;
use super::attempt::{self, Claimed, Failure, Next, Outcome, Tally};
use super::{Hosted, Worker, absent_claim, announce, pull};
use async_trait::async_trait;
use harness::engine::recipe::Finish;
use harness::engine::{self, Cancellation, Event, Invocation, Product, ProductError, Turn};
use local_product::physician_sources::{self, read_sources};
use local_product::runtime::Store;
use local_product::runtime::delivery::{Delivery, DeliveryKind};
use local_product::runtime::memory::tools::MemoryTool;
use local_product::runtime::reply::{
    RevisionClaim, RevisionCommit, requests_finish, revised_text, revision_feedback,
};
use serde_json::{Value, json};
use telemetry::genai::{self, RunContext};
use tokio::sync::watch;
use tracing::Instrument;

const JOB: &str = "physician_reply_revision";

/// Claims the revision `delivery` names and runs it inside its own
/// `physician.worker.attempt` span.
pub(super) async fn deliver(
    worker: &Worker,
    delivery: &Delivery,
    stop: watch::Receiver<bool>,
) -> Next {
    let claim = match worker
        .store
        .claim_revision(delivery, worker.pace.lease)
        .await
    {
        Ok(Some(claim)) => claim,
        Ok(None) => {
            absent_claim(worker, delivery).await;
            return Next::Done;
        }
        Err(_) => {
            let span = attempt::span(DeliveryKind::ReplyRevision, JOB, None);
            attempt::finish(&span, JOB, None, Outcome::ClaimError, Some(Failure::Claim));
            return Next::Done;
        }
    };
    let span = attempt::span(DeliveryKind::ReplyRevision, JOB, claim.origin.as_ref());
    handle(worker, claim, &span, stop)
        .instrument(span.clone())
        .await
}

async fn handle(
    worker: &Worker,
    claim: RevisionClaim,
    span: &tracing::Span,
    stop: watch::Receiver<bool>,
) -> Next {
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
        authority: WorkerAuthority::of(&worker.model),
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
        // The opening starts with the memory the reply work started with; a
        // recovered run replays its saved request instead.
        let memory = worker
            .store
            .reply_memory(&owner.claim.scope)
            .await
            .map_err(|error| engine::Failure::Host(error.into()))?;
        let work = owner
            .claim
            .binding
            .request(context(&owner.claim).items(&owner.claim.sources, memory))
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
    let hosted = tokio::select! {
        hosted = super::while_owned(
            claimed.lease,
            super::STALL,
            &cancel,
            || worker.store.renew_revision_claim(&claimed),
            run.instrument(generation.clone()),
        ) => Some(hosted),
        () = pull::stopping(stop) => None,
    };
    if let Some((input, output)) = owner.usage.known() {
        genai::record_usage(&generation, input, output);
    }
    let run_id = Some(owner.claim.attempt.as_str());
    let Some(hosted) = hosted else {
        // This worker is stopping: the revision is handed back as
        // interrupted, at no cost, and another worker runs it at once.
        cancel.cancel();
        genai::record_error(&generation, "physician worker stopping");
        let saved = worker
            .store
            .settle_revision_attempt(
                &owner.claim,
                engine::AttemptFailure::Interrupted,
                worker.pace.backoff,
            )
            .await;
        if saved.is_err() {
            tracing::warn!("physician worker claim not handed back; its lease will run out");
        }
        attempt::finish(span, JOB, run_id, Outcome::Released, None);
        return Next::Done;
    };
    let result = match hosted {
        Hosted::Returned(result) => result,
        Hosted::Lost => return super::lost(span, JOB, &owner.claim.attempt),
    };
    let next = match result {
        Ok(response) => {
            // Completion has already atomically compared its retained exchange
            // and committed under the real session's publication locks.
            if let Ok(text) = revised_text(&response) {
                genai::record_output(&generation, &text);
            }
            attempt::finish(span, JOB, run_id, Outcome::Committed, None);
            Next::Done
        }
        Err(error) => {
            let reason = engine::AttemptFailure::of(&error, &cancel);
            cancel.cancel();
            let saved = worker
                .store
                .settle_revision_attempt(&owner.claim, reason, worker.pace.backoff)
                .await;
            let (outcome, next) = attempt::settled(span, JOB, &owner.claim.attempt, &saved);
            let failure = match outcome {
                Outcome::Retry | Outcome::GaveUp => {
                    genai::record_error(&generation, "physician reply revision failed");
                    Some(failure)
                }
                Outcome::Committed | Outcome::Settled | Outcome::Released | Outcome::ClaimError => {
                    None
                }
            };
            attempt::finish(span, JOB, run_id, outcome, failure);
            next
        }
    };
    announce(worker, owner.claim.scope.workspace.clone()).await;
    next
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
    ) -> Result<Option<Vec<harness::engine::ProductMessage>>, harness::engine::ProductError> {
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
    ) -> Result<(), ProductError> {
        self.assert_current().await?;
        self.store
            .admit_revision_binding(&self.claim, recipe)
            .await?;
        Ok(())
    }

    async fn window_seed(&mut self) -> Result<engine::WindowSeed, ProductError> {
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
        let memory = self.store.reply_memory(&self.claim.scope).await?;
        Ok(engine::WindowSeed {
            items: context(&self.claim).items(&self.claim.sources, memory),
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

    /// A started memory note's saved result, for the revision's work. The
    /// harness checks the revision is current before it asks.
    fn saved_outcome(
        &mut self,
        call: &Invocation,
    ) -> harness::engine::recipe::ToolFuture<Option<engine::ToolOutcome>> {
        let store = self.store.clone();
        let scope = self.claim.scope.clone();
        let origin = self.claim.accepted.input.note_origin(&call.id);
        let call = call.clone();
        Box::pin(async move {
            Ok(local_product::runtime::memory::tools::saved_outcome(
                &store,
                &scope,
                &origin.work,
                &call,
            )
            .await?)
        })
    }

    fn terminal(&self, response: &Turn) -> bool {
        // Mixed terminal/effect responses also route to completion, which
        // rejects them as a whole, without executing any of their calls.
        requests_finish(response)
    }

    async fn assert_current(&mut self) -> Result<(), ProductError> {
        // The revision supplies its own accepted session reference to the
        // same read-only auth owner used by chat.
        self.store
            .authorize_revision(&self.claim.scope, &mut self.authority)
            .await?;
        self.store.assert_revision_current(&self.claim).await
    }

    async fn persist(&mut self, event: Event) -> Result<(), ProductError> {
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
            if let Some(tool) = MemoryTool::named(&call.name) {
                return Ok(tool
                    .execute(&store, &claim.scope, &call.arguments, || {
                        Ok(claim.accepted.input.note_origin(&call.id))
                    })
                    .await?);
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
            Ok(read_sources(&claim.sources, &claim.scope.workspace, &call)?)
        })
    }

    /// A finish on an older reading of the chart publishes nothing. The
    /// attempt re-pins and the model gets what changed, reads it, and
    /// finishes again under the revision's same call budget.
    async fn finish(&mut self, response: &Turn) -> Result<Finish, ProductError> {
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
    ) -> Result<(), ProductError> {
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
            envelope: None,
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

    /// Opens every draft it is asked to.
    struct OpenAuthority;

    #[async_trait]
    impl local_product::runtime::Authority for OpenAuthority {
        async fn check(
            &mut self,
            _: &local_product::runtime::PublicationTransaction,
            _: &local_product::runtime::Scope,
            _: Option<&[u8]>,
        ) -> Result<(), String> {
            Ok(())
        }

        async fn lock_publication(
            &mut self,
            _: &local_product::runtime::PublicationTransaction,
            _: &local_product::runtime::Scope,
            _: Option<&[u8]>,
        ) -> Result<(), String> {
            Ok(())
        }
    }

    /// After a restart the harness asks the revision host for the saved
    /// outcome of a note call that started and has no recorded result. A
    /// saved note answers with the result the tool returns for it, and the
    /// log still holds one note; a call with no saved note has none.
    #[tokio::test]
    #[ignore = "requires explicit local Postgres"]
    async fn a_revision_host_answers_a_saved_notes_outcome_and_adds_nothing() {
        use local_product::runtime::memory::{FileRead, MemoryPath, NoteDraft, PatientKey};
        let url = std::env::var("PHYSICIAN_RUNTIME_TEST_DATABASE_URL").unwrap();
        let store = Store::connect(&url).await.unwrap();
        store.migrate().await.unwrap();
        let scope = local_product::runtime::Scope {
            practice: uuid::Uuid::new_v4().to_string(),
            physician: "physician".into(),
            workspace: uuid::Uuid::new_v4().to_string(),
            chat: "reply".into(),
        };
        store
            .open_draft_authorized(&scope, "Original", &[7; 32], &mut OpenAuthority)
            .await
            .unwrap()
            .expect("an open draft");
        let skills =
            local_product::physician_chat::PhysicianSkills::configured(None, false).unwrap();
        let binding = local_product::ModelBinding {
            model: "controlled".into(),
            effort: "none".into(),
            adapter: "fixed".into(),
            context_window_tokens: 272_000,
            max_input_tokens: 272_000,
        };
        let input = local_product::runtime::reply::RevisionInput {
            key: "revision".into(),
            expected_revision: 1,
            instruction: "Shorten the reply".into(),
            timezone: "UTC".into(),
            scenario: None,
        };
        store
            .accept_revision(
                &scope,
                &input,
                vec![7; 32],
                &skills,
                &local_product::runtime::reply::recipe(&binding, &skills),
                None,
            )
            .await
            .unwrap();
        let claim = store
            .claim_revision(&Delivery::reply_revision(scope.clone(), input.key), 60)
            .await
            .unwrap()
            .expect("a claimed revision");
        // The note commits; the run stops before its result is recorded.
        let saved = store
            .append_note(
                &claim.scope,
                &claim.accepted.input.note_origin("note"),
                &NoteDraft {
                    text: "She holds ibuprofen. [chart]".into(),
                    quote: None,
                },
            )
            .await
            .unwrap();
        let log = format!("log/{}.md", saved.time.format("%Y-%m-%d"));
        let mut owner = RevisionOwner {
            store: store.clone(),
            claim,
            authority: WorkerAuthority {
                inner: super::super::RuntimeAuthority,
                synthetic_only: false,
            },
            usage: Tally::default(),
        };
        let call = |id: &str| Invocation {
            id: id.into(),
            name: "add_patient_memory_note".into(),
            arguments: json!({"text": "She holds ibuprofen. [chart]", "quote": null}).to_string(),
        };
        let Some(ToolOutcome::Returned(text)) = owner.saved_outcome(&call("note")).await.unwrap()
        else {
            panic!("a saved note has its result");
        };
        assert_eq!(
            serde_json::from_str::<Value>(&text).unwrap(),
            json!({"note": "N1", "path": log})
        );
        assert_eq!(owner.saved_outcome(&call("other")).await.unwrap(), None);
        let key = PatientKey::try_from(&scope).unwrap();
        let FileRead::Log { markdown } = store
            .read_memory(&key, &MemoryPath::parse(&log).unwrap(), None)
            .await
            .unwrap()
        else {
            panic!("a log day has no versions");
        };
        assert_eq!(markdown.matches(", note ").count(), 1, "{markdown}");
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
