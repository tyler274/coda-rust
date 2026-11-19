//! Panel-authorized HTTP projection for saved pre-visit briefings and briefing history.

use std::cmp::Reverse;
use std::collections::BTreeMap;

use axum::Json;
use axum::body::Body;
use axum::extract::Path;
use axum::http::{Request, StatusCode};
use axum::response::{IntoResponse, Response};
use local_product::physician_sources::SourceRevision;
use local_product::runtime::Store;
use local_product::runtime::briefings::{
    BriefingError, SavedVisitBriefing, VisitStanding, visit_can_start_preparation,
};
use local_product::runtime::chat_operations::{RecordLimit, RecordLimitKind};
use local_product::runtime::concerns::ExactRecordReference;
use local_product::runtime::jobs::PracticeAccessError;
use local_product::runtime::schedule::{AppointmentStatus, format_unix_rfc3339};
use local_product::runtime::understanding::SummaryCitation;
use local_product::visit_preparation::{
    BriefingClaimInput, BriefingDecisionInput, SourceRefreshStatusSummary,
};
use physician_wire::briefings::{
    BriefingClaimWire, BriefingDecisionWire, BriefingLimitKindWire, BriefingLimitWire,
    BriefingSourceStatusWire, BriefingSummaryItemWire, BriefingVisitStandingWire,
    VisitBriefingResponse, VisitBriefingWire, WorkspaceBriefingsResponse,
};
use physician_wire::{CitedSourceWire, MessageSegment};

use crate::auth::now;
use crate::care::ConnectedCare;

const DEFAULT_VISIT_LABEL: &str = "Scheduled visit";

/// Serves `GET /patient-workspaces/{id}/briefings`: lists all saved briefings
/// (current, moved, and cancelled) for an authorized panel patient workspace.
pub(crate) async fn workspace_briefings(
    Path(workspace): Path<String>,
    axum::Extension(connected): axum::Extension<ConnectedCare>,
    request: Request<Body>,
) -> Response {
    let extensions = request.extensions();
    if !crate::on_panel_in_practice(&connected, extensions, &workspace).await {
        return StatusCode::NOT_FOUND.into_response();
    }
    let Some(runtime_store) = extensions.get::<Store>().cloned() else {
        return Json(WorkspaceBriefingsResponse::default()).into_response();
    };
    let identity = ConnectedCare::identity(extensions);
    let auth_store = connected.auth_store(extensions);

    let saved = match runtime_store
        .visit_briefings_for_workspace(&identity.practice_id, &workspace, auth_store.as_ref())
        .await
    {
        Ok(items) => items,
        Err(BriefingError::Access(
            PracticeAccessError::InvalidScope
            | PracticeAccessError::UnregisteredPatient { .. }
            | PracticeAccessError::ForeignPractice { .. },
        )) => {
            return StatusCode::NOT_FOUND.into_response();
        }
        Err(error) => {
            tracing::error!(%error, "workspace briefings read failed");
            return StatusCode::SERVICE_UNAVAILABLE.into_response();
        }
    };

    let Ok(clock) = auth_store.practice_clock(&identity.practice_id).await else {
        return StatusCode::SERVICE_UNAVAILABLE.into_response();
    };
    let current_time_unix = clock.now(now()) as i64;
    let appointments = appointment_info(&runtime_store, &identity.practice_id).await;

    let mut briefings: Vec<(i64, BriefingSummaryItemWire)> = saved
        .into_iter()
        .map(|item| {
            let slot_lookup = (
                item.slot.source_id.clone(),
                item.slot.appointment_id.clone(),
            );
            let (visit_label, status) = match appointments.get(&slot_lookup) {
                Some((label, status)) => (label.clone(), Some(*status)),
                None => (DEFAULT_VISIT_LABEL.to_owned(), None),
            };
            let visit_standing = project_standing(
                &item.visit_standing,
                item.ends_at_unix,
                &item.ends_at,
                status,
                current_time_unix,
            );
            (
                item.slot.starts_at_unix,
                BriefingSummaryItemWire {
                    briefing_id: item.briefing_id,
                    appointment_id: item.slot.appointment_id,
                    visit_label,
                    starts_at: item.starts_at,
                    ends_at: item.ends_at,
                    prepared_at: format_unix_rfc3339(item.prepared_at),
                    visit_card_orientation: item.visit_card_orientation,
                    visit_standing,
                },
            )
        })
        .collect();
    briefings.sort_by_key(|(starts_at_unix, item)| {
        let historical_rank = u8::from(!matches!(
            item.visit_standing,
            BriefingVisitStandingWire::Current
        ));
        (
            historical_rank,
            Reverse(*starts_at_unix),
            item.briefing_id.clone(),
        )
    });
    let briefings = briefings.into_iter().map(|(_, item)| item).collect();

    Json(WorkspaceBriefingsResponse { briefings }).into_response()
}

/// Serves `GET /patient-workspaces/{id}/briefings/{briefing_id}`: returns one
/// saved visit briefing with resolved inline citations and record passages.
pub(crate) async fn visit_briefing(
    Path((workspace, briefing_id)): Path<(String, String)>,
    axum::Extension(connected): axum::Extension<ConnectedCare>,
    request: Request<Body>,
) -> Response {
    let extensions = request.extensions();
    if !crate::on_panel_in_practice(&connected, extensions, &workspace).await {
        return StatusCode::NOT_FOUND.into_response();
    }
    let Some(runtime_store) = extensions.get::<Store>().cloned() else {
        return StatusCode::NOT_FOUND.into_response();
    };
    let patient = match connected.patient(extensions, &workspace).await {
        Ok(Some(patient)) => patient,
        Ok(None) => return StatusCode::NOT_FOUND.into_response(),
        Err(_) => return StatusCode::SERVICE_UNAVAILABLE.into_response(),
    };
    let patient_display_name = patient.list_row()["display_name"]
        .as_str()
        .unwrap_or_default()
        .to_owned();

    let identity = ConnectedCare::identity(extensions);
    let auth_store = connected.auth_store(extensions);

    let saved = match runtime_store
        .visit_briefing_by_id(&identity.practice_id, &briefing_id, auth_store.as_ref())
        .await
    {
        Ok(Some(item)) if item.workspace == workspace => item,
        Ok(Some(_) | None) => return StatusCode::NOT_FOUND.into_response(),
        Err(BriefingError::Access(
            PracticeAccessError::InvalidScope
            | PracticeAccessError::UnregisteredPatient { .. }
            | PracticeAccessError::ForeignPractice { .. },
        )) => {
            return StatusCode::NOT_FOUND.into_response();
        }
        Err(error) => {
            tracing::error!(%error, "visit briefing read failed");
            return StatusCode::SERVICE_UNAVAILABLE.into_response();
        }
    };

    let Ok(clock) = auth_store.practice_clock(&identity.practice_id).await else {
        return StatusCode::SERVICE_UNAVAILABLE.into_response();
    };
    let current_time_unix = clock.now(now()) as i64;
    let appointments = appointment_info(&runtime_store, &identity.practice_id).await;
    let slot_lookup = (
        saved.slot.source_id.clone(),
        saved.slot.appointment_id.clone(),
    );
    let (visit_label, appointment_status) = match appointments.get(&slot_lookup) {
        Some((label, status)) => (label.clone(), Some(*status)),
        None => (DEFAULT_VISIT_LABEL.to_owned(), None),
    };

    let observation = match runtime_store.sources(&workspace).await {
        Ok(obs) => obs,
        Err(error) => {
            tracing::error!(%error, "workspace sources read failed for briefing");
            return StatusCode::SERVICE_UNAVAILABLE.into_response();
        }
    };

    let wire = project_briefing(
        saved,
        patient_display_name,
        visit_label,
        appointment_status,
        current_time_unix,
        &observation.sources,
    );
    Json(VisitBriefingResponse { briefing: wire }).into_response()
}

async fn appointment_info(
    runtime_store: &Store,
    practice_id: &str,
) -> BTreeMap<(String, String), (String, AppointmentStatus)> {
    let Ok(schedule) = runtime_store.practice_schedule(practice_id).await else {
        return BTreeMap::new();
    };
    schedule
        .appointments
        .into_iter()
        .map(|appt| {
            (
                (
                    appt.source_id.as_str().to_owned(),
                    appt.appointment_id.as_str().to_owned(),
                ),
                (appt.label, appt.status),
            )
        })
        .collect()
}

struct CitationEntry {
    source_key: String,
    number: usize,
    label: String,
    quote: String,
}

fn project_briefing(
    saved: SavedVisitBriefing,
    patient_display_name: String,
    visit_label: String,
    appointment_status: Option<AppointmentStatus>,
    current_time_unix: i64,
    retained_sources: &[SourceRevision],
) -> VisitBriefingWire {
    let retained_by_ref: BTreeMap<(&str, &str), &SourceRevision> = retained_sources
        .iter()
        .map(|src| ((src.candidate_id.as_str(), src.revision.as_str()), src))
        .collect();

    let mut sources = BTreeMap::new();
    let mut index_by_ref: BTreeMap<(String, String), CitationEntry> = BTreeMap::new();

    for (idx, citation) in saved.citations.iter().enumerate() {
        let number = idx + 1;
        let source_key = format!("b{number}");
        let retained = retained_by_ref
            .get(&(
                citation.reference.candidate_id.as_str(),
                citation.reference.revision.as_str(),
            ))
            .copied();

        let title = resolve_citation_title(citation, retained);
        let quote = clean_excerpt_quote(&citation.excerpt, &title);
        let (body, raw_content, record_type) = build_cited_body(&title, &quote, retained);
        let wire_source = CitedSourceWire {
            source_key: source_key.clone(),
            number,
            source_label: citation.source_label.clone(),
            title: title.clone(),
            record_type,
            recorded_at: citation.as_of.clone(),
            quote: quote.clone(),
            body,
            raw_content,
            external_url: citation.native_uri.clone(),
            origin: Default::default(),
        };
        sources.insert(source_key.clone(), wire_source);
        index_by_ref.insert(
            (
                citation.reference.candidate_id.clone(),
                citation.reference.revision.clone(),
            ),
            CitationEntry {
                source_key,
                number,
                label: title,
                quote,
            },
        );
    }

    let trajectory = saved
        .trajectory
        .iter()
        .map(|claim| project_claim(claim, &index_by_ref))
        .collect();
    let meaningful_changes = saved
        .meaningful_changes
        .iter()
        .map(|claim| project_claim(claim, &index_by_ref))
        .collect();
    let unresolved_issues = saved
        .unresolved_issues
        .iter()
        .map(|claim| project_claim(claim, &index_by_ref))
        .collect();
    let visit_decisions = saved
        .visit_decisions
        .iter()
        .map(|decision| project_decision(decision, &index_by_ref))
        .collect();
    let basis_segments = segments_with_citations(
        &saved.basis_and_limitations,
        &saved.basis_citations,
        &index_by_ref,
    );

    let failed_source_ids: std::collections::BTreeSet<&str> = saved
        .source_freshness
        .iter()
        .filter(|status| status.failure_reason.is_some() || status.last_good_pulled_at.is_none())
        .map(|status| status.source_id.as_str())
        .collect();
    let limits = saved
        .limits
        .iter()
        .filter(|limit| {
            !(limit.kind == RecordLimitKind::Unavailable
                && failed_source_ids.contains(limit.subject.as_str()))
        })
        .map(project_limit)
        .collect();

    let visit_standing = project_standing(
        &saved.visit_standing,
        saved.ends_at_unix,
        &saved.ends_at,
        appointment_status,
        current_time_unix,
    );

    VisitBriefingWire {
        briefing_id: saved.briefing_id,
        patient_workspace_id: saved.workspace,
        patient_display_name,
        source_id: saved.slot.source_id,
        appointment_id: saved.slot.appointment_id,
        visit_label,
        starts_at: saved.starts_at,
        ends_at: saved.ends_at,
        prepared_at: format_unix_rfc3339(saved.prepared_at),
        visit_standing,
        visit_card_orientation: saved.visit_card_orientation,
        source_freshness: saved
            .source_freshness
            .iter()
            .map(project_source_status)
            .collect(),
        limits,
        trajectory,
        meaningful_changes,
        unresolved_issues,
        visit_decisions,
        basis_segments,
        sources,
    }
}

fn resolve_citation_title(citation: &SummaryCitation, retained: Option<&SourceRevision>) -> String {
    let Some(source) = retained else {
        return citation.display_label.clone();
    };
    if !citation
        .display_label
        .trim()
        .eq_ignore_ascii_case(source.record_type.trim())
    {
        return citation.display_label.clone();
    }
    if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(&source.content) {
        let rich = parsed
            .get("description")
            .and_then(serde_json::Value::as_str)
            .or_else(|| {
                parsed
                    .pointer("/code/text")
                    .and_then(serde_json::Value::as_str)
            })
            .map(str::trim)
            .filter(|s| !s.is_empty());
        if let Some(label) = rich {
            return label.to_owned();
        }
    }
    citation.display_label.clone()
}

fn is_generic_part_label(label: &str) -> bool {
    matches!(
        label.trim().to_ascii_lowercase().as_str(),
        "attachment" | "presentedform" | "text" | "note" | "body"
    )
}

fn clean_excerpt_quote(excerpt: &str, title: &str) -> String {
    let normalized_title = title.trim();
    let mut remaining = excerpt.trim();
    while let Some((first_line, rest)) = remaining.split_once('\n') {
        let candidate = first_line.trim();
        if candidate.is_empty()
            || is_generic_part_label(candidate)
            || (!normalized_title.is_empty() && candidate.eq_ignore_ascii_case(normalized_title))
        {
            remaining = rest.trim();
        } else {
            break;
        }
    }
    remaining.to_owned()
}

fn humanize_json_key(key: &str) -> String {
    let replaced = key.replace(['_', '-'], " ");
    let collapsed = replaced.split_whitespace().collect::<Vec<_>>().join(" ");
    let mut chars = collapsed.chars();
    match chars.next() {
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
        None => String::new(),
    }
}

fn render_json_lines(prefix: Option<&str>, value: &serde_json::Value, out: &mut Vec<String>) {
    match value {
        serde_json::Value::Null => {}
        serde_json::Value::Bool(flag) => {
            let word = if *flag { "Yes" } else { "No" };
            match prefix {
                Some(key) if !key.is_empty() => out.push(format!("{key}: {word}")),
                Some(_) | None => out.push(word.to_owned()),
            }
        }
        serde_json::Value::Number(num) => match prefix {
            Some(key) if !key.is_empty() => out.push(format!("{key}: {num}")),
            Some(_) | None => out.push(num.to_string()),
        },
        serde_json::Value::String(text) => {
            let trimmed = text.trim();
            if !trimmed.is_empty() {
                match prefix {
                    Some(key) if !key.is_empty() => out.push(format!("{key}: {trimmed}")),
                    Some(_) | None => out.push(trimmed.to_owned()),
                }
            }
        }
        serde_json::Value::Array(items) => {
            let all_scalar = items.iter().all(|item| {
                matches!(
                    item,
                    serde_json::Value::String(_)
                        | serde_json::Value::Number(_)
                        | serde_json::Value::Bool(_)
                )
            });
            if all_scalar {
                let joined: Vec<String> = items
                    .iter()
                    .filter_map(|item| match item {
                        serde_json::Value::String(s) if !s.trim().is_empty() => {
                            Some(s.trim().to_owned())
                        }
                        serde_json::Value::Number(n) => Some(n.to_string()),
                        serde_json::Value::Bool(b) => {
                            Some(if *b { "Yes" } else { "No" }.to_owned())
                        }
                        serde_json::Value::Null
                        | serde_json::Value::String(_)
                        | serde_json::Value::Array(_)
                        | serde_json::Value::Object(_) => None,
                    })
                    .collect();
                if !joined.is_empty() {
                    let list = joined.join(", ");
                    match prefix {
                        Some(key) if !key.is_empty() => out.push(format!("{key}: {list}")),
                        Some(_) | None => out.push(list),
                    }
                }
            } else {
                for item in items {
                    render_json_lines(prefix, item, out);
                }
            }
        }
        serde_json::Value::Object(map) => {
            for (key, nested) in map {
                let label = humanize_json_key(key);
                let full_key = match prefix {
                    Some(parent) if !parent.is_empty() && !label.is_empty() => {
                        format!("{parent} · {label}")
                    }
                    Some(parent) if !parent.is_empty() => parent.to_owned(),
                    Some(_) | None => label,
                };
                render_json_lines(Some(&full_key), nested, out);
            }
        }
    }
}

fn format_structured_content(raw_content: &str) -> Option<String> {
    let trimmed = raw_content.trim();
    if trimmed.is_empty() {
        return None;
    }
    let parsed = serde_json::from_str::<serde_json::Value>(trimmed).ok()?;
    let mut lines = Vec::new();
    render_json_lines(None, &parsed, &mut lines);
    (!lines.is_empty()).then(|| lines.join("\n"))
}

fn build_cited_body(
    title: &str,
    cleaned_quote: &str,
    retained: Option<&SourceRevision>,
) -> (String, Option<String>, Option<String>) {
    let Some(source) = retained else {
        return (cleaned_quote.to_owned(), None, None);
    };
    let normalized_title = title.trim();
    let sections: Vec<String> = source
        .readable_parts
        .iter()
        .filter_map(|part| {
            let label = part.label.trim();
            let text = part.text.trim();
            let same_as_title = !label.is_empty() && label.eq_ignore_ascii_case(normalized_title);
            if label.is_empty() || same_as_title || is_generic_part_label(label) {
                (!text.is_empty()).then(|| text.to_owned())
            } else if text.is_empty() {
                Some(label.to_owned())
            } else {
                Some(format!("{label}\n{text}"))
            }
        })
        .collect();
    let body = if !sections.is_empty() {
        sections.join("\n\n")
    } else if let Some(formatted_json) = format_structured_content(&source.content) {
        formatted_json
    } else if !cleaned_quote.trim().is_empty() {
        cleaned_quote.to_owned()
    } else {
        source.content.trim().to_owned()
    };
    let raw_content = (!source.content.trim().is_empty()).then(|| source.content.clone());
    let record_type = (!source.record_type.trim().is_empty()).then(|| source.record_type.clone());
    (body, raw_content, record_type)
}

fn project_standing(
    standing: &VisitStanding,
    ends_at_unix: i64,
    ends_at: &str,
    appointment_status: Option<AppointmentStatus>,
    current_time_unix: i64,
) -> BriefingVisitStandingWire {
    match standing {
        VisitStanding::Current => {
            let status = appointment_status.unwrap_or(AppointmentStatus::Scheduled);
            if visit_can_start_preparation(status, ends_at_unix, current_time_unix) {
                BriefingVisitStandingWire::Current
            } else {
                BriefingVisitStandingWire::Ended {
                    ended_at: ends_at.to_owned(),
                }
            }
        }
        VisitStanding::Moved {
            new_starts_at,
            noted_at,
            ..
        } => BriefingVisitStandingWire::Moved {
            new_starts_at: new_starts_at.clone(),
            noted_at: format_unix_rfc3339(*noted_at),
        },
        VisitStanding::Cancelled { noted_at } => BriefingVisitStandingWire::Cancelled {
            noted_at: format_unix_rfc3339(*noted_at),
        },
    }
}

fn project_source_status(summary: &SourceRefreshStatusSummary) -> BriefingSourceStatusWire {
    match (summary.last_good_pulled_at, summary.last_failed_at) {
        (_, Some(failed_at)) => BriefingSourceStatusWire::RefreshFailed {
            source_id: summary.source_id.clone(),
            source_label: summary.source_label.clone(),
            last_good_at: summary.last_good_pulled_at.map(format_unix_rfc3339),
            failed_at: format_unix_rfc3339(failed_at),
            reason: summary.failure_reason.clone(),
        },
        (Some(last_good), None) => BriefingSourceStatusWire::Updated {
            source_id: summary.source_id.clone(),
            source_label: summary.source_label.clone(),
            last_good_at: format_unix_rfc3339(last_good),
        },
        (None, None) => BriefingSourceStatusWire::Unavailable {
            source_id: summary.source_id.clone(),
            source_label: summary.source_label.clone(),
        },
    }
}

fn project_limit(limit: &RecordLimit) -> BriefingLimitWire {
    let kind = match limit.kind {
        RecordLimitKind::Missing => BriefingLimitKindWire::Missing,
        RecordLimitKind::Unavailable => BriefingLimitKindWire::Unavailable,
        RecordLimitKind::Unreadable => BriefingLimitKindWire::Unreadable,
    };
    BriefingLimitWire {
        kind,
        detail: limit.detail.clone(),
    }
}

fn project_claim(
    claim: &BriefingClaimInput,
    index_by_ref: &BTreeMap<(String, String), CitationEntry>,
) -> BriefingClaimWire {
    BriefingClaimWire {
        segments: segments_with_citations(&claim.text, &claim.citations, index_by_ref),
        uncertainty: claim
            .uncertainty
            .as_deref()
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .map(str::to_owned),
    }
}

fn project_decision(
    decision: &BriefingDecisionInput,
    index_by_ref: &BTreeMap<(String, String), CitationEntry>,
) -> BriefingDecisionWire {
    BriefingDecisionWire {
        decision: decision.decision.clone(),
        rationale_segments: segments_with_citations(
            &decision.clinical_rationale,
            &decision.citations,
            index_by_ref,
        ),
        options_or_questions: decision.options_or_questions.clone(),
        uncertainty: decision
            .uncertainty
            .as_deref()
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .map(str::to_owned),
    }
}

fn segments_with_citations(
    prose: &str,
    citations: &[ExactRecordReference],
    index_by_ref: &BTreeMap<(String, String), CitationEntry>,
) -> Vec<MessageSegment> {
    let mut segments = Vec::new();
    let trimmed = prose.trim();
    if !trimmed.is_empty() {
        segments.push(MessageSegment::Text {
            text: trimmed.to_owned(),
        });
    }
    for reference in citations {
        let Some(entry) =
            index_by_ref.get(&(reference.candidate_id.clone(), reference.revision.clone()))
        else {
            continue;
        };
        segments.push(MessageSegment::Text {
            text: " ".to_owned(),
        });
        segments.push(MessageSegment::Citation {
            source_key: entry.source_key.clone(),
            number: entry.number,
            label: entry.label.clone(),
            quote: entry.quote.clone(),
        });
    }
    segments
}

#[cfg(test)]
mod tests {
    use super::*;
    use local_product::physician_sources::ReadablePart;
    use serde_json::json;

    #[test]
    fn generic_part_labels_are_omitted_and_structured_json_renders_key_value_lines() {
        let doc_source = SourceRevision {
            workspace: "pw-1".to_owned(),
            source_id: "synthetic-charts".to_owned(),
            candidate_id: "DocumentReference/doc-1".to_owned(),
            revision: "rev-1".to_owned(),
            record_type: "DocumentReference".to_owned(),
            dates: Vec::new(),
            content: json!({"resourceType": "DocumentReference", "id": "doc-1"}).to_string(),
            sha256: "sha-1".to_owned(),
            readable_parts: vec![
                ReadablePart {
                    label: "attachment".to_owned(),
                    text: "Discharged on oral torsemide 20 mg daily.".to_owned(),
                },
                ReadablePart {
                    label: "presentedForm".to_owned(),
                    text: "Follow up BMP in 72 hours.".to_owned(),
                },
            ],
            provenance: json!({}),
            envelope: None,
        };
        let quote = clean_excerpt_quote(
            "attachment\nDischarged on oral torsemide 20 mg daily.",
            "Discharge Summary",
        );
        assert_eq!(quote, "Discharged on oral torsemide 20 mg daily.");
        let (body, _, _) = build_cited_body("Discharge Summary", &quote, Some(&doc_source));
        assert_eq!(
            body,
            "Discharged on oral torsemide 20 mg daily.\n\nFollow up BMP in 72 hours."
        );

        let obs_source = SourceRevision {
            workspace: "pw-1".to_owned(),
            source_id: "synthetic-charts".to_owned(),
            candidate_id: "Observation/obs-1".to_owned(),
            revision: "rev-2".to_owned(),
            record_type: "Observation".to_owned(),
            dates: Vec::new(),
            content: json!({
                "resourceType": "Observation",
                "status": "final",
                "code": { "text": "Serum creatinine" },
                "valueQuantity": { "value": 1.42, "unit": "mg/dL" }
            })
            .to_string(),
            sha256: "sha-2".to_owned(),
            readable_parts: Vec::new(),
            provenance: json!({}),
            envelope: None,
        };
        let (obs_body, _, _) =
            build_cited_body("Serum creatinine", "Truncated…", Some(&obs_source));
        assert!(
            obs_body.contains("Code · Text: Serum creatinine"),
            "{obs_body}"
        );
        assert!(
            obs_body.contains("ValueQuantity · Value: 1.42"),
            "{obs_body}"
        );
        assert!(!obs_body.starts_with('{'), "{obs_body}");
    }
}
