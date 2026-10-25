//! Write one panel as a multi-patient NDJSON directory in the Roger fixture layout.
//!
//! At the bank time the per-patient charts are concatenated as they are. With an
//! earlier `as_of` (Eastern wall clock), each chart is re-rendered from its ledger
//! by the same renderer, so the directory holds only what the practice knew at that
//! instant: later records are absent, later stops and resolutions have not
//! happened, and results not yet back are "registered". Roger fixture patients are
//! copied only at the bank time.
//!
//! A port of `materialize.py`.

use std::collections::HashMap;
use std::fs;
use std::path::Path;

use crate::bank::{AS_OF, py_str, read_ndjson, write_ndjson};
use crate::catalog::Result;
use crate::json::{self, Json, Layout};
use crate::render::{self, FAMILIES, Renderer};
use crate::{obj, paths, terms};

/// `materialize.py`: write the panel and print its summary. Re-rendering uses this
/// checkout's catalog and the verified extra terms of the bank's ledgers.
pub fn run(bank: &Path, seeds: &Path, panel: i64, out: &Path, as_of: &str) -> Result<()> {
    if as_of > AS_OF {
        return Err(format!("--as-of {as_of} is after the bank time {AS_OF}").into());
    }
    let renderer = if as_of != AS_OF {
        let catalog = render::load_catalog(&paths::catalog());
        let extra = terms::resolve_extras(&bank.join("ledgers"), &catalog, false)?;
        Some(Renderer::new(&catalog, &extra))
    } else {
        None
    };
    let summary = write(bank, seeds, panel, out, as_of, renderer.as_ref())?;
    println!("{}", json::dumps(&summary, Layout::Indent(1), true));
    Ok(())
}

/// Write panel `panel` of `bank` to `out` as of `as_of`, re-rendering with
/// `renderer` (required before the bank time); the result is `materialized.json`.
pub fn write(
    bank: &Path,
    seeds: &Path,
    panel: i64,
    out: &Path,
    as_of: &str,
    renderer: Option<&Renderer>,
) -> Result<Json> {
    let manifest = json::parse(&fs::read_to_string(bank.join("manifest.json"))?)?;
    let panel_doc = json::parse(&fs::read_to_string(
        bank.join("panels").join(format!("panel-{panel}.json")),
    )?)?;
    let by_mrn: HashMap<String, &Json> = manifest
        .get("patients")
        .as_array()
        .iter()
        .map(|p| (py_str(p.get("mrn")), p))
        .collect();
    let seeds = json::parse(&fs::read_to_string(seeds)?)?;
    let mut seed_of: HashMap<String, &Json> = HashMap::new();
    for u in seeds.get("units").as_array() {
        for m in u.get("members").as_array() {
            seed_of.insert(py_str(m.get("mrn")), m);
        }
    }
    let renderer = if as_of == AS_OF {
        None
    } else {
        Some(renderer.ok_or("re-rendering before the bank time needs a renderer")?)
    };
    let mut rows: Vec<Vec<Json>> = vec![Vec::new(); FAMILIES.len()];
    let mut skipped: Vec<Json> = Vec::new();
    for mrn in panel_doc.get("members").as_array() {
        let mrn = py_str(mrn);
        let p = by_mrn
            .get(&mrn)
            .ok_or_else(|| format!("KeyError: {mrn} not in the manifest"))?;
        let Some(r) = renderer else {
            for (slot, fam) in FAMILIES.iter().enumerate() {
                let path = bank
                    .join("patients")
                    .join(&mrn)
                    .join(format!("{fam}.ndjson"));
                rows[slot].extend(read_ndjson(&path)?);
            }
            continue;
        };
        if p.get("unit").as_str() == Some("roger") {
            skipped.push(mrn.into());
            continue;
        }
        let unit = py_str(p.get("unit"));
        let led = json::parse(&fs::read_to_string(
            bank.join("ledgers").join(format!("{unit}.json")),
        )?)?;
        let pled = led
            .get("patients")
            .as_array()
            .iter()
            .find(|x| x.get("mrn").as_str() == Some(&mrn))
            .ok_or_else(|| format!("StopIteration: {mrn} not in {unit}"))?;
        let first = pled
            .get("events")
            .as_array()
            .iter()
            .filter_map(|e| e.get("start").as_str())
            .min()
            .ok_or_else(|| format!("ValueError: {mrn} has no events"))?;
        if first >= as_of {
            skipped.push(mrn.into()); // not yet a patient of the practice
            continue;
        }
        let seed = seed_of
            .get(&mrn)
            .ok_or_else(|| format!("KeyError: {mrn} not in the seeds"))?;
        let (chart, _) = r.render(seed, pled, as_of);
        for (slot, (_, fam_rows)) in chart.iter().enumerate() {
            rows[slot].extend(fam_rows.iter().cloned());
        }
    }
    if out.exists() {
        fs::remove_dir_all(out)?;
    }
    fs::create_dir_all(out)?;
    for (fam, fam_rows) in FAMILIES.iter().zip(&rows) {
        write_ndjson(&out.join(format!("{fam}.ndjson")), fam_rows)?;
    }
    let counts = Json::Obj(
        FAMILIES
            .iter()
            .zip(&rows)
            .map(|(f, v)| (f.to_string(), Json::Int(v.len() as i64)))
            .collect(),
    );
    let summary = obj![
        ("panel", panel),
        ("as_of", as_of),
        ("patients", rows[0].len() as i64),
        ("not_included", skipped),
        ("counts", counts),
    ];
    fs::write(
        out.join("materialized.json"),
        json::dumps(&summary, Layout::Indent(1), true) + "\n",
    )?;
    Ok(summary)
}
