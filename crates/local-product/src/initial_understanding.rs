//! Recipe, initial context, preloaded skill binding, and tool descriptors for
//! building and revising a patient's initial working understanding.
//!
//! When a patient workspace is connected to a practice, Coda reads the available
//! source evidence on [`crate::ModelRoles::background`] and saves two distinct
//! parts of patient memory:
//! - [`OperatingIndex`]: Coda's private cross-run working memory, never treated
//!   as medication truth and never exposed in physician-facing responses.
//! - [`crate::runtime::understanding::PatientSummary`]: the physician-facing
//!   clinical summary on the patient page, carrying explicit material gaps and
//!   verified source citations.
//!
//! Later source deliveries and record corrections revise the existing
//! understanding on the same `(practice, workspace)` link rather than restarting
//! cold.

use harness::engine::Tool;
use harness::engine::recipe::RequestRecipe;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

use crate::ModelBinding;
use crate::physician_chat::PhysicianSkills;
use crate::runtime::concerns::ExactRecordReference;
use crate::runtime::jobs::{Assignment, AssignmentError, PracticeJobKind, PreloadedSkill};

/// Read-only tool returning the patient's current operating index and summary.
pub const READ_UNDERSTANDING_TOOL: &str = "read_patient_understanding";

/// Write-barrier tool saving the patient's operating index and cited summary.
pub const SAVE_UNDERSTANDING_TOOL: &str = "save_patient_understanding";

/// System instructions for building and revising a patient's working understanding.
pub const INSTRUCTIONS: &str = "\
You are Coda, maintaining a practice's longitudinal clinical understanding of one patient.

When a patient is first connected to the practice, read the available source records and build an initial working understanding without waiting for a physician question. When later sources or record corrections arrive, call `read_patient_understanding` first and revise the existing understanding in place so unrelated clinical history, prior context, and valid anchors are preserved rather than restarted.

Patient memory has two distinct parts that you save together through `save_patient_understanding`:
1. Private operating index (`operating_index`): your own cross-run working memory. Record a concise `working_overview`, every connected or chart-mentioned outside facility or specialist in `known_health_systems`, unresolved clinical questions in `open_threads`, and exact record `anchors` (`candidate_id`, `revision`, `source_id`, `topic`). The operating index is working state, never medication truth, and is never shown directly to physicians. Verify active medications, doses, and allergies against source records whenever they matter.
2. Physician-facing summary (`claims`, `material_gaps`, and optional `scope_note`): the `Patient summary` shown on the patient page and open to conversational correction. Organize claims around what matters clinically, with dates and trajectories rather than raw document dumps. State `material_gaps` plainly, including missing outside records, pending tests, unconfirmed medication adherence, or intervals without clinical documentation.

Evidence and citation rules:
- Search the workspace with `search_patient_information` and inspect full records with `read_patient_sources` before relying on or citing them. Never claim to have read every record when you inspected a targeted subset.
- Every `ExactRecordReference` (`candidate_id` and `revision`) attached to `claims` must match a current workspace source record that you read via `read_patient_sources` in this run, or that remains current from the prior revision returned by `read_patient_understanding`.
- Note what other health systems, outside hospitals, or external clinics are mentioned in notes or referrals in `known_health_systems`, distinguishing connected sources from facilities only mentioned in records.
- When Duet patient-shared information is the only available source (`duet_patient_shared_only`), keep the understanding deliberately limited to what the patient chose to share, state that scope in `scope_note` and `material_gaps`, and do not infer an unshared clinical chart.
- Preloaded assignment skills are already open below; do not reload them with `skill_view`. Use `skill_view` when additional guidance from the catalog (such as `patient-communication`) is needed, and use working notes to keep multi-step chart reading organized before calling `save_patient_understanding`.";

/// Whether an understanding run builds a first memory or revises an existing one.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum UnderstandingMode {
    /// First working understanding for a newly linked patient workspace.
    Initial,
    /// In-place revision after new or corrected source records arrived.
    Revision {
        /// Saved understanding revision being revised.
        prior_revision: u64,
        /// Workspace source version covered by `prior_revision`.
        prior_source_version: u64,
    },
}

/// Summary of one connected source attached to a patient workspace.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ConnectedSourceBrief {
    /// Stable source identifier.
    pub source_id: String,
    /// Human-readable source label.
    pub source_label: String,
    /// Unix epoch seconds of the latest recorded pull.
    pub pulled_at: i64,
}

/// Source coverage available when building or revising patient understanding.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "scope", rename_all = "snake_case")]
pub enum EvidenceScope {
    /// Only Duet patient-shared records are present; understanding stays bounded
    /// to what the patient chose to share.
    DuetPatientSharedOnly,
    /// Clinical chart or laboratory sources are connected.
    ClinicalChart {
        /// Connected sources currently recorded on the workspace.
        connected_sources: Vec<ConnectedSourceBrief>,
    },
}

impl EvidenceScope {
    /// Derives [`EvidenceScope`] from recorded source pulls and observed source IDs.
    pub fn classify<'a>(
        pulls: &[ConnectedSourceBrief],
        observed_source_ids: impl IntoIterator<Item = &'a str>,
    ) -> Self {
        let mut has_clinical = false;
        let mut has_duet = false;
        for pull in pulls {
            if is_duet_source_id(&pull.source_id) {
                has_duet = true;
            } else {
                has_clinical = true;
            }
        }
        for source_id in observed_source_ids {
            if is_duet_source_id(source_id) {
                has_duet = true;
            } else {
                has_clinical = true;
            }
        }
        if has_duet && !has_clinical {
            Self::DuetPatientSharedOnly
        } else {
            Self::ClinicalChart {
                connected_sources: pulls.to_vec(),
            }
        }
    }
}

fn is_duet_source_id(source_id: &str) -> bool {
    let lower = source_id.trim().to_ascii_lowercase();
    lower == "duet" || lower.starts_with("duet-") || lower.starts_with("duet_")
}

/// Typed initial context embedded in an [`Assignment`] for patient understanding.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UnderstandingContext {
    /// Target patient workspace identifier.
    pub workspace: String,
    /// Whether this run initializes or revises understanding.
    pub mode: UnderstandingMode,
    /// Workspace source observation version targeted by this run.
    pub target_source_version: u64,
    /// Source coverage available for this run.
    pub evidence_scope: EvidenceScope,
    /// Candidate record IDs added or updated since the prior saved understanding.
    #[serde(default)]
    pub changed_candidates: Vec<String>,
}

/// Whether a health system in the operating index is connected or chart-mentioned.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "standing", rename_all = "snake_case")]
pub enum HealthSystemStanding {
    /// Connected to Coda through `source_id`.
    Connected {
        /// Connected source identifier.
        source_id: String,
    },
    /// Mentioned in clinical notes or referrals without a direct connection.
    MentionedInRecords,
}

/// One connected or chart-mentioned health system tracked in [`OperatingIndex`].
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct KnownHealthSystem {
    /// Facility, clinic, or health system name.
    pub name: String,
    /// Connection standing for this facility.
    pub standing: HealthSystemStanding,
    /// Concise note on what care or records exist at this facility.
    pub note: String,
}

/// Record anchor stored inside [`OperatingIndex`] for fast cross-run lookup.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct IndexedAnchor {
    /// Exact `(candidate_id, revision)` record identity.
    pub reference: ExactRecordReference,
    /// Source system holding the record.
    pub source_id: String,
    /// Clinical topic or finding anchored by this record.
    pub topic: String,
}

/// Coda's strictly private cross-run operating index for a patient workspace.
///
/// Never treated as medication truth and never projected into `physician-wire`
/// or physician-facing HTTP responses.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct OperatingIndex {
    /// Concise longitudinal synthesis for Coda's own orientation across runs.
    pub working_overview: String,
    /// Connected sources and external health systems mentioned in the chart.
    #[serde(default)]
    pub known_health_systems: Vec<KnownHealthSystem>,
    /// Unresolved clinical questions or follow-ups to track across runs.
    #[serde(default)]
    pub open_threads: Vec<String>,
    /// Key record anchors indexed by exact `(candidate_id, revision)`.
    #[serde(default)]
    pub anchors: Vec<IndexedAnchor>,
}

/// One physician-facing summary statement paired with its supporting citations.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SummaryClaimInput {
    /// Physician-facing clinical prose for this statement.
    pub text: String,
    /// Exact chart record revisions supporting `text`.
    #[serde(default)]
    pub citations: Vec<ExactRecordReference>,
}

/// Validated arguments to [`SAVE_UNDERSTANDING_TOOL`].
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SaveUnderstandingArgs {
    /// Optional optimistic revision check (`0` for initial, or current revision).
    #[serde(default)]
    pub expected_revision: Option<u64>,
    /// Private cross-run operating index (never shown to physicians).
    pub operating_index: OperatingIndex,
    /// Ordered physician-facing clinical claims and their exact record citations.
    pub claims: Vec<SummaryClaimInput>,
    /// Explicit material gaps in the available records or follow-up evidence.
    pub material_gaps: Vec<String>,
    /// Optional scope note (required when Duet is the only source).
    #[serde(default)]
    pub scope_note: Option<String>,
}

/// Validation failure on [`SaveUnderstandingArgs`] or [`OperatingIndex`].
#[derive(Debug, thiserror::Error)]
pub enum UnderstandingInputError {
    /// Arguments JSON failed schema decoding.
    #[error("invalid save_patient_understanding arguments: {0}")]
    MalformedJson(#[source] serde_json::Error),
    /// Required text field was empty or whitespace.
    #[error("{field} must not be empty")]
    EmptyField {
        /// Name of the empty field.
        field: &'static str,
    },
    /// Record reference in `operating_index.anchors` or `claims.citations` was invalid.
    #[error("invalid {field}: {cause}")]
    InvalidReference {
        /// Field holding the invalid reference.
        field: &'static str,
        /// Underlying exact-reference validation cause.
        #[source]
        cause: crate::runtime::concerns::ConcernInputError,
    },
    /// No summary claims were supplied.
    #[error("claims must contain at least one clinical summary statement")]
    EmptyClaims,
    /// Material gaps list was empty.
    #[error("material_gaps must name at least one concrete gap or state what remains unconfirmed")]
    EmptyMaterialGaps,
}

fn require_trimmed(field: &'static str, raw: &str) -> Result<String, UnderstandingInputError> {
    let trimmed = raw.trim();
    if trimmed.is_empty() {
        return Err(UnderstandingInputError::EmptyField { field });
    }
    Ok(trimmed.to_owned())
}

impl SaveUnderstandingArgs {
    /// Parses and normalizes raw tool JSON into [`SaveUnderstandingArgs`].
    pub fn parse(arguments: &Value) -> Result<Self, UnderstandingInputError> {
        let parsed: Self = serde_json::from_value(arguments.clone())
            .map_err(UnderstandingInputError::MalformedJson)?;
        parsed.validate()
    }

    /// Validates and trims every nested field on [`SaveUnderstandingArgs`].
    pub fn validate(self) -> Result<Self, UnderstandingInputError> {
        let working_overview = require_trimmed(
            "operating_index.working_overview",
            &self.operating_index.working_overview,
        )?;
        let mut known_health_systems =
            Vec::with_capacity(self.operating_index.known_health_systems.len());
        for system in self.operating_index.known_health_systems {
            let standing = match system.standing {
                HealthSystemStanding::Connected { source_id } => HealthSystemStanding::Connected {
                    source_id: require_trimmed(
                        "known_health_systems.standing.source_id",
                        &source_id,
                    )?,
                },
                HealthSystemStanding::MentionedInRecords => {
                    HealthSystemStanding::MentionedInRecords
                }
            };
            known_health_systems.push(KnownHealthSystem {
                name: require_trimmed("known_health_systems.name", &system.name)?,
                standing,
                note: require_trimmed("known_health_systems.note", &system.note)?,
            });
        }
        let mut open_threads = Vec::with_capacity(self.operating_index.open_threads.len());
        for thread in self.operating_index.open_threads {
            open_threads.push(require_trimmed("operating_index.open_threads", &thread)?);
        }
        let mut anchors = Vec::with_capacity(self.operating_index.anchors.len());
        for anchor in self.operating_index.anchors {
            let reference = ExactRecordReference::new(
                &anchor.reference.candidate_id,
                &anchor.reference.revision,
            )
            .map_err(|cause| UnderstandingInputError::InvalidReference {
                field: "operating_index.anchors.reference",
                cause,
            })?;
            anchors.push(IndexedAnchor {
                reference,
                source_id: require_trimmed("operating_index.anchors.source_id", &anchor.source_id)?,
                topic: require_trimmed("operating_index.anchors.topic", &anchor.topic)?,
            });
        }

        if self.claims.is_empty() {
            return Err(UnderstandingInputError::EmptyClaims);
        }
        let mut claims = Vec::with_capacity(self.claims.len());
        for claim in self.claims {
            let text = require_trimmed("claims.text", &claim.text)?;
            let mut citations = Vec::with_capacity(claim.citations.len());
            for citation in claim.citations {
                let verified =
                    ExactRecordReference::new(&citation.candidate_id, &citation.revision).map_err(
                        |cause| UnderstandingInputError::InvalidReference {
                            field: "claims.citations",
                            cause,
                        },
                    )?;
                if !citations.contains(&verified) {
                    citations.push(verified);
                }
            }
            claims.push(SummaryClaimInput { text, citations });
        }

        if self.material_gaps.is_empty() {
            return Err(UnderstandingInputError::EmptyMaterialGaps);
        }
        let mut material_gaps = Vec::with_capacity(self.material_gaps.len());
        for gap in self.material_gaps {
            material_gaps.push(require_trimmed("material_gaps", &gap)?);
        }

        let scope_note = self
            .scope_note
            .map(|note| require_trimmed("scope_note", &note))
            .transpose()?;

        Ok(Self {
            expected_revision: self.expected_revision,
            operating_index: OperatingIndex {
                working_overview,
                known_health_systems,
                open_threads,
                anchors,
            },
            claims,
            material_gaps,
            scope_note,
        })
    }
}

/// Whether `name` is one of the patient understanding tools.
pub fn is_understanding_tool(name: &str) -> bool {
    matches!(name, READ_UNDERSTANDING_TOOL | SAVE_UNDERSTANDING_TOOL)
}

/// Whether `name` is a read-only understanding tool (`read_patient_understanding`).
pub fn read_only_understanding_tool(name: &str) -> bool {
    name == READ_UNDERSTANDING_TOOL
}

/// Typed tool descriptors for reading and saving patient understanding.
pub fn understanding_tools() -> Vec<Tool> {
    let exact_ref_schema = json!({
        "type": "object",
        "additionalProperties": false,
        "required": ["candidate_id", "revision"],
        "properties": {
            "candidate_id": {
                "type": "string",
                "description": "Opaque record identifier returned by search_patient_information or read_patient_sources."
            },
            "revision": {
                "type": "string",
                "description": "Exact immutable record revision read via read_patient_sources."
            }
        }
    });

    vec![
        Tool {
            name: READ_UNDERSTANDING_TOOL.to_owned(),
            description: "Read the patient's current saved understanding, including the private operating index, current physician-facing summary, material gaps, cited source revisions, and revision number. Returns an uninitialized status when no understanding has been saved yet.".to_owned(),
            parameters: json!({
                "type": "object",
                "additionalProperties": false,
                "properties": {}
            }),
        },
        Tool {
            name: SAVE_UNDERSTANDING_TOOL.to_owned(),
            description: "Save or revise the patient's durable understanding for this workspace. Persists both Coda's private operating index (never shown to physicians) and the physician-facing patient summary with verified exact record citations and explicit material gaps.".to_owned(),
            parameters: json!({
                "type": "object",
                "additionalProperties": false,
                "required": ["operating_index", "claims", "material_gaps"],
                "properties": {
                    "expected_revision": {
                        "type": "integer",
                        "minimum": 0,
                        "description": "Optional optimistic concurrency check: 0 when creating an initial understanding, or the current revision returned by read_patient_understanding when revising."
                    },
                    "operating_index": {
                        "type": "object",
                        "additionalProperties": false,
                        "required": ["working_overview"],
                        "properties": {
                            "working_overview": {
                                "type": "string",
                                "description": "Private longitudinal working overview for Coda across runs; never shown to physicians and never treated as medication truth."
                            },
                            "known_health_systems": {
                                "type": "array",
                                "description": "Connected clinical sources and outside health systems or clinics mentioned in the patient's records.",
                                "items": {
                                    "type": "object",
                                    "additionalProperties": false,
                                    "required": ["name", "standing", "note"],
                                    "properties": {
                                        "name": { "type": "string" },
                                        "standing": {
                                            "oneOf": [
                                                {
                                                    "type": "object",
                                                    "additionalProperties": false,
                                                    "required": ["standing", "source_id"],
                                                    "properties": {
                                                        "standing": { "type": "string", "enum": ["connected"] },
                                                        "source_id": { "type": "string" }
                                                    }
                                                },
                                                {
                                                    "type": "object",
                                                    "additionalProperties": false,
                                                    "required": ["standing"],
                                                    "properties": {
                                                        "standing": { "type": "string", "enum": ["mentioned_in_records"] }
                                                    }
                                                }
                                            ]
                                        },
                                        "note": { "type": "string" }
                                    }
                                }
                            },
                            "open_threads": {
                                "type": "array",
                                "items": { "type": "string" },
                                "description": "Unresolved clinical questions or items to monitor on future runs."
                            },
                            "anchors": {
                                "type": "array",
                                "description": "Exact source record anchors indexed for fast retrieval across runs.",
                                "items": {
                                    "type": "object",
                                    "additionalProperties": false,
                                    "required": ["reference", "source_id", "topic"],
                                    "properties": {
                                        "reference": exact_ref_schema,
                                        "source_id": { "type": "string" },
                                        "topic": { "type": "string" }
                                    }
                                }
                            }
                        }
                    },
                    "claims": {
                        "type": "array",
                        "minItems": 1,
                        "description": "Ordered physician-facing clinical summary statements, each paired with the exact record revisions read via read_patient_sources that support it.",
                        "items": {
                            "type": "object",
                            "additionalProperties": false,
                            "required": ["text", "citations"],
                            "properties": {
                                "text": {
                                    "type": "string",
                                    "description": "Concise physician-facing clinical prose for this summary statement."
                                },
                                "citations": {
                                    "type": "array",
                                    "items": exact_ref_schema,
                                    "description": "Exact chart record revisions supporting this statement."
                                }
                            }
                        }
                    },
                    "material_gaps": {
                        "type": "array",
                        "minItems": 1,
                        "items": { "type": "string" },
                        "description": "Explicit material gaps, missing outside records, unconfirmed medication details, or pending follow-up results."
                    },
                    "scope_note": {
                        "type": "string",
                        "description": "Optional scope note, such as stating that only patient-shared Duet information is currently available."
                    }
                }
            }),
        },
    ]
}

/// Complete tool set offered to a patient understanding background run.
pub fn tools() -> Vec<Tool> {
    let mut offered = crate::runtime::jobs::tools();
    offered.extend(understanding_tools());
    offered
}

/// Builds the full system instructions for a patient understanding run,
/// combining [`INSTRUCTIONS`], the discoverable skill catalog, and any
/// preloaded assignment skills.
pub fn instructions(preloaded_skills: &[PreloadedSkill], skills: &PhysicianSkills) -> String {
    let mut rendered = skills.instructions(
        INSTRUCTIONS,
        crate::physician_chat::Job::InitialUnderstanding,
    );
    if !preloaded_skills.is_empty() {
        let preloaded: Vec<Value> = preloaded_skills
            .iter()
            .map(|skill| {
                json!({
                    "name": skill.name,
                    "sha256": skill.sha256,
                    "content": skill.content,
                })
            })
            .collect();
        let encoded = serde_json::to_string(&json!({
            "kind": "preloaded_assignment_skills",
            "skills": preloaded,
        }))
        .expect("preloaded skills serialize as JSON");
        rendered.push_str(
            "\n\nPreloaded assignment skills (already open in this run; do not reload via skill_view):\n",
        );
        rendered.push_str(&encoded);
    }
    rendered
}

/// Builds the [`RequestRecipe`] for a patient understanding run on `background`.
pub fn recipe(
    background: &ModelBinding,
    assignment: &Assignment,
    skills: &PhysicianSkills,
) -> RequestRecipe {
    let mut built = RequestRecipe {
        model: background.model.clone(),
        context_window_tokens: background.context_window_tokens,
        max_input_tokens: background.max_input_tokens,
        options: json!({ "effort": background.effort }),
        instructions: instructions(&assignment.preloaded_skills, skills),
        tools: tools().into_iter().map(|tool| tool.function()).collect(),
    };
    harness::engine::offer_working_notes(&mut built.instructions, &mut built.tools);
    built
}

/// Constructs a typed [`Assignment`] with preloaded `patient-record-retrieval`
/// guidance and structured [`UnderstandingContext`] for an initial or revision run.
pub fn assignment(
    context: UnderstandingContext,
    skills: &PhysicianSkills,
) -> Result<Assignment, AssignmentError> {
    let workspace = context.workspace.trim().to_owned();
    if workspace.is_empty() {
        return Err(AssignmentError::EmptyAssignment);
    }
    let key = job_key(&workspace, context.target_source_version);
    let initial_context_json =
        serde_json::to_string(&context).expect("UnderstandingContext serializes cleanly as JSON");
    let prompt = match context.mode {
        UnderstandingMode::Initial => {
            let scope_guidance = match &context.evidence_scope {
                EvidenceScope::DuetPatientSharedOnly => {
                    "Duet patient-shared records are currently the only source; keep this first understanding deliberately limited to what the patient chose to share and state that scope explicitly."
                }
                EvidenceScope::ClinicalChart { .. } => {
                    "Search and read the available clinical source records, note any external health systems mentioned in the chart, state material gaps plainly, and cite exact record revisions."
                }
            };
            format!(
                "Build the initial working understanding for patient workspace {workspace} at source version {}.\n\
                 {scope_guidance}\n\
                 Call `save_patient_understanding` once you have read the supporting records.\n\
                 Initial context:\n{initial_context_json}",
                context.target_source_version
            )
        }
        UnderstandingMode::Revision {
            prior_revision,
            prior_source_version,
        } => format!(
            "Revise the working understanding for patient workspace {workspace} from revision {prior_revision} \
             (source version {prior_source_version}) to source version {}.\n\
             Call `read_patient_understanding` first, inspect the added or corrected source records via \
             `read_patient_sources`, and call `save_patient_understanding` to update the operating index \
             and physician-facing summary while preserving unrelated clinical information.\n\
             Initial context:\n{initial_context_json}",
            context.target_source_version
        ),
    };
    let mut created = Assignment::new(
        PracticeJobKind::InitialUnderstanding,
        key,
        prompt,
        &["patient-record-retrieval"],
        skills,
    )?;
    created.understanding = Some(context);
    Ok(created)
}

/// Deterministic practice-job identifier for `(workspace, source_version)`.
pub fn job_key(workspace: &str, source_version: u64) -> String {
    format!("patient-understanding:{workspace}:v{source_version}")
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;
    use crate::runtime::jobs::is_physician_only_tool;

    fn load_skills() -> PhysicianSkills {
        PhysicianSkills::load(&PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../skills"))
            .expect("load physician skills catalog")
    }

    fn background_binding() -> ModelBinding {
        ModelBinding {
            model: "background-understanding-model".into(),
            effort: "high".into(),
            context_window_tokens: 272_000,
            max_input_tokens: 250_000,
            adapter: "fixture".into(),
        }
    }

    #[test]
    fn recipe_uses_background_lane_preloads_retrieval_skill_and_excludes_physician_tools() {
        let skills = load_skills();
        let binding = background_binding();
        let context = UnderstandingContext {
            workspace: "pw-test-1".into(),
            mode: UnderstandingMode::Initial,
            target_source_version: 1,
            evidence_scope: EvidenceScope::ClinicalChart {
                connected_sources: vec![ConnectedSourceBrief {
                    source_id: "clinic-ehr".into(),
                    source_label: "Clinic EHR".into(),
                    pulled_at: 1_787_846_400,
                }],
            },
            changed_candidates: vec!["note-1".into()],
        };
        let created = assignment(context, &skills).expect("build initial assignment");
        assert_eq!(created.kind, PracticeJobKind::InitialUnderstanding);
        assert_eq!(created.preloaded_skills.len(), 1);
        assert_eq!(created.preloaded_skills[0].name, "patient-record-retrieval");

        let built = recipe(&binding, &created, &skills);
        assert_eq!(built.model, "background-understanding-model");
        assert_eq!(built.context_window_tokens, 272_000);
        assert_eq!(built.max_input_tokens, 250_000);
        assert_eq!(built.options, json!({ "effort": "high" }));
        assert!(built.instructions.contains("preloaded_assignment_skills"));
        assert!(built.instructions.contains("patient-record-retrieval"));
        assert!(built.instructions.contains("patient-communication"));

        let mut names = Vec::new();
        for tool in &built.tools {
            let name = tool["name"].as_str().expect("tool name");
            assert!(
                !is_physician_only_tool(name),
                "background understanding recipe must not offer physician-only tool {name}"
            );
            assert!(
                !tool["description"]
                    .as_str()
                    .expect("tool description")
                    .is_empty()
            );
            names.push(name);
        }
        assert!(names.contains(&READ_UNDERSTANDING_TOOL));
        assert!(names.contains(&SAVE_UNDERSTANDING_TOOL));
        assert!(names.contains(&crate::physician_sources::SEARCH_TOOL));
        assert!(names.contains(&crate::physician_sources::READ_TOOL));
        assert!(names.contains(&crate::runtime::refresh::REFRESH_TOOL));
        assert!(names.contains(&crate::physician_chat::SKILL_VIEW));
        let unique: std::collections::BTreeSet<&str> = names.iter().copied().collect();
        assert_eq!(
            names.len(),
            unique.len(),
            "duplicate tool names in {names:?}"
        );
    }

    #[test]
    fn duet_only_and_revision_assignments_encode_scope_and_mode() {
        let skills = load_skills();
        let duet_scope = EvidenceScope::classify(
            &[ConnectedSourceBrief {
                source_id: "duet-patient-share".into(),
                source_label: "Duet".into(),
                pulled_at: 1_787_846_400,
            }],
            ["duet-patient-share"],
        );
        assert_eq!(duet_scope, EvidenceScope::DuetPatientSharedOnly);

        let duet_assignment = assignment(
            UnderstandingContext {
                workspace: "pw-duet-1".into(),
                mode: UnderstandingMode::Initial,
                target_source_version: 1,
                evidence_scope: duet_scope,
                changed_candidates: Vec::new(),
            },
            &skills,
        )
        .expect("duet assignment");
        assert!(
            duet_assignment
                .prompt
                .contains("deliberately limited to what the patient chose to share")
        );

        let revision_assignment = assignment(
            UnderstandingContext {
                workspace: "pw-duet-1".into(),
                mode: UnderstandingMode::Revision {
                    prior_revision: 1,
                    prior_source_version: 1,
                },
                target_source_version: 2,
                evidence_scope: EvidenceScope::ClinicalChart {
                    connected_sources: vec![ConnectedSourceBrief {
                        source_id: "clinic-ehr".into(),
                        source_label: "Clinic EHR".into(),
                        pulled_at: 1_787_850_000,
                    }],
                },
                changed_candidates: vec!["lab-k-corrected".into()],
            },
            &skills,
        )
        .expect("revision assignment");
        assert!(revision_assignment.prompt.contains("revision 1"));
        assert!(revision_assignment.prompt.contains("lab-k-corrected"));
    }

    #[test]
    fn save_arguments_validation_rejects_blank_fields_and_unknown_properties() {
        let missing_gaps = json!({
            "operating_index": { "working_overview": "Overview" },
            "claims": [{ "text": "Summary line.", "citations": [] }],
            "material_gaps": []
        });
        assert!(matches!(
            SaveUnderstandingArgs::parse(&missing_gaps),
            Err(UnderstandingInputError::EmptyMaterialGaps)
        ));

        let unknown_prop = json!({
            "operating_index": { "working_overview": "Overview", "unexpected": true },
            "claims": [{ "text": "Summary line.", "citations": [] }],
            "material_gaps": ["Outside cardiology records pending."]
        });
        let malformed_err = SaveUnderstandingArgs::parse(&unknown_prop)
            .expect_err("unknown property must fail strict schema");
        assert!(matches!(
            malformed_err,
            UnderstandingInputError::MalformedJson(_)
        ));
        assert!(
            std::error::Error::source(&malformed_err).is_some(),
            "MalformedJson must preserve serde_json::Error as #[source]"
        );

        let control_char_anchor = json!({
            "operating_index": {
                "working_overview": "Overview",
                "anchors": [{
                    "reference": { "candidate_id": "Condition/bad\u{0007}id", "revision": "rev-1" },
                    "source_id": "clinic-ehr",
                    "topic": "Topic"
                }]
            },
            "claims": [{ "text": "Summary line.", "citations": [] }],
            "material_gaps": ["Outside cardiology records pending."]
        });
        let ref_err = SaveUnderstandingArgs::parse(&control_char_anchor)
            .expect_err("control characters in anchor reference must fail");
        assert!(matches!(
            ref_err,
            UnderstandingInputError::InvalidReference {
                field: "operating_index.anchors.reference",
                ..
            }
        ));
        assert!(
            std::error::Error::source(&ref_err).is_some(),
            "InvalidReference must preserve ConcernInputError as #[source]"
        );
    }
}
