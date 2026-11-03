//! Admission/read adapter over original retained context owners. No producer API:
//! doctor snapshots, client prose and pending assessments cannot create these owners.
use crate::{ApiError, conversation::Submission};
use duet_wire::models::ConversationSendRequestSelectedContext as Selection;
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use sqlx::{Postgres, Transaction};
use uuid::Uuid;

fn identity(value: &Value) -> String {
    Sha256::digest(value.to_string().as_bytes())
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect()
}
/// Restriction receipts remain authoritative even when an old owner row survives
/// restoration. Match provenance, not today's unrelated context. A source disconnect
/// alone has no deletion receipt and does not erase retained evidence.
async fn unrestricted(
    tx: &mut Transaction<'_, Postgres>,
    account: Uuid,
    body: &Value,
    retained_at: chrono::DateTime<chrono::Utc>,
) -> Result<(), ApiError> {
    let restrictions: Vec<(String,Uuid)> = sqlx::query_as("select operation,scope_id from privacy_recovery_events where account_id=$1 and (operation='evidence_delete' or (operation in ('device_withdraw','records_withdraw','apple_health_withdraw','health_records_withdraw') and occurred_at >= $2)) union all select (convert_from(receipt_bytes,'UTF8')::jsonb)->>'operation',(convert_from(receipt_bytes,'UTF8')::jsonb->>'scope_id')::uuid from duet_legacy_privacy_replays where account_id=$1 and (convert_from(receipt_bytes,'UTF8')::jsonb)->>'operation' in ('apple_health_withdraw','health_records_withdraw') and (convert_from(receipt_bytes,'UTF8')::jsonb->>'occurred_at')::timestamptz >= $2")
        .bind(account).bind(retained_at).fetch_all(&mut **tx).await?;
    fn contains(value: &Value, operation: &str, id: Uuid) -> bool {
        match value {
            Value::Object(map) => {
                let source = map.get("source_kind").and_then(Value::as_str).or_else(|| {
                    map.get("source")
                        .and_then(|s| s.get("kind"))
                        .and_then(Value::as_str)
                });
                (matches!(operation, "device_withdraw" | "apple_health_withdraw")
                    && source == Some("appleHealth"))
                    || (matches!(operation, "records_withdraw" | "health_records_withdraw")
                        && source == Some("healthRecords"))
                    || (operation == "evidence_delete"
                        && ["id", "record_id", "canonical_record_id"]
                            .iter()
                            .any(|key| {
                                map.get(*key).and_then(Value::as_str)
                                    == Some(id.to_string().as_str())
                            }))
                    || map.values().any(|v| contains(v, operation, id))
            }
            Value::Array(items) => items.iter().any(|v| contains(v, operation, id)),
            _ => false,
        }
    }
    if restrictions.iter().any(|(op, id)| contains(body, op, *id)) {
        return Err(ApiError::NotFound);
    }
    Ok(())
}
fn invalid() -> ApiError {
    ApiError::Unprocessable("selected measurement source or period is invalid")
}

/// Preserve the selected publication, including only its own supporting facts.
pub(crate) async fn insight(
    tx: &mut Transaction<'_, Postgres>,
    account: Uuid,
    cycle: Uuid,
    selected: Uuid,
) -> Result<Value, ApiError> {
    let mut row: Value = sqlx::query_scalar(
        "select to_jsonb(c) from insight_cycles c where user_id=$1 and id=$2 and status='ready'",
    )
    .bind(account)
    .bind(cycle)
    .fetch_optional(&mut **tx)
    .await?
    .ok_or(ApiError::NotFound)?;
    // source_portal is a newer nullable output annotation, not evidence authority.
    nullable_portal(&mut row);
    let mut body: duet_wire::models::InsightCycle = serde_json::from_value(row)
        .map_err(|_| ApiError::Unavailable("retained publication is incompatible"))?;
    let observation = body
        .insights
        .take()
        .unwrap_or_default()
        .into_iter()
        .find(|v| v.id == selected)
        .ok_or(ApiError::NotFound)?;
    let ids = observation.fact_ids.clone().unwrap_or_default();
    body.supporting_facts = Some(
        body.supporting_facts
            .unwrap_or_default()
            .into_iter()
            .filter(|f| ids.contains(&f.id))
            .collect(),
    );
    body.concept_ids = observation.concept_ids.clone();
    body.corrected_insight_ids = Some(
        body.corrected_insight_ids
            .unwrap_or_default()
            .into_iter()
            .filter(|id| *id == selected)
            .collect(),
    );
    body.insights = Some(vec![observation]);
    let generated_at = body.generated_at.with_timezone(&chrono::Utc);
    let body = json!(body);
    unrestricted(tx, account, &body, generated_at).await?;
    Ok(body)
}

pub(crate) fn nullable_portal(value: &mut Value) {
    match value {
        Value::Object(map) => {
            if let Some(Value::Array(evidence)) = map.get_mut("evidence") {
                for item in evidence {
                    if let Some(item) = item.as_object_mut() {
                        item.entry("source_portal").or_insert(Value::Null);
                    }
                }
            }
            for child in map.values_mut() {
                nullable_portal(child);
            }
        }
        Value::Array(items) => {
            for item in items {
                nullable_portal(item);
            }
        }
        _ => {}
    }
}

pub(crate) async fn admit(
    tx: &mut Transaction<'_, Postgres>,
    account: Uuid,
    selection: &Selection,
) -> Result<Value, ApiError> {
    match selection {
        Selection::Measurement(m) => {
            let connection = m.source.connection_id.as_ref().and_then(|v| v.as_ref());
            let kinds: Vec<&str> = m
                .fact
                .evidence
                .iter()
                .filter_map(|v| v.source_kind.as_ref().and_then(|v| v.as_deref()))
                .filter(|v| !v.is_empty())
                .collect();
            if m.health_context_revision <= 0
                || m.period_end <= m.period_start
                || m.fact.range_start != m.period_start
                || m.fact.range_end != m.period_end
                || m.measurement_id.is_empty()
                || m.measurement_id.len() > 200
                || m.source.kind.is_empty()
                || m.source.kind.len() > 80
                || (!kinds.is_empty() && !kinds.contains(&m.source.kind.as_str()))
                || connection.is_some_and(|id| {
                    id.is_empty()
                        || id.len() > 200
                        || !m.fact.evidence.iter().any(|v| {
                            v.source_connection_ids
                                .as_ref()
                                .is_some_and(|ids| ids.contains(id))
                        })
                })
            {
                return Err(invalid());
            }
            let (fact, retained_at) =
                crate::health_context::selected_measurement(tx, account, m).await?;
            unrestricted(tx, account, &json!(fact), retained_at)
                .await
                .map_err(|e| match e {
                    ApiError::NotFound => ApiError::Conflict("selected_context_unavailable"),
                    other => other,
                })?;
            let id = identity(&json!([
                "measurement",
                m.source_boundary_id,
                m.source,
                m.measurement_id,
                m.period_start,
                m.period_end
            ]));
            let reference = json!({"id":id,"kind":"native_fact","title":fact.title,"source":m.source.kind,"date":m.period_end});
            Ok(
                json!({"source":{"read_id":id,"reference":reference,"kind":"native_fact","body":fact,"retained_at":retained_at},"selected_context":selection}),
            )
        }
        Selection::FollowUp(f) => {
            if f.revision <= 0 {
                return Err(ApiError::Unprocessable(
                    "follow-up revision must be positive",
                ));
            }
            let body: Value = sqlx::query_scalar("select to_jsonb(f) - 'user_id' from follow_ups f where user_id=$1 and id=$2 and revision=$3 and state<>'cancelled'")
                .bind(account).bind(f.id).bind(i64::from(f.revision)).fetch_optional(&mut **tx).await?.ok_or(ApiError::Conflict("selected_follow_up_unavailable"))?;
            // Follow-ups are accepted work, not health evidence. No source chip is invented.
            Ok(json!({"follow_up":body,"selected_context":selection}))
        }
        Selection::PublishedInsight(i) => {
            let body = match insight(tx, account, i.cycle_id, i.insight_id).await {
                Err(ApiError::NotFound) => {
                    return Err(ApiError::Conflict("selected_insight_unavailable"));
                }
                result => result?,
            };
            if body["corrected_insight_ids"]
                .as_array()
                .is_some_and(|ids| !ids.is_empty())
            {
                return Err(ApiError::Conflict("selected_insight_unavailable"));
            }
            let id = identity(&json!(["insight", i.cycle_id, i.insight_id]));
            let reference = json!({"id":id,"kind":"insight","title":body["insights"][0]["title"],"source":"Duet observation","date":body["generated_at"],"cycle_id":i.cycle_id,"insight_id":i.insight_id});
            Ok(
                json!({"source":{"read_id":id,"reference":reference,"kind":"insight","body":body},"selected_context":selection}),
            )
        }
        Selection::Record(_) => unreachable!("canonical records use their exact reader"),
    }
}

/// Current-work fences are stricter than reopening a historical source.
pub(crate) async fn fence(
    tx: &mut Transaction<'_, Postgres>,
    account: Uuid,
    submission: &Submission,
) -> Result<(), ApiError> {
    if let Some(selection) = &submission.selected_context
        && !matches!(selection, Selection::Record(_))
    {
        admit(tx, account, selection).await?;
    }
    Ok(())
}

/// Frozen facts remain historical after a new context or disconnect. Publications
/// reopen through their retained owner so correction markers cannot be hidden.
pub(crate) async fn detail(
    tx: &mut Transaction<'_, Postgres>,
    account: Uuid,
    retained: &Value,
) -> Result<Value, ApiError> {
    let body = if retained["kind"] == "insight" {
        let cycle: Uuid = serde_json::from_value(retained["reference"]["cycle_id"].clone())
            .map_err(|_| ApiError::NotFound)?;
        let selected: Uuid = serde_json::from_value(retained["reference"]["insight_id"].clone())
            .map_err(|_| ApiError::NotFound)?;
        insight(tx, account, cycle, selected).await?
    } else if retained["kind"] == "native_fact" {
        retained["body"].clone()
    } else {
        return Err(ApiError::NotFound);
    };
    if retained["kind"] == "native_fact" {
        let retained_at = serde_json::from_value(retained["retained_at"].clone())
            .map_err(|_| ApiError::NotFound)?;
        unrestricted(tx, account, &body, retained_at).await?;
    }
    Ok(
        json!({"reference":retained["reference"],"text":body.to_string(),"superseded":null,"current_revision":null}),
    )
}
