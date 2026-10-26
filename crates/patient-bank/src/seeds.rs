//! The panel's seeds: who each patient is before anyone writes their chart.
//!
//! Each unit's `seed.json` in `patients/units/<unit>/` was designed once and is kept
//! as data: a chart is authored against its seed, so a seed never changes under an
//! accepted ledger.

use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::Result;
use crate::ledger::Day;

/// Every unit of a patient set, in unit order.
#[derive(Clone, Debug)]
pub struct Seeds {
    pub units: Vec<Unit>,
}

impl Seeds {
    /// The seed of every unit folder under `patients/units/`.
    pub fn read(patients: &Path) -> Result<Seeds> {
        let dir = patients.join("units");
        let mut units = Vec::new();
        let entries = std::fs::read_dir(&dir).map_err(|e| format!("{}: {e}", dir.display()))?;
        for e in entries {
            let path = e?.path().join("seed.json");
            if path.is_file() {
                let text = std::fs::read_to_string(&path)?;
                let unit: Unit =
                    serde_json::from_str(&text).map_err(|e| format!("{}: {e}", path.display()))?;
                units.push(unit);
            }
        }
        if units.is_empty() {
            return Err(format!("{} has no unit with a seed.json", dir.display()).into());
        }
        units.sort_by(|a, b| a.unit.cmp(&b.unit));
        Ok(Seeds { units })
    }

    pub fn unit(&self, id: &str) -> Option<&Unit> {
        self.units.iter().find(|u| u.unit == id)
    }

    pub fn member(&self, mrn: &str) -> Option<(&Unit, &Seed)> {
        self.units
            .iter()
            .find_map(|u| u.members.iter().find(|m| m.mrn == mrn).map(|m| (u, m)))
    }
}

/// One authoring unit: a patient, or a family authored together.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Unit {
    pub unit: String,
    pub kind: String,
    pub members: Vec<Seed>,
}

impl Unit {
    /// A seed file as written: one-space indented JSON and a trailing newline.
    pub fn to_text(&self) -> Result<String> {
        let mut out = Vec::new();
        let fmt = serde_json::ser::PrettyFormatter::with_indent(b" ");
        self.serialize(&mut serde_json::Serializer::with_formatter(&mut out, fmt))?;
        out.push(b'\n');
        Ok(String::from_utf8(out)?)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Complexity {
    Light,
    Moderate,
    Complex,
}

impl Complexity {
    pub fn as_str(self) -> &'static str {
        match self {
            Complexity::Light => "light",
            Complexity::Moderate => "moderate",
            Complexity::Complex => "complex",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Sex {
    Female,
    Male,
}

impl Sex {
    pub fn as_str(self) -> &'static str {
        match self {
            Sex::Female => "female",
            Sex::Male => "male",
        }
    }
}

/// Fields in the order the seeds file lists them, so a rewrite keeps its bytes.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Seed {
    pub mrn: String,
    pub sex: Sex,
    pub age_at_as_of: u32,
    pub birth_year: i32,
    pub town: String,
    pub postal_code: String,
    pub race: String,
    pub ethnicity: String,
    pub preferred_language: String,
    pub insurance: String,
    pub occupation_hint: String,
    pub complexity: Complexity,
    pub years_with_practice: u32,
    pub target_documents: u32,
    pub target_observations: u32,
    /// The main threads of the history.
    pub anchors: Vec<String>,
    /// Concrete things that happened somewhere in the history.
    pub episodes: Vec<String>,
    pub chart_texture: Vec<String>,
    /// The day the practice last saw the patient: its last held visit. Nothing in the
    /// practice's care comes after it but messages, refills and outside records.
    pub last_seen: Day,
    /// Whether a visit is booked on the practice calendar after today.
    pub booked: bool,
    /// Who the patient is outside the clinic: background, household, work, beliefs.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub life: Vec<String>,
    /// The anchors are exact catalog condition labels.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub coded_anchors: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub family_id: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub relationships: Vec<Relationship>,
    pub birth_date: Day,
    pub given: String,
    pub family: String,
    pub phone: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
}

impl Seed {
    pub fn female(&self) -> bool {
        self.sex == Sex::Female
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Relationship {
    pub mrn: String,
    pub relation: String,
}
