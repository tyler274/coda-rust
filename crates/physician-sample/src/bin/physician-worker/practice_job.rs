//! Fenced execution for practice-owned background jobs (`DeliveryKind::PracticeJob`).
//!
//! Runs with no physician session on the harness background lane
//! (`ModelRoles.background`), verifying practice patient membership through
//! [`physician_auth::AuthStore`] and attributing all events and saved outputs
//! to the practice and job.

use axum::http::StatusCode;
use harness::engine::{self, Cancellation, RunLimits};
use local_product::runtime::Store;
use local_product::runtime::delivery::{AttemptSettlement, Delivery};
use local_product::runtime::jobs::{PracticeJobClaim, PracticeJobHost};
use telemetry::genai::{self, RunContext};
use tracing::Instrument;

use super::attempt::{self, Claimed, Failure, Outcome, Push, Tally};
use super::{Hosted, Worker, absent_claim, announce};

const JOB: &str = "physician_practice_job";

/// Handles one practice-job wakeup inside its `physician.worker.attempt` span.
pub(super) async fn deliver(worker: &Worker, push: &Push) -> StatusCode {
    let span = attempt::span(push, JOB);
    handle(worker, &push.delivery, &span)
        .instrument(span.clone())
        .await
}

async fn handle(worker: &Worker, delivery: &Delivery, span: &tracing::Span) -> StatusCode {
    if worker
        .model
        .authority(worker.store.clone())
        .check_source_boundary(&delivery.scope)
        .await
        .is_err()
    {
        attempt::finish(span, JOB, None, Outcome::ClaimError, Some(Failure::Claim));
        return StatusCode::INTERNAL_SERVER_ERROR;
    }

    let claim = match worker
        .store
        .claim_practice_job_delivery(delivery, super::LEASE_SECONDS, &worker.auth)
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

    let workspace_id = claim.scope.workspace.clone();
    let job_state = worker.store.practice_job(&claim.scope).await.ok().flatten();
    let (origin, scenario_id) = worker.turn_origin(&claim.scope.practice, None);
    let input_text = job_state
        .as_ref()
        .map(|state| state.assignment.prompt.clone())
        .unwrap_or_default();
    let earlier_attempts = job_state.as_ref().map_or(0, |state| {
        let count = state
            .attempts
            .iter()
            .filter(|saved| saved.revision == claim.revision && saved.id != claim.attempt)
            .count();
        u32::try_from(count).unwrap_or(u32::MAX)
    });
    let claimed_attempt = job_state.as_ref().and_then(|state| {
        state
            .attempts
            .iter()
            .find(|saved| saved.id == claim.attempt)
    });

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
        .rehearsed_failure(&claim.scope.practice, "", earlier_attempts)
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
            let reason =
                engine::AttemptFailure::from_error(&engine::Failure::Provider(error), &cancel);
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
    let hosted = super::while_owned(
        claim.lease,
        super::STALL,
        &cancel,
        || worker.store.renew_practice_job_claim(&claim),
        host.run(&mut provider, RunLimits::default(), &cancel)
            .instrument(generation.clone()),
    )
    .await;
    record_practice_job_usage(&worker.store, &claim, &generation).await;

    let result = match hosted {
        Hosted::Returned(result) => result,
        Hosted::Lost => return super::lost(span, JOB, &claim.attempt),
    };
    match result {
        Ok(()) => {
            let output = worker
                .store
                .practice_job(&claim.scope)
                .await
                .ok()
                .flatten()
                .and_then(|saved| saved.result.map(|res| res.output_text))
                .unwrap_or_default();
            genai::record_output(&generation, &output);
            announce(worker, workspace_id).await;
            attempt::finish(span, JOB, Some(&claim.attempt), Outcome::Committed, None);
            StatusCode::NO_CONTENT
        }
        Err(error) => {
            let reason = engine::AttemptFailure::from_error(&error, &cancel);
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
    let Ok(Some(state)) = store.practice_job(&claim.scope).await else {
        return;
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
) -> StatusCode {
    let run_id = Some(claim.attempt.as_str());
    let settlement = worker
        .store
        .settle_practice_job_attempt(claim, reason)
        .await;
    let status = match settlement {
        Err(_) => {
            attempt::retry_not_persisted(span, JOB, &claim.attempt);
            attempt::finish(span, JOB, run_id, Outcome::Retry, Some(failure));
            return StatusCode::INTERNAL_SERVER_ERROR;
        }
        Ok(AttemptSettlement::Applied) if reason.counts_as_failure() => {
            genai::record_error(generation, "practice background job failed");
            attempt::finish(span, JOB, run_id, Outcome::Retry, Some(failure));
            StatusCode::INTERNAL_SERVER_ERROR
        }
        Ok(AttemptSettlement::Applied | AttemptSettlement::Superseded) => {
            attempt::finish(span, JOB, run_id, Outcome::Settled, None);
            StatusCode::NO_CONTENT
        }
    };
    announce(worker, workspace_id).await;
    status
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use std::sync::{Arc, Mutex};

    use axum::Router;
    use axum::body::Body;
    use axum::extract::Request as HttpRequest;
    use axum::http::StatusCode;
    use axum::routing::post;
    use base64::Engine;
    use local_product::physician_chat::PhysicianSkills;
    use local_product::runtime::delivery::DeliveryKind;
    use local_product::runtime::jobs::{Assignment, PracticeJobKind, PracticeScope};
    use local_product::runtime::{AttemptStatus, Resumed, Scope, Stopped, Store, WorkStatus};
    use local_product::{ModelBinding, ModelRoles};
    use physician_auth::{AuthStore, PostgresAuthStore, PracticeClock};
    use serde_json::{Value, json};
    use tower::ServiceExt;
    use uuid::Uuid;

    use super::super::{ModelSource, ModelTransport, Worker, router};

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
    async fn pubsub_push_executes_resumed_practice_job_on_background_lane_and_acks_redelivery() {
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

        // Stop and then Resume through Store::resume_authorized to obtain the broker Delivery.
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
                skills: Some(skills),
                queue: None,
            }),
            clinical_library: None,
        });
        let worker = Arc::new(Worker {
            store: store.clone(),
            auth,
            bus: physician_bus::from_env(|_| None, false).await.unwrap(),
            model: ModelSource {
                transport: ModelTransport::Responses {
                    endpoint: format!("http://127.0.0.1:{port}/responses"),
                    key: "local-proof".into(),
                },
                roles,
            },
            quota: Arc::new(store.clone()),
            verifier: None,
            wake,
            origin: telemetry::genai::Origin::Simulation,
            simulation_practices: physician_sample::hosting::SimulationPractices::from_env(|_| {
                None
            })
            .unwrap(),
        });
        let app = router(worker);

        let push_bytes = serde_json::to_vec(&json!({
            "message": {
                "data": base64::engine::general_purpose::STANDARD
                    .encode(serde_json::to_vec(&delivery).unwrap()),
                "messageId": "msg-practice-1",
            },
            "deliveryAttempt": 1,
        }))
        .unwrap();

        let resp1 = app
            .clone()
            .oneshot(
                HttpRequest::builder()
                    .method("POST")
                    .uri("/pubsub/chat-turn")
                    .header("content-type", "application/json")
                    .body(Body::from(push_bytes.clone()))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(resp1.status(), StatusCode::NO_CONTENT);

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

        // Redelivering the settled push acknowledges 204 without invoking the provider again.
        let resp2 = app
            .oneshot(
                HttpRequest::builder()
                    .method("POST")
                    .uri("/pubsub/chat-turn")
                    .header("content-type", "application/json")
                    .body(Body::from(push_bytes))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(resp2.status(), StatusCode::NO_CONTENT);
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

    struct RecordingQueue(Mutex<Vec<local_product::runtime::delivery::Delivery>>);

    #[local_product::runtime::async_trait]
    impl physician_sample::queue::ChatQueue for RecordingQueue {
        async fn publish(
            &self,
            delivery: &local_product::runtime::delivery::Delivery,
        ) -> Result<(), String> {
            self.0.lock().unwrap().push(delivery.clone());
            Ok(())
        }
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
            pulled_at: 1_100_000,
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
        let queue = Arc::new(RecordingQueue(Mutex::new(Vec::new())));

        let wake = Arc::new(crate::WakeService {
            auth: crate::WakeAuth::LoopbackEmulator,
            mode: crate::WakeMode::RehearsalAllowed,
            handler: Arc::new(crate::StoreWakeHandler {
                store: store.clone(),
                auth: auth.clone(),
                roles: roles.clone(),
                skills: Some(skills),
                queue: Some(queue.clone()),
            }),
            clinical_library: None,
        });
        let worker = Arc::new(Worker {
            store: store.clone(),
            auth,
            bus: physician_bus::from_env(|_| None, false).await.unwrap(),
            model: ModelSource {
                transport: ModelTransport::Responses {
                    endpoint: format!("http://127.0.0.1:{port}/responses"),
                    key: "local-proof".into(),
                },
                roles,
            },
            quota: Arc::new(store.clone()),
            verifier: None,
            wake,
            origin: telemetry::genai::Origin::Simulation,
            simulation_practices: physician_sample::hosting::SimulationPractices::from_env(|_| {
                None
            })
            .unwrap(),
        });
        let app = router(worker);

        // 1. Hit `POST /schedule/wake` -> dispatches our due patient's review and publishes to `ChatQueue`!
        let wake_resp = app
            .clone()
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

        let our_delivery = queue
            .0
            .lock()
            .unwrap()
            .iter()
            .find(|d| d.scope.practice == practice && d.scope.workspace == workspace)
            .cloned()
            .expect("published practice-job wakeup for our workspace");
        assert_eq!(our_delivery.kind, DeliveryKind::PracticeJob);

        // 2. Deliver the published wakeup to `POST /pubsub/chat-turn`!
        let push_bytes = serde_json::to_vec(&json!({
            "message": {
                "data": base64::engine::general_purpose::STANDARD
                    .encode(serde_json::to_vec(&our_delivery).unwrap()),
                "messageId": "msg-wake-review-1",
            },
            "deliveryAttempt": 1,
        }))
        .unwrap();
        let push_resp = app
            .oneshot(
                HttpRequest::builder()
                    .method("POST")
                    .uri("/pubsub/chat-turn")
                    .header("content-type", "application/json")
                    .body(Body::from(push_bytes))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(push_resp.status(), StatusCode::NO_CONTENT);

        // 3. Verify `SavedPatientReview` is durably Completed with Known usage!
        let reviews = store.patient_reviews(&practice, &workspace).await.unwrap();
        assert_eq!(reviews.len(), 1);
        assert_eq!(reviews[0].state.status_str(), "completed");
        assert_eq!(reviews[0].actions.len(), 1);
        assert!(matches!(
            reviews[0].usage,
            local_product::runtime::reviews::ReviewUsage::Known { exchanges: 3, .. }
        ));

        stub_task.abort();
    }
}
