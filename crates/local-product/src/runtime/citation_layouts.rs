//! Durable per-`(source_id, record_type)` citation field layouts and structured
//! record rendering (`Citations` Layout 1–5, Code 12–14).
//!
//! When a structured record supplies no narrative text (`readable_parts` is
//! empty), the citation panel arranges its JSON fields using a validated saved
//! layout for `(source_id, record_type)` if one exists, otherwise falling back
//! to the general view (`every field under its name`). Layouts only arrange and
//! label fields: every rendered value is copied directly from the record,
//! unplaced record fields still appear under their source names (`Code 13`), and
//! `"not supplied"` appears only when the source explicitly marks a part absent
//! (`Code 14`).

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

use sea_orm::{
    ColumnTrait, ConnectionTrait, DbErr, EntityTrait, QueryFilter, Set, sea_query::OnConflict,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::{Store, conversion_error, error, now};

const MAX_KEY_CHARS: usize = 256;
const MAX_LABEL_CHARS: usize = 80;
const MAX_PATH_CHARS: usize = 160;
const MAX_PATH_SEGMENTS: usize = 8;
const MAX_SEGMENT_CHARS: usize = 64;

/// SeaORM table holding validated declarative citation layouts keyed by `(source_id, record_type)`.
pub(in crate::runtime) mod table {
    use sea_orm::entity::prelude::*;

    #[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
    #[sea_orm(table_name = "physician_runtime_citation_layouts")]
    pub struct Model {
        #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
        pub source_id: String,
        #[sea_orm(primary_key, auto_increment = false, column_type = "Text")]
        pub record_type: String,
        #[sea_orm(column_type = "JsonBinary")]
        pub layout: Json,
        pub created_at: i64,
        pub updated_at: i64,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}

    impl ActiveModelBehavior for ActiveModel {}
}

/// Validated composite key `(source_id, record_type)` identifying one layout slot.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub struct LayoutSourceKey {
    /// Source identifier owning the record schema (e.g., `"clinic-ehr"`).
    pub source_id: String,
    /// Source-declared record kind (e.g., `"Observation"`).
    pub record_type: String,
}

impl LayoutSourceKey {
    /// Validates and constructs a [`LayoutSourceKey`].
    pub fn new(source_id: &str, record_type: &str) -> Result<Self, CitationLayoutError> {
        Ok(Self {
            source_id: validate_key_part("source_id", source_id)?,
            record_type: validate_key_part("record_type", record_type)?,
        })
    }
}

/// Typed dot-separated path identifying a field inside a structured JSON record.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FieldPath(Vec<String>);

impl FieldPath {
    /// Parses and validates a dot-separated field path (`"valueQuantity.value"`).
    pub fn parse(raw: &str) -> Result<Self, CitationLayoutError> {
        let trimmed = raw.trim();
        if trimmed.is_empty()
            || trimmed.chars().count() > MAX_PATH_CHARS
            || trimmed.chars().any(char::is_whitespace)
            || trimmed.chars().any(char::is_control)
        {
            return Err(CitationLayoutError::InvalidFieldPath {
                path: raw.to_owned(),
                reason: "field path must be non-empty, bounded, and free of whitespace",
            });
        }
        let segments: Vec<String> = trimmed.split('.').map(str::to_owned).collect();
        if segments.len() > MAX_PATH_SEGMENTS
            || segments
                .iter()
                .any(|seg| seg.is_empty() || seg.chars().count() > MAX_SEGMENT_CHARS)
        {
            return Err(CitationLayoutError::InvalidFieldPath {
                path: raw.to_owned(),
                reason: "field path segments must be 1..=64 chars with no empty segments",
            });
        }
        Ok(Self(segments))
    }

    /// Returns the ordered path segments.
    pub fn segments(&self) -> &[String] {
        &self.0
    }

    /// Returns the canonical dot-separated representation.
    pub fn as_str(&self) -> String {
        self.0.join(".")
    }

    /// Returns true when `other` sits at or beneath `self`.
    fn covers(&self, other: &[String]) -> bool {
        other.len() >= self.0.len() && other[..self.0.len()] == self.0[..]
    }
}

impl fmt::Display for FieldPath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.as_str())
    }
}

impl Serialize for FieldPath {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.as_str())
    }
}

impl<'de> Deserialize<'de> for FieldPath {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let raw = String::deserialize(deserializer)?;
        Self::parse(&raw).map_err(serde::de::Error::custom)
    }
}

/// One placed field slot in a declarative layout. Never carries any value.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LayoutField {
    /// Field path inside the JSON record.
    pub path: FieldPath,
    /// Optional physician-readable field label; defaults to the source's field name.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
}

/// One visual group of placed fields under an optional structural section heading.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LayoutSection {
    /// Optional structural section heading (must name a category, never a clinical value).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub heading: Option<String>,
    /// Ordered field placements belonging to this section.
    pub fields: Vec<LayoutField>,
}

/// Declarative per-`(source_id, record_type)` arrangement of record fields.
///
/// Strictly forbids extra keys (`#[serde(deny_unknown_fields)]`) so a model or
/// caller cannot embed literal clinical values, defaults, or prose templates.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CitationLayoutSpec {
    /// Ordered layout sections.
    pub sections: Vec<LayoutSection>,
}

/// Saved layout metadata returned from Postgres read/write operations.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct SavedCitationLayout {
    /// `(source_id, record_type)` key this layout applies to.
    pub key: LayoutSourceKey,
    /// Validated declarative field layout.
    pub spec: CitationLayoutSpec,
    /// Database-clock Unix second when first saved.
    pub created_at: i64,
    /// Database-clock Unix second when last updated.
    pub updated_at: i64,
}

/// Errors returned when validating, generating, or persisting citation layouts.
#[derive(Debug, thiserror::Error)]
pub enum CitationLayoutError {
    /// Missing or invalid `(source_id, record_type)` identifier.
    #[error("invalid citation layout key `{field}`")]
    InvalidKey {
        /// Key component that failed validation.
        field: &'static str,
    },
    /// Layout JSON violated the strict schema (including extra keys carrying literal values).
    #[error("layout failed schema check (no literal values or extra keys permitted): {0}")]
    InvalidSchema(#[source] serde_json::Error),
    /// Layout contains zero sections or zero fields.
    #[error("layout must contain at least one section with at least one field")]
    EmptyLayout,
    /// A section contained zero fields.
    #[error("layout section {index} has no fields")]
    EmptySection {
        /// Zero-based index of the empty section.
        index: usize,
    },
    /// A field path was syntactically invalid.
    #[error("invalid layout field path `{path}`: {reason}")]
    InvalidFieldPath {
        /// Offending field path string.
        path: String,
        /// Reason the path was rejected.
        reason: &'static str,
    },
    /// Duplicate field path within the same layout.
    #[error("duplicate layout field path `{path}`")]
    DuplicateFieldPath {
        /// Repeated path string.
        path: String,
    },
    /// Heading or label violated structural-name rules (e.g., carried literal numbers or key-value prose).
    #[error("invalid layout structural label `{text}`: {reason}")]
    InvalidLabel {
        /// Offending heading or label text.
        text: String,
        /// Reason the label was rejected.
        reason: &'static str,
    },
    /// Candidate layout matched zero fields in the sample record.
    #[error("layout matches no renderable fields in the sample structured record")]
    NoMatchingFieldsInRecord,
    /// Cannot automatically build a layout from a non-object or empty record.
    #[error("cannot generate layout: {reason}")]
    CannotGenerateLayout {
        /// Reason layout generation could not proceed.
        reason: &'static str,
    },
    /// Postgres persistence error.
    #[error("citation layout storage failure: {0}")]
    Database(#[from] DbErr),
}

fn validate_key_part(field: &'static str, raw: &str) -> Result<String, CitationLayoutError> {
    let trimmed = raw.trim();
    if trimmed.is_empty()
        || trimmed.chars().count() > MAX_KEY_CHARS
        || trimmed.chars().any(char::is_control)
    {
        return Err(CitationLayoutError::InvalidKey { field });
    }
    Ok(trimmed.to_owned())
}

fn validate_structural_name(raw: &str) -> Result<String, CitationLayoutError> {
    let trimmed = raw.trim();
    if trimmed.is_empty() || trimmed.chars().count() > MAX_LABEL_CHARS {
        return Err(CitationLayoutError::InvalidLabel {
            text: raw.to_owned(),
            reason: "structural label must be 1..=80 characters",
        });
    }
    if trimmed
        .chars()
        .any(|ch| ch.is_control() || ch == '\n' || ch == '\r')
    {
        return Err(CitationLayoutError::InvalidLabel {
            text: raw.to_owned(),
            reason: "structural label must be single-line without control characters",
        });
    }
    if [':', '=', '{', '}', '[', ']', '<', '>']
        .iter()
        .any(|forbidden| trimmed.contains(*forbidden))
    {
        return Err(CitationLayoutError::InvalidLabel {
            text: raw.to_owned(),
            reason: "structural label cannot contain value delimiters or template braces",
        });
    }
    if contains_standalone_numeric_literal(trimmed) {
        return Err(CitationLayoutError::InvalidLabel {
            text: raw.to_owned(),
            reason: "structural label cannot contain standalone numeric literals or measurements",
        });
    }
    Ok(trimmed.to_owned())
}

fn contains_standalone_numeric_literal(text: &str) -> bool {
    text.split(|ch: char| ch.is_whitespace() || matches!(ch, ',' | ';' | '(' | ')' | '/'))
        .filter(|tok| !tok.is_empty())
        .any(|tok| {
            let stripped = tok.trim_start_matches(['+', '-']);
            !stripped.is_empty()
                && stripped.starts_with(|ch: char| ch.is_ascii_digit())
                && stripped
                    .chars()
                    .all(|ch| ch.is_ascii_digit() || ch == '.' || ch == '%')
        })
}

impl CitationLayoutSpec {
    /// Parses untrusted JSON strictly and validates structural + `Code 12` rules.
    pub fn parse_and_validate(
        raw: Value,
        sample_record: Option<&Value>,
    ) -> Result<Self, CitationLayoutError> {
        let mut spec: Self =
            serde_json::from_value(raw).map_err(CitationLayoutError::InvalidSchema)?;
        spec.validate(sample_record)?;
        Ok(spec)
    }

    /// Validates all layout invariants and normalizes structural headings/labels.
    /// When `sample_record` is `Some`, also verifies the record is a JSON object
    /// and at least one layout path matches a field in `sample_record`.
    pub fn validate(&mut self, sample_record: Option<&Value>) -> Result<(), CitationLayoutError> {
        if self.sections.is_empty() {
            return Err(CitationLayoutError::EmptyLayout);
        }
        let mut seen_paths = BTreeSet::new();
        let mut total_fields = 0usize;
        for (idx, section) in self.sections.iter_mut().enumerate() {
            if section.fields.is_empty() {
                return Err(CitationLayoutError::EmptySection { index: idx });
            }
            if let Some(heading) = section.heading.take() {
                section.heading = Some(validate_structural_name(&heading)?);
            }
            for field in &mut section.fields {
                // Re-parse path in case a caller constructed FieldPath directly.
                field.path = FieldPath::parse(&field.path.as_str())?;
                if !seen_paths.insert(field.path.as_str()) {
                    return Err(CitationLayoutError::DuplicateFieldPath {
                        path: field.path.as_str(),
                    });
                }
                if let Some(label) = field.label.take() {
                    field.label = Some(validate_structural_name(&label)?);
                }
                total_fields += 1;
            }
        }
        if total_fields == 0 {
            return Err(CitationLayoutError::EmptyLayout);
        }

        if let Some(record) = sample_record {
            let Some(obj) = record.as_object() else {
                return Err(CitationLayoutError::CannotGenerateLayout {
                    reason: "sample record is not a JSON object",
                });
            };
            if obj.is_empty() {
                return Err(CitationLayoutError::CannotGenerateLayout {
                    reason: "sample record has no fields",
                });
            }
            let matches_any = self.sections.iter().any(|section| {
                section
                    .fields
                    .iter()
                    .any(|field| resolve_path(record, &field.path).has_renderable_content())
            });
            if !matches_any {
                return Err(CitationLayoutError::NoMatchingFieldsInRecord);
            }
        }
        Ok(())
    }
}

/// Generates a deterministic validated [`CitationLayoutSpec`] from a structured
/// JSON record (`Layout 4, 5`), placing high-signal clinical fields first while
/// carrying zero literal values.
pub fn generate_layout_for_record(
    record: &Value,
) -> Result<CitationLayoutSpec, CitationLayoutError> {
    let Some(map) = record.as_object() else {
        return Err(CitationLayoutError::CannotGenerateLayout {
            reason: "record is not a structured JSON object",
        });
    };
    let mut keys: Vec<String> = map
        .keys()
        .filter(|key| {
            let is_shadow = key.starts_with('_');
            if is_shadow {
                return false;
            }
            let shadow = map.get(&format!("_{key}"));
            let value = map.get(*key);
            is_renderable_slot(key, value, shadow)
        })
        .cloned()
        .collect();
    if keys.is_empty() {
        return Err(CitationLayoutError::CannotGenerateLayout {
            reason: "structured record contains no renderable fields",
        });
    }
    keys.sort_by_key(|key| (clinical_priority_rank(key), key.to_ascii_lowercase()));

    let fields = keys
        .into_iter()
        .map(|key| {
            let path = FieldPath::parse(&key)?;
            Ok(LayoutField { path, label: None })
        })
        .collect::<Result<Vec<_>, CitationLayoutError>>()?;

    let mut spec = CitationLayoutSpec {
        sections: vec![LayoutSection {
            heading: None,
            fields,
        }],
    };
    spec.validate(Some(record))?;
    Ok(spec)
}

fn clinical_priority_rank(key: &str) -> u8 {
    match key.to_ascii_lowercase().as_str() {
        "title" | "name" | "test" | "code" | "display" => 0,
        "status" | "clinicalstatus" | "clinical_status" | "verificationstatus" => 1,
        "value" | "valuequantity" | "valuestring" | "result" | "finding" | "interpretation" => 2,
        "unit" | "units" | "referencerange" | "reference_range" | "flags" => 3,
        "effectivedatetime" | "effective_datetime" | "date" | "authoredon" | "recordeddate" => 4,
        "dosage" | "dosageinstruction" | "medicationcodeableconcept" => 5,
        _ => 10,
    }
}

fn collapse_whitespace(text: &str) -> String {
    text.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// Converts a raw JSON object key into the source field label used in general-view fallback.
pub(crate) fn humanize_json_key(key: &str) -> String {
    let replaced = key.replace(['_', '-'], " ");
    let collapsed = collapse_whitespace(&replaced);
    let mut chars = collapsed.chars();
    match chars.next() {
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
        None => String::new(),
    }
}

enum ResolvedField<'a> {
    /// Field exists with a non-null JSON value.
    Present(&'a Value),
    /// Source explicitly marked the part absent (e.g., FHIR `dataAbsentReason`, `{ "absent": true }`).
    ExplicitAbsent,
    /// Omitted from record or plain `null` without an explicit absent marker (`Code 14`).
    OmittedOrNull,
}

impl ResolvedField<'_> {
    fn has_renderable_content(&self) -> bool {
        match self {
            Self::ExplicitAbsent => true,
            Self::OmittedOrNull => false,
            Self::Present(val) => value_has_renderable_content(val),
        }
    }
}

fn value_has_renderable_content(value: &Value) -> bool {
    match value {
        Value::Null => false,
        Value::Bool(_) | Value::Number(_) => true,
        Value::String(s) => !s.trim().is_empty(),
        Value::Array(items) => items.iter().any(value_has_renderable_content),
        Value::Object(map) => {
            if is_explicit_absent_object(map) {
                return true;
            }
            map.iter().any(|(k, v)| {
                if k.starts_with('_') {
                    return false;
                }
                let shadow = map.get(&format!("_{k}"));
                is_renderable_slot(k, Some(v), shadow)
            })
        }
    }
}

fn is_renderable_slot(key: &str, value: Option<&Value>, shadow: Option<&Value>) -> bool {
    if is_data_absent_shadow(shadow) {
        return true;
    }
    let Some(val) = value else {
        return false;
    };
    if is_data_absent_key(key) && !val.is_null() {
        return true;
    }
    value_has_renderable_content(val)
}

fn is_data_absent_key(key: &str) -> bool {
    key.eq_ignore_ascii_case("dataAbsentReason") || key.eq_ignore_ascii_case("data_absent_reason")
}

fn is_data_absent_extension(item: &Value) -> bool {
    item.as_object()
        .and_then(|obj| obj.get("url"))
        .and_then(Value::as_str)
        .is_some_and(|url| url.to_ascii_lowercase().contains("data-absent-reason"))
}

fn is_data_absent_shadow(shadow: Option<&Value>) -> bool {
    let Some(Value::Object(map)) = shadow else {
        return false;
    };
    map.get("extension")
        .and_then(Value::as_array)
        .is_some_and(|ext| ext.iter().any(is_data_absent_extension))
}

fn is_explicit_absent_object(map: &serde_json::Map<String, Value>) -> bool {
    if map.get("absent").and_then(Value::as_bool) == Some(true)
        || map.get("not_supplied").and_then(Value::as_bool) == Some(true)
    {
        return true;
    }
    if is_data_absent_shadow(Some(&Value::Object(map.clone()))) {
        let has_concrete_value = map.iter().any(|(k, v)| {
            !matches!(k.as_str(), "extension" | "id") && value_has_renderable_content(v)
        });
        if !has_concrete_value {
            return true;
        }
    }
    let has_reason = map.keys().any(|k| is_data_absent_key(k));
    if has_reason {
        let has_other_value = map.iter().any(|(k, v)| {
            !is_data_absent_key(k)
                && !matches!(k.as_str(), "id" | "extension")
                && value_has_renderable_content(v)
        });
        if !has_other_value {
            return true;
        }
    }
    false
}

fn resolve_path<'a>(record: &'a Value, path: &FieldPath) -> ResolvedField<'a> {
    let segments = path.segments();
    let mut cursor = record;
    for (idx, seg) in segments.iter().enumerate() {
        let Some(obj) = cursor.as_object() else {
            return ResolvedField::OmittedOrNull;
        };
        let is_last = idx + 1 == segments.len();
        let next = obj.get(seg);
        if is_last {
            let shadow = obj.get(&format!("_{seg}"));
            if is_data_absent_shadow(shadow)
                && next.is_none_or(|v| !value_has_renderable_content(v))
            {
                return ResolvedField::ExplicitAbsent;
            }
            let Some(val) = next else {
                return ResolvedField::OmittedOrNull;
            };
            if is_data_absent_key(seg) && !val.is_null() {
                return ResolvedField::ExplicitAbsent;
            }
            if let Value::Object(child_map) = val
                && is_explicit_absent_object(child_map)
            {
                return ResolvedField::ExplicitAbsent;
            }
            return if val.is_null() {
                ResolvedField::OmittedOrNull
            } else {
                ResolvedField::Present(val)
            };
        }
        let Some(child) = next else {
            return ResolvedField::OmittedOrNull;
        };
        cursor = child;
    }
    ResolvedField::OmittedOrNull
}

/// Renders JSON value lines into `out`, omitting keys covered by `covered` (`Code 13`)
/// and emitting `"not supplied"` only when the source explicitly marks absence (`Code 14`).
pub(crate) fn render_json_lines_filtered(
    prefix: Option<&str>,
    path_prefix: &[String],
    value: &Value,
    covered: &[FieldPath],
    out: &mut Vec<String>,
) {
    if !path_prefix.is_empty() && covered.iter().any(|p| p.covers(path_prefix)) {
        return;
    }
    match value {
        Value::Null => {}
        Value::Bool(flag) => {
            let word = if *flag { "Yes" } else { "No" };
            match prefix {
                Some(key) if !key.is_empty() => out.push(format!("{key}: {word}")),
                _ => out.push(word.to_owned()),
            }
        }
        Value::Number(num) => match prefix {
            Some(key) if !key.is_empty() => out.push(format!("{key}: {num}")),
            _ => out.push(num.to_string()),
        },
        Value::String(text) => {
            let trimmed = text.trim();
            if !trimmed.is_empty() {
                match prefix {
                    Some(key) if !key.is_empty() => out.push(format!("{key}: {trimmed}")),
                    _ => out.push(trimmed.to_owned()),
                }
            }
        }
        Value::Array(items) => {
            let all_scalar = items
                .iter()
                .all(|item| matches!(item, Value::String(_) | Value::Number(_) | Value::Bool(_)));
            if all_scalar {
                let joined: Vec<String> = items
                    .iter()
                    .filter_map(|item| match item {
                        Value::String(s) if !s.trim().is_empty() => Some(s.trim().to_owned()),
                        Value::Number(n) => Some(n.to_string()),
                        Value::Bool(b) => Some(if *b { "Yes" } else { "No" }.to_owned()),
                        _ => None,
                    })
                    .collect();
                if !joined.is_empty() {
                    let list = joined.join(", ");
                    match prefix {
                        Some(key) if !key.is_empty() => out.push(format!("{key}: {list}")),
                        _ => out.push(list),
                    }
                }
            } else {
                for item in items {
                    render_json_lines_filtered(prefix, path_prefix, item, covered, out);
                }
            }
        }
        Value::Object(map) => {
            if is_explicit_absent_object(map) {
                match prefix {
                    Some(key) if !key.is_empty() => out.push(format!("{key}: not supplied")),
                    _ => out.push("not supplied".to_owned()),
                }
                return;
            }
            for (key, nested) in map {
                if let Some(primary) = key.strip_prefix('_') {
                    // FHIR `_field` primitive shadow keys fold into their owning field below.
                    if is_data_absent_shadow(Some(nested))
                        && !covered_name(path_prefix, primary, covered)
                    {
                        let primary_has_value =
                            map.get(primary).is_some_and(value_has_renderable_content);
                        if !primary_has_value && !map.contains_key(primary) {
                            let label = humanize_json_key(primary);
                            let full_key = join_label(prefix, &label);
                            out.push(format!("{full_key}: not supplied"));
                        }
                    }
                    continue;
                }
                let label = humanize_json_key(key);
                let full_key = join_label(prefix, &label);
                let mut child_path = path_prefix.to_vec();
                child_path.push(key.clone());
                if covered.iter().any(|p| p.covers(&child_path)) {
                    continue;
                }
                let shadow = map.get(&format!("_{key}"));
                if is_data_absent_shadow(shadow) && !value_has_renderable_content(nested) {
                    out.push(format!("{full_key}: not supplied"));
                    continue;
                }
                if is_data_absent_key(key) && !nested.is_null() {
                    out.push(format!("{full_key}: not supplied"));
                    continue;
                }
                render_json_lines_filtered(Some(&full_key), &child_path, nested, covered, out);
            }
        }
    }
}

fn covered_name(path_prefix: &[String], key: &str, covered: &[FieldPath]) -> bool {
    let mut candidate = path_prefix.to_vec();
    candidate.push(key.to_owned());
    covered.iter().any(|p| p.covers(&candidate))
}

fn join_label(prefix: Option<&str>, label: &str) -> String {
    match prefix {
        Some(parent) if !parent.is_empty() && !label.is_empty() => format!("{parent} · {label}"),
        Some(parent) if !parent.is_empty() => parent.to_owned(),
        _ => label.to_owned(),
    }
}

/// Formats a structured JSON record using `layout` when supplied (`Layout 2`),
/// falling back to the general view (`Layout 3`) when `layout` is `None`.
/// Guarantees `Code 12` (values strictly copied from `record`), `Code 13`
/// (unplaced fields still shown under their source names), and `Code 14`
/// (`"not supplied"` only when the source explicitly marks absence).
pub fn render_structured_record(record: &Value, layout: Option<&CitationLayoutSpec>) -> String {
    let Some(spec) = layout else {
        let mut lines = Vec::new();
        render_json_lines_filtered(None, &[], record, &[], &mut lines);
        return lines.join("\n");
    };

    let mut covered: Vec<FieldPath> = Vec::new();
    let mut blocks: Vec<String> = Vec::new();

    for section in &spec.sections {
        let mut section_lines: Vec<String> = Vec::new();
        for field in &section.fields {
            let label = field.label.clone().unwrap_or_else(|| {
                let last = field.path.segments().last().map_or("", String::as_str);
                humanize_json_key(last)
            });
            match resolve_path(record, &field.path) {
                ResolvedField::OmittedOrNull => {
                    // Code 14: omitted or plain null keys never emit "not supplied".
                }
                ResolvedField::ExplicitAbsent => {
                    covered.push(field.path.clone());
                    section_lines.push(format!("{label}: not supplied"));
                }
                ResolvedField::Present(val) => {
                    let before = section_lines.len();
                    render_json_lines_filtered(Some(&label), &[], val, &[], &mut section_lines);
                    if section_lines.len() > before {
                        covered.push(field.path.clone());
                    }
                }
            }
        }
        if !section_lines.is_empty() {
            let mut block = String::new();
            if let Some(heading) = &section.heading {
                block.push_str(heading);
                block.push('\n');
            }
            block.push_str(&section_lines.join("\n"));
            blocks.push(block);
        }
    }

    // Code 13: append every field in `record` not placed by `spec` under its source name.
    let mut unplaced_lines = Vec::new();
    render_json_lines_filtered(None, &[], record, &covered, &mut unplaced_lines);
    if !unplaced_lines.is_empty() {
        blocks.push(unplaced_lines.join("\n"));
    }

    blocks.join("\n\n")
}

/// Loads saved layouts for a set of `(source_id, record_type)` keys inside an
/// open transaction (`Layout 2`).
pub(in crate::runtime) async fn load_citation_layouts_in_tx(
    tx: &impl ConnectionTrait,
    keys: &[LayoutSourceKey],
) -> Result<BTreeMap<LayoutSourceKey, CitationLayoutSpec>, DbErr> {
    if keys.is_empty() {
        return Ok(BTreeMap::new());
    }
    let source_ids: Vec<String> = keys
        .iter()
        .map(|k| k.source_id.clone())
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect();
    let record_types: Vec<String> = keys
        .iter()
        .map(|k| k.record_type.clone())
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect();

    let rows = table::Entity::find()
        .filter(table::Column::SourceId.is_in(source_ids))
        .filter(table::Column::RecordType.is_in(record_types))
        .all(tx)
        .await?;

    let wanted: BTreeSet<&LayoutSourceKey> = keys.iter().collect();
    let mut out = BTreeMap::new();
    for row in rows {
        let key = LayoutSourceKey {
            source_id: row.source_id,
            record_type: row.record_type,
        };
        if !wanted.contains(&key) {
            continue;
        }
        let mut spec: CitationLayoutSpec = serde_json::from_value(row.layout).map_err(
            conversion_error("citation layout JSON", "CitationLayoutSpec"),
        )?;
        spec.validate(None)
            .map_err(|err| error(&format!("corrupt stored citation layout: {err}")))?;
        out.insert(key, spec);
    }
    Ok(out)
}

impl Store {
    /// Reads the saved citation layout for `(source_id, record_type)`, if one exists.
    pub async fn citation_layout(
        &self,
        source_id: &str,
        record_type: &str,
    ) -> Result<Option<SavedCitationLayout>, CitationLayoutError> {
        let key = LayoutSourceKey::new(source_id, record_type)?;
        let Some(row) = table::Entity::find_by_id((key.source_id.clone(), key.record_type.clone()))
            .one(&self.db)
            .await?
        else {
            return Ok(None);
        };
        let mut spec: CitationLayoutSpec = serde_json::from_value(row.layout).map_err(
            conversion_error("citation layout JSON", "CitationLayoutSpec"),
        )?;
        spec.validate(None)?;
        Ok(Some(SavedCitationLayout {
            key,
            spec,
            created_at: row.created_at,
            updated_at: row.updated_at,
        }))
    }

    /// Reads saved citation layouts for multiple `(source_id, record_type)` keys in one query.
    pub async fn citation_layouts(
        &self,
        keys: &[LayoutSourceKey],
    ) -> Result<BTreeMap<LayoutSourceKey, CitationLayoutSpec>, DbErr> {
        load_citation_layouts_in_tx(&self.db, keys).await
    }

    /// Reads saved citation layouts matching a batch of [`crate::physician_sources::SourceRevision`]s.
    pub async fn citation_layouts_for_revisions<'a>(
        &self,
        revisions: impl IntoIterator<Item = &'a crate::physician_sources::SourceRevision>,
    ) -> Result<BTreeMap<LayoutSourceKey, CitationLayoutSpec>, DbErr> {
        let mut keys: Vec<LayoutSourceKey> = Vec::new();
        for rev in revisions {
            if let Ok(key) = LayoutSourceKey::new(&rev.source_id, &rev.record_type) {
                keys.push(key);
            }
        }
        self.citation_layouts(&keys).await
    }

    /// Validates an untrusted JSON layout strictly (`Layout 4, 5`, `Code 12`) and persists it
    /// in Postgres for `(source_id, record_type)`. If validation fails, saves nothing.
    pub async fn save_validated_citation_layout(
        &self,
        source_id: &str,
        record_type: &str,
        raw_layout: Value,
        sample_record: Option<&Value>,
    ) -> Result<SavedCitationLayout, CitationLayoutError> {
        let key = LayoutSourceKey::new(source_id, record_type)?;
        let spec = CitationLayoutSpec::parse_and_validate(raw_layout, sample_record)?;
        self.upsert_citation_layout(&key, &spec).await
    }

    /// Generates (or validates a candidate) layout for `(source_id, record_type)` when
    /// none is saved yet (`Layout 4, 5`). Returns `Ok(Some(saved))` when saved cleanly,
    /// or `Ok(None)` when generation/validation fails (saving nothing so callers use
    /// the general view).
    pub async fn ensure_citation_layout_for_record(
        &self,
        source_id: &str,
        record_type: &str,
        record: &Value,
        candidate_layout: Option<Value>,
    ) -> Result<Option<SavedCitationLayout>, CitationLayoutError> {
        let key = LayoutSourceKey::new(source_id, record_type)?;
        if let Some(existing) = self
            .citation_layout(&key.source_id, &key.record_type)
            .await?
        {
            return Ok(Some(existing));
        }
        let spec_result = match candidate_layout {
            Some(raw) => CitationLayoutSpec::parse_and_validate(raw, Some(record)),
            None => generate_layout_for_record(record),
        };
        let Ok(spec) = spec_result else {
            return Ok(None);
        };
        let saved = self.upsert_citation_layout(&key, &spec).await?;
        Ok(Some(saved))
    }

    async fn upsert_citation_layout(
        &self,
        key: &LayoutSourceKey,
        spec: &CitationLayoutSpec,
    ) -> Result<SavedCitationLayout, CitationLayoutError> {
        let clock = now(&self.db).await?;
        let layout_json =
            serde_json::to_value(spec).map_err(conversion_error("CitationLayoutSpec", "json"))?;
        table::Entity::insert(table::ActiveModel {
            source_id: Set(key.source_id.clone()),
            record_type: Set(key.record_type.clone()),
            layout: Set(layout_json),
            created_at: Set(clock),
            updated_at: Set(clock),
        })
        .on_conflict(
            OnConflict::columns([table::Column::SourceId, table::Column::RecordType])
                .update_columns([table::Column::Layout, table::Column::UpdatedAt])
                .to_owned(),
        )
        .exec_without_returning(&self.db)
        .await?;
        let row = table::Entity::find_by_id((key.source_id.clone(), key.record_type.clone()))
            .one(&self.db)
            .await?
            .ok_or_else(|| error("saved citation layout missing immediately after upsert"))?;
        Ok(SavedCitationLayout {
            key: key.clone(),
            spec: spec.clone(),
            created_at: row.created_at,
            updated_at: row.updated_at,
        })
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    fn code12_rejects_literal_values_extra_keys_numeric_labels_and_unmatched_paths() {
        let sample = json!({
            "potassium_mmol_l": 5.4,
            "sodium_mmol_l": 139,
            "status": "final"
        });

        // Extra field carrying a literal value must fail schema check.
        let with_literal_key = json!({
            "sections": [{
                "fields": [{ "path": "potassium_mmol_l", "value": "5.4 mmol/L" }]
            }]
        });
        assert!(matches!(
            CitationLayoutSpec::parse_and_validate(with_literal_key, Some(&sample)),
            Err(CitationLayoutError::InvalidSchema(_))
        ));

        // Extra default/fallback field must fail schema check.
        let with_default_key = json!({
            "sections": [{
                "fields": [{ "path": "potassium_mmol_l", "default": "normal" }]
            }]
        });
        assert!(matches!(
            CitationLayoutSpec::parse_and_validate(with_default_key, Some(&sample)),
            Err(CitationLayoutError::InvalidSchema(_))
        ));

        // Label trying to inject numeric measurement literal must be rejected.
        let numeric_label = json!({
            "sections": [{
                "fields": [{ "path": "potassium_mmol_l", "label": "Potassium 5.4" }]
            }]
        });
        assert!(matches!(
            CitationLayoutSpec::parse_and_validate(numeric_label, Some(&sample)),
            Err(CitationLayoutError::InvalidLabel { .. })
        ));

        // Key-value colon injection in label must be rejected.
        let colon_label = json!({
            "sections": [{
                "fields": [{ "path": "potassium_mmol_l", "label": "Potassium: 9.9" }]
            }]
        });
        assert!(matches!(
            CitationLayoutSpec::parse_and_validate(colon_label, Some(&sample)),
            Err(CitationLayoutError::InvalidLabel { .. })
        ));

        // Unmatched paths must fail validation against the sample record.
        let wrong_paths = json!({
            "sections": [{
                "fields": [{ "path": "nonexistent.field", "label": "Missing" }]
            }]
        });
        assert!(matches!(
            CitationLayoutSpec::parse_and_validate(wrong_paths, Some(&sample)),
            Err(CitationLayoutError::NoMatchingFieldsInRecord)
        ));

        // Duplicate paths are rejected.
        let duplicate = json!({
            "sections": [{
                "fields": [
                    { "path": "potassium_mmol_l", "label": "Potassium" },
                    { "path": "potassium_mmol_l", "label": "K+" }
                ]
            }]
        });
        assert!(matches!(
            CitationLayoutSpec::parse_and_validate(duplicate, Some(&sample)),
            Err(CitationLayoutError::DuplicateFieldPath { .. })
        ));
    }

    #[test]
    fn code13_and_code14_keep_unplaced_fields_and_render_not_supplied_only_when_marked_absent() {
        let layout = CitationLayoutSpec::parse_and_validate(
            json!({
                "sections": [{
                    "heading": "Electrolytes",
                    "fields": [
                        { "path": "potassium_mmol_l", "label": "Potassium (mmol/L)" },
                        { "path": "omitted_comment", "label": "Specimen comment" },
                        { "path": "plain_null", "label": "Plain null" },
                        { "path": "chloride", "label": "Chloride" },
                        { "path": "bicarbonate", "label": "Bicarbonate" }
                    ]
                }]
            }),
            None,
        )
        .unwrap();

        let record = json!({
            "potassium_mmol_l": 4.8,
            "plain_null": null,
            "chloride": { "dataAbsentReason": { "coding": [{ "code": "specimen-hemolyzed" }] } },
            "bicarbonate": null,
            "_bicarbonate": {
                "extension": [{
                    "url": "http://hl7.org/fhir/StructureDefinition/data-absent-reason",
                    "valueCode": "unknown"
                }]
            },
            "specimen": { "source": "venous", "hemolyzed": false },
            "unplaced_flag": "Checked"
        });

        let rendered = render_structured_record(&record, Some(&layout));
        // Placed section preserves order, explicit absent markers, and skips omitted/plain-null keys.
        assert!(rendered.contains("Electrolytes\nPotassium (mmol/L): 4.8\nChloride: not supplied\nBicarbonate: not supplied"));
        assert!(!rendered.contains("Specimen comment"));
        assert!(!rendered.contains("Plain null"));
        // Unplaced fields are never dropped (Code 13).
        assert!(rendered.contains("Specimen · Hemolyzed: No"));
        assert!(rendered.contains("Specimen · Source: venous"));
        assert!(rendered.contains("Unplaced flag: Checked"));
    }

    #[test]
    fn general_view_and_generated_layout_preserve_values_and_clinical_labels() {
        let record = json!({
            "title": "Hemoglobin A1c",
            "status": "final",
            "value": 6.8,
            "unit": "%",
            "notes": null,
            "fasting": { "absent": true }
        });
        let generated = generate_layout_for_record(&record).unwrap();
        let arranged = render_structured_record(&record, Some(&generated));
        assert!(arranged.contains("Title: Hemoglobin A1c"));
        assert!(arranged.contains("Status: final"));
        assert!(arranged.contains("Value: 6.8"));
        assert!(arranged.contains("Unit: %"));
        assert!(arranged.contains("Fasting: not supplied"));
        assert!(!arranged.contains("Notes"));
    }
}
