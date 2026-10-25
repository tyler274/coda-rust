//! The panel's seeds: who each patient is before anyone writes their chart.
//!
//! `patients/seeds.json` was designed once (seed 20261024) for all 995 patients and is
//! kept as data: a chart is authored against its seed, so a seed never changes
//! under an accepted ledger. `stories` redraws the clinical story (anchors,
//! episodes, life, occupation) only for units that have no accepted ledger yet.

use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::Result;
use crate::ledger::Day;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Seeds {
    pub seed: u64,
    pub as_of: String,
    pub units: Vec<Unit>,
}

impl Seeds {
    pub fn read(path: &Path) -> Result<Seeds> {
        let text = std::fs::read_to_string(path)?;
        Ok(serde_json::from_str(&text).map_err(|e| format!("{}: {e}", path.display()))?)
    }

    /// The seeds file as written: one-space indented JSON and a trailing newline, the
    /// layout the file was first written in, so untouched seeds keep their bytes.
    pub fn to_text(&self) -> Result<String> {
        let mut out = Vec::new();
        let fmt = serde_json::ser::PrettyFormatter::with_indent(b" ");
        self.serialize(&mut serde_json::Serializer::with_formatter(&mut out, fmt))?;
        out.push(b'\n');
        Ok(String::from_utf8(out)?)
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
    /// The main threads of the history. With `coded_anchors` these are exact catalog
    /// condition labels and make up the problem list the chart must hold.
    pub anchors: Vec<String>,
    /// Concrete things that happened somewhere in the history.
    pub episodes: Vec<String>,
    pub chart_texture: Vec<String>,
    /// Who the patient is outside the clinic: background, household, work, beliefs.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub life: Vec<String>,
    /// Drawn by `stories`: the anchors are catalog labels and `check` holds the
    /// chart's problem list to them.
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
