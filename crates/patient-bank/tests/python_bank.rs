//! The bank build, materialization, panels, future-mention warnings, reading copy
//! and vocabulary match the Python pipeline. `data/bank-unit/` is a real accepted
//! two-patient ledger (unit-0015, events before 2023) with its seed; its
//! `expected.json` holds the extra terms Python resolved for it and the sha256 of
//! every file build_bank.py and materialize.py wrote. `data/panels-design.json` is
//! the full design's units with Python's panels over them. All expected values
//! were produced by CPython 3.14 running scripts/patient-bank.

use std::collections::{BTreeMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};

use patient_bank::bank::{self, AS_OF};
use patient_bank::json::{self, Json, Layout};
use patient_bank::render::{self, Renderer};
use patient_bank::{materialize, unit, validate};
use sha2::{Digest, Sha256};

fn data() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/data")
}

fn catalog() -> Json {
    render::load_catalog(
        &Path::new(env!("CARGO_MANIFEST_DIR")).join("../../scripts/patient-bank/catalog.json"),
    )
}

fn expected() -> Json {
    json::parse(&fs::read_to_string(data().join("bank-unit/expected.json")).unwrap()).unwrap()
}

fn sha256_hex(bytes: &[u8]) -> String {
    Sha256::digest(bytes)
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect()
}

/// Every file under `root` by relative path, with its sha256.
fn tree(root: &Path) -> BTreeMap<String, String> {
    fn walk(root: &Path, dir: &Path, out: &mut BTreeMap<String, String>) {
        for entry in fs::read_dir(dir).unwrap() {
            let path = entry.unwrap().path();
            if path.is_dir() {
                walk(root, &path, out);
            } else {
                let rel = path
                    .strip_prefix(root)
                    .unwrap()
                    .to_string_lossy()
                    .into_owned();
                out.insert(rel, sha256_hex(&fs::read(&path).unwrap()));
            }
        }
    }
    let mut out = BTreeMap::new();
    walk(root, root, &mut out);
    out
}

fn files(v: &Json) -> BTreeMap<String, String> {
    v.as_object()
        .iter()
        .map(|(k, v)| (k.clone(), v.as_str().unwrap().to_string()))
        .collect()
}

fn scratch(name: &str) -> PathBuf {
    Path::new(env!("CARGO_TARGET_TMPDIR")).join(name)
}

#[test]
fn build_and_materialize_write_python_bytes() {
    let want = expected();
    let catalog = catalog();
    let extra = want.get("extra");
    let src = data().join("bank-unit");
    let out = scratch("python-bank");
    let summary = bank::build(
        &catalog,
        extra,
        &src.join("seeds.json"),
        &src.join("ledgers"),
        &out,
        None,
    )
    .unwrap();
    assert_eq!(
        json::dumps(&summary, Layout::Indent(1), true) + "\n",
        want.get("build").get("stdout").as_str().unwrap()
    );
    assert_eq!(tree(&out), files(want.get("build").get("files")));

    let r = Renderer::new(&catalog, extra);
    for case in want.get("materialize").as_array() {
        let as_of = case.get("as_of").as_str().unwrap();
        let panel_out = scratch("python-bank-panel");
        let summary = materialize::write(
            &out,
            &src.join("seeds.json"),
            case.get("panel").as_i64().unwrap(),
            &panel_out,
            as_of,
            Some(&r),
        )
        .unwrap();
        assert_eq!(
            json::dumps(&summary, Layout::Indent(1), true) + "\n",
            case.get("stdout").as_str().unwrap(),
            "{as_of}"
        );
        assert_eq!(tree(&panel_out), files(case.get("files")), "{as_of}");
    }
}

#[test]
fn reading_copy_warnings_and_vocabulary_match_python() {
    let want = expected();
    let catalog = catalog();
    assert_eq!(
        sha256_hex(unit::vocabulary(&catalog).as_bytes()),
        want.get("vocabulary").as_str().unwrap()
    );
    let src = data().join("bank-unit");
    let seeds = json::parse(&fs::read_to_string(src.join("seeds.json")).unwrap()).unwrap();
    let led =
        json::parse(&fs::read_to_string(src.join("ledgers/unit-0015.json")).unwrap()).unwrap();
    let r = Renderer::new(&catalog, want.get("extra"));
    let mut mentioned = 0;
    for m in seeds.get("units").as_array()[0].get("members").as_array() {
        let mrn = m.get("mrn").as_str().unwrap();
        let p = led
            .get("patients")
            .as_array()
            .iter()
            .find(|p| p.get("mrn").as_str() == Some(mrn))
            .unwrap();
        let (chart, _) = r.render(m, p, AS_OF);
        assert_eq!(
            sha256_hex(unit::chart_text(&chart).unwrap().as_bytes()),
            want.get("chart_text").get(mrn).as_str().unwrap(),
            "{mrn} chart.txt"
        );
        let got: Vec<Json> = validate::future_mentions(&chart)
            .unwrap()
            .into_iter()
            .map(|(kind, msg)| Json::Arr(vec![kind.into(), msg.into()]))
            .collect();
        assert_eq!(
            &Json::Arr(got),
            want.get("future_mentions").get(mrn),
            "{mrn}"
        );
        mentioned += want.get("future_mentions").get(mrn).as_array().len();
    }
    assert!(mentioned > 0, "the fixture exercises the warning");
}

#[test]
fn panels_match_python_over_the_design() {
    let data =
        json::parse(&fs::read_to_string(data().join("panels-design.json")).unwrap()).unwrap();
    let units = data.get("units").as_array();
    let all: HashSet<String> = units
        .iter()
        .flat_map(|u| u.get("members").as_array())
        .map(|m| m.get("mrn").as_str().unwrap().to_string())
        .collect();
    let dropped: HashSet<String> = data
        .get("dropped")
        .as_array()
        .iter()
        .map(|m| m.as_str().unwrap().to_string())
        .collect();
    let some: HashSet<String> = all.difference(&dropped).cloned().collect();
    for (key, mrns) in [("all", &all), ("some", &some)] {
        for (size, members) in bank::panels(units, mrns) {
            let list = Json::Arr(members.iter().map(|m| Json::from(m.as_str())).collect());
            assert_eq!(
                sha256_hex(json::dumps(&list, Layout::Default, true).as_bytes()),
                data.get(key).get(&size.to_string()).as_str().unwrap(),
                "{key} panel-{size}"
            );
            if key == "all" && size == 100 {
                assert_eq!(
                    &Json::Arr(list.as_array()[..12].to_vec()),
                    data.get("first")
                );
            }
        }
    }
}
