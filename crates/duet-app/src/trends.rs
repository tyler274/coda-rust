//! Lab and measurement trend views for You.
//!
//! Each source series renders as its own unspliced line (or nightly bars for sleep).
//! Multi-source metrics switch one source series at a time, blood pressure shows both
//! systolic and diastolic readings together, and lab printed reference bands/flags never appear.
use std::collections::BTreeMap;

use dioxus::prelude::*;
use duet_wire::models::{EvidenceBrowseRecord, EvidenceLibraryEntry};
use ui::{
    Button, ButtonVariant, Card, ChartHeader, Choice, Segmented, SeriesChart, SeriesPoint,
    SeriesShape,
};

use crate::roots::LibraryRecord;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TrendKind {
    Lab,
    BloodPressure,
    Sleep,
    Measurement,
}

impl TrendKind {
    pub fn chart_shape(self) -> SeriesShape {
        match self {
            Self::Sleep => SeriesShape::Bars,
            Self::Lab | Self::BloodPressure | Self::Measurement => SeriesShape::Line,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct TrendBucket {
    pub axis: String,
    pub value: f64,
    pub low: Option<f64>,
    pub high: Option<f64>,
    pub words: String,
    pub sort_key: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SourceSeries {
    pub source_label: String,
    pub unit: Option<String>,
    pub points: Vec<TrendBucket>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TrendView {
    pub title: String,
    pub kind: TrendKind,
    pub sources: Vec<SourceSeries>,
}

/// Whether `record` can open a lab or measurement trend view.
pub fn supports_trend(record: &LibraryRecord) -> bool {
    let cat = record.category.as_str();
    if matches!(cat, "lab" | "vital" | "measurement" | "sleep") {
        return true;
    }
    let lower = record.title.to_ascii_lowercase();
    lower.contains("blood pressure") || lower.contains("sleep")
}

/// Builds a strictly per-source trend view from loaded browse records and library entries.
/// Lines from different sources are never spliced together.
pub fn build_trend_view(
    anchor: &LibraryRecord,
    entries: &[EvidenceLibraryEntry],
    browse: &[EvidenceBrowseRecord],
) -> Option<TrendView> {
    let target_title = anchor.title.trim().to_ascii_lowercase();
    let kind = classify_trend(anchor);
    let mut by_source: BTreeMap<String, (Option<String>, Vec<TrendBucket>)> = BTreeMap::new();

    for rec in browse {
        if !rec.title.trim().eq_ignore_ascii_case(&target_title) {
            continue;
        }
        let source_label = browse_source_label(rec);
        let date_raw = rec.date.as_deref().unwrap_or("");
        if let Some((bucket, unit)) = extract_bucket(
            kind,
            rec.value.as_deref(),
            rec.unit.as_deref(),
            rec.payload.as_ref(),
            date_raw,
        ) {
            let entry = by_source.entry(source_label).or_insert((None, Vec::new()));
            if entry.0.is_none() {
                entry.0 = unit;
            }
            entry.1.push(bucket);
        }
    }

    if by_source.is_empty() {
        for entry in entries {
            if !entry.title.trim().eq_ignore_ascii_case(&target_title) {
                continue;
            }
            let source_label = entry
                .source_label
                .as_ref()
                .and_then(|s| s.as_deref())
                .map(str::trim)
                .filter(|s| !s.is_empty())
                .unwrap_or("Source")
                .to_owned();
            let date_raw = entry
                .date
                .map(|d| d.to_rfc3339())
                .or_else(|| entry.day.flatten().map(|d| d.to_string()))
                .unwrap_or_default();
            let summary = entry.summary.as_ref().and_then(|s| s.as_deref());
            if let Some((bucket, unit)) =
                extract_bucket(kind, summary, None, entry.preview.as_ref(), &date_raw)
            {
                let slot = by_source.entry(source_label).or_insert((None, Vec::new()));
                if slot.0.is_none() {
                    slot.0 = unit;
                }
                slot.1.push(bucket);
            }
        }
    }

    if by_source.is_empty() {
        return None;
    }

    let mut sources: Vec<SourceSeries> = by_source
        .into_iter()
        .map(|(source_label, (unit, mut points))| {
            points.sort_by(|a, b| a.sort_key.cmp(&b.sort_key));
            SourceSeries {
                source_label,
                unit,
                points,
            }
        })
        .collect();

    if let Some(anchor_source) = anchor.source.as_deref()
        && let Some(pos) = sources
            .iter()
            .position(|series| series.source_label.eq_ignore_ascii_case(anchor_source))
    {
        let first = sources.remove(pos);
        sources.insert(0, first);
    }

    Some(TrendView {
        title: anchor.title.clone(),
        kind,
        sources,
    })
}

fn classify_trend(anchor: &LibraryRecord) -> TrendKind {
    let lower = anchor.title.to_ascii_lowercase();
    if lower.contains("blood pressure") {
        TrendKind::BloodPressure
    } else if anchor.category == "sleep" || lower.contains("sleep") {
        TrendKind::Sleep
    } else if anchor.category == "lab" {
        TrendKind::Lab
    } else {
        TrendKind::Measurement
    }
}

fn browse_source_label(rec: &EvidenceBrowseRecord) -> String {
    if let Some(label) = rec
        .provenance
        .source_label
        .as_deref()
        .map(str::trim)
        .filter(|s| !s.is_empty())
    {
        return label.to_owned();
    }
    if let Some(payload) = rec.payload.as_ref()
        && let Some(name) = payload
            .get("source")
            .and_then(|s| s.get("source_name"))
            .and_then(serde_json::Value::as_str)
            .map(str::trim)
            .filter(|s| !s.is_empty())
    {
        return name.to_owned();
    }
    match rec.provenance.source_kind.as_str() {
        "appleHealth" => "Apple Health",
        "healthRecords" => "Health Records",
        "epic" => "Epic",
        "oracleHealth" => "Oracle Health",
        "healow" => "healow",
        "googleHealth" => "Google Health",
        "manual" => "Added in Duet",
        other if !other.trim().is_empty() => other,
        _ => "Source",
    }
    .to_owned()
}

fn extract_bucket(
    kind: TrendKind,
    raw_value: Option<&str>,
    raw_unit: Option<&str>,
    payload: Option<&std::collections::HashMap<String, serde_json::Value>>,
    date_raw: &str,
) -> Option<(TrendBucket, Option<String>)> {
    let axis = short_axis_date(date_raw);
    let sort_key = if date_raw.is_empty() {
        "~".to_owned()
    } else {
        date_raw.to_owned()
    };

    if kind == TrendKind::BloodPressure
        && let Some((sys, dia, unit)) = extract_blood_pressure(raw_value, raw_unit, payload)
    {
        let label_date = if axis.is_empty() { "Reading" } else { &axis };
        let unit_text = unit.as_deref().unwrap_or("mmHg");
        let words = format!("{label_date}: {sys}/{dia} {unit_text}");
        return Some((
            TrendBucket {
                axis,
                value: sys,
                low: Some(dia),
                high: Some(sys),
                words,
                sort_key,
            },
            Some(unit_text.to_owned()),
        ));
    }

    let (num, parsed_unit) = extract_numeric(raw_value, payload)?;
    let unit = raw_unit
        .map(str::trim)
        .filter(|u| !u.is_empty())
        .map(str::to_owned)
        .or(parsed_unit);
    let label_date = if axis.is_empty() {
        if kind == TrendKind::Sleep {
            "Night"
        } else {
            "Reading"
        }
    } else {
        &axis
    };
    let words = match unit.as_deref() {
        Some(unit) => format!("{label_date}: {num} {unit}"),
        None => format!("{label_date}: {num}"),
    };
    Some((
        TrendBucket {
            axis,
            value: num,
            low: None,
            high: None,
            words,
            sort_key,
        },
        unit,
    ))
}

fn extract_blood_pressure(
    raw_value: Option<&str>,
    raw_unit: Option<&str>,
    payload: Option<&std::collections::HashMap<String, serde_json::Value>>,
) -> Option<(f64, f64, Option<String>)> {
    if let Some(payload) = payload {
        let sys = payload.get("systolic").and_then(json_number).or_else(|| {
            payload
                .get("value")
                .and_then(|v| v.get("systolic"))
                .and_then(json_number)
        });
        let dia = payload.get("diastolic").and_then(json_number).or_else(|| {
            payload
                .get("value")
                .and_then(|v| v.get("diastolic"))
                .and_then(json_number)
        });
        if let (Some(sys), Some(dia)) = (sys, dia) {
            let unit = raw_unit
                .map(str::trim)
                .filter(|u| !u.is_empty())
                .map(str::to_owned)
                .or_else(|| {
                    payload
                        .get("unit")
                        .and_then(serde_json::Value::as_str)
                        .map(str::to_owned)
                });
            return Some((sys, dia, unit));
        }
    }
    let text = raw_value.or_else(|| {
        payload.and_then(|p| {
            p.get("value")
                .and_then(|v| v.get("literal").or(Some(v)))
                .and_then(serde_json::Value::as_str)
        })
    })?;
    let slash_pos = text.find('/')?;
    let left = first_number(&text[..slash_pos])?;
    let right = first_number(&text[slash_pos + 1..])?;
    let unit = raw_unit
        .map(str::trim)
        .filter(|u| !u.is_empty())
        .map(str::to_owned)
        .or_else(|| text.contains("mmHg").then(|| "mmHg".to_owned()));
    Some((left, right, unit))
}

fn extract_numeric(
    raw_value: Option<&str>,
    payload: Option<&std::collections::HashMap<String, serde_json::Value>>,
) -> Option<(f64, Option<String>)> {
    if let Some(payload) = payload {
        let val = payload.get("value");
        if let Some(num) = val.and_then(json_number) {
            let unit = payload
                .get("unit")
                .and_then(serde_json::Value::as_str)
                .map(str::to_owned);
            return Some((num, unit));
        }
        if let Some(obj) = val.and_then(serde_json::Value::as_object) {
            let num = obj.get("value").and_then(json_number).or_else(|| {
                obj.get("literal")
                    .and_then(serde_json::Value::as_str)
                    .and_then(first_number)
            });
            if let Some(num) = num {
                let unit = obj
                    .get("unit")
                    .and_then(serde_json::Value::as_str)
                    .map(str::to_owned);
                return Some((num, unit));
            }
        }
    }
    let text = raw_value?.trim();
    let num = first_number(text)?;
    let unit = text
        .split_whitespace()
        .skip(1)
        .collect::<Vec<_>>()
        .join(" ");
    let unit = (!unit.is_empty()).then_some(unit);
    Some((num, unit))
}

fn json_number(value: &serde_json::Value) -> Option<f64> {
    value
        .as_f64()
        .or_else(|| value.as_str().and_then(first_number))
}

fn first_number(text: &str) -> Option<f64> {
    let token = text
        .split(|c: char| !(c.is_ascii_digit() || c == '.' || c == '-'))
        .find(|part| part.chars().any(|c| c.is_ascii_digit()))?;
    token.parse().ok()
}

fn short_axis_date(raw: &str) -> String {
    let trimmed = raw.trim();
    if trimmed.len() >= 10 {
        trimmed[..10].to_owned()
    } else {
        trimmed.to_owned()
    }
}

#[component]
pub fn TrendPanel(trend: TrendView, on_close: EventHandler<()>) -> Element {
    let mut selected_source = use_signal(|| 0usize);
    let mut selected_point = use_signal(|| None::<usize>);
    let source_index = selected_source().min(trend.sources.len().saturating_sub(1));
    let series = &trend.sources[source_index];
    let source_options: Vec<Choice> = trend
        .sources
        .iter()
        .map(|s| Choice {
            label: s.source_label.clone(),
            ..Default::default()
        })
        .collect();
    let active_bucket = selected_point()
        .and_then(|idx| series.points.get(idx))
        .or_else(|| series.points.last());
    let figure = active_bucket
        .map(|b| b.words.as_str())
        .unwrap_or("")
        .to_owned();
    let caption = match (series.unit.as_deref(), trend.kind) {
        (_, TrendKind::Sleep) => format!("{} — nightly sleep", series.source_label),
        (_, TrendKind::BloodPressure) => {
            format!("{} — systolic and diastolic", series.source_label)
        }
        (Some(unit), _) => format!("{} ({unit})", series.source_label),
        (None, _) => series.source_label.clone(),
    };
    let points: Vec<SeriesPoint> = series
        .points
        .iter()
        .map(|b| SeriesPoint {
            value: Some(b.value),
            low: b.low,
            high: b.high,
            axis: b.axis.clone(),
            words: b.words.clone(),
            partial: false,
        })
        .collect();
    let chart_label = format!(
        "{} trend from {} ({} points)",
        trend.title,
        series.source_label,
        points.len()
    );
    rsx! {
        div { role: "region", aria_label: "{trend.title} trend",
            Card {
                ChartHeader {
                    label: trend.title.clone(),
                    value: vec![(figure, String::new())],
                    caption,
                    pinned: selected_point().is_some(),
                }
                if source_options.len() > 1 {
                    Segmented {
                        label: format!("{} source", trend.title),
                        options: source_options,
                        selected: source_index,
                        on_select: move |next| {
                            selected_source.set(next);
                            selected_point.set(None);
                        },
                    }
                }
                // Lab printed reference bands and flag tags are never rendered on charts.
                SeriesChart {
                    shape: trend.kind.chart_shape(),
                    points,
                    label: chart_label,
                    selected: selected_point(),
                    on_select: move |idx| selected_point.set(idx),
                }
                Button {
                    variant: ButtonVariant::Quiet,
                    aria_label: "Close trend",
                    onclick: move |_| on_close.call(()),
                    "Close trend"
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::roots::{LibraryKind, RecordEditability};
    use duet_wire::models::{
        CanonicalSourceAddress, EvidenceRecordProvenance, canonical_source_address::Entity,
        evidence_browse_record::Status, evidence_record_provenance::Writer,
    };

    fn lab_anchor(source: &str) -> LibraryRecord {
        LibraryRecord {
            id: "lab-1".into(),
            kind: LibraryKind::HealthEvidence,
            category: "lab".into(),
            title: "LDL cholesterol".into(),
            summary: Some("98 mg/dL".into()),
            when: Some("September 12, 2026".into()),
            source: Some(source.into()),
            status_line: None,
            editability: RecordEditability::ExternalSource,
        }
    }

    fn browse_point(
        id: &str,
        title: &str,
        value: &str,
        unit: &str,
        date: &str,
        source: &str,
    ) -> EvidenceBrowseRecord {
        let mut rec = EvidenceBrowseRecord::new(
            CanonicalSourceAddress::new(uuid::Uuid::nil(), id.into(), Entity::HealthEvidence, 1),
            Status::Current,
            title.into(),
            EvidenceRecordProvenance::new("epic".into(), Writer::Connector),
            None,
        );
        rec.value = Some(value.into());
        rec.unit = Some(unit.into());
        rec.date = Some(date.into());
        rec.provenance.source_label = Some(source.into());
        rec
    }

    #[test]
    fn multi_source_lab_trend_never_splices_across_labs() {
        let anchor = lab_anchor("Quest");
        let browse = vec![
            browse_point(
                "1",
                "LDL cholesterol",
                "110",
                "mg/dL",
                "2026-08-01T00:00:00Z",
                "Quest",
            ),
            browse_point(
                "2",
                "LDL cholesterol",
                "98",
                "mg/dL",
                "2026-09-12T00:00:00Z",
                "Quest",
            ),
            browse_point(
                "3",
                "LDL cholesterol",
                "105",
                "mg/dL",
                "2026-08-15T00:00:00Z",
                "Regional Lab",
            ),
        ];
        let view = build_trend_view(&anchor, &[], &browse).expect("trend view");
        assert_eq!(view.kind, TrendKind::Lab);
        assert_eq!(view.sources.len(), 2);
        assert_eq!(view.sources[0].source_label, "Quest");
        assert_eq!(view.sources[0].points.len(), 2);
        assert_eq!(view.sources[1].source_label, "Regional Lab");
        assert_eq!(view.sources[1].points.len(), 1);
    }

    #[test]
    fn blood_pressure_keeps_both_numbers_and_sleep_uses_bars() {
        let bp_anchor = LibraryRecord {
            id: "bp-1".into(),
            kind: LibraryKind::HealthEvidence,
            category: "vital".into(),
            title: "Blood Pressure".into(),
            summary: Some("122/78 mmHg".into()),
            when: None,
            source: Some("Apple Health".into()),
            status_line: None,
            editability: RecordEditability::ExternalSource,
        };
        let browse = vec![browse_point(
            "bp-1",
            "Blood Pressure",
            "122/78",
            "mmHg",
            "2026-10-10T08:00:00Z",
            "Apple Health",
        )];
        let view = build_trend_view(&bp_anchor, &[], &browse).expect("bp trend");
        assert_eq!(view.kind, TrendKind::BloodPressure);
        let bucket = &view.sources[0].points[0];
        assert_eq!(bucket.value, 122.0);
        assert_eq!(bucket.low, Some(78.0));
        assert_eq!(bucket.high, Some(122.0));
        assert!(bucket.words.contains("122/78"));

        let sleep_anchor = LibraryRecord {
            id: "sl-1".into(),
            kind: LibraryKind::HealthEvidence,
            category: "sleep".into(),
            title: "Sleep".into(),
            summary: Some("7.5 hr".into()),
            when: None,
            source: Some("Apple Watch".into()),
            status_line: None,
            editability: RecordEditability::ExternalSource,
        };
        let sleep_browse = vec![browse_point(
            "sl-1",
            "Sleep",
            "7.5",
            "hr",
            "2026-10-10T07:00:00Z",
            "Apple Watch",
        )];
        let sleep_view = build_trend_view(&sleep_anchor, &[], &sleep_browse).expect("sleep trend");
        assert_eq!(sleep_view.kind.chart_shape(), SeriesShape::Bars);
    }
}
