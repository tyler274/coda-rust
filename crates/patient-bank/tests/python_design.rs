//! The panel design writes the seeds file `design_panel.py` wrote, byte for byte.
//! `data/python-seeds-7.json` and the digests below were produced by CPython 3.14
//! (`design_panel.py --count N --first-mrn M`); the 995-patient digest is the
//! running bank's `seeds.json`.

use patient_bank::design;
use sha2::{Digest, Sha256};

fn sha256(text: &str) -> String {
    Sha256::digest(text.as_bytes())
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect()
}

#[test]
fn small_panel_matches_python_in_full() {
    let got = design::render(&design::seeds(7, 6));
    assert_eq!(got, include_str!("data/python-seeds-7.json"));
}

#[test]
fn bank_panel_matches_python_digest() {
    for (count, first_mrn, want) in [
        (
            995,
            6,
            "6a9572d49a4359f93ea2e285f06b05af693ff521a576171be4cf80e3507816df",
        ),
        (
            50,
            6,
            "e24191b19230fd30e3fb978c8f427944633b1516b9cc99b2c2356883b05946f5",
        ),
        (
            1400,
            100,
            "a6ca5e59ecac6bf2f234809f5f2e7cc6f72d8d6f4b63643f96a3a20d44b78fbc",
        ),
    ] {
        let got = design::render(&design::seeds(count, first_mrn));
        assert_eq!(sha256(&got), want, "count {count}, first MRN {first_mrn}");
    }
}

#[test]
fn write_creates_the_directory_and_the_file() {
    let dir = std::env::temp_dir().join(format!("patient-bank-seeds-{}", std::process::id()));
    let path = dir.join("nested").join("seeds.json");
    let seeds = design::seeds(1, 6);
    design::write(&path, &seeds).unwrap();
    let written = std::fs::read_to_string(&path).unwrap();
    std::fs::remove_dir_all(&dir).unwrap();
    assert_eq!(written, design::render(&seeds));
    assert_eq!(
        sha256(&written),
        "74a126929792a6b5ff688e2ca6a6455b51b22f46aeaf10e9626df584b12043a7"
    );
}
