//! Trigger, background tool execution, conversational handoff, and SeaORM
//! persistence for durable patient understanding.
//!
//! Each `(practice, workspace)` link owns at most one [`PatientUnderstanding`],
//! revised monotonically in place across initial chart ingestion, subsequent
//! source arrivals or record corrections, and conversational physician
//! corrections. Physician-facing summary queries project only summary columns
//! and never select [`crate::initial_understanding::OperatingIndex`].

use std::collections::{BTreeMap, BTreeSet};

use harness::engine::{self, Invocation, ToolOutcome, recipe::RequestRecipe};
use physician_auth::{AuthStore, Membership, PhysicianIdentity};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, ConnectionTrait, DbErr, EntityTrait, QueryFilter,
    QueryOrder, QuerySelect, Set, TransactionTrait,
};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

use super::concerns::ExactRecordReference;
use super::entities::{patient_understanding, patient_understanding_revision, practice_job};
use super::jobs::{
    Assignment, AssignmentError, PracticeAccessError, PracticeAuthority, PracticeJobClaim,
    PracticeJobError, PracticeJobKind, PracticeScope,
};
use super::sources::SourcePull;
use super::{Accepted, Claim, Store, WorkStatus, conversion_error, error, now};
use crate::initial_understanding::{
    ConnectedSourceBrief, EvidenceScope, OperatingIndex, READ_UNDERSTANDING_TOOL,
    SAVE_UNDERSTANDING_TOOL, SaveUnderstandingArgs, SummaryClaimInput, UnderstandingContext,
    UnderstandingInputError, UnderstandingMode, job_key,
};
use crate::physician_chat::PhysicianSkills;
use crate::physician_sources::SourceRevision;
use crate::{ModelBinding, ModelRoles};

const EXCERPT_MAX_CHARS: usize = 360;

/// Verified citation attached to [`PatientSummary`].
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SummaryCitation {
    /// Exact `(candidate_id, revision)` record identity.
    pub reference: ExactRecordReference,
    /// Source identifier owning the record.
    pub source_id: String,
    /// Human-readable source label.
    pub source_label: String,
    /// Verified SHA-256 of the retained source content bytes.
    pub sha256: String,
    /// Display title shown on the numbered citation chip and side panel.
    pub display_label: String,
    /// Effective record date formatted as recorded by the source, when present.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub as_of: Option<String>,
    /// Bounded readable excerpt shown when inspecting the citation.
    pub excerpt: String,
    /// Native chart destination URI when provenance supplies a locator.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub native_uri: Option<String>,
}

/// Ordered prose or citation segment inside [`PatientSummary`].
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum SummarySegment {
    /// Physician-facing prose segment.
    Prose {
        /// Prose text.
        text: String,
    },
    /// Reference into [`PatientSummary::citations`] by zero-based index.
    Citation {
        /// Index into [`PatientSummary::citations`].
        index: usize,
    },
}

/// Physician-facing summary stored for a patient workspace.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PatientSummary {
    /// Complete rendered plain text (claims, scope note, and material gaps).
    pub narrative: String,
    /// Structured clinical statements and their supporting references.
    pub claims: Vec<SummaryClaimInput>,
    /// Explicit material gaps in available sources or clinical follow-up.
    pub material_gaps: Vec<String>,
    /// Explicit scope limitation note when applicable (such as Duet-only scope).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope_note: Option<String>,
    /// Deduplicated, source-verified citations referenced by `segments`.
    pub citations: Vec<SummaryCitation>,
    /// Typed prose and citation segments rendered on the patient page.
    pub segments: Vec<SummarySegment>,
}

/// Work execution that authored a [`PatientUnderstanding`] revision.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum RevisionOrigin {
    /// Authored by a practice-owned background job.
    PracticeRun {
        /// Practice job identifier.
        job_id: String,
        /// Durable work identifier.
        work_id: String,
        /// Attempt identifier that executed the save.
        attempt_id: String,
        /// Whether this background run initialized or revised understanding.
        mode: UnderstandingMode,
    },
    /// Revised in conversation with an authorized panel physician.
    PhysicianConversation {
        /// Authenticated physician identifier.
        physician_id: String,
        /// Conversation identifier.
        chat_id: String,
        /// Durable work identifier.
        work_id: String,
        /// Attempt identifier that executed the save.
        attempt_id: String,
    },
}

impl RevisionOrigin {
    fn work_id(&self) -> &str {
        match self {
            Self::PracticeRun { work_id, .. } | Self::PhysicianConversation { work_id, .. } => {
                work_id
            }
        }
    }
}

/// Complete internal patient understanding, including Coda's private
/// [`OperatingIndex`]. Never serialized into physician-facing wire responses.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PatientUnderstanding {
    /// Owning practice identifier.
    pub practice: String,
    /// Patient workspace identifier.
    pub workspace: String,
    /// Monotonic understanding revision (`>= 1`).
    pub revision: u64,
    /// Workspace source observation version covered by this revision.
    pub source_version: u64,
    /// Source coverage when this revision was written.
    pub evidence_scope: EvidenceScope,
    /// Strictly private cross-run working memory for Coda.
    pub operating_index: OperatingIndex,
    /// Physician-facing patient summary and verified citations.
    pub summary: PatientSummary,
    /// Run or conversation that saved this revision.
    pub origin: RevisionOrigin,
    /// Database-clock Unix epoch seconds when saved.
    pub updated_at: i64,
}

/// Append-only revision record from `physician_runtime_patient_understanding_revisions`.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct UnderstandingRevisionRecord {
    /// Monotonic revision number (`>= 1`).
    pub revision: u64,
    /// Idempotency key that produced this revision.
    pub idempotency_key: String,
    /// Workspace source version covered by this revision.
    pub source_version: u64,
    /// Source coverage at save time.
    pub evidence_scope: EvidenceScope,
    /// Private operating index snapshot at this revision.
    pub operating_index: OperatingIndex,
    /// Physician-facing summary snapshot at this revision.
    pub summary: PatientSummary,
    /// Originating background run or conversation.
    pub origin: RevisionOrigin,
    /// Database-clock Unix epoch seconds when saved.
    pub saved_at: i64,
}

/// Audit standing of an immutable [`UnderstandingRevisionRecord`].
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "standing", rename_all = "snake_case")]
pub enum UnderstandingStanding {
    /// Revision remains valid clinical history.
    Active,
    /// Revision was derived from a wrong-patient message or fenced run and
    /// marked entered-in-error while keeping its row immutable for audit.
    EnteredInError {
        /// Withdrawal receipt identifier.
        withdrawal_id: String,
        /// Workspace withdrawal epoch when marked entered-in-error.
        withdrawal_epoch: u64,
        /// Single-line withdrawal reason.
        reason: String,
        /// Database-clock Unix timestamp when withdrawn.
        withdrawn_at: i64,
    },
}

/// Immutable understanding revision paired with its current [`UnderstandingStanding`].
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct UnderstandingRevisionAudit {
    /// Immutable revision record as originally saved.
    pub record: UnderstandingRevisionRecord,
    /// Standing (`Active` or `EnteredInError`).
    pub standing: UnderstandingStanding,
}

/// Physician-facing summary projection that physically omits [`OperatingIndex`].
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PatientSummaryView {
    /// Owning practice identifier.
    pub practice: String,
    /// Patient workspace identifier.
    pub workspace: String,
    /// Monotonic summary revision (`>= 1`).
    pub revision: u64,
    /// Workspace source version covered by this summary.
    pub source_version: u64,
    /// Source coverage for this summary.
    pub evidence_scope: EvidenceScope,
    /// Physician-facing clinical summary, material gaps, and verified citations.
    pub summary: PatientSummary,
    /// Database-clock Unix epoch seconds when saved.
    pub updated_at: i64,
}

impl PatientSummaryView {
    /// Projects this summary to the `patient_synopsis` wire shape consumed by
    /// `physician-wire` and `physician-app`, with zero [`OperatingIndex`] fields.
    pub fn to_wire_synopsis(&self, authored_at: String) -> Value {
        let mut wire_segments = Vec::with_capacity(self.summary.segments.len());
        for segment in &self.summary.segments {
            match segment {
                SummarySegment::Prose { text } => {
                    wire_segments.push(json!({
                        "kind": "text",
                        "text": text,
                    }));
                }
                SummarySegment::Citation { index } => {
                    if let Some(citation) = self.summary.citations.get(*index) {
                        let destination = citation.native_uri.as_ref().map(|uri| {
                            json!({
                                "kind": "source_native",
                                "source_label": citation.source_label,
                                "uri": uri,
                            })
                        });
                        wire_segments.push(json!({
                            "kind": "citation",
                            "state": "resolved",
                            "display_label": citation.display_label,
                            "source": "patient_source",
                            "source_id": citation.source_id,
                            "handle": format!(
                                "{}@{}",
                                citation.reference.candidate_id,
                                citation.reference.revision
                            ),
                            "source_presentation": {
                                "kind": "coda_authored",
                                "content": {
                                    "as_of": citation.as_of.clone().unwrap_or_default(),
                                    "text": citation.excerpt,
                                }
                            },
                            "destination": destination,
                        }));
                    }
                }
            }
        }
        json!({
            "version_id": format!("understanding-v{}", self.revision),
            "authored_text": self.summary.narrative,
            "segments": wire_segments,
            "authored_at": authored_at,
        })
    }
}

/// Standing of an in-progress or exhausted patient understanding background run.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UnderstandingRunState {
    /// Accepted or retrying within the practice-job retry policy.
    Running,
    /// Exhausted automatic retries without finishing (`"couldn't finish"`).
    Failed,
}

/// Physician-facing run status projected to `PatientWorkspaceResponse.automation_run`.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct PatientUnderstandingRunStatus {
    /// Patient workspace identifier.
    pub workspace: String,
    /// Practice job identifier.
    pub job_id: String,
    /// Latest attempt identifier when claimed at least once.
    pub attempt_id: Option<String>,
    /// Running (including pending retries) vs terminal failed standing.
    pub state: UnderstandingRunState,
    /// Whether the job initializes or revises understanding.
    pub mode: UnderstandingMode,
    /// Database-clock seconds when the job started.
    pub started_at: i64,
}

impl PatientUnderstandingRunStatus {
    /// Projects this background run status to the `automation_run` wire shape.
    pub fn to_wire_automation_run(&self, started_at_rfc3339: String) -> Value {
        let headline = match self.mode {
            UnderstandingMode::Initial => "Building initial patient summary",
            UnderstandingMode::Revision { .. } => "Updating patient summary",
        };
        let state = match self.state {
            UnderstandingRunState::Running => "running",
            UnderstandingRunState::Failed => "failed",
        };
        json!({
            "kind": "initial_understanding",
            "run_id": self.attempt_id.clone().unwrap_or_else(|| self.job_id.clone()),
            "started_at": started_at_rfc3339,
            "state": state,
            "physician_interruptible": false,
            "purpose": {
                "headline": headline,
                "follow_up_descriptions": [],
            }
        })
    }
}

/// Outcome of triggering patient understanding on a patient link or source update.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum UnderstandingTriggerOutcome {
    /// Accepted a new `InitialUnderstanding` practice job on `roles.background`.
    Started {
        /// Scope of the accepted practice job.
        scope: PracticeScope,
        /// Whether this job builds an initial memory or revises an existing one.
        mode: UnderstandingMode,
        /// Workspace source version targeted by the job.
        source_version: u64,
    },
    /// Saved understanding already covers at least `source_version`.
    AlreadyCurrent {
        /// Current saved understanding revision.
        revision: u64,
        /// Covered workspace source version.
        source_version: u64,
    },
    /// Matching or newer understanding job is already queued or running.
    AlreadyQueuedOrActive {
        /// Existing queued or active practice job scope.
        scope: PracticeScope,
        /// Workspace source version targeted by that job.
        source_version: u64,
    },
}

/// Failure when triggering or reading practice-scoped patient understanding.
#[derive(Debug, thiserror::Error)]
pub enum UnderstandingError {
    /// Practice patient membership check failed or was refused.
    #[error("{0}")]
    Access(#[from] PracticeAccessError),
    /// Assignment construction failed.
    #[error("{0}")]
    Assignment(#[from] AssignmentError),
    /// Practice job acceptance or query failed.
    #[error("{0}")]
    PracticeJob(#[from] PracticeJobError),
    /// Database read or write failed.
    #[error("{0}")]
    Database(#[from] DbErr),
}

/// Failure when reading a patient summary on behalf of a signed-in physician.
#[derive(Debug, thiserror::Error)]
pub enum PhysicianSummaryAccessError {
    /// Physician lacks panel access to read clinical summaries for `workspace`.
    #[error(
        "physician {physician_id} in practice {practice_id} is not authorized for patient workspace {workspace} ({membership:?})"
    )]
    Unauthorized {
        /// Physician's practice identifier.
        practice_id: String,
        /// Physician identifier.
        physician_id: String,
        /// Target patient workspace identifier.
        workspace: String,
        /// Resolved membership standing.
        membership: Membership,
    },
    /// Database read failed.
    #[error("{0}")]
    Database(#[from] DbErr),
}

/// Why `save_patient_understanding` refused a completion candidate without saving.
#[derive(Debug)]
enum SaveRefusal {
    InvalidArgs(UnderstandingInputError),
    MissingCitationsForAvailableChart,
    MissingScopeNoteForDuetOnly,
    MissingSourceRecord {
        candidate_id: String,
    },
    StaleSourceRevision {
        candidate_id: String,
        cited_revision: String,
        current_revision: String,
    },
    UnreadSourceRevision {
        candidate_id: String,
        revision: String,
    },
    RevisionConflict {
        expected: u64,
        actual: u64,
    },
}

impl SaveRefusal {
    fn into_outcome(self) -> ToolOutcome {
        let body = match self {
            Self::InvalidArgs(err) => json!({
                "outcome": "invalid_arguments",
                "message": err.to_string(),
            }),
            Self::MissingCitationsForAvailableChart => json!({
                "outcome": "missing_citations",
                "message": "Workspace source records are available, so at least one summary claim must cite an exact record revision read via read_patient_sources. Nothing was saved.",
            }),
            Self::MissingScopeNoteForDuetOnly => json!({
                "outcome": "missing_scope_note",
                "message": "Duet patient-shared information is the only available source; provide scope_note stating that this understanding is limited to what the patient chose to share. Nothing was saved.",
            }),
            Self::MissingSourceRecord { candidate_id } => json!({
                "outcome": "unknown_source_reference",
                "candidate_id": candidate_id,
                "message": format!("Cited record `{candidate_id}` is not present in the workspace's current sources. Nothing was saved."),
            }),
            Self::StaleSourceRevision {
                candidate_id,
                cited_revision,
                current_revision,
            } => json!({
                "outcome": "stale_source_revision",
                "candidate_id": candidate_id,
                "cited_revision": cited_revision,
                "current_revision": current_revision,
                "message": format!(
                    "Cited record `{candidate_id}` moved from `{cited_revision}` to `{current_revision}`. Read `{current_revision}` with read_patient_sources before saving. Nothing was saved."
                ),
            }),
            Self::UnreadSourceRevision {
                candidate_id,
                revision,
            } => json!({
                "outcome": "unread_source_reference",
                "candidate_id": candidate_id,
                "revision": revision,
                "message": format!(
                    "Cited record `{candidate_id}` at revision `{revision}` has not been read via read_patient_sources. Read the record before citing it. Nothing was saved."
                ),
            }),
            Self::RevisionConflict { expected, actual } => json!({
                "outcome": "revision_conflict",
                "expected_revision": expected,
                "current_revision": actual,
                "message": format!(
                    "Patient understanding is at revision {actual}, not {expected}. Call read_patient_understanding to inspect the current understanding before saving. Nothing was saved."
                ),
            }),
        };
        ToolOutcome::refused(body.to_string())
    }
}

fn connected_briefs(pulls: &[SourcePull]) -> Vec<ConnectedSourceBrief> {
    pulls
        .iter()
        .map(|pull| ConnectedSourceBrief {
            source_id: pull.source_id.clone(),
            source_label: pull.source_label.clone(),
            pulled_at: pull.pulled_at.unwrap_or(0),
        })
        .collect()
}

fn source_label_for(source_id: &str, pulls: &[SourcePull]) -> String {
    pulls
        .iter()
        .find(|pull| pull.source_id == source_id)
        .map(|pull| pull.source_label.clone())
        .filter(|label| !label.trim().is_empty())
        .unwrap_or_else(|| source_id.to_owned())
}

fn truncate_excerpt(text: &str) -> String {
    let trimmed = text.trim();
    if trimmed.chars().count() <= EXCERPT_MAX_CHARS {
        return trimmed.to_owned();
    }
    let prefix: String = trimmed.chars().take(EXCERPT_MAX_CHARS).collect();
    let cut = match prefix.rfind(char::is_whitespace) {
        Some(idx) if idx > EXCERPT_MAX_CHARS / 2 => &prefix[..idx],
        _ => prefix.as_str(),
    };
    format!("{}…", cut.trim_end())
}

fn record_display_label(source: &SourceRevision) -> String {
    super::history::cited_record_title(
        &source.provenance,
        &source.record_type,
        &source.candidate_id,
    )
}

pub(super) fn record_excerpt(source: &SourceRevision) -> String {
    let title = record_display_label(source);
    let body =
        super::history::format_cited_source_body(&title, &source.readable_parts, &source.content);
    truncate_excerpt(&body)
}

pub(super) fn build_summary_citation(
    workspace: &str,
    reference: &ExactRecordReference,
    source: &SourceRevision,
    pulls: &[SourcePull],
) -> SummaryCitation {
    SummaryCitation {
        reference: reference.clone(),
        source_id: source.source_id.clone(),
        source_label: source_label_for(&source.source_id, pulls),
        sha256: source.sha256.clone(),
        display_label: record_display_label(source),
        as_of: record_as_of(source),
        excerpt: record_excerpt(source),
        native_uri: record_native_uri(workspace, source),
    }
}

fn record_as_of(source: &SourceRevision) -> Option<String> {
    source
        .dates
        .first()
        .map(|d| d.value.trim())
        .filter(|v| !v.is_empty())
        .map(|raw| raw.split('T').next().unwrap_or(raw).trim().to_owned())
}

fn record_native_uri(workspace: &str, source: &SourceRevision) -> Option<String> {
    let locator = source
        .provenance
        .get("record_locator")
        .or_else(|| source.provenance.get("native_id"))
        .and_then(Value::as_str)
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .unwrap_or(source.candidate_id.as_str());
    Some(format!(
        "/local-ehr/{workspace}/{locator}#/resources/{locator}"
    ))
}

/// Single rule owner collecting `(candidate_id, revision)` pairs already
/// verified and stored on `saved` (across both summary citations and
/// operating-index anchors).
pub(super) fn known_understanding_pairs(
    saved: &PatientUnderstanding,
) -> BTreeSet<(String, String)> {
    let mut pairs: BTreeSet<(String, String)> = saved
        .summary
        .citations
        .iter()
        .map(|c| {
            (
                c.reference.candidate_id.clone(),
                c.reference.revision.clone(),
            )
        })
        .collect();
    for anchor in &saved.operating_index.anchors {
        pairs.insert((
            anchor.reference.candidate_id.clone(),
            anchor.reference.revision.clone(),
        ));
    }
    pairs
}

/// Single rule owner checking one [`ExactRecordReference`] against current
/// workspace observations and `(read_in_work ∪ carried_forward_refs)`.
fn verify_read_current_reference<'a>(
    reference: &ExactRecordReference,
    by_candidate: &BTreeMap<&str, &'a SourceRevision>,
    read_in_work: &BTreeSet<(String, String)>,
    carried_forward_refs: &BTreeSet<(String, String)>,
) -> Result<&'a SourceRevision, SaveRefusal> {
    let Some(source) = by_candidate.get(reference.candidate_id.as_str()).copied() else {
        return Err(SaveRefusal::MissingSourceRecord {
            candidate_id: reference.candidate_id.clone(),
        });
    };
    if source.revision != reference.revision {
        return Err(SaveRefusal::StaleSourceRevision {
            candidate_id: reference.candidate_id.clone(),
            cited_revision: reference.revision.clone(),
            current_revision: source.revision.clone(),
        });
    }
    let key = (reference.candidate_id.clone(), reference.revision.clone());
    if !read_in_work.contains(&key) && !carried_forward_refs.contains(&key) {
        return Err(SaveRefusal::UnreadSourceRevision {
            candidate_id: reference.candidate_id.clone(),
            revision: reference.revision.clone(),
        });
    }
    Ok(source)
}

/// Single rule owner verifying operating-index anchors + summary citations and
/// constructing [`PatientSummary`].
fn build_verified_summary(
    workspace: &str,
    args: &SaveUnderstandingArgs,
    evidence_scope: &EvidenceScope,
    current_sources: &[SourceRevision],
    pulls: &[SourcePull],
    read_in_work: &BTreeSet<(String, String)>,
    carried_forward_refs: &BTreeSet<(String, String)>,
) -> Result<PatientSummary, SaveRefusal> {
    if matches!(evidence_scope, EvidenceScope::DuetPatientSharedOnly) && args.scope_note.is_none() {
        return Err(SaveRefusal::MissingScopeNoteForDuetOnly);
    }

    let by_candidate: BTreeMap<&str, &SourceRevision> = current_sources
        .iter()
        .map(|source| (source.candidate_id.as_str(), source))
        .collect();

    for anchor in &args.operating_index.anchors {
        verify_read_current_reference(
            &anchor.reference,
            &by_candidate,
            read_in_work,
            carried_forward_refs,
        )?;
    }

    let mut citations: Vec<SummaryCitation> = Vec::new();
    let mut citation_index_by_ref: BTreeMap<(String, String), usize> = BTreeMap::new();
    let mut segments: Vec<SummarySegment> = Vec::new();
    let mut prose_parts: Vec<String> = Vec::new();

    if let Some(scope_note) = &args.scope_note {
        let clean = ensure_terminal_period(scope_note);
        segments.push(SummarySegment::Prose {
            text: format!("{clean} "),
        });
        prose_parts.push(clean);
    }

    for (claim_idx, claim) in args.claims.iter().enumerate() {
        let claim_text = ensure_terminal_period(&claim.text);
        let leading = if claim_idx == 0 && args.scope_note.is_none() {
            claim_text.clone()
        } else {
            format!(" {claim_text}")
        };
        let trailing_space = if claim.citations.is_empty() {
            leading
        } else {
            format!("{leading} ")
        };
        segments.push(SummarySegment::Prose {
            text: trailing_space,
        });
        prose_parts.push(claim_text);

        for reference in &claim.citations {
            let source = verify_read_current_reference(
                reference,
                &by_candidate,
                read_in_work,
                carried_forward_refs,
            )?;
            let key = (reference.candidate_id.clone(), reference.revision.clone());
            let idx = if let Some(&existing) = citation_index_by_ref.get(&key) {
                existing
            } else {
                let next_idx = citations.len();
                citations.push(build_summary_citation(workspace, reference, source, pulls));
                citation_index_by_ref.insert(key, next_idx);
                next_idx
            };
            segments.push(SummarySegment::Citation { index: idx });
        }
    }

    if !current_sources.is_empty() && citations.is_empty() {
        return Err(SaveRefusal::MissingCitationsForAvailableChart);
    }

    let gaps_joined = args
        .material_gaps
        .iter()
        .map(|gap| gap.trim().trim_end_matches('.'))
        .collect::<Vec<_>>()
        .join("; ");
    let gaps_sentence = format!("Material gaps: {gaps_joined}.");
    segments.push(SummarySegment::Prose {
        text: format!(" {gaps_sentence}"),
    });
    prose_parts.push(gaps_sentence);

    Ok(PatientSummary {
        narrative: prose_parts.join(" "),
        claims: args.claims.clone(),
        material_gaps: args.material_gaps.clone(),
        scope_note: args.scope_note.clone(),
        citations,
        segments,
    })
}

fn ensure_terminal_period(text: &str) -> String {
    let trimmed = text.trim();
    if trimmed.ends_with('.') || trimmed.ends_with('!') || trimmed.ends_with('?') {
        trimmed.to_owned()
    } else {
        format!("{trimmed}.")
    }
}

fn decode_understanding(row: patient_understanding::Model) -> Result<PatientUnderstanding, DbErr> {
    let revision = u64::try_from(row.revision)
        .map_err(conversion_error("patient_understanding.revision", "u64"))?;
    let source_version = u64::try_from(row.source_version).map_err(conversion_error(
        "patient_understanding.source_version",
        "u64",
    ))?;
    let evidence_scope: EvidenceScope = serde_json::from_value(row.evidence_scope).map_err(
        conversion_error("patient_understanding.evidence_scope", "EvidenceScope"),
    )?;
    let operating_index: OperatingIndex = serde_json::from_value(row.operating_index).map_err(
        conversion_error("patient_understanding.operating_index", "OperatingIndex"),
    )?;
    let summary: PatientSummary = serde_json::from_value(row.summary).map_err(conversion_error(
        "patient_understanding.summary",
        "PatientSummary",
    ))?;
    let origin: RevisionOrigin = serde_json::from_value(row.origin).map_err(conversion_error(
        "patient_understanding.origin",
        "RevisionOrigin",
    ))?;
    Ok(PatientUnderstanding {
        practice: row.practice,
        workspace: row.workspace,
        revision,
        source_version,
        evidence_scope,
        operating_index,
        summary,
        origin,
        updated_at: row.updated_at,
    })
}

fn decode_revision_record(
    row: patient_understanding_revision::Model,
) -> Result<UnderstandingRevisionRecord, DbErr> {
    let revision = u64::try_from(row.revision).map_err(conversion_error(
        "patient_understanding_revision.revision",
        "u64",
    ))?;
    let source_version = u64::try_from(row.source_version).map_err(conversion_error(
        "patient_understanding_revision.source_version",
        "u64",
    ))?;
    let evidence_scope: EvidenceScope =
        serde_json::from_value(row.evidence_scope).map_err(conversion_error(
            "patient_understanding_revision.evidence_scope",
            "EvidenceScope",
        ))?;
    let operating_index: OperatingIndex =
        serde_json::from_value(row.operating_index).map_err(conversion_error(
            "patient_understanding_revision.operating_index",
            "OperatingIndex",
        ))?;
    let summary: PatientSummary = serde_json::from_value(row.summary).map_err(conversion_error(
        "patient_understanding_revision.summary",
        "PatientSummary",
    ))?;
    let origin: RevisionOrigin = serde_json::from_value(row.origin).map_err(conversion_error(
        "patient_understanding_revision.origin",
        "RevisionOrigin",
    ))?;
    Ok(UnderstandingRevisionRecord {
        revision,
        idempotency_key: row.idempotency_key,
        source_version,
        evidence_scope,
        operating_index,
        summary,
        origin,
        saved_at: row.saved_at,
    })
}

type SummaryProjectionTuple = (String, String, i64, i64, Value, Value, i64);

fn decode_summary_projection(tuple: SummaryProjectionTuple) -> Result<PatientSummaryView, DbErr> {
    let (practice, workspace, revision, source_version, evidence_scope, summary, updated_at) =
        tuple;
    let revision = u64::try_from(revision)
        .map_err(conversion_error("patient_understanding.revision", "u64"))?;
    let source_version = u64::try_from(source_version).map_err(conversion_error(
        "patient_understanding.source_version",
        "u64",
    ))?;
    let evidence_scope: EvidenceScope = serde_json::from_value(evidence_scope).map_err(
        conversion_error("patient_understanding.evidence_scope", "EvidenceScope"),
    )?;
    let summary: PatientSummary = serde_json::from_value(summary).map_err(conversion_error(
        "patient_understanding.summary",
        "PatientSummary",
    ))?;
    Ok(PatientSummaryView {
        practice,
        workspace,
        revision,
        source_version,
        evidence_scope,
        summary,
        updated_at,
    })
}

fn understanding_cites_withheld(
    summary: &PatientSummary,
    operating_index: Option<&OperatingIndex>,
    origin: Option<&RevisionOrigin>,
    withheld: &super::link_correction::WithheldWorkspaceState,
) -> bool {
    if summary
        .citations
        .iter()
        .any(|c| withheld.candidate_ids.contains(&c.reference.candidate_id))
    {
        return true;
    }
    if summary.claims.iter().any(|cl| {
        cl.citations
            .iter()
            .any(|r| withheld.candidate_ids.contains(&r.candidate_id))
    }) {
        return true;
    }
    if let Some(idx) = operating_index
        && idx
            .anchors
            .iter()
            .any(|a| withheld.candidate_ids.contains(&a.reference.candidate_id))
    {
        return true;
    }
    if let Some(orig) = origin
        && withheld.work_ids.contains(orig.work_id())
    {
        return true;
    }
    false
}

async fn latest_clean_understanding_in_db(
    db: &impl ConnectionTrait,
    practice: &str,
    workspace: &str,
    withheld: &super::link_correction::WithheldWorkspaceState,
) -> Result<Option<PatientUnderstanding>, DbErr> {
    let withdrawn_rows = super::follow_ups::understanding_withdrawal_entity::Entity::find()
        .filter(super::follow_ups::understanding_withdrawal_entity::Column::Practice.eq(practice))
        .filter(super::follow_ups::understanding_withdrawal_entity::Column::Workspace.eq(workspace))
        .all(db)
        .await?;
    let withdrawn_revs: std::collections::BTreeSet<i64> =
        withdrawn_rows.into_iter().map(|r| r.revision).collect();

    let rev_rows = patient_understanding_revision::Entity::find()
        .filter(patient_understanding_revision::Column::Practice.eq(practice))
        .filter(patient_understanding_revision::Column::Workspace.eq(workspace))
        .order_by_desc(patient_understanding_revision::Column::Revision)
        .all(db)
        .await?;
    for row in rev_rows {
        if withdrawn_revs.contains(&row.revision) {
            continue;
        }
        let rec = decode_revision_record(row)?;
        if !understanding_cites_withheld(
            &rec.summary,
            Some(&rec.operating_index),
            Some(&rec.origin),
            withheld,
        ) {
            return Ok(Some(PatientUnderstanding {
                practice: practice.to_owned(),
                workspace: workspace.to_owned(),
                revision: rec.revision,
                source_version: rec.source_version,
                evidence_scope: rec.evidence_scope,
                operating_index: rec.operating_index,
                summary: rec.summary,
                origin: rec.origin,
                updated_at: rec.saved_at,
            }));
        }
    }
    Ok(None)
}

fn scope_for_membership(practice: &str, workspace: &str) -> PracticeScope {
    PracticeScope {
        practice: practice.to_owned(),
        workspace: workspace.to_owned(),
        job_id: "understanding-access".to_owned(),
    }
}

/// Configured model roles and skill catalog used by [`Store`] to queue
/// background patient understanding runs when source refreshes deliver new or
/// revised records.
#[derive(Clone)]
pub(crate) struct UnderstandingTriggerBinding {
    pub(crate) roles: ModelRoles,
    pub(crate) skills: PhysicianSkills,
}

/// Internal [`PracticeAuthority`] witness constructed only after an entrypoint
/// has already verified practice or physician panel membership for `(practice, workspace)`.
struct VerifiedPracticePatient;

#[async_trait::async_trait]
impl PracticeAuthority for VerifiedPracticePatient {
    async fn check_practice_patient(
        &self,
        scope: &PracticeScope,
    ) -> Result<(), PracticeAccessError> {
        if scope.validate().is_err() {
            return Err(PracticeAccessError::InvalidScope);
        }
        Ok(())
    }
}

impl Store {
    /// Starts an initial patient understanding run on a newly linked patient
    /// workspace, or starts an in-place revision run when `workspace`'s source
    /// observation version has advanced beyond the saved understanding.
    ///
    /// Idempotent across repeated link or source events: returns
    /// [`UnderstandingTriggerOutcome::AlreadyQueuedOrActive`] while a run for
    /// the current source version is pending or active, and
    /// [`UnderstandingTriggerOutcome::AlreadyCurrent`] once saved.
    pub async fn trigger_patient_understanding<A: PracticeAuthority + ?Sized>(
        &self,
        practice: &str,
        workspace: &str,
        roles: &ModelRoles,
        skills: &PhysicianSkills,
        authority: &A,
    ) -> Result<UnderstandingTriggerOutcome, UnderstandingError> {
        authority
            .check_practice_patient(&scope_for_membership(practice, workspace))
            .await?;
        self.trigger_patient_understanding_authorized(practice, workspace, roles, skills)
            .await
    }

    pub(crate) async fn trigger_patient_understanding_authorized(
        &self,
        practice: &str,
        workspace: &str,
        roles: &ModelRoles,
        skills: &PhysicianSkills,
    ) -> Result<UnderstandingTriggerOutcome, UnderstandingError> {
        let tx = self.db.begin().await?;
        let observation = super::sources::observed(&tx, workspace).await?;
        let pulls_map = self.source_pulls(&[workspace.to_owned()]).await?;
        let pulls = pulls_map.get(workspace).cloned().unwrap_or_default();
        let existing =
            patient_understanding::Entity::find_by_id((practice.to_owned(), workspace.to_owned()))
                .one(&tx)
                .await?
                .map(decode_understanding)
                .transpose()?;

        let job_rows = practice_job::Entity::find()
            .filter(
                Condition::all()
                    .add(practice_job::Column::Practice.eq(practice))
                    .add(practice_job::Column::Workspace.eq(workspace))
                    .add(practice_job::Column::Kind.eq(PracticeJobKind::InitialUnderstanding)),
            )
            .order_by_desc(practice_job::Column::CreatedAt)
            .order_by_desc(practice_job::Column::SourcesVersion)
            .order_by_desc(practice_job::Column::JobId)
            .all(&tx)
            .await?;
        tx.commit().await?;

        for row in &job_rows {
            if matches!(row.status, WorkStatus::Pending | WorkStatus::Active) {
                let assignment: Assignment = serde_json::from_value(row.assignment.clone())
                    .map_err(conversion_error("practice_job.assignment", "Assignment"))?;
                let target_version = assignment
                    .understanding
                    .as_ref()
                    .map(|ctx| ctx.target_source_version)
                    .unwrap_or_else(|| u64::try_from(row.sources_version.max(0)).unwrap_or(0));
                if target_version >= observation.version {
                    return Ok(UnderstandingTriggerOutcome::AlreadyQueuedOrActive {
                        scope: PracticeScope {
                            practice: practice.to_owned(),
                            workspace: workspace.to_owned(),
                            job_id: row.job_id.clone(),
                        },
                        source_version: target_version,
                    });
                }
            }
        }

        if let Some(saved) = &existing
            && saved.source_version >= observation.version
        {
            return Ok(UnderstandingTriggerOutcome::AlreadyCurrent {
                revision: saved.revision,
                source_version: saved.source_version,
            });
        }

        let (mode, changed_candidates) = match &existing {
            None => (
                UnderstandingMode::Initial,
                observation
                    .sources
                    .iter()
                    .map(|source| source.candidate_id.clone())
                    .collect(),
            ),
            Some(saved) => {
                let known_pairs = known_understanding_pairs(saved);
                let changed: Vec<String> = observation
                    .sources
                    .iter()
                    .filter(|source| {
                        !known_pairs
                            .contains(&(source.candidate_id.clone(), source.revision.clone()))
                    })
                    .map(|source| source.candidate_id.clone())
                    .collect();
                (
                    UnderstandingMode::Revision {
                        prior_revision: saved.revision,
                        prior_source_version: saved.source_version,
                    },
                    changed,
                )
            }
        };

        let briefs = connected_briefs(&pulls);
        let evidence_scope = EvidenceScope::classify(
            &briefs,
            observation.sources.iter().map(|s| s.source_id.as_str()),
        );
        let context = UnderstandingContext {
            workspace: workspace.to_owned(),
            mode,
            target_source_version: observation.version,
            evidence_scope,
            changed_candidates,
        };
        let assignment = crate::initial_understanding::assignment(context, skills)?;
        let scope = PracticeScope {
            practice: practice.to_owned(),
            workspace: workspace.to_owned(),
            job_id: job_key(workspace, observation.version),
        };
        match self
            .accept_practice_job(&scope, &assignment, roles, skills, &VerifiedPracticePatient)
            .await?
        {
            Accepted::New => Ok(UnderstandingTriggerOutcome::Started {
                scope,
                mode,
                source_version: observation.version,
            }),
            Accepted::Duplicate => Ok(UnderstandingTriggerOutcome::AlreadyQueuedOrActive {
                scope,
                source_version: observation.version,
            }),
        }
    }

    /// Reads the complete internal [`PatientUnderstanding`] (including the
    /// private [`OperatingIndex`]) for a practice-owned background job or
    /// internal Coda agent run after verifying practice patient membership.
    pub async fn patient_understanding_for_practice<A: PracticeAuthority + ?Sized>(
        &self,
        practice: &str,
        workspace: &str,
        authority: &A,
    ) -> Result<Option<PatientUnderstanding>, UnderstandingError> {
        authority
            .check_practice_patient(&scope_for_membership(practice, workspace))
            .await?;
        Ok(self
            .patient_understanding_internal(practice, workspace)
            .await?)
    }

    /// Reads the append-only revision history for `(practice, workspace)` in
    /// ascending revision order after verifying practice patient membership.
    pub async fn patient_understanding_revisions<A: PracticeAuthority + ?Sized>(
        &self,
        practice: &str,
        workspace: &str,
        authority: &A,
    ) -> Result<Vec<UnderstandingRevisionRecord>, UnderstandingError> {
        authority
            .check_practice_patient(&scope_for_membership(practice, workspace))
            .await?;
        let rows = patient_understanding_revision::Entity::find()
            .filter(
                Condition::all()
                    .add(patient_understanding_revision::Column::Practice.eq(practice))
                    .add(patient_understanding_revision::Column::Workspace.eq(workspace)),
            )
            .order_by_asc(patient_understanding_revision::Column::Revision)
            .all(&self.db)
            .await?;
        rows.into_iter()
            .map(decode_revision_record)
            .collect::<Result<Vec<_>, _>>()
            .map_err(Into::into)
    }

    /// Reads the append-only revision history for `(practice, workspace)` annotated
    /// with each revision's [`UnderstandingStanding`] (`Active` vs `EnteredInError`)
    /// after verifying practice patient membership.
    pub async fn patient_understanding_revision_audits<A: PracticeAuthority + ?Sized>(
        &self,
        practice: &str,
        workspace: &str,
        authority: &A,
    ) -> Result<Vec<UnderstandingRevisionAudit>, UnderstandingError> {
        use super::follow_ups::understanding_withdrawal_entity::Column as WC;

        let records = self
            .patient_understanding_revisions(practice, workspace, authority)
            .await?;
        let withdrawals = super::follow_ups::understanding_withdrawal_entity::Entity::find()
            .filter(WC::Practice.eq(practice))
            .filter(WC::Workspace.eq(workspace))
            .all(&self.db)
            .await?;
        let mut by_revision = BTreeMap::new();
        for w in withdrawals {
            if let Ok(rev) = u64::try_from(w.revision) {
                let epoch = u64::try_from(w.withdrawal_epoch).unwrap_or(1);
                by_revision.insert(
                    rev,
                    UnderstandingStanding::EnteredInError {
                        withdrawal_id: w.withdrawal_id,
                        withdrawal_epoch: epoch,
                        reason: w.reason,
                        withdrawn_at: w.withdrawn_at,
                    },
                );
            }
        }
        Ok(records
            .into_iter()
            .map(|record| {
                let standing = by_revision
                    .remove(&record.revision)
                    .unwrap_or(UnderstandingStanding::Active);
                UnderstandingRevisionAudit { record, standing }
            })
            .collect())
    }

    pub(crate) async fn patient_understanding_internal(
        &self,
        practice: &str,
        workspace: &str,
    ) -> Result<Option<PatientUnderstanding>, DbErr> {
        let withheld =
            super::link_correction::load_workspace_withheld_state_in_tx(&self.db, workspace)
                .await?;
        let mut saved =
            patient_understanding::Entity::find_by_id((practice.to_owned(), workspace.to_owned()))
                .one(&self.db)
                .await?
                .map(decode_understanding)
                .transpose()?;
        if let Some(current) = &saved
            && !withheld.is_empty()
            && understanding_cites_withheld(
                &current.summary,
                Some(&current.operating_index),
                Some(&current.origin),
                &withheld,
            )
        {
            saved =
                latest_clean_understanding_in_db(&self.db, practice, workspace, &withheld).await?;
        }
        if let Some(understanding) = saved.as_mut()
            && let Some(boundary) =
                super::follow_ups::load_workspace_withdrawal_boundary(&self.db, practice, workspace)
                    .await?
        {
            let instruction = boundary.instruction_line();
            if !understanding
                .operating_index
                .open_threads
                .iter()
                .any(|t| t == &instruction)
            {
                understanding.operating_index.open_threads.push(instruction);
            }
        }
        Ok(saved)
    }

    /// Whether `work_id` has persisted a [`PatientUnderstanding`] revision for
    /// `(practice, workspace)`.
    pub async fn has_saved_understanding_for_work(
        &self,
        practice: &str,
        workspace: &str,
        work_id: &str,
    ) -> Result<bool, DbErr> {
        let rows = patient_understanding_revision::Entity::find()
            .filter(
                Condition::all()
                    .add(patient_understanding_revision::Column::Practice.eq(practice))
                    .add(patient_understanding_revision::Column::Workspace.eq(workspace)),
            )
            .all(&self.db)
            .await?;
        for row in rows {
            let origin: RevisionOrigin = serde_json::from_value(row.origin).map_err(
                conversion_error("patient_understanding_revision.origin", "RevisionOrigin"),
            )?;
            if origin.work_id() == work_id {
                return Ok(true);
            }
        }
        Ok(false)
    }

    /// Reads the physician-facing [`PatientSummaryView`] for `workspace` after
    /// enforcing panel authorization (`Membership::Allowed`).
    ///
    /// Selects only physician-facing columns and never loads [`OperatingIndex`].
    pub async fn patient_summary_for_physician(
        &self,
        auth: &dyn AuthStore,
        identity: &PhysicianIdentity,
        workspace: &str,
    ) -> Result<Option<PatientSummaryView>, PhysicianSummaryAccessError> {
        let membership = auth.membership(identity, workspace).await;
        if membership != Membership::Allowed {
            return Err(PhysicianSummaryAccessError::Unauthorized {
                practice_id: identity.practice_id.clone(),
                physician_id: identity.physician_id.clone(),
                workspace: workspace.to_owned(),
                membership,
            });
        }
        use patient_understanding::Column as C;
        let tuple = patient_understanding::Entity::find_by_id((
            identity.practice_id.clone(),
            workspace.to_owned(),
        ))
        .select_only()
        .columns([
            C::Practice,
            C::Workspace,
            C::Revision,
            C::SourceVersion,
            C::EvidenceScope,
            C::Summary,
            C::UpdatedAt,
        ])
        .into_tuple::<SummaryProjectionTuple>()
        .one(&self.db)
        .await?;
        let view = tuple.map(decode_summary_projection).transpose()?;
        let withheld =
            super::link_correction::load_workspace_withheld_state_in_tx(&self.db, workspace)
                .await?;
        if view.is_some() && !withheld.is_empty() {
            let clean = latest_clean_understanding_in_db(
                &self.db,
                &identity.practice_id,
                workspace,
                &withheld,
            )
            .await?;
            return Ok(clean.map(|u| PatientSummaryView {
                practice: u.practice,
                workspace: u.workspace,
                revision: u.revision,
                source_version: u.source_version,
                evidence_scope: u.evidence_scope,
                summary: u.summary,
                updated_at: u.updated_at,
            }));
        }
        Ok(view)
    }

    /// Batch physician-facing summary projection keyed by `workspace` for the
    /// patient directory. Selects only summary columns and never loads
    /// [`OperatingIndex`].
    pub async fn patient_summaries_for_workspaces(
        &self,
        workspaces: &[String],
    ) -> Result<BTreeMap<String, PatientSummaryView>, DbErr> {
        if workspaces.is_empty() {
            return Ok(BTreeMap::new());
        }
        use patient_understanding::Column as C;
        let tuples = patient_understanding::Entity::find()
            .select_only()
            .columns([
                C::Practice,
                C::Workspace,
                C::Revision,
                C::SourceVersion,
                C::EvidenceScope,
                C::Summary,
                C::UpdatedAt,
            ])
            .filter(C::Workspace.is_in(workspaces.iter().cloned()))
            .order_by_asc(C::UpdatedAt)
            .into_tuple::<SummaryProjectionTuple>()
            .all(&self.db)
            .await?;
        let mut by_workspace = BTreeMap::new();
        for tuple in tuples {
            let view = decode_summary_projection(tuple)?;
            let withheld = super::link_correction::load_workspace_withheld_state_in_tx(
                &self.db,
                &view.workspace,
            )
            .await?;
            if !withheld.is_empty() {
                if let Some(clean) = latest_clean_understanding_in_db(
                    &self.db,
                    &view.practice,
                    &view.workspace,
                    &withheld,
                )
                .await?
                {
                    by_workspace.insert(
                        clean.workspace.clone(),
                        PatientSummaryView {
                            practice: clean.practice,
                            workspace: clean.workspace,
                            revision: clean.revision,
                            source_version: clean.source_version,
                            evidence_scope: clean.evidence_scope,
                            summary: clean.summary,
                            updated_at: clean.updated_at,
                        },
                    );
                }
                continue;
            }
            by_workspace.insert(view.workspace.clone(), view);
        }
        Ok(by_workspace)
    }

    /// Batch query returning the latest active (including pending retries) or
    /// exhausted (`WorkStatus::Failed`) patient understanding run per workspace.
    pub async fn active_or_failed_understanding_runs(
        &self,
        workspaces: &[String],
    ) -> Result<BTreeMap<String, PatientUnderstandingRunStatus>, DbErr> {
        if workspaces.is_empty() {
            return Ok(BTreeMap::new());
        }
        let rows = practice_job::Entity::find()
            .filter(
                Condition::all()
                    .add(practice_job::Column::Workspace.is_in(workspaces.iter().cloned()))
                    .add(practice_job::Column::Kind.eq(PracticeJobKind::InitialUnderstanding)),
            )
            .order_by_asc(practice_job::Column::UpdatedAt)
            .order_by_asc(practice_job::Column::SourcesVersion)
            .order_by_asc(practice_job::Column::JobId)
            .all(&self.db)
            .await?;
        let mut latest_by_workspace: BTreeMap<String, practice_job::Model> = BTreeMap::new();
        for row in rows {
            latest_by_workspace.insert(row.workspace.clone(), row);
        }
        let mut out = BTreeMap::new();
        for (workspace, row) in latest_by_workspace {
            let state = match row.status {
                WorkStatus::Pending | WorkStatus::Active => UnderstandingRunState::Running,
                WorkStatus::Failed => UnderstandingRunState::Failed,
                WorkStatus::Completed | WorkStatus::Held | WorkStatus::Cancelled => continue,
            };
            let assignment: Assignment = serde_json::from_value(row.assignment)
                .map_err(conversion_error("practice_job.assignment", "Assignment"))?;
            let mode = assignment
                .understanding
                .map(|ctx| ctx.mode)
                .unwrap_or(UnderstandingMode::Initial);
            out.insert(
                workspace.clone(),
                PatientUnderstandingRunStatus {
                    workspace,
                    job_id: row.job_id,
                    attempt_id: None,
                    state,
                    mode,
                    started_at: row.created_at,
                },
            );
        }
        Ok(out)
    }

    /// Formats the workspace's saved [`PatientUnderstanding`] as model-clean
    /// starting context for Coda's internal runs (chat or background), keeping
    /// the operating index strictly private from physician-visible output.
    pub async fn understanding_prompt_context(
        &self,
        practice: &str,
        workspace: &str,
    ) -> Result<Option<String>, DbErr> {
        let memory_standing = self.active_memory_standing(practice, workspace).await?;
        let saved = self
            .patient_understanding_internal(practice, workspace)
            .await?;
        let boundary =
            super::follow_ups::load_workspace_withdrawal_boundary(&self.db, practice, workspace)
                .await?;
        if saved.is_none()
            && boundary.is_none()
            && matches!(
                &memory_standing,
                super::chat_operations::PatientMemoryStanding::Available { entries } if entries.is_empty()
            )
        {
            return Ok(None);
        }
        let mut payload = json!({
            "kind": "patient_understanding_context",
            "revision": saved.as_ref().map_or(0, |u| u.revision),
            "source_version": saved.as_ref().map_or(0, |u| u.source_version),
            "evidence_scope": saved.as_ref().map(|u| &u.evidence_scope),
            "private_operating_index": saved.as_ref().map(|u| &u.operating_index),
            "patient_summary": saved.as_ref().map(|u| json!({
                "narrative": u.summary.narrative,
                "material_gaps": u.summary.material_gaps,
                "scope_note": u.summary.scope_note,
                "citations": u.summary.citations.iter().map(|c| json!({
                    "candidate_id": c.reference.candidate_id,
                    "revision": c.reference.revision,
                    "source_id": c.source_id,
                    "as_of": c.as_of,
                })).collect::<Vec<_>>(),
            })),
        });
        attach_memory_standing(&mut payload, &memory_standing);
        if let Some(boundary_val) = &boundary
            && let Some(map) = payload.as_object_mut()
        {
            map.insert(
                "withdrawal_boundary".into(),
                serde_json::to_value(boundary_val).expect("withdrawal boundary serializes"),
            );
        }
        let encoded =
            serde_json::to_string(&payload).expect("patient understanding context serializes");
        Ok(Some(format!(
            "Saved patient understanding (private_operating_index is Coda's internal working memory, \
             never medication truth, and must never be disclosed or quoted as internal state to the physician; \
             verify active medications and changed records via read_patient_sources, and use \
             save_patient_understanding when the physician corrects patient memory):\n{encoded}"
        )))
    }

    /// Builds a physician chat [`RequestRecipe`] enriched with patient
    /// understanding tools (`read_patient_understanding`,
    /// `save_patient_understanding`) and any saved patient understanding context
    /// for `(practice, workspace)`.
    pub async fn chat_recipe_with_understanding(
        &self,
        chat_binding: &ModelBinding,
        base_instructions: &str,
        skills: &PhysicianSkills,
        practice: &str,
        workspace: &str,
    ) -> Result<RequestRecipe, DbErr> {
        let mut combined = base_instructions.to_owned();
        if let Some(context) = self
            .understanding_prompt_context(practice, workspace)
            .await?
        {
            combined.push_str("\n\n");
            combined.push_str(&context);
        }
        let mut built = crate::physician_chat::recipe(chat_binding, &combined, skills);
        let existing_names: BTreeSet<String> = built
            .tools
            .iter()
            .filter_map(|tool| tool.get("name").and_then(Value::as_str).map(str::to_owned))
            .collect();
        for tool in crate::initial_understanding::understanding_tools() {
            if !existing_names.contains(&tool.name) {
                built.tools.push(tool.function());
            }
        }
        Ok(built)
    }

    /// Executes `read_patient_understanding` or `save_patient_understanding`
    /// for a claimed practice job after verifying practice patient membership.
    pub async fn execute_practice_understanding_tool<A: PracticeAuthority + ?Sized>(
        &self,
        claim: &PracticeJobClaim,
        authority: &A,
        call: &Invocation,
        arguments: &Value,
    ) -> Result<ToolOutcome, engine::ProductError> {
        authority
            .check_practice_patient(&claim.scope)
            .await
            .map_err(|err| engine::ProductError::from(Box::new(err) as engine::Cause))?;
        self.execute_practice_understanding_tool_fenced(claim, call, arguments)
            .await
    }

    pub(crate) async fn execute_practice_understanding_tool_fenced(
        &self,
        claim: &PracticeJobClaim,
        call: &Invocation,
        arguments: &Value,
    ) -> Result<ToolOutcome, engine::ProductError> {
        self.assert_practice_job_current(claim)
            .await
            .map_err(super::persistence_failure)?;

        match call.name.as_str() {
            READ_UNDERSTANDING_TOOL => {
                if let Err(msg) = validate_empty_read_args(arguments) {
                    return Ok(ToolOutcome::refused(msg));
                }
                let saved = self
                    .patient_understanding_internal(&claim.scope.practice, &claim.scope.workspace)
                    .await
                    .map_err(super::persistence_failure)?;
                let memory_standing = self
                    .active_memory_standing(&claim.scope.practice, &claim.scope.workspace)
                    .await
                    .map_err(super::persistence_failure)?;
                let boundary = super::follow_ups::load_workspace_withdrawal_boundary(
                    &self.db,
                    &claim.scope.practice,
                    &claim.scope.workspace,
                )
                .await
                .map_err(super::persistence_failure)?;
                Ok(ToolOutcome::Returned(render_read_understanding(
                    &claim.scope.workspace,
                    saved.as_ref(),
                    &memory_standing,
                    boundary.as_ref(),
                )))
            }
            SAVE_UNDERSTANDING_TOOL => {
                let args = match SaveUnderstandingArgs::parse(arguments) {
                    Ok(args) => args,
                    Err(err) => return Ok(SaveRefusal::InvalidArgs(err).into_outcome()),
                };
                let (job, ordered, _) = self
                    .practice_job_snapshot(&claim.scope)
                    .await
                    .map_err(super::persistence_failure)?;
                let read_in_work = crate::physician_sources::delivered_exact_revisions(
                    ordered.iter(),
                    &job.message_sources(),
                )?;
                let mode = job
                    .assignment
                    .understanding
                    .as_ref()
                    .map(|ctx| ctx.mode)
                    .unwrap_or(UnderstandingMode::Initial);
                let idempotency_key = practice_job_save_idempotency_key(&job.work, &call.id);
                let origin = RevisionOrigin::PracticeRun {
                    job_id: claim.scope.job_id.clone(),
                    work_id: job.work,
                    attempt_id: claim.attempt.clone(),
                    mode,
                };
                self.save_understanding_transactional(
                    &claim.scope.practice,
                    &claim.scope.workspace,
                    &idempotency_key,
                    &args,
                    &read_in_work,
                    origin,
                )
                .await
            }
            other => Ok(ToolOutcome::refused(format!(
                "unknown patient understanding tool {other}"
            ))),
        }
    }

    /// Reconciles an already-committed `save_patient_understanding` receipt
    /// for a claimed practice job when an earlier attempt crashed between
    /// `ToolStarted` and `ToolReturned`. Never mutates state.
    pub(crate) async fn saved_practice_understanding_outcome(
        &self,
        claim: &PracticeJobClaim,
        call: &Invocation,
    ) -> Result<Option<ToolOutcome>, engine::ProductError> {
        if call.name != SAVE_UNDERSTANDING_TOOL {
            return Ok(None);
        }
        self.assert_practice_job_current(claim)
            .await
            .map_err(super::persistence_failure)?;
        let (job, _, _) = self
            .practice_job_snapshot(&claim.scope)
            .await
            .map_err(super::persistence_failure)?;
        let idempotency_key = practice_job_save_idempotency_key(&job.work, &call.id);
        self.saved_understanding_outcome_by_key(
            &claim.scope.practice,
            &claim.scope.workspace,
            &job.work,
            &idempotency_key,
        )
        .await
    }

    /// Executes `read_patient_understanding` or `save_patient_understanding`
    /// inside an authorized [`super::PhysicianHost`] chat run.
    pub async fn execute_chat_understanding_tool(
        &self,
        claim: &Claim,
        call: &Invocation,
        arguments: &Value,
    ) -> Result<ToolOutcome, engine::ProductError> {
        self.assert_current(claim)
            .await
            .map_err(super::persistence_failure)?;
        match call.name.as_str() {
            READ_UNDERSTANDING_TOOL => {
                if let Err(msg) = validate_empty_read_args(arguments) {
                    return Ok(ToolOutcome::refused(msg));
                }
                let saved = self
                    .patient_understanding_internal(&claim.scope.practice, &claim.scope.workspace)
                    .await
                    .map_err(super::persistence_failure)?;
                let memory_standing = self
                    .active_memory_standing(&claim.scope.practice, &claim.scope.workspace)
                    .await
                    .map_err(super::persistence_failure)?;
                let boundary = super::follow_ups::load_workspace_withdrawal_boundary(
                    &self.db,
                    &claim.scope.practice,
                    &claim.scope.workspace,
                )
                .await
                .map_err(super::persistence_failure)?;
                Ok(ToolOutcome::Returned(render_read_understanding(
                    &claim.scope.workspace,
                    saved.as_ref(),
                    &memory_standing,
                    boundary.as_ref(),
                )))
            }
            SAVE_UNDERSTANDING_TOOL => {
                let args = match SaveUnderstandingArgs::parse(arguments) {
                    Ok(args) => args,
                    Err(err) => return Ok(SaveRefusal::InvalidArgs(err).into_outcome()),
                };
                let (state, records) = self
                    .work_snapshot(&claim.scope)
                    .await
                    .map_err(super::persistence_failure)?;
                let read_in_work = crate::physician_sources::delivered_exact_revisions(
                    records.iter(),
                    &state.message_sources(),
                )?;
                let idempotency_key = chat_save_idempotency_key(&state.work, &call.id);
                let origin = RevisionOrigin::PhysicianConversation {
                    physician_id: claim.scope.physician.clone(),
                    chat_id: claim.scope.chat.clone(),
                    work_id: state.work,
                    attempt_id: claim.attempt.clone(),
                };
                self.save_understanding_transactional(
                    &claim.scope.practice,
                    &claim.scope.workspace,
                    &idempotency_key,
                    &args,
                    &read_in_work,
                    origin,
                )
                .await
            }
            other => Ok(ToolOutcome::refused(format!(
                "unknown patient understanding tool {other}"
            ))),
        }
    }

    /// Reconciles an already-committed `save_patient_understanding` receipt
    /// for a claimed chat run when an earlier attempt crashed between
    /// `ToolStarted` and `ToolReturned`. Never mutates state.
    pub(crate) async fn saved_chat_understanding_outcome(
        &self,
        claim: &Claim,
        call: &Invocation,
    ) -> Result<Option<ToolOutcome>, engine::ProductError> {
        if call.name != SAVE_UNDERSTANDING_TOOL {
            return Ok(None);
        }
        self.assert_current(claim)
            .await
            .map_err(super::persistence_failure)?;
        let (state, _) = self
            .work_snapshot(&claim.scope)
            .await
            .map_err(super::persistence_failure)?;
        let idempotency_key = chat_save_idempotency_key(&state.work, &call.id);
        self.saved_understanding_outcome_by_key(
            &claim.scope.practice,
            &claim.scope.workspace,
            &state.work,
            &idempotency_key,
        )
        .await
    }

    async fn saved_understanding_outcome_by_key(
        &self,
        practice: &str,
        workspace: &str,
        work_id: &str,
        idempotency_key: &str,
    ) -> Result<Option<ToolOutcome>, engine::ProductError> {
        let Some(existing_rev) = patient_understanding_revision::Entity::find()
            .filter(
                Condition::all()
                    .add(patient_understanding_revision::Column::Practice.eq(practice))
                    .add(patient_understanding_revision::Column::Workspace.eq(workspace))
                    .add(
                        patient_understanding_revision::Column::IdempotencyKey.eq(idempotency_key),
                    ),
            )
            .one(&self.db)
            .await
            .map_err(super::persistence_failure)?
        else {
            return Ok(None);
        };
        match super::follow_ups::check_withdrawal_fence_in_tx(
            &self.db,
            practice,
            workspace,
            &[],
            None,
            Some(work_id),
        )
        .await
        {
            Ok(()) => {}
            Err(super::follow_ups::WithdrawalFenceCheckError::Violation(violation)) => {
                return Ok(Some(ToolOutcome::refused(
                    json!({
                        "outcome": "withdrawn_fence",
                        "message": violation.to_string(),
                    })
                    .to_string(),
                )));
            }
            Err(super::follow_ups::WithdrawalFenceCheckError::Database(db_err)) => {
                return Err(super::persistence_failure(db_err));
            }
        }
        if let Some(withdrawn) =
            super::follow_ups::understanding_withdrawal_entity::Entity::find_by_id((
                practice.to_owned(),
                workspace.to_owned(),
                existing_rev.revision,
            ))
            .one(&self.db)
            .await
            .map_err(super::persistence_failure)?
        {
            return Ok(Some(ToolOutcome::refused(
                json!({
                    "outcome": "withdrawn_fence",
                    "message": format!(
                        "understanding revision {} was withdrawn as wrong-patient in `{}` and cannot be restored",
                        existing_rev.revision, withdrawn.withdrawal_id
                    ),
                })
                .to_string(),
            )));
        }
        let decoded = decode_revision_record(existing_rev).map_err(super::persistence_failure)?;
        Ok(Some(render_saved_understanding_receipt(
            workspace,
            decoded.revision,
            decoded.source_version,
            decoded.summary.citations.len(),
            &decoded.summary.material_gaps,
            true,
        )))
    }

    async fn save_understanding_transactional(
        &self,
        practice: &str,
        workspace: &str,
        idempotency_key: &str,
        args: &SaveUnderstandingArgs,
        read_in_work: &BTreeSet<(String, String)>,
        origin: RevisionOrigin,
    ) -> Result<ToolOutcome, engine::ProductError> {
        let pulls_map = self
            .source_pulls(&[workspace.to_owned()])
            .await
            .map_err(super::persistence_failure)?;
        let pulls = pulls_map.get(workspace).cloned().unwrap_or_default();

        let mut cited_set = BTreeSet::new();
        for anchor in &args.operating_index.anchors {
            cited_set.insert(anchor.reference.candidate_id.clone());
        }
        for claim in &args.claims {
            for r in &claim.citations {
                cited_set.insert(r.candidate_id.clone());
            }
        }
        let cited_candidates: Vec<String> = cited_set.into_iter().collect();

        let tx = self.db.begin().await.map_err(super::persistence_failure)?;

        match super::follow_ups::check_withdrawal_fence_in_tx(
            &tx,
            practice,
            workspace,
            &cited_candidates,
            None,
            Some(origin.work_id()),
        )
        .await
        {
            Ok(()) => {}
            Err(super::follow_ups::WithdrawalFenceCheckError::Violation(violation)) => {
                return Ok(ToolOutcome::refused(
                    json!({
                        "outcome": "withdrawn_fence",
                        "message": violation.to_string(),
                    })
                    .to_string(),
                ));
            }
            Err(super::follow_ups::WithdrawalFenceCheckError::Database(db_err)) => {
                return Err(super::persistence_failure(db_err));
            }
        }

        if let Some(existing_rev) = patient_understanding_revision::Entity::find()
            .filter(
                Condition::all()
                    .add(patient_understanding_revision::Column::Practice.eq(practice))
                    .add(patient_understanding_revision::Column::Workspace.eq(workspace))
                    .add(
                        patient_understanding_revision::Column::IdempotencyKey.eq(idempotency_key),
                    ),
            )
            .one(&tx)
            .await
            .map_err(super::persistence_failure)?
        {
            if let Some(withdrawn) =
                super::follow_ups::understanding_withdrawal_entity::Entity::find_by_id((
                    practice.to_owned(),
                    workspace.to_owned(),
                    existing_rev.revision,
                ))
                .one(&tx)
                .await
                .map_err(super::persistence_failure)?
            {
                return Ok(ToolOutcome::refused(
                    json!({
                        "outcome": "withdrawn_fence",
                        "message": format!(
                            "understanding revision {} was withdrawn as wrong-patient in `{}` and cannot be restored",
                            existing_rev.revision, withdrawn.withdrawal_id
                        ),
                    })
                    .to_string(),
                ));
            }
            let decoded =
                decode_revision_record(existing_rev).map_err(super::persistence_failure)?;
            tx.commit().await.map_err(super::persistence_failure)?;
            return Ok(render_saved_understanding_receipt(
                workspace,
                decoded.revision,
                decoded.source_version,
                decoded.summary.citations.len(),
                &decoded.summary.material_gaps,
                true,
            ));
        }

        let current_row =
            patient_understanding::Entity::find_by_id((practice.to_owned(), workspace.to_owned()))
                .lock_exclusive()
                .one(&tx)
                .await
                .map_err(super::persistence_failure)?
                .map(decode_understanding)
                .transpose()
                .map_err(super::persistence_failure)?;

        let current_revision = current_row.as_ref().map_or(0, |saved| saved.revision);
        if let Some(expected) = args.expected_revision
            && expected != current_revision
        {
            return Ok(SaveRefusal::RevisionConflict {
                expected,
                actual: current_revision,
            }
            .into_outcome());
        }

        let max_historical_revision = patient_understanding_revision::Entity::find()
            .filter(
                Condition::all()
                    .add(patient_understanding_revision::Column::Practice.eq(practice))
                    .add(patient_understanding_revision::Column::Workspace.eq(workspace)),
            )
            .order_by_desc(patient_understanding_revision::Column::Revision)
            .one(&tx)
            .await
            .map_err(super::persistence_failure)?
            .and_then(|row| u64::try_from(row.revision).ok())
            .unwrap_or(0);

        let observation = super::sources::observed(&tx, workspace)
            .await
            .map_err(super::persistence_failure)?;
        let briefs = connected_briefs(&pulls);
        let evidence_scope = EvidenceScope::classify(
            &briefs,
            observation.sources.iter().map(|s| s.source_id.as_str()),
        );

        let carried_forward_refs: BTreeSet<(String, String)> = current_row
            .as_ref()
            .map(known_understanding_pairs)
            .unwrap_or_default();

        let summary = match build_verified_summary(
            workspace,
            args,
            &evidence_scope,
            &observation.sources,
            &pulls,
            read_in_work,
            &carried_forward_refs,
        ) {
            Ok(summary) => summary,
            Err(refusal) => return Ok(refusal.into_outcome()),
        };

        let next_revision = current_revision
            .max(max_historical_revision)
            .checked_add(1)
            .ok_or_else(|| error("patient understanding revision overflow"))
            .map_err(super::persistence_failure)?;
        let next_revision_i64 = i64::try_from(next_revision)
            .map_err(conversion_error("next_revision", "i64"))
            .map_err(super::persistence_failure)?;
        let source_version_i64 = i64::try_from(observation.version)
            .map_err(conversion_error("source_version", "i64"))
            .map_err(super::persistence_failure)?;
        let saved_at = now(&tx).await.map_err(super::persistence_failure)?;

        let scope_json = serde_json::to_value(&evidence_scope)
            .map_err(conversion_error("EvidenceScope", "evidence_scope"))
            .map_err(super::persistence_failure)?;
        let index_json = serde_json::to_value(&args.operating_index)
            .map_err(conversion_error("OperatingIndex", "operating_index"))
            .map_err(super::persistence_failure)?;
        let summary_json = serde_json::to_value(&summary)
            .map_err(conversion_error("PatientSummary", "summary"))
            .map_err(super::persistence_failure)?;
        let origin_json = serde_json::to_value(&origin)
            .map_err(conversion_error("RevisionOrigin", "origin"))
            .map_err(super::persistence_failure)?;

        let active = patient_understanding::ActiveModel {
            practice: Set(practice.to_owned()),
            workspace: Set(workspace.to_owned()),
            revision: Set(next_revision_i64),
            source_version: Set(source_version_i64),
            evidence_scope: Set(scope_json.clone()),
            operating_index: Set(index_json.clone()),
            summary: Set(summary_json.clone()),
            origin: Set(origin_json.clone()),
            updated_at: Set(saved_at),
        };
        if current_row.is_some() {
            active
                .update(&tx)
                .await
                .map_err(super::persistence_failure)?;
        } else {
            active
                .insert(&tx)
                .await
                .map_err(super::persistence_failure)?;
        }

        patient_understanding_revision::Entity::insert(
            patient_understanding_revision::ActiveModel {
                practice: Set(practice.to_owned()),
                workspace: Set(workspace.to_owned()),
                revision: Set(next_revision_i64),
                idempotency_key: Set(idempotency_key.to_owned()),
                source_version: Set(source_version_i64),
                evidence_scope: Set(scope_json),
                operating_index: Set(index_json),
                summary: Set(summary_json),
                origin: Set(origin_json),
                saved_at: Set(saved_at),
            },
        )
        .exec_without_returning(&tx)
        .await
        .map_err(super::persistence_failure)?;

        tx.commit().await.map_err(super::persistence_failure)?;

        Ok(render_saved_understanding_receipt(
            workspace,
            next_revision,
            observation.version,
            summary.citations.len(),
            &summary.material_gaps,
            false,
        ))
    }
}

fn practice_job_save_idempotency_key(work_id: &str, call_id: &str) -> String {
    format!("practice-job:{work_id}:{}", call_id.trim())
}

fn chat_save_idempotency_key(work_id: &str, call_id: &str) -> String {
    format!("chat:{work_id}:{}", call_id.trim())
}

fn render_saved_understanding_receipt(
    workspace: &str,
    revision: u64,
    source_version: u64,
    citation_count: usize,
    material_gaps: &[String],
    duplicate: bool,
) -> ToolOutcome {
    ToolOutcome::Returned(
        json!({
            "outcome": "saved",
            "duplicate": duplicate,
            "workspace": workspace,
            "revision": revision,
            "source_version": source_version,
            "citation_count": citation_count,
            "material_gaps": material_gaps,
        })
        .to_string(),
    )
}

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct EmptyReadArgs {}

fn validate_empty_read_args(arguments: &Value) -> Result<(), String> {
    if arguments.is_null() {
        return Ok(());
    }
    serde_json::from_value::<EmptyReadArgs>(arguments.clone())
        .map(|_| ())
        .map_err(|err| format!("invalid read_patient_understanding arguments: {err}"))
}

fn attach_memory_standing(
    payload: &mut Value,
    standing: &super::chat_operations::PatientMemoryStanding,
) {
    let Some(map) = payload.as_object_mut() else {
        return;
    };
    match standing {
        super::chat_operations::PatientMemoryStanding::Available { entries } => {
            map.insert(
                "physician_memory_entries".into(),
                serde_json::to_value(entries).expect("memory entries serialize"),
            );
        }
        super::chat_operations::PatientMemoryStanding::Unavailable { reason } => {
            map.insert(
                "physician_memory_standing".into(),
                Value::String("unavailable".into()),
            );
            map.insert(
                "physician_memory_unavailable_reason".into(),
                Value::String(reason.clone()),
            );
        }
    }
}

fn render_read_understanding(
    workspace: &str,
    saved: Option<&PatientUnderstanding>,
    memory_standing: &super::chat_operations::PatientMemoryStanding,
    boundary: Option<&super::follow_ups::WorkspaceWithdrawalBoundary>,
) -> String {
    let mut payload = match saved {
        None => json!({
            "status": "uninitialized",
            "workspace": workspace,
            "revision": 0,
        }),
        Some(understanding) => json!({
            "status": "available",
            "workspace": workspace,
            "revision": understanding.revision,
            "source_version": understanding.source_version,
            "evidence_scope": understanding.evidence_scope,
            "operating_index": understanding.operating_index,
            "summary": {
                "narrative": understanding.summary.narrative,
                "claims": understanding.summary.claims,
                "material_gaps": understanding.summary.material_gaps,
                "scope_note": understanding.summary.scope_note,
                "citations": understanding.summary.citations,
            },
            "updated_at": understanding.updated_at,
        }),
    };
    attach_memory_standing(&mut payload, memory_standing);
    if let Some(boundary_val) = boundary
        && let Some(map) = payload.as_object_mut()
    {
        map.insert(
            "withdrawal_boundary".into(),
            serde_json::to_value(boundary_val).expect("withdrawal boundary serializes"),
        );
    }
    payload.to_string()
}
