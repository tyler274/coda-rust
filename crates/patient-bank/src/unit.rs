//! The authoring agent's tool for one unit (a patient, or a family authored together).
//!
//!   brief   <unit>   seed, targets, paths
//!   check   <unit>   merge parts, check, count; ACCEPTED writes the ledger
//!   preview <unit>   render and write chart.txt for reading
//!
//! Drafts live in `<root>/drafts/<unit>/` (story.md, part-NN.json). An accepted
//! ledger is written to `<root>/ledgers/<unit>.json`, the only input the bank
//! build renders. See authoring-guide.md for the process.
//!
//! A port of `unit.py`, with one difference: where the ledger check meets a
//! ledger it cannot read (Python raised and `unit.py check` stopped with a
//! traceback), `check` reports that as an error line.

use std::collections::HashMap;
use std::fs;
use std::panic::{self, AssertUnwindSafe};
use std::path::{Path, PathBuf};

use base64::Engine as _;

use crate::bank::{AS_OF, py_str};
use crate::catalog::{Result, pathlib_str};
use crate::json::{self, Json, Layout};
use crate::ledger::{self, head};
use crate::render::{self, Chart, Renderer};
use crate::terms::py_strip;
use crate::validate::future_mentions;
use crate::{obj, paths, terms, time};

pub const MODEL: &str = "claude-opus-5-5 (Claude Code workflow subagent)";
/// Share of each target a chart must reach.
pub const FLOOR: f64 = 0.8;
const HEADER: [&str; 8] = [
    "given",
    "family",
    "birth_date",
    "address_line",
    "phone",
    "email",
    "marital_status",
    "summary",
];

fn names(section: &Json) -> Vec<String> {
    let mut out: Vec<String> = section.as_object().iter().map(|(k, _)| k.clone()).collect();
    out.sort();
    out
}

/// The catalog's labels as authors use them (`vocabulary.md`).
pub fn vocabulary(catalog: &Json) -> String {
    let block = |title: &str, names: &[String]| {
        let lines: Vec<String> = names.iter().map(|n| format!("- {n}")).collect();
        format!("### {title}\n{}\n", lines.join("\n"))
    };
    let panels: Vec<String> = catalog
        .get("panels")
        .as_object()
        .iter()
        .map(|(k, v)| {
            let members: Vec<String> = v.get("members").as_array().iter().map(py_str).collect();
            format!("- {k}: {}", members.join(", "))
        })
        .collect();
    let in_panels: Vec<String> = catalog
        .get("panels")
        .as_object()
        .iter()
        .flat_map(|(_, v)| v.get("members").as_array().iter().map(py_str))
        .collect();
    let standalone: Vec<String> = names(catalog.get("labs"))
        .into_iter()
        .filter(|l| !in_panels.contains(l))
        .collect();
    let section = |k: &str| names(catalog.get(k));
    [
        "## Catalog (use these labels exactly)".to_string(),
        block("Conditions (ICD-10-CM)", &section("conditions")),
        block(
            "Medications (RxNorm clinical drug names)",
            &section("medications"),
        ),
        format!("### Lab panels and their members\n{}\n", panels.join("\n")),
        block("Standalone labs (panel: null)", &standalone),
        block("Surveys", &section("surveys")),
        block("Social history items", &section("social")),
        block(
            "Reports (imaging, ECG and other studies)",
            &section("reports"),
        ),
        block("Procedures and visit codes", &section("procedures")),
        block("Allergens", &section("allergens")),
        block("Allergy reactions", &section("reactions")),
    ]
    .join("\n")
}

/// The draft parts merged into one ledger (events still carry `_part`), and the
/// problems found merging.
pub fn merge(drafts: &Path) -> Result<(Json, Vec<String>)> {
    let mut errors: Vec<String> = Vec::new();
    let mut parts: Vec<String> = Vec::new();
    for entry in fs::read_dir(drafts)? {
        let name = entry?.file_name().to_string_lossy().into_owned();
        if name.starts_with("part-") && name.ends_with(".json") && name.len() >= 10 {
            parts.push(name);
        }
    }
    parts.sort();
    if parts.is_empty() {
        return Ok((
            obj![("patients", Json::Arr(Vec::new()))],
            vec![format!("no part-*.json files in {}", pathlib_str(drafts))],
        ));
    }
    let mut patients: Vec<Json> = Vec::new();
    for name in &parts {
        let data = match json::parse(&fs::read_to_string(drafts.join(name))?) {
            Ok(data) => data,
            Err(exc) => {
                errors.push(format!("{name}: invalid JSON ({exc})"));
                continue;
            }
        };
        for p in data.get("patients").as_array() {
            let mrn = p.get("mrn");
            let at = match patients.iter().position(|x| x.get("mrn") == mrn) {
                Some(at) => at,
                None => {
                    patients.push(obj![
                        ("mrn", mrn.clone()),
                        ("events", Json::Arr(Vec::new()))
                    ]);
                    patients.len() - 1
                }
            };
            let into = &mut patients[at];
            for k in HEADER {
                if p.contains(k) && !into.contains(k) {
                    into.set(k, p.get(k).clone());
                }
            }
            let Some(Json::Arr(events)) = into.get_mut("events") else {
                unreachable!("events is a list");
            };
            for ev in p.get("events").as_array() {
                let mut ev = ev.clone();
                if let Json::Obj(_) = ev {
                    ev.set("_part", name.as_str().into());
                }
                events.push(ev);
            }
        }
    }
    for p in &mut patients {
        if let Some(Json::Arr(events)) = p.get_mut("events") {
            events.sort_by(|a, b| {
                let start = |e: &Json| e.get("start").as_str().unwrap_or("").to_string();
                start(a).cmp(&start(b))
            });
        }
        for k in ["given", "family", "birth_date"] {
            if !p.contains(k) {
                errors.push(format!(
                    "{}: {k} missing (put the patient fields in part-01.json)",
                    py_str(p.get("mrn"))
                ));
            }
        }
    }
    Ok((obj![("patients", Json::Arr(patients))], errors))
}

/// The ledger without the `_part` markers.
pub fn strip(led: &Json) -> Json {
    let patients: Vec<Json> = led
        .get("patients")
        .as_array()
        .iter()
        .map(|p| {
            let mut p = p.clone();
            let events: Vec<Json> = p
                .get("events")
                .as_array()
                .iter()
                .map(|e| {
                    let mut e = e.clone();
                    e.remove("_part");
                    e
                })
                .collect();
            p.set("events", Json::Arr(events));
            p
        })
        .collect();
    obj![("patients", Json::Arr(patients))]
}

/// Each member's chart as of the bank time, and the extra terms resolved for the draft.
fn rendered(
    unit: &Json,
    led: &Json,
    catalog: &Json,
    drafts: &Path,
) -> Result<(Vec<(String, Chart)>, Json)> {
    let merged = drafts.join("merged");
    fs::create_dir_all(&merged)?;
    let stripped = strip(led);
    fs::write(
        merged.join("ledger.json"),
        json::dumps(&stripped, Layout::Default, false),
    )?;
    let extra = terms::resolve_extras(&merged, catalog, false)?;
    let r = Renderer::new(catalog, &extra);
    let by = stripped.get("patients").as_array();
    let mut charts: Vec<(String, Chart)> = Vec::new();
    for m in unit.get("members").as_array() {
        // a dict comprehension keeps the last patient with an MRN, and the first key
        let Some(p) = by.iter().rev().find(|p| p.get("mrn") == m.get("mrn")) else {
            continue;
        };
        let mrn = py_str(m.get("mrn"));
        let chart = r.render(m, p, AS_OF).0;
        match charts.iter_mut().find(|(k, _)| *k == mrn) {
            Some(slot) => slot.1 = chart,
            None => charts.push((mrn, chart)),
        }
    }
    Ok((charts, extra))
}

pub fn cmd_brief(unit: &Json, drafts: &Path) {
    let members: Vec<Json> = unit
        .get("members")
        .as_array()
        .iter()
        .map(|m| {
            let mut out = Json::Obj(
                m.as_object()
                    .iter()
                    .filter(|(k, _)| k != "age_at_as_of")
                    .cloned()
                    .collect(),
            );
            out.set("age_on_2026_08_27", m.get("age_at_as_of").clone());
            out
        })
        .collect();
    let n = members.len();
    println!(
        "unit {} ({}, {n} patient{})",
        py_str(unit.get("unit")),
        py_str(unit.get("kind")),
        if n > 1 { "s" } else { "" }
    );
    if n > 1 {
        println!("Author these patients together; they are a family and their charts must agree.");
    }
    println!(
        "\nSeed:\n{}",
        json::dumps(&Json::Arr(members), Layout::Indent(1), true)
    );
    println!("\nTargets (the check requires at least 80% of each):");
    for m in unit.get("members").as_array() {
        println!(
            "  {}: {} documents, {} observations, about {} years with the practice",
            py_str(m.get("mrn")),
            py_str(m.get("target_documents")),
            py_str(m.get("target_observations")),
            py_str(m.get("years_with_practice"))
        );
    }
    let drafts = pathlib_str(drafts);
    println!("\nGuide:      {}", pathlib_str(&paths::authoring_guide()));
    println!("Vocabulary: {}", pathlib_str(&paths::vocabulary()));
    println!("Write to:   {drafts}/story.md, {drafts}/part-01.json, part-02.json, ...");
}

/// `ledger::check`, with a panic (Python's exception) reported as an error.
fn ledger_check(unit: &Json, led: &Json, catalog: &Json) -> (Vec<String>, Vec<String>) {
    let quiet = panic::take_hook();
    panic::set_hook(Box::new(|_| {}));
    let got = panic::catch_unwind(AssertUnwindSafe(|| ledger::check(unit, led, catalog)));
    panic::set_hook(quiet);
    got.unwrap_or_else(|payload| {
        let why = payload
            .downcast_ref::<String>()
            .map(String::as_str)
            .or_else(|| payload.downcast_ref::<&str>().copied())
            .unwrap_or("unknown failure");
        (
            vec![format!(
                "the ledger check could not read this ledger: {why}"
            )],
            Vec::new(),
        )
    })
}

pub fn cmd_check(unit: &Json, drafts: &Path, catalog: &Json) -> Result<i32> {
    let (led, mut errors) = merge(drafts)?;
    let (errs, mut warnings) = ledger_check(unit, &strip(&led), catalog);
    let mut part_of: HashMap<String, String> = HashMap::new();
    for p in led.get("patients").as_array() {
        for e in p.get("events").as_array() {
            part_of.insert(
                format!("{}/{}", py_str(p.get("mrn")), py_str(e.get("id"))),
                py_str(e.get("_part")),
            );
        }
    }
    for e in errs {
        let key = e.split(':').next().unwrap_or("");
        errors.push(match part_of.get(key) {
            Some(part) => format!("[{part}] {e}"),
            None => e,
        });
    }
    let mut counts: Vec<(String, Vec<(&str, usize)>)> = Vec::new();
    if errors.is_empty() {
        let (charts, extra) = rendered(unit, &led, catalog, drafts)?;
        for x in extra.get("rejected").as_array() {
            warnings.push(format!("left text-only (lookup failed): {}", py_str(x)));
        }
        for m in unit.get("members").as_array() {
            let mrn = py_str(m.get("mrn"));
            let Some((_, chart)) = charts.iter().find(|(k, _)| *k == mrn) else {
                continue;
            };
            let docs = chart.get("DocumentReference").len();
            let obs = chart.get("Observation").len();
            counts.push((
                mrn.clone(),
                chart.iter().map(|(f, v)| (f, v.len())).collect(),
            ));
            for (what, have, want) in [
                ("documents", docs, m.get("target_documents")),
                ("observations", obs, m.get("target_observations")),
            ] {
                let want_n = want.as_f64().ok_or("target is not a number")?;
                if (have as f64) < FLOOR * want_n {
                    errors.push(format!(
                        "{mrn}: {have} {what}, target {} (need at least {})",
                        py_str(want),
                        (FLOOR * want_n).trunc() as i64
                    ));
                }
            }
            for (_, msg) in future_mentions(chart)? {
                warnings.push(msg);
            }
        }
    }
    for (mrn, c) in &counts {
        let parts: Vec<String> = c.iter().map(|(k, v)| format!("{k} {v}")).collect();
        println!("{mrn}: {}", parts.join(", "));
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
    let out = paths::root()
        .join("ledgers")
        .join(format!("{}.json", py_str(unit.get("unit"))));
    fs::create_dir_all(out.parent().expect("ledgers directory"))?;
    let tmp = out.with_extension("tmp");
    let mut accepted = obj![
        ("unit", unit.get("unit").clone()),
        ("model", MODEL),
        (
            "warnings",
            warnings
                .iter()
                .map(|w| Json::from(w.as_str()))
                .collect::<Vec<_>>()
        ),
    ];
    accepted.set("patients", strip(&led).get("patients").clone());
    fs::write(&tmp, json::dumps(&accepted, Layout::Indent(1), false))?;
    fs::rename(&tmp, &out)?;
    println!("\nACCEPTED -> {}", pathlib_str(&out));
    if !warnings.is_empty() {
        println!(
            "Review the warnings above; future-mention warnings are often real chronology leaks."
        );
    }
    Ok(0)
}

/// A UTC instant as the practice's Eastern wall clock, the way the chart UI shows it.
fn local(utc: &str) -> String {
    if utc.chars().count() <= 10 {
        return utc.to_string();
    }
    let at = time::utc_to_local(utc).unwrap_or_else(|| panic!("ValueError: {utc:?}"));
    at.strftime("%Y-%m-%d %H:%M").to_string()
}

fn released(row: &Json) -> String {
    let (issued, eff) = (row.get("issued"), row.get("effectiveDateTime"));
    if issued.truthy()
        && eff.truthy()
        && time::utc_seconds_between(py_str(issued).as_str(), py_str(eff).as_str())
            .is_some_and(|s| s >= 6 * 3600)
    {
        return format!(" (resulted {})", local(&py_str(issued)));
    }
    String::new()
}

fn str_or<'a>(v: &'a Json, default: &'a str) -> &'a str {
    v.as_str().unwrap_or(default)
}

fn decode(data: &Json) -> Result<String> {
    let bytes = base64::engine::general_purpose::STANDARD.decode(str_or(data, ""))?;
    Ok(String::from_utf8(bytes)?)
}

/// `(x.get(key) or [{}])[0]`
fn first_or_empty<'a>(v: &'a Json, key: &str) -> &'a Json {
    static EMPTY: Json = Json::Obj(Vec::new());
    v.get(key).as_array().first().unwrap_or(&EMPTY)
}

/// The chart as one reading copy: problems, medications, allergies, then every
/// dated record oldest first on the Eastern wall clock.
pub fn chart_text(chart: &Chart) -> Result<String> {
    let mut out: Vec<String> = Vec::new();
    let p = &chart.get("Patient")[0];
    let name = &p.get("name").as_array()[0];
    out.push(format!(
        "{} {} | {} | born {} | {}",
        py_str(&name.get("given").as_array()[0]),
        py_str(name.get("family")),
        py_str(p.get("gender")),
        py_str(p.get("birthDate")),
        py_str(p.get("address").as_array()[0].get("city"))
    ));
    out.push("\n## Problem list".into());
    let mut conditions: Vec<&Json> = chart.get("Condition").iter().collect();
    conditions
        .sort_by(|a, b| str_or(a.get("recordedDate"), "").cmp(str_or(b.get("recordedDate"), "")));
    for c in conditions {
        let resolved = if c.get("abatementDateTime").truthy() {
            let when = local(&py_str(c.get("abatementDateTime")));
            format!(" (resolved {})", head(&when, 10))
        } else {
            String::new()
        };
        out.push(format!(
            "  recorded {} onset {} {}: {}{resolved}",
            head(&local(str_or(c.get("recordedDate"), "")), 10),
            head(&local(str_or(c.get("onsetDateTime"), "")), 10),
            py_str(c.get("clinicalStatus").get("coding").as_array()[0].get("code")),
            py_str(c.get("code").get("text"))
        ));
    }
    out.push("\n## Medications".into());
    let mut meds: Vec<&Json> = chart.get("MedicationRequest").iter().collect();
    meds.sort_by(|a, b| str_or(a.get("authoredOn"), "").cmp(str_or(b.get("authoredOn"), "")));
    for m in meds {
        let dose = first_or_empty(m, "dosageInstruction");
        let dose = if dose.contains("text") {
            py_str(dose.get("text"))
        } else {
            String::new()
        };
        out.push(format!(
            "  {} {}: {} | {dose}",
            head(&local(&py_str(m.get("authoredOn"))), 10),
            py_str(m.get("status")),
            py_str(m.get("medicationCodeableConcept").get("text"))
        ));
    }
    out.push("\n## Allergies".into());
    let allergies = chart.get("AllergyIntolerance");
    if allergies.is_empty() {
        out.push("  none recorded".into());
    }
    for a in allergies {
        out.push(format!("  {}", py_str(a.get("code").get("text"))));
    }
    let mut timeline: Vec<(String, String)> = Vec::new();
    for o in chart.get("Observation") {
        if o.get("hasMember").truthy() {
            continue;
        }
        let name = if o.get("code").get("text").truthy() {
            o.get("code").get("text")
        } else {
            o.get("code").get("coding").as_array()[0].get("display")
        };
        let v = if o.contains("valueQuantity") {
            let q = o.get("valueQuantity");
            let unit = if q.contains("unit") {
                py_str(q.get("unit"))
            } else {
                String::new()
            };
            py_strip(&format!("{} {unit}", py_str(q.get("value")))).to_string()
        } else if o.contains("component") {
            let values: Vec<String> = o
                .get("component")
                .as_array()
                .iter()
                .map(|c| py_str(c.get("valueQuantity").get("value")))
                .collect();
            values.join("/")
        } else if o.get("valueString").truthy() {
            py_str(o.get("valueString"))
        } else if first_text(o.get("valueCodeableConcept")).truthy() {
            py_str(first_text(o.get("valueCodeableConcept")))
        } else {
            "-".into()
        };
        let interpretation = first_or_empty(o, "interpretation");
        let coding = if interpretation.contains("coding") {
            interpretation.get("coding").as_array()[0].clone()
        } else {
            Json::obj()
        };
        let flag = if coding.contains("code") {
            py_str(coding.get("code"))
        } else {
            String::new()
        };
        let flag = if flag == "H" || flag == "L" {
            format!(" {flag}")
        } else {
            String::new()
        };
        timeline.push((
            local(str_or(o.get("effectiveDateTime"), "")),
            format!("obs  {} {v}{flag}{}", py_str(name), released(o)),
        ));
    }
    for r in chart.get("DiagnosticReport") {
        let text = if r.get("presentedForm").truthy() {
            decode(r.get("presentedForm").as_array()[0].get("data"))?
        } else {
            String::new()
        };
        let body = if text.is_empty() {
            String::new()
        } else {
            format!("\n{}", py_strip(&text))
        };
        timeline.push((
            local(str_or(r.get("effectiveDateTime"), "")),
            format!(
                "REPORT {} [{}]{}{body}",
                py_str(r.get("code").get("text")),
                py_str(r.get("status")),
                released(r)
            ),
        ));
    }
    for x in chart.get("Procedure") {
        timeline.push((
            local(str_or(x.get("performedDateTime"), "")),
            format!("proc {}", py_str(x.get("code").get("text"))),
        ));
    }
    for d in chart.get("DocumentReference") {
        let text = decode(d.get("content").as_array()[0].get("attachment").get("data"))?;
        timeline.push((
            local(&py_str(d.get("date"))),
            format!(
                "DOCUMENT {}\n{}",
                py_str(d.get("type").get("text")),
                py_strip(&text)
            ),
        ));
    }
    out.push(
        "\n## Chart, oldest first (Eastern time; results show when they were released)".into(),
    );
    timeline.sort_by(|a, b| a.0.cmp(&b.0));
    let mut day: Option<String> = None;
    for (when, line) in timeline {
        let today = head(&when, 10).to_string();
        if day.as_deref() != Some(today.as_str()) {
            out.push(format!("\n=== {today}"));
            day = Some(today);
        }
        if line.starts_with("DOCUMENT") || line.starts_with("REPORT") {
            out.push(format!("\n{line}"));
        } else {
            out.push(line);
        }
    }
    Ok(out.join("\n") + "\n")
}

/// `(v or {}).get("text")`
fn first_text(v: &Json) -> &Json {
    if v.truthy() {
        v.get("text")
    } else {
        &Json::Null
    }
}

pub fn cmd_preview(unit: &Json, drafts: &Path, catalog: &Json) -> Result<i32> {
    let (led, errors) = merge(drafts)?;
    if !errors.is_empty() {
        let lines: Vec<String> = errors.iter().map(|e| format!("ERROR: {e}")).collect();
        println!("{}", lines.join("\n"));
        return Ok(1);
    }
    let (charts, _) = rendered(unit, &led, catalog, drafts)?;
    let mut texts = Vec::new();
    for (_, c) in &charts {
        texts.push(chart_text(c)?);
    }
    let path = drafts.join("chart.txt");
    fs::write(&path, texts.join("\n\n"))?;
    let kb = fs::metadata(&path)?.len() / 1024;
    println!(
        "wrote {} ({kb} KB). Read it start to end.",
        pathlib_str(&path)
    );
    Ok(0)
}

/// Run `brief`, `check` or `preview` for `uid`: refresh the shared vocabulary, find
/// the unit in `seeds`, and make its draft directory. The result is the exit status.
pub fn run(command: &str, uid: &str, seeds: &Path) -> Result<i32> {
    let catalog = render::load_catalog(&paths::catalog());
    let vocab = paths::vocabulary();
    let text = vocabulary(&catalog);
    // follows the catalog; agents share it
    if fs::read_to_string(&vocab).ok().as_deref() != Some(text.as_str()) {
        let tmp = vocab.with_file_name(format!("vocabulary.{}.tmp", std::process::id()));
        fs::write(&tmp, &text)?;
        fs::rename(&tmp, &vocab)?;
    }
    let seeds = json::parse(&fs::read_to_string(seeds)?)?;
    let Some(unit) = seeds
        .get("units")
        .as_array()
        .iter()
        .find(|u| u.get("unit").as_str() == Some(uid))
    else {
        return Err(format!("unknown unit {uid}").into());
    };
    let drafts: PathBuf = paths::root().join("drafts").join(uid);
    fs::create_dir_all(&drafts)?;
    match command {
        "brief" => {
            cmd_brief(unit, &drafts);
            Ok(0)
        }
        "check" => cmd_check(unit, &drafts, &catalog),
        "preview" => cmd_preview(unit, &drafts, &catalog),
        other => Err(format!("unknown unit command {other}").into()),
    }
}
