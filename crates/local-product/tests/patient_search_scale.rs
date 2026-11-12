//! Scale measurement for chart-record patient search on one very large
//! patient workspace.
//!
//! Ignored by default: it loads tens of thousands of records plus long text
//! parts standing in for OCR pages, then reports search time and how many rows
//! and blocks Postgres read for each chart table. Run it with
//! `cargo test -p local-product --test patient_search_scale -- --ignored --nocapture`.
//! The store runs on a single-connection pool, so the table counters read after
//! each search belong to that search alone.

use std::collections::BTreeMap;
use std::time::{Duration, Instant};

#[path = "common/disposable_postgres.rs"]
mod disposable_postgres;
use disposable_postgres::DisposablePostgres;

use local_product::physician_sources::{ReadablePart, RecordDate, SourcePatient, SourceRevision};
use local_product::runtime::Store;
use local_product::runtime::memory::{PatientAccess, PatientKey};
use local_product::runtime::patient_search::{
    PatientSearchOutcome, PatientSearchPage, PatientSearchRequest, SearchCaller,
    SearchDomainFilter, SearchStandingScope,
};
use local_product::runtime::sources::SourcePull;
use sea_orm::sea_query::{Alias, Expr, ExprTrait, Func, JoinType, Query};
use sea_orm::{ConnectOptions, ConnectionTrait, Database, DatabaseConnection};
use serde_json::{Value, json};

const WORKSPACE: &str = "patient-scale";
const SOURCE_ID: &str = "epic-fhir";

/// Small structured records (labs, vitals, problems, medications).
const STRUCTURED_RECORDS: usize = 30_000;
/// Documents carrying one long readable part each, standing in for OCR text.
const OCR_DOCUMENTS: usize = 1_000;
/// Bytes of readable text on each OCR document.
const OCR_PART_BYTES: usize = 32 * 1024;
/// Every this-many structured records is corrected by a second delivery.
const CORRECTION_STRIDE: usize = 10;
/// Timed runs per query.
const RUNS: usize = 5;

/// Structured records whose note carries the rare word, and how many.
const RARE_WORD: &str = "zebrafinch";
const RARE_WORD_STRIDE: usize = 10_000;
/// Structured records carrying the rare LOINC code.
const RARE_CODE: &str = "99999-9";
const RARE_CODE_STRIDE: usize = 7_500;
/// OCR documents whose long text ends with the rare OCR word.
const OCR_WORD: &str = "ocrneedleword";
const OCR_WORD_STRIDE: usize = 400;

const FILLER: &str = "Patient seen in clinic today for routine follow up of chronic conditions, \
tolerating current regimen, no new complaints, reviewed home readings and labs with patient. ";

fn revision(candidate: &str, rev: &str, record_type: &str, content: &Value) -> SourceRevision {
    let content = serde_json::to_string(content).expect("encode record content");
    let sha256 = local_product::sha256_hex(content.as_bytes());
    SourceRevision {
        workspace: WORKSPACE.into(),
        source_id: SOURCE_ID.into(),
        candidate_id: candidate.into(),
        revision: rev.into(),
        record_type: record_type.into(),
        dates: vec![RecordDate {
            name: "effectiveDateTime".into(),
            value: "2026-10-01".into(),
        }],
        content,
        sha256,
        readable_parts: Vec::new(),
        provenance: json!({ "source_id": SOURCE_ID, "native_id": candidate }),
    }
}

fn structured_record(index: usize, rev: &str) -> SourceRevision {
    let code = if index.is_multiple_of(RARE_CODE_STRIDE) {
        RARE_CODE.to_owned()
    } else {
        format!("{}-{}", 1000 + index % 4000, index % 10)
    };
    let note = if index % RARE_WORD_STRIDE == 7 {
        format!("{FILLER}Mentions {RARE_WORD} sighting.")
    } else {
        FILLER.to_owned()
    };
    let (record_type, resource) = match index % 4 {
        0 => ("Observation", "Observation"),
        1 => ("Condition", "Condition"),
        2 => ("MedicationRequest", "MedicationRequest"),
        _ => ("Observation", "Observation"),
    };
    let content = json!({
        "resourceType": resource,
        "status": "final",
        "code": { "coding": [{ "system": "http://loinc.org", "code": code }] },
        "performer": [{ "display": format!("Clinician {}", index % 37) }],
        "note": [{ "text": note }],
        "revisionLabel": rev,
    });
    revision(&format!("rec-{index:06}"), rev, record_type, &content)
}

/// Deterministic page text of pseudo-random words and numbers, so it does not
/// compress far below its length the way repeated filler would, and Postgres
/// stores it out of line as real OCR text is.
fn ocr_page_text(index: usize) -> String {
    const WORDS: [&str; 24] = [
        "patient",
        "reports",
        "pain",
        "denies",
        "fever",
        "history",
        "of",
        "hypertension",
        "diabetes",
        "follow",
        "up",
        "with",
        "cardiology",
        "labs",
        "reviewed",
        "stable",
        "imaging",
        "shows",
        "no",
        "acute",
        "findings",
        "plan",
        "continue",
        "medication",
    ];
    let mut state = (index as u64).wrapping_mul(6_364_136_223_846_793_005) ^ 0x9e37_79b9;
    let mut text = String::with_capacity(OCR_PART_BYTES + 64);
    while text.len() < OCR_PART_BYTES {
        state = state
            .wrapping_mul(6_364_136_223_846_793_005)
            .wrapping_add(1_442_695_040_888_963_407);
        let pick = (state >> 33) as usize;
        text.push_str(WORDS[pick % WORDS.len()]);
        text.push(' ');
        if pick.is_multiple_of(5) {
            text.push_str(&format!("{} ", (state >> 13) % 100_000));
        }
    }
    text
}

fn ocr_document(index: usize) -> SourceRevision {
    let content = json!({
        "resourceType": "DocumentReference",
        "status": "current",
        "type": { "coding": [{ "system": "http://loinc.org", "code": "34133-9" }] },
        "author": [{ "display": "Outside Records Intake" }],
    });
    let mut source = revision(
        &format!("doc-{index:05}"),
        "rev-1",
        "DocumentReference",
        &content,
    );
    let text = ocr_page_text(index);
    let mut text = text;
    if index % OCR_WORD_STRIDE == 11 {
        text.push_str(OCR_WORD);
    }
    source.readable_parts = vec![ReadablePart {
        label: "Scanned outside record, page 1".into(),
        text,
    }];
    source
}

fn pull(pulled_at: i64) -> SourcePull {
    SourcePull {
        source_id: SOURCE_ID.into(),
        source_label: "Epic".into(),
        patient: SourcePatient {
            source_patient_id: "scale-patient".into(),
            display_name: "Scale Patient".into(),
            birth_date: None,
            sex_or_gender: None,
            medical_record_numbers: Vec::new(),
        },
        pulled_at,
    }
}

/// Rows and blocks Postgres read from one table, from its cumulative counters.
#[derive(Clone, Copy, Debug, Default)]
struct TableReads {
    rows: i64,
    heap_blocks: i64,
    toast_blocks: i64,
}

impl TableReads {
    fn since(self, before: Self) -> Self {
        Self {
            rows: self.rows - before.rows,
            heap_blocks: self.heap_blocks - before.heap_blocks,
            toast_blocks: self.toast_blocks - before.toast_blocks,
        }
    }
}

/// Flushes this connection's pending table counters, then reads them for
/// every `physician_runtime_source*` table. The pool has one connection, so
/// the flush covers the search that just ran on it.
async fn chart_table_reads(db: &DatabaseConnection) -> BTreeMap<String, TableReads> {
    db.execute(
        &Query::select()
            .expr(Func::cust("pg_stat_force_next_flush"))
            .to_owned(),
    )
    .await
    .expect("flush table counters");
    db.execute(
        &Query::select()
            .expr(Func::cust("pg_stat_clear_snapshot"))
            .to_owned(),
    )
    .await
    .expect("clear stats snapshot");

    let relname = Alias::new("relname");
    let stat = Alias::new("stat");
    let statio = Alias::new("statio");
    let coalesce = |table: &Alias, column: &str| {
        Func::coalesce::<[Expr; 2], Expr>([
            Expr::col((table.clone(), Alias::new(column))),
            Expr::val(0_i64),
        ])
    };
    let rows_read = coalesce(&stat, "seq_tup_read").add(coalesce(&stat, "idx_tup_fetch"));
    let heap = coalesce(&statio, "heap_blks_read").add(coalesce(&statio, "heap_blks_hit"));
    let toast = coalesce(&statio, "toast_blks_read").add(coalesce(&statio, "toast_blks_hit"));
    let query = Query::select()
        .column((stat.clone(), relname.clone()))
        .expr(rows_read)
        .expr(heap)
        .expr(toast)
        .from_as(Alias::new("pg_stat_user_tables"), stat.clone())
        .join_as(
            JoinType::InnerJoin,
            Alias::new("pg_statio_user_tables"),
            statio.clone(),
            Expr::col((stat.clone(), Alias::new("relid")))
                .equals((statio.clone(), Alias::new("relid"))),
        )
        .and_where(Expr::col((stat.clone(), relname.clone())).like("physician_runtime_source%"))
        .to_owned();
    let mut out = BTreeMap::new();
    for row in db.query_all(&query).await.expect("read table counters") {
        let name: String = row.try_get_by_index(0).expect("relname");
        let reads = TableReads {
            rows: row.try_get_by_index(1).expect("rows read"),
            heap_blocks: row.try_get_by_index(2).expect("heap blocks"),
            toast_blocks: row.try_get_by_index(3).expect("toast blocks"),
        };
        out.insert(name, reads);
    }
    out
}

async fn chart_table_bytes(db: &DatabaseConnection) -> BTreeMap<String, i64> {
    let query = Query::select()
        .column(Alias::new("relname"))
        .expr(Func::cust("pg_total_relation_size").arg(Expr::col(Alias::new("relid"))))
        .from(Alias::new("pg_stat_user_tables"))
        .and_where(Expr::col(Alias::new("relname")).like("physician_runtime_source%"))
        .to_owned();
    let mut out = BTreeMap::new();
    for row in db.query_all(&query).await.expect("read table sizes") {
        let name: String = row.try_get_by_index(0).expect("relname");
        let bytes: i64 = row.try_get_by_index(1).expect("bytes");
        out.insert(name, bytes);
    }
    out
}

/// Runs every page of one search, returning the chart hits it delivered.
async fn search_all_pages(
    store: &Store,
    patient: &PatientKey,
    request: &PatientSearchRequest,
) -> usize {
    let caller = SearchCaller::PracticeJob {
        job_kind: "patient-review".into(),
        job_id: "scale-measure".into(),
    };
    let mut request = request.clone();
    let mut hits = 0;
    loop {
        let outcome = store
            .search_patient_for_access(patient, &caller, PatientAccess::ClinicalPanel, &request)
            .await
            .expect("patient search");
        let PatientSearchOutcome::Permitted(PatientSearchPage { results, next }) = outcome else {
            panic!("clinical panel access must permit search");
        };
        hits += results.len();
        match next {
            Some(cursor) => request.after = Some(cursor),
            None => return hits,
        }
    }
}

struct Case {
    label: &'static str,
    request: PatientSearchRequest,
    expected_hits: usize,
}

fn chart_request(query: &str, standing: SearchStandingScope) -> PatientSearchRequest {
    PatientSearchRequest {
        query: query.into(),
        domain: SearchDomainFilter::ChartRecords,
        standing,
        ..PatientSearchRequest::default()
    }
}

fn count_where(total: usize, stride: usize, remainder: usize) -> usize {
    (0..total).filter(|i| i % stride == remainder).count()
}

#[tokio::test]
#[ignore = "scale measurement: loads tens of thousands of records; run with --ignored --nocapture"]
async fn chart_search_on_one_very_large_workspace() {
    let Some(pg) = DisposablePostgres::acquire("patient-search-scale").await else {
        return;
    };
    pg.store().await;
    let mut options = ConnectOptions::new(pg.url());
    options
        .max_connections(1)
        .min_connections(1)
        .sqlx_logging(false);
    let db = Database::connect(options)
        .await
        .expect("connect single-connection pool");
    let store = Store::from_connection(db.clone());

    let structured: Vec<SourceRevision> = (0..STRUCTURED_RECORDS)
        .map(|i| structured_record(i, "rev-1"))
        .collect();
    let documents: Vec<SourceRevision> = (0..OCR_DOCUMENTS).map(ocr_document).collect();
    let mut first_delivery = structured.clone();
    first_delivery.extend(documents.iter().cloned());

    let load_started = Instant::now();
    let changed = store
        .record_chart(WORKSPACE, &first_delivery, &pull(1))
        .await
        .expect("record first delivery");
    let first_load = load_started.elapsed();
    assert_eq!(changed, STRUCTURED_RECORDS + OCR_DOCUMENTS);

    let mut second_delivery: Vec<SourceRevision> = (0..STRUCTURED_RECORDS)
        .map(|i| {
            if i % CORRECTION_STRIDE == 0 {
                structured_record(i, "rev-2")
            } else {
                structured[i].clone()
            }
        })
        .collect();
    second_delivery.extend(documents);
    let load_started = Instant::now();
    let corrected = store
        .record_chart(WORKSPACE, &second_delivery, &pull(2))
        .await
        .expect("record correcting delivery");
    let second_load = load_started.elapsed();
    let corrections = count_where(STRUCTURED_RECORDS, CORRECTION_STRIDE, 0);
    assert_eq!(corrected, corrections);

    let patient = PatientKey::new("practice-scale", WORKSPACE).expect("patient key");
    let rare_word_hits = count_where(STRUCTURED_RECORDS, RARE_WORD_STRIDE, 7);
    let rare_code_hits = count_where(STRUCTURED_RECORDS, RARE_CODE_STRIDE, 0);
    let ocr_hits = count_where(OCR_DOCUMENTS, OCR_WORD_STRIDE, 11);
    // Every rare-code record is also corrected, so its first revision matches
    // as superseded beside the current one.
    let rare_code_corrected = (0..STRUCTURED_RECORDS)
        .filter(|i| i % RARE_CODE_STRIDE == 0 && i % CORRECTION_STRIDE == 0)
        .count();
    let cases = [
        Case {
            label: "rare word, current",
            request: chart_request(RARE_WORD, SearchStandingScope::CurrentOnly),
            expected_hits: rare_word_hits,
        },
        Case {
            label: "rare code, with history",
            request: chart_request(RARE_CODE, SearchStandingScope::CurrentAndHistorical),
            expected_hits: rare_code_hits + rare_code_corrected,
        },
        Case {
            label: "word at end of OCR text",
            request: chart_request(OCR_WORD, SearchStandingScope::CurrentOnly),
            expected_hits: ocr_hits,
        },
        Case {
            label: "no match",
            request: chart_request("nothingmatchesthisterm", SearchStandingScope::CurrentOnly),
            expected_hits: 0,
        },
    ];

    println!();
    println!(
        "workspace: {STRUCTURED_RECORDS} structured records, {OCR_DOCUMENTS} documents with \
         {OCR_PART_BYTES}-byte text parts, {corrections} corrected"
    );
    println!("load: first delivery {first_load:?}, correcting delivery {second_load:?}");
    for (table, bytes) in chart_table_bytes(&db).await {
        println!("size: {table} {:.1} MiB", bytes as f64 / (1024.0 * 1024.0));
    }

    for case in &cases {
        let mut timings: Vec<Duration> = Vec::with_capacity(RUNS);
        let mut reads = BTreeMap::new();
        for run in 0..RUNS {
            let before = chart_table_reads(&db).await;
            let started = Instant::now();
            let hits = search_all_pages(&store, &patient, &case.request).await;
            timings.push(started.elapsed());
            assert_eq!(hits, case.expected_hits, "{}: hit count", case.label);
            if run == 0 {
                let after = chart_table_reads(&db).await;
                for (table, now) in after {
                    let then = before.get(&table).copied().unwrap_or_default();
                    reads.insert(table, now.since(then));
                }
            }
        }
        timings.sort();
        println!(
            "search: {:<26} hits {:>3}  min {:>9.1?}  median {:>9.1?}",
            case.label,
            case.expected_hits,
            timings[0],
            timings[RUNS / 2]
        );
        for (table, read) in reads {
            if read.rows == 0 && read.heap_blocks == 0 && read.toast_blocks == 0 {
                continue;
            }
            println!(
                "        {table:<40} rows {:>7}  heap blocks {:>6}  toast blocks {:>6}",
                read.rows, read.heap_blocks, read.toast_blocks
            );
        }
    }
}
