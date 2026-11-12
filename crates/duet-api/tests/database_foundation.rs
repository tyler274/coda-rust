//! Requires an explicitly supplied disposable PostgreSQL admin URL; never skips.
#![allow(
    clippy::disallowed_methods,
    reason = "converting to the typed database API; the proof also provisions databases"
)]
use duet_api::database::{AccountDatabase, migrate};
use sqlx::AssertSqlSafe;
use sqlx::{Connection, PgConnection};
use uuid::Uuid;

#[tokio::test]
async fn fresh_and_upgrade_keep_identity_and_enforce_account_scope() {
    let admin_url = std::env::var("DUET_TEST_DATABASE_URL")
        .expect("DUET_TEST_DATABASE_URL must name a disposable local PostgreSQL instance");
    let admin_options: sqlx::postgres::PgConnectOptions = admin_url.parse().unwrap();
    assert!(
        matches!(admin_options.get_host(), "localhost" | "127.0.0.1" | "::1"),
        "foundation tests only operate on loopback PostgreSQL"
    );
    let mut admin = PgConnection::connect(&admin_url).await.unwrap();
    let suffix = Uuid::new_v4().simple().to_string();
    let role = format!("duet_test_{suffix}");
    // A per-run random password lets the role log in over TCP whether the
    // disposable server trusts loopback (local) or requires passwords (CI).
    let password = Uuid::new_v4().simple().to_string();
    sqlx::raw_sql(AssertSqlSafe(format!(
        "create role {role} login password '{password}'"
    )))
    .execute(&mut admin)
    .await
    .unwrap();
    for upgrade in [false, true] {
        let db = format!(
            "duet_{}_{}",
            if upgrade { "upgrade" } else { "fresh" },
            suffix
        );
        sqlx::raw_sql(AssertSqlSafe(format!("create database {db}")))
            .execute(&mut admin)
            .await
            .unwrap();
        // Explicit localhost test URLs; credentials are never taken from runtime env.
        let options: sqlx::postgres::PgConnectOptions = admin_url.parse().unwrap();
        let options = options.database(&db);
        use sqlx::ConnectOptions;
        let owner_url = options.to_url_lossy().to_string();
        let runtime_url = options
            .clone()
            .username(&role)
            .password(&password)
            .to_url_lossy()
            .to_string();
        let mut owner = PgConnection::connect(&owner_url).await.unwrap();
        sqlx::raw_sql("revoke create on schema public from public")
            .execute(&mut owner)
            .await
            .unwrap();
        let a = Uuid::new_v4();
        let b = Uuid::new_v4();
        if upgrade {
            sqlx::raw_sql("create table public.users (
                id uuid primary key, auth_user_id text not null unique,
                sharing_label text not null, doctor_note text,
                doctor_note_generation bigint not null default 0,
                doctor_page jsonb not null default '{}');
                create table doctor_collection_items (id uuid primary key, user_id uuid, page_entry jsonb);
                create table schema_migrations(version text primary key);
                insert into schema_migrations values ('0118_model_call_accounting');")
                .execute(&mut owner).await.unwrap();
            sqlx::query("insert into users values ($1, 'existing-subject', 'Existing', 'Keep my words', 42, '{\"questions\":[\"Keep?\"]}')")
                .bind(a).execute(&mut owner).await.unwrap();
            for table in ["doctor_collection_items"] {
                sqlx::query(AssertSqlSafe(format!(
                    "insert into {table} values ($1, $1, '{{\"literal\":\"unchanged\"}}')"
                )))
                .bind(a)
                .execute(&mut owner)
                .await
                .unwrap();
            }
        }
        if upgrade {
            sqlx::raw_sql("create table health_evidence (
                id uuid primary key,user_id uuid not null references users(id),
                concept_id text not null,source_kind text not null,source_connection_id text,source_record_id text,
                measured_at timestamptz not null,received_at timestamptz not null,payload jsonb not null,
                created_at timestamptz not null default now(),updated_at timestamptz not null default now(),unique(user_id,id));")
                .execute(&mut owner).await.unwrap();
            sqlx::query("insert into health_evidence(id,user_id,concept_id,source_kind,measured_at,received_at,payload) values($1,$1,'literal','healthRecords',now(),now(),'{\"value\":\"< 0.03\",\"unit\":\"mg/dL\",\"unknown\":{\"preserve\":true}}')")
                .bind(a).execute(&mut owner).await.unwrap();
        }
        migrate(&owner_url, &role).await.unwrap();
        migrate(&owner_url, &role).await.unwrap();
        if upgrade {
            let retained:(Uuid,String,String)=sqlx::query_as("select evidence_id,payload->>'value',payload->>'unit' from canonical_evidence_revisions where user_id=$1")
                .bind(a).fetch_one(&mut owner).await.unwrap();
            assert_eq!(retained, (a, "< 0.03".into(), "mg/dL".into()));
            let original: String = sqlx::query_scalar(
                "select client_record_id from evidence_sync_changes where user_id=$1",
            )
            .bind(a)
            .fetch_one(&mut owner)
            .await
            .unwrap();
            assert_eq!(original, a.to_string());
        }
        if upgrade {
            let retained: (String, i64, String) = sqlx::query_as(
                "select doctor_note, doctor_note_generation, doctor_page->'questions'->>0 from users where id=$1")
                .bind(a).fetch_one(&mut owner).await.unwrap();
            assert_eq!(retained, ("Keep my words".into(), 42, "Keep?".into()));
            for table in ["doctor_collection_items"] {
                let id: Uuid = sqlx::query_scalar(AssertSqlSafe(format!("select id from {table}")))
                    .fetch_one(&mut owner)
                    .await
                    .unwrap();
                assert_eq!(id, a);
            }
            let legacy: String = sqlx::query_scalar("select version from schema_migrations")
                .fetch_one(&mut owner)
                .await
                .unwrap();
            assert_eq!(legacy, "0118_model_call_accounting");
        } else {
            sqlx::query(
                "insert into users(id,auth_user_id,sharing_label) values ($1,'subject-a','A')",
            )
            .bind(a)
            .execute(&mut owner)
            .await
            .unwrap();
        }
        sqlx::query("insert into users(id,auth_user_id,sharing_label) values ($1,'subject-b','B')")
            .bind(b)
            .execute(&mut owner)
            .await
            .unwrap();
        assert!(AccountDatabase::connect(&owner_url).await.is_err());
        let database = AccountDatabase::connect(&runtime_url).await.unwrap();
        let mut tx = database.begin(a).await.unwrap();
        let ids: Vec<Uuid> = sqlx::query_scalar("select id from users")
            .fetch_all(&mut *tx)
            .await
            .unwrap();
        assert_eq!(ids, vec![a]);
        let updated = sqlx::query("update users set sharing_label='wrong' where id=$1")
            .bind(b)
            .execute(&mut *tx)
            .await
            .unwrap();
        assert_eq!(updated.rows_affected(), 0);
        tx.rollback().await.unwrap();
        let mut runtime = PgConnection::connect(&runtime_url).await.unwrap();
        let count: i64 = sqlx::query_scalar("select count(*) from users")
            .fetch_one(&mut runtime)
            .await
            .unwrap();
        assert_eq!(count, 0);
        assert!(
            sqlx::raw_sql("truncate users")
                .execute(&mut runtime)
                .await
                .is_err()
        );
        assert!(
            sqlx::raw_sql("create table public.forbidden(id int)")
                .execute(&mut runtime)
                .await
                .is_err()
        );
        let mut tx = database.begin(b).await.unwrap();
        let ids: Vec<Uuid> = sqlx::query_scalar("select id from users")
            .fetch_all(&mut *tx)
            .await
            .unwrap();
        assert_eq!(ids, vec![b]);
        tx.commit().await.unwrap();
        let mut tx = database.begin(a).await.unwrap();
        let ids: Vec<Uuid> = sqlx::query_scalar("select id from users")
            .fetch_all(&mut *tx)
            .await
            .unwrap();
        assert_eq!(ids, vec![a]);
        tx.rollback().await.unwrap();
        let mut tx = database.begin(b).await.unwrap();
        assert!(
            sqlx::query(
                "insert into users(id,auth_user_id,sharing_label) values ($1,'foreign','Foreign')"
            )
            .bind(Uuid::new_v4())
            .execute(&mut *tx)
            .await
            .is_err()
        );
        tx.rollback().await.unwrap();
        // Immutable ledger detects changed migrations rather than adopting drift.
        sqlx::query("update duet_schema_migrations set checksum='tampered'")
            .execute(&mut owner)
            .await
            .unwrap();
        assert!(migrate(&owner_url, &role).await.is_err());
        drop(database);
        runtime.close().await.unwrap();
        owner.close().await.unwrap();
        sqlx::raw_sql(AssertSqlSafe(format!("drop database {db} with (force)")))
            .execute(&mut admin)
            .await
            .unwrap();
    }
    sqlx::raw_sql(AssertSqlSafe(format!("drop role {role}")))
        .execute(&mut admin)
        .await
        .unwrap();
}
