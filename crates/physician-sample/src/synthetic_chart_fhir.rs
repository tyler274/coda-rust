//! Files raw synthetic patient-bank NDJSON lines into [`SourceRevision`] records.

use base64::Engine as _;
use local_product::physician_sources::{ReadablePart, RecordDate, SourceRevision};
use local_product::sha256_hex;
use serde_json::{Value, json};

/// The fields that date a record, in the source's names.
const DATE_FIELDS: [&str; 9] = [
    "abatementDateTime",
    "authoredOn",
    "date",
    "effectiveDateTime",
    "issued",
    "occurrenceDateTime",
    "onsetDateTime",
    "performedDateTime",
    "recordedDate",
];

/// Admits one raw NDJSON record line from `source_id` into `workspace`, preserving
/// `line` byte-for-byte in [`SourceRevision::content`].
pub fn source_revision(
    workspace: &str,
    source_id: &str,
    line: &str,
) -> Result<SourceRevision, String> {
    let resource: Value = serde_json::from_str(line).map_err(|error| error.to_string())?;
    let record_type = resource["resourceType"]
        .as_str()
        .ok_or("record has no resourceType")?;
    let id = resource["id"].as_str().ok_or("record has no id")?;
    let candidate_id = format!("{record_type}/{id}");
    let sha256 = sha256_hex(line.as_bytes());
    let source = SourceRevision {
        workspace: workspace.to_owned(),
        source_id: source_id.to_owned(),
        candidate_id: candidate_id.clone(),
        revision: sha256.clone(),
        record_type: record_type.to_owned(),
        dates: record_dates(&resource),
        content: line.to_owned(),
        sha256,
        readable_parts: readable_parts(&resource)?,
        provenance: json!({"native_record": candidate_id}),
        envelope: None,
    };
    source.validate()?;
    Ok(source)
}

fn record_dates(resource: &Value) -> Vec<RecordDate> {
    let date = |name: &str, value: &Value| {
        value.as_str().map(|value| RecordDate {
            name: name.to_owned(),
            value: value.to_owned(),
        })
    };
    DATE_FIELDS
        .iter()
        .filter_map(|name| date(name, &resource[name]))
        .chain(date("period.start", &resource["period"]["start"]))
        .chain(date("period.end", &resource["period"]["end"]))
        .chain(date("meta.lastUpdated", &resource["meta"]["lastUpdated"]))
        .collect()
}

/// The text of the record's attachments, which the exact record holds encoded.
fn readable_parts(resource: &Value) -> Result<Vec<ReadablePart>, String> {
    let documents = resource["content"].as_array().into_iter().flatten();
    let forms = resource["presentedForm"].as_array().into_iter().flatten();
    documents
        .map(|document| ("attachment", &document["attachment"]))
        .chain(forms.map(|form| ("presentedForm", form)))
        .map(|(label, attachment)| {
            let content_type = attachment["contentType"].as_str().unwrap_or_default();
            if !content_type.starts_with("text/") {
                return Err(format!("{label} is not text: {content_type}"));
            }
            let data = attachment["data"].as_str().unwrap_or_default();
            let bytes = base64::engine::general_purpose::STANDARD
                .decode(data)
                .map_err(|error| format!("{label}: {error}"))?;
            let text = String::from_utf8(bytes).map_err(|error| format!("{label}: {error}"))?;
            Ok(ReadablePart {
                label: label.to_owned(),
                text,
            })
        })
        .collect()
}
