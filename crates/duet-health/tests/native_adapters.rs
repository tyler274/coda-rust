//! Native adapter proofs against fake HealthKit and Health Connect bridges.
use duet_health::acquisition::*;
use duet_health::native_adapters::*;
use std::cell::RefCell;
use std::collections::{BTreeMap, VecDeque};

const HR: &str = "HKQuantityTypeIdentifierHeartRate";
const WEIGHT: &str = "HKQuantityTypeIdentifierBodyMass";
const SPO2: &str = "HKQuantityTypeIdentifierOxygenSaturation";

fn specs() -> Vec<TypeSpec> {
    vec![
        TypeSpec::new(HR, "vital:heart_rate", "count/min"),
        TypeSpec::new(WEIGHT, "body:weight", "kg"),
    ]
}

fn sample(id: &str, value: &str, unit: &str) -> NativeSample {
    NativeSample {
        id: id.into(),
        value: value.into(),
        unit: Some(unit.into()),
        start: Some("2026-10-20T07:30:00+09:00".into()),
        end: Some("2026-10-20T07:30:00+09:00".into()),
        time_zone: Some("Asia/Tokyo".into()),
        source_name: Some("Apple Watch".into()),
        writer_id: Some("com.apple.health.ABC".into()),
        device: Some("Watch7,1".into()),
        metadata: None,
    }
}

/// Fake HealthKit: per-type scripted replies, recording every query.
#[derive(Default)]
struct FakeKit {
    unavailable: bool,
    replies: RefCell<BTreeMap<String, VecDeque<Result<HealthKitChanges, BridgeError>>>>,
    queries: RefCell<Vec<HealthKitQuery>>,
}

impl FakeKit {
    fn script(&self, data_type: &str, reply: Result<HealthKitChanges, BridgeError>) {
        self.replies
            .borrow_mut()
            .entry(data_type.into())
            .or_default()
            .push_back(reply);
    }
}

impl HealthKitBridge for &FakeKit {
    fn available(&self) -> bool {
        !self.unavailable
    }
    fn read_type(&self, query: &HealthKitQuery) -> Result<HealthKitChanges, BridgeError> {
        self.queries.borrow_mut().push(query.clone());
        self.replies
            .borrow_mut()
            .get_mut(&query.data_type)
            .and_then(VecDeque::pop_front)
            .unwrap_or_else(|| {
                Ok(HealthKitChanges {
                    anchor: query.anchor.clone().unwrap_or_else(|| "none".into()),
                    ..HealthKitChanges::default()
                })
            })
    }
}

fn fixed_now() -> i64 {
    1_800_000_000
}

fn kit(fake: &FakeKit) -> HealthKitAdapter<&FakeKit> {
    HealthKitAdapter::new(fake)
        .with_types(specs())
        .with_clock(fixed_now)
        .with_initial_window_days(7)
}

fn request(anchor: Option<Anchor>) -> PageRequest {
    PageRequest { anchor, limit: 100 }
}

fn changes(samples: Vec<NativeSample>, deleted: &[&str], anchor: &str) -> HealthKitChanges {
    HealthKitChanges {
        samples,
        deleted: deleted.iter().map(|d| d.to_string()).collect(),
        anchor: anchor.into(),
        has_more: false,
    }
}

#[test]
fn first_read_is_bounded_by_the_initial_window_and_the_bound_is_stable() {
    let open = FakeKit::default();
    let adapter = HealthKitAdapter::new(&open)
        .with_types(specs())
        .with_clock(fixed_now);
    let page = adapter.read_page(&request(None)).unwrap();
    assert!(
        open.queries
            .borrow()
            .iter()
            .all(|q| q.history_start_unix.is_none() && q.anchor.is_none())
    );
    let anchor = page.next_anchor.unwrap();
    assert!(
        !anchor.0.contains("history_start_unix"),
        "a default anchor stores no history floor"
    );
    let floor = (fixed_now() - 30 * 86_400).to_string();
    assert!(!anchor.0.contains(&floor));
    open.queries.borrow_mut().clear();
    HealthKitAdapter::new(&open)
        .with_types(specs())
        .with_clock(|| 2_000_000_000)
        .read_page(&request(Some(anchor)))
        .unwrap();
    assert!(
        open.queries
            .borrow()
            .iter()
            .all(|q| q.history_start_unix.is_none())
    );

    let fake = FakeKit::default();
    let adapter = kit(&fake);
    let page = adapter.read_page(&request(None)).unwrap();
    let first: Vec<Option<i64>> = fake
        .queries
        .borrow()
        .iter()
        .map(|q| q.history_start_unix)
        .collect();
    assert_eq!(first, vec![Some(fixed_now() - 7 * 86_400); 2]);
    assert!(fake.queries.borrow().iter().all(|q| q.anchor.is_none()));
    // An explicit window stays fixed even when the clock moves.
    let adapter = HealthKitAdapter::new(&fake)
        .with_types(specs())
        .with_clock(|| 2_000_000_000)
        .with_initial_window_days(7);
    adapter.read_page(&request(page.next_anchor)).unwrap();
    assert!(
        fake.queries
            .borrow()
            .iter()
            .all(|q| q.history_start_unix == Some(fixed_now() - 7 * 86_400))
    );
}

#[test]
fn a_failed_type_keeps_its_old_sub_anchor_while_others_advance() {
    let fake = FakeKit::default();
    fake.script(HR, Ok(changes(vec![], &[], "hr-1")));
    fake.script(WEIGHT, Ok(changes(vec![], &[], "w-1")));
    let adapter = kit(&fake);
    let first = adapter.read_page(&request(None)).unwrap();

    fake.script(
        HR,
        Ok(changes(vec![sample("a", "72", "count/min")], &[], "hr-2")),
    );
    fake.script(
        WEIGHT,
        Err(BridgeError::Failed {
            message: "boom".into(),
        }),
    );
    let second = adapter.read_page(&request(first.next_anchor)).unwrap();
    let weight = second
        .coverage
        .iter()
        .find(|c| c.data_type == WEIGHT)
        .unwrap();
    assert!(!weight.outcome.read_succeeded());

    fake.queries.borrow_mut().clear();
    adapter.read_page(&request(second.next_anchor)).unwrap();
    let anchors: BTreeMap<String, Option<String>> = fake
        .queries
        .borrow()
        .iter()
        .map(|q| (q.data_type.clone(), q.anchor.clone()))
        .collect();
    assert_eq!(anchors[HR].as_deref(), Some("hr-2"));
    assert_eq!(
        anchors[WEIGHT].as_deref(),
        Some("w-1"),
        "failed type not advanced"
    );
}

#[test]
fn deletions_pass_through_with_the_sample_identity() {
    let fake = FakeKit::default();
    fake.script(
        HR,
        Ok(changes(
            vec![sample("a", "72", "count/min")],
            &["b"],
            "hr-1",
        )),
    );
    let page = kit(&fake).read_page(&request(None)).unwrap();
    assert_eq!(page.records[0].native_id, "HKObject/a");
    assert_eq!(
        page.deletions,
        vec![DeletionRecord {
            native_id: "HKObject/b".into(),
            data_type: HR.into()
        }]
    );
    let hr = page.coverage.iter().find(|c| c.data_type == HR).unwrap();
    assert_eq!(hr.outcome, CoverageOutcome::Data);
}

#[test]
fn oxygen_saturation_percent_is_scaled_once_from_the_native_fraction() {
    let fake = FakeKit::default();
    fake.script(
        SPO2,
        Ok(changes(
            vec![sample("o", "0.98", "%"), sample("p", "0.985", "%")],
            &[],
            "o-1",
        )),
    );
    let page = HealthKitAdapter::new(&fake)
        .with_types(vec![TypeSpec::new(SPO2, "vital:oxygen_saturation", "%")])
        .with_clock(fixed_now)
        .read_page(&request(None))
        .unwrap();
    let oxygen = &page.records[0];
    assert_eq!(oxygen.value.literal, "98");
    assert_eq!(oxygen.value.unit.as_deref(), Some("%"));
    assert_eq!(oxygen.concept_id, "vital:oxygen_saturation");
    let original = oxygen.original.as_ref().unwrap();
    assert_eq!(original["reading"]["literal"], "0.98");
    assert_eq!(original["reading"]["unit"], "%");
    assert_eq!(page.records[1].value.literal, "98.5");
    assert_eq!(
        page.records[1].original.as_ref().unwrap()["reading"]["literal"],
        "0.985"
    );
}

#[test]
fn exact_value_unit_timezone_and_provenance_are_preserved() {
    let fake = FakeKit::default();
    let mut interval = sample("w", "070.40", "kg");
    interval.start = Some("2026-10-20T07:00:00-04:00".into());
    interval.end = Some("2026-10-20T07:05:00-04:00".into());
    interval.time_zone = Some("America/New_York".into());
    fake.script(
        HR,
        Ok(changes(vec![sample("a", "072", "count/min")], &[], "hr-1")),
    );
    fake.script(WEIGHT, Ok(changes(vec![interval], &[], "w-1")));
    let page = kit(&fake).read_page(&request(None)).unwrap();
    let hr = &page.records[0];
    assert_eq!(hr.value.literal, "072");
    assert_eq!(hr.value.unit.as_deref(), Some("count/min"));
    assert_eq!(hr.concept_id, "vital:heart_rate");
    assert_eq!(
        hr.time,
        SourceTime::Instant {
            at: "2026-10-20T07:30:00+09:00".into()
        }
    );
    assert_eq!(hr.time_zone.as_deref(), Some("Asia/Tokyo"));
    assert_eq!(
        hr.provenance.writer_id.as_deref(),
        Some("com.apple.health.ABC")
    );
    assert_eq!(hr.provenance.device.as_deref(), Some("Watch7,1"));
    let weight = &page.records[1];
    assert_eq!(weight.value.literal, "070.40");
    assert_eq!(
        weight.time,
        SourceTime::Interval {
            start: Some("2026-10-20T07:00:00-04:00".into()),
            end: Some("2026-10-20T07:05:00-04:00".into()),
        }
    );
    assert_eq!(weight.time_zone.as_deref(), Some("America/New_York"));
}

#[test]
fn page_limit_is_respected_across_types() {
    let fake = FakeKit::default();
    let mut first = changes(vec![sample("a", "1", "count/min")], &[], "hr-1");
    first.has_more = true;
    fake.script(HR, Ok(first));
    let page = kit(&fake)
        .read_page(&PageRequest {
            anchor: None,
            limit: 1,
        })
        .unwrap();
    assert!(page.has_more);
    assert_eq!(page.records.len(), 1);
    assert_eq!(
        fake.queries.borrow().len(),
        1,
        "weight waits for the next page"
    );
}

#[test]
fn whole_store_failures_are_typed_and_commit_nothing() {
    let unavailable = FakeKit {
        unavailable: true,
        ..FakeKit::default()
    };
    assert_eq!(
        kit(&unavailable).read_page(&request(None)),
        Err(AdapterError::StoreUnavailable)
    );
    let locked = FakeKit::default();
    locked.script(HR, Err(BridgeError::StoreUnavailable));
    locked.script(WEIGHT, Err(BridgeError::StoreUnavailable));
    assert_eq!(
        kit(&locked).read_page(&request(None)),
        Err(AdapterError::StoreUnavailable)
    );
    assert_eq!(
        kit(&locked).read_page(&request(Some(Anchor("garbage".into())))),
        Err(AdapterError::AnchorInvalidated)
    );
}

#[test]
fn a_type_with_an_invalidated_anchor_rereads_only_itself() {
    let fake = FakeKit::default();
    fake.script(HR, Ok(changes(vec![], &[], "hr-1")));
    fake.script(WEIGHT, Ok(changes(vec![], &[], "w-1")));
    let adapter = kit(&fake);
    let first = adapter.read_page(&request(None)).unwrap();
    fake.script(HR, Err(BridgeError::AnchorInvalidated));
    let second = adapter.read_page(&request(first.next_anchor)).unwrap();
    fake.queries.borrow_mut().clear();
    adapter.read_page(&request(second.next_anchor)).unwrap();
    let anchors: BTreeMap<String, Option<String>> = fake
        .queries
        .borrow()
        .iter()
        .map(|q| (q.data_type.clone(), q.anchor.clone()))
        .collect();
    assert_eq!(anchors[HR], None);
    assert_eq!(anchors[WEIGHT].as_deref(), Some("w-1"));
}

#[test]
fn bridge_replies_decode_to_typed_results() {
    let ok = decode_healthkit_reply(
        r#"{"ok":{"samples":[{"id":"a","value":"72","unit":"count/min"}],"deleted":["b"],"anchor":"QQ=="}}"#,
    )
    .unwrap();
    assert_eq!(ok.samples[0].value, "72");
    assert_eq!(ok.deleted, vec!["b".to_string()]);
    assert_eq!(
        decode_healthkit_reply(r#"{"error":{"code":"store_unavailable"}}"#),
        Err(BridgeError::StoreUnavailable)
    );
    assert!(matches!(
        decode_healthkit_reply("not json"),
        Err(BridgeError::Failed { .. })
    ));
}

// ---------------------------------------------------------------------------
// Health Connect
// ---------------------------------------------------------------------------

#[derive(Default)]
struct FakeConnect {
    tokens: RefCell<Vec<Vec<String>>>,
    changes: RefCell<VecDeque<Result<HealthConnectChanges, BridgeError>>>,
    seen: RefCell<Vec<String>>,
}

impl HealthConnectBridge for &FakeConnect {
    fn changes_token(&self, record_types: &[String]) -> Result<String, BridgeError> {
        self.tokens.borrow_mut().push(record_types.to_vec());
        Ok("t0".into())
    }
    fn changes(&self, token: &str) -> Result<HealthConnectChanges, BridgeError> {
        self.seen.borrow_mut().push(token.into());
        self.changes
            .borrow_mut()
            .pop_front()
            .unwrap_or_else(|| Ok(HealthConnectChanges::default()))
    }
}

#[test]
fn health_connect_is_disabled_by_default_and_says_so() {
    let fake = FakeConnect::default();
    let adapter = HealthConnectAdapter::new(&fake);
    assert_eq!(adapter.scope(), &HealthConnectScope::Disabled);
    let page = adapter.read_page(&request(None)).unwrap();
    assert!(page.records.is_empty() && page.deletions.is_empty());
    assert_eq!(
        page.coverage,
        vec![TypeCoverage {
            data_type: HEALTH_CONNECT_FEED.into(),
            outcome: CoverageOutcome::Error {
                error: CoverageError::NotEnabled
            },
        }]
    );
    assert!(
        !page.coverage[0].outcome.read_succeeded(),
        "never silent empty"
    );
    assert!(fake.tokens.borrow().is_empty() && fake.seen.borrow().is_empty());
}

fn enabled(fake: &FakeConnect) -> HealthConnectAdapter<&FakeConnect> {
    HealthConnectAdapter::new(fake).with_scope(HealthConnectScope::Enabled {
        types: vec![TypeSpec::new("HeartRateRecord", "vital:heart_rate", "bpm")],
    })
}

#[test]
fn health_connect_pages_the_changes_feed_and_keeps_the_writing_package() {
    let fake = FakeConnect::default();
    let adapter = enabled(&fake);
    let first = adapter.read_page(&request(None)).unwrap();
    assert_eq!(fake.tokens.borrow()[0], vec!["HeartRateRecord".to_string()]);
    assert!(first.has_more);
    let mut hr = sample("r1", "61", "bpm");
    hr.writer_id = Some("com.sec.android.app.shealth".into());
    hr.source_name = None;
    let mut out_of_scope = sample("r2", "70", "kg");
    out_of_scope.writer_id = Some("com.example".into());
    fake.changes
        .borrow_mut()
        .push_back(Ok(HealthConnectChanges {
            upserts: vec![
                HealthConnectRecord {
                    record_type: "HeartRateRecord".into(),
                    sample: hr,
                },
                HealthConnectRecord {
                    record_type: "WeightRecord".into(),
                    sample: out_of_scope,
                },
            ],
            deleted: vec!["r9".into()],
            next_token: "t1".into(),
            has_more: true,
            token_expired: false,
        }));
    let second = adapter.read_page(&request(first.next_anchor)).unwrap();
    assert_eq!(fake.seen.borrow()[0], "t0");
    assert_eq!(second.records.len(), 1);
    let record = &second.records[0];
    assert_eq!(record.native_id, "HealthConnect/r1");
    assert_eq!(
        record.provenance.writer_id.as_deref(),
        Some("com.sec.android.app.shealth")
    );
    assert_eq!(
        record.provenance.source_name.as_deref(),
        Some("Samsung Health")
    );
    assert_eq!(second.deletions[0].native_id, "HealthConnect/r9");
    assert_eq!(second.deletions[0].data_type, HEALTH_CONNECT_FEED);
    assert!(second.has_more);
    adapter.read_page(&request(second.next_anchor)).unwrap();
    assert_eq!(fake.seen.borrow()[1], "t1");
}

#[test]
fn health_connect_records_without_an_exact_value_are_reported_not_uploaded() {
    let fake = FakeConnect::default();
    let adapter = enabled(&fake);
    let anchor = adapter.read_page(&request(None)).unwrap().next_anchor;
    let mut empty = sample("r1", "", "bpm");
    empty.unit = None;
    fake.changes
        .borrow_mut()
        .push_back(Ok(HealthConnectChanges {
            upserts: vec![HealthConnectRecord {
                record_type: "HeartRateRecord".into(),
                sample: empty,
            }],
            next_token: "t1".into(),
            ..HealthConnectChanges::default()
        }));
    let page = adapter.read_page(&request(anchor)).unwrap();
    assert!(page.records.is_empty());
    let hr = page
        .coverage
        .iter()
        .find(|c| c.data_type == "HeartRateRecord")
        .unwrap();
    assert!(!hr.outcome.read_succeeded());
}

#[test]
fn health_connect_errors_are_typed() {
    let fake = FakeConnect::default();
    let adapter = enabled(&fake);
    let anchor = adapter.read_page(&request(None)).unwrap().next_anchor;
    fake.changes
        .borrow_mut()
        .push_back(Ok(HealthConnectChanges {
            token_expired: true,
            ..HealthConnectChanges::default()
        }));
    assert_eq!(
        adapter.read_page(&request(anchor.clone())),
        Err(AdapterError::AnchorInvalidated)
    );
    fake.changes
        .borrow_mut()
        .push_back(Err(BridgeError::PermissionDenied));
    assert_eq!(
        adapter.read_page(&request(anchor)),
        Err(AdapterError::PermissionLost)
    );
    assert_eq!(
        decode_changes_reply(r#"{"error":{"code":"store_unavailable"}}"#),
        Err(BridgeError::StoreUnavailable)
    );
    assert_eq!(
        decode_changes_token_reply(r#"{"ok":"tok"}"#),
        Ok("tok".into())
    );
}

// ---------------------------------------------------------------------------
// Access honesty
// ---------------------------------------------------------------------------

#[test]
fn permission_request_completion_is_not_readable_data() {
    assert_eq!(read_access(false, []), ReadAccess::NotRequested);
    assert_eq!(read_access(true, []), ReadAccess::RequestCompleted);
    assert_eq!(
        read_access(true, [&CoverageOutcome::Empty]),
        ReadAccess::RequestCompleted
    );
    assert_eq!(
        read_access(true, [&CoverageOutcome::PermissionLost]),
        ReadAccess::Unreadable
    );
    assert_eq!(
        read_access(true, [&CoverageOutcome::Empty, &CoverageOutcome::Data]),
        ReadAccess::ReadableData
    );
}
