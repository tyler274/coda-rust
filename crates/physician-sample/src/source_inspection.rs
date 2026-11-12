//! How a physician reads a cited record: the source's own statement and dates first, the exact
//! retained bytes and storage details under Technical details. Nothing here infers clinical truth.
//!
//! A record type's layout places the fields it knows and removes each one it shows from a copy of
//! the record. Whatever is left is shown by the general view under the source's own field names,
//! so no recorded field depends on a layout knowing it.
use local_product::physician_chat::Citation;
use physician_wire::{SourceField, SourceInspection, SourcePreviewContent, SourceTechnicalDetails};
use serde_json::Value;

/// The storage envelope, already shown under Technical details as the record's identifier,
/// revision, storage dates, and exact bytes.
const ENVELOPE: [&str; 3] = ["resourceType", "id", "meta"];

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
        },
    };
    if let Some(label) = text(&citation.provenance["source_label"]) {
        view.attribution.push(field("Source system", label));
    }
    if let Some(resource) = resource.as_ref().filter(|resource| resource.is_object()) {
        let mut rest = resource.clone();
        for date in &citation.dates {
            remove_path(&mut rest, &date.name);
        }
        if resource["resourceType"] == "AllergyIntolerance" {
            allergy(&mut rest, &mut view);
        } else if text(&resource["resourceType"]).is_some() {
            record(&mut rest, &mut view);
        }
        if let Some(narrative) = narrative(resource) {
            view.sections
                .push(field("Narrative in this record", &narrative));
        }
        // The narrative's markup and generation status are its form, kept in Technical details;
        // a narrative with no readable text has nothing more to show.
        if resource["text"]["div"].is_string() {
            remove(&mut rest, "text");
        }
        general(rest, &mut view);
    }
    if view.sections.is_empty() && view.record_details.is_empty() {
        view.sections.push(match resource {
            Some(_) => field(
                "Source content",
                "A readable presentation is not available for this format. The retained source is available under Technical details.",
            ),
            None => field("Source text", &citation.content),
        });
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

fn owned_text(value: &Value) -> Option<String> {
    text(value).map(str::to_owned)
}

fn readable_label(value: &Value) -> Option<String> {
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

/// A coded concept's readable label, or its supplied code when the record gives no label.
fn concept(value: &Value) -> Option<String> {
    if let Some(label) = readable_label(value) {
        return Some(label);
    }
    let codings = value["coding"]
        .as_array()
        .map(Vec::as_slice)
        .unwrap_or_default();
    let codes: Vec<_> = codings
        .iter()
        .filter_map(|coding| {
            let code = text(&coding["code"])?;
            Some(match text(&coding["system"]) {
                Some(system) => format!("Code {code} ({system})"),
                None => format!("Code {code}"),
            })
        })
        .collect();
    (!codes.is_empty()).then(|| codes.join("; "))
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

fn remove(rest: &mut Value, name: &str) {
    if let Some(fields) = rest.as_object_mut() {
        fields.remove(name);
    }
}

/// Removes a dotted field such as `period.start`, which a retained date names.
fn remove_path(rest: &mut Value, path: &str) {
    match path.split_once('.') {
        Some((head, tail)) => {
            if let Some(inner) = rest.get_mut(head) {
                remove_path(inner, tail);
            }
        }
        None => remove(rest, path),
    }
}

/// How a layout reads one kind of value, and which of its parts that reading shows. Parts it
/// does not show stay for the general view.
struct Reader {
    show: fn(&Value) -> Option<String>,
    /// The parts of an object value the reading shows; empty when it shows the whole value.
    parts: &'static [&'static str],
}

const WHOLE_TEXT: Reader = Reader {
    show: owned_text,
    parts: &[],
};
const QUANTITY: Reader = Reader {
    show: quantity,
    parts: &[],
};
/// A labelled concept reads as its label; its codings are the terminology behind it.
const CONCEPT: Reader = Reader {
    show: concept,
    parts: &["text", "coding"],
};
const CODING: Reader = Reader {
    show: coding,
    parts: &["display", "code", "system"],
};
const CONCEPT_OR_CODING: Reader = Reader {
    show: concept_or_coding,
    parts: &["text", "coding", "display", "code", "system"],
};
const RANGE: Reader = Reader {
    show: reference_range,
    parts: &["text", "low", "high"],
};
/// A reference reads as its display; the identifier behind it is technical.
const PERSON: Reader = Reader {
    show: display,
    parts: &["display", "reference"],
};
/// An allergy status reads as its label, or as its first code when it has no label.
const STATUS: Reader = Reader {
    show: status,
    parts: &["text", "coding"],
};
const NOTE: Reader = Reader {
    show: note_text,
    parts: &["text"],
};

fn concept_or_coding(value: &Value) -> Option<String> {
    concept(value).or_else(|| coding(value))
}

fn display(value: &Value) -> Option<String> {
    owned_text(&value["display"])
}

fn status(value: &Value) -> Option<String> {
    readable_label(value).or_else(|| {
        value["coding"]
            .as_array()
            .and_then(|coding| coding.first())
            .and_then(|coding| owned_text(&coding["code"]))
    })
}

fn note_text(value: &Value) -> Option<String> {
    owned_text(&value["text"])
}

/// Reads `value`, and when it shows, removes the parts it showed. Returns whether anything of
/// the value is left to show.
fn read_out(value: &mut Value, reader: &Reader) -> (Option<String>, bool) {
    let Some(shown) = (reader.show)(value) else {
        return (None, true);
    };
    let left = match value.as_object_mut() {
        Some(parts) if !reader.parts.is_empty() => {
            for part in reader.parts {
                parts.remove(*part);
            }
            !parts.is_empty()
        }
        _ => false,
    };
    (Some(shown), left)
}

/// The field's presentation when `reader` can read it, removing what it showed from what is
/// left to show.
fn take(rest: &mut Value, name: &str, reader: &Reader) -> Option<String> {
    let (shown, left) = read_out(rest.get_mut(name)?, reader);
    if !left {
        remove(rest, name);
    }
    shown
}

/// Each value of a field that holds one value or a list of them that `reader` can read, with
/// what it showed removed. Values and parts it cannot read stay for the general view.
fn take_each(rest: &mut Value, name: &str, reader: &Reader) -> Vec<String> {
    match rest.get_mut(name) {
        Some(Value::Array(items)) => {
            let mut shown = Vec::new();
            items.retain_mut(|item| {
                let (value, left) = read_out(item, reader);
                shown.extend(value);
                left
            });
            if items.is_empty() {
                remove(rest, name);
            }
            shown
        }
        Some(_) => take(rest, name, reader).into_iter().collect(),
        None => Vec::new(),
    }
}

/// The fields that name what a record is about, in the order a record type uses them.
const STATEMENT_FIELDS: [&str; 6] = [
    "code",
    "vaccineCode",
    "medicationCodeableConcept",
    "type",
    "serviceType",
    "reasonCode",
];

/// Any other chart record, arranged so it reads as a record: its statement, its recorded values,
/// and its status and attribution. Every value shown is copied from the record; a field the
/// record does not carry is left out rather than reported missing.
fn record(rest: &mut Value, view: &mut SourceInspection) {
    let statement = STATEMENT_FIELDS
        .iter()
        .map(|name| take_each(rest, name, &CONCEPT))
        .find(|shown| !shown.is_empty())
        .map(|shown| shown.join("; "));
    if let Some(statement) = statement {
        view.title = statement.clone();
        view.sections
            .insert(0, field("Statement in this record", &statement));
    } else if let Some(name) = take_first_name(rest) {
        view.title = name.clone();
        view.sections.insert(0, field("Name recorded", &name));
    }
    for (name, label) in [
        ("gender", "Sex or gender recorded"),
        ("birthDate", "Birth date recorded"),
    ] {
        if let Some(value) = take(rest, name, &WHOLE_TEXT) {
            view.record_details.push(field(label, &value));
        }
    }
    if let Some(value) = take_value(rest) {
        view.sections.push(field("Recorded value", &value));
    }
    for component in rest
        .get_mut("component")
        .and_then(Value::as_array_mut)
        .into_iter()
        .flatten()
    {
        let label = concept(&component["code"]).unwrap_or_else(|| "Component".into());
        let mut lines: Vec<_> = take_value(component)
            .or_else(|| take(component, "dataAbsentReason", &CONCEPT))
            .into_iter()
            .collect();
        let ranges = take_each(component, "referenceRange", &RANGE);
        if !ranges.is_empty() {
            lines.push(format!("Reference range recorded: {}", ranges.join("; ")));
        }
        let interpretations = take_each(component, "interpretation", &CONCEPT);
        if !interpretations.is_empty() {
            lines.push(format!(
                "Interpretation recorded: {}",
                interpretations.join("; ")
            ));
        }
        if !lines.is_empty() {
            // The name stays with any fields left for the general view, so they read as this
            // component's.
            if component
                .as_object()
                .is_some_and(|fields| fields.len() == 1)
            {
                remove(component, "code");
            }
            view.sections.push(field(&label, &lines.join("\n")));
        }
    }
    let ranges = take_each(rest, "referenceRange", &RANGE);
    if !ranges.is_empty() {
        view.sections
            .push(field("Reference range recorded", &ranges.join("\n")));
    }
    let interpretations = take_each(rest, "interpretation", &CONCEPT);
    if !interpretations.is_empty() {
        view.sections.push(field(
            "Interpretation recorded",
            &interpretations.join("; "),
        ));
    }
    let dosage: Vec<_> = rest
        .get_mut("dosageInstruction")
        .and_then(Value::as_array_mut)
        .into_iter()
        .flatten()
        .filter_map(|dosage| take(dosage, "text", &WHOLE_TEXT))
        .collect();
    if !dosage.is_empty() {
        view.sections
            .push(field("Dosage instruction", &dosage.join("\n")));
    }
    for (name, label) in [("description", "Description"), ("conclusion", "Conclusion")] {
        if let Some(value) = take(rest, name, &WHOLE_TEXT) {
            view.sections.push(field(label, &value));
        }
    }
    let reasons = take_each(rest, "reasonCode", &CONCEPT);
    if !reasons.is_empty() {
        view.sections
            .push(field("Reason recorded", &reasons.join("; ")));
    }
    for note in take_each(rest, "note", &NOTE) {
        view.sections.push(field("Source note", &note));
    }
    for (name, label) in [("status", "Status recorded"), ("intent", "Intent recorded")] {
        if let Some(value) = take(rest, name, &WHOLE_TEXT) {
            view.record_details.push(field(label, &value));
        }
    }
    for (name, label) in [
        ("clinicalStatus", "Source status recorded"),
        ("verificationStatus", "Verification status recorded"),
        ("category", "Category recorded"),
        ("class", "Class recorded"),
    ] {
        let values = take_each(rest, name, &CONCEPT_OR_CODING);
        if !values.is_empty() {
            view.record_details.push(field(label, &values.join("; ")));
        }
    }
    for (name, label) in [
        ("performer", "Performed by"),
        ("author", "Authored by"),
        ("requester", "Requested by"),
        ("recorder", "Recorded by"),
        ("asserter", "Asserted by"),
    ] {
        let people = take_each(rest, name, &PERSON);
        if !people.is_empty() {
            view.attribution.push(field(label, &people.join("; ")));
        }
    }
}

/// The first recorded name of a person, as written or from its given and family parts. The
/// name's other parts and any further names stay for the general view.
fn take_first_name(rest: &mut Value) -> Option<String> {
    let names = rest.get_mut("name")?.as_array_mut()?;
    let name = names.first_mut()?;
    let shown = match take(name, "text", &WHOLE_TEXT) {
        Some(text) => text,
        None => {
            let parts: Vec<_> = name["given"]
                .as_array()
                .into_iter()
                .flatten()
                .chain(std::iter::once(&name["family"]))
                .filter_map(text)
                .collect();
            if parts.is_empty() {
                return None;
            }
            let shown = parts.join(" ");
            remove(name, "given");
            remove(name, "family");
            shown
        }
    };
    if name.as_object().is_some_and(serde_json::Map::is_empty) {
        names.remove(0);
    }
    Some(shown)
}

/// A bare `Coding`'s label or code, as `class` carries it.
fn coding(value: &Value) -> Option<String> {
    text(&value["display"])
        .or_else(|| text(&value["code"]))
        .map(str::to_owned)
}

/// The value a record or component carries, copied as recorded with its unit.
fn take_value(rest: &mut Value) -> Option<String> {
    fn scalar(value: &Value) -> Option<String> {
        match value {
            Value::String(text) => Some(text.clone()),
            Value::Null => None,
            other => Some(other.to_string()),
        }
    }
    const SCALAR: Reader = Reader {
        show: scalar,
        parts: &[],
    };
    let readers = [
        ("valueQuantity", &QUANTITY),
        ("valueCodeableConcept", &CONCEPT),
        ("valueCoding", &CODING),
        ("valueString", &WHOLE_TEXT),
        ("valueBoolean", &SCALAR),
        ("valueInteger", &SCALAR),
        ("valueDateTime", &SCALAR),
    ];
    readers
        .into_iter()
        .find_map(|(name, read)| take(rest, name, read))
}

fn quantity(value: &Value) -> Option<String> {
    let number = match &value["value"] {
        Value::Number(number) => number.to_string(),
        _ => return None,
    };
    let comparator = text(&value["comparator"]).unwrap_or_default();
    Some(
        match text(&value["unit"]).or_else(|| text(&value["code"])) {
            Some(unit) => format!("{comparator}{number} {unit}"),
            None => format!("{comparator}{number}"),
        },
    )
}

fn reference_range(range: &Value) -> Option<String> {
    if let Some(text) = text(&range["text"]) {
        return Some(text.into());
    }
    match (quantity(&range["low"]), quantity(&range["high"])) {
        (Some(low), Some(high)) => Some(format!("{low} to {high}")),
        (Some(low), None) => Some(format!("At least {low}")),
        (None, Some(high)) => Some(format!("At most {high}")),
        (None, None) => None,
    }
}

/// The general view: every field no layout placed, one section per field under the source's own
/// name for it, and one per entry when the field lists several.
fn general(rest: Value, view: &mut SourceInspection) {
    let Value::Object(fields) = rest else {
        return;
    };
    for (name, value) in fields {
        if ENVELOPE.contains(&name.as_str()) {
            continue;
        }
        match value {
            Value::Array(items) => {
                let items: Vec<_> = items.iter().filter_map(readable).collect();
                let numbered = items.len() > 1;
                for (index, item) in items.into_iter().enumerate() {
                    let label = if numbered {
                        format!("{name} {}", index + 1)
                    } else {
                        name.clone()
                    };
                    view.sections.push(field(&label, &item));
                }
            }
            value => {
                if let Some(text) = readable(&value) {
                    view.sections.push(field(&name, &text));
                }
            }
        }
    }
}

/// Any recorded value as readable lines: a concept by its label, a quantity with its unit, a
/// reference by its display, and anything else as its own field names and values.
fn readable(value: &Value) -> Option<String> {
    let mut lines = Vec::new();
    readable_lines("", value, &mut lines);
    (!lines.is_empty()).then(|| lines.join("\n"))
}

fn readable_lines(path: &str, value: &Value, lines: &mut Vec<String>) {
    let leaf = match value {
        Value::Null => return,
        Value::String(text) => Some(text.trim().to_owned()).filter(|text| !text.is_empty()),
        Value::Bool(_) | Value::Number(_) => Some(value.to_string()),
        Value::Array(items) => {
            let numbered = items.len() > 1;
            for (index, item) in items.iter().enumerate() {
                let path = match (numbered, path.is_empty()) {
                    (false, _) => path.to_owned(),
                    (true, true) => format!("{}", index + 1),
                    (true, false) => format!("{path} {}", index + 1),
                };
                readable_lines(&path, item, lines);
            }
            return;
        }
        Value::Object(fields) => {
            let only = |allowed: &[&str]| fields.keys().all(|key| allowed.contains(&key.as_str()));
            // A reference with no display names a record by its identifier, which, like this
            // record's own identifier, is technical rather than something the record says.
            if only(&["reference", "type"]) {
                return;
            }
            if only(&["value", "unit", "code", "system", "comparator"]) {
                quantity(value)
            } else if only(&["coding", "text"]) {
                concept(value)
            } else if only(&["reference", "display", "type"]) {
                text(&value["display"])
                    .or_else(|| text(&value["reference"]))
                    .map(str::to_owned)
            } else {
                None
            }
            .or_else(|| {
                // An attachment's encoded bytes are not readable as written: its text, when the
                // source gives one, is its own section, and the bytes stay in Technical details.
                let attachment = fields.contains_key("contentType");
                for (name, inner) in fields {
                    let path = if path.is_empty() {
                        name.clone()
                    } else {
                        format!("{path}.{name}")
                    };
                    if attachment && name == "data" {
                        lines.push(format!(
                            "{path}: encoded content, retained under Technical details"
                        ));
                    } else {
                        readable_lines(&path, inner, lines);
                    }
                }
                None
            })
        }
    };
    if let Some(leaf) = leaf {
        lines.push(if path.is_empty() {
            leaf
        } else {
            format!("{path}: {leaf}")
        });
    }
}

/// The record's own human-readable narrative (`text.div`), as plain text. Blocks and table rows
/// start new lines, table cells stay apart, and a superscript keeps its meaning as `^`.
fn narrative(resource: &Value) -> Option<String> {
    let div = text(&resource["text"]["div"])?;
    let mut plain = String::new();
    let mut cells = 0;
    let mut rest = div;
    while let Some(start) = rest.find('<') {
        push_text(&mut plain, &rest[..start]);
        let Some(end) = rest[start..].find('>') else {
            rest = "";
            break;
        };
        let tag = &rest[start + 1..start + end];
        let opening = !tag.starts_with('/');
        let name = tag
            .trim_start_matches('/')
            .split([' ', '/', '\t', '\n'])
            .next()
            .unwrap_or_default()
            .to_ascii_lowercase();
        match name.as_str() {
            "td" | "th" if opening => {
                trim_line_end(&mut plain);
                if cells > 0 {
                    plain.push_str(" | ");
                }
                cells += 1;
            }
            "sup" if opening => {
                trim_line_end(&mut plain);
                plain.push('^');
            }
            "br" | "p" | "div" | "li" | "tr" | "table" | "thead" | "tbody" | "tfoot" | "ul"
            | "ol" | "caption" | "h1" | "h2" | "h3" | "h4" | "h5" | "h6" => {
                cells = 0;
                trim_line_end(&mut plain);
                if !plain.is_empty() && !plain.ends_with('\n') {
                    plain.push('\n');
                }
            }
            _ => {}
        }
        rest = &rest[start + end + 1..];
    }
    push_text(&mut plain, rest);
    let lines: Vec<_> = plain
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .collect();
    (!lines.is_empty()).then(|| lines.join("\n"))
}

/// Text between tags, with its whitespace collapsed as XHTML displays it and its character
/// references decoded.
fn push_text(plain: &mut String, raw: &str) {
    for character in decode_entities(raw).chars() {
        if character.is_whitespace() {
            if !plain.is_empty() && !plain.ends_with([' ', '\n', '^']) {
                plain.push(' ');
            }
        } else {
            plain.push(character);
        }
    }
}

fn trim_line_end(plain: &mut String) {
    while plain.ends_with(' ') {
        plain.pop();
    }
}

fn decode_entities(raw: &str) -> String {
    let mut decoded = String::with_capacity(raw.len());
    let mut rest = raw;
    while let Some(start) = rest.find('&') {
        decoded.push_str(&rest[..start]);
        rest = &rest[start..];
        let reference = rest[1..]
            .find(';')
            .filter(|end| *end <= 10)
            .and_then(|end| Some((entity(&rest[1..=end])?, end + 2)));
        match reference {
            Some((character, length)) => {
                decoded.push(character);
                rest = &rest[length..];
            }
            None => {
                decoded.push('&');
                rest = &rest[1..];
            }
        }
    }
    decoded.push_str(rest);
    decoded
}

fn entity(name: &str) -> Option<char> {
    if let Some(number) = name.strip_prefix('#') {
        let code = match number.strip_prefix(['x', 'X']) {
            Some(hex) => u32::from_str_radix(hex, 16).ok()?,
            None => number.parse().ok()?,
        };
        return char::from_u32(code);
    }
    Some(match name {
        "amp" => '&',
        "lt" => '<',
        "gt" => '>',
        "quot" => '"',
        "apos" => '\'',
        "nbsp" => ' ',
        "times" => '×',
        "deg" => '°',
        "micro" => 'µ',
        "plusmn" => '±',
        "le" => '≤',
        "ge" => '≥',
        _ => return None,
    })
}

fn allergy(rest: &mut Value, view: &mut SourceInspection) {
    view.record_kind = allergy_kind(rest);
    view.title = take(rest, "code", &CONCEPT)
        .unwrap_or_else(|| "Allergy or intolerance statement not supplied".into());
    if let Some(value) = take(rest, "type", &WHOLE_TEXT) {
        view.record_details.push(field("Type recorded", &value));
    }
    let medication_only = rest["category"]
        .as_array()
        .is_some_and(|categories| categories.iter().all(|value| value == "medication"));
    if medication_only {
        remove(rest, "category");
    }
    view.sections
        .insert(0, field("Statement in this record", &view.title));
    for name in ["recordedDate", "onsetDateTime", "lastOccurrence"] {
        if let Some(value) = take(rest, name, &WHOLE_TEXT)
            && !view
                .dates
                .iter()
                .any(|date| date.label == date_label(name) && date.text == value)
        {
            view.dates.push(field(date_label(name), &value));
        }
    }
    for (name, label) in [
        ("patient", "Patient in source"),
        ("recorder", "Recorded by"),
        ("asserter", "Asserted by"),
    ] {
        if let Some(value) = take(rest, name, &PERSON) {
            view.attribution.push(field(label, &value));
        }
    }
    for (name, label) in [
        ("clinicalStatus", "Source status recorded"),
        ("verificationStatus", "Verification status recorded"),
    ] {
        let status = take(rest, name, &STATUS);
        if let Some(value) = status {
            view.record_details.push(field(label, &value));
        }
    }
    if let Some(value) = take(rest, "criticality", &WHOLE_TEXT) {
        view.record_details.push(field(
            "Criticality recorded (not reaction severity)",
            &value,
        ));
    }
    let reactions = rest
        .get_mut("reaction")
        .and_then(Value::as_array_mut)
        .map(Vec::as_mut_slice)
        .unwrap_or_default();
    if reactions.is_empty() {
        view.sections.push(field(
            "Reactions",
            "Reaction details not supplied in this record.",
        ));
    }
    for (index, reaction) in reactions.iter_mut().enumerate() {
        let mut values = Vec::new();
        if let Some(substance) = take(reaction, "substance", &CONCEPT) {
            values.push(format!("Substance: {substance}"));
        }
        let manifestations = take_each(reaction, "manifestation", &CONCEPT);
        values.push(if manifestations.is_empty() {
            "Manifestation not supplied.".into()
        } else {
            format!("Manifestation: {}", manifestations.join("; "))
        });
        if let Some(value) = take(reaction, "description", &WHOLE_TEXT) {
            values.push(value);
        }
        if let Some(value) = take(reaction, "severity", &WHOLE_TEXT) {
            values.push(format!("Reaction severity recorded: {value}"));
        }
        if let Some(value) = take(reaction, "onset", &WHOLE_TEXT) {
            values.push(format!("Reaction onset recorded: {value}"));
        }
        view.sections.push(field(
            &format!("Reaction {}", index + 1),
            &values.join("\n"),
        ));
    }
    for note in take_each(rest, "note", &NOTE) {
        view.sections.push(field("Source note", &note));
    }
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
    fn a_code_without_a_label_is_shown_not_reported_missing() {
        let view = inspection(&citation(
            r#"{"resourceType":"AllergyIntolerance","code":{"coding":[{"system":"http://snomed.info/sct","code":"91936005"}]},
            "reaction":[{"manifestation":[{"coding":[{"code":"247472004"}]}]}]}"#,
        ));
        assert_eq!(view.title, "Code 91936005 (http://snomed.info/sct)");
        assert!(
            view.sections
                .iter()
                .any(|section| section.text == "Manifestation: Code 247472004")
        );
        assert!(
            !view
                .sections
                .iter()
                .any(|section| section.text.contains("not supplied"))
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
        assert_eq!(general.sections, vec![field("value", "42")]);
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
                field("value", "42")
            ]
        );
        let fallback = inspection(&citation(r#"["not", "a record"]"#));
        assert!(
            fallback.sections[0]
                .text
                .contains("retained source is available")
        );
    }

    /// The patient bank's own potassium result reads as its test, value, unit and range, with
    /// no fallback sentence and the LOINC code kept to Technical details.
    #[test]
    fn a_bank_observation_reads_as_its_recorded_value() {
        let line = r#"{"resourceType":"Observation","id":"obs-447fdd24f33f","meta":{"versionId":"1","lastUpdated":"2019-03-12T00:00:00+00:00"},"status":"final","category":[{"coding":[{"system":"http://terminology.hl7.org/CodeSystem/observation-category","code":"laboratory","display":"Laboratory"}]}],"code":{"coding":[{"system":"http://loinc.org","code":"2823-3","display":"Potassium [Moles/volume] in Serum or Plasma"}],"text":"Potassium"},"subject":{"reference":"Patient/MRN00001101"},"encounter":{"reference":"Encounter/enc-26183a468076"},"effectiveDateTime":"2019-03-12T14:10:00+00:00","issued":"2019-03-12T17:32:00+00:00","valueQuantity":{"value":3.9,"unit":"mmol/L","system":"http://unitsofmeasure.org","code":"mmol/L"},"referenceRange":[{"low":{"value":3.5,"unit":"mmol/L","system":"http://unitsofmeasure.org","code":"mmol/L"},"high":{"value":5.0,"unit":"mmol/L","system":"http://unitsofmeasure.org","code":"mmol/L"}}]}"#;
        let mut retained = citation(line);
        retained.record_type = "Observation".into();
        retained.dates = [
            ("effectiveDateTime", "2019-03-12T14:10:00+00:00"),
            ("issued", "2019-03-12T17:32:00+00:00"),
            ("meta.lastUpdated", "2019-03-12T00:00:00+00:00"),
        ]
        .map(
            |(name, value)| local_product::physician_sources::RecordDate {
                name: name.into(),
                value: value.into(),
            },
        )
        .into();
        let view = inspection(&retained);
        assert_eq!(view.title, "Potassium");
        assert_eq!(
            view.sections,
            vec![
                field("Statement in this record", "Potassium"),
                field("Recorded value", "3.9 mmol/L"),
                field("Reference range recorded", "3.5 mmol/L to 5.0 mmol/L"),
            ]
        );
        assert!(
            view.record_details
                .contains(&field("Status recorded", "final"))
        );
        assert!(
            view.record_details
                .contains(&field("Category recorded", "Laboratory"))
        );
        let shown = serde_json::to_string(&(&view.sections, &view.record_details)).unwrap();
        assert!(!shown.contains("2823-3"), "{shown}");
        assert!(!shown.contains("Technical details"), "{shown}");
        assert_eq!(view.technical.original, line);
    }

    #[test]
    fn a_panel_reads_each_component_and_a_narrative_reads_as_text() {
        let view = inspection(&citation(
            r#"{"resourceType":"Observation","code":{"text":"Blood Pressure"},
            "component":[{"code":{"text":"Systolic Blood Pressure"},"valueQuantity":{"value":159,"unit":"mmHg"}},
                         {"code":{"coding":[{"display":"Diastolic blood pressure"}]},"valueQuantity":{"value":99,"unit":"mmHg"}}],
            "text":{"status":"generated","div":"<div xmlns=\"http://www.w3.org/1999/xhtml\"><p>BP 159/99 &amp; rising</p><p>Recheck in 2 weeks</p></div>"}}"#,
        ));
        assert_eq!(
            view.sections,
            vec![
                field("Statement in this record", "Blood Pressure"),
                field("Systolic Blood Pressure", "159 mmHg"),
                field("Diastolic blood pressure", "99 mmHg"),
                field(
                    "Narrative in this record",
                    "BP 159/99 & rising\nRecheck in 2 weeks"
                ),
            ]
        );
        let narrated = inspection(&citation(
            r#"{"resourceType":"Basic","text":{"div":"<div>Only a narrative</div>"}}"#,
        ));
        assert_eq!(
            narrated.sections,
            vec![field("Narrative in this record", "Only a narrative")]
        );
    }

    #[test]
    fn an_order_reads_as_its_medication_dosage_and_reason() {
        let view = inspection(&citation(
            r#"{"resourceType":"MedicationRequest","status":"active","intent":"order",
            "medicationCodeableConcept":{"text":"metformin hydrochloride 1000 mg oral tablet"},
            "dosageInstruction":[{"text":"Take 1 tablet by mouth twice daily with meals"}],
            "reasonCode":[{"text":"Type 2 diabetes"}],"requester":{"display":"Dr. Order"}}"#,
        ));
        assert_eq!(view.title, "metformin hydrochloride 1000 mg oral tablet");
        assert!(view.sections.contains(&field(
            "Dosage instruction",
            "Take 1 tablet by mouth twice daily with meals"
        )));
        assert!(
            view.sections
                .contains(&field("Reason recorded", "Type 2 diabetes"))
        );
        assert!(
            view.record_details
                .contains(&field("Intent recorded", "order"))
        );
        assert!(
            view.attribution
                .contains(&field("Requested by", "Dr. Order"))
        );
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

    /// An order with a structured dose and route but no dosage text keeps both readable.
    #[test]
    fn a_structured_dosage_without_text_stays_readable() {
        let view = inspection(&citation(
            r#"{"resourceType":"MedicationRequest","status":"active","intent":"order",
            "medicationCodeableConcept":{"text":"lisinopril 10 mg oral tablet"},
            "dosageInstruction":[{"route":{"coding":[{"display":"Oral route"}]},
                "doseAndRate":[{"doseQuantity":{"value":10,"unit":"mg"}}],
                "timing":{"repeat":{"frequency":1,"period":1,"periodUnit":"d"}}}]}"#,
        ));
        assert_eq!(view.title, "lisinopril 10 mg oral tablet");
        assert_eq!(
            view.sections[1..],
            [field(
                "dosageInstruction",
                "doseAndRate.doseQuantity: 10 mg\nroute: Oral route\n\
                 timing.repeat.frequency: 1\ntiming.repeat.period: 1\ntiming.repeat.periodUnit: d"
            )]
        );
    }

    /// A panel component's own range and interpretation read beside its value.
    #[test]
    fn a_component_reads_with_its_range_and_interpretation() {
        let view = inspection(&citation(
            r#"{"resourceType":"Observation","code":{"text":"Basic metabolic panel"},
            "component":[{"code":{"text":"Glucose"},"valueQuantity":{"value":108,"unit":"mg/dL"},
                "referenceRange":[{"low":{"value":70,"unit":"mg/dL"},"high":{"value":99,"unit":"mg/dL"}}],
                "interpretation":[{"coding":[{"code":"H","display":"High"}]}]}]}"#,
        ));
        assert_eq!(
            view.sections,
            vec![
                field("Statement in this record", "Basic metabolic panel"),
                field(
                    "Glucose",
                    "108 mg/dL\nReference range recorded: 70 mg/dL to 99 mg/dL\n\
                     Interpretation recorded: High"
                ),
            ]
        );
    }

    /// A field no layout knows still reads under the source's name for it, nested values
    /// included, and nothing a layout placed is repeated.
    #[test]
    fn a_field_no_layout_places_reads_under_its_source_name() {
        let view = inspection(&citation(
            r#"{"resourceType":"Procedure","id":"proc-1","meta":{"versionId":"3"},
            "code":{"text":"Colonoscopy"},"status":"completed",
            "bodySite":[{"coding":[{"display":"Colon"}]}],
            "outcome":{"text":"Two polyps removed"},
            "complication":[{"text":"None observed"},{"coding":[{"code":"X1"}]}],
            "focalDevice":[{"action":{"text":"Implanted"},"manipulated":{"display":"Clip"}}],
            "usedCode":[],"futureFlag":true,"emptyNote":"  ",
            "subject":{"reference":"Patient/MRN0001"},"location":{"reference":"Location/1","display":"Endoscopy suite"}}"#,
        ));
        assert_eq!(
            view.sections,
            vec![
                field("Statement in this record", "Colonoscopy"),
                field("bodySite", "Colon"),
                field("complication 1", "None observed"),
                field("complication 2", "Code X1"),
                field("focalDevice", "action: Implanted\nmanipulated: Clip"),
                field("futureFlag", "true"),
                field("location", "Endoscopy suite"),
                field("outcome", "Two polyps removed"),
            ]
        );
        assert_eq!(
            view.record_details,
            vec![field("Status recorded", "completed")]
        );
        let shown = serde_json::to_string(&view.sections).unwrap();
        for technical in ["proc-1", "versionId", "MRN0001", "Location/1"] {
            assert!(!shown.contains(technical), "{technical} shows: {shown}");
        }
    }

    /// A retained date is shown once, as a date, and an attachment's encoded bytes stay technical.
    #[test]
    fn dates_and_encoded_attachments_are_not_repeated_as_fields() {
        let mut cited = citation(
            r#"{"resourceType":"DocumentReference","date":"2026-08-12","period":{"start":"2026-08-01","end":"2026-08-02"},
            "content":[{"attachment":{"contentType":"text/plain","data":"U2Vlbg=="}}]}"#,
        );
        cited.dates = ["date", "period.start", "period.end"]
            .map(|name| local_product::physician_sources::RecordDate {
                name: name.into(),
                value: "x".into(),
            })
            .into();
        let view = inspection(&cited);
        assert_eq!(
            view.sections,
            vec![field(
                "content",
                "attachment.contentType: text/plain\n\
                 attachment.data: encoded content, retained under Technical details"
            )]
        );
    }

    #[test]
    fn a_narrative_table_keeps_its_cells_and_superscripts() {
        let view = inspection(&citation(
            r#"{"resourceType":"Basic","text":{"div":"<div xmlns=\"http://www.w3.org/1999/xhtml\"><table><thead><tr><th>Test</th><th>Value</th><th>Range</th></tr></thead><tbody><tr><td>Glucose</td><td>108</td><td>70-99</td></tr><tr><td>Flag</td><td></td><td>H</td></tr></tbody></table><p>WBC 4.5 x 10<sup>9</sup>/L, area 2 m<sup>2</sup> &#215; 1</p></div>"}}"#,
        ));
        assert_eq!(
            view.sections,
            vec![field(
                "Narrative in this record",
                "Test | Value | Range\nGlucose | 108 | 70-99\nFlag | | H\n\
                 WBC 4.5 x 10^9/L, area 2 m^2 × 1"
            )]
        );
    }
}
