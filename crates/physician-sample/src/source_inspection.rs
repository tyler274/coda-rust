//! How a physician reads a cited record: the source's own statement and dates first, the exact
//! retained bytes and storage details under Technical details. Nothing here infers clinical truth.
use local_product::physician_chat::Citation;
use physician_wire::{SourceField, SourceInspection, SourcePreviewContent, SourceTechnicalDetails};
use serde_json::Value;
use serde_json::value::RawValue;
use std::collections::BTreeMap;

/// Presents a retained citation exactly as it was read, never a newer revision.
pub fn inspection(citation: &Citation) -> SourceInspection {
    let resource = serde_json::from_str::<Value>(&citation.content).ok();
    let mut view = SourceInspection {
        title: citation.record_type.clone(),
        record_kind: citation.record_type.clone(),
        dates: citation
            .dates
            .iter()
            .filter(|date| !date.name.starts_with("meta."))
            .map(|date| field(date_label(&date.name), &date.value))
            .collect(),
        sections: citation
            .readable_parts
            .iter()
            .map(|part| field(&part.label, &part.text))
            .collect(),
        cited_passage: None,
        coda_preview: None,
        attribution: Vec::new(),
        record_details: Vec::new(),
        technical: SourceTechnicalDetails {
            original: citation.content.clone(),
            reference: citation.reference.clone(),
            revision: citation.revision.clone(),
            sha256: citation.sha256.clone(),
            provenance: serde_json::to_string_pretty(&citation.provenance).expect("JSON value"),
            storage_dates: citation
                .dates
                .iter()
                .filter(|date| date.name.starts_with("meta."))
                .map(|date| field(&date.name, &date.value))
                .collect(),
            fields: Vec::new(),
        },
    };
    if let Some(label) = text(&citation.provenance["source_label"]) {
        view.attribution.push(field("Source system", label));
    }
    match resource {
        Some(resource) if resource["resourceType"] == "AllergyIntolerance" => allergy(&resource, &mut view),
        Some(Value::Object(resource)) => general(
            &resource,
            &NumberLiterals::read(&citation.content),
            &mut view,
        ),
        Some(_) if view.sections.is_empty() => view.sections.push(field(
            "Source content",
            "A readable presentation is not available for this format. The retained source is available under Technical details.",
        )),
        None if view.sections.is_empty() => view.sections.push(field("Source text", &citation.content)),
        _ => {}
    }
    view.coda_preview = citation.summary.as_ref().map(|summary| {
        let (label, as_of) = summary_date(&view.dates);
        SourcePreviewContent {
            text: summary.clone(),
            as_of: as_of.to_owned(),
            as_of_label: label.to_owned(),
        }
    });
    view
}

/// A date that dates the record, preferred over an onset, abatement, or period bound.
/// The label is empty when no date was retained.
fn summary_date(dates: &[SourceField]) -> (&str, &str) {
    const RECORD_DATES: [&str; 5] = ["Recorded", "Authored", "Effective", "Issued", "Dated"];
    for label in RECORD_DATES {
        if let Some(date) = dates.iter().find(|date| date.label == label) {
            return (date.label.as_str(), date.text.as_str());
        }
    }
    dates
        .first()
        .map(|date| (date.label.as_str(), date.text.as_str()))
        .unwrap_or(("", ""))
}

fn field(label: &str, value: &str) -> SourceField {
    SourceField {
        label: label.into(),
        text: value.into(),
    }
}

fn text(value: &Value) -> Option<&str> {
    value.as_str().filter(|text| !text.trim().is_empty())
}

fn concept(value: &Value) -> Option<String> {
    if let Some(text) = text(&value["text"]) {
        return Some(text.into());
    }
    let displays: Vec<_> = value["coding"]
        .as_array()
        .into_iter()
        .flatten()
        .filter_map(|coding| text(&coding["display"]))
        .collect();
    (!displays.is_empty()).then(|| displays.join("; "))
}

fn date_label(name: &str) -> &str {
    match name {
        "recordedDate" => "Recorded",
        "onsetDateTime" => "Onset recorded",
        "lastOccurrence" => "Last occurrence recorded",
        "abatementDateTime" => "Abatement recorded",
        "authoredOn" => "Authored",
        "date" => "Dated",
        "effectiveDateTime" => "Effective",
        "issued" => "Issued",
        "occurrenceDateTime" => "Occurred",
        "performedDateTime" => "Performed",
        "period.start" => "Period start",
        "period.end" => "Period end",
        other => other,
    }
}

/// The record's own kind: FHIR `type` separates allergy from intolerance, and an absent
/// `type` claims neither.
fn allergy_kind(resource: &Value) -> String {
    let kind = match resource["type"].as_str() {
        Some("allergy") => "allergy",
        Some("intolerance") => "intolerance",
        _ => "allergy or intolerance",
    };
    let medication = resource["category"]
        .as_array()
        .is_some_and(|categories| categories.iter().any(|value| value == "medication"));
    if medication {
        format!("Medication {kind} record")
    } else {
        let mut kind = kind.to_owned();
        kind[..1].make_ascii_uppercase();
        format!("{kind} record")
    }
}

fn allergy(resource: &Value, view: &mut SourceInspection) {
    view.title = concept(&resource["code"])
        .unwrap_or_else(|| "Allergy or intolerance statement not supplied".into());
    view.record_kind = allergy_kind(resource);
    if let Some(value) = text(&resource["type"]) {
        view.record_details.push(field("Type recorded", value));
    }
    view.sections
        .insert(0, field("Statement in this record", &view.title));
    for name in ["recordedDate", "onsetDateTime", "lastOccurrence"] {
        if let Some(value) = text(&resource[name])
            && !view
                .dates
                .iter()
                .any(|date| date.label == date_label(name) && date.text == value)
        {
            view.dates.push(field(date_label(name), value));
        }
    }
    for (name, label) in [
        ("patient", "Patient in source"),
        ("recorder", "Recorded by"),
        ("asserter", "Asserted by"),
    ] {
        if let Some(value) = text(&resource[name]["display"]) {
            view.attribution.push(field(label, value));
        }
    }
    for (name, label) in [
        ("clinicalStatus", "Source status recorded"),
        ("verificationStatus", "Verification status recorded"),
    ] {
        if let Some(value) = concept(&resource[name]).or_else(|| {
            resource[name]["coding"]
                .as_array()
                .and_then(|coding| coding.first())
                .and_then(|coding| text(&coding["code"]))
                .map(str::to_owned)
        }) {
            view.record_details.push(field(label, &value));
        }
    }
    if let Some(value) = text(&resource["criticality"]) {
        view.record_details
            .push(field("Criticality recorded (not reaction severity)", value));
    }
    let reactions = resource["reaction"]
        .as_array()
        .map(Vec::as_slice)
        .unwrap_or_default();
    if reactions.is_empty() {
        view.sections.push(field(
            "Reactions",
            "Reaction details not supplied in this record.",
        ));
    }
    for (index, reaction) in reactions.iter().enumerate() {
        let mut values = Vec::new();
        if let Some(substance) = concept(&reaction["substance"]) {
            values.push(format!("Substance: {substance}"));
        }
        let manifestations: Vec<_> = reaction["manifestation"]
            .as_array()
            .into_iter()
            .flatten()
            .filter_map(concept)
            .collect();
        values.push(if manifestations.is_empty() {
            "Manifestation not supplied.".into()
        } else {
            format!("Manifestation: {}", manifestations.join("; "))
        });
        if let Some(value) = text(&reaction["description"]) {
            values.push(value.into());
        }
        if let Some(value) = text(&reaction["severity"]) {
            values.push(format!("Reaction severity recorded: {value}"));
        }
        if let Some(value) = text(&reaction["onset"]) {
            values.push(format!("Reaction onset recorded: {value}"));
        }
        view.sections.push(field(
            &format!("Reaction {}", index + 1),
            &values.join("\n"),
        ));
    }
    for note in resource["note"].as_array().into_iter().flatten() {
        if let Some(value) = text(&note["text"]) {
            view.sections.push(field("Source note", value));
        }
    }
}

/// Where a top-level element of a record reads in the general view.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Placement {
    /// What the record states: its concept, value, range, interpretation and notes.
    Statement,
    /// Clinical dates, never storage timestamps.
    Date,
    /// Who the record names as performing, recording, requesting or asserting it.
    Attribution,
    /// The record's own statuses, read before its other details.
    Status,
    /// Every other recorded element.
    Detail,
    /// Identity, storage metadata and extensions, kept under Technical details.
    Technical,
}

/// Whether `key` is the element `name` or one of its FHIR choice forms (`valueQuantity`).
fn is_element(key: &str, name: &str) -> bool {
    key.strip_prefix(name)
        .is_some_and(|rest| rest.chars().next().is_none_or(char::is_uppercase))
}

/// Elements in the order they read within their placement; others follow in record order.
const READING_ORDER: [&str; 28] = [
    "code",
    "vaccineCode",
    "medication",
    "value",
    "dataAbsentReason",
    "interpretation",
    "referenceRange",
    "low",
    "high",
    "component",
    "note",
    "effective",
    "onset",
    "occurrence",
    "performed",
    "recordedDate",
    "recorded",
    "issued",
    "authoredOn",
    "period",
    "abatement",
    "lastOccurrence",
    "date",
    "status",
    "clinicalStatus",
    "verificationStatus",
    "intent",
    "performer",
];

fn reading_rank(key: &str) -> usize {
    READING_ORDER
        .iter()
        .position(|name| is_element(key, name))
        .unwrap_or(READING_ORDER.len())
}

impl Placement {
    fn of(key: &str) -> Self {
        let element = |names: &[&str]| names.iter().any(|name| is_element(key, name));
        match key {
            // `text` is the XHTML narrative; a readable rendering of it arrives as a readable part.
            "resourceType" | "id" | "meta" | "identifier" | "extension" | "modifierExtension"
            | "implicitRules" | "language" | "text" => Self::Technical,
            "code" | "vaccineCode" | "component" | "interpretation" | "referenceRange"
            | "dataAbsentReason" | "note" => Self::Statement,
            _ if element(&["medication", "value"]) => Self::Statement,
            "recordedDate" | "recorded" | "issued" | "authoredOn" | "period" | "date"
            | "lastOccurrence" => Self::Date,
            _ if element(&["effective", "onset", "abatement", "occurrence", "performed"]) => {
                Self::Date
            }
            "performer" | "recorder" | "requester" | "asserter" | "author" | "patient"
            | "subject" => Self::Attribution,
            "status" | "clinicalStatus" | "verificationStatus" | "intent" => Self::Status,
            _ => Self::Detail,
        }
    }
}

/// Each number in a record as its source wrote it, by path: `7.20` stays `7.20`, not `7.2`.
#[derive(Default)]
struct NumberLiterals(BTreeMap<String, String>);

impl NumberLiterals {
    fn read(content: &str) -> Self {
        let mut literals = Self::default();
        if let Ok(raw) = serde_json::from_str::<&RawValue>(content) {
            literals.collect(raw, "");
        }
        literals
    }

    fn collect(&mut self, raw: &RawValue, path: &str) {
        let text = raw.get().trim();
        if text.starts_with('{') {
            let Ok(members) = serde_json::from_str::<BTreeMap<String, &RawValue>>(text) else {
                return;
            };
            for (key, member) in members {
                let path = if path.is_empty() {
                    key
                } else {
                    format!("{path}.{key}")
                };
                self.collect(member, &path);
            }
        } else if text.starts_with('[') {
            let Ok(items) = serde_json::from_str::<Vec<&RawValue>>(text) else {
                return;
            };
            for (index, item) in items.into_iter().enumerate() {
                self.collect(item, &format!("{path}[{index}]"));
            }
        } else if text.starts_with(|first: char| first == '-' || first.is_ascii_digit()) {
            self.0.insert(path.to_owned(), text.to_owned());
        }
    }

    /// A scalar exactly as the record wrote it.
    fn recorded(&self, value: &Value, path: &str) -> String {
        match value {
            Value::String(text) => text.clone(),
            Value::Number(number) => self
                .0
                .get(path)
                .cloned()
                .unwrap_or_else(|| number.to_string()),
            other => other.to_string(),
        }
    }
}

/// The readable and technical fields one element contributes, in record order.
struct Flattened<'a> {
    numbers: &'a NumberLiterals,
    readable: Vec<SourceField>,
    technical: Vec<SourceField>,
}

impl Flattened<'_> {
    /// Visits every leaf under `value`: composite FHIR values read as one field, technical
    /// leaves keep their source path as their label, and nothing is skipped.
    fn walk(&mut self, value: &Value, path: &str, label: &str, technical: bool) {
        if technical {
            self.technical_leaves(value, path);
            return;
        }
        match value {
            Value::Array(items) => {
                for (index, item) in items.iter().enumerate() {
                    let label = if items.len() > 1 {
                        format!("{label} {}", index + 1)
                    } else {
                        label.to_owned()
                    };
                    self.walk(item, &format!("{path}[{index}]"), &label, false);
                }
            }
            Value::Object(object) => {
                let amount = |value: &Value| self.numbers.recorded(value, &format!("{path}.value"));
                if let Some(text) = coded(object).or_else(|| quantity(object, amount)) {
                    self.readable.push(field(label, &text));
                    return;
                }
                if is_reference(object) {
                    for (key, value) in object {
                        let path = format!("{path}.{key}");
                        match key.as_str() {
                            "display" => self.walk(value, &path, label, false),
                            _ => self.technical_leaves(value, &path),
                        }
                    }
                    return;
                }
                // A coded result such as a blood pressure component reads under its own name.
                let name = object
                    .keys()
                    .any(|key| is_element(key, "value"))
                    .then(|| object.get("code").and_then(concept))
                    .flatten();
                let mut keys: Vec<_> = object.keys().collect();
                keys.sort_by_key(|key| reading_rank(key));
                for key in keys {
                    let value = &object[key.as_str()];
                    // An attachment's encoded `data` reads through its readable part, if any.
                    let technical = matches!(
                        key.as_str(),
                        "extension" | "modifierExtension" | "identifier" | "data"
                    );
                    let label = match &name {
                        Some(name) if is_element(key, "value") => name.clone(),
                        Some(name) => format!("{name} {}", humanize(key).to_lowercase()),
                        None => format!("{label} {}", humanize(key).to_lowercase()),
                    };
                    self.walk(value, &format!("{path}.{key}"), &label, technical);
                }
            }
            scalar => self
                .readable
                .push(field(label, &self.numbers.recorded(scalar, path))),
        }
    }

    fn technical_leaves(&mut self, value: &Value, path: &str) {
        match value {
            Value::Array(items) => {
                for (index, item) in items.iter().enumerate() {
                    self.technical_leaves(item, &format!("{path}[{index}]"));
                }
            }
            Value::Object(object) => {
                for (key, value) in object {
                    self.technical_leaves(value, &format!("{path}.{key}"));
                }
            }
            scalar => self
                .technical
                .push(field(path, &self.numbers.recorded(scalar, path))),
        }
    }
}

/// The general view for any structured record without a type-specific reading: every leaf of
/// the record appears once, labeled from its path, with its value as recorded.
fn general(
    resource: &serde_json::Map<String, Value>,
    numbers: &NumberLiterals,
    view: &mut SourceInspection,
) {
    let kind = resource
        .get("resourceType")
        .and_then(text)
        .map(words)
        .unwrap_or_else(|| words(&view.record_kind));
    view.record_kind = format!("{kind} record");
    view.title = ["code", "medicationCodeableConcept", "vaccineCode"]
        .into_iter()
        .find_map(|key| resource.get(key).and_then(concept))
        .or_else(|| {
            let reference = resource.get("medicationReference")?;
            text(&reference["display"]).map(str::to_owned)
        })
        .unwrap_or_else(|| view.record_kind.clone());
    let mut statuses = Vec::new();
    let mut details = Vec::new();
    let mut keys: Vec<_> = resource.keys().collect();
    keys.sort_by_key(|key| reading_rank(key));
    for key in keys {
        let value = &resource[key.as_str()];
        let placement = Placement::of(key);
        let mut flattened = Flattened {
            numbers,
            readable: Vec::new(),
            technical: Vec::new(),
        };
        flattened.walk(
            value,
            key,
            &top_label(key, placement),
            placement == Placement::Technical,
        );
        for technical in flattened.technical {
            if !view.technical.storage_dates.contains(&technical) {
                view.technical.fields.push(technical);
            }
        }
        let readable = flattened.readable;
        match placement {
            Placement::Statement => view.sections.extend(readable),
            Placement::Date => {
                for date in readable {
                    if !view.dates.contains(&date) {
                        view.dates.push(date);
                    }
                }
            }
            Placement::Attribution => view.attribution.extend(readable),
            Placement::Status => statuses.extend(readable),
            Placement::Detail | Placement::Technical => details.extend(readable),
        }
    }
    view.record_details.extend(statuses);
    view.record_details.extend(details);
}

/// The label for a top-level element: the record's own date meaning where one is known,
/// otherwise the element name in words.
fn top_label(key: &str, placement: Placement) -> String {
    match (key, placement) {
        ("code" | "vaccineCode", _) => "Statement in this record".into(),
        (_, Placement::Date) if date_label(key) != key => date_label(key).into(),
        ("patient" | "subject", _) => "Patient in source".into(),
        ("performer", _) => "Performed by".into(),
        ("recorder", _) => "Recorded by".into(),
        ("requester", _) => "Requested by".into(),
        ("asserter", _) => "Asserted by".into(),
        _ => humanize(key),
    }
}

/// `dosageInstruction` reads as "Dosage instruction"; a FHIR choice element such as
/// `valueQuantity` or `effectiveDateTime` reads under its element name.
fn humanize(key: &str) -> String {
    const CHOICE_TYPES: [&str; 13] = [
        "CodeableConcept",
        "Reference",
        "Quantity",
        "DateTime",
        "Period",
        "String",
        "Boolean",
        "Integer",
        "Decimal",
        "Range",
        "Ratio",
        "Attachment",
        "SampledData",
    ];
    const CHOICE_ELEMENTS: [&str; 12] = [
        "value",
        "effective",
        "onset",
        "abatement",
        "occurrence",
        "performed",
        "medication",
        "reported",
        "dose",
        "rate",
        "asNeeded",
        "deceased",
    ];
    let name = CHOICE_ELEMENTS
        .into_iter()
        .find(|name| {
            key.strip_prefix(name)
                .is_some_and(|suffix| CHOICE_TYPES.contains(&suffix))
        })
        .unwrap_or(key);
    words(name)
}

/// `MedicationRequest` reads as "Medication request".
fn words(name: &str) -> String {
    let mut words = String::with_capacity(name.len() + 4);
    for (index, character) in name.chars().enumerate() {
        if index == 0 {
            words.extend(character.to_uppercase());
        } else if character.is_uppercase() {
            words.push(' ');
            words.extend(character.to_lowercase());
        } else {
            words.push(character);
        }
    }
    words
}

/// The object's members as text, when every member is an allowed key with a string value.
fn strings<'a>(
    object: &'a serde_json::Map<String, Value>,
    allowed: &[&str],
) -> Option<Vec<(&'a str, &'a str)>> {
    object
        .iter()
        .map(|(key, value)| {
            let text = value.as_str().filter(|_| allowed.contains(&key.as_str()))?;
            Some((key.as_str(), text))
        })
        .collect()
}

/// A FHIR Coding: display text, with its code and system kept visible.
fn coding(object: &serde_json::Map<String, Value>) -> Option<String> {
    let parts = strings(object, &["system", "code", "display"])?;
    let part = |name: &str| {
        parts
            .iter()
            .find(|(key, _)| *key == name)
            .map(|(_, text)| *text)
    };
    let codes: Vec<_> = ["code", "system"].into_iter().filter_map(part).collect();
    if codes.is_empty() {
        return None;
    }
    let codes = codes.join(" · ");
    Some(match part("display") {
        Some(display) => format!("{display} ({codes})"),
        None => codes,
    })
}

/// A FHIR Coding or CodeableConcept, one line per coding after any differing text.
fn coded(object: &serde_json::Map<String, Value>) -> Option<String> {
    if let Some(coding) = coding(object) {
        return Some(coding);
    }
    if object.is_empty() || object.keys().any(|key| key != "text" && key != "coding") {
        return None;
    }
    let concept_text = match object.get("text") {
        Some(value) => Some(value.as_str()?),
        None => None,
    };
    let codings = match object.get("coding") {
        Some(Value::Array(codings)) => codings
            .iter()
            .map(|coding| match coding {
                Value::Object(object) => self::coding(object)
                    .map(|line| (object.get("display").and_then(Value::as_str), line)),
                _ => None,
            })
            .collect::<Option<Vec<_>>>()?,
        Some(_) => return None,
        None => Vec::new(),
    };
    let mut lines = Vec::new();
    if let Some(concept_text) = concept_text
        && !codings
            .iter()
            .any(|(display, _)| *display == Some(concept_text))
    {
        lines.push(concept_text.to_owned());
    }
    lines.extend(codings.into_iter().map(|(_, line)| line));
    Some(lines.join("\n"))
}

/// A FHIR Quantity: comparator, value and unit, with a differing unit code and its system.
fn quantity(
    object: &serde_json::Map<String, Value>,
    recorded: impl Fn(&Value) -> String,
) -> Option<String> {
    let value = recorded(object.get("value").filter(|value| value.is_number())?);
    let mut rest = object.clone();
    rest.remove("value");
    let parts = strings(&rest, &["comparator", "unit", "system", "code"])?;
    let part = |name: &str| {
        parts
            .iter()
            .find(|(key, _)| *key == name)
            .map(|(_, text)| *text)
    };
    let mut reading = format!("{}{value}", part("comparator").unwrap_or_default());
    if let Some(unit) = part("unit") {
        reading.push(' ');
        reading.push_str(unit);
    }
    let codes: Vec<_> = [
        part("code").filter(|code| Some(*code) != part("unit")),
        part("system"),
    ]
    .into_iter()
    .flatten()
    .collect();
    if !codes.is_empty() {
        reading.push_str(&format!(" ({})", codes.join(" · ")));
    }
    Some(reading)
}

/// A FHIR Reference: its display reads in place, its target under Technical details.
fn is_reference(object: &serde_json::Map<String, Value>) -> bool {
    (object.contains_key("reference") || object.contains_key("display"))
        && strings(object, &["reference", "display", "type"]).is_some()
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn citation(content: &str) -> Citation {
        serde_json::from_value(json!({
            "id":"citation-1", "reference":"allergy-1", "revision":"original-revision",
            "record_type":"AllergyIntolerance", "content":content,
            "sha256":local_product::sha256_hex(content.as_bytes()),
            "provenance":{"native_record":"allergy-1"}
        }))
        .unwrap()
    }

    #[test]
    fn recorded_absence_is_a_source_statement_not_a_coda_preview() {
        let original = " {\"resourceType\":\"AllergyIntolerance\",\"code\":{\"text\":\"No known drug allergies\"},\"category\":[\"medication\"],\"recordedDate\":\"2020-09-14\",\"clinicalStatus\":{\"coding\":[{\"code\":\"active\"}]},\"patient\":{\"reference\":\"Patient/opaque\"}}\n";
        let view = inspection(&citation(original));
        assert_eq!(view.title, "No known drug allergies");
        assert_eq!(view.record_kind, "Medication allergy or intolerance record");
        assert_eq!(view.dates, vec![field("Recorded", "2020-09-14")]);
        assert_eq!(
            view.sections[0],
            field("Statement in this record", "No known drug allergies")
        );
        assert!(view.coda_preview.is_none());
        assert!(view.cited_passage.is_none());
        assert!(view.attribution.is_empty());
        assert_eq!(view.technical.original.as_bytes(), original.as_bytes());
        assert_eq!(
            view.record_details,
            vec![field("Source status recorded", "active")]
        );
    }

    #[test]
    fn coda_summary_is_kept_apart_from_the_record_and_dated_by_it() {
        let original = json!({"resourceType":"AllergyIntolerance",
            "code":{"text":"No known drug allergies"}})
        .to_string();
        let mut cited = citation(&original);
        cited.summary = Some("Records no known drug allergies.".into());
        cited.dates = vec![local_product::physician_sources::RecordDate {
            name: "recordedDate".into(),
            value: "2018-09-11".into(),
        }];
        let view = inspection(&cited);
        assert_eq!(
            view.coda_preview,
            Some(SourcePreviewContent {
                text: "Records no known drug allergies.".into(),
                as_of: "2018-09-11".into(),
                as_of_label: "Recorded".into(),
            })
        );
        let mut onset_first = cited.clone();
        onset_first.dates.insert(
            0,
            local_product::physician_sources::RecordDate {
                name: "onsetDateTime".into(),
                value: "2010-01-02T08:00:00+00:00".into(),
            },
        );
        onset_first
            .dates
            .iter_mut()
            .find(|date| date.name == "recordedDate")
            .unwrap()
            .value = "2018-09-11T12:20:00+00:00".into();
        let preferred = inspection(&onset_first).coda_preview.unwrap();
        assert_eq!(preferred.as_of_label, "Recorded");
        assert_eq!(preferred.as_of, "2018-09-11T12:20:00+00:00");
        onset_first.dates.retain(|date| date.name != "recordedDate");
        let onset_only = inspection(&onset_first);
        assert_eq!(
            onset_only.coda_preview,
            Some(SourcePreviewContent {
                text: "Records no known drug allergies.".into(),
                as_of: "2010-01-02T08:00:00+00:00".into(),
                as_of_label: "Onset recorded".into(),
            })
        );
        assert_eq!(
            view.sections[0],
            field("Statement in this record", "No known drug allergies")
        );
    }

    #[test]
    fn reaction_severity_and_criticality_keep_their_different_meanings() {
        let original = json!({"resourceType":"AllergyIntolerance", "code":{"coding":[{"display":"Penicillin"}]},
            "criticality":"high", "verificationStatus":{"text":"Confirmed"},
            "reaction":[{"manifestation":[{"text":"Hives"}],"severity":"mild"}],
            "recorder":{"display":"Dr. Recorded"},"unknownField":{"future":"retained"}}).to_string();
        let view = inspection(&citation(&original));
        assert_eq!(view.title, "Penicillin");
        assert!(view.dates.is_empty());
        assert!(view.sections.iter().any(
            |section| section.text == "Manifestation: Hives\nReaction severity recorded: mild"
        ));
        assert!(view.record_details.contains(&field(
            "Criticality recorded (not reaction severity)",
            "high"
        )));
        assert!(
            view.record_details
                .contains(&field("Verification status recorded", "Confirmed"))
        );
        assert_eq!(view.technical.original, original);
    }

    #[test]
    fn missing_fields_are_not_negative_findings() {
        let view = inspection(&citation(
            r#"{"resourceType":"AllergyIntolerance","criticality":"low"}"#,
        ));
        assert_eq!(view.title, "Allergy or intolerance statement not supplied");
        assert!(view.dates.is_empty());
        assert!(view.sections.contains(&field(
            "Reactions",
            "Reaction details not supplied in this record."
        )));
        assert!(
            !view
                .sections
                .iter()
                .any(|section| section.text.contains("No known"))
        );
    }

    #[test]
    fn narrative_unknown_and_historical_bytes_remain_inspectable() {
        let original = "First line\nSecond line\n";
        let mut retained = citation(original);
        let before = inspection(&retained);
        assert_eq!(before.sections, vec![field("Source text", original)]);
        retained.content = "Newer record".into();
        assert_eq!(before.technical.original, original);
        let mut unknown = citation(r#"{ "resourceType":"Unknown", "value":42 }"#);
        let general = inspection(&unknown);
        assert_eq!(general.sections, vec![field("Value", "42")]);
        assert_eq!(general.record_kind, "Unknown record");
        assert_eq!(general.technical.original, unknown.content);
        unknown
            .readable_parts
            .push(local_product::physician_sources::ReadablePart {
                label: "Document".into(),
                text: "Readable attachment".into(),
            });
        assert_eq!(
            inspection(&unknown).sections,
            vec![
                field("Document", "Readable attachment"),
                field("Value", "42")
            ]
        );
        let array = inspection(&citation(r#"[{"value":42}]"#));
        assert!(
            array.sections[0]
                .text
                .contains("retained source is available")
        );
    }

    fn general_citation(record: &Value) -> Citation {
        let mut cited = citation(&record.to_string());
        cited.record_type = record["resourceType"].as_str().unwrap_or("Record").into();
        cited
    }

    /// Every scalar leaf of `record`, with its source path and recorded text.
    fn leaves(
        value: &Value,
        numbers: &NumberLiterals,
        path: String,
        out: &mut Vec<(String, String)>,
    ) {
        match value {
            Value::Array(items) => {
                for (index, item) in items.iter().enumerate() {
                    leaves(item, numbers, format!("{path}[{index}]"), out);
                }
            }
            Value::Object(object) => {
                for (key, value) in object {
                    let path = if path.is_empty() {
                        key.clone()
                    } else {
                        format!("{path}.{key}")
                    };
                    leaves(value, numbers, path, out);
                }
            }
            scalar => out.push((path.clone(), numbers.recorded(scalar, &path))),
        }
    }

    /// Fails naming the first leaf of `record` that its general view does not show.
    fn assert_nothing_dropped(record: &Value) {
        assert_content_shown(&general_citation(record));
    }

    fn assert_content_shown(cited: &Citation) {
        let record: Value = serde_json::from_str(&cited.content).unwrap();
        let numbers = NumberLiterals::read(&cited.content);
        let view = inspection(cited);
        let shown: Vec<&SourceField> = view
            .sections
            .iter()
            .chain(&view.dates)
            .chain(&view.attribution)
            .chain(&view.record_details)
            .chain(&view.technical.fields)
            .chain(&view.technical.storage_dates)
            .collect();
        let mut all = Vec::new();
        leaves(&record, &numbers, String::new(), &mut all);
        for (path, value) in all {
            let technical = shown
                .iter()
                .any(|field| field.label == path && field.text == value);
            let readable = shown.iter().any(|field| field.text.contains(&value));
            assert!(technical || readable, "{path} = {value} is not shown");
        }
    }

    fn observation() -> Value {
        json!({"resourceType":"Observation","id":"obs-1",
            "meta":{"versionId":"1","lastUpdated":"2019-03-12T00:00:00+00:00"},
            "status":"final",
            "category":[{"coding":[{"system":"http://terminology.hl7.org/CodeSystem/observation-category","code":"laboratory","display":"Laboratory"}]}],
            "code":{"coding":[{"system":"http://loinc.org","code":"4548-4","display":"Hemoglobin A1c/Hemoglobin.total in Blood"}],"text":"Hemoglobin A1c"},
            "subject":{"reference":"Patient/MRN00001101"},
            "effectiveDateTime":"2019-03-12T13:30:00+00:00",
            "issued":"2019-03-13T09:00:00+00:00",
            "valueQuantity":{"value":7.20,"unit":"%","system":"http://unitsofmeasure.org","code":"%"},
            "interpretation":[{"coding":[{"system":"http://terminology.hl7.org/CodeSystem/v3-ObservationInterpretation","code":"H","display":"High"}]}],
            "referenceRange":[{"low":{"value":4.0,"unit":"%"},"high":{"value":5.6,"unit":"%"},"text":"4.0-5.6 %"}],
            "performer":[{"reference":"Practitioner/p-1","display":"Dr. Lab"}],
            "extension":[{"url":"http://example.org/ext","valueString":"kept"}]})
    }

    #[test]
    fn an_observation_reads_value_range_and_interpretation_as_recorded() {
        let mut cited = general_citation(&observation());
        cited.dates = vec![local_product::physician_sources::RecordDate {
            name: "effectiveDateTime".into(),
            value: "2019-03-12T13:30:00+00:00".into(),
        }];
        let view = inspection(&cited);
        assert_eq!(view.title, "Hemoglobin A1c");
        assert_eq!(view.record_kind, "Observation record");
        assert_eq!(
            view.sections,
            vec![
                field(
                    "Statement in this record",
                    "Hemoglobin A1c\nHemoglobin A1c/Hemoglobin.total in Blood (4548-4 · http://loinc.org)"
                ),
                field("Value", "7.2 % (http://unitsofmeasure.org)"),
                field(
                    "Interpretation",
                    "High (H · http://terminology.hl7.org/CodeSystem/v3-ObservationInterpretation)"
                ),
                field("Reference range low", "4.0 %"),
                field("Reference range high", "5.6 %"),
                field("Reference range text", "4.0-5.6 %"),
            ]
        );
        assert_eq!(
            view.dates,
            vec![
                field("Effective", "2019-03-12T13:30:00+00:00"),
                field("Issued", "2019-03-13T09:00:00+00:00"),
            ]
        );
        assert_eq!(view.attribution, vec![field("Performed by", "Dr. Lab")]);
        assert_eq!(view.record_details[0], field("Status", "final"));
        assert!(view.technical.fields.contains(&field("id", "obs-1")));
        assert!(
            view.technical
                .fields
                .contains(&field("performer[0].reference", "Practitioner/p-1"))
        );
        assert!(
            view.technical
                .fields
                .contains(&field("extension[0].valueString", "kept"))
        );
        assert!(
            view.technical
                .fields
                .contains(&field("meta.lastUpdated", "2019-03-12T00:00:00+00:00"))
        );
        assert!(view.sections.iter().all(|field| !field.text.contains('{')));
        assert_nothing_dropped(&observation());
    }

    fn blood_pressure() -> Value {
        json!({"resourceType":"Observation","id":"obs-314a3cecf88a","status":"final",
            "code":{"coding":[{"system":"http://loinc.org","code":"85354-9","display":"Blood pressure panel with all children optional"}],"text":"Blood Pressure"},
            "subject":{"reference":"Patient/MRN00001101"},
            "encounter":{"reference":"Encounter/enc-26183a468076"},
            "effectiveDateTime":"2019-03-12T13:30:00+00:00",
            "component":[
                {"code":{"coding":[{"system":"http://loinc.org","code":"8480-6","display":"Systolic blood pressure"}],"text":"Systolic Blood Pressure"},
                 "valueQuantity":{"value":159,"unit":"mmHg","system":"http://unitsofmeasure.org","code":"mm[Hg]"}},
                {"code":{"coding":[{"system":"http://loinc.org","code":"8462-4","display":"Diastolic blood pressure"}],"text":"Diastolic Blood Pressure"},
                 "valueQuantity":{"value":99,"unit":"mmHg","system":"http://unitsofmeasure.org","code":"mm[Hg]"}}]})
    }

    #[test]
    fn each_blood_pressure_component_reads_under_its_own_name() {
        let view = inspection(&general_citation(&blood_pressure()));
        assert_eq!(view.title, "Blood Pressure");
        assert!(view.sections.contains(&field(
            "Systolic Blood Pressure",
            "159 mmHg (mm[Hg] · http://unitsofmeasure.org)"
        )));
        assert!(view.sections.contains(&field(
            "Diastolic Blood Pressure",
            "99 mmHg (mm[Hg] · http://unitsofmeasure.org)"
        )));
        assert!(view.sections.contains(&field(
            "Systolic Blood Pressure code",
            "Systolic Blood Pressure\nSystolic blood pressure (8480-6 · http://loinc.org)"
        )));
        assert_eq!(
            view.dates,
            vec![field("Effective", "2019-03-12T13:30:00+00:00")]
        );
        assert!(
            view.technical
                .fields
                .contains(&field("encounter.reference", "Encounter/enc-26183a468076"))
        );
        assert_nothing_dropped(&blood_pressure());
    }

    fn condition() -> Value {
        json!({"resourceType":"Condition","id":"cond-db76cb24b2c2",
            "clinicalStatus":{"coding":[{"system":"http://terminology.hl7.org/CodeSystem/condition-clinical","code":"resolved","display":"Resolved"}]},
            "verificationStatus":{"coding":[{"system":"http://terminology.hl7.org/CodeSystem/condition-ver-status","code":"confirmed","display":"Confirmed"}]},
            "category":[{"coding":[{"system":"http://terminology.hl7.org/CodeSystem/condition-category","code":"problem-list-item","display":"Problem List Item"}],"text":"Problem List Item"}],
            "code":{"coding":[{"system":"http://hl7.org/fhir/sid/icd-10-cm","code":"E11.9","display":"Type 2 diabetes mellitus without complications"}],"text":"Type 2 diabetes mellitus without complications"},
            "subject":{"reference":"Patient/MRN00001101"},
            "onsetDateTime":"2005-06-01","recordedDate":"2019-03-12T13:20:00+00:00",
            "abatementDateTime":"2019-06-19T19:00:00+00:00",
            "note":[{"text":"about 14 years per patient and prior records"}]})
    }

    #[test]
    fn a_condition_leads_with_its_statement_dates_and_statuses() {
        let view = inspection(&general_citation(&condition()));
        assert_eq!(view.title, "Type 2 diabetes mellitus without complications");
        assert_eq!(
            view.sections,
            vec![
                field(
                    "Statement in this record",
                    "Type 2 diabetes mellitus without complications (E11.9 · http://hl7.org/fhir/sid/icd-10-cm)"
                ),
                field("Note", "about 14 years per patient and prior records"),
            ]
        );
        assert_eq!(
            view.dates,
            vec![
                field("Onset recorded", "2005-06-01"),
                field("Recorded", "2019-03-12T13:20:00+00:00"),
                field("Abatement recorded", "2019-06-19T19:00:00+00:00"),
            ]
        );
        assert_eq!(
            view.record_details[..2],
            [
                field(
                    "Clinical status",
                    "Resolved (resolved · http://terminology.hl7.org/CodeSystem/condition-clinical)"
                ),
                field(
                    "Verification status",
                    "Confirmed (confirmed · http://terminology.hl7.org/CodeSystem/condition-ver-status)"
                ),
            ]
        );
        assert_nothing_dropped(&condition());
    }

    fn medication_request() -> Value {
        json!({"resourceType":"MedicationRequest","id":"medrx-24ab1968c855","status":"active","intent":"order",
            "medicationCodeableConcept":{"coding":[{"system":"http://www.nlm.nih.gov/research/umls/rxnorm","code":"861004","display":"metformin hydrochloride 1000 MG Oral Tablet"}],"text":"metformin hydrochloride 1000 mg oral tablet"},
            "subject":{"reference":"Patient/MRN00001101"},
            "requester":{"reference":"Practitioner/p-2","display":"Dr. Prescriber"},
            "authoredOn":"2019-03-12T13:20:00+00:00","reportedBoolean":true,
            "dosageInstruction":[{"text":"Take 1 tablet by mouth twice daily with meals","timing":{"code":{"text":"BID"}},
                "doseAndRate":[{"doseQuantity":{"value":1000,"unit":"mg"}}],"route":{"text":"Oral"}}],
            "reasonCode":[{"text":"Type 2 diabetes"}]})
    }

    #[test]
    fn a_medication_request_reads_its_medication_dose_and_request() {
        let view = inspection(&general_citation(&medication_request()));
        assert_eq!(view.title, "metformin hydrochloride 1000 mg oral tablet");
        assert_eq!(view.record_kind, "Medication request record");
        assert_eq!(
            view.sections[0],
            field(
                "Medication",
                "metformin hydrochloride 1000 mg oral tablet\nmetformin hydrochloride 1000 MG Oral Tablet (861004 · http://www.nlm.nih.gov/research/umls/rxnorm)"
            )
        );
        assert_eq!(
            view.dates,
            vec![field("Authored", "2019-03-12T13:20:00+00:00")]
        );
        assert_eq!(
            view.attribution,
            vec![field("Requested by", "Dr. Prescriber")]
        );
        assert_eq!(
            view.record_details[..2],
            [field("Status", "active"), field("Intent", "order")]
        );
        for detail in [
            field(
                "Dosage instruction text",
                "Take 1 tablet by mouth twice daily with meals",
            ),
            field("Dosage instruction timing code", "BID"),
            field("Dosage instruction dose and rate dose", "1000 mg"),
            field("Dosage instruction route", "Oral"),
            field("Reported", "true"),
            field("Reason code", "Type 2 diabetes"),
        ] {
            assert!(view.record_details.contains(&detail), "{detail:?}");
        }
        assert_nothing_dropped(&medication_request());
    }

    fn immunization() -> Value {
        json!({"resourceType":"Immunization","id":"imm-26a721e56006",
            "meta":{"versionId":"1","lastUpdated":"2019-03-12T00:00:00+00:00"},"status":"completed",
            "vaccineCode":{"coding":[{"system":"http://hl7.org/fhir/sid/cvx","code":"115","display":"Tdap"}],"text":"Tdap"},
            "patient":{"reference":"Patient/MRN00001101"},
            "encounter":{"reference":"Encounter/enc-26183a468076"},
            "occurrenceDateTime":"2019-03-12T14:15:00+00:00","primarySource":true,
            "identifier":[{"system":"urn:example","value":"imm-1"}]})
    }

    #[test]
    fn an_immunization_reads_its_vaccine_and_occurrence() {
        let view = inspection(&general_citation(&immunization()));
        assert_eq!(view.title, "Tdap");
        assert_eq!(
            view.sections,
            vec![field(
                "Statement in this record",
                "Tdap (115 · http://hl7.org/fhir/sid/cvx)"
            )]
        );
        assert_eq!(
            view.dates,
            vec![field("Occurred", "2019-03-12T14:15:00+00:00")]
        );
        assert!(
            view.record_details
                .contains(&field("Primary source", "true"))
        );
        assert!(
            view.technical
                .fields
                .contains(&field("identifier[0].value", "imm-1"))
        );
        assert_nothing_dropped(&immunization());
    }

    #[test]
    fn numbers_read_exactly_as_the_source_wrote_them() {
        let original = r#"{"resourceType":"Observation","code":{"text":"Hemoglobin A1c"},
            "valueQuantity":{"value":7.20,"unit":"%"},
            "referenceRange":[{"low":{"value":4.0,"unit":"%"},"high":{"value":5.60,"unit":"%"}}],
            "component":[{"code":{"text":"Count"},"valueInteger":1e3}],"extra":-0.50}"#;
        let cited = citation(original);
        let view = inspection(&cited);
        assert!(view.sections.contains(&field("Value", "7.20 %")));
        assert!(
            view.sections
                .contains(&field("Reference range low", "4.0 %"))
        );
        assert!(
            view.sections
                .contains(&field("Reference range high", "5.60 %"))
        );
        assert!(view.sections.contains(&field("Count", "1e3")));
        assert!(view.record_details.contains(&field("Extra", "-0.50")));
        assert_content_shown(&cited);
    }

    #[test]
    fn a_shape_never_seen_still_reads_every_value_under_its_label() {
        let record = json!({"kind":"lab-result","analyte":{"name":"Potassium","flags":["H",null]},
            "result":{"value":5.9,"units":"mmol/L"},"collected":"2024-02-01","n":[1,2.50]});
        let view = inspection(&general_citation(&record));
        assert_eq!(view.record_kind, "Record record");
        assert!(
            view.record_details
                .contains(&field("Analyte name", "Potassium"))
        );
        assert!(
            view.record_details
                .contains(&field("Analyte flags 2", "null"))
        );
        assert!(
            view.record_details
                .contains(&field("Result units", "mmol/L"))
        );
        assert_nothing_dropped(&record);
    }

    #[test]
    fn an_intolerance_is_never_headed_as_an_allergy() {
        let view = inspection(&citation(
            r#"{"resourceType":"AllergyIntolerance","type":"intolerance","category":["medication"],"code":{"text":"Metformin"}}"#,
        ));
        assert_eq!(view.record_kind, "Medication intolerance record");
        assert!(
            view.record_details
                .contains(&field("Type recorded", "intolerance"))
        );
        let food = inspection(&citation(
            r#"{"resourceType":"AllergyIntolerance","type":"allergy","category":["food"],"code":{"text":"Peanut"}}"#,
        ));
        assert_eq!(food.record_kind, "Allergy record");
    }

    /// The patient bank's own record, read the way the synthetic chart adapter retains it.
    #[test]
    fn a_bank_allergy_leads_with_its_clinical_date_and_keeps_storage_time_technical() {
        let line = r#"{"resourceType":"AllergyIntolerance","id":"allergy-0493010f2a16","meta":{"versionId":"1","lastUpdated":"2018-09-11T00:00:00+00:00"},"clinicalStatus":{"coding":[{"system":"http://terminology.hl7.org/CodeSystem/allergyintolerance-clinical","code":"active","display":"Active"}]},"verificationStatus":{"coding":[{"system":"http://terminology.hl7.org/CodeSystem/allergyintolerance-verification","code":"confirmed","display":"Confirmed"}]},"type":"allergy","category":["medication"],"criticality":"low","code":{"coding":[{"system":"http://snomed.info/sct","code":"409137002","display":"No known drug allergy"}],"text":"No known drug allergies"},"patient":{"reference":"Patient/MRN00001776"},"recordedDate":"2018-09-11T12:20:00+00:00"}"#;
        let mut retained = citation(line);
        retained.dates = vec![
            local_product::physician_sources::RecordDate {
                name: "recordedDate".into(),
                value: "2018-09-11T12:20:00+00:00".into(),
            },
            local_product::physician_sources::RecordDate {
                name: "meta.lastUpdated".into(),
                value: "2018-09-11T00:00:00+00:00".into(),
            },
        ];
        let view = inspection(&retained);
        assert_eq!(view.title, "No known drug allergies");
        assert_eq!(view.record_kind, "Medication allergy record");
        assert_eq!(
            view.dates,
            vec![field("Recorded", "2018-09-11T12:20:00+00:00")]
        );
        assert_eq!(
            view.technical.storage_dates,
            vec![field("meta.lastUpdated", "2018-09-11T00:00:00+00:00")]
        );
        assert!(
            view.record_details
                .contains(&field("Source status recorded", "Active"))
        );
        assert!(view.record_details.contains(&field(
            "Criticality recorded (not reaction severity)",
            "low"
        )));
        assert_eq!(view.technical.original, line);
    }
}
