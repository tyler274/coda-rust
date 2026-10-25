//! The authoring agent's tool for one unit (a patient, or a family authored together).
//!
//! ```text
//! brief   <unit>   seed, targets, paths
//! check   <unit>   merge parts, check, verify extra codes, count; ACCEPTED writes the ledger
//! preview <unit>   render to the horizon and write chart.txt for reading
//! ```
//!
//! Drafts live in `<root>/drafts/<unit>/` (story.md, part-NN.json). An accepted
//! ledger is written to `patients/ledgers/<unit>.json`, the only input the bank build
//! renders; `check` is its only writer. See `data/authoring-guide.md`.

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

use base64::Engine as _;

use crate::catalog::{Catalog, Lookup, write_atomic};
use crate::fhir::{Chart, Observation};
use crate::ledger::{Ledger, Part, Patient, Stamp, Terms};
use crate::render::Renderer;
use crate::seeds::{Seeds, Unit};
use crate::{Result, check, paths, terms, time};

pub const MODEL: &str = "claude-opus-5-5 (Claude Code workflow subagent)";
/// The share of each density target a chart must reach.
const FLOOR: f64 = 0.8;

/// The catalog's labels as authors see them.
pub fn vocabulary(c: &Catalog) -> String {
    fn block<'a>(title: &str, names: impl IntoIterator<Item = &'a String>) -> String {
        let lines: Vec<String> = names.into_iter().map(|n| format!("- {n}")).collect();
        format!("### {title}\n{}\n", lines.join("\n"))
    }
    let panels: Vec<String> = c
        .panels
        .iter()
        .map(|(k, v)| format!("- {k}: {}", v.members.join(", ")))
        .collect();
    let standalone: Vec<String> = c.standalone_labs().into_iter().map(String::from).collect();
    [
        "## Catalog (use these labels exactly)".to_string(),
        block("Conditions (ICD-10-CM)", c.conditions.keys()),
        block(
            "Medications (RxNorm clinical drug names)",
            c.medications.keys(),
        ),
        format!("### Lab panels and their members\n{}\n", panels.join("\n")),
        block("Standalone labs (panel: null)", &standalone),
        block("Surveys", c.surveys.keys()),
        block("Social history items", c.social.keys()),
        block("Reports (imaging, ECG and other studies)", c.reports.keys()),
        block("Procedures and visit codes", c.procedures.keys()),
        block("Allergens", c.allergens.keys()),
        block("Allergy reactions", c.reactions.keys()),
        block("Vaccines (CVX)", c.vaccines.keys()),
    ]
    .join("\n")
}

/// The unit's draft parts merged into charts, and which part each event came from.
pub struct Merged {
    pub patients: Vec<Patient>,
    pub part_of: BTreeMap<String, String>,
}

pub fn merge(drafts: &Path) -> (Merged, Vec<String>) {
    let mut errors = Vec::new();
    let mut part_of = BTreeMap::new();
    let mut parts: Vec<PathBuf> = fs::read_dir(drafts)
        .map(|dir| {
            dir.filter_map(|e| e.ok().map(|e| e.path()))
                .filter(|p| {
                    p.file_name()
                        .and_then(|n| n.to_str())
                        .is_some_and(|n| n.starts_with("part-") && n.ends_with(".json"))
                })
                .collect()
        })
        .unwrap_or_default();
    parts.sort();
    if parts.is_empty() {
        errors.push(format!("no part-*.json files in {}", drafts.display()));
    }
    let mut merged: Vec<crate::ledger::PartPatient> = Vec::new();
    for path in &parts {
        let name = path
            .file_name()
            .expect("file")
            .to_string_lossy()
            .to_string();
        let part: Part = match fs::read_to_string(path)
            .map_err(|e| e.to_string())
            .and_then(|t| serde_json::from_str(&t).map_err(|e| e.to_string()))
        {
            Ok(p) => p,
            Err(e) => {
                errors.push(format!("{name}: {e}"));
                continue;
            }
        };
        for p in part.patients {
            for ev in &p.events {
                part_of.insert(format!("{}/{}", p.mrn, ev.id), name.clone());
            }
            match merged.iter_mut().find(|m| m.mrn == p.mrn) {
                None => merged.push(p),
                Some(into) => {
                    // the first part that gives a patient field wins
                    into.given = into.given.take().or(p.given);
                    into.family = into.family.take().or(p.family);
                    into.birth_date = into.birth_date.take().or(p.birth_date);
                    into.address_line = into.address_line.take().or(p.address_line);
                    into.phone = into.phone.take().or(p.phone);
                    into.email = into.email.take().or(p.email);
                    into.marital_status = into.marital_status.take().or(p.marital_status);
                    into.summary = into.summary.take().or(p.summary);
                    into.events.extend(p.events);
                }
            }
        }
    }
    let mut patients = Vec::new();
    for mut p in merged {
        p.events.sort_by_key(|e| e.start);
        let (Some(given), Some(family), Some(birth_date)) = (p.given, p.family, p.birth_date)
        else {
            errors.push(format!("{}: given, family and birth_date are required (put the patient fields in part-01.json)", p.mrn));
            continue;
        };
        patients.push(Patient {
            mrn: p.mrn,
            given,
            family,
            birth_date,
            address_line: p.address_line,
            phone: p.phone,
            email: p.email,
            marital_status: p.marital_status,
            summary: p.summary,
            events: p.events,
        });
    }
    (Merged { patients, part_of }, errors)
}

/// Every member's chart at `at`, rendered with the unit's verified codes.
fn render(
    unit: &Unit,
    patients: &[Patient],
    catalog: &Catalog,
    terms: &Terms,
    at: Stamp,
) -> Vec<(String, Chart)> {
    let r = Renderer { catalog, terms };
    unit.members
        .iter()
        .filter_map(|m| {
            Some((
                m.mrn.clone(),
                r.render(m, patients.iter().find(|p| p.mrn == m.mrn)?, at).0,
            ))
        })
        .collect()
}

fn verify(drafts: &Path, patients: &[Patient], catalog: &Catalog) -> Result<Terms> {
    let mut look = Lookup::layered(&drafts.join("lookups.json"), false)?;
    terms::resolve(patients, catalog, &mut look)
}

fn brief(unit: &Unit, drafts: &Path) -> Result<i32> {
    let n = unit.members.len();
    println!(
        "unit {} ({}, {n} patient{})",
        unit.unit,
        unit.kind,
        if n > 1 { "s" } else { "" }
    );
    if n > 1 {
        println!("Author these patients together; they are a family and their charts must agree.");
    }
    println!("\nSeed:\n{}", serde_json::to_string_pretty(&unit.members)?);
    println!("\nTargets (the check requires at least 80% of each):");
    for m in &unit.members {
        println!(
            "  {}: {} documents, {} observations, about {} years with the practice",
            m.mrn, m.target_documents, m.target_observations, m.years_with_practice
        );
    }
    println!("\nGuide:      {}", paths::authoring_guide().display());
    println!("Vocabulary: {}", paths::vocabulary().display());
    println!(
        "Write to:   {0}/story.md, {0}/part-01.json, part-02.json, ...",
        drafts.display()
    );
    Ok(0)
}

fn accept(unit: &Unit, drafts: &Path, catalog: &Catalog) -> Result<i32> {
    let (merged, mut errors) = merge(drafts);
    let (errs, mut warnings) = check::check(unit, &merged.patients, catalog);
    errors.extend(errs.into_iter().map(|e| {
        let eid = e.split(':').next().unwrap_or("");
        match merged.part_of.get(eid) {
            Some(part) => format!("[{part}] {e}"),
            None => e,
        }
    }));
    let mut terms = Terms::default();
    if errors.is_empty() {
        terms = verify(drafts, &merged.patients, catalog)?;
        warnings.extend(
            terms
                .rejected
                .iter()
                .map(|x| format!("left text-only (not verified): {x}")),
        );
        for (mrn, chart) in render(unit, &merged.patients, catalog, &terms, time::as_of()) {
            let seed = unit.members.iter().find(|m| m.mrn == mrn).expect("member");
            let counts: Vec<String> = chart
                .counts()
                .iter()
                .map(|(f, n)| format!("{f} {n}"))
                .collect();
            println!("{mrn}: {}", counts.join(", "));
            for (what, have, want) in [
                ("documents", chart.documents.len(), seed.target_documents),
                (
                    "observations",
                    chart.observations.len(),
                    seed.target_observations,
                ),
            ] {
                let need = (FLOOR * f64::from(want)) as usize;
                if have < need {
                    errors.push(format!(
                        "{mrn}: {have} {what}, target {want} (need at least {need})"
                    ));
                }
            }
        }
    }
    for w in &warnings {
        println!("warning: {w}");
    }
    for e in &errors {
        println!("ERROR: {e}");
    }
    if !errors.is_empty() {
        println!(
            "\n{} errors. Fix them in the part files and run check again.",
            errors.len()
        );
        return Ok(1);
    }
    let ledger = Ledger {
        unit: unit.unit.clone(),
        model: MODEL.into(),
        warnings,
        patients: merged.patients,
        terms,
    };
    let out = paths::ledgers().join(format!("{}.json", unit.unit));
    fs::create_dir_all(out.parent().expect("ledgers dir"))?;
    write_atomic(&out, &serde_json::to_string_pretty(&ledger)?)?;
    println!("\nACCEPTED -> {}", out.display());
    Ok(0)
}

fn preview(unit: &Unit, drafts: &Path, catalog: &Catalog) -> Result<i32> {
    let (merged, errors) = merge(drafts);
    if !errors.is_empty() {
        for e in &errors {
            println!("ERROR: {e}");
        }
        return Ok(1);
    }
    let terms = verify(drafts, &merged.patients, catalog)?;
    let text: Vec<String> = render(unit, &merged.patients, catalog, &terms, time::horizon())
        .iter()
        .map(|(_, c)| chart_text(c))
        .collect();
    let path = drafts.join("chart.txt");
    fs::write(&path, text.join("\n\n"))?;
    println!(
        "wrote {} ({} KB). Read it start to end.",
        path.display(),
        fs::metadata(&path)?.len() / 1024
    );
    Ok(0)
}

pub fn run(command: &str, uid: &str, seeds: &Path) -> Result<i32> {
    let catalog = Catalog::load(&paths::catalog())?;
    let vocab = paths::vocabulary();
    let text = vocabulary(&catalog);
    if fs::read_to_string(&vocab).ok().as_deref() != Some(text.as_str()) {
        fs::create_dir_all(vocab.parent().expect("root"))?;
        write_atomic(&vocab, &text)?;
    }
    let seeds = Seeds::read(seeds)?;
    let unit = seeds
        .unit(uid)
        .ok_or_else(|| format!("unknown unit {uid}"))?;
    let drafts = paths::root().join("drafts").join(uid);
    fs::create_dir_all(&drafts)?;
    match command {
        "brief" => brief(unit, &drafts),
        "check" => accept(unit, &drafts, &catalog),
        _ => preview(unit, &drafts, &catalog),
    }
}

// ---------------------------------------------------------------- chart.txt

/// A FHIR instant on the practice's wall clock, the way the chart UI shows it.
fn local(utc: &str) -> String {
    if utc.len() <= 10 {
        return utc.to_string();
    }
    time::eastern_of(utc).map_or_else(
        || utc.to_string(),
        |s| s.0.strftime("%Y-%m-%d %H:%M").to_string(),
    )
}

/// " (resulted …)" when a result came back six or more hours after collection.
fn released(issued: Option<&str>, effective: &str) -> String {
    match (
        issued.and_then(time::eastern_of),
        time::eastern_of(effective),
    ) {
        (Some(i), Some(e)) if time::minutes_between(e, i) >= 6 * 60 => {
            format!(" (resulted {})", i.0.strftime("%Y-%m-%d %H:%M"))
        }
        _ => String::new(),
    }
}

fn decode(data: &str) -> String {
    let bytes = base64::engine::general_purpose::STANDARD
        .decode(data)
        .unwrap_or_default();
    String::from_utf8_lossy(&bytes).into_owned()
}

fn value(o: &Observation) -> String {
    if let Some(q) = &o.value_quantity {
        return format!("{} {}", q.value, q.unit.as_deref().unwrap_or(""))
            .trim()
            .to_string();
    }
    if !o.component.is_empty() {
        return o
            .component
            .iter()
            .map(|c| c.value_quantity.value.to_string())
            .collect::<Vec<_>>()
            .join("/");
    }
    o.value_codeable_concept
        .as_ref()
        .and_then(|c| c.text.clone())
        .unwrap_or_else(|| "-".into())
}

/// A chronological reading copy of one chart.
pub fn chart_text(chart: &Chart) -> String {
    let p = &chart.patient;
    let mut out = vec![format!(
        "{} {} | {} | born {} | {}",
        p.name[0].given[0], p.name[0].family, p.gender, p.birth_date, p.address[0].city
    )];
    out.push("\n## Problem list".into());
    let mut conditions: Vec<_> = chart.conditions.iter().collect();
    conditions.sort_by(|a, b| a.recorded_date.cmp(&b.recorded_date));
    for c in conditions {
        let resolved = c
            .abatement_date_time
            .as_deref()
            .map(|a| format!(" (resolved {})", &local(a)[..10]))
            .unwrap_or_default();
        out.push(format!(
            "  recorded {} onset {} {}: {}{resolved}",
            &local(&c.recorded_date)[..10],
            &local(&c.onset_date_time)[..10],
            c.clinical_status.coding[0].code,
            c.code.text.as_deref().unwrap_or("")
        ));
    }
    out.push("\n## Medications".into());
    let mut meds: Vec<_> = chart.medications.iter().collect();
    meds.sort_by(|a, b| a.authored_on.cmp(&b.authored_on));
    for m in meds {
        let dose = m
            .dosage_instruction
            .first()
            .map(|d| d.text.as_str())
            .unwrap_or("");
        out.push(format!(
            "  {} {}: {} | {dose}",
            &local(&m.authored_on)[..10],
            m.status,
            m.medication_codeable_concept.text.as_deref().unwrap_or("")
        ));
    }
    out.push("\n## Allergies".into());
    if chart.allergies.is_empty() {
        out.push("  none recorded".into());
    }
    out.extend(
        chart
            .allergies
            .iter()
            .map(|a| format!("  {}", a.code.text.as_deref().unwrap_or(""))),
    );

    let mut timeline: Vec<(String, String)> = Vec::new();
    for o in chart
        .observations
        .iter()
        .filter(|o| o.has_member.is_empty())
    {
        let name = o
            .code
            .text
            .clone()
            .or_else(|| o.code.coding.first().and_then(|c| c.display.clone()))
            .unwrap_or_default();
        let flag = o
            .interpretation
            .first()
            .and_then(|i| i.coding.first())
            .map(|c| c.code.as_str())
            .filter(|c| matches!(*c, "H" | "L"))
            .map(|c| format!(" {c}"))
            .unwrap_or_default();
        let line = format!(
            "obs  {name} {}{flag}{}",
            value(o),
            released(o.issued.as_deref(), &o.effective_date_time)
        );
        timeline.push((local(&o.effective_date_time), line));
    }
    for r in &chart.reports {
        let text = r
            .presented_form
            .first()
            .map(|f| decode(&f.data))
            .unwrap_or_default();
        let mut line = format!(
            "REPORT {} [{}]{}",
            r.code.text.as_deref().unwrap_or(""),
            r.status,
            released(r.issued.as_deref(), &r.effective_date_time)
        );
        if !text.is_empty() {
            line.push_str(&format!("\n{}", text.trim()));
        }
        timeline.push((local(&r.effective_date_time), line));
    }
    for x in &chart.procedures {
        timeline.push((
            local(&x.performed_date_time),
            format!("proc {}", x.code.text.as_deref().unwrap_or("")),
        ));
    }
    for e in &chart.encounters {
        let label = e.kind.first().and_then(|k| k.text.as_deref()).unwrap_or("");
        let reason = e
            .reason_code
            .first()
            .and_then(|r| r.text.as_deref())
            .map(|r| format!(": {r}"))
            .unwrap_or_default();
        timeline.push((
            local(&e.period.start),
            format!("VISIT {label} ({}){reason}", e.class.code),
        ));
    }
    for i in &chart.immunizations {
        timeline.push((
            local(&i.occurrence_date_time),
            format!(
                "imm  {}{}",
                i.vaccine_code.text.as_deref().unwrap_or(""),
                if i.primary_source { "" } else { " (reported)" }
            ),
        ));
    }
    for o in &chart.service_requests {
        let to = o
            .performer_type
            .first()
            .and_then(|p| p.text.as_deref())
            .map(|p| format!(" -> {p}"))
            .unwrap_or_default();
        timeline.push((
            local(&o.authored_on),
            format!(
                "order [{}] {}{to}",
                o.status,
                o.code.text.as_deref().unwrap_or("")
            ),
        ));
    }
    for d in &chart.documents {
        let text = decode(&d.content[0].attachment.data);
        timeline.push((
            local(&d.date),
            format!(
                "DOCUMENT {} by {}\n{}",
                d.kind.text.as_deref().unwrap_or(""),
                d.author
                    .first()
                    .map_or("(no author)", |a| a.display.as_str()),
                text.trim()
            ),
        ));
    }
    out.push(
        "\n## Chart, oldest first (Eastern time; results show when they were released)".into(),
    );
    timeline.sort_by(|a, b| a.0.cmp(&b.0));
    let bank_time = time::AS_OF.replace('T', " ");
    let mut day = String::new();
    let mut later = false;
    for (when, line) in timeline {
        if !later && when >= bank_time {
            later = true;
            out.push(format!(
                "\n----- bank time {bank_time}: everything below reaches the practice later"
            ));
            day.clear();
        }
        if when[..10] != day {
            day = when[..10].to_string();
            out.push(format!("\n=== {day}"));
        }
        if line.starts_with("DOCUMENT") || line.starts_with("REPORT") {
            out.push(format!("\n{line}"));
        } else {
            out.push(line);
        }
    }
    out.join("\n") + "\n"
}
