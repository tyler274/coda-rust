//! Fenced execution for practice-owned background jobs (`DeliveryKind::PracticeJob`).
//!
//! Runs with no physician session on the harness background lane
//! (`ModelRoles.background`), verifying practice patient membership through
//! [`physician_auth::AuthStore`] and attributing all events and saved outputs
//! to the practice and job.

use harness::engine::{self, Cancellation, RunLimits};
use local_product::runtime::Store;
use local_product::runtime::delivery::{Delivery, DeliveryKind};
use local_product::runtime::jobs::{PracticeJobClaim, PracticeJobHost};
use telemetry::genai::{self, RunContext};
use tokio::sync::watch;
use tracing::Instrument;

use super::attempt::{self, Claimed, Failure, Next, Outcome, Tally};
use super::{Hosted, Worker, absent_claim, announce, pull};

const JOB: &str = "physician_practice_job";

/// Claims the practice job `delivery` names and runs it inside its own
/// `physician.worker.attempt` span.
pub(super) async fn deliver(
    worker: &Worker,
    delivery: &Delivery,
    stop: watch::Receiver<bool>,
) -> Next {
    if super::WorkerAuthority::of(&worker.model)
        .check_store_source_boundary(&worker.store, &delivery.scope)
        .await
        .is_err()
    {
        let span = attempt::span(DeliveryKind::PracticeJob, JOB, None);
        attempt::finish(&span, JOB, None, Outcome::ClaimError, Some(Failure::Claim));
        return Next::Done;
    }

    let claim = match worker
        .store
        .claim_practice_job_delivery(delivery, worker.pace.lease, &worker.auth)
        .await
    {
        Ok(Some(claim)) => claim,
        Ok(None) => {
            absent_claim(worker, delivery).await;
            return Next::Done;
        }
        Err(_) => {
            let span = attempt::span(DeliveryKind::PracticeJob, JOB, None);
            attempt::finish(&span, JOB, None, Outcome::ClaimError, Some(Failure::Claim));
            return Next::Done;
        }
    };

    let span = attempt::span(DeliveryKind::PracticeJob, JOB, None);
    handle(worker, claim, &span, stop)
        .instrument(span.clone())
        .await
}

async fn handle(
    worker: &Worker,
    claim: PracticeJobClaim,
    span: &tracing::Span,
    stop: watch::Receiver<bool>,
) -> Next {
    let workspace_id = claim.scope.workspace.clone();
    let job_state = match worker.store.practice_job(&claim.scope).await {
        Ok(Some(state)) => state,
        Ok(None) => {
            tracing::warn!(
                job_id = %claim.scope.job_id,
                "claimed practice job record is missing"
            );
            attempt::finish(
                span,
                JOB,
                Some(&claim.attempt),
                Outcome::ClaimError,
                Some(Failure::Claim),
            );
            return Next::Done;
        }
        Err(error) => {
            tracing::warn!(
                job_id = %claim.scope.job_id,
                "could not read claimed practice job: {error}"
            );
            let _ = worker
                .store
                .settle_practice_job_attempt(
                    &claim,
                    engine::AttemptFailure::Interrupted,
                    worker.pace.backoff,
                )
                .await;
            attempt::finish(
                span,
                JOB,
                Some(&claim.attempt),
                Outcome::ClaimError,
                Some(Failure::Claim),
            );
            return Next::Done;
        }
    };
    let (origin, scenario_id) = worker.turn_origin(&claim.scope.practice, None);
    let input_text = job_state.assignment.prompt.clone();
    let earlier_attempts = {
        let count = job_state
            .attempts
            .iter()
            .filter(|saved| saved.revision == claim.revision && saved.id != claim.attempt)
            .count();
        u32::try_from(count).unwrap_or(u32::MAX)
    };
    let claimed_attempt = job_state
        .attempts
        .iter()
        .find(|saved| saved.id == claim.attempt);

    attempt::record_claim(
        span,
        &Claimed {
            run_id: &claim.attempt,
            work_id: Some(&claim.work),
            predecessor_run_id: claimed_attempt.and_then(|saved| saved.predecessor.as_deref()),
            input_revision: Some(claim.revision),
            attempt_number: earlier_attempts.saturating_add(1),
        },
    );
    drop(job_state);

    let generation = genai::generation_span(
        &RunContext {
            run_id: claim.attempt.clone(),
            job: JOB,
            recipe_version: genai::recipe_version(),
            origin,
            scenario_id,
            session_id: Some(claim.scope.job_id.clone()),
        },
        "openai",
        &worker.model.roles.background.adapter,
    );
    genai::record_input(&generation, &input_text);
    drop(input_text);

    if worker
        .simulation_practices
        .rehearsed_failure(&claim.scope.practice, None, earlier_attempts)
    {
        genai::record_error(&generation, Failure::Rehearsed.description());
        return settle_attempt(
            worker,
            &claim,
            workspace_id,
            engine::AttemptFailure::Failed,
            span,
            &generation,
            Failure::Rehearsed,
        )
        .await;
    }

    let cancel = Cancellation::default();
    let mut provider = match worker.model.provider(worker.quota.clone()).await {
        Ok(provider) => provider,
        Err(error) => {
            genai::record_error(&generation, "practice background job provider unavailable");
            let reason = engine::AttemptFailure::of(&engine::Failure::Provider(error), &cancel);
            return settle_attempt(
                worker,
                &claim,
                workspace_id,
                reason,
                span,
                &generation,
                Failure::ProviderInit,
            )
            .await;
        }
    };

    let mut host = PracticeJobHost {
        store: worker.store.clone(),
        claim: claim.clone(),
        authority: &worker.auth,
    };
    let hosted = tokio::select! {
        hosted = super::while_owned(
            claim.lease,
            super::STALL,
            &cancel,
            || worker.store.renew_practice_job_claim(&claim),
            host.run(&mut provider, RunLimits::default(), &cancel)
                .instrument(generation.clone()),
        ) => Some(hosted),
        () = pull::stopping(stop) => None,
    };
    record_practice_job_usage(&worker.store, &claim, &generation).await;

    let Some(hosted) = hosted else {
        cancel.cancel();
        genai::record_error(&generation, "physician worker stopping");
        let saved = worker
            .store
            .settle_practice_job_attempt(
                &claim,
                engine::AttemptFailure::Interrupted,
                worker.pace.backoff,
            )
            .await;
        if saved.is_err() {
            tracing::warn!("physician worker claim not handed back; its lease will run out");
        }
        attempt::finish(span, JOB, Some(&claim.attempt), Outcome::Released, None);
        return Next::Done;
    };

    let result = match hosted {
        Hosted::Returned(result) => result,
        Hosted::Lost => return super::lost(span, JOB, &claim.attempt),
    };
    match result {
        Ok(()) => {
            match worker.store.practice_job(&claim.scope).await {
                Ok(Some(saved)) => {
                    let output = saved.result.map(|res| res.output_text).unwrap_or_default();
                    genai::record_output(&generation, &output);
                }
                Ok(None) => {
                    tracing::warn!(
                        job_id = %claim.scope.job_id,
                        "completed practice job record is missing"
                    );
                }
                Err(error) => {
                    tracing::warn!(
                        job_id = %claim.scope.job_id,
                        "could not read completed practice job output: {error}"
                    );
                }
            }
            announce(worker, workspace_id).await;
            attempt::finish(span, JOB, Some(&claim.attempt), Outcome::Committed, None);
            Next::Done
        }
        Err(error) => {
            let reason = engine::AttemptFailure::of(&error, &cancel);
            cancel.cancel();
            settle_attempt(
                worker,
                &claim,
                workspace_id,
                reason,
                span,
                &generation,
                Failure::ModelCall,
            )
            .await
        }
    }
}

async fn record_practice_job_usage(
    store: &Store,
    claim: &PracticeJobClaim,
    generation: &tracing::Span,
) {
    let state = match store.practice_job(&claim.scope).await {
        Ok(Some(state)) => state,
        Ok(None) => return,
        Err(error) => {
            tracing::warn!(
                job_id = %claim.scope.job_id,
                "could not read practice job usage: {error}"
            );
            return;
        }
    };
    let usage = state
        .attempts
        .iter()
        .find(|saved| saved.id == claim.attempt && saved.revision == claim.revision)
        .map(|saved| Tally::from_events(&saved.events))
        .and_then(|tally| tally.known());
    if let Some((input, output)) = usage {
        genai::record_usage(generation, input, output);
    }
}

async fn settle_attempt(
    worker: &Worker,
    claim: &PracticeJobClaim,
    workspace_id: String,
    reason: engine::AttemptFailure,
    span: &tracing::Span,
    generation: &tracing::Span,
    failure: Failure,
) -> Next {
    let run_id = Some(claim.attempt.as_str());
    let saved = worker
        .store
        .settle_practice_job_attempt(claim, reason, worker.pace.backoff)
        .await;
    let (outcome, next) = attempt::settled(span, JOB, &claim.attempt, &saved);
    let recorded_failure = match outcome {
        Outcome::Retry | Outcome::GaveUp => {
            genai::record_error(generation, "practice background job failed");
            Some(failure)
        }
        Outcome::Committed | Outcome::Settled | Outcome::Released | Outcome::ClaimError => None,
    };
    attempt::finish(span, JOB, run_id, outcome, recorded_failure);
    announce(worker, workspace_id).await;
    next
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use std::sync::{Arc, Mutex};
    use std::time::Duration;

    use axum::Router;
    use axum::body::Body;
    use axum::http::{Request as HttpRequest, StatusCode};
    use axum::routing::post;
    use local_product::physician_chat::PhysicianSkills;
    use local_product::runtime::delivery::{Backoff, DeliveryKind};
    use local_product::runtime::jobs::{Assignment, PracticeJobKind, PracticeScope};
    use local_product::runtime::{AttemptStatus, Resumed, Scope, Stopped, Store, WorkStatus};
    use local_product::{ModelBinding, ModelRoles};
    use physician_auth::{AuthStore, PostgresAuthStore, PracticeClock};
    use serde_json::{Value, json};
    use tokio::sync::watch;
    use tower::ServiceExt as _;
    use uuid::Uuid;

    use super::super::attempt::Next;
    use super::super::{ModelSource, ModelTransport, Pace, Worker, router, run_delivery};

    fn sse_assistant_reply(text: &str) -> String {
        let item = json!({
            "id": "msg_practice_1",
            "type": "message",
            "role": "assistant",
            "status": "completed",
            "content": [{"type": "output_text", "text": text}],
        });
        let response = json!({
            "id": "resp_practice_1",
            "status": "completed",
            "output": [item],
            "usage": {
                "input_tokens": 24,
                "input_tokens_details": {"cached_tokens": 8},
                "output_tokens": 12
            },
        });
        let events = [
            json!({"type": "response.created", "response": {"id": "resp_practice_1"}}),
            json!({"type": "response.output_item.added", "output_index": 0, "item": item}),
            json!({"type": "response.content_part.added", "output_index": 0, "item_id": "msg_practice_1", "content_index": 0, "part": {"type": "output_text", "text": ""}}),
            json!({"type": "response.output_text.delta", "output_index": 0, "item_id": "msg_practice_1", "content_index": 0, "delta": text}),
            json!({"type": "response.output_text.done", "output_index": 0, "item_id": "msg_practice_1", "content_index": 0, "text": text}),
            json!({"type": "response.content_part.done", "output_index": 0, "item_id": "msg_practice_1", "content_index": 0, "part": {"type": "output_text", "text": text}}),
            json!({"type": "response.output_item.done", "output_index": 0, "item": item}),
            json!({"type": "response.completed", "response": response}),
        ];
        events
            .iter()
            .enumerate()
            .map(|(seq, ev)| {
                let mut map = ev.as_object().unwrap().clone();
                map.insert("sequence_number".into(), json!(seq));
                format!("data: {}\n\n", Value::Object(map))
            })
            .collect()
    }

    #[tokio::test]
    async fn worker_pull_delivery_executes_resumed_practice_job_on_background_lane_and_ignores_redelivery()
     {
        let url = match std::env::var("PHYSICIAN_RUNTIME_TEST_DATABASE_URL")
            .or_else(|_| std::env::var("PHYSICIAN_DATABASE_URL"))
        {
            Ok(v) if !v.trim().is_empty() => v,
            _ => {
                assert!(
                    std::env::var_os("CODA_REQUIRE_POSTGRES_TESTS").is_none(),
                    "PHYSICIAN_RUNTIME_TEST_DATABASE_URL is required when CODA_REQUIRE_POSTGRES_TESTS=1"
                );
                return;
            }
        };

        let store = Store::connect(&url).await.unwrap();
        store.migrate().await.unwrap();
        let auth = PostgresAuthStore::connect_with_migrations(&url)
            .await
            .unwrap();

        let observed_models: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));
        let recorded = observed_models.clone();
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let port = listener.local_addr().unwrap().port();
        let stub = Router::new().route(
            "/responses",
            post(move |body: axum::body::Bytes| {
                let recorded = recorded.clone();
                async move {
                    let payload: Value = serde_json::from_slice(&body).unwrap();
                    recorded
                        .lock()
                        .unwrap()
                        .push(payload["model"].as_str().unwrap_or_default().to_owned());
                    (
                        StatusCode::OK,
                        [("content-type", "text/event-stream")],
                        sse_assistant_reply("Resumed background review completed."),
                    )
                }
            }),
        );
        let stub_task = tokio::spawn(async move {
            axum::serve(listener, stub).await.unwrap();
        });

        let roles = ModelRoles {
            chat: ModelBinding {
                model: "chat-interactive-lane".into(),
                effort: "low".into(),
                context_window_tokens: 128_000,
                max_input_tokens: 100_000,
                adapter: "openai-responses".into(),
            },
            background: ModelBinding {
                model: "background-worker-lane".into(),
                effort: "high".into(),
                context_window_tokens: 272_000,
                max_input_tokens: 250_000,
                adapter: "openai-responses".into(),
            },
        };
        let skills =
            PhysicianSkills::load(&PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../skills"))
                .unwrap();

        let practice = format!("practice-wkr-{}", Uuid::new_v4().simple());
        let workspace = format!("pw-wkr-{}", Uuid::new_v4().simple());
        auth.ensure_practice(&practice, PracticeClock::RealTime)
            .await
            .unwrap();
        auth.register_patient(&practice, &workspace).await.unwrap();

        let scope = PracticeScope {
            practice: practice.clone(),
            workspace: workspace.clone(),
            job_id: format!("job-wkr-{}", Uuid::new_v4().simple()),
        };
        let assignment = Assignment::new(
            PracticeJobKind::PatientReview,
            format!("key-wkr-{}", Uuid::new_v4().simple()),
            "Conduct a periodic chart review for this patient workspace.",
            &["patient-record-retrieval"],
            &skills,
        )
        .unwrap();
        store
            .accept_practice_job(&scope, &assignment, &roles, &skills, &auth)
            .await
            .unwrap();

        // Stop and then Resume through Store::resume_authorized to obtain the wakeup Delivery.
        let before = store.practice_job(&scope).await.unwrap().unwrap();
        let first_run_id = before.current_run().unwrap().id.to_owned();
        let physician_scope = Scope {
            practice: practice.clone(),
            physician: "dr-alice".into(),
            workspace: workspace.clone(),
            chat: scope.job_id.clone(),
        };
        assert_eq!(
            store
                .stop_authorized(
                    &physician_scope,
                    "stop-wkr-1",
                    vec![0_u8; 32],
                    &scope.job_id
                )
                .await
                .unwrap(),
            Stopped::Held,
        );
        let Resumed::Pending { delivery } = store
            .resume_authorized(
                &physician_scope,
                "resume-wkr-1",
                vec![0_u8; 32],
                &scope.job_id,
                &first_run_id,
                None,
            )
            .await
            .unwrap()
        else {
            panic!("expected Resumed::Pending");
        };
        assert_eq!(delivery.kind, DeliveryKind::PracticeJob);

        let wake = Arc::new(crate::WakeService {
            auth: crate::WakeAuth::Disabled { verifier: None },
            mode: crate::WakeMode::RehearsalAllowed,
            handler: Arc::new(crate::StoreWakeHandler {
                store: store.clone(),
                auth: auth.clone(),
                roles: roles.clone(),
                skills: Ok(skills),
            }),
            clinical_library: None,
        });
        let worker = Worker {
            store: store.clone(),
            auth: auth.clone(),
            bus: physician_bus::from_env(|_| None, false).await.unwrap(),
            model: ModelSource {
                transport: ModelTransport::Responses {
                    endpoint: format!("http://127.0.0.1:{port}/responses"),
                    key: "local-proof".into(),
                },
                roles,
            },
            quota: Arc::new(store.clone()),
            pace: Pace {
                poll: Duration::from_secs(1),
                lease: 60,
                concurrency: 4,
                backoff: Backoff::IMMEDIATE,
            },
            wake,
            origin: telemetry::genai::Origin::Simulation,
            simulation_practices: physician_sample::hosting::SimulationPractices::from_env(|_| {
                None
            })
            .unwrap(),
        };
        let (_stop_tx, stop_rx) = watch::channel(false);

        assert_eq!(
            run_delivery(&worker, &delivery, stop_rx.clone()).await,
            Next::Done
        );

        let completed = store.practice_job(&scope).await.unwrap().unwrap();
        assert_eq!(completed.status, WorkStatus::Completed);
        assert_eq!(completed.attempts.len(), 1);
        assert_eq!(completed.attempts[0].status, AttemptStatus::Completed);
        assert_eq!(
            completed.result.as_ref().unwrap().output_text,
            "Resumed background review completed."
        );
        assert_eq!(
            *observed_models.lock().unwrap(),
            vec!["background-worker-lane".to_owned()],
        );

        // Delivering the settled wakeup again returns Done without invoking the provider again.
        assert_eq!(run_delivery(&worker, &delivery, stop_rx).await, Next::Done);
        assert_eq!(observed_models.lock().unwrap().len(), 1);

        stub_task.abort();
    }

    fn sse_function_calls(resp_id: &str, calls: &[(&str, &str, Value)]) -> String {
        let mut events = vec![json!({"type": "response.created", "response": {"id": resp_id}})];
        let mut output = Vec::with_capacity(calls.len());
        for (idx, (call_id, name, args)) in calls.iter().enumerate() {
            let item_id = format!("fc_{call_id}");
            let args_str = args.to_string();
            let completed_item = json!({
                "id": item_id,
                "type": "function_call",
                "status": "completed",
                "call_id": call_id,
                "name": name,
                "arguments": args_str,
            });
            events.push(json!({
                "type": "response.output_item.added",
                "output_index": idx,
                "item": {
                    "id": item_id,
                    "type": "function_call",
                    "call_id": call_id,
                    "name": name,
                }
            }));
            events.push(json!({
                "type": "response.function_call_arguments.delta",
                "output_index": idx,
                "item_id": item_id,
                "delta": args_str,
            }));
            events.push(json!({
                "type": "response.function_call_arguments.done",
                "output_index": idx,
                "item_id": item_id,
                "arguments": args_str,
            }));
            events.push(json!({
                "type": "response.output_item.done",
                "output_index": idx,
                "item": completed_item,
            }));
            output.push(completed_item);
        }
        let response = json!({
            "id": resp_id,
            "status": "completed",
            "output": output,
            "usage": {
                "input_tokens": 310,
                "input_tokens_details": {"cached_tokens": 120},
                "output_tokens": 65
            },
        });
        events.push(json!({"type": "response.completed", "response": response}));
        events
            .iter()
            .enumerate()
            .map(|(seq, ev)| {
                let mut map = ev.as_object().unwrap().clone();
                map.insert("sequence_number".into(), json!(seq));
                format!("data: {}\n\n", Value::Object(map))
            })
            .collect()
    }

    #[tokio::test]
    async fn schedule_wake_dispatches_due_patient_reviews_and_pubsub_push_commits_saved_review() {
        let url = match std::env::var("PHYSICIAN_RUNTIME_TEST_DATABASE_URL")
            .or_else(|_| std::env::var("PHYSICIAN_DATABASE_URL"))
        {
            Ok(v) if !v.trim().is_empty() => v,
            _ => {
                assert!(
                    std::env::var_os("CODA_REQUIRE_POSTGRES_TESTS").is_none(),
                    "PHYSICIAN_RUNTIME_TEST_DATABASE_URL is required when CODA_REQUIRE_POSTGRES_TESTS=1"
                );
                return;
            }
        };

        let store = Store::connect(&url).await.unwrap();
        store.migrate().await.unwrap();
        let auth = PostgresAuthStore::connect_with_migrations(&url)
            .await
            .unwrap();

        let practice = format!("practice-wake-e2e-{}", Uuid::new_v4().simple());
        let workspace = format!("pw-wake-e2e-{}", Uuid::new_v4().simple());
        auth.ensure_practice(&practice, PracticeClock::RealTime)
            .await
            .unwrap();
        auth.register_patient(&practice, &workspace).await.unwrap();

        // Register dispatch baseline at t=1_000_000, then deliver a new abnormal potassium record!
        store
            .register_patient_dispatch(&auth, &practice, &workspace, 1_000_000)
            .await
            .unwrap();
        let content = "Serum potassium 6.0 mmol/L (critical high) on spironolactone.";
        let record = local_product::physician_sources::SourceRevision {
            workspace: workspace.clone(),
            source_id: "synthetic-practice".into(),
            candidate_id: "Observation/k-wake-1".into(),
            revision: "rev-1".into(),
            record_type: "Observation".into(),
            dates: vec![local_product::physician_sources::RecordDate {
                name: "effectiveDateTime".into(),
                value: "2026-10-20".into(),
            }],
            content: content.into(),
            sha256: local_product::sha256_hex(content.as_bytes()),
            readable_parts: vec![local_product::physician_sources::ReadablePart {
                label: "STAT Potassium".into(),
                text: content.into(),
            }],
            provenance: json!({ "title": "STAT Potassium" }),
            envelope: None,
        };
        let pull = local_product::runtime::sources::SourcePull {
            source_id: "synthetic-practice".into(),
            source_label: "Synthetic Chart".into(),
            patient: local_product::physician_sources::SourcePatient {
                source_patient_id: "pt-wake-1".into(),
                display_name: "Evelyn Carter".into(),
                birth_date: Some("1965-03-11".into()),
                sex_or_gender: Some("female".into()),
                medical_record_numbers: vec![],
            },
            pulled_at: Some(1_100_000),
        };
        store
            .record_chart(&workspace, &[record], &pull)
            .await
            .unwrap();

        let turn_counter = Arc::new(Mutex::new(0_usize));
        let counter_ref = turn_counter.clone();
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let port = listener.local_addr().unwrap().port();
        let stub = Router::new().route(
            "/responses",
            post(move |_body: axum::body::Bytes| {
                let counter_ref = counter_ref.clone();
                async move {
                    let step = {
                        let mut g = counter_ref.lock().unwrap();
                        *g += 1;
                        *g
                    };
                    let sse = match step {
                        1 => sse_function_calls(
                            "resp_wake_1",
                            &[(
                                "call_read_k",
                                local_product::physician_sources::READ_TOOL,
                                json!({
                                    "references": [{
                                        "candidate_id": "Observation/k-wake-1",
                                        "revision": "rev-1",
                                        "cursor": null
                                    }],
                                    "max_result_bytes": 16_384
                                }),
                            )],
                        ),
                        2 => sse_function_calls(
                            "resp_wake_2",
                            &[
                                (
                                    "call_raise_k",
                                    local_product::runtime::concerns::CONCERN_TOOL,
                                    json!({
                                        "operation": "raise",
                                        "concern_id": "concern-wake-k",
                                        "urgency": "now",
                                        "reason": "Critical potassium 6.0 mmol/L on spironolactone; prompt physician review required.",
                                        "records": [{
                                            "candidate_id": "Observation/k-wake-1",
                                            "revision": "rev-1"
                                        }]
                                    }),
                                ),
                                (
                                    "call_complete_k",
                                    local_product::patient_reviews::COMPLETE_PATIENT_REVIEW_TOOL,
                                    json!({
                                        "disposition": "assessed",
                                        "summary": "Critical potassium 6.0 mmol/L; raised urgent concern.",
                                        "citations": [{
                                            "candidate_id": "Observation/k-wake-1",
                                            "revision": "rev-1"
                                        }],
                                        "basis_and_limitations": "Synthetic Chart STAT Potassium (2026-10-20).",
                                        "unfinished_work": []
                                    }),
                                ),
                            ],
                        ),
                        3 => sse_assistant_reply("Completed scheduled wake patient review."),
                        other => panic!("unexpected SSE step {other}"),
                    };
                    (
                        StatusCode::OK,
                        [("content-type", "text/event-stream")],
                        sse,
                    )
                }
            }),
        );
        let stub_task = tokio::spawn(async move {
            axum::serve(listener, stub).await.unwrap();
        });

        let roles = ModelRoles {
            chat: ModelBinding {
                model: "chat-interactive-lane".into(),
                effort: "low".into(),
                context_window_tokens: 128_000,
                max_input_tokens: 100_000,
                adapter: "openai-responses".into(),
            },
            background: ModelBinding {
                model: "background-worker-lane".into(),
                effort: "high".into(),
                context_window_tokens: 272_000,
                max_input_tokens: 250_000,
                adapter: "openai-responses".into(),
            },
        };
        let skills_root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../skills");
        let skills = PhysicianSkills::load(&skills_root).unwrap();
        let mut wakeups = store.wakeups().await.unwrap();

        let wake = Arc::new(crate::WakeService {
            auth: crate::WakeAuth::LoopbackEmulator,
            mode: crate::WakeMode::RehearsalAllowed,
            handler: Arc::new(crate::StoreWakeHandler {
                store: store.clone(),
                auth: auth.clone(),
                roles: roles.clone(),
                skills: Ok(skills),
            }),
            clinical_library: None,
        });
        let worker = Arc::new(Worker {
            store: store.clone(),
            auth: auth.clone(),
            bus: physician_bus::from_env(|_| None, false).await.unwrap(),
            model: ModelSource {
                transport: ModelTransport::Responses {
                    endpoint: format!("http://127.0.0.1:{port}/responses"),
                    key: "local-proof".into(),
                },
                roles,
            },
            quota: Arc::new(store.clone()),
            pace: Pace {
                poll: Duration::from_secs(1),
                lease: 60,
                concurrency: 4,
                backoff: Backoff::IMMEDIATE,
            },
            wake,
            origin: telemetry::genai::Origin::Simulation,
            simulation_practices: physician_sample::hosting::SimulationPractices::from_env(|_| {
                None
            })
            .unwrap(),
        });
        let app = router(Arc::clone(&worker));

        // 1. Hit `POST /schedule/wake` -> dispatches our due patient's review and notifies `WORK_CHANNEL`!
        let wake_resp = app
            .oneshot(
                HttpRequest::builder()
                    .method("POST")
                    .uri("/schedule/wake")
                    .header("content-type", "application/json")
                    .body(Body::from("{}"))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(wake_resp.status(), StatusCode::OK);
        let wake_bytes = axum::body::to_bytes(wake_resp.into_body(), 64 * 1024)
            .await
            .unwrap();
        let wake_json: Value = serde_json::from_slice(&wake_bytes).unwrap();
        assert_eq!(wake_json["status"], "ok");
        assert!(wake_json["due_patients_checked"].as_u64().unwrap() >= 1);
        assert!(wake_json["reviews_started"].as_u64().unwrap() >= 1);

        let our_delivery = tokio::time::timeout(Duration::from_secs(5), async {
            loop {
                let d = wakeups
                    .next()
                    .await
                    .unwrap()
                    .expect("work notification stream open");
                if d.scope.practice == practice && d.scope.workspace == workspace {
                    return d;
                }
            }
        })
        .await
        .expect("announced practice-job wakeup for our workspace");
        assert_eq!(our_delivery.kind, DeliveryKind::PracticeJob);

        // 2. Run the announced wakeup through `run_delivery`!
        let (_stop_tx, stop_rx) = watch::channel(false);
        assert_eq!(
            run_delivery(&worker, &our_delivery, stop_rx).await,
            Next::Done
        );

        // 3. Verify `SavedPatientReview` is durably Completed with Known usage!
        let reviews = store
            .patient_reviews(&practice, &workspace, &auth)
            .await
            .unwrap();
        assert_eq!(reviews.len(), 1);
        assert_eq!(reviews[0].state.status_str(), "completed");
        assert_eq!(reviews[0].actions.len(), 1);
        assert!(matches!(
            reviews[0].usage,
            local_product::runtime::reviews::ReviewUsage::Known { exchanges: 3, .. }
        ));

        stub_task.abort();
    }

    #[tokio::test]
    async fn wake_skills_error_preserves_package_cause_and_invalid_skills_dir_fails_from_env() {
        use std::error::Error as _;

        let missing_dir = format!("/tmp/nonexistent-skills-{}", Uuid::new_v4());
        let from_env_err = crate::Worker::from_env(|key| match key {
            "PUBSUB_EMULATOR_HOST" => Some("127.0.0.1:8085".into()),
            "PHYSICIAN_RESPONSES_STUB_PORT" => Some("8080".into()),
            "CODA_CHAT_MODEL" | "CODA_BACKGROUND_MODEL" => Some("gpt-5.4-mini".into()),
            "CODA_CHAT_EFFORT" | "CODA_BACKGROUND_EFFORT" => Some("low".into()),
            "CODA_CHAT_CONTEXT_WINDOW_TOKENS" | "CODA_BACKGROUND_CONTEXT_WINDOW_TOKENS" => {
                Some("272000".into())
            }
            "CODA_CHAT_INPUT_LIMIT_TOKENS" | "CODA_BACKGROUND_INPUT_LIMIT_TOKENS" => {
                Some("400000".into())
            }
            "PHYSICIAN_SKILLS_DIR" => Some(missing_dir.clone()),
            _ => None,
        })
        .await
        .err()
        .expect("invalid PHYSICIAN_SKILLS_DIR must fail Worker::from_env");
        assert!(
            from_env_err.to_string().contains(&missing_dir),
            "expected package load error cause, got: {from_env_err}"
        );

        let url = match std::env::var("PHYSICIAN_RUNTIME_TEST_DATABASE_URL")
            .or_else(|_| std::env::var("PHYSICIAN_DATABASE_URL"))
        {
            Ok(v) if !v.trim().is_empty() => v,
            _ => {
                assert!(
                    std::env::var_os("CODA_REQUIRE_POSTGRES_TESTS").is_none(),
                    "PHYSICIAN_RUNTIME_TEST_DATABASE_URL is required when CODA_REQUIRE_POSTGRES_TESTS=1"
                );
                return;
            }
        };
        let store = Store::connect(&url).await.unwrap();
        store.migrate().await.unwrap();
        let auth = PostgresAuthStore::connect_with_migrations(&url)
            .await
            .unwrap();
        let practice = format!("prac-wake-skills-err-{}", Uuid::new_v4().simple());
        let workspace = format!("ws-wake-skills-err-{}", Uuid::new_v4().simple());
        auth.ensure_practice(&practice, PracticeClock::RealTime)
            .await
            .unwrap();
        auth.register_patient(&practice, &workspace).await.unwrap();
        store
            .register_patient_dispatch(&auth, &practice, &workspace, 1_700_000_000)
            .await
            .unwrap();

        let broken_skills = PhysicianSkills::load(std::path::Path::new(&missing_dir));
        assert!(matches!(
            broken_skills,
            Err(local_product::physician_chat::SkillsError::Package { .. })
        ));
        let handler = crate::StoreWakeHandler {
            store,
            auth,
            roles: ModelRoles {
                chat: ModelBinding {
                    model: "chat-interactive-lane".into(),
                    effort: "low".into(),
                    context_window_tokens: 128_000,
                    max_input_tokens: 100_000,
                    adapter: "openai-responses".into(),
                },
                background: ModelBinding {
                    model: "background-worker-lane".into(),
                    effort: "high".into(),
                    context_window_tokens: 272_000,
                    max_input_tokens: 250_000,
                    adapter: "openai-responses".into(),
                },
            },
            skills: broken_skills,
        };
        let wake_err = crate::WakeHandler::wake(
            &handler,
            &crate::WakeTick {
                job_name: None,
                schedule_time: None,
                retry_count: 0,
                directive: crate::WakeDirective::Normal,
            },
        )
        .await
        .expect_err("wake with broken skills must fail with preserved SkillsError::Package");
        let source = wake_err
            .source()
            .expect("WakeError::SkillsUnavailable must keep original SkillsError source");
        assert!(
            source.to_string().contains(&missing_dir),
            "expected preserved SkillsError::Package source, got: {source}"
        );
    }

    #[tokio::test]
    async fn scheduled_wake_handler_executes_both_patient_dispatch_and_visit_preparation_in_one_tick()
     {
        use local_product::physician_sources::{
            MedicalRecordNumber, READ_TOOL, ReadablePart, RecordDate, SourcePatient,
            SourceRevision, workspace_id,
        };
        use local_product::runtime::briefings::VisitSlotKey;
        use local_product::runtime::concerns::ExactRecordReference;
        use local_product::runtime::schedule::AppointmentWindow;
        use local_product::runtime::sources::SourcePull;
        use local_product::visit_preparation::{
            BriefingClaimInput, BriefingDecisionInput, SAVE_BRIEFING_TOOL, SELECT_VISITS_TOOL,
            SaveBriefingArgs,
        };

        let gate_url = match std::env::var("PHYSICIAN_RUNTIME_TEST_DATABASE_URL")
            .or_else(|_| std::env::var("PHYSICIAN_DATABASE_URL"))
        {
            Ok(v) if !v.trim().is_empty() => v.trim().to_owned(),
            _ => {
                assert!(
                    std::env::var_os("CODA_REQUIRE_POSTGRES_TESTS").is_none(),
                    "PHYSICIAN_RUNTIME_TEST_DATABASE_URL is required when CODA_REQUIRE_POSTGRES_TESTS=1"
                );
                return;
            }
        };

        let (server, _) = gate_url.rsplit_once('/').expect("database URL");
        let admin_url = format!("{server}/postgres");
        let db_name = format!(
            "worker_sched_wake_{}",
            &Uuid::new_v4().simple().to_string()[..12]
        );
        let admin = sea_orm::Database::connect(&admin_url).await.unwrap();
        crate::disposable::replace_database(&admin, &db_name).await;
        admin.close().await.unwrap();
        let url = format!("{server}/{db_name}");

        let store = Store::connect(&url).await.unwrap();
        store.migrate().await.unwrap();
        let auth = PostgresAuthStore::connect_with_migrations(&url)
            .await
            .unwrap();

        let practice = format!("practice-sched-wake-{}", Uuid::new_v4().simple());
        let fixed_now: u64 = 1_787_846_400; // 2026-08-27T16:00:00Z
        auth.ensure_practice(&practice, PracticeClock::Fixed(fixed_now))
            .await
            .unwrap();

        let mrn = format!("MRN-SW-{}", &Uuid::new_v4().simple().to_string()[..8]);
        let workspace = workspace_id(physician_sample::synthetic_chart::SOURCE_ID, &mrn);
        auth.register_patient(&practice, &workspace).await.unwrap();

        // Register dispatch baseline, then record a new chart observation so dispatch is due.
        store
            .register_patient_dispatch(&auth, &practice, &workspace, 1_000_000)
            .await
            .unwrap();
        let note_text = "Stage 3a CKD (eGFR 51 mL/min), A1c 8.1%, and potassium 5.8 mmol/L on spironolactone ahead of tomorrow's follow-up.";
        store
            .record_chart(
                &workspace,
                &[SourceRevision {
                    workspace: workspace.clone(),
                    source_id: physician_sample::synthetic_chart::SOURCE_ID.into(),
                    candidate_id: "Observation/ckd-k-1".into(),
                    revision: "rev-sw-1".into(),
                    record_type: "Observation".into(),
                    dates: vec![RecordDate {
                        name: "effectiveDateTime".into(),
                        value: "2026-08-26".into(),
                    }],
                    sha256: local_product::sha256_hex(note_text.as_bytes()),
                    content: note_text.into(),
                    readable_parts: vec![ReadablePart {
                        label: "Renal, A1c & Potassium".into(),
                        text: note_text.into(),
                    }],
                    provenance: json!({ "title": "Renal, A1c & Potassium" }),
                    envelope: None,
                }],
                &SourcePull {
                    source_id: physician_sample::synthetic_chart::SOURCE_ID.into(),
                    source_label: "Synthetic chart".into(),
                    patient: SourcePatient {
                        source_patient_id: mrn.clone(),
                        display_name: "Scheduled Wake Patient".into(),
                        birth_date: Some("1965-09-12".into()),
                        sex_or_gender: Some("male".into()),
                        medical_record_numbers: vec![MedicalRecordNumber {
                            system: "urn:mrn".into(),
                            value: mrn.clone(),
                        }],
                    },
                    pulled_at: Some(fixed_now as i64 - 600),
                },
            )
            .await
            .unwrap();

        let temp_dir =
            std::env::temp_dir().join(format!("coda-sched-wake-cal-{}", Uuid::new_v4().simple()));
        std::fs::create_dir_all(temp_dir.join("calendar")).unwrap();
        let appt_line = format!(
            r#"{{"resourceType":"Appointment","id":"appt-sw-tomorrow","meta":{{"versionId":"1","lastUpdated":"2026-08-27T00:00:00+00:00"}},"status":"booked","description":"CKD & diabetes follow-up","start":"2026-08-28T15:00:00+00:00","end":"2026-08-28T15:30:00+00:00","participant":[{{"actor":{{"reference":"Patient/{mrn}"}},"status":"accepted"}}]}}"#
        );
        std::fs::write(
            temp_dir.join("calendar/Appointment.ndjson"),
            format!("{appt_line}\n"),
        )
        .unwrap();
        let calendar = physician_sample::schedule::SyntheticBankCalendar::open(&temp_dir);

        let starts_at_unix =
            AppointmentWindow::parse("2026-08-28T15:00:00+00:00", "2026-08-28T15:30:00+00:00")
                .unwrap()
                .starts_at_unix();
        let save_briefing = serde_json::to_value(SaveBriefingArgs {
            visit_card_orientation:
                "CKD 3a (eGFR 51), A1c 8.1%, and potassium 5.8 mmol/L on spironolactone ahead of tomorrow's visit."
                    .into(),
            trajectory: vec![BriefingClaimInput {
                text: "Stage 3a CKD (eGFR 51 mL/min) with A1c 8.1% and potassium 5.8 mmol/L.".into(),
                uncertainty: None,
                citations: vec![ExactRecordReference {
                    candidate_id: "Observation/ckd-k-1".into(),
                    revision: "rev-sw-1".into(),
                }],
            }],
            meaningful_changes: vec![BriefingClaimInput {
                text: "Potassium rose to 5.8 mmol/L on spironolactone.".into(),
                uncertainty: None,
                citations: vec![ExactRecordReference {
                    candidate_id: "Observation/ckd-k-1".into(),
                    revision: "rev-sw-1".into(),
                }],
            }],
            unresolved_issues: vec![BriefingClaimInput {
                text: "Spot urine albumin-to-creatinine ratio is not in the connected chart.".into(),
                uncertainty: Some(
                    "Albuminuria severity is unconfirmed pending spot UACR.".into(),
                ),
                citations: vec![ExactRecordReference {
                    candidate_id: "Observation/ckd-k-1".into(),
                    revision: "rev-sw-1".into(),
                }],
            }],
            visit_decisions: vec![BriefingDecisionInput {
                decision: "Potassium management and glycemic regimen adjustment".into(),
                clinical_rationale:
                    "Potassium 5.8 mmol/L on spironolactone alongside A1c 8.1% warrants medication adjustment.".into(),
                options_or_questions: vec![
                    "Assess spironolactone dose and discuss adding an SGLT2 inhibitor.".into(),
                ],
                uncertainty: None,
                citations: vec![ExactRecordReference {
                    candidate_id: "Observation/ckd-k-1".into(),
                    revision: "rev-sw-1".into(),
                }],
            }],
            basis_and_limitations: "Prepared from synthetic chart panel 2026-08-26.".into(),
            basis_citations: Vec::new(),
            limits: Vec::new(),
            visit_concern: None,
        })
        .unwrap();

        let turn_counter = Arc::new(Mutex::new(0_usize));
        let counter_ref = turn_counter.clone();
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let port = listener.local_addr().unwrap().port();
        let stub = Router::new().route(
            "/responses",
            post(move |_body: axum::body::Bytes| {
                let counter_ref = counter_ref.clone();
                let save_briefing = save_briefing.clone();
                async move {
                    let step = {
                        let mut g = counter_ref.lock().unwrap();
                        *g += 1;
                        *g
                    };
                    let sse = match step {
                        1 => sse_function_calls(
                            "resp_vp_1",
                            &[(
                                "call_sel",
                                SELECT_VISITS_TOOL,
                                json!({
                                    "decisions": [{
                                        "source_id": physician_sample::schedule::SOURCE_ID,
                                        "appointment_id": "appt-sw-tomorrow",
                                        "starts_at_unix": starts_at_unix,
                                        "action": "prepare_now",
                                        "reason": "Tomorrow's CKD follow-up has K+ 5.8 and A1c 8.1%."
                                    }]
                                }),
                            )],
                        ),
                        2 => sse_assistant_reply("Selected tomorrow's visit."),
                        3 => sse_function_calls(
                            "resp_vp_3",
                            &[(
                                "call_vp_read",
                                READ_TOOL,
                                json!({
                                    "references": [{
                                        "candidate_id": "Observation/ckd-k-1",
                                        "revision": "rev-sw-1",
                                        "cursor": null
                                    }],
                                    "max_result_bytes": 16_384
                                }),
                            )],
                        ),
                        4 => sse_function_calls(
                            "resp_vp_4",
                            &[("call_vp_save", SAVE_BRIEFING_TOOL, save_briefing)],
                        ),
                        5 => sse_assistant_reply("Saved tomorrow's visit briefing."),
                        6 => sse_function_calls(
                            "resp_pr_1",
                            &[(
                                "call_pr_read",
                                READ_TOOL,
                                json!({
                                    "references": [{
                                        "candidate_id": "Observation/ckd-k-1",
                                        "revision": "rev-sw-1",
                                        "cursor": null
                                    }],
                                    "max_result_bytes": 16_384
                                }),
                            )],
                        ),
                        7 => sse_function_calls(
                            "resp_pr_2",
                            &[
                                (
                                    "call_pr_raise",
                                    local_product::runtime::concerns::CONCERN_TOOL,
                                    json!({
                                        "operation": "raise",
                                        "concern_id": "concern-sw-k",
                                        "urgency": "today",
                                        "reason": "Potassium 5.8 mmol/L on spironolactone ahead of CKD visit.",
                                        "records": [{
                                            "candidate_id": "Observation/ckd-k-1",
                                            "revision": "rev-sw-1"
                                        }]
                                    }),
                                ),
                                (
                                    "call_pr_done",
                                    local_product::patient_reviews::COMPLETE_PATIENT_REVIEW_TOOL,
                                    json!({
                                        "disposition": "assessed",
                                        "summary": "Potassium 5.8 mmol/L and A1c 8.1% reviewed ahead of CKD visit.",
                                        "citations": [{
                                            "candidate_id": "Observation/ckd-k-1",
                                            "revision": "rev-sw-1"
                                        }],
                                        "basis_and_limitations": "Synthetic chart panel (2026-08-26).",
                                        "unfinished_work": []
                                    }),
                                ),
                            ],
                        ),
                        8 => sse_assistant_reply("Completed scheduled patient review."),
                        other => panic!("unexpected SSE step {other}"),
                    };
                    (
                        StatusCode::OK,
                        [("content-type", "text/event-stream")],
                        sse,
                    )
                }
            }),
        );
        let stub_task = tokio::spawn(async move {
            axum::serve(listener, stub).await.unwrap();
        });

        let roles = ModelRoles {
            chat: ModelBinding {
                model: "chat-interactive-lane".into(),
                effort: "low".into(),
                context_window_tokens: 128_000,
                max_input_tokens: 100_000,
                adapter: "openai-responses".into(),
            },
            background: ModelBinding {
                model: "background-worker-lane".into(),
                effort: "high".into(),
                context_window_tokens: 272_000,
                max_input_tokens: 250_000,
                adapter: "openai-responses".into(),
            },
        };
        let model = ModelSource {
            transport: ModelTransport::Responses {
                endpoint: format!("http://127.0.0.1:{port}/responses"),
                key: "local-proof".into(),
            },
            roles: roles.clone(),
        };
        let quota: Arc<dyn harness::provider::QuotaGate> = Arc::new(store.clone());
        let mut wakeups = store.wakeups().await.unwrap();

        let bus = physician_bus::from_env(|_| None, false).await.unwrap();
        let wake = Arc::new(crate::WakeService {
            auth: crate::WakeAuth::LoopbackEmulator,
            mode: crate::WakeMode::RehearsalAllowed,
            handler: Arc::new(crate::ScheduledWakeHandler {
                dispatch: crate::StoreWakeHandler {
                    store: store.clone(),
                    auth: auth.clone(),
                    roles: roles.clone(),
                    skills: Err(local_product::physician_chat::SkillsError::Unconfigured),
                },
                visit_prep: crate::VisitPrepWakeHandler {
                    store: store.clone(),
                    auth: auth.clone(),
                    bus: Arc::clone(&bus),
                    model: model.clone(),
                    quota: quota.clone(),
                    skills_dir: None,
                    calendar: Some(crate::PracticeSyntheticCalendar {
                        practice: practice.clone(),
                        source: calendar,
                    }),
                    elation: None,
                },
            }),
            clinical_library: None,
        });
        let worker = Arc::new(Worker {
            store: store.clone(),
            auth: auth.clone(),
            bus,
            model,
            quota,
            pace: Pace {
                poll: Duration::from_secs(1),
                lease: 60,
                concurrency: 4,
                backoff: Backoff::IMMEDIATE,
            },
            wake,
            origin: telemetry::genai::Origin::Simulation,
            simulation_practices: physician_sample::hosting::SimulationPractices::from_env(|_| {
                None
            })
            .unwrap(),
        });
        let app = router(Arc::clone(&worker));

        let wake_resp = app
            .oneshot(
                HttpRequest::builder()
                    .method("POST")
                    .uri("/schedule/wake")
                    .header("content-type", "application/json")
                    .body(Body::from("{}"))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(wake_resp.status(), StatusCode::OK);
        let wake_bytes = axum::body::to_bytes(wake_resp.into_body(), 64 * 1024)
            .await
            .unwrap();
        let wake_json: Value = serde_json::from_slice(&wake_bytes).unwrap();
        assert_eq!(wake_json["status"], "ok");
        assert_eq!(wake_json["reviews_started"], 1);
        assert_eq!(wake_json["practices_checked"], 1);
        assert_eq!(wake_json["briefings_prepared"], 1);
        assert_eq!(wake_json["due_patients_checked"], 2);

        let saved_briefing = store
            .visit_briefing(
                &VisitSlotKey {
                    practice: practice.clone(),
                    source_id: physician_sample::schedule::SOURCE_ID.into(),
                    appointment_id: "appt-sw-tomorrow".into(),
                    starts_at_unix,
                },
                &auth,
            )
            .await
            .unwrap()
            .expect("POST /schedule/wake saved tomorrow's visit briefing");
        assert!(
            saved_briefing
                .visit_card_orientation
                .contains("potassium 5.8")
        );

        let our_delivery = tokio::time::timeout(Duration::from_secs(5), async {
            loop {
                let d = wakeups
                    .next()
                    .await
                    .unwrap()
                    .expect("work notification stream open");
                if d.scope.practice == practice && d.scope.workspace == workspace {
                    return d;
                }
            }
        })
        .await
        .expect("announced practice-job wakeup for due patient review");
        assert_eq!(our_delivery.kind, DeliveryKind::PracticeJob);

        let (_stop_tx, stop_rx) = watch::channel(false);
        assert_eq!(
            run_delivery(&worker, &our_delivery, stop_rx).await,
            Next::Done
        );

        let reviews = store
            .patient_reviews(&practice, &workspace, &auth)
            .await
            .unwrap();
        assert_eq!(reviews.len(), 1);
        assert_eq!(reviews[0].state.status_str(), "completed");
        assert_eq!(reviews[0].actions.len(), 1);
        assert!(matches!(
            reviews[0].usage,
            local_product::runtime::reviews::ReviewUsage::Known { exchanges: 3, .. }
        ));

        stub_task.abort();
        drop(store);
        drop(auth);
        let cleanup = sea_orm::Database::connect(&admin_url).await.unwrap();
        crate::disposable::drop_database(&cleanup, &db_name).await;
        let _ = cleanup.close().await;
        let _ = std::fs::remove_dir_all(&temp_dir);
    }
}
