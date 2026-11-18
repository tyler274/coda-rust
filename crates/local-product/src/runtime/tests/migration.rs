//! The runtime chain's ledger decides whether its schema runs.
use super::*;

/// Creates the per-test database and returns its URL. The database is left in
/// place after the test; the next run's `DROP DATABASE IF EXISTS` clears it.
pub(super) async fn database(name: &str) -> String {
    let url = std::env::var("PHYSICIAN_RUNTIME_TEST_DATABASE_URL").unwrap();
    let (base, db) = url.trim().rsplit_once('/').expect("database URL");
    let test_db = format!("{db}_t_{name}");
    // CREATE and DROP DATABASE take no bind parameters, so the name goes in
    // as an identifier the query builder quoted.
    let quoted = {
        use sea_orm::sea_query::{PostgresQueryBuilder, QuotedBuilder};
        let mut quoted = String::new();
        PostgresQueryBuilder.prepare_iden(&test_db.clone().into(), &mut quoted);
        quoted
    };
    let admin = Database::connect(format!("{base}/postgres"))
        .await
        .expect("admin connection");
    admin
        .execute_unprepared(&format!("DROP DATABASE IF EXISTS {quoted}"))
        .await
        .expect("drop stale test database");
    admin
        .execute_unprepared(&format!("CREATE DATABASE {quoted}"))
        .await
        .expect("create test database");
    admin.close().await.expect("close admin connection");
    format!("{base}/{test_db}")
}

async fn names(db: &DatabaseConnection, query: &str) -> Vec<String> {
    db.query_all_raw(sea_orm::Statement::from_string(
        sea_orm::DbBackend::Postgres,
        query,
    ))
    .await
    .expect("catalog read")
    .into_iter()
    .map(|row| row.try_get("", "name").unwrap())
    .collect()
}

#[tokio::test]
#[ignore = "requires explicit local Postgres"]
async fn a_second_migrate_records_nothing_new() {
    let store = Store::connect(&database("runtime_rerun").await)
        .await
        .unwrap();
    store.migrate().await.unwrap();
    store.migrate().await.unwrap();
    assert_eq!(
        names(
            &store.db,
            "SELECT version AS name FROM physician_runtime_migrations ORDER BY version"
        )
        .await,
        [
            "m20261111_init",
            "m20261111_response_feedback",
            "m20261112_source_pull_missing_candidates",
            "m20261113_source_links_and_standing",
            "m20261114_reconnect_commands_and_adapter_state",
            "m20261115_patient_memory",
            "m20261118_01_concerns",
            "m20261118_02_practice_jobs_dispatch",
            "m20261118_03_source_refresh",
            "m20261118_04_practice_schedule",
            "m20261118_05_patient_understanding",
            "m20261118_06_patient_reviews",
            "m20261118_07_visit_briefings",
            "m20261118_08_today_updates_seen",
            "m20261118_09_conversation_activity",
        ]
    );
}

#[tokio::test]
#[ignore = "requires explicit local Postgres"]
async fn leftover_runtime_tables_fail_the_migration_and_record_nothing() {
    let store = Store::connect(&database("runtime_leftover").await)
        .await
        .unwrap();
    // A runtime table in an earlier shape, with no ledger beside it.
    store
        .db
        .execute_unprepared("CREATE TABLE physician_runtime_conversations (practice text)")
        .await
        .unwrap();
    let error = store.migrate().await.unwrap_err().to_string();
    assert!(
        str::contains(
            &error,
            r#"relation "physician_runtime_conversations" already exists"#
        ),
        "{error}"
    );
    // The failed migration leaves no ledger and no other table behind.
    assert_eq!(
        names(
            &store.db,
            "SELECT tablename::text AS name FROM pg_tables WHERE schemaname = 'public'"
        )
        .await,
        ["physician_runtime_conversations"]
    );
}

#[tokio::test]
#[ignore = "requires explicit local Postgres"]
async fn source_pull_missing_candidates_migration_refuses_down_rollback() {
    use sea_orm_migration::{MigrationTrait, SchemaManager};

    let store = Store::connect(&database("runtime_down_refused").await)
        .await
        .unwrap();
    store.migrate().await.unwrap();

    let manager = SchemaManager::new(&store.db);
    let error = migration::m20261112_source_pull_missing_candidates::Migration
        .down(&manager)
        .await
        .expect_err("forward-only runtime migration must refuse rollback")
        .to_string();
    assert!(
        str::contains(
            &error,
            "the physician runtime's records can't be rolled back"
        ),
        "{error}"
    );
    assert!(
        names(
            &store.db,
            "SELECT column_name::text AS name FROM information_schema.columns WHERE table_name = 'physician_runtime_source_pulls' ORDER BY column_name"
        )
        .await
        .iter()
        .any(|n| n == "missing_candidates")
    );

    let links_error = migration::m20261113_source_links_and_standing::Migration
        .down(&manager)
        .await
        .expect_err("forward-only runtime migration must refuse rollback")
        .to_string();
    assert!(
        str::contains(
            &links_error,
            "the physician runtime's records can't be rolled back"
        ),
        "{links_error}"
    );
    assert!(
        names(
            &store.db,
            "SELECT tablename::text AS name FROM pg_tables WHERE schemaname = 'public' ORDER BY name"
        )
        .await
        .iter()
        .any(|n| n == "physician_runtime_patient_source_status")
    );

    let reconnect_error = migration::m20261114_reconnect_commands_and_adapter_state::Migration
        .down(&manager)
        .await
        .expect_err("forward-only runtime migration must refuse rollback")
        .to_string();
    assert!(
        str::contains(
            &reconnect_error,
            "the physician runtime's records can't be rolled back"
        ),
        "{reconnect_error}"
    );
    let tables = names(
        &store.db,
        "SELECT tablename::text AS name FROM pg_tables WHERE schemaname = 'public' ORDER BY name",
    )
    .await;
    assert!(
        tables
            .iter()
            .any(|n| n == "physician_runtime_reconnect_commands")
    );
    assert!(
        tables
            .iter()
            .any(|n| n == "physician_runtime_source_adapter_state")
    );
}

#[tokio::test]
#[ignore = "requires explicit local Postgres"]
async fn conversation_activity_migration_backfills_later_of_open_and_last_activity() {
    use sea_orm::sea_query::PostgresQueryBuilder;
    use sea_orm_migration::MigratorTrait;

    let store = Store::connect(&database("conv_activity_backfill").await)
        .await
        .unwrap();
    // Apply every migration before `m20261118_09_conversation_activity` so
    // pre-existing rows have no `last_activity_at` column yet.
    let steps_before = migration::Migrator::migrations()
        .iter()
        .position(|m| m.name() == "m20261118_09_conversation_activity")
        .expect("conversation_activity migration") as u32;
    migration::Migrator::up(&store.db, Some(steps_before))
        .await
        .unwrap();

    let skills: SkillPackageId =
        serde_json::from_value(serde_json::json!("fixture-package")).unwrap();
    let mut template = Conversation::new(skills);
    template.work = "w1".to_owned();
    template.messages.push(Message {
        role: MessageRole::User {
            id: "m1".to_owned(),
            work: "w1".to_owned(),
            revision: 1,
        },
        text: "Question before migration".to_owned(),
        saved_at: SavedAt(1_750_000_000),
    });
    let template_json = control::conversation_state(&template).unwrap();
    let base_state = |extra: serde_json::Value| {
        let mut obj = template_json.clone();
        if let Some(target) = obj.as_object_mut() {
            target.remove("opened_at");
            target.remove("last_activity_at");
            if let Some(patch) = extra.as_object() {
                for (k, v) in patch {
                    target.insert(k.clone(), v.clone());
                }
            }
        }
        obj
    };

    let seeded = [
        (
            "chat-activity-later",
            base_state(serde_json::json!({
                "opened_at": 1_700_000_100_i64,
                "last_activity_at": 1_700_000_350_i64,
            })),
            1_700_000_350_i64,
        ),
        (
            "chat-open-later",
            base_state(serde_json::json!({
                "opened_at": 1_700_000_500_i64,
                "last_activity_at": 1_700_000_200_i64,
            })),
            1_700_000_500_i64,
        ),
        (
            "chat-open-only",
            base_state(serde_json::json!({
                "opened_at": 1_700_000_250_i64,
            })),
            1_700_000_250_i64,
        ),
        (
            "chat-missing-both",
            base_state(serde_json::json!({})),
            0_i64,
        ),
    ];

    for (chat_id, state_json, _) in &seeded {
        let mut insert = Query::insert();
        insert
            .into_table(Alias::new("physician_runtime_conversations"))
            .columns([
                Alias::new("practice"),
                Alias::new("physician"),
                Alias::new("workspace"),
                Alias::new("chat"),
                Alias::new("status"),
                Alias::new("revision"),
                Alias::new("state"),
            ])
            .values_panic([
                Expr::val("practice-1"),
                Expr::val("phys-1"),
                Expr::val("ws-1"),
                Expr::val(*chat_id),
                Expr::val("completed"),
                Expr::val(1_i64),
                Expr::val(state_json.to_string()).cast_as(Alias::new("jsonb")),
            ]);
        let (sql, values) = insert.build(PostgresQueryBuilder);
        store
            .db
            .execute_raw(sea_orm::Statement::from_sql_and_values(
                sea_orm::DbBackend::Postgres,
                sql,
                values,
            ))
            .await
            .unwrap();
    }

    store.migrate().await.unwrap();
    store.migrate().await.unwrap();

    for (chat_id, _, expected_activity) in &seeded {
        let row = conversation::Entity::find_by_id((
            "practice-1".to_owned(),
            "phys-1".to_owned(),
            "ws-1".to_owned(),
            (*chat_id).to_owned(),
        ))
        .one(&store.db)
        .await
        .unwrap()
        .expect("migrated row");
        assert_eq!(
            row.last_activity_at, *expected_activity,
            "backfilled column mismatch for {chat_id}"
        );
    }

    let listed: Vec<_> = store
        .chats("practice-1", "phys-1", "ws-1")
        .await
        .unwrap()
        .into_iter()
        .map(|item| (item.chat, item.last_activity_at))
        .collect();
    assert_eq!(
        listed,
        vec![
            ("chat-open-later".to_owned(), 1_700_000_500),
            ("chat-activity-later".to_owned(), 1_700_000_350),
            ("chat-open-only".to_owned(), 1_700_000_250),
            ("chat-missing-both".to_owned(), 0),
        ]
    );
}

#[tokio::test]
#[ignore = "requires explicit local Postgres"]
async fn sending_message_and_publishing_reply_both_advance_activity_column_and_order() {
    let store = Store::connect(&database("conv_activity_moves").await)
        .await
        .unwrap();
    store.migrate().await.unwrap();

    let practice = "practice-activity";
    let scope_for = |chat: &str| Scope {
        practice: practice.to_owned(),
        physician: "phys-1".to_owned(),
        workspace: "ws-1".to_owned(),
        chat: chat.to_owned(),
    };
    let msg = |key: &str, text: &str| Input {
        key: key.to_owned(),
        text: text.to_owned(),
        timezone: "UTC".to_owned(),
        scenario: None,
    };
    let next_clock_tick = |after: i64| {
        let store = store.clone();
        async move {
            let deadline = std::time::Instant::now() + std::time::Duration::from_secs(5);
            loop {
                let tick = now(&store.db).await.unwrap();
                if tick > after {
                    return tick;
                }
                assert!(
                    std::time::Instant::now() < deadline,
                    "database clock stalled at {tick}"
                );
                tokio::time::sleep(std::time::Duration::from_millis(80)).await;
            }
        }
    };
    let column_activity = |chat: &str| {
        let store = store.clone();
        let chat = chat.to_owned();
        async move {
            conversation::Entity::find_by_id((
                practice.to_owned(),
                "phys-1".to_owned(),
                "ws-1".to_owned(),
                chat,
            ))
            .one(&store.db)
            .await
            .unwrap()
            .expect("saved conversation")
            .last_activity_at
        }
    };

    let alpha = scope_for("chat-alpha");
    let beta = scope_for("chat-beta");

    assert_eq!(
        accept(&store, &alpha, &msg("k1", "Check potassium"), 1)
            .await
            .unwrap(),
        Accepted::New
    );
    let alpha_open_col = column_activity("chat-alpha").await;
    assert!(alpha_open_col > 0);
    next_clock_tick(alpha_open_col).await;

    assert_eq!(
        accept(&store, &beta, &msg("k2", "Check creatinine"), 1)
            .await
            .unwrap(),
        Accepted::New
    );
    let beta_open_col = column_activity("chat-beta").await;
    assert!(beta_open_col > alpha_open_col);

    let initial_order: Vec<_> = store
        .chats(practice, "phys-1", "ws-1")
        .await
        .unwrap()
        .into_iter()
        .map(|c| c.chat)
        .collect();
    assert_eq!(initial_order, ["chat-beta", "chat-alpha"]);

    // 1. Sending a follow-up message on chat-alpha moves its column ahead of chat-beta.
    next_clock_tick(beta_open_col).await;
    assert_eq!(
        accept(&store, &alpha, &msg("k3", "Include prior lab"), 1)
            .await
            .unwrap(),
        Accepted::New
    );
    let alpha_after_send = column_activity("chat-alpha").await;
    assert!(alpha_after_send > beta_open_col);
    let after_send_order: Vec<_> = store
        .chats(practice, "phys-1", "ws-1")
        .await
        .unwrap()
        .into_iter()
        .map(|c| c.chat)
        .collect();
    assert_eq!(after_send_order, ["chat-alpha", "chat-beta"]);

    // Move chat-beta ahead, then publish a reply on chat-alpha to prove reply
    // publication also advances the column and reorders the workspace chat list.
    next_clock_tick(alpha_after_send).await;
    assert_eq!(
        accept(&store, &beta, &msg("k4", "Compare baseline"), 1)
            .await
            .unwrap(),
        Accepted::New
    );
    let beta_after_send = column_activity("chat-beta").await;
    assert!(beta_after_send > alpha_after_send);
    assert_eq!(
        store
            .chats(practice, "phys-1", "ws-1")
            .await
            .unwrap()
            .into_iter()
            .map(|c| c.chat)
            .collect::<Vec<_>>(),
        ["chat-beta", "chat-alpha"]
    );

    // 2. Claim and publish a reply on chat-alpha in a newer second.
    let claim = store.claim(&alpha, 60).await.unwrap().expect("claim alpha");
    let reply_turn = engine::Turn {
        items: vec![engine::Item::Call(engine::Invocation {
            id: "finish-alpha".into(),
            name: crate::physician_sources::citations::FINISH_TOOL.into(),
            arguments: serde_json::json!({
                "answer": "Potassium is 4.2 mmol/L.",
                "basis": "Physician-provided value.",
                "descriptions": [], "quotes": [],
                "cites_none": true,
            })
            .to_string(),
        })],
        receipt: serde_json::json!({"controlled": true}),
        usage: Default::default(),
    };
    for (sequence, kind) in [
        (1, engine::EventKind::ProviderReturned(reply_turn.clone())),
        (
            2,
            engine::EventKind::Candidate {
                text: reply_turn.text(),
            },
        ),
    ] {
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
    next_clock_tick(beta_after_send).await;
    assert_eq!(
        store
            .commit_authorized(&claim, &reply_turn, &mut OpenAuthority)
            .await
            .unwrap(),
        engine::recipe::Finish::Committed
    );

    let alpha_after_reply = column_activity("chat-alpha").await;
    assert!(
        alpha_after_reply > beta_after_send,
        "publishing a reply must advance last_activity_at ({alpha_after_reply} vs {beta_after_send})"
    );
    let state_alpha = store.conversation(&alpha).await.unwrap();
    assert_eq!(alpha_after_reply, state_alpha.latest_activity_at());

    let final_order: Vec<_> = store
        .chats(practice, "phys-1", "ws-1")
        .await
        .unwrap()
        .into_iter()
        .map(|c| (c.chat, c.last_activity_at))
        .collect();
    assert_eq!(
        final_order,
        vec![
            ("chat-alpha".to_owned(), alpha_after_reply),
            ("chat-beta".to_owned(), beta_after_send),
        ]
    );
}

#[tokio::test]
#[ignore = "requires explicit local Postgres"]
async fn physician_newest_twenty_five_conversations_plan_uses_activity_index() {
    use sea_orm::QueryTrait;

    let store = Store::connect(&database("conv_activity_plan").await)
        .await
        .unwrap();
    store.migrate().await.unwrap();

    let skills: SkillPackageId =
        serde_json::from_value(serde_json::json!("fixture-package")).unwrap();
    let mut batch = Vec::with_capacity(600);
    for idx in 0..600_i64 {
        let (practice, physician) = if idx % 3 == 0 {
            ("practice-target", "phys-target")
        } else if idx % 3 == 1 {
            ("practice-target", "phys-colleague")
        } else {
            ("practice-other", "phys-other")
        };
        let workspace = format!("ws-{:02}", idx % 40);
        let chat = format!("chat-{idx:04}");
        let activity = 1_750_000_000 + idx * 10;
        let mut conv = Conversation::new(skills.clone());
        conv.opened_at = activity - 5;
        conv.record_activity(activity);
        conv.messages.push(Message {
            role: MessageRole::User {
                id: format!("msg-{idx}"),
                work: format!("work-{idx}"),
                revision: 1,
            },
            text: format!("Clinical question {idx}"),
            saved_at: SavedAt(activity),
        });
        let scope = Scope {
            practice: practice.to_owned(),
            physician: physician.to_owned(),
            workspace,
            chat,
        };
        batch.push(conversation_model(&scope, &conv).unwrap());
    }
    conversation::Entity::insert_many(batch)
        .exec(&store.db)
        .await
        .unwrap();

    // Refresh PostgreSQL planner statistics on the newly populated table;
    // SeaQuery has no ANALYZE builder.
    store
        .db
        .execute_unprepared("ANALYZE physician_runtime_conversations")
        .await
        .unwrap();

    let newest_25 = store
        .physician_conversations("practice-target", "phys-target", 25)
        .await
        .unwrap();
    assert_eq!(newest_25.len(), 25);
    assert!(
        newest_25
            .windows(2)
            .all(|pair| (pair[0].last_activity_at, &pair[0].chat)
                >= (pair[1].last_activity_at, &pair[1].chat))
    );

    let select = order_by_latest_activity(
        conversation::Entity::find()
            .filter(conversation::Column::Practice.eq("practice-target"))
            .filter(conversation::Column::Physician.eq("phys-target")),
    )
    .limit(25);
    let built = select.build(sea_orm::DbBackend::Postgres);
    // Inspect PostgreSQL's execution plan for the exact SeaORM query; EXPLAIN has no SeaQuery builder.
    let explain = sea_orm::Statement::from_sql_and_values(
        sea_orm::DbBackend::Postgres,
        format!("EXPLAIN (COSTS OFF) {}", built.sql),
        built.values.unwrap_or(sea_orm::Values(Vec::new())),
    );
    let plan_lines: Vec<String> = store
        .db
        .query_all_raw(explain)
        .await
        .unwrap()
        .into_iter()
        .map(|row| row.try_get("", "QUERY PLAN").unwrap())
        .collect();
    let plan = plan_lines.join("\n");
    println!("EXPLAIN newest 25 across patients:\n{plan}");
    assert!(
        str::contains(
            &plan,
            "idx-physician_runtime_conversations-physician_activity"
        ),
        "expected physician activity index scan, got:\n{plan}"
    );
    assert!(
        !str::contains(&plan, "Seq Scan"),
        "expected index plan rather than sequential scan, got:\n{plan}"
    );
    assert!(
        !str::contains(&plan, "Sort"),
        "expected index-ordered scan without sort, got:\n{plan}"
    );
}
