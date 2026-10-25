//! Design the synthetic adult primary-care panel: one seed per patient.
//!
//! A seed fixes who the patient is before anyone writes their chart: MRN, sex, birth
//! year, town, household, how long they have been with the practice, how complex
//! their history is, and a few clinical anchors drawn from adult primary-care
//! prevalence. Family members are designed together so their charts can agree.
//! The author model writes the history; this module only guarantees spread and
//! reproducibility (fixed seed, no clock or network).
//!
//! This is a port of `design_panel.py` that writes the same seeds file byte for
//! byte, so every draw happens in the order Python made it.

use std::collections::HashSet;
use std::io;
use std::path::Path;

use jiff::ToSpan;
use jiff::civil::Date;

use crate::json::{self, Json, Layout};
use crate::names;
use crate::obj;
use crate::rng::PyRandom;

pub const SEED: u64 = 20261024;
pub const AS_OF: &str = "2026-08-27"; // Roger practice simulation time (coda-agents fixture)

/// (town, postal code, weight).
pub type Town = (&'static str, &'static str, i64);
/// (race label, ethnicity, weight).
pub type Ancestry = (&'static str, &'static str, i64);

#[rustfmt::skip]
pub const TOWNS: &[Town] = &[ // Pioneer Valley and nearby, where the Roger fixture patients live
    ("Northampton", "01060", 14), ("Florence", "01062", 6), ("Easthampton", "01027", 9),
    ("Amherst", "01002", 10), ("Hadley", "01035", 4), ("Holyoke", "01040", 10),
    ("South Hadley", "01075", 6), ("Hatfield", "01038", 2), ("Williamsburg", "01096", 2),
    ("Southampton", "01073", 2), ("Westhampton", "01027", 1), ("Leeds", "01053", 2),
    ("Haydenville", "01039", 1), ("Belchertown", "01007", 4), ("Granby", "01033", 2),
    ("Sunderland", "01375", 2), ("Deerfield", "01342", 2), ("Greenfield", "01301", 4),
    ("Westfield", "01085", 4), ("Chicopee", "01020", 4), ("Springfield", "01109", 3),
    ("Ludlow", "01056", 2), ("Chesterfield", "01012", 1), ("Goshen", "01032", 1),
];

/// Adult primary-care age mix (years) -> weight.
#[rustfmt::skip]
pub const AGE_BANDS: &[((i64, i64), i64)] = &[
    ((18, 29), 14), ((30, 39), 15), ((40, 49), 15), ((50, 59), 17),
    ((60, 69), 18), ((70, 79), 14), ((80, 94), 7),
];

/// Race / ethnicity mix loosely reflecting Hampshire and Hampden counties.
pub const ANCESTRY: &[Ancestry] = &[
    ("White", "Not Hispanic or Latino", 66),
    ("White", "Hispanic or Latino", 6),
    ("Other Race", "Hispanic or Latino", 7), // e.g. Puerto Rican heritage, race unrecorded
    ("Black or African American", "Not Hispanic or Latino", 6),
    ("Asian", "Not Hispanic or Latino", 7),
    ("Two or more races", "Not Hispanic or Latino", 4),
    ("Declined", "Declined", 4),
];

/// Chronic anchors with rough adult prevalence, age-dependent multipliers, sex:
/// (anchor, base weight, min age, sex or None).
#[rustfmt::skip]
pub const ANCHORS: &[(&str, i64, i64, Option<&str>)] = &[
    ("hypertension", 30, 30, None), ("hyperlipidemia", 28, 35, None),
    ("obesity", 22, 18, None), ("prediabetes", 12, 30, None),
    ("type 2 diabetes", 11, 30, None), ("depression", 12, 18, None),
    ("anxiety disorder", 12, 18, None), ("hypothyroidism", 7, 25, Some("female")),
    ("hypothyroidism", 2, 30, Some("male")), ("GERD", 10, 25, None),
    ("osteoarthritis", 12, 50, None), ("chronic low back pain", 9, 25, None),
    ("asthma", 8, 18, None), ("COPD", 5, 50, None), ("tobacco use", 9, 18, None),
    ("atrial fibrillation", 4, 60, None), ("coronary artery disease", 5, 55, None),
    ("heart failure", 2, 65, None), ("chronic kidney disease", 6, 55, None),
    ("obstructive sleep apnea", 7, 35, None), ("migraine", 7, 18, Some("female")),
    ("migraine", 2, 18, Some("male")), ("insomnia", 5, 25, None),
    ("benign prostatic hyperplasia", 10, 55, Some("male")), ("osteoporosis", 8, 60, Some("female")),
    ("gout", 3, 40, Some("male")), ("allergic rhinitis", 10, 18, None),
    ("ADHD", 4, 18, None), ("alcohol use disorder", 3, 22, None),
    ("opioid use disorder in recovery", 1, 22, None), ("cannabis use", 4, 18, None),
    ("cancer survivorship", 4, 45, None), ("breast cancer survivorship", 3, 45, Some("female")),
    ("prostate cancer history", 3, 60, Some("male")), ("rheumatoid arthritis", 1, 30, None),
    ("psoriasis", 2, 18, None), ("inflammatory bowel disease", 1, 18, None),
    ("irritable bowel syndrome", 5, 18, None), ("celiac disease", 1, 18, None),
    ("hepatitis C (treated or untreated)", 1, 25, None), ("HIV on treatment", 1, 25, None),
    ("PTSD", 3, 18, None), ("bipolar disorder", 1, 18, None),
    ("vitamin D deficiency", 5, 18, None), ("iron deficiency anemia", 3, 18, Some("female")),
    ("polycystic ovary syndrome", 3, 18, Some("female")), ("menopause symptoms", 7, 45, Some("female")),
    ("peripheral neuropathy", 3, 50, None), ("mild cognitive impairment or dementia", 4, 70, None),
    ("Parkinson disease", 1, 60, None), ("glaucoma or cataract", 6, 60, None),
    ("hearing loss", 6, 60, None), ("peripheral arterial disease", 2, 60, None),
    ("history of venous thromboembolism", 2, 30, None), ("thyroid nodule", 2, 30, None),
    ("kidney stones", 2, 25, None), ("chronic hepatitis B", 1, 25, None),
    ("multiple sclerosis", 1, 20, Some("female")), ("fibromyalgia", 2, 25, Some("female")),
    ("skin cancer history", 3, 50, None), ("diverticular disease", 3, 50, None),
    ("recurrent urinary tract infections", 3, 18, Some("female")),
];

/// Anchors whose weight grows with age past 45.
const AGE_SCALED: &[&str] = &[
    "hypertension",
    "hyperlipidemia",
    "type 2 diabetes",
    "osteoarthritis",
    "chronic kidney disease",
    "atrial fibrillation",
    "coronary artery disease",
];

/// Episodic texture that real primary-care charts accumulate.
pub const EPISODES: &[&str] = &[
    "an upper respiratory infection or sinusitis visit",
    "a COVID-19 illness",
    "a musculoskeletal injury (sprain, strain or minor fracture)",
    "a urinary tract infection",
    "a skin lesion evaluated or removed",
    "an emergency department visit",
    "a hospital admission",
    "an elective surgery or procedure",
    "a medication side effect",
    "a dermatology, cardiology, GI or other specialist consultation",
    "screening colonoscopy, mammography or other age-appropriate screening",
    "an abnormal lab or imaging finding needing follow-up",
    "a portal message thread",
    "a telephone triage call",
    "a pregnancy, miscarriage or fertility care (if applicable)",
    "a mental-health crisis or counseling referral",
    "a fall",
    "a travel or vaccine visit",
    "an outside record arriving from a prior physician or hospital",
];

/// Realistic chart imperfections (applied to a minority of patients).
pub const TEXTURE: &[&str] = &[
    "an allergy or adverse reaction documented only in one note and never added to the allergy list",
    "the patient stopped a medication on their own, documented in a phone note, while the order remains active",
    "an outside medication list that disagrees with the practice list",
    "a recommended follow-up (imaging, referral, lab) that was never completed",
    "a gap of more than two years without visits",
    "a result reviewed and communicated through a portal message",
    "a specialist changed a medication between primary-care visits",
    "conflicting family-history details across notes",
    "a duplicate or outdated problem-list entry",
    "a patient-reported outside event (urgent care visit, outside lab) with no records received",
];

/// Inclusive ranges a complexity tier draws from.
pub struct Tier {
    pub years: (i64, i64),
    pub documents: (i64, i64),
    pub observations: (i64, i64),
}

/// Targets measured against coda-gym charts (median ~108 notes, ~320 observations).
#[rustfmt::skip]
pub const COMPLEXITY: &[(&str, Tier)] = &[
    ("light", Tier { years: (5, 8), documents: (40, 80), observations: (200, 500) }),
    ("moderate", Tier { years: (8, 15), documents: (80, 150), observations: (500, 1200) }),
    ("complex", Tier { years: (12, 20), documents: (150, 300), observations: (1000, 2500) }),
];

#[rustfmt::skip]
pub const OCCUPATIONS: &[&str] = &[
    "teacher", "nurse", "graduate student", "line cook", "software developer",
    "home health aide", "dairy farmer", "UPS driver", "accountant", "carpenter", "librarian",
    "college professor", "warehouse associate", "hair stylist", "small business owner", "electrician", "social worker", "bus driver", "barista",
    "physical therapist", "musician", "state employee", "landscaper",
    "pharmacy technician", "unemployed", "on disability", "paralegal", "veterinary technician",
    "construction laborer", "restaurant owner", "data analyst", "chef",
    "custodian", "bank teller", "artist", "clergy", "police officer", "firefighter",
    "medical assistant", "orchard worker", "childcare provider",
    "real estate agent", "auto mechanic", "homemaker", "undergraduate student",
];

#[rustfmt::skip]
const RETIRED: &[&str] = &[
    "retired teacher", "retired machinist", "retired nurse", "retired postal worker",
    "retired engineer", "retired farmer", "retired bookkeeper", "part-time library volunteer",
];

#[rustfmt::skip]
pub const INSURANCE: &[(&str, i64)] = &[("commercial employer plan", 46), ("Medicare", 22), ("Medicare Advantage", 8),
                                       ("MassHealth (Medicaid)", 14), ("Health Connector marketplace plan", 6),
                                       ("uninsured, self-pay", 2), ("VA plus Medicare", 2)];

/// A weight as Python held it: an int, or a float once an age scale applied.
#[derive(Clone, Copy)]
pub enum Weight {
    Int(i64),
    Float(f64),
}

impl Weight {
    fn value(self) -> f64 {
        match self {
            Weight::Int(i) => i as f64,
            Weight::Float(f) => f,
        }
    }
}

/// Python's builtin `sum` over ints and floats: a leading run of ints adds
/// exactly, the first float joins with a plain addition, and from there on the
/// total is a compensated (Neumaier) sum, as CPython 3.12+ does.
fn python_sum(weights: impl IntoIterator<Item = Weight>) -> f64 {
    let mut weights = weights.into_iter();
    let mut ints = 0i64;
    let mut hi = loop {
        match weights.next() {
            None => return ints as f64,
            Some(Weight::Int(i)) => ints += i,
            Some(Weight::Float(f)) => break ints as f64 + f,
        }
    };
    let mut lo = 0.0;
    for w in weights {
        let x = w.value();
        let t = hi + x;
        if hi.abs() >= x.abs() {
            lo += (hi - t) + x;
        } else {
            lo += (x - t) + hi;
        }
        hi = t;
    }
    if lo != 0.0 && lo.is_finite() {
        hi + lo
    } else {
        hi
    }
}

/// `weighted(rng, items)`: one item, with probability proportional to its weight.
pub fn weighted<'a, T>(rng: &mut PyRandom, items: &'a [T], weight: impl Fn(&T) -> Weight) -> &'a T {
    let total = python_sum(items.iter().map(&weight));
    let mut pick = rng.uniform(0.0, total);
    for item in items {
        pick -= weight(item).value();
        if pick <= 0.0 {
            return item;
        }
    }
    items.last().expect("weighted over an empty list")
}

fn tier(complexity: &str) -> &'static Tier {
    COMPLEXITY
        .iter()
        .find(|(name, _)| *name == complexity)
        .map(|(_, t)| t)
        .expect("known complexity")
}

/// `{"light": a, "moderate": b, "complex": c}[complexity]`.
fn by_tier<T>(complexity: &str, light: T, moderate: T, complex: T) -> T {
    match complexity {
        "light" => light,
        "moderate" => moderate,
        "complex" => complex,
        _ => panic!("unknown complexity {complexity}"),
    }
}

pub fn anchors_for(rng: &mut PyRandom, age: i64, sex: &str, complexity: &str) -> Vec<&'static str> {
    // Python builds the whole dict, so all three counts are drawn.
    let light = *rng.choice(&[0usize, 0, 1, 1, 2]);
    let moderate = *rng.choice(&[1usize, 2, 2, 3]);
    let complex = *rng.choice(&[3usize, 4, 4, 5, 6]);
    let n = by_tier(complexity, light, moderate, complex);
    let mut pool: Vec<(&str, Weight)> = Vec::new();
    for &(name, w, min_age, s) in ANCHORS {
        if age < min_age || s.is_some_and(|s| s != sex) {
            continue;
        }
        let weight = if AGE_SCALED.contains(&name) {
            let scale = 1.0 + (0.max(age - 45) as f64) / 25.0;
            Weight::Float(w as f64 * scale)
        } else {
            Weight::Int(w)
        };
        pool.push((name, weight));
    }
    let mut chosen: Vec<&str> = Vec::new();
    while !pool.is_empty() && chosen.len() < n {
        let name = weighted(rng, &pool, |p| p.1).0;
        pool.retain(|p| p.0 != name);
        if !chosen.contains(&name) {
            chosen.push(name);
        }
    }
    chosen
}

pub fn complexity_for(rng: &mut PyRandom, age: i64) -> &'static str {
    let w: &[(&str, i64)] = if age < 35 {
        &[("light", 62), ("moderate", 33), ("complex", 5)]
    } else if age < 60 {
        &[("light", 30), ("moderate", 52), ("complex", 18)]
    } else {
        &[("light", 12), ("moderate", 50), ("complex", 38)]
    };
    weighted(rng, w, |i| Weight::Int(i.1)).0
}

fn draw(rng: &mut PyRandom, (lo, hi): (i64, i64)) -> i64 {
    rng.randint(lo, hi)
}

fn strs(items: &[&str]) -> Json {
    Json::Arr(items.iter().map(|s| Json::from(*s)).collect())
}

pub fn person(
    rng: &mut PyRandom,
    mrn: u64,
    age: Option<i64>,
    sex: Option<&'static str>,
    town: Option<&'static Town>,
    ancestry: Option<&'static Ancestry>,
) -> Json {
    let age = match age {
        Some(age) => age,
        None => {
            let (band, _) = *weighted(rng, AGE_BANDS, |b| Weight::Int(b.1));
            draw(rng, band)
        }
    };
    let sex = sex.unwrap_or_else(|| {
        if rng.random() < 0.53 {
            "female"
        } else {
            "male"
        }
    });
    let town = town.unwrap_or_else(|| weighted(rng, TOWNS, |t| Weight::Int(t.2)));
    let ancestry = ancestry.unwrap_or_else(|| weighted(rng, ANCESTRY, |a| Weight::Int(a.2)));
    let complexity = complexity_for(rng, age);
    let span = tier(complexity);
    let wanted = draw(rng, span.years);
    let years = wanted.min(1.max(age - 17)); // an adult practice: joined at 18 at the earliest
    let scale = 0.3f64.max(years as f64 / wanted as f64); // fewer years, proportionally fewer records
    let as_of_year: i64 = AS_OF[..4].parse().expect("AS_OF year");
    let birth_year = as_of_year - age - i64::from(rng.random() < 0.6);
    let eligible: Vec<(&str, i64)> = INSURANCE
        .iter()
        .copied()
        .filter(|i| {
            let too_young = (i.0.starts_with("Medicare") && age < 63)
                || (i.0 == "VA plus Medicare" && age < 65);
            !too_young
        })
        .collect();
    let mut insurance = weighted(rng, &eligible, |i| Weight::Int(i.1)).0;
    if age >= 66 && rng.random() < 0.85 {
        insurance = *rng.choice(&[
            "Medicare",
            "Medicare",
            "Medicare Advantage",
            "Medicare with a supplement plan",
        ]);
    }
    let mut language = "English";
    if ancestry.1 == "Hispanic or Latino" && rng.random() < 0.3 {
        language = "Spanish";
    } else if ancestry.0 == "Asian" && rng.random() < 0.25 {
        language = *rng.choice(&["Vietnamese", "Mandarin", "Korean", "Khmer", "Hindi"]);
    } else if rng.random() < 0.02 {
        language = *rng.choice(&["Portuguese", "Russian", "Haitian Creole", "Polish"]);
    }
    let episodes = rng.sample(EPISODES, by_tier(complexity, 4, 7, 10));
    let texture = rng.sample(TEXTURE, by_tier(complexity, 1, 2, 3));
    // The dict literal's remaining values, drawn in key order.
    let occupation = if age < 66 {
        *rng.choice(OCCUPATIONS)
    } else {
        *rng.choice(RETIRED)
    };
    let documents = (draw(rng, span.documents) as f64 * scale) as i64;
    let observations = (draw(rng, span.observations) as f64 * scale) as i64;
    let anchors = anchors_for(rng, age, sex, complexity);
    obj![
        ("mrn", format!("MRN{mrn:08}")),
        ("sex", sex),
        ("age_at_as_of", age),
        ("birth_year", birth_year),
        ("town", town.0),
        ("postal_code", town.1),
        ("race", ancestry.0),
        ("ethnicity", ancestry.1),
        ("preferred_language", language),
        ("insurance", insurance),
        ("occupation_hint", occupation),
        ("complexity", complexity),
        ("years_with_practice", years),
        ("target_documents", documents),
        ("target_observations", observations),
        ("anchors", strs(&anchors)),
        ("episodes", strs(&episodes)),
        ("chart_texture", strs(&texture)),
    ]
}

/// The authoring units: individuals and families designed together.
pub fn design(count: usize, first_mrn: u64) -> Vec<Json> {
    let mut rng = PyRandom::new(u128::from(SEED));
    let mut units = Vec::new();
    let mut mrn = first_mrn;
    let mut remaining = count;
    let mut unit_no = 0;
    while remaining > 0 {
        unit_no += 1;
        let roll = rng.random();
        let kind = if remaining >= 2 && roll < 0.09 {
            Some("couple")
        } else if remaining >= 2 && roll < 0.12 {
            Some("parent and adult child")
        } else if remaining >= 3 && roll < 0.135 {
            Some("parents and adult child")
        } else if remaining >= 2 && roll < 0.15 {
            Some("adult siblings")
        } else {
            None
        };
        let mut members = Vec::new();
        match kind {
            None => members.push(person(&mut rng, mrn, None, None, None, None)),
            Some(kind) => {
                let town = weighted(&mut rng, TOWNS, |t| Weight::Int(t.2));
                let anc = weighted(&mut rng, ANCESTRY, |a| Weight::Int(a.2));
                let rel: &[(&str, usize, usize)] = match kind {
                    "couple" => {
                        let a = rng.randint(28, 85);
                        let sexes = *rng.choice(&[
                            ("female", "male"),
                            ("female", "male"),
                            ("female", "male"),
                            ("female", "female"),
                            ("male", "male"),
                        ]);
                        members.push(person(
                            &mut rng,
                            mrn,
                            Some(a),
                            Some(sexes.0),
                            Some(town),
                            Some(anc),
                        ));
                        let age = 18.max(94.min(a + rng.randint(-6, 6)));
                        members.push(person(
                            &mut rng,
                            mrn + 1,
                            Some(age),
                            Some(sexes.1),
                            Some(town),
                            Some(anc),
                        ));
                        &[("spouse", 0, 1)]
                    }
                    "parent and adult child" => {
                        let p = rng.randint(50, 88);
                        members.push(person(&mut rng, mrn, Some(p), None, Some(town), Some(anc)));
                        let age = 18.max(p - rng.randint(22, 36));
                        let child_town = if rng.random() < 0.4 { Some(town) } else { None };
                        members.push(person(
                            &mut rng,
                            mrn + 1,
                            Some(age),
                            None,
                            child_town,
                            Some(anc),
                        ));
                        &[("parent", 0, 1)]
                    }
                    "parents and adult child" => {
                        let p = rng.randint(48, 80);
                        members.push(person(
                            &mut rng,
                            mrn,
                            Some(p),
                            Some("female"),
                            Some(town),
                            Some(anc),
                        ));
                        let age = 94.min(p + rng.randint(-3, 5));
                        members.push(person(
                            &mut rng,
                            mrn + 1,
                            Some(age),
                            Some("male"),
                            Some(town),
                            Some(anc),
                        ));
                        let age = 18.max(p - rng.randint(22, 32));
                        members.push(person(
                            &mut rng,
                            mrn + 2,
                            Some(age),
                            None,
                            Some(town),
                            Some(anc),
                        ));
                        &[("spouse", 0, 1), ("parent", 0, 2), ("parent", 1, 2)]
                    }
                    _ => {
                        let a = rng.randint(25, 80);
                        members.push(person(&mut rng, mrn, Some(a), None, None, Some(anc)));
                        let age = 18.max(a + rng.randint(-8, 8));
                        members.push(person(&mut rng, mrn + 1, Some(age), None, None, Some(anc)));
                        &[("sibling", 0, 1)]
                    }
                };
                let fam = format!("family-{unit_no:04}");
                for m in &mut members {
                    m.set("family_id", fam.clone().into());
                    m.set("relationships", Json::Arr(Vec::new()));
                }
                // Each entry names the other person's role relative to this patient;
                // for ("parent", i, j), member i is the parent of member j.
                for &(r, i, j) in rel {
                    let (forward, back) = match r {
                        "spouse" => ("spouse", "spouse"),
                        "parent" => ("child", "parent"),
                        _ => ("sibling", "sibling"),
                    };
                    let (mi, mj) = (members[i].get("mrn").clone(), members[j].get("mrn").clone());
                    push(&mut members[i], obj![("mrn", mj), ("relation", forward)]);
                    push(&mut members[j], obj![("mrn", mi), ("relation", back)]);
                }
            }
        }
        let n = members.len();
        units.push(obj![
            ("unit", format!("unit-{unit_no:04}")),
            ("kind", kind.unwrap_or("individual")),
            ("members", Json::Arr(members)),
        ]);
        mrn += n as u64;
        remaining -= n;
    }
    units
}

fn push(member: &mut Json, relationship: Json) {
    if let Some(Json::Arr(list)) = member.get_mut("relationships") {
        list.push(relationship);
    }
}

fn text<'a>(m: &'a Json, key: &str) -> &'a str {
    m.get(key)
        .as_str()
        .unwrap_or_else(|| panic!("seed field {key}"))
}

pub fn heritage(m: &Json) -> &'static str {
    if text(m, "ethnicity") == "Hispanic or Latino" {
        return "hispanic";
    }
    if text(m, "race") == "Asian" {
        return "asian";
    }
    if text(m, "race") == "Black or African American" {
        return "black";
    }
    if matches!(text(m, "preferred_language"), "Russian" | "Polish") {
        return "slavic";
    }
    "general"
}

/// `re.sub(r"[^a-z]", "", s.lower())`.
fn email_part(s: &str) -> String {
    s.to_lowercase()
        .chars()
        .filter(char::is_ascii_lowercase)
        .collect()
}

/// Give every patient a unique fictional name, birth date, phone and email.
pub fn assign_identity(units: &mut [Json]) {
    let mut rng = PyRandom::new(u128::from(SEED + 1));
    let as_of: Date = AS_OF.parse().expect("AS_OF date");
    let mut used_names: HashSet<(&str, &str)> = HashSet::new();
    let mut used_phones: HashSet<String> = HashSet::new();
    let mut used_emails: HashSet<String> = HashSet::new();
    let mut origins: Vec<&str> = names::ASIAN_ORIGINS.iter().map(|o| o.0).collect();
    origins.sort_unstable();
    for unit in units {
        let couple = unit.get("kind").as_str() == Some("couple");
        let mut shared_family: Option<&str> = None;
        // Python's unit["_origin"]: the first Asian member's origin, kept for the unit.
        let mut unit_origin: Option<&str> = None;
        let Some(Json::Arr(members)) = unit.get_mut("members") else {
            panic!("unit without members");
        };
        for m in members {
            let h = heritage(m);
            let sex = if text(m, "sex") == "female" {
                "female"
            } else {
                "male"
            };
            let age = m.get("age_at_as_of").as_i64().expect("age_at_as_of");
            let year = |back: i64| {
                Date::new(
                    (i64::from(as_of.year()) - back) as i16,
                    as_of.month(),
                    as_of.day(),
                )
                .expect("as_of.replace(year=...)")
            };
            let latest = year(age);
            let earliest = year(age + 1).tomorrow().expect("date in range");
            let span = earliest.until(latest).expect("date span").get_days();
            let dob = earliest
                .checked_add(rng.randint(0, i64::from(span)).days())
                .expect("date in range");
            m.set("birth_date", dob.to_string().into());
            m.set("birth_year", i64::from(dob.year()).into());
            let era = if dob.year() < 1960 {
                "older"
            } else if dob.year() < 1990 {
                "middle"
            } else {
                "younger"
            };
            let mut origin = None;
            if h == "asian" {
                let own = match names::language_origin(text(m, "preferred_language")) {
                    Some(o) => o,
                    None => *rng.choice(&origins),
                };
                origin = Some(*unit_origin.get_or_insert(own));
            }
            let (mut given, mut family) = ("", "");
            for _ in 0..500 {
                given = if let Some(o) = origin {
                    let (f_names, m_names, _) = names::asian_origin(o);
                    *rng.choice(if sex == "female" { f_names } else { m_names })
                } else if h != "general" && rng.random() < 0.85 {
                    *rng.choice(names::given_heritage(h, sex))
                } else {
                    *rng.choice(names::given(sex, era))
                };
                family = match shared_family {
                    Some(shared) if !couple || rng.random() < 0.7 => shared,
                    _ => {
                        let pool = if let Some(o) = origin {
                            names::asian_origin(o).2
                        } else if h != "general" && rng.random() < 0.85 {
                            names::family(h)
                        } else {
                            names::family("general")
                        };
                        *rng.choice(pool)
                    }
                };
                if !used_names.contains(&(given, family)) {
                    break;
                }
            }
            used_names.insert((given, family));
            shared_family = shared_family.or(Some(family));
            m.set("given", given.into());
            m.set("family", family.into());
            let phone = loop {
                let area = rng.choice(names::AREA_CODES);
                let phone = format!("+1-{area}-555-01{:02}", rng.randint(0, 99));
                if !used_phones.contains(&phone) {
                    break phone;
                }
            };
            used_phones.insert(phone.clone());
            m.set("phone", phone.into());
            if rng.random() < 0.82 {
                let base = format!("{}.{}", email_part(given), email_part(family));
                let mut email = format!("{base}@example.test");
                let mut n = 2;
                while used_emails.contains(&email) {
                    email = format!("{base}{n}@example.test");
                    n += 1;
                }
                used_emails.insert(email.clone());
                m.set("email", email.into());
            }
        }
    }
}

/// The whole seeds file: `{"seed", "as_of", "units"}`.
pub fn seeds(count: usize, first_mrn: u64) -> Json {
    let mut units = design(count, first_mrn);
    assign_identity(&mut units);
    obj![
        ("seed", SEED as i64),
        ("as_of", AS_OF),
        ("units", Json::Arr(units))
    ]
}

/// The seeds file text exactly as `design_panel.py` writes it.
pub fn render(seeds: &Json) -> String {
    json::dumps(seeds, Layout::Indent(1), true) + "\n"
}

/// Write the seeds file, creating its directory.
pub fn write(path: &Path, seeds: &Json) -> io::Result<()> {
    if let Some(dir) = path.parent() {
        std::fs::create_dir_all(dir)?;
    }
    std::fs::write(path, render(seeds))
}
