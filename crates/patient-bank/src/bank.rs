//! Render accepted ledgers into the patient bank and its manifest.
//!
//! ```text
//! bank/patients/<MRN>/<Family>.ndjson   one chart per patient, as of 2026-08-27
//! bank/calendar/Appointment.ndjson      the practice's visits from the bank day on:
//!                                       today's, and those booked after it
//! bank/manifest.json                    every patient with its unit and counts
//! bank/ledgers/<unit>.json              the authored source each chart was rendered from
//! ```
//!
//! The build reads every input before writing, writes the bank into a fresh
//! directory beside the target and swaps it in, and refuses an output directory
//! that overlaps its inputs. It needs no network: each ledger carries the codes
//! verified when it was accepted.

use std::collections::{BTreeMap, HashSet};
use std::fs;
use std::path::{Path, PathBuf};

use serde::Serialize;

use crate::catalog::Catalog;
use crate::fhir::FAMILIES;
use crate::ledger::Ledger;
use crate::render::Renderer;
use crate::seeds::Seeds;
use crate::{Result, calendar, paths, time};

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
    families: [&'static str; 11],
    patients: Vec<ManifestPatient>,
    totals: BTreeMap<&'static str, usize>,
    appointments: BTreeMap<&'static str, usize>,
}

#[derive(Serialize)]
struct ManifestPatient {
    mrn: String,
    name: String,
    complexity: &'static str,
    unit: String,
    counts: BTreeMap<&'static str, usize>,
}

type Lines = BTreeMap<&'static str, Vec<String>>;

/// Build the bank into `out`, replacing it, and print its summary.
pub fn run(patients: &Path, out: &Path) -> Result<()> {
    refuse_overlap(out, &[patients])?;
    let catalog = Catalog::load(&paths::catalog())?;
    let seeds = Seeds::read(patients)?;

    // Read every input before writing anything.
    let mut accepted: Vec<(&crate::seeds::Unit, Vec<u8>, Ledger)> = Vec::new();
    for unit in &seeds.units {
        let path = patients.join("units").join(&unit.unit).join("ledger.json");
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

    let mut manifest = Manifest {
        as_of: time::AS_OF,
        format: "FHIR R4 4.0.1 NDJSON, one directory per patient",
        families: FAMILIES,
        patients: Vec::new(),
        totals: FAMILIES.iter().map(|f| (*f, 0)).collect(),
        appointments: BTreeMap::new(),
    };
    let mut charts: Vec<(String, Lines)> = Vec::new();
    let mut appointments = Vec::new();
    for (unit, _, ledger) in &accepted {
        let catalog = catalog.with_terms(&ledger.terms);
        let r = Renderer {
            catalog: &catalog,
            terms: &ledger.terms,
        };
        for seed in &unit.members {
            let patient = ledger.patient(&seed.mrn).expect("checked above");
            let (chart, _) = r.render(seed, patient, time::as_of());
            for a in calendar::visits(seed, patient) {
                *manifest.appointments.entry(a.status).or_default() += 1;
                appointments.push(a);
            }
            let lines: Lines = chart.lines().into_iter().collect();
            for (fam, rows) in &lines {
                *manifest.totals.get_mut(fam).expect("family") += rows.len();
            }
            manifest.patients.push(ManifestPatient {
                mrn: seed.mrn.clone(),
                name: format!("{} {}", seed.given, seed.family),
                complexity: seed.complexity.as_str(),
                unit: unit.unit.clone(),
                counts: lines.iter().map(|(f, r)| (*f, r.len())).collect(),
            });
            charts.push((seed.mrn.clone(), lines));
        }
    }
    let mrns: HashSet<&str> = manifest.patients.iter().map(|p| p.mrn.as_str()).collect();
    if mrns.len() != manifest.patients.len() {
        return Err("an MRN appears twice in the bank".into());
    }

    appointments.sort_by(|a, b| (&a.start, &a.id).cmp(&(&b.start, &b.id)));
    let calendar: Vec<String> = appointments
        .iter()
        .map(|a| serde_json::to_string(a).expect("FHIR types serialize"))
        .collect();

    replace_dir(out, |dir| {
        fs::create_dir(dir.join("patients"))?;
        fs::create_dir(dir.join("ledgers"))?;
        fs::create_dir(dir.join("calendar"))?;
        write_lines(&dir.join("calendar").join("Appointment.ndjson"), &calendar)?;
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
        fs::write(
            dir.join("manifest.json"),
            serde_json::to_string_pretty(&manifest)? + "\n",
        )?;
        Ok(())
    })?;
    let summary = serde_json::json!({
        "patients": manifest.patients.len(),
        "totals": manifest.totals,
        "appointments": manifest.appointments,
    });
    println!("{}", serde_json::to_string_pretty(&summary)?);
    Ok(())
}
