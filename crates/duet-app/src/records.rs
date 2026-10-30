//! You → "All records": the complete exact inventory, paged by server cursor and
//! never truncated. Each row is a person-facing record: title, literal value and
//! unit, date or interval with its zone, a plain source, and a source-declared
//! status. Selecting a record sends that exact revision with the next message.
//! The page cursor and health-records generation stay on this client; they are
//! not part of the send.
use dioxus::prelude::*;
use duet_wire::models::{
    EvidenceBrowseRecord, EvidenceRevisionView, evidence_browse_record::Status as BrowseStatus,
    evidence_revision_view::Status as RevisionStatus,
};

/// Shown when a record has no declared date. Never replaced by an import time.
pub const DATE_UNKNOWN: &str = "Date unknown";
/// Shown on a source-removed record. It stays listed and citations still reopen it.
pub const REMOVED_BY_SOURCE: &str = "Removed by source";
/// Shown when a citation names a revision that has since been corrected.
pub const NEWER_VERSION: &str = "This is the cited version. A newer version of this record exists.";
/// Shown when a citation names a record its source has since removed.
pub const CITED_REMOVED: &str =
    "This is the cited version. The source has since removed this record.";
/// Shown when privacy erasure removed a cited record.
pub const NO_LONGER_AVAILABLE: &str = "This record is no longer available.";

/// One exact record revision selected for chat.
///
/// `evidence_revision` and `health_records_generation` remember the browse page.
/// The message sends only the record id and revision.
#[derive(Clone, Debug, PartialEq)]
pub struct RecordSelection {
    /// Canonical record identity.
    pub record_id: uuid::Uuid,
    /// Exact immutable revision the person reviewed.
    pub revision: i32,
    /// Visible title of that revision.
    pub title: String,
    /// Account evidence cursor of the page the person reviewed.
    pub evidence_revision: i64,
    /// Health Records withdrawal generation of that page.
    pub health_records_generation: i32,
}

impl RecordSelection {
    /// Chip text for the composer. Names the record; the revision is sent, not shown.
    pub fn chip(&self) -> String {
        format!("Asking about {}", self.title)
    }
}

/// Root-owned chat handoff shared by You and the conversation tab.
#[derive(Clone, Copy)]
pub struct ChatHandoff {
    /// The record awaiting the next send, if any.
    pub selected: Signal<Option<RecordSelection>>,
    /// Root tab selector; selection switches to the conversation tab.
    pub tab: Signal<usize>,
}

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

/// The notice above a reopened cited revision, if any.
pub fn revision_notice(view: &EvidenceRevisionView) -> Option<&'static str> {
    match view.status {
        RevisionStatus::Current => None,
        RevisionStatus::Superseded => Some(NEWER_VERSION),
        RevisionStatus::RemovedBySource => Some(CITED_REMOVED),
        RevisionStatus::Unavailable => Some(NO_LONGER_AVAILABLE),
    }
}

/// Text body for a reopened cited revision: the status notice, then the same
/// person-facing lines as You. The exact payload stays on the revision for its
/// owner and is not rendered here.
pub fn revision_text(view: &EvidenceRevisionView) -> String {
    if view.status == RevisionStatus::Unavailable {
        return NO_LONGER_AVAILABLE.to_owned();
    }
    let Some(payload) = view.payload.clone().flatten() else {
        return revision_notice(view)
            .unwrap_or(NO_LONGER_AVAILABLE)
            .to_owned();
    };
    let (date, date_end) = declared_span(&payload);
    let (value, unit) = value_from_map(&payload);
    let lines = person_lines(Shown {
        removed: false,
        value,
        unit,
        time_zone: zone_from_map(&payload, date.as_deref()),
        date,
        date_end,
        source: source_from_map(&payload),
        source_status: source_status_from_map(&payload),
    });
    let body = lines.join("\n");
    match revision_notice(view) {
        Some(notice) => format!("{notice}\n\n{body}"),
        None => body,
    }
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
    if let Some(status) = shown
        .source_status
        .filter(|status| !status.trim().is_empty())
    {
        lines.push(status);
    }
    lines
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

fn source_from_map(payload: &Payload) -> Option<String> {
    if let Some(label) = text_at(payload, &["source_label"]) {
        return Some(label.to_owned());
    }
    if let Some(name) = pointer_str(payload, &["source", "source_name"]) {
        return Some(name.to_owned());
    }
    plain_kind(pointer_str(payload, &["source", "kind"]).unwrap_or("")).map(str::to_owned)
}

fn source_status_from_map(payload: &Payload) -> Option<String> {
    text_at(payload, &["status"])
        .or_else(|| pointer_str(payload, &["clinical_record", "record_status"]))
        .map(str::to_owned)
}

type Payload = std::collections::HashMap<String, serde_json::Value>;

/// Start and end using the browse date rule: projected role, then common fields,
/// then an RFC 3339 `measured_interval`.
fn declared_span(payload: &Payload) -> (Option<String>, Option<String>) {
    if let Some(role) = pointer_str(payload, &["source", "projected_date_role"])
        && let Some(date) = payload
            .get("source")
            .and_then(|source| source.get("declared_dates"))
            .and_then(|dates| dates.get(role))
            .and_then(serde_json::Value::as_str)
    {
        return (Some(date.to_owned()), None);
    }
    for field in [
        "measured_at",
        "recorded_at",
        "occurred_at",
        "effective_at",
        "date",
    ] {
        if let Some(date) = text_at(payload, &[field]) {
            return (Some(date.to_owned()), None);
        }
    }
    let Some(start) =
        pointer_str(payload, &["measured_interval", "start"]).filter(|start| is_instant(start))
    else {
        return (None, None);
    };
    let end = pointer_str(payload, &["measured_interval", "end"])
        .filter(|end| *end != start && is_instant(end));
    (Some(start.to_owned()), end.map(str::to_owned))
}

fn is_instant(text: &str) -> bool {
    text.parse::<chrono::DateTime<chrono::Utc>>().is_ok()
}

fn zone_from_map(payload: &Payload, date: Option<&str>) -> Option<String> {
    if let Some(zone) = text_at(payload, &["time_zone"]) {
        return Some(zone.to_owned());
    }
    if let Some(zone) = pointer_str(payload, &["source", "time_zone"]) {
        return Some(zone.to_owned());
    }
    if let Some(zone) = text_at(payload, &["timezone"]) {
        return Some(zone.to_owned());
    }
    let date = date?;
    let parsed = chrono::DateTime::parse_from_rfc3339(date).ok()?;
    if date.ends_with('Z') || date.ends_with('z') {
        return Some("UTC".into());
    }
    Some(parsed.offset().to_string())
}

fn value_from_map(payload: &Payload) -> (Option<String>, Option<String>) {
    match payload.get("value") {
        Some(serde_json::Value::Object(value)) => {
            let shown = value
                .get("literal")
                .or_else(|| value.get("value"))
                .and_then(json_text);
            let unit = value
                .get("unit")
                .and_then(serde_json::Value::as_str)
                .map(str::to_owned);
            (shown, unit)
        }
        Some(other) if !other.is_null() => (
            json_text(other),
            text_at(payload, &["unit"]).map(str::to_owned),
        ),
        _ => (text_at(payload, &["body"]).map(str::to_owned), None),
    }
}

fn json_text(value: &serde_json::Value) -> Option<String> {
    match value {
        serde_json::Value::Null => None,
        serde_json::Value::String(text) => Some(text.clone()),
        serde_json::Value::Number(number) => Some(number.to_string()),
        serde_json::Value::Bool(flag) => Some(flag.to_string()),
        other => Some(other.to_string()),
    }
}

fn text_at<'a>(payload: &'a Payload, path: &[&str]) -> Option<&'a str> {
    pointer_str(payload, path).filter(|text| !text.trim().is_empty())
}

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
    let handoff = try_use_context::<ChatHandoff>();
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
                            let lines = record_lines(&record);
                            let title = record.title.clone();
                            let selection = RecordSelection {
                                record_id: id,
                                revision: record.address.revision,
                                title: title.clone(),
                                evidence_revision: state.evidence_revision,
                                health_records_generation: state.generation,
                            };
                            let current = record.status == BrowseStatus::Current;
                            rsx! {
                                article { class: "record-row", "data-record": "{id}",
                                    p { class: "card-text", "{title}" }
                                    for line in lines { p { class: "record-meta", "{line}" } }
                                    if current && handoff.is_some() {
                                        button {
                                            class: "duet-retry",
                                            r#type: "button",
                                            aria_label: "Ask about {title}",
                                            onclick: move |_| {
                                                if let Some(mut handoff) = handoff {
                                                    handoff.selected.set(Some(selection.clone()));
                                                    handoff.tab.set(0);
                                                }
                                            },
                                            "Ask about this"
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
            lines.iter().any(|line| line == "entered-in-error"),
            "{lines:?}"
        );
        assert!(!lines.iter().any(|line| line.contains("HKQuantitySample")
            || line.contains("vital:steps")
            || line.contains("phone")));
        hides_inspector(&lines);
    }

    #[test]
    fn cited_revisions_name_their_status() {
        let address =
            CanonicalSourceAddress::new(uuid::Uuid::nil(), "id".into(), Entity::ClinicalNote, 1);
        let mut view = EvidenceRevisionView::new(address, RevisionStatus::Superseded, false);
        view.payload = Some(Some(
            [("body".to_owned(), serde_json::json!("Original"))].into(),
        ));
        let text = revision_text(&view);
        assert!(text.starts_with(NEWER_VERSION), "{text}");
        assert!(text.contains("Original"), "{text}");
        assert!(!text.contains('{'), "{text}");
        view.status = RevisionStatus::RemovedBySource;
        assert!(revision_text(&view).starts_with(CITED_REMOVED));
        view.status = RevisionStatus::Unavailable;
        view.payload = Some(None);
        assert_eq!(revision_text(&view), NO_LONGER_AVAILABLE);
        view.status = RevisionStatus::Current;
        view.payload = Some(Some(Default::default()));
        assert_eq!(revision_notice(&view), None);
    }

    #[test]
    fn cited_acquisition_interval_keeps_the_range_without_json() {
        let address = CanonicalSourceAddress::new(
            uuid::Uuid::nil(),
            "HKQuantitySample/steps-day".into(),
            Entity::HealthEvidence,
            1,
        );
        let mut view = EvidenceRevisionView::new(address, RevisionStatus::Current, true);
        view.payload = Some(Some(
            serde_json::from_value(acquisition_interval()).unwrap(),
        ));
        let text = revision_text(&view);
        assert!(text.contains("7654 count"), "{text}");
        assert!(
            text.contains(
                "2026-10-28T00:00:00-07:00 to 2026-10-29T00:00:00-07:00 (America/Los_Angeles)"
            ),
            "{text}"
        );
        assert!(text.contains("Source: Apple Watch"), "{text}");
        assert!(text.contains("entered-in-error"), "{text}");
        assert!(!text.contains("HKQuantitySample"), "{text}");
        assert!(!text.contains("vital:steps"), "{text}");
        assert!(!text.contains('{'), "{text}");
    }
}
