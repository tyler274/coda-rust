use super::*;
use crate::identity::{LUNA_VERSION, TERMS_VERSION};
use sea_orm::Database;
fn principal() -> Principal {
    let id = Uuid::new_v4();
    Principal {
        project: "fixture".into(),
        subject: id.to_string(),
        email: format!("{id}@example.test"),
        authenticated_at: 10,
    }
}
fn input(text: &str) -> Input {
    Input {
        id: Uuid::new_v4(),
        text: text.into(),
        timezone: "America/New_York".into(),
    }
}
#[tokio::test]
async fn postgres_acceptance_isolation_dedup_restart_and_withdrawal() {
    let url = match std::env::var("DUET_IDENTITY_TEST_DATABASE_URL") {
        Ok(v) if !v.trim().is_empty() => v,
        _ => {
            assert!(
                std::env::var_os("CODA_REQUIRE_POSTGRES_TESTS").is_none(),
                "DUET_IDENTITY_TEST_DATABASE_URL is required"
            );
            return;
        }
    };
    let base = url.rsplit_once('/').unwrap().0;
    let admin = Database::connect(format!("{base}/postgres")).await.unwrap();
    let name = format!("duet_owner_{}", Uuid::new_v4().simple());
    admin
        .execute_unprepared(&format!("CREATE DATABASE {name}"))
        .await
        .unwrap();
    let url = format!("{base}/{name}");
    let store = PostgresFrontDoorStore::connect(&url).await.unwrap();
    store.migrate().await.unwrap();
    store.migrate().await.unwrap();
    let alice = principal();
    let bob = principal();
    let first = input("  exact text\nnot normalized  ");
    // Capability is constructed only inside this cfg(test) child module. There
    // is no feature/env/trait hook that can manufacture admission in production.
    let admission = Admission { limit: 1000 };
    store
        .set_model_execution(true, "fixture-operator", "controlled test")
        .await
        .unwrap();
    assert!(
        store
            .accept_conversation(&alice, &first, Some(&admission))
            .await
            .is_err()
    );
    for p in [&alice, &bob] {
        store.provision_identity(p, 20).await.unwrap();
        assert!(
            store
                .accept_conversation(p, &first, Some(&admission))
                .await
                .is_err()
        );
        store
            .accept_identity_terms(p, TERMS_VERSION, 20)
            .await
            .unwrap();
        assert!(
            store
                .accept_conversation(p, &first, Some(&admission))
                .await
                .is_err()
        );
        store
            .set_identity_luna(p, LUNA_VERSION, true, 20)
            .await
            .unwrap();
    }
    assert!(
        store
            .accept_conversation(&alice, &first, None)
            .await
            .is_err()
    );
    assert!(
        store
            .conversation_messages(&alice, None, 100)
            .await
            .unwrap()
            .is_empty()
    );
    let (a, b) = tokio::join!(
        store.accept_conversation(&alice, &first, Some(&admission)),
        store.accept_conversation(&alice, &first, Some(&admission))
    );
    let a = a.unwrap();
    assert_eq!(a, b.unwrap());
    assert_eq!(a.state, "queued");
    let page = store
        .conversation_messages(&alice, None, 100)
        .await
        .unwrap();
    assert_eq!(page.len(), 2);
    assert_eq!(page[0].text, first.text);
    assert_eq!(page[0].state, "complete");
    assert_eq!(page[1].text, "");
    assert_eq!(page[1].state, "pending");
    assert_eq!(page[1].id, a.reply_id);
    assert!(
        store
            .conversation_acceptance(&bob, first.id)
            .await
            .unwrap()
            .is_none()
    );
    assert!(
        store
            .conversation_messages(&bob, None, 100)
            .await
            .unwrap()
            .is_empty()
    );
    let same_id = store
        .accept_conversation(&bob, &first, Some(&admission))
        .await
        .unwrap();
    assert_ne!(same_id.run_id, a.run_id);
    let changed = Input {
        id: first.id,
        text: "different".into(),
        timezone: "UTC".into(),
    };
    assert!(matches!(
        store
            .accept_conversation(&alice, &changed, Some(&admission))
            .await,
        Err(AuthorityError::AgreementChanged)
    ));
    let retry = Input {
        id: first.id,
        text: first.text.clone(),
        timezone: "UTC".into(),
    };
    assert_eq!(
        store
            .accept_conversation(&alice, &retry, None)
            .await
            .unwrap(),
        a
    );
    drop(store);
    let store = PostgresFrontDoorStore::connect(&url).await.unwrap();
    assert_eq!(
        store
            .conversation_acceptance(&alice, first.id)
            .await
            .unwrap()
            .unwrap(),
        a
    );
    assert_eq!(
        store
            .conversation_messages(&alice, None, 100)
            .await
            .unwrap(),
        page
    );
    let old_claim = store
        .claim_conversation(&alice, a.run_id)
        .await
        .unwrap()
        .unwrap();
    let second = input("new direction");
    let newer = store
        .accept_conversation(&alice, &second, Some(&admission))
        .await
        .unwrap();
    assert!(newer.revision > a.revision);
    assert!(store.assert_conversation_claim(&old_claim).await.is_err());
    assert!(
        store
            .complete_conversation(&old_claim, "late reply")
            .await
            .is_err()
    );
    let active = store
        .claim_conversation(&alice, newer.run_id)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(
        store
            .conversation_acceptance(&alice, first.id)
            .await
            .unwrap()
            .unwrap()
            .state,
        "interrupted"
    );
    assert_eq!(
        store
            .conversation_messages(&alice, Some(3), 100)
            .await
            .unwrap()
            .len(),
        2
    );
    // Permission mutation shares the same account row lock; either acceptance
    // wins and is interrupted by withdrawal, or no draft is stored at all.
    let raced = input("racing withdrawal");
    let (accepted, withdrawn) = tokio::join!(
        store.accept_conversation(&alice, &raced, Some(&admission)),
        store.set_identity_luna(&alice, LUNA_VERSION, false, 30)
    );
    withdrawn.unwrap();
    assert!(store.assert_conversation_claim(&active).await.is_err());
    assert!(
        store
            .complete_conversation(&active, "late after withdrawal")
            .await
            .is_err()
    );
    if accepted.is_ok() {
        assert_eq!(
            store
                .conversation_acceptance(&alice, raced.id)
                .await
                .unwrap()
                .unwrap()
                .state,
            "interrupted"
        );
    } else {
        assert!(
            store
                .conversation_acceptance(&alice, raced.id)
                .await
                .unwrap()
                .is_none()
        );
    }
    assert_eq!(
        store
            .conversation_acceptance(&alice, second.id)
            .await
            .unwrap()
            .unwrap()
            .state,
        "interrupted"
    );
    assert!(
        store
            .accept_conversation(&alice, &input("not permitted"), Some(&admission))
            .await
            .is_err()
    );
    assert!(
        store
            .accept_conversation(&alice, &first, None)
            .await
            .is_ok(),
        "retry after withdrawal resolves retained acceptance"
    );
    let mut foreign = alice.clone();
    foreign.project = "foreign".into();
    assert!(
        store
            .conversation_messages(&foreign, None, 100)
            .await
            .is_err()
    );
    store.revoke_identity_sessions(&alice, 40).await.unwrap();
    assert!(
        store
            .conversation_messages(&alice, None, 100)
            .await
            .is_err()
    );
    assert!(
        store
            .accept_conversation(&alice, &first, None)
            .await
            .is_err()
    );
    let mut invalid = bob.clone();
    invalid.authenticated_at = 0;
    assert!(
        store
            .conversation_acceptance(&invalid, first.id)
            .await
            .is_err()
    );
    // Refused invalid payloads leave no partial rows or sequence holes.
    for bad in [
        input("   "),
        input(&"x".repeat(32769)),
        Input {
            timezone: "not-a-zone".into(),
            ..input("valid text")
        },
    ] {
        assert!(
            store
                .accept_conversation(&bob, &bad, Some(&admission))
                .await
                .is_err()
        );
        assert!(
            store
                .conversation_acceptance(&bob, bad.id)
                .await
                .unwrap()
                .is_none()
        );
    }
    assert_eq!(
        store
            .conversation_messages(&bob, None, 100)
            .await
            .unwrap()
            .len(),
        2
    );
    // Fault after run insertion must roll back run, messages, revision and prior
    // interruption together (not an accepted orphan or partial message pair).
    store.db.execute_unprepared("ALTER TABLE duet_conversation_messages ADD CONSTRAINT fixture_failure CHECK (text <> 'fail-insert')").await.unwrap();
    let bad = input("fail-insert");
    assert!(
        store
            .accept_conversation(&bob, &bad, Some(&admission))
            .await
            .is_err()
    );
    assert!(
        store
            .conversation_acceptance(&bob, bad.id)
            .await
            .unwrap()
            .is_none()
    );
    assert_eq!(
        store
            .conversation_acceptance(&bob, first.id)
            .await
            .unwrap()
            .unwrap()
            .state,
        "queued"
    );
    assert_eq!(
        store
            .conversation_messages(&bob, None, 100)
            .await
            .unwrap()
            .len(),
        2
    );
    // Concurrent revocation either fences acceptance or follows its commit;
    // neither a retry nor readback can use the revoked principal afterward.
    let raced = input("racing revocation");
    let (result, revoked) = tokio::join!(
        store.accept_conversation(&bob, &raced, Some(&admission)),
        store.revoke_identity_sessions(&bob, 40)
    );
    revoked.unwrap();
    assert!(store.conversation_acceptance(&bob, raced.id).await.is_err());
    assert!(
        store
            .accept_conversation(&bob, &raced, Some(&admission))
            .await
            .is_err()
    );
    let mut refreshed = bob.clone();
    refreshed.authenticated_at = 41;
    assert_eq!(
        store
            .conversation_acceptance(&refreshed, raced.id)
            .await
            .unwrap()
            .is_some(),
        result.is_ok()
    );
    assert!(
        store
            .conversation_messages(&refreshed, None, 0)
            .await
            .is_err()
    );
    assert!(
        store
            .conversation_messages(&refreshed, Some(0), 100)
            .await
            .is_err()
    );
    assert!(
        store
            .conversation_messages(&refreshed, None, 101)
            .await
            .is_err()
    );
    // Current terms, not merely historical consent, are an admission fence.
    let owner = store.identity_access(&refreshed).await.unwrap().account_id;
    store
        .db
        .execute_raw(sql(
            "UPDATE front_door_permissions SET terms_version='outdated' WHERE account_id=$1",
            vec![owner.into()],
        ))
        .await
        .unwrap();
    let stale = input("stale terms cannot admit");
    assert!(
        store
            .accept_conversation(&refreshed, &stale, Some(&admission))
            .await
            .is_err()
    );
    assert!(
        store
            .conversation_acceptance(&refreshed, stale.id)
            .await
            .unwrap()
            .is_none()
    );
    assert!(
        store
            .accept_conversation(&refreshed, &first, None)
            .await
            .is_ok()
    );
    store
        .db
        .execute_raw(sql(
            "UPDATE front_door_accounts SET active=false WHERE id=$1",
            vec![owner.into()],
        ))
        .await
        .unwrap();
    assert!(
        store
            .conversation_messages(&refreshed, None, 100)
            .await
            .is_err()
    );
    // Accepted-input -> claim -> candidate races use the real owner, not SQL
    // authority synthesis. If a control wins, a late candidate cannot publish.
    for control in ["withdraw", "revoke", "new-input"] {
        let p = principal();
        store.provision_identity(&p, 20).await.unwrap();
        store
            .accept_identity_terms(&p, TERMS_VERSION, 20)
            .await
            .unwrap();
        store
            .set_identity_luna(&p, LUNA_VERSION, true, 20)
            .await
            .unwrap();
        let accepted = store
            .accept_conversation(&p, &input("raced work"), Some(&admission))
            .await
            .unwrap();
        let claim = store
            .claim_conversation(&p, accepted.run_id)
            .await
            .unwrap()
            .unwrap();
        let event = execution::StoredEvent {
            sequence: 1,
            body: "fixture candidate".into(),
            delta: None,
            candidate: Some("candidate".into()),
        };
        store
            .acknowledge_conversation_event(&claim, &event)
            .await
            .unwrap();
        let mutate = async {
            match control {
                "withdraw" => {
                    store
                        .set_identity_luna(&p, LUNA_VERSION, false, 30)
                        .await
                        .unwrap();
                }
                "revoke" => {
                    store.revoke_identity_sessions(&p, 30).await.unwrap();
                }
                _ => {
                    store
                        .accept_conversation(&p, &input("new direction"), Some(&admission))
                        .await
                        .unwrap();
                }
            }
        };
        let (completion, ()) =
            tokio::join!(store.complete_conversation(&claim, "candidate"), mutate);
        let mut reader = p.clone();
        reader.authenticated_at = 31;
        let stored = store
            .conversation_acceptance(&reader, accepted.message_id)
            .await
            .unwrap()
            .unwrap();
        if completion.is_ok() {
            assert_eq!(stored.state, "completed");
        } else {
            assert_ne!(stored.state, "completed");
        }
        // Even when the revoked run is still marked running until recovery,
        // its original principal is fenced; reauthentication cannot revive it.
        if control == "revoke" || completion.is_err() {
            assert!(
                store
                    .complete_conversation(&claim, "candidate")
                    .await
                    .is_err()
            );
        }
    }
    drop(store);
    admin
        .execute_unprepared(&format!("DROP DATABASE {name} WITH (FORCE)"))
        .await
        .unwrap();
    admin.close().await.unwrap();
}
async fn scratch(
    prefix: &str,
) -> Option<(sea_orm::DatabaseConnection, String, PostgresFrontDoorStore)> {
    let url = match std::env::var("DUET_IDENTITY_TEST_DATABASE_URL") {
        Ok(v) if !v.trim().is_empty() => v,
        _ => {
            assert!(
                std::env::var_os("CODA_REQUIRE_POSTGRES_TESTS").is_none(),
                "DUET_IDENTITY_TEST_DATABASE_URL is required"
            );
            return None;
        }
    };
    let base = url.rsplit_once('/').unwrap().0;
    let admin = Database::connect(format!("{base}/postgres")).await.unwrap();
    let name = format!("{prefix}_{}", Uuid::new_v4().simple());
    admin
        .execute_unprepared(&format!("CREATE DATABASE {name}"))
        .await
        .unwrap();
    let store = PostgresFrontDoorStore::connect(&format!("{base}/{name}"))
        .await
        .unwrap();
    store.migrate().await.unwrap();
    Some((admin, name, store))
}
async fn consenting(store: &PostgresFrontDoorStore) -> Principal {
    let p = principal();
    store.provision_identity(&p, 20).await.unwrap();
    store
        .accept_identity_terms(&p, TERMS_VERSION, 20)
        .await
        .unwrap();
    store
        .set_identity_luna(&p, LUNA_VERSION, true, 20)
        .await
        .unwrap();
    p
}
#[tokio::test]
async fn postgres_operator_switch_allowance_and_context() {
    let Some((admin, name, store)) = scratch("duet_admission").await else {
        return;
    };
    let p = consenting(&store).await;
    let admission = Admission { limit: 2 };
    // No switch history is off: refused before anything is stored.
    assert!(!store.model_execution_enabled().await.unwrap());
    let first = input("first question");
    assert!(matches!(
        store
            .accept_conversation(&p, &first, Some(&admission))
            .await,
        Err(AuthorityError::ModelExecutionOff)
    ));
    assert!(
        store
            .conversation_messages(&p, None, 10)
            .await
            .unwrap()
            .is_empty()
    );
    // Audit rows require an operator and reason, and history is append-only.
    assert!(store.set_model_execution(true, " ", "x").await.is_err());
    let on = store
        .set_model_execution(true, "operator@example.test", "fixture enable")
        .await
        .unwrap();
    assert!(on.enabled && store.model_execution_enabled().await.unwrap());
    assert!(
        store
            .db
            .execute_unprepared("DELETE FROM duet_model_execution_switch")
            .await
            .is_err()
    );
    // Allowance: limit 2, resend never counts twice, invalid zone refused.
    let a = store
        .accept_conversation(&p, &first, Some(&admission))
        .await
        .unwrap();
    store
        .accept_conversation(&p, &first, Some(&admission))
        .await
        .unwrap();
    let mut bad = input("bad zone");
    bad.timezone = "Not/AZone".into();
    assert!(
        store
            .accept_conversation(&p, &bad, Some(&admission))
            .await
            .is_err()
    );
    let second = input("second question");
    store
        .accept_conversation(&p, &second, Some(&admission))
        .await
        .unwrap();
    let third = input("third question");
    let limited = store
        .accept_conversation(&p, &third, Some(&admission))
        .await;
    let Err(AuthorityError::DailyLimit { resets_at }) = limited else {
        panic!("expected daily limit, got {limited:?}");
    };
    assert!(resets_at > 0);
    // A different zone opens a different local day but the rolling backstop
    // (2 x limit in 24h) still binds once reached.
    let mut tokyo = input("other zone");
    tokyo.timezone = "Pacific/Kiritimati".into();
    let roll = Admission { limit: 1 };
    assert!(matches!(
        store.accept_conversation(&p, &tokyo, Some(&roll)).await,
        Err(AuthorityError::DailyLimit { .. })
    ));
    // Failed work does not count against the allowance.
    store
        .db
        .execute_unprepared("UPDATE duet_conversation_inputs SET state='failed'")
        .await
        .unwrap();
    store
        .accept_conversation(&p, &third, Some(&admission))
        .await
        .unwrap();
    // Context is assembled only under a live claim, from Duet-owned rows.
    let run = store
        .conversation_acceptance(&p, third.id)
        .await
        .unwrap()
        .unwrap()
        .run_id;
    let claim = store.claim_conversation(&p, run).await.unwrap().unwrap();
    let ctx = store.conversation_context(&claim).await.unwrap();
    assert_eq!(ctx.current_message_id, third.id);
    assert_eq!(ctx.attempt, 1);
    let convo = ctx.payload["conversation"].as_array().unwrap();
    let texts: Vec<_> = convo.iter().map(|m| m["text"].as_str().unwrap()).collect();
    // Interrupted empty assistant placeholders are not things Duet said.
    assert_eq!(
        texts,
        ["first question", "second question", "third question"]
    );
    assert_eq!(ctx.payload["local_timezone"], "America/New_York");
    assert_eq!(ctx.payload["assignment"]["kind"], "conversation");
    assert!(serde_json::Value::is_null(
        &ctx.payload["working_understanding"]
    ));
    assert!(ctx.payload.get("earlier_attempt").is_none());
    assert_eq!(
        ctx.payload["conversation_history_boundary"]["first_message_id"],
        a.message_id.to_string()
    );
    // Another account's claim never sees this transcript.
    let other = consenting(&store).await;
    let theirs = input("other person");
    let o = store
        .accept_conversation(&other, &theirs, Some(&admission))
        .await
        .unwrap();
    let oc = store
        .claim_conversation(&other, o.run_id)
        .await
        .unwrap()
        .unwrap();
    let octx = store.conversation_context(&oc).await.unwrap();
    assert_eq!(octx.payload["conversation"].as_array().unwrap().len(), 1);
    // Switching off pauses running and queued work as the same runs, fences
    // context, events and completion, and refuses claims until resumed.
    store
        .set_model_execution(false, "operator@example.test", "fixture pause")
        .await
        .unwrap();
    assert!(matches!(
        store.conversation_context(&claim).await,
        Err(AuthorityError::ModelExecutionOff)
    ));
    assert!(store.complete_conversation(&claim, "late").await.is_err());
    let paused = store
        .conversation_acceptance(&p, third.id)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(paused.state, "paused");
    assert!(store.claim_conversation(&p, run).await.unwrap().is_none());
    store
        .set_model_execution(true, "operator@example.test", "fixture resume")
        .await
        .unwrap();
    // Resumption is not implemented: paused work is retained, not re-run.
    assert!(store.claim_conversation(&p, run).await.unwrap().is_none());
    // Paused work keeps its allowance reservation.
    let fourth = input("fourth question");
    store
        .accept_conversation(&p, &fourth, Some(&admission))
        .await
        .unwrap();
    assert!(matches!(
        store
            .accept_conversation(&p, &input("fifth"), Some(&admission))
            .await,
        Err(AuthorityError::DailyLimit { .. })
    ));
    drop(store);
    admin
        .execute_unprepared(&format!("DROP DATABASE {name} WITH (FORCE)"))
        .await
        .unwrap();
}
#[tokio::test]
async fn postgres_retry_context_reports_earlier_attempt() {
    let Some((admin, name, store)) = scratch("duet_ctx_retry").await else {
        return;
    };
    let p = consenting(&store).await;
    store
        .set_model_execution(true, "operator@example.test", "fixture enable")
        .await
        .unwrap();
    let admission = Admission { limit: 50 };
    let msg = input("retry me");
    let run = store
        .accept_conversation(&p, &msg, Some(&admission))
        .await
        .unwrap()
        .run_id;
    store.claim_conversation(&p, run).await.unwrap().unwrap();
    store
        .db
        .execute_unprepared(
            "UPDATE duet_conversation_inputs SET lease_until=clock_timestamp()-interval '1 second'",
        )
        .await
        .unwrap();
    let claim = store.claim_conversation(&p, run).await.unwrap().unwrap();
    let ctx = store.conversation_context(&claim).await.unwrap();
    assert_eq!(ctx.attempt, 2);
    assert_eq!(ctx.payload["earlier_attempt"]["attempt"], 2);
    assert_eq!(
        ctx.payload["earlier_attempt"]["note"],
        context::EARLIER_ATTEMPT_NOTE
    );
    drop(store);
    admin
        .execute_unprepared(&format!("DROP DATABASE {name} WITH (FORCE)"))
        .await
        .unwrap();
}
