//! How a physician reads a cited record: the source's own statement and dates first, the exact
//! retained bytes and storage details under Technical details. Nothing here infers clinical truth.
use local_product::physician_chat::Citation;
use physician_wire::{SourceField, SourceInspection, SourceTechnicalDetails};
use serde_json::Value;

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
    match resource {
        Some(resource) if resource["resourceType"] == "AllergyIntolerance" => allergy(&resource, &mut view),
        Some(_) if view.sections.is_empty() => view.sections.push(field(
            "Source content",
            "A readable presentation is not available for this format. The retained source is available under Technical details.",
        )),
        None if view.sections.is_empty() => view.sections.push(field("Source text", &citation.content)),
        _ => {}
    }
    view
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
        let fallback = inspection(&unknown);
        assert!(
            fallback.sections[0]
                .text
                .contains("retained source is available")
        );
        assert_eq!(fallback.technical.original, unknown.content);
        unknown
            .readable_parts
            .push(local_product::physician_sources::ReadablePart {
                label: "Document".into(),
                text: "Readable attachment".into(),
            });
        assert_eq!(
            inspection(&unknown).sections,
            vec![field("Document", "Readable attachment")]
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
}
