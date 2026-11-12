//! Semantic comparison of generated layouts against source records for label meaning and section
//! grouping, separate from mechanical `Layout::validate_sample` checks.
//!
//! Mechanical validation verifies JSON schema shape, RFC 6901 pointer resolution to scalar leaves,
//! verbatim value copy, no duplicate bindings, and structural role rules (`referenceRange` in FHIR,
//! quantity `unit`/`code`, FHIR dates, and status suffixes). It cannot know every vendor's field
//! names or cross-element pairing intent — for example, a non-FHIR vendor record's
//! `reference_range` bound as `SlotRole::Result` resolves a valid pointer and copies the exact
//! string `"3.5-5.1"`, yet mislabels a reference range as the patient's result.

use std::collections::BTreeSet;
use std::fmt::Write as _;
use std::fs;
use std::path::PathBuf;

use local_product::physician_chat::Citation;
use physician_sample::source_inspection::inspection_with_layout;
use physician_sample::source_layout::{
    FixedHeading, Heading, Layout, Loggable, Presence, Slot, SlotRole,
};
use serde::Serialize;
use serde_json::{Value, json};

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(tag = "status", rename_all = "snake_case")]
enum MechanicalOutcome {
    Accepted,
    Refused { category: String, summary: String },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
enum PresentationErrorKind {
    RangeMislabeledAsResult,
    CodeOrIdentifierMislabeledAsResult,
    StatusOrFlagMislabeledAsResult,
    MeasurementMislabeledAsCode,
    CrossElementGrouping,
    HeadingMeaningMismatch,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
struct PresentationError {
    kind: PresentationErrorKind,
    section_index: usize,
    section_heading: String,
    pointer: String,
    pointer_valid: bool,
    verbatim_value: Option<String>,
    assigned_role: String,
    rendered_label: String,
    explanation: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
struct SlotComparison {
    section_index: usize,
    section_heading: String,
    pointer: String,
    pointer_valid: bool,
    verbatim_value: Option<String>,
    assigned_role: String,
    rendered_label: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
struct SampleComparison {
    name: String,
    record_type: String,
    mechanical: MechanicalOutcome,
    all_pointers_valid: bool,
    all_values_verbatim: bool,
    slots: Vec<SlotComparison>,
    rendered_sections: Vec<RenderedSectionSummary>,
    presentation_errors: Vec<PresentationError>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
struct RenderedSectionSummary {
    heading: String,
    fields: Vec<(String, String, Option<String>)>,
}

fn scalar_text(value: &Value) -> Option<String> {
    match value {
        Value::String(s) => Some(s.clone()),
        Value::Number(n) => Some(n.to_string()),
        Value::Bool(b) => Some(b.to_string()),
        Value::Null => Some("null".to_owned()),
        Value::Array(_) | Value::Object(_) => None,
    }
}

fn pointer_members(pointer: &str) -> Vec<String> {
    pointer
        .split('/')
        .skip(1)
        .map(|token| token.replace("~1", "/").replace("~0", "~"))
        .filter(|token| token.parse::<usize>().is_err())
        .collect()
}

/// Array element coordinate `(array_prefix, element_index)` along an absolute RFC 6901 pointer,
/// such as `("/component", 0)` for `/component/0/valueQuantity/value`.
fn array_coordinates(record: &Value, pointer: &str) -> Vec<(String, usize)> {
    let mut coords = Vec::new();
    let mut current = record;
    let mut prefix = String::new();
    for raw in pointer.split('/').skip(1) {
        let token = raw.replace("~1", "/").replace("~0", "~");
        match current {
            Value::Object(map) => {
                let Some(next) = map.get(&token) else {
                    break;
                };
                prefix.push('/');
                prefix.push_str(raw);
                current = next;
            }
            Value::Array(items) => {
                let Ok(index) = token.parse::<usize>() else {
                    break;
                };
                let Some(next) = items.get(index) else {
                    break;
                };
                coords.push((prefix.clone(), index));
                prefix.push('/');
                prefix.push_str(raw);
                current = next;
            }
            _ => break,
        }
    }
    coords
}

fn is_range_member(name: &str) -> bool {
    let lower = name.to_ascii_lowercase();
    lower.contains("referencerange")
        || lower.contains("reference_range")
        || lower.contains("ref_range")
        || lower.contains("normal_range")
}

fn is_code_or_id_member(name: &str) -> bool {
    matches!(
        name,
        "code" | "coding" | "system" | "id" | "identifier" | "test_code" | "loinc" | "rxnorm"
    )
}

fn is_status_or_flag_member(name: &str) -> bool {
    let lower = name.to_ascii_lowercase();
    lower.ends_with("status") || lower.contains("flag")
}

fn is_measurement_member(name: &str) -> bool {
    let lower = name.to_ascii_lowercase();
    lower.starts_with("value") || is_range_member(name)
}

fn display_label_for(role: SlotRole, pointer: &str) -> String {
    let members = pointer_members(pointer);
    let last = members.last().map(String::as_str);
    if let Some(fixed) = role.fixed_label(last) {
        return fixed.to_owned();
    }
    last.unwrap_or("Field").to_owned()
}

fn section_heading_text(heading: Heading) -> String {
    match heading {
        Heading::Fixed(fixed) => fixed.text().to_owned(),
        Heading::FromCode(from) => format!("from_code({})", from.from_code.text()),
    }
}

fn check_slot_meaning(
    section_index: usize,
    section_heading: &str,
    slot: &Slot,
    absolute_pointer: &str,
    record: &Value,
    slots_out: &mut Vec<SlotComparison>,
    errors_out: &mut Vec<PresentationError>,
) {
    let resolved = record.pointer(absolute_pointer);
    let verbatim = resolved.and_then(scalar_text);
    let pointer_valid = verbatim.is_some();
    let rendered_label = display_label_for(slot.role, absolute_pointer);

    slots_out.push(SlotComparison {
        section_index,
        section_heading: section_heading.to_owned(),
        pointer: absolute_pointer.to_owned(),
        pointer_valid,
        verbatim_value: verbatim.clone(),
        assigned_role: slot.role.name().to_owned(),
        rendered_label: rendered_label.clone(),
    });

    if !pointer_valid {
        return;
    }

    let members = pointer_members(absolute_pointer);
    let last = members.last().map(String::as_str).unwrap_or_default();

    if slot.role == SlotRole::Result && members.iter().any(|m| is_range_member(m)) {
        errors_out.push(PresentationError {
            kind: PresentationErrorKind::RangeMislabeledAsResult,
            section_index,
            section_heading: section_heading.to_owned(),
            pointer: absolute_pointer.to_owned(),
            pointer_valid,
            verbatim_value: verbatim,
            assigned_role: slot.role.name().to_owned(),
            rendered_label,
            explanation: format!(
                "Pointer {absolute_pointer} resolves inside a reference-range field, yet is presented under label 'Result'."
            ),
        });
    } else if slot.role == SlotRole::Result
        && (is_code_or_id_member(last)
            || members.iter().any(|m| m == "coding" || m == "identifier"))
    {
        errors_out.push(PresentationError {
            kind: PresentationErrorKind::CodeOrIdentifierMislabeledAsResult,
            section_index,
            section_heading: section_heading.to_owned(),
            pointer: absolute_pointer.to_owned(),
            pointer_valid,
            verbatim_value: verbatim,
            assigned_role: slot.role.name().to_owned(),
            rendered_label,
            explanation: format!(
                "Pointer {absolute_pointer} names a code, code-system, or identifier leaf, yet is presented under label 'Result'."
            ),
        });
    } else if slot.role == SlotRole::Result && is_status_or_flag_member(last) {
        errors_out.push(PresentationError {
            kind: PresentationErrorKind::StatusOrFlagMislabeledAsResult,
            section_index,
            section_heading: section_heading.to_owned(),
            pointer: absolute_pointer.to_owned(),
            pointer_valid,
            verbatim_value: verbatim,
            assigned_role: slot.role.name().to_owned(),
            rendered_label,
            explanation: format!(
                "Pointer {absolute_pointer} names a status or abnormal flag, yet is presented under label 'Result'."
            ),
        });
    } else if slot.role == SlotRole::Code
        && members.iter().any(|m| is_measurement_member(m))
        && !members.iter().any(|m| m == "code" || m == "coding")
    {
        errors_out.push(PresentationError {
            kind: PresentationErrorKind::MeasurementMislabeledAsCode,
            section_index,
            section_heading: section_heading.to_owned(),
            pointer: absolute_pointer.to_owned(),
            pointer_valid,
            verbatim_value: verbatim,
            assigned_role: slot.role.name().to_owned(),
            rendered_label,
            explanation: format!(
                "Pointer {absolute_pointer} names a measurement or reference-range quantity, yet is bound in the 'code' role."
            ),
        });
    }
}

fn compare_layout(
    name: &str,
    record_type: &str,
    content: &str,
    layout: &Layout,
) -> SampleComparison {
    let record: Value = serde_json::from_str(content).expect("valid source JSON");

    let mechanical = match layout.validate_sample(record_type, &record) {
        Ok(_) => MechanicalOutcome::Accepted,
        Err(refusal) => MechanicalOutcome::Refused {
            category: refusal.cause.category().to_owned(),
            summary: Loggable(&refusal).to_string(),
        },
    };

    let citation = Citation {
        id: format!("cmp-{name}"),
        source_id: None,
        reference: format!("{record_type}/{name}"),
        revision: "rev-1".into(),
        record_type: record_type.to_owned(),
        content: content.to_owned(),
        dates: Vec::new(),
        sha256: local_product::sha256_hex(content.as_bytes()),
        readable_parts: Vec::new(),
        provenance: json!({"native_record": format!("{record_type}/{name}")}),
        native: None,
        summary: None,
        locator: None,
    };
    let view = inspection_with_layout(&citation, Some(layout));
    let rendered_sections = view
        .layout_sections
        .iter()
        .map(|s| RenderedSectionSummary {
            heading: s.heading.clone(),
            fields: s
                .fields
                .iter()
                .map(|f| (f.label.clone(), f.text.clone(), f.path.clone()))
                .collect(),
        })
        .collect();

    let mut slots = Vec::new();
    let mut errors = Vec::new();

    for (section_index, section) in layout.sections.iter().enumerate() {
        let heading_label = section_heading_text(section.heading);
        match &section.repeat {
            Some(repeat) => {
                if let Some(Value::Array(elements)) = record.pointer(repeat) {
                    for (elem_idx, _) in elements.iter().enumerate() {
                        let elem_heading = format!("{heading_label}[{elem_idx}]");
                        for slot in &section.slots {
                            let abs = format!("{repeat}/{elem_idx}{}", slot.pointer);
                            let any_element_has_it = elements
                                .iter()
                                .any(|el| el.pointer(&slot.pointer).is_some());
                            if record.pointer(&abs).is_none()
                                && slot.role.presence() == Presence::Optional
                                && any_element_has_it
                            {
                                continue;
                            }
                            check_slot_meaning(
                                section_index,
                                &elem_heading,
                                slot,
                                &abs,
                                &record,
                                &mut slots,
                                &mut errors,
                            );
                        }
                    }
                }
            }
            None => {
                let mut array_indices_by_prefix: std::collections::BTreeMap<
                    String,
                    BTreeSet<usize>,
                > = std::collections::BTreeMap::new();
                for slot in &section.slots {
                    check_slot_meaning(
                        section_index,
                        &heading_label,
                        slot,
                        &slot.pointer,
                        &record,
                        &mut slots,
                        &mut errors,
                    );
                    for (prefix, idx) in array_coordinates(&record, &slot.pointer) {
                        array_indices_by_prefix
                            .entry(prefix)
                            .or_default()
                            .insert(idx);
                    }
                }
                for (prefix, indices) in array_indices_by_prefix {
                    if indices.len() > 1 {
                        let joined: Vec<String> = indices.iter().map(usize::to_string).collect();
                        errors.push(PresentationError {
                            kind: PresentationErrorKind::CrossElementGrouping,
                            section_index,
                            section_heading: heading_label.clone(),
                            pointer: prefix.clone(),
                            pointer_valid: true,
                            verbatim_value: None,
                            assigned_role: "section".into(),
                            rendered_label: heading_label.clone(),
                            explanation: format!(
                                "Non-repeating section '{heading_label}' pairs fields across distinct array elements at {prefix} (indices {}).",
                                joined.join(", ")
                            ),
                        });
                    }
                }

                if let Heading::Fixed(FixedHeading::ReferenceRange) = section.heading {
                    let has_any_range = section.slots.iter().any(|s| {
                        matches!(
                            s.role,
                            SlotRole::ReferenceRangeLow
                                | SlotRole::ReferenceRangeHigh
                                | SlotRole::ReferenceRangeText
                        ) || pointer_members(&s.pointer)
                            .iter()
                            .any(|m| is_range_member(m))
                    });
                    if !section.slots.is_empty() && !has_any_range {
                        let first_ptr = &section.slots[0].pointer;
                        let verbatim = record.pointer(first_ptr).and_then(scalar_text);
                        errors.push(PresentationError {
                            kind: PresentationErrorKind::HeadingMeaningMismatch,
                            section_index,
                            section_heading: heading_label.clone(),
                            pointer: first_ptr.clone(),
                            pointer_valid: verbatim.is_some(),
                            verbatim_value: verbatim,
                            assigned_role: section.slots[0].role.name().to_owned(),
                            rendered_label: heading_label,
                            explanation: "Section is headed 'Reference range' yet contains no reference-range fields.".into(),
                        });
                    }
                }
            }
        }
    }

    let all_pointers_valid = !slots.is_empty() && slots.iter().all(|s| s.pointer_valid);
    let all_values_verbatim = !slots.is_empty() && slots.iter().all(|s| s.verbatim_value.is_some());

    SampleComparison {
        name: name.to_owned(),
        record_type: record_type.to_owned(),
        mechanical,
        all_pointers_valid,
        all_values_verbatim,
        slots,
        rendered_sections,
        presentation_errors: errors,
    }
}

fn render_markdown_report(comparisons: &[SampleComparison]) -> String {
    let mut md = String::new();
    writeln!(
        &mut md,
        "# ENG-353 Layout-vs-Source Semantic Presentation Comparisons\n"
    )
    .unwrap();
    writeln!(
        &mut md,
        "| Sample | Record type | Mechanical validation | Pointers & values verbatim | Presentation errors |"
    )
    .unwrap();
    writeln!(&mut md, "| --- | --- | --- | --- | --- |").unwrap();
    for c in comparisons {
        let mech = match &c.mechanical {
            MechanicalOutcome::Accepted => "Accepted".to_owned(),
            MechanicalOutcome::Refused { category, .. } => format!("Refused (`{category}`)"),
        };
        let verbatim = if c.all_pointers_valid && c.all_values_verbatim {
            "Yes"
        } else {
            "No"
        };
        let errs = if c.presentation_errors.is_empty() {
            "None".to_owned()
        } else {
            c.presentation_errors
                .iter()
                .map(|e| format!("`{:?}` at `{}`", e.kind, e.pointer))
                .collect::<Vec<_>>()
                .join("; ")
        };
        writeln!(
            &mut md,
            "| `{}` | `{}` | {} | {} | {} |",
            c.name, c.record_type, mech, verbatim, errs
        )
        .unwrap();
    }
    writeln!(&mut md, "\n## Reported Presentation Errors\n").unwrap();
    for c in comparisons
        .iter()
        .filter(|c| !c.presentation_errors.is_empty())
    {
        writeln!(&mut md, "### `{}` (`{}`)", c.name, c.record_type).unwrap();
        writeln!(&mut md, "- Mechanical check: `{:?}`", c.mechanical).unwrap();
        for err in &c.presentation_errors {
            writeln!(
                &mut md,
                "- **`{:?}`** — pointer `{}` (valid: `{}`, verbatim value: `{:?}`), role `{}`, rendered label `{}`: {}",
                err.kind,
                err.pointer,
                err.pointer_valid,
                err.verbatim_value,
                err.assigned_role,
                err.rendered_label,
                err.explanation
            )
            .unwrap();
        }
        writeln!(&mut md).unwrap();
    }
    md
}

const BANK_POTASSIUM: &str = r#"{"resourceType":"Observation","id":"obs-5c1d2e7a9b40","meta":{"versionId":"1","lastUpdated":"2026-08-20T00:00:00+00:00"},"status":"final","category":[{"coding":[{"system":"http://terminology.hl7.org/CodeSystem/observation-category","code":"laboratory","display":"Laboratory"}]}],"code":{"coding":[{"system":"http://loinc.org","code":"2823-3","display":"Potassium [Moles/volume] in Serum or Plasma"}],"text":"Potassium"},"subject":{"reference":"Patient/MRN00001776"},"effectiveDateTime":"2026-08-20T15:15:00+00:00","issued":"2026-08-21T17:02:00+00:00","valueQuantity":{"value":5.9,"unit":"mmol/L","system":"http://unitsofmeasure.org","code":"mmol/L"},"interpretation":[{"coding":[{"system":"http://terminology.hl7.org/CodeSystem/v3-ObservationInterpretation","code":"H","display":"High"}]}],"referenceRange":[{"low":{"value":3.5,"unit":"mmol/L","system":"http://unitsofmeasure.org","code":"mmol/L"},"high":{"value":5.1,"unit":"mmol/L","system":"http://unitsofmeasure.org","code":"mmol/L"}}]}"#;

const BANK_BLOOD_PRESSURE: &str = r#"{"resourceType":"Observation","id":"obs-314a3cecf88a","status":"final","code":{"coding":[{"system":"http://loinc.org","code":"85354-9","display":"Blood pressure panel with all children optional"}],"text":"Blood Pressure"},"subject":{"reference":"Patient/MRN00001101"},"effectiveDateTime":"2019-03-12T13:30:00+00:00","component":[{"code":{"coding":[{"system":"http://loinc.org","code":"8480-6","display":"Systolic blood pressure"}],"text":"Systolic Blood Pressure"},"valueQuantity":{"value":159,"unit":"mmHg","system":"http://unitsofmeasure.org","code":"mm[Hg]"}},{"code":{"coding":[{"system":"http://loinc.org","code":"8462-4","display":"Diastolic blood pressure"}],"text":"Diastolic Blood Pressure"},"valueQuantity":{"value":99,"unit":"mmHg","system":"http://unitsofmeasure.org","code":"mm[Hg]"}}]}"#;

const BANK_CONDITION: &str = r#"{"resourceType":"Condition","id":"cond-db76cb24b2c2","clinicalStatus":{"coding":[{"system":"http://terminology.hl7.org/CodeSystem/condition-clinical","code":"resolved","display":"Resolved"}]},"verificationStatus":{"coding":[{"system":"http://terminology.hl7.org/CodeSystem/condition-ver-status","code":"confirmed","display":"Confirmed"}]},"code":{"coding":[{"system":"http://hl7.org/fhir/sid/icd-10-cm","code":"J20.9","display":"Acute bronchitis, unspecified"}],"text":"Acute bronchitis"},"onsetDateTime":"2019-03-05","recordedDate":"2019-03-12T13:20:00+00:00","abatementDateTime":"2019-04-02T19:00:00+00:00"}"#;

const BANK_MEDICATION: &str = r#"{"resourceType":"MedicationRequest","id":"medrx-24ab1968c855","status":"active","intent":"order","medicationCodeableConcept":{"coding":[{"system":"http://www.nlm.nih.gov/research/umls/rxnorm","code":"861004","display":"metformin hydrochloride 1000 MG Oral Tablet"}],"text":"metformin hydrochloride 1000 mg oral tablet"},"authoredOn":"2019-03-12T13:20:00+00:00","dosageInstruction":[{"text":"Take 1 tablet by mouth twice daily with meals","doseAndRate":[{"doseQuantity":{"value":1000,"unit":"mg"}}]}]}"#;

#[test]
fn semantic_comparison_evaluates_label_meaning_and_grouping_separately_from_mechanical_checks() {
    let vendor_lab = include_str!("../fixtures/records/vendor-lab-result.json");

    let clean_observation = Layout::parse(
        r#"{
            "record_type": "Observation",
            "statuses": [{"role": "status", "pointer": "/status"}],
            "sections": [
                {
                    "heading": "result",
                    "slots": [
                        {"role": "code", "pointer": "/code/text"},
                        {"role": "result", "pointer": "/valueQuantity/value"},
                        {"role": "unit", "pointer": "/valueQuantity/unit"},
                        {"role": "interpretation", "pointer": "/interpretation/0/coding/0/display"}
                    ]
                },
                {
                    "heading": "reference_range",
                    "slots": [
                        {"role": "reference_range_low", "pointer": "/referenceRange/0/low/value"},
                        {"role": "reference_range_high", "pointer": "/referenceRange/0/high/value"},
                        {"role": "unit", "pointer": "/referenceRange/0/low/unit"}
                    ]
                }
            ]
        }"#,
    )
    .unwrap();

    let clean_bp = Layout::parse(
        r#"{
            "record_type": "Observation",
            "statuses": [{"role": "status", "pointer": "/status"}],
            "sections": [
                {
                    "heading": {"from_code": "component"},
                    "repeat": "/component",
                    "slots": [
                        {"role": "result", "pointer": "/valueQuantity/value"},
                        {"role": "unit", "pointer": "/valueQuantity/unit"}
                    ]
                }
            ]
        }"#,
    )
    .unwrap();

    let clean_condition = Layout::parse(
        r#"{
            "record_type": "Condition",
            "statuses": [
                {"role": "status", "pointer": "/clinicalStatus/coding/0/display"},
                {"role": "status", "pointer": "/verificationStatus/coding/0/display"}
            ],
            "sections": [
                {
                    "heading": "problem",
                    "slots": [
                        {"role": "code", "pointer": "/code/text"},
                        {"role": "code", "pointer": "/code/coding/0/code"}
                    ]
                },
                {
                    "heading": "dates",
                    "slots": [
                        {"role": "date", "pointer": "/onsetDateTime"},
                        {"role": "date", "pointer": "/abatementDateTime"}
                    ]
                }
            ]
        }"#,
    )
    .unwrap();

    let clean_medication = Layout::parse(
        r#"{
            "record_type": "MedicationRequest",
            "statuses": [{"role": "status", "pointer": "/status"}],
            "sections": [
                {
                    "heading": "medication",
                    "slots": [
                        {"role": "code", "pointer": "/medicationCodeableConcept/text"}
                    ]
                },
                {
                    "heading": "dosage",
                    "repeat": "/dosageInstruction",
                    "slots": [
                        {"role": "other", "pointer": "/text"},
                        {"role": "result", "pointer": "/doseAndRate/0/doseQuantity/value"},
                        {"role": "unit", "pointer": "/doseAndRate/0/doseQuantity/unit"}
                    ]
                }
            ]
        }"#,
    )
    .unwrap();

    let clean_vendor = Layout::parse(
        r#"{
            "record_type": "LabResult",
            "statuses": [{"role": "status", "pointer": "/status"}],
            "sections": [
                {
                    "heading": "results",
                    "repeat": "/results",
                    "slots": [
                        {"role": "code", "pointer": "/test_name"},
                        {"role": "result", "pointer": "/value"},
                        {"role": "other", "pointer": "/units"},
                        {"role": "other", "pointer": "/reference_range"},
                        {"role": "other", "pointer": "/abnormal_flag"}
                    ]
                }
            ]
        }"#,
    )
    .unwrap();

    // Planted presentation error 1: non-FHIR vendor reference_range bound as SlotRole::Result.
    // Every pointer is valid, every value is copied verbatim, and mechanical validation ACCEPTS it,
    // yet the reader labels the reference range ("3.5-5.1") as "Result".
    let vendor_range_as_result = Layout::parse(
        r#"{
            "record_type": "LabResult",
            "sections": [
                {
                    "heading": "results",
                    "repeat": "/results",
                    "slots": [
                        {"role": "code", "pointer": "/test_name"},
                        {"role": "result", "pointer": "/reference_range"}
                    ]
                }
            ]
        }"#,
    )
    .unwrap();
    // Use a single-row vendor sample where reference_range is present so validate_sample accepts it.
    let single_row_vendor = r#"{"lab_result_id":"88213","status":"final","results":[{"test_code":"K","test_name":"Potassium","value":5.9,"units":"mmol/L","reference_range":"3.5-5.1","abnormal_flag":"H"}]}"#;

    // Planted presentation error 2: FHIR Observation referenceRange low bound as SlotRole::Result.
    // Valid pointer (/referenceRange/0/low/value) and verbatim value (3.5); refused mechanically
    // AND independently flagged by the semantic comparator as RangeMislabeledAsResult.
    let fhir_range_as_result = Layout::parse(
        r#"{
            "record_type": "Observation",
            "sections": [
                {
                    "heading": "result",
                    "slots": [
                        {"role": "code", "pointer": "/code/text"},
                        {"role": "result", "pointer": "/referenceRange/0/low/value"}
                    ]
                }
            ]
        }"#,
    )
    .unwrap();

    // Planted presentation error 3: LOINC code ("2823-3") bound as SlotRole::Result.
    // Passes mechanical checks with valid pointer and verbatim value, yet mislabels a code as Result.
    let code_as_result = Layout::parse(
        r#"{
            "record_type": "Observation",
            "sections": [
                {
                    "heading": "result",
                    "slots": [
                        {"role": "result", "pointer": "/code/coding/0/code"}
                    ]
                }
            ]
        }"#,
    )
    .unwrap();

    // Planted presentation error 4: Cross-component grouping in Blood Pressure Observation
    // (systolic value paired with diastolic unit in one non-repeating section).
    // Passes mechanical checks with valid pointers and verbatim values, yet misgroups elements.
    let cross_component = Layout::parse(
        r#"{
            "record_type": "Observation",
            "sections": [
                {
                    "heading": "result",
                    "slots": [
                        {"role": "result", "pointer": "/component/0/valueQuantity/value"},
                        {"role": "unit", "pointer": "/component/1/valueQuantity/unit"}
                    ]
                }
            ]
        }"#,
    )
    .unwrap();

    // Planted presentation error 5: Primary result placed under "Reference range" heading.
    let wrong_heading = Layout::parse(
        r#"{
            "record_type": "Observation",
            "sections": [
                {
                    "heading": "reference_range",
                    "slots": [
                        {"role": "result", "pointer": "/valueQuantity/value"},
                        {"role": "unit", "pointer": "/valueQuantity/unit"}
                    ]
                }
            ]
        }"#,
    )
    .unwrap();

    let mut comparisons = vec![
        compare_layout(
            "clean_observation_potassium",
            "Observation",
            BANK_POTASSIUM,
            &clean_observation,
        ),
        compare_layout(
            "clean_observation_blood_pressure",
            "Observation",
            BANK_BLOOD_PRESSURE,
            &clean_bp,
        ),
        compare_layout(
            "clean_condition",
            "Condition",
            BANK_CONDITION,
            &clean_condition,
        ),
        compare_layout(
            "clean_medication_request",
            "MedicationRequest",
            BANK_MEDICATION,
            &clean_medication,
        ),
        compare_layout(
            "clean_vendor_lab_result",
            "LabResult",
            vendor_lab,
            &clean_vendor,
        ),
        compare_layout(
            "planted_vendor_range_mislabeled_as_result",
            "LabResult",
            single_row_vendor,
            &vendor_range_as_result,
        ),
        compare_layout(
            "planted_fhir_range_low_mislabeled_as_result",
            "Observation",
            BANK_POTASSIUM,
            &fhir_range_as_result,
        ),
        compare_layout(
            "planted_loinc_code_mislabeled_as_result",
            "Observation",
            BANK_POTASSIUM,
            &code_as_result,
        ),
        compare_layout(
            "planted_cross_component_grouping",
            "Observation",
            BANK_BLOOD_PRESSURE,
            &cross_component,
        ),
        compare_layout(
            "planted_heading_meaning_mismatch",
            "Observation",
            BANK_POTASSIUM,
            &wrong_heading,
        ),
    ];

    let workspace = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let live_evidence = workspace.join("target/verification/eng-353-live-model-layout.json");
    if let Ok(bytes) = fs::read(&live_evidence)
        && let Ok(doc) = serde_json::from_slice::<Value>(&bytes)
        && let (Some(record), Some(layout_val)) = (doc.get("record"), doc.get("layout"))
        && !layout_val.is_null()
    {
        let live_layout: Layout = serde_json::from_value(layout_val.clone()).unwrap();
        let live_cmp = compare_layout(
            "live_background_model_observation",
            "Observation",
            &record.to_string(),
            &live_layout,
        );
        assert_eq!(live_cmp.mechanical, MechanicalOutcome::Accepted);
        assert!(
            live_cmp.presentation_errors.is_empty(),
            "live model layout had presentation errors: {:?}",
            live_cmp.presentation_errors
        );
        comparisons.push(live_cmp);
    }

    // Clean samples pass both mechanical validation and semantic presentation checks.
    for clean in &comparisons[..5] {
        assert_eq!(
            clean.mechanical,
            MechanicalOutcome::Accepted,
            "{}",
            clean.name
        );
        assert!(clean.all_pointers_valid, "{}", clean.name);
        assert!(clean.all_values_verbatim, "{}", clean.name);
        assert!(
            clean.presentation_errors.is_empty(),
            "{}: {:?}",
            clean.name,
            clean.presentation_errors
        );
    }

    // Planted error 1: valid pointer + verbatim value + mechanical ACCEPTED, yet presentation error!
    let vendor_err = &comparisons[5];
    assert_eq!(vendor_err.mechanical, MechanicalOutcome::Accepted);
    assert!(vendor_err.all_pointers_valid);
    assert!(vendor_err.all_values_verbatim);
    assert_eq!(vendor_err.presentation_errors.len(), 1);
    assert_eq!(
        vendor_err.presentation_errors[0].kind,
        PresentationErrorKind::RangeMislabeledAsResult
    );
    assert_eq!(
        vendor_err.presentation_errors[0].verbatim_value.as_deref(),
        Some("3.5-5.1")
    );
    assert_eq!(vendor_err.presentation_errors[0].rendered_label, "Result");

    // Planted error 2: valid pointer + verbatim value on FHIR referenceRange/0/low/value ->
    // refused mechanically AND recorded as RangeMislabeledAsResult by the semantic comparator.
    let fhir_err = &comparisons[6];
    assert!(matches!(
        fhir_err.mechanical,
        MechanicalOutcome::Refused { ref category, .. } if category == "role_mismatch"
    ));
    assert!(fhir_err.all_pointers_valid);
    assert!(fhir_err.all_values_verbatim);
    assert_eq!(
        fhir_err.presentation_errors[0].kind,
        PresentationErrorKind::RangeMislabeledAsResult
    );
    assert_eq!(
        fhir_err.presentation_errors[0].verbatim_value.as_deref(),
        Some("3.5")
    );

    // Planted error 3: LOINC code bound as Result -> mechanical ACCEPTED, semantic error recorded.
    let code_err = &comparisons[7];
    assert_eq!(code_err.mechanical, MechanicalOutcome::Accepted);
    assert!(code_err.all_pointers_valid);
    assert_eq!(
        code_err.presentation_errors[0].kind,
        PresentationErrorKind::CodeOrIdentifierMislabeledAsResult
    );

    // Planted error 4: Cross-component grouping -> mechanical ACCEPTED, semantic error recorded.
    let group_err = &comparisons[8];
    assert_eq!(group_err.mechanical, MechanicalOutcome::Accepted);
    assert!(group_err.all_pointers_valid);
    assert_eq!(
        group_err.presentation_errors[0].kind,
        PresentationErrorKind::CrossElementGrouping
    );

    // Planted error 5: Heading meaning mismatch -> mechanical ACCEPTED, semantic error recorded.
    let heading_err = &comparisons[9];
    assert_eq!(heading_err.mechanical, MechanicalOutcome::Accepted);
    assert!(heading_err.all_pointers_valid);
    assert_eq!(
        heading_err.presentation_errors[0].kind,
        PresentationErrorKind::HeadingMeaningMismatch
    );

    let out_dir = workspace.join("target/verification");
    fs::create_dir_all(&out_dir).unwrap();
    fs::write(
        out_dir.join("eng-353-layout-comparisons.json"),
        serde_json::to_vec_pretty(&comparisons).unwrap(),
    )
    .unwrap();
    fs::write(
        out_dir.join("eng-353-layout-comparisons.md"),
        render_markdown_report(&comparisons),
    )
    .unwrap();
}
