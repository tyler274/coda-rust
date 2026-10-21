//! Existing read DTO shapes backed by the same canonical owner, never a second evidence store.
use crate::{ApiError, durable::DurableState, evidence_paging as paging};
use axum::{
    Json, Router,
    extract::{Query, RawQuery, State},
    http::HeaderMap,
    routing::get,
};
use serde::Deserialize;
use serde_json::{Value, json};
/// Mount existing library, exact record inventory and incremental synchronization reads.
pub fn router(state: DurableState) -> Router {
    Router::new()
        .route("/api/evidence/library", get(library).post(library_post))
        .route("/api/evidence/records", get(records))
        .route("/api/evidence/state", get(state_read))
        .with_state(state)
}
fn decorate_group(
    entry: &mut Value,
    snapshot: &crate::evidence::EvidenceSnapshot,
    kind: &str,
    ids: &[uuid::Uuid],
) {
    let records: Vec<_> = snapshot
        .records
        .iter()
        .filter(|r| ids.contains(&r.address.canonical_record_id))
        .collect();
    let mut codes = Vec::new();
    for r in &records {
        for code in paging::codes(&r.payload) {
            if !codes.contains(&code) {
                codes.push(code);
            }
        }
    }
    entry["group_kind"] = json!(kind);
    entry["member_count"] = json!(ids.len());
    entry["coding_identities"] = json!(codes);
    entry["status_conflict"] = json!(
        ["clinical_status", "verification_status", "record_status"]
            .iter()
            .any(|field| {
                records
                    .iter()
                    .filter_map(|r| r.payload["clinical_record"][field].as_str())
                    .map(|s| s.trim().to_lowercase().replace([' ', '_'], "-"))
                    .collect::<std::collections::HashSet<_>>()
                    .len()
                    > 1
            })
    );
    entry["condition_status_conflict"] =
        json!(kind == "condition" && entry["status_conflict"] == true);
}
#[derive(Deserialize, Default)]
struct ReadQuery {
    since_revision: Option<i64>,
    expected_revision: Option<i64>,
    record_id: Option<uuid::Uuid>,
    entity: Option<String>,
    concept_id: Option<String>,
    clinical_kind: Option<String>,
    start: Option<chrono::DateTime<chrono::Utc>>,
    end: Option<chrono::DateTime<chrono::Utc>>,
    scope: Option<String>,
    text: Option<String>,
    source_kind: Option<String>,
    group_kind: Option<String>,
    anchor_record_id: Option<uuid::Uuid>,
    cursor: Option<String>,
}
async fn snapshot(
    s: &DurableState,
    h: &HeaderMap,
    q: &ReadQuery,
) -> Result<crate::evidence::EvidenceSnapshot, ApiError> {
    if q.group_kind.is_some() != q.anchor_record_id.is_some() {
        return Err(ApiError::Unprocessable("group requires kind and anchor"));
    }
    if q.scope
        .as_deref()
        .is_some_and(|s| !["summary", "sourceSummary", "completeRecord"].contains(&s))
    {
        return Err(ApiError::Unprocessable("invalid library scope"));
    }
    let c = s.authenticate(h).await?;
    let snapshot = s
        .evidence
        .snapshot(&c, q.since_revision.unwrap_or(0))
        .await?;
    if q.expected_revision.is_some_and(|r| r != snapshot.cursor) {
        return Err(ApiError::Conflict("evidence changed"));
    }
    Ok(snapshot)
}
async fn records(
    State(s): State<DurableState>,
    h: HeaderMap,
    Query(q): Query<ReadQuery>,
    RawQuery(raw): RawQuery,
) -> Result<Json<Value>, ApiError> {
    let snap = snapshot(&s, &h, &q).await?;
    let url = reqwest::Url::parse(&format!(
        "http://localhost/?{}",
        raw.as_deref().unwrap_or_default()
    ))
    .map_err(|_| ApiError::Unprocessable("invalid query"))?;
    let ids = url
        .query_pairs()
        .filter(|(key, _)| key == "record_ids")
        .map(|(_, value)| {
            value
                .parse::<uuid::Uuid>()
                .map_err(|_| ApiError::Unprocessable("invalid canonical identity"))
        })
        .collect::<Result<Vec<_>, _>>()?;
    if ids.len() > 50 {
        return Err(ApiError::Unprocessable("at most fifty exact records"));
    }
    let mut selected: Vec<_> = snap
        .records
        .iter()
        .filter(|r| {
            q.record_id
                .is_none_or(|id| r.address.canonical_record_id == id)
                && (ids.is_empty() || ids.contains(&r.address.canonical_record_id))
                && q.entity.as_deref().is_none_or(|e| {
                    serde_json::to_value(r.address.entity)
                        .ok()
                        .as_ref()
                        .and_then(Value::as_str)
                        == Some(e)
                })
                && q.concept_id
                    .as_deref()
                    .is_none_or(|id| r.payload["concept_id"].as_str() == Some(id))
                && q.source_kind.as_deref().is_none_or(|kind| {
                    r.payload.pointer("/source/kind").and_then(Value::as_str) == Some(kind)
                })
                && q.clinical_kind
                    .as_deref()
                    .is_none_or(|kind| r.payload["clinical_kind"].as_str() == Some(kind))
                && q.start.is_none_or(|start| {
                    r.payload["measured_at"]
                        .as_str()
                        .or(r.payload["recorded_at"].as_str())
                        .and_then(|s| s.parse::<chrono::DateTime<chrono::Utc>>().ok())
                        .is_some_and(|date| date >= start)
                })
                && q.end.is_none_or(|end| {
                    r.payload["measured_at"]
                        .as_str()
                        .or(r.payload["recorded_at"].as_str())
                        .and_then(|s| s.parse::<chrono::DateTime<chrono::Utc>>().ok())
                        .is_some_and(|date| date < end)
                })
        })
        .collect();
    if q.group_kind.is_some() || q.anchor_record_id.is_some() {
        return Err(ApiError::Unprocessable(
            "use complete group library membership before exact record reads",
        ));
    }
    selected.sort_by_key(|r| r.address.canonical_record_id);
    let unavailable: Vec<_> = ids
        .iter()
        .filter(|id| {
            !selected
                .iter()
                .any(|r| r.address.canonical_record_id == **id)
        })
        .collect();
    if q.record_id.is_some() && selected.is_empty() {
        return Err(ApiError::NotFound);
    }
    let c = s.authenticate(&h).await?;
    let key = paging::query_key("records", raw.as_deref())?;
    let offset = paging::offset(q.cursor.as_deref(), &c, &snap, &key)?;
    let total = selected.len();
    let end = paging::end(&selected, offset)?;
    let next = paging::next(&c, &snap, &key, end, total);
    let selected = &selected[offset..end];
    Ok(Json(
        json!({"canonical_revisions":selected.iter().map(|r|&r.address).collect::<Vec<_>>(),"catalog_revision":null,"clinical_notes":selected.iter().filter(|r|matches!(r.address.entity,duet_wire::models::addressed_record_revision::Entity::ClinicalNote)).map(|r|&r.payload).collect::<Vec<_>>(),"health_evidence":selected.iter().filter(|r|matches!(r.address.entity,duet_wire::models::addressed_record_revision::Entity::HealthEvidence)).map(|r|&r.payload).collect::<Vec<_>>(),"health_records_generation":snap.health_records_generation,"next_cursor":next,"revision":snap.cursor,"total_count":total,"unavailable":unavailable}),
    ))
}
async fn state_read(
    State(s): State<DurableState>,
    h: HeaderMap,
    Query(q): Query<ReadQuery>,
    RawQuery(raw): RawQuery,
) -> Result<Json<Value>, ApiError> {
    let mut snap = snapshot(&s, &h, &q).await?;
    let c = s.authenticate(&h).await?;
    let key = paging::query_key("state", raw.as_deref())?;
    let offset = paging::offset(q.cursor.as_deref(), &c, &snap, &key)?;
    let units:Vec<_>=snap.changes.iter().map(|change| json!({"change":change,"record":snap.records.iter().find(|r|r.address.canonical_record_id==change.canonical_record_id)})).collect();
    let end = paging::end(&units, offset)?;
    let next = paging::next(&c, &snap, &key, end, units.len());
    snap.changes = snap.changes[offset..end].to_vec();
    snap.records.retain(|record| {
        snap.changes
            .iter()
            .any(|change| change.canonical_record_id == record.address.canonical_record_id)
    });
    Ok(Json(
        json!({"canonical_revisions":snap.records.iter().map(|r|json!({"entity":r.address.entity,"record_id":r.address.record_id,"canonical_record_id":r.address.canonical_record_id,"revision":r.address.revision})).collect::<Vec<_>>(),"catalog_revision":null,"clinical_notes":snap.records.iter().filter(|r|matches!(r.address.entity,duet_wire::models::addressed_record_revision::Entity::ClinicalNote)).map(|r|&r.payload).collect::<Vec<_>>(),"health_evidence":snap.records.iter().filter(|r|matches!(r.address.entity,duet_wire::models::addressed_record_revision::Entity::HealthEvidence)).map(|r|&r.payload).collect::<Vec<_>>(),"deleted_records":snap.changes.iter().filter(|r|r.deleted).collect::<Vec<_>>(),"health_records_generation":snap.health_records_generation,"mode":if q.since_revision.is_some(){"changes"}else{"snapshot"},"revision":snap.cursor,"next_cursor":next}),
    ))
}
async fn library_post(
    State(s): State<DurableState>,
    h: HeaderMap,
    Json(body): Json<duet_wire::models::EvidenceLibraryQuery>,
) -> Result<Json<Value>, ApiError> {
    let value =
        serde_json::to_value(body).map_err(|_| ApiError::Unprocessable("invalid library query"))?;
    let mut url = reqwest::Url::parse("http://localhost/").expect("constant URL");
    for (key, value) in value.as_object().expect("query object") {
        if value.is_null() {
            continue;
        }
        if let Some(values) = value.as_array() {
            for value in values {
                url.query_pairs_mut()
                    .append_pair(key, value.as_str().unwrap_or(&value.to_string()));
            }
        } else {
            url.query_pairs_mut()
                .append_pair(key, value.as_str().unwrap_or(&value.to_string()));
        }
    }
    let q: ReadQuery = serde_json::from_value(value)
        .map_err(|_| ApiError::Unprocessable("invalid library query"))?;
    library(
        State(s),
        h,
        Query(q),
        RawQuery(url.query().map(str::to_owned)),
    )
    .await
}
async fn library(
    State(s): State<DurableState>,
    h: HeaderMap,
    Query(q): Query<ReadQuery>,
    RawQuery(raw): RawQuery,
) -> Result<Json<Value>, ApiError> {
    let snap = snapshot(&s, &h, &q).await?;
    let url = reqwest::Url::parse(&format!(
        "http://localhost/?{}",
        raw.as_deref().unwrap_or_default()
    ))
    .map_err(|_| ApiError::Unprocessable("invalid library query"))?;
    let categories: Vec<String> = url
        .query_pairs()
        .filter(|(k, _)| k == "categories")
        .map(|(_, v)| v.into_owned())
        .collect();
    if q.group_kind.is_some()
        && (!categories.is_empty()
            || q.text.as_deref().is_some_and(|s| !s.is_empty())
            || q.source_kind.is_some()
            || q.scope.as_deref() != Some("completeRecord"))
    {
        return Err(ApiError::Unprocessable(
            "group cannot be narrowed by list filters",
        ));
    }
    let members = q
        .anchor_record_id
        .zip(q.group_kind.as_deref())
        .map(|(id, kind)| paging::members(&snap.records, id, kind))
        .transpose()?;
    let requested_codes: Vec<Value> = url
        .query_pairs()
        .filter(|(key, _)| key == "coding_identities")
        .map(|(_, v)| {
            serde_json::from_str(&v).map_err(|_| ApiError::Unprocessable("invalid coding identity"))
        })
        .collect::<Result<_, _>>()?;
    if !requested_codes.is_empty() {
        let Some(ids) = &members else {
            return Err(ApiError::Unprocessable("coding identities require a group"));
        };
        let valid = snap
            .records
            .iter()
            .filter(|r| ids.contains(&r.address.canonical_record_id))
            .flat_map(|r| paging::codes(&r.payload))
            .any(|code| {
                requested_codes.iter().any(|r| {
                    r["system"] == code["system"]
                        && r["code"] == code["code"]
                        && r["version"] == code["version"]
                })
            });
        if !valid {
            return Err(ApiError::Conflict("group coding identity changed"));
        }
    }
    let mut selected: Vec<_> = snap
        .records
        .iter()
        .filter(|r| {
            categories.is_empty()
                || categories
                    .iter()
                    .any(|category| r.payload["category"].as_str() == Some(category))
        })
        .filter(|r| {
            q.text.as_deref().is_none_or(|text| {
                r.payload
                    .to_string()
                    .to_lowercase()
                    .contains(&text.to_lowercase())
            }) && q.source_kind.as_deref().is_none_or(|kind| {
                r.payload.pointer("/source/kind").and_then(Value::as_str) == Some(kind)
            })
        })
        .filter(|r| {
            members
                .as_ref()
                .is_none_or(|ids| ids.contains(&r.address.canonical_record_id))
        })
        .collect();
    selected.sort_by_key(|r| r.address.canonical_record_id);
    let mut counts = serde_json::Map::new();
    for r in &selected {
        let category = r.payload["category"].as_str().unwrap_or("other");
        let count = counts.get(category).and_then(Value::as_u64).unwrap_or(0);
        counts.insert(category.into(), json!(count + 1));
    }
    let c = s.authenticate(&h).await?;
    let key = paging::query_key("library", raw.as_deref())?;
    let offset = paging::offset(q.cursor.as_deref(), &c, &snap, &key)?;
    let mut entries:Vec<_>=selected.iter().map(|r|json!({"category":r.payload["category"].as_str().unwrap_or("other"),"coding_identities":paging::codes(&r.payload),"condition_status_conflict":false,"date":r.payload["recorded_at"],"entity":r.address.entity,"group_kind":null,"id":r.address.canonical_record_id.to_string(),"record_id":r.address.canonical_record_id,"member_count":1,"preview":r.payload,"status_conflict":false,"title":r.payload["display_name"].as_str().or(r.payload["title"].as_str()).unwrap_or("Health record"),"source_label":r.payload["source_label"]})).collect();
    let mut group = None;
    if members.is_none() {
        let mut seen = std::collections::HashSet::new();
        let mut grouped = Vec::new();
        for (r, mut entry) in selected.iter().zip(entries) {
            if !seen.insert(r.address.canonical_record_id) {
                continue;
            }
            if let Some(kind) = paging::kind(&r.payload) {
                let ids = paging::members(&snap.records, r.address.canonical_record_id, kind)?;
                seen.extend(ids.iter().copied());
                decorate_group(&mut entry, &snap, kind, &ids);
            }
            grouped.push(entry);
        }
        entries = grouped;
    } else if let (Some(ids), Some(kind)) = (&members, q.group_kind.as_deref())
        && let Some(mut entry) = entries.first().cloned()
    {
        decorate_group(&mut entry, &snap, kind, ids);
        group = Some(entry);
    }
    let total = entries.len();
    let end = paging::end(&entries, offset)?;
    let next = paging::next(&c, &snap, &key, end, total);
    let entries = &entries[offset..end];
    Ok(Json(
        json!({"catalog_revision":null,"counts":counts,"entries":entries,"group":group,"health_records_generation":snap.health_records_generation,"is_refreshing":false,"next_cursor":next,"revision":snap.cursor,"total_count":total}),
    ))
}
