//! Real Postgres proof. Run explicitly with PHYSICIAN_RUNTIME_TEST_DATABASE_URL.
use local_product::runtime::{
    Accepted, Authority, Input, PhysicianHost, Scope, Store, async_trait, engine,
};
use serde_json::json;

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
#[tokio::test]
#[ignore = "requires dedicated Postgres cluster and PHYSICIAN_RUNTIME_TEST_PGDATA"]
async fn database_crash_restart_retains_acknowledged_command_and_event() {
    let pgdata = std::env::var("PHYSICIAN_RUNTIME_TEST_PGDATA")
        .expect("dedicated disposable cluster required");
    assert!(
        pgdata.starts_with("/tmp/eng80-"),
        "refuse to restart an unrelated database"
    );
    let s = store().await;
    let scope = scope();
    let command = input();
    s.accept(&scope, &command).await.unwrap();
    let claim = s.claim(&scope, 60).await.unwrap().unwrap();
    let kinds = [
        engine::EventKind::ProviderReturned(engine::Turn {
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
                "/tmp/eng80-physician-postgres.log",
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
        s.accept(&scope, &command).await.unwrap(),
        Accepted::Duplicate
    );
    let state = s.conversation(&scope).await.unwrap();
    assert_eq!(state.messages.len(), 1);
    assert_eq!(state.attempts[0].events.len(), 2);
    s.commit(
        &claim,
        &engine::Candidate {
            text: "durable candidate".into(),
            transcript: vec![],
            usage: engine::Usage::default(),
        },
    )
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
    async fn check(&mut self, _: &Scope) -> Result<(), String> {
        Ok(())
    }
}
struct Provider;
#[async_trait]
impl engine::Provider for Provider {
    fn request_bytes(&self, _: &engine::Request<'_>) -> Result<usize, String> {
        Ok(50)
    }
    async fn complete(&mut self, request: engine::Request<'_>) -> Result<engine::Turn, String> {
        assert!(request.context.tools.is_empty());
        assert!(request.context.items.iter().any(|i| matches!(i, engine::Item::Message { role, text } if role == "user" && text.contains("Exact input"))));
        if request.ordinal == 1 {
            return Ok(engine::Turn {
                items: vec![engine::Item::Call(engine::Invocation {
                    id: "unavailable-call".into(),
                    name: "write_patient_memory".into(),
                    arguments: "{}".into(),
                })],
                receipt: json!({"controlled": true}),
            });
        }
        assert_eq!(
            request
                .context
                .items
                .iter()
                .filter(|item| matches!(item, engine::Item::Message { role, .. } if role == "user"))
                .count(),
            1
        );
        assert!(request.context.items.iter().any(|item| matches!(item, engine::Item::Result { output, .. } if output.contains("unknown or unoffered tool"))));
        Ok(engine::Turn {
            items: vec![engine::Item::Message {
                role: "assistant".into(),
                text: "Controlled response".into(),
            }],
            receipt: json!({"controlled": true}),
        })
    }
}
struct MustNotDispatch;
#[async_trait]
impl engine::Provider for MustNotDispatch {
    fn request_bytes(&self, _: &engine::Request<'_>) -> Result<usize, String> {
        Ok(1)
    }
    async fn complete(&mut self, _: engine::Request<'_>) -> Result<engine::Turn, String> {
        panic!("provider dispatched without durable event acknowledgment")
    }
}

#[tokio::test]
#[ignore = "requires explicit local Postgres"]
async fn refused_event_ack_prevents_provider_dispatch_and_tools_fail_closed() {
    use engine::Host;
    let s = store().await;
    let scope = scope();
    s.accept(&scope, &input()).await.unwrap();
    let claim = s.claim(&scope, 60).await.unwrap().unwrap();
    // A conflicting first event cannot be silently overwritten by a reused attempt.
    s.persist(
        &claim,
        engine::Event {
            attempt_id: claim.attempt.clone(),
            sequence: 1,
            kind: engine::EventKind::Candidate {
                text: "previous evidence".into(),
            },
        },
    )
    .await
    .unwrap();
    let mut host = PhysicianHost {
        store: s.clone(),
        claim,
        authority: Allowed,
        instructions: "versioned test".into(),
        skills: None,
    };
    for name in [
        "write_patient_memory",
        "delegate_work",
        "finish_reply_revision",
    ] {
        let call = engine::Invocation {
            id: name.into(),
            name: name.into(),
            arguments: "{}".into(),
        };
        assert!(matches!(
            host.admit(&call, &json!({})).await.unwrap(),
            engine::Admission::Deny(_)
        ));
        assert!(host.execute(&call, json!({})).await.is_err());
    }
    assert!(
        host.run(
            &mut MustNotDispatch,
            &limits(),
            &engine::Cancellation::default()
        )
        .await
        .is_err()
    );
    let state = s.conversation(&scope).await.unwrap();
    // A provider or event failure leaves the attempt leased. Ending it here
    // would make one crash terminal before retry_delivery or lease expiry.
    assert_eq!(state.status, "active");
    assert_eq!(state.attempts[0].status, "active");
    assert_eq!(state.messages.len(), 1);
    assert_eq!(state.attempts[0].events.len(), 1);
}

fn limits() -> engine::Limits {
    engine::Limits {
        model_calls: 2,
        tool_calls: 0,
        calls_per_turn: 4,
        request_bytes: 10_000,
        response_bytes: 10_000,
        tool_result_bytes: 1000,
        total_tool_result_bytes: 1000,
        output_tokens: 500,
        elapsed: std::time::Duration::from_secs(5),
    }
}

#[tokio::test]
#[ignore = "requires explicit local Postgres"]
async fn durable_acceptance_dedup_tenants_successor_and_engine_commit() {
    let s = store().await;
    let scope = scope();
    let command = input();
    let (a, b) = tokio::join!(s.accept(&scope, &command), s.accept(&scope, &command));
    let outcomes = [a.unwrap(), b.unwrap()];
    assert!(outcomes.contains(&Accepted::New) && outcomes.contains(&Accepted::Duplicate));
    let mut collision = command.clone();
    collision.text = "changed".into();
    assert!(s.accept(&scope, &collision).await.is_err());
    let mut other = scope.clone();
    other.practice = uuid::Uuid::new_v4().to_string();
    assert!(s.conversation(&other).await.is_err());
    assert_eq!(s.accept(&other, &command).await.unwrap(), Accepted::New);
    other = scope.clone();
    other.physician = "different-physician".into();
    assert!(s.conversation(&other).await.is_err());
    assert_eq!(s.accept(&other, &command).await.unwrap(), Accepted::New);
    let first = s.claim(&scope, 60).await.unwrap().unwrap();
    assert!(s.claim(&scope, 60).await.unwrap().is_none());
    s.interrupt(&first, false).await.unwrap();
    drop(s);
    let s = store().await;
    assert_eq!(
        s.accept(&scope, &command).await.unwrap(),
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
        instructions: "test instruction v1".into(),
        skills: None,
    };
    host.run(&mut Provider, &limits(), &engine::Cancellation::default())
        .await
        .unwrap();
    drop(host);
    drop(s);
    let s = store().await;
    let state = s.conversation(&scope).await.unwrap();
    assert_eq!(state.messages.len(), 2);
    assert_eq!(state.status, "completed");
    assert_eq!(state.attempts[1].events.len(), 6);
    assert_eq!(
        s.accept(&scope, &command).await.unwrap(),
        Accepted::Duplicate
    );
    assert!(s.claim(&scope, 60).await.unwrap().is_none());
    s.interrupt(&successor, false).await.unwrap();
    assert_eq!(s.conversation(&scope).await.unwrap().status, "completed");
}

#[tokio::test]
#[ignore = "requires explicit local Postgres"]
async fn event_ack_stop_steering_and_late_candidate_fences() {
    let s = store().await;
    let scope = scope();
    s.accept(&scope, &input()).await.unwrap();
    let claim = s.claim(&scope, 60).await.unwrap().unwrap();
    let event = engine::Event {
        attempt_id: claim.attempt.clone(),
        sequence: 1,
        kind: engine::EventKind::Candidate {
            text: "late".into(),
        },
    };
    let candidate = engine::Candidate {
        text: "late".into(),
        transcript: vec![],
        usage: engine::Usage::default(),
    };
    assert!(s.commit(&claim, &candidate).await.is_err());
    s.persist(&claim, event.clone()).await.unwrap();
    s.persist(&claim, event.clone()).await.unwrap();
    let mut different = event.clone();
    different.kind = engine::EventKind::Candidate {
        text: "collision".into(),
    };
    assert!(s.persist(&claim, different).await.is_err());
    let mut gap = event.clone();
    gap.sequence = 3;
    assert!(s.persist(&claim, gap).await.is_err());
    s.stop(&scope).await.unwrap();
    assert!(s.commit(&claim, &candidate).await.is_err());
    assert!(s.persist(&claim, event).await.is_err());
    s.persist(
        &claim,
        engine::Event {
            attempt_id: claim.attempt.clone(),
            sequence: 2,
            kind: engine::EventKind::ProviderReturned(engine::Turn {
                items: vec![],
                receipt: json!({"late": true}),
            }),
        },
    )
    .await
    .unwrap();
    assert_eq!(s.conversation(&scope).await.unwrap().status, "held");
    drop(s);
    let s = store().await;
    assert!(s.claim(&scope, 60).await.unwrap().is_none());
    let mut steer = input();
    steer.key = "steer".into();
    steer.text = "new instruction".into();
    s.accept(&scope, &steer).await.unwrap();
    assert!(s.claim(&scope, 60).await.unwrap().is_none());
    s.resume(&scope).await.unwrap();
    let resumed = s.claim(&scope, 60).await.unwrap().unwrap();
    steer.key = "steer-again".into();
    s.accept(&scope, &steer).await.unwrap();
    assert!(s.assert_current(&resumed).await.is_err());
    assert!(s.commit(&resumed, &candidate).await.is_err());
    let next = s.claim(&scope, 60).await.unwrap().unwrap();
    assert_ne!(resumed.attempt, next.attempt);
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

fn result<'a>(context: &'a engine::Context, id: &str) -> &'a str {
    context
        .items
        .iter()
        .find_map(|item| match item {
            engine::Item::Result { call_id, output } if call_id == id => Some(output.as_str()),
            _ => None,
        })
        .unwrap_or_else(|| panic!("no result for {id}"))
}

/// Round 1 reads a skill body alongside bad, unknown, unavailable and
/// over-budget calls; round 2 reads a supporting resource the body named;
/// round 3 answers. Every request is checked against the loader's exact bytes.
struct SkillReader {
    skills: local_product::physician_chat::PhysicianSkills,
    instructions: String,
}
#[async_trait]
impl engine::Provider for SkillReader {
    fn request_bytes(&self, _: &engine::Request<'_>) -> Result<usize, String> {
        Ok(100)
    }
    async fn complete(&mut self, request: engine::Request<'_>) -> Result<engine::Turn, String> {
        let context = request.context;
        assert_eq!(context.instructions, self.instructions);
        assert!(context.instructions.contains(self.skills.catalog()));
        assert_eq!(
            context
                .tools
                .iter()
                .map(|t| t.name.as_str())
                .collect::<Vec<_>>(),
            ["skill_view"]
        );
        let view = |id: &str, name: &str, path: Option<&str>, bytes: u32| {
            let harness = harness::Harness::load(
                &std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../skills"),
            )
            .unwrap();
            assert_eq!(
                result(context, id),
                harness.skill_view(name, path, bytes).unwrap().to_json(),
                "{id} must be the loader's exact result"
            );
        };
        let turn = |items| {
            Ok(engine::Turn {
                items,
                receipt: json!({"controlled": request.ordinal}),
            })
        };
        match request.ordinal {
            1 => turn(vec![
                engine::Item::Message {
                    role: "assistant".into(),
                    text: "Reading the pre-visit skill. ".into(),
                },
                read(
                    "body",
                    "skill_view",
                    r#"{"name":"pre-visit-preparation","file_path":null,"max_result_bytes":65536}"#,
                ),
                read(
                    "malformed",
                    "skill_view",
                    r#"{"name":"pre-visit-preparation"}"#,
                ),
                read("not-json", "skill_view", "not json"),
                read(
                    "unknown",
                    "write_patient_memory",
                    r#"{"name":"pre-visit-preparation","file_path":null,"max_result_bytes":4096}"#,
                ),
                read(
                    "missing",
                    "skill_view",
                    r#"{"name":"not-a-skill","file_path":null,"max_result_bytes":4096}"#,
                ),
                read(
                    "over",
                    "skill_view",
                    r#"{"name":"pre-visit-preparation","file_path":null,"max_result_bytes":4096}"#,
                ),
            ]),
            2 => {
                view("body", "pre-visit-preparation", None, 65536);
                view("missing", "not-a-skill", None, 4096);
                view("over", "pre-visit-preparation", None, 4096);
                for refused in ["malformed", "not-json", "unknown"] {
                    let output: serde_json::Value =
                        serde_json::from_str(result(context, refused)).unwrap();
                    assert_eq!(output["executed"], false, "{refused}");
                    assert!(output["error"].as_str().is_some_and(|e| !e.is_empty()));
                }
                assert!(result(context, "unknown").contains("unknown or unoffered tool"));
                assert!(result(context, "malformed").contains("Invalid skill_view arguments"));
                turn(vec![
                    engine::Item::Message {
                        role: "assistant".into(),
                        text: "Checking a supporting guide. ".into(),
                    },
                    read(
                        "resource",
                        "skill_view",
                        r#"{"name":"patient-memory-maintenance","file_path":"references/authoring-guide.md","max_result_bytes":65536}"#,
                    ),
                ])
            }
            3 => {
                view(
                    "resource",
                    "patient-memory-maintenance",
                    Some("references/authoring-guide.md"),
                    65536,
                );
                turn(vec![
                    engine::Item::Message {
                        role: "assistant".into(),
                        text: "Final answer, ".into(),
                    },
                    engine::Item::Message {
                        role: "assistant".into(),
                        text: "in two parts.".into(),
                    },
                ])
            }
            other => panic!("unexpected request {other}"),
        }
    }
}

fn skill_limits() -> engine::Limits {
    engine::Limits {
        model_calls: 8,
        tool_calls: 16,
        calls_per_turn: 8,
        request_bytes: 1_048_576,
        response_bytes: 131_072,
        tool_result_bytes: 65_536,
        total_tool_result_bytes: 262_144,
        output_tokens: 500,
        elapsed: std::time::Duration::from_secs(30),
    }
}

#[tokio::test]
#[ignore = "requires explicit local Postgres"]
async fn skill_reads_reach_the_next_request_and_only_the_answer_is_published() {
    let s = store().await;
    let scope = scope();
    s.accept(&scope, &input()).await.unwrap();
    let claim = s.claim(&scope, 60).await.unwrap().unwrap();
    let skills = skills();
    let instructions = skills.instructions(local_product::physician_chat::INSTRUCTIONS);
    let mut host = PhysicianHost {
        store: s.clone(),
        claim,
        authority: Allowed,
        instructions: instructions.clone(),
        skills: Some(skills.clone()),
    };
    host.run(
        &mut SkillReader {
            skills,
            instructions: instructions.clone(),
        },
        &skill_limits(),
        &engine::Cancellation::default(),
    )
    .await
    .unwrap();
    drop(host);
    drop(s);
    // Read back through a new connection.
    let s = store().await;
    let state = s.conversation(&scope).await.unwrap();
    assert_eq!(state.status, "completed");
    assert_eq!(state.messages.len(), 2);
    assert_eq!(state.messages[1].text, "Final answer, in two parts.");
    let events = &state.attempts[0].events;
    // Every request recorded the exact instructions, catalog and tool.
    let started: Vec<_> = events
        .iter()
        .filter_map(|e| match &e.kind {
            engine::EventKind::ProviderStarted { context, .. } => Some(context),
            _ => None,
        })
        .collect();
    assert_eq!(started.len(), 3);
    assert!(started.iter().all(|c| c.instructions == instructions));
    assert!(instructions.contains(local_product::physician_chat::INSTRUCTIONS_VERSION));
    // Executed reads are recorded with their version identities; refusals
    // are recorded as not executed.
    let returned: std::collections::HashMap<_, _> = events
        .iter()
        .filter_map(|e| match &e.kind {
            engine::EventKind::ToolReturned {
                call_id,
                output,
                executed,
            } => Some((call_id.as_str(), (output.as_str(), *executed))),
            _ => None,
        })
        .collect();
    assert_eq!(returned.len(), 7);
    for id in ["body", "missing", "over", "resource"] {
        assert!(returned[id].1, "{id} executed");
    }
    for id in ["malformed", "not-json", "unknown"] {
        assert!(!returned[id].1, "{id} refused");
    }
    let body: serde_json::Value = serde_json::from_str(returned["body"].0).unwrap();
    assert_eq!(body["status"], "available");
    assert!(body["version_id"].as_str().is_some_and(|v| !v.is_empty()));
    let resource: serde_json::Value = serde_json::from_str(returned["resource"].0).unwrap();
    assert!(
        resource["version_id"]
            .as_str()
            .is_some_and(|v| !v.is_empty())
    );
    // Narration stays in the record and out of the published answer.
    assert!(events.iter().any(|e| matches!(&e.kind,
        engine::EventKind::ProviderReturned(turn)
            if turn.items.iter().any(|i| matches!(i, engine::Item::Message { text, .. } if text.starts_with("Reading the pre-visit")))
    )));
    assert!(!state.messages[1].text.contains("Reading"));
}

/// Keeps requesting reads; never answers.
struct Endless;
#[async_trait]
impl engine::Provider for Endless {
    fn request_bytes(&self, _: &engine::Request<'_>) -> Result<usize, String> {
        Ok(100)
    }
    async fn complete(&mut self, request: engine::Request<'_>) -> Result<engine::Turn, String> {
        Ok(engine::Turn {
            items: vec![
                engine::Item::Message {
                    role: "assistant".into(),
                    text: format!("Still reading {}. ", request.ordinal),
                },
                read(
                    &format!("read-{}", request.ordinal),
                    "skill_view",
                    r#"{"name":"pre-visit-preparation","file_path":null,"max_result_bytes":65536}"#,
                ),
            ],
            receipt: json!({}),
        })
    }
}

#[tokio::test]
#[ignore = "requires explicit local Postgres"]
async fn exhausted_bounds_end_the_attempt_without_publishing_narration() {
    let s = store().await;
    let scope = scope();
    s.accept(&scope, &input()).await.unwrap();
    let claim = s.claim(&scope, 60).await.unwrap().unwrap();
    let skills = skills();
    let mut host = PhysicianHost {
        store: s.clone(),
        claim,
        authority: Allowed,
        instructions: skills.instructions(local_product::physician_chat::INSTRUCTIONS),
        skills: Some(skills),
    };
    let error = host
        .run(
            &mut Endless,
            &engine::Limits {
                model_calls: 3,
                ..skill_limits()
            },
            &engine::Cancellation::default(),
        )
        .await
        .unwrap_err();
    assert!(error.contains("model calls"), "{error}");
    let state = s.conversation(&scope).await.unwrap();
    // Bound exhaustion leaves the attempt leased, like any other engine
    // failure. Ending it here would make one crash terminal before
    // retry_delivery or lease expiry. Narration still must not publish.
    assert_eq!(state.status, "active");
    assert_eq!(state.attempts[0].status, "active");
    assert_eq!(state.messages.len(), 1, "narration must not be published");
    let executed = state.attempts[0]
        .events
        .iter()
        .filter(|e| {
            matches!(
                e.kind,
                engine::EventKind::ToolReturned { executed: true, .. }
            )
        })
        .count();
    assert_eq!(executed, 3);
}
