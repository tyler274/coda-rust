//! How a set of charts compares with real practice: the one owner of every
//! statistical texture rule.
//!
//! A [`Profile`] measures charts as FHIR R4 JSON, so the same code measures the
//! bank and a reference corpus. `data/realism.json` is the reference, written by
//! `patient-bank realism --gym <dir>`. Each metric names its source:
//!
//! ```text
//! coda-gym   measured from coda-gym charts (resources/ only): the share of the
//!            chart that is office notes, and note length by kind
//! device     automated cuffs and oximeters read any final digit with equal odds,
//!            and an exact repeat of a blood-pressure pair is rare
//! practice   the practice books visits Monday to Friday, about evenly
//! ```
//!
//! coda-gym charts are authored too, and they carry the same biases on the device
//! and calendar metrics (most systolics even, a third of notes on Tuesdays), so
//! those references come from how devices and clinics work, not from coda-gym.
//!
//! `check` gates a unit on the metrics that are reliable for one chart; `validate`
//! reports every metric across the panel.

use std::collections::{BTreeMap, HashSet};
use std::fs;
use std::path::Path;

use base64::Engine as _;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::ledger::PRIMARY_CARE;
use crate::{Result, time};

const DAYS: [&str; 7] = ["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"];

/// Document kinds compared across corpora, from a document's type text.
fn kind(type_text: &str) -> &'static str {
    let t = type_text.to_lowercase();
    if t.contains("administrative")
        || t.contains("scheduling")
        || t.contains("records")
        || t.contains("care coordination")
        || t.contains("registration")
    {
        "office"
    } else if t.contains("nursing") {
        "nursing"
    } else if t.contains("telephone") {
        "telephone"
    } else if t.contains("message") {
        "message"
    } else if t.ends_with("progress notes") {
        "visit"
    } else {
        "other"
    }
}

/// What a set of charts looks like, as counts that add across patients.
#[derive(Clone, Debug, Default)]
pub struct Profile {
    pub patients: usize,
    pub systolic_digit: [usize; 10],
    pub heart_rate_digit: [usize; 10],
    pub visit_weekday: [usize; 7],
    pub bp_pairs: usize,
    pub bp_repeats: usize,
    pub temperatures: BTreeMap<String, usize>,
    pub documents: usize,
    pub office_documents: usize,
    pub words: BTreeMap<&'static str, Vec<usize>>,
    /// Booked practice visits, and how many were cancelled or missed.
    pub booked: usize,
    pub missed: usize,
}

impl Profile {
    /// Add one patient's chart: (family, resource) for every resource.
    pub fn add_patient<'a>(&mut self, resources: impl IntoIterator<Item = (&'a str, &'a Value)>) {
        self.patients += 1;
        let mut pairs: HashSet<(i64, i64)> = HashSet::new();
        for (fam, r) in resources {
            match fam {
                "Observation" => self.add_observation(r, &mut pairs),
                "DocumentReference" => self.add_document(r),
                "Encounter" => self.add_encounter(r),
                _ => {}
            }
        }
    }

    fn add_observation(&mut self, r: &Value, pairs: &mut HashSet<(i64, i64)>) {
        let code = r["code"]["coding"][0]["code"].as_str().unwrap_or("");
        let value = |v: &Value| v["valueQuantity"]["value"].as_f64();
        match code {
            "85354-9" => {
                let (Some(s), Some(d)) = (value(&r["component"][0]), value(&r["component"][1]))
                else {
                    return;
                };
                let (s, d) = (s.round() as i64, d.round() as i64);
                self.systolic_digit[(s.rem_euclid(10)) as usize] += 1;
                self.bp_pairs += 1;
                if !pairs.insert((s, d)) {
                    self.bp_repeats += 1;
                }
            }
            "8867-4" => {
                if let Some(h) = value(r) {
                    self.heart_rate_digit[((h.round() as i64).rem_euclid(10)) as usize] += 1;
                }
            }
            "8310-5" => {
                if let Some(t) = value(r) {
                    *self.temperatures.entry(format!("{t:.1}")).or_default() += 1;
                }
            }
            _ => {}
        }
    }

    fn add_document(&mut self, r: &Value) {
        let k = kind(r["type"]["text"].as_str().unwrap_or(""));
        self.documents += 1;
        self.office_documents += usize::from(k == "office");
        let data = r["content"][0]["attachment"]["data"].as_str().unwrap_or("");
        let bytes = base64::engine::general_purpose::STANDARD
            .decode(data)
            .unwrap_or_default();
        let text = String::from_utf8_lossy(&bytes);
        // the header (title, date, patient, service) ends at the first blank line
        let body = text.split_once("\n\n").map_or(&*text, |(_, b)| b);
        self.words
            .entry(k)
            .or_default()
            .push(body.split_whitespace().count());
    }

    fn add_encounter(&mut self, r: &Value) {
        let practice = r["serviceType"]["text"].as_str() == Some(PRIMARY_CARE);
        let class = r["class"]["code"].as_str().unwrap_or("");
        if !practice || !matches!(class, "AMB" | "VR") {
            return;
        }
        if let Some(start) = r["period"]["start"].as_str().and_then(time::eastern_of) {
            let d = start.0.date().weekday().to_sunday_zero_offset();
            self.visit_weekday[d as usize] += 1;
        }
    }

    pub fn add(&mut self, other: &Profile) {
        self.patients += other.patients;
        for i in 0..10 {
            self.systolic_digit[i] += other.systolic_digit[i];
            self.heart_rate_digit[i] += other.heart_rate_digit[i];
        }
        for i in 0..7 {
            self.visit_weekday[i] += other.visit_weekday[i];
        }
        self.bp_pairs += other.bp_pairs;
        self.bp_repeats += other.bp_repeats;
        for (t, n) in &other.temperatures {
            *self.temperatures.entry(t.clone()).or_default() += n;
        }
        self.documents += other.documents;
        self.office_documents += other.office_documents;
        for (k, v) in &other.words {
            self.words.entry(k).or_default().extend(v);
        }
        self.booked += other.booked;
        self.missed += other.missed;
    }
}

/// What real practice looks like, metric by metric, with each metric's source.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Reference {
    pub sources: BTreeMap<String, String>,
    pub systolic_digit: [f64; 10],
    pub heart_rate_digit: [f64; 10],
    pub visit_weekday: [f64; 7],
    /// At most this share of a patient's BP readings repeat an earlier pair.
    pub bp_repeat_max: f64,
    /// At most this share of temperatures are one value.
    pub temperature_mode_max: f64,
    pub office_share: f64,
    /// Median words per document kind.
    pub words: BTreeMap<String, f64>,
    /// Booked visits cancelled or missed, as a range.
    pub missed_share: [f64; 2],
}

impl Reference {
    pub fn load(path: &Path) -> Result<Reference> {
        let text = fs::read_to_string(path).map_err(|e| format!("{}: {e}", path.display()))?;
        Ok(serde_json::from_str(&text).map_err(|e| format!("{}: {e}", path.display()))?)
    }
}

/// Build the reference from coda-gym charts (each a `resources/` directory of
/// NDJSON) and the device and practice facts above, and write it.
pub fn build(gym: &[std::path::PathBuf], out: &Path) -> Result<Reference> {
    let mut p = Profile::default();
    for dir in gym {
        let mut rows: Vec<(&'static str, Value)> = Vec::new();
        for fam in ["Observation", "DocumentReference", "Encounter"] {
            let Ok(text) = fs::read_to_string(dir.join(format!("{fam}.ndjson"))) else {
                continue;
            };
            for line in text.lines() {
                rows.push((fam, serde_json::from_str(line)?));
            }
        }
        p.add_patient(rows.iter().map(|(f, v)| (*f, v)));
    }
    if p.documents == 0 {
        return Err("no coda-gym documents found".into());
    }
    let median = |v: &Vec<usize>| {
        let mut v = v.clone();
        v.sort_unstable();
        v[v.len() / 2] as f64
    };
    let words = p
        .words
        .iter()
        .filter(|(_, v)| v.len() >= 15)
        .map(|(k, v)| (k.to_string(), median(v)))
        .collect();
    let sources = [
        (
            "systolic_digit",
            "device: automated cuffs read any final digit",
        ),
        ("heart_rate_digit", "device: oximeters read any final digit"),
        (
            "bp_repeat_max",
            "device: exact repeats of a BP pair are rare",
        ),
        (
            "temperature_mode_max",
            "device: temperatures vary from visit to visit",
        ),
        (
            "visit_weekday",
            "practice: visits booked Monday to Friday, about evenly",
        ),
        (
            "missed_share",
            "practice: about one booked visit in twelve is missed or cancelled",
        ),
        ("office_share", "coda-gym"),
        ("words", "coda-gym"),
    ];
    let reference = Reference {
        sources: sources
            .iter()
            .map(|(k, v)| ((*k).to_string(), (*v).to_string()))
            .chain([("coda-gym charts".into(), gym.len().to_string())])
            .collect(),
        systolic_digit: [0.1; 10],
        heart_rate_digit: [0.1; 10],
        visit_weekday: [0.0, 0.2, 0.2, 0.2, 0.2, 0.2, 0.0],
        bp_repeat_max: 0.2,
        temperature_mode_max: 0.4,
        office_share: p.office_documents as f64 / p.documents as f64,
        words,
        missed_share: [0.03, 0.15],
    };
    crate::catalog::write_atomic(out, &(serde_json::to_string_pretty(&reference)? + "\n"))?;
    Ok(reference)
}

/// One metric, measured against its reference.
#[derive(Clone, Debug, Serialize)]
pub struct Drift {
    pub metric: String,
    pub measured: String,
    pub expected: String,
    pub within: bool,
}

/// Total variation distance between counts and a reference distribution.
fn distance(counts: &[usize], reference: &[f64]) -> f64 {
    let n: usize = counts.iter().sum();
    0.5 * counts
        .iter()
        .zip(reference)
        .map(|(c, r)| (*c as f64 / n as f64 - r).abs())
        .sum::<f64>()
}

/// How far a sample of `n` may sit from its reference before it is a drift:
/// small samples wander further by chance.
fn allowed(n: usize) -> f64 {
    0.15 + 1.0 / (n as f64).sqrt()
}

fn show(counts: &[usize], labels: &[&str]) -> String {
    let n: usize = counts.iter().sum();
    counts
        .iter()
        .zip(labels)
        .filter(|(c, _)| **c > 0)
        .map(|(c, l)| format!("{l} {:.0}%", 100.0 * *c as f64 / n as f64))
        .collect::<Vec<_>>()
        .join(", ")
}

/// Compare a profile with the reference. `chart` keeps to the metrics that are
/// reliable for one patient's chart and to samples large enough to judge.
pub fn compare(p: &Profile, r: &Reference, chart: bool) -> Vec<Drift> {
    let mut out = Vec::new();
    let digits: Vec<String> = (0..10).map(|d| d.to_string()).collect();
    let digit_labels: Vec<&str> = digits.iter().map(String::as_str).collect();
    for (metric, counts, reference, labels, min) in [
        (
            "systolic final digit",
            &p.systolic_digit[..],
            &r.systolic_digit[..],
            &digit_labels[..],
            15,
        ),
        (
            "heart-rate final digit",
            &p.heart_rate_digit[..],
            &r.heart_rate_digit[..],
            &digit_labels[..],
            15,
        ),
        (
            "practice-visit weekday",
            &p.visit_weekday[..],
            &r.visit_weekday[..],
            &DAYS[..],
            12,
        ),
    ] {
        let n: usize = counts.iter().sum();
        if n < min {
            continue;
        }
        let d = distance(counts, reference);
        out.push(Drift {
            metric: metric.into(),
            measured: format!("{} (distance {d:.2}, n {n})", show(counts, labels)),
            expected: format!("distance at most {:.2}", allowed(n)),
            within: d <= allowed(n),
        });
    }
    if p.bp_pairs >= 15 {
        let share = p.bp_repeats as f64 / p.bp_pairs as f64;
        out.push(Drift {
            metric: "repeated BP pairs".into(),
            measured: format!("{:.0}% of {}", 100.0 * share, p.bp_pairs),
            expected: format!("at most {:.0}%", 100.0 * r.bp_repeat_max),
            within: share <= r.bp_repeat_max,
        });
    }
    let temps: usize = p.temperatures.values().sum();
    if temps >= 10 {
        let (value, top) = p
            .temperatures
            .iter()
            .max_by_key(|(t, n)| (**n, *t))
            .expect("temperatures");
        let share = *top as f64 / temps as f64;
        out.push(Drift {
            metric: "most common temperature".into(),
            measured: format!("{value} C in {:.0}% of {temps}", 100.0 * share),
            expected: format!("at most {:.0}%", 100.0 * r.temperature_mode_max),
            within: share <= r.temperature_mode_max,
        });
    }
    if p.documents >= 40 {
        let share = p.office_documents as f64 / p.documents as f64;
        out.push(Drift {
            metric: "office notes".into(),
            measured: format!("{:.0}% of {} documents", 100.0 * share, p.documents),
            expected: format!(
                "at least {:.0}% (coda-gym {:.0}%)",
                50.0 * r.office_share,
                100.0 * r.office_share
            ),
            within: share >= 0.5 * r.office_share,
        });
    }
    if chart {
        return out;
    }
    for (k, words) in &p.words {
        let Some(want) = r.words.get(*k) else {
            continue;
        };
        if words.len() < 15 {
            continue;
        }
        let mut v = words.clone();
        v.sort_unstable();
        let have = v[v.len() / 2] as f64;
        out.push(Drift {
            metric: format!("median words, {k} notes"),
            measured: format!("{have:.0} (n {})", v.len()),
            expected: format!(
                "{:.0} to {:.0} (coda-gym {want:.0})",
                0.5 * want,
                2.0 * want
            ),
            within: have >= 0.5 * want && have <= 2.0 * want,
        });
    }
    if p.booked >= 50 {
        let share = p.missed as f64 / p.booked as f64;
        out.push(Drift {
            metric: "missed or cancelled visits".into(),
            measured: format!("{:.1}% of {}", 100.0 * share, p.booked),
            expected: format!(
                "{:.0}% to {:.0}%",
                100.0 * r.missed_share[0],
                100.0 * r.missed_share[1]
            ),
            within: share >= r.missed_share[0] && share <= r.missed_share[1],
        });
    }
    out
}
