//! What the bank's charts repeat from one another.
//!
//! Each unit is authored alone, so authors fall back on the same defaults: the same
//! lab value, the same relative's name, the same turn of phrase. Across accepted
//! ledgers this finds:
//!
//! ```text
//! names    capitalized words mid-sentence that never appear lowercase (people,
//!          places, pets), shared by a few units but not by the practice
//! phrases  six-word runs in notes, shared by a few units but not by the practice
//!          (a practice template is shared by many)
//! values   a stated lab value used by far more units than the values beside it
//!          (2.14 against 2.13 and 2.15): a default, not a measurement
//! ```
//!
//! `validate` reports it for the bank, and `brief` hands an author the list from
//! every other unit as taken.

use std::collections::{HashMap, HashSet};
use std::sync::LazyLock;

use regex::Regex;
use serde::Serialize;

use crate::ledger::Ledger;
use crate::seeds::Seeds;

/// Words in a phrase.
const PHRASE: usize = 6;
/// A name or phrase in more than this share of units belongs to the practice.
const PRACTICE: f64 = 0.2;
/// A value must be used by this many units, and this many times its neighbors.
const VALUE_UNITS: usize = 3;
const SPIKE: f64 = 3.0;
/// How many of each the taken list shows.
const LISTED: usize = 80;

static WORD: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"[A-Za-z][A-Za-z'-]*|\d+(?:\.\d+)?").expect("regex"));
static PROPER: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"[A-Za-z0-9,;)] ([A-Z][a-z]{2,})\b").expect("regex"));

/// Each item with the number of units that use it.
#[derive(Debug, Default, Serialize)]
pub struct Repeats {
    pub names: Vec<(String, usize)>,
    pub phrases: Vec<(String, usize)>,
    pub values: Vec<(String, usize)>,
}

/// Count what `ledgers` use in at least `units_at_least` units (2 for what the bank
/// repeats, 1 for what is taken). A unit's `own` words (its patients' names and
/// towns) do not count as names in that unit.
pub fn find<'a>(
    ledgers: impl IntoIterator<Item = &'a Ledger>,
    own: &HashMap<String, HashSet<String>>,
    units_at_least: usize,
) -> Repeats {
    let mut names: HashMap<String, HashSet<&str>> = HashMap::new();
    let mut phrases: HashMap<String, HashSet<&str>> = HashMap::new();
    // test -> stated value as written -> units
    let mut values: HashMap<&str, HashMap<String, HashSet<&str>>> = HashMap::new();
    let mut lowercase: HashSet<String> = HashSet::new();
    let mut units = 0;
    for l in ledgers {
        units += 1;
        let unit = l.unit.as_str();
        let mine = own.get(unit);
        for ev in l.patients.iter().flat_map(|p| &p.events) {
            for r in ev.labs.iter().flat_map(|lab| &lab.results) {
                if let Some(v) = &r.value {
                    values
                        .entry(r.test.as_str())
                        .or_default()
                        .entry(v.to_string())
                        .or_default()
                        .insert(unit);
                }
            }
            for d in &ev.documents {
                let words: Vec<&str> = WORD.find_iter(&d.body).map(|m| m.as_str()).collect();
                lowercase.extend(
                    words
                        .iter()
                        .filter(|w| w.starts_with(|c: char| c.is_ascii_lowercase()))
                        .map(|w| w.to_string()),
                );
                for m in PROPER.captures_iter(&d.body) {
                    if !mine.is_some_and(|w| w.contains(&m[1])) {
                        names.entry(m[1].to_string()).or_default().insert(unit);
                    }
                }
                for w in words
                    .windows(PHRASE)
                    .filter(|w| w.iter().all(|t| t.starts_with(|c: char| c.is_alphabetic())))
                {
                    phrases
                        .entry(w.join(" ").to_lowercase())
                        .or_default()
                        .insert(unit);
                }
            }
        }
    }
    let few = (PRACTICE * units as f64).max(2.0) as usize;
    names.retain(|n, _| !lowercase.contains(&n.to_lowercase()));
    let names = ranked(
        names.into_iter().map(|(k, u)| (k, u.len())),
        units_at_least,
        few,
    );
    let phrases = distinct(ranked(
        phrases.into_iter().map(|(k, u)| (k, u.len())),
        units_at_least.max(2),
        few,
    ));
    let mut spikes = Vec::new();
    for (test, by) in &values {
        let at: HashMap<String, usize> = by.iter().map(|(v, u)| (v.clone(), u.len())).collect();
        for (v, n) in &at {
            let Some((x, step)) = parse(v) else { continue };
            let side = |d: f64| at.get(&format!("{:.*}", decimals(v), x + d)).copied();
            let beside = (side(-step).unwrap_or(0) + side(step).unwrap_or(0)) as f64 / 2.0;
            if *n >= VALUE_UNITS && *n as f64 >= SPIKE * beside.max(1.0) {
                spikes.push((format!("{test} {v}"), *n));
            }
        }
    }
    let values = ranked(spikes.into_iter(), VALUE_UNITS, usize::MAX);
    Repeats {
        names,
        phrases,
        values,
    }
}

/// A value and its last digit's step: 2.14 is (2.14, 0.01).
fn parse(v: &str) -> Option<(f64, f64)> {
    let x: f64 = v.parse().ok()?;
    Some((x, 10f64.powi(-(decimals(v) as i32))))
}

fn decimals(v: &str) -> usize {
    v.split_once('.').map_or(0, |(_, d)| d.len())
}

fn ranked(
    items: impl Iterator<Item = (String, usize)>,
    at_least: usize,
    at_most: usize,
) -> Vec<(String, usize)> {
    let mut out: Vec<(String, usize)> = items
        .filter(|(_, n)| (at_least..=at_most).contains(n))
        .collect();
    out.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
    out
}

/// Overlapping runs joined back into the phrase they came from: a run whose first
/// five words end another run continues it. A joined phrase keeps the fewest units
/// any of its runs has.
fn distinct(runs: Vec<(String, usize)>) -> Vec<(String, usize)> {
    let split = |r: &str| -> (String, String) {
        let w: Vec<&str> = r.split(' ').collect();
        (w[..PHRASE - 1].join(" "), w[1..].join(" "))
    };
    let count: HashMap<&str, usize> = runs.iter().map(|(r, n)| (r.as_str(), *n)).collect();
    let by_head: HashMap<String, &str> =
        runs.iter().map(|(r, _)| (split(r).0, r.as_str())).collect();
    let tails: HashSet<String> = runs.iter().map(|(r, _)| split(r).1).collect();
    let mut out = Vec::new();
    for (start, _) in runs.iter().filter(|(r, _)| !tails.contains(&split(r).0)) {
        let (mut phrase, mut n, mut at) = (start.clone(), count[start.as_str()], start.as_str());
        let mut seen = HashSet::from([at]);
        while let Some(next) = by_head.get(&split(at).1).copied() {
            if !seen.insert(next) {
                break;
            }
            phrase.push(' ');
            phrase.push_str(next.rsplit(' ').next().expect("a word"));
            n = n.min(count[next]);
            at = next;
        }
        out.push((phrase, n));
    }
    out.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));
    out
}

/// Each unit's own words: its patients' names and towns.
pub fn own(seeds: &Seeds) -> HashMap<String, HashSet<String>> {
    seeds
        .units
        .iter()
        .map(|u| {
            let words = u
                .members
                .iter()
                .flat_map(|m| [m.given.as_str(), m.family.as_str(), m.town.as_str()])
                .flat_map(|s| s.split([' ', '-']))
                .map(String::from)
                .collect();
            (u.unit.clone(), words)
        })
        .collect()
}

/// The taken list an author reads before writing.
pub fn taken(r: &Repeats, units: usize) -> String {
    let block = |title: &str, items: &[(String, usize)]| {
        let body: Vec<String> = items
            .iter()
            .take(LISTED)
            .map(|(k, n)| format!("- {k} ({n})"))
            .collect();
        format!("## {title}\n{}\n", body.join("\n"))
    };
    let names: Vec<&str> = r.names.iter().map(|(k, _)| k.as_str()).collect();
    format!(
        "# Taken by other charts\n\nThe other {units} accepted units already use these. Give \
         this patient their own people, places, pets, wording and values. A drug, a test, an \
         exam sign or the practice's own exam template may repeat.\n\n\
         ## Names (people, places, pets and things)\n{}\n\n{}\n{}",
        names.join(", "),
        block("Phrases, with how many units use each", &r.phrases),
        block(
            "Lab values used far more often than the values beside them",
            &r.values
        ),
    )
}
