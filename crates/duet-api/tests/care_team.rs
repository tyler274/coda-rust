//! Focused disposable PostgreSQL proof; missing DB configuration fails, never skips.
use async_trait::async_trait;
use chrono::{Duration, Utc};
use duet_api::{
    ApiError,
    care_team::{CareTeamStore, MemberPatch, ReleaseInput, ReleaseRecipient, ReleaseScope},
    database::{AccountDatabase, migrate},
    durable::DurableState,
    evidence::EvidenceStore,
    identity::{IdentityStore, IdentityVerifier, VerifiedIdentity},
    privacy::{FileJournal, PrivacyStore, Restriction},
};
use sqlx::{AssertSqlSafe, ConnectOptions, Connection, PgConnection};
use std::sync::Arc;
use uuid::Uuid;
struct PausedDirectory {
    entered: tokio::sync::Notify,
    resume: tokio::sync::Notify,
}
#[async_trait]
impl npi_registry::Directory for PausedDirectory {
    async fn by_number(
        &self,
        npi: &npi_registry::Npi,
    ) -> Result<Option<npi_registry::Clinician>, npi_registry::RegistryError> {
        self.entered.notify_one();
        self.resume.notified().await;
        npi_registry::Directory::by_number(&npi_registry::FixtureDirectory::sample(), npi).await
    }
    async fn search(
        &self,
        q: &npi_registry::SearchQuery,
    ) -> Result<Vec<npi_registry::Clinician>, npi_registry::RegistryError> {
        npi_registry::Directory::search(&npi_registry::FixtureDirectory::sample(), q).await
    }
}
struct Verifier;
#[async_trait]
impl IdentityVerifier for Verifier {
    async fn verify(&self, bearer: &str) -> Result<VerifiedIdentity, ApiError> {
        Ok(VerifiedIdentity {
            subject: bearer.to_owned(),
            authenticated_at: Utc::now() - Duration::minutes(1),
        })
    }
}
#[tokio::test]
async fn account_owned_crud_release_and_deletion_fences() {
    let url = std::env::var("DUET_TEST_DATABASE_URL")
        .expect("DUET_TEST_DATABASE_URL must point to disposable loopback PostgreSQL");
    let opts: sqlx::postgres::PgConnectOptions = url.parse().unwrap();
    assert!(["localhost", "127.0.0.1", "::1"].contains(&opts.get_host()));
    let suffix = Uuid::new_v4().simple().to_string();
    let role = format!("care_{suffix}");
    let name = format!("care_{suffix}");
    let mut admin = PgConnection::connect(&url).await.unwrap();
    sqlx::raw_sql(AssertSqlSafe(format!("create role {role} login")))
        .execute(&mut admin)
        .await
        .unwrap();
    sqlx::raw_sql(AssertSqlSafe(format!("create database {name}")))
        .execute(&mut admin)
        .await
        .unwrap();
    let owner_url = opts.clone().database(&name).to_url_lossy().to_string();
    let runtime_url = opts
        .database(&name)
        .username(&role)
        .to_url_lossy()
        .to_string();
    let mut owner = PgConnection::connect(&owner_url).await.unwrap();
    sqlx::raw_sql("revoke create on schema public from public")
        .execute(&mut owner)
        .await
        .unwrap();
    migrate(&owner_url, &role).await.unwrap();
    let db = AccountDatabase::connect(&runtime_url).await.unwrap();
    let identity = IdentityStore::new(db.clone(), Arc::new(Verifier));
    let a = identity.authenticate("a", true).await.unwrap();
    let b = identity.authenticate("b", true).await.unwrap();
    let journal = std::env::temp_dir().join(format!("care-journal-{suffix}"));
    let privacy = PrivacyStore::new(
        db.clone(),
        Arc::new(FileJournal::open(journal.clone()).await.unwrap()),
    );
    let store = CareTeamStore {
        durable: DurableState {
            identity,
            privacy: privacy.clone(),
            evidence: EvidenceStore::new(db.clone()),
        },
        directory: Arc::new(npi_registry::FixtureDirectory::sample()),
    };
    let npi = npi_registry::parse_response(npi_registry::samples::INDIVIDUAL).unwrap()[0]
        .npi
        .to_string();
    let member = store.add(&a, &npi).await.unwrap();
    assert_eq!(store.add(&a, &npi).await.unwrap().id, member.id);
    assert_ne!(store.add(&b, &npi).await.unwrap().id, member.id);
    assert_eq!(store.list(&a).await.unwrap().len(), 1);
    assert!(matches!(
        store.patch(&b, member.id, MemberPatch::default()).await,
        Err(ApiError::NotFound)
    ));
    assert!(matches!(
        store.delete(&b, member.id).await,
        Err(ApiError::NotFound)
    ));
    assert!(matches!(
        store.refresh(&b, member.id).await,
        Err(ApiError::NotFound)
    ));
    let patch = MemberPatch {
        nickname: Some("My clinician".into()),
        ..Default::default()
    };
    store.patch(&a, member.id, patch).await.unwrap();
    assert_eq!(
        store.refresh(&a, member.id).await.unwrap().nickname,
        "My clinician"
    );
    let body = ReleaseInput {
        discloser_member_id: member.id,
        patient: None,
        recipient: ReleaseRecipient::Patient,
        scope: ReleaseScope::default(),
        purpose: None,
        expires_on: None,
        expiration_event: None,
        signed_name: None,
    };
    assert!(matches!(
        store.save_release(&b, None, body.clone()).await,
        Err(ApiError::NotFound)
    ));
    let release = store.save_release(&a, None, body.clone()).await.unwrap();
    // Generated DTOs decode the exact backend projections, including null fields.
    let _: duet_wire::models::CareTeamMember =
        serde_json::from_value(serde_json::to_value(&member).unwrap()).unwrap();
    let _: duet_wire::models::ReleaseAuthorizationView =
        serde_json::from_value(serde_json::to_value(&release).unwrap()).unwrap();
    use tower::ServiceExt;
    let router = duet_api::care_team_routes::router(store.clone());
    for (bearer, status) in [("a", 200), ("b", 404)] {
        let response = router
            .clone()
            .oneshot(
                axum::http::Request::builder()
                    .uri(format!("/api/release-authorizations/{}/pdf", release.id))
                    .header("authorization", format!("Bearer {bearer}"))
                    .body(axum::body::Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status().as_u16(), status);
        assert_eq!(response.headers()["cache-control"], "private, no-store");
    }
    assert!(matches!(
        store.render_release(&b, release.id).await,
        Err(ApiError::NotFound)
    ));
    assert!(matches!(
        store.save_release(&b, Some(release.id), body.clone()).await,
        Err(ApiError::NotFound)
    ));
    assert!(
        store
            .render_release(&a, release.id)
            .await
            .unwrap()
            .preview_svg()
            .contains("svg")
    );
    let mut signed = body.clone();
    signed.expiration_event = Some("End of treatment".into());
    signed.signed_name = Some("Synthetic Patient".into());
    assert!(
        store
            .save_release(&a, Some(release.id), signed)
            .await
            .unwrap()
            .signed_at
            .is_some()
    );
    assert!(
        store
            .save_release(&a, Some(release.id), body)
            .await
            .unwrap()
            .signed_at
            .is_none()
    );
    // Deletion during remote lookup must win: no stale account can resurrect rows.
    let paused = Arc::new(PausedDirectory {
        entered: tokio::sync::Notify::new(),
        resume: tokio::sync::Notify::new(),
    });
    let racing = CareTeamStore {
        durable: store.durable.clone(),
        directory: paused.clone(),
    };
    let context = a.clone();
    let lookup_npi = npi.clone();
    let lookup = tokio::spawn(async move { racing.add(&context, &lookup_npi).await });
    paused.entered.notified().await;
    privacy
        .restrict(
            &a,
            Uuid::new_v4(),
            Restriction::AccountDelete,
            a.account_id,
            0,
        )
        .await
        .unwrap();
    paused.resume.notify_one();
    assert!(matches!(
        lookup.await.unwrap(),
        Err(ApiError::AbsentAccount)
    ));
    assert!(matches!(
        store.add(&a, &npi).await,
        Err(ApiError::AbsentAccount)
    ));
    assert!(store.render_release(&a, release.id).await.is_err());
    for table in ["duet_care_team", "duet_release_authorizations"] {
        let count: i64 = sqlx::query_scalar(AssertSqlSafe(format!(
            "select count(*) from {table} where user_id=$1"
        )))
        .bind(a.account_id)
        .fetch_one(&mut owner)
        .await
        .unwrap();
        assert_eq!(count, 0);
    }
    assert_eq!(store.list(&b).await.unwrap().len(), 1);
    drop(store);
    drop(privacy);
    drop(db);
    drop(owner);
    sqlx::raw_sql(AssertSqlSafe(format!("drop database {name} with (force)")))
        .execute(&mut admin)
        .await
        .unwrap();
    sqlx::raw_sql(AssertSqlSafe(format!("drop role {role}")))
        .execute(&mut admin)
        .await
        .unwrap();
    tokio::fs::remove_dir_all(journal).await.unwrap();
}
