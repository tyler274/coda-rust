//! Render accepted ledgers into the patient bank, its panels and manifest.
//!
//! ```text
//! bank/patients/<MRN>/<Family>.ndjson   one chart per patient, as of 2026-08-27
//! bank/panels/panel-<N>.json            nested panels (100 ⊂ 300 ⊂ 500 ⊂ 1000)
//! bank/manifest.json                    every patient with lineage, counts and hashes
//! bank/ledgers/<unit>.json              the authored source each chart was rendered from
//! ```
//!
//! The build reads every input once (a ledger is parsed, archived and hashed from
//! the same bytes), writes the bank into a fresh directory beside the target and
//! swaps it in, and refuses an output directory that overlaps its inputs. It needs
//! no network: each ledger carries the codes verified when it was accepted.

use std::collections::{BTreeMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};

use serde::Serialize;
use serde_json::Value;
use sha2::{Digest, Sha256};

use crate::catalog::Catalog;
use crate::fhir::FAMILIES;
use crate::ledger::Ledger;
use crate::render::{Receipt, Renderer};
use crate::rng::Rng;
use crate::seeds::{Complexity, Seeds};
use crate::{Result, paths, time};

pub const PANEL_SIZES: [usize; 4] = [100, 300, 500, 1000];
pub const PANEL_SEED: u128 = 20261024;

pub fn sha256_hex(bytes: &[u8]) -> String {
    Sha256::digest(bytes)
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect()
}

fn absolute(p: &Path) -> PathBuf {
    let p = if p.is_absolute() {
        p.to_path_buf()
    } else {
        std::env::current_dir()
            .map(|d| d.join(p))
            .unwrap_or_else(|_| p.to_path_buf())
    };
    p.canonicalize().unwrap_or(p)
}

/// Refuse an output directory that is, contains or sits inside one of the inputs.
pub fn refuse_overlap(out: &Path, inputs: &[&Path]) -> Result<()> {
    let out = absolute(out);
    for input in inputs {
        let input = absolute(input);
        if input.starts_with(&out) || out.starts_with(&input) {
            return Err(format!(
                "output {} overlaps input {}; choose a separate directory",
                out.display(),
                input.display()
            )
            .into());
        }
    }
    Ok(())
}

/// Fill a fresh directory beside `out` with `fill`, then put it in `out`'s place.
pub fn replace_dir(out: &Path, fill: impl FnOnce(&Path) -> Result<()>) -> Result<()> {
    let name = out
        .file_name()
        .ok_or("output has no directory name")?
        .to_string_lossy();
    let fresh = out.with_file_name(format!(".{name}.{}.new", std::process::id()));
    let old = out.with_file_name(format!(".{name}.{}.old", std::process::id()));
    if fresh.exists() {
        fs::remove_dir_all(&fresh)?;
    }
    fs::create_dir_all(&fresh)?;
    if let Err(e) = fill(&fresh) {
        let _ = fs::remove_dir_all(&fresh);
        return Err(e);
    }
    if out.exists() {
        fs::rename(out, &old)?;
    }
    fs::rename(&fresh, out)?;
    if old.exists() {
        fs::remove_dir_all(&old)?;
    }
    Ok(())
}

pub fn write_lines(path: &Path, lines: &[String]) -> Result<()> {
    let mut text = String::new();
    for l in lines {
        text.push_str(l);
        text.push('\n');
    }
    fs::write(path, text)?;
    Ok(())
}

#[derive(Serialize)]
struct Manifest {
    as_of: &'static str,
    format: &'static str,
    families: [&'static str; 8],
    patients: Vec<ManifestPatient>,
    units: Vec<ManifestUnit>,
    totals: BTreeMap<&'static str, usize>,
    coding: BTreeMap<String, usize>,
    patient_count: usize,
    panels: BTreeMap<String, usize>,
}

#[derive(Serialize)]
struct ManifestPatient {
    mrn: String,
    name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    sex: Option<&'static str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    birth_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    town: Option<String>,
    complexity: String,
    unit: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    family_id: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    relationships: Vec<crate::seeds::Relationship>,
    lineage: BTreeMap<&'static str, String>,
    counts: BTreeMap<&'static str, usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    render_receipt: Option<ReceiptSummary>,
    sha256: BTreeMap<&'static str, String>,
}

#[derive(Serialize)]
struct ReceiptSummary {
    filled_panel_values: usize,
    derived_values: usize,
    uncoded: Vec<String>,
    skipped: Vec<String>,
}

impl From<&Receipt> for ReceiptSummary {
    fn from(r: &Receipt) -> ReceiptSummary {
        ReceiptSummary {
            filled_panel_values: r.filled.len(),
            derived_values: r.derived.len(),
            uncoded: r.uncoded.clone(),
            skipped: r.skipped.clone(),
        }
    }
}

#[derive(Serialize)]
struct ManifestUnit {
    unit: String,
    kind: String,
    members: Vec<String>,
    model: String,
    ledger_sha256: String,
}

#[derive(Serialize)]
struct PanelFile<'a> {
    size: usize,
    requested: usize,
    nested_in: Vec<usize>,
    members: &'a [String],
}

/// A panel candidate: one authoring unit's patients and its most complex tier.
pub struct Group {
    pub mrns: Vec<String>,
    pub tier: Complexity,
}

/// Nested panels that keep authoring units (families) together.
///
/// Units are shuffled with a fixed seed, then stratified by complexity so each
/// prefix has the bank's mix; a family enters a panel whole.
pub fn panels(groups: &[Group]) -> Vec<(usize, Vec<String>)> {
    let mut order: Vec<&Group> = groups.iter().collect();
    Rng::new(PANEL_SEED).shuffle(&mut order);
    let tiers = [Complexity::Light, Complexity::Moderate, Complexity::Complex];
    let by: Vec<Vec<&Group>> = tiers
        .iter()
        .map(|t| order.iter().copied().filter(|g| g.tier == *t).collect())
        .collect();
    let total: usize = by.iter().map(Vec::len).sum();
    let share: Vec<f64> = by
        .iter()
        .map(|v| v.len() as f64 / total.max(1) as f64)
        .collect();
    let mut taken = [0usize; 3];
    let mut sequence: Vec<&Group> = Vec::new();
    while (0..3).any(|k| taken[k] < by[k].len()) {
        // the tier furthest below its share so far
        let n = sequence.len().max(1) as f64;
        let k = (0..3)
            .filter(|k| taken[*k] < by[*k].len())
            .min_by(|a, b| {
                (taken[*a] as f64 / n - share[*a]).total_cmp(&(taken[*b] as f64 / n - share[*b]))
            })
            .expect("a tier with units left");
        sequence.push(by[k][taken[k]]);
        taken[k] += 1;
    }
    PANEL_SIZES
        .iter()
        .map(|&size| {
            let mut members: Vec<String> = Vec::new();
            for g in sequence.iter().filter(|g| !g.mrns.is_empty()) {
                if members.len() + g.mrns.len() <= size {
                    members.extend(g.mrns.iter().cloned()); // a family enters whole or waits
                }
                if members.len() == size {
                    break;
                }
            }
            (size, members)
        })
        .collect()
}

type Lines = BTreeMap<&'static str, Vec<String>>;

/// The Roger fixture's NDJSON lines, by patient and family, verbatim.
fn roger_charts(roger: &Path) -> Result<BTreeMap<String, Lines>> {
    let mut charts: BTreeMap<String, Lines> = BTreeMap::new();
    for fam in FAMILIES {
        let path = roger.join(format!("{fam}.ndjson"));
        for line in fs::read_to_string(&path)?
            .lines()
            .filter(|l| !l.trim().is_empty())
        {
            let r: Value =
                serde_json::from_str(line).map_err(|e| format!("{}: {e}", path.display()))?;
            let id = if fam == "Patient" {
                r["id"].as_str().map(String::from)
            } else {
                let at = if r["subject"].is_object() {
                    &r["subject"]
                } else {
                    &r["patient"]
                };
                at["reference"]
                    .as_str()
                    .and_then(|s| s.split('/').nth(1))
                    .map(String::from)
            };
            let id = id.ok_or_else(|| format!("{fam}: a row without a patient"))?;
            let chart = charts
                .entry(id)
                .or_insert_with(|| FAMILIES.iter().map(|f| (*f, Vec::new())).collect());
            chart.get_mut(fam).expect("family").push(line.to_string());
        }
    }
    Ok(charts)
}

fn family_hashes(lines: &Lines) -> BTreeMap<&'static str, String> {
    lines
        .iter()
        .map(|(fam, rows)| {
            let text: String = rows.iter().map(|l| format!("{l}\n")).collect();
            (*fam, sha256_hex(text.as_bytes()))
        })
        .collect()
}

/// Build the bank into `out`, replacing it, and print its summary.
pub fn run(seeds_path: &Path, ledgers: &Path, out: &Path, roger: Option<&Path>) -> Result<()> {
    let mut inputs = vec![seeds_path, ledgers];
    inputs.extend(roger);
    refuse_overlap(out, &inputs)?;
    if !ledgers.is_dir() {
        return Err(format!("{} is not a directory of ledgers", ledgers.display()).into());
    }
    let catalog = Catalog::load(&paths::catalog())?;
    let seeds = Seeds::read(seeds_path)?;

    // Read every input before writing anything.
    let mut accepted: Vec<(&crate::seeds::Unit, Vec<u8>, Ledger)> = Vec::new();
    for unit in &seeds.units {
        let path = ledgers.join(format!("{}.json", unit.unit));
        let bytes = match fs::read(&path) {
            Ok(b) => b,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => continue,
            Err(e) => return Err(e.into()),
        };
        let ledger: Ledger =
            serde_json::from_slice(&bytes).map_err(|e| format!("{}: {e}", path.display()))?;
        for m in &unit.members {
            if ledger.patient(&m.mrn).is_none() {
                return Err(format!("{}: no chart for {}", path.display(), m.mrn).into());
            }
        }
        accepted.push((unit, bytes, ledger));
    }
    let roger = roger.map(roger_charts).transpose()?;

    let mut manifest = Manifest {
        as_of: time::AS_OF,
        format: "FHIR R4 4.0.1 NDJSON, one directory per patient",
        families: FAMILIES,
        patients: Vec::new(),
        units: Vec::new(),
        totals: FAMILIES.iter().map(|f| (*f, 0)).collect(),
        coding: BTreeMap::new(),
        patient_count: 0,
        panels: BTreeMap::new(),
    };
    let mut groups: Vec<Group> = Vec::new();
    let mut charts: Vec<(String, Lines)> = Vec::new();

    for (mrn, lines) in roger.into_iter().flatten() {
        let name = serde_json::from_str::<Value>(&lines["Patient"][0])
            .map(|p| {
                format!(
                    "{} {}",
                    p["name"][0]["given"][0].as_str().unwrap_or(""),
                    p["name"][0]["family"].as_str().unwrap_or("")
                )
            })
            .unwrap_or_default();
        for (fam, rows) in &lines {
            *manifest.totals.get_mut(fam).expect("family") += rows.len();
        }
        manifest.patients.push(ManifestPatient {
            mrn: mrn.clone(),
            name,
            sex: None,
            birth_date: None,
            town: None,
            complexity: "fixture".into(),
            unit: "roger".into(),
            family_id: None,
            relationships: Vec::new(),
            lineage: [
                (
                    "source",
                    "coda-agents docs/local-product-fixtures (Roger practice)",
                ),
                (
                    "authoring",
                    "authored through coda-gym's chart flow; copied byte-for-byte",
                ),
                ("clinical_review", "not physician reviewed"),
            ]
            .into_iter()
            .map(|(k, v)| (k, v.to_string()))
            .collect(),
            counts: lines.iter().map(|(f, r)| (*f, r.len())).collect(),
            render_receipt: None,
            sha256: family_hashes(&lines),
        });
        groups.push(Group {
            mrns: vec![mrn.clone()],
            tier: Complexity::Complex,
        });
        charts.push((mrn, lines));
    }

    for (unit, bytes, ledger) in &accepted {
        let r = Renderer {
            catalog: &catalog,
            terms: &ledger.terms,
        };
        manifest.units.push(ManifestUnit {
            unit: unit.unit.clone(),
            kind: unit.kind.clone(),
            members: unit.members.iter().map(|m| m.mrn.clone()).collect(),
            model: ledger.model.clone(),
            ledger_sha256: sha256_hex(bytes),
        });
        groups.push(Group {
            mrns: unit.members.iter().map(|m| m.mrn.clone()).collect(),
            tier: unit
                .members
                .iter()
                .map(|m| m.complexity)
                .max()
                .expect("a unit has members"),
        });
        for seed in &unit.members {
            let (chart, receipt) = r.render(
                seed,
                ledger.patient(&seed.mrn).expect("checked above"),
                time::as_of(),
            );
            let lines: Lines = chart.lines().into_iter().collect();
            for (fam, rows) in &lines {
                *manifest.totals.get_mut(fam).expect("family") += rows.len();
            }
            let coded: [(&str, Vec<bool>); 6] = [
                (
                    "Condition",
                    chart
                        .conditions
                        .iter()
                        .map(|x| !x.code.coding.is_empty())
                        .collect(),
                ),
                (
                    "MedicationRequest",
                    chart
                        .medications
                        .iter()
                        .map(|x| !x.medication_codeable_concept.coding.is_empty())
                        .collect(),
                ),
                (
                    "Observation",
                    chart
                        .observations
                        .iter()
                        .map(|x| !x.code.coding.is_empty())
                        .collect(),
                ),
                (
                    "DiagnosticReport",
                    chart
                        .reports
                        .iter()
                        .map(|x| !x.code.coding.is_empty())
                        .collect(),
                ),
                (
                    "Procedure",
                    chart
                        .procedures
                        .iter()
                        .map(|x| !x.code.coding.is_empty())
                        .collect(),
                ),
                (
                    "AllergyIntolerance",
                    chart
                        .allergies
                        .iter()
                        .map(|x| !x.code.coding.is_empty())
                        .collect(),
                ),
            ];
            for (fam, flags) in coded {
                for c in flags {
                    *manifest
                        .coding
                        .entry(format!("{fam}:{}", if c { "coded" } else { "text" }))
                        .or_default() += 1;
                }
            }
            manifest.patients.push(ManifestPatient {
                mrn: seed.mrn.clone(),
                name: format!("{} {}", seed.given, seed.family),
                sex: Some(seed.sex.as_str()),
                birth_date: Some(seed.birth_date.0.to_string()),
                town: Some(seed.town.clone()),
                complexity: seed.complexity.as_str().into(),
                unit: unit.unit.clone(),
                family_id: seed.family_id.clone(),
                relationships: seed.relationships.clone(),
                lineage: [
                    (
                        "source",
                        "authored for the Coda synthetic patient bank".to_string(),
                    ),
                    (
                        "seed",
                        format!(
                            "crates/patient-bank/data/seeds.json (designed with seed {})",
                            seeds.seed
                        ),
                    ),
                    ("author_model", ledger.model.clone()),
                    (
                        "authoring_guide",
                        "crates/patient-bank/data/authoring-guide.md".to_string(),
                    ),
                    (
                        "clinical_review",
                        "not physician reviewed; generated, not clinically validated".to_string(),
                    ),
                ]
                .into_iter()
                .collect(),
                counts: lines.iter().map(|(f, r)| (*f, r.len())).collect(),
                render_receipt: Some(ReceiptSummary::from(&receipt)),
                sha256: family_hashes(&lines),
            });
            charts.push((seed.mrn.clone(), lines));
        }
    }

    let pan = panels(&groups);
    for (size, members) in &pan {
        manifest.panels.insert(size.to_string(), members.len());
    }
    manifest.patient_count = manifest.patients.len();
    let mrns: HashSet<&str> = manifest.patients.iter().map(|p| p.mrn.as_str()).collect();
    if mrns.len() != manifest.patients.len() {
        return Err("an MRN appears twice in the bank".into());
    }

    replace_dir(out, |dir| {
        fs::create_dir(dir.join("patients"))?;
        fs::create_dir(dir.join("ledgers"))?;
        fs::create_dir(dir.join("panels"))?;
        for (mrn, lines) in &charts {
            let d = dir.join("patients").join(mrn);
            fs::create_dir(&d)?;
            for (fam, rows) in lines {
                write_lines(&d.join(format!("{fam}.ndjson")), rows)?;
            }
        }
        for (unit, bytes, _) in &accepted {
            fs::write(
                dir.join("ledgers").join(format!("{}.json", unit.unit)),
                bytes,
            )?;
        }
        for (size, members) in &pan {
            let doc = PanelFile {
                size: members.len(),
                requested: *size,
                nested_in: PANEL_SIZES.iter().copied().filter(|s| s > size).collect(),
                members,
            };
            fs::write(
                dir.join("panels").join(format!("panel-{size}.json")),
                serde_json::to_string_pretty(&doc)? + "\n",
            )?;
        }
        fs::write(
            dir.join("manifest.json"),
            serde_json::to_string_pretty(&manifest)? + "\n",
        )?;
        Ok(())
    })?;
    let summary = serde_json::json!({
        "patients": manifest.patient_count,
        "totals": manifest.totals,
        "panels": manifest.panels,
        "coding": manifest.coding,
    });
    println!("{}", serde_json::to_string_pretty(&summary)?);
    Ok(())
}
