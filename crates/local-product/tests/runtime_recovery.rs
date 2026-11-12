//! Controlled real-Postgres owner integration, never provider or browser proof.
use local_product::runtime::{
    engine::{self, Event, EventKind, Turn},
    *,
};
use physician_auth::{AuthStore, Membership, PhysicianIdentity, PostgresAuthStore};
use sea_orm::{Database, TransactionTrait};
#[path = "support/authority.rs"]
mod authority;
use authority::Locks;

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
                scenario: None,
            },
            PostgresAuthStore::session_reference(&token).to_vec(),
            &local_product::physician_chat::PhysicianSkills::configured(None, false).unwrap(),
            &test_chat_recipe(),
            None,
        )
        .await
        .unwrap();
    (store, auth, scope, token, email)
}
async fn candidate(store: &Store, scope: &Scope) -> (Claim, Turn) {
    let claim = store.claim(scope, 60).await.unwrap().unwrap();
    let candidate = Turn {
        items: vec![engine::Item::assistant("controlled completion")],
        receipt: serde_json::json!({"controlled":true}),
        usage: Default::default(),
    };
    // The terminal turn first, so only the authority fence can refuse it.
    let kinds = [
        EventKind::ProviderReturned(engine::Turn {
            usage: Default::default(),
            items: vec![engine::Item::Message {
                role: "assistant".into(),
                text: candidate.text(),
            }],
            receipt: serde_json::json!({"controlled": true}),
        }),
        EventKind::Candidate {
            text: candidate.text(),
        },
    ];
    for (sequence, kind) in (1..).zip(kinds) {
        store
            .persist(
                &claim,
                Event {
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
        let check_identity = identity.clone();
        let check_session = PostgresAuthStore::session_reference(&token);
        let revoker = tokio::spawn(async move {
            match action {
                "session" => auth.end_session(&token).await.unwrap(),
                "panel" => auth
                    .set_membership(&identity, &workspace, Membership::Denied)
                    .await
                    .unwrap(),
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
        let read = db
            .begin_with_config(None, Some(sea_orm::AccessMode::ReadOnly))
            .await
            .unwrap();
        let checked = tokio::time::timeout(
            std::time::Duration::from_secs(1),
            PostgresAuthStore::check_runtime_authority(
                &read,
                &check_session,
                &check_identity,
                &scope.workspace,
            ),
        )
        .await;
        read.rollback().await.unwrap();
        assert!(
            checked.is_ok(),
            "authority currency must not queue behind revocation locks"
        );
        checked.unwrap().unwrap();
        tx.commit().await.unwrap();
        revoker.await.unwrap();
        assert!(
            store
                .commit_authorized(&claim, &candidate, &mut Locks)
                .await
                .is_err()
        );
        assert_eq!(store.conversation(&scope).await.unwrap().messages.len(), 1);
    }
}

struct Stream(Scope);
impl engine::recipe::Provider for Stream {
    async fn exchange(
        &mut self,
        _: &engine::recipe::Request,
        _: &engine::Cancellation,
        _: &engine::RunLimits,
        observations: &mut dyn engine::recipe::Observer,
    ) -> Result<engine::Turn, engine::Cause> {
        observations
            .emit(engine::recipe::StreamEvent::Text(engine::TextDelta {
                location: Some(engine::DeltaLocation {
                    response_id: "r".into(),
                    item_id: "i".into(),
                    sequence: 1,
                    output_index: 0,
                    content_index: 0,
                }),
                text: "controlled completion".into(),
            }))
            .await?;
        // Live text is not retained by Chords and cannot publish a reply.
        let s = Store::connect(&std::env::var("PHYSICIAN_RUNTIME_TEST_DATABASE_URL").unwrap())
            .await
            .unwrap();
        let state = s.conversation(&self.0).await.unwrap();
        assert_eq!(state.messages.len(), 1);
        assert!(matches!(
            state.attempts.last().unwrap().events.last().unwrap().kind,
            EventKind::ProviderStarted { .. }
        ));
        Ok(engine::Turn {
            usage: Default::default(),
            items: vec![engine::Item::assistant("controlled completion")],
            receipt: serde_json::json!({"controlled":true}),
        })
    }
}

fn limits() -> engine::RunLimits {
    engine::RunLimits {
        max_response_bytes: 10000,
        ..engine::RunLimits::default()
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
    let due = s
        .dispatch_due(100, None)
        .await
        .unwrap()
        .deliveries
        .into_iter()
        .find(|d| d.scope == scope)
        .unwrap();
    let (a, b) = tokio::join!(s.claim_delivery(&due, 60), s.claim_delivery(&due, 60));
    let claims: Vec<_> = [a.unwrap(), b.unwrap()].into_iter().flatten().collect();
    let [claim]: [Claim; 1] = claims.try_into().unwrap();
    let mut host = PhysicianHost {
        store: s.clone(),
        claim,
        authority: Locks,
    };
    host.run(
        &mut Stream(scope.clone()),
        limits(),
        &engine::Cancellation::default(),
    )
    .await
    .unwrap();
    let state = s.conversation(&scope).await.unwrap();
    assert_eq!(state.status, WorkStatus::Completed);
    assert_eq!(
        state.attempts.last().unwrap().predecessor.as_deref(),
        Some(first.attempt.as_str())
    );
    assert_eq!(state.messages.len(), 2);
    assert!(s.claim_delivery(&due, 60).await.unwrap().is_none());
    let (held, _, scope, token, _) = setup().await;
    let work = held.conversation(&scope).await.unwrap().work;
    let session = PostgresAuthStore::session_reference(&token).to_vec();
    assert_eq!(
        held.stop_authorized(&scope, "stop-pending", session, &work)
            .await
            .unwrap(),
        local_product::runtime::Stopped::Held
    );
    assert!(
        held.dispatch_due(100, None)
            .await
            .unwrap()
            .deliveries
            .iter()
            .all(|d| d.scope != scope)
    );
    assert_eq!(
        held.conversation(&scope).await.unwrap().status,
        WorkStatus::Held
    );
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
