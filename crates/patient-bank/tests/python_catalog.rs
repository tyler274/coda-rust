//! The catalog build and `resolve_extras` write exactly what the Python originals
//! wrote. Inputs are real: `data/catalog-cache.json` is the shared terminology cache
//! trimmed to the rows an offline build reads, `catalog-roger` keeps the Roger
//! fixture's CPT codes and displays, and the `terms-*` case is cut from the bank's
//! ledgers and cache. Expected outputs were produced by CPython 3.14; the offline
//! build reproduces the committed `scripts/patient-bank/catalog.json` (sha256 below).

use std::fs;
use std::path::{Path, PathBuf};

use patient_bank::catalog::{self, Lookup, py_repr, urlencode};
use patient_bank::json::{self, Layout};
use patient_bank::terms;
use sha2::{Digest, Sha256};

const CATALOG_SHA256: &str = "bb92c8e5d992582bee7358742bc9b8db6cf181530d6ebdc1eb42240128d013e8";

fn data(name: &str) -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/data")
        .join(name)
}

fn scratch(name: &str) -> PathBuf {
    let dir = Path::new(env!("CARGO_TARGET_TMPDIR")).join(name);
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    dir
}

#[test]
fn offline_build_reproduces_the_python_catalog() {
    let dir = scratch("catalog-build");
    let cache = dir.join("terminology-cache.json");
    fs::copy(data("catalog-cache.json"), &cache).unwrap();
    let out = dir.join("catalog.json");
    let mut look = Lookup::new(&cache, true).unwrap();
    let status = catalog::build_with(
        &mut look,
        &data("catalog-gym-lab-ranges.json"),
        &data("catalog-roger"),
        &out,
    )
    .unwrap();
    assert_eq!(status, 0);
    let digest: String = Sha256::digest(fs::read(&out).unwrap())
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect();
    assert_eq!(digest, CATALOG_SHA256, "catalog.json differs; see {out:?}");
    // save() rewrites the cache as json.dumps(indent=0, sort_keys=True): unchanged bytes
    assert_eq!(
        fs::read(&cache).unwrap(),
        fs::read(data("catalog-cache.json")).unwrap()
    );
}

#[test]
fn offline_build_refuses_an_uncached_lookup() {
    let dir = scratch("catalog-offline-miss");
    let mut look = Lookup::new(dir.join("terminology-cache.json"), true).unwrap();
    let err = catalog::build_with(
        &mut look,
        &data("catalog-gym-lab-ranges.json"),
        &data("catalog-roger"),
        &dir.join("catalog.json"),
    )
    .unwrap_err();
    assert!(err.to_string().starts_with("offline and not cached: "));
}

#[test]
fn resolve_extras_matches_python_and_keeps_entries_written_meanwhile() {
    let dir = scratch("terms");
    let live = dir.join("terminology-cache.json");
    fs::copy(data("terms-cache.json"), &live).unwrap();
    let mut look = Lookup::new(&live, true).unwrap();
    // Another authoring agent rewrites the shared cache after this lookup loaded it.
    let mut disk = json::parse(&fs::read_to_string(&live).unwrap()).unwrap();
    let update = json::parse(&fs::read_to_string(data("terms-disk-update.json")).unwrap()).unwrap();
    for (url, value) in update.as_object() {
        disk.set(url, value.clone());
    }
    fs::write(&live, json::dumps(&disk, Layout::Default, true)).unwrap();

    let catalog = json::parse(&fs::read_to_string(data("terms-catalog.json")).unwrap()).unwrap();
    let got = terms::resolve_extras_with(&mut look, &data("terms-ledgers"), &catalog).unwrap();
    assert_eq!(
        json::dumps(&got, Layout::Indent(1), false) + "\n",
        include_str!("data/terms-expected.json")
    );
    assert_eq!(
        fs::read_to_string(&live).unwrap(),
        include_str!("data/terms-expected-cache.json")
    );
    let left: Vec<_> = fs::read_dir(&dir).unwrap().collect();
    assert_eq!(
        left.len(),
        1,
        "the temporary cache file was replaced into place"
    );
}

#[test]
fn repr_and_urlencode_match_python() {
    let cases = json::parse(include_str!("data/python-catalog-text.json")).unwrap();
    for case in cases.get("repr").as_array() {
        let c = case.as_array();
        assert_eq!(py_repr(c[0].as_str().unwrap()), c[1].as_str().unwrap());
    }
    for case in cases.get("urlencode").as_array() {
        let c = case.as_array();
        let got = urlencode(&[("term", c[0].as_str().unwrap()), ("maxEntries", "5")]);
        assert_eq!(got, c[1].as_str().unwrap());
    }
}

#[test]
fn resolve_extras_matches_python_on_unusual_ledger_fields() {
    // CPython 3.14 wrote expected.json: str.strip() also drops U+001C..U+001F, the
    // first Clinical Tables row with the code decides (a null name rejects), and a
    // float label formats as str(float).
    let dir = scratch("terms-edge");
    let cache = dir.join("terminology-cache.json");
    fs::copy(data("terms-edge/cache.json"), &cache).unwrap();
    let mut look = Lookup::new(&cache, true).unwrap();
    let catalog = json::parse(include_str!("data/terms-edge/catalog.json")).unwrap();
    let got = terms::resolve_extras_with(&mut look, &data("terms-edge/ledgers"), &catalog).unwrap();
    assert_eq!(
        json::dumps(&got, Layout::Indent(1), false) + "\n",
        include_str!("data/terms-edge/expected.json")
    );
}

#[test]
fn fixture_path_displays_as_pathlib_does() {
    // str(PurePosixPath(p) / "Procedure.ndjson") from CPython 3.14
    let cases = [
        ("./missing//dir/", "missing/dir/Procedure.ndjson"),
        ("missing/./dir", "missing/dir/Procedure.ndjson"),
        ("", "Procedure.ndjson"),
        (".", "Procedure.ndjson"),
        ("/abs//x/", "/abs/x/Procedure.ndjson"),
        ("//net/x", "//net/x/Procedure.ndjson"),
        ("///three/x", "/three/x/Procedure.ndjson"),
        ("a/../b", "a/../b/Procedure.ndjson"),
        ("../up", "../up/Procedure.ndjson"),
    ];
    for (roger, want) in cases {
        let got = catalog::pathlib_str(&Path::new(roger).join("Procedure.ndjson"));
        assert_eq!(got, want, "roger fixture {roger:?}");
    }
}
