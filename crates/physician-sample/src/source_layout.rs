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

/// A headed group of fields, or one such group for each element of a recorded array.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Section {
    /// The group's heading, written by the layout and never read as source text. A repeated
    /// group reads under its element's own code instead, and under this heading numbered from 1
    /// when the element has none.
    pub heading: String,
    /// A pointer to an array in the record, such as `/component`. When set, the section repeats
    /// once per element and each slot's pointer is relative to that element.
    #[serde(default)]
    pub repeat: Option<String>,
    /// The group's fields, in reading order.
    pub slots: Vec<Slot>,
}

/// A label for the single value at `pointer`, such as `/valueQuantity/value`.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Slot {
    /// What the value means, written by the layout.
    pub label: String,
    /// What part the value plays, checked against where it sits in the record.
    pub role: SlotRole,
    /// Where the value is in the record.
    pub pointer: String,
}

/// What part a slot's value plays in the record. A role is checked against the path it binds,
/// so a layout cannot present a reference-range limit as the result, or a code as a date.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SlotRole {
    /// The recorded result; never part of a reference range.
    Result,
    /// The unit of a quantity.
    Unit,
    /// A reference range's lower limit.
    ReferenceRangeLow,
    /// A reference range's upper limit.
    ReferenceRangeHigh,
    /// A reference range as its source wrote it.
    ReferenceRangeText,
    /// The record's interpretation of its result.
    Interpretation,
    /// One of the record's statuses.
    Status,
    /// A FHIR date or dateTime.
    Date,
    /// A code or coded display.
    Code,
    /// A note attached to the record.
    Note,
    /// Any other recorded value.
    Other,
}

impl fmt::Display for SlotRole {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Result => "result",
            Self::Unit => "unit",
            Self::ReferenceRangeLow => "reference range low",
            Self::ReferenceRangeHigh => "reference range high",
            Self::ReferenceRangeText => "reference range text",
            Self::Interpretation => "interpretation",
            Self::Status => "status",
            Self::Date => "date",
            Self::Code => "code",
            Self::Note => "note",
            Self::Other => "other",
        })
    }
}

impl SlotRole {
    /// Whether an element of a repeated section may simply lack this role's value. A result,
    /// unit, code, status or date is what an element is read for, so its absence must be
    /// recorded in the element's `dataAbsentReason`; notes, interpretations, reference ranges
    /// and other values are often left out.
    fn absence_is_ordinary(self) -> bool {
        match self {
            Self::Note
            | Self::Interpretation
            | Self::ReferenceRangeLow
            | Self::ReferenceRangeHigh
            | Self::ReferenceRangeText
            | Self::Other => true,
            Self::Result | Self::Unit | Self::Code | Self::Status | Self::Date => false,
        }
    }
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
    /// A slot's or a section repeat's pointer names nothing in this record.
    Unresolved {
        /// The pointer as the layout wrote it.
        pointer: String,
    },
    /// A section repeats over something other than an array.
    RepeatNotAnArray {
        /// The section's repeat pointer as the layout wrote it.
        pointer: String,
    },
    /// An element of a repeated section lacks a value its slot's role requires, and does not
    /// record why in a `dataAbsentReason`.
    MissingInElement {
        /// The slot's pointer as the layout wrote it, relative to the element.
        pointer: String,
        /// The element's index in the repeated array.
        index: usize,
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
    /// A slot's role does not fit the value it binds.
    RoleMismatch {
        /// The slot's pointer as the layout wrote it.
        pointer: String,
        /// The role the layout gave the slot.
        role: SlotRole,
        /// Why the bound value cannot play that role.
        reason: RoleMismatchReason,
    },
}

/// Why a bound value cannot play the role its slot gives it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RoleMismatchReason {
    /// A result is never part of a reference range.
    UnderReferenceRange,
    /// A unit is the `unit` or `code` of a quantity.
    NotAQuantityUnit,
    /// The value lies outside the element the role reads.
    OutsideItsElement,
    /// A status is the record's own `status` or one of its `…Status` elements.
    NotAStatus,
    /// A date is a FHIR date or dateTime string.
    NotADate,
    /// A slot among the record's statuses must have the status role.
    StatusesHoldOnlyStatuses,
}

impl fmt::Display for RoleMismatchReason {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnderReferenceRange => write!(f, "it lies inside a reference range"),
            Self::NotAQuantityUnit => write!(f, "it is not the unit of a quantity"),
            Self::OutsideItsElement => write!(f, "it lies outside the element that role reads"),
            Self::NotAStatus => write!(f, "it is not one of the record's statuses"),
            Self::NotADate => write!(f, "it is not a FHIR date or dateTime"),
            Self::StatusesHoldOnlyStatuses => {
                write!(f, "the record's statuses hold only status slots")
            }
        }
    }
}

impl std::error::Error for RoleMismatchReason {}

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
            Self::RepeatNotAnArray { pointer } => {
                write!(f, "{pointer} is not an array a section can repeat over")
            }
            Self::MissingInElement { pointer, index } => write!(
                f,
                "element {index} has no {pointer} and does not record why it is absent"
            ),
            Self::NotALeaf { pointer } => {
                write!(
                    f,
                    "{pointer} points at an object or array, not a single recorded value"
                )
            }
            Self::DuplicateBinding { pointer } => write!(f, "{pointer} is bound more than once"),
            Self::RoleMismatch { pointer, role, .. } => {
                write!(f, "{pointer} cannot be read as the {role}")
            }
        }
    }
}

impl std::error::Error for LayoutRefusal {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::MalformedPointer { reason, .. } => Some(reason),
            Self::RoleMismatch { reason, .. } => Some(reason),
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
        let mut statuses = Vec::with_capacity(self.statuses.len());
        for slot in &self.statuses {
            if slot.role != SlotRole::Status {
                return Err(LayoutRefusal::RoleMismatch {
                    pointer: slot.pointer.clone(),
                    role: slot.role,
                    reason: RoleMismatchReason::StatusesHoldOnlyStatuses,
                });
            }
            statuses.push(bind(slot, record, &mut bound)?);
        }
        let mut sections = Vec::with_capacity(self.sections.len());
        for (index, section) in self.sections.iter().enumerate() {
            if section.heading.trim().is_empty() {
                return Err(LayoutRefusal::EmptyHeading { index });
            }
            match &section.repeat {
                Some(repeat) => sections.extend(repeated(section, repeat, record, &mut bound)?),
                None => sections.push(BoundSection {
                    heading: section.heading.clone(),
                    bindings: section
                        .slots
                        .iter()
                        .map(|slot| bind(slot, record, &mut bound))
                        .collect::<Result<_, _>>()?,
                }),
            }
        }
        Ok(ValidatedLayout {
            record,
            statuses,
            sections,
        })
    }
}

/// Binds a slot of a section that does not repeat, or of the record's statuses.
fn bind<'r>(
    slot: &Slot,
    record: &'r Value,
    bound: &mut BTreeSet<String>,
) -> Result<Binding<'r>, LayoutRefusal> {
    let tokens = checked(slot)?;
    let found = follow(record, &tokens, String::new(), Vec::new()).ok_or_else(|| {
        LayoutRefusal::Unresolved {
            pointer: slot.pointer.clone(),
        }
    })?;
    place(slot, found, &slot.pointer, bound)
}

/// Binds a repeated section once per element of the array at `repeat`, each slot's pointer read
/// from that element. An element may lack a slot's value only when the slot's role allows it or
/// the element records a `dataAbsentReason`.
fn repeated<'r>(
    section: &Section,
    repeat: &str,
    record: &'r Value,
    bound: &mut BTreeSet<String>,
) -> Result<Vec<BoundSection<'r>>, LayoutRefusal> {
    let repeat_tokens = tokens(repeat).map_err(|reason| LayoutRefusal::MalformedPointer {
        pointer: repeat.to_owned(),
        reason,
    })?;
    let array = follow(record, &repeat_tokens, String::new(), Vec::new()).ok_or_else(|| {
        LayoutRefusal::Unresolved {
            pointer: repeat.to_owned(),
        }
    })?;
    let Value::Array(elements) = array.value else {
        return Err(LayoutRefusal::RepeatNotAnArray {
            pointer: repeat.to_owned(),
        });
    };
    // Labels and pointers are checked even when the array is empty.
    let slots = section
        .slots
        .iter()
        .map(|slot| Ok((slot, checked(slot)?)))
        .collect::<Result<Vec<_>, LayoutRefusal>>()?;
    let mut sections = Vec::with_capacity(elements.len());
    for (index, element) in elements.iter().enumerate() {
        let path = format!("{}[{index}]", array.path);
        let recorded_absence = element.get("dataAbsentReason").is_some();
        let mut bindings = Vec::with_capacity(slots.len());
        for (slot, tokens) in &slots {
            match follow(element, tokens, path.clone(), array.members.clone()) {
                Some(found) => {
                    let absolute = format!("{repeat}/{index}{}", slot.pointer);
                    bindings.push(place(slot, found, &absolute, bound)?);
                }
                None if slot.role.absence_is_ordinary() || recorded_absence => {}
                None => {
                    return Err(LayoutRefusal::MissingInElement {
                        pointer: slot.pointer.clone(),
                        index,
                    });
                }
            }
        }
        sections.push(BoundSection {
            heading: element_heading(element, &section.heading, index),
            bindings,
        });
    }
    Ok(sections)
}

/// A repeated section's heading for one element: the element's own coded display or code text,
/// or else the layout's heading numbered from 1.
fn element_heading(element: &Value, heading: &str, index: usize) -> String {
    let code = &element["code"];
    [&code["coding"][0]["display"], &code["text"]]
        .into_iter()
        .filter_map(Value::as_str)
        .find(|text| !text.trim().is_empty())
        .map_or_else(|| format!("{heading} {}", index + 1), str::to_owned)
}

/// The slot's pointer tokens, once its label and pointer are well formed.
fn checked(slot: &Slot) -> Result<Vec<String>, LayoutRefusal> {
    if slot.label.trim().is_empty() {
        return Err(LayoutRefusal::EmptyLabel {
            pointer: slot.pointer.clone(),
        });
    }
    tokens(&slot.pointer).map_err(|reason| LayoutRefusal::MalformedPointer {
        pointer: slot.pointer.clone(),
        reason,
    })
}

/// A value a pointer names, with what its role is checked against.
#[derive(Debug)]
struct Found<'r> {
    /// Its path from the record root, in the inspection's path notation.
    path: String,
    /// The member names along that path, without array indexes.
    members: Vec<String>,
    /// The object or array holding it.
    parent: &'r Value,
    value: &'r Value,
}

/// Follows `tokens` from `start`, which sits at `path` under `members`; `None` when they name
/// nothing.
fn follow<'r>(
    start: &'r Value,
    tokens: &[String],
    mut path: String,
    mut members: Vec<String>,
) -> Option<Found<'r>> {
    let mut value = start;
    let mut parent = start;
    for token in tokens {
        parent = value;
        value = match value {
            Value::Object(object) => {
                let member = object.get(token)?;
                if !path.is_empty() {
                    path.push('.');
                }
                path.push_str(token);
                members.push(token.clone());
                member
            }
            Value::Array(items) => {
                let index = index(token)?;
                let item = items.get(index)?;
                path.push_str(&format!("[{index}]"));
                item
            }
            _ => return None,
        };
    }
    Some(Found {
        path,
        members,
        parent,
        value,
    })
}

/// Binds `slot` to the value it found, refusing a composite value, a role that does not fit it,
/// or a value already bound. Refusals name the value by `pointer`.
fn place<'r>(
    slot: &Slot,
    found: Found<'r>,
    pointer: &str,
    bound: &mut BTreeSet<String>,
) -> Result<Binding<'r>, LayoutRefusal> {
    let pointer = || pointer.to_owned();
    if found.value.is_object() || found.value.is_array() {
        return Err(LayoutRefusal::NotALeaf { pointer: pointer() });
    }
    fits(slot.role, &found.members, found.parent, found.value).map_err(|reason| {
        LayoutRefusal::RoleMismatch {
            pointer: pointer(),
            role: slot.role,
            reason,
        }
    })?;
    // Two spellings of one pointer bind the same value, so duplicates compare by path.
    if !bound.insert(found.path.clone()) {
        return Err(LayoutRefusal::DuplicateBinding { pointer: pointer() });
    }
    Ok(Binding {
        label: slot.label.clone(),
        path: found.path,
        value: found.value,
    })
}

/// Whether a value can play `role`, given the member names along its path (`members`, without
/// array indexes), the object or array holding it, and the value itself.
fn fits(
    role: SlotRole,
    members: &[String],
    parent: &Value,
    value: &Value,
) -> Result<(), RoleMismatchReason> {
    let has = |name: &str| members.iter().any(|member| member == name);
    // The member right after `referenceRange`: `low`, `high` or `text`.
    let range_part = members
        .iter()
        .position(|member| member == "referenceRange")
        .and_then(|at| members.get(at + 1))
        .map(String::as_str);
    let holds = |fits: bool, reason| if fits { Ok(()) } else { Err(reason) };
    match role {
        SlotRole::Result => holds(
            !has("referenceRange"),
            RoleMismatchReason::UnderReferenceRange,
        ),
        SlotRole::Unit => {
            let last = members.last().map(String::as_str);
            let holder = members.iter().rev().nth(1).map(String::as_str);
            let quantity = parent.get("value").is_some()
                || holder.is_some_and(|holder| {
                    holder.ends_with("Quantity") || holder == "low" || holder == "high"
                });
            holds(
                matches!(last, Some("unit" | "code")) && quantity,
                RoleMismatchReason::NotAQuantityUnit,
            )
        }
        SlotRole::ReferenceRangeLow => holds(
            range_part == Some("low"),
            RoleMismatchReason::OutsideItsElement,
        ),
        SlotRole::ReferenceRangeHigh => holds(
            range_part == Some("high"),
            RoleMismatchReason::OutsideItsElement,
        ),
        SlotRole::ReferenceRangeText => holds(
            range_part == Some("text"),
            RoleMismatchReason::OutsideItsElement,
        ),
        SlotRole::Interpretation => {
            holds(has("interpretation"), RoleMismatchReason::OutsideItsElement)
        }
        SlotRole::Status => holds(
            members
                .first()
                .is_some_and(|first| first.ends_with("status") || first.ends_with("Status")),
            RoleMismatchReason::NotAStatus,
        ),
        SlotRole::Date => holds(
            value.as_str().is_some_and(is_fhir_date),
            RoleMismatchReason::NotADate,
        ),
        SlotRole::Note => holds(has("note"), RoleMismatchReason::OutsideItsElement),
        SlotRole::Code | SlotRole::Other => Ok(()),
    }
}

/// Whether `text` is a FHIR `date` (`YYYY`, `YYYY-MM`, `YYYY-MM-DD`) or `dateTime`
/// (`YYYY-MM-DDThh:mm:ss[.fff]` with `Z` or a `±hh:mm` offset).
fn is_fhir_date(text: &str) -> bool {
    let (date, time) = match text.split_once('T') {
        Some((date, time)) => (date, Some(time)),
        None => (text, None),
    };
    let mut parts = date.split('-');
    let year = parts.next().and_then(|year| number(year, 4));
    let month = parts.next().map(|month| number(month, 2));
    let day = parts.next().map(|day| number(day, 2));
    if parts.next().is_some() {
        return false;
    }
    let Some(year) = year.filter(|year| *year >= 1) else {
        return false;
    };
    let month = match month {
        None => return time.is_none(),
        Some(month) => match month.filter(|month| (1..=12).contains(month)) {
            Some(month) => month,
            None => return false,
        },
    };
    let Some(day) = day else {
        return time.is_none();
    };
    let leap = year % 4 == 0 && (year % 100 != 0 || year % 400 == 0);
    let days = match month {
        2 if leap => 29,
        2 => 28,
        4 | 6 | 9 | 11 => 30,
        _ => 31,
    };
    if !day.is_some_and(|day| (1..=days).contains(&day)) {
        return false;
    }
    time.is_none_or(is_fhir_time)
}

/// Whether `text` is `hh:mm:ss[.fff]` followed by `Z` or a `±hh:mm` offset.
fn is_fhir_time(text: &str) -> bool {
    let (clock, offset) = match text.strip_suffix('Z') {
        Some(clock) => (clock, None),
        None => match text.rfind(['+', '-']) {
            Some(at) => (&text[..at], Some(&text[at + 1..])),
            None => return false,
        },
    };
    let (clock, fraction) = match clock.split_once('.') {
        Some((clock, fraction)) => (clock, Some(fraction)),
        None => (clock, None),
    };
    if fraction.is_some_and(|fraction| {
        fraction.is_empty() || !fraction.bytes().all(|byte| byte.is_ascii_digit())
    }) {
        return false;
    }
    fields_within(clock, &[23, 59, 60])
        && offset.is_none_or(|offset| fields_within(offset, &[14, 59]))
}

/// Whether `text` is exactly as many `:`-separated two-digit fields as `limits`, each at most
/// its limit.
fn fields_within(text: &str, limits: &[u32]) -> bool {
    let fields: Vec<&str> = text.split(':').collect();
    fields.len() == limits.len()
        && fields
            .iter()
            .zip(limits)
            .all(|(field, limit)| number(field, 2).is_some_and(|value| value <= *limit))
}

/// `text` as a number, when it is exactly `width` ASCII digits.
fn number(text: &str, width: usize) -> Option<u32> {
    (text.len() == width && text.bytes().all(|byte| byte.is_ascii_digit()))
        .then(|| text.parse().ok())
        .flatten()
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
            "statuses":[{"label":"Result status","role":"status","pointer":"/status"}],
            "sections":[
                {"heading":"Result","slots":[
                    {"label":"Test","role":"code","pointer":"/code/text"},
                    {"label":"Result","role":"result","pointer":"/valueQuantity/value"},
                    {"label":"Unit","role":"unit","pointer":"/valueQuantity/unit"},
                    {"label":"Interpretation","role":"interpretation","pointer":"/interpretation/0/coding/0/display"}]},
                {"heading":"Reference range","slots":[
                    {"label":"Low","role":"reference_range_low","pointer":"/referenceRange/0/low/value"},
                    {"label":"High","role":"reference_range_high","pointer":"/referenceRange/0/high/value"},
                    {"label":"Range as recorded","role":"reference_range_text","pointer":"/referenceRange/0/text"}]}]}))
    }

    /// A hand-written Condition layout: the problem and its note, then its coding.
    fn condition_layout() -> Layout {
        layout(json!({"record_type":"Condition",
            "statuses":[
                {"label":"Clinical status","role":"status","pointer":"/clinicalStatus/coding/0/display"},
                {"label":"Verification status","role":"status","pointer":"/verificationStatus/coding/0/display"}],
            "sections":[
                {"heading":"Problem","slots":[
                    {"label":"Problem","role":"code","pointer":"/code/text"},
                    {"label":"Note","role":"note","pointer":"/note/0/text"}]},
                {"heading":"Coding","slots":[
                    {"label":"ICD-10-CM code","role":"code","pointer":"/code/coding/0/code"}]}]}))
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
        with_role(label, SlotRole::Other, pointer)
    }

    /// The Observation layout with one slot of its first section replaced by one with `role`.
    fn with_role(label: &str, role: SlotRole, pointer: &str) -> Layout {
        let mut changed = observation_layout();
        changed.sections[0].slots[0] = Slot {
            label: label.into(),
            role,
            pointer: pointer.into(),
        };
        changed
    }

    /// The refusal for a slot giving the value at `pointer` the `role`.
    fn mismatch(role: SlotRole, pointer: &str, reason: RoleMismatchReason) -> LayoutRefusal {
        LayoutRefusal::RoleMismatch {
            pointer: pointer.into(),
            role,
            reason,
        }
    }

    #[test]
    fn a_reference_range_low_cannot_be_read_as_the_result() {
        let pointer = "/referenceRange/0/low/value";
        let mut planted = observation_layout();
        planted.sections[0].slots[1].pointer = pointer.into();
        planted.sections[1].slots.remove(0);
        assert_eq!(
            refusal(&planted, &observation()),
            mismatch(
                SlotRole::Result,
                pointer,
                RoleMismatchReason::UnderReferenceRange
            )
        );
        let view = inspection_with_layout(&retained_observation(), Some(&planted));
        assert!(view.layout_sections.is_empty());
    }

    #[test]
    fn a_role_that_does_not_fit_its_value_refuses_the_layout() {
        use RoleMismatchReason::{
            NotADate, NotAQuantityUnit, NotAStatus, OutsideItsElement, UnderReferenceRange,
        };
        use SlotRole::*;
        let record = observation();
        let cases = [
            (Result, "/referenceRange/0/high/value", UnderReferenceRange),
            (Result, "/referenceRange/0/text", UnderReferenceRange),
            (Unit, "/code/text", NotAQuantityUnit),
            (Unit, "/valueQuantity/system", NotAQuantityUnit),
            (Unit, "/code/coding/0/code", NotAQuantityUnit),
            (
                ReferenceRangeLow,
                "/referenceRange/0/high/value",
                OutsideItsElement,
            ),
            (ReferenceRangeLow, "/valueQuantity/value", OutsideItsElement),
            (
                ReferenceRangeHigh,
                "/referenceRange/0/low/value",
                OutsideItsElement,
            ),
            (
                ReferenceRangeText,
                "/referenceRange/0/low/unit",
                OutsideItsElement,
            ),
            (ReferenceRangeText, "/code/text", OutsideItsElement),
            (Interpretation, "/code/coding/0/display", OutsideItsElement),
            (Status, "/code/text", NotAStatus),
            (Status, "/category/0/coding/0/code", NotAStatus),
            (Date, "/code/text", NotADate),
            (Date, "/valueQuantity/value", NotADate),
            (Note, "/code/text", OutsideItsElement),
        ];
        for (role, pointer, reason) in cases {
            assert_eq!(
                refusal(&with_role("Field", role, pointer), &record),
                mismatch(role, pointer, reason),
                "{role} at {pointer}"
            );
        }
        let mut statuses = observation_layout();
        statuses.statuses[0].role = SlotRole::Code;
        assert_eq!(
            refusal(&statuses, &record),
            mismatch(
                SlotRole::Code,
                "/status",
                RoleMismatchReason::StatusesHoldOnlyStatuses
            )
        );
    }

    #[test]
    fn each_role_reads_a_value_that_fits_it() {
        use SlotRole::*;
        let observation = observation();
        let fitting = [
            (Result, "/valueQuantity/value"),
            (Unit, "/valueQuantity/unit"),
            (Unit, "/valueQuantity/code"),
            (Unit, "/referenceRange/0/low/unit"),
            (ReferenceRangeLow, "/referenceRange/0/low/value"),
            (ReferenceRangeHigh, "/referenceRange/0/high/value"),
            (ReferenceRangeText, "/referenceRange/0/text"),
            (Interpretation, "/interpretation/0/coding/0/code"),
            (Date, "/effectiveDateTime"),
            (Date, "/issued"),
            (Code, "/code/coding/0/code"),
            (Other, "/performer/0/display"),
        ];
        for (role, pointer) in fitting {
            let mut layout = with_role("Field", role, pointer);
            layout.sections[1].slots.clear();
            layout.sections[0].slots.truncate(1);
            assert!(
                layout.validate(&observation).is_ok(),
                "{role} at {pointer}: {:?}",
                layout.validate(&observation).err()
            );
        }
        let condition = condition();
        for (role, pointer) in [
            (Status, "/clinicalStatus/coding/0/code"),
            (Note, "/note/0/text"),
            (Date, "/onsetDateTime"),
        ] {
            let layout = Layout {
                record_type: "Condition".into(),
                statuses: Vec::new(),
                sections: vec![Section {
                    heading: "Fields".into(),
                    repeat: None,
                    slots: vec![Slot {
                        label: "Field".into(),
                        role,
                        pointer: pointer.into(),
                    }],
                }],
            };
            assert!(layout.validate(&condition).is_ok(), "{role} at {pointer}");
        }
        let mut status = with_role("Status", Status, "/status");
        status.statuses.clear();
        assert!(status.validate(&observation).is_ok());
    }

    #[test]
    fn a_date_is_a_fhir_date_or_date_time() {
        for date in [
            "2019",
            "2019-03",
            "2019-03-12",
            "2020-02-29",
            "2019-03-12T13:30:00Z",
            "2019-03-12T13:30:00+00:00",
            "2019-03-12T13:30:00.123-07:00",
        ] {
            assert!(is_fhir_date(date), "{date}");
        }
        for not_a_date in [
            "",
            "19",
            "0000",
            "2019-3",
            "2019-13",
            "2019-02-29",
            "2019-04-31",
            "2019-03-12-01",
            "2019-03T13:30:00Z",
            "2019-03-12T",
            "2019-03-12T13:30Z",
            "2019-03-12T13:30:00",
            "2019-03-12T24:00:00Z",
            "2019-03-12T13:30:00.Z",
            "2019-03-12T13:30:00+0000",
            "March 12, 2019",
        ] {
            assert!(!is_fhir_date(not_a_date), "{not_a_date}");
        }
    }

    #[test]
    fn a_layout_cannot_carry_a_value_of_its_own() {
        let typed_in = json!({"record_type":"Observation","sections":[{"heading":"Result",
            "slots":[{"label":"Result","role":"result","pointer":"/valueQuantity/value",
            "value":"5.4"}]}]});
        assert!(serde_json::from_value::<Layout>(typed_in).is_err());
        let roleless = json!({"record_type":"Observation","sections":[{"heading":"Result",
            "slots":[{"label":"Result","pointer":"/valueQuantity/value"}]}]});
        assert!(serde_json::from_value::<Layout>(roleless).is_err());
        let unknown_role = json!({"record_type":"Observation","sections":[{"heading":"Result",
            "slots":[{"label":"Result","role":"headline","pointer":"/valueQuantity/value"}]}]});
        assert!(serde_json::from_value::<Layout>(unknown_role).is_err());
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
            "slots":[{"label":"Field","role":"other","pointer":"/a~1b/c~0d"}]}]}),
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

    /// A blood-pressure Observation: a systolic component named by its coding and interpreted,
    /// and a diastolic component named by its text and not interpreted.
    fn blood_pressure() -> Value {
        json!({"resourceType":"Observation","status":"final",
            "code":{"coding":[{"system":"http://loinc.org","code":"85354-9",
                "display":"Blood pressure panel"}],"text":"Blood pressure"},
            "effectiveDateTime":"2019-03-12T13:30:00Z",
            "component":[
                {"code":{"coding":[{"system":"http://loinc.org","code":"8480-6",
                    "display":"Systolic blood pressure"}]},
                 "valueQuantity":{"value":142,"unit":"mmHg",
                    "system":"http://unitsofmeasure.org","code":"mm[Hg]"},
                 "interpretation":[{"coding":[{"code":"H","display":"High"}]}],
                 "referenceRange":[{"high":{"value":130,"unit":"mmHg"}}]},
                {"code":{"text":"Diastolic blood pressure"},
                 "valueQuantity":{"value":81.5,"unit":"mm Hg",
                    "system":"http://unitsofmeasure.org","code":"mm[Hg]"}}]})
    }

    /// The panel's name, then one section per component.
    fn blood_pressure_layout() -> Layout {
        layout(json!({"record_type":"Observation",
            "statuses":[{"label":"Result status","role":"status","pointer":"/status"}],
            "sections":[
                {"heading":"Panel","slots":[
                    {"label":"Test","role":"code","pointer":"/code/text"}]},
                {"heading":"Component","repeat":"/component","slots":[
                    {"label":"Result","role":"result","pointer":"/valueQuantity/value"},
                    {"label":"Unit","role":"unit","pointer":"/valueQuantity/unit"},
                    {"label":"Unit code","role":"unit","pointer":"/valueQuantity/code"},
                    {"label":"Interpretation","role":"interpretation",
                     "pointer":"/interpretation/0/coding/0/display"}]}]}))
    }

    #[test]
    fn a_repeated_section_reads_each_component_with_its_own_unit() {
        let mut cited = general_citation(&blood_pressure());
        let written = cited.content.replace("81.5", "81.50");
        assert_ne!(written, cited.content);
        cited.content = written;
        let view = inspection_with_layout(&cited, Some(&blood_pressure_layout()));
        assert_eq!(
            view.layout_sections,
            vec![
                SourceLayoutSection {
                    heading: "Panel".into(),
                    fields: vec![located("Test", "Blood pressure", "code.text")],
                },
                SourceLayoutSection {
                    heading: "Systolic blood pressure".into(),
                    fields: vec![
                        located("Result", "142", "component[0].valueQuantity.value"),
                        located("Unit", "mmHg", "component[0].valueQuantity.unit"),
                        located("Unit code", "mm[Hg]", "component[0].valueQuantity.code"),
                        located(
                            "Interpretation",
                            "High",
                            "component[0].interpretation[0].coding[0].display"
                        ),
                    ],
                },
                SourceLayoutSection {
                    heading: "Diastolic blood pressure".into(),
                    fields: vec![
                        located("Result", "81.50", "component[1].valueQuantity.value"),
                        located("Unit", "mm Hg", "component[1].valueQuantity.unit"),
                        located("Unit code", "mm[Hg]", "component[1].valueQuantity.code"),
                    ],
                },
            ]
        );
        assert_shown_values_are_recorded(&cited, &view);
        assert_view_shows_every_leaf(&cited, &view);
    }

    #[test]
    fn a_repeated_element_without_a_code_reads_under_the_numbered_heading() {
        let mut record = blood_pressure();
        for component in record["component"].as_array_mut().unwrap() {
            component.as_object_mut().unwrap().remove("code");
        }
        let validated = blood_pressure_layout().validate(&record).unwrap();
        let headings: Vec<&str> = validated
            .sections
            .iter()
            .map(|section| section.heading.as_str())
            .collect();
        assert_eq!(headings, ["Panel", "Component 1", "Component 2"]);
    }

    #[test]
    fn an_empty_repeated_array_reads_no_sections() {
        let mut record = blood_pressure();
        record["component"] = json!([]);
        let cited = general_citation(&record);
        let view = inspection_with_layout(&cited, Some(&blood_pressure_layout()));
        assert_eq!(
            view.layout_sections,
            vec![SourceLayoutSection {
                heading: "Panel".into(),
                fields: vec![located("Test", "Blood pressure", "code.text")],
            }]
        );
        assert_view_shows_every_leaf(&cited, &view);
    }

    #[test]
    fn an_element_may_lack_a_value_whose_absence_is_ordinary() {
        let record = blood_pressure();
        assert!(record["component"][1].get("interpretation").is_none());
        let validated = blood_pressure_layout().validate(&record).unwrap();
        let labels: Vec<&str> = validated.sections[2]
            .bindings
            .iter()
            .map(|binding| binding.label.as_str())
            .collect();
        assert_eq!(labels, ["Result", "Unit", "Unit code"]);
    }

    #[test]
    fn an_element_lacking_a_required_value_refuses_unless_it_records_why() {
        let mut record = blood_pressure();
        record["component"][1]
            .as_object_mut()
            .unwrap()
            .remove("valueQuantity");
        assert_eq!(
            refusal(&blood_pressure_layout(), &record),
            LayoutRefusal::MissingInElement {
                pointer: "/valueQuantity/value".into(),
                index: 1,
            }
        );
        let mut coded = blood_pressure_layout();
        coded.sections[1].slots = vec![Slot {
            label: "Code".into(),
            role: SlotRole::Code,
            pointer: "/code/coding/0/code".into(),
        }];
        assert_eq!(
            refusal(&coded, &record),
            LayoutRefusal::MissingInElement {
                pointer: "/code/coding/0/code".into(),
                index: 1,
            }
        );
        record["component"][1]["dataAbsentReason"] = json!({"coding":[{"code":"not-performed"}]});
        let cited = general_citation(&record);
        let view = inspection_with_layout(&cited, Some(&blood_pressure_layout()));
        assert_eq!(view.layout_sections[2].heading, "Diastolic blood pressure");
        assert!(view.layout_sections[2].fields.is_empty());
        assert_view_shows_every_leaf(&cited, &view);
    }

    #[test]
    fn each_repeat_fault_refuses_the_layout() {
        let record = blood_pressure();
        let repeating = |repeat: &str| {
            let mut changed = blood_pressure_layout();
            changed.sections[1].repeat = Some(repeat.into());
            changed
        };
        for not_an_array in ["/code", "/status", ""] {
            assert_eq!(
                refusal(&repeating(not_an_array), &record),
                LayoutRefusal::RepeatNotAnArray {
                    pointer: not_an_array.into()
                }
            );
        }
        assert_eq!(
            refusal(&repeating("/components"), &record),
            LayoutRefusal::Unresolved {
                pointer: "/components".into()
            }
        );
        assert_eq!(
            refusal(&repeating("component"), &record),
            LayoutRefusal::MalformedPointer {
                pointer: "component".into(),
                reason: PointerError::MissingLeadingSlash,
            }
        );
        // A role is checked against the value's place in the whole record.
        let mut ranged = blood_pressure_layout();
        ranged.sections[1].slots[0].pointer = "/referenceRange/0/high/value".into();
        assert_eq!(
            refusal(&ranged, &record),
            mismatch(
                SlotRole::Result,
                "/component/0/referenceRange/0/high/value",
                RoleMismatchReason::UnderReferenceRange
            )
        );
        // A plain section and a repeated one cannot both bind one value.
        let mut twice = blood_pressure_layout();
        twice.sections[0].slots.push(Slot {
            label: "Systolic".into(),
            role: SlotRole::Result,
            pointer: "/component/0/valueQuantity/value".into(),
        });
        assert_eq!(
            refusal(&twice, &record),
            LayoutRefusal::DuplicateBinding {
                pointer: "/component/0/valueQuantity/value".into()
            }
        );
        // An empty array still checks the slots' labels and pointers.
        let mut empty = record.clone();
        empty["component"] = json!([]);
        let mut unlabelled = blood_pressure_layout();
        unlabelled.sections[1].slots[0].label = " ".into();
        assert_eq!(
            refusal(&unlabelled, &empty),
            LayoutRefusal::EmptyLabel {
                pointer: "/valueQuantity/value".into()
            }
        );
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
