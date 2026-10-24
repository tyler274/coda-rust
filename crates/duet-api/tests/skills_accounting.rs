//! Deterministic provider + real isolated PostgreSQL; no paid calls.
use async_trait::async_trait;
use duet_api::{
    ApiError,
    conversation::ConversationStore,
    database::{AccountDatabase, migrate},
    identity::{IdentityStore, IdentityVerifier, VerifiedIdentity},
};
use harness::execution::{
    Call, Cancellation, Limits, Owner, Provider, Record, Request, Response, StreamEvent, Usage,
};
use serde_json::{Value, json};
use sqlx::AssertSqlSafe;
use sqlx::{ConnectOptions, Connection};
use std::sync::Arc;
use uuid::Uuid;
struct Identity;
#[async_trait]
impl IdentityVerifier for Identity {
    async fn verify(&self, bearer: &str) -> Result<VerifiedIdentity, ApiError> {
        Ok(VerifiedIdentity {
            subject: format!("accounting-{bearer}"),
            authenticated_at: chrono::Utc::now(),
        })
    }
}
struct Local {
    step: usize,
    fail: bool,
}
impl Provider for Local {
    async fn exchange(
        &mut self,
        request: &Request,
        _: &Cancellation,
        emit: &mut (dyn FnMut(StreamEvent) -> Result<(), String> + Send),
    ) -> Result<Response, String> {
        assert!(request.instructions.contains("Bundled Duet guidance"));
        assert!(request.tools.iter().any(|t| t["name"] == "skill_view"));
        assert_eq!(request.options["max_output_tokens"], 16384);
        self.step += 1;
        if self.fail {
            return Err("deterministic transport failure".into());
        }
        let usage = Usage {
            input_tokens: Some(100),
            output_tokens: Some(20),
        };
        emit(StreamEvent::Usage(usage.clone()))?;
        Ok(Response {
            text: if self.step == 1 {
                String::new()
            } else {
                "Final answer".into()
            },
            calls: if self.step == 1 {
                vec![Call {
                    id: "guidance".into(),
                    name: "skill_view".into(),
                    arguments: json!({"name":"health-evidence"}),
                }]
            } else {
                vec![]
            },
            usage,
            provenance: json!({"mode":"local","response_id":format!("local-{}",self.step),"cached_input_tokens":10}),
        })
    }
}
#[tokio::test]
async fn admitted_skills_recovery_accounting_and_bounds() {
    let url = std::env::var("DUET_TEST_DATABASE_URL").expect("real local PostgreSQL required");
    let options: sqlx::postgres::PgConnectOptions = url.parse().unwrap();
    assert!(matches!(
        options.get_host(),
        "127.0.0.1" | "localhost" | "::1"
    ));
    let mut admin = sqlx::PgConnection::connect(&url).await.unwrap();
    let suffix = Uuid::new_v4().simple().to_string();
    let name = format!("skills_{suffix}");
    let role = format!("skills_role_{suffix}");
    sqlx::query(AssertSqlSafe(format!("create role {role} login")))
        .execute(&mut admin)
        .await
        .unwrap();
    sqlx::query(AssertSqlSafe(format!("create database {name}")))
        .execute(&mut admin)
        .await
        .unwrap();
    let options = options.database(&name);
    migrate(options.to_url_lossy().as_str(), &role)
        .await
        .unwrap();
    let runtime = options.username(&role).to_url_lossy().to_string();
    let db = AccountDatabase::connect(&runtime).await.unwrap();
    let identity = IdentityStore::new(db.clone(), Arc::new(Identity));
    let a = identity.authenticate("a", true).await.unwrap();
    let b = identity.authenticate("b", true).await.unwrap();
    // Only identity/consent setup; no producer or usage fixture rows.
    let mut tx = db.begin(a.account_id).await.unwrap();
    sqlx::query("update users set terms_accepted_at=now(),terms_version='2026-09-23',luna_consent_decision='accepted',luna_consent_version='2026-09-24' where id=$1").bind(a.account_id).execute(&mut *tx).await.unwrap();
    tx.commit().await.unwrap();
    let a = identity.authenticate("a", true).await.unwrap();
    let store = ConversationStore::new(db.clone());
    let id = Uuid::new_v4();
    store
        .phone_accept(&a, id, "Help with evidence", "gpt-6-luna")
        .await
        .unwrap();
    let mut owner = store.claim(&a, id).await.unwrap();
    assert!(
        owner
            .execute(
                &mut Local {
                    step: 0,
                    fail: true
                },
                Limits::default(),
                &Cancellation::default()
            )
            .await
            .is_err()
    );
    let usage = store.usage(&a, id).await.unwrap();
    assert_eq!(usage["model_calls"], 1);
    assert_eq!(usage["unknown_usage_model_calls"], 1);
    assert!(usage["input_tokens"].is_null());
    assert_eq!(usage["cost_status"], "unknown");
    assert!(store.usage(&b, id).await.is_err());
    let restored = ConversationStore::new(AccountDatabase::connect(&runtime).await.unwrap());
    let mut recovered = restored.claim(&a, id).await.unwrap();
    recovered
        .execute(
            &mut Local {
                step: 0,
                fail: false,
            },
            Limits::default(),
            &Cancellation::default(),
        )
        .await
        .unwrap();
    // Late usage for the old claim corrects precisely its original attempt.
    owner
        .record(Record::Observation {
            sequence: 1,
            usage: Usage {
                input_tokens: Some(0),
                output_tokens: Some(0),
            },
            provenance: json!({"response_id":"late"}),
        })
        .await
        .unwrap();
    owner
        .record(Record::Observation {
            sequence: 1,
            usage: Usage::default(),
            provenance: Value::Null,
        })
        .await
        .unwrap();
    let usage = restored.usage(&a, id).await.unwrap();
    assert_eq!(usage["model_calls"], 3);
    assert_eq!(usage["unknown_usage_model_calls"], 0);
    assert_eq!(usage["input_tokens"], 200);
    assert_eq!(usage["output_tokens"], 40);
    assert_eq!(usage["cost_status"], "estimated");
    assert_eq!(
        usage["estimated_cost_usd"]
            .as_str()
            .unwrap()
            .parse::<f64>()
            .unwrap(),
        0.0000382
    );
    let mut tx = db.begin(a.account_id).await.unwrap();
    let opened:Value=sqlx::query_scalar("select result from duet_tool_effects where user_id=$1 and command_id=$2 and invocation_id='guidance'").bind(a.account_id).bind(id).fetch_one(&mut *tx).await.unwrap();
    assert_eq!(
        opened["library_version"],
        duet_api::skills::catalog()["version"]
    );
    assert_eq!(opened["skill"]["name"], "health-evidence");
    let frozen: Value =
        sqlx::query_scalar("select skill_catalog from duet_commands where user_id=$1 and id=$2")
            .bind(a.account_id)
            .bind(id)
            .fetch_one(&mut *tx)
            .await
            .unwrap();
    assert_eq!(frozen, duet_api::skills::catalog());
    tx.commit().await.unwrap();
    // Capability admission rejects guessed physician tools and filesystem reads.
    let id2 = Uuid::new_v4();
    store
        .phone_accept(&a, id2, "Next", "unpriced-local")
        .await
        .unwrap();
    let mut next = store.claim(&a, id2).await.unwrap();
    assert!(
        next.dispatch(
            &Call {
                id: "x".into(),
                name: "physician_write".into(),
                arguments: json!({})
            },
            &Cancellation::default()
        )
        .await
        .is_err()
    );
    assert!(
        next.dispatch(
            &Call {
                id: "y".into(),
                name: "skill_view".into(),
                arguments: json!({"name":"../../etc/passwd"})
            },
            &Cancellation::default()
        )
        .await
        .is_err()
    );
    // Owner bounds cannot be raised by the caller and survive same-command recovery.
    for n in 0..40 {
        next.dispatch(
            &Call {
                id: format!("skill-{n}"),
                name: "skill_view".into(),
                arguments: json!({"name":"health-evidence"}),
            },
            &Cancellation::default(),
        )
        .await
        .unwrap();
    }
    assert!(
        next.dispatch(
            &Call {
                id: "overflow".into(),
                name: "skill_view".into(),
                arguments: json!({"name":"health-evidence"})
            },
            &Cancellation::default()
        )
        .await
        .is_err()
    );
    // Idempotent effect replay consumes neither another call nor more bytes.
    next.dispatch(
        &Call {
            id: "skill-0".into(),
            name: "skill_view".into(),
            arguments: json!({"name":"health-evidence"}),
        },
        &Cancellation::default(),
    )
    .await
    .unwrap();
    next.execute(
        &mut Local {
            step: 1,
            fail: false,
        },
        Limits::default(),
        &Cancellation::default(),
    )
    .await
    .unwrap();
    assert_eq!(
        store.usage(&a, id2).await.unwrap()["cost_status"],
        "unknown"
    );
    // Persistent provider-call budget survives recovery and cannot be widened.
    let capped = Uuid::new_v4();
    store
        .phone_accept(&a, capped, "Bounded request", "local")
        .await
        .unwrap();
    for _ in 0..16 {
        let mut claim = store.claim(&a, capped).await.unwrap();
        assert!(
            claim
                .execute(
                    &mut Local {
                        step: 0,
                        fail: true
                    },
                    Limits {
                        max_calls: 100,
                        ..Limits::default()
                    },
                    &Cancellation::default()
                )
                .await
                .is_err()
        );
    }
    let mut claim = store.claim(&a, capped).await.unwrap();
    let mut provider = Local {
        step: 0,
        fail: true,
    };
    assert!(
        claim
            .execute(
                &mut provider,
                Limits {
                    max_calls: 100,
                    ..Limits::default()
                },
                &Cancellation::default()
            )
            .await
            .is_err()
    );
    assert_eq!(provider.step, 0, "budget exhausted before network dispatch");
    assert_eq!(store.usage(&a, capped).await.unwrap()["model_calls"], 16);
    // The admission start is durable, not reset by claim recovery.
    let mut tx = db.begin(a.account_id).await.unwrap();
    sqlx::query("update duet_commands set execution_started_at=clock_timestamp()-interval '661 seconds' where user_id=$1 and id=$2").bind(a.account_id).bind(capped).execute(&mut *tx).await.unwrap();
    tx.commit().await.unwrap();
    let mut claim = store.claim(&a, capped).await.unwrap();
    assert!(
        claim
            .execute(&mut provider, Limits::default(), &Cancellation::default())
            .await
            .is_err()
    );
    assert_eq!(provider.step, 0);
    // The bounded command is still in flight. Park it so these price proofs
    // can admit their own commands; the account admits one at a time.
    let mut tx = db.begin(a.account_id).await.unwrap();
    sqlx::query("update duet_commands set state='cancelled' where user_id=$1 and id=$2")
        .bind(a.account_id)
        .bind(capped)
        .execute(&mut *tx)
        .await
        .unwrap();
    tx.commit().await.unwrap();
    // Price boundary is taken from the reference rate, not a local estimate.
    for (input, expected) in [(272_000u64, "0.0272491"), (272_001, "0.0544733")] {
        let priced = Uuid::new_v4();
        store
            .accept(&a, priced, Uuid::new_v4(), 0, "Pricing proof", "gpt-6-luna")
            .await
            .unwrap();
        let mut price_owner = store.claim(&a, priced).await.unwrap();
        price_owner
            .execute(
                &mut Local {
                    step: 1,
                    fail: false,
                },
                Limits::default(),
                &Cancellation::default(),
            )
            .await
            .unwrap();
        price_owner
            .record(Record::Observation {
                sequence: 1,
                usage: Usage {
                    input_tokens: Some(input),
                    output_tokens: Some(100),
                },
                provenance: json!({"cached_input_tokens":10}),
            })
            .await
            .unwrap();
        let priced_usage = store.usage(&a, priced).await.unwrap();
        assert_eq!(
            priced_usage["estimated_cost_usd"]
                .as_str()
                .unwrap()
                .parse::<f64>()
                .unwrap(),
            expected.parse::<f64>().unwrap()
        );
    }
    // A different account cannot see accounting even with a raw runtime query.
    let mut tx = db.begin(b.account_id).await.unwrap();
    let count: i64 = sqlx::query_scalar("select count(*) from duet_model_calls where user_id=$1")
        .bind(a.account_id)
        .fetch_one(&mut *tx)
        .await
        .unwrap();
    assert_eq!(count, 0);
    tx.rollback().await.unwrap();
    drop(claim);
    drop(next);
    drop(recovered);
    drop(owner);
    drop(restored);
    drop(store);
    drop(identity);
    drop(db);
    sqlx::query(AssertSqlSafe(format!("drop database {name} with (force)")))
        .execute(&mut admin)
        .await
        .unwrap();
    sqlx::query(AssertSqlSafe(format!("drop role {role}")))
        .execute(&mut admin)
        .await
        .unwrap();
}
