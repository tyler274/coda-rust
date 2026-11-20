//! Public retrieval budget through real PostgreSQL.
#![allow(
    clippy::disallowed_methods,
    reason = "converting to the typed database API; the proof also provisions databases"
)]
use duet_api::database::AccountDatabase;
use sqlx::AssertSqlSafe;
use sqlx::{ConnectOptions, Connection};
use uuid::Uuid;

#[tokio::test]
async fn public_retrieval_budget_spans_pools() {
    let url = std::env::var("DUET_TEST_DATABASE_URL").expect("isolated PostgreSQL required");
    let opts: sqlx::postgres::PgConnectOptions = url.parse().unwrap();
    assert!(["localhost", "127.0.0.1", "::1"].contains(&opts.get_host()));
    let mut admin = sqlx::PgConnection::connect(&url).await.unwrap();
    let suffix = Uuid::new_v4().simple().to_string();
    let name = format!("duet_budget_{suffix}");
    let role = format!("duet_budget_{suffix}");
    sqlx::query(AssertSqlSafe(format!("create role {role} login")))
        .execute(&mut admin)
        .await
        .unwrap();
    sqlx::query(AssertSqlSafe(format!("create database {name}")))
        .execute(&mut admin)
        .await
        .unwrap();
    let owner_url = opts.clone().database(&name).to_url_lossy().to_string();
    let runtime_url = opts
        .database(&name)
        .username(&role)
        .to_url_lossy()
        .to_string();
    duet_api::database::migrate(&owner_url, &role)
        .await
        .unwrap();
    let db = AccountDatabase::connect(&runtime_url).await.unwrap();
    let mut admin_db = sqlx::PgConnection::connect(&owner_url).await.unwrap();
    // Shared rate budget is persisted across independent pools.
    let db2 = AccountDatabase::connect(&runtime_url).await.unwrap();
    assert!(db.admit_public_retrieval().await.unwrap());
    sqlx::query("update duet_public_budget set used=600,window_start=now()")
        .execute(&mut admin_db)
        .await
        .unwrap();
    assert!(!db2.admit_public_retrieval().await.unwrap());
    sqlx::query("update duet_public_budget set used=0,window_start=now() where id=2")
        .execute(&mut admin_db)
        .await
        .unwrap();
    assert!(db2.admit_public_retrieval_miss().await.unwrap());
    assert!(!db2.admit_public_retrieval().await.unwrap());
    sqlx::query("update duet_public_budget set used=0,window_start=now() where id=1")
        .execute(&mut admin_db)
        .await
        .unwrap();
    sqlx::query("update duet_public_budget set used=600,window_start=now() where id=2")
        .execute(&mut admin_db)
        .await
        .unwrap();
    assert!(db2.admit_public_retrieval().await.unwrap());
    assert!(!db2.admit_public_retrieval_miss().await.unwrap());
    sqlx::query("update duet_public_budget set used=0,window_start=now()")
        .execute(&mut admin_db)
        .await
        .unwrap();
    assert!(db2.admit_public_retrieval().await.unwrap());
    assert!(db2.admit_public_retrieval_miss().await.unwrap());
    let (hit, miss): (i32, i32) = sqlx::query_as(
        "select (select used from duet_public_budget where id=1),(select used from duet_public_budget where id=2)",
    )
    .fetch_one(&mut admin_db)
    .await
    .unwrap();
    assert_eq!((hit, miss), (1, 1));

    drop(db);
    drop(db2);
    drop(admin_db);
    sqlx::query(AssertSqlSafe(format!("drop database {name} with (force)")))
        .execute(&mut admin)
        .await
        .unwrap();
    sqlx::query(AssertSqlSafe(format!("drop role {role}")))
        .execute(&mut admin)
        .await
        .unwrap();
}
