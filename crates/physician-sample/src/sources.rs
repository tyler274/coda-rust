//! Practice source connections, health summaries, check operations, and reconnect commands.
//!
//! All practice-wide outage and patient standing decisions delegate to the single
//! authoritative rules in [`local_product::source_status`].

use std::collections::{BTreeMap, HashSet};
use std::sync::{Arc, RwLock};

use axum::Json;
use axum::Router;
use axum::body::Body;
use axum::extract::Path;
use axum::http::{Extensions, Method, Request, StatusCode};
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};
use axum::routing::post;
use local_product::physician_sources::{PatientSource, workspace_id};
use local_product::runtime::Store;
use local_product::source_status::{
    LinkTarget, PracticeOutageKind, PracticeSourceReport, PracticeSourceSummary,
    ReconnectCommandOutcome, SourceCoordinator, SourcePatientKey, SourceStatusError,
};
use physician_auth::{AuthStore, Membership, PhysicianIdentity};
use serde::Deserialize;
use serde_json::{Value, json};

use crate::auth;
use crate::care::ConnectedCare;
use crate::synthetic_chart::{self, SyntheticCharts, SyntheticSourceMode};

const BODY_LIMIT: usize = 1_000_000;

/// Process-scoped registry of connected [`PatientSource`] adapters and the runtime [`Store`].
#[derive(Default)]
pub struct SourceRegistry {
    runtime: Option<Store>,
    adapters: RwLock<BTreeMap<String, Arc<dyn PatientSource>>>,
    synthetic: RwLock<Option<Arc<SyntheticCharts>>>,
}

impl SourceRegistry {
    /// Creates a registry backed by `runtime` (when a database is configured).
    pub fn new(runtime: Option<Store>) -> Arc<Self> {
        Arc::new(Self {
            runtime,
            adapters: RwLock::new(BTreeMap::new()),
            synthetic: RwLock::new(None),
        })
    }

    /// Registers a [`PatientSource`] adapter under its `source_id()`.
    pub fn register(&self, source: Arc<dyn PatientSource>) {
        self.adapters
            .write()
            .expect("source adapter registry lock")
            .insert(source.source_id().to_owned(), source);
    }

    /// Registers the synthetic chart adapter so connector checks, reconnects, and local-proof
    /// mode switches act on the same shared adapter instance.
    pub fn register_synthetic(&self, charts: Arc<SyntheticCharts>) {
        *self.synthetic.write().expect("synthetic adapter lock") = Some(Arc::clone(&charts));
        self.register(charts);
    }

    /// Looks up a registered [`PatientSource`] by `source_id`.
    pub fn adapter(&self, source_id: &str) -> Option<Arc<dyn PatientSource>> {
        self.adapters
            .read()
            .expect("source adapter registry lock")
            .get(source_id)
            .cloned()
    }

    /// Returns the registered [`SyntheticCharts`] adapter when present.
    pub fn synthetic(&self) -> Option<Arc<SyntheticCharts>> {
        self.synthetic
            .read()
            .expect("synthetic adapter lock")
            .clone()
    }

    /// Returns the runtime [`Store`] when configured.
    pub fn runtime(&self) -> Option<&Store> {
        self.runtime.as_ref()
    }

    /// Sorted identifiers of all registered [`PatientSource`] adapters.
    pub fn registered_source_ids(&self) -> Vec<String> {
        self.adapters
            .read()
            .expect("source adapter registry lock")
            .keys()
            .cloned()
            .collect()
    }
}

fn registry_from(extensions: &Extensions) -> Option<&Arc<SourceRegistry>> {
    extensions.get::<Arc<SourceRegistry>>()
}

async fn practice_now_seconds(
    extensions: &Extensions,
    practice_id: &str,
) -> Result<i64, StatusCode> {
    if let Some(auth_store) = extensions.get::<Arc<dyn AuthStore>>() {
        let clock = auth_store
            .practice_clock(practice_id)
            .await
            .map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?;
        Ok(clock.now(auth::now()) as i64)
    } else {
        Ok(auth::now() as i64)
    }
}

async fn panel_workspaces(
    extensions: &Extensions,
    identity: &PhysicianIdentity,
) -> Option<HashSet<String>> {
    let auth_store = extensions.get::<Arc<dyn AuthStore>>()?;
    Some(auth_store.panel(identity).await.into_iter().collect())
}

fn scope_summary_to_panel(
    summary: &PracticeSourceSummary,
    panel: Option<&HashSet<String>>,
) -> PracticeSourceSummary {
    let Some(allowed) = panel else {
        return summary.clone();
    };
    match summary {
        PracticeSourceSummary::PatientIssues {
            source_id,
            total_patients,
            issues,
        } => PracticeSourceSummary::PatientIssues {
            source_id: source_id.clone(),
            total_patients: *total_patients,
            issues: issues
                .iter()
                .filter(|issue| allowed.contains(&issue.workspace_id))
                .cloned()
                .collect(),
        },
        other => other.clone(),
    }
}

fn format_report(
    registry: &SourceRegistry,
    report: &PracticeSourceReport,
    panel: Option<&HashSet<String>>,
) -> (Value, Option<Value>, Vec<Value>) {
    let source_label = registry
        .adapter(&report.source_id)
        .map(|adapter| adapter.source_label().to_owned())
        .unwrap_or_else(|| report.source_id.clone());
    let summary = scope_summary_to_panel(&report.summary, panel);

    match &summary {
        PracticeSourceSummary::Healthy { .. } => (
            json!({
                "source_id": report.source_id,
                "source_label": source_label,
                "access": report.access,
                "summary": summary,
                "patient_issues": [],
            }),
            None,
            Vec::new(),
        ),
        PracticeSourceSummary::PracticeWideOutage(outage) => {
            let outage_val = json!(outage);
            (
                json!({
                    "source_id": report.source_id,
                    "source_label": source_label,
                    "access": report.access,
                    "summary": summary,
                    "outage": outage_val,
                    "patient_issues": [],
                }),
                Some(json!(outage)),
                Vec::new(),
            )
        }
        PracticeSourceSummary::PatientIssues { issues, .. } => {
            let issue_vals: Vec<Value> = issues.iter().map(|issue| json!(issue)).collect();
            (
                json!({
                    "source_id": report.source_id,
                    "source_label": source_label,
                    "access": report.access,
                    "summary": summary,
                    "patient_issues": issue_vals,
                }),
                None,
                issue_vals,
            )
        }
    }
}

/// Computes `/today`'s `connections` and `source_outages` arrays from the authoritative
/// practice source reports when a runtime store is attached.
pub(crate) async fn today_connections_and_outages(
    extensions: &Extensions,
) -> Result<(Vec<Value>, Vec<Value>), StatusCode> {
    let Some(registry) = registry_from(extensions) else {
        return Ok((Vec::new(), Vec::new()));
    };
    let Some(store) = registry.runtime() else {
        return Ok((Vec::new(), Vec::new()));
    };
    let practice_id = ConnectedCare::identity(extensions).practice_id;
    let reports = store
        .practice_all_source_reports(&practice_id, &registry.registered_source_ids())
        .await
        .map_err(|_| StatusCode::SERVICE_UNAVAILABLE)?;

    let mut connections = Vec::new();
    let mut source_outages = Vec::new();
    for report in reports {
        if let PracticeSourceSummary::PracticeWideOutage(outage) = report.summary {
            let condition = match &outage.kind {
                PracticeOutageKind::ReconnectNeeded { .. } => "reconnect_required",
                PracticeOutageKind::SourceDown { .. } => "temporarily_unavailable",
            };
            connections.push(json!({
                "condition": condition,
                "source_id": outage.source_id,
                "scope": "practice_wide_outage",
                "affected_patients": outage.affected_patients,
                "total_patients": outage.total_patients,
                "kind": outage.kind,
            }));
            source_outages.push(json!(outage));
        }
    }
    Ok((connections, source_outages))
}

/// `GET /connections` — returns practice source summaries, access state, practice-wide outages,
/// and non-outage per-patient issues scoped to the caller's panel.
pub(crate) async fn get_connections(request: Request<Body>) -> Response {
    let extensions = request.extensions();
    let identity = ConnectedCare::identity(extensions);
    let practice_id = identity.practice_id.clone();
    let Some(registry) = registry_from(extensions) else {
        return Json(json!({
            "practice_id": practice_id,
            "sources": [],
            "outages": [],
            "patient_issues": [],
        }))
        .into_response();
    };
    let Some(store) = registry.runtime() else {
        return Json(json!({
            "practice_id": practice_id,
            "sources": [],
            "outages": [],
            "patient_issues": [],
        }))
        .into_response();
    };

    let Ok(reports) = store
        .practice_all_source_reports(&practice_id, &registry.registered_source_ids())
        .await
    else {
        return StatusCode::SERVICE_UNAVAILABLE.into_response();
    };
    let panel = panel_workspaces(extensions, &identity).await;

    let mut sources = Vec::with_capacity(reports.len());
    let mut outages = Vec::new();
    let mut patient_issues = Vec::new();
    for report in &reports {
        let (source_val, outage, issues) = format_report(registry, report, panel.as_ref());
        sources.push(source_val);
        if let Some(outage) = outage {
            outages.push(outage);
        }
        patient_issues.extend(issues);
    }

    Json(json!({
        "practice_id": practice_id,
        "sources": sources,
        "outages": outages,
        "patient_issues": patient_issues,
    }))
    .into_response()
}

#[derive(Default, Deserialize)]
struct CheckRequestBody {
    patient_workspace_id: Option<String>,
    source_patient_id: Option<String>,
}

/// `POST /connections/{source_id}/check` — runs `SourceCoordinator::check_practice` or
/// `SourceCoordinator::check_patient` through the registered [`PatientSource`].
pub(crate) async fn post_check(Path(source_id): Path<String>, request: Request<Body>) -> Response {
    let (parts, body) = request.into_parts();
    let Ok(bytes) = axum::body::to_bytes(body, BODY_LIMIT).await else {
        return StatusCode::BAD_REQUEST.into_response();
    };
    let parsed: CheckRequestBody = if bytes.iter().all(u8::is_ascii_whitespace) {
        CheckRequestBody::default()
    } else {
        let Ok(parsed) = serde_json::from_slice(&bytes) else {
            return StatusCode::BAD_REQUEST.into_response();
        };
        parsed
    };

    let Some(registry) = registry_from(&parts.extensions) else {
        return StatusCode::SERVICE_UNAVAILABLE.into_response();
    };
    let Some(store) = registry.runtime() else {
        return StatusCode::SERVICE_UNAVAILABLE.into_response();
    };
    let Some(adapter) = registry.adapter(&source_id) else {
        return StatusCode::NOT_FOUND.into_response();
    };

    let identity = ConnectedCare::identity(&parts.extensions);
    let practice_id = identity.practice_id.clone();
    let Ok(now) = practice_now_seconds(&parts.extensions, &practice_id).await else {
        return StatusCode::SERVICE_UNAVAILABLE.into_response();
    };

    if parsed.patient_workspace_id.is_some() || parsed.source_patient_id.is_some() {
        let resolved = match (&parsed.patient_workspace_id, &parsed.source_patient_id) {
            (Some(ws), spid_opt) => {
                if let Some(auth_store) = parts.extensions.get::<Arc<dyn AuthStore>>()
                    && auth_store.membership(&identity, ws).await != Membership::Allowed
                {
                    return auth::not_found();
                }
                match store
                    .source_link_for_workspace(&practice_id, &source_id, ws)
                    .await
                {
                    Ok(Some(link)) => {
                        if let Some(spid) = spid_opt
                            && spid != &link.key.source_patient_id
                        {
                            return StatusCode::CONFLICT.into_response();
                        }
                        (link.key.source_patient_id, link.workspace_id, true)
                    }
                    Ok(None) => {
                        let Some(spid) = spid_opt else {
                            return StatusCode::NOT_FOUND.into_response();
                        };
                        (spid.clone(), ws.clone(), true)
                    }
                    Err(SourceStatusError::PracticeMismatch { .. }) => {
                        return StatusCode::NOT_FOUND.into_response();
                    }
                    Err(_) => return StatusCode::SERVICE_UNAVAILABLE.into_response(),
                }
            }
            (None, Some(spid)) => {
                let existing = match store
                    .source_patient_link(&SourcePatientKey {
                        practice_id: practice_id.clone(),
                        source_id: source_id.clone(),
                        source_patient_id: spid.clone(),
                    })
                    .await
                {
                    Ok(existing) => existing,
                    Err(_) => return StatusCode::SERVICE_UNAVAILABLE.into_response(),
                };
                let target_ws = existing
                    .as_ref()
                    .map(|link| link.workspace_id.clone())
                    .unwrap_or_else(|| workspace_id(&source_id, spid));
                if let Some(auth_store) = parts.extensions.get::<Arc<dyn AuthStore>>()
                    && auth_store.membership(&identity, &target_ws).await != Membership::Allowed
                {
                    return auth::not_found();
                }
                (spid.clone(), target_ws, existing.is_some())
            }
            (None, None) => unreachable!(),
        };

        let target = if resolved.2 {
            LinkTarget::Exact(&resolved.1)
        } else {
            LinkTarget::ExistingOrDefault(&resolved.1)
        };

        let patient_report = match SourceCoordinator::check_patient(
            store,
            &*adapter,
            &practice_id,
            &resolved.0,
            target,
            now,
        )
        .await
        {
            Ok(report) => report,
            Err(SourceStatusError::PracticeMismatch { .. }) => {
                return StatusCode::NOT_FOUND.into_response();
            }
            Err(SourceStatusError::ConflictingWorkspaceLink { .. }) => {
                return StatusCode::CONFLICT.into_response();
            }
            Err(SourceStatusError::EmptyIdentity { .. }) => {
                return StatusCode::BAD_REQUEST.into_response();
            }
            Err(SourceStatusError::SourceAdapter { .. } | SourceStatusError::Database(_)) => {
                return StatusCode::SERVICE_UNAVAILABLE.into_response();
            }
        };

        let Ok(practice_report) = store.practice_source_report(&practice_id, &source_id).await
        else {
            return StatusCode::SERVICE_UNAVAILABLE.into_response();
        };
        let panel = panel_workspaces(&parts.extensions, &identity).await;
        let (_, outage, issues) = format_report(registry, &practice_report, panel.as_ref());
        let summary = scope_summary_to_panel(&practice_report.summary, panel.as_ref());

        return Json(json!({
            "scope": "patient",
            "practice_id": practice_id,
            "source_id": source_id,
            "patient": patient_report,
            "access": practice_report.access,
            "summary": summary,
            "outages": outage.into_iter().collect::<Vec<_>>(),
            "patient_issues": issues,
        }))
        .into_response();
    }

    let check = match SourceCoordinator::check_practice(store, &*adapter, &practice_id, now).await {
        Ok(check) => check,
        Err(SourceStatusError::EmptyIdentity { .. }) => {
            return StatusCode::BAD_REQUEST.into_response();
        }
        Err(_) => return StatusCode::SERVICE_UNAVAILABLE.into_response(),
    };

    let Ok(practice_report) = store.practice_source_report(&practice_id, &source_id).await else {
        return StatusCode::SERVICE_UNAVAILABLE.into_response();
    };
    let panel = panel_workspaces(&parts.extensions, &identity).await;
    let (_, outage, issues) = format_report(registry, &practice_report, panel.as_ref());
    let summary = scope_summary_to_panel(&check.summary, panel.as_ref());

    Json(json!({
        "scope": "practice",
        "practice_id": check.practice_id,
        "source_id": check.source_id,
        "checked_patients": check.patients.len(),
        "access": check.access,
        "summary": summary,
        "outages": outage.into_iter().collect::<Vec<_>>(),
        "patient_issues": issues,
    }))
    .into_response()
}

/// `POST /connections/{source_id}/reconnect` — reconnects `source_id` for the caller's practice
/// through the registered [`PatientSource`].
pub(crate) async fn post_reconnect(
    Path(source_id): Path<String>,
    request: Request<Body>,
) -> Response {
    let extensions = request.extensions();
    let Some(registry) = registry_from(extensions) else {
        return StatusCode::SERVICE_UNAVAILABLE.into_response();
    };
    let Some(store) = registry.runtime() else {
        return StatusCode::SERVICE_UNAVAILABLE.into_response();
    };
    let Some(adapter) = registry.adapter(&source_id) else {
        return StatusCode::NOT_FOUND.into_response();
    };

    let identity = ConnectedCare::identity(extensions);
    let practice_id = identity.practice_id.clone();
    let Ok(now) = practice_now_seconds(extensions, &practice_id).await else {
        return StatusCode::SERVICE_UNAVAILABLE.into_response();
    };
    let panel = panel_workspaces(extensions, &identity).await;

    match SourceCoordinator::reconnect_source(store, &*adapter, &practice_id, now).await {
        Ok(report) => Json(json!({
            "outcome": "reconnected",
            "receipt": report.receipt,
            "access": report.access,
            "summary": scope_summary_to_panel(&report.check.summary, panel.as_ref()),
        }))
        .into_response(),
        Err(SourceStatusError::SourceAdapter { source, .. }) => (
            StatusCode::BAD_GATEWAY,
            Json(json!({
                "outcome": "failed",
                "detail": source.to_string(),
            })),
        )
            .into_response(),
        Err(_) => StatusCode::SERVICE_UNAVAILABLE.into_response(),
    }
}

/// Middleware that handles `POST /commands` when `"type" == "reconnect_source"`, passing all
/// other commands untouched to the downstream command handler.
pub(crate) async fn intercept_reconnect_command(request: Request<Body>, next: Next) -> Response {
    if request.method() != Method::POST || request.uri().path() != "/commands" {
        return next.run(request).await;
    }

    let (parts, body) = request.into_parts();
    let Ok(bytes) = axum::body::to_bytes(body, BODY_LIMIT).await else {
        return StatusCode::BAD_REQUEST.into_response();
    };

    let parsed = serde_json::from_slice::<Value>(&bytes).ok();
    let is_reconnect = parsed
        .as_ref()
        .and_then(|val| val.get("type"))
        .and_then(Value::as_str)
        .is_some_and(|kind| kind.trim() == "reconnect_source");

    if !is_reconnect {
        return next
            .run(Request::from_parts(parts, Body::from(bytes)))
            .await;
    }

    let body_json = parsed.unwrap_or(Value::Null);
    let idempotency_key = body_json
        .get("idempotency_key")
        .and_then(Value::as_str)
        .map(str::trim)
        .filter(|key| !key.is_empty());
    let source_id = body_json
        .get("payload")
        .and_then(|payload| payload.get("source_id"))
        .and_then(Value::as_str)
        .or_else(|| body_json.get("source_id").and_then(Value::as_str))
        .map(str::trim)
        .filter(|id| !id.is_empty());

    let reject = || {
        (
            StatusCode::OK,
            Json(json!({
                "outcome": "rejected",
                "message": crate::rejection_message("reconnect_source"),
            })),
        )
            .into_response()
    };

    let (Some(idempotency_key), Some(source_id)) = (idempotency_key, source_id) else {
        return reject();
    };
    let Some(registry) = registry_from(&parts.extensions) else {
        return reject();
    };
    let Some(store) = registry.runtime() else {
        return reject();
    };
    let Some(adapter) = registry.adapter(source_id) else {
        return reject();
    };

    let identity = ConnectedCare::identity(&parts.extensions);
    let practice_id = identity.practice_id.clone();
    let physician_id = identity.physician_id.clone();
    let Ok(now) = practice_now_seconds(&parts.extensions, &practice_id).await else {
        return StatusCode::SERVICE_UNAVAILABLE.into_response();
    };
    let panel = panel_workspaces(&parts.extensions, &identity).await;

    match SourceCoordinator::reconnect_source_command(
        store,
        &*adapter,
        &practice_id,
        &physician_id,
        idempotency_key,
        now,
    )
    .await
    {
        Ok(ReconnectCommandOutcome::Accepted {
            receipt,
            access,
            summary,
        }) => Json(json!({
            "outcome": "accepted",
            "type": "reconnect_source",
            "receipt": receipt,
            "access": access,
            "summary": scope_summary_to_panel(&summary, panel.as_ref()),
        }))
        .into_response(),
        Ok(ReconnectCommandOutcome::ConflictingKey) | Err(_) => reject(),
    }
}

#[derive(Deserialize)]
struct LocalProofModeBody {
    practice_id: Option<String>,
    source_patient_id: Option<String>,
    #[serde(flatten)]
    mode: SyntheticSourceMode,
}

async fn post_local_proof_mode(Path(source_id): Path<String>, request: Request<Body>) -> Response {
    if source_id != synthetic_chart::SOURCE_ID {
        return StatusCode::NOT_FOUND.into_response();
    }
    let (parts, body) = request.into_parts();
    let Some(registry) = registry_from(&parts.extensions) else {
        return StatusCode::NOT_FOUND.into_response();
    };
    let Some(charts) = registry.synthetic() else {
        return StatusCode::NOT_FOUND.into_response();
    };
    let Ok(bytes) = axum::body::to_bytes(body, BODY_LIMIT).await else {
        return StatusCode::BAD_REQUEST.into_response();
    };
    let Ok(parsed) = serde_json::from_slice::<LocalProofModeBody>(&bytes) else {
        return StatusCode::BAD_REQUEST.into_response();
    };

    let practice_id = parsed
        .practice_id
        .filter(|id| !id.trim().is_empty())
        .unwrap_or_else(|| ConnectedCare::identity(&parts.extensions).practice_id);

    if let Some(source_patient_id) = parsed
        .source_patient_id
        .as_deref()
        .map(str::trim)
        .filter(|id| !id.is_empty())
    {
        charts.set_patient_mode(&practice_id, source_patient_id, parsed.mode.clone());
        return Json(json!({
            "source_id": source_id,
            "practice_id": practice_id,
            "source_patient_id": source_patient_id,
            "active_mode": charts.effective_mode(&practice_id, source_patient_id),
        }))
        .into_response();
    }

    charts.set_practice_mode(&practice_id, parsed.mode.clone());
    Json(json!({
        "source_id": source_id,
        "practice_id": practice_id,
        "active_mode": charts.practice_mode(&practice_id),
    }))
    .into_response()
}

/// Local-only proof routes (`POST /local-proof/sources/{source_id}/mode`), mounted strictly
/// when `PHYSICIAN_LOCAL_PROOF=1` outside hosted environments.
pub(crate) fn local_proof_routes() -> Router {
    Router::new().route(
        "/local-proof/sources/{source_id}/mode",
        post(post_local_proof_mode),
    )
}
