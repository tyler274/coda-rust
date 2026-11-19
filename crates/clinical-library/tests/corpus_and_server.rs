//! Integration proof tests for clinical library freshness (AC3) and
//! multi-source ingestion + active Postgres snapshot serving (AC4).

mod common;

use clinical_library::{
    ArticleState, CorpusFeedItem, CorpusNightJob, HolderState, ItemKey, LibraryFreshnessReport,
    LibraryStanding, Night, NightOutcome, OtherSource, ReadItemOutcome, ReadUnavailableCause,
    RunId, SearchResult, SlotName, Source, SourceFreshnessState, VersionId,
    pubmed::{Pmid, Relation, StatusCheck, StatusRecord},
    run_night, server,
};
use common::{Says, Scratch, complete, hold_slot};
use serde_json::json;
use sha2::{Digest, Sha256};
use tokio::net::TcpListener;

const PMC_JATS_VALID: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
<article>
  <front>
    <article-meta>
      <article-id pub-id-type="pmc">PMC9911001</article-id>
      <article-id pub-id-type="pmid">9911001</article-id>
      <article-id pub-id-type="doi">10.1000/pmc.9911001</article-id>
      <title-group>
        <article-title>Early Norepinephrine Initiation in Septic Shock</article-title>
      </title-group>
      <pub-date><year>2025</year><month>04</month><day>12</day></pub-date>
      <permissions>
        <license license-type="cc-by">CC BY 4.0 Open Access</license>
      </permissions>
      <abstract>
        <p>Early norepinephrine stabilizes sepsis perfusion while awaiting definitive fluid reassessment.</p>
      </abstract>
    </article-meta>
  </front>
  <body>
    <sec id="results">
      <title>Results</title>
      <p>Six-hour shock control occurred more frequently with early vasopressor support.</p>
    </sec>
  </body>
</article>"#;

const PMC_JATS_RESTRICTED: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
<article>
  <front>
    <article-meta>
      <article-id pub-id-type="pmc">PMC9911002</article-id>
      <title-group><article-title>Restricted Manuscript</article-title></title-group>
      <permissions>
        <license license-type="all-rights-reserved">All Rights Reserved</license>
      </permissions>
      <abstract><p>Restricted text must not be stored.</p></abstract>
    </article-meta>
  </front>
</article>"#;

const DAILYMED_SPL_VALID: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
<document>
  <title>FARXIGA (dapagliflozin) tablets, for oral use</title>
  <effectiveTime value="20251103"/>
  <versionNumber value="14"/>
  <component>
    <structuredBody>
      <component>
        <section>
          <code code="34068-7" codeSystem="2.16.840.1.113883.6.1"/>
          <title>DOSAGE AND ADMINISTRATION</title>
          <text>Initiate dapagliflozin 10 mg orally once daily in patients with chronic kidney disease when eGFR is at least 25 mL/min/1.73 m2.</text>
        </section>
      </component>
    </structuredBody>
  </component>
</document>"#;

const MMWR_XML_VALID: &str = r#"<?xml version="1.0" encoding="UTF-8"?>
<mmwr>
  <title>Advisory Update on Myocarditis Monitoring After Immunization</title>
  <published>2025-10-18</published>
  <section id="summary">
    <heading>Clinical Summary</heading>
    <p>Evaluate persistent chest pain or elevated troponin promptly for vaccine-associated myocarditis.</p>
  </section>
</mmwr>"#;

const USPSTF_JSON_VALID: &str = r#"{
  "title": "Screening for Lung Cancer With Low-Dose Computed Tomography",
  "grade": "B",
  "published_date": "2025-03-09",
  "sections": [
    {
      "id": "rec",
      "heading": "Recommendation Statement",
      "text": "The USPSTF recommends annual screening for lung cancer with low-dose computed tomography in adults aged 50 to 80 years who have a 20 pack-year smoking history."
    }
  ]
}"#;

#[tokio::test]
async fn ac4_multi_source_ingestors_run_through_run_night_and_serve_http_with_pubmed_status() {
    let scratch = Scratch::new().await;
    let store = scratch.store().await;
    let slot = SlotName::new("nightly-corpus").unwrap();
    let version = VersionId::new("2026-11-19-v1").unwrap();

    let feed = vec![
        CorpusFeedItem::PmcJats {
            pmcid: "PMC9911001".into(),
            xml: PMC_JATS_VALID.into(),
        },
        CorpusFeedItem::PmcJats {
            pmcid: "PMC9911002".into(),
            xml: PMC_JATS_RESTRICTED.into(),
        },
        CorpusFeedItem::DailyMedSpl {
            setid: "setid-dapa".into(),
            xml: DAILYMED_SPL_VALID.into(),
        },
        CorpusFeedItem::MmwrXml {
            mmwr_id: "mm7440a1".into(),
            xml: MMWR_XML_VALID.into(),
        },
        CorpusFeedItem::MmwrXml {
            mmwr_id: "mm7440-bad".into(),
            xml: "<unclosed>broken".into(),
        },
        CorpusFeedItem::UspstfJson {
            recommendation_id: "uspstf-lung-ca".into(),
            json: USPSTF_JSON_VALID.into(),
        },
    ];

    let job = CorpusNightJob::new(
        vec![Source::Pmc, Source::DailyMed, Source::Mmwr, Source::Uspstf],
        version.clone(),
        feed,
    );

    let night = run_night(&store, &Says(HolderState::Alive), &slot, RunId::new(), &job)
        .await
        .unwrap();
    assert!(
        matches!(night, Night::Finished(NightOutcome::Deferred(_))),
        "partial run with two isolated bad items must defer as published_with_set_aside, got: {night:?}"
    );
    assert_eq!(store.active_version().await.unwrap(), Some(version));

    // Confirm failed items remain isolated while valid documents are Admitted.
    assert_eq!(
        store
            .item(&ItemKey::other(OtherSource::Pmc, "PMC9911001").unwrap())
            .await
            .unwrap(),
        Some(ArticleState::Admitted)
    );
    assert!(matches!(
        store
            .item(&ItemKey::other(OtherSource::Pmc, "PMC9911002").unwrap())
            .await
            .unwrap(),
        Some(ArticleState::WaitingToRetry(_))
    ));
    assert!(matches!(
        store
            .item(&ItemKey::other(OtherSource::Mmwr, "mm7440-bad").unwrap())
            .await
            .unwrap(),
        Some(ArticleState::WaitingToRetry(_))
    ));

    // Update PubMed status independently (without rebuilding the content snapshot) to retract PMID 9911001.
    let pubmed_slot = SlotName::new("nightly-pubmed").unwrap();
    let pmid = Pmid::new(9911001).unwrap();
    let pubmed_lease = match store
        .acquire(
            &pubmed_slot,
            &[Source::PubMed],
            RunId::new(),
            &Says(HolderState::Alive),
        )
        .await
        .unwrap()
    {
        clinical_library::Acquired::Holds(l) => l,
        other => panic!("expected lease, got {other:?}"),
    };
    let pubmed_key = ItemKey::pubmed(pmid);
    store.qualify(&pubmed_lease, &pubmed_key).await.unwrap();
    let retraction_xml =
        "<PubmedArticle><MedlineCitation><PMID>9911001</PMID></MedlineCitation></PubmedArticle>"
            .to_owned();
    let retraction_sha: [u8; 32] = Sha256::digest(retraction_xml.as_bytes()).into();
    store
        .record_status_check(
            &pubmed_lease,
            &StatusCheck::Found(StatusRecord {
                pmid,
                publication_types: vec!["Retracted Publication".into()],
                relations: vec![Relation {
                    ref_type: "RetractionIn".into(),
                    target: None,
                    source_reference: Some("N Engl J Med 2026".into()),
                    note: Some("Retracted by authors".into()),
                }],
                revised: None,
                record_sha256: retraction_sha,
                record_xml: retraction_xml,
            }),
        )
        .await
        .unwrap();
    let _ = complete(&store, pubmed_lease).await;

    // Serve HTTP endpoints via Axum router.
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    let app = server::router(store.clone());
    let server_task = tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });

    let base_url = format!("http://{addr}");
    let client = reqwest::Client::new();

    // Verify all 4 content collections respond cleanly via POST /v1/search.
    let mut pmc_candidate_id = None;
    for (query, expected_source) in [
        ("norepinephrine sepsis", "pmc"),
        ("dapagliflozin eGFR", "dailymed"),
        ("myocarditis chest pain", "cdc-mmwr"),
        ("lung cancer tomography", "uspstf"),
    ] {
        let body = serde_json::to_vec(&json!({
            "mode": "query",
            "query": query,
            "scheme": null,
            "value": null,
            "candidate_id": null,
            "relationship": null,
            "cursor": null
        }))
        .unwrap();
        let resp = client
            .post(format!("{base_url}/v1/search"))
            .header("content-type", "application/json")
            .body(body)
            .send()
            .await
            .unwrap();
        assert_eq!(resp.status(), reqwest::StatusCode::OK);
        let parsed: SearchResult = serde_json::from_slice(&resp.bytes().await.unwrap()).unwrap();
        let hit = parsed
            .candidates
            .iter()
            .find(|c| c.source_code == expected_source)
            .unwrap_or_else(|| {
                panic!("expected hit from {expected_source} for `{query}`: {parsed:?}")
            });
        if expected_source == "pmc" {
            pmc_candidate_id = Some(hit.candidate_id.clone());
        }
    }

    // Verify POST /v1/read enriches PMC9911001 with live PubMed RetractionIn observations
    // and surfaces invalid/unknown IDs cleanly.
    let pmc_id = pmc_candidate_id.unwrap();
    let missing_uuid = uuid::Uuid::new_v4().to_string();
    let read_req_body = serde_json::to_vec(&json!({
        "candidate_ids": [pmc_id, "not-a-uuid", missing_uuid]
    }))
    .unwrap();
    let read_resp = client
        .post(format!("{base_url}/v1/read"))
        .header("content-type", "application/json")
        .body(read_req_body)
        .send()
        .await
        .unwrap();
    assert_eq!(read_resp.status(), reqwest::StatusCode::OK);
    let read_body: serde_json::Value =
        serde_json::from_slice(&read_resp.bytes().await.unwrap()).unwrap();
    let results: Vec<ReadItemOutcome> =
        serde_json::from_value(read_body["results"].clone()).unwrap();
    assert_eq!(results.len(), 3);
    match &results[0] {
        ReadItemOutcome::Available { unit, .. } => {
            assert_eq!(
                unit.lifecycle_state,
                clinical_library::LifecycleState::Retracted
            );
            let obs = unit
                .bibliographic_observations
                .as_ref()
                .expect("linked PMID must attach live PubMed status");
            assert_eq!(obs[0].subject_pmid, "9911001");
            assert!(
                obs[0]
                    .publication_types
                    .iter()
                    .any(|p| p == "Retracted Publication")
            );
        }
        other => panic!("expected available unit, got {other:?}"),
    }
    assert!(matches!(
        &results[1],
        ReadItemOutcome::Unavailable {
            cause: ReadUnavailableCause::InvalidCandidateId,
            ..
        }
    ));
    assert!(matches!(
        &results[2],
        ReadItemOutcome::Unavailable {
            cause: ReadUnavailableCause::NotFoundOrNotAuthorized,
            ..
        }
    ));

    // Verify GET /v1/freshness returns recorded search latencies within target bounds
    // AND reflects stale standing (because PMC9911002 & mm7440-bad are waiting_to_retry).
    let fresh_resp = client
        .get(format!("{base_url}/v1/freshness"))
        .send()
        .await
        .unwrap();
    assert_eq!(fresh_resp.status(), reqwest::StatusCode::OK);
    let report: LibraryFreshnessReport =
        serde_json::from_slice(&fresh_resp.bytes().await.unwrap()).unwrap();
    assert!(matches!(report.standing, LibraryStanding::Stale { .. }));
    assert!(report.search_latency.sample_count >= 4);
    assert_eq!(report.search_latency.within_targets, Some(true));
    assert!(report.search_latency.p95_ms <= 3_000.0);
    assert!(report.search_latency.p99_ms <= 5_000.0);

    server_task.abort();
    scratch.drop().await;
}

#[tokio::test]
async fn ac3_freshness_report_tracks_unrun_lagging_failed_and_current_states() {
    let scratch = Scratch::new().await;
    let store = scratch.store().await;
    let slot = SlotName::new("nightly-freshness").unwrap();

    // 1. Fresh DB with zero nights must never report Current.
    let initial = store.freshness_report().await.unwrap();
    assert!(
        matches!(initial.standing, LibraryStanding::Stale { ref reason } if reason.contains("initial nightly refresh")),
        "expected unrun library to be stale, got: {initial:?}"
    );
    assert_eq!(initial.search_latency.within_targets, None);
    assert!(
        initial
            .sources
            .iter()
            .all(|s| matches!(s.status, SourceFreshnessState::NeverRefreshed))
    );

    // 2. Partial refresh covering only DailyMed leaves unrun sources marked NeverRefreshed and standing Stale.
    let partial_job = CorpusNightJob::new(
        vec![Source::DailyMed],
        VersionId::new("v-partial").unwrap(),
        vec![CorpusFeedItem::DailyMedSpl {
            setid: "setid-dapa".into(),
            xml: DAILYMED_SPL_VALID.into(),
        }],
    );
    let partial_night = run_night(
        &store,
        &Says(HolderState::Alive),
        &slot,
        RunId::new(),
        &partial_job,
    )
    .await
    .unwrap();
    assert!(matches!(
        partial_night,
        Night::Finished(NightOutcome::Succeeded(_))
    ));

    let after_partial = store.freshness_report().await.unwrap();
    assert!(matches!(
        after_partial.standing,
        LibraryStanding::Stale { .. }
    ));
    let dm = after_partial
        .sources
        .iter()
        .find(|s| s.source_code == "dailymed")
        .unwrap();
    assert!(matches!(dm.status, SourceFreshnessState::Current { .. }));
    let pmc = after_partial
        .sources
        .iter()
        .find(|s| s.source_code == "pmc")
        .unwrap();
    assert!(matches!(pmc.status, SourceFreshnessState::NeverRefreshed));

    // 3. Complete clean refresh across all 5 sources -> LibraryStanding::Current.
    let pubmed_lease = hold_slot(&store, &slot, RunId::new()).await;
    let pmid = Pmid::new(550011).unwrap();
    let pubmed_key = ItemKey::pubmed(pmid);
    store.qualify(&pubmed_lease, &pubmed_key).await.unwrap();
    let pubmed_xml =
        "<PubmedArticle><MedlineCitation><PMID>550011</PMID></MedlineCitation></PubmedArticle>"
            .to_owned();
    let pubmed_sha: [u8; 32] = Sha256::digest(pubmed_xml.as_bytes()).into();
    store
        .record_status_check(
            &pubmed_lease,
            &StatusCheck::Found(StatusRecord {
                pmid,
                publication_types: vec!["Journal Article".into()],
                relations: vec![],
                revised: None,
                record_sha256: pubmed_sha,
                record_xml: pubmed_xml,
            }),
        )
        .await
        .unwrap();
    store
        .record_source_check_status(
            &pubmed_lease,
            Source::PubMed,
            clinical_library::SourceCheckStanding::Succeeded,
        )
        .await
        .unwrap();
    assert!(complete(&store, pubmed_lease).await.success().is_some());

    let full_job = CorpusNightJob::new(
        vec![Source::Pmc, Source::DailyMed, Source::Mmwr, Source::Uspstf],
        VersionId::new("v-full").unwrap(),
        vec![
            CorpusFeedItem::PmcJats {
                pmcid: "PMC9911001".into(),
                xml: PMC_JATS_VALID.into(),
            },
            CorpusFeedItem::MmwrXml {
                mmwr_id: "mm7440a1".into(),
                xml: MMWR_XML_VALID.into(),
            },
            CorpusFeedItem::UspstfJson {
                recommendation_id: "uspstf-lung-ca".into(),
                json: USPSTF_JSON_VALID.into(),
            },
        ],
    );
    let full_night = run_night(
        &store,
        &Says(HolderState::Alive),
        &slot,
        RunId::new(),
        &full_job,
    )
    .await
    .unwrap();
    assert!(matches!(
        full_night,
        Night::Finished(NightOutcome::Succeeded(_))
    ));

    let current_report = store.freshness_report().await.unwrap();
    assert_eq!(
        current_report.standing,
        LibraryStanding::Current,
        "all 5 clean sources should yield LibraryStanding::Current: {current_report:?}"
    );
    assert!(
        current_report
            .sources
            .iter()
            .all(|s| matches!(s.status, SourceFreshnessState::Current { .. }))
    );

    // 3b. Advancing the clock past PubMed's 1-day SLA and content's 3-day SLA degrades Current -> Stale/Lagging.
    let now = chrono::DateTime::<chrono::Utc>::from(std::time::SystemTime::now());
    let past_pubmed_sla = store
        .freshness_report_at(now + chrono::TimeDelta::hours(26))
        .await
        .unwrap();
    assert!(matches!(
        past_pubmed_sla.standing,
        LibraryStanding::Stale { ref reason } if reason.contains("1-day freshness SLA")
    ));
    let pubmed_overdue = past_pubmed_sla
        .sources
        .iter()
        .find(|s| s.source_code == "pubmed")
        .unwrap();
    assert!(matches!(
        pubmed_overdue.status,
        SourceFreshnessState::Lagging { .. }
    ));
    let dailymed_still_ok = past_pubmed_sla
        .sources
        .iter()
        .find(|s| s.source_code == "dailymed")
        .unwrap();
    assert!(matches!(
        dailymed_still_ok.status,
        SourceFreshnessState::Current { .. }
    ));

    let past_content_sla = store
        .freshness_report_at(now + chrono::TimeDelta::days(4))
        .await
        .unwrap();
    assert!(
        past_content_sla
            .sources
            .iter()
            .all(|s| matches!(s.status, SourceFreshnessState::Lagging { .. }))
    );

    // 4. Injecting a failing night immediately moves standing back to Stale.
    let fail_lease = hold_slot(&store, &slot, RunId::new()).await;
    let fail_key = ItemKey::other(OtherSource::Mmwr, "mm-broken").unwrap();
    store.qualify(&fail_lease, &fail_key).await.unwrap();
    let _ = complete(&store, fail_lease).await;

    let degraded = store.freshness_report().await.unwrap();
    assert!(matches!(degraded.standing, LibraryStanding::Stale { .. }));

    scratch.drop().await;
}
