//! Render authored ledgers into the patient bank, its panels and manifest.
//!
//! Output:
//!   bank/patients/<MRN>/<Family>.ndjson   one chart per patient, as of 2026-08-27
//!   bank/panels/panel-<N>.json            nested panels (100 ⊂ 300 ⊂ 500 ⊂ 1000)
//!   bank/manifest.json                    every patient with lineage, counts and hashes
//!   bank/ledgers/<unit>.json              the authored source each chart was rendered from
//!
//! `materialize` turns a panel (optionally as of an earlier instant) into one
//! multi-patient NDJSON directory in the Roger fixture layout.
//!
//! A port of `build_bank.py`; the bank it writes is byte-identical.

use std::collections::{BTreeMap, HashSet};
use std::fs;
use std::io::{BufWriter, Write as _};
use std::path::Path;

use sha2::{Digest, Sha256};

use crate::catalog::Result;
use crate::json::{self, Json, Layout};
use crate::render::{self, FAMILIES, Renderer};
use crate::rng::PyRandom;
use crate::{obj, paths, terms};

pub const AS_OF: &str = "2026-08-27T12:00";
pub const PANEL_SIZES: [usize; 4] = [100, 300, 500, 1000];
pub const PANEL_SEED: u128 = 20261024;
const TIERS: [&str; 3] = ["light", "moderate", "complex"];

/// A `collections.Counter`: keys in first-insertion order, zero counts kept.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Counter(pub Vec<(String, i64)>);

impl Counter {
    pub fn add(&mut self, key: &str, n: i64) {
        match self.0.iter_mut().find(|(k, _)| k == key) {
            Some(slot) => slot.1 += n,
            None => self.0.push((key.to_string(), n)),
        }
    }

    pub fn set(&mut self, key: &str, n: i64) {
        match self.0.iter_mut().find(|(k, _)| k == key) {
            Some(slot) => slot.1 = n,
            None => self.0.push((key.to_string(), n)),
        }
    }

    /// `Counter.update(mapping)`
    pub fn update(&mut self, other: &Counter) {
        for (k, n) in &other.0 {
            self.add(k, *n);
        }
    }

    pub fn to_json(&self) -> Json {
        Json::Obj(
            self.0
                .iter()
                .map(|(k, n)| (k.clone(), Json::Int(*n)))
                .collect(),
        )
    }
}

/// One resource per line, as `json.dumps(r, ensure_ascii=False, separators=(",", ":"))`.
pub fn write_ndjson(path: &Path, rows: &[Json]) -> Result<()> {
    let mut f = BufWriter::new(fs::File::create(path)?);
    for r in rows {
        f.write_all(json::dumps(r, Layout::Compact, false).as_bytes())?;
        f.write_all(b"\n")?;
    }
    f.flush()?;
    Ok(())
}

/// `path.read_text().splitlines()`, each line parsed.
pub fn read_ndjson(path: &Path) -> Result<Vec<Json>> {
    let text = fs::read_to_string(path)?;
    let mut rows = Vec::new();
    for line in splitlines(&text) {
        rows.push(json::parse(line).map_err(|e| format!("{}: {e}", path.display()))?);
    }
    Ok(rows)
}

/// Python's `str.splitlines()`: every Unicode line boundary, `\r\n` as one.
pub fn splitlines(text: &str) -> Vec<&str> {
    let mut lines = Vec::new();
    let mut start = 0;
    let mut chars = text.char_indices().peekable();
    while let Some((i, c)) = chars.next() {
        let end = match c {
            '\r' if chars.peek().is_some_and(|(_, n)| *n == '\n') => {
                chars.next();
                i + 2
            }
            '\n' | '\r' | '\u{b}' | '\u{c}' | '\u{1c}' | '\u{1d}' | '\u{1e}' | '\u{85}'
            | '\u{2028}' | '\u{2029}' => i + c.len_utf8(),
            _ => continue,
        };
        lines.push(&text[start..i]);
        start = end;
    }
    if start < text.len() {
        lines.push(&text[start..]);
    }
    lines
}

pub fn sha256_file(path: &Path) -> Result<String> {
    Ok(Sha256::digest(fs::read(path)?)
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect())
}

/// Python `str()` of a JSON scalar, as f-strings write it.
pub(crate) fn py_str(v: &Json) -> String {
    match v {
        Json::Null => "None".into(),
        Json::Bool(b) => if *b { "True" } else { "False" }.into(),
        Json::Int(i) => i.to_string(),
        Json::Float(f) if f.is_nan() => "nan".into(),
        Json::Float(f) if f.is_infinite() => if *f > 0.0 { "inf" } else { "-inf" }.into(),
        Json::Float(f) => json::float_repr(*f),
        Json::Str(s) => s.clone(),
        other => json::dumps(other, Layout::Default, false),
    }
}

type Charts = BTreeMap<String, Vec<Vec<Json>>>;

/// The Roger fixture's resources by patient, each with every family.
fn roger_patients(roger: &Path) -> Result<Charts> {
    let mut charts: Charts = BTreeMap::new();
    for (slot, fam) in FAMILIES.iter().enumerate() {
        for r in read_ndjson(&roger.join(format!("{fam}.ndjson")))? {
            let id = if *fam == "Patient" {
                py_str(r.get("id"))
            } else {
                let at = if r.get("subject").truthy() {
                    r.get("subject")
                } else {
                    r.get("patient")
                };
                let reference = at.get("reference").as_str().unwrap_or("");
                reference
                    .split('/')
                    .nth(1)
                    .ok_or_else(|| format!("{fam}: reference {reference:?} has no id"))?
                    .to_string()
            };
            charts
                .entry(id)
                .or_insert_with(|| vec![Vec::new(); FAMILIES.len()])[slot]
                .push(r);
        }
    }
    Ok(charts)
}

/// Nested panels that keep authoring units (families) together.
///
/// Units are shuffled with a fixed seed, then stratified by complexity so each
/// prefix has the bank's mix; a family enters a panel whole.
pub fn panels(units: &[Json], mrns: &HashSet<String>) -> Vec<(usize, Vec<String>)> {
    let mut order: Vec<&Json> = units.iter().collect();
    PyRandom::new(PANEL_SEED).shuffle(&mut order);
    let mut by: [Vec<&Json>; 3] = Default::default();
    for u in order {
        let tier = u
            .get("members")
            .as_array()
            .iter()
            .map(|m| {
                let c = m.get("complexity").as_str().unwrap_or("");
                TIERS
                    .iter()
                    .position(|t| *t == c)
                    .unwrap_or_else(|| panic!("ValueError: {c:?} is not a complexity"))
            })
            .reduce(|best, t| if t > best { t } else { best })
            .expect("ValueError: max() arg is an empty sequence");
        by[tier].push(u);
    }
    let total: usize = by.iter().map(Vec::len).sum();
    let share: Vec<f64> = by.iter().map(|v| v.len() as f64 / total as f64).collect();
    let mut taken = [0usize; 3];
    let mut sequence: Vec<&Json> = Vec::new();
    while (0..3).any(|k| taken[k] < by[k].len()) {
        // pick the tier furthest below its share so far
        let n = sequence.len().max(1) as f64;
        let mut pick: Option<(usize, f64)> = None;
        for k in (0..3).filter(|k| taken[*k] < by[*k].len()) {
            let key = taken[k] as f64 / n - share[k];
            if pick.is_none_or(|(_, best)| key < best) {
                pick = Some((k, key));
            }
        }
        let (k, _) = pick.expect("a tier with units left");
        sequence.push(by[k][taken[k]]);
        taken[k] += 1;
    }
    let flat: Vec<Vec<String>> = sequence
        .iter()
        .map(|u| {
            u.get("members")
                .as_array()
                .iter()
                .map(|m| py_str(m.get("mrn")))
                .filter(|mrn| mrns.contains(mrn))
                .collect::<Vec<_>>()
        })
        .filter(|g| !g.is_empty())
        .collect();
    let mut result = Vec::new();
    for size in PANEL_SIZES {
        let mut members: Vec<String> = Vec::new();
        for g in &flat {
            if members.len() + g.len() <= size {
                // a family enters whole or waits
                members.extend(g.iter().cloned());
            }
            if members.len() == size {
                break;
            }
        }
        result.push((size, members));
    }
    for pair in result.windows(2) {
        let large: HashSet<&String> = pair[1].1.iter().collect();
        assert!(
            pair[0].1.iter().all(|m| large.contains(m)),
            "panel-{} not nested in panel-{}",
            pair[0].0,
            pair[1].0
        );
    }
    result
}

fn family_counts(rows: &[Vec<Json>]) -> Json {
    Json::Obj(
        FAMILIES
            .iter()
            .zip(rows)
            .map(|(f, v)| (f.to_string(), Json::Int(v.len() as i64)))
            .collect(),
    )
}

fn family_hashes(dir: &Path) -> Result<Json> {
    let mut out = Vec::new();
    for f in FAMILIES {
        out.push((
            f.to_string(),
            Json::from(sha256_file(&dir.join(format!("{f}.ndjson")))?),
        ));
    }
    Ok(Json::Obj(out))
}

/// `build_bank.py`: build the bank from this checkout's catalog and the verified
/// extra terms of `ledgers`, and print its counts.
pub fn run(seeds: &Path, ledgers: &Path, out: &Path, roger: Option<&Path>) -> Result<()> {
    let catalog = render::load_catalog(&paths::catalog());
    let extra = terms::resolve_extras(ledgers, &catalog, false)?;
    let summary = build(&catalog, &extra, seeds, ledgers, out, roger)?;
    println!("{}", json::dumps(&summary, Layout::Indent(1), true));
    Ok(())
}

/// Build the bank in `out` (replacing it); the result is the printed summary.
pub fn build(
    catalog: &Json,
    extra: &Json,
    seeds: &Path,
    ledgers: &Path,
    out: &Path,
    roger: Option<&Path>,
) -> Result<Json> {
    let seeds = json::parse(&fs::read_to_string(seeds)?)?;
    let units = seeds.get("units").as_array();
    let r = Renderer::new(catalog, extra);
    if out.exists() {
        fs::remove_dir_all(out)?;
    }
    fs::create_dir_all(out.join("patients"))?;
    fs::create_dir(out.join("ledgers"))?;
    fs::create_dir(out.join("panels"))?;
    let mut patients: Vec<Json> = Vec::new();
    let mut manifest_units: Vec<Json> = Vec::new();
    let mut totals = Counter::default();
    let mut coding: BTreeMap<(&str, &str), i64> = BTreeMap::new();
    let mut all_units: Vec<Json> = Vec::new();
    if let Some(roger) = roger {
        for (mrn, chart) in roger_patients(roger)? {
            let d = out.join("patients").join(&mrn);
            fs::create_dir(&d)?;
            for (fam, rows) in FAMILIES.iter().zip(&chart) {
                write_ndjson(&d.join(format!("{fam}.ndjson")), rows)?;
                totals.add(fam, rows.len() as i64);
            }
            let name = chart[0][0].get("name").as_array()[0].clone();
            patients.push(obj![
                ("mrn", mrn.as_str()),
                (
                    "name",
                    format!(
                        "{} {}",
                        py_str(&name.get("given").as_array()[0]),
                        py_str(name.get("family"))
                    )
                ),
                (
                    "lineage",
                    obj![
                        (
                            "source",
                            "coda-agents docs/local-product-fixtures (Roger practice)"
                        ),
                        (
                            "authoring",
                            "authored through coda-gym's chart flow; copied byte-for-byte"
                        ),
                        ("clinical_review", "not physician reviewed"),
                    ]
                ),
                ("unit", "roger"),
                ("complexity", "fixture"),
                ("counts", family_counts(&chart)),
                ("sha256", family_hashes(&d)?),
            ]);
            all_units.push(obj![
                ("unit", "roger"),
                ("kind", "individual"),
                (
                    "members",
                    vec![obj![("mrn", mrn.as_str()), ("complexity", "complex")]]
                ),
            ]);
        }
    }
    for unit in units {
        let uid = py_str(unit.get("unit"));
        let lp = ledgers.join(format!("{uid}.json"));
        if !lp.exists() {
            continue;
        }
        let led = json::parse(&fs::read_to_string(&lp)?)?;
        fs::copy(&lp, out.join("ledgers").join(format!("{uid}.json")))?;
        let led_patients = led.get("patients").as_array();
        let by_mrn = |mrn: &Json| {
            led_patients
                .iter()
                .rev() // a dict comprehension keeps the last patient with an MRN
                .find(|p| p.get("mrn") == mrn)
                .ok_or_else(|| format!("KeyError: {} not in {uid}", py_str(mrn)))
        };
        let members = unit.get("members").as_array();
        manifest_units.push(obj![
            ("unit", unit.get("unit").clone()),
            ("kind", unit.get("kind").clone()),
            (
                "members",
                members
                    .iter()
                    .map(|m| m.get("mrn").clone())
                    .collect::<Vec<_>>()
            ),
            ("model", led.get("model").clone()),
            ("ledger_sha256", sha256_file(&lp)?),
        ]);
        all_units.push(unit.clone());
        for seed in members {
            let (chart, receipt) = r.render(seed, by_mrn(seed.get("mrn"))?, AS_OF);
            let mrn = py_str(seed.get("mrn"));
            let d = out.join("patients").join(&mrn);
            fs::create_dir(&d)?;
            let mut counts = Vec::new();
            for (fam, rows) in chart.iter() {
                write_ndjson(&d.join(format!("{fam}.ndjson")), rows)?;
                totals.add(fam, rows.len() as i64);
                counts.push((fam.to_string(), Json::Int(rows.len() as i64)));
                if matches!(
                    fam,
                    "Condition"
                        | "MedicationRequest"
                        | "Observation"
                        | "Procedure"
                        | "AllergyIntolerance"
                        | "DiagnosticReport"
                ) {
                    for row in rows {
                        let cc = if row.get("code").truthy() {
                            row.get("code")
                        } else {
                            row.get("medicationCodeableConcept")
                        };
                        let kind = if cc.truthy() && cc.get("coding").truthy() {
                            "coded"
                        } else {
                            "text"
                        };
                        *coding.entry((fam, kind)).or_default() += 1;
                    }
                }
            }
            let strings =
                |v: &[String]| Json::Arr(v.iter().map(|s| Json::from(s.as_str())).collect());
            patients.push(obj![
                ("mrn", seed.get("mrn").clone()),
                (
                    "name",
                    format!(
                        "{} {}",
                        py_str(seed.get("given")),
                        py_str(seed.get("family"))
                    )
                ),
                ("sex", seed.get("sex").clone()),
                ("birth_date", seed.get("birth_date").clone()),
                ("town", seed.get("town").clone()),
                ("complexity", seed.get("complexity").clone()),
                ("unit", unit.get("unit").clone()),
                ("family_id", seed.get("family_id").clone()),
                (
                    "relationships",
                    if seed.contains("relationships") {
                        seed.get("relationships").clone()
                    } else {
                        Json::Arr(Vec::new())
                    }
                ),
                (
                    "lineage",
                    obj![
                        ("source", "authored for the Coda synthetic patient bank"),
                        (
                            "seed",
                            "scripts/patient-bank/design_panel.py (seed 20261024)"
                        ),
                        ("author_model", led.get("model").clone()),
                        ("authoring_guide", "scripts/patient-bank/authoring-guide.md"),
                        (
                            "clinical_review",
                            "not physician reviewed; generated, not clinically validated"
                        ),
                    ]
                ),
                ("counts", Json::Obj(counts)),
                (
                    "render_receipt",
                    obj![
                        ("filled_panel_values", receipt.filled.len() as i64),
                        ("derived_values", receipt.derived.len() as i64),
                        ("uncoded", strings(&receipt.uncoded)),
                        ("skipped", strings(&receipt.skipped)),
                    ]
                ),
                ("sha256", family_hashes(&d)?),
            ]);
        }
    }
    let mrns: HashSet<String> = patients.iter().map(|p| py_str(p.get("mrn"))).collect();
    let pan = panels(&all_units, &mrns);
    for (size, members) in &pan {
        let nested_in: Vec<Json> = PANEL_SIZES
            .iter()
            .filter(|s| *s > size)
            .map(|s| Json::Int(*s as i64))
            .collect();
        let doc = obj![
            ("size", members.len() as i64),
            ("requested", *size as i64),
            ("nested_in", nested_in),
            (
                "members",
                members
                    .iter()
                    .map(|m| Json::from(m.as_str()))
                    .collect::<Vec<_>>()
            ),
        ];
        fs::write(
            out.join("panels").join(format!("panel-{size}.json")),
            json::dumps(&doc, Layout::Indent(1), true) + "\n",
        )?;
    }
    let coding = Json::Obj(
        coding
            .iter()
            .map(|((fam, kind), n)| (format!("{fam}:{kind}"), Json::Int(*n)))
            .collect(),
    );
    let panel_counts = Json::Obj(
        pan.iter()
            .map(|(size, m)| (size.to_string(), Json::Int(m.len() as i64)))
            .collect(),
    );
    let patient_count = patients.len() as i64;
    let manifest = obj![
        ("as_of", AS_OF),
        ("format", "FHIR R4 4.0.1 NDJSON, one directory per patient"),
        (
            "families",
            FAMILIES.iter().map(|f| Json::from(*f)).collect::<Vec<_>>()
        ),
        ("patients", patients),
        ("units", manifest_units),
        ("totals", totals.to_json()),
        ("coding", coding.clone()),
        ("patient_count", patient_count),
        ("panels", panel_counts.clone()),
    ];
    fs::write(
        out.join("manifest.json"),
        json::dumps(&manifest, Layout::Indent(1), false) + "\n",
    )?;
    Ok(obj![
        ("patients", patient_count),
        ("totals", totals.to_json()),
        ("panels", panel_counts),
        ("coding", coding),
    ])
}
