//! A layout arranges records of one type into headed sections. It holds only fixed heading
//! choices, roles and JSON pointers (RFC 6901) into the record, never text of its own: every
//! value it shows is copied from the record, every label comes from the slot's role or the
//! record's own field name, and every heading is a fixed one or the repeated element's own code.
//! A layout is checked against each record before it renders; a refused layout never shows in
//! part, and the record reads through the general view instead.
use crate::source_inspection::{NumberLiterals, humanize, located};
use crate::source_path::{self, SourcePath, Step};
use physician_wire::{SourceField, SourceInspection, SourceLayoutSection};
use serde::Deserialize;
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

/// An unchecked layout, in the shape a model or a hand-written fixture supplies. Unknown members
/// are refused when it is read, so a layout cannot carry a literal value beside its pointers.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Layout {
    /// The record type it reads, compared with the type the record's source adapter gave the
    /// record (a layout key's record type), so a record that is not FHIR can have a layout too.
    pub record_type: String,
    /// Status slots read beside the record's kind and date. The general view's statuses always
    /// read there too, so these add only a status no general status holds.
    #[serde(default)]
    pub statuses: Vec<Slot>,
    /// Headed groups of fields, in reading order.
    pub sections: Vec<Section>,
}

/// A headed group of fields, or one such group for each element of a recorded array.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Section {
    /// Where the group's heading comes from; a layout never writes one.
    pub heading: Heading,
    /// A pointer to an array in the record, such as `/component`. When set, the section repeats
    /// once per element and each slot's pointer is relative to that element.
    #[serde(default)]
    pub repeat: Option<String>,
    /// The group's fields, in reading order.
    pub slots: Vec<Slot>,
}

/// Where a section's heading comes from: `"result"`, one of the fixed headings, or
/// `{"from_code": "component"}`, each repeated element's own code with a fixed heading to fall
/// back on. Neither holds text the layout wrote, so a heading cannot state a finding.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(untagged)]
pub enum Heading {
    /// The fixed heading, numbered from 1 for each element when the section repeats.
    Fixed(FixedHeading),
    /// Each element's own code; only a section that repeats has elements to read it from.
    FromCode(FromCode),
}

/// A repeated section's heading taken from each element's `code.coding[0].display`, else its
/// `code.text`, else the fixed heading numbered from 1.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FromCode {
    /// The heading an element without a code reads under, numbered from 1.
    pub from_code: FixedHeading,
}

/// The headings a layout may give a section, each with its fixed wording.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FixedHeading {
    /// Reads “Result”.
    Result,
    /// Reads “Results”.
    Results,
    /// Reads “Panel”.
    Panel,
    /// Reads “Component”.
    Component,
    /// Reads “Reference range”.
    ReferenceRange,
    /// Reads “Interpretation”.
    Interpretation,
    /// Reads “Problem”.
    Problem,
    /// Reads “Substance”.
    Substance,
    /// Reads “Reaction”.
    Reaction,
    /// Reads “Medication”.
    Medication,
    /// Reads “Dosage”.
    Dosage,
    /// Reads “Coding”.
    Coding,
    /// Reads “Status”.
    Status,
    /// Reads “Dates”.
    Dates,
    /// Reads “Notes”.
    Notes,
    /// Reads “Details”.
    Details,
}

impl FixedHeading {
    /// Every fixed heading, in the order the schema lists them.
    pub const ALL: [Self; 16] = [
        Self::Result,
        Self::Results,
        Self::Panel,
        Self::Component,
        Self::ReferenceRange,
        Self::Interpretation,
        Self::Problem,
        Self::Substance,
        Self::Reaction,
        Self::Medication,
        Self::Dosage,
        Self::Coding,
        Self::Status,
        Self::Dates,
        Self::Notes,
        Self::Details,
    ];

    /// The heading as the physician reads it.
    pub fn text(self) -> &'static str {
        match self {
            Self::Result => "Result",
            Self::Results => "Results",
            Self::Panel => "Panel",
            Self::Component => "Component",
            Self::ReferenceRange => "Reference range",
            Self::Interpretation => "Interpretation",
            Self::Problem => "Problem",
            Self::Substance => "Substance",
            Self::Reaction => "Reaction",
            Self::Medication => "Medication",
            Self::Dosage => "Dosage",
            Self::Coding => "Coding",
            Self::Status => "Status",
            Self::Dates => "Dates",
            Self::Notes => "Notes",
            Self::Details => "Details",
        }
    }
}

/// The single value at `pointer`, such as `/valueQuantity/value`, read as `role`. Its label is
/// never written by the layout: the role names it, or the record's own field name does.
#[derive(Debug, Clone, PartialEq, Eq, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Slot {
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

impl SlotRole {
    /// Every role, in the order the schema lists them.
    pub const ALL: [Self; 11] = [
        Self::Result,
        Self::Unit,
        Self::ReferenceRangeLow,
        Self::ReferenceRangeHigh,
        Self::ReferenceRangeText,
        Self::Interpretation,
        Self::Status,
        Self::Date,
        Self::Code,
        Self::Note,
        Self::Other,
    ];

    /// The fixed label a value bound in this role reads under, given the last member of its
    /// path. `None` for a role whose values read under the record's own name for the field, so a
    /// value bound as `other` can never read as a result or a range limit.
    pub fn fixed_label(self, last_member: Option<&str>) -> Option<&'static str> {
        match self {
            Self::Result => Some("Result"),
            Self::Unit if last_member == Some("code") => Some("Unit code"),
            Self::Unit => Some("Unit"),
            Self::ReferenceRangeLow => Some("Reference range low"),
            Self::ReferenceRangeHigh => Some("Reference range high"),
            Self::ReferenceRangeText => Some("Reference range"),
            Self::Interpretation => Some("Interpretation"),
            Self::Status | Self::Date | Self::Code | Self::Note | Self::Other => None,
        }
    }

    /// The role in words, as logs and refusals name it.
    pub fn name(self) -> &'static str {
        match self {
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
        }
    }
}

impl fmt::Display for SlotRole {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.name())
    }
}

/// Whether a record read under a layout must hold a slot's value.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Presence {
    /// The record is read for this value: a record without it does not fit the layout, unless
    /// the repeated element it belongs to records why in a `dataAbsentReason`.
    Required,
    /// Records often leave this value out; a record without it reads under the layout with the
    /// slot omitted, and nothing stands in its place.
    Optional,
}

impl SlotRole {
    /// Whether a record must hold this role's value. A result, code, status or date is what a
    /// record or element is read for; a unit, a reference range, an interpretation, a note and
    /// any other value are often left out.
    pub fn presence(self) -> Presence {
        match self {
            Self::Unit
            | Self::ReferenceRangeLow
            | Self::ReferenceRangeHigh
            | Self::ReferenceRangeText
            | Self::Interpretation
            | Self::Note
            | Self::Other => Presence::Optional,
            Self::Result | Self::Code | Self::Status | Self::Date => Presence::Required,
        }
    }
}

/// A layout's refusal of one record, and where in the layout it arose. Its fields keep what the
/// layout and record wrote for programmatic use; its `Display`, and that of every cause, names
/// only the category, the role and positions, since logs carry no clinical content.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Refusal {
    /// Where in the layout the fault is.
    pub at: Position,
    /// The role of the slot at fault, when the fault is in one slot.
    pub role: Option<SlotRole>,
    /// What the fault is.
    pub cause: LayoutRefusal,
}

impl fmt::Display for Refusal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "the layout is refused at {}", self.at)?;
        if let Some(role) = self.role {
            write!(f, " ({role} slot)")?;
        }
        Ok(())
    }
}

impl std::error::Error for Refusal {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.cause)
    }
}

/// A place in a layout, by zero-based positions only.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Position {
    /// The part of the layout.
    pub part: Part,
    /// The slot's position within that part, when one slot is at fault.
    pub slot: Option<usize>,
    /// The element of a repeated section the slot was read from.
    pub element: Option<usize>,
}

/// A part of a layout.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Part {
    /// The layout as a whole, against the record as a whole.
    Whole,
    /// The record's statuses.
    Statuses,
    /// The section at this position.
    Section(usize),
}

impl Part {
    /// The kind of part, a fixed name that logs can carry beside [`Position::section`].
    pub fn name(self) -> &'static str {
        match self {
            Self::Whole => "whole",
            Self::Statuses => "statuses",
            Self::Section(_) => "section",
        }
    }
}

impl Position {
    fn of(part: Part) -> Self {
        Self {
            part,
            slot: None,
            element: None,
        }
    }

    fn slot(part: Part, slot: usize) -> Self {
        Self {
            slot: Some(slot),
            ..Self::of(part)
        }
    }

    /// The section's position, when the fault is in a section.
    pub fn section(&self) -> Option<usize> {
        match self.part {
            Part::Section(index) => Some(index),
            Part::Whole | Part::Statuses => None,
        }
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.part {
            Part::Whole => f.write_str("the whole layout")?,
            Part::Statuses => f.write_str("statuses")?,
            Part::Section(index) => write!(f, "section {index}")?,
        }
        if let Some(slot) = self.slot {
            write!(f, ", slot {slot}")?;
        }
        if let Some(element) = self.element {
            write!(f, ", element {element}")?;
        }
        Ok(())
    }
}

/// An error and its causes, outermost first, as logs show them. Every cause's `Display` in this
/// crate's layout path is free of record and model text; a JSON error, whose text can quote the
/// record or the reply, reads as its kind and position only.
pub struct Loggable<'e>(pub &'e (dyn std::error::Error + 'static));

impl fmt::Display for Loggable<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut cause = Some(self.0);
        let mut first = true;
        while let Some(error) = cause {
            if !first {
                f.write_str(": ")?;
            }
            first = false;
            match error.downcast_ref::<serde_json::Error>() {
                Some(json) => write!(f, "{}", JsonFault(json))?,
                None => write!(f, "{error}")?,
            }
            cause = error.source();
        }
        Ok(())
    }
}

/// A JSON error by its kind and position, never its text.
pub struct JsonFault<'e>(pub &'e serde_json::Error);

impl fmt::Display for JsonFault<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:?} error at line {}, column {}",
            self.0.classify(),
            self.0.line(),
            self.0.column()
        )
    }
}

/// Why a layout cannot read a record. A refused layout is never shown, not even in part.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LayoutRefusal {
    /// The record is not a JSON object.
    NotStructured,
    /// A section takes its heading from an element's code, but does not repeat over elements.
    CodeHeadingWithoutRepeat,
    /// The record's type, as its source adapter gave it, is not the one the layout reads.
    WrongRecordType {
        /// The record type the layout reads.
        expected: String,
        /// The record's type, as its source adapter gave it.
        found: String,
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
    /// A slot binds part of the record's own narrative (`text`), which reads first as plain
    /// text and never as markup inside a layout.
    IntoNarrative {
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

impl LayoutRefusal {
    /// The refusal's category, a fixed name that logs can carry.
    pub fn category(&self) -> &'static str {
        match self {
            Self::NotStructured => "not_structured",
            Self::CodeHeadingWithoutRepeat => "code_heading_without_repeat",
            Self::WrongRecordType { .. } => "wrong_record_type",
            Self::MalformedPointer { .. } => "malformed_pointer",
            Self::Unresolved { .. } => "unresolved",
            Self::RepeatNotAnArray { .. } => "repeat_not_an_array",
            Self::MissingInElement { .. } => "missing_in_element",
            Self::NotALeaf { .. } => "not_a_leaf",
            Self::DuplicateBinding { .. } => "duplicate_binding",
            Self::IntoNarrative { .. } => "into_narrative",
            Self::RoleMismatch { .. } => "role_mismatch",
        }
    }
}

/// Names the fault without the pointers or record types it carries: those are written by the
/// model or the record, and can hold clinical text.
impl fmt::Display for LayoutRefusal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::NotStructured => "the record is not a structured object",
            Self::CodeHeadingWithoutRepeat => {
                "a section that does not repeat takes its heading from an element's code"
            }
            Self::WrongRecordType { .. } => "the record is not of the type the layout reads",
            Self::MalformedPointer { .. } => "a pointer is not an RFC 6901 JSON pointer",
            Self::Unresolved { .. } => "a pointer names nothing in this record",
            Self::RepeatNotAnArray { .. } => "a section repeats over something other than an array",
            Self::MissingInElement { .. } => {
                "an element lacks a value its slot requires and does not record why"
            }
            Self::NotALeaf { .. } => "a pointer names an object or array, not one recorded value",
            Self::DuplicateBinding { .. } => "a recorded value is bound more than once",
            Self::IntoNarrative { .. } => "a slot binds part of the record's own narrative",
            Self::RoleMismatch { .. } => "a slot's role does not fit the value it binds",
        })
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
    /// Optional slots outside repeated sections that found nothing in this record.
    omitted: Vec<Omission>,
}

#[derive(Debug)]
struct BoundSection<'r> {
    heading: String,
    bindings: Vec<Binding<'r>>,
}

/// The recorded value at `path`, bound in `role`. It carries no label: one is derived from the
/// role or the record's own name for the field when it renders.
#[derive(Debug)]
struct Binding<'r> {
    role: SlotRole,
    path: SourcePath,
    value: &'r Value,
}

impl Layout {
    /// Reads a layout as the layout store holds it. A layout that does not parse is refused
    /// whole, the same as one that does not validate.
    pub fn parse(stored: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(stored)
    }

    /// Checks the layout against `record`, of `record_type` as its source adapter gave it (the
    /// type a citation and a layout key carry, never a member of the record), then every role
    /// and pointer, refusing at the first fault.
    pub fn validate<'r>(
        &self,
        record_type: &str,
        record: &'r Value,
    ) -> Result<ValidatedLayout<'r>, Refusal> {
        let whole = |cause| Refusal {
            at: Position::of(Part::Whole),
            role: None,
            cause,
        };
        if !record.is_object() {
            return Err(whole(LayoutRefusal::NotStructured));
        }
        if record_type != self.record_type {
            return Err(whole(LayoutRefusal::WrongRecordType {
                expected: self.record_type.clone(),
                found: record_type.to_owned(),
            }));
        }
        let mut bound = BTreeSet::new();
        let mut omitted = Vec::new();
        let mut statuses = Vec::with_capacity(self.statuses.len());
        for (index, slot) in self.statuses.iter().enumerate() {
            let position = Position::slot(Part::Statuses, index);
            let at = in_slot(position, slot);
            if slot.role != SlotRole::Status {
                return Err(at(LayoutRefusal::RoleMismatch {
                    pointer: slot.pointer.clone(),
                    role: slot.role,
                    reason: RoleMismatchReason::StatusesHoldOnlyStatuses,
                }));
            }
            match bind(slot, record, &mut bound).map_err(at)? {
                Some(binding) => statuses.push(binding),
                None => omitted.push(Omission::of(position, slot)),
            }
        }
        let mut sections = Vec::with_capacity(self.sections.len());
        for (index, section) in self.sections.iter().enumerate() {
            let part = Part::Section(index);
            match (&section.repeat, section.heading) {
                (Some(repeat), _) => {
                    sections.extend(repeated(section, part, repeat, record, &mut bound)?);
                }
                (None, Heading::FromCode(_)) => {
                    return Err(Refusal {
                        at: Position::of(part),
                        role: None,
                        cause: LayoutRefusal::CodeHeadingWithoutRepeat,
                    });
                }
                (None, Heading::Fixed(heading)) => {
                    let mut bindings = Vec::with_capacity(section.slots.len());
                    for (at_slot, slot) in section.slots.iter().enumerate() {
                        let position = Position::slot(part, at_slot);
                        match bind(slot, record, &mut bound).map_err(in_slot(position, slot))? {
                            Some(binding) => bindings.push(binding),
                            None => omitted.push(Omission::of(position, slot)),
                        }
                    }
                    // A heading over nothing would read as if the record had something there.
                    if bindings.is_empty() && !section.slots.is_empty() {
                        continue;
                    }
                    sections.push(BoundSection {
                        heading: heading.text().to_owned(),
                        bindings,
                    });
                }
            }
        }
        Ok(ValidatedLayout {
            record,
            statuses,
            sections,
            omitted,
        })
    }

    /// Checks a newly written layout against the sample record it was written from. On top of
    /// [`Layout::validate`], every pointer outside a repeated section must name a value in the
    /// sample: an optional slot may find nothing in a later record of the type, but a layout
    /// written from this record has no reason to point at what it lacks.
    pub fn validate_sample<'r>(
        &self,
        record_type: &str,
        record: &'r Value,
    ) -> Result<ValidatedLayout<'r>, Refusal> {
        let validated = self.validate(record_type, record)?;
        match validated.omitted.first() {
            Some(omission) => Err(omission.clone().into()),
            None => Ok(validated),
        }
    }
}

/// An optional slot outside a repeated section whose pointer names nothing in the record, so
/// the layout reads the record without it.
#[derive(Debug, Clone)]
struct Omission {
    at: Position,
    role: SlotRole,
    pointer: String,
}

impl Omission {
    fn of(at: Position, slot: &Slot) -> Self {
        Self {
            at,
            role: slot.role,
            pointer: slot.pointer.clone(),
        }
    }
}

/// Where a sample record is concerned, an omission is a pointer that names nothing.
impl From<Omission> for Refusal {
    fn from(omission: Omission) -> Self {
        Self {
            at: omission.at,
            role: Some(omission.role),
            cause: LayoutRefusal::Unresolved {
                pointer: omission.pointer,
            },
        }
    }
}

/// Places a fault in `slot`, which sits `at` in the layout.
fn in_slot(at: Position, slot: &Slot) -> impl Fn(LayoutRefusal) -> Refusal {
    let role = slot.role;
    move |cause| Refusal {
        at,
        role: Some(role),
        cause,
    }
}

/// Binds a slot of a section that does not repeat, or of the record's statuses. `None` when the
/// record lacks a value the slot's role leaves optional; the slot is then omitted.
fn bind<'r>(
    slot: &Slot,
    record: &'r Value,
    bound: &mut BTreeSet<SourcePath>,
) -> Result<Option<Binding<'r>>, LayoutRefusal> {
    let tokens = checked(slot)?;
    let Some(found) = follow(record, &tokens, SourcePath::root(), Vec::new()) else {
        return match slot.role.presence() {
            Presence::Optional => Ok(None),
            Presence::Required => Err(LayoutRefusal::Unresolved {
                pointer: slot.pointer.clone(),
            }),
        };
    };
    place(slot, found, &slot.pointer, bound).map(Some)
}

/// Binds a repeated section once per element of the array at `repeat`, each slot's pointer read
/// from that element. An element may lack a slot's value only when the slot's role allows it or
/// the element records a `dataAbsentReason`.
fn repeated<'r>(
    section: &Section,
    part: Part,
    repeat: &str,
    record: &'r Value,
    bound: &mut BTreeSet<SourcePath>,
) -> Result<Vec<BoundSection<'r>>, Refusal> {
    let at_repeat = |cause| Refusal {
        at: Position::of(part),
        role: None,
        cause,
    };
    let repeat_tokens = tokens(repeat).map_err(|reason| {
        at_repeat(LayoutRefusal::MalformedPointer {
            pointer: repeat.to_owned(),
            reason,
        })
    })?;
    let array =
        follow(record, &repeat_tokens, SourcePath::root(), Vec::new()).ok_or_else(|| {
            at_repeat(LayoutRefusal::Unresolved {
                pointer: repeat.to_owned(),
            })
        })?;
    let Value::Array(elements) = array.value else {
        return Err(at_repeat(LayoutRefusal::RepeatNotAnArray {
            pointer: repeat.to_owned(),
        }));
    };
    // Pointers are checked even when the array is empty.
    let slots = section
        .slots
        .iter()
        .enumerate()
        .map(|(at_slot, slot)| {
            let tokens = checked(slot).map_err(in_slot(Position::slot(part, at_slot), slot))?;
            Ok((at_slot, slot, tokens))
        })
        .collect::<Result<Vec<_>, Refusal>>()?;
    let mut sections = Vec::with_capacity(elements.len());
    for (index, element) in elements.iter().enumerate() {
        let path = array.path.index(index);
        let recorded_absence = element.get("dataAbsentReason").is_some();
        let mut bindings = Vec::with_capacity(slots.len());
        for (at_slot, slot, tokens) in &slots {
            let at = in_slot(
                Position {
                    element: Some(index),
                    ..Position::slot(part, *at_slot)
                },
                slot,
            );
            match follow(element, tokens, path.clone(), array.members.clone()) {
                Some(found) => {
                    let absolute = format!("{repeat}/{index}{}", slot.pointer);
                    bindings.push(place(slot, found, &absolute, bound).map_err(at)?);
                }
                None if slot.role.presence() == Presence::Optional || recorded_absence => {}
                None => {
                    return Err(at(LayoutRefusal::MissingInElement {
                        pointer: slot.pointer.clone(),
                        index,
                    }));
                }
            }
        }
        sections.push(BoundSection {
            heading: element_heading(element, section.heading, index),
            bindings,
        });
    }
    Ok(sections)
}

/// A repeated section's heading for one element. A fixed heading reads numbered from 1; a code
/// heading reads the element's own coded display or code text, else its fixed heading numbered.
fn element_heading(element: &Value, heading: Heading, index: usize) -> String {
    let numbered = |fixed: FixedHeading| format!("{} {}", fixed.text(), index + 1);
    let fixed = match heading {
        Heading::Fixed(fixed) => return numbered(fixed),
        Heading::FromCode(FromCode { from_code }) => from_code,
    };
    let code = &element["code"];
    [&code["coding"][0]["display"], &code["text"]]
        .into_iter()
        .filter_map(Value::as_str)
        .find(|text| !text.trim().is_empty())
        .map_or_else(|| numbered(fixed), str::to_owned)
}

/// The slot's pointer tokens, once its pointer is well formed.
fn checked(slot: &Slot) -> Result<Vec<String>, LayoutRefusal> {
    tokens(&slot.pointer).map_err(|reason| LayoutRefusal::MalformedPointer {
        pointer: slot.pointer.clone(),
        reason,
    })
}

/// A value a pointer names, with what its role is checked against.
#[derive(Debug)]
struct Found<'r> {
    /// Its path from the record root.
    path: SourcePath,
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
    mut path: SourcePath,
    mut members: Vec<String>,
) -> Option<Found<'r>> {
    let mut value = start;
    let mut parent = start;
    for token in tokens {
        parent = value;
        value = match value {
            Value::Object(object) => {
                let member = object.get(token)?;
                path.push_member(token);
                members.push(token.clone());
                member
            }
            Value::Array(items) => {
                let index = index(token)?;
                let item = items.get(index)?;
                path.push_index(index);
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
    bound: &mut BTreeSet<SourcePath>,
) -> Result<Binding<'r>, LayoutRefusal> {
    let pointer = || pointer.to_owned();
    if found.value.is_object() || found.value.is_array() {
        return Err(LayoutRefusal::NotALeaf { pointer: pointer() });
    }
    // The narrative is the root `text` object; a record's plain top-level `text` string is not.
    if matches!(found.members.as_slice(), [first, _, ..] if first == "text") {
        return Err(LayoutRefusal::IntoNarrative { pointer: pointer() });
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
        role: slot.role,
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

/// Rearranges the record's general view under the layout. Bound values read under labels derived
/// from their roles or the record's own field names (see [`label`]), with their source paths. The
/// general statuses always stay, and a bound status joins them only when none of them holds it.
/// A general field outside the statuses that the layout binds part of gives way to its
/// unbound leaves, each under the general label extended by its own path; leaves already under
/// Technical details stay only there. Every other general field reads under Other fields. Dates,
/// attribution, readable source text and Technical details keep their general reading, so
/// nothing in the record is dropped.
pub(crate) fn render(
    layout: &ValidatedLayout<'_>,
    numbers: &NumberLiterals,
    view: &mut SourceInspection,
) {
    let nodes = source_path::nodes(layout.record);
    // A field carries its path in the readable form, which no two paths share.
    let path_of: BTreeMap<String, &SourcePath> = nodes
        .iter()
        .map(|(path, _)| (path.to_string(), path))
        .collect();
    // The general view's readable fields by path: the record's own names for its values.
    let general: Vec<(&SourcePath, &str)> = view
        .sections
        .iter()
        .chain(&view.record_details)
        .chain(&view.statuses)
        .chain(&view.dates)
        .chain(&view.attribution)
        .filter_map(|field| {
            let path = path_of.get(field.path.as_deref()?)?;
            Some((*path, field.label.as_str()))
        })
        .collect();
    let shown = |binding: &Binding| {
        located(
            &label(binding, &general),
            &numbers.recorded(binding.value, &binding.path),
            &binding.path,
        )
    };
    let placed_statuses: Vec<SourceField> = layout.statuses.iter().map(shown).collect();
    let placed_sections: Vec<SourceLayoutSection> = layout
        .sections
        .iter()
        .map(|section| SourceLayoutSection {
            heading: section.heading.clone(),
            fields: section.bindings.iter().map(shown).collect(),
        })
        .collect();
    let bound: BTreeSet<&SourcePath> = layout
        .statuses
        .iter()
        .chain(layout.sections.iter().flat_map(|section| &section.bindings))
        .map(|binding| &binding.path)
        .collect();
    let technical: BTreeSet<&str> = view
        .technical
        .fields
        .iter()
        .filter_map(|field| field.path.as_deref())
        .collect();
    let leaves: Vec<(&SourcePath, &Value)> = nodes
        .iter()
        .filter(|(_, value)| !value.is_object() && !value.is_array())
        .map(|(path, value)| (path, *value))
        .collect();
    // What remains of a general field once the layout has placed its bound leaves.
    let remaining = |field: SourceField| -> Vec<SourceField> {
        let Some(&path) = field.path.as_deref().and_then(|path| path_of.get(path)) else {
            return vec![field];
        };
        let under: Vec<_> = leaves
            .iter()
            .filter(|(leaf, _)| leaf.is_under(path))
            .collect();
        if !under.iter().any(|(leaf, _)| bound.contains(leaf)) {
            return vec![field];
        }
        under
            .into_iter()
            .filter(|(leaf, _)| {
                !bound.contains(leaf) && !technical.contains(leaf.to_string().as_str())
            })
            .map(|(leaf, value)| {
                let label = match leaf_words(leaf.below(path)) {
                    words if words.is_empty() => field.label.clone(),
                    words => format!("{} {words}", field.label),
                };
                located(&label, &numbers.recorded(value, leaf), leaf)
            })
            .collect()
    };
    // The general statuses are the record's status metadata, which the pane header and hover
    // preview read: they stay whole whatever the layout places. A layout's status slot adds
    // only a status no general status already holds, such as one a vendor record names.
    let general_status_paths: Vec<&SourcePath> = view
        .statuses
        .iter()
        .filter_map(|field| path_of.get(field.path.as_deref()?).copied())
        .collect();
    let added_statuses = layout
        .statuses
        .iter()
        .zip(placed_statuses)
        .filter(|(binding, _)| {
            !general_status_paths
                .iter()
                .any(|path| binding.path.is_under(path))
        })
        .map(|(_, field)| field);
    view.statuses.extend(added_statuses);
    let (readable_text, fields): (Vec<_>, Vec<_>) = std::mem::take(&mut view.sections)
        .into_iter()
        .partition(|field| field.path.is_none());
    view.sections = readable_text;
    view.other_fields = fields
        .into_iter()
        .chain(std::mem::take(&mut view.record_details))
        .flat_map(&remaining)
        .collect();
    view.layout_sections = placed_sections;
}

/// The label a bound value reads under. A role that names its value's part, such as a result or
/// a reference range limit, reads under that role's fixed label; any other role reads under the
/// record's own name for the field, from `general`, the general view's readable fields by path.
/// The layout writes no label, so a value bound as `other` can never read as a result.
fn label(binding: &Binding, general: &[(&SourcePath, &str)]) -> String {
    let last_member = binding
        .path
        .steps()
        .iter()
        .rev()
        .find_map(|step| match step {
            Step::Member(name) => Some(name.as_str()),
            Step::Index(_) => None,
        });
    match binding.role.fixed_label(last_member) {
        Some(fixed) => fixed.to_owned(),
        None => source_label(&binding.path, general),
    }
}

/// The record's own name for the leaf at `leaf`: the label of the nearest general field holding
/// it, extended by the leaf's path below that field. A concept's text or first coded display
/// reads as the concept's own label. A leaf no readable field holds reads as its path in words.
fn source_label(leaf: &SourcePath, general: &[(&SourcePath, &str)]) -> String {
    let nearest = general
        .iter()
        .filter(|(path, _)| leaf.is_under(path))
        .max_by_key(|(path, _)| path.steps().len());
    let Some((path, label)) = nearest else {
        let mut words = leaf_words(leaf.steps());
        if let Some(first) = words.get_mut(..1) {
            first.make_ascii_uppercase();
        }
        return words;
    };
    let below = leaf.below(path);
    let names_the_concept = match below {
        [] => true,
        [Step::Member(name)] => name == "text" || name == "display",
        [Step::Member(coding), Step::Index(0), Step::Member(display)] => {
            coding == "coding" && display == "display"
        }
        _ => false,
    };
    if names_the_concept {
        (*label).to_owned()
    } else {
        format!("{label} {}", leaf_words(below))
    }
}

/// The steps below a general field, such as `coding[0].system`, in words: "coding 1 system".
fn leaf_words(steps: &[Step]) -> String {
    steps
        .iter()
        .map(|step| match step {
            Step::Index(index) => (index + 1).to_string(),
            Step::Member(name) => humanize(name).to_lowercase(),
        })
        .collect::<Vec<_>>()
        .join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::source_inspection::tests::{
        assert_view_shows_every_leaf, at, citation, condition, general_citation, observation,
        readable, vendor_lab,
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
            "statuses":[{"role":"status","pointer":"/status"}],
            "sections":[
                {"heading":"result","slots":[
                    {"role":"code","pointer":"/code/text"},
                    {"role":"result","pointer":"/valueQuantity/value"},
                    {"role":"unit","pointer":"/valueQuantity/unit"},
                    {"role":"interpretation","pointer":"/interpretation/0/coding/0/display"}]},
                {"heading":"reference_range","slots":[
                    {"role":"reference_range_low","pointer":"/referenceRange/0/low/value"},
                    {"role":"reference_range_high","pointer":"/referenceRange/0/high/value"},
                    {"role":"reference_range_text","pointer":"/referenceRange/0/text"}]}]}))
    }

    /// A hand-written Condition layout: the problem and its note, then its coding.
    fn condition_layout() -> Layout {
        layout(json!({"record_type":"Condition",
            "statuses":[
                {"role":"status","pointer":"/clinicalStatus/coding/0/display"},
                {"role":"status","pointer":"/verificationStatus/coding/0/display"}],
            "sections":[
                {"heading":"problem","slots":[
                    {"role":"code","pointer":"/code/text"},
                    {"role":"note","pointer":"/note/0/text"}]},
                {"heading":"coding","slots":[
                    {"role":"code","pointer":"/code/coding/0/code"}]}]}))
    }

    /// The observation as retained, with its result written `7.20` as some sources write it.
    fn retained_observation() -> Citation {
        let mut cited = general_citation(&observation());
        let written = cited.content.replace("\"value\":7.2", "\"value\":7.20");
        assert_ne!(written, cited.content);
        cited.content = written;
        cited
    }

    /// The type a FHIR source adapter gives `record`: its `resourceType`.
    fn fhir_type(record: &Value) -> &str {
        record["resourceType"].as_str().unwrap_or_default()
    }

    fn refusal(layout: &Layout, record: &Value) -> LayoutRefusal {
        layout
            .validate(fhir_type(record), record)
            .unwrap_err()
            .cause
    }

    /// Clinical text a model or a record could write into a pointer, a type or a label.
    const CLINICAL: &str = "Potassium 5.9 mmol/L";

    #[test]
    fn a_refusal_names_its_place_and_logs_no_text_the_layout_or_record_wrote() {
        let record = observation();
        let in_pointer = format!("/{CLINICAL}");
        let mut typed = observation_layout();
        typed.record_type = CLINICAL.into();
        let mut from_code = observation_layout();
        from_code.sections[0].heading = Heading::FromCode(FromCode {
            from_code: FixedHeading::Result,
        });
        let mut status = observation_layout();
        status.statuses[0].pointer = in_pointer.clone();
        let mut keyed = record.clone();
        keyed[CLINICAL] = json!("recorded");
        let mut repeated = blood_pressure_layout();
        repeated.sections[1].slots[0].pointer = in_pointer.clone();
        let cases = [
            (
                typed,
                record.clone(),
                Position::of(Part::Whole),
                None,
                "wrong_record_type",
            ),
            (
                with_slot(&in_pointer),
                record.clone(),
                Position::slot(Part::Section(0), 0),
                Some(SlotRole::Code),
                "unresolved",
            ),
            (
                with_slot(CLINICAL),
                record.clone(),
                Position::slot(Part::Section(0), 0),
                Some(SlotRole::Code),
                "malformed_pointer",
            ),
            (
                from_code,
                record.clone(),
                Position::of(Part::Section(0)),
                None,
                "code_heading_without_repeat",
            ),
            (
                status,
                record.clone(),
                Position::slot(Part::Statuses, 0),
                Some(SlotRole::Status),
                "unresolved",
            ),
            (
                with_role(SlotRole::Date, &format!("/{}", CLINICAL.replace('/', "~1"))),
                keyed,
                Position::slot(Part::Section(0), 0),
                Some(SlotRole::Date),
                "role_mismatch",
            ),
            (
                repeated,
                blood_pressure(),
                Position {
                    element: Some(0),
                    ..Position::slot(Part::Section(1), 0)
                },
                Some(SlotRole::Result),
                "missing_in_element",
            ),
        ];
        for (layout, record, at, role, category) in cases {
            let refused = layout.validate(fhir_type(&record), &record).unwrap_err();
            assert_eq!(
                (refused.at, refused.role, refused.cause.category()),
                (at, role, category)
            );
            let logged = Loggable(&refused).to_string();
            assert!(!logged.contains("Potassium"), "{logged}");
            assert!(logged.contains(&at.to_string()), "{logged}");
            // The typed cause keeps what the layout or record wrote, for programmatic use; a
            // heading's cause has nothing written to keep.
            if refused.cause != LayoutRefusal::CodeHeadingWithoutRepeat {
                assert!(format!("{:?}", refused.cause).contains("Potassium"));
            }
        }
    }

    #[test]
    fn a_layout_that_does_not_parse_logs_its_fault_without_quoting_it() {
        let stored = format!(r#"{{"record_type":"Observation","{CLINICAL}":1,"sections":[]}}"#);
        let error = Layout::parse(&stored).unwrap_err();
        assert!(error.to_string().contains("Potassium"), "{error}");
        let logged = Loggable(&error).to_string();
        assert!(!logged.contains("Potassium"), "{logged}");
        assert!(logged.contains("line 1"), "{logged}");
    }

    /// Every field of every event in this test process, as text, so a test can search what a log
    /// line carries. It is the process's global subscriber: tracing caches each callsite's
    /// interest process-wide, so a subscriber scoped to one test thread misses events whose
    /// callsite another thread reached first.
    fn captured_logs() -> &'static std::sync::Mutex<Vec<String>> {
        static LINES: std::sync::Mutex<Vec<String>> = std::sync::Mutex::new(Vec::new());
        static INSTALLED: std::sync::Once = std::sync::Once::new();
        INSTALLED.call_once(|| {
            use tracing_subscriber::layer::SubscriberExt;
            let subscriber = tracing_subscriber::registry().with(Captured);
            tracing::subscriber::set_global_default(subscriber)
                .expect("no other lib test installs a global subscriber");
        });
        &LINES
    }

    struct Captured;

    impl<S: tracing::Subscriber> tracing_subscriber::Layer<S> for Captured {
        fn on_event(
            &self,
            event: &tracing::Event<'_>,
            _: tracing_subscriber::layer::Context<'_, S>,
        ) {
            struct Fields<'a>(&'a mut String);
            impl tracing::field::Visit for Fields<'_> {
                fn record_debug(
                    &mut self,
                    field: &tracing::field::Field,
                    value: &dyn std::fmt::Debug,
                ) {
                    self.0.push_str(&format!("{}={value:?} ", field.name()));
                }
            }
            let mut line = String::new();
            event.record(&mut Fields(&mut line));
            captured_logs().lock().unwrap().push(line);
        }
    }

    /// A stored layout that does not fit one record is logged with its category and place, not
    /// dropped silently, and the log quotes nothing the layout or the record wrote.
    #[test]
    fn a_stored_layout_refused_for_one_record_is_logged_without_its_text() {
        let logs = captured_logs();
        let mut cited = retained_observation();
        let source_id = format!("ehr-{}", uuid::Uuid::new_v4());
        cited.source_id = Some(source_id.clone());
        let planted = with_slot(&format!("/{CLINICAL}"));
        let view = inspection_with_layout(&cited, Some(&planted));
        assert_eq!(view, inspection(&cited));
        let lines: Vec<String> = logs
            .lock()
            .unwrap()
            .iter()
            .filter(|line| line.contains(&source_id))
            .cloned()
            .collect();
        let [line] = lines.as_slice() else {
            panic!("{lines:?}");
        };
        assert!(!line.contains("Potassium"), "{line}");
        for field in [
            "refusal=\"unresolved\"",
            "role=\"code\"",
            "part=\"section\"",
            "section=0",
            "slot=0",
        ] {
            assert!(line.contains(field), "{field} in {line}");
        }
    }

    /// The Observation layout with one slot of its first section replaced.
    fn with_slot(pointer: &str) -> Layout {
        with_role(SlotRole::Code, pointer)
    }

    /// The Observation layout with one slot of its first section replaced by one with `role`.
    fn with_role(role: SlotRole, pointer: &str) -> Layout {
        let mut changed = observation_layout();
        changed.sections[0].slots[0] = Slot {
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
                refusal(&with_role(role, pointer), &record),
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
            let mut layout = with_role(role, pointer);
            layout.sections[1].slots.clear();
            layout.sections[0].slots.truncate(1);
            assert!(
                layout
                    .validate(fhir_type(&observation), &observation)
                    .is_ok(),
                "{role} at {pointer}: {:?}",
                layout.validate(fhir_type(&observation), &observation).err()
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
                    heading: Heading::Fixed(FixedHeading::Details),
                    repeat: None,
                    slots: vec![Slot {
                        role,
                        pointer: pointer.into(),
                    }],
                }],
            };
            assert!(
                layout.validate(fhir_type(&condition), &condition).is_ok(),
                "{role} at {pointer}"
            );
        }
        let mut status = with_role(Status, "/status");
        status.statuses.clear();
        assert!(
            status
                .validate(fhir_type(&observation), &observation)
                .is_ok()
        );
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
        let typed_in = json!({"record_type":"Observation","sections":[{"heading":"result",
            "slots":[{"role":"result","pointer":"/valueQuantity/value",
            "value":"5.4"}]}]});
        assert!(serde_json::from_value::<Layout>(typed_in).is_err());
        let roleless = json!({"record_type":"Observation","sections":[{"heading":"result",
            "slots":[{"label":"Result","pointer":"/valueQuantity/value"}]}]});
        assert!(serde_json::from_value::<Layout>(roleless).is_err());
        let unknown_role = json!({"record_type":"Observation","sections":[{"heading":"result",
            "slots":[{"role":"headline","pointer":"/valueQuantity/value"}]}]});
        assert!(serde_json::from_value::<Layout>(unknown_role).is_err());
        let typed_section = json!({"record_type":"Observation","sections":[{"heading":"result",
            "text":"Normal","slots":[]}]});
        assert!(serde_json::from_value::<Layout>(typed_section).is_err());
    }

    /// A layout writes no wording: a slot's label and a section's heading text are refused
    /// when the layout is read, wherever the model puts them.
    #[test]
    fn a_layout_cannot_write_a_label_or_a_heading() {
        let read = |layout: Value| serde_json::from_value::<Layout>(layout);
        let fixed = json!({"record_type":"Observation","sections":[{"heading":"result",
            "slots":[{"role":"result","pointer":"/valueQuantity/value"}]}]});
        assert!(read(fixed).is_ok());
        let labelled = json!({"record_type":"Observation","sections":[{"heading":"result",
            "slots":[{"label":"Result: 99 mmol/L","role":"result","pointer":"/valueQuantity/value"}]}]});
        assert!(read(labelled).is_err());
        let status_labelled = json!({"record_type":"Observation","statuses":[{"label":"Final",
            "role":"status","pointer":"/status"}],"sections":[]});
        assert!(read(status_labelled).is_err());
        for heading in [
            json!("Potassium is normal at 4.0 mmol/L"),
            json!("Result"),
            json!({"text":"Result"}),
            json!({"from_code":"Potassium is normal"}),
            json!({"from_code":"component","text":"Potassium"}),
            json!({"fixed":"result"}),
        ] {
            let written = json!({"record_type":"Observation","sections":[{"heading":heading,
                "slots":[{"role":"result","pointer":"/valueQuantity/value"}]}]});
            assert!(read(written).is_err(), "{heading}");
        }
    }

    /// The schema the model fills offers exactly the headings and roles a layout can hold, in
    /// the same order, so the model is never offered a choice the reader refuses or denied one
    /// it accepts.
    #[test]
    fn the_schemas_headings_and_roles_are_the_layouts() {
        let schema: Value = serde_json::from_str(crate::layout_generation::LAYOUT_SCHEMA).unwrap();
        let listed = |pointer: &str| -> Vec<Value> {
            schema
                .pointer(pointer)
                .and_then(Value::as_array)
                .unwrap_or_else(|| panic!("the schema lists {pointer}"))
                .clone()
        };
        let headings: Vec<FixedHeading> = listed("/$defs/fixed_heading/enum")
            .into_iter()
            .map(|name| serde_json::from_value(name).unwrap())
            .collect();
        assert_eq!(headings, FixedHeading::ALL);
        let roles: Vec<SlotRole> = listed("/$defs/slot/properties/role/enum")
            .into_iter()
            .map(|name| serde_json::from_value(name).unwrap())
            .collect();
        assert_eq!(roles, SlotRole::ALL);
        // `ALL` holds every variant: each one's position is fixed by an exhaustive match.
        for (position, heading) in FixedHeading::ALL.into_iter().enumerate() {
            let expected = match heading {
                FixedHeading::Result => 0,
                FixedHeading::Results => 1,
                FixedHeading::Panel => 2,
                FixedHeading::Component => 3,
                FixedHeading::ReferenceRange => 4,
                FixedHeading::Interpretation => 5,
                FixedHeading::Problem => 6,
                FixedHeading::Substance => 7,
                FixedHeading::Reaction => 8,
                FixedHeading::Medication => 9,
                FixedHeading::Dosage => 10,
                FixedHeading::Coding => 11,
                FixedHeading::Status => 12,
                FixedHeading::Dates => 13,
                FixedHeading::Notes => 14,
                FixedHeading::Details => 15,
            };
            assert_eq!(position, expected, "{heading:?}");
        }
        for (position, role) in SlotRole::ALL.into_iter().enumerate() {
            let expected = match role {
                SlotRole::Result => 0,
                SlotRole::Unit => 1,
                SlotRole::ReferenceRangeLow => 2,
                SlotRole::ReferenceRangeHigh => 3,
                SlotRole::ReferenceRangeText => 4,
                SlotRole::Interpretation => 5,
                SlotRole::Status => 6,
                SlotRole::Date => 7,
                SlotRole::Code => 8,
                SlotRole::Note => 9,
                SlotRole::Other => 10,
            };
            assert_eq!(position, expected, "{role:?}");
        }
    }

    /// Labels come from roles, so a value bound under another role cannot borrow a role's
    /// label: the reviewer's range low bound as `other` reads under the record's own name.
    #[test]
    fn a_range_limit_bound_as_other_reads_under_its_source_name_not_as_the_result() {
        let cited = retained_observation();
        let mut planted = observation_layout();
        planted.sections[0].slots[1] = Slot {
            role: SlotRole::Other,
            pointer: "/referenceRange/0/low/value".into(),
        };
        planted.sections[1].slots.remove(0);
        let view = inspection_with_layout(&cited, Some(&planted));
        let low = view.layout_sections[0]
            .fields
            .iter()
            .find(|field| field.path.as_deref() == Some("referenceRange[0].low.value"))
            .unwrap_or_else(|| panic!("{:#?}", view.layout_sections));
        assert_eq!(low.label, "Reference range low value");
        assert_eq!(low.text, "4.0");
        let labels: Vec<&str> = view
            .layout_sections
            .iter()
            .flat_map(|section| &section.fields)
            .map(|field| field.label.as_str())
            .collect();
        assert!(!labels.contains(&"Result"), "{labels:?}");
        assert_shown_values_are_recorded(&cited, &view);
        assert_view_shows_every_leaf(&cited, &view);
    }

    /// Each role that names its value reads under its fixed label, and the others read under the
    /// record's own name for the field: a code under the general view's label for its concept.
    #[test]
    fn each_bound_value_reads_under_its_roles_label_or_its_source_name() {
        let cited = retained_observation();
        let roles = [
            (SlotRole::Result, "/valueQuantity/value", "Result"),
            (SlotRole::Unit, "/valueQuantity/unit", "Unit"),
            (SlotRole::Unit, "/valueQuantity/code", "Unit code"),
            (
                SlotRole::ReferenceRangeLow,
                "/referenceRange/0/low/value",
                "Reference range low",
            ),
            (
                SlotRole::ReferenceRangeHigh,
                "/referenceRange/0/high/value",
                "Reference range high",
            ),
            (
                SlotRole::ReferenceRangeText,
                "/referenceRange/0/text",
                "Reference range",
            ),
            (
                SlotRole::Interpretation,
                "/interpretation/0/coding/0/display",
                "Interpretation",
            ),
            (SlotRole::Code, "/code/text", "Statement in this record"),
            (
                SlotRole::Code,
                "/code/coding/0/code",
                "Statement in this record coding 1 code",
            ),
            (SlotRole::Date, "/effectiveDateTime", "Effective"),
            (SlotRole::Other, "/performer/0/display", "Performed by"),
            (SlotRole::Other, "/valueQuantity/system", "Value system"),
        ];
        for (role, pointer, expected) in roles {
            let mut layout = with_role(role, pointer);
            layout.statuses.clear();
            layout.sections[1].slots.clear();
            layout.sections[0].slots.truncate(1);
            let view = inspection_with_layout(&cited, Some(&layout));
            assert_eq!(
                view.layout_sections[0].fields[0].label, expected,
                "{role} at {pointer}"
            );
        }
        let condition = general_citation(&condition());
        let view = inspection_with_layout(&condition, Some(&condition_layout()));
        assert_eq!(view.statuses[0].label, "Clinical status");
        assert_eq!(view.layout_sections[0].fields[1].label, "Note");
    }

    #[test]
    fn each_fault_refuses_the_layout() {
        let record = observation();
        assert_eq!(
            refusal(&observation_layout(), &condition()),
            LayoutRefusal::WrongRecordType {
                expected: "Observation".into(),
                found: "Condition".into(),
            }
        );
        assert_eq!(
            refusal(&observation_layout(), &json!([record.clone()])),
            LayoutRefusal::NotStructured
        );
        assert_eq!(
            refusal(&with_slot("valueQuantity/value"), &record),
            LayoutRefusal::MalformedPointer {
                pointer: "valueQuantity/value".into(),
                reason: PointerError::MissingLeadingSlash,
            }
        );
        assert_eq!(
            refusal(&with_slot("/value~2"), &record),
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
                refusal(&with_slot(pointer), &record),
                LayoutRefusal::Unresolved {
                    pointer: pointer.into()
                },
            );
        }
        for pointer in ["/valueQuantity", "/referenceRange", ""] {
            assert_eq!(
                refusal(&with_slot(pointer), &record),
                LayoutRefusal::NotALeaf {
                    pointer: pointer.into()
                },
            );
        }
        assert_eq!(
            refusal(&with_slot("/status"), &record),
            LayoutRefusal::DuplicateBinding {
                pointer: "/status".into()
            }
        );
        let mut from_code = observation_layout();
        from_code.sections[1].heading = Heading::FromCode(FromCode {
            from_code: FixedHeading::ReferenceRange,
        });
        assert_eq!(
            refusal(&from_code, &record),
            LayoutRefusal::CodeHeadingWithoutRepeat
        );
        assert!(
            observation_layout()
                .validate(fhir_type(&record), &record)
                .is_ok()
        );
        assert!(
            condition_layout()
                .validate("Condition", &condition())
                .is_ok()
        );
    }

    #[test]
    fn an_escaped_pointer_reads_the_member_it_names() {
        let record = json!({"resourceType":"Record","a/b":{"c~d":"recorded"}});
        let escaped = layout(
            json!({"record_type":"Record","sections":[{"heading":"details",
            "slots":[{"role":"other","pointer":"/a~1b/c~0d"}]}]}),
        );
        let validated = escaped.validate(fhir_type(&record), &record).unwrap();
        assert_eq!(
            validated.sections[0].bindings[0].path.to_string(),
            "a/b.c~d"
        );
        assert_eq!(validated.sections[0].bindings[0].value, "recorded");
    }

    #[test]
    fn a_member_named_like_a_path_keeps_its_own_value() {
        let cited = general_citation(&json!({"resourceType":"Observation"}));
        let cited = Citation {
            content: r#"{"resourceType":"Observation","status":"final","valueQuantity":{"value":7.20,"unit":"%"},"valueQuantity.value":999}"#.into(),
            ..cited
        };
        let nested = "valueQuantity.value";
        let dotted = r#"["valueQuantity.value"]"#;
        let path_of = |fields: &[&SourceField], text: &str| {
            fields
                .iter()
                .filter(|field| field.text == text)
                .filter_map(|field| field.path.clone())
                .collect::<Vec<_>>()
        };

        let general = inspection(&cited);
        let shown = readable(&general);
        assert_eq!(path_of(&shown, "7.20 %"), ["valueQuantity"]);
        assert_eq!(path_of(&shown, "999"), [dotted]);
        assert_view_shows_every_leaf(&cited, &general);

        let result = layout(
            json!({"record_type":"Observation","sections":[{"heading":"result",
            "slots":[{"role":"result","pointer":"/valueQuantity/value"}]}]}),
        );
        let view = inspection_with_layout(&cited, Some(&result));
        assert_eq!(
            view.layout_sections[0].fields,
            [located("Result", "7.20", nested)]
        );
        let shown = readable(&view);
        assert_eq!(path_of(&shown, "999"), [dotted]);
        assert!(path_of(&shown, "%").contains(&"valueQuantity.unit".to_owned()));
        assert_shown_values_are_recorded(&cited, &view);
        assert_view_shows_every_leaf(&cited, &view);

        // The two are two values, so a layout may bind both.
        let both = layout(
            json!({"record_type":"Observation","sections":[{"heading":"result",
            "slots":[{"role":"result","pointer":"/valueQuantity/value"},
                     {"role":"other","pointer":"/valueQuantity.value"}]}]}),
        );
        let view = inspection_with_layout(&cited, Some(&both));
        assert_eq!(
            view.layout_sections[0].fields,
            [
                located("Result", "7.20", nested),
                located("Value quantity.value", "999", dotted)
            ]
        );
        assert_shown_values_are_recorded(&cited, &view);
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
            with_slot("/valueString"),
            with_role(SlotRole::Date, "/code/text"),
            with_slot("/valueQuantity"),
            condition_layout(),
        ];
        for cited in [&observation, &condition, &allergy, &unreadable] {
            for layout in &refused {
                let record = serde_json::from_str::<Value>(&cited.content);
                if record.is_ok_and(|record| layout.validate(&cited.record_type, &record).is_ok()) {
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
    fn a_layout_leaves_the_records_narrative_first_and_unbound() {
        let mut record = observation();
        record["text"] = json!({"status":"generated",
            "div":"<div xmlns=\"http://www.w3.org/1999/xhtml\"><p>A1c 7.2 %</p></div>"});
        let cited = general_citation(&record);
        let view = inspection_with_layout(&cited, Some(&observation_layout()));
        assert_eq!(
            view.sections[0],
            SourceField {
                label: "The record's own narrative".into(),
                text: "A1c 7.2 %".into(),
                path: None,
            }
        );
        assert!(!view.layout_sections.is_empty());
        assert!(
            view.layout_sections
                .iter()
                .flat_map(|section| &section.fields)
                .chain(&view.other_fields)
                .all(|field| field.text != "A1c 7.2 %")
        );
        // A slot pointing into the narrative is refused, so its markup never reads in a layout.
        for pointer in ["/text/div", "/text/status"] {
            let mut into_narrative = observation_layout();
            into_narrative.sections[0]
                .slots
                .push(serde_json::from_value(json!({"role":"other","pointer":pointer})).unwrap());
            assert_eq!(
                into_narrative
                    .validate("Observation", &record)
                    .map(|_| ())
                    .unwrap_err()
                    .cause,
                LayoutRefusal::IntoNarrative {
                    pointer: pointer.into()
                }
            );
        }
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
                        located("Statement in this record", "Hemoglobin A1c", "code.text"),
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
                        located("Reference range low", "4.0", "referenceRange[0].low.value"),
                        located(
                            "Reference range high",
                            "5.6",
                            "referenceRange[0].high.value"
                        ),
                        located("Reference range", "4.0-5.6 %", "referenceRange[0].text"),
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
        // The layout's status slots read values the general statuses already hold, so the
        // statuses stay exactly the general ones.
        assert_eq!(view.statuses, inspection(&cited).statuses);
        assert_eq!(
            view.statuses,
            vec![
                located("Clinical status", "Resolved", "clinicalStatus"),
                located("Verification status", "Confirmed", "verificationStatus"),
            ]
        );
        assert_eq!(
            view.layout_sections,
            vec![
                SourceLayoutSection {
                    heading: "Problem".into(),
                    fields: vec![
                        located(
                            "Statement in this record",
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
                    fields: vec![located(
                        "Statement in this record coding 1 code",
                        "E11.9",
                        "code.coding[0].code"
                    )],
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

    /// A layout for a vendor lab export, which names no FHIR `resourceType`: its source adapter
    /// gives it the type `LabResult`, and the layout reads one section per result.
    fn vendor_lab_layout(record_type: &str) -> Layout {
        layout(json!({"record_type":record_type,
            "statuses":[{"role":"status","pointer":"/status"}],
            "sections":[{"heading":"result","repeat":"/results","slots":[
                {"role":"code","pointer":"/test_name"},
                {"role":"result","pointer":"/value"}]}]}))
    }

    /// A record that is not FHIR is checked against the record type its source adapter gave it,
    /// the type its citation and layout key carry, so a matching layout reads it and a layout for
    /// another type is refused.
    #[test]
    fn a_record_that_is_not_fhir_reads_under_a_layout_for_its_adapters_type() {
        let cited = vendor_lab();
        assert_eq!(cited.record_type, "LabResult");
        let record: Value = serde_json::from_str(&cited.content).unwrap();
        assert!(record.get("resourceType").is_none());

        let matching = vendor_lab_layout("LabResult");
        let validated = matching.validate(&cited.record_type, &record).unwrap();
        let headings: Vec<&str> = validated
            .sections
            .iter()
            .map(|section| section.heading.as_str())
            .collect();
        assert_eq!(headings, ["Result 1", "Result 2", "Result 3"]);
        let view = inspection_with_layout(&cited, Some(&matching));
        assert_eq!(
            view.layout_sections[0].fields,
            [
                located("Results 1 test name", "Potassium", "results[0].test_name"),
                located("Result", "5.9", "results[0].value"),
            ]
        );
        assert_eq!(view.statuses, inspection(&cited).statuses);
        assert_shown_values_are_recorded(&cited, &view);
        assert_view_shows_every_leaf(&cited, &view);

        let mismatched = vendor_lab_layout("Observation");
        let refused = mismatched
            .validate(&cited.record_type, &record)
            .unwrap_err();
        assert_eq!(
            (refused.at, refused.cause),
            (
                Position::of(Part::Whole),
                LayoutRefusal::WrongRecordType {
                    expected: "Observation".into(),
                    found: "LabResult".into(),
                }
            )
        );
        assert_eq!(
            serde_json::to_value(inspection_with_layout(&cited, Some(&mismatched))).unwrap(),
            serde_json::to_value(inspection(&cited)).unwrap()
        );
        // The record's own members never decide its type: a FHIR-shaped member does not let a
        // FHIR layout read a record its adapter typed otherwise.
        let mut claims = record.clone();
        claims["resourceType"] = json!("Observation");
        assert!(mismatched.validate("LabResult", &claims).is_err());
    }

    /// The statuses are the record's status metadata, read by the pane header and the hover
    /// preview: a layout that shows the status only in a section still leaves it there.
    #[test]
    fn a_status_placed_in_a_section_stays_among_the_statuses() {
        let cited = retained_observation();
        let general = inspection(&cited);
        assert_eq!(
            general.statuses,
            [located("Result status", "final", "status")]
        );
        let mut in_section = observation_layout();
        in_section.statuses.clear();
        in_section.sections[0].slots.push(Slot {
            role: SlotRole::Status,
            pointer: "/status".into(),
        });
        let view = inspection_with_layout(&cited, Some(&in_section));
        assert_eq!(
            view.layout_sections[0].fields.last(),
            Some(&located("Result status", "final", "status"))
        );
        assert_eq!(view.statuses, general.statuses);
        // Bound among the statuses as well, it still reads once there.
        let view = inspection_with_layout(&cited, Some(&observation_layout()));
        assert_eq!(view.statuses, general.statuses);
        // Neither placement repeats it under Other fields.
        assert!(
            view.other_fields
                .iter()
                .all(|field| field.path.as_deref() != Some("status"))
        );
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
            "statuses":[{"role":"status","pointer":"/status"}],
            "sections":[
                {"heading":"panel","slots":[
                    {"role":"code","pointer":"/code/text"}]},
                {"heading":{"from_code":"component"},"repeat":"/component","slots":[
                    {"role":"result","pointer":"/valueQuantity/value"},
                    {"role":"unit","pointer":"/valueQuantity/unit"},
                    {"role":"unit","pointer":"/valueQuantity/code"},
                    {"role":"interpretation",
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
                    fields: vec![located(
                        "Statement in this record",
                        "Blood pressure",
                        "code.text"
                    )],
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
        let validated = blood_pressure_layout()
            .validate(fhir_type(&record), &record)
            .unwrap();
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
                fields: vec![located(
                    "Statement in this record",
                    "Blood pressure",
                    "code.text"
                )],
            }]
        );
        assert_view_shows_every_leaf(&cited, &view);
    }

    #[test]
    fn an_element_may_lack_a_value_whose_absence_is_ordinary() {
        let record = blood_pressure();
        assert!(record["component"][1].get("interpretation").is_none());
        let validated = blood_pressure_layout()
            .validate(fhir_type(&record), &record)
            .unwrap();
        let roles: Vec<SlotRole> = validated.sections[2]
            .bindings
            .iter()
            .map(|binding| binding.role)
            .collect();
        assert_eq!(roles, [SlotRole::Result, SlotRole::Unit, SlotRole::Unit]);
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
    fn whether_a_record_must_hold_a_value_is_its_roles() {
        let required: Vec<SlotRole> = SlotRole::ALL
            .into_iter()
            .filter(|role| role.presence() == Presence::Required)
            .collect();
        assert_eq!(
            required,
            [
                SlotRole::Result,
                SlotRole::Status,
                SlotRole::Date,
                SlotRole::Code
            ]
        );
    }

    #[test]
    fn a_record_without_a_unit_or_range_reads_under_the_layout_with_nothing_invented() {
        let mut record = observation();
        record["valueQuantity"]
            .as_object_mut()
            .unwrap()
            .remove("unit");
        record.as_object_mut().unwrap().remove("referenceRange");
        let cited = general_citation(&record);
        let view = inspection_with_layout(&cited, Some(&observation_layout()));
        // The unit slot is omitted, and the reference range section has nothing to head.
        assert_eq!(
            view.layout_sections,
            vec![SourceLayoutSection {
                heading: "Result".into(),
                fields: vec![
                    located("Statement in this record", "Hemoglobin A1c", "code.text"),
                    located("Result", "7.2", "valueQuantity.value"),
                    located(
                        "Interpretation",
                        "High",
                        "interpretation[0].coding[0].display"
                    ),
                ],
            }]
        );
        assert_eq!(
            view.statuses,
            vec![located("Result status", "final", "status")]
        );
        assert_shown_values_are_recorded(&cited, &view);
        assert_view_shows_every_leaf(&cited, &view);
        // A layout written from this record could not have pointed at what it lacks.
        let refused = observation_layout()
            .validate_sample(fhir_type(&record), &record)
            .unwrap_err();
        assert_eq!(
            (refused.at, refused.role, refused.cause),
            (
                Position::slot(Part::Section(0), 2),
                Some(SlotRole::Unit),
                LayoutRefusal::Unresolved {
                    pointer: "/valueQuantity/unit".into()
                }
            )
        );
    }

    #[test]
    fn a_record_without_its_result_or_status_still_refuses_the_layout() {
        let mut record = observation();
        record.as_object_mut().unwrap().remove("valueQuantity");
        assert_eq!(
            refusal(&observation_layout(), &record),
            LayoutRefusal::Unresolved {
                pointer: "/valueQuantity/value".into()
            }
        );
        let mut record = observation();
        record.as_object_mut().unwrap().remove("status");
        assert_eq!(
            refusal(&observation_layout(), &record),
            LayoutRefusal::Unresolved {
                pointer: "/status".into()
            }
        );
        // An optional value of the wrong kind is still refused.
        assert_eq!(
            refusal(&with_role(SlotRole::Unit, "/code/text"), &observation()),
            mismatch(
                SlotRole::Unit,
                "/code/text",
                RoleMismatchReason::NotAQuantityUnit
            )
        );
    }

    #[test]
    fn a_component_without_its_unit_reads_under_the_layout_with_the_unit_omitted() {
        let mut record = blood_pressure();
        record["component"][1]["valueQuantity"]
            .as_object_mut()
            .unwrap()
            .remove("unit");
        let cited = general_citation(&record);
        let view = inspection_with_layout(&cited, Some(&blood_pressure_layout()));
        let fields = |index: usize| -> Vec<(&str, &str)> {
            view.layout_sections[index]
                .fields
                .iter()
                .map(|field| (field.label.as_str(), field.text.as_str()))
                .collect()
        };
        assert_eq!(
            fields(1),
            [
                ("Result", "142"),
                ("Unit", "mmHg"),
                ("Unit code", "mm[Hg]"),
                ("Interpretation", "High")
            ]
        );
        assert_eq!(fields(2), [("Result", "81.5"), ("Unit code", "mm[Hg]")]);
        assert_shown_values_are_recorded(&cited, &view);
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
            role: SlotRole::Result,
            pointer: "/component/0/valueQuantity/value".into(),
        });
        assert_eq!(
            refusal(&twice, &record),
            LayoutRefusal::DuplicateBinding {
                pointer: "/component/0/valueQuantity/value".into()
            }
        );
        // An empty array still checks the slots' pointers.
        let mut empty = record.clone();
        empty["component"] = json!([]);
        let mut malformed = blood_pressure_layout();
        malformed.sections[1].slots[0].pointer = "valueQuantity/value".into();
        assert_eq!(
            refusal(&malformed, &empty),
            LayoutRefusal::MalformedPointer {
                pointer: "valueQuantity/value".into(),
                reason: PointerError::MissingLeadingSlash,
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
            let (path, value) = at(&record, path).unwrap_or_else(|| panic!("no {path}"));
            if !value.is_object() && !value.is_array() {
                assert_eq!(shown.text, numbers.recorded(value, &path), "{path}");
            }
        }
    }
}
