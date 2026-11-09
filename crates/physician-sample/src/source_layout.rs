//! A layout arranges records of one type into headed sections. It holds only headings, labels
//! and JSON pointers (RFC 6901) into the record, never a value, so everything it shows is copied
//! from the record. A layout is checked against each record before it renders; a refused layout
//! never shows in part, and the record reads through the general view instead.
use crate::source_inspection::{NumberLiterals, humanize, located};
use physician_wire::{SourceField, SourceInspection, SourceLayoutSection};
use serde::Deserialize;
use serde_json::Value;
use std::collections::BTreeSet;
use std::fmt;

/// An unchecked layout, in the shape a model or a hand-written fixture supplies. Unknown members
/// are refused when it is read, so a layout cannot carry a literal value beside its pointers.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Layout {
    /// The record type it reads, compared with the record's `resourceType`.
    pub record_type: String,
    /// The record's own statuses, read beside its kind and date.
    #[serde(default)]
    pub statuses: Vec<Slot>,
    /// Headed groups of fields, in reading order.
    pub sections: Vec<Section>,
}

/// A headed group of fields.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Section {
    /// The group's heading, written by the layout and never read as source text.
    pub heading: String,
    /// The group's fields, in reading order.
    pub slots: Vec<Slot>,
}

/// A label for the single value at `pointer`, such as `/valueQuantity/value`.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Slot {
    /// What the value means, written by the layout.
    pub label: String,
    /// Where the value is in the record.
    pub pointer: String,
}

/// Why a layout cannot read a record. A refused layout is never shown, not even in part.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LayoutRefusal {
    /// The record is not a JSON object.
    NotStructured,
    /// The record's `resourceType` is not the one the layout reads.
    WrongRecordType {
        /// The record type the layout reads.
        expected: String,
        /// The record's own type, if it names one.
        found: Option<String>,
    },
    /// A section's heading is empty or whitespace.
    EmptyHeading {
        /// The section's position in the layout.
        index: usize,
    },
    /// A slot's label is empty or whitespace.
    EmptyLabel {
        /// The slot's pointer as the layout wrote it.
        pointer: String,
    },
    /// A slot's pointer is not an RFC 6901 JSON pointer.
    MalformedPointer {
        /// The slot's pointer as the layout wrote it.
        pointer: String,
        /// How it departs from RFC 6901.
        reason: PointerError,
    },
    /// A slot's pointer names nothing in this record.
    Unresolved {
        /// The slot's pointer as the layout wrote it.
        pointer: String,
    },
    /// A slot shows one recorded value; an object or array would need the layout to compose it.
    NotALeaf {
        /// The slot's pointer as the layout wrote it.
        pointer: String,
    },
    /// Two slots bind the same recorded value.
    DuplicateBinding {
        /// The slot's pointer as the layout wrote it.
        pointer: String,
    },
}

impl fmt::Display for LayoutRefusal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotStructured => write!(f, "the record is not a structured object"),
            Self::WrongRecordType { expected, found } => {
                write!(f, "the layout reads {expected} records, not {found:?}")
            }
            Self::EmptyHeading { index } => write!(f, "section {index} has no heading"),
            Self::EmptyLabel { pointer } => write!(f, "the slot for {pointer} has no label"),
            Self::MalformedPointer { pointer, .. } => write!(f, "{pointer} is not a JSON pointer"),
            Self::Unresolved { pointer } => write!(f, "{pointer} does not resolve in this record"),
            Self::NotALeaf { pointer } => {
                write!(
                    f,
                    "{pointer} points at an object or array, not a single recorded value"
                )
            }
            Self::DuplicateBinding { pointer } => write!(f, "{pointer} is bound more than once"),
        }
    }
}

impl std::error::Error for LayoutRefusal {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::MalformedPointer { reason, .. } => Some(reason),
            _ => None,
        }
    }
}

/// How a pointer departs from RFC 6901.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PointerError {
    /// A non-empty pointer does not start with `/`.
    MissingLeadingSlash,
    /// `~` is followed by something other than `0` or `1`.
    BadEscape,
}

impl fmt::Display for PointerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingLeadingSlash => write!(f, "a non-empty pointer starts with '/'"),
            Self::BadEscape => write!(f, "'~' is followed by something other than '0' or '1'"),
        }
    }
}

impl std::error::Error for PointerError {}

/// A layout checked against one record: every slot names a single recorded value, once. It
/// borrows that record, so it can render no other.
#[derive(Debug)]
pub struct ValidatedLayout<'r> {
    record: &'r Value,
    statuses: Vec<Binding<'r>>,
    sections: Vec<BoundSection<'r>>,
}

#[derive(Debug)]
struct BoundSection<'r> {
    heading: String,
    bindings: Vec<Binding<'r>>,
}

/// A label bound to the recorded value at `path`, in the inspection's path notation.
#[derive(Debug)]
struct Binding<'r> {
    label: String,
    path: String,
    value: &'r Value,
}

impl Layout {
    /// Checks every heading, label and pointer against `record`, refusing at the first fault.
    pub fn validate<'r>(&self, record: &'r Value) -> Result<ValidatedLayout<'r>, LayoutRefusal> {
        if !record.is_object() {
            return Err(LayoutRefusal::NotStructured);
        }
        let found = record["resourceType"].as_str();
        if found != Some(self.record_type.as_str()) {
            return Err(LayoutRefusal::WrongRecordType {
                expected: self.record_type.clone(),
                found: found.map(str::to_owned),
            });
        }
        let mut bound = BTreeSet::new();
        let mut bind = |slot: &Slot| bind(slot, record, &mut bound);
        let statuses = self
            .statuses
            .iter()
            .map(&mut bind)
            .collect::<Result<_, _>>()?;
        let mut sections = Vec::with_capacity(self.sections.len());
        for (index, section) in self.sections.iter().enumerate() {
            if section.heading.trim().is_empty() {
                return Err(LayoutRefusal::EmptyHeading { index });
            }
            sections.push(BoundSection {
                heading: section.heading.clone(),
                bindings: section
                    .slots
                    .iter()
                    .map(&mut bind)
                    .collect::<Result<_, _>>()?,
            });
        }
        Ok(ValidatedLayout {
            record,
            statuses,
            sections,
        })
    }
}

fn bind<'r>(
    slot: &Slot,
    record: &'r Value,
    bound: &mut BTreeSet<String>,
) -> Result<Binding<'r>, LayoutRefusal> {
    let pointer = || slot.pointer.clone();
    if slot.label.trim().is_empty() {
        return Err(LayoutRefusal::EmptyLabel { pointer: pointer() });
    }
    let tokens = tokens(&slot.pointer).map_err(|reason| LayoutRefusal::MalformedPointer {
        pointer: pointer(),
        reason,
    })?;
    let mut value = record;
    let mut path = String::new();
    for token in tokens {
        value = match value {
            Value::Object(object) => {
                let member = object
                    .get(&token)
                    .ok_or_else(|| LayoutRefusal::Unresolved { pointer: pointer() })?;
                if !path.is_empty() {
                    path.push('.');
                }
                path.push_str(&token);
                member
            }
            Value::Array(items) => {
                let item = index(&token)
                    .and_then(|index| items.get(index).map(|item| (index, item)))
                    .ok_or_else(|| LayoutRefusal::Unresolved { pointer: pointer() });
                let (index, item) = item?;
                path.push_str(&format!("[{index}]"));
                item
            }
            _ => return Err(LayoutRefusal::Unresolved { pointer: pointer() }),
        };
    }
    if value.is_object() || value.is_array() {
        return Err(LayoutRefusal::NotALeaf { pointer: pointer() });
    }
    // Two spellings of one pointer bind the same value, so duplicates compare by path.
    if !bound.insert(path.clone()) {
        return Err(LayoutRefusal::DuplicateBinding { pointer: pointer() });
    }
    Ok(Binding {
        label: slot.label.clone(),
        path,
        value,
    })
}

/// The unescaped reference tokens of an RFC 6901 pointer; `""` is the whole record.
fn tokens(pointer: &str) -> Result<Vec<String>, PointerError> {
    if pointer.is_empty() {
        return Ok(Vec::new());
    }
    let rest = pointer
        .strip_prefix('/')
        .ok_or(PointerError::MissingLeadingSlash)?;
    rest.split('/')
        .map(|token| {
            let mut unescaped = String::with_capacity(token.len());
            let mut characters = token.chars();
            while let Some(character) = characters.next() {
                unescaped.push(match (character, character == '~') {
                    (_, false) => character,
                    (_, true) => match characters.next() {
                        Some('0') => '~',
                        Some('1') => '/',
                        _ => return Err(PointerError::BadEscape),
                    },
                });
            }
            Ok(unescaped)
        })
        .collect()
}

/// An RFC 6901 array index: `0` or digits without a leading zero.
fn index(token: &str) -> Option<usize> {
    let digits = !token.is_empty() && token.bytes().all(|byte| byte.is_ascii_digit());
    let canonical = token == "0" || !token.starts_with('0');
    (digits && canonical).then(|| token.parse().ok()).flatten()
}

/// Rearranges the record's general view under the layout. Bound values read under the layout's
/// labels with their source paths, and bound statuses lead the statuses. A general field the
/// layout binds part of gives way to its unbound leaves, each under the general label extended by
/// its own path; leaves already under Technical details stay only there. Every other general field
/// reads under Other fields. Dates, attribution, readable source text and Technical details keep
/// their general reading, so nothing in the record is dropped.
pub(crate) fn render(
    layout: &ValidatedLayout<'_>,
    numbers: &NumberLiterals,
    view: &mut SourceInspection,
) {
    let shown = |binding: &Binding| {
        located(
            &binding.label,
            &numbers.recorded(binding.value, &binding.path),
            &binding.path,
        )
    };
    let bound: BTreeSet<&str> = layout
        .statuses
        .iter()
        .chain(layout.sections.iter().flat_map(|section| &section.bindings))
        .map(|binding| binding.path.as_str())
        .collect();
    let technical: BTreeSet<String> = view
        .technical
        .fields
        .iter()
        .filter_map(|field| field.path.clone())
        .collect();
    let mut leaves = Vec::new();
    leaf_values(layout.record, String::new(), &mut leaves);
    // What remains of a general field once the layout has placed its bound leaves.
    let remaining = |field: SourceField| -> Vec<SourceField> {
        let Some(path) = field.path.as_deref() else {
            return vec![field];
        };
        let under: Vec<&(String, &Value)> = leaves
            .iter()
            .filter(|(leaf, _)| is_under(leaf, path))
            .collect();
        if !under.iter().any(|(leaf, _)| bound.contains(leaf.as_str())) {
            return vec![field];
        }
        under
            .into_iter()
            .filter(|(leaf, _)| !bound.contains(leaf.as_str()) && !technical.contains(leaf))
            .map(|(leaf, value)| {
                let label = match leaf_words(&leaf[path.len()..]) {
                    words if words.is_empty() => field.label.clone(),
                    words => format!("{} {words}", field.label),
                };
                located(&label, &numbers.recorded(value, leaf), leaf)
            })
            .collect()
    };
    let general_statuses = std::mem::take(&mut view.statuses);
    view.statuses = layout.statuses.iter().map(shown).collect();
    view.statuses
        .extend(general_statuses.into_iter().flat_map(&remaining));
    let (readable_text, fields): (Vec<_>, Vec<_>) = std::mem::take(&mut view.sections)
        .into_iter()
        .partition(|field| field.path.is_none());
    view.sections = readable_text;
    view.other_fields = fields
        .into_iter()
        .chain(std::mem::take(&mut view.record_details))
        .flat_map(&remaining)
        .collect();
    view.layout_sections = layout
        .sections
        .iter()
        .map(|section| SourceLayoutSection {
            heading: section.heading.clone(),
            fields: section.bindings.iter().map(shown).collect(),
        })
        .collect();
}

/// Whether the leaf at `leaf` lies at or under `path` in the inspection's path notation.
fn is_under(leaf: &str, path: &str) -> bool {
    leaf.strip_prefix(path)
        .is_some_and(|rest| rest.is_empty() || rest.starts_with(['.', '[']))
}

/// A relative path such as `.coding[0].system` in words: "coding 1 system".
fn leaf_words(rest: &str) -> String {
    rest.split(['.', '['])
        .filter(|step| !step.is_empty())
        .map(
            |step| match step.strip_suffix(']').map(str::parse::<usize>) {
                Some(Ok(index)) => (index + 1).to_string(),
                _ => humanize(step).to_lowercase(),
            },
        )
        .collect::<Vec<_>>()
        .join(" ")
}

/// Every scalar leaf under `value`, with its path.
fn leaf_values<'r>(value: &'r Value, path: String, out: &mut Vec<(String, &'r Value)>) {
    match value {
        Value::Array(items) => {
            for (index, item) in items.iter().enumerate() {
                leaf_values(item, format!("{path}[{index}]"), out);
            }
        }
        Value::Object(object) => {
            for (key, member) in object {
                let path = if path.is_empty() {
                    key.clone()
                } else {
                    format!("{path}.{key}")
                };
                leaf_values(member, path, out);
            }
        }
        _ => out.push((path, value)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::source_inspection::tests::{
        assert_view_shows_every_leaf, at, citation, condition, general_citation, observation,
        readable,
    };
    use crate::source_inspection::{inspection, inspection_with_layout};
    use local_product::physician_chat::Citation;
    use serde_json::json;

    fn layout(value: Value) -> Layout {
        serde_json::from_value(value).unwrap()
    }

    /// A hand-written Observation layout: the result, then its reference range.
    fn observation_layout() -> Layout {
        layout(json!({"record_type":"Observation",
            "statuses":[{"label":"Result status","pointer":"/status"}],
            "sections":[
                {"heading":"Result","slots":[
                    {"label":"Test","pointer":"/code/text"},
                    {"label":"Result","pointer":"/valueQuantity/value"},
                    {"label":"Unit","pointer":"/valueQuantity/unit"},
                    {"label":"Interpretation","pointer":"/interpretation/0/coding/0/display"}]},
                {"heading":"Reference range","slots":[
                    {"label":"Low","pointer":"/referenceRange/0/low/value"},
                    {"label":"High","pointer":"/referenceRange/0/high/value"},
                    {"label":"Range as recorded","pointer":"/referenceRange/0/text"}]}]}))
    }

    /// A hand-written Condition layout: the problem and its note, then its coding.
    fn condition_layout() -> Layout {
        layout(json!({"record_type":"Condition",
            "statuses":[
                {"label":"Clinical status","pointer":"/clinicalStatus/coding/0/display"},
                {"label":"Verification status","pointer":"/verificationStatus/coding/0/display"}],
            "sections":[
                {"heading":"Problem","slots":[
                    {"label":"Problem","pointer":"/code/text"},
                    {"label":"Note","pointer":"/note/0/text"}]},
                {"heading":"Coding","slots":[
                    {"label":"ICD-10-CM code","pointer":"/code/coding/0/code"}]}]}))
    }

    /// The observation as retained, with its result written `7.20` as some sources write it.
    fn retained_observation() -> Citation {
        let mut cited = general_citation(&observation());
        let written = cited.content.replace("\"value\":7.2", "\"value\":7.20");
        assert_ne!(written, cited.content);
        cited.content = written;
        cited
    }

    fn refusal(layout: &Layout, record: &Value) -> LayoutRefusal {
        layout.validate(record).unwrap_err()
    }

    /// The Observation layout with one slot of its first section replaced.
    fn with_slot(label: &str, pointer: &str) -> Layout {
        let mut changed = observation_layout();
        changed.sections[0].slots[0] = Slot {
            label: label.into(),
            pointer: pointer.into(),
        };
        changed
    }

    #[test]
    fn a_layout_cannot_carry_a_value_of_its_own() {
        let typed_in = json!({"record_type":"Observation","sections":[{"heading":"Result",
            "slots":[{"label":"Result","pointer":"/valueQuantity/value","value":"5.4"}]}]});
        assert!(serde_json::from_value::<Layout>(typed_in).is_err());
        let typed_section = json!({"record_type":"Observation","sections":[{"heading":"Result",
            "text":"Normal","slots":[]}]});
        assert!(serde_json::from_value::<Layout>(typed_section).is_err());
    }

    #[test]
    fn each_fault_refuses_the_layout() {
        let record = observation();
        assert_eq!(
            refusal(&observation_layout(), &condition()),
            LayoutRefusal::WrongRecordType {
                expected: "Observation".into(),
                found: Some("Condition".into()),
            }
        );
        assert_eq!(
            refusal(&observation_layout(), &json!([record.clone()])),
            LayoutRefusal::NotStructured
        );
        assert_eq!(
            refusal(&with_slot("Result", "valueQuantity/value"), &record),
            LayoutRefusal::MalformedPointer {
                pointer: "valueQuantity/value".into(),
                reason: PointerError::MissingLeadingSlash,
            }
        );
        assert_eq!(
            refusal(&with_slot("Result", "/value~2"), &record),
            LayoutRefusal::MalformedPointer {
                pointer: "/value~2".into(),
                reason: PointerError::BadEscape,
            }
        );
        for pointer in [
            "/valueString",
            "/interpretation/1",
            "/interpretation/01",
            "/interpretation/-",
            "/status/text",
        ] {
            assert_eq!(
                refusal(&with_slot("Result", pointer), &record),
                LayoutRefusal::Unresolved {
                    pointer: pointer.into()
                },
            );
        }
        for pointer in ["/valueQuantity", "/referenceRange", ""] {
            assert_eq!(
                refusal(&with_slot("Result", pointer), &record),
                LayoutRefusal::NotALeaf {
                    pointer: pointer.into()
                },
            );
        }
        assert_eq!(
            refusal(&with_slot("Result status", "/status"), &record),
            LayoutRefusal::DuplicateBinding {
                pointer: "/status".into()
            }
        );
        assert_eq!(
            refusal(&with_slot(" ", "/code/text"), &record),
            LayoutRefusal::EmptyLabel {
                pointer: "/code/text".into()
            }
        );
        let mut headless = observation_layout();
        headless.sections[1].heading = String::new();
        assert_eq!(
            refusal(&headless, &record),
            LayoutRefusal::EmptyHeading { index: 1 }
        );
        assert!(observation_layout().validate(&record).is_ok());
        assert!(condition_layout().validate(&condition()).is_ok());
    }

    #[test]
    fn an_escaped_pointer_reads_the_member_it_names() {
        let record = json!({"resourceType":"Record","a/b":{"c~d":"recorded"}});
        let escaped = layout(
            json!({"record_type":"Record","sections":[{"heading":"Fields",
            "slots":[{"label":"Field","pointer":"/a~1b/c~0d"}]}]}),
        );
        let validated = escaped.validate(&record).unwrap();
        assert_eq!(validated.sections[0].bindings[0].path, "a/b.c~d");
        assert_eq!(validated.sections[0].bindings[0].value, "recorded");
    }

    #[test]
    fn a_refused_layout_reads_exactly_as_the_general_view() {
        let observation = retained_observation();
        let condition = general_citation(&condition());
        let allergy = citation(
            r#"{"resourceType":"AllergyIntolerance","code":{"text":"Penicillin"},"criticality":"high"}"#,
        );
        let unreadable = citation("Plain text note");
        let refused = [
            with_slot("Result", "/valueString"),
            with_slot("", "/code/text"),
            with_slot("Result", "/valueQuantity"),
            condition_layout(),
        ];
        for cited in [&observation, &condition, &allergy, &unreadable] {
            for layout in &refused {
                if cited.record_type == layout.record_type
                    && layout
                        .validate(&serde_json::from_str(&cited.content).unwrap())
                        .is_ok()
                {
                    continue;
                }
                assert_eq!(
                    serde_json::to_value(inspection_with_layout(cited, Some(layout))).unwrap(),
                    serde_json::to_value(inspection(cited)).unwrap(),
                    "{} through a refused layout",
                    cited.record_type
                );
            }
        }
        let general = inspection(&observation);
        assert!(general.layout_sections.is_empty());
        assert!(general.other_fields.is_empty());
    }

    #[test]
    fn an_observation_reads_under_its_layout_with_values_as_recorded() {
        let cited = retained_observation();
        let view = inspection_with_layout(&cited, Some(&observation_layout()));
        assert_eq!(
            view.layout_sections,
            vec![
                SourceLayoutSection {
                    heading: "Result".into(),
                    fields: vec![
                        located("Test", "Hemoglobin A1c", "code.text"),
                        located("Result", "7.20", "valueQuantity.value"),
                        located("Unit", "%", "valueQuantity.unit"),
                        located(
                            "Interpretation",
                            "High",
                            "interpretation[0].coding[0].display"
                        ),
                    ],
                },
                SourceLayoutSection {
                    heading: "Reference range".into(),
                    fields: vec![
                        located("Low", "4.0", "referenceRange[0].low.value"),
                        located("High", "5.6", "referenceRange[0].high.value"),
                        located("Range as recorded", "4.0-5.6 %", "referenceRange[0].text"),
                    ],
                },
            ]
        );
        assert_eq!(
            view.statuses,
            vec![located("Result status", "final", "status")]
        );
        // What the layout leaves of a field it reads from stays, under the general label.
        for other in [
            located(
                "Value system",
                "http://unitsofmeasure.org",
                "valueQuantity.system",
            ),
            located(
                "Statement in this record coding 1 code",
                "4548-4",
                "code.coding[0].code",
            ),
            located(
                "Reference range low unit",
                "%",
                "referenceRange[0].low.unit",
            ),
            located(
                "Category",
                "Laboratory (laboratory · http://terminology.hl7.org/CodeSystem/observation-category)",
                "category[0]",
            ),
        ] {
            assert!(
                view.other_fields.contains(&other),
                "{other:?} in {:#?}",
                view.other_fields
            );
        }
        assert!(view.record_details.is_empty());
        assert!(view.sections.is_empty());
        // Dates, attribution and Technical details keep their general reading.
        let general = inspection(&cited);
        assert_eq!(view.dates, general.dates);
        assert_eq!(view.attribution, general.attribution);
        assert_eq!(view.technical, general.technical);
        assert_eq!(view.title, general.title);
        assert_eq!(view.record_kind, general.record_kind);
        assert_shown_values_are_recorded(&cited, &view);
        assert_view_shows_every_leaf(&cited, &view);
    }

    #[test]
    fn a_condition_reads_under_its_layout_with_statuses_first() {
        let cited = general_citation(&condition());
        let view = inspection_with_layout(&cited, Some(&condition_layout()));
        assert_eq!(
            view.statuses,
            vec![
                located(
                    "Clinical status",
                    "Resolved",
                    "clinicalStatus.coding[0].display"
                ),
                located(
                    "Verification status",
                    "Confirmed",
                    "verificationStatus.coding[0].display"
                ),
            ]
        );
        assert_eq!(
            view.layout_sections,
            vec![
                SourceLayoutSection {
                    heading: "Problem".into(),
                    fields: vec![
                        located(
                            "Problem",
                            "Type 2 diabetes mellitus without complications",
                            "code.text"
                        ),
                        located(
                            "Note",
                            "about 14 years per patient and prior records",
                            "note[0].text"
                        ),
                    ],
                },
                SourceLayoutSection {
                    heading: "Coding".into(),
                    fields: vec![located("ICD-10-CM code", "E11.9", "code.coding[0].code")],
                },
            ]
        );
        assert!(view.other_fields.contains(&located(
            "Statement in this record coding 1 system",
            "http://hl7.org/fhir/sid/icd-10-cm",
            "code.coding[0].system"
        )));
        // The status codes the layout does not bind stay under Technical details only.
        assert!(
            readable(&view)
                .iter()
                .all(|field| field.path.as_deref() != Some("clinicalStatus.coding[0].code"))
        );
        assert_eq!(view.dates, inspection(&cited).dates);
        assert_shown_values_are_recorded(&cited, &view);
        assert_view_shows_every_leaf(&cited, &view);
    }

    #[test]
    fn a_layout_reads_a_record_with_a_field_it_has_never_seen() {
        let mut record = observation();
        record["futureElement"] = json!({"kind":"unrecognized"});
        let cited = general_citation(&record);
        let view = inspection_with_layout(&cited, Some(&observation_layout()));
        assert!(view.other_fields.contains(&located(
            "Future element kind",
            "unrecognized",
            "futureElement.kind"
        )));
        assert_view_shows_every_leaf(&cited, &view);
    }

    /// Every layout and Other field reads exactly the record's value at its path.
    fn assert_shown_values_are_recorded(cited: &Citation, view: &SourceInspection) {
        let record: Value = serde_json::from_str(&cited.content).unwrap();
        let numbers = NumberLiterals::read(&cited.content);
        let placed = view
            .layout_sections
            .iter()
            .flat_map(|section| &section.fields)
            .chain(&view.other_fields)
            .chain(&view.statuses);
        for shown in placed {
            let path = shown.path.as_deref().expect("a placed field has a path");
            let value = at(&record, path).unwrap_or_else(|| panic!("no {path}"));
            if !value.is_object() && !value.is_array() {
                assert_eq!(shown.text, numbers.recorded(value, path), "{path}");
            }
        }
    }
}
