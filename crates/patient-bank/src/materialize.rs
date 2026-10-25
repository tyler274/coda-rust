//! Write one panel as a multi-patient NDJSON directory in the Roger fixture layout.
//!
//! At the bank time the per-patient charts are concatenated as they are. At an
//! earlier moment (Eastern wall clock) each chart is re-rendered from the ledger in
//! the bank, so the directory holds only what the practice knew then: later
//! records are absent, later stops and resolutions have not happened, and results
//! not yet back are `registered`. Roger fixture patients exist only at the bank time.

use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

use serde::Serialize;
use serde_json::Value;

use crate::bank::{refuse_overlap, replace_dir, write_lines};
use crate::catalog::Catalog;
use crate::fhir::FAMILIES;
use crate::ledger::{Ledger, Stamp};
use crate::render::Renderer;
use crate::seeds::Seeds;
use crate::timeline::Timeline;
use crate::{Result, paths, time};

#[derive(Serialize)]
struct Summary {
    panel: usize,
    as_of: String,
    patients: usize,
    not_included: Vec<String>,
    counts: BTreeMap<&'static str, usize>,
}

fn read_json(path: &Path) -> Result<Value> {
    Ok(serde_json::from_str(&fs::read_to_string(path)?)
        .map_err(|e| format!("{}: {e}", path.display()))?)
}

pub fn run(bank: &Path, seeds: &Path, panel: usize, out: &Path, as_of: &str) -> Result<()> {
    let at = Stamp::parse(as_of)
        .ok_or_else(|| format!("--as-of {as_of:?} is not a 'YYYY-MM-DDTHH:MM' time"))?;
    if at > time::as_of() {
        return Err(format!("--as-of {as_of} is after the bank time {}", time::AS_OF).into());
    }
    refuse_overlap(out, &[bank, seeds])?;
    let manifest = read_json(&bank.join("manifest.json"))?;
    let members = read_json(&bank.join("panels").join(format!("panel-{panel}.json")))?;
    let unit_of: BTreeMap<&str, &str> = manifest["patients"]
        .as_array()
        .into_iter()
        .flatten()
        .filter_map(|p| Some((p["mrn"].as_str()?, p["unit"].as_str()?)))
        .collect();
    let rerender = at != time::as_of();
    let seeds = Seeds::read(seeds)?;
    let catalog = Catalog::load(&paths::catalog())?;
    let mut ledgers: BTreeMap<String, Ledger> = BTreeMap::new();
    let mut rows: BTreeMap<&'static str, Vec<String>> =
        FAMILIES.iter().map(|f| (*f, Vec::new())).collect();
    let mut skipped = Vec::new();
    for mrn in members["members"]
        .as_array()
        .into_iter()
        .flatten()
        .filter_map(Value::as_str)
    {
        let unit = *unit_of
            .get(mrn)
            .ok_or_else(|| format!("{mrn} is in the panel but not the manifest"))?;
        if !rerender {
            for fam in FAMILIES {
                let text = fs::read_to_string(
                    bank.join("patients")
                        .join(mrn)
                        .join(format!("{fam}.ndjson")),
                )?;
                rows.get_mut(fam)
                    .expect("family")
                    .extend(text.lines().map(String::from));
            }
            continue;
        }
        if unit == "roger" {
            skipped.push(mrn.to_string());
            continue;
        }
        if !ledgers.contains_key(unit) {
            ledgers.insert(
                unit.to_string(),
                Ledger::read(&bank.join("ledgers").join(format!("{unit}.json")))?,
            );
        }
        let ledger = &ledgers[unit];
        let patient = ledger
            .patient(mrn)
            .ok_or_else(|| format!("{unit}: no chart for {mrn}"))?;
        if Timeline::new(patient)
            .first()
            .is_none_or(|first| first >= at)
        {
            skipped.push(mrn.to_string()); // not yet a patient of the practice
            continue;
        }
        let (_, seed) = seeds
            .member(mrn)
            .ok_or_else(|| format!("{mrn} has no seed"))?;
        let (chart, _) = Renderer {
            catalog: &catalog,
            terms: &ledger.terms,
        }
        .render(seed, patient, at);
        for (fam, lines) in chart.lines() {
            rows.get_mut(fam).expect("family").extend(lines);
        }
    }
    let summary = Summary {
        panel,
        as_of: as_of.to_string(),
        patients: rows["Patient"].len(),
        not_included: skipped,
        counts: rows.iter().map(|(f, r)| (*f, r.len())).collect(),
    };
    replace_dir(out, |dir| {
        for (fam, lines) in &rows {
            write_lines(&dir.join(format!("{fam}.ndjson")), lines)?;
        }
        fs::write(
            dir.join("materialized.json"),
            serde_json::to_string_pretty(&summary)? + "\n",
        )?;
        Ok(())
    })?;
    println!("{}", serde_json::to_string_pretty(&summary)?);
    Ok(())
}
