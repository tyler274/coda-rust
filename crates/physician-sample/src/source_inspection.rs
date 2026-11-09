//! How a physician reads a cited record: the source's own statement and dates first, the exact
//! retained bytes and storage details under Technical details. Nothing here infers clinical truth.
use crate::source_layout::{self, Layout, Loggable, SlotRole};
use crate::source_path::SourcePath;
use local_product::physician_chat::Citation;
use physician_wire::{SourceField, SourceInspection, SourcePreviewContent, SourceTechnicalDetails};
use serde_json::Value;
use serde_json::value::RawValue;
use std::collections::BTreeMap;
use std::fmt;

/// Presents a retained citation exactly as it was read, never a newer revision.
pub fn inspection(citation: &Citation) -> SourceInspection {
    inspection_with_layout(citation, None)
}

/// Presents a retained citation through its record type's layout when that layout validates
/// against the record. A refused layout never shows, even in part: the record reads exactly as it
/// would with no layout.
pub fn inspection_with_layout(citation: &Citation, layout: Option<&Layout>) -> SourceInspection {
    let resource = serde_json::from_str::<Value>(&citation.content).ok();
    let mut view = SourceInspection {
        title: citation.record_type.clone(),
        record_kind: citation.record_type.clone(),
        dates: citation
            .dates
            .iter()
            .filter(|date| !date.name.starts_with("meta."))
            .map(|date| located(date_label(&date.name), &date.value, &date.name))
            .collect(),
        // Readable text decoded from an attachment has no field mapping and claims no path.
        sections: citation
            .readable_parts
            .iter()
            .map(|part| field(&part.label, &part.text))
            .collect(),
        cited_passage: None,
        coda_preview: None,
        attribution: Vec::new(),
        statuses: Vec::new(),
        record_details: Vec::new(),
        layout_sections: Vec::new(),
        other_fields: Vec::new(),
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
                .map(|date| located(&date.name, &date.value, &date.name))
                .collect(),
            fields: Vec::new(),
        },
    };
    if let Some(label) = text(&citation.provenance["source_label"]) {
        view.attribution.push(field("Source system", label));
    }
    let numbers = NumberLiterals::read(&citation.content);
    let layout = layout.zip(resource.as_ref()).and_then(|(layout, record)| {
        match layout.validate(&citation.record_type, record) {
            Ok(validated) => Some(validated),
            Err(refusal) => {
                tracing::warn!(
                    source_id = citation.source_id.as_deref(),
                    record_type = citation.record_type,
                    refusal = refusal.cause.category(),
                    role = refusal.role.map(SlotRole::name),
                    part = refusal.at.part.name(),
                    section = refusal.at.section(),
                    slot = refusal.at.slot,
                    element = refusal.at.element,
                    error = %Loggable(&refusal),
                    "stored source layout refused for this record; reading through the general view"
                );
                None
            }
        }
    });
    match (&resource, layout) {
        // A layout rearranges the general view, which shows every leaf of the record.
        (Some(Value::Object(record)), Some(layout)) => {
            general(record, &numbers, &mut view);
            source_layout::render(&layout, &numbers, &mut view);
        }
        (Some(resource), _) if resource["resourceType"] == "AllergyIntolerance" => {
            allergy(resource, &mut view)
        }
        (Some(Value::Object(resource)), _) => general(resource, &numbers, &mut view),
        (Some(_), _) if view.sections.is_empty() => view.sections.push(field(
            "Source content",
            "A readable presentation is not available for this format. The retained source is available under Technical details.",
        )),
        (None, _) if view.sections.is_empty() => view.sections.push(field("Source text", &citation.content)),
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

/// Text Coda supplies about the record, such as a part it does not contain; no source path.
fn field(label: &str, value: &str) -> SourceField {
    SourceField {
        label: label.into(),
        text: value.into(),
        path: None,
    }
}

/// A value copied from the record at `path`, carried in its readable form (see [`SourcePath`]).
pub(crate) fn located(label: &str, value: &str, path: impl fmt::Display) -> SourceField {
    SourceField {
        label: label.into(),
        text: value.into(),
        path: Some(path.to_string()),
    }
}

/// How a part the record marks absent reads: never as a normal or negative finding.
fn not_supplied(reason: &str) -> String {
    format!("Not supplied. Reason recorded: {reason}")
}

/// FHIR's extension marking a primitive or complex value absent, with its reason code.
const DATA_ABSENT_REASON: &str = "http://hl7.org/fhir/StructureDefinition/data-absent-reason";

/// A status as the record names it: its text or coding display, else its first code. The
/// coding's code and system stay under Technical details.
fn status_text(value: &Value) -> Option<String> {
    concept(value).or_else(|| {
        value["coding"]
            .as_array()
            .and_then(|coding| coding.first())
            .and_then(|coding| text(&coding["code"]))
            .map(str::to_owned)
    })
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
    let statement = concept(&resource["code"]);
    view.title = statement
        .clone()
        .unwrap_or_else(|| "Allergy or intolerance statement not supplied".into());
    view.record_kind = allergy_kind(resource);
    if let Some(value) = text(&resource["type"]) {
        view.record_details
            .push(located("Type recorded", value, "type"));
    }
    view.sections.insert(
        0,
        match &statement {
            Some(statement) => located("Statement in this record", statement, "code"),
            None => field("Statement in this record", &view.title),
        },
    );
    for name in ["recordedDate", "onsetDateTime", "lastOccurrence"] {
        if let Some(value) = text(&resource[name])
            && !view
                .dates
                .iter()
                .any(|date| date.label == date_label(name) && date.text == value)
        {
            view.dates.push(located(date_label(name), value, name));
        }
    }
    for (name, label) in [
        ("patient", "Patient in source"),
        ("recorder", "Recorded by"),
        ("asserter", "Asserted by"),
    ] {
        if let Some(value) = text(&resource[name]["display"]) {
            view.attribution.push(located(
                label,
                value,
                SourcePath::root().member(name).member("display"),
            ));
        }
    }
    for (name, label) in [
        ("clinicalStatus", "Source status recorded"),
        ("verificationStatus", "Verification status recorded"),
    ] {
        if let Some(value) = status_text(&resource[name]) {
            view.statuses.push(located(label, &value, name));
        }
    }
    if let Some(value) = text(&resource["criticality"]) {
        view.record_details.push(located(
            "Criticality recorded (not reaction severity)",
            value,
            "criticality",
        ));
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
        view.sections.push(located(
            &format!("Reaction {}", index + 1),
            &values.join("\n"),
            SourcePath::root().member("reaction").index(index),
        ));
    }
    for (index, note) in resource["note"]
        .as_array()
        .into_iter()
        .flatten()
        .enumerate()
    {
        if let Some(value) = text(&note["text"]) {
            view.sections.push(located(
                "Source note",
                value,
                SourcePath::root()
                    .member("note")
                    .index(index)
                    .member("text"),
            ));
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
    /// The record's own clinical, verification or result status, read beside its kind and date.
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
            "status" | "clinicalStatus" | "verificationStatus" => Self::Status,
            _ => Self::Detail,
        }
    }
}

/// Each number in a record as its source wrote it, by path: `7.20` stays `7.20`, not `7.2`.
#[derive(Default)]
pub(crate) struct NumberLiterals(BTreeMap<SourcePath, String>);

impl NumberLiterals {
    pub(crate) fn read(content: &str) -> Self {
        let mut literals = Self::default();
        if let Ok(raw) = serde_json::from_str::<&RawValue>(content) {
            literals.collect(raw, &SourcePath::root());
        }
        literals
    }

    fn collect(&mut self, raw: &RawValue, path: &SourcePath) {
        let text = raw.get().trim();
        if text.starts_with('{') {
            let Ok(members) = serde_json::from_str::<BTreeMap<String, &RawValue>>(text) else {
                return;
            };
            for (key, member) in members {
                self.collect(member, &path.member(&key));
            }
        } else if text.starts_with('[') {
            let Ok(items) = serde_json::from_str::<Vec<&RawValue>>(text) else {
                return;
            };
            for (index, item) in items.into_iter().enumerate() {
                self.collect(item, &path.index(index));
            }
        } else if text.starts_with(|first: char| first == '-' || first.is_ascii_digit()) {
            self.0.insert(path.clone(), text.to_owned());
        }
    }

    /// A scalar exactly as the record wrote it.
    pub(crate) fn recorded(&self, value: &Value, path: &SourcePath) -> String {
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
    fn walk(&mut self, value: &Value, path: &SourcePath, label: &str, technical: bool) {
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
                    self.walk(item, &path.index(index), &label, false);
                }
            }
            Value::Object(object) => {
                if let Some(reason) = absent_extension(object) {
                    self.readable
                        .push(located(label, &not_supplied(reason), path));
                    self.technical_leaves(value, path);
                    return;
                }
                let amount = |value: &Value| self.numbers.recorded(value, &path.member("value"));
                if let Some(text) = coded(object).or_else(|| quantity(object, amount)) {
                    // A concept that is only text points at that text, not its container.
                    let path = match object.len() {
                        1 if object.contains_key("text") => path.member("text"),
                        _ => path.clone(),
                    };
                    self.readable.push(located(label, &text, &path));
                    return;
                }
                if is_reference(object) {
                    for (key, value) in object {
                        let path = path.member(key);
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
                    .any(|key| is_element(key, "value") || key == "dataAbsentReason")
                    .then(|| object.get("code").and_then(concept))
                    .flatten();
                let mut keys: Vec<_> = object.keys().collect();
                keys.sort_by_key(|key| reading_rank(key));
                for key in keys {
                    let value = &object[key.as_str()];
                    let path = path.member(key);
                    // An attachment's encoded `data` reads through its readable part, if any.
                    let technical = matches!(
                        key.as_str(),
                        "extension" | "modifierExtension" | "identifier" | "data"
                    );
                    // A result the record marks absent reads under the result's own name.
                    if key == "dataAbsentReason" {
                        self.absent(value, &path, name.as_deref().unwrap_or(label));
                        continue;
                    }
                    let label = match &name {
                        Some(name) if is_element(key, "value") => name.clone(),
                        Some(name) => format!("{name} {}", humanize(key).to_lowercase()),
                        None => format!("{label} {}", humanize(key).to_lowercase()),
                    };
                    self.walk(value, &path, &label, technical);
                }
            }
            scalar => {
                self.readable
                    .push(located(label, &self.numbers.recorded(scalar, path), path))
            }
        }
    }

    /// A FHIR `dataAbsentReason`: the value it stands for reads as not supplied, with the
    /// recorded reason, never as a normal or negative finding.
    fn absent(&mut self, reason: &Value, path: &SourcePath, label: &str) {
        let Some(recorded) = status_text(reason).or_else(|| text(reason).map(str::to_owned)) else {
            self.walk(reason, path, &format!("{label} data absent reason"), false);
            return;
        };
        self.readable
            .push(located(label, &not_supplied(&recorded), path));
        // The reason's codes stay inspectable without crowding the reading.
        if reason.is_object() {
            self.technical_leaves(reason, path);
        }
    }

    fn technical_leaves(&mut self, value: &Value, path: &SourcePath) {
        match value {
            Value::Array(items) => {
                for (index, item) in items.iter().enumerate() {
                    self.technical_leaves(item, &path.index(index));
                }
            }
            Value::Object(object) => {
                for (key, value) in object {
                    self.technical_leaves(value, &path.member(key));
                }
            }
            scalar => self.technical.push(located(
                &path.to_string(),
                &self.numbers.recorded(scalar, path),
                path,
            )),
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
    let result = matches!(
        resource.get("resourceType").and_then(Value::as_str),
        Some("Observation" | "DiagnosticReport")
    );
    let mut keys: Vec<_> = resource.keys().collect();
    keys.sort_by_key(|key| reading_rank(element_name(key)));
    for key in keys {
        let value = &resource[key.as_str()];
        let path = SourcePath::root().member(key);
        // FHIR carries a primitive's extensions under `_name`; one marking the value absent
        // reads where the value would, and any other stays technical.
        let element = element_name(key);
        let marks_absent = value.as_object().and_then(absent_extension).is_some();
        let placement = if element == key || marks_absent {
            Placement::of(element)
        } else {
            Placement::Technical
        };
        let mut flattened = Flattened {
            numbers,
            readable: Vec::new(),
            technical: Vec::new(),
        };
        let label = match element {
            "status" if result => "Result status".to_owned(),
            _ => top_label(element, placement),
        };
        if key == "dataAbsentReason" {
            flattened.absent(value, &path, "Value");
        } else if placement == Placement::Status
            && value.is_object()
            && let Some(status) = status_text(value)
        {
            // The status reads by name beside the record kind; its codes stay technical.
            flattened.readable.push(located(&label, &status, &path));
            flattened.technical_leaves(value, &path);
        } else {
            flattened.walk(value, &path, &label, placement == Placement::Technical);
        }
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
            Placement::Status => view.statuses.extend(readable),
            Placement::Detail | Placement::Technical => view.record_details.extend(readable),
        }
    }
}

/// The element a top-level key carries: `_status` holds the extensions of `status`.
fn element_name(key: &str) -> &str {
    key.strip_prefix('_').unwrap_or(key)
}

/// The recorded reason when an element holds only FHIR's data-absent-reason extension.
fn absent_extension(object: &serde_json::Map<String, Value>) -> Option<&str> {
    if object.keys().any(|key| key != "extension" && key != "id") {
        return None;
    }
    object
        .get("extension")?
        .as_array()?
        .iter()
        .find(|extension| extension["url"] == DATA_ABSENT_REASON)
        .and_then(|extension| text(&extension["valueCode"]))
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
pub(crate) fn humanize(key: &str) -> String {
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
        } else if character == '_' || character == '-' {
            words.push(' ');
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
pub(crate) mod tests {
    use super::*;
    use serde_json::json;

    pub(crate) fn citation(content: &str) -> Citation {
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
        assert_eq!(
            view.dates,
            vec![located("Recorded", "2020-09-14", "recordedDate")]
        );
        assert_eq!(
            view.sections[0],
            located(
                "Statement in this record",
                "No known drug allergies",
                "code"
            )
        );
        assert!(view.coda_preview.is_none());
        assert!(view.cited_passage.is_none());
        assert!(view.attribution.is_empty());
        assert_eq!(view.technical.original.as_bytes(), original.as_bytes());
        assert_eq!(
            view.statuses,
            vec![located(
                "Source status recorded",
                "active",
                "clinicalStatus"
            )]
        );
        assert!(view.record_details.is_empty());
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
            located(
                "Statement in this record",
                "No known drug allergies",
                "code"
            )
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
        assert!(view.record_details.contains(&located(
            "Criticality recorded (not reaction severity)",
            "high",
            "criticality"
        )));
        assert_eq!(
            view.statuses,
            vec![located(
                "Verification status recorded",
                "Confirmed",
                "verificationStatus"
            )]
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
        assert_eq!(general.sections, vec![located("Value", "42", "value")]);
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
                located("Value", "42", "value")
            ]
        );
        let array = inspection(&citation(r#"[{"value":42}]"#));
        assert!(
            array.sections[0]
                .text
                .contains("retained source is available")
        );
    }

    pub(crate) fn general_citation(record: &Value) -> Citation {
        let mut cited = citation(&record.to_string());
        cited.record_type = record["resourceType"].as_str().unwrap_or("Record").into();
        cited
    }

    /// Every scalar leaf of `record`, with its source path and recorded text.
    fn leaves(record: &Value, numbers: &NumberLiterals) -> Vec<(String, String)> {
        crate::source_path::nodes(record)
            .into_iter()
            .filter(|(_, value)| !value.is_object() && !value.is_array())
            .map(|(path, value)| (path.to_string(), numbers.recorded(value, &path)))
            .collect()
    }

    /// Fails naming the first leaf of `record` that its general view does not show.
    fn assert_nothing_dropped(record: &Value) {
        assert_content_shown(&general_citation(record));
    }

    fn assert_content_shown(cited: &Citation) {
        assert_view_shows_every_leaf(cited, &inspection(cited));
    }

    /// Fails naming the first leaf of the cited record that `view` does not show.
    pub(crate) fn assert_view_shows_every_leaf(cited: &Citation, view: &SourceInspection) {
        let record: Value = serde_json::from_str(&cited.content).unwrap();
        let numbers = NumberLiterals::read(&cited.content);
        let shown: Vec<&SourceField> = readable(view)
            .into_iter()
            .chain(&view.technical.fields)
            .chain(&view.technical.storage_dates)
            .collect();
        for (path, value) in leaves(&record, &numbers) {
            let technical = shown
                .iter()
                .any(|field| field.label == path && field.text == value);
            let readable = shown.iter().any(|field| field.text.contains(&value));
            assert!(technical || readable, "{path} = {value} is not shown");
        }
    }

    /// Whether Technical details carries `value` under its source path.
    fn technical(view: &SourceInspection, path: &str, value: &str) -> bool {
        view.technical.fields.contains(&located(path, value, path))
    }

    pub(crate) fn observation() -> Value {
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
                located(
                    "Statement in this record",
                    "Hemoglobin A1c\nHemoglobin A1c/Hemoglobin.total in Blood (4548-4 · http://loinc.org)",
                    "code"
                ),
                located(
                    "Value",
                    "7.2 % (http://unitsofmeasure.org)",
                    "valueQuantity"
                ),
                located(
                    "Interpretation",
                    "High (H · http://terminology.hl7.org/CodeSystem/v3-ObservationInterpretation)",
                    "interpretation[0]"
                ),
                located("Reference range low", "4.0 %", "referenceRange[0].low"),
                located("Reference range high", "5.6 %", "referenceRange[0].high"),
                located(
                    "Reference range text",
                    "4.0-5.6 %",
                    "referenceRange[0].text"
                ),
            ]
        );
        assert_eq!(
            view.dates,
            vec![
                located(
                    "Effective",
                    "2019-03-12T13:30:00+00:00",
                    "effectiveDateTime"
                ),
                located("Issued", "2019-03-13T09:00:00+00:00", "issued"),
            ]
        );
        assert_eq!(
            view.attribution,
            vec![located("Performed by", "Dr. Lab", "performer[0].display")]
        );
        assert_eq!(
            view.statuses,
            vec![located("Result status", "final", "status")]
        );
        assert!(technical(&view, "id", "obs-1"));
        assert!(technical(
            &view,
            "performer[0].reference",
            "Practitioner/p-1"
        ));
        assert!(technical(&view, "extension[0].valueString", "kept"));
        assert!(technical(
            &view,
            "meta.lastUpdated",
            "2019-03-12T00:00:00+00:00"
        ));
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
        assert!(view.sections.contains(&located(
            "Systolic Blood Pressure",
            "159 mmHg (mm[Hg] · http://unitsofmeasure.org)",
            "component[0].valueQuantity"
        )));
        assert!(view.sections.contains(&located(
            "Diastolic Blood Pressure",
            "99 mmHg (mm[Hg] · http://unitsofmeasure.org)",
            "component[1].valueQuantity"
        )));
        assert!(view.sections.contains(&located(
            "Systolic Blood Pressure code",
            "Systolic Blood Pressure\nSystolic blood pressure (8480-6 · http://loinc.org)",
            "component[0].code"
        )));
        assert_eq!(
            view.dates,
            vec![located(
                "Effective",
                "2019-03-12T13:30:00+00:00",
                "effectiveDateTime"
            )]
        );
        assert!(technical(
            &view,
            "encounter.reference",
            "Encounter/enc-26183a468076"
        ));
        assert_nothing_dropped(&blood_pressure());
    }

    pub(crate) fn condition() -> Value {
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
                located(
                    "Statement in this record",
                    "Type 2 diabetes mellitus without complications (E11.9 · http://hl7.org/fhir/sid/icd-10-cm)",
                    "code"
                ),
                located(
                    "Note",
                    "about 14 years per patient and prior records",
                    "note[0].text"
                ),
            ]
        );
        assert_eq!(
            view.dates,
            vec![
                located("Onset recorded", "2005-06-01", "onsetDateTime"),
                located("Recorded", "2019-03-12T13:20:00+00:00", "recordedDate"),
                located(
                    "Abatement recorded",
                    "2019-06-19T19:00:00+00:00",
                    "abatementDateTime"
                ),
            ]
        );
        // Statuses read by name beside the kind; their codes stay under Technical details.
        assert_eq!(
            view.statuses,
            vec![
                located("Clinical status", "Resolved", "clinicalStatus"),
                located("Verification status", "Confirmed", "verificationStatus"),
            ]
        );
        assert!(technical(
            &view,
            "clinicalStatus.coding[0].code",
            "resolved"
        ));
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
            located(
                "Medication",
                "metformin hydrochloride 1000 mg oral tablet\nmetformin hydrochloride 1000 MG Oral Tablet (861004 · http://www.nlm.nih.gov/research/umls/rxnorm)",
                "medicationCodeableConcept"
            )
        );
        assert_eq!(
            view.dates,
            vec![located(
                "Authored",
                "2019-03-12T13:20:00+00:00",
                "authoredOn"
            )]
        );
        assert_eq!(
            view.attribution,
            vec![located(
                "Requested by",
                "Dr. Prescriber",
                "requester.display"
            )]
        );
        assert_eq!(view.statuses, vec![located("Status", "active", "status")]);
        assert_eq!(view.record_details[0], located("Intent", "order", "intent"));
        for detail in [
            located(
                "Dosage instruction text",
                "Take 1 tablet by mouth twice daily with meals",
                "dosageInstruction[0].text",
            ),
            located(
                "Dosage instruction timing code",
                "BID",
                "dosageInstruction[0].timing.code.text",
            ),
            located(
                "Dosage instruction dose and rate dose",
                "1000 mg",
                "dosageInstruction[0].doseAndRate[0].doseQuantity",
            ),
            located(
                "Dosage instruction route",
                "Oral",
                "dosageInstruction[0].route.text",
            ),
            located("Reported", "true", "reportedBoolean"),
            located("Reason code", "Type 2 diabetes", "reasonCode[0].text"),
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
            vec![located(
                "Statement in this record",
                "Tdap (115 · http://hl7.org/fhir/sid/cvx)",
                "vaccineCode"
            )]
        );
        assert_eq!(
            view.dates,
            vec![located(
                "Occurred",
                "2019-03-12T14:15:00+00:00",
                "occurrenceDateTime"
            )]
        );
        assert_eq!(
            view.statuses,
            vec![located("Status", "completed", "status")]
        );
        assert!(
            view.record_details
                .contains(&located("Primary source", "true", "primarySource"))
        );
        assert!(technical(&view, "identifier[0].value", "imm-1"));
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
        assert!(
            view.sections
                .contains(&located("Value", "7.20 %", "valueQuantity"))
        );
        assert!(view.sections.contains(&located(
            "Reference range low",
            "4.0 %",
            "referenceRange[0].low"
        )));
        assert!(view.sections.contains(&located(
            "Reference range high",
            "5.60 %",
            "referenceRange[0].high"
        )));
        assert!(
            view.sections
                .contains(&located("Count", "1e3", "component[0].valueInteger"))
        );
        assert!(
            view.record_details
                .contains(&located("Extra", "-0.50", "extra"))
        );
        assert_content_shown(&cited);
    }

    #[test]
    fn a_shape_never_seen_still_reads_every_value_under_its_label() {
        let record = json!({"kind":"lab-result","analyte":{"name":"Potassium","flags":["H",null]},
            "result":{"value":5.9,"units":"mmol/L"},"collected":"2024-02-01","n":[1,2.50]});
        let view = inspection(&general_citation(&record));
        assert_eq!(view.record_kind, "Record record");
        assert!(view.record_details.contains(&located(
            "Analyte name",
            "Potassium",
            "analyte.name"
        )));
        assert!(view.record_details.contains(&located(
            "Analyte flags 2",
            "null",
            "analyte.flags[1]"
        )));
        assert!(
            view.record_details
                .contains(&located("Result units", "mmol/L", "result.units"))
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
                .contains(&located("Type recorded", "intolerance", "type"))
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
            vec![located(
                "Recorded",
                "2018-09-11T12:20:00+00:00",
                "recordedDate"
            )]
        );
        assert_eq!(
            view.technical.storage_dates,
            vec![located(
                "meta.lastUpdated",
                "2018-09-11T00:00:00+00:00",
                "meta.lastUpdated"
            )]
        );
        assert_eq!(
            view.statuses,
            vec![
                located("Source status recorded", "Active", "clinicalStatus"),
                located(
                    "Verification status recorded",
                    "Confirmed",
                    "verificationStatus"
                ),
            ]
        );
        assert!(view.record_details.contains(&located(
            "Criticality recorded (not reaction severity)",
            "low",
            "criticality"
        )));
        assert_eq!(view.technical.original, line);
    }

    /// Every readable field of `view`, wherever it reads.
    pub(crate) fn readable(view: &SourceInspection) -> Vec<&SourceField> {
        view.sections
            .iter()
            .chain(&view.dates)
            .chain(&view.attribution)
            .chain(&view.statuses)
            .chain(&view.record_details)
            .chain(
                view.layout_sections
                    .iter()
                    .flat_map(|section| &section.fields),
            )
            .chain(&view.other_fields)
            .collect()
    }

    /// The value at a field's path such as `results[1].units`, in the readable form the field
    /// carries, if the record has one.
    pub(crate) fn at<'a>(record: &'a Value, path: &str) -> Option<(SourcePath, &'a Value)> {
        crate::source_path::nodes(record)
            .into_iter()
            .find(|(at, _)| at.to_string() == path)
    }

    /// A non-FHIR source: a lab export in its own field names, read as recorded.
    pub(crate) fn vendor_lab() -> Citation {
        let content = include_str!("../fixtures/records/vendor-lab-result.json");
        let mut cited = citation(content);
        cited.record_type = "LabResult".into();
        cited.reference = "lab-result/88213".into();
        cited
    }

    #[test]
    fn a_record_from_a_source_that_is_not_fhir_reads_as_labeled_fields() {
        let cited = vendor_lab();
        let view = inspection(&cited);
        assert_eq!(view.record_kind, "Lab result record");
        assert_eq!(view.title, "Lab result record");
        assert_eq!(view.statuses, vec![located("Status", "final", "status")]);
        for expected in [
            located("Results 1 test name", "Potassium", "results[0].test_name"),
            located("Results 1 value", "5.9", "results[0].value"),
            located("Results 1 units", "mmol/L", "results[0].units"),
            located(
                "Results 1 reference range",
                "3.5-5.1",
                "results[0].reference_range",
            ),
            located("Results 1 abnormal flag", "H", "results[0].abnormal_flag"),
            located(
                "Results 3 result status",
                "preliminary",
                "results[2].result_status",
            ),
            located(
                "Performing lab",
                "Synthetic Reference Lab",
                "performing_lab",
            ),
            located("Lab result id", "88213", "lab_result_id"),
        ] {
            assert!(
                view.record_details.contains(&expected),
                "{expected:?} in {:#?}",
                view.record_details
            );
        }
        assert!(view.attribution.contains(&located(
            "Patient in source display name",
            "Synthetic Patient",
            "patient.display_name"
        )));
        // The result without a unit or range shows none, and is not given another's.
        assert!(
            !readable(&view)
                .iter()
                .any(|field| field.path.as_deref() == Some("results[2].units"))
        );
        assert!(
            readable(&view)
                .iter()
                .all(|field| !field.text.starts_with('{') && !field.text.starts_with('['))
        );
        assert_content_shown(&cited);
    }

    #[test]
    fn every_readable_field_keeps_the_exact_path_its_value_was_copied_from() {
        for cited in [
            vendor_lab(),
            general_citation(&observation()),
            general_citation(&blood_pressure()),
            general_citation(&condition()),
            general_citation(&medication_request()),
            general_citation(&immunization()),
        ] {
            let record: Value = serde_json::from_str(&cited.content).unwrap();
            let numbers = NumberLiterals::read(&cited.content);
            let view = inspection(&cited);
            for shown in readable(&view).into_iter().chain(&view.technical.fields) {
                let path = shown.path.as_deref().unwrap_or_else(|| {
                    panic!("{} {shown:?} has no source path", cited.record_type)
                });
                let (path, value) = at(&record, path)
                    .unwrap_or_else(|| panic!("{} has no {path}", cited.record_type));
                if !value.is_object() && !value.is_array() {
                    assert_eq!(
                        shown.text,
                        numbers.recorded(value, &path),
                        "{} {path}",
                        cited.record_type
                    );
                }
            }
        }
    }

    #[test]
    fn values_with_identical_text_stay_distinguishable_by_path() {
        let view = inspection(&vendor_lab());
        let paths = |text: &str| -> Vec<String> {
            readable(&view)
                .into_iter()
                .filter(|field| field.text == text)
                .filter_map(|field| field.path.clone())
                .collect()
        };
        assert_eq!(
            paths("final"),
            [
                "status",
                "results[0].result_status",
                "results[1].result_status"
            ]
        );
        assert_eq!(paths("mmol/L"), ["results[0].units", "results[1].units"]);
    }

    #[test]
    fn readable_text_without_a_field_mapping_claims_no_path() {
        let mut cited = general_citation(&observation());
        cited
            .readable_parts
            .push(local_product::physician_sources::ReadablePart {
                label: "Report attachment".into(),
                text: "A1c 7.2%".into(),
            });
        let view = inspection(&cited);
        assert_eq!(view.sections[0], field("Report attachment", "A1c 7.2%"));
        assert_eq!(view.sections[0].path, None);
        let attachment = inspection(&citation("Plain text note"));
        assert_eq!(attachment.sections[0].path, None);
    }

    #[test]
    fn status_reads_beside_kind_and_date_not_among_record_details() {
        // The bank's inactive "no known drug allergies" record.
        let inactive = inspection(&citation(
            r#"{"resourceType":"AllergyIntolerance","clinicalStatus":{"coding":[{"system":"http://terminology.hl7.org/CodeSystem/allergyintolerance-clinical","code":"inactive","display":"Inactive"}]},"verificationStatus":{"coding":[{"code":"confirmed","display":"Confirmed"}]},"category":["medication"],"code":{"text":"No known drug allergies"},"recordedDate":"2018-09-11T12:20:00+00:00"}"#,
        ));
        assert_eq!(
            inactive.statuses[0],
            located("Source status recorded", "Inactive", "clinicalStatus")
        );
        assert!(
            inactive
                .record_details
                .iter()
                .all(|detail| !detail.label.contains("status"))
        );
        let mut preliminary = observation();
        preliminary["status"] = json!("preliminary");
        let preliminary = inspection(&general_citation(&preliminary));
        assert_eq!(
            preliminary.statuses,
            vec![located("Result status", "preliminary", "status")]
        );
        let refuted = inspection(&general_citation(&json!({"resourceType":"Condition",
            "verificationStatus":{"coding":[{"code":"refuted"}]},"code":{"text":"Asthma"}})));
        assert_eq!(
            refuted.statuses,
            vec![located(
                "Verification status",
                "refuted",
                "verificationStatus"
            )]
        );
        let mut errored = medication_request();
        errored["status"] = json!("entered-in-error");
        let errored = inspection(&general_citation(&errored));
        assert_eq!(
            errored.statuses,
            vec![located("Status", "entered-in-error", "status")]
        );
        assert!(
            errored
                .record_details
                .contains(&located("Intent", "order", "intent"))
        );
    }

    #[test]
    fn a_value_the_record_marks_absent_reads_as_not_supplied_with_its_reason() {
        let record = json!({"resourceType":"Observation","status":"final",
            "code":{"text":"Hemoglobin A1c"},
            "dataAbsentReason":{"coding":[{"system":"http://terminology.hl7.org/CodeSystem/data-absent-reason","code":"not-performed","display":"Not Performed"}]},
            "component":[
                {"code":{"text":"Systolic"},"dataAbsentReason":{"text":"Patient refused"}},
                {"code":{"text":"Diastolic"},"valueQuantity":{"value":80,"unit":"mmHg"}}],
            "effectiveDateTime":"2019-03-12",
            "_issued":{"extension":[{"url":"http://hl7.org/fhir/StructureDefinition/data-absent-reason","valueCode":"unknown"}]},
            "futureElement":{"kind":"unrecognized"}});
        let view = inspection(&general_citation(&record));
        assert!(view.sections.contains(&located(
            "Value",
            "Not supplied. Reason recorded: Not Performed",
            "dataAbsentReason"
        )));
        assert!(view.sections.contains(&located(
            "Systolic",
            "Not supplied. Reason recorded: Patient refused",
            "component[0].dataAbsentReason"
        )));
        assert!(view.dates.contains(&located(
            "Issued",
            "Not supplied. Reason recorded: unknown",
            "_issued"
        )));
        assert!(technical(
            &view,
            "dataAbsentReason.coding[0].code",
            "not-performed"
        ));
        // An unfamiliar element stays visible under its source label and is not called absent.
        assert!(view.record_details.contains(&located(
            "Future element kind",
            "unrecognized",
            "futureElement.kind"
        )));
        assert_eq!(
            readable(&view)
                .iter()
                .filter(|field| field.text.starts_with("Not supplied"))
                .count(),
            3
        );
        assert_nothing_dropped(&record);
    }

    #[test]
    fn a_missing_field_is_not_reported_absent() {
        let view = inspection(&general_citation(
            &json!({"resourceType":"Observation","status":"final","code":{"text":"Sodium"}}),
        ));
        assert!(
            readable(&view)
                .iter()
                .all(|field| !field.text.contains("Not supplied"))
        );
        assert!(!view.sections.iter().any(|field| field.label == "Value"));
    }
}
