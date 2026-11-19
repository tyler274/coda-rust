//! Declared adapter mapping tables and shared record filing.
//!
//! Source-aware extraction and simple declared tables share one retained
//! revision constructor. FHIR interprets its namespaced declarations through
//! [`file_fhir_record`]; table-based formats use [`file_json_record`]. Native
//! content and unavailable parts remain distinct from derived index metadata.

use base64::Engine as _;
use serde_json::{Value, json};

use super::{
    Bucket, DateRole, FailedPart, FiledDate, ReadablePart, RecordCode, RecordDate, RecordEnvelope,
    RecordVersion, SourceRecordKind, SourceRevision,
};

/// Category matching rule for a declared [`AdapterRow`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CategoryMatch {
    /// Matches regardless of whether the source record carries a category.
    Any,
    /// Matches when the extracted category equals one of these source tokens.
    OneOf(&'static [&'static str]),
    /// Matches when no earlier [`Self::OneOf`] row for the same kind matched.
    OtherOrNone,
}

/// Declarative extractor for scalar, concept, or name values in a JSON record.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ValueExtractor {
    /// Scalar string or number at a dot-separated path (such as `"status"` or `"meta.versionId"`).
    Scalar(&'static str),
    /// First code or string token from a concept or array at path (such as `"clinicalStatus"` or `"category"`).
    ConceptCode(&'static str),
    /// Human-readable text or display label from a concept or concept array at path.
    ConceptLabel(&'static str),
    /// Structured person name list at path (such as FHIR `HumanName` at `"name"`).
    HumanName(&'static str),
}

/// Rule for decoding text attachments alongside the untouched native record bytes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AttachmentRule {
    /// Base64-encoded `text/*` payload inside an array of attachment objects.
    Base64Text {
        /// Dot-separated path to the array holding attachment items.
        items_path: &'static str,
        /// Source label attached to each decoded [`ReadablePart`].
        label: &'static str,
        /// Dot-separated subpath within each item naming the MIME type.
        content_type_path: &'static str,
        /// Dot-separated subpath within each item holding the base64 bytes.
        data_path: &'static str,
    },
}

/// One declared mapping row from a source's record kind (and optional category)
/// into a shared chart [`Bucket`] and field extraction specification.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AdapterRow {
    /// Source-native record kind name (such as `"Observation"` or `"DocumentReference"`).
    pub kind: &'static str,
    /// Category match condition within `kind`.
    pub category: CategoryMatch,
    /// Destination chart bucket when this row matches.
    pub bucket: Bucket,
    /// Declared date field paths paired with their semantic roles.
    pub dates: &'static [(DateRole, &'static str)],
    /// Ordered status extractors; the first present value wins.
    pub status: &'static [ValueExtractor],
    /// Declared coded-concept field paths.
    pub codes: &'static [&'static str],
    /// Ordered display-title extractors; the first present value wins.
    pub title: &'static [ValueExtractor],
    /// Attachment decoding rules for readable text parts.
    pub attachments: &'static [AttachmentRule],
}

/// Fallback filing rule applied when a source record's kind has no matching [`AdapterRow`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct UnmappedRecordRule {
    /// Destination bucket for unmapped kinds (typically [`Bucket::Other`]).
    pub bucket: Bucket,
    /// Optional date paths extracted when present on unmapped kinds.
    pub dates: &'static [(DateRole, &'static str)],
    /// Optional status extractors checked on unmapped kinds.
    pub status: &'static [ValueExtractor],
    /// Optional code paths checked on unmapped kinds.
    pub codes: &'static [&'static str],
    /// Optional title extractors checked on unmapped kinds.
    pub title: &'static [ValueExtractor],
    /// Optional attachment rules checked on unmapped kinds.
    pub attachments: &'static [AttachmentRule],
}

/// Complete declarative mapping table for one source format.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AdapterTable {
    /// Default source identifier associated with this table.
    pub source_id: &'static str,
    /// Dot-separated path to the source's record kind field (such as `"resourceType"`).
    pub kind_field: &'static str,
    /// Dot-separated path to the source's native record ID field (such as `"id"`).
    pub id_field: &'static str,
    /// Dot-separated path to an explicit source version when the format carries one.
    pub version_field: Option<&'static str>,
    /// Extractors tried in order to read the source's category label when present.
    pub category_extractors: &'static [ValueExtractor],
    /// Ordered mapping rows evaluated top-to-bottom.
    pub rows: &'static [AdapterRow],
    /// Rule used when no row in `rows` matches a record's kind, so unmapped kinds never halt a pull.
    pub unmapped: UnmappedRecordRule,
}

/// Failure while filing a raw source record through an [`AdapterTable`].
#[derive(Debug, thiserror::Error)]
pub enum RecordFilingError {
    /// A supplied patient identity contradicts the acquisition scope.
    #[error("patient binding mismatch: expected {expected}, received {actual}")]
    PatientMismatch {
        /// Patient whose chart is being acquired.
        expected: String,
        /// Contradictory source identity.
        actual: String,
    },
    /// Raw payload is not valid JSON.
    #[error("invalid JSON source record: {0}")]
    InvalidJson(#[source] serde_json::Error),
    /// Required identity or kind field was absent or empty.
    #[error("record is missing required field `{0}`")]
    MissingField(&'static str),
    /// Attachment metadata contains neither inline bytes nor a location to read.
    #[error("{label} has no inline data or external URL")]
    UnavailableAttachment {
        /// Declared attachment label.
        label: &'static str,
    },
    /// Attachment carried a non-text media type that Coda cannot decode as text or document/image OCR.
    #[error("{label} is not supported: {content_type}")]
    UnsupportedAttachment {
        /// Declared attachment label.
        label: &'static str,
        /// Media type sent by the source.
        content_type: String,
    },
    /// Attachment binary payload could not be decoded or OCR'd cleanly.
    #[error("{label}: {source}")]
    AttachmentDecode {
        /// Declared attachment label.
        label: &'static str,
        /// Underlying attachment/document read failure.
        #[source]
        source: Box<crate::attachments::AttachmentReadFailure>,
    },
    /// Attachment pointed to an external URL without inline text bytes Coda can read.
    #[error("{label} has external attachment URL `{url}` without inline data")]
    ExternalAttachment {
        /// Declared attachment label.
        label: &'static str,
        /// External URL carried by the source attachment.
        url: String,
    },
    /// Attachment base64 payload could not be decoded.
    #[error("{label}: {source}")]
    InvalidBase64 {
        /// Declared attachment label.
        label: &'static str,
        /// Underlying base64 decode error.
        #[source]
        source: base64::DecodeError,
    },
    /// Decoded attachment bytes were not valid UTF-8.
    #[error("{label}: {source}")]
    InvalidUtf8 {
        /// Declared attachment label.
        label: &'static str,
        /// Underlying UTF-8 error.
        #[source]
        source: std::string::FromUtf8Error,
    },
    /// Resulting [`SourceRevision`] failed structural validation.
    #[error("{0}")]
    InvalidRevision(String),
}

const OBSERVATION_DATES: &[(DateRole, &str)] = &[
    (DateRole::Effective, "effectiveDateTime"),
    (DateRole::Start, "effectivePeriod.start"),
    (DateRole::End, "effectivePeriod.end"),
    (DateRole::Issued, "issued"),
];

const DOCUMENT_REFERENCE_DATES: &[(DateRole, &str)] = &[
    (DateRole::Recorded, "date"),
    (DateRole::Start, "context.period.start"),
    (DateRole::End, "context.period.end"),
];

const DOCUMENT_REFERENCE_ATTACHMENTS: &[AttachmentRule] = &[AttachmentRule::Base64Text {
    items_path: "content",
    label: "attachment",
    content_type_path: "attachment.contentType",
    data_path: "attachment.data",
}];

const DIAGNOSTIC_REPORT_ATTACHMENTS: &[AttachmentRule] = &[AttachmentRule::Base64Text {
    items_path: "presentedForm",
    label: "presentedForm",
    content_type_path: "contentType",
    data_path: "data",
}];

const STATUS_SCALAR: &[ValueExtractor] = &[ValueExtractor::Scalar("status")];
const CLINICAL_STATUS_CODE: &[ValueExtractor] = &[ValueExtractor::ConceptCode("clinicalStatus")];
const CODE_CONCEPT_LABEL: &[ValueExtractor] = &[ValueExtractor::ConceptLabel("code")];
const MEDICATION_CONCEPT_LABEL: &[ValueExtractor] =
    &[ValueExtractor::ConceptLabel("medicationCodeableConcept")];
const DOCUMENT_TITLE: &[ValueExtractor] = &[
    ValueExtractor::Scalar("description"),
    ValueExtractor::ConceptLabel("type"),
];

/// Declared mapping table for FHIR R4 resources across all sixteen chart buckets.
const FHIR_R4_TABLE: AdapterTable = AdapterTable {
    source_id: "fhir-r4",
    kind_field: "resourceType",
    id_field: "id",
    version_field: Some("meta.versionId"),
    category_extractors: &[ValueExtractor::ConceptCode("category")],
    rows: &[
        AdapterRow {
            kind: "Patient",
            category: CategoryMatch::Any,
            bucket: Bucket::Patient,
            dates: &[(DateRole::BirthDate, "birthDate")],
            status: &[],
            codes: &[],
            title: &[ValueExtractor::HumanName("name")],
            attachments: &[],
        },
        AdapterRow {
            kind: "Condition",
            category: CategoryMatch::Any,
            bucket: Bucket::Problems,
            dates: &[
                (DateRole::Onset, "onsetDateTime"),
                (DateRole::Recorded, "recordedDate"),
                (DateRole::Abatement, "abatementDateTime"),
            ],
            status: CLINICAL_STATUS_CODE,
            codes: &["code"],
            title: CODE_CONCEPT_LABEL,
            attachments: &[],
        },
        AdapterRow {
            kind: "MedicationRequest",
            category: CategoryMatch::Any,
            bucket: Bucket::Medications,
            dates: &[(DateRole::Authored, "authoredOn")],
            status: STATUS_SCALAR,
            codes: &["medicationCodeableConcept"],
            title: MEDICATION_CONCEPT_LABEL,
            attachments: &[],
        },
        AdapterRow {
            kind: "MedicationStatement",
            category: CategoryMatch::Any,
            bucket: Bucket::Medications,
            dates: &[
                (DateRole::Effective, "effectiveDateTime"),
                (DateRole::Start, "effectivePeriod.start"),
                (DateRole::End, "effectivePeriod.end"),
                (DateRole::Recorded, "dateAsserted"),
            ],
            status: STATUS_SCALAR,
            codes: &["medicationCodeableConcept"],
            title: MEDICATION_CONCEPT_LABEL,
            attachments: &[],
        },
        AdapterRow {
            kind: "MedicationDispense",
            category: CategoryMatch::Any,
            bucket: Bucket::Medications,
            dates: &[
                (DateRole::Occurred, "whenHandedOver"),
                (DateRole::Recorded, "whenPrepared"),
            ],
            status: STATUS_SCALAR,
            codes: &["medicationCodeableConcept"],
            title: MEDICATION_CONCEPT_LABEL,
            attachments: &[],
        },
        AdapterRow {
            kind: "MedicationAdministration",
            category: CategoryMatch::Any,
            bucket: Bucket::Medications,
            dates: &[
                (DateRole::Effective, "effectiveDateTime"),
                (DateRole::Start, "effectivePeriod.start"),
                (DateRole::End, "effectivePeriod.end"),
            ],
            status: STATUS_SCALAR,
            codes: &["medicationCodeableConcept"],
            title: MEDICATION_CONCEPT_LABEL,
            attachments: &[],
        },
        AdapterRow {
            kind: "AllergyIntolerance",
            category: CategoryMatch::Any,
            bucket: Bucket::Allergies,
            dates: &[
                (DateRole::Recorded, "recordedDate"),
                (DateRole::Onset, "onsetDateTime"),
            ],
            status: CLINICAL_STATUS_CODE,
            codes: &["code"],
            title: CODE_CONCEPT_LABEL,
            attachments: &[],
        },
        AdapterRow {
            kind: "DiagnosticReport",
            category: CategoryMatch::Any,
            bucket: Bucket::Results,
            dates: OBSERVATION_DATES,
            status: STATUS_SCALAR,
            codes: &["code"],
            title: CODE_CONCEPT_LABEL,
            attachments: DIAGNOSTIC_REPORT_ATTACHMENTS,
        },
        AdapterRow {
            kind: "Observation",
            category: CategoryMatch::OtherOrNone,
            bucket: Bucket::Other,
            dates: OBSERVATION_DATES,
            status: STATUS_SCALAR,
            codes: &["code"],
            title: CODE_CONCEPT_LABEL,
            attachments: &[],
        },
        AdapterRow {
            kind: "DocumentReference",
            category: CategoryMatch::OtherOrNone,
            bucket: Bucket::Documents,
            dates: DOCUMENT_REFERENCE_DATES,
            status: STATUS_SCALAR,
            codes: &["type"],
            title: DOCUMENT_TITLE,
            attachments: DOCUMENT_REFERENCE_ATTACHMENTS,
        },
        AdapterRow {
            kind: "Encounter",
            category: CategoryMatch::Any,
            bucket: Bucket::Visits,
            dates: &[
                (DateRole::Start, "period.start"),
                (DateRole::End, "period.end"),
            ],
            status: STATUS_SCALAR,
            codes: &["type", "serviceType"],
            title: &[
                ValueExtractor::ConceptLabel("type"),
                ValueExtractor::ConceptLabel("serviceType"),
            ],
            attachments: &[],
        },
        AdapterRow {
            kind: "Appointment",
            category: CategoryMatch::Any,
            bucket: Bucket::Visits,
            dates: &[
                (DateRole::Start, "start"),
                (DateRole::End, "end"),
                (DateRole::Recorded, "created"),
            ],
            status: STATUS_SCALAR,
            codes: &["appointmentType", "serviceType"],
            title: &[
                ValueExtractor::Scalar("description"),
                ValueExtractor::ConceptLabel("appointmentType"),
            ],
            attachments: &[],
        },
        AdapterRow {
            kind: "Procedure",
            category: CategoryMatch::Any,
            bucket: Bucket::Procedures,
            dates: &[
                (DateRole::Performed, "performedDateTime"),
                (DateRole::Start, "performedPeriod.start"),
                (DateRole::End, "performedPeriod.end"),
            ],
            status: STATUS_SCALAR,
            codes: &["code"],
            title: CODE_CONCEPT_LABEL,
            attachments: &[],
        },
        AdapterRow {
            kind: "Immunization",
            category: CategoryMatch::Any,
            bucket: Bucket::Immunizations,
            dates: &[
                (DateRole::Occurred, "occurrenceDateTime"),
                (DateRole::Recorded, "recorded"),
            ],
            status: STATUS_SCALAR,
            codes: &["vaccineCode"],
            title: &[ValueExtractor::ConceptLabel("vaccineCode")],
            attachments: &[],
        },
        AdapterRow {
            kind: "ServiceRequest",
            category: CategoryMatch::Any,
            bucket: Bucket::Orders,
            dates: &[
                (DateRole::Authored, "authoredOn"),
                (DateRole::Effective, "occurrenceDateTime"),
            ],
            status: STATUS_SCALAR,
            codes: &["code"],
            title: CODE_CONCEPT_LABEL,
            attachments: &[],
        },
        AdapterRow {
            kind: "FamilyMemberHistory",
            category: CategoryMatch::Any,
            bucket: Bucket::History,
            dates: &[(DateRole::Recorded, "date")],
            status: STATUS_SCALAR,
            codes: &["relationship"],
            title: &[ValueExtractor::ConceptLabel("relationship")],
            attachments: &[],
        },
        AdapterRow {
            kind: "QuestionnaireResponse",
            category: CategoryMatch::Any,
            bucket: Bucket::Forms,
            dates: &[(DateRole::Authored, "authored")],
            status: STATUS_SCALAR,
            codes: &[],
            title: &[ValueExtractor::Scalar("questionnaire")],
            attachments: &[],
        },
        AdapterRow {
            kind: "Communication",
            category: CategoryMatch::Any,
            bucket: Bucket::Messages,
            dates: &[(DateRole::Sent, "sent"), (DateRole::Received, "received")],
            status: STATUS_SCALAR,
            codes: &["category"],
            title: &[ValueExtractor::ConceptLabel("category")],
            attachments: &[],
        },
    ],
    unmapped: UnmappedRecordRule {
        bucket: Bucket::Other,
        dates: &[],
        status: STATUS_SCALAR,
        codes: &["code"],
        title: &[
            ValueExtractor::Scalar("title"),
            ValueExtractor::Scalar("description"),
            ValueExtractor::ConceptLabel("code"),
        ],
        attachments: &[],
    },
};

/// Checks explicit FHIR patient identities against a patient-scoped acquisition.
/// Records without explicit patient references inherit that acquisition scope.
pub fn validate_fhir_patient_binding(
    value: &serde_json::Value,
    patient_id: &str,
) -> Result<(), RecordFilingError> {
    if value["resourceType"] == "Patient" {
        let actual = value["id"]
            .as_str()
            .filter(|id| !id.is_empty())
            .ok_or(RecordFilingError::MissingField("id"))?;
        return compare_patient(patient_id, actual);
    }
    let mut references = ["/subject/reference", "/patient/reference"]
        .into_iter()
        .filter_map(|pointer| value.pointer(pointer).and_then(Value::as_str))
        .collect::<Vec<_>>();
    if value["resourceType"] == "Appointment" {
        references.extend(
            value["participant"]
                .as_array()
                .into_iter()
                .flatten()
                .filter_map(|participant| {
                    participant
                        .pointer("/actor/reference")
                        .and_then(Value::as_str)
                }),
        );
    }
    for reference in references {
        // Absolute references retain the server in native content; a contradictory
        // Patient ID still cannot be admitted to this patient-scoped bank.
        let parsed = reqwest::Url::parse(reference).ok();
        let path = parsed.as_ref().map_or(reference, |url| url.path());
        let segments: Vec<_> = path.trim_matches('/').split('/').collect();
        if let Some(index) = segments.iter().position(|segment| *segment == "Patient")
            && let Some(actual) = segments.get(index + 1)
        {
            compare_patient(patient_id, actual)?;
        }
    }
    Ok(())
}

fn compare_patient(expected: &str, actual: &str) -> Result<(), RecordFilingError> {
    if actual != expected {
        return Err(RecordFilingError::PatientMismatch {
            expected: expected.to_owned(),
            actual: actual.to_owned(),
        });
    }
    Ok(())
}

/// Files FHIR R4 evidence using namespaced source declarations. Unknown and
/// contradictory classifications retain the whole record in Other.
pub fn file_fhir_record(
    workspace: &str,
    source_id: &str,
    raw_json: &str,
    native_link: Option<String>,
) -> Result<SourceRevision, RecordFilingError> {
    let resource: Value = serde_json::from_str(raw_json).map_err(RecordFilingError::InvalidJson)?;
    let classification = match resource.get("resourceType").and_then(Value::as_str) {
        Some("Observation") => Some(observation_classification(&resource)),
        Some("DocumentReference") => {
            let note = extract_all_codes(&resource, &["category"]).iter().any(|code|
                code.system.as_deref() == Some("http://hl7.org/fhir/us/core/CodeSystem/us-core-documentreference-category")
                && code.code.as_deref() == Some("clinical-note"));
            Some((
                if note {
                    Bucket::Notes
                } else {
                    Bucket::Documents
                },
                note.then(|| "clinical-note".to_owned()),
            ))
        }
        _ => None,
    };
    file_parsed_record(
        &FHIR_R4_TABLE,
        workspace,
        source_id,
        raw_json,
        native_link,
        &resource,
        classification,
    )
}

fn observation_classification(resource: &Value) -> (Bucket, Option<String>) {
    let categories = extract_all_codes(resource, &["category"]);
    let mut recognized = categories
        .iter()
        .filter(|code| {
            code.system.as_deref()
                == Some("http://terminology.hl7.org/CodeSystem/observation-category")
        })
        .filter_map(|code| match code.code.as_deref() {
            Some("laboratory" | "imaging") => Some((Bucket::Results, code.code.clone())),
            Some("vital-signs") => Some((Bucket::Vitals, code.code.clone())),
            Some("social-history") => Some((Bucket::History, code.code.clone())),
            _ => None,
        })
        .collect::<Vec<_>>();
    recognized.sort();
    if let Some((bucket, category)) = recognized.first() {
        return if recognized.iter().all(|(other, _)| other == bucket) {
            (*bucket, category.clone())
        } else {
            (Bucket::Other, None)
        };
    }
    // A source-local category needs its own mapping; code matching cannot silently
    // reinterpret unknown declarations. These exact identities apply without categories.
    if !categories.is_empty() {
        return (Bucket::Other, None);
    }
    let mut buckets = extract_all_codes(resource, &["code"])
        .iter()
        .filter(|code| code.system.as_deref() == Some("http://loinc.org"))
        .filter_map(|code| match code.code.as_deref() {
            Some(
                "9279-1" | "8867-4" | "8310-5" | "8302-2" | "9843-4" | "29463-7" | "39156-5"
                | "85353-1" | "85354-9" | "8480-6" | "8462-4",
            ) => Some(Bucket::Vitals),
            Some("2823-3") => Some(Bucket::Results),
            _ => None,
        })
        .collect::<Vec<_>>();
    buckets.sort();
    buckets.dedup();
    (
        if buckets.len() == 1 {
            buckets[0]
        } else {
            Bucket::Other
        },
        None,
    )
}

/// Files one raw JSON record from `source_id` into `workspace` using `table`,
/// preserving `raw_json` byte-for-byte in [`SourceRevision::content`].
pub fn file_json_record(
    table: &AdapterTable,
    workspace: &str,
    source_id: &str,
    raw_json: &str,
    native_link: Option<String>,
) -> Result<SourceRevision, RecordFilingError> {
    let resource: Value = serde_json::from_str(raw_json).map_err(RecordFilingError::InvalidJson)?;
    file_parsed_record(
        table,
        workspace,
        source_id,
        raw_json,
        native_link,
        &resource,
        None,
    )
}

fn file_parsed_record(
    table: &AdapterTable,
    workspace: &str,
    source_id: &str,
    raw_json: &str,
    native_link: Option<String>,
    resource: &Value,
    classification: Option<(Bucket, Option<String>)>,
) -> Result<SourceRevision, RecordFilingError> {
    let kind = lookup_path(resource, table.kind_field)
        .and_then(scalar_string)
        .ok_or(RecordFilingError::MissingField(table.kind_field))?;
    let native_id = lookup_path(resource, table.id_field)
        .and_then(scalar_string)
        .ok_or(RecordFilingError::MissingField(table.id_field))?;

    let categories = extract_all_values(resource, table.category_extractors);
    let (matched, category) = match_row(table, &kind, &categories);

    let filed_dates = extract_dates(resource, matched.dates);
    let status = first_extracted(resource, matched.status);
    let codes = extract_all_codes(resource, matched.codes);
    let title = first_extracted(resource, matched.title);

    let sha256 = crate::sha256_hex(raw_json.as_bytes());
    let raw_version = table
        .version_field
        .and_then(|path| lookup_path(resource, path))
        .and_then(scalar_string);
    let document_version = raw_version
        .as_deref()
        .and_then(|v| v.strip_prefix('v').unwrap_or(v).parse::<u64>().ok())
        .filter(|v| *v >= 1)
        .unwrap_or(1);
    let candidate_id = format!("{source_id}/{kind}/{native_id}");
    let authored_date = filed_dates.first().map(|d| d.value.as_str()).unwrap_or("");
    let attachment_ctx = AttachmentContext {
        workspace,
        source_id,
        candidate_id: &candidate_id,
        authored_date,
        record_title: title.as_deref().unwrap_or(&kind),
        document_version,
    };
    let (readable_parts, unavailable_parts, extra_provenance) =
        extract_attachments(resource, matched.attachments, &attachment_ctx);

    let version = match raw_version {
        Some(source_version) => RecordVersion::Source(source_version),
        None => RecordVersion::ContentHash(sha256.clone()),
    };

    let (bucket, category) = classification.unwrap_or((matched.bucket, category));
    let envelope = RecordEnvelope {
        bucket,
        source_kind: SourceRecordKind {
            kind: kind.clone(),
            category,
        },
        native_id,
        version,
        dates: filed_dates,
        status,
        codes,
        title,
        native_link,
        unavailable_parts,
    };
    file_record_with_provenance(
        workspace,
        source_id,
        raw_json,
        envelope,
        readable_parts,
        extra_provenance,
    )
}

/// Builds a retained revision from source-specific metadata and native content.
/// Identity depends on source family and native ID, never on classification.
pub(super) fn file_record(
    workspace: &str,
    source_id: &str,
    raw_json: &str,
    envelope: RecordEnvelope,
    readable_parts: Vec<ReadablePart>,
) -> Result<SourceRevision, RecordFilingError> {
    file_record_with_provenance(
        workspace,
        source_id,
        raw_json,
        envelope,
        readable_parts,
        serde_json::Map::new(),
    )
}

fn file_record_with_provenance(
    workspace: &str,
    source_id: &str,
    raw_json: &str,
    envelope: RecordEnvelope,
    readable_parts: Vec<ReadablePart>,
    extra_provenance: serde_json::Map<String, Value>,
) -> Result<SourceRevision, RecordFilingError> {
    let kind = envelope.source_kind.kind.clone();
    let native_record = format!("{kind}/{}", envelope.native_id);
    let candidate_id = format!("{source_id}/{native_record}");
    let sha256 = crate::sha256_hex(raw_json.as_bytes());
    let legacy_dates = envelope
        .dates
        .iter()
        .map(|d| RecordDate {
            name: d.field.clone(),
            value: d.value.clone(),
        })
        .collect();
    let mut provenance_obj = serde_json::Map::new();
    for (k, v) in extra_provenance {
        provenance_obj.insert(k, v);
    }
    provenance_obj.insert("source_id".to_owned(), json!(source_id));
    provenance_obj.insert("native_record".to_owned(), json!(native_record));
    // Filing metadata and decoded parts are retained evidence too. Re-filing
    // unchanged native bytes after a mapping correction must preserve the old
    // observation rather than conflict with its immutable revision identity.
    let revision_id = crate::sha256_hex(
        &serde_json::to_vec(&(raw_json, &envelope, &readable_parts))
            .expect("filed evidence serializes"),
    );
    let revision = SourceRevision {
        workspace: workspace.to_owned(),
        source_id: source_id.to_owned(),
        candidate_id,
        revision: revision_id,
        record_type: kind,
        dates: legacy_dates,
        content: raw_json.to_owned(),
        sha256,
        readable_parts,
        provenance: Value::Object(provenance_obj),
        envelope: Some(envelope),
    };
    revision
        .validate()
        .map_err(RecordFilingError::InvalidRevision)?;
    Ok(revision)
}

struct MatchedSpec {
    bucket: Bucket,
    dates: &'static [(DateRole, &'static str)],
    status: &'static [ValueExtractor],
    codes: &'static [&'static str],
    title: &'static [ValueExtractor],
    attachments: &'static [AttachmentRule],
}

fn match_row(
    table: &AdapterTable,
    kind: &str,
    categories: &[String],
) -> (MatchedSpec, Option<String>) {
    let first_category = || categories.first().cloned();
    for row in table.rows.iter().filter(|row| row.kind == kind) {
        let matched_category = match row.category {
            CategoryMatch::Any => Some(first_category()),
            CategoryMatch::OneOf(allowed) => categories
                .iter()
                .find(|cat| allowed.contains(&cat.as_str()))
                .cloned()
                .map(Some),
            CategoryMatch::OtherOrNone => Some(first_category()),
        };
        if let Some(resolved_category) = matched_category {
            return (
                MatchedSpec {
                    bucket: row.bucket,
                    dates: row.dates,
                    status: row.status,
                    codes: row.codes,
                    title: row.title,
                    attachments: row.attachments,
                },
                resolved_category,
            );
        }
    }
    (
        MatchedSpec {
            bucket: table.unmapped.bucket,
            dates: table.unmapped.dates,
            status: table.unmapped.status,
            codes: table.unmapped.codes,
            title: table.unmapped.title,
            attachments: table.unmapped.attachments,
        },
        first_category(),
    )
}

fn lookup_path<'a>(root: &'a Value, path: &str) -> Option<&'a Value> {
    let mut current = root;
    for segment in path.split('.') {
        current = current.get(segment)?;
    }
    Some(current)
}

fn scalar_string(value: &Value) -> Option<String> {
    match value {
        Value::String(s) if !s.is_empty() => Some(s.clone()),
        Value::Number(n) => Some(n.to_string()),
        _ => None,
    }
}

fn first_extracted(resource: &Value, extractors: &[ValueExtractor]) -> Option<String> {
    extractors
        .iter()
        .find_map(|extractor| apply_extractor(resource, *extractor))
}

fn extract_all_values(resource: &Value, extractors: &[ValueExtractor]) -> Vec<String> {
    let mut out = Vec::new();
    for extractor in extractors {
        match *extractor {
            ValueExtractor::Scalar(path) => {
                if let Some(value) = lookup_path(resource, path).and_then(scalar_string) {
                    out.push(value);
                }
            }
            ValueExtractor::ConceptCode(path) => {
                if let Some(value) = lookup_path(resource, path) {
                    collect_concept_codes(value, &mut out);
                }
            }
            ValueExtractor::ConceptLabel(path) => {
                if let Some(value) = lookup_path(resource, path).and_then(concept_label) {
                    out.push(value);
                }
            }
            ValueExtractor::HumanName(path) => {
                if let Some(value) = lookup_path(resource, path).and_then(human_name) {
                    out.push(value);
                }
            }
        }
    }
    out
}

fn collect_concept_codes(value: &Value, out: &mut Vec<String>) {
    match value {
        Value::String(s) if !s.is_empty() => out.push(s.clone()),
        Value::Object(obj) => {
            let before = out.len();
            if let Some(codings) = obj.get("coding").and_then(Value::as_array) {
                for item in codings {
                    if let Some(code) = item.get("code").and_then(scalar_string) {
                        out.push(code);
                    }
                }
            }
            if out.len() == before
                && let Some(fallback) = obj
                    .get("code")
                    .and_then(scalar_string)
                    .or_else(|| obj.get("text").and_then(scalar_string))
            {
                out.push(fallback);
            }
        }
        Value::Array(items) => {
            for item in items {
                collect_concept_codes(item, out);
            }
        }
        _ => {}
    }
}

fn apply_extractor(resource: &Value, extractor: ValueExtractor) -> Option<String> {
    match extractor {
        ValueExtractor::Scalar(path) => lookup_path(resource, path).and_then(scalar_string),
        ValueExtractor::ConceptCode(path) => lookup_path(resource, path).and_then(concept_code),
        ValueExtractor::ConceptLabel(path) => lookup_path(resource, path).and_then(concept_label),
        ValueExtractor::HumanName(path) => lookup_path(resource, path).and_then(human_name),
    }
}

fn concept_code(value: &Value) -> Option<String> {
    match value {
        Value::String(s) if !s.is_empty() => Some(s.clone()),
        Value::Object(obj) => {
            if let Some(codings) = obj.get("coding").and_then(Value::as_array)
                && let Some(code) = codings
                    .iter()
                    .find_map(|item| item.get("code").and_then(scalar_string))
            {
                return Some(code);
            }
            obj.get("code")
                .and_then(scalar_string)
                .or_else(|| obj.get("text").and_then(scalar_string))
        }
        Value::Array(items) => items.iter().find_map(concept_code),
        _ => None,
    }
}

fn concept_label(value: &Value) -> Option<String> {
    match value {
        Value::String(s) if !s.is_empty() => Some(s.clone()),
        Value::Object(obj) => {
            if let Some(text) = obj.get("text").and_then(scalar_string) {
                return Some(text);
            }
            if let Some(codings) = obj.get("coding").and_then(Value::as_array) {
                if let Some(display) = codings
                    .iter()
                    .find_map(|item| item.get("display").and_then(scalar_string))
                {
                    return Some(display);
                }
                if let Some(code) = codings
                    .iter()
                    .find_map(|item| item.get("code").and_then(scalar_string))
                {
                    return Some(code);
                }
            }
            obj.get("display")
                .and_then(scalar_string)
                .or_else(|| obj.get("code").and_then(scalar_string))
        }
        Value::Array(items) => items.iter().find_map(concept_label),
        _ => None,
    }
}

fn human_name(value: &Value) -> Option<String> {
    match value {
        Value::String(s) if !s.is_empty() => Some(s.clone()),
        Value::Object(obj) => {
            if let Some(text) = obj.get("text").and_then(scalar_string) {
                return Some(text);
            }
            let given = obj
                .get("given")
                .and_then(Value::as_array)
                .into_iter()
                .flatten()
                .filter_map(Value::as_str)
                .filter(|part| !part.is_empty());
            let family = obj
                .get("family")
                .and_then(Value::as_str)
                .filter(|part| !part.is_empty());
            let parts: Vec<&str> = given.chain(family).collect();
            (!parts.is_empty()).then(|| parts.join(" "))
        }
        Value::Array(items) => items.iter().find_map(human_name),
        _ => None,
    }
}

fn extract_dates(resource: &Value, specs: &[(DateRole, &'static str)]) -> Vec<FiledDate> {
    specs
        .iter()
        .filter_map(|(role, path)| {
            let value = lookup_path(resource, path).and_then(scalar_string)?;
            Some(FiledDate {
                role: *role,
                field: (*path).to_owned(),
                value,
            })
        })
        .collect()
}

fn extract_all_codes(resource: &Value, code_paths: &[&'static str]) -> Vec<RecordCode> {
    let mut out = Vec::new();
    for path in code_paths {
        if let Some(value) = lookup_path(resource, path) {
            extract_codes_from_value(path, value, &mut out);
        }
    }
    out
}

fn extract_codes_from_value(field: &str, value: &Value, out: &mut Vec<RecordCode>) {
    match value {
        Value::String(s) if !s.is_empty() => {
            out.push(RecordCode {
                field: field.to_owned(),
                system: None,
                code: Some(s.clone()),
                display: None,
                text: None,
            });
        }
        Value::Object(obj) => {
            let text = obj.get("text").and_then(scalar_string);
            if let Some(codings) = obj.get("coding").and_then(Value::as_array) {
                let before = out.len();
                for item in codings {
                    let system = item.get("system").and_then(scalar_string);
                    let code = item.get("code").and_then(scalar_string);
                    let display = item.get("display").and_then(scalar_string);
                    if system.is_some() || code.is_some() || display.is_some() {
                        out.push(RecordCode {
                            field: field.to_owned(),
                            system,
                            code,
                            display,
                            text: text.clone(),
                        });
                    }
                }
                if out.len() == before && text.is_some() {
                    out.push(RecordCode {
                        field: field.to_owned(),
                        system: None,
                        code: None,
                        display: None,
                        text,
                    });
                }
                return;
            }
            let system = obj.get("system").and_then(scalar_string);
            let code = obj.get("code").and_then(scalar_string);
            let display = obj.get("display").and_then(scalar_string);
            if system.is_some() || code.is_some() || display.is_some() || text.is_some() {
                out.push(RecordCode {
                    field: field.to_owned(),
                    system,
                    code,
                    display,
                    text,
                });
            }
        }
        Value::Array(items) => {
            for item in items {
                extract_codes_from_value(field, item, out);
            }
        }
        _ => {}
    }
}

struct AttachmentContext<'a> {
    workspace: &'a str,
    source_id: &'a str,
    candidate_id: &'a str,
    authored_date: &'a str,
    record_title: &'a str,
    document_version: u64,
}

struct DecodedAttachmentOutcome {
    parts: Vec<ReadablePart>,
    failed_pages: Vec<FailedPart>,
    provenance: Option<Value>,
}

fn extract_attachments(
    resource: &Value,
    rules: &[AttachmentRule],
    ctx: &AttachmentContext<'_>,
) -> (
    Vec<ReadablePart>,
    Vec<FailedPart>,
    serde_json::Map<String, Value>,
) {
    let mut parts = Vec::new();
    let mut unavailable = Vec::new();
    let mut extra_provenance = serde_json::Map::new();
    for rule in rules {
        let AttachmentRule::Base64Text {
            items_path,
            label,
            content_type_path,
            data_path,
        } = *rule;
        let Some(items) = lookup_path(resource, items_path).and_then(Value::as_array) else {
            continue;
        };
        for (index, item) in items.iter().enumerate() {
            let is_primary_pdf_slot = !extra_provenance.contains_key("document_id");
            match decode_attachment(
                item,
                label,
                content_type_path,
                data_path,
                ctx,
                index,
                is_primary_pdf_slot,
            ) {
                Ok(outcome) => {
                    parts.extend(outcome.parts);
                    for page_err in outcome.failed_pages {
                        unavailable.push(FailedPart {
                            part: format!("{items_path}[{index}].{}", page_err.part),
                            reason: page_err.reason,
                        });
                    }
                    if let Some(Value::Object(obj)) = outcome.provenance {
                        for (k, v) in obj {
                            extra_provenance.entry(k).or_insert(v);
                        }
                    }
                }
                Err(error) => unavailable.push(FailedPart {
                    part: format!("{items_path}[{index}]"),
                    reason: error.to_string(),
                }),
            }
        }
    }
    (parts, unavailable, extra_provenance)
}

fn decode_attachment(
    item: &Value,
    label: &'static str,
    content_type_path: &str,
    data_path: &str,
    ctx: &AttachmentContext<'_>,
    index: usize,
    is_primary_pdf_slot: bool,
) -> Result<DecodedAttachmentOutcome, RecordFilingError> {
    use crate::attachments::{
        AttachmentMediaKind, PagedDocumentInput, SavedAttachmentInput, decode_saved_attachment,
        image_part_readable_and_provenance, ingest_paged_document,
    };

    let content_type = lookup_path(item, content_type_path)
        .and_then(Value::as_str)
        .unwrap_or_default();
    let parent = match data_path.rsplit_once('.') {
        Some((prefix, _)) => lookup_path(item, prefix),
        None => Some(item),
    };
    let data = lookup_path(item, data_path).and_then(Value::as_str);
    let Some(encoded) = data else {
        if let Some(url) = parent
            .and_then(|obj| obj.get("url"))
            .and_then(scalar_string)
        {
            return Err(RecordFilingError::ExternalAttachment { label, url });
        }
        return Err(RecordFilingError::UnavailableAttachment { label });
    };

    let attachment_title = parent
        .and_then(|obj| obj.get("title"))
        .and_then(scalar_string);
    let item_title = attachment_title.as_deref().unwrap_or(ctx.record_title);
    let normalized_ct = content_type
        .split(';')
        .next()
        .unwrap_or("")
        .trim()
        .to_ascii_lowercase();
    let media_kind = AttachmentMediaKind::parse(content_type, attachment_title.as_deref());

    match media_kind {
        Some(
            kind @ (AttachmentMediaKind::PdfDocument
            | AttachmentMediaKind::SpruceFaxPdf
            | AttachmentMediaKind::ChartScanPdf),
        ) => {
            let bytes = base64::engine::general_purpose::STANDARD
                .decode(encoded)
                .map_err(|source| RecordFilingError::InvalidBase64 { label, source })?;
            let document_id = if is_primary_pdf_slot {
                ctx.candidate_id.to_owned()
            } else {
                format!("{}#att-{}", ctx.candidate_id, index + 1)
            };
            let ingested = ingest_paged_document(&PagedDocumentInput {
                document_id,
                version: ctx.document_version,
                workspace: ctx.workspace.to_owned(),
                source_id: ctx.source_id.to_owned(),
                authored_date: ctx.authored_date.to_owned(),
                title: item_title.to_owned(),
                media_kind: kind,
                pdf_bytes: bytes,
                expected_sha256: None,
            })
            .map_err(|source| RecordFilingError::AttachmentDecode {
                label,
                source: Box::new(source),
            })?;
            let failed_pages = ingested
                .pages
                .iter()
                .filter_map(|page_res| match page_res {
                    Ok(_) => None,
                    Err(failure) => {
                        let page_num = match failure {
                            crate::attachments::AttachmentReadFailure::CorruptedPage {
                                page_number,
                                ..
                            } => *page_number,
                            _ => 0,
                        };
                        Some(FailedPart {
                            part: format!("page[{page_num}]"),
                            reason: failure.to_string(),
                        })
                    }
                })
                .collect();
            Ok(DecodedAttachmentOutcome {
                parts: ingested.source_revision.readable_parts,
                failed_pages,
                provenance: Some(ingested.source_revision.provenance),
            })
        }
        Some(kind @ (AttachmentMediaKind::PngPhoto | AttachmentMediaKind::JpegPhoto)) => {
            let bytes = base64::engine::general_purpose::STANDARD
                .decode(encoded)
                .map_err(|source| RecordFilingError::InvalidBase64 { label, source })?;
            let attachment_id = if index == 0 {
                ctx.candidate_id.to_owned()
            } else {
                format!("{}#att-{}", ctx.candidate_id, index + 1)
            };
            let part = decode_saved_attachment(&SavedAttachmentInput {
                attachment_id,
                revision: format!("v{}", ctx.document_version),
                workspace: ctx.workspace.to_owned(),
                source_id: ctx.source_id.to_owned(),
                authored_date: ctx.authored_date.to_owned(),
                title: item_title.to_owned(),
                media_kind: kind,
                bytes,
                expected_sha256: None,
            })
            .map_err(|source| RecordFilingError::AttachmentDecode {
                label,
                source: Box::new(source),
            })?;
            let (readable, provenance) = image_part_readable_and_provenance(item_title, &part)
                .ok_or_else(|| RecordFilingError::UnsupportedAttachment {
                    label,
                    content_type: content_type.to_owned(),
                })?;
            Ok(DecodedAttachmentOutcome {
                parts: vec![readable],
                failed_pages: Vec::new(),
                provenance: Some(provenance),
            })
        }
        Some(
            AttachmentMediaKind::PlainText
            | AttachmentMediaKind::MarkdownText
            | AttachmentMediaKind::CsvTable,
        ) => decode_text_attachment(encoded, label),
        None if normalized_ct.starts_with("text/") => decode_text_attachment(encoded, label),
        None => Err(RecordFilingError::UnsupportedAttachment {
            label,
            content_type: content_type.to_owned(),
        }),
    }
}

fn decode_text_attachment(
    encoded: &str,
    label: &'static str,
) -> Result<DecodedAttachmentOutcome, RecordFilingError> {
    let bytes = base64::engine::general_purpose::STANDARD
        .decode(encoded)
        .map_err(|source| RecordFilingError::InvalidBase64 { label, source })?;
    let text = String::from_utf8(bytes)
        .map_err(|source| RecordFilingError::InvalidUtf8 { label, source })?;
    Ok(DecodedAttachmentOutcome {
        parts: vec![ReadablePart {
            label: label.to_owned(),
            text,
        }],
        failed_pages: Vec::new(),
        provenance: None,
    })
}
