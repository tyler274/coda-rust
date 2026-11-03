//! Controlled real-Postgres owner integration, never provider or browser proof.
use local_product::runtime::{
    engine::{self, Candidate, Event, EventKind},
    *,
};
use physician_auth::{AuthStore, Membership, PhysicianIdentity, PostgresAuthStore};
use sea_orm::{Database, TransactionTrait};

struct Auth;
#[async_trait]
impl Authority for Auth {
    async fn check(&mut self, _: &Scope) -> Result<(), String> {
        Ok(())
    }
    async fn lock_publication(
        &mut self,
        tx: &PublicationTransaction,
        scope: &Scope,
        hash: Option<&[u8]>,
    ) -> Result<(), String> {
        PostgresAuthStore::lock_runtime_authority(
            tx,
            hash.ok_or("missing session")?,
            &PhysicianIdentity {
                practice_id: scope.practice.clone(),
                physician_id: scope.physician.clone(),
            },
            &scope.workspace,
        )
        .await
        .map_err(|e| e.to_string())
    }
}
async fn setup() -> (Store, PostgresAuthStore, Scope, String, String) {
    let url = std::env::var("PHYSICIAN_RUNTIME_TEST_DATABASE_URL").unwrap();
    let store = Store::connect(&url).await.unwrap();
    store.migrate().await.unwrap();
    let auth = PostgresAuthStore::connect_with_migrations(&url)
        .await
        .unwrap();
    let id = uuid::Uuid::new_v4().to_string();
    let email = format!("{id}@example.test");
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let invite = auth
        .invite(&id, &email, "Doctor", false, now)
        .await
        .unwrap();
    let (identity, token) = auth.redeem_invitation(&email, &invite, now).await.unwrap();
    let scope = Scope {
        practice: identity.practice_id.clone(),
        physician: identity.physician_id.clone(),
        workspace: id.clone(),
        chat: id,
    };
    auth.register_patient(&scope.practice, &scope.workspace)
        .await
        .unwrap();
    auth.set_membership(&identity, &scope.workspace, Membership::Allowed)
        .await
        .unwrap();
    store
        .accept_authorized(
            &scope,
            &Input {
                key: "input".into(),
                text: "accepted".into(),
                timezone: "UTC".into(),
            },
            PostgresAuthStore::session_reference(&token).to_vec(),
        )
        .await
        .unwrap();
    (store, auth, scope, token, email)
}
async fn candidate(store: &Store, scope: &Scope) -> (Claim, Candidate) {
    let claim = store.claim(scope, 60).await.unwrap().unwrap();
    let candidate = Candidate {
        text: "controlled completion".into(),
        transcript: vec![],
        usage: engine::Usage::default(),
    };
    // The terminal turn first, so only the authority fence can refuse it.
    let kinds = [
        EventKind::ProviderReturned(engine::Turn {
            items: vec![engine::Item::Message {
                role: "assistant".into(),
                text: candidate.text.clone(),
            }],
            receipt: serde_json::json!({"controlled": true}),
        }),
        EventKind::Candidate {
            text: candidate.text.clone(),
        },
    ];
    for (sequence, kind) in (1..).zip(kinds) {
        store
            .persist(
                &claim,
                Event {
                    attempt_id: claim.attempt.clone(),
                    sequence,
                    kind,
                },
            )
            .await
            .unwrap();
    }
    (claim, candidate)
}

#[tokio::test]
#[ignore = "requires local Postgres"]
async fn auth_revocation_and_publication_use_conflicting_real_row_locks() {
    for action in ["session", "panel", "account"] {
        let (store, auth, scope, token, email) = setup().await;
        let (claim, candidate) = candidate(&store, &scope).await;
        let db = Database::connect(std::env::var("PHYSICIAN_RUNTIME_TEST_DATABASE_URL").unwrap())
            .await
            .unwrap();
        let tx = db.begin().await.unwrap();
        let identity = PhysicianIdentity {
            practice_id: scope.practice.clone(),
            physician_id: scope.physician.clone(),
        };
        PostgresAuthStore::lock_runtime_authority(
            &tx,
            &PostgresAuthStore::session_reference(&token),
            &identity,
            &scope.workspace,
        )
        .await
        .unwrap();
        let workspace = scope.workspace.clone();
        let revoker = tokio::spawn(async move {
            match action {
                "session" => auth.end_session(&token).await,
                "panel" => {
                    auth.set_membership(&identity, &workspace, Membership::Denied)
                        .await
                        .unwrap();
                }
                _ => {
                    assert!(auth.deactivate(&email).await);
                }
            }
        });
        tokio::time::sleep(std::time::Duration::from_millis(75)).await;
        assert!(
            !revoker.is_finished(),
            "revocation must wait for publication authority locks"
        );
        tx.commit().await.unwrap();
        revoker.await.unwrap();
        assert!(
            store
                .commit_authorized(&claim, &candidate, &mut Auth)
                .await
                .is_err()
        );
        assert!(
            store.commit(&claim, &candidate).await.is_err(),
            "no unfenced hosted bypass"
        );
        assert_eq!(store.conversation(&scope).await.unwrap().messages.len(), 1);
    }
}

struct Execute {
    completed: usize,
}
#[async_trait]
impl recovery::Executor for Execute {
    async fn authorize(&mut self, scope: &Scope, _: &[u8]) -> Result<(), String> {
        let s = Store::connect(&std::env::var("PHYSICIAN_RUNTIME_TEST_DATABASE_URL").unwrap())
            .await
            .unwrap();
        s.authorize(scope, &mut Auth)
            .await
            .map_err(|e| e.to_string())
    }
    async fn execute(&mut self, store: Store, claim: Claim, _: Vec<u8>) -> Result<(), String> {
        let mut host = PhysicianHost {
            store,
            claim,
            authority: Auth,
            instructions: "controlled version 1".into(),
            skills: None,
        };
        let scope = host.claim.scope.clone();
        host.run(
            &mut Stream(scope),
            &limits(),
            &engine::Cancellation::default(),
        )
        .await?;
        self.completed += 1;
        Ok(())
    }
}
struct Stream(Scope);
#[async_trait]
impl engine::Provider for Stream {
    fn request_bytes(&self, _: &engine::Request<'_>) -> Result<usize, String> {
        Ok(100)
    }
    async fn complete(&mut self, _: engine::Request<'_>) -> Result<engine::Turn, String> {
        panic!("stream expected")
    }
    async fn complete_stream(
        &mut self,
        request: engine::Request<'_>,
        sink: &mut dyn engine::DeltaSink,
    ) -> Result<engine::Turn, engine::Failure> {
        sink.deliver(engine::TextDelta {
            response_id: "r".into(),
            item_id: "i".into(),
            sequence: 1,
            output_index: 0,
            content_index: 0,
            text: "controlled completion".into(),
        })
        .await?;
        // Sink returned only after persistence, but no authored reply is published.
        let s = Store::connect(&std::env::var("PHYSICIAN_RUNTIME_TEST_DATABASE_URL").unwrap())
            .await
            .unwrap();
        for scope in [self.0.clone()] {
            let state = s.conversation(&scope).await.unwrap();
            if state
                .attempts
                .last()
                .is_some_and(|a| a.id == request.attempt_id)
            {
                assert_eq!(state.messages.len(), 1);
                assert!(matches!(
                    state.attempts.last().unwrap().events.last().unwrap().kind,
                    EventKind::TextDelta { .. }
                ));
            }
        }
        Ok(engine::Turn {
            items: vec![engine::Item::Message {
                role: "assistant".into(),
                text: "controlled completion".into(),
            }],
            receipt: serde_json::json!({"controlled": true}),
        })
    }
}
fn limits() -> engine::Limits {
    engine::Limits {
        model_calls: 1,
        tool_calls: 0,
        calls_per_turn: 1,
        request_bytes: 10000,
        response_bytes: 10000,
        tool_result_bytes: 1000,
        total_tool_result_bytes: 1000,
        output_tokens: 200,
        elapsed: std::time::Duration::from_secs(5),
    }
}

#[tokio::test]
#[ignore = "requires local Postgres"]
async fn restart_dispatches_original_session_without_new_input_and_holds_stop() {
    let (s, _, scope, _, _) = setup().await;
    let first = s.claim(&scope, 1).await.unwrap().unwrap();
    drop(s);
    tokio::time::sleep(std::time::Duration::from_secs(2)).await;
    let s = Store::connect(&std::env::var("PHYSICIAN_RUNTIME_TEST_DATABASE_URL").unwrap())
        .await
        .unwrap();
    let mut execute = Execute { completed: 0 };
    let mut other = Execute { completed: 0 };
    let (a, b) = tokio::join!(
        s.dispatch_pending(&mut execute, 100, 60),
        s.dispatch_pending(&mut other, 100, 60)
    );
    a.unwrap();
    b.unwrap();
    assert!(execute.completed + other.completed >= 1);
    let state = s.conversation(&scope).await.unwrap();
    assert_eq!(state.status, "completed");
    assert_eq!(
        state.attempts.last().unwrap().predecessor.as_deref(),
        Some(first.attempt.as_str())
    );
    assert_eq!(state.messages.len(), 2);
    let count = execute.completed;
    s.dispatch_pending(&mut execute, 100, 60).await.unwrap();
    assert_eq!(execute.completed, count);
    let (held, _, scope, _, _) = setup().await;
    held.stop(&scope).await.unwrap();
    held.dispatch_pending(&mut execute, 100, 60).await.unwrap();
    assert_eq!(held.conversation(&scope).await.unwrap().status, "held");
}
