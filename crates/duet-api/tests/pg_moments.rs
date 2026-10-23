//! Bounded PgMoments proof against a disposable loopback PostgreSQL database.
//! The fixture represents the externally owned relation; no migration is run or
//! rewritten. Other Duet SQLx transactions are deliberately outside this slice.

use duet_api::{ApiError, Moments, PgMoments};
use sqlx::{AssertSqlSafe, Connection, PgConnection, postgres::PgPoolOptions};
use uuid::Uuid;

#[tokio::test]
async fn moments_preserve_account_scope_order_kind_and_delete_semantics() {
    let url = std::env::var("DUET_TEST_DATABASE_URL")
        .expect("DUET_TEST_DATABASE_URL must name disposable loopback PostgreSQL; never skips");
    let options: sqlx::postgres::PgConnectOptions = url.parse().unwrap();
    assert!(matches!(
        options.get_host(),
        "localhost" | "127.0.0.1" | "::1"
    ));
    let mut admin = PgConnection::connect_with(&options).await.unwrap();
    // Database identifiers are code-owned UUIDs; SQL DDL is only fixture setup,
    // not production CRUD or a new schema/migration owner.
    let name = format!("duet_moments_{}", Uuid::new_v4().simple());
    sqlx::raw_sql(AssertSqlSafe(format!("create database {name}")))
        .execute(&mut admin)
        .await
        .unwrap();
    let options = options.database(&name);
    let pool = PgPoolOptions::new()
        .max_connections(2)
        .connect_with(options.clone())
        .await
        .unwrap();
    sqlx::raw_sql(
        "create table public.account_moments (
        id uuid primary key, user_id uuid not null, kind text not null,
        occurred_at timestamptz not null);
        create table public.schema_migrations(version text primary key);
        insert into public.schema_migrations values ('0120_one_review_bookmark');",
    )
    .execute(&pool)
    .await
    .unwrap();
    let owner = Uuid::new_v4();
    let other = Uuid::new_v4();
    let ids = [
        Uuid::from_u128(1),
        Uuid::from_u128(2),
        Uuid::from_u128(3),
        Uuid::from_u128(4),
    ];
    let timestamp = chrono::DateTime::parse_from_rfc3339("2026-10-20T12:00:00+05:30").unwrap();
    // ID 3 is older, proving timestamp ordering precedes descending ID ties.
    for (id, account, time) in [
        (ids[0], owner, timestamp),
        (ids[1], owner, timestamp),
        (ids[2], owner, timestamp - chrono::Duration::seconds(1)),
        (ids[3], other, timestamp + chrono::Duration::seconds(1)),
    ] {
        sqlx::query("insert into public.account_moments values ($1,$2,$3,$4)")
            .bind(id)
            .bind(account)
            .bind("first_summary_prepared")
            .bind(time)
            .execute(&pool)
            .await
            .unwrap();
    }
    let moments = PgMoments::new(pool.clone());
    let listed = moments.list(owner).await.unwrap();
    assert_eq!(
        listed.iter().map(|row| row.id).collect::<Vec<_>>(),
        vec![ids[1], ids[0], ids[2]]
    );
    assert_eq!(listed[0].occurred_at, timestamp);
    assert_eq!(
        listed[0].kind,
        duet_wire::models::MomentKind::FirstSummaryPrepared
    );
    assert_eq!(moments.list(other).await.unwrap()[0].id, ids[3]);
    assert!(moments.list(Uuid::new_v4()).await.unwrap().is_empty());

    // Environment policy is tested in a child process: no racy process-global
    // environment mutation in a multi-threaded test runtime.
    use sqlx::ConnectOptions;
    for bucket in ["configured-test-journal", "  configured-test-journal  "] {
        let status = std::process::Command::new(std::env::current_exe().unwrap())
            .args(["--exact", "privacy_gate_worker", "--ignored"])
            .env("DUET_MOMENTS_WORKER_URL", options.to_url_lossy().as_str())
            .env("DUET_MOMENTS_WORKER_OWNER", owner.to_string())
            .env("DUET_MOMENTS_WORKER_ID", ids[0].to_string())
            .env("DUET_PRIVACY_JOURNAL_BUCKET", bucket)
            .status()
            .unwrap();
        assert!(status.success());
        assert_eq!(moments.list(owner).await.unwrap().len(), 3);
    }
    // Run deletion proof in a child with explicit blank/absent policy, independent
    // of a developer's configured journal environment.
    for bucket in [None, Some("   ")] {
        let mut command = std::process::Command::new(std::env::current_exe().unwrap());
        command
            .args(["--exact", "delete_scope_worker", "--ignored"])
            .env("DUET_MOMENTS_WORKER_URL", options.to_url_lossy().as_str())
            .env("DUET_MOMENTS_WORKER_OWNER", owner.to_string())
            .env("DUET_MOMENTS_WORKER_OTHER", other.to_string())
            .env("DUET_MOMENTS_WORKER_ID", ids[3].to_string())
            .env_remove("DUET_PRIVACY_JOURNAL_BUCKET");
        if let Some(bucket) = bucket {
            command.env("DUET_PRIVACY_JOURNAL_BUCKET", bucket);
        }
        assert!(command.status().unwrap().success());
        // Restore the other account's row for the second policy case.
        sqlx::query("insert into public.account_moments values ($1,$2,$3,$4)")
            .bind(ids[3])
            .bind(other)
            .bind("first_summary_prepared")
            .bind(timestamp)
            .execute(&pool)
            .await
            .unwrap();
    }
    sqlx::query("update public.account_moments set kind=$1 where id=$2")
        .bind("future_unknown_kind")
        .bind(ids[3])
        .execute(&pool)
        .await
        .unwrap();
    // An unknown kind in another account cannot poison this account's list.
    assert_eq!(moments.list(owner).await.unwrap().len(), 3);
    assert!(
        matches!(moments.list(other).await, Err(ApiError::Storage(message))
        if message == "account moment kind is not a known value")
    );
    let versions: Vec<String> = sqlx::query_scalar("select version from public.schema_migrations")
        .fetch_all(&pool)
        .await
        .unwrap();
    assert_eq!(versions, vec!["0120_one_review_bookmark"]);
    drop(moments);
    pool.close().await;
    sqlx::raw_sql(AssertSqlSafe(format!("drop database {name}")))
        .execute(&mut admin)
        .await
        .unwrap();
}

async fn worker() -> (PgMoments, Uuid, Uuid) {
    let pool = PgPoolOptions::new()
        .max_connections(1)
        .connect(&std::env::var("DUET_MOMENTS_WORKER_URL").unwrap())
        .await
        .unwrap();
    (
        PgMoments::new(pool),
        std::env::var("DUET_MOMENTS_WORKER_OWNER")
            .unwrap()
            .parse()
            .unwrap(),
        std::env::var("DUET_MOMENTS_WORKER_ID")
            .unwrap()
            .parse()
            .unwrap(),
    )
}

#[tokio::test]
#[ignore = "invoked by disposable PostgreSQL parent with isolated policy environment"]
async fn privacy_gate_worker() {
    let (moments, owner, id) = worker().await;
    assert!(matches!(
        moments.remove(owner, id).await,
        Err(ApiError::Unavailable(
            "Privacy journal confirmation is not served by this process."
        ))
    ));
    assert!(
        moments
            .list(owner)
            .await
            .unwrap()
            .iter()
            .any(|row| row.id == id)
    );
}

#[tokio::test]
#[ignore = "invoked by disposable PostgreSQL parent with isolated policy environment"]
async fn delete_scope_worker() {
    let (moments, owner, id) = worker().await;
    let other = std::env::var("DUET_MOMENTS_WORKER_OTHER")
        .unwrap()
        .parse()
        .unwrap();
    assert!(
        !moments.remove(owner, id).await.unwrap(),
        "cross-account deletion must affect zero rows"
    );
    assert_eq!(moments.list(other).await.unwrap()[0].id, id);
    assert!(!moments.remove(owner, Uuid::new_v4()).await.unwrap());
    assert!(moments.remove(other, id).await.unwrap());
    assert!(
        !moments.remove(other, id).await.unwrap(),
        "repeat delete must affect zero rows"
    );
    assert!(moments.list(other).await.unwrap().is_empty());
}
