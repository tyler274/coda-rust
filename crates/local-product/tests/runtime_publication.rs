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
            .await;
        auth.set_membership(&identity, &scope.workspace, Membership::Allowed)
            .await;
        store
            .accept_authorized(
                &scope,
                &Input {
                    key: "k".into(),
                    text: "input".into(),
                    timezone: "UTC".into(),
                },
                PostgresAuthStore::session_reference(&token),
            )
            .await
            .unwrap();
        let claim = store
            .claim(&scope, if expire { 1 } else { 60 })
            .await
            .unwrap()
            .unwrap();
        let candidate = engine::Candidate {
            text: "controlled".into(),
            transcript: vec![],
            usage: engine::Usage::default(),
        };
        store
            .persist(
                &claim,
                engine::Event {
                    attempt_id: claim.attempt.clone(),
                    sequence: 1,
                    kind: engine::EventKind::Candidate {
                        text: candidate.text.clone(),
                    },
                },
            )
            .await
            .unwrap();
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
        let revocation = tokio::spawn(async move { a.end_session(&token).await });
        tokio::time::sleep(std::time::Duration::from_millis(if expire {
            2100
        } else {
            75
        }))
        .await;
        assert!(!revocation.is_finished());
        proceed_tx.send(()).unwrap();
        let result = publication.await.unwrap();
        assert_eq!(result.is_ok(), !expire);
        revocation.await.unwrap();
        assert_eq!(
            store.conversation(&scope).await.unwrap().messages.len(),
            if expire { 1 } else { 2 }
        );
    }
}
