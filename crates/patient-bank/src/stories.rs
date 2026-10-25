//! Draw a concrete clinical story into every seed that has no accepted chart yet.
//!
//! A practice of a thousand patients is diverse the way a real roster is: common
//! conditions at primary-care rates, and every patient also carrying something of
//! their own. From `data/stories.json` each patient gets
//!
//! - common chronic conditions, as many as their complexity tier allows, weighted by
//!   age and sex;
//! - minor problems and smoking history, each at its own rate;
//! - one distinctive long-tail condition (two when complex), each used by at most
//!   five patients in the panel;
//! - for some, one hospital-grade event, each used by at most three patients;
//! - concrete episodes (which injury, what the colonoscopy found) drawn at real rates;
//! - a life context (two when complex), each shared by at most ten patients, and
//!   an occupation.
//!
//! Labels that exclude one another share a group, and an item is drawn only if it
//! fits the labels already chosen. Anchors are exact catalog labels, so `check` can
//! hold the chart's problem list to them. Seeds with an accepted ledger keep their
//! story: the chart was written against it. Draws are seeded per patient, and the
//! panel-wide use limits are counted in seeds order, so a rerun gives the same file.

use std::collections::{BTreeMap, HashMap, HashSet};
use std::path::Path;

use serde::Deserialize;

use crate::catalog::{Catalog, write_atomic};
use crate::rng::Rng;
use crate::seeds::{Complexity, Seed, Seeds, Sex};
use crate::{Result, paths};

const DISTINCT_USES: usize = 5;
const SERIOUS_USES: usize = 3;
const LIFE_USES: usize = 10;

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Stories {
    groups: Vec<Grouped>,
    common: Vec<Common>,
    minor: Vec<Minor>,
    distinct: Vec<Distinct>,
    serious: Vec<Serious>,
    episodes: Vec<Kind>,
    life: Vec<Life>,
    occupations: Vec<String>,
    retired: Vec<String>,
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Grouped {
    label: String,
    group: String,
}

/// Who an item can happen to.
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Fits {
    min_age: u32,
    max_age: u32,
    sex: Option<Sex>,
}

impl Fits {
    fn fits(&self, s: &Seed) -> bool {
        (self.min_age..=self.max_age).contains(&s.age_at_as_of)
            && self.sex.is_none_or(|x| x == s.sex)
    }
}

#[derive(Deserialize)]
struct Common {
    label: String,
    weight: f64,
    age_scaled: bool,
    #[serde(flatten)]
    fits: Fits,
}

#[derive(Deserialize)]
struct Minor {
    label: String,
    rate: f64,
    #[serde(flatten)]
    fits: Fits,
}

#[derive(Deserialize)]
struct Distinct {
    label: String,
    story: String,
    #[serde(flatten)]
    fits: Fits,
}

#[derive(Deserialize)]
struct Serious {
    story: String,
    adds: Vec<String>,
    #[serde(flatten)]
    fits: Fits,
}

#[derive(Deserialize)]
struct Kind {
    /// Names the row for readers of the table.
    #[serde(rename = "kind")]
    _kind: String,
    options: Vec<Episode>,
    #[serde(flatten)]
    fits: Fits,
}

#[derive(Deserialize)]
struct Episode {
    story: String,
    weight: f64,
    adds: Vec<String>,
    #[serde(flatten)]
    fits: Fits,
}

#[derive(Deserialize)]
struct Life {
    story: String,
    #[serde(flatten)]
    fits: Fits,
}

impl Stories {
    pub fn read(path: &Path) -> Result<Stories> {
        let text = std::fs::read_to_string(path)?;
        Ok(serde_json::from_str(&text).map_err(|e| format!("{}: {e}", path.display()))?)
    }

    /// Every label the tables can put on a problem list.
    fn labels(&self) -> impl Iterator<Item = &str> {
        let common = self.common.iter().map(|c| c.label.as_str());
        let minor = self.minor.iter().map(|m| m.label.as_str());
        let distinct = self.distinct.iter().map(|d| d.label.as_str());
        let serious = self
            .serious
            .iter()
            .flat_map(|s| s.adds.iter().map(String::as_str));
        let episodes = self.episodes.iter().flat_map(|k| {
            k.options
                .iter()
                .flat_map(|o| o.adds.iter().map(String::as_str))
        });
        common
            .chain(minor)
            .chain(distinct)
            .chain(serious)
            .chain(episodes)
    }
}

/// One patient's chosen labels, with the groups they occupy.
struct Chosen<'a> {
    labels: Vec<&'a str>,
    groups: &'a HashMap<&'a str, &'a str>,
}

impl<'a> Chosen<'a> {
    /// A label fits if it is already chosen or its group is still free.
    fn fits(&self, label: &str) -> bool {
        if self.labels.contains(&label) {
            return true;
        }
        let Some(group) = self.groups.get(label) else {
            return true;
        };
        !self
            .labels
            .iter()
            .any(|l| self.groups.get(l) == Some(group))
    }

    fn all_fit(&self, labels: &[String]) -> bool {
        labels.iter().all(|l| self.fits(l))
    }

    fn add(&mut self, label: &'a str) {
        if !self.labels.contains(&label) {
            self.labels.push(label);
        }
    }
}

fn pick_index(rng: &mut Rng, n: usize) -> usize {
    rng.randint(0, n as i64 - 1) as usize
}

fn weighted<T>(rng: &mut Rng, items: &[(T, f64)]) -> Option<usize> {
    let total: f64 = items.iter().map(|i| i.1).sum();
    if items.is_empty() || total <= 0.0 {
        return None;
    }
    let mut left = rng.uniform(0.0, total);
    for (k, item) in items.iter().enumerate() {
        left -= item.1;
        if left <= 0.0 {
            return Some(k);
        }
    }
    Some(items.len() - 1)
}

/// Among the eligible items, one of those used least so far, so the long tail
/// spreads across the panel instead of clustering.
fn least_used<T>(rng: &mut Rng, eligible: &[(usize, &T)], uses: &[usize]) -> Option<usize> {
    let fewest = eligible.iter().map(|(k, _)| uses[*k]).min()?;
    let tied: Vec<usize> = eligible
        .iter()
        .map(|(k, _)| *k)
        .filter(|k| uses[*k] == fewest)
        .collect();
    Some(tied[pick_index(rng, tied.len())])
}

/// Panel-wide use counts for the capped tables.
struct Uses {
    distinct: Vec<usize>,
    serious: Vec<usize>,
    life: Vec<usize>,
}

fn draw(st: &Stories, groups: &HashMap<&str, &str>, uses: &mut Uses, seed: &Seed) -> Seed {
    let mut rng = Rng::from_text(&format!("stories:{}", seed.mrn));
    let age = seed.age_at_as_of;
    let tier = seed.complexity;
    let mut chosen = Chosen {
        labels: Vec::new(),
        groups,
    };
    let mut episodes: Vec<String> = Vec::new();

    // Common chronic conditions, as many as the tier allows.
    let counts: &[usize] = match tier {
        Complexity::Light => &[0, 0, 1, 1, 2],
        Complexity::Moderate => &[1, 2, 2, 3],
        Complexity::Complex => &[3, 4, 4, 5, 6],
    };
    let wanted = counts[pick_index(&mut rng, counts.len())];
    for _ in 0..wanted {
        let pool: Vec<(&Common, f64)> = st
            .common
            .iter()
            .filter(|c| {
                c.fits.fits(seed)
                    && !chosen.labels.contains(&c.label.as_str())
                    && chosen.fits(&c.label)
            })
            .map(|c| {
                let scale = if c.age_scaled {
                    1.0 + f64::from(age.saturating_sub(45)) / 25.0
                } else {
                    1.0
                };
                (c, c.weight * scale)
            })
            .collect();
        let Some(k) = weighted(&mut rng, &pool) else {
            break;
        };
        chosen.add(&pool[k].0.label);
    }

    // Minor problems and smoking history, each at its own rate.
    for m in &st.minor {
        if m.fits.fits(seed) && chosen.fits(&m.label) && rng.random() < m.rate {
            chosen.add(&m.label);
        }
    }

    // The distinctive long tail.
    let distinct_count = if tier == Complexity::Complex { 2 } else { 1 };
    for _ in 0..distinct_count {
        let eligible: Vec<(usize, &Distinct)> = st
            .distinct
            .iter()
            .enumerate()
            .filter(|(k, d)| {
                uses.distinct[*k] < DISTINCT_USES
                    && d.fits.fits(seed)
                    && !chosen.labels.contains(&d.label.as_str())
                    && chosen.fits(&d.label)
            })
            .collect();
        let Some(k) = least_used(&mut rng, &eligible, &uses.distinct) else {
            break;
        };
        uses.distinct[k] += 1;
        chosen.add(&st.distinct[k].label);
        episodes.push(st.distinct[k].story.clone());
    }

    // A hospital-grade event for some.
    let serious_rate = match tier {
        Complexity::Light => 0.08,
        Complexity::Moderate => 0.18,
        Complexity::Complex => 0.32,
    };
    if rng.random() < serious_rate {
        let eligible: Vec<(usize, &Serious)> = st
            .serious
            .iter()
            .enumerate()
            .filter(|(k, s)| {
                uses.serious[*k] < SERIOUS_USES && s.fits.fits(seed) && chosen.all_fit(&s.adds)
            })
            .collect();
        if let Some(k) = least_used(&mut rng, &eligible, &uses.serious) {
            uses.serious[k] += 1;
            for a in &st.serious[k].adds {
                chosen.add(a);
            }
            episodes.push(st.serious[k].story.clone());
        }
    }

    // Concrete episodes: a few kinds, each resolved to one concrete thing.
    let wanted = match tier {
        Complexity::Light => 4,
        Complexity::Moderate => 7,
        Complexity::Complex => 10,
    };
    let mut kinds: Vec<&Kind> = st.episodes.iter().filter(|k| k.fits.fits(seed)).collect();
    rng.shuffle(&mut kinds);
    let mut taken = 0;
    for kind in kinds {
        if taken == wanted {
            break;
        }
        let options: Vec<(&Episode, f64)> = kind
            .options
            .iter()
            .filter(|o| o.fits.fits(seed) && chosen.all_fit(&o.adds))
            .map(|o| (o, o.weight))
            .collect();
        let Some(k) = weighted(&mut rng, &options) else {
            continue;
        };
        let option = options[k].0;
        for a in &option.adds {
            chosen.add(a);
        }
        episodes.push(option.story.clone());
        taken += 1;
    }

    // Life context and occupation.
    let life_count = if tier == Complexity::Complex { 2 } else { 1 };
    let mut life = Vec::new();
    for _ in 0..life_count {
        let eligible: Vec<(usize, &Life)> = st
            .life
            .iter()
            .enumerate()
            .filter(|(k, l)| {
                uses.life[*k] < LIFE_USES && l.fits.fits(seed) && !life.contains(&l.story)
            })
            .collect();
        let Some(k) = least_used(&mut rng, &eligible, &uses.life) else {
            break;
        };
        uses.life[k] += 1;
        life.push(st.life[k].story.clone());
    }
    let jobs: Vec<&String> = if age >= 66 {
        st.retired.iter().collect()
    } else {
        st.occupations
            .iter()
            .filter(|o| age < 32 || !o.contains("student"))
            .collect()
    };
    let occupation = jobs[pick_index(&mut rng, jobs.len())].clone();

    Seed {
        anchors: chosen.labels.iter().map(|l| l.to_string()).collect(),
        episodes,
        life,
        occupation_hint: occupation,
        coded_anchors: true,
        ..seed.clone()
    }
}

/// Redraw the story of every unit with no accepted ledger in `ledgers`, and write
/// the seeds back in place. Returns the count of units redrawn.
pub fn run(seeds_path: &Path, ledgers: &Path) -> Result<i32> {
    if !ledgers.is_dir() {
        return Err(format!("{} is not a directory of ledgers", ledgers.display()).into());
    }
    let st = Stories::read(&paths::stories())?;
    let catalog = Catalog::load(&paths::catalog())?;
    let unknown: Vec<&str> = st
        .labels()
        .filter(|l| !catalog.conditions.contains_key(*l))
        .collect();
    if !unknown.is_empty() {
        return Err(format!("stories.json names labels not in the catalog: {unknown:?}").into());
    }
    let groups: HashMap<&str, &str> = st
        .groups
        .iter()
        .map(|g| (g.label.as_str(), g.group.as_str()))
        .collect();
    let mut seeds = Seeds::read(seeds_path)?;
    let mut uses = Uses {
        distinct: vec![0; st.distinct.len()],
        serious: vec![0; st.serious.len()],
        life: vec![0; st.life.len()],
    };
    let mut kept = HashSet::new();
    let mut redrawn = 0;
    for unit in &mut seeds.units {
        if ledgers.join(format!("{}.json", unit.unit)).exists() {
            kept.insert(unit.unit.clone());
            continue;
        }
        for m in &mut unit.members {
            *m = draw(&st, &groups, &mut uses, m);
        }
        redrawn += 1;
    }
    write_atomic(seeds_path, &seeds.to_text()?)?;

    // How the long tail spread, for the report.
    let members: Vec<&Seed> = seeds
        .units
        .iter()
        .filter(|u| !kept.contains(&u.unit))
        .flat_map(|u| &u.members)
        .collect();
    let mut prevalence: BTreeMap<&str, usize> = BTreeMap::new();
    for m in &members {
        for a in &m.anchors {
            *prevalence.entry(a).or_default() += 1;
        }
    }
    let mut top: Vec<(&str, usize)> = prevalence.iter().map(|(k, v)| (*k, *v)).collect();
    top.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(b.0)));
    println!(
        "redrew {redrawn} units ({} patients); kept {} with accepted ledgers",
        members.len(),
        kept.len()
    );
    println!(
        "distinct problem labels across redrawn patients: {}",
        prevalence.len()
    );
    for (label, n) in top.iter().take(15) {
        println!(
            "  {:>4.1}%  {label}",
            100.0 * *n as f64 / members.len().max(1) as f64
        );
    }
    Ok(0)
}
