//! HTTP handlers and test-only fault injection for the durable Today and Updates
//! read model (`GET /today`, `GET /updates`, `POST /today-updates/open`, and
//! `POST /today-updates/done`).

use std::collections::{BTreeMap, BTreeSet};
use std::sync::{Arc, RwLock};
use std::time::{Duration, UNIX_EPOCH};

use axum::Json;
use axum::body::Body;
use axum::http::{Extensions, Request, StatusCode};
use axum::response::{IntoResponse, Response};
use local_product::runtime::concerns::CommandKey;
use local_product::runtime::today_updates::{
    ActiveUpdateRun, ClearItemDoneRequest, EarlierDayUpdatesGroup, ItemWorkEvidence,
    NoopTodayUpdatesProbe, OpenItemRequest, PracticeOutageLine, ProducerFailure, ProducerKind,
    RecognizableRunKind, SeenItemKey, SeenStanding, TodayDoneResult, TodayProjection, TodayRow,
    TodayRowPayload, TodayUpdatesError, TodayUpdatesProbe, UnhandledWorkKind, UpdateEntry,
    UpdateEntryKind, UpdatesProjection,
};
use local_product::runtime::{Authority, PublicationTransaction, Scope, Store, async_trait};
use physician_auth::{AuthStore, Membership};
use physician_wire::ActiveReviewRun;
use physician_wire::today_updates::{
    EarlierDayUpdatesWire, FailedProducerWire, ItemCitationWire, ItemWorkEvidenceWire,
    MarkTodayDoneBody, OpenTodayUpdateItemBody, PracticeOutageWire, ProducerSliceWire,
    SeenStandingWire, TodayDoneOutcomeWire, TodayDoneResponseWire, UpdateItemKindWire,
    UpdateItemWire,
};
use serde_json::{Value, json};

use crate::SampleFeed;
use crate::auth::{self, AuthContext};
use crate::care::ConnectedCare;
use crate::patients::{PatientDirectory, read_patients};
use crate::timestamp::format_rfc3339;

/// In-process fault injection hook for integration and browser qualification tests.
/// Production routers do not attach this extension.
#[derive(Default)]
pub struct TodayUpdatesFaultInjection {
    fail_producers: RwLock<BTreeSet<ProducerKind>>,
    fail_open_items: RwLock<BTreeSet<String>>,
}

impl TodayUpdatesFaultInjection {
    /// Creates an empty fault-injection controller with all producers healthy.
    #[must_use]
    pub fn new() -> Arc<Self> {
        Arc::new(Self::default())
    }

    /// Forces `producer` to fail on subsequent Today and Updates reads.
    pub fn fail_producer(&self, producer: ProducerKind) {
        self.fail_producers
            .write()
            .expect("fault lock")
            .insert(producer);
    }

    /// Clears all forced producer failures.
    pub fn clear_producer_failures(&self) {
        self.fail_producers.write().expect("fault lock").clear();
    }

    /// Forces `POST /today-updates/open` for `item_id` to fail with a storage fault.
    pub fn fail_open_item(&self, item_id: impl Into<String>) {
        self.fail_open_items
            .write()
            .expect("fault lock")
            .insert(item_id.into());
    }

    /// Clears all forced item-open failures.
    pub fn clear_open_failures(&self) {
        self.fail_open_items.write().expect("fault lock").clear();
    }
}

impl TodayUpdatesProbe for TodayUpdatesFaultInjection {
    fn check_producer(&self, producer: ProducerKind) -> Result<(), String> {
        if self
            .fail_producers
            .read()
            .expect("fault lock")
            .contains(&producer)
        {
            return Err(format!("{} temporarily unavailable", producer.label()));
        }
        Ok(())
    }

    fn check_open_item(&self, _workspace: &str, item_key: &str) -> Result<(), String> {
        if self
            .fail_open_items
            .read()
            .expect("fault lock")
            .contains(item_key)
        {
            return Err("item detail temporarily unavailable".to_owned());
        }
        Ok(())
    }
}

/// Returns an Axum extension layer installing `faults` on a test router.
pub fn fault_layer(
    faults: Arc<TodayUpdatesFaultInjection>,
) -> axum::Extension<Arc<TodayUpdatesFaultInjection>> {
    axum::Extension(faults)
}

struct SampleRuntimeAuthority<'a> {
    auth: &'a dyn AuthStore,
    token: &'a str,
    now_unix: u64,
}

#[async_trait]
impl Authority for SampleRuntimeAuthority<'_> {
    async fn check(
        &mut self,
        _tx: &PublicationTransaction,
        scope: &Scope,
        _session_hash: Option<&[u8]>,
    ) -> Result<(), String> {
        let resolved = self
            .auth
            .identity(self.token, self.now_unix)
            .await
            .map_err(|err| format!("session invalid: {err:?}"))?;
        if resolved.practice_id != scope.practice || resolved.physician_id != scope.physician {
            return Err("session scope mismatch".to_owned());
        }
        if self.auth.membership(&resolved, &scope.workspace).await != Membership::Allowed {
            return Err("workspace not on physician panel".to_owned());
        }
        Ok(())
    }

    async fn lock_publication(
        &mut self,
        tx: &PublicationTransaction,
        scope: &Scope,
        session_hash: Option<&[u8]>,
    ) -> Result<(), String> {
        self.check(tx, scope, session_hash).await
    }
}

fn unix_to_rfc3339(seconds: i64) -> String {
    format_rfc3339(UNIX_EPOCH + Duration::from_secs(seconds.max(0) as u64))
}

fn map_seen_standing(standing: &SeenStanding) -> SeenStandingWire {
    match standing {
        SeenStanding::Unseen => SeenStandingWire::Unseen,
        SeenStanding::Seen { seen_at } => SeenStandingWire::Seen {
            seen_at: unix_to_rfc3339(*seen_at),
        },
        SeenStanding::ChangedSinceSeen {
            seen_at,
            what_changed,
        } => SeenStandingWire::ChangedSinceSeen {
            seen_at: unix_to_rfc3339(*seen_at),
            what_changed: what_changed.clone(),
        },
    }
}

fn map_failed_parts(parts: &[ProducerFailure]) -> Vec<FailedProducerWire> {
    parts
        .iter()
        .map(|part| FailedProducerWire {
            producer: match part.producer {
                ProducerKind::Concerns => ProducerSliceWire::Concerns,
                ProducerKind::PatientReviews => ProducerSliceWire::PatientReviews,
                ProducerKind::VisitPreparation => ProducerSliceWire::VisitPreparation,
                ProducerKind::PatientUnderstanding => ProducerSliceWire::PatientUnderstanding,
                ProducerKind::RunningConversations => ProducerSliceWire::RunningConversations,
                ProducerKind::SourceStatuses => ProducerSliceWire::SourceStatuses,
                ProducerKind::Messages => ProducerSliceWire::SpruceMessages,
                ProducerKind::FollowUps => ProducerSliceWire::FollowUps,
            },
            label: part.producer.label().to_owned(),
            message: part.message.clone(),
        })
        .collect()
}

fn map_practice_outage(outage: &PracticeOutageLine) -> PracticeOutageWire {
    PracticeOutageWire {
        source_id: outage.source_id.clone(),
        source_label: outage.source_label.clone(),
        affected_workspace_count: outage.affected_workspace_count,
        since: unix_to_rfc3339(outage.since_unix),
        failure_detail: outage.failure_detail.clone(),
    }
}

fn serialize_today_row(row: &TodayRow) -> Value {
    let (urgency, kind, work_kind, linked_chat_id) = match &row.payload {
        TodayRowPayload::OpenConcern {
            urgency,
            visit_slot,
            ..
        } => (
            urgency.as_str(),
            if visit_slot.is_some() {
                "visit_nudge"
            } else {
                "concern"
            },
            None,
            None,
        ),
        TodayRowPayload::UnhandledWork { work_kind, .. } => {
            let wk = match work_kind {
                UnhandledWorkKind::PatientReview => "review",
                UnhandledWorkKind::VisitPreparation => "pre_visit_preparation",
                UnhandledWorkKind::InitialUnderstanding => "initial_understanding",
                UnhandledWorkKind::FollowUpReview => "follow_up",
            };
            ("today", "failed_review", Some(wk), None)
        }
        TodayRowPayload::RunningWork {
            run_kind, chat_id, ..
        } => {
            let wk = match run_kind {
                RecognizableRunKind::PhysicianChatResponse => "conversation",
                RecognizableRunKind::PhysicianRequestedReview => "review",
            };
            ("today", "running_work", Some(wk), chat_id.clone())
        }
        TodayRowPayload::MessageThread { urgency, .. } => {
            (urgency.as_str(), "message_thread", None, None)
        }
        TodayRowPayload::MessageToMatch { leads_as_now, .. } => (
            if *leads_as_now { "now" } else { "today" },
            "message_to_match",
            None,
            None,
        ),
        TodayRowPayload::UnreadableMessage { .. } => ("today", "unreadable_message", None, None),
        TodayRowPayload::MessagingChannelProblem { .. } => {
            ("today", "messaging_channel", None, None)
        }
    };
    let seen = matches!(row.seen_standing, SeenStanding::Seen { .. });
    let standing = map_seen_standing(&row.seen_standing);
    let reason = row.detail.clone();

    let wire_title = match &row.payload {
        TodayRowPayload::OpenConcern {
            visit_slot: Some(_),
            ..
        } => "Pre-visit concern".to_owned(),
        TodayRowPayload::OpenConcern {
            visit_slot: None, ..
        } => "Flagged concern".to_owned(),
        _ => row.title.clone(),
    };

    json!({
        "patient_workspace_id": row.workspace,
        "patient_display_name": row.patient_display_name,
        "item_id": row.item_key.as_str(),
        "version_id": row.version_id,
        "waiting_since": unix_to_rfc3339(row.waiting_since_unix),
        "urgency": urgency,
        "kind": kind,
        "work_kind": work_kind,
        "title": wire_title,
        "reason": reason,
        "seen": seen,
        "standing": standing,
        "can_mark_done": row.can_mark_done,
        "linked_chat_id": linked_chat_id,
    })
}

fn map_update_entry(entry: &UpdateEntry) -> UpdateItemWire {
    let kind = match &entry.kind {
        UpdateEntryKind::ConcernRaised { .. } => UpdateItemKindWire::ConcernRaised,
        UpdateEntryKind::ConcernRetired { .. } => UpdateItemKindWire::ConcernRetired,
        UpdateEntryKind::QuietReview { .. } => UpdateItemKindWire::QuietReview,
        UpdateEntryKind::AssessedReview { .. } | UpdateEntryKind::ReplyDrafted { .. } => {
            UpdateItemKindWire::AssessedReview
        }
        UpdateEntryKind::FollowUpChanged { .. } => UpdateItemKindWire::FollowUpChanged,
        UpdateEntryKind::VisitBriefing { .. } => UpdateItemKindWire::VisitBriefing,
        UpdateEntryKind::PatientSummary { .. } => UpdateItemKindWire::PatientSummary,
        UpdateEntryKind::EndedNothingToDo { .. } => UpdateItemKindWire::EndedNothingToDo,
    };
    UpdateItemWire {
        item_id: entry.item_key.as_str().to_owned(),
        version_id: entry.version_id.clone(),
        patient_workspace_id: entry.workspace.clone(),
        patient_display_name: entry.patient_display_name.clone(),
        kind,
        headline: entry.title.clone(),
        detail: entry.detail.clone(),
        occurred_at: unix_to_rfc3339(entry.occurred_at_unix),
        standing: map_seen_standing(&entry.seen_standing),
    }
}

fn map_evidence_wire(evidence: ItemWorkEvidence) -> ItemWorkEvidenceWire {
    ItemWorkEvidenceWire {
        item_id: evidence.item_key.as_str().to_owned(),
        version_id: evidence.version_id,
        patient_workspace_id: evidence.workspace,
        patient_display_name: evidence.patient_display_name,
        kind_label: evidence.kind_label,
        headline: evidence.headline,
        narrative: evidence.clinical_body,
        basis_and_limitations: evidence.basis_and_limitations,
        citations: evidence
            .citations
            .into_iter()
            .map(|c| ItemCitationWire {
                candidate_id: c.reference.candidate_id,
                revision: c.reference.revision,
                source_id: c.source_id,
                source_label: c.source_label,
                label: c.display_label,
                as_of: c.as_of,
                excerpt: c.excerpt,
                body: c.body,
                native_uri: c.native_uri,
            })
            .collect(),
        linked_chat_id: evidence.chat_id,
        standing: map_seen_standing(&evidence.seen_standing),
    }
}

fn error_response(err: TodayUpdatesError) -> Response {
    match err {
        TodayUpdatesError::Session(_) => StatusCode::UNAUTHORIZED.into_response(),
        TodayUpdatesError::AccessDenied { .. } | TodayUpdatesError::NotFound { .. } => (
            StatusCode::NOT_FOUND,
            Json(json!({ "detail": "Not found." })),
        )
            .into_response(),
        TodayUpdatesError::InvalidInput { .. }
        | TodayUpdatesError::CannotMarkDone { .. }
        | TodayUpdatesError::IdempotencyCollision { .. }
        | TodayUpdatesError::ConcernInput(_) => (
            StatusCode::BAD_REQUEST,
            Json(json!({ "detail": err.to_string() })),
        )
            .into_response(),
        TodayUpdatesError::AuthUnavailable(_)
        | TodayUpdatesError::ItemOpenFailed { .. }
        | TodayUpdatesError::Concern(_)
        | TodayUpdatesError::SpruceMatching(_)
        | TodayUpdatesError::ReplyProposal(_)
        | TodayUpdatesError::Database(_) => StatusCode::SERVICE_UNAVAILABLE.into_response(),
    }
}

async fn directory_patient_names(
    directory: &dyn PatientDirectory,
    panel: &[String],
) -> BTreeMap<String, String> {
    let Ok(patients) = read_patients(directory, panel).await else {
        return BTreeMap::new();
    };
    patients
        .into_iter()
        .filter_map(|patient| {
            let row = patient.list_row();
            let ws = row.get("patient_workspace_id")?.as_str()?.to_owned();
            let name = row.get("display_name")?.as_str()?.to_owned();
            if ws.is_empty() || name.is_empty() {
                None
            } else {
                Some((ws, name))
            }
        })
        .collect()
}

fn active_probe(extensions: &Extensions) -> &dyn TodayUpdatesProbe {
    match extensions.get::<Arc<TodayUpdatesFaultInjection>>() {
        Some(faults) => faults.as_ref(),
        None => &NoopTodayUpdatesProbe,
    }
}

/// Serves `GET /today` from the durable `Store` when `Store` and `AuthContext` are attached.
#[allow(clippy::too_many_arguments)]
pub(crate) async fn serve_durable_today(
    store: &Store,
    ctx: &AuthContext,
    connected: &ConnectedCare,
    extensions: &Extensions,
    moment: &crate::care::PracticeMoment,
    target_workspace: Option<&str>,
    connections: Vec<Value>,
    source_outages: Vec<Value>,
) -> Response {
    let panel = ctx.auth.store.panel(&ctx.identity).await;
    let directory = connected.patients(extensions);
    let names = directory_patient_names(&*directory, &panel).await;
    let probe = active_probe(extensions);
    let real_now = auth::now();

    let projection: TodayProjection = match store
        .read_today_with_probe(
            ctx.auth.store.as_ref(),
            &ctx.token,
            real_now,
            target_workspace,
            &names,
            probe,
        )
        .await
    {
        Ok(proj) => proj,
        Err(err) => return error_response(err),
    };

    let seen: Vec<String> = projection
        .rows
        .iter()
        .filter(|row| matches!(row.seen_standing, SeenStanding::Seen { .. }))
        .map(|row| row.item_key.as_str().to_owned())
        .collect();
    let rows: Vec<Value> = projection.rows.iter().map(serialize_today_row).collect();
    let practice_outages: Vec<PracticeOutageWire> = projection
        .practice_outages
        .iter()
        .map(map_practice_outage)
        .collect();
    let failed_parts = map_failed_parts(projection.completeness.failed_parts());

    let Ok(setup_connections) =
        crate::source_setup::today_connections(extensions, &ctx.identity.practice_id).await
    else {
        return StatusCode::SERVICE_UNAVAILABLE.into_response();
    };
    let mut connections = connections;
    connections.extend(
        setup_connections
            .into_iter()
            .map(|conn| serde_json::to_value(conn).expect("serialize today connection")),
    );

    Json(json!({
        "date": moment.date_iso,
        "time_zone": moment.time_zone.as_str(),
        "rows": rows,
        "seen": seen,
        "connections": connections,
        "source_outages": source_outages,
        "cursors": {},
        "as_of": projection.clinical_now_unix,
        "also_reviewed": projection.also_reviewed_today_count,
        "incomplete": !projection.completeness.is_complete(),
        "practice_outages": practice_outages,
        "failed_parts": failed_parts,
    }))
    .into_response()
}

/// Serves `GET /updates` from the durable `Store` when `Store` and `AuthContext` are attached.
pub(crate) async fn serve_durable_updates(
    store: &Store,
    ctx: &AuthContext,
    connected: &ConnectedCare,
    extensions: &Extensions,
    moment: &crate::care::PracticeMoment,
) -> Response {
    let panel = ctx.auth.store.panel(&ctx.identity).await;
    let directory = connected.patients(extensions);
    let names = directory_patient_names(&*directory, &panel).await;
    let probe = active_probe(extensions);
    let real_now = auth::now();

    let projection: UpdatesProjection = match store
        .read_updates_with_probe(ctx.auth.store.as_ref(), &ctx.token, real_now, &names, probe)
        .await
    {
        Ok(proj) => proj,
        Err(err) => return error_response(err),
    };

    let active_runs: Vec<ActiveReviewRun> = projection
        .active_runs
        .iter()
        .map(|run: &ActiveUpdateRun| ActiveReviewRun {
            run_id: run.run_id.clone(),
            patient_workspace_id: run.workspace.clone(),
            patient_display_name: run.patient_display_name.clone(),
            headline: run.label.clone(),
        })
        .collect();
    let today_items: Vec<UpdateItemWire> = projection
        .today_items
        .iter()
        .map(map_update_entry)
        .collect();
    let reviewed_nothing_for_you: Vec<UpdateItemWire> = projection
        .reviewed_nothing_for_you
        .iter()
        .map(map_update_entry)
        .collect();
    let earlier_days: Vec<EarlierDayUpdatesWire> = projection
        .earlier_days
        .iter()
        .map(|group: &EarlierDayUpdatesGroup| EarlierDayUpdatesWire {
            date: group.day_iso.clone(),
            label: group.day_label.clone(),
            items: group.items.iter().map(map_update_entry).collect(),
        })
        .collect();
    let failed_parts = map_failed_parts(projection.completeness.failed_parts());

    let preview_rows: Vec<Value> = today_items
        .iter()
        .map(|item| {
            json!({
                "item_id": item.item_id,
                "patient_workspace_id": item.patient_workspace_id,
                "patient_display_name": item.patient_display_name,
                "headline": item.headline,
                "summary": item.detail,
            })
        })
        .collect();

    Json(json!({
        "date": moment.date_iso,
        "time_zone": moment.time_zone.as_str(),
        "active_runs": active_runs,
        "rows": preview_rows,
        "today_items": today_items,
        "reviewed_nothing_for_you": reviewed_nothing_for_you,
        "earlier_days": earlier_days,
        "failed_parts": failed_parts,
        "as_of": projection.clinical_now_unix,
        "also_reviewed": reviewed_nothing_for_you.len(),
        "next_cursor": null,
        "previous_date": earlier_days.first().map(|day| day.date.clone()),
    }))
    .into_response()
}

/// Marks durable seen state when `item.open` arrives via `/commands` on a durable site.
pub(crate) async fn record_durable_item_open(
    store: &Store,
    ctx: &AuthContext,
    workspace: &str,
    item_id: &str,
    idempotency_key: &str,
) {
    let Ok(seen_key) = SeenItemKey::new(item_id.to_owned()) else {
        return;
    };
    let Ok(cmd_key) = CommandKey::new(idempotency_key) else {
        return;
    };
    let real_now = auth::now();
    let names = BTreeMap::new();
    let _guard = ctx.auth.publication.lock().await;
    let mut authority = SampleRuntimeAuthority {
        auth: ctx.auth.store.as_ref(),
        token: &ctx.token,
        now_unix: real_now,
    };
    let _ = store
        .open_today_or_update_item(
            ctx.auth.store.as_ref(),
            &mut authority,
            &OpenItemRequest {
                token: &ctx.token,
                real_now_unix: real_now,
                workspace,
                item_key: &seen_key,
                command_key: &cmd_key,
                patient_names: &names,
            },
        )
        .await;
}

/// Handles `POST /today-updates/open`, recording per-physician seen state and
/// returning the item's work and verified chart citations.
pub(crate) async fn open_route(
    axum::Extension(connected): axum::Extension<ConnectedCare>,
    request: Request<Body>,
) -> Response {
    let (parts, body) = request.into_parts();
    let Ok(bytes) = axum::body::to_bytes(body, 64_000).await else {
        return StatusCode::BAD_REQUEST.into_response();
    };
    let Ok(payload) = serde_json::from_slice::<OpenTodayUpdateItemBody>(&bytes) else {
        return StatusCode::BAD_REQUEST.into_response();
    };
    let extensions = &parts.extensions;
    let probe = active_probe(extensions);

    if let (Some(store), Some(ctx)) = (extensions.get::<Store>(), extensions.get::<AuthContext>()) {
        let Ok(seen_key) = SeenItemKey::new(payload.item_id.clone()) else {
            return StatusCode::BAD_REQUEST.into_response();
        };
        let Ok(cmd_key) = CommandKey::new(&payload.idempotency_key) else {
            return StatusCode::BAD_REQUEST.into_response();
        };
        let panel = ctx.auth.store.panel(&ctx.identity).await;
        let directory = connected.patients(extensions);
        let names = directory_patient_names(&*directory, &panel).await;
        let real_now = auth::now();

        let _guard = ctx.auth.publication.lock().await;
        let mut authority = SampleRuntimeAuthority {
            auth: ctx.auth.store.as_ref(),
            token: &ctx.token,
            now_unix: real_now,
        };
        return match store
            .open_today_or_update_item_with_probe(
                ctx.auth.store.as_ref(),
                &mut authority,
                &OpenItemRequest {
                    token: &ctx.token,
                    real_now_unix: real_now,
                    workspace: &payload.patient_workspace_id,
                    item_key: &seen_key,
                    command_key: &cmd_key,
                    patient_names: &names,
                },
                probe,
            )
            .await
        {
            Ok(evidence) => {
                let _ = ctx.auth.events.publish(physician_bus::Event::Resync).await;
                Json(map_evidence_wire(evidence)).into_response()
            }
            Err(err) => error_response(err),
        };
    }

    // Fixture fallback (`fixture_site_with_feed`):
    if probe
        .check_open_item(&payload.patient_workspace_id, &payload.item_id)
        .is_err()
    {
        return StatusCode::SERVICE_UNAVAILABLE.into_response();
    }
    let identity = ConnectedCare::identity(extensions);
    let auth_store = connected.auth_store(extensions);
    if auth_store
        .membership(&identity, &payload.patient_workspace_id)
        .await
        != Membership::Allowed
    {
        return StatusCode::NOT_FOUND.into_response();
    }
    let now_str = connected
        .practice_now(extensions)
        .await
        .unwrap_or_else(|_| "2026-08-27T16:00:00.000Z".to_owned());

    if let Some(feed) = extensions.get::<Arc<SampleFeed>>() {
        feed.mark_item_open(
            &identity.practice_id,
            &identity.physician_id,
            &payload.item_id,
        );
        if let Some(evidence) =
            feed.find_fixture_evidence(&payload.patient_workspace_id, &payload.item_id, &now_str)
        {
            return Json(evidence).into_response();
        }
    }

    Json(ItemWorkEvidenceWire {
        item_id: payload.item_id,
        version_id: "v1".to_owned(),
        patient_workspace_id: payload.patient_workspace_id.clone(),
        patient_display_name: payload.patient_workspace_id,
        kind_label: "Attention item".to_owned(),
        headline: "Patient item".to_owned(),
        narrative: "Opened item.".to_owned(),
        basis_and_limitations: None,
        citations: Vec::new(),
        linked_chat_id: None,
        standing: SeenStandingWire::Seen { seen_at: now_str },
    })
    .into_response()
}

/// Handles `POST /today-updates/done`, clearing an open concern or unhandled work item.
pub(crate) async fn done_route(request: Request<Body>) -> Response {
    let (parts, body) = request.into_parts();
    let Ok(bytes) = axum::body::to_bytes(body, 64_000).await else {
        return StatusCode::BAD_REQUEST.into_response();
    };
    let Ok(payload) = serde_json::from_slice::<MarkTodayDoneBody>(&bytes) else {
        return StatusCode::BAD_REQUEST.into_response();
    };
    let extensions = &parts.extensions;
    let (Some(store), Some(ctx)) = (extensions.get::<Store>(), extensions.get::<AuthContext>())
    else {
        return Json(TodayDoneResponseWire {
            outcome: TodayDoneOutcomeWire::Rejected,
            item_id: payload.item_id,
            message: Some("Coda couldn’t mark that done.".to_owned()),
        })
        .into_response();
    };
    let Ok(seen_key) = SeenItemKey::new(payload.item_id.clone()) else {
        return StatusCode::BAD_REQUEST.into_response();
    };
    let Ok(cmd_key) = CommandKey::new(&payload.idempotency_key) else {
        return StatusCode::BAD_REQUEST.into_response();
    };
    let real_now = auth::now();

    let _guard = ctx.auth.publication.lock().await;
    let mut authority = SampleRuntimeAuthority {
        auth: ctx.auth.store.as_ref(),
        token: &ctx.token,
        now_unix: real_now,
    };
    match store
        .clear_today_item_done(
            ctx.auth.store.as_ref(),
            &mut authority,
            &ClearItemDoneRequest {
                token: &ctx.token,
                real_now_unix: real_now,
                workspace: &payload.patient_workspace_id,
                item_key: &seen_key,
                version_id: &payload.version_id,
                command_key: &cmd_key,
            },
        )
        .await
    {
        Ok(
            TodayDoneResult::Cleared { .. }
            | TodayDoneResult::Duplicate { .. }
            | TodayDoneResult::AlreadyCleared { .. },
        ) => {
            let _ = ctx.auth.events.publish(physician_bus::Event::Resync).await;
            Json(TodayDoneResponseWire {
                outcome: TodayDoneOutcomeWire::Accepted,
                item_id: payload.item_id,
                message: None,
            })
            .into_response()
        }
        Err(
            err @ (TodayUpdatesError::CannotMarkDone { .. }
            | TodayUpdatesError::IdempotencyCollision { .. }
            | TodayUpdatesError::InvalidInput { .. }
            | TodayUpdatesError::ConcernInput(_)),
        ) => Json(TodayDoneResponseWire {
            outcome: TodayDoneOutcomeWire::Rejected,
            item_id: payload.item_id,
            message: Some(err.to_string()),
        })
        .into_response(),
        Err(err) => error_response(err),
    }
}
