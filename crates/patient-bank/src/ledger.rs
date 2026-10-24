//! Check an authored ledger against its seed, the catalog and chronology.
//!
//! Errors make the author retry with the error list; warnings are recorded for
//! review. The checks are mechanical: they never judge clinical reasoning.
//!
//! A port of `ledger.py`. The messages embed Python `repr`/`str` of ledger
//! values and Python number formatting, and the word patterns use Python's
//! Unicode `\b`, `\d` and `re.I`, so those are reproduced here exactly. Where
//! Python raises on a malformed ledger (a list where a dict belongs, an
//! unhashable label, an invalid document date), `check` panics, as `unit.py
//! check` stopped with a traceback.

use std::collections::HashMap;
use std::sync::LazyLock;

use jiff::civil::Date;
use regex::Regex;

use crate::json::{Json, float_repr};
use crate::time::age_on;

pub const AS_OF: &str = "2026-08-27T12:00";
pub const KINDS: [&str; 16] = [
    "establish_care",
    "office_visit",
    "annual_physical",
    "telehealth_visit",
    "nurse_visit",
    "telephone",
    "portal_message",
    "lab_only",
    "imaging",
    "specialist_visit",
    "urgent_care",
    "ed_visit",
    "hospital_admission",
    "procedure_visit",
    "outside_records",
    "pharmacy",
];
/// Python's `^\d{4}-\d{2}-\d{2}T\d{2}:\d{2}$`: `\d` is any Unicode decimal and
/// `$` also matches before a final newline.
pub static STAMP: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\A\d{4}-\d{2}-\d{2}T\d{2}:\d{2}\n?\z").expect("STAMP"));
/// `\b(...)\b`, case-insensitive.
pub const FORBIDDEN: [&str; 10] = [
    "synthetic",
    "fictional",
    "generated",
    "template",
    "simulation",
    "simulated",
    "test patient",
    "answer key",
    "placeholder",
    "lorem",
];
/// Real care organizations and record systems a chart must not name (the guide asks for roles and
/// generic places). Drug, vaccine and assay product names stay, as in real charts. `\b(...)\b`,
/// case-sensitive.
pub const REAL_NAMES: [&str; 25] = [
    "MyChart",
    "Epic",
    "Cerner",
    "athenahealth",
    "Quest Diagnostics",
    "LabCorp",
    "CVS",
    "Walgreens",
    "Walmart",
    "Rite Aid",
    "Stop & Shop",
    "Big Y",
    "Baystate",
    "Cooley Dickinson",
    "Mercy Medical",
    "Holyoke Medical",
    "UMass",
    "Mass General",
    "Brigham",
    "Dartmouth",
    "Hartford Hospital",
    "Yale New Haven",
    "Valley Medical Group",
    "Kaiser",
    "Mayo Clinic",
];
pub const DIFFERENTIAL: [&str; 5] = [
    "Neutrophils %",
    "Lymphocytes %",
    "Monocytes %",
    "Eosinophils %",
    "Basophils %",
];
/// Words near a stated age that make it someone else's age (or a past age). `\b(...)\b`,
/// case-insensitive.
const KIN: [&str; 18] = [
    "mother", "father", "sister", "brother", "son", "daughter", "wife", "husband", "spouse",
    "partner", "grand", "aunt", "uncle", "cousin", "friend", "at age", "in her", "in his",
];
const VISITS: [&str; 4] = [
    "office_visit",
    "annual_physical",
    "establish_care",
    "telehealth_visit",
];

/// Every timestamp an event carries, with where it sits.
fn stamps(ev: &Json) -> Vec<(&'static str, &Json)> {
    let mut out = vec![("start", ev.get("start"))];
    if ev.get("end").truthy() {
        out.push(("end", ev.get("end")));
    }
    for (key, field, what) in [
        ("labs", "collected", "labs.collected"),
        ("reports", "performed", "reports.performed"),
        ("procedures", "performed", "procedures.performed"),
        ("immunizations", "date", "immunizations.date"),
        ("documents", "date", "documents.date"),
    ] {
        for x in dicts(ev, key) {
            out.push((what, x.get(field)));
        }
    }
    if let Json::Arr(vit) = ev.get("vitals") {
        for x in vit {
            let x = dict(x);
            if x.get("time").truthy() || vit.len() > 1 {
                out.push(("vitals.time", x.get("time")));
            }
        }
    }
    out
}

pub fn check(unit: &Json, ledger: &Json, catalog: &Json) -> (Vec<String>, Vec<String>) {
    let mut errors: Vec<String> = Vec::new();
    let mut warnings: Vec<String> = Vec::new();
    let seeds: Vec<(&str, &Json)> = unit
        .get("members")
        .as_array()
        .iter()
        .map(|m| (m.get("mrn").as_str().expect("seed mrn"), m))
        .collect();
    let patients = match dict(ledger).get("patients") {
        Json::Arr(p) => p,
        _ => {
            return (
                vec!["top level must be {\"patients\": [...]}".to_string()],
                warnings,
            );
        }
    };
    let mut got = KeySet::default();
    for p in patients {
        got.insert(dict(p).get("mrn"));
    }
    let mut want: Vec<&str> = seeds.iter().map(|(m, _)| *m).collect();
    want.sort();
    want.dedup();
    if got.len() != want.len() || !want.iter().all(|m| got.contains(&Json::from(*m))) {
        errors.push(format!(
            "patients must be exactly {}; got {}",
            repr_list(want.iter().map(|m| Json::from(*m)).collect()),
            repr_list(sorted_truthy(&got))
        ));
    }
    let mut bodies: HashMap<String, String> = HashMap::new();
    for p in patients {
        let Some(seed) = p.get("mrn").as_str().and_then(|m| {
            seeds
                .iter()
                .rev()
                .find(|(s, _)| *s == m)
                .map(|(_, seed)| *seed)
        }) else {
            continue;
        };
        if !seed.truthy() {
            continue;
        }
        let mrn = seed.get("mrn").as_str().expect("seed mrn");
        let birth_date = seed.get("birth_date").as_str().expect("seed birth_date");
        let birth = py_date(birth_date).expect("seed birth_date is an ISO date");
        for k in ["given", "family", "birth_date"] {
            if p.get(k).truthy() && !py_eq(p.get(k), seed.get(k)) {
                errors.push(format!(
                    "{mrn}: {k} must be {} (seed), got {}",
                    repr(seed.get(k)),
                    repr(p.get(k))
                ));
            }
        }
        let events: Vec<&Json> = if p.get("events").truthy() {
            iter_dicts(p.get("events"))
        } else {
            Vec::new()
        };
        if events.is_empty() {
            errors.push(format!("{mrn}: no events"));
            continue;
        }
        let mut ids = KeySet::default();
        for e in &events {
            ids.insert(e.get("id"));
        }
        if ids.len() != events.len() {
            errors.push(format!("{mrn}: duplicate event ids"));
        }
        let mut started = KeySet::default();
        let mut diagnosed = KeySet::default();
        let mut n_docs: i64 = 0;
        for ev in &events {
            let eid = format!("{mrn}/{}", py_str(ev.get("id")));
            hash(ev.get("kind"));
            if !in_list(ev.get("kind"), &KINDS) {
                errors.push(format!("{eid}: unknown kind {}", repr(ev.get("kind"))));
            }
            for (place, stamp) in stamps(ev) {
                let Some(s) = stamp.as_str().filter(|s| STAMP.is_match(s)) else {
                    if !stamp.is_null() || place == "start" {
                        errors.push(format!(
                            "{eid}: {place} must be 'YYYY-MM-DDTHH:MM', got {}",
                            repr(stamp)
                        ));
                    }
                    continue;
                };
                if !py_datetime_ok(s) {
                    errors.push(format!("{eid}: {place} invalid date {}", repr(stamp)));
                    continue;
                }
                if s >= AS_OF && place != "end" {
                    errors.push(format!("{eid}: {place} {s} is not before {AS_OF}"));
                }
                if head(s, 10) < birth_date {
                    errors.push(format!("{eid}: {place} {s} precedes birth {birth_date}"));
                }
            }
            let start = if ev.get("start").truthy() {
                ev.get("start")
            } else {
                &Json::Null
            };
            let start_str = || match start {
                Json::Null => "",
                Json::Str(s) => s.as_str(),
                other => panic!("TypeError: {} is not subscriptable", repr(other)),
            };
            for c in dicts(ev, "conditions") {
                let label = c.get("label");
                if !in_list(c.get("action"), &["diagnosed", "history", "resolved"]) {
                    errors.push(format!(
                        "{eid}: condition {} action must be diagnosed|history|resolved",
                        repr(label)
                    ));
                }
                if !in_dict(label, catalog.get("conditions")) && !c.get("icd10").truthy() {
                    errors.push(format!(
                        "{eid}: condition {} is not a catalog label and has no icd10",
                        repr(label)
                    ));
                }
                if in_list(c.get("action"), &["diagnosed", "history"]) {
                    diagnosed.insert(label);
                    let onset = c.get("onset");
                    if onset.truthy() {
                        let start10 = head(start_str(), 10);
                        let onset = py_str(onset);
                        if head(&onset, 10) > start10 {
                            errors.push(format!(
                                "{eid}: condition {} onset {onset} after recording event {start10}",
                                repr(label)
                            ));
                        }
                    }
                } else if c.get("action").as_str() == Some("resolved") && !diagnosed.contains(label)
                {
                    errors.push(format!(
                        "{eid}: condition {} resolved before being diagnosed",
                        repr(label)
                    ));
                }
            }
            for m in dicts(ev, "medications") {
                let key = if m.get("drug").truthy() {
                    m.get("drug")
                } else {
                    m.get("drug_text")
                };
                if m.get("drug").truthy() && !in_dict(m.get("drug"), catalog.get("medications")) {
                    errors.push(format!(
                        "{eid}: drug {} is not a catalog name (use drug_text instead)",
                        repr(m.get("drug"))
                    ));
                }
                if !key.truthy() {
                    errors.push(format!("{eid}: medication without drug or drug_text"));
                }
                let action = m.get("action").as_str();
                if matches!(action, Some("start" | "historical")) {
                    started.insert(key);
                } else if action == Some("stop") {
                    if !started.contains(key) {
                        errors.push(format!(
                            "{eid}: medication {} stopped before it was started",
                            repr(key)
                        ));
                    }
                } else {
                    errors.push(format!(
                        "{eid}: medication {} action must be start|stop|historical",
                        repr(key)
                    ));
                }
            }
            for lab in dicts(ev, "labs") {
                check_lab(&eid, lab, catalog, &mut errors);
            }
            for s in dicts(ev, "surveys") {
                if !in_dict(s.get("survey"), catalog.get("surveys")) {
                    errors.push(format!("{eid}: unknown survey {}", repr(s.get("survey"))));
                }
            }
            for s in dicts(ev, "social") {
                if !in_dict(s.get("item"), catalog.get("social")) {
                    errors.push(format!(
                        "{eid}: unknown social item {}",
                        repr(s.get("item"))
                    ));
                }
            }
            for r in dicts(ev, "reports") {
                let study = repr(r.get("study"));
                if !in_dict(r.get("study"), catalog.get("reports")) {
                    errors.push(format!("{eid}: unknown report study {study}"));
                }
                let text = text_or_empty(r.get("text"));
                if py_strip(text).is_empty() {
                    errors.push(format!("{eid}: report {study} has no text"));
                }
                let chars: Vec<char> = text.chars().collect();
                if let Some((a, b)) = search_words(&chars, &REAL_NAMES, false) {
                    errors.push(format!(
                        "{eid}: report {study} names a real organization or product ({})",
                        chars[a..b].iter().collect::<String>()
                    ));
                }
            }
            for r in dicts(ev, "procedures") {
                if !in_dict(r.get("procedure"), catalog.get("procedures")) {
                    errors.push(format!(
                        "{eid}: unknown procedure {}",
                        repr(r.get("procedure"))
                    ));
                }
            }
            for a in dicts(ev, "allergies") {
                if !in_dict(a.get("substance"), catalog.get("allergens")) {
                    warnings.push(format!(
                        "{eid}: allergen {} not in catalog (text only)",
                        repr(a.get("substance"))
                    ));
                }
                if a.get("reaction").truthy()
                    && !in_dict(a.get("reaction"), catalog.get("reactions"))
                {
                    warnings.push(format!(
                        "{eid}: reaction {} not in catalog (text only)",
                        repr(a.get("reaction"))
                    ));
                }
            }
            for d in dicts(ev, "documents") {
                n_docs += 1;
                let title = repr(d.get("title"));
                let body = py_strip(text_or_empty(d.get("body")));
                let chars: Vec<char> = body.chars().collect();
                if chars.len() < 40 {
                    errors.push(format!("{eid}: document {title} body too short"));
                }
                if let Some((a, b)) = search_words(&chars, &REAL_NAMES, false) {
                    errors.push(format!(
                        "{eid}: document {title} names a real organization or product ({}); use a role or generic description",
                        chars[a..b].iter().collect::<String>()
                    ));
                }
                if let Some((a, b)) = search_words(&chars, &FORBIDDEN, true) {
                    errors.push(format!(
                        "{eid}: document {title} uses a forbidden word ({})",
                        chars[a..b].iter().collect::<String>()
                    ));
                }
                if let Some(first) = bodies.get(body) {
                    errors.push(format!("{eid}: document body duplicates {first}"));
                }
                bodies.insert(body.to_string(), eid.clone());
                let when = if d.get("date").truthy() {
                    d.get("date")
                } else {
                    start
                };
                if let Some(when) = when.as_str().filter(|w| STAMP.is_match(w)) {
                    let on = head(when, 10);
                    let on_date = py_date(on).unwrap_or_else(|| {
                        panic!("ValueError: Invalid isoformat string: {}", repr_str(on))
                    });
                    let actual = age_on(birth, on_date);
                    for (at, end, stated) in age_in_text(&chars) {
                        if (stated - actual).abs() >= 1 && stated < 120 {
                            let ctx = &chars[at.saturating_sub(40)..(end + 10).min(chars.len())];
                            if search_words(ctx, &KIN, true).is_none() {
                                errors.push(format!(
                                    "{eid}: note says {stated}-year-old but patient is {actual} on {on}"
                                ));
                            }
                        }
                    }
                }
            }
            if in_list(ev.get("kind"), &VISITS) && !ev.get("documents").truthy() {
                errors.push(format!("{eid}: visit without a note"));
            }
        }
        let target = seed.get("target_documents");
        if (n_docs as f64) < 0.6 * target.as_f64().expect("seed target_documents") {
            warnings.push(format!(
                "{mrn}: {n_docs} documents vs target {}",
                py_str(target)
            ));
        }
        let first = events
            .iter()
            .filter(|e| e.get("kind").as_str() != Some("outside_records"))
            .map(|e| match e.get("start") {
                _ if !e.contains("start") => "9999",
                Json::Str(s) => s.as_str(),
                other => panic!("TypeError: cannot order start {}", repr(other)),
            })
            .min()
            .expect("ValueError: min() iterable argument is empty");
        let expected = 2026
            - seed
                .get("years_with_practice")
                .as_i64()
                .expect("seed years");
        let year = py_int(head(first, 4)).unwrap_or_else(|| {
            panic!(
                "ValueError: invalid literal for int() with base 10: {}",
                repr_str(head(first, 4))
            )
        });
        if (year - expected).abs() > 2 {
            warnings.push(format!(
                "{mrn}: first practice event {} vs about {expected}",
                head(first, 10)
            ));
        }
    }
    (errors, warnings)
}

/// The panel, result and red-cell-index checks of one lab entry.
fn check_lab(eid: &str, lab: &Json, catalog: &Json, errors: &mut Vec<String>) {
    let panels = catalog.get("panels");
    let panel = lab.get("panel");
    if panel.truthy() && !in_dict(panel, panels) {
        errors.push(format!("{eid}: unknown panel {}", repr(panel)));
    }
    let results = dicts(lab, "results");
    for r in &results {
        let test = r.get("test");
        if !in_dict(test, catalog.get("labs")) {
            errors.push(format!("{eid}: unknown lab test {}", repr(test)));
        } else if !r.contains("value") && !r.contains("text") {
            errors.push(format!("{eid}: lab {} needs value or text", repr(test)));
        }
        if in_dict(panel, panels) {
            let members = panels
                .get(panel.as_str().unwrap_or_default())
                .get("members");
            if !members.as_array().iter().any(|m| py_eq(m, test)) {
                errors.push(format!(
                    "{eid}: {} is not a member of {}",
                    repr(test),
                    repr(panel)
                ));
            }
        }
    }
    // {test: value} over numeric values, the last result for a test winning.
    let mut vals: HashMap<&str, Num> = HashMap::new();
    for r in &results {
        if let (Some(t), Some(v)) = (r.get("test").as_str(), Num::of(r.get("value"))) {
            vals.insert(t, v);
        }
    }
    let stated: Vec<Num> = DIFFERENTIAL
        .iter()
        .filter_map(|t| vals.get(t).copied())
        .collect();
    let total = py_sum(&stated);
    if total.f() > 101.0 {
        errors.push(format!(
            "{eid}: stated differential totals {}% (over 100); state only the members you mean and code fills the rest to 100",
            fmt_g(total.f())
        ));
    } else if stated.len() == DIFFERENTIAL.len() && !(98.0..=102.0).contains(&total.f()) {
        errors.push(format!(
            "{eid}: differential totals {}%, must total 100",
            fmt_g(total.f())
        ));
    }
    let [hgb, hct, rbc, mcv] = [
        "Hemoglobin",
        "Hematocrit",
        "Red Blood Cells",
        "Mean Corpuscular Volume",
    ]
    .map(|t| vals.get(t).copied());
    let truthy = |n: Option<Num>| n.is_some_and(Num::truthy);
    if truthy(rbc) && !truthy(mcv) {
        errors.push(format!(
            "{eid}: Red Blood Cells stated without Mean Corpuscular Volume; state Hgb and MCV (code derives Hct, RBC, MCH and MCHC) or state the indices together"
        ));
    }
    if let (Some(hct), Some(rbc), Some(mcv)) = (hct, rbc, mcv)
        && hct.truthy()
        && rbc.truthy()
        && mcv.truthy()
        && (hct.f() / rbc.f() * 10.0 - mcv.f()).abs() > 3.0
    {
        errors.push(format!(
            "{eid}: MCV {} disagrees with Hct/RBC ({})",
            mcv.str(),
            fmt_f(hct.f() / rbc.f() * 10.0, 0)
        ));
    }
    let implied_hct = match (hct, rbc, mcv) {
        (Some(h), _, _) if h.truthy() => Some(h.f()),
        (_, Some(r), Some(m)) if r.truthy() && m.truthy() => Some(r.f() * m.f() / 10.0),
        _ => None,
    };
    if let (Some(hgb), Some(implied)) = (hgb, implied_hct)
        && hgb.truthy()
        && implied != 0.0
        && !(30.0..=37.0).contains(&(hgb.f() / implied * 100.0))
    {
        errors.push(format!(
            "{eid}: Hgb {} with Hct {} gives MCHC {}, outside what an analyzer reports (30-37)",
            hgb.str(),
            fmt_f(implied, 1),
            fmt_f(hgb.f() / implied * 100.0, 1)
        ));
    }
}

/// `AGE_IN_TEXT.finditer`, Python's `\b(\d{2,3})[- ](?:year|yr)s?[- ]old\b` with `re.I`:
/// (start, end, stated age) in characters.
fn age_in_text(text: &[char]) -> Vec<(usize, usize, i64)> {
    let mut out = Vec::new();
    let mut at = 0;
    while at < text.len() {
        match age_at(text, at) {
            Some((end, digits)) => {
                let stated = text[at..at + digits]
                    .iter()
                    .fold(0, |n, c| n * 10 + digit_value(*c).expect("decimal"));
                out.push((at, end, stated));
                at = end;
            }
            None => at += 1,
        }
    }
    out
}

/// One anchored match of the age pattern at `at`, trying alternatives in Python's order:
/// (end, number of digits).
fn age_at(text: &[char], at: usize) -> Option<(usize, usize)> {
    if !boundary(text, at) {
        return None;
    }
    let sep = |i: usize| text.get(i).is_some_and(|c| *c == '-' || *c == ' ');
    for digits in [3, 2] {
        if at + digits > text.len() || !text[at..at + digits].iter().all(|c| is_decimal(*c)) {
            continue;
        }
        let q = at + digits;
        if !sep(q) {
            continue;
        }
        for unit in ["year", "yr"] {
            let Some(r) = match_at(text, q + 1, unit, true) else {
                continue;
            };
            let plural = usize::from(text.get(r).is_some_and(|c| fold_eq('s', *c)));
            for s in (0..=plural).rev() {
                let t = r + s;
                if !sep(t) {
                    continue;
                }
                if let Some(end) = match_at(text, t + 1, "old", true)
                    && boundary(text, end)
                {
                    return Some((end, digits));
                }
            }
        }
    }
    None
}

/// Python's `re.search(r"\b(w1|w2|...)\b", text)` (with `re.I` when `ignore_case`): the first
/// match as character offsets. Every word starts and ends with a word character.
fn search_words(text: &[char], words: &[&str], ignore_case: bool) -> Option<(usize, usize)> {
    for at in 0..text.len() {
        let mut checked = false;
        for w in words {
            let Some(end) = match_at(text, at, w, ignore_case) else {
                continue;
            };
            if !checked {
                if !boundary(text, at) {
                    break;
                }
                checked = true;
            }
            if boundary(text, end) {
                return Some((at, end));
            }
        }
    }
    None
}

/// The end of `word` matched at `at`, or None.
fn match_at(text: &[char], at: usize, word: &str, ignore_case: bool) -> Option<usize> {
    let mut i = at;
    for p in word.chars() {
        let c = *text.get(i)?;
        if !(c == p || ignore_case && fold_eq(p, c)) {
            return None;
        }
        i += 1;
    }
    Some(i)
}

/// Python `re.I` for a lowercase ASCII pattern character: its uppercase, plus the
/// non-ASCII characters CPython's case tables also map to it.
fn fold_eq(p: char, c: char) -> bool {
    c == p
        || c == p.to_ascii_uppercase()
        || matches!(
            (p, c),
            ('i', '\u{130}' | '\u{131}') | ('k', '\u{212a}') | ('s', '\u{17f}')
        )
}

/// Python's `\b` at character offset `i`.
fn boundary(text: &[char], i: usize) -> bool {
    let before = i > 0 && is_word(text[i - 1]);
    let after = i < text.len() && is_word(text[i]);
    before != after
}

// ---------------------------------------------------------------- Python's Unicode predicates

static ALNUM: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"\A[\p{L}\p{N}]\z").expect("ALNUM"));
static NONPRINTABLE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\A[\p{C}\p{Z}]\z").expect("NONPRINTABLE"));
static DECIMAL: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\A\p{Nd}\z")
        .expect("184257ae64493754f7a23679b8ffd791c70374300dea133c1fb0b3ddd979432b")
});

fn char_is(re: &Regex, c: char) -> bool {
    re.is_match(c.encode_utf8(&mut [0; 4]))
}

/// A word character of Python's Unicode `\b` and `\w`: `str.isalnum()` or `_`.
fn is_word(c: char) -> bool {
    if c.is_ascii() {
        c.is_ascii_alphanumeric() || c == '_'
    } else {
        char_is(&ALNUM, c)
    }
}

/// `str.isprintable()` of one character.
fn is_printable(c: char) -> bool {
    c == ' ' || !char_is(&NONPRINTABLE, c)
}

/// `str.isdecimal()` of one character: Python's `\d`.
fn is_decimal(c: char) -> bool {
    if c.is_ascii() {
        c.is_ascii_digit()
    } else {
        char_is(&DECIMAL, c)
    }
}

/// The value of a Unicode decimal digit. Decimal digits are encoded in contiguous runs of
/// complete 0-9 sets, so the offset into the run gives the value.
fn digit_value(c: char) -> Option<i64> {
    if !is_decimal(c) {
        return None;
    }
    let mut zero = c as u32;
    while zero > 0 && char::from_u32(zero - 1).is_some_and(is_decimal) {
        zero -= 1;
    }
    Some(i64::from((c as u32 - zero) % 10))
}

/// `str.isspace()` of one character: Unicode White_Space plus the ASCII separators U+001C-U+001F.
fn is_space(c: char) -> bool {
    c.is_whitespace() || ('\u{1c}'..='\u{1f}').contains(&c)
}

fn py_strip(s: &str) -> &str {
    s.trim_matches(is_space)
}

/// `(x or "")` for a value that is then used as a string.
fn text_or_empty(v: &Json) -> &str {
    match v {
        Json::Str(s) => s,
        _ if !v.truthy() => "",
        other => panic!("AttributeError: {} has no attribute 'strip'", repr(other)),
    }
}

/// `s[:n]` in characters.
fn head(s: &str, n: usize) -> &str {
    s.char_indices().nth(n).map_or(s, |(i, _)| &s[..i])
}

// ---------------------------------------------------------------- Python values

/// Python `repr` of a str.
fn repr_str(s: &str) -> String {
    let quote = if s.contains('\'') && !s.contains('"') {
        '"'
    } else {
        '\''
    };
    let mut out = String::with_capacity(s.len() + 2);
    out.push(quote);
    for c in s.chars() {
        match c {
            '\\' => out.push_str("\\\\"),
            '\t' => out.push_str("\\t"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            c if c == quote => {
                out.push('\\');
                out.push(c);
            }
            c if (c as u32) < 0x20 || c == '\u{7f}' => {
                out.push_str(&format!("\\x{:02x}", c as u32))
            }
            c if c.is_ascii() || is_printable(c) => out.push(c),
            c if (c as u32) <= 0xff => out.push_str(&format!("\\x{:02x}", c as u32)),
            c if (c as u32) <= 0xffff => out.push_str(&format!("\\u{:04x}", c as u32)),
            c => out.push_str(&format!("\\U{:08x}", c as u32)),
        }
    }
    out.push(quote);
    out
}

fn repr_float(f: f64) -> String {
    if f.is_nan() {
        "nan".into()
    } else if f.is_infinite() {
        if f > 0.0 { "inf" } else { "-inf" }.into()
    } else {
        float_repr(f)
    }
}

/// Python `repr` of a value as `json.loads` built it.
fn repr(v: &Json) -> String {
    match v {
        Json::Null => "None".into(),
        Json::Bool(b) => if *b { "True" } else { "False" }.into(),
        Json::Int(i) => i.to_string(),
        Json::Float(f) => repr_float(*f),
        Json::Str(s) => repr_str(s),
        Json::Arr(a) => format!("[{}]", a.iter().map(repr).collect::<Vec<_>>().join(", ")),
        Json::Obj(m) => format!(
            "{{{}}}",
            m.iter()
                .map(|(k, v)| format!("{}: {}", repr_str(k), repr(v)))
                .collect::<Vec<_>>()
                .join(", ")
        ),
    }
}

fn repr_list(items: Vec<Json>) -> String {
    repr(&Json::Arr(items))
}

/// Python `str` of a value.
fn py_str(v: &Json) -> String {
    match v {
        Json::Str(s) => s.clone(),
        other => repr(other),
    }
}

/// Python `==` between two JSON-built values.
fn py_eq(a: &Json, b: &Json) -> bool {
    match (a, b) {
        (Json::Null, Json::Null) => true,
        (Json::Str(x), Json::Str(y)) => x == y,
        (Json::Arr(x), Json::Arr(y)) => {
            x.len() == y.len() && x.iter().zip(y).all(|(x, y)| py_eq(x, y))
        }
        (Json::Obj(x), Json::Obj(y)) => {
            x.len() == y.len()
                && x.iter()
                    .all(|(k, v)| y.iter().any(|(k2, v2)| k == k2 && py_eq(v, v2)))
        }
        _ => match (Num::of(a), Num::of(b)) {
            (Some(Num::Int(x)), Some(Num::Int(y))) => x == y,
            (Some(x), Some(y)) => x.f() == y.f(),
            _ => false,
        },
    }
}

/// Python raises TypeError when a list or dict is used as a set member or dict key.
fn hash(v: &Json) {
    if matches!(v, Json::Arr(_) | Json::Obj(_)) {
        panic!("TypeError: unhashable type: {}", repr(v));
    }
}

/// `v in d` for a dict keyed by strings.
fn in_dict(v: &Json, d: &Json) -> bool {
    hash(v);
    v.as_str().is_some_and(|s| d.contains(s))
}

/// `v in (s1, s2, ...)`.
fn in_list(v: &Json, list: &[&str]) -> bool {
    v.as_str().is_some_and(|s| list.contains(&s))
}

/// A Python set (or dict keys) of JSON-built hashable values, in insertion order.
#[derive(Default)]
struct KeySet<'a>(Vec<&'a Json>);

impl<'a> KeySet<'a> {
    fn insert(&mut self, v: &'a Json) {
        hash(v);
        if !self.contains(v) {
            self.0.push(v);
        }
    }

    fn contains(&self, v: &Json) -> bool {
        hash(v);
        self.0.iter().any(|x| py_eq(x, v))
    }

    fn len(&self) -> usize {
        self.0.len()
    }
}

/// `sorted(x for x in s if x)`; Python raises when strings and numbers mix.
fn sorted_truthy(s: &KeySet) -> Vec<Json> {
    let mut items: Vec<Json> =
        s.0.iter()
            .filter(|x| x.truthy())
            .map(|x| (*x).clone())
            .collect();
    if items.iter().all(|x| x.as_str().is_some()) {
        items.sort_by(|a, b| a.as_str().cmp(&b.as_str()));
    } else if items.iter().all(|x| Num::of(x).is_some()) {
        items.sort_by(|a, b| {
            let (a, b) = (Num::of(a).unwrap().f(), Num::of(b).unwrap().f());
            a.partial_cmp(&b).unwrap_or(std::cmp::Ordering::Equal)
        });
    } else {
        panic!("TypeError: '<' not supported between str and a number");
    }
    items
}

/// The value as a dict, as Python's `x.get(...)` required.
fn dict(v: &Json) -> &Json {
    match v {
        Json::Obj(_) => v,
        other => panic!("AttributeError: {} has no attribute 'get'", repr(other)),
    }
}

/// `for x in v` where each x must be a dict.
fn iter_dicts(v: &Json) -> Vec<&Json> {
    match v {
        Json::Arr(a) => a.iter().map(dict).collect(),
        // Iterating an empty string or dict yields nothing; anything else is not a dict.
        Json::Str(s) if s.is_empty() => Vec::new(),
        Json::Obj(m) if m.is_empty() => Vec::new(),
        Json::Str(_) | Json::Obj(_) => {
            panic!("AttributeError: 'str' object has no attribute 'get'")
        }
        other => panic!("TypeError: {} is not iterable", repr(other)),
    }
}

/// `for x in obj.get(key, [])` where each x must be a dict.
fn dicts<'a>(obj: &'a Json, key: &str) -> Vec<&'a Json> {
    if obj.contains(key) {
        iter_dicts(obj.get(key))
    } else {
        Vec::new()
    }
}

/// A Python number as `json.loads` built it; `isinstance(x, (int, float))` includes bool.
#[derive(Clone, Copy)]
enum Num {
    Int(i64),
    Float(f64),
    Bool(bool),
}

impl Num {
    fn of(v: &Json) -> Option<Num> {
        match v {
            Json::Int(i) => Some(Num::Int(*i)),
            Json::Float(f) => Some(Num::Float(*f)),
            Json::Bool(b) => Some(Num::Bool(*b)),
            _ => None,
        }
    }

    fn f(self) -> f64 {
        match self {
            Num::Int(i) => i as f64,
            Num::Float(f) => f,
            Num::Bool(b) => f64::from(u8::from(b)),
        }
    }

    fn truthy(self) -> bool {
        self.f() != 0.0
    }

    /// Python `str`.
    fn str(self) -> String {
        match self {
            Num::Int(i) => i.to_string(),
            Num::Float(f) => repr_float(f),
            Num::Bool(b) => if b { "True" } else { "False" }.into(),
        }
    }
}

/// Python's `sum()`: integers add exactly until the first float, then the rest add with
/// Neumaier compensation (CPython 3.12+).
fn py_sum(items: &[Num]) -> Num {
    let mut int_total: i64 = 0;
    let mut rest = items.iter();
    let (mut hi, mut lo) = loop {
        match rest.next() {
            None => return Num::Int(int_total),
            Some(Num::Float(x)) => break (int_total as f64 + x, 0.0),
            Some(n) => int_total += n.f() as i64,
        }
    };
    for n in rest {
        let x = n.f();
        let t = hi + x;
        if hi.abs() >= x.abs() {
            lo += (hi - t) + x;
        } else {
            lo += (x - t) + hi;
        }
        hi = t;
    }
    if lo != 0.0 && lo.is_finite() {
        hi += lo;
    }
    Num::Float(hi)
}

/// Python `format(x, f".{prec}f")`.
fn fmt_f(x: f64, prec: usize) -> String {
    if x.is_nan() {
        return "nan".into();
    }
    format!("{x:.prec$}")
}

/// Python `format(x, "g")`: six significant digits, trailing zeros dropped, scientific
/// below 1e-4 and from 1e6.
fn fmt_g(x: f64) -> String {
    if x.is_nan() {
        return "nan".into();
    }
    if x.is_infinite() {
        return if x > 0.0 { "inf" } else { "-inf" }.into();
    }
    if x == 0.0 {
        return if x.is_sign_negative() { "-0" } else { "0" }.into();
    }
    let sci = format!("{x:.5e}");
    let (mantissa, exp) = sci.split_once('e').expect("exponent");
    let exp: i32 = exp.parse().expect("exponent digits");
    let trim = |s: String| -> String {
        if s.contains('.') {
            s.trim_end_matches('0').trim_end_matches('.').to_string()
        } else {
            s
        }
    };
    if (-4..6).contains(&exp) {
        trim(format!("{x:.*}", (5 - exp) as usize))
    } else {
        let sign = if exp < 0 { '-' } else { '+' };
        format!("{}e{sign}{:02}", trim(mantissa.to_string()), exp.abs())
    }
}

/// Python `int(s)` for a str: surrounding whitespace, a sign, Unicode decimal digits with
/// single underscores between them.
fn py_int(s: &str) -> Option<i64> {
    let s = py_strip(s);
    let (neg, digits) = match s.chars().next()? {
        '-' => (true, &s[1..]),
        '+' => (false, &s[1..]),
        _ => (false, s),
    };
    let mut n: i64 = 0;
    let mut last_digit = false;
    for c in digits.chars() {
        if c == '_' {
            if !last_digit {
                return None;
            }
            last_digit = false;
            continue;
        }
        n = n.checked_mul(10)?.checked_add(digit_value(c)?)?;
        last_digit = true;
    }
    last_digit.then_some(if neg { -n } else { n })
}

/// `date.fromisoformat(s)` for the `YYYY-MM-DD` shape a stamp has (ASCII digits only).
fn py_date(s: &str) -> Option<Date> {
    let b = s.as_bytes();
    if b.len() != 10 || b[4] != b'-' || b[7] != b'-' {
        return None;
    }
    let num = |r: std::ops::Range<usize>| -> Option<i16> {
        b[r].iter().try_fold(0i16, |n, d| {
            d.is_ascii_digit().then(|| n * 10 + i16::from(d - b'0'))
        })
    };
    let (y, m, d) = (num(0..4)?, num(5..7)?, num(8..10)?);
    if y < 1 {
        return None;
    }
    Date::new(y, i8::try_from(m).ok()?, i8::try_from(d).ok()?).ok()
}

/// Whether `datetime.fromisoformat(s)` accepts a string STAMP matched: ASCII digits, a real
/// date, 00:00-23:59, or 24:00 when the next day is representable.
fn py_datetime_ok(s: &str) -> bool {
    let b = s.as_bytes();
    if b.len() != 16
        || !b[11..]
            .iter()
            .enumerate()
            .all(|(i, c)| i == 2 || c.is_ascii_digit())
    {
        return false;
    }
    let Some(date) = py_date(&s[..10]) else {
        return false;
    };
    let hour = (b[11] - b'0') * 10 + (b[12] - b'0');
    let minute = (b[14] - b'0') * 10 + (b[15] - b'0');
    match hour {
        0..=23 => minute <= 59,
        24 => minute == 0 && (date.year() < 9999 || (date.month(), date.day()) != (12, 31)),
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sha2::{Digest, Sha256};

    /// sha256 over U+0000..U+10FFFF (surrogates skipped) of '1'/'0' for a predicate; the
    /// expected digests come from CPython 3.14 (Unicode 16.0.0):
    /// `''.join('1' if f(chr(c)) else '0' for c in range(0x110000) if not 0xd800 <= c < 0xe000)`.
    fn digest(f: impl Fn(char) -> bool) -> String {
        let bits: String = (0..0x110000u32)
            .filter_map(char::from_u32)
            .map(|c| if f(c) { '1' } else { '0' })
            .collect();
        Sha256::digest(bits.as_bytes())
            .iter()
            .map(|b| format!("{b:02x}"))
            .collect()
    }

    #[test]
    fn unicode_predicates_match_cpython() {
        // c.isalnum() or c == '_'
        assert_eq!(
            digest(is_word),
            "e554238be8eb26e1d273e5c789db57a76d431532f1c7ee79ba1476f8ab8800bb"
        );
        // c.isprintable()
        assert_eq!(
            digest(is_printable),
            "0c288a3c50d6e1cf68efda776bd66b6c20d952e03eb5b7328308c6d55691d095"
        );
        // c.isdecimal()
        assert_eq!(
            digest(is_decimal),
            "184257ae64493754f7a23679b8ffd791c70374300dea133c1fb0b3ddd979432b"
        );
        // c.isspace()
        assert_eq!(
            digest(is_space),
            "c288835013403941e55c84f9c3f66ba0c3c11f80bd5745e95622db8c61d3bf81"
        );
        // c.isdecimal() and int(c) == 7
        assert_eq!(
            digest(|c| digit_value(c) == Some(7)),
            "486c1061d4f47c55131b4e3f4a97245b35647dc789c54b3dbc6385e0ab4d87d7"
        );
    }
}
