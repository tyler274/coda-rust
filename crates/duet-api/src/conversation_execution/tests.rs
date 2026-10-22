use super::*;
use front_door::identity::{LUNA_VERSION, Principal, TERMS_VERSION};
use harness::engine::{DeltaSink, Failure, Request, TextDelta, Turn};
use sqlx::{Connection, Executor, Row};
use uuid::Uuid;
struct Fixture {
    calls: usize,
    fail_after_delta: bool,
}
#[async_trait::async_trait]
impl Provider for Fixture {
    fn request_bytes(&self, _: &Request<'_>) -> Result<usize, String> {
        Ok(32)
    }
    async fn complete(&mut self, _: Request<'_>) -> Result<Turn, String> {
        unreachable!()
    }
    async fn complete_stream(
        &mut self,
        _: Request<'_>,
        sink: &mut dyn DeltaSink,
    ) -> Result<Turn, Failure> {
        self.calls += 1;
        sink.deliver(TextDelta {
            response_id: "fixture-response".into(),
            item_id: "fixture-item".into(),
            sequence: 1,
            output_index: 0,
            content_index: 0,
            text: "Controlled answer".into(),
        })
        .await?;
        if self.fail_after_delta {
            return Err(Failure::Provider("controlled interruption".into()));
        }
        Ok(Turn {
            items: vec![Item::Message {
                role: "assistant".into(),
                text: "Controlled answer".into(),
            }],
            receipt: serde_json::json!({"fixture":true}),
        })
    }
}
fn context() -> Context {
    Context {
        instructions: "Controlled local proof; no effects".into(),
        items: vec![Item::Message {
            role: "user".into(),
            text: "fixture input".into(),
        }],
        tools: vec![],
    }
}
#[tokio::test]
async fn postgres_dispatch_claim_stream_completion_and_recovery() {
    let url = match std::env::var("DUET_IDENTITY_TEST_DATABASE_URL") {
        Ok(v) if !v.is_empty() => v,
        _ => {
            assert!(
                std::env::var_os("CODA_REQUIRE_POSTGRES_TESTS").is_none(),
                "DUET_IDENTITY_TEST_DATABASE_URL required"
            );
            return;
        }
    };
    let base = url.rsplit_once('/').unwrap().0;
    let mut admin = sqlx::PgConnection::connect(&format!("{base}/postgres"))
        .await
        .unwrap();
    let name = format!("duet_exec_{}", Uuid::new_v4().simple());
    sqlx::query(sqlx::AssertSqlSafe(format!("CREATE DATABASE {name}")))
        .execute(&mut admin)
        .await
        .unwrap();
    let url = format!("{base}/{name}");
    let mut db = sqlx::PgConnection::connect(&url).await.unwrap();
    let store = Arc::new(PostgresFrontDoorStore::connect(&url).await.unwrap());
    store.migrate().await.unwrap();
    let p = Principal {
        project: "fixture".into(),
        subject: "alice".into(),
        email: "alice@example.test".into(),
        authenticated_at: 10,
    };
    let b = Principal {
        subject: "bob".into(),
        email: "bob@example.test".into(),
        ..p.clone()
    };
    let a = store.provision_identity(&p, 20).await.unwrap().account_id;
    store
        .accept_identity_terms(&p, TERMS_VERSION, 20)
        .await
        .unwrap();
    store
        .set_identity_luna(&p, LUNA_VERSION, true, 20)
        .await
        .unwrap();
    store.provision_identity(&b, 20).await.unwrap();
    store
        .set_model_execution(true, "fixture-operator", "controlled test")
        .await
        .unwrap();
    // Controlled SQL fixture seeds already-accepted work. It does NOT expose
    // an Admission constructor or enable a production qualification bypass.
    async fn seed(db: &mut sqlx::PgConnection, a: Uuid) -> Uuid {
        let run = Uuid::new_v4();
        let id = Uuid::new_v4();
        let reply = Uuid::new_v4();
        let mut tx = db.begin().await.unwrap();
        sqlx::query(
            "INSERT INTO duet_conversation_state(account_id) VALUES($1) ON CONFLICT DO NOTHING",
        )
        .bind(a)
        .execute(&mut *tx)
        .await
        .unwrap();
        let seq:i64=sqlx::query_scalar("UPDATE duet_conversation_state SET next_sequence=next_sequence+2,revision=revision+1 WHERE account_id=$1 RETURNING next_sequence-2").bind(a).fetch_one(&mut *tx).await.unwrap();
        sqlx::query("INSERT INTO duet_conversation_inputs(account_id,message_id,run_id,reply_id,timezone,state,permission_revision,revision,origin_project,origin_subject,origin_authenticated_at) VALUES($1,$2,$3,$4,'UTC','queued',2,1,'fixture','alice',10)").bind(a).bind(id).bind(run).bind(reply).execute(&mut *tx).await.unwrap();
        sqlx::query("INSERT INTO duet_conversation_messages(account_id,id,input_id,sequence,revision,role,text,state) VALUES($1,$2,$2,$4,1,'user','fixture input','complete'),($1,$3,$2,$4+1,1,'assistant','','pending')").bind(a).bind(id).bind(reply).bind(seq).execute(&mut *tx).await.unwrap();
        tx.commit().await.unwrap();
        run
    }
    async fn expire(db: &mut sqlx::PgConnection, run: Uuid) {
        sqlx::query("UPDATE duet_conversation_inputs SET lease_until=clock_timestamp()-interval '1 second' WHERE run_id=$1").bind(run).execute(db).await.unwrap();
    }
    let run = seed(&mut db, a).await;
    assert_eq!(store.pending_conversations(&p).await.unwrap(), vec![run]);
    assert!(store.claim_conversation(&b, run).await.unwrap().is_none());
    let (x, y) = tokio::join!(
        store.claim_conversation(&p, run),
        store.claim_conversation(&p, run)
    );
    let x = x.unwrap();
    let y = y.unwrap();
    assert_ne!(x.is_some(), y.is_some());
    let claim = x.or(y).unwrap();
    let permit = ExecutionPermit { _private: () };
    let cancel = Cancellation::default();
    let mut fixture = Fixture {
        calls: 0,
        fail_after_delta: true,
    };
    assert!(
        execute(
            store.clone(),
            claim.clone(),
            context(),
            &mut fixture,
            &cancel,
            &permit
        )
        .await
        .is_err()
    );
    let calls = fixture.calls;
    assert!(
        execute(
            store.clone(),
            claim.clone(),
            context(),
            &mut fixture,
            &cancel,
            &permit
        )
        .await
        .is_err()
    );
    assert_eq!(
        fixture.calls, calls,
        "same attempt cannot invoke the provider twice"
    );
    let events = store
        .conversation_events(&p, claim.attempt_id(), 0)
        .await
        .unwrap();
    assert_eq!(events.len(), 2);
    assert!(
        store
            .conversation_events(&b, claim.attempt_id(), 0)
            .await
            .unwrap()
            .is_empty()
    );
    store
        .acknowledge_conversation_event(&claim, &events[1])
        .await
        .unwrap();
    assert_eq!(
        store.conversation_messages(&p, None, 100).await.unwrap()[1].text,
        "Controlled answer"
    );
    let mut conflict = events[1].clone();
    conflict.body.push(' ');
    assert!(
        store
            .acknowledge_conversation_event(&claim, &conflict)
            .await
            .is_err()
    );
    expire(&mut db, run).await;
    assert!(
        store
            .complete_conversation(&claim, "Controlled answer")
            .await
            .is_err()
    );
    drop(store);
    let store = Arc::new(PostgresFrontDoorStore::connect(&url).await.unwrap());
    assert_eq!(store.pending_conversations(&p).await.unwrap(), vec![run]);
    let successor = store.claim_conversation(&p, run).await.unwrap().unwrap();
    assert_ne!(successor.attempt_id(), claim.attempt_id());
    assert_eq!(successor.run_id(), run);
    assert!(
        store
            .acknowledge_conversation_event(&claim, &events[1])
            .await
            .is_err()
    );
    assert_eq!(
        store.conversation_messages(&p, None, 100).await.unwrap()[1].text,
        ""
    );
    fixture.fail_after_delta = false;
    execute(
        store.clone(),
        successor.clone(),
        context(),
        &mut fixture,
        &cancel,
        &permit,
    )
    .await
    .unwrap();
    assert_eq!(fixture.calls, 2);
    store
        .complete_conversation(&successor, "Controlled answer")
        .await
        .unwrap();
    assert!(
        store
            .complete_conversation(&successor, "changed")
            .await
            .is_err()
    );
    assert!(store.claim_conversation(&p, run).await.unwrap().is_none());
    let messages = store.conversation_messages(&p, None, 100).await.unwrap();
    assert_eq!(messages[1].state, "complete");
    assert_eq!(messages[1].text, "Controlled answer");
    let saved = store
        .conversation_events(&p, successor.attempt_id(), 0)
        .await
        .unwrap();
    assert_eq!(saved.len(), 4);
    assert_eq!(
        store
            .conversation_events(&p, successor.attempt_id(), 2)
            .await
            .unwrap(),
        saved[2..]
    );
    // Intentional stop retains prose and is never recovered.
    let stopped = seed(&mut db, a).await;
    let c = store
        .claim_conversation(&p, stopped)
        .await
        .unwrap()
        .unwrap();
    fixture.fail_after_delta = true;
    assert!(
        execute(
            store.clone(),
            c.clone(),
            context(),
            &mut fixture,
            &cancel,
            &permit
        )
        .await
        .is_err()
    );
    store.stop_conversation(&p, stopped).await.unwrap();
    assert!(store.assert_conversation_claim(&c).await.is_err());
    assert!(
        store
            .claim_conversation(&p, stopped)
            .await
            .unwrap()
            .is_none()
    );
    // Bounded expiry exhaustion retains one failed response and all attempts.
    let exhausted = seed(&mut db, a).await;
    for _ in 0..3 {
        store
            .claim_conversation(&p, exhausted)
            .await
            .unwrap()
            .unwrap();
        expire(&mut db, exhausted).await;
    }
    assert!(
        store
            .claim_conversation(&p, exhausted)
            .await
            .unwrap()
            .is_none()
    );
    let state: String =
        sqlx::query_scalar("SELECT state FROM duet_conversation_inputs WHERE run_id=$1")
            .bind(exhausted)
            .fetch_one(&mut db)
            .await
            .unwrap();
    assert_eq!(state, "failed");
    // Unknown event gap and refused acknowledgment prevent dependent provider work.
    let fenced = seed(&mut db, a).await;
    let c = store.claim_conversation(&p, fenced).await.unwrap().unwrap();
    let gap = StoredEvent {
        sequence: 2,
        body: "gap".into(),
        delta: None,
        candidate: None,
    };
    assert!(
        store
            .acknowledge_conversation_event(&c, &gap)
            .await
            .is_err()
    );
    db.execute("ALTER TABLE duet_attempt_events ADD CONSTRAINT fixture_ack_failure CHECK (body NOT LIKE '%ProviderStarted%') NOT VALID").await.unwrap();
    let calls = fixture.calls;
    assert!(
        execute(
            store.clone(),
            c.clone(),
            context(),
            &mut fixture,
            &cancel,
            &permit
        )
        .await
        .is_err()
    );
    assert_eq!(fixture.calls, calls);
    db.execute("ALTER TABLE duet_attempt_events DROP CONSTRAINT fixture_ack_failure")
        .await
        .unwrap();
    // Completion rollback: candidate evidence is not a committed answer. A
    // failed final write leaves streaming work intact and can reconcile later.
    let atomic = seed(&mut db, a).await;
    let atomic_claim = store.claim_conversation(&p, atomic).await.unwrap().unwrap();
    let candidate = StoredEvent {
        sequence: 1,
        body: "controlled candidate".into(),
        delta: None,
        candidate: Some("atomic answer".into()),
    };
    store
        .acknowledge_conversation_event(&atomic_claim, &candidate)
        .await
        .unwrap();
    db.execute("ALTER TABLE duet_conversation_messages ADD CONSTRAINT fixture_completion_failure CHECK (text <> 'atomic answer')").await.unwrap();
    assert!(
        store
            .complete_conversation(&atomic_claim, "atomic answer")
            .await
            .is_err()
    );
    store
        .assert_conversation_claim(&atomic_claim)
        .await
        .unwrap();
    db.execute("ALTER TABLE duet_conversation_messages DROP CONSTRAINT fixture_completion_failure")
        .await
        .unwrap();
    store
        .complete_conversation(&atomic_claim, "atomic answer")
        .await
        .unwrap();
    // Stop and completion serialize on the account. Either completion wins,
    // or no result can overwrite the person's stop; repeated stop is harmless.
    let race = seed(&mut db, a).await;
    let rc = store.claim_conversation(&p, race).await.unwrap().unwrap();
    store
        .acknowledge_conversation_event(&rc, &candidate)
        .await
        .unwrap();
    let (done, stopped) = tokio::join!(
        store.complete_conversation(&rc, "atomic answer"),
        store.stop_conversation(&p, race)
    );
    stopped.unwrap();
    let state: String =
        sqlx::query_scalar("SELECT state FROM duet_conversation_inputs WHERE run_id=$1")
            .bind(race)
            .fetch_one(&mut db)
            .await
            .unwrap();
    assert_eq!(
        state,
        if done.is_ok() {
            "completed"
        } else {
            "interrupted"
        }
    );
    assert!(store.claim_conversation(&p, race).await.unwrap().is_none());
    // Retry deadline has independent force even before the three-attempt cap.
    let deadline = seed(&mut db, a).await;
    store
        .claim_conversation(&p, deadline)
        .await
        .unwrap()
        .unwrap();
    expire(&mut db, deadline).await;
    sqlx::query("UPDATE duet_conversation_inputs SET retry_deadline=clock_timestamp()-interval '1 second' WHERE run_id=$1").bind(deadline).execute(&mut db).await.unwrap();
    assert!(
        store
            .claim_conversation(&p, deadline)
            .await
            .unwrap()
            .is_none()
    );
    // A candidate blocked on the canonical account lock must evaluate expiry
    // after obtaining the lock, not from transaction-start time.
    let waiting = seed(&mut db, a).await;
    let waiting_claim = store
        .claim_conversation(&p, waiting)
        .await
        .unwrap()
        .unwrap();
    store
        .acknowledge_conversation_event(&waiting_claim, &candidate)
        .await
        .unwrap();
    let mut held = db.begin().await.unwrap();
    sqlx::query("SELECT id FROM front_door_accounts WHERE id=$1 FOR UPDATE")
        .bind(a)
        .fetch_one(&mut *held)
        .await
        .unwrap();
    let owner = store.clone();
    let mut completion = tokio::spawn(async move {
        owner
            .complete_conversation(&waiting_claim, "atomic answer")
            .await
    });
    assert!(
        tokio::time::timeout(Duration::from_millis(50), &mut completion)
            .await
            .is_err()
    );
    sqlx::query("UPDATE duet_conversation_inputs SET lease_until=clock_timestamp()-interval '1 second' WHERE run_id=$1").bind(waiting).execute(&mut *held).await.unwrap();
    held.commit().await.unwrap();
    assert!(completion.await.unwrap().is_err());
    // Terms pause is retained but cannot silently resume in this bounded slice.
    let paused = seed(&mut db, a).await;
    sqlx::query("UPDATE front_door_permissions SET terms_version='stale' WHERE account_id=$1")
        .bind(a)
        .execute(&mut db)
        .await
        .unwrap();
    assert!(
        store
            .claim_conversation(&p, paused)
            .await
            .unwrap()
            .is_none()
    );
    store
        .accept_identity_terms(&p, TERMS_VERSION, 25)
        .await
        .unwrap();
    assert!(
        store
            .claim_conversation(&p, paused)
            .await
            .unwrap()
            .is_none()
    );
    let state: String =
        sqlx::query_scalar("SELECT state FROM duet_conversation_inputs WHERE run_id=$1")
            .bind(paused)
            .fetch_one(&mut db)
            .await
            .unwrap();
    assert_eq!(state, "paused");
    // Migration never invents origin authority for accepted rows predating it.
    let legacy = seed(&mut db, a).await;
    sqlx::query("UPDATE duet_conversation_inputs SET origin_authenticated_at=NULL WHERE run_id=$1")
        .bind(legacy)
        .execute(&mut db)
        .await
        .unwrap();
    assert!(
        store
            .claim_conversation(&p, legacy)
            .await
            .unwrap()
            .is_none()
    );
    // Revocation fences running work; a freshly authenticated session must not
    // renew the original revoked run's authority when its lease expires.
    store.revoke_identity_sessions(&p, 30).await.unwrap();
    assert!(
        store
            .complete_conversation(&c, "Controlled answer")
            .await
            .is_err()
    );
    assert!(
        store
            .conversation_events(&p, c.attempt_id(), 0)
            .await
            .is_err()
    );
    expire(&mut db, fenced).await;
    let mut fresh = p.clone();
    fresh.authenticated_at = 31;
    assert!(
        store
            .claim_conversation(&fresh, fenced)
            .await
            .unwrap()
            .is_none()
    );
    let r = sqlx::query("SELECT state FROM duet_conversation_inputs WHERE run_id=$1")
        .bind(fenced)
        .fetch_one(&mut db)
        .await
        .unwrap();
    assert_eq!(r.get::<String, _>("state"), "failed");
    drop(store);
    db.close().await.unwrap();
    sqlx::query(sqlx::AssertSqlSafe(format!(
        "DROP DATABASE {name} WITH (FORCE)"
    )))
    .execute(&mut admin)
    .await
    .unwrap();
}
