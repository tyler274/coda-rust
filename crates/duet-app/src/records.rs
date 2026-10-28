//! You → "All records": the complete exact inventory, paged by server cursor and
//! never truncated, with provenance, literal values/units/zones and truthful status.
//! Selecting a record hands its exact revision and evidence window to chat.
use dioxus::prelude::*;
use duet_wire::models::{
    EvidenceBrowseRecord, EvidenceRevisionView, evidence_browse_record::Status as BrowseStatus,
    evidence_record_provenance::Writer, evidence_revision_view::Status as RevisionStatus,
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

/// One exact record revision selected for chat, with the evidence window it was read in.
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
    /// Chip text for the composer. Names the exact revision.
    pub fn chip(&self) -> String {
        format!("Asking about {} (version {})", self.title, self.revision)
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

/// The lines shown for one record, in display order. Pure for unit testing.
pub fn record_lines(record: &EvidenceBrowseRecord) -> Vec<String> {
    let mut lines = Vec::new();
    if record.status == BrowseStatus::RemovedBySource {
        lines.push(REMOVED_BY_SOURCE.to_owned());
    }
    match (&record.value, &record.unit) {
        (Some(value), Some(unit)) => lines.push(format!("{value} {unit}")),
        (Some(value), None) => lines.push(value.clone()),
        _ => {}
    }
    lines.push(match (&record.date, &record.date_role) {
        (Some(date), Some(role)) => match &record.time_zone {
            Some(zone) => format!("{date} ({role}, {zone})"),
            None => format!("{date} ({role})"),
        },
        (Some(date), None) => date.clone(),
        _ => DATE_UNKNOWN.to_owned(),
    });
    let p = &record.provenance;
    let writer = match p.writer {
        Writer::Connector => "connected portal",
        Writer::Device => "this device",
    };
    let mut source = format!("Source: {} via {writer}", p.source_kind);
    if let Some(connection) = &p.connection_id {
        source.push_str(&format!(" · connection {connection}"));
    }
    if let Some(original) = &p.source_record_id {
        source.push_str(&format!(" · original id {original}"));
    }
    lines.push(source);
    if let Some(status) = &record.source_status {
        lines.push(format!("Source status: {status}"));
    }
    lines.push(format!("Version {}", record.address.revision));
    lines
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

/// Text body for a reopened cited revision: notice, then the exact original payload.
pub fn revision_text(view: &EvidenceRevisionView) -> String {
    let payload = view
        .payload
        .clone()
        .flatten()
        .map(|p| serde_json::to_string_pretty(&p).unwrap_or_default());
    match (revision_notice(view), payload) {
        (Some(notice), Some(payload)) => format!("{notice}\n\n{payload}"),
        (Some(notice), None) => notice.to_owned(),
        (None, Some(payload)) => payload,
        (None, None) => NO_LONGER_AVAILABLE.to_owned(),
    }
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
    let mut opened = use_signal(|| None::<uuid::Uuid>);
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
                            let is_open = opened() == Some(id);
                            let selection = RecordSelection {
                                record_id: id,
                                revision: record.address.revision,
                                title: title.clone(),
                                evidence_revision: state.evidence_revision,
                                health_records_generation: state.generation,
                            };
                            let current = record.status == BrowseStatus::Current;
                            let payload = record.payload.clone().map(|p| serde_json::to_string_pretty(&p).unwrap_or_default()).unwrap_or_default();
                            rsx! {
                                article { class: "record-row", "data-record": "{id}",
                                    p { class: "card-text", "{title}" }
                                    for line in lines { p { class: "record-meta", "{line}" } }
                                    button {
                                        class: "duet-retry",
                                        r#type: "button",
                                        aria_label: "Original record {title}",
                                        onclick: move |_| opened.set(if is_open { None } else { Some(id) }),
                                        if is_open { "Hide original" } else { "Show original" }
                                    }
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
                                    if is_open { pre { class: "record-original", "{payload}" } }
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
    };

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

    #[test]
    fn undated_records_say_date_unknown() {
        let lines = record_lines(&record(BrowseStatus::Current));
        assert!(lines.contains(&DATE_UNKNOWN.to_owned()), "{lines:?}");
        assert!(!lines.contains(&REMOVED_BY_SOURCE.to_owned()));
        assert!(lines.contains(&"Version 3".to_owned()));
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
        let lines = record_lines(&r);
        assert_eq!(lines[0], REMOVED_BY_SOURCE);
        assert_eq!(lines[1], "4.1 mmol/L");
        assert_eq!(lines[2], "2026-09-30T08:15:00-07:00 (measured_at, -07:00)");
        assert!(lines[3].contains("epic via connected portal · connection conn-1"));
    }

    #[test]
    fn cited_revisions_name_their_status() {
        let address =
            CanonicalSourceAddress::new(uuid::Uuid::nil(), "id".into(), Entity::ClinicalNote, 1);
        let mut view = EvidenceRevisionView::new(address, RevisionStatus::Superseded, false);
        view.payload = Some(Some(
            [("body".to_owned(), serde_json::json!("Original"))].into(),
        ));
        assert!(revision_text(&view).starts_with(NEWER_VERSION));
        assert!(revision_text(&view).contains("Original"));
        view.status = RevisionStatus::RemovedBySource;
        assert!(revision_text(&view).starts_with(CITED_REMOVED));
        view.status = RevisionStatus::Unavailable;
        view.payload = Some(None);
        assert_eq!(revision_text(&view), NO_LONGER_AVAILABLE);
        view.status = RevisionStatus::Current;
        view.payload = Some(Some(Default::default()));
        assert_eq!(revision_notice(&view), None);
    }
}
