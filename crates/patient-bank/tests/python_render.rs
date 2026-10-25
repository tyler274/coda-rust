//! The renderer writes the same NDJSON bytes and receipts as render.py.
//! `data/render-unit.json` is a real accepted ledger (unit-0080) with its seed
//! and the extra terms it needs; `data/render-edges.json` is a small hand-made
//! ledger that reaches branches the bank data does not (unknown terms, skipped
//! items, stated differentials, DST gap and fold stamps, pending results), plus
//! `friendly_drug` and `ckd_epi_2021` tables. Every expected value was produced
//! by CPython 3.14 running scripts/patient-bank/render.py.

use std::fmt::Write as _;
use std::path::Path;

use patient_bank::json::{self, Json, Layout};
use patient_bank::render::{self, Receipt, Renderer};
use sha2::{Digest, Sha256};

fn catalog() -> Json {
    render::load_catalog(
        &Path::new(env!("CARGO_MANIFEST_DIR")).join("../../scripts/patient-bank/catalog.json"),
    )
}

fn sha256_hex(text: &str) -> String {
    Sha256::digest(text.as_bytes())
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect()
}

fn strings(v: &Json) -> Vec<String> {
    v.as_array()
        .iter()
        .map(|s| s.as_str().unwrap().to_string())
        .collect()
}

fn check_cases(fixture: &Json) {
    let r = Renderer::new(&catalog(), fixture.get("extra"));
    let ledger = fixture.get("ledger");
    let cases = fixture.get("cases").as_array();
    assert!(!cases.is_empty());
    for case in cases {
        let mrn = case.get("mrn").as_str().unwrap();
        let as_of = case.get("as_of").as_str().unwrap();
        let find = |list: &Json| {
            list.as_array()
                .iter()
                .find(|p| p.get("mrn").as_str() == Some(mrn))
                .unwrap()
                .clone()
        };
        let (chart, receipt) = r.render(
            &find(fixture.get("seeds")),
            &find(ledger.get("patients")),
            as_of,
        );
        for (family, rows) in chart.iter() {
            let mut text = String::new();
            for row in rows {
                writeln!(text, "{}", json::dumps(row, Layout::Compact, false)).unwrap();
            }
            assert_eq!(
                Some(sha256_hex(&text).as_str()),
                case.get("sha256").get(family).as_str(),
                "{mrn} as of {as_of}: {family}"
            );
        }
        let want = case.get("receipt");
        let want = Receipt {
            filled: strings(want.get("filled")),
            derived: strings(want.get("derived")),
            uncoded: strings(want.get("uncoded")),
            pending: strings(want.get("pending")),
            skipped: strings(want.get("skipped")),
        };
        assert_eq!(receipt, want, "{mrn} as of {as_of}: receipt");
    }
}

#[test]
fn real_unit_renders_like_python() {
    check_cases(&json::parse(include_str!("data/render-unit.json")).unwrap());
}

#[test]
fn edge_ledger_renders_like_python() {
    check_cases(&json::parse(include_str!("data/render-edges.json")).unwrap());
}

#[test]
fn helpers_match_python() {
    let edges = json::parse(include_str!("data/render-edges.json")).unwrap();
    let names = edges.get("friendly_drug").as_object();
    assert!(names.len() > 300);
    for (name, want) in names {
        assert_eq!(
            Some(render::friendly_drug(name).as_str()),
            want.as_str(),
            "{name}"
        );
    }
    for row in edges.get("ckd_epi_2021").as_array() {
        let r = row.as_array();
        let (scr, age, female) = (
            r[0].as_f64().unwrap(),
            r[1].as_i64().unwrap(),
            r[2] == Json::Bool(true),
        );
        assert_eq!(
            render::ckd_epi_2021(scr, age, female),
            r[3].as_i64().unwrap(),
            "{row:?}"
        );
    }
}
