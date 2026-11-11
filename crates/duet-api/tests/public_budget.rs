//! Public retrieval budget and workspace publication through real PostgreSQL.
use duet_api::{database::AccountDatabase, identity::AccountContext};
use sqlx::AssertSqlSafe;
use sqlx::{ConnectOptions, Connection};
use uuid::Uuid;

#[tokio::test]
async fn public_retrieval_budget_spans_pools_and_publication() {
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
    let account = Uuid::new_v4();
    sqlx::query("insert into users(id,auth_user_id,sharing_label,terms_version) values($1,'budget-person','Duet',$2)").bind(account).bind("2026-09-23").execute(&mut admin_db).await.unwrap();
    let (incarnation, revision): (Uuid, i64) = sqlx::query_as(
        "select duet_incarnation,intelligence_control_revision from users where id=$1",
    )
    .bind(account)
    .fetch_one(&mut admin_db)
    .await
    .unwrap();
    let c = AccountContext {
        account_id: account,
        incarnation,
        permission_revision: revision,
        authenticated_at: chrono::Utc::now(),
    };
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
    let publications = duet_api::publication::PublicationStore::new(
        db.clone(),
        "https://duet.example".into(),
        "https://viewer.example".into(),
        [7; 32],
    )
    .unwrap();
    assert!(matches!(
        publications.retrieve("bad", false).await,
        Err(duet_api::ApiError::NotFound)
    ));
    assert!(matches!(
        publications.retrieve(&"a".repeat(43), false).await,
        Err(duet_api::ApiError::NotFound)
    ));
    let (hit, miss): (i32, i32) = sqlx::query_as(
        "select (select used from duet_public_budget where id=1),(select used from duet_public_budget where id=2)",
    )
    .fetch_one(&mut admin_db)
    .await
    .unwrap();
    assert_eq!((hit, miss), (0, 2));
    let workspace = duet_api::workspace::WorkspaceStore::new(db.clone());
    let current = workspace.read(&c).await.unwrap();
    let mut content = duet_wire::models::DoctorPageContent::new();
    content.questions = Some(vec!["What should we discuss?".into()]);
    let save = duet_wire::models::DoctorPageSave::new(
        current["note_generation"].as_i64().unwrap() as i32,
        vec![],
        content,
    );
    let approved = workspace.save(&c, &save).await.unwrap();
    let view = publications
        .publish(
            &c,
            &duet_api::publication::PublishRequest {
                operation_id: Uuid::new_v4(),
                expected_generation: approved["note_generation"].as_i64().unwrap(),
                expires_in_seconds: 3600,
            },
        )
        .await
        .unwrap();
    let token = view.web_url.rsplit('/').next().unwrap().to_owned();
    sqlx::query("update duet_public_budget set used=600,window_start=now() where id=2")
        .execute(&mut admin_db)
        .await
        .unwrap();
    publications.retrieve(&token, false).await.unwrap();
    let (hit, miss): (i32, i32) = sqlx::query_as(
        "select (select used from duet_public_budget where id=1),(select used from duet_public_budget where id=2)",
    )
    .fetch_one(&mut admin_db)
    .await
    .unwrap();
    assert_eq!((hit, miss), (1, 600));
    assert!(matches!(
        publications.retrieve("bad", false).await,
        Err(duet_api::ApiError::Unavailable(_))
    ));
    let hit_after: i32 = sqlx::query_scalar("select used from duet_public_budget where id=1")
        .fetch_one(&mut admin_db)
        .await
        .unwrap();
    assert_eq!(hit_after, 1);
    sqlx::query("update duet_public_budget set used=0,window_start=now() where id=2")
        .execute(&mut admin_db)
        .await
        .unwrap();
    assert!(matches!(
        publications.shl_file("not-a-location").await,
        Err(duet_api::ApiError::NotFound)
    ));
    let (hit, miss): (i32, i32) = sqlx::query_as(
        "select (select used from duet_public_budget where id=1),(select used from duet_public_budget where id=2)",
    )
    .fetch_one(&mut admin_db)
    .await
    .unwrap();
    assert_eq!((hit, miss), (1, 1));

    drop(publications);
    drop(workspace);
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
