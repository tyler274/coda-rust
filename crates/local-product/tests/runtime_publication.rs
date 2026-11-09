//! Deterministic publication-versus-revocation races use the auth owner's locks.
use local_product::runtime::*;
use physician_auth::{AuthStore, Membership, PhysicianIdentity, PostgresAuthStore};
use tokio::sync::oneshot;

struct Paused {
    locked: Option<oneshot::Sender<()>>,
    proceed: Option<oneshot::Receiver<()>>,
}
#[async_trait]
impl Authority for Paused {
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
            hash.unwrap(),
            &PhysicianIdentity {
                practice_id: scope.practice.clone(),
                physician_id: scope.physician.clone(),
            },
            &scope.workspace,
        )
        .await
        .map_err(|e| e.to_string())?;
        self.locked.take().unwrap().send(()).unwrap();
        self.proceed.take().unwrap().await.unwrap();
        Ok(())
    }
}

/// The recorded call-free terminal turn and its candidate, as the engine
/// acknowledges them before publication.
fn terminal(text: &str) -> Vec<engine::EventKind> {
    vec![
        engine::EventKind::ProviderReturned(engine::Turn {
            items: vec![engine::Item::Message {
                role: "assistant".into(),
                text: text.into(),
            }],
            receipt: serde_json::json!({"controlled": true}),
        }),
        engine::EventKind::Candidate { text: text.into() },
    ]
}

/// Holds the real auth owner's locks on the publication transaction.
struct Locks;
#[async_trait]
impl Authority for Locks {
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

/// A signed-in physician with one accepted hosted chat turn.
async fn accepted(store: &Store, auth: &PostgresAuthStore) -> Scope {
    let id = uuid::Uuid::new_v4().to_string();
    let email = format!("{id}@example.test");
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let invitation = auth
        .invite(&id, &email, "Doctor", false, now)
        .await
        .unwrap();
    let (identity, token) = auth
        .redeem_invitation(&email, &invitation, now)
        .await
        .unwrap();
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
                key: "k".into(),
                text: "input".into(),
                timezone: "UTC".into(),
                scenario: None,
            },
            PostgresAuthStore::session_reference(&token),
            None,
        )
        .await
        .unwrap();
    scope
}

/// One attempt of two provider rounds: narration with a skill read, then the
/// given terminal items. Returns the claim and the engine's aggregate candidate.
async fn two_rounds(
    store: &Store,
    scope: &Scope,
    terminal: Vec<engine::Item>,
) -> (Claim, engine::Candidate) {
    let claim = store.claim(scope, 60).await.unwrap().unwrap();
    let call = engine::Invocation {
        id: "read".into(),
        name: "skill_view".into(),
        arguments: r#"{"name":"pre-visit-preparation","file_path":null,"max_result_bytes":65536}"#
            .into(),
    };
    let first = vec![
        engine::Item::Message {
            role: "assistant".into(),
            text: "Let me read the pre-visit skill first. ".into(),
        },
        engine::Item::Call(call.clone()),
    ];
    let text: String = first
        .iter()
        .chain(&terminal)
        .filter_map(|item| match item {
            engine::Item::Message { text, .. } => Some(text.as_str()),
            _ => None,
        })
        .collect();
    let kinds = [
        engine::EventKind::ProviderReturned(engine::Turn {
            items: first,
            receipt: serde_json::json!({"round": 1}),
        }),
        engine::EventKind::ToolStarted(call),
        engine::EventKind::ToolReturned {
            call_id: "read".into(),
            output: r#"{"status":"available"}"#.into(),
            executed: true,
        },
        engine::EventKind::ProviderReturned(engine::Turn {
            items: terminal,
            receipt: serde_json::json!({"round": 2}),
        }),
        engine::EventKind::Candidate { text: text.clone() },
    ];
    for (sequence, kind) in (1..).zip(kinds) {
        store
            .persist(
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
    let candidate = engine::Candidate {
        text,
        transcript: vec![],
        usage: engine::Usage::default(),
    };
    (claim, candidate)
}

/// Pre-tool narration stays in the attempt's events. The physician reads only
/// the terminal turn, every text part in order.
#[tokio::test]
#[ignore = "requires local Postgres"]
async fn only_the_terminal_turn_is_published_with_its_parts_in_order() {
    let url = std::env::var("PHYSICIAN_RUNTIME_TEST_DATABASE_URL").unwrap();
    let store = Store::connect(&url).await.unwrap();
    store.migrate().await.unwrap();
    let auth = PostgresAuthStore::connect_with_migrations(&url)
        .await
        .unwrap();
    let scope = accepted(&store, &auth).await;
    let (claim, candidate) = two_rounds(
        &store,
        &scope,
        vec![
            engine::Item::Message {
                role: "assistant".into(),
                text: "First part. ".into(),
            },
            engine::Item::Opaque(serde_json::json!({"type": "reasoning"})),
            engine::Item::Message {
                role: "assistant".into(),
                text: "Second part.".into(),
            },
        ],
    )
    .await;
    assert!(candidate.text.starts_with("Let me read"));
    store
        .commit_authorized(&claim, &candidate, &mut Locks)
        .await
        .unwrap();
    let state = store.conversation(&scope).await.unwrap();
    assert_eq!(state.status, "completed");
    assert_eq!(state.messages.len(), 2);
    assert_eq!(state.messages[1].text, "First part. Second part.");
    // The full exchange is retained, narration included.
    let events = &state.attempts.last().unwrap().events;
    assert_eq!(events.len(), 5);
    assert!(matches!(
        &events[0].kind,
        engine::EventKind::ProviderReturned(turn)
            if matches!(&turn.items[0], engine::Item::Message { text, .. } if text.starts_with("Let me read"))
    ));
}

/// A blank terminal turn never publishes earlier narration or history; the
/// attempt stays open for its owner to fail or retry.
#[tokio::test]
#[ignore = "requires local Postgres"]
async fn a_blank_terminal_turn_publishes_nothing() {
    let url = std::env::var("PHYSICIAN_RUNTIME_TEST_DATABASE_URL").unwrap();
    let store = Store::connect(&url).await.unwrap();
    store.migrate().await.unwrap();
    let auth = PostgresAuthStore::connect_with_migrations(&url)
        .await
        .unwrap();
    for terminal in [
        vec![],
        vec![engine::Item::Message {
            role: "assistant".into(),
            text: " \n".into(),
        }],
        vec![engine::Item::Opaque(
            serde_json::json!({"type": "reasoning"}),
        )],
    ] {
        let scope = accepted(&store, &auth).await;
        let (claim, candidate) = two_rounds(&store, &scope, terminal).await;
        assert!(!candidate.text.trim().is_empty());
        assert!(
            store
                .commit_authorized(&claim, &candidate, &mut Locks)
                .await
                .is_err()
        );
        let state = store.conversation(&scope).await.unwrap();
        assert_eq!(state.messages.len(), 1, "earlier narration was published");
        assert_ne!(state.status, "completed");
        assert_eq!(state.attempts.last().unwrap().status, "active");
    }
}

#[tokio::test]
#[ignore = "requires local Postgres"]
async fn publication_wins_revocation_or_expires_while_holding_authority() {
    let url = std::env::var("PHYSICIAN_RUNTIME_TEST_DATABASE_URL").unwrap();
    let store = Store::connect(&url).await.unwrap();
    store.migrate().await.unwrap();
    let auth = PostgresAuthStore::connect_with_migrations(&url)
        .await
        .unwrap();
    for expire in [false, true] {
        let id = uuid::Uuid::new_v4().to_string();
        let email = format!("{id}@example.test");
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let invitation = auth
            .invite(&id, &email, "Doctor", false, now)
            .await
            .unwrap();
        let (identity, token) = auth
            .redeem_invitation(&email, &invitation, now)
            .await
            .unwrap();
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
                    key: "k".into(),
                    text: "input".into(),
                    timezone: "UTC".into(),
                    scenario: None,
                },
                PostgresAuthStore::session_reference(&token),
                None,
            )
            .await
            .unwrap();
        let claim = store.claim(&scope, 60).await.unwrap().unwrap();
        let candidate = engine::Candidate {
            text: "controlled".into(),
            transcript: vec![],
            usage: engine::Usage::default(),
        };
        for (sequence, kind) in (1..).zip(terminal(&candidate.text)) {
            store
                .persist(
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
        if expire {
            // Start the expiry window after saving the candidate. A one-second
            // lease can end at the very next tick of the database clock.
            store.heartbeat(&claim, 2).await.unwrap();
        }
        let (locked_tx, locked_rx) = oneshot::channel();
        let (proceed_tx, proceed_rx) = oneshot::channel();
        let s = store.clone();
        let publication = tokio::spawn(async move {
            s.commit_authorized(
                &claim,
                &candidate,
                &mut Paused {
                    locked: Some(locked_tx),
                    proceed: Some(proceed_rx),
                },
            )
            .await
        });
        locked_rx.await.unwrap();
        let a = auth.clone();
        let revocation = tokio::spawn(async move { a.end_session(&token).await.unwrap() });
        tokio::time::sleep(std::time::Duration::from_millis(if expire {
            2100
        } else {
            75
        }))
        .await;
        assert!(!revocation.is_finished());
        proceed_tx.send(()).unwrap();
        let result = publication.await.unwrap();
        if expire {
            assert_eq!(
                result,
                Err(sea_orm::DbErr::Custom("stale completion".into()))
            );
        } else {
            result.unwrap();
        }
        revocation.await.unwrap();
        assert_eq!(
            store.conversation(&scope).await.unwrap().messages.len(),
            if expire { 1 } else { 2 }
        );
    }
}
