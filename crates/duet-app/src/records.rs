//! You → "All records": the complete exact inventory, paged by server cursor and
//! never truncated. Each row is a person-facing record: title, literal value and
//! unit, date or interval with its zone, a plain source, and a source-declared
//! status.
use dioxus::prelude::*;
use duet_client::for_my_doctor::{MarkForDoctorRequest, MarkTargetKind, RecordFamily};
use duet_wire::models::{
    EvidenceBrowseRecord, canonical_source_address::Entity as BrowseEntity,
    evidence_browse_record::Status as BrowseStatus,
};
use ui::ActionRow;

use crate::for_my_doctor::{MarkToggleButton, record_target_key};

/// Shown when a record has no declared date. Never replaced by an import time.
pub const DATE_UNKNOWN: &str = "Date unknown";
/// Shown on a source-removed record. It stays listed.
pub const REMOVED_BY_SOURCE: &str = "Removed by source";

/// The lines shown under a record's title, in display order. Pure for unit testing.
pub fn record_lines(record: &EvidenceBrowseRecord) -> Vec<String> {
    person_lines(Shown {
        removed: record.status == BrowseStatus::RemovedBySource,
        value: record.value.clone(),
        unit: record.unit.clone(),
        date: record.date.clone(),
        date_end: record.date_end.clone(),
        time_zone: record.time_zone.clone(),
        source: source_from_record(record),
        source_status: record.source_status.clone(),
    })
}

struct Shown {
    removed: bool,
    value: Option<String>,
    unit: Option<String>,
    date: Option<String>,
    date_end: Option<String>,
    time_zone: Option<String>,
    source: Option<String>,
    source_status: Option<String>,
}

fn person_lines(shown: Shown) -> Vec<String> {
    let mut lines = Vec::new();
    if shown.removed {
        lines.push(REMOVED_BY_SOURCE.to_owned());
    }
    match (shown.value, shown.unit) {
        (Some(value), Some(unit)) => lines.push(format!("{value} {unit}")),
        (Some(value), None) => lines.push(value),
        _ => {}
    }
    lines.push(when_line(
        shown.date.as_deref(),
        shown.date_end.as_deref(),
        shown.time_zone.as_deref(),
    ));
    if let Some(source) = shown.source {
        lines.push(format!("Source: {source}"));
    }
    if let Some(status) = shown.source_status.as_deref().and_then(source_status_line) {
        lines.push(status);
    }
    lines
}

/// Plain-language explanation for a source-declared status, preserving the literal status.
pub fn source_status_line(status: &str) -> Option<String> {
    let raw = status.trim();
    if raw.is_empty() {
        return None;
    }
    let normalized = raw.to_ascii_lowercase().replace([' ', '_'], "-");
    let explanation = match normalized.as_str() {
        "entered-in-error" => "Marked by the source as recorded by mistake.",
        "cancelled" | "canceled" => "Cancelled at the source and did not happen.",
        "corrected" | "amended" => "Updated by the source after it was first recorded.",
        "preliminary" => "Preliminary result; the source may still update it.",
        _ => return Some(raw.to_owned()),
    };
    Some(format!("{raw} — {explanation}"))
}

/// Date or interval. The zone is appended when it is not already in the literal.
fn when_line(start: Option<&str>, end: Option<&str>, zone: Option<&str>) -> String {
    let Some(start) = start.map(str::trim).filter(|start| !start.is_empty()) else {
        return DATE_UNKNOWN.to_owned();
    };
    let span = match end
        .map(str::trim)
        .filter(|end| !end.is_empty() && *end != start)
    {
        Some(end) => format!("{start} to {end}"),
        None => start.to_owned(),
    };
    match zone
        .map(str::trim)
        .filter(|zone| !zone.is_empty() && !span.contains(*zone))
    {
        Some(zone) => format!("{span} ({zone})"),
        None => span,
    }
}

fn plain_kind(kind: &str) -> Option<&'static str> {
    Some(match kind {
        "appleHealth" => "Apple Health",
        "healthRecords" => "Health Records",
        "manual" => "Manual",
        "physical" => "Physical",
        "epic" => "Epic",
        "oracleHealth" => "Oracle Health",
        "healow" => "healow",
        "googleHealth" => "Google Health",
        "note" => "Note",
        _ => return None,
    })
}

fn source_from_record(record: &EvidenceBrowseRecord) -> Option<String> {
    if let Some(label) = record
        .provenance
        .source_label
        .as_deref()
        .map(str::trim)
        .filter(|label| !label.is_empty())
    {
        return Some(label.to_owned());
    }
    if let Some(payload) = record.payload.as_ref()
        && let Some(name) = pointer_str(payload, &["source", "source_name"])
    {
        return Some(name.to_owned());
    }
    plain_kind(&record.provenance.source_kind).map(str::to_owned)
}

type Payload = std::collections::HashMap<String, serde_json::Value>;

fn pointer_str<'a>(payload: &'a Payload, path: &[&str]) -> Option<&'a str> {
    let (first, rest) = path.split_first()?;
    let mut current = payload.get(*first)?;
    for key in rest {
        current = current.get(*key)?;
    }
    current.as_str()
}

#[derive(Clone, PartialEq)]
struct Loaded {
    records: Vec<EvidenceBrowseRecord>,
    total: i32,
    removed: i32,
    next: Option<String>,
    evidence_revision: i64,
    generation: i32,
}

/// Complete record browser. Loads the first page, then "Show more records" follows the
/// server cursor until the inventory is exhausted; a changed snapshot restarts.
#[component]
pub fn RecordBrowser(base_url: String, bearer: String, reload: Signal<u32>) -> Element {
    let mut loaded = use_signal(|| None::<Loaded>);
    let mut failed = use_signal(|| false);
    let mut busy = use_signal(|| false);
    let first_url = base_url.clone();
    let first_bearer = bearer.clone();
    let _first = use_resource(move || {
        let _ = reload();
        let url = first_url.clone();
        let bearer = first_bearer.clone();
        async move {
            busy.set(true);
            let page = match crate::open_client(url, bearer) {
                Ok(api) => api.browse_records(None).await,
                Err(error) => Err(error),
            };
            match page {
                Ok(page) => {
                    failed.set(false);
                    loaded.set(Some(Loaded {
                        total: page.total_count,
                        removed: page.removed_count.unwrap_or(0),
                        next: page.next_cursor,
                        evidence_revision: page.revision,
                        generation: page.health_records_generation,
                        records: page.records,
                    }));
                }
                Err(_) => failed.set(true),
            }
            busy.set(false);
        }
    });
    let more_url = base_url.clone();
    let more_bearer = bearer.clone();
    let show_more = move |_| {
        let Some(current) = loaded() else { return };
        let Some(cursor) = current.next.clone() else {
            return;
        };
        if busy() {
            return;
        }
        busy.set(true);
        let url = more_url.clone();
        let bearer = more_bearer.clone();
        spawn(async move {
            let page = match crate::open_client(url, bearer) {
                Ok(api) => match api.browse_records(Some(cursor)).await {
                    // The snapshot changed underneath the cursor: restart, never splice.
                    Err(_) => api.browse_records(None).await.map(|p| (p, true)),
                    Ok(p) => Ok((p, false)),
                },
                Err(error) => Err(error),
            };
            match page {
                Ok((page, restarted)) => {
                    let mut records = if restarted {
                        Vec::new()
                    } else {
                        current.records.clone()
                    };
                    records.extend(page.records);
                    loaded.set(Some(Loaded {
                        records,
                        total: page.total_count,
                        removed: page.removed_count.unwrap_or(0),
                        next: page.next_cursor,
                        evidence_revision: page.revision,
                        generation: page.health_records_generation,
                    }));
                    failed.set(false);
                }
                Err(_) => failed.set(true),
            }
            busy.set(false);
        });
    };
    rsx! {
        details { class: "record-browser",
            summary { "All records" }
            match loaded() {
                None if failed() => rsx! { p { class: "empty", role: "alert", "Records couldn’t load." } },
                None => rsx! { p { class: "record-meta", "Loading records…" } },
                Some(state) => rsx! {
                    p { class: "record-meta", role: "status",
                        "Showing {state.records.len()} of {state.total} records"
                        if state.removed > 0 { " · {state.removed} removed by source" }
                    }
                    if state.total == 0 { p { class: "empty", "No records yet." } }
                    for record in state.records.clone() {
                        {
                            let id = record.address.canonical_record_id;
                            let id_str = id.to_string();
                            let target_key = record_target_key(&id_str);
                            let lines = record_lines(&record);
                            let title = record.title.clone();
                            let can_mark = record.status != BrowseStatus::RemovedBySource;
                            let entity = match record.address.entity {
                                BrowseEntity::HealthEvidence => RecordFamily::HealthEvidence,
                                BrowseEntity::ClinicalNote => RecordFamily::ClinicalNote,
                            };
                            let mark_req = MarkForDoctorRequest {
                                target_kind: MarkTargetKind::Record,
                                target_key: target_key.clone(),
                                record_id: Some(id),
                                entity: Some(entity),
                                title: title.clone(),
                                summary_text: match (record.value.as_deref(), record.unit.as_deref()) {
                                    (Some(v), Some(u)) => format!("{v} {u}"),
                                    (Some(v), None) => v.to_owned(),
                                    _ => String::new(),
                                },
                                range_label: None,
                                source_label: source_from_record(&record),
                                payload: serde_json::json!({
                                    "date": record.date,
                                    "source_status": record.source_status,
                                }),
                            };
                            rsx! {
                                article { class: "record-row", "data-record": "{id}",
                                    p { class: "card-text", "{title}" }
                                    for line in lines { p { class: "record-meta", "{line}" } }
                                    if can_mark {
                                        ActionRow {
                                            MarkToggleButton {
                                                target_key,
                                                request: mark_req,
                                                base_url: base_url.clone(),
                                                bearer: bearer.clone(),
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    if state.next.is_some() {
                        button { class: "duet-retry", r#type: "button", disabled: busy(), onclick: show_more, "Show more records" }
                    }
                    if failed() { p { class: "empty", role: "alert", "More records couldn’t load. Try again." } }
                },
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use duet_wire::models::{
        CanonicalSourceAddress, EvidenceRecordProvenance, canonical_source_address::Entity,
        evidence_record_provenance::Writer,
    };

    fn acquisition_interval() -> serde_json::Value {
        serde_json::json!({
            "concept_id": "vital:steps",
            "timezone": "America/Los_Angeles",
            "source": {
                "kind": "appleHealth",
                "record_id": "HKQuantitySample/steps-day",
                "data_type": "HKQuantityTypeIdentifierStepCount",
                "source_name": "Apple Watch",
                "writer_id": "com.apple.health.ABC",
                "device": "Watch7,1"
            },
            "value": {"literal": "7654", "unit": "count"},
            "measured_interval": {
                "start": "2026-10-28T00:00:00-07:00",
                "end": "2026-10-29T00:00:00-07:00"
            },
            "status": "entered-in-error"
        })
    }

    fn acquisition_record() -> EvidenceBrowseRecord {
        let payload: Payload = serde_json::from_value(acquisition_interval()).unwrap();
        let mut record = EvidenceBrowseRecord::new(
            CanonicalSourceAddress::new(
                uuid::Uuid::nil(),
                "HKQuantitySample/steps-day".into(),
                Entity::HealthEvidence,
                1,
            ),
            BrowseStatus::Current,
            "Health record".into(),
            EvidenceRecordProvenance::new("appleHealth".into(), Writer::Device),
            Some(payload),
        );
        record.date = Some("2026-10-28T00:00:00-07:00".into());
        record.date_end = Some("2026-10-29T00:00:00-07:00".into());
        record.time_zone = Some("America/Los_Angeles".into());
        record.value = Some("7654".into());
        record.unit = Some("count".into());
        record.source_status = Some("entered-in-error".into());
        record.provenance.source_record_id = Some("HKQuantitySample/steps-day".into());
        record.provenance.connection_id = Some("phone".into());
        record
    }

    fn record(status: BrowseStatus) -> EvidenceBrowseRecord {
        EvidenceBrowseRecord::new(
            CanonicalSourceAddress::new(
                uuid::Uuid::nil(),
                "literal/source/id".into(),
                Entity::HealthEvidence,
                3,
            ),
            status,
            "Potassium".into(),
            EvidenceRecordProvenance::new("epic".into(), Writer::Connector),
            Default::default(),
        )
    }

    fn hides_inspector(lines: &[String]) {
        let text = lines.join("\n");
        assert!(!text.contains("Version"), "{text}");
        assert!(!text.contains("connection"), "{text}");
        assert!(!text.contains('{'), "{text}");
        assert!(!text.contains("measured_at"), "{text}");
    }

    #[test]
    fn omitted_interval_end_stays_absent() {
        let json = serde_json::to_string(&record(BrowseStatus::Current)).unwrap();
        assert!(!json.contains("date_end"), "{json}");
        let back: EvidenceBrowseRecord = serde_json::from_str(&json).unwrap();
        assert!(back.date.is_none() && back.date_end.is_none());
    }

    #[test]
    fn undated_records_say_date_unknown() {
        let lines = record_lines(&record(BrowseStatus::Current));
        assert!(lines.contains(&DATE_UNKNOWN.to_owned()), "{lines:?}");
        assert!(!lines.contains(&REMOVED_BY_SOURCE.to_owned()));
        assert!(lines.iter().any(|line| line == "Source: Epic"));
        hides_inspector(&lines);
    }

    #[test]
    fn removed_records_are_labelled_with_literal_values_and_zone() {
        let mut r = record(BrowseStatus::RemovedBySource);
        r.value = Some("4.1".into());
        r.unit = Some("mmol/L".into());
        r.date = Some("2026-09-30T08:15:00-07:00".into());
        r.date_role = Some("measured_at".into());
        r.time_zone = Some("-07:00".into());
        r.provenance.connection_id = Some("conn-1".into());
        r.provenance.source_record_id = Some("native-1".into());
        let lines = record_lines(&r);
        assert_eq!(lines[0], REMOVED_BY_SOURCE);
        assert_eq!(lines[1], "4.1 mmol/L");
        assert_eq!(lines[2], "2026-09-30T08:15:00-07:00");
        assert!(lines.iter().any(|line| line == "Source: Epic"), "{lines:?}");
        hides_inspector(&lines);
        assert!(
            !lines
                .iter()
                .any(|line| line.contains("conn-1") || line.contains("native-1"))
        );
    }

    #[test]
    fn acquisition_interval_is_person_facing() {
        let lines = record_lines(&acquisition_record());
        assert_eq!(lines[0], "7654 count");
        assert_eq!(
            lines[1],
            "2026-10-28T00:00:00-07:00 to 2026-10-29T00:00:00-07:00 (America/Los_Angeles)"
        );
        assert!(
            lines.iter().any(|line| line == "Source: Apple Watch"),
            "{lines:?}"
        );
        assert!(
            lines
                .iter()
                .any(|line| line.contains("entered-in-error")
                    && line.contains("recorded by mistake")),
            "{lines:?}"
        );
        assert!(
            source_status_line("cancelled")
                .is_some_and(|line| line.contains("cancelled") && line.contains("did not happen"))
        );
        assert!(!lines.iter().any(|line| line.contains("HKQuantitySample")
            || line.contains("vital:steps")
            || line.contains("phone")));
        hides_inspector(&lines);
    }
}
