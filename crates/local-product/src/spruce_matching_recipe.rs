//! Unmatched Spruce message classification and patient-match suggestion recipe.
//!
//! Runs on [`crate::ModelRoles::background`] over a saved [`MessageStanding::Deciding`]
//! Spruce message with authorized non-chart matching context: the saved message
//! envelope and body, conversation outside participants, channel contact/EHR
//! tie state, confirmed sender bindings, and read-only `patient_lookup` over the
//! practice's demographic and contact roster. Clinical chart tools are absent
//! from the recipe and refused at execution time, and [`crate::runtime::spruce_matching`]
//! retains sole authority over automatic conversation linking and physician confirmation.

use std::collections::BTreeSet;

use async_trait::async_trait;
use harness::engine::recipe::{Finish, Provider, RequestBinding, RequestRecipe};
use harness::engine::{self, Event, EventKind, Invocation, Item, Product, Tool, ToolOutcome, Turn};
use physician_auth::{AuthStore, StoreUnavailable};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DbErr, EntityTrait, QueryFilter, QueryOrder, QuerySelect, Set,
    TransactionTrait,
};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use uuid::Uuid;

use crate::ModelRoles;
use crate::physician_chat::{PhysicianSkills, SKILL_VIEW, harness_work_tools};
use crate::physician_sources::{READ_TOOL, SEARCH_TOOL};
use crate::runtime::dispatch::ReviewReceiver;
use crate::runtime::spruce_matching::{
    ChannelTieEvaluation, MatchAssessment, MatchingDecision, MatchingRosterPatient,
    MessageRoutingOutcome, MessageStanding, MessageUrgencyChoice, NonPatientCategory,
    OutsideParticipant, PendingSpruceReviewTrigger, SenderAddress, SpruceAttachmentSummary,
    SpruceMatchingError, allowed_workspaces_in_practice, dispatch_single_trigger, entities,
    execute_spruce_working_note, is_spruce_working_note_tool,
    spruce_conversion_error as conversion_error, spruce_db_error as error,
    spruce_working_note_references,
};
use crate::runtime::{BindingRef, SkillPackageId, Store};

/// Immutable recipe identifier recorded with every unmatched message classification run.
pub const RECIPE_VERSION: &str = "spruce-message-matching/v1";

/// Immutable resident guidance version recorded with every unmatched message classification run.
pub const GUIDANCE_VERSION: &str = "spruce-matching-guidance/v1";

/// Read-only demographic and contact roster lookup tool offered to the matching recipe.
pub const PATIENT_LOOKUP_TOOL: &str = "patient_lookup";

/// Structured decision barrier submitting Coda's classification and match assessment.
pub const SUBMIT_CLASSIFICATION_TOOL: &str = "submit_message_classification";

/// Maximum roster entries returned in a single `patient_lookup` call.
pub const MAX_PATIENT_LOOKUP_RESULTS: usize = 50;

/// Resident instructions for classifying unmatched Spruce messages and suggesting
/// identity-grounded patient matches without accessing any clinical chart.
pub const INSTRUCTIONS: &str = "\
You are Coda classifying an incoming unmatched practice message from Spruce and assessing \
whether permitted identity evidence matches a registered patient in this practice. No physician \
is signed in to this background run, and no patient clinical chart is open or accessible.

Follow one disciplined process:

1. Classify patient mail versus non-patient mail:
   - Classify as `patient_mail` whenever the message is from a patient, from a parent, spouse, \
     family member, or caregiver writing about a patient, or from a pharmacy, laboratory, or \
     outside clinician writing about a specific patient's care — including brief, informal, or \
     ambiguous messages where a caregiver writes from their own phone or omits a full name. \
     When uncertain whether an inquiry concerns a patient, keep it as `patient_mail` so an \
     authorized physician sees and routes it rather than filtering a patient message.
   - Classify as `not_patient_mail` (`vendor_or_spam`, `administrative_notice`, or \
     `non_patient_outreach`) only when the message is clearly not from or about a patient, such \
     as commercial vendor outreach, fax marketing blasts, general payer bulletins, directory \
     verification notices, or general administrative announcements.

2. Flag clinically urgent unmatched patient mail without guessing a patient link:
   - When a `patient_mail` message describes potentially acute or time-sensitive clinical \
     symptoms or safety concerns (for example chest pain, shortness of breath, focal neurologic \
     symptoms, severe bleeding, allergic reaction, acute decompensation, or self-harm risk), \
     mark `urgency` as `urgent` with a concise plain-language clinical `reason` so it leads \
     Today as `Now` while remaining unlinked until an authorized physician confirms the patient. \
     Otherwise mark `urgency` as `ordinary`.
   - Never guess a patient match merely because a message is urgent, and never attempt to open \
     any patient clinical chart, note, laboratory report, medication list, or clinical summary.

3. Assess patient identity using only permitted demographic and contact evidence:
   - Call `patient_lookup` to search the practice's registered patient demographic and contact \
     roster by name, birth date, medical record number, sender phone/email address, or Spruce \
     contact/EHR identifier (or omit `query` to inspect the practice roster when sender details \
     are sparse).
   - Rely strictly on permitted identity evidence: confirmed sender bindings, Spruce contact and \
     EHR patient links, patient name, birth date, medical record number, and explicit identity \
     or family/caregiver relationship statements in the message itself.
   - Never produce numeric confidence scores, percentages, or probability ratings.
   - Select the exact match assessment variant:
     * `suggested_patient`: permitted identity evidence (including a confirmed sender address \
       or unique channel contact/EHR tie when the message contains no contrary third-party \
       clues) points cleanly to one registered patient workspace and nothing points to another \
       person.
     * `ambiguous_candidates`: two or more registered patients could plausibly match (for example \
       siblings sharing a parent's phone number, a shared household contact, or matching names \
       without a distinguishing birth date); name every plausible workspace in \
       `candidate_workspaces` and explain the ambiguity in `evidence_summary`.
     * `conflicting_evidence`: sender or channel metadata points to one patient while the message \
       names or describes a different registered patient (for example a spouse or parent texting \
       from their own confirmed phone about another registered patient); name the conflicting \
       workspaces in `candidate_workspaces` and state the contradiction plainly in \
       `evidence_summary`.
     * `no_match_found`: no patient on the practice roster matches the person the message is \
       about — including when a message arrives from a confirmed sender phone/email or linked \
       channel contact (such as a registered parent, spouse, or family member) but asks about an \
       unregistered child, relative, or third party who is not on the practice roster.

4. Consult approved skill guidance when helpful and submit once:
   - Review the approved skill catalog metadata below and open a skill via `skill_view` only when \
     its guidance materially helps.
   - Call `submit_message_classification` with your structured classification, urgency, \
     identity assessment, and concise physician-readable rationale before finishing with a brief \
     plain-language summary.";

/// Lowercase hex SHA-256 of [`INSTRUCTIONS`].
#[must_use]
pub fn instructions_sha256() -> String {
    crate::sha256_hex(INSTRUCTIONS.as_bytes())
}

/// Authorized non-chart starting context supplied to an unmatched message matching run.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MatchingStartingContext {
    /// Owning practice identifier.
    pub practice: String,
    /// Saved Spruce message identifier.
    pub message_id: String,
    /// Spruce conversation identifier.
    pub conversation_id: String,
    /// Content version digest of the saved message.
    pub content_version: String,
    /// Sender endpoint address on the message, when present.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sender_address: Option<SenderAddress>,
    /// Author display name on the message, when present.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub author_name: Option<String>,
    /// Readable message body text.
    pub body_text: String,
    /// Attachment summaries on the message (metadata only, no clinical chart records).
    pub attachments: Vec<SpruceAttachmentSummary>,
    /// Spruce creation timestamp when supplied.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_at_spruce: Option<String>,
    /// Outside participants recorded on the conversation.
    pub outside_participants: Vec<OutsideParticipant>,
    /// Workspaces in this practice for which `sender_address` is already confirmed.
    pub confirmed_sender_workspaces: Vec<String>,
    /// Pre-evaluated channel contact -> Spruce patient -> EHR -> Coda workspace tie status.
    pub channel_tie: ChannelTieEvaluation,
}

impl MatchingStartingContext {
    /// Serializes this authorized matching context for the run's initial user message.
    #[must_use]
    pub fn prompt_text(&self) -> String {
        let payload = json!({
            "kind": "spruce_unmatched_message_context",
            "context": self,
        });
        serde_json::to_string_pretty(&payload).expect("MatchingStartingContext serializes to JSON")
    }
}

/// One item eagerly loaded at the start of a matching run.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum MatchingInitialLoad {
    /// Resident classification and matching instructions embedded in the frozen recipe.
    ResidentGuidance {
        /// Guidance version string ([`GUIDANCE_VERSION`]).
        version: String,
        /// Lowercase SHA-256 hex digest of [`INSTRUCTIONS`].
        sha256: String,
    },
    /// Approved skill catalog metadata embedded in instructions (`skill_view` bodies remain on demand).
    SkillCatalogMetadata {
        /// Saved skill package digest in Postgres.
        skills_package_id: SkillPackageId,
    },
    /// Saved Spruce message envelope and readable text supplied in the initial message.
    SavedMessageEnvelope {
        /// Spruce message identifier.
        message_id: String,
        /// Spruce conversation identifier.
        conversation_id: String,
        /// Saved content version digest.
        content_version: String,
    },
    /// Conversation outside participants and channel tie metadata supplied in the initial message.
    ConversationParticipants {
        /// Count of outside participants on the conversation.
        outside_participant_count: usize,
        /// Summary tag of channel tie (`unique_patient`, `contradictory`, or `untied`).
        channel_tie_kind: String,
    },
}

/// Whether a tool invocation during a matching run returned data or was refused.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "outcome", rename_all = "snake_case")]
pub enum ToolDiscoveryDisposition {
    /// Tool executed within the matching recipe's authorized non-chart boundary.
    Returned,
    /// Tool was refused by the host or schema validation.
    Refused {
        /// Refusal explanation recorded in the run trace.
        reason: String,
    },
}

/// One later tool discovery or invocation recorded during a matching run.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RecordedToolDiscovery {
    /// Provider exchange ordinal in the harness run.
    pub ordinal: u64,
    /// Provider call identifier.
    pub call_id: String,
    /// Tool name requested by the model.
    pub tool_name: String,
    /// Whether the call returned or was refused.
    pub disposition: ToolDiscoveryDisposition,
    /// Unix second when recorded.
    pub recorded_at: i64,
}

/// Exact model settings, recipe & guidance versions, and frozen storage references
/// recorded at the start of a matching run.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct MatchingRecipeProvenance {
    /// Recipe version ([`RECIPE_VERSION`]).
    pub recipe_version: String,
    /// Guidance version ([`GUIDANCE_VERSION`]).
    pub guidance_version: String,
    /// Lowercase SHA-256 digest of [`INSTRUCTIONS`].
    pub guidance_sha256: String,
    /// Bound background-lane model name (`roles.background.model`).
    pub model: String,
    /// Bound reasoning effort (`roles.background.effort`).
    pub effort: String,
    /// Bound working context window (`roles.background.context_window_tokens`).
    pub context_window_tokens: u64,
    /// Bound input token ceiling (`roles.background.max_input_tokens`).
    pub max_input_tokens: u64,
    /// Adapter label (`roles.background.adapter`).
    pub adapter: String,
    /// Frozen recipe reference in `physician_runtime_frozen_recipes`.
    pub binding: BindingRef,
    /// Frozen skill catalog reference in `physician_runtime_skill_packages`.
    pub skills: SkillPackageId,
}

/// Lifecycle state of a persisted [`SpruceMatchingRunRecord`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MatchingRunStatus {
    /// Run is currently executing.
    Running,
    /// Run submitted a valid [`MatchingDecision`] and persisted classification/routing atomically.
    Completed,
    /// Run hit a mechanical limit before committing; message stays `Deciding` for the 15-minute fallback.
    Held,
    /// Run failed before committing; message stays `Deciding` for retry or the 15-minute fallback.
    Failed,
}

impl MatchingRunStatus {
    const fn as_str(self) -> &'static str {
        match self {
            Self::Running => "running",
            Self::Completed => "completed",
            Self::Held => "held",
            Self::Failed => "failed",
        }
    }

    fn parse(raw: &str) -> Result<Self, DbErr> {
        match raw {
            "running" => Ok(Self::Running),
            "completed" => Ok(Self::Completed),
            "held" => Ok(Self::Held),
            "failed" => Ok(Self::Failed),
            other => Err(error(&format!(
                "unknown spruce matching run status `{other}`"
            ))),
        }
    }
}

/// Staged decision recorded by `submit_message_classification` prior to `finish()`.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct StagedMatchingDecision {
    /// Tool call identifier that recorded this decision.
    pub call_id: String,
    /// Validated classification and match assessment.
    pub decision: MatchingDecision,
    /// Unix second when staged.
    pub staged_at: i64,
}

/// Durable inspection record for one unmatched Spruce message classification run.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SpruceMatchingRunRecord {
    /// Practice identifier.
    pub practice: String,
    /// Unique run identifier.
    pub run_id: String,
    /// Classified Spruce message identifier.
    pub message_id: String,
    /// Spruce conversation identifier.
    pub conversation_id: String,
    /// Message content version classified by this run.
    pub content_version: String,
    /// Harness attempt identifier.
    pub attempt_id: String,
    /// Run status (`Running`, `Completed`, `Held`, or `Failed`).
    pub status: MatchingRunStatus,
    /// Exact model settings, recipe/guidance versions, and frozen refs.
    pub provenance: MatchingRecipeProvenance,
    /// Authorized non-chart starting context supplied to the run.
    pub starting_context: MatchingStartingContext,
    /// Items eagerly loaded at run start.
    pub initial_loads: Vec<MatchingInitialLoad>,
    /// Ordered tool discoveries and refusals recorded during execution.
    pub tool_discoveries: Vec<RecordedToolDiscovery>,
    /// Complete ordered harness event trace.
    pub events: Vec<Event>,
    /// Staged classification decision once `submit_message_classification` succeeded.
    pub staged_decision: Option<StagedMatchingDecision>,
    /// Final plain-language assistant summary when completed or held.
    pub final_summary: Option<String>,
    /// Unix second when started.
    pub started_at: i64,
    /// Unix second when completed, held, or failed.
    pub completed_at: Option<i64>,
}

/// Result returned when [`run_spruce_message_matching`] finishes cleanly.
#[derive(Clone, Debug)]
pub struct CompletedMatchingRun {
    /// Persisted run record including provenance, initial loads, tool discoveries, and events.
    pub run: SpruceMatchingRunRecord,
    /// Resulting message routing outcome (`Linked`, `Waiting`, `Filtered`, or `NotAPatient`).
    pub routing_outcome: MessageRoutingOutcome,
}

/// Errors returned when starting or running unmatched Spruce message classification.
#[derive(Debug, thiserror::Error)]
pub enum SpruceMatchingRecipeError {
    /// Underlying Spruce matching state, authority, or review-handoff failure.
    #[error(transparent)]
    Matching(#[from] SpruceMatchingError),
    /// Target message is not in [`MessageStanding::Deciding`].
    #[error("spruce message `{message_id}` is in standing `{standing}`, expected `deciding`")]
    MessageNotDeciding {
        /// Spruce message identifier.
        message_id: String,
        /// Current standing kind string.
        standing: String,
    },
    /// Harness run stopped on a limit backstop before submitting a classification.
    #[error(
        "spruce matching run `{run_id}` held on limit backstop before classification committed"
    )]
    HeldWithoutClassification {
        /// Run identifier.
        run_id: String,
    },
    /// Harness execution failed (provider or host error).
    #[error("spruce matching harness execution failed for run `{run_id}`: {source}")]
    Harness {
        /// Run identifier.
        run_id: String,
        /// Underlying harness failure.
        #[source]
        source: engine::Failure,
    },
    /// Authority store failure.
    #[error("physician authority store unavailable: {0}")]
    AuthorityUnavailable(#[from] StoreUnavailable),
    /// Database error from SeaORM.
    #[error("spruce matching run database failure: {0}")]
    Database(#[from] DbErr),
}

/// Authorized tool set for unmatched Spruce message classification:
/// shared non-chart work tools (`skill_view`, `read_tool_result`, conversation history),
/// `patient_lookup` (demographic/contact roster only), and `submit_message_classification`.
/// Contains **zero** patient clinical chart tools.
#[must_use]
pub fn tools() -> Vec<Tool> {
    let mut offered = harness_work_tools();
    offered.push(patient_lookup_tool());
    offered.push(submit_classification_tool());
    offered
}

/// Builds the immutable background-lane [`RequestRecipe`] for unmatched message matching.
#[must_use]
pub fn recipe(roles: &ModelRoles, skills: &PhysicianSkills) -> RequestRecipe {
    let binding = &roles.background;
    let mut built = RequestRecipe {
        model: binding.model.clone(),
        context_window_tokens: binding.context_window_tokens,
        max_input_tokens: binding.max_input_tokens,
        options: json!({ "effort": binding.effort }),
        instructions: skills.instructions(INSTRUCTIONS, crate::physician_chat::Job::SpruceMatching),
        tools: tools().into_iter().map(|tool| tool.function()).collect(),
    };
    engine::offer_working_notes(&mut built.instructions, &mut built.tools);
    built
}

/// Declares the read-only `patient_lookup` tool over the practice's demographic and contact roster.
#[must_use]
pub fn patient_lookup_tool() -> Tool {
    Tool {
        name: PATIENT_LOOKUP_TOOL.into(),
        description: "Search or list registered patients in this practice using permitted demographic and contact identity fields only (display name, birth date, medical record number, confirmed sender address, Spruce contact ID, or EHR patient ID). Never accesses clinical chart notes, labs, medications, or summaries.".into(),
        parameters: json!({
            "type": "object",
            "properties": {
                "query": {
                    "type": ["string", "null"],
                    "description": "Optional case-insensitive substring filter matched against patient name, birth date, MRN, sender phone/email, Spruce contact ID, EHR ID, or workspace ID. Omit or pass null to list the practice's registered patient identity roster."
                }
            },
            "additionalProperties": false
        }),
    }
}

/// Declares the `submit_message_classification` write-barrier tool.
#[must_use]
pub fn submit_classification_tool() -> Tool {
    Tool {
        name: SUBMIT_CLASSIFICATION_TOOL.into(),
        description: "Record Coda's classification (`patient_mail` vs `not_patient_mail`), unmatched urgency flag (`ordinary` vs `urgent`), and identity-based patient match assessment (`suggested_patient`, `ambiguous_candidates`, `conflicting_evidence`, or `no_match_found`) with zero confidence scores.".into(),
        parameters: json!({
            "type": "object",
            "properties": {
                "classification": {
                    "type": "string",
                    "enum": ["patient_mail", "not_patient_mail"],
                    "description": "Whether the message is from or about a patient (`patient_mail`) or non-patient mail (`not_patient_mail`)."
                },
                "non_patient_category": {
                    "type": ["string", "null"],
                    "enum": ["vendor_or_spam", "administrative_notice", "non_patient_outreach", null],
                    "description": "Required when `classification` is `not_patient_mail`; null when `patient_mail`."
                },
                "urgency": {
                    "type": ["string", "null"],
                    "enum": ["ordinary", "urgent", null],
                    "description": "Required when `classification` is `patient_mail`: `urgent` when acute clinical symptoms need prompt attention on Today as `Now`, otherwise `ordinary`."
                },
                "urgent_reason": {
                    "type": ["string", "null"],
                    "description": "Plain-language clinical reason required when `urgency` is `urgent`."
                },
                "match_outcome": {
                    "type": ["string", "null"],
                    "enum": [
                        "suggested_patient",
                        "ambiguous_candidates",
                        "conflicting_evidence",
                        "no_match_found",
                        null
                    ],
                    "description": "Identity assessment required when `classification` is `patient_mail`."
                },
                "suggested_workspace": {
                    "type": ["string", "null"],
                    "description": "Single registered patient workspace ID when `match_outcome` is `suggested_patient`."
                },
                "candidate_workspaces": {
                    "type": ["array", "null"],
                    "items": { "type": "string" },
                    "description": "Registered patient workspace IDs involved when `match_outcome` is `ambiguous_candidates` (>= 2) or `conflicting_evidence`."
                },
                "evidence_summary": {
                    "type": ["string", "null"],
                    "description": "Permitted identity basis or conflict/ambiguity explanation (no confidence scores). Required when `classification` is `patient_mail`."
                },
                "rationale": {
                    "type": "string",
                    "description": "Concise physician-facing explanation of the classification and match assessment."
                }
            },
            "required": ["classification", "rationale"],
            "additionalProperties": false
        }),
    }
}

#[derive(Debug, Default, Deserialize)]
#[serde(deny_unknown_fields)]
struct PatientLookupArgs {
    #[serde(default)]
    query: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct SubmitClassificationArgs {
    classification: SubmittedKind,
    #[serde(default)]
    non_patient_category: Option<NonPatientCategory>,
    #[serde(default)]
    urgency: Option<SubmittedUrgency>,
    #[serde(default)]
    urgent_reason: Option<String>,
    #[serde(default)]
    match_outcome: Option<SubmittedMatchOutcome>,
    #[serde(default)]
    suggested_workspace: Option<String>,
    #[serde(default)]
    candidate_workspaces: Option<Vec<String>>,
    #[serde(default)]
    evidence_summary: Option<String>,
    rationale: String,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "snake_case")]
enum SubmittedKind {
    PatientMail,
    NotPatientMail,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "snake_case")]
enum SubmittedUrgency {
    Ordinary,
    Urgent,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "snake_case")]
enum SubmittedMatchOutcome {
    SuggestedPatient,
    AmbiguousCandidates,
    ConflictingEvidence,
    NoMatchFound,
}

fn is_forbidden_chart_or_physician_tool(name: &str) -> bool {
    matches!(
        name,
        SEARCH_TOOL
            | READ_TOOL
            | "refresh_patient_sources"
            | "list_patient_memory"
            | "read_patient_memory"
            | "search_patient_memory"
            | "add_patient_memory_note"
            | "edit_patient_memory"
            | "retrieve_patient_work"
            | "read_patient_understanding"
            | "save_patient_understanding"
            | "patient_concern"
            | "read_patient_reviews"
            | "complete_patient_review"
            | "search_clinical_library"
            | "read_clinical_library"
            | "save_visit_briefing"
            | "select_visits_for_preparation"
            | "propose_patient_reply"
    )
}

fn matches_roster_query(entry: &MatchingRosterPatient, query_lower: &str) -> bool {
    if entry.workspace.to_ascii_lowercase().contains(query_lower) {
        return true;
    }
    if let Some(demo) = &entry.demographics
        && (demo.display_name.to_ascii_lowercase().contains(query_lower)
            || demo
                .source_patient_id
                .to_ascii_lowercase()
                .contains(query_lower)
            || demo
                .birth_date
                .as_ref()
                .is_some_and(|d| d.to_ascii_lowercase().contains(query_lower))
            || demo.medical_record_numbers.iter().any(|m| {
                m.value.to_ascii_lowercase().contains(query_lower)
                    || m.system.to_ascii_lowercase().contains(query_lower)
            }))
    {
        return true;
    }
    if entry
        .confirmed_senders
        .iter()
        .any(|s| s.as_str().to_ascii_lowercase().contains(query_lower))
    {
        return true;
    }
    if entry
        .linked_spruce_contacts
        .iter()
        .any(|c| c.to_ascii_lowercase().contains(query_lower))
    {
        return true;
    }
    entry.ehr_links.iter().any(|e| {
        e.source_id.to_ascii_lowercase().contains(query_lower)
            || e.source_patient_id
                .to_ascii_lowercase()
                .contains(query_lower)
    })
}

fn validate_and_build_decision(
    args: SubmitClassificationArgs,
    registered_workspaces: &BTreeSet<String>,
) -> Result<MatchingDecision, String> {
    let rationale = args.rationale.trim().to_owned();
    if rationale.is_empty() {
        return Err("`rationale` must not be empty".to_owned());
    }

    match args.classification {
        SubmittedKind::NotPatientMail => {
            let Some(category) = args.non_patient_category else {
                return Err(
                    "`non_patient_category` is required when `classification` is `not_patient_mail`"
                        .to_owned(),
                );
            };
            Ok(MatchingDecision::NotPatientMail {
                category,
                rationale,
            })
        }
        SubmittedKind::PatientMail => {
            let urgency = match args.urgency.unwrap_or(SubmittedUrgency::Ordinary) {
                SubmittedUrgency::Ordinary => MessageUrgencyChoice::Ordinary,
                SubmittedUrgency::Urgent => {
                    let reason = args
                        .urgent_reason
                        .as_deref()
                        .map(str::trim)
                        .filter(|s| !s.is_empty())
                        .ok_or_else(|| {
                            "`urgent_reason` is required when `urgency` is `urgent`".to_owned()
                        })?
                        .to_owned();
                    MessageUrgencyChoice::Urgent { reason }
                }
            };

            let outcome = args.match_outcome.ok_or_else(|| {
                "`match_outcome` is required when `classification` is `patient_mail`".to_owned()
            })?;
            let evidence_summary = args
                .evidence_summary
                .as_deref()
                .map(str::trim)
                .filter(|s| !s.is_empty())
                .ok_or_else(|| {
                    "`evidence_summary` is required when `classification` is `patient_mail`"
                        .to_owned()
                })?
                .to_owned();

            let assessment = match outcome {
                SubmittedMatchOutcome::SuggestedPatient => {
                    let ws = args
                        .suggested_workspace
                        .as_deref()
                        .map(str::trim)
                        .filter(|s| !s.is_empty())
                        .ok_or_else(|| {
                            "`suggested_workspace` is required for `suggested_patient`".to_owned()
                        })?;
                    if !registered_workspaces.contains(ws) {
                        return Err(format!(
                            "workspace `{ws}` is not a registered patient in this practice"
                        ));
                    }
                    MatchAssessment::SuggestedPatient {
                        workspace: ws.to_owned(),
                        evidence_summary,
                    }
                }
                SubmittedMatchOutcome::AmbiguousCandidates => {
                    let raw = args.candidate_workspaces.unwrap_or_default();
                    let mut deduped = Vec::new();
                    for item in raw {
                        let trimmed = item.trim();
                        if trimmed.is_empty() {
                            continue;
                        }
                        if !registered_workspaces.contains(trimmed) {
                            return Err(format!(
                                "candidate workspace `{trimmed}` is not a registered patient in this practice"
                            ));
                        }
                        if !deduped.iter().any(|existing| existing == trimmed) {
                            deduped.push(trimmed.to_owned());
                        }
                    }
                    if deduped.len() < 2 {
                        return Err(
                            "`ambiguous_candidates` requires at least 2 distinct registered `candidate_workspaces`"
                                .to_owned(),
                        );
                    }
                    MatchAssessment::AmbiguousCandidates {
                        candidate_workspaces: deduped,
                        evidence_summary,
                    }
                }
                SubmittedMatchOutcome::ConflictingEvidence => {
                    let raw = args.candidate_workspaces.unwrap_or_default();
                    let mut deduped = Vec::new();
                    for item in raw {
                        let trimmed = item.trim();
                        if trimmed.is_empty() {
                            continue;
                        }
                        if !registered_workspaces.contains(trimmed) {
                            return Err(format!(
                                "candidate workspace `{trimmed}` is not a registered patient in this practice"
                            ));
                        }
                        if !deduped.iter().any(|existing| existing == trimmed) {
                            deduped.push(trimmed.to_owned());
                        }
                    }
                    if deduped.is_empty() {
                        return Err(
                            "`conflicting_evidence` requires at least 1 registered workspace in `candidate_workspaces`"
                                .to_owned(),
                        );
                    }
                    MatchAssessment::ConflictingEvidence {
                        candidate_workspaces: deduped,
                        evidence_summary,
                    }
                }
                SubmittedMatchOutcome::NoMatchFound => {
                    MatchAssessment::NoMatchFound { evidence_summary }
                }
            };

            Ok(MatchingDecision::PatientMail {
                urgency,
                assessment,
                rationale,
            })
        }
    }
}

fn decode_run_row(
    row: entities::spruce_matching_run::Model,
) -> Result<SpruceMatchingRunRecord, DbErr> {
    let status = MatchingRunStatus::parse(&row.status)?;
    let provenance: MatchingRecipeProvenance = serde_json::from_value(row.provenance).map_err(
        conversion_error("spruce_matching_run.provenance", "MatchingRecipeProvenance"),
    )?;
    let starting_context: MatchingStartingContext = serde_json::from_value(row.starting_context)
        .map_err(conversion_error(
            "spruce_matching_run.starting_context",
            "MatchingStartingContext",
        ))?;
    let initial_loads: Vec<MatchingInitialLoad> = serde_json::from_value(row.initial_loads)
        .map_err(conversion_error(
            "spruce_matching_run.initial_loads",
            "Vec<MatchingInitialLoad>",
        ))?;
    let tool_discoveries: Vec<RecordedToolDiscovery> = serde_json::from_value(row.tool_discoveries)
        .map_err(conversion_error(
            "spruce_matching_run.tool_discoveries",
            "Vec<RecordedToolDiscovery>",
        ))?;
    let events: Vec<Event> = serde_json::from_value(row.events)
        .map_err(conversion_error("spruce_matching_run.events", "Vec<Event>"))?;
    let staged_decision: Option<StagedMatchingDecision> = row
        .staged_decision
        .map(serde_json::from_value)
        .transpose()
        .map_err(conversion_error(
            "spruce_matching_run.staged_decision",
            "StagedMatchingDecision",
        ))?;

    Ok(SpruceMatchingRunRecord {
        practice: row.practice,
        run_id: row.run_id,
        message_id: row.message_id,
        conversation_id: row.conversation_id,
        content_version: row.content_version,
        attempt_id: row.attempt_id,
        status,
        provenance,
        starting_context,
        initial_loads,
        tool_discoveries,
        events,
        staged_decision,
        final_summary: row.final_summary,
        started_at: row.started_at,
        completed_at: row.completed_at,
    })
}

impl Store {
    /// Reads one persisted unmatched Spruce message classification run.
    pub async fn spruce_matching_run(
        &self,
        practice: &str,
        run_id: &str,
    ) -> Result<Option<SpruceMatchingRunRecord>, SpruceMatchingError> {
        let practice = practice.trim();
        let run_id = run_id.trim();
        if practice.is_empty() || run_id.is_empty() {
            return Err(SpruceMatchingError::InvalidIdentifier { field: "run_id" });
        }
        entities::spruce_matching_run::Entity::find_by_id((practice.to_owned(), run_id.to_owned()))
            .one(self.spruce_db())
            .await?
            .map(decode_run_row)
            .transpose()
            .map_err(Into::into)
    }

    /// Lists all persisted classification runs for `(practice, message_id)` in chronological order.
    pub async fn spruce_matching_runs_for_message(
        &self,
        practice: &str,
        message_id: &str,
    ) -> Result<Vec<SpruceMatchingRunRecord>, SpruceMatchingError> {
        let practice = practice.trim();
        let message_id = message_id.trim();
        if practice.is_empty() || message_id.is_empty() {
            return Err(SpruceMatchingError::InvalidIdentifier {
                field: "message_id",
            });
        }
        let rows = entities::spruce_matching_run::Entity::find()
            .filter(entities::spruce_matching_run::Column::Practice.eq(practice))
            .filter(entities::spruce_matching_run::Column::MessageId.eq(message_id))
            .order_by_asc(entities::spruce_matching_run::Column::StartedAt)
            .order_by_asc(entities::spruce_matching_run::Column::RunId)
            .all(self.spruce_db())
            .await?;
        rows.into_iter()
            .map(decode_run_row)
            .collect::<Result<Vec<_>, _>>()
            .map_err(Into::into)
    }

    /// Appends one harness execution event to `(practice, run_id)` for `attempt_id`,
    /// enforcing attempt ownership, currency fencing, contiguous sequence numbers,
    /// and idempotent `(attempt_id, sequence)` replay without duplicating tool discoveries.
    pub async fn persist_spruce_matching_run_event(
        &self,
        practice: &str,
        run_id: &str,
        attempt_id: &str,
        event: Event,
    ) -> Result<(), engine::ProductError> {
        if event.attempt_id != attempt_id {
            return Err(error("spruce matching event belongs to another attempt").into());
        }
        if event.sequence == 0 {
            return Err(error("invalid spruce matching event sequence").into());
        }

        let clock = self.spruce_now().await?;
        let tx = self.spruce_db().begin().await?;
        let row = entities::spruce_matching_run::Entity::find_by_id((
            practice.to_owned(),
            run_id.to_owned(),
        ))
        .lock_exclusive()
        .one(&tx)
        .await?
        .ok_or_else(|| error("spruce matching run missing"))?;
        let mut run = decode_run_row(row.clone())?;

        let msg_is_deciding = entities::spruce_message::Entity::find_by_id((
            practice.to_owned(),
            run.message_id.clone(),
        ))
        .one(&tx)
        .await?
        .is_some_and(|m| m.standing_kind == "deciding");
        let is_current = run.status == MatchingRunStatus::Running
            && run.attempt_id == attempt_id
            && msg_is_deciding;
        if !is_current && !event.kind.survives_loss_of_currency() {
            let reason = if run.attempt_id != attempt_id {
                engine::NotCurrent::Lost
            } else {
                engine::NotCurrent::Withdrawn
            };
            return Err(engine::ProductError::NotCurrent(reason));
        }

        if let Some(saved) = run
            .events
            .iter()
            .find(|prior| prior.attempt_id == event.attempt_id && prior.sequence == event.sequence)
        {
            let saved_record =
                serde_json::to_value(&saved.kind).map_err(conversion_error("EventKind", "json"))?;
            let incoming_record =
                serde_json::to_value(&event.kind).map_err(conversion_error("EventKind", "json"))?;
            if saved.ordinal != event.ordinal || saved_record != incoming_record {
                return Err(error("spruce matching event collision").into());
            }
            tx.commit().await?;
            return Ok(());
        }

        let last_seq = run
            .events
            .iter()
            .filter(|prior| prior.attempt_id == event.attempt_id)
            .map(|prior| prior.sequence)
            .max();
        if event.sequence != last_seq.map_or(1, |seq| seq + 1) {
            return Err(error("spruce matching event sequence gap").into());
        }

        if let EventKind::ToolReturned { call_id, result } = &event.kind {
            let tool_name = run
                .events
                .iter()
                .rev()
                .find_map(|prior| match &prior.kind {
                    EventKind::ToolStarted(call) if call.id == *call_id => Some(call.name.clone()),
                    EventKind::ProviderReturned(turn) => turn
                        .calls()
                        .find(|c| c.id == *call_id)
                        .map(|c| c.name.clone()),
                    _ => None,
                })
                .unwrap_or_else(|| "unknown".to_owned());
            let disposition = match result {
                engine::results::ToolResult::Refused(refusal) => {
                    ToolDiscoveryDisposition::Refused {
                        reason: refusal.reason().to_owned(),
                    }
                }
                engine::results::ToolResult::Original(_)
                | engine::results::ToolResult::History(_)
                | engine::results::ToolResult::View(_) => ToolDiscoveryDisposition::Returned,
            };
            run.tool_discoveries.push(RecordedToolDiscovery {
                ordinal: event.ordinal,
                call_id: call_id.clone(),
                tool_name,
                disposition,
                recorded_at: clock,
            });
        }

        run.events.push(event);
        let discoveries_json = serde_json::to_value(&run.tool_discoveries)
            .map_err(conversion_error("Vec<RecordedToolDiscovery>", "json"))?;
        let events_json =
            serde_json::to_value(&run.events).map_err(conversion_error("Vec<Event>", "json"))?;

        let mut active: entities::spruce_matching_run::ActiveModel = row.into();
        active.tool_discoveries = Set(discoveries_json);
        active.events = Set(events_json);
        active.update(&tx).await?;
        tx.commit().await?;
        Ok(())
    }
}

/// Shared-harness [`Product`] host executing one unmatched Spruce message classification run.
pub struct SpruceMatchingHost<'a> {
    store: Store,
    auth: &'a dyn AuthStore,
    registered_workspaces: std::sync::Arc<Vec<String>>,
    practice: String,
    run_id: String,
    attempt_id: String,
    committed_outcome: Option<MessageRoutingOutcome>,
    pending_trigger: Option<PendingSpruceReviewTrigger>,
}

impl SpruceMatchingHost<'_> {
    fn source_message(ctx: &MatchingStartingContext) -> engine::ProductMessage {
        engine::ProductMessage {
            id: format!("spruce-matching:{}:{}", ctx.message_id, ctx.content_version),
            role: engine::history::Role::User,
            text: ctx.prompt_text(),
        }
    }

    async fn load_run(&self) -> Result<SpruceMatchingRunRecord, DbErr> {
        let row = entities::spruce_matching_run::Entity::find_by_id((
            self.practice.clone(),
            self.run_id.clone(),
        ))
        .one(self.store.spruce_db())
        .await?
        .ok_or_else(|| error("spruce matching run missing"))?;
        decode_run_row(row)
    }
}

#[async_trait]
impl Product for SpruceMatchingHost<'_> {
    async fn message_sources(
        &mut self,
    ) -> Result<Option<Vec<engine::ProductMessage>>, engine::ProductError> {
        let run = self
            .load_run()
            .await
            .map_err(crate::runtime::persistence_failure)?;
        Ok(Some(vec![Self::source_message(&run.starting_context)]))
    }

    async fn admit_binding(&mut self, recipe: &RequestBinding) -> Result<(), engine::ProductError> {
        let run = self
            .load_run()
            .await
            .map_err(crate::runtime::persistence_failure)?;
        let (saved_binding, _) = self
            .store
            .load_spruce_matching_frozen(
                &self.practice,
                &run.provenance.binding,
                &run.provenance.skills,
            )
            .await
            .map_err(crate::runtime::persistence_failure)?;
        if saved_binding != *recipe {
            return Err(error("spruce matching dispatch changed frozen recipe").into());
        }
        Ok(())
    }

    fn read_only(&self, name: &str) -> bool {
        name == PATIENT_LOOKUP_TOOL
    }

    fn read_result(&mut self, id: &str) -> engine::recipe::ToolFuture<Option<String>> {
        let store = self.store.clone();
        let practice = self.practice.clone();
        let run_id = self.run_id.clone();
        let id = id.to_owned();
        Box::pin(async move {
            let row = entities::spruce_matching_run::Entity::find_by_id((practice, run_id))
                .one(store.spruce_db())
                .await?
                .ok_or_else(|| error("spruce matching run missing"))?;
            let run = decode_run_row(row)?;
            let messages = vec![Self::source_message(&run.starting_context)];
            let admitted: Vec<(Event, bool)> = run.events.into_iter().map(|e| (e, true)).collect();
            engine::results::resolve(admitted.iter(), &id, &messages).map_err(Into::into)
        })
    }

    async fn assert_current(&mut self) -> Result<(), engine::ProductError> {
        let run = self
            .load_run()
            .await
            .map_err(crate::runtime::persistence_failure)?;
        if run.attempt_id != self.attempt_id {
            return Err(engine::ProductError::NotCurrent(engine::NotCurrent::Lost));
        }
        if run.status != MatchingRunStatus::Running {
            return Err(engine::ProductError::NotCurrent(
                engine::NotCurrent::Withdrawn,
            ));
        }
        let msg = self
            .store
            .spruce_message(&self.practice, &run.message_id)
            .await
            .map_err(crate::runtime::persistence_failure)?
            .ok_or_else(|| error("spruce message disappeared during matching run"))?;
        if !matches!(msg.standing, MessageStanding::Deciding { .. }) {
            return Err(engine::ProductError::NotCurrent(
                engine::NotCurrent::Withdrawn,
            ));
        }
        Ok(())
    }

    async fn persist(&mut self, event: Event) -> Result<(), engine::ProductError> {
        self.store
            .persist_spruce_matching_run_event(
                &self.practice,
                &self.run_id,
                &self.attempt_id,
                event,
            )
            .await
            .map_err(crate::runtime::persistence_failure)
    }

    fn saved_outcome(
        &mut self,
        call: &Invocation,
    ) -> engine::recipe::ToolFuture<Option<ToolOutcome>> {
        if call.name != SUBMIT_CLASSIFICATION_TOOL {
            return Box::pin(async { Ok(None) });
        }
        let store = self.store.clone();
        let practice = self.practice.clone();
        let run_id = self.run_id.clone();
        let call_id = call.id.clone();
        Box::pin(async move {
            let Some(row) = entities::spruce_matching_run::Entity::find_by_id((practice, run_id))
                .one(store.spruce_db())
                .await?
            else {
                return Ok(None);
            };
            let run = decode_run_row(row)?;
            if let Some(staged) = run.staged_decision
                && staged.call_id == call_id
            {
                return Ok(Some(ToolOutcome::Returned(
                    json!({
                        "outcome": "staged",
                        "decision": staged.decision,
                    })
                    .to_string(),
                )));
            }
            Ok(None)
        })
    }

    fn execute(&mut self, call: &Invocation, arguments: Value) -> engine::recipe::ToolFuture {
        let store = self.store.clone();
        let registered_workspaces = std::sync::Arc::clone(&self.registered_workspaces);
        let practice = self.practice.clone();
        let run_id = self.run_id.clone();
        let call = call.clone();
        Box::pin(async move {
            if is_forbidden_chart_or_physician_tool(&call.name) {
                return Ok(ToolOutcome::refused(format!(
                    "unmatched message classification cannot access any patient clinical chart or physician tool: `{}` is refused",
                    call.name
                )));
            }

            let row = entities::spruce_matching_run::Entity::find_by_id((
                practice.clone(),
                run_id.clone(),
            ))
            .one(store.spruce_db())
            .await?
            .ok_or_else(|| error("spruce matching run missing"))?;
            let run = decode_run_row(row)?;
            let (_, skills) = store
                .load_spruce_matching_frozen(
                    &practice,
                    &run.provenance.binding,
                    &run.provenance.skills,
                )
                .await?;

            if is_spruce_working_note_tool(&call.name) {
                let note_call = Invocation {
                    id: call.id.clone(),
                    name: call.name.clone(),
                    arguments,
                };
                return execute_spruce_working_note(
                    &run.run_id,
                    &note_call,
                    run.events.iter().map(|e| (run.run_id.as_str(), e)),
                )?
                .ok_or_else(|| "working note tool was not handled".into());
            }

            if engine::history::is_tool(&call.name) {
                let src = Self::source_message(&run.starting_context);
                let messages = vec![engine::history::Message {
                    id: src.id,
                    role: src.role,
                    text: src.text,
                    metadata: json!({
                        "practice": practice,
                        "run_id": run.run_id,
                        "message_id": run.message_id,
                    }),
                }];
                let admitted: Vec<(Event, bool)> =
                    run.events.iter().cloned().map(|e| (e, true)).collect();
                let page = match engine::history::execute(
                    &call.name,
                    &arguments,
                    &messages,
                    admitted.iter(),
                ) {
                    Ok(page) => page,
                    Err(reason) => return Ok(ToolOutcome::refused(reason)),
                };
                return Ok(ToolOutcome::History(page));
            }

            if call.name == SKILL_VIEW {
                return Ok(skills.execute(&call.name, &arguments));
            }

            if call.name == PATIENT_LOOKUP_TOOL {
                let parsed: PatientLookupArgs = match serde_json::from_value(arguments) {
                    Ok(args) => args,
                    Err(err) => {
                        return Ok(ToolOutcome::refused(format!(
                            "invalid `patient_lookup` arguments: {err}"
                        )));
                    }
                };
                let roster = store
                    .practice_roster_for_workspaces(&practice, &registered_workspaces)
                    .await?;
                let query_trimmed = parsed
                    .query
                    .as_deref()
                    .map(str::trim)
                    .filter(|q| !q.is_empty());
                let matched: Vec<MatchingRosterPatient> = match query_trimmed {
                    Some(q) => {
                        let q_lower = q.to_ascii_lowercase();
                        roster
                            .into_iter()
                            .filter(|entry| matches_roster_query(entry, &q_lower))
                            .take(MAX_PATIENT_LOOKUP_RESULTS)
                            .collect()
                    }
                    None => roster
                        .into_iter()
                        .take(MAX_PATIENT_LOOKUP_RESULTS)
                        .collect(),
                };
                return Ok(ToolOutcome::Returned(
                    json!({
                        "practice": practice,
                        "query": query_trimmed,
                        "patients": matched,
                    })
                    .to_string(),
                ));
            }

            if call.name == SUBMIT_CLASSIFICATION_TOOL {
                let parsed: SubmitClassificationArgs = match serde_json::from_value(arguments) {
                    Ok(args) => args,
                    Err(err) => {
                        return Ok(ToolOutcome::refused(format!(
                            "invalid `submit_message_classification` arguments: {err}"
                        )));
                    }
                };
                let registered: BTreeSet<String> = registered_workspaces.iter().cloned().collect();
                let decision = match validate_and_build_decision(parsed, &registered) {
                    Ok(d) => d,
                    Err(reason) => return Ok(ToolOutcome::refused(reason)),
                };
                let clock = store.spruce_now().await?;
                let staged = StagedMatchingDecision {
                    call_id: call.id.clone(),
                    decision: decision.clone(),
                    staged_at: clock,
                };
                let staged_json = serde_json::to_value(&staged)
                    .map_err(conversion_error("StagedMatchingDecision", "json"))?;

                let tx = store.spruce_db().begin().await?;
                let locked = entities::spruce_matching_run::Entity::find_by_id((
                    practice.clone(),
                    run_id.clone(),
                ))
                .lock_exclusive()
                .one(&tx)
                .await?
                .ok_or_else(|| error("spruce matching run missing"))?;
                let mut active: entities::spruce_matching_run::ActiveModel = locked.into();
                active.staged_decision = Set(Some(staged_json));
                active.update(&tx).await?;
                tx.commit().await?;

                return Ok(ToolOutcome::Returned(
                    json!({
                        "outcome": "staged",
                        "decision": decision,
                    })
                    .to_string(),
                ));
            }

            Ok(ToolOutcome::refused(format!(
                "tool `{}` is not available in unmatched Spruce message classification",
                call.name
            )))
        })
    }

    async fn finish(&mut self, turn: &Turn) -> Result<Finish, engine::ProductError> {
        self.assert_current().await?;
        let run = self
            .load_run()
            .await
            .map_err(crate::runtime::persistence_failure)?;
        let Some(staged) = run.staged_decision else {
            return Ok(Finish::Continue(json!({
                "outcome": "classification_not_submitted",
                "message": "Call `submit_message_classification` to record your classification and match assessment before finishing.",
            })));
        };

        let registered: BTreeSet<String> = self
            .auth
            .practice_patients(&self.practice)
            .await?
            .into_iter()
            .collect();
        let allowed = allowed_workspaces_in_practice(self.auth, &self.practice).await?;
        let clock = self
            .store
            .spruce_now()
            .await
            .map_err(crate::runtime::persistence_failure)?;

        let tx = self
            .store
            .spruce_db()
            .begin()
            .await
            .map_err(crate::runtime::persistence_failure)?;
        let locked_run = entities::spruce_matching_run::Entity::find_by_id((
            self.practice.clone(),
            self.run_id.clone(),
        ))
        .lock_exclusive()
        .one(&tx)
        .await
        .map_err(crate::runtime::persistence_failure)?
        .ok_or_else(|| error("spruce matching run missing"))?;

        let (routing_outcome, trigger) = self
            .store
            .apply_classification_in_tx(
                &tx,
                &self.practice,
                &run.message_id,
                Some(&self.run_id),
                RECIPE_VERSION,
                GUIDANCE_VERSION,
                staged.decision,
                &registered,
                &allowed,
                clock,
            )
            .await?;

        let mut active: entities::spruce_matching_run::ActiveModel = locked_run.into();
        active.status = Set(MatchingRunStatus::Completed.as_str().to_owned());
        active.final_summary = Set(Some(turn.text()));
        active.completed_at = Set(Some(clock));
        active
            .update(&tx)
            .await
            .map_err(crate::runtime::persistence_failure)?;
        tx.commit()
            .await
            .map_err(crate::runtime::persistence_failure)?;

        self.committed_outcome = Some(routing_outcome);
        self.pending_trigger = trigger;
        Ok(Finish::Committed)
    }

    async fn finish_partial(
        &mut self,
        turn: &Turn,
        _backstops: &[engine::LimitBackstop],
    ) -> Result<(), engine::ProductError> {
        self.assert_current().await?;
        let clock = self
            .store
            .spruce_now()
            .await
            .map_err(crate::runtime::persistence_failure)?;
        let tx = self
            .store
            .spruce_db()
            .begin()
            .await
            .map_err(crate::runtime::persistence_failure)?;
        let locked_run = entities::spruce_matching_run::Entity::find_by_id((
            self.practice.clone(),
            self.run_id.clone(),
        ))
        .lock_exclusive()
        .one(&tx)
        .await
        .map_err(crate::runtime::persistence_failure)?
        .ok_or_else(|| error("spruce matching run missing"))?;
        let mut active: entities::spruce_matching_run::ActiveModel = locked_run.into();
        active.status = Set(MatchingRunStatus::Held.as_str().to_owned());
        active.final_summary = Set(Some(turn.text()));
        active.completed_at = Set(Some(clock));
        active
            .update(&tx)
            .await
            .map_err(crate::runtime::persistence_failure)?;
        tx.commit()
            .await
            .map_err(crate::runtime::persistence_failure)?;
        Ok(())
    }

    async fn window_seed(&mut self) -> Result<engine::WindowSeed, engine::ProductError> {
        self.assert_current().await?;
        let run = self
            .load_run()
            .await
            .map_err(crate::runtime::persistence_failure)?;
        let src = Self::source_message(&run.starting_context);
        let note_references =
            spruce_working_note_references(run.events.iter().map(|e| (run.run_id.as_str(), e)))?;
        Ok(engine::WindowSeed {
            items: vec![Item::ProductMessage(src)],
            note_references,
        })
    }
}

/// Assembles authorized non-chart matching context for `(practice, message_id)` without
/// reading any patient's clinical chart.
pub async fn build_matching_starting_context(
    store: &Store,
    auth: &dyn AuthStore,
    practice: &str,
    message_id: &str,
) -> Result<MatchingStartingContext, SpruceMatchingRecipeError> {
    let msg = store
        .spruce_message(practice, message_id)
        .await?
        .ok_or_else(|| SpruceMatchingError::NotFound {
            practice: practice.to_owned(),
            kind: "message",
            id: message_id.to_owned(),
        })?;
    if !matches!(msg.standing, MessageStanding::Deciding { .. }) {
        return Err(SpruceMatchingRecipeError::MessageNotDeciding {
            message_id: msg.message_id,
            standing: msg.standing.kind_str().to_owned(),
        });
    }
    let conv = store
        .spruce_conversation(practice, &msg.conversation_id)
        .await?
        .ok_or_else(|| SpruceMatchingError::NotFound {
            practice: practice.to_owned(),
            kind: "conversation",
            id: msg.conversation_id.clone(),
        })?;

    let registered_workspaces: BTreeSet<String> = auth
        .practice_patients(practice)
        .await?
        .into_iter()
        .collect();
    let (confirmed_set, channel_tie) = store
        .evaluate_conversation_identity_for_matching(
            practice,
            msg.sender_address.as_ref(),
            &conv.outside_participants,
            &registered_workspaces,
        )
        .await?;
    let confirmed_sender_workspaces: Vec<String> = confirmed_set.into_iter().collect();

    Ok(MatchingStartingContext {
        practice: msg.practice,
        message_id: msg.message_id,
        conversation_id: msg.conversation_id,
        content_version: msg.content_version,
        sender_address: msg.sender_address,
        author_name: msg.author_name,
        body_text: msg.body_text,
        attachments: msg.attachments,
        created_at_spruce: msg.created_at_spruce,
        outside_participants: conv.outside_participants,
        confirmed_sender_workspaces,
        channel_tie,
    })
}

/// Executes the unmatched Spruce message classification and matching recipe on
/// `roles.background`, recording exact model settings, recipe & guidance versions,
/// initial loads, later tool discoveries, and the full harness event trace.
#[expect(
    clippy::too_many_arguments,
    reason = "harness entry point binds store, authority, target message, model roles, skills, provider, optional review receiver, limits, and cancellation"
)]
pub async fn run_spruce_message_matching<P: Provider>(
    store: &Store,
    auth: &dyn AuthStore,
    practice: &str,
    message_id: &str,
    roles: &ModelRoles,
    skills: &PhysicianSkills,
    provider: &mut P,
    review_receiver: Option<&dyn ReviewReceiver>,
    limits: engine::RunLimits,
    cancel: &engine::Cancellation,
) -> Result<CompletedMatchingRun, SpruceMatchingRecipeError> {
    let starting_context =
        build_matching_starting_context(store, auth, practice, message_id).await?;
    let run_id = format!("spruce-match-{}", Uuid::new_v4());
    let attempt_id = format!("attempt-{}", Uuid::new_v4());
    let cache_key = harness::PromptCacheKey::scoped(
        "chords-spruce-matching",
        &[
            &starting_context.practice,
            &starting_context.conversation_id,
            &starting_context.message_id,
        ],
    );
    let built_binding = recipe(roles, skills).bind(Some(cache_key));
    let guidance_sha = instructions_sha256();
    let tie_kind = match &starting_context.channel_tie {
        ChannelTieEvaluation::UniquePatient { .. } => "unique_patient",
        ChannelTieEvaluation::Contradictory { .. } => "contradictory",
        ChannelTieEvaluation::Untied => "untied",
    }
    .to_owned();

    let clock = store.spruce_now().await?;
    let tx = store.spruce_db().begin().await?;
    let (binding_ref, skills_id) = store
        .keep_spruce_matching_frozen(&tx, &starting_context.practice, &built_binding, skills)
        .await?;

    let provenance = MatchingRecipeProvenance {
        recipe_version: RECIPE_VERSION.to_owned(),
        guidance_version: GUIDANCE_VERSION.to_owned(),
        guidance_sha256: guidance_sha.clone(),
        model: roles.background.model.clone(),
        effort: roles.background.effort.clone(),
        context_window_tokens: roles.background.context_window_tokens,
        max_input_tokens: roles.background.max_input_tokens,
        adapter: roles.background.adapter.clone(),
        binding: binding_ref,
        skills: skills_id.clone(),
    };

    let initial_loads = vec![
        MatchingInitialLoad::ResidentGuidance {
            version: GUIDANCE_VERSION.to_owned(),
            sha256: guidance_sha,
        },
        MatchingInitialLoad::SkillCatalogMetadata {
            skills_package_id: skills_id,
        },
        MatchingInitialLoad::SavedMessageEnvelope {
            message_id: starting_context.message_id.clone(),
            conversation_id: starting_context.conversation_id.clone(),
            content_version: starting_context.content_version.clone(),
        },
        MatchingInitialLoad::ConversationParticipants {
            outside_participant_count: starting_context.outside_participants.len(),
            channel_tie_kind: tie_kind,
        },
    ];

    entities::spruce_matching_run::Entity::insert(entities::spruce_matching_run::ActiveModel {
        practice: Set(starting_context.practice.clone()),
        run_id: Set(run_id.clone()),
        message_id: Set(starting_context.message_id.clone()),
        conversation_id: Set(starting_context.conversation_id.clone()),
        content_version: Set(starting_context.content_version.clone()),
        attempt_id: Set(attempt_id.clone()),
        status: Set(MatchingRunStatus::Running.as_str().to_owned()),
        provenance: Set(serde_json::to_value(&provenance)
            .map_err(conversion_error("MatchingRecipeProvenance", "json"))?),
        starting_context: Set(serde_json::to_value(&starting_context)
            .map_err(conversion_error("MatchingStartingContext", "json"))?),
        initial_loads: Set(serde_json::to_value(&initial_loads)
            .map_err(conversion_error("Vec<MatchingInitialLoad>", "json"))?),
        tool_discoveries: Set(json!([])),
        events: Set(json!([])),
        staged_decision: Set(None),
        final_summary: Set(None),
        started_at: Set(clock),
        completed_at: Set(None),
    })
    .exec_without_returning(&tx)
    .await?;
    tx.commit().await?;

    let registered_workspaces =
        std::sync::Arc::new(auth.practice_patients(&starting_context.practice).await?);
    let mut host = SpruceMatchingHost {
        store: store.clone(),
        auth,
        registered_workspaces,
        practice: starting_context.practice.clone(),
        run_id: run_id.clone(),
        attempt_id: attempt_id.clone(),
        committed_outcome: None,
        pending_trigger: None,
    };

    let source_msg = SpruceMatchingHost::source_message(&starting_context);
    let accepted_input = engine::AcceptedInput {
        id: format!("accept:{run_id}"),
        items: vec![Item::ProductMessage(source_msg.clone())],
    };
    let mut request = built_binding.request(Vec::new());
    request.items.clear();

    let run_result = async {
        let work = request.freeze()?.recover(
            &attempt_id,
            engine::recovery::Records {
                events: Vec::new(),
                allowance_after: 0,
            },
            engine::recovery::Messages {
                sources: &[source_msg],
                partial: None,
            },
            engine::recovery::Inputs {
                accepted: vec![accepted_input],
                source_feedback: None,
            },
        )?;
        work.run(provider, &mut host, limits, cancel).await
    }
    .await;

    if let Err(err) = run_result {
        let fail_clock = store.spruce_now().await?;
        let tx = store.spruce_db().begin().await?;
        if let Some(locked) = entities::spruce_matching_run::Entity::find_by_id((
            starting_context.practice.clone(),
            run_id.clone(),
        ))
        .lock_exclusive()
        .one(&tx)
        .await?
            && locked.status == MatchingRunStatus::Running.as_str()
        {
            let mut active: entities::spruce_matching_run::ActiveModel = locked.into();
            active.status = Set(MatchingRunStatus::Failed.as_str().to_owned());
            active.completed_at = Set(Some(fail_clock));
            active.update(&tx).await?;
        }
        tx.commit().await?;
        return Err(SpruceMatchingRecipeError::Harness {
            run_id,
            source: err,
        });
    }

    if let (Some(trigger), Some(receiver)) = (host.pending_trigger.take(), review_receiver) {
        let now_after = store.spruce_now().await?;
        dispatch_single_trigger(store, &trigger, receiver, now_after).await?;
    }

    let persisted = store
        .spruce_matching_run(&starting_context.practice, &run_id)
        .await?
        .ok_or_else(|| error("completed spruce matching run missing"))?;
    let Some(routing_outcome) = host.committed_outcome else {
        return Err(SpruceMatchingRecipeError::HeldWithoutClassification { run_id });
    };

    Ok(CompletedMatchingRun {
        run: persisted,
        routing_outcome,
    })
}
