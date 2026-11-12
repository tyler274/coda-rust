//! Connected Care state and handlers for `duet-api` (`ENG-44`, `ENG-39`, `ENG-45`, `ENG-46`).
//!
//! Provides:
//! - Practice invitation acceptance, versioned `"share with my physician"` consent,
//!   and one-step `"stop sharing"` revocation (`ENG-47`).
//! - Structurally unexportable private `PrivateOperatingIndex` alongside patient-approved
//!   short dated Luna notes (`ENG-52`).
//! - Consented read API (`GET /api/physician-sharing/consented-records`) returning
//!   consented records and approved Luna notes with `sha256` revision cursors (`ENG-50`).
//! - Expiring, revocable read-only `For my doctor` share links (`ENG-46`).

use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};

use axum::Json;
use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sha2::{Digest, Sha256};
use uuid::Uuid;

use crate::AppState;

/// Default TTL for a `For my doctor` share link: 7 days.
pub const DEFAULT_SHARE_LINK_TTL_SECONDS: u64 = 7 * 86_400;

/// One entry in Luna's private `operating_index` (`ENG-52`).
///
/// Intentionally does NOT implement `Serialize` or `Deserialize` so it is
/// structurally impossible to include in any API response or export payload.
#[derive(Debug, Clone)]
pub(crate) struct PrivateOperatingIndexEntry {
    pub(crate) _id: String,
    pub(crate) _internal_note: String,
}

/// Luna's private operating index (`ENG-52`).
///
/// Structurally unexportable: does not implement `Serialize`, and exposes no
/// method returning its internal text to HTTP routes.
#[derive(Debug, Default, Clone)]
pub struct PrivateOperatingIndex {
    entries: Vec<PrivateOperatingIndexEntry>,
}

impl PrivateOperatingIndex {
    /// Records an internal operating-index thought that must never leave Duet.
    pub fn record_internal(&mut self, id: impl Into<String>, note: impl Into<String>) {
        self.entries.push(PrivateOperatingIndexEntry {
            _id: id.into(),
            _internal_note: note.into(),
        });
    }

    /// Returns the count of private operating-index entries (for structural isolation tests).
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Returns true when no private operating-index entries have been recorded.
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}

/// A short dated Luna note proposed in conversation (`ENG-52`).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LunaNoteProposal {
    /// Proposal identifier.
    pub id: String,
    /// Short ISO date label (such as `"2026-10-13"`).
    pub date: String,
    /// Short note title.
    pub title: String,
    /// Patient-visible dated note text.
    pub summary: String,
    /// Whether the patient has approved saving this note to `You`.
    pub approved: bool,
    /// Timestamp when the patient approved the note, if approved.
    pub approved_at: Option<String>,
    /// Person-entered record id in `You` once approved.
    pub saved_record_id: Option<String>,
}

/// One patient record in `You` eligible for consented physician sharing (`ENG-50`).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PatientContributedRecord {
    /// Stable record identifier in Duet.
    pub record_id: String,
    /// Canonical source identity for cross-source deduplication.
    pub source_identity: String,
    /// Optional upstream source label (such as `"from Quest"` or `""` for person-entered).
    #[serde(default)]
    pub origin_label: String,
    /// Record category.
    pub category: String,
    /// Short display title.
    pub title: String,
    /// Record summary text.
    pub summary: String,
    /// Date or ISO timestamp when recorded.
    pub recorded_at: String,
    /// Whether this record originated from a patient-approved Luna note (`ENG-52`).
    #[serde(default)]
    pub patient_approved_note: bool,
}

/// One exported item on the consented Duet read API (`ENG-50`).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConsentedExportItem {
    /// Stable record identifier in Duet.
    pub record_id: String,
    /// Deterministic `sha256` revision cursor.
    pub revision_cursor: String,
    /// Deterministic `sha256` content hash.
    pub content_hash: String,
    /// Canonical source identity for cross-source deduplication.
    pub source_identity: String,
    /// Upstream origin label (such as `"from Quest"` or `""`).
    pub origin_label: String,
    /// Record category.
    pub category: String,
    /// Short display title.
    pub title: String,
    /// Record summary text.
    pub summary: String,
    /// Date or ISO timestamp when recorded.
    pub recorded_at: String,
    /// Whether this record is a patient-approved Luna note.
    pub patient_approved_note: bool,
}

/// Response from `GET /api/physician-sharing/consented-records` (`ENG-50`).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConsentedRecordsResponse {
    /// Linked practice identifier.
    pub practice_id: String,
    /// Linked roster patient workspace identifier.
    pub patient_workspace_id: String,
    /// Active consent version (such as `"v1"`).
    pub consent_version: String,
    /// Latest `sha256` revision cursor across all consented records.
    pub latest_cursor: Option<String>,
    /// Consented records and patient-approved Luna notes after the requested cursor.
    pub items: Vec<ConsentedExportItem>,
}

/// Status of the patient's practice link and versioned `"share with my physician"` consent (`ENG-47`).
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct PhysicianSharingStatus {
    /// Whether the patient has accepted an explicit practice invitation.
    pub linked: bool,
    /// Linked practice identifier, if linked.
    pub practice_id: Option<String>,
    /// Linked practice display name, if linked.
    pub practice_name: Option<String>,
    /// Linked roster patient workspace identifier, if linked.
    pub patient_workspace_id: Option<String>,
    /// Whether `"share with my physician"` consent is currently active.
    pub sharing_consented: bool,
    /// Accepted consent version (such as `"v1"`).
    pub consent_version: Option<String>,
    /// Timestamp when consent was granted.
    pub consented_at: Option<String>,
    /// Timestamp when consent was revoked via one-step `"stop sharing"`, if revoked.
    pub revoked_at: Option<String>,
    /// Count of patient-approved Luna notes saved to `You`.
    pub approved_note_count: usize,
    /// Count of total consented records available to the linked physician when consent is active.
    pub consented_record_count: usize,
}

/// Expiring, revocable read-only share link for `For my doctor` (`ENG-46`).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ForMyDoctorShareLink {
    /// The opaque share token.
    pub token: String,
    /// Relative share path (`/shared/for-my-doctor/{token}`).
    pub share_path: String,
    /// Unix timestamp (seconds) when the link expires.
    pub expires_at_unix: u64,
    /// Whether the patient has revoked the link.
    pub revoked: bool,
}

/// Unauthenticated read-only view returned by `GET /shared/for-my-doctor/{token}` (`ENG-46`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ForMyDoctorSharedView {
    /// Always `true`: unlinked physicians receive a strictly read-only view.
    pub read_only: bool,
    /// Unix timestamp when this share link expires.
    pub expires_at_unix: u64,
    /// Curated `For my doctor` note generation.
    pub note_generation: i64,
    /// Curated item IDs included in `For my doctor`.
    pub item_ids: Vec<Uuid>,
    /// Curated `For my doctor` page content.
    pub page: Option<duet_wire::models::DoctorPageContent>,
    /// Patient-approved short dated Luna notes saved in `You`.
    pub approved_luna_notes: Vec<LunaNoteProposal>,
}

/// Connected Care state held by `duet-api`.
#[derive(Debug, Default)]
pub struct ApiConnectedCare {
    /// Practice invitation and versioned consent state (`ENG-47`).
    pub sharing: PhysicianSharingStatus,
    /// SHA-256 hex digest of the redeemed invitation token.
    pub invitation_token_hash: Option<String>,
    /// Proposed and patient-approved short dated Luna notes (`ENG-52`).
    pub luna_proposals: Vec<LunaNoteProposal>,
    /// Patient-contributed records in `You` eligible for consented sharing (`ENG-50`).
    pub patient_records: Vec<PatientContributedRecord>,
    /// Active or revoked `For my doctor` share link (`ENG-46`).
    pub share_link: Option<ForMyDoctorShareLink>,
    /// Structurally unexportable private operating index (`ENG-52`).
    pub private_operating_index: PrivateOperatingIndex,
}

/// Computes a lowercase hexadecimal SHA-256 digest.
pub fn sha256_hex(bytes: &[u8]) -> String {
    let digest = Sha256::digest(bytes);
    let mut out = String::with_capacity(64);
    for byte in digest {
        use std::fmt::Write as _;
        let _ = write!(&mut out, "{byte:02x}");
    }
    out
}

fn now_unix() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|since| since.as_secs())
        .unwrap_or_default()
}

fn refresh_counts(care: &mut ApiConnectedCare) {
    care.sharing.approved_note_count = care
        .luna_proposals
        .iter()
        .filter(|proposal| proposal.approved)
        .count();
    care.sharing.consented_record_count = care.patient_records.len();
}

fn build_consented_items(care: &ApiConnectedCare) -> Vec<ConsentedExportItem> {
    care.patient_records
        .iter()
        .enumerate()
        .map(|(idx, record)| {
            let content_hash = sha256_hex(
                format!(
                    "{}|{}|{}|{}|{}",
                    record.source_identity,
                    record.category,
                    record.title,
                    record.summary,
                    record.recorded_at
                )
                .as_bytes(),
            );
            let revision_cursor =
                sha256_hex(format!("{}:{}:{}", idx + 1, record.record_id, content_hash).as_bytes());
            ConsentedExportItem {
                record_id: record.record_id.clone(),
                revision_cursor,
                content_hash,
                source_identity: record.source_identity.clone(),
                origin_label: record.origin_label.clone(),
                category: record.category.clone(),
                title: record.title.clone(),
                summary: record.summary.clone(),
                recorded_at: record.recorded_at.clone(),
                patient_approved_note: record.patient_approved_note,
            }
        })
        .collect()
}

pub(crate) async fn get_sharing_status(
    State(state): State<AppState>,
) -> Json<PhysicianSharingStatus> {
    let mut care = state.care.lock().expect("care lock");
    refresh_counts(&mut care);
    Json(care.sharing.clone())
}

#[derive(Debug, Deserialize)]
pub(crate) struct AcceptInvitationRequest {
    practice_id: String,
    #[serde(default)]
    practice_name: Option<String>,
    patient_workspace_id: String,
    invitation_token: String,
}

pub(crate) async fn post_accept_invitation(
    State(state): State<AppState>,
    Json(body): Json<AcceptInvitationRequest>,
) -> Response {
    let practice_id = body.practice_id.trim();
    let workspace_id = body.patient_workspace_id.trim();
    let token = body.invitation_token.trim();
    if practice_id.is_empty() || workspace_id.is_empty() || token.is_empty() {
        return StatusCode::BAD_REQUEST.into_response();
    }
    let mut care = state.care.lock().expect("care lock");
    care.invitation_token_hash = Some(sha256_hex(token.as_bytes()));
    care.sharing.linked = true;
    care.sharing.practice_id = Some(practice_id.to_owned());
    care.sharing.practice_name = Some(
        body.practice_name
            .as_deref()
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .unwrap_or("Connected Practice")
            .to_owned(),
    );
    care.sharing.patient_workspace_id = Some(workspace_id.to_owned());
    refresh_counts(&mut care);
    Json(care.sharing.clone()).into_response()
}

#[derive(Debug, Deserialize)]
pub(crate) struct GrantConsentRequest {
    consent_version: String,
}

pub(crate) async fn put_grant_consent(
    State(state): State<AppState>,
    Json(body): Json<GrantConsentRequest>,
) -> Response {
    let version = body.consent_version.trim();
    if version.is_empty() {
        return StatusCode::BAD_REQUEST.into_response();
    }
    let mut care = state.care.lock().expect("care lock");
    if !care.sharing.linked {
        return StatusCode::BAD_REQUEST.into_response();
    }
    care.sharing.sharing_consented = true;
    care.sharing.consent_version = Some(version.to_owned());
    care.sharing.consented_at = Some("2026-10-13T12:00:00Z".to_owned());
    care.sharing.revoked_at = None;
    refresh_counts(&mut care);
    Json(care.sharing.clone()).into_response()
}

pub(crate) async fn post_stop_sharing(State(state): State<AppState>) -> Response {
    let mut care = state.care.lock().expect("care lock");
    if !care.sharing.linked {
        return StatusCode::BAD_REQUEST.into_response();
    }
    care.sharing.sharing_consented = false;
    care.sharing.revoked_at = Some("2026-10-13T12:30:00Z".to_owned());
    refresh_counts(&mut care);
    Json(care.sharing.clone()).into_response()
}

#[derive(Debug, Deserialize)]
pub(crate) struct ProposeLunaNoteRequest {
    #[serde(default)]
    id: Option<String>,
    date: String,
    title: String,
    summary: String,
    #[serde(default)]
    internal_operating_note: Option<String>,
}

pub(crate) async fn post_propose_luna_note(
    State(state): State<AppState>,
    Json(body): Json<ProposeLunaNoteRequest>,
) -> Response {
    if body.date.trim().is_empty() || body.title.trim().is_empty() || body.summary.trim().is_empty()
    {
        return StatusCode::BAD_REQUEST.into_response();
    }
    let mut care = state.care.lock().expect("care lock");
    if let Some(internal) = body
        .internal_operating_note
        .as_deref()
        .map(str::trim)
        .filter(|s| !s.is_empty())
    {
        let next_idx = care.private_operating_index.len() + 1;
        care.private_operating_index
            .record_internal(format!("op-{next_idx}"), internal);
    }
    let id = body
        .id
        .filter(|id| !id.trim().is_empty())
        .unwrap_or_else(|| format!("luna-note-{}", care.luna_proposals.len() + 1));
    let proposal = LunaNoteProposal {
        id,
        date: body.date.trim().to_owned(),
        title: body.title.trim().to_owned(),
        summary: body.summary.trim().to_owned(),
        approved: false,
        approved_at: None,
        saved_record_id: None,
    };
    care.luna_proposals.push(proposal.clone());
    Json(proposal).into_response()
}

pub(crate) async fn get_luna_note_proposals(
    State(state): State<AppState>,
) -> Json<Vec<LunaNoteProposal>> {
    let care = state.care.lock().expect("care lock");
    Json(care.luna_proposals.clone())
}

pub(crate) async fn post_approve_luna_note(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Response {
    let mut care = state.care.lock().expect("care lock");
    let Some(proposal) = care.luna_proposals.iter_mut().find(|p| p.id == id) else {
        return StatusCode::NOT_FOUND.into_response();
    };
    let record_id = proposal
        .saved_record_id
        .clone()
        .unwrap_or_else(|| format!("you-note-{}", proposal.id));
    proposal.approved = true;
    proposal.approved_at = Some("2026-10-13T12:05:00Z".to_owned());
    proposal.saved_record_id = Some(record_id.clone());
    let approved_proposal = proposal.clone();
    if !care
        .patient_records
        .iter()
        .any(|rec| rec.record_id == record_id)
    {
        care.patient_records.push(PatientContributedRecord {
            record_id,
            source_identity: format!("duet-approved-note:{}", approved_proposal.id),
            origin_label: String::new(),
            category: "patient_approved_luna_note".to_owned(),
            title: format!("{} ({})", approved_proposal.title, approved_proposal.date),
            summary: approved_proposal.summary.clone(),
            recorded_at: approved_proposal.date.clone(),
            patient_approved_note: true,
        });
    }
    refresh_counts(&mut care);
    Json(approved_proposal).into_response()
}

pub(crate) async fn post_patient_record(
    State(state): State<AppState>,
    Json(record): Json<PatientContributedRecord>,
) -> Response {
    if record.record_id.trim().is_empty()
        || record.source_identity.trim().is_empty()
        || record.title.trim().is_empty()
    {
        return StatusCode::BAD_REQUEST.into_response();
    }
    let mut care = state.care.lock().expect("care lock");
    if let Some(existing) = care
        .patient_records
        .iter_mut()
        .find(|existing| existing.record_id == record.record_id)
    {
        *existing = record.clone();
    } else {
        care.patient_records.push(record.clone());
    }
    refresh_counts(&mut care);
    Json(record).into_response()
}

#[derive(Debug, Default, Deserialize)]
pub(crate) struct ConsentedRecordsQuery {
    #[serde(default)]
    after_cursor: Option<String>,
}

pub(crate) async fn get_consented_records(
    State(state): State<AppState>,
    Query(query): Query<ConsentedRecordsQuery>,
) -> Response {
    let care = state.care.lock().expect("care lock");
    if !care.sharing.linked || !care.sharing.sharing_consented {
        return (
            StatusCode::FORBIDDEN,
            Json(json!({ "detail": "Patient has not granted active physician sharing consent." })),
        )
            .into_response();
    }
    let all_items = build_consented_items(&care);
    let latest_cursor = all_items.last().map(|item| item.revision_cursor.clone());
    let items = match query
        .after_cursor
        .as_deref()
        .map(str::trim)
        .filter(|s| !s.is_empty())
    {
        Some(cursor) => {
            if let Some(pos) = all_items
                .iter()
                .position(|item| item.revision_cursor == cursor)
            {
                all_items.into_iter().skip(pos + 1).collect()
            } else {
                all_items
            }
        }
        None => all_items,
    };
    Json(ConsentedRecordsResponse {
        practice_id: care.sharing.practice_id.clone().unwrap_or_default(),
        patient_workspace_id: care
            .sharing
            .patient_workspace_id
            .clone()
            .unwrap_or_default(),
        consent_version: care
            .sharing
            .consent_version
            .clone()
            .unwrap_or_else(|| "v1".to_owned()),
        latest_cursor,
        items,
    })
    .into_response()
}

#[derive(Debug, Default, Deserialize)]
pub(crate) struct CreateShareLinkRequest {
    #[serde(default)]
    ttl_seconds: Option<u64>,
    #[serde(default)]
    expires_at_unix: Option<u64>,
}

pub(crate) async fn post_create_share_link(
    State(state): State<AppState>,
    body: Option<Json<CreateShareLinkRequest>>,
) -> Json<ForMyDoctorShareLink> {
    let mut care = state.care.lock().expect("care lock");
    let token = format!("for-doc-{}", Uuid::new_v4().simple());
    let now = now_unix();
    let expires_at_unix = body
        .as_ref()
        .and_then(|b| b.expires_at_unix)
        .unwrap_or_else(|| {
            let ttl = body
                .as_ref()
                .and_then(|b| b.ttl_seconds)
                .unwrap_or(DEFAULT_SHARE_LINK_TTL_SECONDS);
            now.saturating_add(ttl)
        });
    let link = ForMyDoctorShareLink {
        share_path: format!("/shared/for-my-doctor/{token}"),
        token,
        expires_at_unix,
        revoked: false,
    };
    care.share_link = Some(link.clone());
    Json(link)
}

pub(crate) async fn delete_share_link(State(state): State<AppState>) -> Response {
    let mut care = state.care.lock().expect("care lock");
    let Some(link) = care.share_link.as_mut() else {
        return StatusCode::NOT_FOUND.into_response();
    };
    link.revoked = true;
    Json(link.clone()).into_response()
}

pub(crate) async fn get_shared_for_my_doctor(
    State(state): State<AppState>,
    Path(token): Path<String>,
) -> Response {
    let care = state.care.lock().expect("care lock");
    let Some(link) = care.share_link.as_ref() else {
        return StatusCode::NOT_FOUND.into_response();
    };
    if link.token != token || link.revoked || now_unix() >= link.expires_at_unix {
        return StatusCode::NOT_FOUND.into_response();
    }
    let approved_luna_notes = care
        .luna_proposals
        .iter()
        .filter(|proposal| proposal.approved)
        .cloned()
        .collect();
    Json(ForMyDoctorSharedView {
        read_only: true,
        expires_at_unix: link.expires_at_unix,
        note_generation: 1,
        item_ids: Vec::new(),
        page: None,
        approved_luna_notes,
    })
    .into_response()
}

/// Helper for tests to inspect the count of private operating-index entries without exposing their content.
pub fn private_operating_index_len(care: &Arc<Mutex<ApiConnectedCare>>) -> usize {
    care.lock()
        .expect("care lock")
        .private_operating_index
        .len()
}
