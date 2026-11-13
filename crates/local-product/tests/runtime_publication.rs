//! Deterministic publication-versus-revocation races use the auth owner's locks.
use local_product::runtime::*;
use physician_auth::{AuthStore, Membership, PhysicianIdentity, PostgresAuthStore};
use tokio::sync::oneshot;
#[path = "support/authority.rs"]
mod authority;
#[path = "support/finish.rs"]
mod finish;
use authority::Locks;

struct Paused {
    locked: Option<oneshot::Sender<()>>,
    proceed: Option<oneshot::Receiver<()>>,
}
#[async_trait]
impl Authority for Paused {
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

/// The recorded finish turn and its candidate, as the engine acknowledges
/// them before publication.
fn terminal(turn: &engine::Turn) -> Vec<engine::EventKind> {
    vec![
        engine::EventKind::ProviderReturned(turn.clone()),
        engine::EventKind::Candidate { text: turn.text() },
    ]
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
            &local_product::physician_chat::PhysicianSkills::configured(None, false).unwrap(),
            &test_chat_recipe(),
            None,
        )
        .await
        .unwrap();
    scope
}

/// One attempt of two provider rounds: narration with a skill read, then the
/// given terminal items. Returns the claim and the engine's terminal turn.
async fn two_rounds(
    store: &Store,
    scope: &Scope,
    terminal: Vec<engine::Item>,
) -> (Claim, engine::Turn) {
    let claim = store.claim(scope, 60).await.unwrap().unwrap();
    let call = engine::Invocation {
        id: "read".into(),
        name: "skill_view".into(),
        arguments: r#"{"name":"patient-record-retrieval"}"#.into(),
    };
    let first = vec![
        engine::Item::Message {
            role: "assistant".into(),
            text: "Let me read the pre-visit skill first. ".into(),
        },
        engine::Item::Call(call.clone()),
    ];
    let candidate = engine::Turn {
        items: terminal.clone(),
        usage: Default::default(),
        receipt: serde_json::json!({"round":2}),
    };
    let text = candidate.text();
    let kinds = [
        engine::EventKind::ProviderReturned(engine::Turn {
            usage: Default::default(),
            items: first,
            receipt: serde_json::json!({"round": 1}),
        }),
        engine::EventKind::ToolStarted(call),
        engine::EventKind::ToolReturned {
            call_id: "read".into(),
            result: engine::results::ToolResult::Original(r#"{"status":"available"}"#.into()),
        },
        engine::EventKind::ProviderReturned(engine::Turn {
            usage: Default::default(),
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
                    ordinal: 1,
                    sequence,
                    kind,
                },
            )
            .await
            .unwrap();
    }
    (claim, candidate)
}

/// Narration stays in the attempt's events. The physician reads only the
/// answer handed in through the finish.
#[tokio::test]
#[ignore = "requires local Postgres"]
async fn only_the_finished_answer_is_published() {
    let url = std::env::var("PHYSICIAN_RUNTIME_TEST_DATABASE_URL").unwrap();
    let store = Store::connect(&url).await.unwrap();
    store.migrate().await.unwrap();
    let auth = PostgresAuthStore::connect_with_migrations(&url)
        .await
        .unwrap();
    let scope = accepted(&store, &auth).await;
    let mut items = vec![
        engine::Item::assistant("Narration before the finish. "),
        engine::Item::Opaque(serde_json::json!({"type": "reasoning"})),
    ];
    items.push(finish::answered("The answer.").remove(1));
    let (claim, candidate) = two_rounds(&store, &scope, items).await;
    assert!(matches!(
        store
            .commit_authorized(&claim, &candidate, &mut Locks)
            .await
            .unwrap(),
        engine::recipe::Finish::Committed
    ));
    let state = store.conversation(&scope).await.unwrap();
    assert_eq!(state.status, WorkStatus::Completed);
    assert_eq!(state.messages.len(), 2);
    assert_eq!(state.messages[1].text, "The answer.");
    // The full exchange is retained, narration included.
    let events = &state.attempts.last().unwrap().events;
    assert_eq!(events.len(), 5);
    assert!(matches!(
        &events[0].kind,
        engine::EventKind::ProviderReturned(turn)
            if matches!(&turn.items[0], engine::Item::Message { text, .. } if text.starts_with("Let me read"))
    ));
}

/// A blank turn, plain text, or a finish beside another call is refused: it
/// publishes no narration or history, and the attempt stays open so the
/// model can finish.
#[tokio::test]
#[ignore = "requires local Postgres"]
async fn a_terminal_turn_without_a_lone_finish_is_refused_and_publishes_nothing() {
    let url = std::env::var("PHYSICIAN_RUNTIME_TEST_DATABASE_URL").unwrap();
    let store = Store::connect(&url).await.unwrap();
    store.migrate().await.unwrap();
    let auth = PostgresAuthStore::connect_with_migrations(&url)
        .await
        .unwrap();
    let mut beside_a_call = finish::answered("Answer");
    beside_a_call.push(engine::Item::Call(engine::Invocation {
        id: "unexecuted-call".into(),
        name: "skill_view".into(),
        arguments: "{}".into(),
    }));
    for terminal in [
        vec![],
        vec![engine::Item::Message {
            role: "assistant".into(),
            text: " \n".into(),
        }],
        vec![engine::Item::Opaque(
            serde_json::json!({"type": "reasoning"}),
        )],
        vec![engine::Item::Message {
            role: "assistant".into(),
            text: "A plain answer.".into(),
        }],
        beside_a_call,
    ] {
        let scope = accepted(&store, &auth).await;
        let (claim, candidate) = two_rounds(&store, &scope, terminal).await;
        assert!(matches!(
            store
                .commit_authorized(&claim, &candidate, &mut Locks)
                .await
                .unwrap(),
            engine::recipe::Finish::Refused(_)
        ));
        let state = store.conversation(&scope).await.unwrap();
        assert_eq!(state.messages.len(), 1, "earlier narration was published");
        assert_ne!(state.status, WorkStatus::Completed);
        assert_eq!(state.attempts.last().unwrap().status, AttemptStatus::Active);
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
                &local_product::physician_chat::PhysicianSkills::configured(None, false).unwrap(),
                &test_chat_recipe(),
                None,
            )
            .await
            .unwrap();
        let claim = store.claim(&scope, 60).await.unwrap().unwrap();
        let candidate = engine::Turn {
            items: finish::answered("controlled"),
            receipt: serde_json::json!({"controlled":true}),
            usage: Default::default(),
        };
        for (sequence, kind) in (1..).zip(terminal(&candidate)) {
            store
                .persist(
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
        if expire {
            // Start the expiry window after saving the candidate. A one-second
            // lease can end at the very next tick of the database clock.
            let mut expiring = claim.clone();
            expiring.lease = local_product::runtime::Lease::new(2).unwrap();
            store.renew_claim(&expiring).await.unwrap();
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
            let cause = result.unwrap_err();
            assert!(
                matches!(
                    cause,
                    engine::ProductError::NotCurrent(engine::NotCurrent::Lost)
                ),
                "lease expiry remains distinct from input supersession"
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

fn test_chat_recipe() -> harness::engine::recipe::RequestRecipe {
    let binding = local_product::ModelBinding {
        model: "controlled".into(),
        effort: "none".into(),
        adapter: "fixed".into(),
        context_window_tokens: 272_000,
        max_input_tokens: 272_000,
    };
    let skills = local_product::physician_chat::PhysicianSkills::configured(None, false).unwrap();
    local_product::physician_chat::recipe(&binding, "Owner", &skills)
}
