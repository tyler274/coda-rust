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
    use axum::http::StatusCode;
    use axum::routing::post;
    use local_product::physician_chat::PhysicianSkills;
    use local_product::runtime::delivery::{Backoff, DeliveryKind};
    use local_product::runtime::jobs::{Assignment, PracticeJobKind, PracticeScope};
    use local_product::runtime::{AttemptStatus, Resumed, Scope, Stopped, Store, WorkStatus};
    use local_product::{ModelBinding, ModelRoles};
    use physician_auth::{AuthStore, PostgresAuthStore, PracticeClock};
    use serde_json::{Value, json};
    use tokio::sync::watch;
    use uuid::Uuid;

    use super::super::attempt::Next;
    use super::super::{ModelSource, ModelTransport, Pace, Worker, run_delivery};

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
            "usage": {"input_tokens": 24, "output_tokens": 12},
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
            }),
            clinical_library: None,
        });
        let worker = Worker {
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
}
