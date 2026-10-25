//! One year of a patient's care, rolled by code before anyone writes it.
//!
//! A chart is written forward, one year at a time, by an author who sees only the
//! past. Code decides what the year holds: the routine visits the patient's
//! problems call for, the minor illnesses, the visits missed, the office traffic and
//! how each piece of it turns out, the rare emergency, and when the seed's
//! conditions and episodes arrive. The author decides what each contact was and
//! writes it. Nobody plans the whole life, so no chart is plotted toward an ending.
//!
//! The rates are rough adult primary-care figures, not calibrated yet.

use std::fmt::Write as _;
use std::path::Path;
use std::sync::LazyLock;

use jiff::ToSpan;
use jiff::civil::{Date, Weekday, date};
use regex::Regex;

use crate::Result;
use crate::ledger::{Patient, Stamp};
use crate::rng::Rng;
use crate::seeds::{Seed, Unit};
use crate::time;
use crate::timeline::{OrderStatus, ProblemStatus, Timeline};

/// Each booked visit is missed or cancelled this often.
const MISSED: f64 = 0.08;
/// A seed condition is already known when the patient joins this often.
const KNOWN_AT_JOINING: f64 = 0.5;
/// Seed episodes can fall this many years before the patient joined.
const HISTORY_YEARS: i16 = 12;
/// Office traffic and how it turns out, by weight.
const ADMIN: &[&str] = &[
    "a refill request",
    "a refill request",
    "a records request or a fax from elsewhere",
    "a prior authorization or an insurance question",
    "a form to fill out (work, DMV, disability, travel)",
    "a recall or reminder about overdue care",
    "a lab or imaging order that needs fixing",
    "a billing or scheduling mix-up",
];
const OUTCOMES: &[(&str, f64)] = &[
    ("done without comment", 0.55),
    ("done, but late", 0.2),
    ("still open at the end of the year", 0.12),
    ("dropped, and nobody follows up", 0.13),
];

fn poisson(rng: &mut Rng, mean: f64) -> usize {
    let (limit, mut p, mut k) = ((-mean).exp(), 1.0, 0);
    loop {
        p *= rng.uniform(0.0, 1.0);
        if p <= limit {
            return k;
        }
        k += 1;
    }
}

fn chance(rng: &mut Rng, p: f64) -> bool {
    rng.uniform(0.0, 1.0) < p
}

fn pick<'a>(rng: &mut Rng, weighted: &[(&'a str, f64)]) -> &'a str {
    let total: f64 = weighted.iter().map(|w| w.1).sum();
    let mut x = rng.uniform(0.0, total);
    for (item, w) in weighted {
        if x < *w {
            return item;
        }
        x -= w;
    }
    weighted[weighted.len() - 1].0
}

/// A clinic time on a weekday between `from` and `to`.
fn clinic(rng: &mut Rng, from: Date, to: Date) -> Stamp {
    let days = (to - from).get_days().max(1);
    let mut d = from + rng.randint(0, i64::from(days) - 1).days();
    while matches!(d.weekday(), Weekday::Saturday | Weekday::Sunday) {
        d = d.tomorrow().expect("a date");
    }
    let minute = rng.randint(8 * 60, 16 * 60 + 40);
    Stamp(d.at((minute / 60) as i8, (minute % 60) as i8, 0, 0))
}

/// Any day, waking hours: when patients write and call.
fn any(rng: &mut Rng, from: Date, to: Date) -> Stamp {
    let days = (to - from).get_days().max(1);
    let d = from + rng.randint(0, i64::from(days) - 1).days();
    let minute = rng.randint(7 * 60, 22 * 60);
    Stamp(d.at((minute / 60) as i8, (minute % 60) as i8, 0, 0))
}

/// The year the patient joined the practice.
pub fn joined(seed: &Seed) -> i16 {
    time::as_of().date().year() - seed.years_with_practice as i16
}

/// The first year something could appear in a chart, for what did not always exist.
const EARLIEST: &[(&str, i16)] = &[
    ("covid", 2020),
    ("nirmatrelvir", 2021),
    ("paxlovid", 2021),
    ("rsv vaccine", 2023),
    ("semaglutide", 2018),
    ("tirzepatide", 2022),
    ("masld", 2023),
];

static YEAR: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\b(19[5-9]\d|20[0-2]\d)\b").expect("regex"));

/// When each of the seed's conditions and episodes arrives. An episode that names a
/// condition arrives with it; a year written in the text wins; nothing arrives
/// before it existed.
fn placed(seed: &Seed) -> Vec<(i16, String)> {
    let mut rng = Rng::from_text(&format!("placed:{}", seed.mrn));
    let (first, last) = (joined(seed), time::as_of().date().year());
    let words = |t: &str| -> Vec<String> {
        t.to_lowercase()
            .split(|c: char| !c.is_alphanumeric())
            .filter(|w| w.len() >= 5)
            .map(String::from)
            .collect()
    };
    let floor = |t: &str| -> i16 {
        let lower = t.to_lowercase();
        EARLIEST
            .iter()
            .filter(|(k, _)| lower.contains(k))
            .map(|(_, y)| *y)
            .max()
            .unwrap_or(i16::MIN)
    };
    let stated = |t: &str| YEAR.captures(t).and_then(|c| c[1].parse::<i16>().ok());
    let mut out = Vec::new();
    let mut taken = vec![false; seed.episodes.len()];
    for a in &seed.anchors {
        let mine: Vec<usize> = seed
            .episodes
            .iter()
            .enumerate()
            .filter(|(i, e)| {
                // more than half of the condition's words name it
                let (aw, ew) = (words(a), words(e));
                !taken[*i] && 2 * aw.iter().filter(|w| ew.contains(w)).count() > aw.len()
            })
            .map(|(i, _)| i)
            .collect();
        let texts: Vec<&str> = std::iter::once(a.as_str())
            .chain(mine.iter().map(|i| seed.episodes[*i].as_str()))
            .collect();
        let low = texts.iter().map(|t| floor(t)).max().unwrap_or(i16::MIN);
        let y = texts.iter().find_map(|t| stated(t)).unwrap_or_else(|| {
            if chance(&mut rng, KNOWN_AT_JOINING) {
                first - rng.randint(0, i64::from(HISTORY_YEARS)) as i16
            } else {
                rng.randint(i64::from(first), i64::from(last)) as i16
            }
        });
        let y = y.max(low).min(last);
        out.push((y, format!("the condition \"{a}\" begins or is recognized")));
        for i in mine {
            taken[i] = true;
            out.push((y, format!("with it, an episode: {}", seed.episodes[i])));
        }
    }
    for (_, e) in seed.episodes.iter().enumerate().filter(|(i, _)| !taken[*i]) {
        let y = stated(e)
            .unwrap_or_else(|| {
                rng.randint(i64::from(first - HISTORY_YEARS), i64::from(last)) as i16
            })
            .max(floor(e))
            .min(last);
        out.push((y, format!("an episode: {e}")));
    }
    out.sort();
    out
}

/// Where the chart stands at `at`: problems, drugs, levels, open orders, recent
/// documents.
fn standing(p: &Patient, at: Stamp) -> String {
    let tl = Timeline::new(p);
    let mut s = String::new();
    let problems: Vec<String> = tl
        .problems
        .iter()
        .filter(|x| x.recorded < at)
        .filter(|x| !matches!(x.status(at), ProblemStatus::Resolved(_)))
        .map(|x| x.label.to_string())
        .collect();
    let drugs: Vec<String> = tl
        .orders
        .iter()
        .filter(|o| o.authored < at && o.status(at) == OrderStatus::Active)
        .map(|o| {
            let sig = o.item.sig.as_deref().unwrap_or("");
            format!("{} ({sig})", o.item.key().unwrap_or(""))
        })
        .collect();
    let open: Vec<String> = tl
        .service_orders
        .iter()
        .filter(|o| o.placed < at && o.closed.is_none_or(|(c, _)| c >= at))
        .map(|o| o.item.order.clone().unwrap_or_default())
        .collect();
    let _ = writeln!(s, "Problems: {}", list(&problems));
    let _ = writeln!(s, "Medications: {}", list(&drugs));
    let _ = writeln!(s, "Open orders: {}", list(&open));
    let past: Vec<_> = tl.events.iter().filter(|e| e.start < at).collect();
    if let Some(last) = past
        .iter()
        .rev()
        .find_map(|e| e.state.as_ref().map(|s| (e, s)))
    {
        let _ = writeln!(
            s,
            "Last state set ({}): {}",
            last.0.start.date(),
            serde_json::to_string(last.1).unwrap_or_default()
        );
    }
    let docs: Vec<String> = past
        .iter()
        .rev()
        .flat_map(|e| e.documents.iter().map(move |d| (e, d)))
        .take(6)
        .map(|(e, d)| format!("{} {}", e.start.date(), d.title()))
        .collect();
    let _ = writeln!(s, "Latest documents: {}", list(&docs));
    s
}

fn list(items: &[String]) -> String {
    if items.is_empty() {
        "none".into()
    } else {
        items.join("; ")
    }
}

/// The brief for `year`: who the patient is, where the chart stands, and what the
/// year holds.
pub fn brief(unit: &Unit, patients: &[Patient], year: i16) -> Result<String> {
    let mut out = String::new();
    let today = time::as_of();
    for seed in &unit.members {
        let first = joined(seed);
        if year < first || year > today.date().year() {
            return Err(format!(
                "{} is with the practice {first} to {}",
                seed.mrn,
                today.date().year()
            )
            .into());
        }
        let start = date(year, 1, 1);
        let end = if year == today.date().year() {
            today.date()
        } else {
            date(year + 1, 1, 1)
        };
        let from = if year == first {
            start
                + Rng::from_text(&format!("join:{}", seed.mrn))
                    .randint(0, 200)
                    .days()
        } else {
            start
        };
        let age = time::age_on(seed.birth_date.0, from);
        let _ = writeln!(
            out,
            "# {} {} ({}), {year}: year {} of {} with the practice\n",
            seed.given,
            seed.family,
            seed.mrn,
            year - first + 1,
            today.date().year() - first + 1
        );
        let _ = writeln!(
            out,
            "{}, born {}, {age} this year; {}; {}; {}.",
            seed.sex.as_str(),
            seed.birth_date.0,
            seed.town,
            seed.insurance,
            seed.occupation_hint
        );
        if !seed.life.is_empty() {
            let _ = writeln!(out, "Life: {}.", seed.life.join("; "));
        }
        let patient = patients.iter().find(|p| p.mrn == seed.mrn);
        let at = Stamp(from.at(0, 0, 0, 0));
        match patient {
            Some(p) if year > first => {
                let _ = writeln!(
                    out,
                    "\n## Where the chart stands on {from}\n{}",
                    standing(p, at)
                );
            }
            _ => {}
        }

        let mut rng = Rng::from_text(&format!("year:{}:{year}", seed.mrn));
        let chronic = patient
            .map(|p| {
                let tl = Timeline::new(p);
                tl.problems
                    .iter()
                    .filter(|x| x.recorded < at && x.status(at) == ProblemStatus::Active)
                    .count()
            })
            .unwrap_or(0);
        let mut rolled: Vec<(Stamp, String)> = Vec::new();
        let book = |rng: &mut Rng, when: Stamp, what: &str| -> (Stamp, String) {
            if chance(rng, MISSED) {
                let how = if chance(rng, 0.5) {
                    "cancelled"
                } else {
                    "no-show"
                };
                (when, format!("{what}, but the visit is {how}"))
            } else {
                (when, what.to_string())
            }
        };
        if year == first {
            rolled.push((
                clinic(&mut rng, from, (from + 20.days()).min(end)),
                "establish_care: the first visit".into(),
            ));
        } else {
            let month = Rng::from_text(&format!("annual:{}", seed.mrn)).randint(1, 12) as i8;
            let around = date(year, month, 15) + rng.randint(-45, 45).days();
            if around < end && chance(&mut rng, if chronic == 0 { 0.75 } else { 0.9 }) {
                let when = clinic(&mut rng, around.max(start), (around + 7.days()).min(end));
                let v = book(&mut rng, when, "annual_physical");
                rolled.push(v);
            }
        }
        let followups = match chronic {
            0 => usize::from(chance(&mut rng, 0.25)),
            1 | 2 => 1 + usize::from(chance(&mut rng, 0.4)),
            _ => 2 + usize::from(chance(&mut rng, 0.5)),
        };
        for _ in 0..followups {
            let when = clinic(&mut rng, from, end);
            let v = book(
                &mut rng,
                when,
                "office_visit: follow-up of what is on the problem list",
            );
            rolled.push(v);
        }
        for _ in 0..poisson(&mut rng, 0.9 + 0.25 * chronic as f64) {
            let channel = pick(
                &mut rng,
                &[
                    ("portal_message", 0.35),
                    ("telephone", 0.3),
                    ("office_visit", 0.2),
                    ("urgent_care", 0.15),
                ],
            );
            let when = if channel == "office_visit" {
                clinic(&mut rng, from, end)
            } else {
                any(&mut rng, from, end)
            };
            rolled.push((
                when,
                format!("{channel}: a minor illness or new symptom (most are self-limited)"),
            ));
        }
        for _ in 0..poisson(&mut rng, 1.2) {
            rolled.push((
                any(&mut rng, from, end),
                "portal_message: a question, a home reading, a form or a refill".into(),
            ));
        }
        if chance(&mut rng, 0.04 + 0.002 * (age - 40).max(0) as f64) {
            rolled.push((
                any(&mut rng, from, end),
                "ed_visit: an emergency visit elsewhere".into(),
            ));
        }
        if chance(&mut rng, 0.01 + 0.0015 * (age - 50).max(0) as f64) {
            rolled.push((
                any(&mut rng, from, end),
                "hospital_admission: a stay at the regional hospital".into(),
            ));
        }
        for _ in 0..poisson(&mut rng, 2.0 + 0.5 * chronic as f64) {
            let what = ADMIN[rng.randint(0, ADMIN.len() as i64 - 1) as usize];
            let outcome = pick(&mut rng, OUTCOMES);
            rolled.push((
                clinic(&mut rng, from, end),
                format!("office: {what}; {outcome}"),
            ));
        }
        if chance(&mut rng, 0.08) {
            rolled.push((any(&mut rng, from, end), "life: something changes at home or work (you choose; it may reach the chart only as a line)".into()));
        }
        let arriving = placed(seed);
        if year == first {
            let before: Vec<&str> = arriving
                .iter()
                .filter(|(y, _)| *y < first)
                .map(|(_, w)| w.as_str())
                .collect();
            let _ = writeln!(
                out,
                "\n## History the patient brings (before joining; it reaches the chart through the establish-care history and outside records)\n{}",
                before
                    .iter()
                    .map(|w| format!("- {w}"))
                    .collect::<Vec<_>>()
                    .join("\n")
            );
        }
        for (_, what) in arriving.iter().filter(|(y, _)| *y == year) {
            rolled.push((any(&mut rng, from, end), format!("seed: {what}")));
        }
        rolled.retain(|(when, _)| *when < today);
        rolled.sort();
        let _ = writeln!(out, "\n## What this year holds (rolled by code)\n");
        for (when, what) in &rolled {
            let _ = writeln!(out, "- {} {} {what}", when, when.date().strftime("%a"));
        }
        if year == today.date().year() {
            let next = clinic(&mut rng, today.date() + 7.days(), today.date() + 75.days());
            let _ = writeln!(
                out,
                "\nToday is {}. The chart ends there. Booked after today: one primary-care visit on {next}, as an empty booked event. A draw from the last two weeks may still be pending.",
                time::AS_OF
            );
        }
        let _ = writeln!(
            out,
            "\nWrite these contacts as they happened, plus the contacts that follow from them \
             (a result call, a referral visit, a refill, outside records). Add no other \
             illness, emergency or life event. A dated item is when it starts; move it a few \
             days if the story needs it."
        );
    }
    Ok(out)
}

/// Write the year's brief into the drafts and print it.
pub fn run(unit: &Unit, patients: &[Patient], year: i16, drafts: &Path) -> Result<i32> {
    let text = brief(unit, patients, year)?;
    let path = drafts.join(format!("year-{year}.md"));
    std::fs::write(&path, &text)?;
    print!("{text}");
    println!("\n(written to {})", path.display());
    Ok(0)
}
