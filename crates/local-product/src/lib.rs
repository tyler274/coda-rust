//! Physician model execution and durable product state.
//! Postgres owns chat, reply revisions, durable patient memory, and versioned chart
//! source reads; the worker runs chat and reply revisions through the shared harness
//! using the selected local subscription or API transport.
//! Book: `book/src/physician-site.md`.
#![deny(missing_docs)]
#![allow(
    clippy::map_err_ignore,
    reason = "converting to errors that keep their cause"
)]

use std::collections::{HashMap, HashSet};

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

pub mod chatgpt;
/// Hosted physician chat instructions and the `skill_view` tool.
pub mod physician_chat;
pub mod physician_sources;
/// Physician-owned durable conversations and execution authority.
pub mod runtime;

/// How this process runs a model. A process without a selection does not run models.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModelExecution {
    /// No local inference has been selected.
    Disabled,
    /// Direct Responses calls authorized on a local developer's ChatGPT plan.
    Subscription,
    /// OpenAI Responses. Requires an authorized key.
    Api,
}

/// One role's model, effort, and adapter.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModelBinding {
    /// The provider model name.
    pub model: String,
    /// Product-selected working window, measured with the harness estimate.
    pub context_window_tokens: u64,
    /// Provider input ceiling, checked against the estimated dispatch size.
    pub max_input_tokens: u64,
    /// The reasoning effort.
    pub effort: String,
    /// The adapter identity.
    pub adapter: String,
}

/// The jobs the local product binds.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModelRoles {
    /// Physician chat.
    pub chat: ModelBinding,
    /// Background work.
    pub background: ModelBinding,
}

/// Selects the explicitly configured local inference mode.
pub fn model_execution(value: Option<&str>) -> Result<ModelExecution, String> {
    match value {
        None => Ok(ModelExecution::Disabled),
        Some("subscription") => Ok(ModelExecution::Subscription),
        Some("api") => Ok(ModelExecution::Api),
        Some(_) => Err("CODA_LOCAL_MODEL_EXECUTION must be subscription or api".into()),
    }
}

/// The key, after the allow flag. Does not call the provider.
pub fn open_ai_api_key(allow: Option<&str>, key: Option<&str>) -> Result<String, String> {
    if allow != Some("1") {
        return Err("Set CODA_ALLOW_MODEL_CALLS=1 to authorize real model calls".to_owned());
    }
    let key = key.unwrap_or("");
    if key.is_empty() {
        return Err("OPENAI_API_KEY is required".to_owned());
    }
    Ok(key.to_owned())
}

/// Shared validation of the product's configured model roles.
pub use harness::settings::Error as ModelBindingError;

/// The same explicit role settings are read by local and hosted Chords.
pub const MODEL_SETTING_KEYS: [&str; 8] = [
    "CODA_CHAT_MODEL",
    "CODA_CHAT_EFFORT",
    "CODA_CHAT_CONTEXT_WINDOW_TOKENS",
    "CODA_CHAT_INPUT_LIMIT_TOKENS",
    "CODA_BACKGROUND_MODEL",
    "CODA_BACKGROUND_EFFORT",
    "CODA_BACKGROUND_CONTEXT_WINDOW_TOKENS",
    "CODA_BACKGROUND_INPUT_LIMIT_TOKENS",
];

/// Bind each role to its required model, effort, working window and input limit.
pub fn model_roles(
    adapter: &str,
    environment: &[(&str, &str)],
) -> Result<ModelRoles, ModelBindingError> {
    let settings = harness::settings::Reader(|key: &str| {
        environment
            .iter()
            .find(|(name, _)| *name == key)
            .map(|(_, value)| (*value).to_owned())
    });
    let binding = |role: &str| -> Result<ModelBinding, ModelBindingError> {
        let job = settings.job(&format!("CODA_{role}"))?;
        Ok(ModelBinding {
            model: job.model,
            effort: job.effort,
            context_window_tokens: job.context_window_tokens,
            max_input_tokens: job.max_input_tokens,
            adapter: adapter.to_owned(),
        })
    };
    Ok(ModelRoles {
        chat: binding("CHAT")?,
        background: binding("BACKGROUND")?,
    })
}

/// Computes a lowercase hexadecimal SHA-256 digest of `bytes`.
pub fn sha256_hex(bytes: &[u8]) -> String {
    let digest = Sha256::digest(bytes);
    let mut out = String::with_capacity(64);
    for byte in digest {
        use std::fmt::Write as _;
        let _ = write!(&mut out, "{byte:02x}");
    }
    out
}

/// Legacy Duet-shaped input supplied by the physician preview route.
/// Source labels and approval claims in this input are not authenticated.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConsentedDuetItem {
    /// Stable record identifier in Duet.
    pub record_id: String,
    /// Deterministic `sha256` revision cursor for incremental sync.
    pub revision_cursor: String,
    /// Deterministic `sha256` content hash for cross-source deduplication.
    pub content_hash: String,
    /// Canonical source identity (such as `"quest:lipid-2026-10"` or `"duet-note:note-1"`).
    pub source_identity: String,
    /// Optional upstream source label (such as `"Quest"` or `"from Quest"`).
    #[serde(default)]
    pub origin_label: String,
    /// Record category (`"lab"`, `"clinical_note"`, `"medication"`, `"wearable"`, etc.).
    pub category: String,
    /// Short display title.
    pub title: String,
    /// Clinical/person-entered summary text.
    pub summary: String,
    /// ISO-8601 or date label when recorded.
    pub recorded_at: String,
    /// Whether this item originated as a patient-approved short dated Luna note (`ENG-52`).
    #[serde(default)]
    pub patient_approved_note: bool,
}

/// One physician-entered preview record in a patient workspace.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct IngestedDuetRecord {
    /// Stable record identifier in Duet.
    pub record_id: String,
    /// Deterministic `sha256` revision cursor at ingestion time.
    pub revision_cursor: String,
    /// Deterministic `sha256` content hash.
    pub content_hash: String,
    /// Canonical source identity used for cross-source deduplication.
    pub source_identity: String,
    /// Provenance assigned by the ingestion owner, never by the input.
    pub provenance: String,
    /// Record category.
    pub category: String,
    /// Short display title.
    pub title: String,
    /// Summary text.
    pub summary: String,
    /// Recorded timestamp or date string.
    pub recorded_at: String,
    /// Always false for physician-entered preview data.
    pub patient_approved_note: bool,
}

#[derive(Debug, Default, Clone)]
struct WorkspaceDuetIngestion {
    practice_hashes: HashSet<String>,
    practice_identities: HashSet<String>,
    records: Vec<IngestedDuetRecord>,
    last_cursor: Option<String>,
}

/// Per-workspace preview ingestion and cross-source deduplication.
/// The legacy wire shape does not establish a patient-origin feed.
#[derive(Debug, Default, Clone)]
pub struct DuetSourceStore {
    workspaces: HashMap<String, WorkspaceDuetIngestion>,
}

impl DuetSourceStore {
    /// Creates an empty Duet source ingestion store.
    pub fn new() -> Self {
        Self::default()
    }

    /// Registers an existing practice/EHR/Kno2 record's content hash and source identity
    /// so identical records arriving via the patient's Duet are deduplicated.
    pub fn register_practice_source(
        &mut self,
        workspace_id: &str,
        content_hash: &str,
        source_identity: &str,
    ) {
        let entry = self.workspaces.entry(workspace_id.to_owned()).or_default();
        if !content_hash.trim().is_empty() {
            entry.practice_hashes.insert(content_hash.trim().to_owned());
        }
        if !source_identity.trim().is_empty() {
            entry
                .practice_identities
                .insert(source_identity.trim().to_owned());
        }
    }

    /// Ingests physician-entered preview items under the existing consent gate.
    /// Caller-supplied labels and approval flags cannot establish patient origin.
    /// Deduplication still uses `content_hash` and `source_identity`.
    pub fn ingest_preview_items(
        &mut self,
        workspace_id: &str,
        sharing_consented: bool,
        items: &[ConsentedDuetItem],
    ) -> Result<usize, String> {
        if !sharing_consented {
            return Err("patient sharing consent is not active".to_owned());
        }
        let entry = self.workspaces.entry(workspace_id.to_owned()).or_default();
        let mut added_or_updated = 0usize;
        for item in items {
            entry.last_cursor = Some(item.revision_cursor.clone());
            // Skip if already present from a practice/Kno2 source by content hash or identity.
            if entry.practice_hashes.contains(&item.content_hash)
                || entry.practice_identities.contains(&item.source_identity)
            {
                continue;
            }
            let provenance = "Physician-entered preview".to_owned();
            // If the same Duet record_id is revised with a new revision_cursor, update it in place.
            if let Some(existing) = entry
                .records
                .iter_mut()
                .find(|record| record.record_id == item.record_id)
            {
                if existing.revision_cursor != item.revision_cursor
                    && existing.content_hash != item.content_hash
                {
                    existing.revision_cursor = item.revision_cursor.clone();
                    existing.content_hash = item.content_hash.clone();
                    existing.source_identity = item.source_identity.clone();
                    existing.provenance = provenance;
                    existing.category = item.category.clone();
                    existing.title = item.title.clone();
                    existing.summary = item.summary.clone();
                    existing.recorded_at = item.recorded_at.clone();
                    existing.patient_approved_note = false;
                    added_or_updated += 1;
                }
                continue;
            }
            // Deduplicate against already-ingested Duet records by content_hash or source_identity.
            if entry.records.iter().any(|record| {
                record.content_hash == item.content_hash
                    || record.source_identity == item.source_identity
            }) {
                continue;
            }
            entry.records.push(IngestedDuetRecord {
                record_id: item.record_id.clone(),
                revision_cursor: item.revision_cursor.clone(),
                content_hash: item.content_hash.clone(),
                source_identity: item.source_identity.clone(),
                provenance,
                category: item.category.clone(),
                title: item.title.clone(),
                summary: item.summary.clone(),
                recorded_at: item.recorded_at.clone(),
                patient_approved_note: false,
            });
            added_or_updated += 1;
        }
        Ok(added_or_updated)
    }

    /// Returns the ingested Duet records for `workspace_id`.
    pub fn records(&self, workspace_id: &str) -> Vec<IngestedDuetRecord> {
        self.workspaces
            .get(workspace_id)
            .map(|entry| entry.records.clone())
            .unwrap_or_default()
    }

    /// Returns the latest `sha256` revision cursor synced for `workspace_id`.
    pub fn last_cursor(&self, workspace_id: &str) -> Option<String> {
        self.workspaces
            .get(workspace_id)
            .and_then(|entry| entry.last_cursor.clone())
    }
}

/// One item in a physician-authored or approved patient plan (`ENG-53` / `ENG-54`).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlanItemInput {
    /// Stable item identifier within the plan.
    pub id: String,
    /// Item heading (such as `"Lipid re-check"`).
    pub title: String,
    /// Exact clinical guidance text visible to the patient.
    pub body: String,
}

/// One item in the physician's plan draft with exact-text approval tracking (`ENG-53`).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DoctorPlanItem {
    /// Stable item identifier within the plan.
    pub id: String,
    /// Item heading.
    pub title: String,
    /// Current drafted body text.
    pub body: String,
    /// Previously approved exact title, if this item was approved in an earlier version.
    pub approved_title: Option<String>,
    /// Previously approved exact body text, if this item was approved in an earlier version.
    pub approved_body: Option<String>,
    /// Whether this item is new or edited since the last approval.
    pub needs_approval: bool,
}

/// One batched item diff approved by the physician (`ENG-53`).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PlanDiffEntry {
    /// Stable item identifier.
    pub item_id: String,
    /// Approved item title.
    pub title: String,
    /// Previous approved body text (`None` when the item was newly added).
    pub previous_body: Option<String>,
    /// Newly approved exact body text.
    pub approved_body: String,
}

/// The physician-approved patient plan displayed read-only in Duet as `"your doctor's plan"` (`ENG-54`).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ApprovedDoctorPlan {
    /// Patient workspace identifier.
    pub patient_workspace_id: String,
    /// Practice identifier.
    pub practice_id: String,
    /// Approving physician's display name.
    pub physician_name: String,
    /// Patient-facing section label (always `"your doctor's plan"`).
    pub label: String,
    /// Monotonically increasing approved version (`1` for full first version, `2+` for batched diffs).
    pub version: u32,
    /// Timestamp or date string when approved.
    pub approved_at: String,
    /// Full list of exact-text approved items in the plan.
    pub items: Vec<PlanItemInput>,
    /// The exact-text items approved in this version (all items for `v1`, changed items for `v2+`).
    pub last_diff: Vec<PlanDiffEntry>,
}

/// Physician workspace state for drafting and approving a patient plan (`ENG-53`).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DoctorPlanDraft {
    /// Patient workspace identifier.
    pub patient_workspace_id: String,
    /// Practice identifier.
    pub practice_id: String,
    /// Authoring physician's display name.
    pub physician_name: String,
    /// Highest approved version (`0` before the first approval).
    pub approved_version: u32,
    /// Whether the current draft's exact text is approved with zero unapproved edits.
    pub currently_approved: bool,
    /// Current drafted items and their per-item approval state.
    pub items: Vec<DoctorPlanItem>,
    /// Latest approved snapshot delivered to Duet, if any.
    pub latest_approved: Option<ApprovedDoctorPlan>,
}

/// Per-workspace store for physician plan drafting and exact-text approval (`ENG-53`).
#[derive(Debug, Default, Clone)]
pub struct DoctorPlanStore {
    plans: HashMap<String, DoctorPlanDraft>,
}

impl DoctorPlanStore {
    /// Creates an empty doctor's plan store.
    pub fn new() -> Self {
        Self::default()
    }

    /// Saves or edits the drafted plan for `workspace_id`.
    ///
    /// Any edit to an item's `title` or `body` (or adding/removing items) immediately
    /// invalidates prior approval (`currently_approved = false`) and marks only the
    /// added or modified items with `needs_approval = true` so subsequent approval
    /// operates on batched item diffs.
    pub fn save_draft(
        &mut self,
        workspace_id: &str,
        practice_id: &str,
        physician_name: &str,
        items: Vec<PlanItemInput>,
    ) -> Result<DoctorPlanDraft, String> {
        if items.is_empty() {
            return Err("doctor's plan draft must contain at least one item".to_owned());
        }
        for item in &items {
            if item.id.trim().is_empty()
                || item.title.trim().is_empty()
                || item.body.trim().is_empty()
            {
                return Err("every plan item requires a non-empty id, title, and body".to_owned());
            }
        }

        let existing = self.plans.get(workspace_id).cloned();
        let approved_version = existing
            .as_ref()
            .map(|draft| draft.approved_version)
            .unwrap_or(0);
        let latest_approved = existing.and_then(|draft| draft.latest_approved);

        let approved_map: HashMap<String, (String, String)> = latest_approved
            .as_ref()
            .map(|approved| {
                approved
                    .items
                    .iter()
                    .map(|item| (item.id.clone(), (item.title.clone(), item.body.clone())))
                    .collect()
            })
            .unwrap_or_default();

        let mut draft_items = Vec::with_capacity(items.len());
        let mut any_changed = approved_version == 0
            || latest_approved
                .as_ref()
                .is_some_and(|approved| approved.items.len() != items.len());

        for item in items {
            let prev = approved_map.get(&item.id);
            let unchanged = prev.is_some_and(|(prev_title, prev_body)| {
                *prev_title == item.title && *prev_body == item.body
            });
            if !unchanged {
                any_changed = true;
            }
            draft_items.push(DoctorPlanItem {
                id: item.id,
                title: item.title,
                body: item.body,
                approved_title: prev.map(|(t, _)| t.clone()),
                approved_body: prev.map(|(_, b)| b.clone()),
                needs_approval: !unchanged,
            });
        }

        let draft = DoctorPlanDraft {
            patient_workspace_id: workspace_id.to_owned(),
            practice_id: practice_id.to_owned(),
            physician_name: physician_name.to_owned(),
            approved_version,
            currently_approved: !any_changed && approved_version > 0,
            items: draft_items,
            latest_approved,
        };
        self.plans.insert(workspace_id.to_owned(), draft.clone());
        Ok(draft)
    }

    /// Approves the current draft using exact-text verification (`ENG-53`).
    ///
    /// - For the first version (`approved_version == 0`), `approved_items` must contain
    ///   every item in the full plan with its exact `id`, `title`, and `body`.
    /// - For subsequent versions (`approved_version >= 1`), `approved_items` must contain
    ///   the batched item diffs (every item where `needs_approval == true`, or all items)
    ///   with exact matching `id`, `title`, and `body`.
    pub fn approve_exact_text(
        &mut self,
        workspace_id: &str,
        approved_items: &[PlanItemInput],
        approved_at: &str,
    ) -> Result<ApprovedDoctorPlan, String> {
        let draft = self
            .plans
            .get_mut(workspace_id)
            .ok_or_else(|| "no doctor's plan draft exists for this workspace".to_owned())?;

        let provided: HashMap<&str, &PlanItemInput> = approved_items
            .iter()
            .map(|item| (item.id.as_str(), item))
            .collect();

        if draft.approved_version == 0 {
            if approved_items.len() != draft.items.len() {
                return Err(
                    "first version approval requires exact text match for every plan item"
                        .to_owned(),
                );
            }
            for item in &draft.items {
                let Some(candidate) = provided.get(item.id.as_str()) else {
                    return Err(
                        "first version approval requires exact text match for every plan item"
                            .to_owned(),
                    );
                };
                if candidate.title != item.title || candidate.body != item.body {
                    return Err(
                        "first version approval requires exact text match for every plan item"
                            .to_owned(),
                    );
                }
            }
        } else {
            let changed: Vec<&DoctorPlanItem> = draft
                .items
                .iter()
                .filter(|item| item.needs_approval)
                .collect();
            if changed.is_empty() && draft.currently_approved {
                return Err("plan is already approved with no pending edits".to_owned());
            }
            for item in &changed {
                let Some(candidate) = provided.get(item.id.as_str()) else {
                    return Err(
                        "batched diff approval requires exact text match for every changed plan item"
                            .to_owned(),
                    );
                };
                if candidate.title != item.title || candidate.body != item.body {
                    return Err(
                        "batched diff approval requires exact text match for every changed plan item"
                            .to_owned(),
                    );
                }
            }
            // Also verify any unchanged items included in `approved_items` match their exact text.
            for candidate in approved_items {
                let Some(draft_item) = draft.items.iter().find(|i| i.id == candidate.id) else {
                    return Err("batched diff approval includes an unknown plan item id".to_owned());
                };
                if candidate.title != draft_item.title || candidate.body != draft_item.body {
                    return Err(
                        "batched diff approval requires exact text match for every changed plan item"
                            .to_owned(),
                    );
                }
            }
        }

        let next_version = draft.approved_version + 1;
        let mut last_diff = Vec::new();
        for item in &mut draft.items {
            if draft.approved_version == 0 || item.needs_approval {
                last_diff.push(PlanDiffEntry {
                    item_id: item.id.clone(),
                    title: item.title.clone(),
                    previous_body: item.approved_body.clone(),
                    approved_body: item.body.clone(),
                });
            }
            item.approved_title = Some(item.title.clone());
            item.approved_body = Some(item.body.clone());
            item.needs_approval = false;
        }

        let full_items: Vec<PlanItemInput> = draft
            .items
            .iter()
            .map(|item| PlanItemInput {
                id: item.id.clone(),
                title: item.title.clone(),
                body: item.body.clone(),
            })
            .collect();

        let approved = ApprovedDoctorPlan {
            patient_workspace_id: draft.patient_workspace_id.clone(),
            practice_id: draft.practice_id.clone(),
            physician_name: draft.physician_name.clone(),
            label: "your doctor's plan".to_owned(),
            version: next_version,
            approved_at: approved_at.to_owned(),
            items: full_items,
            last_diff,
        };

        draft.approved_version = next_version;
        draft.currently_approved = true;
        draft.latest_approved = Some(approved.clone());
        Ok(approved)
    }

    /// Reads the current plan draft (and latest approved snapshot) for `workspace_id`.
    pub fn draft(&self, workspace_id: &str) -> Option<DoctorPlanDraft> {
        self.plans.get(workspace_id).cloned()
    }

    /// Reads the latest approved plan for `workspace_id`.
    pub fn approved(&self, workspace_id: &str) -> Option<ApprovedDoctorPlan> {
        self.plans
            .get(workspace_id)
            .and_then(|draft| draft.latest_approved.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn api_mode_refuses_without_the_allow_flag_or_key() {
        assert!(open_ai_api_key(None, Some("test-key")).is_err());
        assert!(
            open_ai_api_key(None, Some("test-key"))
                .unwrap_err()
                .contains("CODA_ALLOW_MODEL_CALLS=1")
        );
        assert!(
            open_ai_api_key(Some("1"), None)
                .unwrap_err()
                .contains("OPENAI_API_KEY is required")
        );
        assert_eq!(
            open_ai_api_key(Some("1"), Some("test-key")).unwrap(),
            "test-key"
        );
        assert_eq!(model_execution(None).unwrap(), ModelExecution::Disabled);
        assert_eq!(model_execution(Some("api")).unwrap(), ModelExecution::Api);
        assert_eq!(
            model_execution(Some("subscription")).unwrap(),
            ModelExecution::Subscription
        );
        assert!(model_execution(Some("paid")).is_err());
    }

    #[test]
    fn every_role_requires_a_model_effort_and_both_capacities() {
        let environment = [
            ("CODA_CHAT_MODEL", "chat-model"),
            ("CODA_CHAT_EFFORT", "high"),
            ("CODA_CHAT_CONTEXT_WINDOW_TOKENS", "272000"),
            ("CODA_CHAT_INPUT_LIMIT_TOKENS", "922000"),
            ("CODA_BACKGROUND_MODEL", "background-model"),
            ("CODA_BACKGROUND_EFFORT", "max"),
            ("CODA_BACKGROUND_CONTEXT_WINDOW_TOKENS", "200000"),
            ("CODA_BACKGROUND_INPUT_LIMIT_TOKENS", "150000"),
        ];
        let roles = model_roles("openai-responses", &environment).unwrap();
        for (binding, expected) in [
            (&roles.chat, ("chat-model", "high", 272000, 922000)),
            (
                &roles.background,
                ("background-model", "max", 200000, 150000),
            ),
        ] {
            assert_eq!(
                (
                    binding.model.as_str(),
                    binding.effort.as_str(),
                    binding.context_window_tokens,
                    binding.max_input_tokens
                ),
                expected
            );
            assert_eq!(binding.adapter, "openai-responses");
        }
        // Each deployment key must reach startup; value validation belongs to the shared reader.
        for (missing, _) in environment {
            let incomplete: Vec<_> = environment
                .iter()
                .copied()
                .filter(|(key, _)| *key != missing)
                .collect();
            assert!(
                matches!(model_roles("openai-responses", &incomplete), Err(ModelBindingError::Missing(key)) if key == missing)
            );
        }
    }

    #[test]
    fn preview_ingestion_preserves_dedup_without_accepting_patient_origin_claims() {
        let mut store = DuetSourceStore::new();
        // Register an existing Kno2/EHR Quest lipid record already in the practice workspace.
        let shared_hash = sha256_hex(b"quest:lipid:2026-10:ldl-142");
        store.register_practice_source("pw-elena", &shared_hash, "quest:lipid-2026-10");

        let duplicate_lab = ConsentedDuetItem {
            record_id: "duet-rec-1".to_owned(),
            revision_cursor: sha256_hex(b"rev-1"),
            content_hash: shared_hash,
            source_identity: "quest:lipid-2026-10".to_owned(),
            origin_label: "from Quest".to_owned(),
            category: "lab".to_owned(),
            title: "Lipid Panel".to_owned(),
            summary: "LDL 142 mg/dL".to_owned(),
            recorded_at: "2026-10-10".to_owned(),
            patient_approved_note: false,
        };
        let approved_luna_note = ConsentedDuetItem {
            record_id: "duet-note-1".to_owned(),
            revision_cursor: sha256_hex(b"rev-2"),
            content_hash: sha256_hex(b"note:2026-10-12:morning-fatigue"),
            source_identity: "duet-note:note-1".to_owned(),
            origin_label: String::new(),
            category: "clinical_note".to_owned(),
            title: "Dated Luna Note (2026-10-12)".to_owned(),
            summary: "Fatigue resolved after shifting magnesium to 8pm.".to_owned(),
            recorded_at: "2026-10-12".to_owned(),
            patient_approved_note: true,
        };
        let outside_lab = ConsentedDuetItem {
            record_id: "duet-rec-2".to_owned(),
            revision_cursor: sha256_hex(b"rev-3"),
            content_hash: sha256_hex(b"labcorp:apob-108"),
            source_identity: "labcorp:apob-2026-10".to_owned(),
            origin_label: "Labcorp".to_owned(),
            category: "lab".to_owned(),
            title: "ApoB".to_owned(),
            summary: "ApoB 108 mg/dL".to_owned(),
            recorded_at: "2026-10-11".to_owned(),
            patient_approved_note: false,
        };

        // Without active consent, ingestion is rejected.
        assert!(
            store
                .ingest_preview_items("pw-elena", false, std::slice::from_ref(&approved_luna_note))
                .is_err()
        );

        // A preview consent flag does not authenticate the submitted origin or approval.
        // Keep legitimate deduplication while recording who supplied the content.
        let mut revised = approved_luna_note.clone();
        let added = store
            .ingest_preview_items(
                "pw-elena",
                true,
                &[duplicate_lab, approved_luna_note, outside_lab],
            )
            .unwrap();
        assert_eq!(added, 2);
        let records = store.records("pw-elena");
        assert_eq!(records.len(), 2);
        assert_eq!(records[0].provenance, "Physician-entered preview");
        assert!(!records[0].patient_approved_note);
        assert_eq!(records[1].provenance, "Physician-entered preview");
        assert_eq!(
            store.last_cursor("pw-elena"),
            Some(sha256_hex(b"rev-3")),
            "tracks latest revision cursor"
        );
        revised.revision_cursor = sha256_hex(b"revised-cursor");
        revised.content_hash = sha256_hex(b"revised-content");
        revised.summary = "Revised preview content".to_owned();
        revised.origin_label = "fromé, via the patient's Duet".to_owned();
        revised.patient_approved_note = true;
        assert_eq!(
            store
                .ingest_preview_items("pw-elena", true, std::slice::from_ref(&revised))
                .unwrap(),
            1
        );
        let records = store.records("pw-elena");
        assert_eq!(records.len(), 2);
        assert_eq!(records[0].summary, revised.summary);
        assert_eq!(records[0].provenance, "Physician-entered preview");
        assert!(!records[0].patient_approved_note);
        assert_eq!(
            store
                .ingest_preview_items("pw-elena", true, std::slice::from_ref(&revised))
                .unwrap(),
            0
        );
        assert_eq!(store.last_cursor("pw-elena"), Some(revised.revision_cursor));
    }

    #[test]
    fn doctor_plan_enforces_full_first_version_and_batched_diff_exact_text_approval() {
        let mut plans = DoctorPlanStore::new();
        let v1_items = vec![
            PlanItemInput {
                id: "item-1".to_owned(),
                title: "Statin initiation".to_owned(),
                body: "Start rosuvastatin 10mg nightly with dinner.".to_owned(),
            },
            PlanItemInput {
                id: "item-2".to_owned(),
                title: "Repeat labs".to_owned(),
                body: "Recheck lipid panel and ALT in 8 weeks.".to_owned(),
            },
        ];

        let draft = plans
            .save_draft(
                "pw-elena",
                "practice-1",
                "Dr. Sophia Chen",
                v1_items.clone(),
            )
            .unwrap();
        assert!(!draft.currently_approved);
        assert_eq!(draft.approved_version, 0);
        assert!(draft.items.iter().all(|i| i.needs_approval));

        // Partial approval or mismatched text on v1 fails.
        assert!(
            plans
                .approve_exact_text("pw-elena", &[v1_items[0].clone()], "2026-10-13T10:00:00Z")
                .is_err()
        );
        let mut wrong_text = v1_items.clone();
        wrong_text[0].body = "Start rosuvastatin 5mg nightly.".to_owned();
        assert!(
            plans
                .approve_exact_text("pw-elena", &wrong_text, "2026-10-13T10:00:00Z")
                .is_err()
        );

        // Full exact-text approval succeeds for v1.
        let approved_v1 = plans
            .approve_exact_text("pw-elena", &v1_items, "2026-10-13T10:00:00Z")
            .unwrap();
        assert_eq!(approved_v1.version, 1);
        assert_eq!(approved_v1.label, "your doctor's plan");
        assert_eq!(approved_v1.last_diff.len(), 2);
        assert!(plans.draft("pw-elena").unwrap().currently_approved);

        // Editing item-2 immediately invalidates prior approval (`currently_approved = false`),
        // while keeping item-1 approved so only item-2 needs batched diff approval.
        let mut edited_items = v1_items.clone();
        edited_items[1].body = "Recheck lipid panel, ApoB, and ALT in 6 weeks.".to_owned();
        let draft_after_edit = plans
            .save_draft("pw-elena", "practice-1", "Dr. Sophia Chen", edited_items)
            .unwrap();
        assert!(!draft_after_edit.currently_approved);
        assert!(!draft_after_edit.items[0].needs_approval);
        assert!(draft_after_edit.items[1].needs_approval);

        // Approving only the batched item diff (with exact text) produces v2.
        let approved_v2 = plans
            .approve_exact_text(
                "pw-elena",
                &[PlanItemInput {
                    id: "item-2".to_owned(),
                    title: "Repeat labs".to_owned(),
                    body: "Recheck lipid panel, ApoB, and ALT in 6 weeks.".to_owned(),
                }],
                "2026-10-13T11:00:00Z",
            )
            .unwrap();
        assert_eq!(approved_v2.version, 2);
        assert_eq!(approved_v2.last_diff.len(), 1);
        assert_eq!(approved_v2.last_diff[0].item_id, "item-2");
        assert_eq!(
            approved_v2.last_diff[0].previous_body.as_deref(),
            Some("Recheck lipid panel and ALT in 8 weeks.")
        );
        assert!(plans.draft("pw-elena").unwrap().currently_approved);
    }
}
