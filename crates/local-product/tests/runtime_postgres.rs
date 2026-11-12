//! Real Postgres proof. Run explicitly with PHYSICIAN_RUNTIME_TEST_DATABASE_URL.
use engine::recipe::{Observer, Request};
use local_product::runtime::delivery::Backoff;
use local_product::runtime::{
    Accepted, AttemptStatus, Authority, Input, PhysicianHost, RuntimeError, Scope, Store,
    WorkStatus, async_trait, engine,
};
use serde_json::json;

fn turn(text: &str) -> engine::Turn {
    engine::Turn {
        items: vec![engine::Item::assistant(text)],
        receipt: json!({"controlled":true}),
        usage: Default::default(),
    }
}

fn scope() -> Scope {
    Scope {
        practice: uuid::Uuid::new_v4().to_string(),
        physician: "physician-a".into(),
        workspace: "patient-a".into(),
        chat: "chat-a".into(),
    }
}
fn input() -> Input {
    Input {
        key: "command-a".into(),
        text: "Exact input\n  whitespace".into(),
        timezone: "America/Los_Angeles".into(),
        scenario: None,
    }
}
async fn store() -> Store {
    let url = std::env::var("PHYSICIAN_RUNTIME_TEST_DATABASE_URL")
        .expect("explicit local Postgres URL required");
    let store = Store::connect(&url).await.unwrap();
    store.migrate().await.unwrap();
    store.migrate().await.unwrap();
    store
}
// A hosted attempt includes the hosting settlement, outside the engine. Assert
// Applied so an accidental engine-side interruption cannot mask classification.
async fn hosted_attempt<A: Authority, P: engine::recipe::Provider>(
    host: &mut PhysicianHost<A>,
    provider: &mut P,
    limits: engine::RunLimits,
    cancel: &engine::Cancellation,
) -> Result<(), engine::Failure> {
    let result = host.run(provider, limits, cancel).await;
    if let Err(error) = &result {
        assert_eq!(
            host.store
                .settle_chat_attempt(
                    &host.claim,
                    engine::AttemptFailure::from_error(error, cancel),
                    Backoff::IMMEDIATE
                )
                .await
                .unwrap(),
            local_product::runtime::delivery::AttemptSettlement::Applied
        );
    }
    result
}

#[tokio::test]
#[ignore = "requires dedicated Postgres cluster and PHYSICIAN_RUNTIME_TEST_PGDATA"]
async fn database_crash_restart_retains_acknowledged_command_and_event() {
    let pgdata = std::env::var("PHYSICIAN_RUNTIME_TEST_PGDATA")
        .expect("dedicated disposable cluster required");
    assert!(
        pgdata.starts_with("/tmp/physician-restart-"),
        "refuse to restart an unrelated database"
    );
    let s = store().await;
    let scope = scope();
    let command = input();
    accept(&s, &scope, &command).await.unwrap();
    let claim = s.claim(&scope, 60).await.unwrap().unwrap();
    let kinds = [
        engine::EventKind::ProviderReturned(engine::Turn {
            usage: Default::default(),
            items: vec![engine::Item::Message {
                role: "assistant".into(),
                text: "durable candidate".into(),
            }],
            receipt: json!({"controlled": true}),
        }),
        engine::EventKind::Candidate {
            text: "durable candidate".into(),
        },
    ];
    for (sequence, kind) in (1..).zip(kinds) {
        s.persist(
            &claim,
            engine::Event {
                attempt_id: claim.attempt.clone(),
                ordinal: 1,
                sequence,
                kind,
            },
        )
        .await
        .unwrap();
    }
    drop(s);
    assert!(
        std::process::Command::new("pg_ctl")
            .args(["-D", &pgdata, "-m", "immediate", "stop"])
            .status()
            .unwrap()
            .success()
    );
    assert!(
        std::process::Command::new("pg_ctl")
            .args([
                "-D",
                &pgdata,
                "-l",
                "/tmp/physician-restart-physician-postgres.log",
                "-o",
                "-p 55489 -h 127.0.0.1 -k /tmp",
                "start"
            ])
            .status()
            .unwrap()
            .success()
    );
    let s = store().await;
    assert_eq!(
        accept(&s, &scope, &command).await.unwrap(),
        Accepted::Duplicate
    );
    let state = s.conversation(&scope).await.unwrap();
    assert_eq!(state.messages.len(), 1);
    assert_eq!(state.attempts[0].events.len(), 2);
    s.commit_authorized(&claim, &turn("durable candidate"), &mut Allowed)
        .await
        .unwrap();
    assert_eq!(s.conversation(&scope).await.unwrap().messages.len(), 2);
}

struct Allowed;
#[async_trait]
impl Authority for Allowed {
    async fn lock_publication(
        &mut self,
        _: &local_product::runtime::PublicationTransaction,
        _: &Scope,
        _: Option<&[u8]>,
    ) -> Result<(), String> {
        Ok(())
    }
    async fn check(
        &mut self,
        _: &local_product::runtime::PublicationTransaction,
        _: &Scope,
        _: Option<&[u8]>,
    ) -> Result<(), String> {
        Ok(())
    }
}
struct PublicationInterrupted;
#[async_trait]
impl Authority for PublicationInterrupted {
    async fn check(
        &mut self,
        _: &local_product::runtime::PublicationTransaction,
        _: &Scope,
        _: Option<&[u8]>,
    ) -> Result<(), String> {
        Ok(())
    }
    async fn lock_publication(
        &mut self,
        _: &local_product::runtime::PublicationTransaction,
        _: &Scope,
        _: Option<&[u8]>,
    ) -> Result<(), String> {
        Err("synthetic publication interrupted after the paid return".into())
    }
}

struct Provider;
impl engine::recipe::Provider for Provider {
    async fn exchange(
        &mut self,
        request: &Request,
        _: &engine::Cancellation,
        _: &engine::RunLimits,
        _: &mut dyn Observer,
    ) -> Result<engine::Turn, engine::Cause> {
        assert!(request.items.iter().any(|i| matches!(i, engine::Item::Message { role, text } if role == "user" && text.contains("Exact input"))));
        Ok(engine::Turn {
            usage: Default::default(),
            items: vec![engine::Item::Message {
                role: "assistant".into(),
                text: "Controlled response".into(),
            }],
            receipt: json!({"controlled": true}),
        })
    }
}
struct MustNotDispatch;
impl engine::recipe::Provider for MustNotDispatch {
    async fn exchange(
        &mut self,
        _: &Request,
        _: &engine::Cancellation,
        _: &engine::RunLimits,
        _: &mut dyn Observer,
    ) -> Result<engine::Turn, engine::Cause> {
        panic!("provider dispatched without durable event acknowledgment")
    }
}

#[tokio::test]
#[ignore = "requires explicit local Postgres"]
async fn successor_replaces_the_abandoned_draft_and_reuses_the_confirmed_read() {
    use engine::recipe::{Provider, StreamEvent};
    struct Interrupted {
        dispatches: usize,
        interrupted_request: Option<Request>,
    }
    impl Provider for Interrupted {
        async fn exchange(
            &mut self,
            request: &Request,
            _: &engine::Cancellation,
            _: &engine::RunLimits,
            observations: &mut dyn Observer,
        ) -> Result<engine::Turn, engine::Cause> {
            self.dispatches += 1;
            if self.dispatches == 1 {
                return Ok(engine::Turn {
                    items: vec![engine::Item::Call(engine::Invocation {
                        id: "confirmed-read".into(),
                        name: "skill_view".into(),
                        arguments: json!({"name":"patient-record-retrieval"}).to_string(),
                    })],
                    receipt: json!({"id":"read-turn"}),
                    usage: Default::default(),
                });
            }
            assert!(request.items.iter().any(|item| matches!(item, engine::Item::MaterializedResult { call_id, output } if call_id == "confirmed-read" && output.contains("available"))));
            self.interrupted_request = Some(request.clone());
            observations
                .emit(StreamEvent::Text("Unpublished draft. ".into()))
                .await?;
            Err("synthetic visible transport disconnect".into())
        }
    }
    struct Successor {
        dispatches: usize,
        expected_request: Request,
    }
    impl Provider for Successor {
        async fn exchange(
            &mut self,
            request: &Request,
            _: &engine::Cancellation,
            _: &engine::RunLimits,
            _: &mut dyn Observer,
        ) -> Result<engine::Turn, engine::Cause> {
            self.dispatches += 1;
            assert_eq!(self.dispatches, 1);
            assert!(request.items.iter().any(|item| matches!(item, engine::Item::MaterializedResult { call_id, output } if call_id == "confirmed-read" && output.contains("available"))));
            assert_eq!(
                request, &self.expected_request,
                "the saved request keeps its confirmed read and excludes the abandoned draft"
            );
            Ok(turn("Complete replacement answer."))
        }
    }
    let s = store().await;
    let scope = scope();
    accept(&s, &scope, &input()).await.unwrap();
    let claim = s.claim(&scope, 60).await.unwrap().unwrap();
    let mut host = PhysicianHost {
        store: s.clone(),
        claim: claim.clone(),
        authority: Allowed,
    };
    let mut first = Interrupted {
        dispatches: 0,
        interrupted_request: None,
    };
    assert!(
        host.run(
            &mut first,
            harness::engine::RunLimits::default(),
            &engine::Cancellation::default()
        )
        .await
        .is_err()
    );
    s.settle_chat_attempt(
        &claim,
        harness::engine::AttemptFailure::Failed,
        Backoff::IMMEDIATE,
    )
    .await
    .unwrap();
    drop(host);
    drop(s);
    // The browser and first worker can disappear; only the durable owner is read.
    let reopened = store().await;
    let saved = reopened.conversation(&scope).await.unwrap();
    assert_eq!(
        saved.messages.len(),
        1,
        "provisional text is not a published message"
    );
    let successor = reopened.claim(&scope, 60).await.unwrap().unwrap();
    let mut host = PhysicianHost {
        store: reopened.clone(),
        claim: successor,
        authority: Allowed,
    };
    let mut provider = Successor {
        dispatches: 0,
        expected_request: first.interrupted_request.unwrap(),
    };
    host.run(
        &mut provider,
        harness::engine::RunLimits::default(),
        &engine::Cancellation::default(),
    )
    .await
    .unwrap();
    let saved = reopened.conversation(&scope).await.unwrap();
    assert_eq!(saved.messages.len(), 2);
    assert_eq!(
        saved.messages.last().unwrap().text,
        "Complete replacement answer."
    );
    assert_eq!(
        saved.failures, 1,
        "steering stays inside the same run and does not reset its failure count"
    );
    assert_eq!(saved.attempts.iter().flat_map(|attempt| &attempt.events).filter(|event| matches!(&event.kind, engine::EventKind::ToolReturned { call_id, .. } if call_id == "confirmed-read")).count(), 1);
    assert_eq!(saved.status, WorkStatus::Completed);
}

#[tokio::test]
#[ignore = "requires explicit local Postgres"]
async fn a_read_recorded_after_a_steer_is_retained_by_the_successor() {
    use sea_orm::{ConnectionTrait, DatabaseBackend, Statement};
    struct Read;
    impl engine::recipe::Provider for Read {
        async fn exchange(
            &mut self,
            _: &Request,
            _: &engine::Cancellation,
            _: &engine::RunLimits,
            _: &mut dyn Observer,
        ) -> Result<engine::Turn, engine::Cause> {
            let mut output = turn("");
            output.items = vec![read(
                "steering-late-read",
                local_product::physician_sources::READ_TOOL,
                r#"{"references":[{"candidate_id":"late-read-lab","revision":"r1"}],"max_result_bytes":65536}"#,
            )];
            Ok(output)
        }
    }
    struct Successor;
    impl engine::recipe::Provider for Successor {
        async fn exchange(
            &mut self,
            request: &Request,
            _: &engine::Cancellation,
            _: &engine::RunLimits,
            _: &mut dyn Observer,
        ) -> Result<engine::Turn, engine::Cause> {
            assert!(request.items.iter().any(|item| matches!(item,
                engine::Item::MaterializedResult { call_id, output }
                if call_id == "steering-late-read" && output.contains("Potassium 4.1"))));
            assert!(request.items.iter().any(|item| matches!(item,
                engine::Item::Message { text, .. } if text == "Use the retained read")));
            Ok(turn("Completed using the retained read"))
        }
    }
    let s = store().await;
    let mut scope = scope();
    scope.workspace = uuid::Uuid::new_v4().to_string();
    s.record_source(&local_product::physician_sources::SourceRevision {
        workspace: scope.workspace.clone(),
        source_id: "ehr".into(),
        candidate_id: "late-read-lab".into(),
        revision: "r1".into(),
        record_type: "Observation".into(),
        dates: vec![],
        content: "Potassium 4.1".into(),
        sha256: local_product::sha256_hex(b"Potassium 4.1"),
        readable_parts: vec![],
        provenance: json!({"native_record":"late-read-lab"}),
    })
    .await
    .unwrap();
    accept(&s, &scope, &input()).await.unwrap();
    let claim = s.claim(&scope, 60).await.unwrap().unwrap();
    let db =
        sea_orm::Database::connect(std::env::var("PHYSICIAN_RUNTIME_TEST_DATABASE_URL").unwrap())
            .await
            .unwrap();
    // Trigger DDL has no SeaQuery owner. This fixture holds the real receipt
    // transaction, then rolls it back so the product can accept the steer.
    db.execute_unprepared("drop trigger if exists hold_chord_read on physician_runtime_events; drop function if exists hold_chord_read(); drop sequence if exists chord_read_barrier; create sequence chord_read_barrier; create function hold_chord_read() returns trigger language plpgsql as $$ begin if new.record->'ToolReturned'->>'call_id'='steering-late-read' and not exists(select 1 from physician_runtime_conversations c join physician_runtime_chat_attempts a on a.practice=c.practice and a.physician=c.physician and a.workspace=c.workspace and a.chat=c.chat where c.practice=new.practice and c.physician=new.physician and c.workspace=new.workspace and c.chat=new.target and a.id=new.attempt and c.revision>a.revision) then perform nextval('chord_read_barrier'); raise exception 'read receipt waits for steer' using errcode='40001'; end if; return new; end $$; create trigger hold_chord_read before insert on physician_runtime_events for each row execute function hold_chord_read();")
        .await.unwrap();
    let mut host = PhysicianHost {
        store: s.clone(),
        claim: claim.clone(),
        authority: Allowed,
    };
    let running = tokio::spawn(async move {
        host.run(
            &mut Read,
            engine::RunLimits::default(),
            &engine::Cancellation::default(),
        )
        .await
    });
    tokio::time::timeout(std::time::Duration::from_secs(5), async {
        loop {
            let row = db
                .query_one_raw(Statement::from_string(
                    DatabaseBackend::Postgres,
                    "select is_called from chord_read_barrier",
                ))
                .await
                .unwrap()
                .unwrap();
            if row.try_get::<bool>("", "is_called").unwrap() {
                break;
            }
            tokio::time::sleep(std::time::Duration::from_millis(5)).await;
        }
    })
    .await
    .unwrap();
    accept(
        &s,
        &scope,
        &Input {
            key: "steering-next".into(),
            text: "Use the retained read".into(),
            ..input()
        },
    )
    .await
    .unwrap();
    let error = running.await.unwrap().unwrap_err();
    s.settle_chat_attempt(
        &claim,
        engine::AttemptFailure::from_error(&error, &engine::Cancellation::default()),
        Backoff::IMMEDIATE,
    )
    .await
    .unwrap();
    let mut host = PhysicianHost {
        store: s.clone(),
        claim: s.claim(&scope, 60).await.unwrap().unwrap(),
        authority: Allowed,
    };
    host.run(
        &mut Successor,
        engine::RunLimits::default(),
        &engine::Cancellation::default(),
    )
    .await
    .unwrap();
    let saved = s.conversation(&scope).await.unwrap();
    assert_eq!(saved.status, WorkStatus::Completed);
    assert_eq!(
        saved.messages.last().unwrap().text,
        "Completed using the retained read"
    );
    let events: Vec<_> = saved
        .attempts
        .iter()
        .flat_map(|attempt| &attempt.events)
        .collect();
    assert_eq!(
        events
            .iter()
            .filter(|event| matches!(&event.kind,
        engine::EventKind::ToolStarted(call) if call.id == "steering-late-read"))
            .count(),
        1
    );
    assert_eq!(
        events
            .iter()
            .filter(|event| matches!(&event.kind,
        engine::EventKind::ToolReturned { call_id, .. } if call_id == "steering-late-read"))
            .count(),
        1
    );
    db.execute_unprepared("drop trigger hold_chord_read on physician_runtime_events; drop function hold_chord_read(); drop sequence chord_read_barrier;")
        .await.unwrap();
}

#[tokio::test]
#[ignore = "requires explicit local Postgres"]
async fn a_steered_run_that_reaches_its_limit_keeps_the_partial_answer() {
    // Reads once, then the physician steers while the next request runs.
    struct ReadThenSteered {
        store: Store,
        scope: Scope,
        dispatches: usize,
    }
    impl engine::recipe::Provider for ReadThenSteered {
        async fn exchange(
            &mut self,
            _: &Request,
            _: &engine::Cancellation,
            _: &engine::RunLimits,
            _: &mut dyn Observer,
        ) -> Result<engine::Turn, engine::Cause> {
            self.dispatches += 1;
            if self.dispatches == 1 {
                let mut output = turn("");
                output.items = vec![read(
                    "before-steer",
                    "skill_view",
                    r#"{"name":"patient-record-retrieval"}"#,
                )];
                return Ok(output);
            }
            let steer = Input {
                key: "steer".into(),
                text: "Answer from what you have.".into(),
                ..input()
            };
            accept(&self.store, &self.scope, &steer).await.unwrap();
            Ok(turn("Superseded draft."))
        }
    }
    struct Final;
    impl engine::recipe::Provider for Final {
        async fn exchange(
            &mut self,
            request: &Request,
            _: &engine::Cancellation,
            _: &engine::RunLimits,
            _: &mut dyn Observer,
        ) -> Result<engine::Turn, engine::Cause> {
            assert!(request.items.iter().any(|item| matches!(item,
                engine::Item::Message { role, text } if role == "developer" && text == engine::FINAL_REQUEST)));
            assert!(request.items.iter().any(|item| matches!(item,
                engine::Item::MaterializedResult { call_id, .. } if call_id == "before-steer")));
            Ok(turn("Partial answer after the steer."))
        }
    }
    let s = store().await;
    let scope = scope();
    accept(&s, &scope, &input()).await.unwrap();
    let claim = s.claim(&scope, 60).await.unwrap().unwrap();
    let mut steered = ReadThenSteered {
        store: s.clone(),
        scope: scope.clone(),
        dispatches: 0,
    };
    let error = PhysicianHost {
        store: s.clone(),
        claim: claim.clone(),
        authority: Allowed,
    }
    .run(
        &mut steered,
        skill_limits(),
        &engine::Cancellation::default(),
    )
    .await
    .unwrap_err();
    s.settle_chat_attempt(
        &claim,
        engine::AttemptFailure::from_error(&error, &engine::Cancellation::default()),
        Backoff::IMMEDIATE,
    )
    .await
    .unwrap();
    let saved = s.conversation(&scope).await.unwrap();
    assert_eq!(saved.revision, claim.revision + 1);
    let dispatched = saved.attempts[0]
        .events
        .iter()
        .filter(|event| matches!(event.kind, engine::EventKind::ProviderStarted { .. }))
        .count() as u64;
    // The successor's first request is the reserved final request.
    let successor = s.claim(&scope, 60).await.unwrap().unwrap();
    assert_eq!(successor.revision, claim.revision + 1);
    let result = PhysicianHost {
        store: s.clone(),
        claim: successor,
        authority: Allowed,
    }
    .run(
        &mut Final,
        engine::RunLimits {
            max_calls: dispatched + 1,
            ..skill_limits()
        },
        &engine::Cancellation::default(),
    )
    .await;
    assert!(
        result.is_ok(),
        "the final request refers to items saved before the steer: {result:?}"
    );
    let held = s.conversation(&scope).await.unwrap();
    assert_eq!(held.status, WorkStatus::Held);
    let last = held.messages.last().unwrap();
    assert!(last.partial());
    assert_eq!(last.text, "Partial answer after the steer.");
}

#[tokio::test]
#[ignore = "requires explicit local Postgres"]
async fn successor_publishes_retained_paid_answer_without_redispatch_or_duplicate_partial() {
    struct Paid;
    impl engine::recipe::Provider for Paid {
        async fn exchange(
            &mut self,
            _: &Request,
            _: &engine::Cancellation,
            _: &engine::RunLimits,
            observations: &mut dyn Observer,
        ) -> Result<engine::Turn, engine::Cause> {
            observations
                .emit(engine::recipe::StreamEvent::Text(
                    "Paid saved answer".into(),
                ))
                .await?;
            Ok(turn("Paid saved answer"))
        }
    }
    for final_request in [false, true] {
        let bounds = engine::RunLimits {
            max_calls: if final_request { 1 } else { 3 },
            ..limits()
        };
        let s = store().await;
        let scope = scope();
        accept(&s, &scope, &input()).await.unwrap();
        let claim = s.claim(&scope, 60).await.unwrap().unwrap();
        let mut host = PhysicianHost {
            store: s.clone(),
            claim: claim.clone(),
            authority: PublicationInterrupted,
        };
        assert!(
            host.run(&mut Paid, bounds, &engine::Cancellation::default())
                .await
                .is_err()
        );
        s.settle_chat_attempt(
            &claim,
            harness::engine::AttemptFailure::Failed,
            Backoff::IMMEDIATE,
        )
        .await
        .unwrap();
        drop(s);
        let reopened = store().await;
        let successor = reopened.claim(&scope, 60).await.unwrap().unwrap();
        let mut host = PhysicianHost {
            store: reopened.clone(),
            claim: successor,
            authority: Allowed,
        };
        host.run(
            &mut MustNotDispatch,
            bounds,
            &engine::Cancellation::default(),
        )
        .await
        .unwrap();
        let saved = reopened.conversation(&scope).await.unwrap();
        assert_eq!(
            saved.status,
            if final_request {
                WorkStatus::Held
            } else {
                WorkStatus::Completed
            }
        );
        assert_eq!(saved.messages[1].partial(), final_request);
        assert_eq!(saved.failures, 1);
        assert_eq!(saved.attempts.len(), 2);
        assert_eq!(
            saved.attempts[1].status,
            if final_request {
                AttemptStatus::Held
            } else {
                AttemptStatus::Completed
            }
        );
        assert_eq!(saved.messages.len(), 2);
        assert_eq!(saved.messages[1].text, "Paid saved answer");
        let visible = saved.messages;
        assert_eq!(
            visible
                .iter()
                .filter(|message| message.role.as_str() == "assistant")
                .count(),
            1
        );
        assert_eq!(
            saved
                .attempts
                .iter()
                .flat_map(|attempt| &attempt.events)
                .filter(|event| matches!(event.kind, engine::EventKind::ProviderReturned(_)))
                .count(),
            1
        );
        assert!(
            !visible
                .iter()
                .any(|message| message.role.as_str() == "mechanical")
        );
    }
}

#[tokio::test]
#[ignore = "requires explicit local Postgres"]
async fn resume_after_stop_does_not_replay_an_unpublished_answer_as_visible() {
    struct Answer(bool);
    impl engine::recipe::Provider for Answer {
        async fn exchange(
            &mut self,
            request: &Request,
            _: &engine::Cancellation,
            _: &engine::RunLimits,
            _: &mut dyn Observer,
        ) -> Result<engine::Turn, engine::Cause> {
            if self.0 {
                assert!(!request.items.iter().any(|i|matches!(i,engine::Item::Message{text,..} if text.contains("Unpublished full answer."))));
                return Ok(turn("Fresh resumed answer."));
            }
            Ok(turn("Unpublished full answer."))
        }
    }
    let s = store().await;
    let scope = scope();
    accept(&s, &scope, &input()).await.unwrap();
    let first = s.claim(&scope, 60).await.unwrap().unwrap();
    let mut host = PhysicianHost {
        store: s.clone(),
        claim: first,
        authority: PublicationInterrupted,
    };
    assert!(
        host.run(
            &mut Answer(false),
            harness::engine::RunLimits::default(),
            &engine::Cancellation::default()
        )
        .await
        .is_err()
    );
    let state = s.conversation(&scope).await.unwrap();
    assert_eq!(state.messages.len(), 1);
    assert!(state.attempts[0].events.iter().any(|e|matches!(&e.kind,engine::EventKind::ProviderReturned(t) if t.text()=="Unpublished full answer.")));
    s.stop_authorized(&scope, "stop-paid", vec![7; 32], &state.work)
        .await
        .unwrap();
    s.settle_chat_attempt(
        &host.claim,
        engine::AttemptFailure::Interrupted,
        Backoff::IMMEDIATE,
    )
    .await
    .unwrap();
    let held = s.conversation(&scope).await.unwrap();
    s.resume_authorized(
        &scope,
        "resume-paid",
        vec![7; 32],
        &held.work,
        held.run().unwrap().id,
        None,
    )
    .await
    .unwrap();
    let reopened = store().await;
    let mut successor = PhysicianHost {
        store: reopened.clone(),
        claim: reopened.claim(&scope, 60).await.unwrap().unwrap(),
        authority: Allowed,
    };
    successor
        .run(
            &mut Answer(true),
            harness::engine::RunLimits::default(),
            &engine::Cancellation::default(),
        )
        .await
        .unwrap();
    let published = reopened.conversation(&scope).await.unwrap().messages;
    assert_eq!(published.len(), 2);
    assert_eq!(published[1].text, "Fresh resumed answer.");
}

#[tokio::test]
#[ignore = "requires explicit local Postgres"]
async fn exhausted_provider_credit_holds_chat_atomically_before_worker_settlement() {
    let s = store().await;
    let scope = scope();
    accept(&s, &scope, &input()).await.unwrap();
    let claim = s.claim(&scope, 60).await.unwrap().unwrap();
    s.persist(
        &claim,
        engine::Event {
            attempt_id: claim.attempt.clone(),
            sequence: 1,
            ordinal: 1,
            kind: engine::EventKind::ProviderBlocked {
                reason: "Provider credits are exhausted. Restore billing before trying again."
                    .into(),
            },
        },
    )
    .await
    .unwrap();
    // The receipt transaction holds the work before hosting settles it.
    drop(s);
    let reopened = store().await;
    let state = reopened.conversation(&scope).await.unwrap();
    assert_eq!(state.status, WorkStatus::Held);
    assert_eq!(state.attempts[0].status, AttemptStatus::Held);
    assert_eq!(
        state.run().unwrap().outcome,
        Some(engine::runs::RunOutcome::Held {
            reason: engine::runs::HoldReason::ProviderBlocked
        })
    );
    assert_eq!(state.failures, 0);
    let blocked = state.attempts[0]
        .events
        .iter()
        .find(|event| matches!(event.kind, engine::EventKind::ProviderBlocked { .. }))
        .unwrap()
        .clone();
    assert!(
        matches!(&blocked.kind, engine::EventKind::ProviderBlocked { reason } if reason.contains("credits are exhausted") && reason.contains("Restore billing"))
    );
    // Lost COMMIT acknowledgment can replay the event without charging twice.
    reopened.persist(&claim, blocked).await.unwrap();
    reopened
        .settle_chat_attempt(
            &claim,
            harness::engine::AttemptFailure::Failed,
            Backoff::IMMEDIATE,
        )
        .await
        .unwrap();
    assert!(reopened.claim(&scope, 60).await.unwrap().is_none());
    let state = reopened.conversation(&scope).await.unwrap();
    assert_eq!(state.attempts.len(), 1);
    assert_eq!(state.failures, 0);
    assert_eq!(state.messages.len(), 1);
    assert_eq!(
        state.attempts[0].events.len(),
        1,
        "replayed block remains one receipt"
    );
    assert!(matches!(
        reopened
            .resume_authorized(
                &scope,
                "try-after-billing",
                vec![7; 32],
                &state.work,
                state.run().unwrap().id,
                None
            )
            .await
            .unwrap(),
        local_product::runtime::Resumed::Pending { .. }
    ));
    assert!(reopened.claim(&scope, 60).await.unwrap().is_some());
}

fn limits() -> engine::RunLimits {
    engine::RunLimits {
        max_response_bytes: 10_000,
        ..engine::RunLimits::default()
    }
}

#[tokio::test]
#[ignore = "requires explicit local Postgres"]
async fn durable_acceptance_dedup_tenants_successor_and_engine_commit() {
    let s = store().await;
    let scope = scope();
    let command = input();
    let accepted_recipe = test_chat_recipe("test instruction v1");
    let accepted_skills = skills();
    let (a, b) = tokio::join!(
        s.accept_authorized(
            &scope,
            &command,
            vec![0; 32],
            &accepted_skills,
            &accepted_recipe,
            None
        ),
        s.accept_authorized(
            &scope,
            &command,
            vec![0; 32],
            &accepted_skills,
            &accepted_recipe,
            None
        )
    );
    let outcomes = [a.unwrap(), b.unwrap()];
    assert!(outcomes.contains(&Accepted::New) && outcomes.contains(&Accepted::Duplicate));
    let mut collision = command.clone();
    collision.text = "changed".into();
    assert!(
        accept_with(
            &s,
            &scope,
            &collision,
            &test_chat_recipe("test instruction v1")
        )
        .await
        .is_err()
    );
    let mut other = scope.clone();
    other.practice = uuid::Uuid::new_v4().to_string();
    assert!(s.conversation(&other).await.is_err());
    assert_eq!(
        accept_with(
            &s,
            &other,
            &command,
            &test_chat_recipe("test instruction v1")
        )
        .await
        .unwrap(),
        Accepted::New
    );
    other = scope.clone();
    other.physician = "different-physician".into();
    assert!(s.conversation(&other).await.is_err());
    assert_eq!(
        accept_with(
            &s,
            &other,
            &command,
            &test_chat_recipe("test instruction v1")
        )
        .await
        .unwrap(),
        Accepted::New
    );
    let first = s.claim(&scope, 60).await.unwrap().unwrap();
    assert!(s.claim(&scope, 60).await.unwrap().is_none());
    s.settle_chat_attempt(
        &first,
        harness::engine::AttemptFailure::Interrupted,
        Backoff::IMMEDIATE,
    )
    .await
    .unwrap();
    drop(s);
    let s = store().await;
    assert_eq!(
        accept_with(
            &s,
            &scope,
            &command,
            &test_chat_recipe("test instruction v1")
        )
        .await
        .unwrap(),
        Accepted::Duplicate
    );
    let successor = s.claim(&scope, 60).await.unwrap().unwrap();
    assert_ne!(first.attempt, successor.attempt);
    let state = s.conversation(&scope).await.unwrap();
    assert_eq!(
        state.attempts[1].predecessor.as_deref(),
        Some(first.attempt.as_str())
    );
    assert_eq!(state.attempts[0].work, state.attempts[1].work);
    let mut host = PhysicianHost {
        store: s.clone(),
        claim: successor.clone(),
        authority: Allowed,
    };
    host.run(&mut Provider, limits(), &engine::Cancellation::default())
        .await
        .unwrap();
    drop(host);
    drop(s);
    let s = store().await;
    let state = s.conversation(&scope).await.unwrap();
    assert_eq!(state.messages.len(), 2);
    assert_eq!(state.status, WorkStatus::Completed);
    assert_eq!(
        accept_with(
            &s,
            &scope,
            &command,
            &test_chat_recipe("test instruction v1")
        )
        .await
        .unwrap(),
        Accepted::Duplicate
    );
    assert!(s.claim(&scope, 60).await.unwrap().is_none());
    s.settle_chat_attempt(
        &successor,
        harness::engine::AttemptFailure::Interrupted,
        Backoff::IMMEDIATE,
    )
    .await
    .unwrap();
    assert_eq!(
        s.conversation(&scope).await.unwrap().status,
        WorkStatus::Completed
    );
}

#[tokio::test]
#[ignore = "requires explicit local Postgres"]
async fn event_ack_stop_steering_and_late_candidate_fences() {
    let s = store().await;
    let scope = scope();
    accept(&s, &scope, &input()).await.unwrap();
    let claim = s.claim(&scope, 60).await.unwrap().unwrap();
    let paid = engine::Event {
        attempt_id: claim.attempt.clone(),
        ordinal: 1,
        sequence: 1,
        kind: engine::EventKind::ProviderReturned(turn("late")),
    };
    s.persist(&claim, paid.clone()).await.unwrap();
    s.persist(&claim, paid.clone()).await.unwrap();
    let reopened = store().await.conversation(&scope).await.unwrap();
    assert_eq!(reopened.attempts[0].events.len(), 1);
    assert_eq!(
        serde_json::to_value(&reopened.attempts[0].events[0]).unwrap(),
        serde_json::to_value(&paid).unwrap()
    );
    assert_eq!(
        reopened.messages.len(),
        1,
        "a paid receipt alone never publishes"
    );
    let event = engine::Event {
        attempt_id: claim.attempt.clone(),
        ordinal: 1,
        sequence: 2,
        kind: engine::EventKind::Candidate {
            text: "late".into(),
        },
    };
    let candidate = turn("late");
    assert!(
        s.commit_authorized(&claim, &candidate, &mut Allowed)
            .await
            .is_err()
    );
    s.persist(&claim, event.clone()).await.unwrap();
    s.persist(&claim, event.clone()).await.unwrap();
    let mut different = event.clone();
    different.kind = engine::EventKind::Candidate {
        text: "collision".into(),
    };
    assert!(s.persist(&claim, different).await.is_err());
    let mut gap = event.clone();
    gap.sequence = 4;
    assert!(s.persist(&claim, gap).await.is_err());
    let work = s.conversation(&scope).await.unwrap().work;
    assert_eq!(
        s.stop_authorized(&scope, "stop-current", vec![7; 32], &work)
            .await
            .unwrap(),
        local_product::runtime::Stopped::Held
    );
    assert!(
        s.commit_authorized(&claim, &candidate, &mut Allowed)
            .await
            .is_err()
    );
    assert!(s.persist(&claim, event).await.is_err());
    s.persist(
        &claim,
        engine::Event {
            attempt_id: claim.attempt.clone(),
            ordinal: 1,
            sequence: 3,
            kind: engine::EventKind::ProviderReturned(engine::Turn {
                usage: Default::default(),
                items: vec![],
                receipt: json!({"late": true}),
            }),
        },
    )
    .await
    .unwrap();
    assert_eq!(
        s.conversation(&scope).await.unwrap().status,
        WorkStatus::Held
    );
    drop(s);
    let s = store().await;
    assert!(s.claim(&scope, 60).await.unwrap().is_none());
    // A new message releases the hold and a successor answers it. The
    // stopped claim still cannot commit.
    let mut steer = input();
    steer.key = "steer".into();
    steer.text = "new instruction".into();
    accept(&s, &scope, &steer).await.unwrap();
    assert!(
        s.commit_authorized(&claim, &candidate, &mut Allowed)
            .await
            .is_err()
    );
    assert!(
        s.claim(&scope, 60).await.unwrap().is_none(),
        "a stopped worker must settle before new work starts"
    );
    s.settle_chat_attempt(
        &claim,
        engine::AttemptFailure::Interrupted,
        Backoff::IMMEDIATE,
    )
    .await
    .unwrap();
    let answering = s.claim(&scope, 60).await.unwrap().unwrap();
    assert_ne!(claim.attempt, answering.attempt);
    steer.key = "steer-again".into();
    accept(&s, &scope, &steer).await.unwrap();
    // The currency read can succeed immediately before a steer commits; each
    // write-time fence must report the same supersession to the harness.
    for cause in [
        s.admit_binding(
            &answering,
            &test_chat_recipe("Owner").bind(Some(scope.cache_key("chords-chat"))),
        )
        .await
        .unwrap_err(),
        s.persist(
            &answering,
            engine::Event {
                attempt_id: answering.attempt.clone(),
                ordinal: 1,
                sequence: 1,
                kind: engine::EventKind::Candidate {
                    text: "obsolete".into(),
                },
            },
        )
        .await
        .unwrap_err(),
        s.commit_authorized(&answering, &candidate, &mut Allowed)
            .await
            .unwrap_err(),
        s.retain_partial_authorized(&answering, &candidate, &mut Allowed)
            .await
            .unwrap_err(),
    ] {
        assert_eq!(
            engine::AttemptFailure::from_cause(cause.as_ref()),
            engine::AttemptFailure::Interrupted,
            "write-time currency fences must classify a steer as interrupted"
        );
    }
    let cause = s.assert_current(&answering).await.unwrap_err();
    let reason = engine::AttemptFailure::from_cause(cause.as_ref());
    assert_eq!(
        reason,
        engine::AttemptFailure::Interrupted,
        "the harness must classify a Chords steer before product settlement"
    );
    assert!(
        s.commit_authorized(&answering, &candidate, &mut Allowed)
            .await
            .is_err()
    );
    let delivery =
        local_product::runtime::delivery::Delivery::chat_turn(scope.clone(), steer.key.clone());
    assert_eq!(
        s.delivery_state(&delivery).await.unwrap(),
        local_product::runtime::delivery::DeliveryState::Due,
        "a pending steer wakeup must stay due while its predecessor drains"
    );
    s.renew_claim(&answering)
        .await
        .expect("a draining attempt keeps its lease without publication authority");

    assert!(
        s.claim(&scope, 60).await.unwrap().is_none(),
        "a steered worker must drain before its successor starts"
    );
    let start = Scope {
        practice: scope.practice.clone(),
        physician: String::new(),
        workspace: String::new(),
        chat: String::new(),
    };
    let ours = |page: local_product::runtime::delivery::DispatchPage| -> Vec<_> {
        page.deliveries
            .into_iter()
            .filter(|d| d.scope == scope)
            .collect()
    };
    assert!(
        ours(s.dispatch_due(100, Some(&start)).await.unwrap()).is_empty(),
        "a steer is not due while its predecessor drains"
    );
    let settlement = s
        .settle_chat_attempt(&answering, reason, Backoff::IMMEDIATE)
        .await
        .unwrap();
    assert_eq!(
        ours(s.dispatch_due(100, Some(&start)).await.unwrap()),
        vec![delivery],
        "settlement makes the waiting steer due for the recovery sweep"
    );
    assert_eq!(
        settlement,
        local_product::runtime::delivery::AttemptSettlement::Applied
    );
    let settled = s.conversation(&scope).await.unwrap();
    assert_eq!(settled.failures, 0, "a steer consumes no failure allowance");
    assert_eq!(
        settled.attempts.last().unwrap().status,
        AttemptStatus::Interrupted
    );
    let next = s.claim(&scope, 60).await.unwrap().unwrap();
    assert_ne!(answering.attempt, next.attempt);
    assert_eq!(s.conversation(&scope).await.unwrap().messages.len(), 3);
}

fn skills() -> local_product::physician_chat::PhysicianSkills {
    local_product::physician_chat::PhysicianSkills::load(
        &std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../skills"),
    )
    .unwrap()
}

fn read(id: &str, name: &str, arguments: &str) -> engine::Item {
    engine::Item::Call(engine::Invocation {
        id: id.into(),
        name: name.into(),
        arguments: arguments.into(),
    })
}

async fn accept_history_assignment(
    store: &Store,
    scope: &Scope,
    recipe: &engine::recipe::RequestRecipe,
) {
    for index in 0..4 {
        let command = Input {
            key: format!("earlier-{index}"),
            text: format!("Earlier exact {index}: {}", "x".repeat(80)),
            timezone: "UTC".into(),
            scenario: None,
        };
        accept(store, scope, &command).await.unwrap();
        let claim = store.claim(scope, 60).await.unwrap().unwrap();
        let answer = turn(&format!("Earlier answer {index}: {}", "y".repeat(80)));
        for (sequence, kind) in [
            engine::EventKind::ProviderReturned(answer.clone()),
            engine::EventKind::Candidate {
                text: answer.text(),
            },
        ]
        .into_iter()
        .enumerate()
        {
            store
                .persist(
                    &claim,
                    engine::Event {
                        attempt_id: claim.attempt.clone(),
                        sequence: sequence as u64 + 1,
                        ordinal: 1,
                        kind,
                    },
                )
                .await
                .unwrap();
        }
        store
            .commit_authorized(&claim, &answer, &mut Allowed)
            .await
            .unwrap();
    }
    // One earlier work reaches its request limit, so the product holds its
    // final answer as a partial.
    struct HeldPartial;
    impl engine::recipe::Provider for HeldPartial {
        async fn exchange(
            &mut self,
            _: &Request,
            _: &engine::Cancellation,
            _: &engine::RunLimits,
            _: &mut dyn Observer,
        ) -> Result<engine::Turn, engine::Cause> {
            Ok(turn("Held partial answer: the last check did not finish."))
        }
    }
    accept(
        store,
        scope,
        &Input {
            key: "held".into(),
            text: "Earlier request that reached its limit.".into(),
            timezone: "UTC".into(),
            scenario: None,
        },
    )
    .await
    .unwrap();
    PhysicianHost {
        store: store.clone(),
        claim: store.claim(scope, 60).await.unwrap().unwrap(),
        authority: Allowed,
    }
    .run(
        &mut HeldPartial,
        engine::RunLimits {
            max_calls: 1,
            ..skill_limits()
        },
        &engine::Cancellation::default(),
    )
    .await
    .unwrap();
    accept_with(
        store,
        scope,
        &Input {
            key: "current".into(),
            text: format!("Current exact request: {}", "z".repeat(80)),
            timezone: "UTC".into(),
            scenario: None,
        },
        recipe,
    )
    .await
    .unwrap();
}

#[tokio::test]
#[ignore = "requires explicit local Postgres"]
async fn first_final_without_a_result_rearms_the_exact_admitted_recipe() {
    struct EarlierPartial;
    impl engine::recipe::Provider for EarlierPartial {
        async fn exchange(
            &mut self,
            _: &Request,
            _: &engine::Cancellation,
            _: &engine::RunLimits,
            _: &mut dyn Observer,
        ) -> Result<engine::Turn, engine::Cause> {
            Ok(turn("Partial answer: earlier work remains visible."))
        }
    }
    struct NoFinalReturn;
    impl engine::recipe::Provider for NoFinalReturn {
        async fn exchange(
            &mut self,
            request: &Request,
            _: &engine::Cancellation,
            _: &engine::RunLimits,
            _: &mut dyn Observer,
        ) -> Result<engine::Turn, engine::Cause> {
            assert_eq!(request.model, "controlled");
            assert_eq!(request.options, json!({"effort":"none"}));
            assert_eq!(request.context_window_tokens, 272_000);
            assert_eq!(request.max_input_tokens, 272_000);
            assert_eq!(
                request.instructions,
                test_chat_recipe("original accepted recipe").instructions
            );
            assert!(!request.tools.is_empty());
            Err("synthetic interruption before the first final result".into())
        }
    }
    struct RearmedRecipe(Request);
    impl engine::recipe::Provider for RearmedRecipe {
        async fn exchange(
            &mut self,
            request: &Request,
            _: &engine::Cancellation,
            _: &engine::RunLimits,
            _: &mut dyn Observer,
        ) -> Result<engine::Turn, engine::Cause> {
            assert_eq!(request.model, self.0.model);
            assert_eq!(request.options, self.0.options);
            assert_eq!(request.instructions, self.0.instructions);
            assert_eq!(request.cache_key, self.0.cache_key);
            assert_eq!(request.tools, self.0.tools);
            assert!(!request.tools.is_empty());
            assert!(!request.items.iter().any(|item| matches!(item, engine::Item::Message { text, .. } if text == engine::FINAL_REQUEST)));
            assert_eq!(request.items.iter().filter(|item| matches!(item, engine::Item::Message { role, text } if role == "assistant" && text.starts_with("Partial answer:"))).count(), 1, "an earlier work partial remains history, not this work's rearm output");
            Ok(turn("Complete after a failed first final request."))
        }
    }
    let store = store().await;
    let scope = scope();
    // A partial from an earlier held work remains ordinary conversation history.
    accept_with(
        &store,
        &scope,
        &Input {
            key: "earlier-work".into(),
            ..input()
        },
        &test_chat_recipe("earlier work"),
    )
    .await
    .unwrap();
    let earlier = store.claim(&scope, 60).await.unwrap().unwrap();
    PhysicianHost {
        store: store.clone(),
        claim: earlier,
        authority: Allowed,
    }
    .run(
        &mut EarlierPartial,
        engine::RunLimits {
            max_calls: 1,
            ..skill_limits()
        },
        &engine::Cancellation::default(),
    )
    .await
    .unwrap();
    let mut configured_recipe = test_chat_recipe("original accepted recipe");
    accept_with(&store, &scope, &input(), &configured_recipe)
        .await
        .unwrap();
    configured_recipe.model = "different-model-after-acceptance".into();
    configured_recipe.instructions = "Different instructions after acceptance".into();
    configured_recipe.options = json!({"effort":"high"});
    configured_recipe.context_window_tokens = 200_000;
    configured_recipe.max_input_tokens = 150_000;
    configured_recipe.tools.clear();
    assert_eq!(
        accept_with(&store, &scope, &input(), &configured_recipe)
            .await
            .unwrap(),
        Accepted::Duplicate
    );
    fail_earlier_attempts(&store, &scope).await;
    let accepted_before_claim = store.conversation(&scope).await.unwrap();
    assert!(
        accepted_before_claim
            .bindings
            .contains_key(&accepted_before_claim.work),
        "the recipe must be frozen at acceptance, before any worker claims it"
    );
    let first = store.claim(&scope, 60).await.unwrap().unwrap();
    let limits = engine::RunLimits {
        max_calls: 1,
        max_response_bytes: 100_000,
        ..engine::RunLimits::default()
    };
    let error = PhysicianHost {
        store: store.clone(),
        claim: first.clone(),
        authority: Allowed,
    }
    .run(&mut NoFinalReturn, limits, &engine::Cancellation::default())
    .await
    .unwrap_err();
    assert_eq!(
        store
            .settle_chat_attempt(
                &first,
                engine::AttemptFailure::from_error(&error, &engine::Cancellation::default()),
                Backoff::IMMEDIATE,
            )
            .await
            .unwrap(),
        local_product::runtime::delivery::AttemptSettlement::Applied,
    );
    let held = store.conversation(&scope).await.unwrap();
    assert_eq!(held.status, WorkStatus::Held);
    assert_eq!(
        held.failures,
        harness::engine::MAX_FAILED_ATTEMPTS - 1,
        "a failed reserved final cannot add a failure after reaching the limit"
    );
    assert_eq!(held.messages.len(), 3);
    let accepted = store
        .frozen_binding(&scope, held.bindings.get(&held.work).unwrap())
        .await
        .unwrap()
        .request(Vec::new());
    assert!(
        !accepted.tools.is_empty(),
        "reserve cannot erase the accepted tools"
    );
    assert!(held.attempts.last().unwrap().events.iter().any(|event| matches!(&event.kind, engine::EventKind::ProviderStarted { request, .. } if request.added.iter().any(|item| matches!(item, engine::Item::Message { text, .. } if text == engine::FINAL_REQUEST)))));
    let resumed = store
        .resume_authorized(
            &scope,
            "try-again",
            vec![7; 32],
            &held.work,
            held.run().unwrap().id,
            None,
        )
        .await
        .unwrap();
    assert!(matches!(
        resumed,
        local_product::runtime::Resumed::Pending { .. }
    ));
    let second = store.claim(&scope, 60).await.unwrap().unwrap();
    PhysicianHost {
        store: store.clone(),
        claim: second,
        authority: Allowed,
    }
    .run(
        &mut RearmedRecipe(accepted),
        engine::RunLimits {
            max_calls: 3,
            ..limits
        },
        &engine::Cancellation::default(),
    )
    .await
    .unwrap();
    let saved = store.conversation(&scope).await.unwrap();
    assert_eq!(saved.status, WorkStatus::Completed);
    assert_eq!(saved.budget_resume_after, 1);
    assert_eq!(saved.messages.len(), 4);
    assert!(!saved.messages[3].partial());
    assert_eq!(
        saved.messages[3].text,
        "Complete after a failed first final request."
    );
}

struct RearmedAcrossClaims {
    next: usize,
    input: String,
}
impl engine::recipe::Provider for RearmedAcrossClaims {
    async fn exchange(
        &mut self,
        request: &Request,
        _: &engine::Cancellation,
        _: &engine::RunLimits,
        _: &mut dyn Observer,
    ) -> Result<engine::Turn, engine::Cause> {
        let step = self.next;
        self.next += 1;
        let items = match step {
            0..=2 => vec![read(
                &format!("initial-{step}"),
                engine::history::READ,
                r#"{"id":"command-a","limit_chars":65536}"#,
            )],
            3 => {
                assert!(!request.tools.is_empty());
                vec![engine::Item::assistant(
                    "Retained partial before explicit retry.",
                )]
            }
            4 => {
                assert!(
                    request
                        .tools
                        .iter()
                        .any(|tool| tool["name"] == "save_working_note")
                );
                vec![read(
                    "saved-after-retry",
                    "save_working_note",
                    r#"{"text":"Private exact working note body.","fresh_window":true}"#,
                )]
            }
            5 | 6 => {
                assert!(request.items.iter().any(|item| matches!(item, engine::Item::Message { role, text } if role == "user" && text == &self.input)));
                assert!(request.items.iter().any(|item| matches!(item, engine::Item::Message { text, .. } if text.contains(":saved-after-retry"))));
                assert!(
                    request
                        .items
                        .contains(&engine::Item::assistant("Private exact working note body."))
                );
                assert!(
                    !request.tools.is_empty(),
                    "fresh allowance must survive a second claim"
                );
                assert_eq!(
                    request.instructions,
                    test_chat_recipe("frozen original policy").instructions
                );
                if step == 5 {
                    return Err("synthetic worker interruption in the fresh window".into());
                }
                vec![engine::Item::assistant(
                    "Complete answer after fresh window and second claim.",
                )]
            }
            _ => panic!("unexpected provider dispatch"),
        };
        Ok(engine::Turn {
            items,
            usage: Default::default(),
            receipt: json!({"controlled":"rearm-across-claims"}),
        })
    }
}

#[tokio::test]
#[ignore = "requires explicit local Postgres"]
async fn explicit_retry_keeps_fresh_allowance_and_original_input_across_two_claims() {
    let store = store().await;
    let scope = scope();
    accept_with(
        &store,
        &scope,
        &input(),
        &test_chat_recipe("frozen original policy"),
    )
    .await
    .unwrap();
    let limits = engine::RunLimits {
        max_calls: 4,
        max_response_bytes: 100_000,
        ..engine::RunLimits::default()
    };
    let mut provider = RearmedAcrossClaims {
        next: 0,
        input: input().text,
    };
    let first = store.claim(&scope, 60).await.unwrap().unwrap();
    PhysicianHost {
        store: store.clone(),
        claim: first,
        authority: Allowed,
    }
    .run(&mut provider, limits, &engine::Cancellation::default())
    .await
    .unwrap();
    let partial = store.conversation(&scope).await.unwrap();
    assert_eq!(partial.status, WorkStatus::Held);
    assert_eq!(partial.failures, 0);
    assert!(store.claim(&scope, 60).await.unwrap().is_none());
    let retained = serde_json::to_value(&partial.attempts).unwrap();
    let resumed = store
        .resume_authorized(
            &scope,
            "try-again",
            vec![7; 32],
            &partial.work,
            partial.run().unwrap().id,
            None,
        )
        .await
        .unwrap();
    assert!(matches!(
        resumed,
        local_product::runtime::Resumed::Pending { .. }
    ));
    let second = store.claim(&scope, 60).await.unwrap().unwrap();
    assert_eq!(
        store
            .conversation(&scope)
            .await
            .unwrap()
            .budget_resume_after,
        4
    );
    assert!(
        PhysicianHost {
            store: store.clone(),
            claim: second.clone(),
            authority: Allowed,
        }
        .run(&mut provider, limits, &engine::Cancellation::default())
        .await
        .is_err()
    );
    store
        .settle_chat_attempt(
            &second,
            harness::engine::AttemptFailure::Failed,
            Backoff::IMMEDIATE,
        )
        .await
        .unwrap();
    let third = store.claim(&scope, 60).await.unwrap().unwrap();
    PhysicianHost {
        store: store.clone(),
        claim: third,
        authority: Allowed,
    }
    .run(&mut provider, limits, &engine::Cancellation::default())
    .await
    .unwrap();
    let saved = store.conversation(&scope).await.unwrap();
    assert_eq!(saved.status, WorkStatus::Completed);
    assert_eq!(
        serde_json::to_value(&saved.attempts[..partial.attempts.len()]).unwrap(),
        retained
    );
    for index in 0..3 {
        assert_eq!(saved.attempts.iter().flat_map(|a| &a.events).filter(|e| matches!(&e.kind, engine::EventKind::ToolReturned { call_id, .. } if call_id == &format!("initial-{index}"))).count(), 1);
    }
    assert_eq!(saved.budget_resume_after, 4);
    assert_eq!(
        saved.messages[1].text,
        "Retained partial before explicit retry."
    );
    assert!(saved.messages[1].partial());
    assert_eq!(
        saved.messages.last().unwrap().text,
        "Complete answer after fresh window and second claim."
    );
    assert!(!saved.messages.last().unwrap().partial());
}

async fn fail_earlier_attempts(store: &Store, scope: &Scope) {
    for _ in 1..harness::engine::MAX_FAILED_ATTEMPTS {
        let claim = store.claim(scope, 60).await.unwrap().unwrap();
        store
            .settle_chat_attempt(
                &claim,
                harness::engine::AttemptFailure::Failed,
                Backoff::IMMEDIATE,
            )
            .await
            .unwrap();
    }
}

fn skill_limits() -> engine::RunLimits {
    engine::RunLimits {
        max_response_bytes: 131_072,
        ..engine::RunLimits::default()
    }
}

#[tokio::test]
#[ignore = "requires explicit local Postgres"]
async fn accepted_input_ids_survive_interruption_and_identical_message_text() {
    struct Answer;
    impl engine::recipe::Provider for Answer {
        async fn exchange(
            &mut self,
            request: &Request,
            _: &engine::Cancellation,
            _: &engine::RunLimits,
            _: &mut dyn Observer,
        ) -> Result<engine::Turn, engine::Cause> {
            assert_eq!(request.items.iter().filter(|item| matches!(item, engine::Item::Message { role, text } if role == "user" && text == &input().text)).count(), 3);
            assert!(!request.items.iter().any(|item| matches!(item, engine::Item::Message { role, text } if role == "assistant" && text == "Unfinished provisional text")));
            Ok(turn("Answer to the current physician input."))
        }
    }
    let initial = store().await;
    let scope = scope();
    let mut command = input();
    accept_with(
        &initial,
        &scope,
        &command,
        &test_chat_recipe("original guidance"),
    )
    .await
    .unwrap();
    let first = initial.claim(&scope, 60).await.unwrap().unwrap();
    let mut host = PhysicianHost {
        store: initial.clone(),
        claim: first,
        authority: Allowed,
    };
    assert!(
        host.run(
            &mut InterruptedRead("unstarted"),
            skill_limits(),
            &engine::Cancellation::default()
        )
        .await
        .is_err()
    );
    initial
        .settle_chat_attempt(
            &host.claim,
            engine::AttemptFailure::Interrupted,
            Backoff::IMMEDIATE,
        )
        .await
        .unwrap();
    drop(host);
    drop(initial);

    let reopened = store().await;
    for key in ["second", "third"] {
        command.key = key.into();
        accept_with(
            &reopened,
            &scope,
            &command,
            &test_chat_recipe("original guidance"),
        )
        .await
        .unwrap();
    }
    let successor = reopened.claim(&scope, 60).await.unwrap().unwrap();
    let mut host = PhysicianHost {
        store: reopened.clone(),
        claim: successor,
        authority: Allowed,
    };
    host.run(
        &mut Answer,
        skill_limits(),
        &engine::Cancellation::default(),
    )
    .await
    .unwrap();
    let saved = reopened.conversation(&scope).await.unwrap();
    assert_eq!(
        saved.messages.last().unwrap().text,
        "Answer to the current physician input."
    );
    let carried: Vec<_> = saved
        .attempts
        .iter()
        .flat_map(|attempt| &attempt.events)
        .filter_map(|event| match &event.kind {
            engine::EventKind::ProviderStarted { context, .. } => {
                Some(context.carried_inputs.clone())
            }
            _ => None,
        })
        .collect();
    assert_eq!(carried, vec![vec!["command-a"], vec!["second", "third"]]);
    for attempt in &saved.attempts {
        for event in &attempt.events {
            if let engine::EventKind::ProviderStarted { request, .. } = &event.kind {
                assert!(
                    !serde_json::to_string(request)
                        .unwrap()
                        .contains("Exact input"),
                    "dispatch records must refer to authored messages instead of copying their text"
                );
            }
        }
    }
}

struct InterruptedRead(&'static str);
impl engine::recipe::Provider for InterruptedRead {
    async fn exchange(
        &mut self,
        _: &Request,
        cancel: &engine::Cancellation,
        _: &engine::RunLimits,
        observations: &mut dyn Observer,
    ) -> Result<engine::Turn, engine::Cause> {
        observations
            .emit(engine::recipe::StreamEvent::Text(engine::TextDelta {
                text: "Unfinished provisional text".into(),
                location: Some(engine::DeltaLocation {
                    response_id: "interrupted-response".into(),
                    item_id: "interrupted-item".into(),
                    sequence: 1,
                    output_index: 0,
                    content_index: 0,
                }),
            }))
            .await?;
        cancel.cancel();
        Ok(engine::Turn {
            usage: engine::recipe::Usage {
                cached_input_tokens: None,
                input_tokens: Some(12),
                output_tokens: Some(4),
            },
            items: vec![
                engine::Item::assistant("Unfinished provisional text"),
                read(
                    self.0,
                    "skill_view",
                    r#"{"name":"patient-record-retrieval"}"#,
                ),
            ],
            receipt: json!({"id":"interrupted-response"}),
        })
    }
}

#[tokio::test]
#[ignore = "requires explicit local Postgres"]
async fn reclaimed_hosted_chat_cannot_reset_the_request_limit() {
    let s = store().await;
    let scope = scope();
    accept_with(
        &s,
        &scope,
        &input(),
        &test_chat_recipe("frozen hosted cumulative requests"),
    )
    .await
    .unwrap();
    drop(s);
    let mut retained_attempts = Vec::new();
    for call_id in ["first-read", "second-read"] {
        let s = store().await;
        let claim = s.claim(&scope, 60).await.unwrap().unwrap();
        let mut host = PhysicianHost {
            store: s.clone(),
            claim,
            authority: Allowed,
        };
        let error = hosted_attempt(
            &mut host,
            &mut InterruptedRead(call_id),
            engine::RunLimits {
                max_calls: 2,
                ..skill_limits()
            },
            &engine::Cancellation::default(),
        )
        .await
        .unwrap_err();
        assert!(matches!(error, engine::Failure::Cancelled), "{error}");
        let state = s.conversation(&scope).await.unwrap();
        let attempt = state.attempts.last().unwrap();
        assert_eq!(attempt.status, AttemptStatus::Interrupted);
        assert_eq!(state.messages.len(), 1);
        retained_attempts.push(serde_json::to_value(attempt).unwrap());
    }
    let s = store().await;
    let claim = s.claim(&scope, 60).await.unwrap().unwrap();
    let mut host = PhysicianHost {
        store: s.clone(),
        claim,
        authority: Allowed,
    };
    hosted_attempt(
        &mut host,
        &mut MustNotDispatch,
        engine::RunLimits {
            max_calls: 2,
            ..skill_limits()
        },
        &engine::Cancellation::default(),
    )
    .await
    .unwrap();
    drop(host);
    drop(s);
    let saved = store().await.conversation(&scope).await.unwrap();
    assert_eq!(saved.status, WorkStatus::Held);
    assert_eq!(saved.messages.len(), 2);
    assert!(saved.messages[1].partial());
    assert_eq!(saved.attempts.len(), 3);
    for (index, retained) in retained_attempts.iter().enumerate() {
        assert_eq!(
            serde_json::to_value(&saved.attempts[index]).unwrap(),
            *retained
        );
        assert_eq!(
            saved.attempts[index + 1].predecessor.as_deref(),
            Some(saved.attempts[index].id.as_str())
        );
    }
    assert_eq!(saved.messages[1].text, "Unfinished provisional text");
    assert!(
        !saved.attempts[2].events.iter().any(|event| matches!(
            event.kind,
            engine::EventKind::ProviderStarted { .. } | engine::EventKind::ToolStarted(_)
        )),
        "an exhausted successor cannot dispatch or execute the reserved return’s tool call"
    );
}

fn history_page(outcome: engine::ToolOutcome) -> serde_json::Value {
    let engine::ToolOutcome::History(page) = outcome else {
        panic!("history must return a typed page: {outcome:?}");
    };
    serde_json::to_value(page).unwrap()
}

#[tokio::test]
#[ignore = "requires explicit local Postgres"]
async fn history_cursor_pages_preserve_exact_authored_messages() {
    use engine::Product;
    let store = store().await;
    let scope = scope();
    accept_history_assignment(&store, &scope, &test_chat_recipe("History protocol proof")).await;
    let expected = store.conversation(&scope).await.unwrap().messages;
    assert!(
        expected[9].partial(),
        "the held answer is the product's partial"
    );
    let claim = store.claim(&scope, 60).await.unwrap().unwrap();
    let mut host = PhysicianHost {
        store,
        claim,
        authority: Allowed,
    };
    for (name, arguments, position) in [
        (
            engine::history::LIST,
            json!({"order":"oldest_first","limit":1}),
            0,
        ),
        (
            engine::history::READ,
            json!({"id":expected[1].source().id,"limit_chars":40}),
            1,
        ),
        (
            engine::history::SEARCH,
            json!({"text":"Earlier answer 2","max_chars":40}),
            5,
        ),
        (
            engine::history::SEARCH,
            json!({"text":"Held partial answer","max_chars":40}),
            9,
        ),
    ] {
        let call = engine::Invocation {
            id: name.into(),
            name: name.into(),
            arguments: arguments.to_string(),
        };
        let page = history_page(host.execute(&call, arguments).await.unwrap());
        let entry = &page["items"][0];
        assert_eq!(entry["id"], expected[position].source().id);
        assert_eq!(entry["role"], expected[position].role.as_str());
        assert_eq!(entry["metadata"]["partial"], expected[position].partial());
        assert!(
            expected[position]
                .text
                .contains(entry["text"].as_str().unwrap())
        );
    }
    let foreign = Scope {
        physician: "another-physician".into(),
        ..scope.clone()
    };
    host.claim.scope = foreign;
    assert!(
        host.execute(
            &engine::Invocation {
                id: "foreign".into(),
                name: engine::history::LIST.into(),
                arguments: "{}".into()
            },
            json!({})
        )
        .await
        .is_err()
    );
    host.claim.scope = scope.clone();
    let retained = "An exact earlier tool result";
    for (index, kind) in [
        engine::EventKind::ToolStarted(engine::Invocation {
            id: "retained-source".into(),
            name: "read_patient_sources".into(),
            arguments: "{}".into(),
        }),
        engine::EventKind::ToolReturned {
            call_id: "retained-source".into(),
            result: engine::results::ToolResult::Original(retained.into()),
        },
        engine::EventKind::WindowOpened {
            opened_at_unix_ms: 0,
            handoff: None,
            reason: engine::WindowReason::ModelAsked,
        },
    ]
    .into_iter()
    .enumerate()
    {
        host.persist(engine::Event {
            attempt_id: host.claim.attempt.clone(),
            sequence: index as u64 + 1,
            ordinal: 1,
            kind,
        })
        .await
        .unwrap();
    }
    let arguments = json!({"tool_name":"read_patient_sources", "window":0});
    let outcome = host
        .execute(
            &engine::Invocation {
                id: "tools".into(),
                name: engine::history::LIST.into(),
                arguments: arguments.to_string(),
            },
            arguments,
        )
        .await
        .unwrap();
    let directory: serde_json::Value = history_page(outcome);
    assert_eq!(directory["items"].as_array().unwrap().len(), 1);
    assert_eq!(directory["items"][0]["call"]["id"], "retained-source");
    assert_eq!(
        host.read_result("retained-source")
            .await
            .unwrap()
            .as_deref(),
        Some(retained)
    );
}

fn test_chat_recipe(instructions: &str) -> harness::engine::recipe::RequestRecipe {
    let binding = local_product::ModelBinding {
        model: "controlled".into(),
        effort: "none".into(),
        adapter: "fixed".into(),
        context_window_tokens: 272_000,
        max_input_tokens: 272_000,
    };
    let skills = local_product::physician_chat::PhysicianSkills::configured(None, false).unwrap();
    local_product::physician_chat::recipe(&binding, instructions, &skills)
}

/// Accepts an input with the test skills and the default chat recipe.
async fn accept(s: &Store, scope: &Scope, input: &Input) -> Result<Accepted, RuntimeError> {
    accept_with(s, scope, input, &test_chat_recipe("Owner")).await
}
/// Accepts an input with the test skills and a recipe the test chose.
async fn accept_with(
    s: &Store,
    scope: &Scope,
    input: &Input,
    recipe: &harness::engine::recipe::RequestRecipe,
) -> Result<Accepted, RuntimeError> {
    s.accept_authorized(scope, input, vec![0; 32], &skills(), recipe, None)
        .await
}

#[path = "../../harness/tests/support/earlier_notes.rs"]
mod earlier_notes;

#[tokio::test]
#[ignore = "requires explicit local Postgres"]
async fn a_starting_fresh_window_can_open_an_earlier_runs_note() {
    let store = store().await;
    let scope = scope();
    let mut provider = earlier_notes::AcrossRuns::default();
    for (key, text, window) in [
        ("earlier", "x".repeat(80), 272_000),
        ("next", "Continue the work".into(), 32_000),
    ] {
        let input = Input {
            key: key.into(),
            text,
            timezone: "UTC".into(),
            scenario: None,
        };
        let mut recipe = test_chat_recipe("Owner");
        recipe.context_window_tokens = window;
        recipe.max_input_tokens = window;
        accept_with(&store, &scope, &input, &recipe).await.unwrap();
        let claim = store.claim(&scope, 60).await.unwrap().unwrap();
        PhysicianHost {
            store: store.clone(),
            claim,
            authority: Allowed,
        }
        .run(
            &mut provider,
            engine::RunLimits::default(),
            &engine::Cancellation::default(),
        )
        .await
        .unwrap();
    }
    let saved = store.conversation(&scope).await.unwrap();
    assert_eq!(provider.calls, 4);
    assert!(
        saved
            .attempts
            .last()
            .unwrap()
            .events
            .iter()
            .any(|event| matches!(event.kind, engine::EventKind::WindowOpened { .. }))
    );
    assert_eq!(
        saved.messages.last().unwrap().text,
        "Continued from the earlier run's exact note."
    );
}
