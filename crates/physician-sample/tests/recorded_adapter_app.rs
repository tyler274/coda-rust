//! Captured acquisition, real application enrollment/checks, and Postgres readback.
//! No extension transport, model provider or source-store writes are simulated here.
use async_trait::async_trait;
use local_product::{
    physician_sources::{hint::ChartCapture, *},
    runtime::Store,
};
use physician_auth::{AuthStore, Membership, PhysicianIdentity, PostgresAuthStore, PracticeClock};
use physician_sample::{
    chart_load::load_practice_from_source, sources::SourceRegistry,
    synthetic_chart::SyntheticCharts,
};
use serde_json::{Value, json};
use std::{collections::BTreeMap, fs, path::PathBuf, sync::Arc};
#[path = "common/disposable.rs"]
mod disposable;

struct CapturedHint(BTreeMap<String, Value>);
impl CapturedHint {
    fn pull(&self, id: &str, workspace: &str) -> SourcePullOutput {
        serde_json::from_value::<ChartCapture>(self.0[id].clone())
            .unwrap()
            .file(workspace, self.source_id(), id)
    }
}
#[async_trait]
impl PatientSource for CapturedHint {
    fn source_id(&self) -> &str {
        "hint-captured"
    }
    fn source_label(&self) -> &str {
        "Captured Hint"
    }
    async fn patient_ids(&self) -> Result<Vec<String>, SourceAdapterError> {
        Ok(self.0.keys().cloned().collect())
    }
    async fn patient(&self, id: &str) -> Result<SourcePatient, SourceAdapterError> {
        match self.pull(id, &workspace_id(self.source_id(), id)) {
            SourcePullOutput::Complete { patient, .. }
            | SourcePullOutput::Partial {
                patient: Some(patient),
                ..
            } => Ok(patient),
            SourcePullOutput::Partial { patient: None, .. }
            | SourcePullOutput::Failed { .. }
            | SourcePullOutput::LapsedAccess { .. } => {
                panic!("qualification inputs must identify their patient")
            }
        }
    }
    async fn records(
        &self,
        patient: &SourcePatient,
        workspace: &str,
    ) -> Result<Vec<SourceRevision>, SourceAdapterError> {
        match self.pull(&patient.source_patient_id, workspace) {
            SourcePullOutput::Complete { records, .. } => Ok(records),
            SourcePullOutput::Partial { failed_parts, .. } => {
                Err(SourceAdapterError::Incomplete { failed_parts })
            }
            SourcePullOutput::Failed { .. } | SourcePullOutput::LapsedAccess { .. } => {
                panic!("qualification input failed")
            }
        }
    }
    async fn pull_patient(&self, _: &str, id: &str, workspace: &str) -> SourcePullOutput {
        self.pull(id, workspace)
    }
}

#[tokio::test]
#[ignore = "requires ADAPTER_APP_INPUTS and disposable Postgres"]
async fn captured_sources_enroll_refresh_and_reopen_through_the_app() {
    let inputs = PathBuf::from(std::env::var("ADAPTER_APP_INPUTS").unwrap());
    let mut captures = BTreeMap::new();
    for path in fs::read_dir(inputs.join("hint")).unwrap() {
        let path = path.unwrap().path();
        if path.extension().and_then(|s| s.to_str()) != Some("json") {
            continue;
        }
        let capture: Value = serde_json::from_slice(&fs::read(path).unwrap()).unwrap();
        let id = capture["request"]["patient_id"]
            .as_str()
            .unwrap()
            .to_owned();
        assert!(
            captures.insert(id, capture).is_none(),
            "one capture per patient"
        );
    }
    assert_eq!(captures.len(), 4);
    let admin_url = std::env::var("PHYSICIAN_RUNTIME_TEST_DATABASE_URL").unwrap();
    let admin = sea_orm::Database::connect(&admin_url).await.unwrap();
    let name = format!("adapter_app_{}", uuid::Uuid::new_v4().simple());
    disposable::create_empty_database(&admin, &name).await;
    let mut url = reqwest::Url::parse(&admin_url).unwrap();
    url.set_path(&name);
    let auth = Arc::new(
        PostgresAuthStore::connect_with_migrations(url.as_str())
            .await
            .unwrap(),
    );
    let front = Arc::new(
        front_door::PostgresFrontDoorStore::connect_with_migrations(url.as_str())
            .await
            .unwrap(),
    );
    let store = Store::connect(url.as_str()).await.unwrap();
    store.migrate().await.unwrap();
    let practice = "adapter-proof";
    auth.ensure_practice(practice, PracticeClock::RealTime)
        .await
        .unwrap();
    auth.ensure_account(
        practice,
        "reader",
        "reader@codahealth.test",
        "Reader",
        true,
        1,
    )
    .await
    .unwrap();
    let registry = SourceRegistry::new(Some(store.clone()));
    let expected_hint = captures.clone();
    let expected_fhir: Vec<Value> =
        serde_json::from_slice(&fs::read(inputs.join("mimic-expected.json")).unwrap()).unwrap();
    let hint: Arc<dyn PatientSource> = Arc::new(CapturedHint(captures));
    let fhir: Arc<dyn PatientSource> = Arc::new(SyntheticCharts::open(&inputs.join("mimic")));
    for (source, patients, records) in [(&hint, 4, 30), (&fhir, 11, 74)] {
        registry.register(Arc::clone(source));
        let loaded = load_practice_from_source(practice, &**source, &*auth, &store)
            .await
            .unwrap();
        assert!(loaded.failed.is_empty(), "{:?}", loaded.failed);
        assert_eq!((loaded.patients, loaded.records), (patients, records));
        let repeated = load_practice_from_source(practice, &**source, &*auth, &store)
            .await
            .unwrap();
        assert_eq!(repeated.changed, 0);
        for id in source.patient_ids().await.unwrap() {
            let workspace = workspace_id(source.source_id(), &id);
            assert_eq!(
                auth.patient_practice(&workspace).await.unwrap().as_deref(),
                Some(practice)
            );
        }
    }
    let identity = PhysicianIdentity {
        practice_id: practice.into(),
        physician_id: "reader".into(),
    };
    for source in [&hint, &fhir] {
        for id in source.patient_ids().await.unwrap() {
            auth.set_membership(
                &identity,
                &workspace_id(source.source_id(), &id),
                Membership::Allowed,
            )
            .await
            .unwrap();
        }
    }
    let router = physician_sample::authed_router_on(
        physician_sample::fixture_site().layer(axum::Extension(registry)),
        auth.clone(),
        front,
    )
    .await;
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let base = format!("http://{}", listener.local_addr().unwrap());
    let server = tokio::spawn(async move {
        axum::serve(listener, router).await.unwrap();
    });
    let client = reqwest::Client::new();
    let start: Value = client
        .post(format!("{base}/auth/sign-in"))
        .json(&json!({"email":"reader@codahealth.test"}))
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();
    let signed: Value = client
        .post(format!("{base}/auth/verify"))
        .json(&json!({"email":"reader@codahealth.test","token":start["token"]}))
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap();
    let token = signed["session"].as_str().unwrap();
    for source in [&hint, &fhir] {
        for id in source.patient_ids().await.unwrap() {
            let workspace = workspace_id(source.source_id(), &id);
            let before = store.sources(&workspace).await.unwrap();
            let checked_at = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs();
            let response = client
                .post(format!("{base}/connections/{}/check", source.source_id()))
                .bearer_auth(token)
                .json(&json!({"patient_workspace_id":workspace}))
                .send()
                .await
                .unwrap();
            assert_eq!(response.status(), 200);
            let response: Value = response.json().await.unwrap();
            let report = &response["patient"];
            assert_eq!(report["applied"], "applied");
            assert_eq!(report["changed"], 0);
            assert_eq!(report["records_delivered"], before.sources.len());
            let standing = &report["standing"];
            if source.source_id() == "hint-captured" {
                assert_eq!(standing["standing"], "partial");
                assert!(standing["last_good_read_at"].is_null());
                assert!(standing["attempted_at"].as_u64().unwrap() >= checked_at);
                assert!(
                    standing["failed_parts"]
                        .as_array()
                        .unwrap()
                        .iter()
                        .any(|part| part["part"]
                            .as_str()
                            .unwrap()
                            .contains("appointment date range"))
                );
            } else {
                assert_eq!(standing["standing"], "up_to_date");
                assert!(standing["last_good_read_at"].as_u64().unwrap() >= checked_at);
            }
            let reopened = Store::connect(url.as_str()).await.unwrap();
            let observation = reopened.sources(&workspace).await.unwrap();
            assert_eq!(observation.sources.len(), before.sources.len());
            assert_eq!(observation.version, before.version);
            for record in &observation.sources {
                let envelope = record.envelope.as_ref().unwrap();
                if source.source_id() == "hint-captured" {
                    let capture = &expected_hint[&id];
                    let section = capture["sections"]
                        .as_array()
                        .unwrap()
                        .iter()
                        .find(|section| section["section"] == envelope.source_kind.kind)
                        .unwrap();
                    let identity_field = match envelope.source_kind.kind.as_str() {
                        "patient" => "id",
                        "notes" => "noteId",
                        "files" => "Key",
                        _ => "ident",
                    };
                    let native = section["result"]["body"]["items"]
                        .as_array()
                        .unwrap()
                        .iter()
                        .find(|item| {
                            item["body"][identity_field].as_str()
                                == Some(envelope.native_id.as_str())
                        })
                        .unwrap();
                    assert_eq!(
                        serde_json::from_str::<Value>(&record.content).unwrap(),
                        native["body"]
                    );
                } else {
                    let native = expected_fhir
                        .iter()
                        .find(|item| {
                            item["patient"] == id
                                && record
                                    .candidate_id
                                    .ends_with(item["native"].as_str().unwrap())
                        })
                        .unwrap();
                    assert_eq!(record.content, native["raw"].as_str().unwrap());
                    assert_eq!(
                        serde_json::to_value(envelope.bucket).unwrap(),
                        native["bucket"]
                    );
                }
                let request = ChartRequest::parse(READ_TOOL, &json!({"references":[{"candidate_id":record.candidate_id,"revision":record.revision,"cursor":null}],"max_result_bytes":65536}), ChartScope::Chat).unwrap();
                let read = request
                    .run(&workspace, observation.version, &observation.sources)
                    .unwrap();
                let read: Value = serde_json::from_str(read.text().unwrap()).unwrap();
                assert_eq!(read["results"][0]["content"], record.content);
                assert_eq!(
                    read["results"][0]["bucket"],
                    serde_json::to_value(envelope.bucket).unwrap()
                );
            }
        }
    }
    server.abort();
    disposable::drop_database(&admin, &name).await;
}
