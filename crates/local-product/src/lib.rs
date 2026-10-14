//! Reimplementation of the local product's model execution.
//!
//! The coda-agents checkout is the reference and is not modified. Chat,
//! background, worker, and linking use the same bindings as
//! `modelRoleConfiguration.ts`. `api` calls OpenAI Responses. The key is an
//! argument from the environment; this crate does not read it from disk and
//! does not call the provider.
//! Book: `book/src/north-star.md`.
#![deny(missing_docs)]

use std::collections::{HashMap, HashSet};

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};

/// How this process runs a model. Unset means `live`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModelExecution {
    /// The Codex CLI, the default.
    Live,
    /// Record a Codex cassette.
    Record,
    /// Replay a Codex cassette.
    Replay,
    /// OpenAI Responses. Requires an authorized key.
    Api,
}

/// One role's model, effort, and adapter.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModelBinding {
    /// The provider model name.
    pub model: String,
    /// The reasoning effort.
    pub effort: String,
    /// The adapter identity.
    pub adapter: String,
}

/// The four roles the local product binds.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModelRoles {
    /// Physician chat.
    pub chat: ModelBinding,
    /// Background work.
    pub background: ModelBinding,
    /// Worker work.
    pub worker: ModelBinding,
    /// Linking work.
    pub linking: ModelBinding,
}

/// One turn the Responses request will carry.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChatTurn {
    /// `user` for the physician, `assistant` for the model.
    pub role: String,
    /// The authored text.
    pub text: String,
}

/// The result of accepting a physician message.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Submit {
    /// The first time this idempotency key was seen. `request` is ready to send.
    Accepted {
        /// The Responses request body.
        request: Value,
    },
    /// This idempotency key was already accepted.
    Duplicate,
    /// The message has no text.
    Rejected,
}

/// Turns for one process. The HTTP server owns persistence later.
#[derive(Debug)]
pub struct ChatSession {
    role: ModelBinding,
    chats: HashMap<(String, String), Vec<ChatTurn>>,
    seen: HashMap<String, (String, String)>,
}

impl ChatSession {
    /// Binds the chat role. Other roles are not called from a physician message.
    pub fn new(role: ModelBinding) -> Self {
        Self {
            role,
            chats: HashMap::new(),
            seen: HashMap::new(),
        }
    }

    /// Records the physician turn and builds the Responses request.
    pub fn submit(
        &mut self,
        idempotency_key: &str,
        workspace_id: &str,
        chat_id: &str,
        text: &str,
    ) -> Submit {
        if let Some(existing) = self.seen.get(idempotency_key) {
            return if existing == &(workspace_id.to_owned(), chat_id.to_owned()) {
                Submit::Duplicate
            } else {
                Submit::Rejected
            };
        }
        if text.trim().is_empty() {
            return Submit::Rejected;
        }
        let key = (workspace_id.to_owned(), chat_id.to_owned());
        let turns = self.chats.entry(key.clone()).or_default();
        turns.push(ChatTurn {
            role: "user".to_owned(),
            text: text.to_owned(),
        });
        self.seen.insert(
            idempotency_key.to_owned(),
            (workspace_id.to_owned(), chat_id.to_owned()),
        );
        Submit::Accepted {
            request: responses_body(&self.role, turns),
        }
    }

    /// Records the model's authored text on the chat the request was for.
    pub fn record_reply(&mut self, workspace_id: &str, chat_id: &str, text: &str) {
        self.chats
            .entry((workspace_id.to_owned(), chat_id.to_owned()))
            .or_default()
            .push(ChatTurn {
                role: "assistant".to_owned(),
                text: text.to_owned(),
            });
    }

    /// The authored texts in order, physician and model.
    pub fn conversation(&self, workspace_id: &str, chat_id: &str) -> Vec<String> {
        self.chats
            .get(&(workspace_id.to_owned(), chat_id.to_owned()))
            .map(|turns| turns.iter().map(|turn| turn.text.clone()).collect())
            .unwrap_or_default()
    }
}

/// Reads `CODA_LOCAL_MODEL_EXECUTION`. Unset is `live`.
pub fn model_execution(value: Option<&str>) -> Result<ModelExecution, String> {
    match value.unwrap_or("live") {
        "live" => Ok(ModelExecution::Live),
        "record" => Ok(ModelExecution::Record),
        "replay" => Ok(ModelExecution::Replay),
        "api" => Ok(ModelExecution::Api),
        _ => Err(
            "CODA_LOCAL_MODEL_EXECUTION must be live, record, replay, or api; live is the default"
                .to_owned(),
        ),
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

/// Binds every role to `adapter`. `CODA_<ROLE>_MODEL` and `CODA_<ROLE>_EFFORT` override one role.
pub fn model_roles(adapter: &str, environment: &[(&str, &str)]) -> Result<ModelRoles, String> {
    Ok(ModelRoles {
        chat: binding("chat", "gpt-6-sol", "high", adapter, environment)?,
        background: binding("background", "gpt-6-luna", "max", adapter, environment)?,
        worker: binding("worker", "gpt-6-luna", "max", adapter, environment)?,
        linking: binding("linking", "gpt-6-luna", "max", adapter, environment)?,
    })
}

/// The Responses body for one chat role. `store` stays off.
pub fn responses_body(role: &ModelBinding, turns: &[ChatTurn]) -> Value {
    json!({
        "model": role.model,
        "reasoning": { "effort": role.effort },
        "input": turns.iter().map(|turn| json!({
            "role": turn.role,
            "content": turn.text,
        })).collect::<Vec<_>>(),
        "store": false,
    })
}

/// The authored text from a Responses JSON body.
pub fn authored_text(response: &Value) -> Option<String> {
    let output = response.get("output")?.as_array()?;
    let mut text = String::new();
    for item in output {
        if item.get("type").and_then(Value::as_str) != Some("message") {
            continue;
        }
        let Some(content) = item.get("content").and_then(Value::as_array) else {
            continue;
        };
        for part in content {
            if part.get("type").and_then(Value::as_str) == Some("output_text")
                && let Some(piece) = part.get("text").and_then(Value::as_str)
            {
                text.push_str(piece);
            }
        }
    }
    if text.is_empty() {
        return None;
    }
    if let Ok(value) = serde_json::from_str::<Value>(&text)
        && let Some(authored) = value.get("text").and_then(Value::as_str)
        && !authored.is_empty()
    {
        return Some(authored.to_owned());
    }
    Some(text)
}

fn binding(
    role: &str,
    model: &str,
    effort: &str,
    adapter: &str,
    environment: &[(&str, &str)],
) -> Result<ModelBinding, String> {
    let prefix = format!("CODA_{}", role.to_uppercase());
    let model_key = format!("{prefix}_MODEL");
    let effort_key = format!("{prefix}_EFFORT");
    let model = environment
        .iter()
        .find(|(key, _)| *key == model_key)
        .map(|(_, value)| *value)
        .unwrap_or(model);
    let effort = environment
        .iter()
        .find(|(key, _)| *key == effort_key)
        .map(|(_, value)| *value)
        .unwrap_or(effort);
    Ok(ModelBinding {
        model: one_word(model, &model_key, &effort_key)?,
        effort: one_word(effort, &model_key, &effort_key)?,
        adapter: adapter.to_owned(),
    })
}

fn one_word(value: &str, model_key: &str, effort_key: &str) -> Result<String, String> {
    if value.is_empty() || value.chars().any(char::is_whitespace) {
        return Err(format!(
            "{model_key} and {effort_key} must each be one nonblank word"
        ));
    }
    Ok(value.to_owned())
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

/// Formats provenance for a patient-contributed record arriving through Duet (`ENG-51`).
///
/// Every Duet-contributed record carries `"via the patient's Duet"`. When an upstream
/// source label is present (such as `"from Quest"` or `"Quest"`), it renders as
/// `"from Quest, via the patient's Duet"`.
pub fn format_duet_provenance(origin_label: &str) -> String {
    let trimmed = origin_label.trim();
    if trimmed.is_empty() || trimmed.eq_ignore_ascii_case("via the patient's Duet") {
        return "via the patient's Duet".to_owned();
    }
    if trimmed.ends_with("via the patient's Duet") {
        return trimmed.to_owned();
    }
    if trimmed.len() >= 5 && trimmed[..5].eq_ignore_ascii_case("from ") {
        format!("{trimmed}, via the patient's Duet")
    } else {
        format!("from {trimmed}, via the patient's Duet")
    }
}

/// One consented record or patient-approved Luna note exported by Duet (`ENG-50` / `ENG-51`).
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

/// One patient-contributed Duet record ingested into a physician's patient workspace (`ENG-51`).
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
    /// Provenance string shown to the physician (always includes `"via the patient's Duet"`).
    pub provenance: String,
    /// Record category.
    pub category: String,
    /// Short display title.
    pub title: String,
    /// Summary text.
    pub summary: String,
    /// Recorded timestamp or date string.
    pub recorded_at: String,
    /// Whether this record is a patient-approved Luna note.
    pub patient_approved_note: bool,
}

#[derive(Debug, Default, Clone)]
struct WorkspaceDuetIngestion {
    practice_hashes: HashSet<String>,
    practice_identities: HashSet<String>,
    records: Vec<IngestedDuetRecord>,
    last_cursor: Option<String>,
}

/// Per-workspace Duet source ingestion and cross-source deduplication store (`ENG-51`).
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

    /// Ingests consented Duet items into `workspace_id`, enforcing active consent,
    /// formatting `"via the patient's Duet"` provenance, and deduplicating by
    /// `content_hash` and `source_identity`.
    pub fn ingest_consented_items(
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
            let provenance = format_duet_provenance(&item.origin_label);
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
                    existing.patient_approved_note = item.patient_approved_note;
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
                patient_approved_note: item.patient_approved_note,
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
        assert_eq!(model_execution(None).unwrap(), ModelExecution::Live);
        assert_eq!(model_execution(Some("api")).unwrap(), ModelExecution::Api);
        assert!(
            model_execution(Some("paid"))
                .unwrap_err()
                .contains("live, record, replay, or api")
        );
    }

    #[test]
    fn roles_match_the_existing_luna_bindings() {
        let roles = model_roles("openai-responses", &[]).unwrap();
        assert_eq!(roles.chat.model, "gpt-6-sol");
        assert_eq!(roles.chat.effort, "high");
        assert_eq!(roles.chat.adapter, "openai-responses");
        for binding in [&roles.background, &roles.worker, &roles.linking] {
            assert_eq!(binding.model, "gpt-6-luna");
            assert_eq!(binding.effort, "max");
            assert_eq!(binding.adapter, "openai-responses");
        }
        let overridden =
            model_roles("openai-responses", &[("CODA_BACKGROUND_EFFORT", "high")]).unwrap();
        assert_eq!(overridden.background.effort, "high");
        assert_eq!(overridden.background.model, "gpt-6-luna");
        assert!(model_roles("openai-responses", &[("CODA_CHAT_MODEL", "two words")]).is_err());
    }

    #[test]
    fn a_chat_request_uses_the_chat_role_and_keeps_one_reply() {
        let roles = model_roles("openai-responses", &[]).unwrap();
        let mut session = ChatSession::new(roles.chat);
        let Submit::Accepted { request } = session.submit("k1", "pw", "chat", "Hello") else {
            panic!("expected the first submit to be accepted");
        };
        assert_eq!(request["model"], "gpt-6-sol");
        assert_eq!(request["reasoning"]["effort"], "high");
        assert_eq!(request["store"], false);
        assert_eq!(request["input"][0]["content"], "Hello");
        assert_eq!(
            session.submit("k1", "pw", "chat", "Hello"),
            Submit::Duplicate
        );
        assert_eq!(session.conversation("pw", "chat"), vec!["Hello".to_owned()]);

        let response = json!({
            "output": [{
                "type": "message",
                "content": [{ "type": "output_text", "text": "{\"text\":\"Noted.\",\"completion\":null}" }]
            }]
        });
        let text = authored_text(&response).unwrap();
        assert_eq!(text, "Noted.");
        session.record_reply("pw", "chat", &text);
        assert_eq!(
            session.conversation("pw", "chat"),
            vec!["Hello".to_owned(), "Noted.".to_owned()]
        );
    }

    #[test]
    fn duet_ingestion_enforces_consent_formats_provenance_and_deduplicates_across_sources() {
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
                .ingest_consented_items("pw-elena", false, &[approved_luna_note.clone()])
                .is_err()
        );

        // With active consent, the duplicate Quest lab is deduplicated while the approved
        // Luna note and Labcorp result are ingested with "via the patient's Duet" provenance.
        let added = store
            .ingest_consented_items(
                "pw-elena",
                true,
                &[duplicate_lab, approved_luna_note, outside_lab],
            )
            .unwrap();
        assert_eq!(added, 2);
        let records = store.records("pw-elena");
        assert_eq!(records.len(), 2);
        assert_eq!(records[0].provenance, "via the patient's Duet");
        assert!(records[0].patient_approved_note);
        assert_eq!(
            records[1].provenance,
            "from Labcorp, via the patient's Duet"
        );
        assert_eq!(
            store.last_cursor("pw-elena"),
            Some(sha256_hex(b"rev-3")),
            "tracks latest revision cursor"
        );
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
