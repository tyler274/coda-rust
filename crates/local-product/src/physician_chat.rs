//! Hosted physician chat: physician-owned instructions and the skill tool.
//!
//! The hosted worker composes these with [`crate::runtime::PhysicianHost`].
//! The catalog is read once through [`harness::skills::Package`]; this module adds no
//! parser, filesystem reader or second catalog. A skill is guidance only: it
//! never grants a tool, a patient read or an action. The chart tools are scoped separately. Book: `book/src/physician-site.md`.

use std::path::{Path, PathBuf};

use harness::engine::{Event, Tool, ToolOutcome};
use harness::skills::{Package, PreloadError, PreloadedGuidance, SkillRunTrace, SkillTarget};
use serde_json::{Value, json};

/// Audience and workflow role of a Chords job, deciding its resident prompt and initial preloaded skills.
#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PhysicianJob {
    /// Interactive conversation with the physician; starts with catalog metadata only and opens skills on demand.
    Conversation,
    /// Unattended chart and message review deciding between quiet Updates and Today concerns.
    PatientReview,
    /// Physician-directed revision of a saved patient-facing reply draft.
    ReplyRevision,
    /// Pre-visit synthesis preparing trajectory, visit decisions, and the visit card orientation sentence.
    VisitBriefing,
}

impl PhysicianJob {
    /// Every Chords job role defined by this module.
    pub const ALL: [Self; 4] = [
        Self::Conversation,
        Self::PatientReview,
        Self::ReplyRevision,
        Self::VisitBriefing,
    ];

    /// Exact packaged guidance targets preloaded at job start. Interactive chat starts empty.
    pub fn preloaded_targets(self) -> Vec<SkillTarget> {
        match self {
            Self::Conversation => Vec::new(),
            Self::PatientReview => vec![
                SkillTarget::body("clinical-communication"),
                SkillTarget::body("physician-clinical-communication"),
                SkillTarget::resource(
                    "physician-clinical-communication",
                    "references/reviews-and-briefings.md",
                ),
                SkillTarget::body("patient-memory-maintenance"),
                SkillTarget::body("patient-record-retrieval"),
            ],
            Self::VisitBriefing => vec![
                SkillTarget::body("clinical-communication"),
                SkillTarget::body("physician-clinical-communication"),
                SkillTarget::resource(
                    "physician-clinical-communication",
                    "references/reviews-and-briefings.md",
                ),
                SkillTarget::body("patient-record-retrieval"),
            ],
            Self::ReplyRevision => vec![
                SkillTarget::body("clinical-communication"),
                SkillTarget::body("patient-communication"),
            ],
        }
    }
}

/// Immutable chat recipe selected by the site when it accepts new work.
pub fn recipe(
    binding: &crate::ModelBinding,
    instructions: &str,
    skills: &PhysicianSkills,
) -> harness::engine::recipe::RequestRecipe {
    let mut recipe = harness::engine::recipe::RequestRecipe {
        model: binding.model.clone(),
        context_window_tokens: binding.context_window_tokens,
        max_input_tokens: binding.max_input_tokens,
        options: json!({"effort": binding.effort}),
        instructions: skills.instructions(instructions),
        tools: tools().into_iter().map(|tool| tool.function()).collect(),
        preloaded_skills: Vec::new(),
    };
    harness::engine::offer_working_notes(&mut recipe.instructions, &mut recipe.tools);
    recipe
}

/// Immutable recipe for a specific [`PhysicianJob`], preloading only the guidance its assignment always needs.
pub fn job_recipe(
    binding: &crate::ModelBinding,
    job: PhysicianJob,
    skills: &PhysicianSkills,
) -> harness::engine::recipe::RequestRecipe {
    match job {
        PhysicianJob::Conversation => recipe(binding, INSTRUCTIONS, skills),
        PhysicianJob::ReplyRevision => crate::runtime::reply::recipe(binding, skills),
        PhysicianJob::PatientReview => background_physician_recipe(
            binding,
            REVIEW_INSTRUCTIONS,
            PhysicianJob::PatientReview,
            skills,
        ),
        PhysicianJob::VisitBriefing => background_physician_recipe(
            binding,
            BRIEFING_INSTRUCTIONS,
            PhysicianJob::VisitBriefing,
            skills,
        ),
    }
}

fn background_physician_recipe(
    binding: &crate::ModelBinding,
    base_instructions: &str,
    job: PhysicianJob,
    skills: &PhysicianSkills,
) -> harness::engine::recipe::RequestRecipe {
    let (instructions, preloaded_skills) = skills.prepare_job(base_instructions, job);
    let mut recipe = harness::engine::recipe::RequestRecipe {
        model: binding.model.clone(),
        context_window_tokens: binding.context_window_tokens,
        max_input_tokens: binding.max_input_tokens,
        options: json!({"effort": binding.effort}),
        instructions,
        tools: tools().into_iter().map(|tool| tool.function()).collect(),
        preloaded_skills,
    };
    harness::engine::offer_working_notes(&mut recipe.instructions, &mut recipe.tools);
    recipe
}

/// Exact retained conversation reads for history omitted from a fresh window.
pub use harness::engine::history::LIST as LIST_CONVERSATION;
pub use harness::skills::SKILL_VIEW;

/// Resident guidance for Chords chat.
pub const INSTRUCTIONS: &str = "\
You are Coda's lead for this conversation, its chief resident for the physician and this patient. \
Own the clinical understanding, investigation, judgment and useful follow-through the physician asks for, \
within the capabilities available here.

Work from the physician's intent and the conversation so far. Understand what outcome is needed, choose a \
responsible approach and carry it through. Ordinary conversation is enough to direct this work; the \
physician need not name tools or use special phrases. Take the next useful cognitive step: make a \
recommendation and proceed through ordinary uncertainty with responsible assumptions. Ask when a material \
ambiguity or decision requires physician input.

Treat new physician input as part of a continuing relationship. Reconcile corrections and changed direction \
with prior understanding, and explain a material disagreement rather than silently accepting or dismissing it. \
Honor clear physician direction within available authority.

Use the tools offered in this conversation. For patient-specific facts across chart records, earlier work, \
and durable memory, discover candidates with search_patient_information and open exact chart revisions with \
read_patient_sources.

The approved skill catalog describes reusable capability guidance and exact packaged resources. Review it \
when a task begins and whenever the task scope changes. Before relying on a matching, materially \
overlapping, or explicitly invoked skill whose body is absent, load it through skill_view, even when the \
work seems familiar. Read a skill's returned supporting resources when they are useful. Do not reread a \
body that is already in this conversation. Group independent reads with known arguments in one response; \
wait for results before choosing dependent reads.

Physician direction governs this conversation. You may reason, recommend, and draft within the tools \
offered here; you cannot execute orders, send messages, or write back.

Answer the physician's request directly, applying the relevant clinical and audience communication skills \
from the catalog. Requesting tools continues work; your final response without tool calls is the complete \
answer the physician reads, so do not leave part of the answer only in earlier tool-round messages.";

/// Assignment guidance for unattended Chords patient reviews.
pub const REVIEW_INSTRUCTIONS: &str = "\
Review new patient workspace evidence and messages for the physician, applying the preloaded clinical, \
physician communication, patient memory maintenance, and patient record retrieval guidance. Read durable \
patient memory with read_patient_memory, discover items with search_patient_information, and open exact \
revisions with read_patient_sources before drawing conclusions or updating memory. When the review also \
requires guidance that is not preloaded — such as drafting a patient-facing reply — open that skill from \
the catalog with skill_view. Requesting tools continues work; your final response without tool calls \
records the completed review.";

/// Assignment guidance for Chords pre-visit briefings.
pub const BRIEFING_INSTRUCTIONS: &str = "\
Prepare a pre-visit briefing for the physician ahead of the upcoming encounter, applying the preloaded \
clinical, physician communication, and patient record retrieval guidance. Discover relevant items with \
search_patient_information and open exact revisions with read_patient_sources before synthesizing the \
patient's trajectory and visit decisions, and open patient-memory-maintenance with skill_view when reading \
or updating durable patient memory. Open additional catalog skills with skill_view only when the task \
calls for them. Requesting tools continues work; your final response without tool calls delivers the \
completed briefing.";

/// Capabilities shared by every ordinary Chords chat request.
pub fn tools() -> Vec<Tool> {
    let mut offered = work_tools();
    offered.extend(chart_tools());
    offered.extend(crate::runtime::memory::tools());
    offered
}

/// Shared capabilities implemented by durable Chords chat and revision work.
/// Each recipe adds the working notes with their guidance.
pub fn work_tools() -> Vec<Tool> {
    let mut offered = vec![PhysicianSkills::tool(), harness::engine::results::tool()];
    offered.extend(harness::engine::history::tools());
    offered
}

/// Source and conversation reads have no product effects or shared session mutation.
pub fn read_only(name: &str) -> bool {
    matches!(
        name,
        crate::runtime::patient_search::SEARCH_TOOL | crate::physician_sources::READ_TOOL
    ) || crate::runtime::memory::is_read_only(name)
}

fn chart_tools() -> [Tool; 2] {
    [
        crate::runtime::patient_search::search_tool(),
        crate::physician_sources::read_tool(crate::physician_sources::ChartScope::Chat),
    ]
}

/// The whole Chords skill package. Every Chords job lists and opens every skill
/// in it. The package is saved with accepted work and every attempt reads that
/// same package. Construction fails when the catalog is missing or invalid;
/// there is no empty fallback.
#[derive(Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(transparent)]
pub struct PhysicianSkills {
    package: Package,
}

/// The physician skill catalog could not be loaded.
#[derive(Debug, thiserror::Error)]
pub enum SkillsError {
    /// A hosted site names no catalog folder.
    #[error("hosted physician site requires PHYSICIAN_SKILLS_DIR")]
    Unconfigured,
    /// The package at `root` is missing or invalid, with the package's reason.
    #[error("physician skill catalog at {} is unavailable: {reason}", .root.display())]
    Package {
        /// The catalog folder.
        root: PathBuf,
        /// Why the package did not load, in the package loader's words.
        reason: String,
    },
    /// A packaged skill or supporting resource required by a Chords job is missing.
    #[error("physician skill catalog at {} is missing required guidance for {job:?}", .root.display())]
    RequiredGuidance {
        /// The catalog folder.
        root: PathBuf,
        /// The Chords job whose preloaded guidance failed validation.
        job: PhysicianJob,
        /// Exact missing skill or resource from the package loader.
        #[source]
        source: PreloadError,
    },
}

impl PhysicianSkills {
    /// Loads the configured package, or the workspace skills during local development.
    pub fn configured(dir: Option<&str>, hosted: bool) -> Result<Self, SkillsError> {
        let root = match dir.filter(|dir| !dir.trim().is_empty()) {
            Some(dir) => Path::new(dir),
            None if hosted => return Err(SkillsError::Unconfigured),
            None => Path::new(concat!(env!("CARGO_MANIFEST_DIR"), "/../../skills")),
        };
        Self::load(root)
    }

    /// Loads `root` once under the Chords visibility rule and verifies every job's preload targets.
    pub fn load(root: &Path) -> Result<Self, SkillsError> {
        let package = Package::load_visible("chords", root, |skill| {
            skill.visibility.admits_or_unrestricted("chords")
        })
        .map_err(|reason| SkillsError::Package {
            root: root.to_owned(),
            reason,
        })?;
        for job in PhysicianJob::ALL {
            package
                .preload(&job.preloaded_targets())
                .map_err(|source| SkillsError::RequiredGuidance {
                    root: root.to_owned(),
                    job,
                    source,
                })?;
        }
        Ok(Self { package })
    }

    /// Admitted Chords [`Package`] frozen with accepted work.
    pub fn package(&self) -> &Package {
        &self.package
    }

    /// Validated preloaded guidance and exact `version_id`s for `job`.
    pub fn preload_for_job(&self, job: PhysicianJob) -> Result<PreloadedGuidance, PreloadError> {
        self.package.preload(&job.preloaded_targets())
    }

    /// `base` followed by the catalog section. The accepted recipe keeps the exact text.
    pub fn instructions(&self, base: &str) -> String {
        format!("{base}\n\n{}", self.package.catalog_instructions())
    }

    /// `base` followed by `job`'s preloaded guidance (when non-empty) and the metadata-only catalog,
    /// paired with the exact [`SkillTarget`] selection recorded on [`RequestRecipe`](harness::engine::recipe::RequestRecipe).
    pub fn prepare_job(&self, base: &str, job: PhysicianJob) -> (String, Vec<SkillTarget>) {
        let targets = job.preloaded_targets();
        let preloaded = self
            .package
            .preload(&targets)
            .expect("PhysicianSkills validated job preload targets at construction");
        let instructions = if preloaded.instructions.is_empty() {
            self.instructions(base)
        } else {
            format!(
                "{base}\n\n{}\n\n{}",
                preloaded.instructions,
                self.package.catalog_instructions()
            )
        };
        (instructions, targets)
    }

    /// `base` followed by `job`'s preloaded guidance (when non-empty) and the metadata-only catalog.
    pub fn instructions_for_job(&self, base: &str, job: PhysicianJob) -> String {
        self.prepare_job(base, job).0
    }

    /// Reconstructs preloaded and `skill_view`-discovered skill versions from a run's frozen recipe and events.
    pub fn trace_run(
        &self,
        recipe: &harness::engine::recipe::RequestRecipe,
        events: &[Event],
    ) -> Result<SkillRunTrace, PreloadError> {
        harness::skills::trace_run(&self.package, recipe, events)
    }

    /// The common skill tool offered for this product's package.
    pub fn tool() -> Tool {
        harness::skills::tool()
    }

    /// Runs one validated read through the loaded catalog. Unavailable and
    /// over-budget outcomes are results, not errors. The returned JSON is the
    /// exact result the model receives; refusals carry no executed read.
    pub fn execute(&self, name: &str, arguments: &Value) -> ToolOutcome {
        if name != SKILL_VIEW {
            return refusal(&format!("unknown tool {name}"));
        }
        match self.package.execute_skill_view(arguments) {
            Ok(view) => ToolOutcome::Returned(view.to_json()),
            Err(reason) => refusal(reason.message()),
        }
    }
}

fn refusal(reason: &str) -> ToolOutcome {
    ToolOutcome::refused(reason)
}

#[cfg(test)]
#[path = "../../../verification/model_text.rs"]
mod model_text;

#[cfg(test)]
mod tests {
    use super::model_text::{assert_clean_model_text, assert_skill_tools};
    use super::*;
    use std::path::PathBuf;

    fn skills() -> PhysicianSkills {
        PhysicianSkills::load(&PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../skills"))
            .unwrap()
    }

    #[test]
    fn instructions_carry_the_metadata_only_catalog() {
        let skills = skills();
        let text = skills.instructions(INSTRUCTIONS);
        assert!(text.starts_with(INSTRUCTIONS));
        assert!(text.contains("\"kind\":\"approved_skill_catalog\""));
        assert!(text.contains("clinical-communication"));
        assert!(text.contains("physician-clinical-communication"));
        assert!(text.contains("patient-communication"));
        assert!(text.contains("patient-memory-maintenance"));
        assert!(text.contains("patient-record-retrieval"));
        assert!(!text.contains("duet-health-communication"));
        assert!(!text.contains("\"content\""));
        assert_eq!(PhysicianSkills::tool().name, SKILL_VIEW);
    }

    #[test]
    fn a_missing_or_empty_catalog_fails_loading() {
        let missing = std::env::temp_dir().join(format!(
            "physician-skills-missing-{}",
            uuid::Uuid::new_v4().simple()
        ));
        let error = PhysicianSkills::load(&missing).err().unwrap().to_string();
        assert!(error.contains("physician skill catalog"), "{error}");
        std::fs::create_dir_all(&missing).unwrap();
        let error = PhysicianSkills::load(&missing).err().unwrap().to_string();
        assert!(error.contains("No packaged skills"), "{error}");
        std::fs::remove_dir_all(&missing).unwrap();
    }

    #[test]
    fn missing_required_job_skill_fails_loading_with_preload_cause() {
        let incomplete = std::env::temp_dir().join(format!(
            "physician-skills-incomplete-{}",
            uuid::Uuid::new_v4().simple()
        ));
        let retrieval = incomplete.join("patient-record-retrieval");
        std::fs::create_dir_all(&retrieval).unwrap();
        std::fs::copy(
            PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("../../skills/patient-record-retrieval/SKILL.md"),
            retrieval.join("SKILL.md"),
        )
        .unwrap();
        let err = PhysicianSkills::load(&incomplete).unwrap_err();
        assert!(
            matches!(err, SkillsError::RequiredGuidance { .. }),
            "{err:?}"
        );
        assert!(std::error::Error::source(&err).is_some());
        std::fs::remove_dir_all(&incomplete).unwrap();
    }

    #[test]
    fn model_text_has_no_implementation_labels() {
        let skills = skills();
        let binding = crate::ModelBinding {
            model: "fixture".into(),
            effort: "high".into(),
            context_window_tokens: 272_000,
            max_input_tokens: 250_000,
            adapter: "fixture".into(),
        };
        for job in PhysicianJob::ALL {
            let built = job_recipe(&binding, job, &skills);
            assert_clean_model_text(&built.instructions);
            assert_clean_model_text(&serde_json::to_string(&built.tools).unwrap());
        }
    }

    #[test]
    fn every_recipe_offers_working_notes_with_their_guidance_once() {
        let skills = skills();
        let binding = crate::ModelBinding {
            model: "fixture".into(),
            effort: "high".into(),
            context_window_tokens: 272_000,
            max_input_tokens: 250_000,
            adapter: "fixture".into(),
        };
        for job in PhysicianJob::ALL {
            let built = job_recipe(&binding, job, &skills);
            assert_eq!(
                built
                    .instructions
                    .matches("This work can outlast one context window.")
                    .count(),
                1
            );
            assert!(
                built
                    .tools
                    .iter()
                    .any(|tool| tool["name"] == "save_working_note")
            );
        }
    }

    #[test]
    fn chords_skills_only_name_tools_some_chords_job_offers() {
        let mut offered = tools().iter().map(Tool::function).collect::<Vec<_>>();
        offered.extend(crate::runtime::reply::tools());
        assert_skill_tools(&serde_json::to_value(skills()).unwrap(), &offered, &[]);
    }
}
