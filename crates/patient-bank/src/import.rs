//! Import the patients from synthetic_hospital: every adult becomes a unit folder
//! with its `source.md` and `seed.json`.
//!
//! The source is synthetic_hospital's `benchmark_v1.3.db` (MIT, fully synthetic),
//! read read-only. Every draw is seeded by the patient, so an import gives the same
//! files every time:
//!
//! - **source.md**: the patient's profile and encounters, each encounter moved to a
//!   practice date: the source span shifted to end 3 weeks to 11 months before
//!   today, then off the 15th and onto a weekday for clinic visits.
//! - **seed.json**: who the patient is in the practice. Name, town and phone are
//!   fictional, drawn from `data/name-pool.json` by origin, sex and birth year; age,
//!   sex, race, insurance and the clinical anchors come from the source. Then how
//!   complete the outside records are, when the practice last saw the patient, and
//!   whether a visit is booked.
//!
//! The draws follow Python's `random` exactly (see `rng`), because the set was first
//! drawn in Python. The import only adds files. A seed is data once it exists,
//! because charts are written against it, so an existing seed.json is never redrawn;
//! a source.md never changes, so one that differs from the database is an error.

use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;

use jiff::ToSpan;
use jiff::civil::{Date, date};
use rusqlite::{Connection, OpenFlags};
use serde::Deserialize;
use serde_json::json;
use sha2::{Digest, Sha256};

use crate::catalog::write_atomic;
use crate::rng::Rng;
use crate::seeds::Unit;
use crate::{Result, paths};

const AS_OF: Date = date(2026, 8, 27);
/// Made by hand before the import; their files are kept as they are.
const PILOTS: [i64; 3] = [2013, 2188, 2797];
const HISPANIC_TOWNS: [&str; 3] = ["Holyoke", "Springfield", "Chicopee"];
const AREA: [&str; 18] = [
    "413", "413", "413", "413", "413", "413", "774", "860", "802", "518", "617", "508", "978",
    "603", "201", "646", "347", "917",
];
/// How long since the practice last saw a patient: (share, fewest days, most days).
const RECENCY: [(f64, i64, i64); 5] = [
    (0.35, 7, 91),
    (0.25, 92, 182),
    (0.20, 183, 365),
    (0.15, 366, 730),
    (0.05, 731, 1460),
];
const TEXTURE: [(&str, &str); 3] = [
    (
        "complete",
        "Records: complete. Outside letters, reports and summaries arrive and are filed; \
         lists are kept current.",
    ),
    (
        "gaps",
        "Records: gaps. Some outside care never sends its letter or sends it late; the \
         practice learns of it from the patient or a pharmacy fill; a list lags.",
    ),
    (
        "sparse",
        "Records: sparse. Most outside care reaches the practice only as the patient's \
         account, a short ED note or a fill history, with few numbers.",
    ),
];

/// Origins whose names the pool cannot give well: (origin, family, male, female).
type Names = (
    &'static str,
    [&'static str; 10],
    [&'static str; 8],
    [&'static str; 8],
);
const CURATED: [Names; 5] = [
    (
        "jewish",
        [
            "Rosenblum",
            "Feldman",
            "Weinstock",
            "Halpern",
            "Shapiro",
            "Katzman",
            "Gutman",
            "Lipschitz",
            "Adler",
            "Bernstein",
        ],
        [
            "David", "Michael", "Daniel", "Joel", "Aaron", "Jonathan", "Barry", "Steven",
        ],
        [
            "Rachel", "Deborah", "Miriam", "Sharon", "Leah", "Rebecca", "Judith", "Naomi",
        ],
    ),
    (
        "south_asian",
        [
            "Patel",
            "Rahman",
            "Iyer",
            "Chowdhury",
            "Reddy",
            "Desai",
            "Hossain",
            "Menon",
            "Bhatt",
            "Nair",
        ],
        [
            "Arjun", "Rafiq", "Vikram", "Sanjay", "Imran", "Rohan", "Anil", "Tanvir",
        ],
        [
            "Priya", "Nasrin", "Anjali", "Deepa", "Farhana", "Kavya", "Sunita", "Meera",
        ],
    ),
    (
        "southeast_asian",
        [
            "Nguyen",
            "Tran",
            "Pham",
            "Srisai",
            "Wongsakul",
            "Santos",
            "Reyes",
            "Bautista",
            "Le",
            "Chanthavong",
        ],
        [
            "Minh", "Tuan", "Somchai", "Anan", "Joel", "Ramon", "Bao", "Kiet",
        ],
        [
            "Linh", "Thao", "Malee", "Siriporn", "Maricel", "Joy", "Hoa", "Ngoc",
        ],
    ),
    (
        "east_asian",
        [
            "Chen", "Wang", "Liu", "Zhang", "Kim", "Park", "Choi", "Tanaka", "Lin", "Huang",
        ],
        [
            "Wei", "Jun", "David", "Min-jun", "Kenji", "Hao", "Daniel", "Jason",
        ],
        [
            "Mei", "Lily", "Ji-woo", "Yuki", "Grace", "Hui", "Amy", "Jennifer",
        ],
    ),
    (
        "middle_eastern",
        [
            "Haddad", "Khoury", "Yilmaz", "Demir", "Karimov", "Nasser", "Saleh", "Aziz",
            "Georgiou", "Mansour",
        ],
        [
            "Elias", "Omar", "Mehmet", "Rustam", "Karim", "Andreas", "Sami", "Tariq",
        ],
        [
            "Nadia", "Leila", "Ayse", "Dilnoza", "Rania", "Elena", "Mariam", "Samira",
        ],
    ),
];

/// The source's race and ethnicity text, read as an origin: (words, origin).
const ORIGINS: [(&[&str], &str); 8] = [
    (&["ashkenazi", "jewish"], "jewish"),
    (
        &[
            "hispanic",
            "latin",
            "mexican",
            "guatemal",
            "puerto",
            "dominican",
        ],
        "hispanic",
    ),
    (&["black", "african"], "black"),
    (
        &["south asian", "indian", "bangladesh", "pakistan"],
        "south_asian",
    ),
    (
        &["vietnam", "thai", "filipin", "southeast"],
        "southeast_asian",
    ),
    (
        &["chinese", "korean", "japanese", "east asian", "asian"],
        "east_asian",
    ),
    (
        &[
            "lebanese",
            "turkish",
            "uzbek",
            "middle eastern",
            "arab",
            "cypriot",
            "persian",
            "iran",
        ],
        "middle_eastern",
    ),
    (&["italian", "greek", "mediterranean"], "mediterranean"),
];

/// One fictional person of the name and town pool.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Person {
    given: String,
    family: String,
    sex: String,
    birth_year: i32,
    race: String,
    ethnicity: String,
    town: String,
    postal_code: String,
}

impl Person {
    fn origin(&self) -> &'static str {
        if self.ethnicity == "Hispanic or Latino" {
            return "hispanic";
        }
        match self.race.as_str() {
            "Black or African American" => "black",
            "Asian" => "east_asian",
            _ => "white",
        }
    }
}

struct Encounter {
    order: i64,
    date: Date,
    kind: String,
    dept: String,
    complaint: Option<String>,
    note: Option<String>,
}

/// `random.Random(int(sha256(key).hexdigest()[:12], 16))`.
fn rng_for(key: &str) -> Rng {
    let h = Sha256::digest(key.as_bytes());
    Rng::new(h[..6].iter().fold(0u128, |n, b| (n << 8) | u128::from(*b)))
}

fn days(n: i64) -> jiff::Span {
    n.days()
}

fn between(later: Date, earlier: Date) -> i64 {
    later.duration_since(earlier).as_secs() / 86_400
}

/// The source's race and ethnicity read as an origin, "white" when nothing matches.
fn origin(text: &str) -> &'static str {
    let s = text.to_lowercase();
    let starts_word = |w: &str| {
        s.match_indices(w).any(|(i, _)| {
            s[..i]
                .chars()
                .next_back()
                .is_none_or(|c| !(c.is_alphanumeric() || c == '_'))
        })
    };
    for (words, o) in ORIGINS {
        if words.iter().any(|w| starts_word(w)) {
            let not_hispanic = ["non-hispanic", "non hispanic", "not hispanic"]
                .iter()
                .any(|n| s.contains(n));
            if o == "hispanic" && not_hispanic {
                continue;
            }
            return o;
        }
    }
    "white"
}

/// (race, ethnicity) as the seeds spell them.
fn omb(origin: &str) -> (&'static str, &'static str) {
    match origin {
        "hispanic" => ("Other Race", "Hispanic or Latino"),
        "black" => ("Black or African American", "Not Hispanic or Latino"),
        "south_asian" | "east_asian" | "southeast_asian" => ("Asian", "Not Hispanic or Latino"),
        _ => ("White", "Not Hispanic or Latino"),
    }
}

/// Shift the source span to end before today, then move each date off the 15th.
fn practice_dates(pid: i64, encs: &[Encounter]) -> (Vec<Date>, i64) {
    let mut r = rng_for(&format!("dates:{pid}"));
    let last = encs.last().expect("a patient has encounters").date;
    let target = AS_OF - days(r.randint(21, 330));
    let shift = between(target, last);
    let mut out: Vec<Date> = Vec::new();
    for e in encs {
        let mut jr = rng_for(&format!("jitter:{pid}:{}", e.order));
        let mut p = e.date + days(shift + jr.randint(-9, 9));
        if ["outpatient", "follow_up", "telehealth", "procedure"].contains(&e.kind.as_str()) {
            while p.weekday().to_monday_zero_offset() >= 5 {
                p += days(1);
            }
        }
        if let Some(&prev) = out.last()
            && p <= prev
        {
            let same_day = e.date == encs[out.len() - 1].date;
            p = if same_day { prev } else { prev + days(1) };
        }
        if p >= AS_OF {
            p = AS_OF - days(1);
        }
        out.push(p);
    }
    (out, shift)
}

/// Draw from `RECENCY`: how many days before today the practice last saw the patient.
fn recency_days(r: &mut Rng) -> i64 {
    let mut x = r.random();
    for (share, lo, hi) in RECENCY {
        if x < share {
            return r.randint(lo, hi);
        }
        x -= share;
    }
    let (_, lo, hi) = RECENCY[RECENCY.len() - 1];
    r.randint(lo, hi)
}

/// Python's `str.strip()`.
fn strip(s: &str) -> &str {
    s.trim_matches(|c: char| c.is_whitespace() || ('\x1c'..='\x1f').contains(&c))
}

fn or_none(s: &Option<String>) -> &str {
    s.as_deref().unwrap_or("None")
}

/// Everything one import draws for the panel, in patient order.
struct Drawn {
    unit: String,
    source: String,
    seed: Unit,
}

fn draw_all(db: &Path) -> Result<Vec<Drawn>> {
    // Reading a third-party SQLite file; no ORM owns it, and nothing is written.
    let c = Connection::open_with_flags(db, OpenFlags::SQLITE_OPEN_READ_ONLY)
        .map_err(|e| format!("{}: {e}", db.display()))?;
    let pool: Vec<Person> = serde_json::from_str(&fs::read_to_string(paths::name_pool())?)?;
    let towns: Vec<(&str, &str)> = (pool.iter())
        .map(|m| (m.town.as_str(), m.postal_code.as_str()))
        .collect();
    let hispanic_towns: Vec<(&str, &str)> = (towns.iter().copied())
        .filter(|t| HISPANIC_TOWNS.contains(&t.0))
        .collect();

    let mut used_names: HashSet<(String, String)> = HashSet::new();
    let mut used_given: HashMap<String, i64> = HashMap::new();
    let mut used_family: HashMap<String, i64> = HashMap::new();
    let mut used_phone: HashSet<String> = HashSet::new();
    let mut used_email: HashSet<String> = HashSet::new();
    let mut out = Vec::new();

    let mut patients = c.prepare(
        "select patient_id, age, sex, race_ethnicity, insurance, profile, primary_diagnoses, \
         comorbidities from longitudinal_patients where age >= 18 order by patient_id",
    )?;
    let mut encounters = c.prepare(
        "select encounter_order, encounter_date, encounter_type, department, chief_complaint, \
         note_text from longitudinal_encounters where patient_id = ?1 order by encounter_order",
    )?;
    type Row = (
        i64,
        i64,
        String,
        Option<String>,
        Option<String>,
        Option<String>,
        Option<String>,
        Option<String>,
    );
    let rows: Vec<Row> = patients
        .query_map([], |r| {
            Ok((
                r.get(0)?,
                r.get(1)?,
                r.get(2)?,
                r.get(3)?,
                r.get(4)?,
                r.get(5)?,
                r.get(6)?,
                r.get(7)?,
            ))
        })?
        .collect::<std::result::Result<_, _>>()?;
    for (pid, age, sex, race_eth, insurance, profile, primary, comorbid) in rows {
        if PILOTS.contains(&pid) {
            continue;
        }
        let prof = Json::parse(profile.as_deref().unwrap_or("{}"))?;
        let encs: Vec<Encounter> = encounters
            .query_map([pid], |r| {
                Ok((
                    r.get::<_, i64>(0)?,
                    r.get::<_, String>(1)?,
                    r.get::<_, String>(2)?,
                    r.get::<_, String>(3)?,
                    r.get::<_, Option<String>>(4)?,
                    r.get::<_, Option<String>>(5)?,
                ))
            })?
            .map(|row| -> Result<Encounter> {
                let (order, d, kind, dept, complaint, note) = row?;
                Ok(Encounter {
                    order,
                    date: d.parse()?,
                    kind,
                    dept,
                    complaint,
                    note,
                })
            })
            .collect::<Result<_>>()?;
        let unit = format!("unit-{pid}");
        let mrn = format!("MRN0000{pid}");
        let mut r = rng_for(&format!("seed:{pid}"));
        let (dates, shift) = practice_dates(pid, &encs);
        let a0 = prof
            .get("age_at_first_encounter")
            .and_then(Json::int)
            .filter(|a| *a != 0)
            .unwrap_or(age);
        let birth =
            encs[0].date + days(shift) - days((a0 as f64 * 365.25) as i64 + r.randint(0, 364));
        let before_birthday = (AS_OF.month(), AS_OF.day()) < (birth.month(), birth.day());
        let age_now = i64::from(AS_OF.year() - birth.year()) - i64::from(before_birthday);
        let sx = if sex == "F" { "female" } else { "male" };
        let o = origin(race_eth.as_deref().unwrap_or(""));
        let (race, eth) = omb(o);

        let curated = CURATED.iter().find(|c| c.0 == o);
        let (mut given, mut family) = (String::new(), String::new());
        for attempt in 0..200i64 {
            if let Some((_, fams, men, women)) = curated {
                given = (*r.choice(if sx == "male" { &men[..] } else { &women[..] })).into();
                family = (*r.choice(&fams[..])).into();
            } else {
                let mut fams: Vec<&str> = (pool.iter())
                    .filter(|m| m.origin() == o)
                    .map(|m| m.family.as_str())
                    .collect();
                if fams.is_empty() {
                    fams = pool.iter().map(|m| m.family.as_str()).collect();
                }
                let near = |span: i64, same_origin: bool| -> Vec<&str> {
                    (pool.iter())
                        .filter(|m| m.sex == sx && (!same_origin || m.origin() == o))
                        .filter(|m| i64::from((m.birth_year - birth.year() as i32).abs()) <= span)
                        .map(|m| m.given.as_str())
                        .collect()
                };
                let mut names = near(7 + attempt / 20, true);
                if names.is_empty() {
                    names = near(10, false);
                }
                given = (*r.choice(&names)).into();
                family = (*r.choice(&fams)).into();
            }
            let fresh = !used_names.contains(&(given.clone(), family.clone()))
                && used_given.get(&given).copied().unwrap_or(0) < 3 + attempt / 50
                && used_family.get(&family).copied().unwrap_or(0) < 2 + attempt / 40;
            if fresh {
                break;
            }
        }
        used_names.insert((given.clone(), family.clone()));
        *used_given.entry(given.clone()).or_default() += 1;
        *used_family.entry(family.clone()).or_default() += 1;

        let (town, postal) = if o == "hispanic" && r.random() < 0.7 {
            *r.choice(&hispanic_towns)
        } else {
            *r.choice(&towns)
        };
        let phone = loop {
            let p = format!("+1-{}-555-01{:02}", r.choice(&AREA), r.randint(0, 99));
            if used_phone.insert(p.clone()) {
                break p;
            }
        };
        let email = if age_now >= 75 && r.random() < 0.3 {
            None
        } else {
            let base = format!("{given}.{family}")
                .to_lowercase()
                .replace([' ', '\''], "");
            let mut email = format!("{base}@example.test");
            let mut n = 2;
            while used_email.contains(&email) {
                email = format!("{base}{n}@example.test");
                n += 1;
            }
            used_email.insert(email.clone());
            Some(email)
        };

        let mut anchors: Vec<String> = Vec::new();
        let primary_list =
            Json::parse(primary.as_deref().unwrap_or("[]")).unwrap_or(Json::Arr(vec![]));
        let chronic = prof
            .get("chronic_conditions")
            .cloned()
            .unwrap_or(Json::Arr(vec![]));
        for a in chronic.items().iter().chain(primary_list.items()) {
            if let Json::Str(s) = a
                && !anchors.contains(s)
            {
                anchors.push(s.clone());
            }
        }
        let heavy = encs
            .iter()
            .any(|e| e.kind == "inpatient" || e.kind == "icu");
        let complexity = if anchors.len() <= 2 && encs.len() <= 3 {
            "light"
        } else if anchors.len() >= 7 || heavy {
            "complex"
        } else {
            "moderate"
        };
        let joined = dates[0] - days(r.randint(0, 3 * 365));
        let years = ((between(AS_OF, joined) as f64 / 365.25).round_ties_even() as i64).max(1);
        let docs = years * 8 + encs.len() as i64 * 4 + if complexity == "complex" { 20 } else { 0 };
        let text = |k: &str| prof.get(k).and_then(Json::text).filter(|s| !s.is_empty());
        let life: Vec<String> = [
            text("occupation").map(String::from),
            text("smoking_status").map(|s| format!("smoking: {s}")),
            text("alcohol_use").map(|s| format!("alcohol: {s}")),
        ]
        .into_iter()
        .flatten()
        .collect();

        // how complete the outside records are
        let h = Sha256::digest(format!("records:{unit}").as_bytes());
        let bucket = u32::from_be_bytes([h[0], h[1], h[2], h[3]]) % 100;
        let texture = TEXTURE[if bucket < 50 {
            0
        } else if bucket < 85 {
            1
        } else {
            2
        }]
        .1;

        // when the practice last saw the patient: never before its own visits in the
        // source, or before joining
        let mut rr = rng_for(&format!("recency:{mrn}"));
        let target = AS_OF - days(recency_days(&mut rr));
        let floor = (encs.iter().zip(&dates))
            .filter(|(e, _)| e.dept == "Internal Medicine" || e.dept == "Family Medicine")
            .map(|(_, d)| *d)
            .max();
        let joined_back = AS_OF - days((years as f64 * 365.25) as i64);
        let mut seen = target.max(joined_back + days(60));
        if let Some(f) = floor {
            seen = seen.max(f);
        }
        while seen.weekday().to_monday_zero_offset() >= 5 {
            seen -= days(1);
        }
        let least = floor.map_or(joined_back, |f| f.max(joined_back));
        if seen < least {
            seen = least;
        }
        seen = seen.min(AS_OF - days(1));
        let since = between(AS_OF, seen);
        let odds = if since < 183 {
            0.70
        } else if since < 366 {
            0.40
        } else {
            0.15
        };
        let booked = rr.random() < odds;

        let insurance_seed = (insurance.clone().filter(|s| !s.is_empty()))
            .or_else(|| text("insurance").map(String::from))
            .unwrap_or_else(|| "Commercial PPO".into());
        let mut member = json!({
            "mrn": mrn, "sex": sx, "age_at_as_of": age_now, "birth_year": birth.year(),
            "town": town, "postal_code": postal, "race": race, "ethnicity": eth,
            "preferred_language": text("language").unwrap_or("English"),
            "insurance": insurance_seed,
            "occupation_hint": text("occupation").unwrap_or("").to_lowercase(),
            "complexity": complexity, "years_with_practice": years,
            "target_documents": docs, "target_observations": docs * 7,
            "anchors": anchors,
            "episodes": ["source-first: every event comes from source.md and build-req.md"],
            "chart_texture": [texture], "last_seen": seen.to_string(), "booked": booked,
            "life": life, "birth_date": birth.to_string(), "given": given, "family": family,
            "phone": phone,
        });
        if let Some(e) = email {
            member["email"] = json!(e);
        }
        let seed = serde_json::from_value(json!({
            "unit": unit, "kind": "individual", "members": [member]
        }))?;

        let mut lines: Vec<String> = vec![
            format!("# synthetic_hospital patient {pid}"),
            String::new(),
            "From synthetic_hospital (https://github.com/sparkcpark/synthetic_hospital), \
             benchmark_v1.3.db, MIT, fully synthetic."
                .into(),
            format!(
                "Source: age {a0} at first encounter, sex {sex}, {}, {}.",
                or_none(&race_eth),
                or_none(&insurance)
            ),
            String::new(),
            "## Practice dates".into(),
            String::new(),
            "Each source encounter moved to end before the practice's today (2026-08-27) and \
             off the 15th."
                .into(),
            String::new(),
            "| # | Source date | Practice date | Type | Department |".into(),
            "|---|---|---|---|---|".into(),
        ];
        for (e, p) in encs.iter().zip(&dates) {
            lines.push(format!(
                "| {} | {} | {p} ({}) | {} | {} |",
                e.order,
                e.date,
                p.strftime("%a"),
                e.kind,
                e.dept
            ));
        }
        lines.extend([
            String::new(),
            "## Profile".into(),
            String::new(),
            "```json".into(),
            prof.python(0),
            "```".into(),
            String::new(),
            format!("Primary diagnoses: {}", or_none(&primary)),
            format!("Comorbidities: {}", or_none(&comorbid)),
            String::new(),
        ]);
        for (e, p) in encs.iter().zip(&dates) {
            lines.extend([
                format!(
                    "## Encounter {}: source {}, practice {p}, {}, {}",
                    e.order, e.date, e.kind, e.dept
                ),
                String::new(),
                format!("Chief complaint: {}", or_none(&e.complaint)),
                String::new(),
                strip(e.note.as_deref().unwrap_or("")).to_string(),
                String::new(),
            ]);
        }
        out.push(Drawn {
            unit,
            source: lines.join("\n"),
            seed,
        });
    }
    Ok(out)
}

/// Import into `patients`: add each unit's source.md and seed.json where it has
/// none, and never overwrite either. Returns 1 when an existing source.md differs
/// from the database.
pub fn run(db: &Path, patients: &Path) -> Result<i32> {
    let drawn = draw_all(db)?;
    let (mut sources, mut seeds, mut changed) = (0, 0, Vec::new());
    for d in &drawn {
        let dir = patients.join("units").join(&d.unit);
        fs::create_dir_all(&dir)?;
        let source = dir.join("source.md");
        match fs::read_to_string(&source) {
            Ok(old) if old == d.source => {}
            Ok(_) => changed.push(d.unit.as_str()),
            Err(_) => {
                write_atomic(&source, &d.source)?;
                sources += 1;
            }
        }
        let seed = dir.join("seed.json");
        if !seed.exists() {
            write_atomic(&seed, &d.seed.to_text()?)?;
            seeds += 1;
        }
    }
    println!(
        "{} adult patients, the {} pilots aside: {sources} source.md and {seeds} seed.json \
         added, {} source.md differing from the database",
        drawn.len(),
        PILOTS.len(),
        changed.len()
    );
    if !changed.is_empty() {
        println!("differs: {}", changed.join(" "));
    }
    Ok(i32::from(!changed.is_empty()))
}

/// JSON that keeps its key order, printed as Python's `json.dumps(v, indent=n)`.
#[derive(Clone, Debug)]
enum Json {
    Null,
    Bool(bool),
    Num(String),
    Str(String),
    Arr(Vec<Json>),
    Obj(Vec<(String, Json)>),
}

impl Json {
    fn parse(text: &str) -> Result<Json> {
        let mut p = Parser {
            s: text.as_bytes(),
            i: 0,
        };
        let v = p.value()?;
        p.ws();
        if p.i != p.s.len() {
            return Err("trailing text after JSON".into());
        }
        Ok(v)
    }

    fn get(&self, key: &str) -> Option<&Json> {
        match self {
            Json::Obj(kv) => kv.iter().find(|(k, _)| k == key).map(|(_, v)| v),
            _ => None,
        }
    }

    fn items(&self) -> &[Json] {
        match self {
            Json::Arr(v) => v,
            _ => &[],
        }
    }

    fn text(&self) -> Option<&str> {
        match self {
            Json::Str(s) => Some(s),
            _ => None,
        }
    }

    fn int(&self) -> Option<i64> {
        match self {
            Json::Num(n) => n.parse().ok(),
            _ => None,
        }
    }

    /// `json.dumps(self, indent=1)` at nesting `level`, ASCII only.
    fn python(&self, level: usize) -> String {
        let pad = |n: usize| " ".repeat(n);
        match self {
            Json::Null => "null".into(),
            Json::Bool(b) => b.to_string(),
            Json::Num(n) => n.clone(),
            Json::Str(s) => quote(s),
            Json::Arr(v) if v.is_empty() => "[]".into(),
            Json::Obj(v) if v.is_empty() => "{}".into(),
            Json::Arr(v) => {
                let inner: Vec<String> = (v.iter())
                    .map(|x| format!("{}{}", pad(level + 1), x.python(level + 1)))
                    .collect();
                format!("[\n{}\n{}]", inner.join(",\n"), pad(level))
            }
            Json::Obj(v) => {
                let inner: Vec<String> = (v.iter())
                    .map(|(k, x)| {
                        format!("{}{}: {}", pad(level + 1), quote(k), x.python(level + 1))
                    })
                    .collect();
                format!("{{\n{}\n{}}}", inner.join(",\n"), pad(level))
            }
        }
    }
}

/// A string as Python's `json.dumps` writes it with `ensure_ascii`.
fn quote(s: &str) -> String {
    let mut out = String::from("\"");
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            '\u{8}' => out.push_str("\\b"),
            '\u{c}' => out.push_str("\\f"),
            c if c.is_ascii() && (c as u32) >= 0x20 => out.push(c),
            c => {
                let mut buf = [0u16; 2];
                for unit in c.encode_utf16(&mut buf) {
                    out.push_str(&format!("\\u{unit:04x}"));
                }
            }
        }
    }
    out.push('"');
    out
}

struct Parser<'a> {
    s: &'a [u8],
    i: usize,
}

impl Parser<'_> {
    fn ws(&mut self) {
        while self.i < self.s.len() && matches!(self.s[self.i], b' ' | b'\n' | b'\r' | b'\t') {
            self.i += 1;
        }
    }

    fn eat(&mut self, b: u8) -> Result<()> {
        self.ws();
        if self.s.get(self.i) == Some(&b) {
            self.i += 1;
            Ok(())
        } else {
            Err(format!("JSON: expected {:?} at {}", b as char, self.i).into())
        }
    }

    fn value(&mut self) -> Result<Json> {
        self.ws();
        let rest = &self.s[self.i..];
        let word = |w: &str, v: Json, p: &mut Self| -> Result<Json> {
            p.i += w.len();
            Ok(v)
        };
        match rest.first() {
            Some(b'{') => {
                self.i += 1;
                let mut kv = Vec::new();
                self.ws();
                if self.s.get(self.i) == Some(&b'}') {
                    self.i += 1;
                    return Ok(Json::Obj(kv));
                }
                loop {
                    self.ws();
                    let k = self.string()?;
                    self.eat(b':')?;
                    kv.push((k, self.value()?));
                    self.ws();
                    match self.s.get(self.i) {
                        Some(b',') => self.i += 1,
                        Some(b'}') => {
                            self.i += 1;
                            return Ok(Json::Obj(kv));
                        }
                        _ => return Err(format!("JSON: bad object at {}", self.i).into()),
                    }
                }
            }
            Some(b'[') => {
                self.i += 1;
                let mut v = Vec::new();
                self.ws();
                if self.s.get(self.i) == Some(&b']') {
                    self.i += 1;
                    return Ok(Json::Arr(v));
                }
                loop {
                    v.push(self.value()?);
                    self.ws();
                    match self.s.get(self.i) {
                        Some(b',') => self.i += 1,
                        Some(b']') => {
                            self.i += 1;
                            return Ok(Json::Arr(v));
                        }
                        _ => return Err(format!("JSON: bad array at {}", self.i).into()),
                    }
                }
            }
            Some(b'"') => Ok(Json::Str(self.string()?)),
            _ if rest.starts_with(b"null") => word("null", Json::Null, self),
            _ if rest.starts_with(b"true") => word("true", Json::Bool(true), self),
            _ if rest.starts_with(b"false") => word("false", Json::Bool(false), self),
            _ => {
                let start = self.i;
                while self.i < self.s.len()
                    && matches!(
                        self.s[self.i],
                        b'-' | b'+' | b'.' | b'e' | b'E' | b'0'..=b'9'
                    )
                {
                    self.i += 1;
                }
                if start == self.i {
                    return Err(format!("JSON: unexpected byte at {start}").into());
                }
                Ok(Json::Num(
                    String::from_utf8_lossy(&self.s[start..self.i]).into(),
                ))
            }
        }
    }

    fn string(&mut self) -> Result<String> {
        if self.s.get(self.i) != Some(&b'"') {
            return Err(format!("JSON: expected a string at {}", self.i).into());
        }
        self.i += 1;
        let mut out: Vec<u16> = Vec::new();
        let mut raw: Vec<u8> = Vec::new();
        let flush = |raw: &mut Vec<u8>, out: &mut Vec<u16>| {
            out.extend(String::from_utf8_lossy(raw).encode_utf16());
            raw.clear();
        };
        loop {
            let Some(&b) = self.s.get(self.i) else {
                return Err("JSON: unterminated string".into());
            };
            self.i += 1;
            match b {
                b'"' => break,
                b'\\' => {
                    flush(&mut raw, &mut out);
                    let e = *self.s.get(self.i).ok_or("JSON: bad escape")?;
                    self.i += 1;
                    match e {
                        b'u' => {
                            let hex = std::str::from_utf8(&self.s[self.i..self.i + 4])?;
                            out.push(u16::from_str_radix(hex, 16)?);
                            self.i += 4;
                        }
                        b'n' => out.push(u16::from(b'\n')),
                        b't' => out.push(u16::from(b'\t')),
                        b'r' => out.push(u16::from(b'\r')),
                        b'b' => out.push(8),
                        b'f' => out.push(12),
                        other => out.push(u16::from(other)),
                    }
                }
                b => raw.push(b),
            }
        }
        flush(&mut raw, &mut out);
        Ok(String::from_utf16_lossy(&out))
    }
}
