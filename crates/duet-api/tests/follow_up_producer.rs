//! Real PostgreSQL proof from empty installation. No model calls and no silent skip.
//! Path inclusion keeps this isolated subtask compilable before lib integration.
pub use duet_api::{ApiError, identity};
#[allow(dead_code)]
#[path = "../src/follow_up.rs"]
mod follow_up;
use chrono::{DateTime, Utc};
use duet_api::{
    conversation::ConversationStore,
    database::{AccountDatabase, migrate},
    identity::AccountContext,
};
use follow_up::*;
use harness::execution::{Call, Cancellation, Owner, Response, Usage};
use serde_json::json;
use sqlx::{ConnectOptions, Connection, Row};
use uuid::Uuid;

fn instant(s: &str) -> DateTime<Utc> {
    s.parse().unwrap()
}
fn once(date: &str) -> Schedule {
    serde_json::from_value(
        json!({"kind":"once","timezone":"UTC","local_date":date,"local_time":"09:00:00"}),
    )
    .unwrap()
}
async fn authority(db: &AccountDatabase, c: &AccountContext, id: Uuid) -> AgentAuthority {
    let mut tx = db.begin(c.account_id).await.unwrap();
    let row = sqlx::query("select lease_id,claim from duet_commands where user_id=$1 and id=$2")
        .bind(c.account_id)
        .bind(id)
        .fetch_one(&mut *tx)
        .await
        .unwrap();
    AgentAuthority::from_owner(c.clone(), id, row.get("lease_id"), row.get("claim"))
}
async fn record_read(db: &AccountDatabase, c: &AccountContext, command: Uuid, id: Uuid) {
    let mut tx = db.begin(c.account_id).await.unwrap();
    let result = read(&mut tx, c.account_id, id).await.unwrap();
    sqlx::query("insert into duet_tool_effects(user_id,command_id,invocation_id,call,result) values($1,$2,$3,$4,$5)").bind(c.account_id).bind(command).bind(Uuid::new_v4().to_string()).bind(json!({"name":"read_follow_up","arguments":{"id":id}})).bind(result).execute(&mut *tx).await.unwrap();
    tx.commit().await.unwrap();
}
#[tokio::test]
async fn fresh_accepted_follow_up_authority_lifecycle_and_calendar() {
    let admin_url = std::env::var("DUET_TEST_DATABASE_URL")
        .expect("DUET_TEST_DATABASE_URL must be disposable local PostgreSQL");
    let options: sqlx::postgres::PgConnectOptions = admin_url.parse().unwrap();
    assert!(matches!(
        options.get_host(),
        "localhost" | "127.0.0.1" | "::1"
    ));
    let mut admin = sqlx::PgConnection::connect(&admin_url).await.unwrap();
    let suffix = Uuid::new_v4().simple().to_string();
    let database_name = format!("follow_test_{suffix}");
    let role = format!("follow_role_{suffix}");
    sqlx::raw_sql(&format!("create role {role} login"))
        .execute(&mut admin)
        .await
        .unwrap();
    sqlx::raw_sql(&format!("create database {database_name}"))
        .execute(&mut admin)
        .await
        .unwrap();
    let opts = options.database(&database_name);
    let owner_url = opts.to_url_lossy().to_string();
    let runtime_url = opts.username(&role).to_url_lossy().to_string();
    let mut admin_db = sqlx::PgConnection::connect(&owner_url).await.unwrap();
    sqlx::raw_sql("revoke create on schema public from public")
        .execute(&mut admin_db)
        .await
        .unwrap();
    migrate(&owner_url, &role).await.unwrap();
    // Coordinator will register 0011 in migrate; tolerate that integration landing.
    let installed: bool =
        sqlx::query_scalar("select to_regclass('public.duet_follow_up_origins') is not null")
            .fetch_one(&mut admin_db)
            .await
            .unwrap();
    if !installed {
        sqlx::raw_sql(include_str!("../migrations/0011_follow_up_producer.sql"))
            .execute(&mut admin_db)
            .await
            .unwrap();
    }
    sqlx::raw_sql(&format!("grant select,insert,update on follow_ups,duet_follow_up_occurrences,duet_follow_up_settings to {role}; grant select,insert on duet_follow_up_origins to {role}; grant select on duet_due_follow_up_accounts to {role}")).execute(&mut admin_db).await.unwrap();
    let db = AccountDatabase::connect(&runtime_url).await.unwrap();
    let mut contexts = Vec::new();
    for _ in 0..2 {
        let id = Uuid::new_v4();
        sqlx::query("insert into users(id,auth_user_id,sharing_label,intelligence_enabled,luna_consent_decision,luna_consent_version,terms_version) values($1,$2,'Synthetic',true,'accepted','2026-09-24','2026-09-23')").bind(id).bind(id.to_string()).execute(&mut admin_db).await.unwrap();
        let (incarnation, permission_revision): (Uuid, i64) = sqlx::query_as(
            "select duet_incarnation,intelligence_control_revision from users where id=$1",
        )
        .bind(id)
        .fetch_one(&mut admin_db)
        .await
        .unwrap();
        contexts.push(AccountContext {
            account_id: id,
            incarnation,
            permission_revision,
            authenticated_at: Utc::now(),
        });
    }
    let c = &contexts[0];
    let foreign = &contexts[1];
    // Real calendar with DST first-fold, forward-gap, absent month day, weekly anchor.
    let mut tx = db.begin(c.account_id).await.unwrap();
    for (schedule, after, expected) in [
        (
            json!({"kind":"once","timezone":"America/New_York","local_date":"2026-11-01","local_time":"01:30:00"}),
            "2026-10-31T00:00:00Z",
            "2026-11-01T05:30:00Z",
        ),
        (
            json!({"kind":"once","timezone":"America/New_York","local_date":"2026-03-08","local_time":"02:30:00"}),
            "2026-03-07T00:00:00Z",
            "2026-03-08T07:30:00Z",
        ),
        (
            json!({"kind":"recurring","timezone":"UTC","start_date":"2026-01-31","local_time":"09:00:00","frequency":"monthly","interval":1,"day_of_month":31}),
            "2026-02-01T00:00:00Z",
            "2026-03-31T09:00:00Z",
        ),
        (
            json!({"kind":"recurring","timezone":"UTC","start_date":"2026-01-01","local_time":"09:00:00","frequency":"weekly","interval":2,"weekdays":["monday"]}),
            "2026-01-06T00:00:00Z",
            "2026-01-19T09:00:00Z",
        ),
    ] {
        assert_eq!(
            next_occurrence(
                &mut tx,
                &serde_json::from_value(schedule).unwrap(),
                instant(after),
                false
            )
            .await
            .unwrap(),
            Some(instant(expected))
        );
    }
    assert!(next_occurrence(&mut tx,&serde_json::from_value(json!({"kind":"once","timezone":"Not/AZone","local_date":"2030-01-01","local_time":"09:00:00"})).unwrap(),Utc::now(),true).await.is_err());
    tx.rollback().await.unwrap();
    let store = ConversationStore::new(db.clone());
    let prior = Uuid::new_v4();
    store
        .phone_accept_submission(
            c,
            prior,
            "Help me prepare questions",
            "controlled",
            &Default::default(),
        )
        .await
        .unwrap();
    let mut prior_owner = store.claim(c, prior).await.unwrap();
    prior_owner
        .complete(&Response {
            text: "Would you like a reminder to bring your questions?".into(),
            calls: vec![],
            usage: Usage::default(),
            provenance: json!({}),
        })
        .await
        .unwrap();
    let command = Uuid::new_v4();
    store
        .phone_accept_submission(
            c,
            command,
            "Remind me to bring my questions on January 1, 2035",
            "controlled",
            &Default::default(),
        )
        .await
        .unwrap();
    let mut owner = store.claim(c, command).await.unwrap();
    let actor = authority(&db, c, command).await;
    let mut definition = Definition {
        id: Uuid::new_v4(),
        operation: Operation::Reminder,
        accepted_instruction: "Bring my questions".into(),
        schedule: once("2035-01-01"),
        source_message_ids: vec![command],
        display_title: Some("Questions".into()),
    };
    // Rejected provenance and stale lease cannot import accepted text.
    let mut tx = db.begin(c.account_id).await.unwrap();
    let bad = AgentAuthority::from_owner(c.clone(), command, Uuid::new_v4(), 1);
    assert!(create(&mut tx, &bad, &definition).await.is_err());
    tx.rollback().await.unwrap();
    let mut tx = db.begin(c.account_id).await.unwrap();
    definition.source_message_ids = vec![Uuid::new_v4()];
    assert!(create(&mut tx, &actor, &definition).await.is_err());
    tx.rollback().await.unwrap();
    definition.source_message_ids = vec![command];
    // Prior source needs delivered exact read, not merely inclusion in transcript.
    definition
        .source_message_ids
        .push(Uuid::from_u128(prior.as_u128() ^ (1u128 << 127)));
    let mut tx = db.begin(c.account_id).await.unwrap();
    assert!(create(&mut tx, &actor, &definition).await.is_err());
    tx.rollback().await.unwrap();
    owner
        .dispatch(
            &Call {
                id: "exact-prior".into(),
                name: "read_message".into(),
                arguments: json!({"command_id":prior}),
            },
            &Cancellation::default(),
        )
        .await
        .unwrap();
    let mut tx = db.begin(c.account_id).await.unwrap();
    let fresh = create(&mut tx, &actor, &definition).await.unwrap();
    assert_eq!(fresh["state"], "active");
    tx.commit().await.unwrap();
    // Duplicate title is excluded from identity; changed accepted work is refused.
    let mut tx = db.begin(c.account_id).await.unwrap();
    definition.display_title = Some("Cosmetic retry".into());
    assert_eq!(
        create(&mut tx, &actor, &definition).await.unwrap()["display_title"],
        "Questions"
    );
    definition.accepted_instruction = "Different words".into();
    assert!(create(&mut tx, &actor, &definition).await.is_err());
    tx.rollback().await.unwrap();
    definition.accepted_instruction = "Bring my questions".into();
    definition.display_title = Some("Questions".into());
    let mut tx = db.begin(foreign.account_id).await.unwrap();
    assert!(
        read(&mut tx, foreign.account_id, definition.id)
            .await
            .is_err()
    );
    tx.rollback().await.unwrap();
    // An arbitrary historical source ID is neither a delivered message nor a grant.
    let mut invalid_source = definition.clone();
    invalid_source.id = Uuid::new_v4();
    invalid_source.source_message_ids.push(Uuid::new_v4());
    let mut tx = db.begin(c.account_id).await.unwrap();
    assert!(create(&mut tx, &actor, &invalid_source).await.is_err());
    tx.rollback().await.unwrap();
    // Exact-read revision is required for edit. Real search output taints future writes.
    let mut tx = db.begin(c.account_id).await.unwrap();
    assert!(edit(&mut tx, &actor, 1, &definition).await.is_err());
    tx.rollback().await.unwrap();
    owner
        .dispatch(
            &Call {
                id: "untrusted-search".into(),
                name: "search_health_sources".into(),
                arguments: json!({"text":"questions"}),
            },
            &Cancellation::default(),
        )
        .await
        .unwrap();
    record_read(&db, c, command, definition.id).await;
    definition.accepted_instruction = "Bring my written questions".into();
    let mut tx = db.begin(c.account_id).await.unwrap();
    let edited = edit(&mut tx, &actor, 1, &definition).await.unwrap();
    assert_eq!(edited["revision"], 2);
    assert_eq!(edited["pause_reason"], "confirmation_required");
    tx.commit().await.unwrap();
    record_read(&db, c, command, definition.id).await;
    let mut tx = db.begin(c.account_id).await.unwrap();
    assert!(
        agent_control(&mut tx, &actor, definition.id, 2, Control::Resume)
            .await
            .is_err()
    );
    assert!(edit(&mut tx, &actor, 1, &definition).await.is_err());
    tx.rollback().await.unwrap();
    // Person can start; stale person writes cannot overwrite a later revision.
    let mut tx = db.begin(c.account_id).await.unwrap();
    assert!(
        person_control(&mut tx, c, definition.id, 1, Control::Resume)
            .await
            .is_err()
    );
    let started = person_control(&mut tx, c, definition.id, 2, Control::Resume)
        .await
        .unwrap();
    assert_eq!(started["state"], "active");
    assert_eq!(started["confirmation_required"], false);
    tx.commit().await.unwrap();
    // Restriction and caller receipt can roll back atomically.
    let mut tx = db.begin(c.account_id).await.unwrap();
    person_control(&mut tx, c, definition.id, 3, Control::Pause)
        .await
        .unwrap();
    tx.rollback().await.unwrap();
    let mut tx = db.begin(c.account_id).await.unwrap();
    assert_eq!(
        read(&mut tx, c.account_id, definition.id).await.unwrap()["state"],
        "active"
    );
    replay_restriction(&mut tx, c.account_id, Uuid::new_v4(), definition.id, true)
        .await
        .unwrap();
    assert_eq!(
        read(&mut tx, c.account_id, definition.id).await.unwrap()["state"],
        "active"
    );
    replay_restriction(&mut tx, c.account_id, c.incarnation, definition.id, true)
        .await
        .unwrap();
    replay_restriction(&mut tx, c.account_id, c.incarnation, definition.id, false)
        .await
        .unwrap();
    assert_eq!(
        read(&mut tx, c.account_id, definition.id).await.unwrap()["state"],
        "cancelled"
    );
    assert!(
        person_control(&mut tx, c, definition.id, 4, Control::Resume)
            .await
            .is_err()
    );
    tx.commit().await.unwrap();
    // A post-untrusted new definition is also held until the person's own start.
    definition.id = Uuid::new_v4();
    let mut tx = db.begin(c.account_id).await.unwrap();
    assert_eq!(
        create(&mut tx, &actor, &definition).await.unwrap()["pause_reason"],
        "confirmation_required"
    );
    tx.commit().await.unwrap();
    // Complete actual accepted command before due reservation. No model invoked.
    owner
        .complete(&Response {
            text: "Your follow-up waits for your confirmation.".into(),
            calls: vec![],
            usage: Usage::default(),
            provenance: json!({}),
        })
        .await
        .unwrap();
    // Move only the test clock boundary, never seed accepted follow-up rows.
    let mut tx = db.begin(c.account_id).await.unwrap();
    person_control(&mut tx, c, definition.id, 1, Control::Resume)
        .await
        .unwrap();
    sqlx::query("update follow_ups set next_occurrence_at=now()-interval '1 minute' where user_id=$1 and id=$2").bind(c.account_id).bind(definition.id).execute(&mut *tx).await.unwrap();
    tx.commit().await.unwrap();
    // Orphan reservations cannot commit: the same transaction must insert command.
    let scheduled = Uuid::new_v4();
    let mut tx = db.begin(c.account_id).await.unwrap();
    assert!(reserve_due(&mut tx, c, scheduled).await.unwrap().is_some());
    assert!(tx.commit().await.is_err());
    // Production admission atomically reserves the occurrence and command;
    // no fixture SQL may impersonate accepted scheduled work.
    assert_eq!(
        store
            .accept_due_follow_up(c, scheduled, "local-test")
            .await
            .unwrap(),
        Some(scheduled)
    );
    let mut tx = db.begin(c.account_id).await.unwrap();
    mark_running(&mut tx, c, scheduled).await.unwrap();
    hold_scheduled(&mut tx, c.account_id, scheduled)
        .await
        .unwrap();
    assert_eq!(
        fence_scheduled(&mut tx, c, scheduled)
            .await
            .unwrap()
            .unwrap()["status"],
        "queued"
    );
    set_enabled(&mut tx, c, 1, false).await.unwrap();
    assert!(fence_scheduled(&mut tx, c, scheduled).await.is_err());
    set_enabled(&mut tx, c, 2, true).await.unwrap();
    assert!(fence_scheduled(&mut tx, c, scheduled).await.is_ok());
    tx.commit().await.unwrap();
    let restored = AccountDatabase::connect(&runtime_url).await.unwrap();
    let mut tx = restored.begin(c.account_id).await.unwrap();
    assert!(
        reserve_due(&mut tx, c, Uuid::new_v4())
            .await
            .unwrap()
            .is_none()
    );
    tx.rollback().await.unwrap();
    // Completion rides the actual recovered command owner, publishing its
    // assistant message and occurrence transition in the same transaction.
    let recovered = ConversationStore::new(restored.clone());
    let mut scheduled_owner = recovered.claim(c, scheduled).await.unwrap();
    scheduled_owner
        .complete(&Response {
            text: "Bring my questions".into(),
            calls: vec![],
            usage: Usage::default(),
            provenance: json!({"mode":"deterministic-local"}),
        })
        .await
        .unwrap();
    let mut tx = restored.begin(c.account_id).await.unwrap();
    assert!(fence_scheduled(&mut tx, c, scheduled).await.is_err());
    tx.rollback().await.unwrap();
    drop(scheduled_owner);
    drop(recovered);
    // Role cannot bypass RLS or modify immutable accepted origins.
    let mut tx = db.begin(c.account_id).await.unwrap();
    assert!(
        sqlx::query("update duet_follow_up_origins set creation_identity='{}'")
            .execute(&mut *tx)
            .await
            .is_err()
    );
    tx.rollback().await.unwrap();
    drop(restored);
    drop(owner);
    drop(store);
    drop(db);
    admin_db.close().await.unwrap();
    sqlx::raw_sql(&format!("drop database {database_name} with (force)"))
        .execute(&mut admin)
        .await
        .unwrap();
    sqlx::raw_sql(&format!("drop role {role}"))
        .execute(&mut admin)
        .await
        .unwrap();
}
