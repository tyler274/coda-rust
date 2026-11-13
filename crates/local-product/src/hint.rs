//! Hint chart filing table and [`PatientSource`] adapter.
//!
//! Translates structured browser-acquired Hint chart captures into Coda's fixed
//! bucket index (`intent/physician.md`) and [`SourceRevision`] envelopes without
//! altering clinical values, status strings, codes, or typed-text dates. Billing
//! fields on patient records and price/invoice fields on prescriptions are
//! stripped before retention.

use std::collections::{BTreeMap, BTreeSet};

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value, json};

use crate::physician_sources::{
    MedicalRecordNumber, PatientSource, ReadablePart, RecordDate, SourcePatient, SourceRevision,
};
use crate::sha256_hex;

/// Stable source identifier for Hint chart records.
pub const HINT_SOURCE_ID: &str = "hint";

/// Display name for Hint shown to physicians.
pub const HINT_SOURCE_LABEL: &str = "Hint";

/// Base web URL of the Hint clinician portal used for deep links.
pub const HINT_APP_BASE_URL: &str = "https://app.hint.com";

/// All 16 patient-scoped Hint chart reads required for a complete whole-chart snapshot.
pub const HINT_WHOLE_CHART_READS: [&str; 16] = [
    "patient",
    "diagnoses",
    "allergies",
    "medications",
    "freeform-medications",
    "surgeries",
    "social-history",
    "family-history",
    "vaccines",
    "general",
    "vitals",
    "notes",
    "tasks",
    "appointments",
    "messages",
    "patient-files",
];

/// Billing and membership-payment fields dropped from Hint patient records.
pub const PATIENT_BILLING_FIELDS: &[&str] = &[
    "billing",
    "balance",
    "balances",
    "account_balance",
    "accountBalance",
    "credit_balance",
    "creditBalance",
    "payment_method",
    "payment_methods",
    "paymentMethod",
    "paymentMethods",
    "stripe_customer_id",
    "stripeCustomerId",
    "invoices",
    "billing_address",
    "billingAddress",
    "membership_fee",
    "membershipFee",
    "plan_price",
    "planPrice",
];

/// Pricing, cost, dispensing-fee, and invoice fields dropped from Hint prescriptions.
pub const RX_PRICE_INVOICE_FIELDS: &[&str] = &[
    "price",
    "unit_price",
    "unitPrice",
    "total_price",
    "totalPrice",
    "cost",
    "wholesale_price",
    "wholesalePrice",
    "retail_price",
    "retailPrice",
    "dispensing_fee",
    "dispensingFee",
    "invoice",
    "invoice_id",
    "invoiceId",
    "invoice_number",
    "invoiceNumber",
    "invoice_status",
    "invoiceStatus",
    "charge",
    "charge_id",
    "chargeId",
    "line_item_id",
    "lineItemId",
    "amount_cents",
    "amountCents",
];

/// Fixed set of Coda chart buckets defined in `intent/physician.md`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum ChartBucket {
    /// Patient identity, birth date, sex, identifiers, and contact details.
    Patient,
    /// Diagnoses and health concerns on the problem list.
    Problems,
    /// Medications prescribed, dispensed, or reported, including stopped ones.
    Medications,
    /// Allergies and intolerances, including no-known-allergies records.
    Allergies,
    /// Test results: laboratory results, imaging, pathology, and diagnostic reports.
    Results,
    /// Vital signs and body measurements.
    Vitals,
    /// Visit notes, phone notes, sticky notes, and other chart notes.
    Notes,
    /// Scans, faxes, letters, uploaded files, and outside records.
    Documents,
    /// Appointments and encounters.
    Visits,
    /// Procedures and surgeries that were performed.
    Procedures,
    /// Vaccines given or reported.
    Immunizations,
    /// Orders, eConsults, and referrals.
    Orders,
    /// Family, social, past medical, and past surgical history.
    History,
    /// Filled-in forms and questionnaires.
    Forms,
    /// Messages between the patient and the practice.
    Messages,
    /// Any record whose source kind has no dedicated bucket.
    Other,
}

impl ChartBucket {
    /// Canonical bucket for laboratory results (`Results` in `intent/physician.md`).
    pub const LABS: Self = Self::Results;

    /// All sixteen canonical Coda buckets in specification order.
    pub const ALL: [Self; 16] = [
        Self::Patient,
        Self::Problems,
        Self::Medications,
        Self::Allergies,
        Self::Results,
        Self::Vitals,
        Self::Notes,
        Self::Documents,
        Self::Visits,
        Self::Procedures,
        Self::Immunizations,
        Self::Orders,
        Self::History,
        Self::Forms,
        Self::Messages,
        Self::Other,
    ];

    /// Canonical bucket name as written in `intent/physician.md`.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Patient => "Patient",
            Self::Problems => "Problems",
            Self::Medications => "Medications",
            Self::Allergies => "Allergies",
            Self::Results => "Results",
            Self::Vitals => "Vitals",
            Self::Notes => "Notes",
            Self::Documents => "Documents",
            Self::Visits => "Visits",
            Self::Procedures => "Procedures",
            Self::Immunizations => "Immunizations",
            Self::Orders => "Orders",
            Self::History => "History",
            Self::Forms => "Forms",
            Self::Messages => "Messages",
            Self::Other => "Other",
        }
    }
}

/// Whether a Hint filing-table row has been confirmed against live Hint specimens
/// or derives from Hint's shipped page code awaiting live specimen confirmation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ConfirmationStatus {
    /// Confirmed against recorded Hint responses.
    Confirmed,
    /// Derived from Hint's web page code and API contracts; awaiting live specimen check.
    Unconfirmed,
}

/// Semantic role of a date field on a Hint record.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum DateRole {
    /// Patient date of birth.
    BirthDate,
    /// Clinical onset date.
    Onset,
    /// Clinical resolution or abatement date.
    Resolved,
    /// Medication start date.
    Started,
    /// Medication stop or end date.
    Stopped,
    /// When a note, prescription, or document was authored or written.
    Authored,
    /// When a clinical note was signed.
    Signed,
    /// When a procedure, vaccine, or encounter was performed.
    Performed,
    /// When a measurement or history item was observed.
    Observed,
    /// Scheduled appointment start.
    ScheduledStart,
    /// Scheduled appointment end.
    ScheduledEnd,
    /// Specimen collection timestamp.
    Collected,
    /// Diagnostic result or report timestamp.
    Resulted,
    /// Order or referral placement timestamp.
    Ordered,
    /// Task or order due date.
    Due,
    /// Completion or submission timestamp.
    Completed,
    /// Message send timestamp.
    Sent,
    /// Inbound message, fax, or file receipt timestamp.
    Received,
    /// Record creation timestamp in Hint.
    Recorded,
    /// Record last-updated timestamp in Hint.
    Updated,
}

/// One date-field mapping rule in the Hint filing table.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DateFieldRule {
    /// Dot-separated path in the Hint JSON object (e.g. `"onsetDate"` or `"labOrder.orderedAt"`).
    pub field: &'static str,
    /// Semantic role of that date field.
    pub role: DateRole,
}

/// One extracted date on a filed Hint record, keeping Hint's exact text.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FiledDate {
    /// Semantic role from the Hint filing table.
    pub role: DateRole,
    /// Hint's own field name.
    pub field: String,
    /// Value exactly as Hint sent it (including typed-text dates like `"01/2020"`).
    pub value: String,
}

/// One clinical or administrative code extracted into the record envelope.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RecordCode {
    /// Hint field or coding system label where the code appeared.
    pub field: String,
    /// Code value as Hint sent it.
    pub code: String,
}

/// Field-stripping policy applied before retaining a Hint record.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RedactionRule {
    /// Retain every field exactly as Hint sent it.
    None,
    /// Drop [`PATIENT_BILLING_FIELDS`] from a patient demographics record.
    DropPatientBilling,
    /// Drop [`RX_PRICE_INVOICE_FIELDS`] from prescription records or embedded Rx items.
    DropRxPricingAndInvoice,
}

/// Deep-link construction rule for a Hint record kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HintLinkRule {
    /// Patient chart root (`/patients/{patient_id}`), which is the patient record's own page.
    DirectPatientChart,
    /// Dedicated note page (`/patients/{patient_id}/notes/{record_id}`).
    DirectNote,
    /// Dedicated appointment page (`/patients/{patient_id}/appointments/{record_id}`).
    DirectAppointment,
    /// Dedicated patient-file page (`/patients/{patient_id}/files/{record_id}`).
    DirectFile,
    /// Dedicated message page (`/patients/{patient_id}/messages/{record_id}`).
    DirectMessage,
    /// Opens `/patients/{patient_id}/notes/{note_id}` when backed by a Hint note ID,
    /// otherwise falls back to the patient chart with explicit disclosure.
    NoteWhenPresentElseChart {
        /// Why standalone items of this kind without a note ID fall back to the chart.
        fallback_reason: &'static str,
    },
    /// Opens `/patients/{patient_id}/files/{file_id}` when backed by a Hint file ID,
    /// otherwise falls back to the patient chart with explicit disclosure.
    FileWhenPresentElseChart {
        /// Why items without a file ID fall back to the chart.
        fallback_reason: &'static str,
    },
    /// Hint exposes no per-record URL for this kind; opens the patient chart and discloses why.
    ChartFallback {
        /// Why this record kind opens the patient chart instead of a per-record URL.
        reason: &'static str,
    },
}

/// Link into Hint carried on a filed record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum HintLink {
    /// Opens the specific record directly in Hint.
    DirectRecord {
        /// Deep-link URL to that record in Hint.
        url: String,
    },
    /// Opens the patient's Hint chart because Hint exposes no per-record URL for this record.
    ChartFallback {
        /// URL to the patient's chart in Hint.
        url: String,
        /// Explicit disclosure explaining why the link opens the chart rather than a single item.
        disclosure: String,
    },
}

impl HintLink {
    /// Destination URL in Hint.
    pub fn url(&self) -> &str {
        match self {
            Self::DirectRecord { url } | Self::ChartFallback { url, .. } => url,
        }
    }

    /// Whether this link falls back to the patient chart rather than addressing the single record.
    pub const fn is_chart_fallback(&self) -> bool {
        matches!(self, Self::ChartFallback { .. })
    }
}

/// Declared Hint record kinds in [`HINT_TABLE`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum KnownHintKind {
    /// Patient demographics and contact record.
    Patient,
    /// Problem-list diagnosis.
    Diagnosis,
    /// Allergy or intolerance entry.
    Allergy,
    /// Active or historical medication entry.
    Medication,
    /// Freeform medication entry.
    FreeformMedication,
    /// In-house dispensed or prescribed Rx.
    InHouseRx,
    /// Past surgical procedure entry.
    Surgery,
    /// Social history entry.
    SocialHistory,
    /// Family history entry.
    FamilyHistory,
    /// Vaccine / immunization entry.
    Vaccine,
    /// General chart info entry.
    GeneralInfo,
    /// Vital signs measurement record.
    Vitals,
    /// Clinical / encounter / visit / phone chart note.
    ClinicalNote,
    /// Sticky note on the chart.
    StickyNote,
    /// Care-team task / reminder.
    Task,
    /// Scheduled or past appointment.
    Appointment,
    /// Structured lab note / result in Hint's page-code shape.
    Lab,
    /// Inbound or outbound fax record.
    Fax,
    /// Completed or sent intake form / questionnaire.
    Form,
    /// External electronic prescription (eRx).
    ERx,
    /// Patient–practice message.
    Message,
    /// Uploaded chart document / patient file.
    PatientFile,
    /// Specialty eConsult record.
    EConsult,
    /// Outbound or inbound clinical referral.
    Referral,
    /// Clinical order record.
    Order,
    /// Partner / integration record.
    PartnerRecord,
}

/// Resolved Hint record classification: either a declared row in [`HINT_TABLE`]
/// or an unrecognized Hint kind filed into [`ChartBucket::Other`].
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(tag = "classification", content = "value", rename_all = "snake_case")]
pub enum HintKind {
    /// Matches a declared row in [`HINT_TABLE`].
    Known(KnownHintKind),
    /// Unrecognized Hint kind preserved in [`ChartBucket::Other`] and reported.
    Unknown(String),
}

/// One row of the Hint filing table.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HintRecordRow {
    /// Typed Hint record kind.
    pub kind: KnownHintKind,
    /// Hint's canonical name for this kind (including category where categorized).
    pub source_name: &'static str,
    /// Destination Coda bucket.
    pub bucket: ChartBucket,
    /// Whether this row is confirmed or unconfirmed at the start.
    pub confirmation: ConfirmationStatus,
    /// Declared date fields and their semantic roles.
    pub date_fields: &'static [DateFieldRule],
    /// Candidate JSON paths carrying the record's status in Hint's words.
    pub status_paths: &'static [&'static str],
    /// Candidate JSON paths carrying clinical or administrative codes.
    pub code_paths: &'static [&'static str],
    /// Candidate JSON paths carrying a human-readable title.
    pub title_paths: &'static [&'static str],
    /// Field-redaction rule before retention.
    pub redaction: RedactionRule,
    /// Deep-link rule into Hint.
    pub link_rule: HintLinkRule,
}

/// Report entry describing a Hint record kind that falls back (always or conditionally) to the chart URL.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct FallbackKindReport {
    /// Canonical Hint source kind name.
    pub source_name: &'static str,
    /// Destination Coda bucket.
    pub bucket: ChartBucket,
    /// Whether every record of this kind falls back (`true`) or only records without a backing note/file ID (`false`).
    pub always_falls_back: bool,
    /// Disclosure reason explaining why Hint has no standalone URL for those records.
    pub disclosure: &'static str,
}

const PATIENT_DATES: &[DateFieldRule] = &[
    DateFieldRule {
        field: "dob",
        role: DateRole::BirthDate,
    },
    DateFieldRule {
        field: "birth_date",
        role: DateRole::BirthDate,
    },
    DateFieldRule {
        field: "created_at",
        role: DateRole::Recorded,
    },
    DateFieldRule {
        field: "createdAt",
        role: DateRole::Recorded,
    },
    DateFieldRule {
        field: "updated_at",
        role: DateRole::Updated,
    },
    DateFieldRule {
        field: "updatedAt",
        role: DateRole::Updated,
    },
];

const DIAGNOSIS_DATES: &[DateFieldRule] = &[
    DateFieldRule {
        field: "onsetDate",
        role: DateRole::Onset,
    },
    DateFieldRule {
        field: "onset_date",
        role: DateRole::Onset,
    },
    DateFieldRule {
        field: "resolvedDate",
        role: DateRole::Resolved,
    },
    DateFieldRule {
        field: "resolved_date",
        role: DateRole::Resolved,
    },
    DateFieldRule {
        field: "createdAt",
        role: DateRole::Recorded,
    },
    DateFieldRule {
        field: "updatedAt",
        role: DateRole::Updated,
    },
];

const ALLERGY_DATES: &[DateFieldRule] = &[
    DateFieldRule {
        field: "onsetDate",
        role: DateRole::Onset,
    },
    DateFieldRule {
        field: "recordedDate",
        role: DateRole::Observed,
    },
    DateFieldRule {
        field: "createdAt",
        role: DateRole::Recorded,
    },
    DateFieldRule {
        field: "updatedAt",
        role: DateRole::Updated,
    },
];

const MEDICATION_DATES: &[DateFieldRule] = &[
    DateFieldRule {
        field: "startDate",
        role: DateRole::Started,
    },
    DateFieldRule {
        field: "endDate",
        role: DateRole::Stopped,
    },
    DateFieldRule {
        field: "stoppedDate",
        role: DateRole::Stopped,
    },
    DateFieldRule {
        field: "prescribedAt",
        role: DateRole::Authored,
    },
    DateFieldRule {
        field: "dispensedAt",
        role: DateRole::Performed,
    },
    DateFieldRule {
        field: "writtenAt",
        role: DateRole::Authored,
    },
    DateFieldRule {
        field: "sentAt",
        role: DateRole::Sent,
    },
    DateFieldRule {
        field: "createdAt",
        role: DateRole::Recorded,
    },
    DateFieldRule {
        field: "updatedAt",
        role: DateRole::Updated,
    },
];

const SURGERY_DATES: &[DateFieldRule] = &[
    DateFieldRule {
        field: "performedDate",
        role: DateRole::Performed,
    },
    DateFieldRule {
        field: "surgeryDate",
        role: DateRole::Performed,
    },
    DateFieldRule {
        field: "date",
        role: DateRole::Performed,
    },
    DateFieldRule {
        field: "createdAt",
        role: DateRole::Recorded,
    },
    DateFieldRule {
        field: "updatedAt",
        role: DateRole::Updated,
    },
];

const HISTORY_DATES: &[DateFieldRule] = &[
    DateFieldRule {
        field: "onsetDate",
        role: DateRole::Onset,
    },
    DateFieldRule {
        field: "recordedDate",
        role: DateRole::Observed,
    },
    DateFieldRule {
        field: "effectiveDate",
        role: DateRole::Observed,
    },
    DateFieldRule {
        field: "createdAt",
        role: DateRole::Recorded,
    },
    DateFieldRule {
        field: "updatedAt",
        role: DateRole::Updated,
    },
];

const VACCINE_DATES: &[DateFieldRule] = &[
    DateFieldRule {
        field: "administeredDate",
        role: DateRole::Performed,
    },
    DateFieldRule {
        field: "administeredAt",
        role: DateRole::Performed,
    },
    DateFieldRule {
        field: "date",
        role: DateRole::Performed,
    },
    DateFieldRule {
        field: "expirationDate",
        role: DateRole::Due,
    },
    DateFieldRule {
        field: "createdAt",
        role: DateRole::Recorded,
    },
    DateFieldRule {
        field: "updatedAt",
        role: DateRole::Updated,
    },
];

const GENERAL_DATES: &[DateFieldRule] = &[
    DateFieldRule {
        field: "recordedDate",
        role: DateRole::Observed,
    },
    DateFieldRule {
        field: "recordedAt",
        role: DateRole::Observed,
    },
    DateFieldRule {
        field: "syncedAt",
        role: DateRole::Observed,
    },
    DateFieldRule {
        field: "createdAt",
        role: DateRole::Recorded,
    },
    DateFieldRule {
        field: "updatedAt",
        role: DateRole::Updated,
    },
];

const VITALS_DATES: &[DateFieldRule] = &[
    DateFieldRule {
        field: "measuredAt",
        role: DateRole::Observed,
    },
    DateFieldRule {
        field: "takenAt",
        role: DateRole::Observed,
    },
    DateFieldRule {
        field: "observedAt",
        role: DateRole::Observed,
    },
    DateFieldRule {
        field: "effectiveDateTime",
        role: DateRole::Observed,
    },
    DateFieldRule {
        field: "createdAt",
        role: DateRole::Recorded,
    },
    DateFieldRule {
        field: "updatedAt",
        role: DateRole::Updated,
    },
];

const NOTE_DATES: &[DateFieldRule] = &[
    DateFieldRule {
        field: "encounterDate",
        role: DateRole::Performed,
    },
    DateFieldRule {
        field: "serviceDate",
        role: DateRole::Performed,
    },
    DateFieldRule {
        field: "authoredAt",
        role: DateRole::Authored,
    },
    DateFieldRule {
        field: "signedAt",
        role: DateRole::Signed,
    },
    DateFieldRule {
        field: "createdAt",
        role: DateRole::Recorded,
    },
    DateFieldRule {
        field: "updatedAt",
        role: DateRole::Updated,
    },
];

const TASK_DATES: &[DateFieldRule] = &[
    DateFieldRule {
        field: "dueOn",
        role: DateRole::Due,
    },
    DateFieldRule {
        field: "dueDate",
        role: DateRole::Due,
    },
    DateFieldRule {
        field: "completedAt",
        role: DateRole::Completed,
    },
    DateFieldRule {
        field: "createdAt",
        role: DateRole::Recorded,
    },
    DateFieldRule {
        field: "updatedAt",
        role: DateRole::Updated,
    },
];

const APPOINTMENT_DATES: &[DateFieldRule] = &[
    DateFieldRule {
        field: "starts_at",
        role: DateRole::ScheduledStart,
    },
    DateFieldRule {
        field: "startAt",
        role: DateRole::ScheduledStart,
    },
    DateFieldRule {
        field: "scheduledAt",
        role: DateRole::ScheduledStart,
    },
    DateFieldRule {
        field: "ends_at",
        role: DateRole::ScheduledEnd,
    },
    DateFieldRule {
        field: "endAt",
        role: DateRole::ScheduledEnd,
    },
    DateFieldRule {
        field: "checkedInAt",
        role: DateRole::Performed,
    },
    DateFieldRule {
        field: "created_at",
        role: DateRole::Recorded,
    },
    DateFieldRule {
        field: "createdAt",
        role: DateRole::Recorded,
    },
    DateFieldRule {
        field: "updated_at",
        role: DateRole::Updated,
    },
    DateFieldRule {
        field: "updatedAt",
        role: DateRole::Updated,
    },
];

const LAB_DATES: &[DateFieldRule] = &[
    DateFieldRule {
        field: "specimenCollectedAt",
        role: DateRole::Collected,
    },
    DateFieldRule {
        field: "collectedAt",
        role: DateRole::Collected,
    },
    DateFieldRule {
        field: "labOrder.collectedAt",
        role: DateRole::Collected,
    },
    DateFieldRule {
        field: "resultedAt",
        role: DateRole::Resulted,
    },
    DateFieldRule {
        field: "reportedAt",
        role: DateRole::Resulted,
    },
    DateFieldRule {
        field: "labOrder.resultedAt",
        role: DateRole::Resulted,
    },
    DateFieldRule {
        field: "orderedAt",
        role: DateRole::Ordered,
    },
    DateFieldRule {
        field: "labOrder.orderedAt",
        role: DateRole::Ordered,
    },
    DateFieldRule {
        field: "createdAt",
        role: DateRole::Recorded,
    },
    DateFieldRule {
        field: "updatedAt",
        role: DateRole::Updated,
    },
];

const DOCUMENT_DATES: &[DateFieldRule] = &[
    DateFieldRule {
        field: "receivedAt",
        role: DateRole::Received,
    },
    DateFieldRule {
        field: "uploadedAt",
        role: DateRole::Received,
    },
    DateFieldRule {
        field: "sentAt",
        role: DateRole::Sent,
    },
    DateFieldRule {
        field: "documentDate",
        role: DateRole::Authored,
    },
    DateFieldRule {
        field: "createdAt",
        role: DateRole::Recorded,
    },
    DateFieldRule {
        field: "updatedAt",
        role: DateRole::Updated,
    },
];

const FORM_DATES: &[DateFieldRule] = &[
    DateFieldRule {
        field: "submittedAt",
        role: DateRole::Completed,
    },
    DateFieldRule {
        field: "completedAt",
        role: DateRole::Completed,
    },
    DateFieldRule {
        field: "sentAt",
        role: DateRole::Sent,
    },
    DateFieldRule {
        field: "createdAt",
        role: DateRole::Recorded,
    },
    DateFieldRule {
        field: "updatedAt",
        role: DateRole::Updated,
    },
];

const MESSAGE_DATES: &[DateFieldRule] = &[
    DateFieldRule {
        field: "sentAt",
        role: DateRole::Sent,
    },
    DateFieldRule {
        field: "receivedAt",
        role: DateRole::Received,
    },
    DateFieldRule {
        field: "readAt",
        role: DateRole::Observed,
    },
    DateFieldRule {
        field: "createdAt",
        role: DateRole::Recorded,
    },
    DateFieldRule {
        field: "updatedAt",
        role: DateRole::Updated,
    },
];

const ORDER_DATES: &[DateFieldRule] = &[
    DateFieldRule {
        field: "orderedAt",
        role: DateRole::Ordered,
    },
    DateFieldRule {
        field: "requestedAt",
        role: DateRole::Ordered,
    },
    DateFieldRule {
        field: "referredAt",
        role: DateRole::Ordered,
    },
    DateFieldRule {
        field: "scheduledAt",
        role: DateRole::ScheduledStart,
    },
    DateFieldRule {
        field: "dueOn",
        role: DateRole::Due,
    },
    DateFieldRule {
        field: "respondedAt",
        role: DateRole::Completed,
    },
    DateFieldRule {
        field: "completedAt",
        role: DateRole::Completed,
    },
    DateFieldRule {
        field: "createdAt",
        role: DateRole::Recorded,
    },
    DateFieldRule {
        field: "updatedAt",
        role: DateRole::Updated,
    },
];

/// Complete declared Hint filing table (16 confirmed rows and 10 unconfirmed rows).
pub const HINT_TABLE: &[HintRecordRow] = &[
    // --- Confirmed rows ---
    HintRecordRow {
        kind: KnownHintKind::Patient,
        source_name: "patient",
        bucket: ChartBucket::Patient,
        confirmation: ConfirmationStatus::Confirmed,
        date_fields: PATIENT_DATES,
        status_paths: &["status", "membership_status", "membershipStatus"],
        code_paths: &["mrn", "identifiers"],
        title_paths: &["preferred_name", "full_name", "name"],
        redaction: RedactionRule::DropPatientBilling,
        link_rule: HintLinkRule::DirectPatientChart,
    },
    HintRecordRow {
        kind: KnownHintKind::Diagnosis,
        source_name: "diagnoses",
        bucket: ChartBucket::Problems,
        confirmation: ConfirmationStatus::Confirmed,
        date_fields: DIAGNOSIS_DATES,
        status_paths: &["status", "clinicalStatus"],
        code_paths: &["icd10Code", "snomedCode", "code", "codes"],
        title_paths: &["description", "name", "title"],
        redaction: RedactionRule::None,
        link_rule: HintLinkRule::ChartFallback {
            reason: "Hint problem-list entries live inside the patient chart summary without a standalone per-diagnosis URL.",
        },
    },
    HintRecordRow {
        kind: KnownHintKind::Allergy,
        source_name: "allergies",
        bucket: ChartBucket::Allergies,
        confirmation: ConfirmationStatus::Confirmed,
        date_fields: ALLERGY_DATES,
        status_paths: &["status", "clinicalStatus"],
        code_paths: &["rxnormCode", "allergenCode", "snomedCode", "code", "codes"],
        title_paths: &["allergen", "substance", "description", "name"],
        redaction: RedactionRule::None,
        link_rule: HintLinkRule::ChartFallback {
            reason: "Hint allergy entries live inside the patient chart summary without a standalone per-allergy URL.",
        },
    },
    HintRecordRow {
        kind: KnownHintKind::Medication,
        source_name: "medications",
        bucket: ChartBucket::Medications,
        confirmation: ConfirmationStatus::Confirmed,
        date_fields: MEDICATION_DATES,
        status_paths: &["status"],
        code_paths: &["rxnormCode", "ndcCode", "ndc", "code", "codes"],
        title_paths: &["name", "medicationName", "drugName", "description"],
        redaction: RedactionRule::DropRxPricingAndInvoice,
        link_rule: HintLinkRule::ChartFallback {
            reason: "Hint medication-list entries live inside the patient chart summary without a standalone per-medication URL.",
        },
    },
    HintRecordRow {
        kind: KnownHintKind::FreeformMedication,
        source_name: "freeform-medications",
        bucket: ChartBucket::Medications,
        confirmation: ConfirmationStatus::Confirmed,
        date_fields: MEDICATION_DATES,
        status_paths: &["status"],
        code_paths: &["rxnormCode", "code", "codes"],
        title_paths: &["name", "description", "text"],
        redaction: RedactionRule::DropRxPricingAndInvoice,
        link_rule: HintLinkRule::ChartFallback {
            reason: "Hint freeform medication entries live inside the patient chart summary without a standalone URL.",
        },
    },
    HintRecordRow {
        kind: KnownHintKind::InHouseRx,
        source_name: "in_house_rx",
        bucket: ChartBucket::Medications,
        confirmation: ConfirmationStatus::Confirmed,
        date_fields: MEDICATION_DATES,
        status_paths: &["status", "dispenseStatus"],
        code_paths: &["ndc", "ndcCode", "rxnormCode", "code", "codes"],
        title_paths: &["medicationName", "drugName", "name", "title"],
        redaction: RedactionRule::DropRxPricingAndInvoice,
        link_rule: HintLinkRule::NoteWhenPresentElseChart {
            fallback_reason: "In-house prescriptions without a backing note ID open on the patient chart.",
        },
    },
    HintRecordRow {
        kind: KnownHintKind::Surgery,
        source_name: "surgeries",
        bucket: ChartBucket::Procedures,
        confirmation: ConfirmationStatus::Confirmed,
        date_fields: SURGERY_DATES,
        status_paths: &["status"],
        code_paths: &["cptCode", "snomedCode", "code", "codes"],
        title_paths: &["description", "procedure", "name", "title"],
        redaction: RedactionRule::None,
        link_rule: HintLinkRule::ChartFallback {
            reason: "Hint surgical-history entries live inside the patient chart summary without a standalone URL.",
        },
    },
    HintRecordRow {
        kind: KnownHintKind::SocialHistory,
        source_name: "social-history",
        bucket: ChartBucket::History,
        confirmation: ConfirmationStatus::Confirmed,
        date_fields: HISTORY_DATES,
        status_paths: &["status"],
        code_paths: &["snomedCode", "loincCode", "code", "codes"],
        title_paths: &["category", "description", "name", "title"],
        redaction: RedactionRule::None,
        link_rule: HintLinkRule::ChartFallback {
            reason: "Hint social-history entries live inside the patient chart summary without a standalone URL.",
        },
    },
    HintRecordRow {
        kind: KnownHintKind::FamilyHistory,
        source_name: "family-history",
        bucket: ChartBucket::History,
        confirmation: ConfirmationStatus::Confirmed,
        date_fields: HISTORY_DATES,
        status_paths: &["status"],
        code_paths: &["icd10Code", "snomedCode", "code", "codes"],
        title_paths: &["condition", "description", "relationship", "name"],
        redaction: RedactionRule::None,
        link_rule: HintLinkRule::ChartFallback {
            reason: "Hint family-history entries live inside the patient chart summary without a standalone URL.",
        },
    },
    HintRecordRow {
        kind: KnownHintKind::Vaccine,
        source_name: "vaccines",
        bucket: ChartBucket::Immunizations,
        confirmation: ConfirmationStatus::Confirmed,
        date_fields: VACCINE_DATES,
        status_paths: &["status"],
        code_paths: &["cvxCode", "cptCode", "mvxCode", "code", "codes"],
        title_paths: &["vaccineName", "name", "description"],
        redaction: RedactionRule::None,
        link_rule: HintLinkRule::ChartFallback {
            reason: "Hint immunization entries live inside the patient chart summary without a standalone URL.",
        },
    },
    HintRecordRow {
        kind: KnownHintKind::GeneralInfo,
        source_name: "general",
        bucket: ChartBucket::Other,
        confirmation: ConfirmationStatus::Confirmed,
        date_fields: GENERAL_DATES,
        status_paths: &["status"],
        code_paths: &["code", "codes"],
        title_paths: &["label", "title", "description", "name"],
        redaction: RedactionRule::None,
        link_rule: HintLinkRule::ChartFallback {
            reason: "Hint general-info entries live inside the patient chart summary without a standalone URL.",
        },
    },
    HintRecordRow {
        kind: KnownHintKind::Vitals,
        source_name: "vitals",
        bucket: ChartBucket::Vitals,
        confirmation: ConfirmationStatus::Confirmed,
        date_fields: VITALS_DATES,
        status_paths: &["status"],
        code_paths: &["loincCode", "code", "codes"],
        title_paths: &["name", "vitalType", "title"],
        redaction: RedactionRule::None,
        link_rule: HintLinkRule::NoteWhenPresentElseChart {
            fallback_reason: "Standalone Hint vitals entries without an encounter note ID open on the patient chart.",
        },
    },
    HintRecordRow {
        kind: KnownHintKind::ClinicalNote,
        source_name: "note",
        bucket: ChartBucket::Notes,
        confirmation: ConfirmationStatus::Confirmed,
        date_fields: NOTE_DATES,
        status_paths: &["status", "signStatus", "state"],
        code_paths: &["cptCodes", "icd10Codes", "codes"],
        title_paths: &["title", "subject", "noteType"],
        redaction: RedactionRule::DropRxPricingAndInvoice,
        link_rule: HintLinkRule::DirectNote,
    },
    HintRecordRow {
        kind: KnownHintKind::StickyNote,
        source_name: "sticky_note",
        bucket: ChartBucket::Notes,
        confirmation: ConfirmationStatus::Confirmed,
        date_fields: NOTE_DATES,
        status_paths: &["status"],
        code_paths: &["codes"],
        title_paths: &["title", "subject", "text"],
        redaction: RedactionRule::None,
        link_rule: HintLinkRule::NoteWhenPresentElseChart {
            fallback_reason: "Patient-header sticky notes without a dedicated note route open on the patient chart.",
        },
    },
    HintRecordRow {
        kind: KnownHintKind::Task,
        source_name: "tasks",
        bucket: ChartBucket::Other,
        confirmation: ConfirmationStatus::Confirmed,
        date_fields: TASK_DATES,
        status_paths: &["status"],
        code_paths: &["codes"],
        title_paths: &["title", "description", "summary"],
        redaction: RedactionRule::None,
        link_rule: HintLinkRule::ChartFallback {
            reason: "Hint chart tasks open inside the patient chart drawer without a standalone per-task page URL.",
        },
    },
    HintRecordRow {
        kind: KnownHintKind::Appointment,
        source_name: "appointment",
        bucket: ChartBucket::Visits,
        confirmation: ConfirmationStatus::Confirmed,
        date_fields: APPOINTMENT_DATES,
        status_paths: &["status", "state"],
        code_paths: &["serviceTypeCode", "cptCodes", "codes"],
        title_paths: &["title", "appointmentType", "reason"],
        redaction: RedactionRule::None,
        link_rule: HintLinkRule::DirectAppointment,
    },
    // --- Unconfirmed rows at the start ---
    HintRecordRow {
        kind: KnownHintKind::Lab,
        source_name: "note:lab",
        bucket: ChartBucket::Results,
        confirmation: ConfirmationStatus::Unconfirmed,
        date_fields: LAB_DATES,
        status_paths: &["status", "resultStatus", "labOrder.status"],
        code_paths: &[
            "loincCode",
            "panelCode",
            "labOrder.loincCode",
            "labResults.loincCode",
            "codes",
        ],
        title_paths: &[
            "panelName",
            "testName",
            "labOrder.panelName",
            "title",
            "subject",
        ],
        redaction: RedactionRule::None,
        link_rule: HintLinkRule::NoteWhenPresentElseChart {
            fallback_reason: "Lab records without a backing Hint note ID open on the patient chart.",
        },
    },
    HintRecordRow {
        kind: KnownHintKind::Fax,
        source_name: "fax",
        bucket: ChartBucket::Documents,
        confirmation: ConfirmationStatus::Unconfirmed,
        date_fields: DOCUMENT_DATES,
        status_paths: &["status", "transmissionStatus"],
        code_paths: &["codes"],
        title_paths: &["subject", "filename", "title"],
        redaction: RedactionRule::None,
        link_rule: HintLinkRule::FileWhenPresentElseChart {
            fallback_reason: "Fax items without a dedicated patient-file route open on the patient chart.",
        },
    },
    HintRecordRow {
        kind: KnownHintKind::Form,
        source_name: "form",
        bucket: ChartBucket::Forms,
        confirmation: ConfirmationStatus::Unconfirmed,
        date_fields: FORM_DATES,
        status_paths: &["status"],
        code_paths: &["formCode", "codes"],
        title_paths: &["formName", "title", "name"],
        redaction: RedactionRule::None,
        link_rule: HintLinkRule::NoteWhenPresentElseChart {
            fallback_reason: "Questionnaire submissions without a backing note route open on the patient chart.",
        },
    },
    HintRecordRow {
        kind: KnownHintKind::ERx,
        source_name: "erx",
        bucket: ChartBucket::Medications,
        confirmation: ConfirmationStatus::Unconfirmed,
        date_fields: MEDICATION_DATES,
        status_paths: &["status", "transmissionStatus"],
        code_paths: &["rxnormCode", "ndcCode", "ndc", "code", "codes"],
        title_paths: &["medicationName", "drugName", "name", "title"],
        redaction: RedactionRule::DropRxPricingAndInvoice,
        link_rule: HintLinkRule::NoteWhenPresentElseChart {
            fallback_reason: "External eRx records without a Hint note URL open on the patient chart.",
        },
    },
    HintRecordRow {
        kind: KnownHintKind::Message,
        source_name: "messages",
        bucket: ChartBucket::Messages,
        confirmation: ConfirmationStatus::Unconfirmed,
        date_fields: MESSAGE_DATES,
        status_paths: &["status", "deliveryStatus"],
        code_paths: &["codes"],
        title_paths: &["subject", "channel", "title"],
        redaction: RedactionRule::None,
        link_rule: HintLinkRule::DirectMessage,
    },
    HintRecordRow {
        kind: KnownHintKind::PatientFile,
        source_name: "patient-files",
        bucket: ChartBucket::Documents,
        confirmation: ConfirmationStatus::Unconfirmed,
        date_fields: DOCUMENT_DATES,
        status_paths: &["status", "reviewStatus"],
        code_paths: &["categoryCode", "codes"],
        title_paths: &["filename", "name", "title"],
        redaction: RedactionRule::None,
        link_rule: HintLinkRule::DirectFile,
    },
    HintRecordRow {
        kind: KnownHintKind::EConsult,
        source_name: "econsult",
        bucket: ChartBucket::Orders,
        confirmation: ConfirmationStatus::Unconfirmed,
        date_fields: ORDER_DATES,
        status_paths: &["status"],
        code_paths: &["specialtyCode", "cptCode", "codes"],
        title_paths: &["specialty", "title", "reason"],
        redaction: RedactionRule::None,
        link_rule: HintLinkRule::NoteWhenPresentElseChart {
            fallback_reason: "eConsult records without a dedicated Hint note route open on the patient chart.",
        },
    },
    HintRecordRow {
        kind: KnownHintKind::Referral,
        source_name: "referral",
        bucket: ChartBucket::Orders,
        confirmation: ConfirmationStatus::Unconfirmed,
        date_fields: ORDER_DATES,
        status_paths: &["status"],
        code_paths: &["specialtyCode", "icd10Codes", "codes"],
        title_paths: &["referredTo", "specialty", "title", "reason"],
        redaction: RedactionRule::None,
        link_rule: HintLinkRule::NoteWhenPresentElseChart {
            fallback_reason: "Referral records without a dedicated Hint note route open on the patient chart.",
        },
    },
    HintRecordRow {
        kind: KnownHintKind::Order,
        source_name: "order",
        bucket: ChartBucket::Orders,
        confirmation: ConfirmationStatus::Unconfirmed,
        date_fields: ORDER_DATES,
        status_paths: &["status", "orderStatus"],
        code_paths: &["loincCode", "cptCode", "code", "codes"],
        title_paths: &["orderName", "description", "title"],
        redaction: RedactionRule::None,
        link_rule: HintLinkRule::NoteWhenPresentElseChart {
            fallback_reason: "Orders without a dedicated Hint note route open on the patient chart.",
        },
    },
    HintRecordRow {
        kind: KnownHintKind::PartnerRecord,
        source_name: "partner_record",
        bucket: ChartBucket::Other,
        confirmation: ConfirmationStatus::Unconfirmed,
        date_fields: GENERAL_DATES,
        status_paths: &["status"],
        code_paths: &["code", "codes"],
        title_paths: &["partnerName", "title", "name"],
        redaction: RedactionRule::None,
        link_rule: HintLinkRule::ChartFallback {
            reason: "Partner integration records have no standalone per-record URL in Hint and open on the patient chart.",
        },
    },
];

/// Returns the complete Hint filing table.
pub const fn hint_table() -> &'static [HintRecordRow] {
    HINT_TABLE
}

/// Returns the declared table row for `kind`.
pub fn row_for_kind(kind: KnownHintKind) -> &'static HintRecordRow {
    HINT_TABLE
        .iter()
        .find(|row| row.kind == kind)
        .unwrap_or(&HINT_TABLE[0])
}

/// Returns all confirmed rows in [`HINT_TABLE`].
pub fn confirmed_rows() -> Vec<&'static HintRecordRow> {
    HINT_TABLE
        .iter()
        .filter(|row| row.confirmation == ConfirmationStatus::Confirmed)
        .collect()
}

/// Returns all unconfirmed rows in [`HINT_TABLE`].
pub fn unconfirmed_rows() -> Vec<&'static HintRecordRow> {
    HINT_TABLE
        .iter()
        .filter(|row| row.confirmation == ConfirmationStatus::Unconfirmed)
        .collect()
}

/// Reports every declared Hint record kind that falls back (always or when lacking a note/file ID)
/// to the patient chart URL.
pub fn chart_fallback_kinds() -> Vec<FallbackKindReport> {
    HINT_TABLE
        .iter()
        .filter_map(|row| match row.link_rule {
            HintLinkRule::ChartFallback { reason } => Some(FallbackKindReport {
                source_name: row.source_name,
                bucket: row.bucket,
                always_falls_back: true,
                disclosure: reason,
            }),
            HintLinkRule::NoteWhenPresentElseChart { fallback_reason }
            | HintLinkRule::FileWhenPresentElseChart { fallback_reason } => {
                Some(FallbackKindReport {
                    source_name: row.source_name,
                    bucket: row.bucket,
                    always_falls_back: false,
                    disclosure: fallback_reason,
                })
            }
            HintLinkRule::DirectPatientChart
            | HintLinkRule::DirectNote
            | HintLinkRule::DirectAppointment
            | HintLinkRule::DirectFile
            | HintLinkRule::DirectMessage => None,
        })
        .collect()
}

/// Outcome of one named Hint connector read.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "outcome", rename_all = "snake_case")]
pub enum HintReadCapture {
    /// Complete read with its JSON body and SHA-256 hex digest.
    Complete {
        /// Parsed JSON response body.
        body: Value,
        /// SHA-256 hex digest of the exact JSON text.
        sha256: String,
    },
    /// Multi-page read that obtained at least one page before a later page failed.
    Partial {
        /// Partial JSON response body obtained before the fault.
        body: Value,
        /// SHA-256 hex digest of the partial JSON text.
        sha256: String,
        /// Fault code explaining why pagination ended early.
        reason: String,
    },
    /// Response whose JSON shape differed from the Hint reader's validator.
    Changed {
        /// Validator reason code.
        reason: String,
        /// SHA-256 hex digest of the unaccepted response bytes.
        sha256: String,
    },
    /// Read failed without producing usable chart records.
    Failed {
        /// Connector or HTTP fault code.
        reason: String,
        /// Optional HTTP status when an upstream response arrived.
        http_status: Option<u16>,
    },
}

/// Standing of one named read after filing, keeping empty, populated, partial,
/// changed, and failed reads distinct.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "standing", rename_all = "snake_case")]
pub enum ReadStanding {
    /// Read completed cleanly and Hint held zero records in this section.
    Empty,
    /// Read completed cleanly and yielded `record_count` filed records.
    Populated {
        /// Number of records filed from this read.
        record_count: usize,
    },
    /// Read stopped mid-pagination; `record_count` records were filed, with `reason` recorded.
    Partial {
        /// Number of records filed from completed pages.
        record_count: usize,
        /// Why remaining pages were not acquired.
        reason: String,
    },
    /// Hint's response shape changed; zero records were admitted as complete.
    Changed {
        /// Shape-mismatch reason.
        reason: String,
    },
    /// Read failed; never treated as empty.
    Failed {
        /// Failure reason code.
        reason: String,
        /// Optional upstream HTTP status.
        http_status: Option<u16>,
    },
}

impl ReadStanding {
    /// True only when the read finished completely (`Empty` or `Populated`).
    pub const fn is_complete(&self) -> bool {
        matches!(self, Self::Empty | Self::Populated { .. })
    }
}

/// Notice emitted whenever an unrecognized Hint record kind or note category is filed into `Other`.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct UnknownRecordNotice {
    /// Raw Hint read or note-category name that was not in [`HINT_TABLE`].
    pub source_kind: String,
    /// Candidate identifier assigned in `Other`.
    pub candidate_id: String,
    /// Source record identifier.
    pub record_id: String,
}

/// Captured Hint reads for a single patient.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct HintPatientCapture {
    /// Named section reads keyed by Hint read name (`"patient"`, `"diagnoses"`, `"notes"`, etc.).
    pub reads: BTreeMap<String, HintReadCapture>,
    /// Full-text individual note reads (`read = "note"`) keyed by `note_id`.
    pub full_notes: BTreeMap<String, HintReadCapture>,
}

/// One filed Hint record with both its typed Hint envelope and its validated [`SourceRevision`].
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FiledHintRecord {
    /// Destination Coda bucket.
    pub bucket: ChartBucket,
    /// Resolved Hint record kind (`Known` or `Unknown`).
    pub kind: HintKind,
    /// Hint's own name for this record kind (including category when categorized).
    pub source_kind: String,
    /// Whether this row's shape is `Confirmed` or `Unconfirmed` (or `Unconfirmed` for `Unknown`).
    pub confirmation: ConfirmationStatus,
    /// Hint's native record ID.
    pub record_id: String,
    /// Record version (source version when Hint supplies one, else SHA-256 of retained content).
    pub version: String,
    /// Extracted dates with their table roles and exact Hint strings.
    pub dates: Vec<FiledDate>,
    /// Record status in Hint's exact words, if present.
    pub status: Option<String>,
    /// Clinical or administrative codes found on the record.
    pub codes: Vec<RecordCode>,
    /// Record title, if present.
    pub title: Option<String>,
    /// Deep link into Hint (or chart fallback with explicit disclosure).
    pub link: HintLink,
    /// Exact JSON value retained after applying only the row's [`RedactionRule`].
    pub exact_record: Value,
    /// Validated [`SourceRevision`] admitted to the patient workspace.
    pub revision: SourceRevision,
}

/// Result of filing a captured Hint patient chart.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FiledHintChart {
    /// Patient identifier in Hint.
    pub patient_id: String,
    /// Filed records in deterministic candidate order.
    pub records: Vec<FiledHintRecord>,
    /// Per-read completeness standing (`Empty`, `Populated`, `Partial`, `Changed`, `Failed`).
    pub read_standings: BTreeMap<String, ReadStanding>,
    /// Full-note completeness standing keyed by `note_id`.
    pub note_standings: BTreeMap<String, ReadStanding>,
    /// Note IDs from a complete `notes` summary read whose full-note bodies have not been captured
    /// and whose summary items carry no inline readable text.
    pub missing_note_reads: Vec<String>,
    /// Unrecognized Hint record kinds that were filed into [`ChartBucket::Other`].
    pub unknown_kinds: Vec<UnknownRecordNotice>,
}

impl FiledHintChart {
    /// Validates whole-chart completeness: every read in [`HINT_WHOLE_CHART_READS`] is present and
    /// [`ReadStanding::is_complete`], no additional captured section or full-note read is
    /// incomplete, and every note in a complete `notes` list either has a complete `full_notes`
    /// read or inline readable text.
    pub fn validate_whole_chart(&self) -> Result<(), HintFilingError> {
        let mut issues = Vec::new();
        for &required in &HINT_WHOLE_CHART_READS {
            match self.read_standings.get(required) {
                None => issues.push(format!("missing_read:{required}")),
                Some(standing) if !standing.is_complete() => {
                    issues.push(format!("{required}={standing:?}"));
                }
                Some(_) => {}
            }
        }
        for (read_name, standing) in &self.read_standings {
            if !HINT_WHOLE_CHART_READS.contains(&read_name.as_str()) && !standing.is_complete() {
                issues.push(format!("{read_name}={standing:?}"));
            }
        }
        for note_id in &self.missing_note_reads {
            issues.push(format!("missing_note:{note_id}"));
        }
        for (note_id, standing) in &self.note_standings {
            if !standing.is_complete() {
                issues.push(format!("note:{note_id}={standing:?}"));
            }
        }
        if issues.is_empty() {
            Ok(())
        } else {
            Err(HintFilingError::IncompleteCapture {
                patient_id: self.patient_id.clone(),
                details: issues.join(", "),
            })
        }
    }

    /// True when [`Self::validate_whole_chart`] succeeds.
    pub fn is_complete(&self) -> bool {
        self.validate_whole_chart().is_ok()
    }

    /// Groups filed records by [`ChartBucket`].
    pub fn by_bucket(&self) -> BTreeMap<ChartBucket, Vec<&FiledHintRecord>> {
        let mut grouped: BTreeMap<ChartBucket, Vec<&FiledHintRecord>> = BTreeMap::new();
        for record in &self.records {
            grouped.entry(record.bucket).or_default().push(record);
        }
        grouped
    }
}

/// Errors produced when validating or filing Hint records.
#[derive(Debug, thiserror::Error)]
pub enum HintFilingError {
    /// A Hint record was not a JSON object or lacked a usable identifier.
    #[error("Hint record in `{read}` is malformed: {reason}")]
    MalformedRecord {
        /// Read name where the malformed record appeared.
        read: String,
        /// Structural reason.
        reason: String,
    },
    /// Serializing the redacted Hint record failed.
    #[error("failed to serialize Hint record `{record_id}`")]
    Serialize {
        /// Record ID.
        record_id: String,
        /// Underlying JSON error.
        #[source]
        source: serde_json::Error,
    },
    /// `SourceRevision::validate` rejected the constructed revision.
    #[error("invalid SourceRevision for `{candidate_id}`: {reason}")]
    InvalidRevision {
        /// Candidate ID.
        candidate_id: String,
        /// Validation error detail.
        reason: String,
    },
    /// Whole-chart read through `PatientSource` encountered incomplete or failed reads.
    #[error("Hint chart capture for `{patient_id}` is incomplete: {details}")]
    IncompleteCapture {
        /// Patient ID.
        patient_id: String,
        /// Summary of partial, changed, or failed reads.
        details: String,
    },
}

fn lookup_path<'a>(value: &'a Value, path: &str) -> Option<&'a Value> {
    let mut cursor = value;
    for segment in path.split('.') {
        cursor = cursor.as_object()?.get(segment)?;
    }
    Some(cursor)
}

fn scalar_string(value: &Value) -> Option<String> {
    match value {
        Value::String(s) if !s.trim().is_empty() => Some(s.clone()),
        Value::Number(n) => Some(n.to_string()),
        _ => None,
    }
}

fn apply_redaction(value: &Value, rule: RedactionRule) -> Value {
    let drop_set: BTreeSet<&str> = match rule {
        RedactionRule::None => return value.clone(),
        RedactionRule::DropPatientBilling => PATIENT_BILLING_FIELDS.iter().copied().collect(),
        RedactionRule::DropRxPricingAndInvoice => RX_PRICE_INVOICE_FIELDS.iter().copied().collect(),
    };
    strip_fields_recursive(value, &drop_set)
}

fn strip_fields_recursive(value: &Value, drop_set: &BTreeSet<&str>) -> Value {
    match value {
        Value::Array(items) => Value::Array(
            items
                .iter()
                .map(|item| strip_fields_recursive(item, drop_set))
                .collect(),
        ),
        Value::Object(map) => {
            let mut out = Map::new();
            for (key, val) in map {
                if drop_set.contains(key.as_str()) {
                    continue;
                }
                out.insert(key.clone(), strip_fields_recursive(val, drop_set));
            }
            Value::Object(out)
        }
        other => other.clone(),
    }
}

fn extract_dates(record: &Value, rules: &[DateFieldRule]) -> Vec<FiledDate> {
    let mut seen = BTreeSet::new();
    let mut out = Vec::new();
    for rule in rules {
        if let Some(found) = lookup_path(record, rule.field)
            && let Some(text) = scalar_string(found)
            && seen.insert((rule.field, text.clone()))
        {
            out.push(FiledDate {
                role: rule.role,
                field: rule.field.to_owned(),
                value: text,
            });
        }
    }
    if rules.is_empty()
        && let Some(obj) = record.as_object()
    {
        for (key, val) in obj {
            let lower = key.to_ascii_lowercase();
            if (lower.ends_with("date") || lower.ends_with("at") || lower == "dob")
                && let Some(text) = scalar_string(val)
            {
                out.push(FiledDate {
                    role: DateRole::Observed,
                    field: key.clone(),
                    value: text,
                });
            }
        }
    }
    out
}

fn extract_first_string(record: &Value, paths: &[&str]) -> Option<String> {
    for path in paths {
        if let Some(found) = lookup_path(record, path)
            && let Some(text) = scalar_string(found)
        {
            return Some(text);
        }
    }
    None
}

fn extract_patient_title(record: &Value) -> Option<String> {
    let first = record
        .get("first_name")
        .or_else(|| record.get("firstName"))
        .and_then(Value::as_str)
        .unwrap_or_default()
        .trim();
    let last = record
        .get("last_name")
        .or_else(|| record.get("lastName"))
        .and_then(Value::as_str)
        .unwrap_or_default()
        .trim();
    let combined = format!("{first} {last}").trim().to_owned();
    if !combined.is_empty() {
        return Some(combined);
    }
    extract_first_string(record, &["preferred_name", "full_name", "name"])
}

fn collect_codes_from_value(
    field: &str,
    value: &Value,
    out: &mut Vec<RecordCode>,
    seen: &mut BTreeSet<(String, String)>,
) {
    match value {
        Value::String(s) if !s.trim().is_empty() => {
            if seen.insert((field.to_owned(), s.clone())) {
                out.push(RecordCode {
                    field: field.to_owned(),
                    code: s.clone(),
                });
            }
        }
        Value::Array(items) => {
            for item in items {
                collect_codes_from_value(field, item, out, seen);
            }
        }
        Value::Object(map) => {
            for key in [
                "code",
                "value",
                "icd10Code",
                "loincCode",
                "rxnormCode",
                "snomedCode",
                "cptCode",
            ] {
                if let Some(inner) = map.get(key)
                    && let Some(code) = scalar_string(inner)
                {
                    let system = map.get("system").and_then(Value::as_str).unwrap_or(field);
                    if seen.insert((system.to_owned(), code.clone())) {
                        out.push(RecordCode {
                            field: system.to_owned(),
                            code,
                        });
                    }
                }
            }
        }
        _ => {}
    }
}

fn extract_codes(record: &Value, paths: &[&str]) -> Vec<RecordCode> {
    let mut out = Vec::new();
    let mut seen = BTreeSet::new();
    for path in paths {
        if let Some(found) = lookup_path(record, path) {
            collect_codes_from_value(path, found, &mut out, &mut seen);
        }
        if let Some((prefix, leaf)) = path.split_once('.')
            && let Some(Value::Array(items)) = record.get(prefix)
        {
            for item in items {
                if let Some(found) = lookup_path(item, leaf) {
                    collect_codes_from_value(path, found, &mut out, &mut seen);
                }
            }
        }
    }
    out
}

/// Builds the Hint link for a record using its [`HintLinkRule`].
pub fn build_hint_link(
    base_url: &str,
    patient_id: &str,
    record_id: &str,
    record: &Value,
    rule: HintLinkRule,
) -> HintLink {
    let base = base_url.trim_end_matches('/');
    let chart_url = format!("{base}/patients/{patient_id}");
    let note_id = record
        .get("noteId")
        .or_else(|| record.get("note_id"))
        .and_then(Value::as_str)
        .filter(|s| !s.is_empty());
    let file_id = record
        .get("fileId")
        .or_else(|| record.get("file_id"))
        .and_then(Value::as_str)
        .filter(|s| !s.is_empty());
    let has_own_note_route = record
        .get("hasNoteRoute")
        .and_then(Value::as_bool)
        .unwrap_or(false);

    match rule {
        HintLinkRule::DirectPatientChart => HintLink::DirectRecord { url: chart_url },
        HintLinkRule::DirectNote => HintLink::DirectRecord {
            url: format!("{chart_url}/notes/{record_id}"),
        },
        HintLinkRule::DirectAppointment => HintLink::DirectRecord {
            url: format!("{chart_url}/appointments/{record_id}"),
        },
        HintLinkRule::DirectFile => HintLink::DirectRecord {
            url: format!("{chart_url}/files/{record_id}"),
        },
        HintLinkRule::DirectMessage => HintLink::DirectRecord {
            url: format!("{chart_url}/messages/{record_id}"),
        },
        HintLinkRule::NoteWhenPresentElseChart { fallback_reason } => {
            if let Some(nid) = note_id {
                HintLink::DirectRecord {
                    url: format!("{chart_url}/notes/{nid}"),
                }
            } else if has_own_note_route {
                HintLink::DirectRecord {
                    url: format!("{chart_url}/notes/{record_id}"),
                }
            } else {
                HintLink::ChartFallback {
                    url: chart_url,
                    disclosure: fallback_reason.to_owned(),
                }
            }
        }
        HintLinkRule::FileWhenPresentElseChart { fallback_reason } => {
            if let Some(fid) = file_id {
                HintLink::DirectRecord {
                    url: format!("{chart_url}/files/{fid}"),
                }
            } else {
                HintLink::ChartFallback {
                    url: chart_url,
                    disclosure: fallback_reason.to_owned(),
                }
            }
        }
        HintLinkRule::ChartFallback { reason } => HintLink::ChartFallback {
            url: chart_url,
            disclosure: reason.to_owned(),
        },
    }
}

/// Classifies a Hint note object using Hint's own `noteType` / `category` / `type` fields
/// (`intent/physician.md`: "Where a source labels a record with a category, the category is part
/// of the source's name for it").
pub fn classify_note_record(record: &Value) -> (HintKind, String) {
    let raw_category = record
        .get("noteType")
        .or_else(|| record.get("note_type"))
        .or_else(|| record.get("category"))
        .or_else(|| record.get("type"))
        .and_then(Value::as_str)
        .map(str::trim)
        .filter(|s| !s.is_empty());

    let Some(cat) = raw_category else {
        if record.get("labResults").is_some() || record.get("labOrder").is_some() {
            return (HintKind::Known(KnownHintKind::Lab), "note:lab".to_owned());
        }
        return (
            HintKind::Known(KnownHintKind::ClinicalNote),
            "note".to_owned(),
        );
    };

    let normalized = cat.to_ascii_lowercase().replace('-', "_");
    match normalized.as_str() {
        "encounter" | "clinical" | "clinical_note" | "visit" | "visit_note" | "phone"
        | "phone_note" | "chart" | "chart_note" | "progress" | "soap" | "note" => (
            HintKind::Known(KnownHintKind::ClinicalNote),
            format!("note:{normalized}"),
        ),
        "sticky" | "sticky_note" => (
            HintKind::Known(KnownHintKind::StickyNote),
            "sticky_note".to_owned(),
        ),
        "in_house_rx" | "inhouserx" | "dispense" | "in_house_prescription" => (
            HintKind::Known(KnownHintKind::InHouseRx),
            "in_house_rx".to_owned(),
        ),
        "lab" | "lab_result" | "lab_note" | "laboratory" => {
            (HintKind::Known(KnownHintKind::Lab), "note:lab".to_owned())
        }
        "fax" => (HintKind::Known(KnownHintKind::Fax), "note:fax".to_owned()),
        "form" | "questionnaire" | "intake_form" => {
            (HintKind::Known(KnownHintKind::Form), "note:form".to_owned())
        }
        "erx" | "e_rx" | "eprescription" => {
            (HintKind::Known(KnownHintKind::ERx), "note:erx".to_owned())
        }
        "econsult" | "e_consult" => (
            HintKind::Known(KnownHintKind::EConsult),
            "note:econsult".to_owned(),
        ),
        "referral" => (
            HintKind::Known(KnownHintKind::Referral),
            "note:referral".to_owned(),
        ),
        "order" => (
            HintKind::Known(KnownHintKind::Order),
            "note:order".to_owned(),
        ),
        "vitals" | "vital_signs" => (
            HintKind::Known(KnownHintKind::Vitals),
            "note:vitals".to_owned(),
        ),
        "partner" | "partner_record" | "integration_record" => (
            HintKind::Known(KnownHintKind::PartnerRecord),
            "partner_record".to_owned(),
        ),
        other => {
            let source_name = format!("note:{other}");
            (HintKind::Unknown(source_name.clone()), source_name)
        }
    }
}

/// Classifies a top-level Hint read/section name and individual item into a [`HintKind`] and source kind name.
pub fn classify_hint_item(read_name: &str, record: &Value) -> (HintKind, String) {
    let normalized = read_name.trim().to_ascii_lowercase();
    match normalized.as_str() {
        "patient" => (
            HintKind::Known(KnownHintKind::Patient),
            "patient".to_owned(),
        ),
        "diagnoses" | "diagnosis" => (
            HintKind::Known(KnownHintKind::Diagnosis),
            "diagnoses".to_owned(),
        ),
        "allergies" | "allergy" => (
            HintKind::Known(KnownHintKind::Allergy),
            "allergies".to_owned(),
        ),
        "medications" | "medication" => (
            HintKind::Known(KnownHintKind::Medication),
            "medications".to_owned(),
        ),
        "freeform-medications" | "freeform_medications" => (
            HintKind::Known(KnownHintKind::FreeformMedication),
            "freeform-medications".to_owned(),
        ),
        "in_house_rx" | "in-house-rx" => (
            HintKind::Known(KnownHintKind::InHouseRx),
            "in_house_rx".to_owned(),
        ),
        "surgeries" | "surgery" => (
            HintKind::Known(KnownHintKind::Surgery),
            "surgeries".to_owned(),
        ),
        "social-history" | "social_history" => (
            HintKind::Known(KnownHintKind::SocialHistory),
            "social-history".to_owned(),
        ),
        "family-history" | "family_history" => (
            HintKind::Known(KnownHintKind::FamilyHistory),
            "family-history".to_owned(),
        ),
        "vaccines" | "vaccine" => (
            HintKind::Known(KnownHintKind::Vaccine),
            "vaccines".to_owned(),
        ),
        "general" | "general_info" => (
            HintKind::Known(KnownHintKind::GeneralInfo),
            "general".to_owned(),
        ),
        "vitals" => (HintKind::Known(KnownHintKind::Vitals), "vitals".to_owned()),
        "notes" | "note" => classify_note_record(record),
        "sticky_note" | "sticky-note" => (
            HintKind::Known(KnownHintKind::StickyNote),
            "sticky_note".to_owned(),
        ),
        "tasks" | "task" => (HintKind::Known(KnownHintKind::Task), "tasks".to_owned()),
        "appointments" | "appointment" => (
            HintKind::Known(KnownHintKind::Appointment),
            "appointment".to_owned(),
        ),
        "lab" | "labs" => (HintKind::Known(KnownHintKind::Lab), "note:lab".to_owned()),
        "fax" | "faxes" => (HintKind::Known(KnownHintKind::Fax), "fax".to_owned()),
        "form" | "forms" => (HintKind::Known(KnownHintKind::Form), "form".to_owned()),
        "erx" => (HintKind::Known(KnownHintKind::ERx), "erx".to_owned()),
        "messages" | "message" => (
            HintKind::Known(KnownHintKind::Message),
            "messages".to_owned(),
        ),
        "patient-files" | "patient_files" | "patient_file" | "files" => (
            HintKind::Known(KnownHintKind::PatientFile),
            "patient-files".to_owned(),
        ),
        "econsult" | "econsults" => (
            HintKind::Known(KnownHintKind::EConsult),
            "econsult".to_owned(),
        ),
        "referral" | "referrals" => (
            HintKind::Known(KnownHintKind::Referral),
            "referral".to_owned(),
        ),
        "order" | "orders" => (HintKind::Known(KnownHintKind::Order), "order".to_owned()),
        "partner_record" | "partner-records" | "partner_records" => (
            HintKind::Known(KnownHintKind::PartnerRecord),
            "partner_record".to_owned(),
        ),
        other => (HintKind::Unknown(other.to_owned()), other.to_owned()),
    }
}

fn extract_readable_parts(record: &Value) -> Vec<ReadablePart> {
    let mut parts = Vec::new();
    for (field, label) in [
        ("body", "body"),
        ("plainText", "plainText"),
        ("formattedText", "formattedText"),
        ("content", "content"),
        ("text", "text"),
        ("summary", "summary"),
    ] {
        if let Some(Value::String(text)) = record.get(field)
            && !text.trim().is_empty()
        {
            parts.push(ReadablePart {
                label: label.to_owned(),
                text: text.clone(),
            });
        }
    }
    if let Some(Value::Array(sections)) = record.get("sections") {
        for (idx, section) in sections.iter().enumerate() {
            if let Some(Value::String(text)) = section.get("text").or_else(|| section.get("body"))
                && !text.trim().is_empty()
            {
                let label = section
                    .get("heading")
                    .or_else(|| section.get("title"))
                    .and_then(Value::as_str)
                    .map(str::to_owned)
                    .unwrap_or_else(|| format!("section_{idx}"));
                parts.push(ReadablePart {
                    label,
                    text: text.clone(),
                });
            }
        }
    }
    parts
}

/// Files one Hint JSON record into its Coda bucket, redacting only declared billing or
/// Rx price/invoice fields, building its Hint link, and producing a validated [`SourceRevision`].
pub fn file_hint_record(
    workspace: &str,
    patient_id: &str,
    read_name: &str,
    raw_record: &Value,
) -> Result<FiledHintRecord, HintFilingError> {
    let Some(obj) = raw_record.as_object() else {
        return Err(HintFilingError::MalformedRecord {
            read: read_name.to_owned(),
            reason: "record must be a JSON object".to_owned(),
        });
    };
    let record_id =
        obj.get("id")
            .and_then(scalar_string)
            .ok_or_else(|| HintFilingError::MalformedRecord {
                read: read_name.to_owned(),
                reason: "record has no non-empty `id`".to_owned(),
            })?;

    let (kind, source_kind) = classify_hint_item(read_name, raw_record);
    let (bucket, confirmation, dates, status, codes, title, exact_record, link) = match &kind {
        HintKind::Known(known) => {
            let row = row_for_kind(*known);
            let redacted = apply_redaction(raw_record, row.redaction);
            let dates = extract_dates(&redacted, row.date_fields);
            let status = extract_first_string(&redacted, row.status_paths);
            let codes = extract_codes(&redacted, row.code_paths);
            let title = if *known == KnownHintKind::Patient {
                extract_patient_title(&redacted)
            } else {
                extract_first_string(&redacted, row.title_paths)
            };
            let link_record = if matches!(
                *known,
                KnownHintKind::Lab
                    | KnownHintKind::InHouseRx
                    | KnownHintKind::StickyNote
                    | KnownHintKind::Form
                    | KnownHintKind::ERx
                    | KnownHintKind::EConsult
                    | KnownHintKind::Referral
                    | KnownHintKind::Order
                    | KnownHintKind::Vitals
            ) && (read_name == "notes" || read_name == "note")
                && redacted.get("noteId").is_none()
                && redacted.get("note_id").is_none()
            {
                let mut augmented = redacted.clone();
                if let Some(m) = augmented.as_object_mut() {
                    m.insert("hasNoteRoute".to_owned(), Value::Bool(true));
                }
                build_hint_link(
                    HINT_APP_BASE_URL,
                    patient_id,
                    &record_id,
                    &augmented,
                    row.link_rule,
                )
            } else {
                build_hint_link(
                    HINT_APP_BASE_URL,
                    patient_id,
                    &record_id,
                    &redacted,
                    row.link_rule,
                )
            };
            (
                row.bucket,
                row.confirmation,
                dates,
                status,
                codes,
                title,
                redacted,
                link_record,
            )
        }
        HintKind::Unknown(_) => {
            let dates = extract_dates(raw_record, &[]);
            let status = extract_first_string(raw_record, &["status", "state"]);
            let codes = extract_codes(raw_record, &["code", "codes"]);
            let title =
                extract_first_string(raw_record, &["title", "name", "description", "label"]);
            let link = HintLink::ChartFallback {
                url: format!("{HINT_APP_BASE_URL}/patients/{patient_id}"),
                disclosure: format!(
                    "Unrecognized Hint record kind `{source_kind}` has no declared per-record URL and opens on the patient chart."
                ),
            };
            (
                ChartBucket::Other,
                ConfirmationStatus::Unconfirmed,
                dates,
                status,
                codes,
                title,
                raw_record.clone(),
                link,
            )
        }
    };

    let content =
        serde_json::to_string(&exact_record).map_err(|source| HintFilingError::Serialize {
            record_id: record_id.clone(),
            source,
        })?;
    let sha256 = sha256_hex(content.as_bytes());
    let version = exact_record
        .get("version")
        .or_else(|| exact_record.get("revision"))
        .or_else(|| exact_record.get("lockVersion"))
        .and_then(scalar_string)
        .map(|v| format!("{v}:{sha256}"))
        .unwrap_or_else(|| sha256.clone());

    let candidate_id = format!("{source_kind}/{record_id}");
    let record_dates: Vec<RecordDate> = dates
        .iter()
        .map(|d| RecordDate {
            name: d.field.clone(),
            value: d.value.clone(),
        })
        .collect();
    let readable_parts = extract_readable_parts(&exact_record);

    let provenance = json!({
        "bucket": bucket.as_str(),
        "source_kind": source_kind,
        "confirmation": confirmation,
        "source_id": HINT_SOURCE_ID,
        "record_id": record_id,
        "version": version,
        "status": status,
        "codes": codes,
        "title": title,
        "dated_roles": dates,
        "link": link,
    });

    let revision = SourceRevision {
        workspace: workspace.to_owned(),
        source_id: HINT_SOURCE_ID.to_owned(),
        candidate_id: candidate_id.clone(),
        revision: version.clone(),
        record_type: bucket.as_str().to_owned(),
        dates: record_dates,
        content,
        sha256,
        readable_parts,
        provenance,
    };
    revision
        .validate()
        .map_err(|reason| HintFilingError::InvalidRevision {
            candidate_id,
            reason,
        })?;

    Ok(FiledHintRecord {
        bucket,
        kind,
        source_kind,
        confirmation,
        record_id,
        version,
        dates,
        status,
        codes,
        title,
        link,
        exact_record,
        revision,
    })
}

fn items_in_section_body<'a>(
    read_name: &str,
    body: &'a Value,
) -> Result<Vec<&'a Value>, HintFilingError> {
    if read_name == "patient" {
        if !body.is_object() {
            return Err(HintFilingError::MalformedRecord {
                read: read_name.to_owned(),
                reason: "patient payload must be a JSON object".to_owned(),
            });
        }
        return Ok(vec![body]);
    }
    if let Some(arr) = body.as_array() {
        return Ok(arr.iter().collect());
    }
    if let Some(obj) = body.as_object() {
        if let Some(arr) = obj.get("entities").and_then(Value::as_array) {
            return Ok(arr.iter().collect());
        }
        if let Some(arr) = obj.get("data").and_then(Value::as_array) {
            return Ok(arr.iter().collect());
        }
        if obj.get("id").is_some() {
            return Ok(vec![body]);
        }
    }
    Err(HintFilingError::MalformedRecord {
        read: read_name.to_owned(),
        reason: "section payload has neither array items nor `entities`/`data` list".to_owned(),
    })
}

fn merge_note_summary_and_full(summary: &Value, full: &Value) -> Value {
    let (Some(sum_obj), Some(full_obj)) = (summary.as_object(), full.as_object()) else {
        return full.clone();
    };
    let mut merged = sum_obj.clone();
    for (k, v) in full_obj {
        merged.insert(k.clone(), v.clone());
    }
    Value::Object(merged)
}

/// Files a complete or partial [`HintPatientCapture`] into Coda's buckets, keeping empty,
/// populated, partial, changed, and failed reads distinct and listing any unknown kinds.
pub fn file_patient_chart(
    workspace: &str,
    patient_id: &str,
    capture: &HintPatientCapture,
) -> Result<FiledHintChart, HintFilingError> {
    let mut records = Vec::new();
    let mut read_standings = BTreeMap::new();
    let mut note_standings = BTreeMap::new();
    let mut missing_note_reads = BTreeSet::new();
    let mut unknown_kinds = Vec::new();
    let mut consumed_full_notes = BTreeSet::new();

    for (note_id, note_capture) in &capture.full_notes {
        let standing = match note_capture {
            HintReadCapture::Complete { .. } => ReadStanding::Populated { record_count: 1 },
            HintReadCapture::Partial { reason, .. } => ReadStanding::Partial {
                record_count: 1,
                reason: reason.clone(),
            },
            HintReadCapture::Changed { reason, .. } => ReadStanding::Changed {
                reason: reason.clone(),
            },
            HintReadCapture::Failed {
                reason,
                http_status,
            } => ReadStanding::Failed {
                reason: reason.clone(),
                http_status: *http_status,
            },
        };
        note_standings.insert(note_id.clone(), standing);
    }

    for (read_name, section_capture) in &capture.reads {
        match section_capture {
            HintReadCapture::Complete { body, .. } | HintReadCapture::Partial { body, .. } => {
                let items = items_in_section_body(read_name, body)?;
                let mut filed_count = 0usize;
                for item in items {
                    if read_name == "notes"
                        && matches!(section_capture, HintReadCapture::Complete { .. })
                        && let Some(nid) = item.get("id").and_then(Value::as_str)
                        && !capture.full_notes.contains_key(nid)
                        && extract_readable_parts(item).is_empty()
                    {
                        missing_note_reads.insert(nid.to_owned());
                    }
                    let effective_item = if read_name == "notes"
                        && let Some(nid) = item.get("id").and_then(Value::as_str)
                        && let Some(
                            HintReadCapture::Complete { body: full, .. }
                            | HintReadCapture::Partial { body: full, .. },
                        ) = capture.full_notes.get(nid)
                    {
                        consumed_full_notes.insert(nid.to_owned());
                        merge_note_summary_and_full(item, full)
                    } else {
                        item.clone()
                    };

                    let filed =
                        file_hint_record(workspace, patient_id, read_name, &effective_item)?;
                    if let HintKind::Unknown(raw_kind) = &filed.kind {
                        unknown_kinds.push(UnknownRecordNotice {
                            source_kind: raw_kind.clone(),
                            candidate_id: filed.revision.candidate_id.clone(),
                            record_id: filed.record_id.clone(),
                        });
                    }
                    records.push(filed);
                    filed_count += 1;
                }
                let standing = match section_capture {
                    HintReadCapture::Partial { reason, .. } => ReadStanding::Partial {
                        record_count: filed_count,
                        reason: reason.clone(),
                    },
                    _ if filed_count == 0 => ReadStanding::Empty,
                    _ => ReadStanding::Populated {
                        record_count: filed_count,
                    },
                };
                read_standings.insert(read_name.clone(), standing);
            }
            HintReadCapture::Changed { reason, .. } => {
                read_standings.insert(
                    read_name.clone(),
                    ReadStanding::Changed {
                        reason: reason.clone(),
                    },
                );
            }
            HintReadCapture::Failed {
                reason,
                http_status,
            } => {
                read_standings.insert(
                    read_name.clone(),
                    ReadStanding::Failed {
                        reason: reason.clone(),
                        http_status: *http_status,
                    },
                );
            }
        }
    }

    for (note_id, note_capture) in &capture.full_notes {
        if consumed_full_notes.contains(note_id) {
            continue;
        }
        if let HintReadCapture::Complete { body, .. } | HintReadCapture::Partial { body, .. } =
            note_capture
        {
            let filed = file_hint_record(workspace, patient_id, "note", body)?;
            if let HintKind::Unknown(raw_kind) = &filed.kind {
                unknown_kinds.push(UnknownRecordNotice {
                    source_kind: raw_kind.clone(),
                    candidate_id: filed.revision.candidate_id.clone(),
                    record_id: filed.record_id.clone(),
                });
            }
            records.push(filed);
        }
    }

    records.sort_by(|a, b| a.revision.candidate_id.cmp(&b.revision.candidate_id));

    Ok(FiledHintChart {
        patient_id: patient_id.to_owned(),
        records,
        read_standings,
        note_standings,
        missing_note_reads: missing_note_reads.into_iter().collect(),
        unknown_kinds,
    })
}

/// Parses a Hint patient JSON object into a [`SourcePatient`].
pub fn source_patient_from_hint(
    source_patient_id: &str,
    patient_json: &Value,
) -> Result<SourcePatient, HintFilingError> {
    let Some(obj) = patient_json.as_object() else {
        return Err(HintFilingError::MalformedRecord {
            read: "patient".to_owned(),
            reason: "patient record must be a JSON object".to_owned(),
        });
    };
    let display_name =
        extract_patient_title(patient_json).ok_or_else(|| HintFilingError::MalformedRecord {
            read: "patient".to_owned(),
            reason: "patient has no name".to_owned(),
        })?;
    let birth_date = obj
        .get("dob")
        .or_else(|| obj.get("birth_date"))
        .or_else(|| obj.get("birthDate"))
        .and_then(scalar_string);
    let sex_or_gender = obj
        .get("sex")
        .or_else(|| obj.get("gender"))
        .and_then(scalar_string);

    let mut medical_record_numbers = Vec::new();
    if let Some(mrn) = obj.get("mrn").and_then(scalar_string) {
        medical_record_numbers.push(MedicalRecordNumber {
            system: "hint:mrn".to_owned(),
            value: mrn,
        });
    }
    if let Some(Value::Array(identifiers)) = obj.get("identifiers") {
        for identifier in identifiers {
            if let (Some(system), Some(value)) = (
                identifier.get("system").and_then(Value::as_str),
                identifier.get("value").and_then(scalar_string),
            ) {
                medical_record_numbers.push(MedicalRecordNumber {
                    system: system.to_owned(),
                    value,
                });
            }
        }
    }

    Ok(SourcePatient {
        source_patient_id: source_patient_id.to_owned(),
        display_name,
        birth_date,
        sex_or_gender,
        medical_record_numbers,
    })
}

/// [`PatientSource`] backed by browser-captured Hint roster and per-patient chart captures.
#[derive(Debug, Clone, Default)]
pub struct HintChartSource {
    roster: Vec<Value>,
    charts: BTreeMap<String, HintPatientCapture>,
}

impl HintChartSource {
    /// Creates a [`HintChartSource`] from a captured Hint practice roster and per-patient captures.
    pub fn new(roster: Vec<Value>, charts: BTreeMap<String, HintPatientCapture>) -> Self {
        Self { roster, charts }
    }
}

#[async_trait]
impl PatientSource for HintChartSource {
    fn source_id(&self) -> &str {
        HINT_SOURCE_ID
    }

    fn source_label(&self) -> &str {
        HINT_SOURCE_LABEL
    }

    async fn patient_ids(&self) -> Result<Vec<String>, String> {
        let mut ids = Vec::with_capacity(self.roster.len());
        for entry in &self.roster {
            let id = entry
                .get("id")
                .and_then(Value::as_str)
                .filter(|s| !s.is_empty())
                .ok_or_else(|| "Hint roster entry is missing `id`".to_owned())?;
            ids.push(id.to_owned());
        }
        ids.sort();
        ids.dedup();
        Ok(ids)
    }

    async fn patient(&self, source_patient_id: &str) -> Result<SourcePatient, String> {
        if let Some(capture) = self.charts.get(source_patient_id)
            && let Some(HintReadCapture::Complete { body, .. }) = capture.reads.get("patient")
        {
            return source_patient_from_hint(source_patient_id, body).map_err(|e| e.to_string());
        }
        let roster_entry = self
            .roster
            .iter()
            .find(|p| p.get("id").and_then(Value::as_str) == Some(source_patient_id))
            .ok_or_else(|| format!("unknown Hint patient `{source_patient_id}`"))?;
        source_patient_from_hint(source_patient_id, roster_entry).map_err(|e| e.to_string())
    }

    async fn records(
        &self,
        patient: &SourcePatient,
        workspace: &str,
    ) -> Result<Vec<SourceRevision>, String> {
        let capture = self
            .charts
            .get(&patient.source_patient_id)
            .ok_or_else(|| format!("no Hint chart capture for `{}`", patient.source_patient_id))?;
        let filed = file_patient_chart(workspace, &patient.source_patient_id, capture)
            .map_err(|e| e.to_string())?;
        filed.validate_whole_chart().map_err(|e| e.to_string())?;
        Ok(filed.records.into_iter().map(|r| r.revision).collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::physician_sources::{
        ChartRequest, ChartScope, READ_TOOL, SEARCH_TOOL, workspace_id,
    };

    fn complete_capture(body: Value) -> HintReadCapture {
        let bytes = serde_json::to_vec(&body).unwrap();
        HintReadCapture::Complete {
            sha256: sha256_hex(&bytes),
            body,
        }
    }

    #[test]
    fn bucket_by_bucket_whole_hint_chart_filing_covers_all_confirmed_and_paged_sections() {
        let pid = "pat-whole-001";
        let ws = workspace_id(HINT_SOURCE_ID, pid);

        let mut reads = BTreeMap::new();
        reads.insert(
            "patient".to_owned(),
            complete_capture(json!({
                "id": pid,
                "first_name": "Elena",
                "last_name": "Vance",
                "dob": "1984-06-15",
                "sex": "female",
                "mrn": "H-10042",
                "status": "active",
                "balance": 19900,
                "stripeCustomerId": "cus_secret_billing",
                "paymentMethods": [{"brand": "visa", "last4": "4242"}],
                "membershipFee": 19900
            })),
        );
        reads.insert(
            "diagnoses".to_owned(),
            complete_capture(json!({
                "total": 1,
                "entities": [{
                    "id": "dx-1",
                    "patientId": pid,
                    "description": "Essential hypertension",
                    "icd10Code": "I10",
                    "status": "active",
                    "onsetDate": "01/2020"
                }]
            })),
        );
        reads.insert(
            "allergies".to_owned(),
            complete_capture(json!({
                "total": 1,
                "entities": [{
                    "id": "alg-1",
                    "patientId": pid,
                    "allergen": "Penicillin",
                    "rxnormCode": "7980",
                    "status": "active",
                    "onsetDate": "2012"
                }]
            })),
        );
        reads.insert(
            "medications".to_owned(),
            complete_capture(json!({
                "total": 1,
                "entities": [{
                    "id": "med-1",
                    "patientId": pid,
                    "name": "Lisinopril 10 mg tablet",
                    "rxnormCode": "314076",
                    "status": "active",
                    "startDate": "01/2020",
                    "price": 15.00,
                    "invoiceId": "inv_drop_1"
                }]
            })),
        );
        reads.insert(
            "freeform-medications".to_owned(),
            complete_capture(json!({
                "total": 1,
                "entities": [{
                    "id": "fmed-1",
                    "patientId": pid,
                    "name": "Magnesium glycinate 400 mg nightly",
                    "status": "patient_reported"
                }]
            })),
        );
        reads.insert(
            "surgeries".to_owned(),
            complete_capture(json!({
                "total": 1,
                "entities": [{
                    "id": "surg-1",
                    "patientId": pid,
                    "description": "Laparoscopic appendectomy",
                    "cptCode": "44970",
                    "performedDate": "01/2020"
                }]
            })),
        );
        reads.insert(
            "social-history".to_owned(),
            complete_capture(json!({
                "total": 1,
                "entities": [{
                    "id": "shx-1",
                    "patientId": pid,
                    "category": "Tobacco",
                    "description": "Never smoker",
                    "recordedDate": "2025-04-01"
                }]
            })),
        );
        reads.insert(
            "family-history".to_owned(),
            complete_capture(json!({
                "total": 1,
                "entities": [{
                    "id": "fhx-1",
                    "patientId": pid,
                    "relationship": "Father",
                    "condition": "Type 2 diabetes",
                    "icd10Code": "E11.9"
                }]
            })),
        );
        reads.insert(
            "vaccines".to_owned(),
            complete_capture(json!({
                "total": 1,
                "entities": [{
                    "id": "vax-1",
                    "patientId": pid,
                    "vaccineName": "Influenza, quadrivalent",
                    "cvxCode": "158",
                    "administeredDate": "2026-10-01"
                }]
            })),
        );
        reads.insert(
            "general".to_owned(),
            complete_capture(json!({
                "total": 1,
                "entities": [{
                    "id": "gen-1",
                    "patientId": pid,
                    "label": "Preferred pharmacy",
                    "value": "Northwind Community Pharmacy"
                }]
            })),
        );
        reads.insert(
            "vitals".to_owned(),
            complete_capture(json!({
                "total": 1,
                "entities": [{
                    "id": "vit-1",
                    "patientId": pid,
                    "name": "Blood pressure",
                    "loincCode": "85354-9",
                    "systolic": 118,
                    "diastolic": 74,
                    "measuredAt": "2026-10-12T14:20:00Z"
                }]
            })),
        );

        // Multi-page notes list (102 notes combined across pages, including clinical encounter,
        // sticky note, and in-house Rx) plus full text of notes.
        let mut note_summaries = vec![
            json!({
                "id": "note-visit-1",
                "patientId": pid,
                "noteType": "encounter",
                "title": "Annual Wellness Visit",
                "status": "signed",
                "encounterDate": "2026-10-12",
                "signedAt": "2026-10-12T17:00:00Z"
            }),
            json!({
                "id": "note-sticky-1",
                "patientId": pid,
                "noteType": "sticky_note",
                "title": "Prefers afternoon calls",
                "createdAt": "2026-01-10T09:00:00Z"
            }),
            json!({
                "id": "note-rx-1",
                "patientId": pid,
                "noteType": "in_house_rx",
                "medicationName": "Atorvastatin 20 mg tablet",
                "ndcCode": "00071-0156-23",
                "status": "dispensed",
                "dispensedAt": "2026-10-12T16:30:00Z",
                "price": 22.50,
                "unitPrice": 0.75,
                "dispensingFee": 4.00,
                "invoiceId": "inv_rx_77",
                "invoiceNumber": "INV-2026-77",
                "amountCents": 2650
            }),
        ];
        for extra in 4..=105 {
            note_summaries.push(json!({
                "id": format!("note-paged-{extra}"),
                "patientId": pid,
                "noteType": "encounter",
                "title": format!("Paged Follow-up #{extra}"),
                "status": "signed",
                "encounterDate": "2026-09-01"
            }));
        }
        reads.insert(
            "notes".to_owned(),
            complete_capture(json!({
                "total": note_summaries.len(),
                "data": note_summaries
            })),
        );

        let mut full_notes = BTreeMap::new();
        full_notes.insert(
            "note-visit-1".to_owned(),
            complete_capture(json!({
                "id": "note-visit-1",
                "patientId": pid,
                "body": "Full subjective and objective assessment: BP well controlled on lisinopril since 01/2020."
            })),
        );
        full_notes.insert(
            "note-sticky-1".to_owned(),
            complete_capture(json!({
                "id": "note-sticky-1",
                "patientId": pid,
                "body": "Call after 2 PM local time."
            })),
        );
        full_notes.insert(
            "note-rx-1".to_owned(),
            complete_capture(json!({
                "id": "note-rx-1",
                "patientId": pid,
                "body": "Dispensed Atorvastatin 20 mg tablet x30 in clinic."
            })),
        );
        for extra in 4..=105 {
            let nid = format!("note-paged-{extra}");
            full_notes.insert(
                nid.clone(),
                complete_capture(json!({
                    "id": nid,
                    "patientId": pid,
                    "body": format!("Full encounter note text for paged follow-up #{extra}.")
                })),
            );
        }

        reads.insert(
            "tasks".to_owned(),
            complete_capture(json!({
                "total": 2,
                "entities": [
                    {"id": "tsk-1", "patientId": pid, "title": "Check lipid panel", "status": "TO_DO", "dueOn": "2026-12-01"},
                    {"id": "tsk-2", "patientId": pid, "title": "Welcome packet sent", "status": "COMPLETED", "completedAt": "2026-01-05T12:00:00Z"}
                ]
            })),
        );
        reads.insert(
            "appointments".to_owned(),
            complete_capture(json!([
                {
                    "id": "appt-1",
                    "patient_id": pid,
                    "title": "Follow-up Blood Pressure Check",
                    "status": "confirmed",
                    "starts_at": "2026-11-20T15:00:00Z",
                    "ends_at": "2026-11-20T15:30:00Z"
                }
            ])),
        );
        reads.insert(
            "messages".to_owned(),
            complete_capture(json!({
                "total": 1,
                "data": [{
                    "id": "msg-1",
                    "patientId": pid,
                    "subject": "Home BP log",
                    "body": "My home readings averaged 118/74 this week.",
                    "status": "delivered",
                    "sentAt": "2026-10-14T11:00:00Z"
                }]
            })),
        );
        reads.insert(
            "patient-files".to_owned(),
            complete_capture(json!({
                "total": 1,
                "data": [{
                    "id": "file-1",
                    "patientId": pid,
                    "filename": "outside-echocardiogram.pdf",
                    "status": "reviewed",
                    "uploadedAt": "2026-08-19T10:00:00Z"
                }]
            })),
        );

        let capture = HintPatientCapture { reads, full_notes };
        let filed = file_patient_chart(&ws, pid, &capture).unwrap();
        assert!(filed.is_complete());
        assert!(filed.unknown_kinds.is_empty());

        let buckets = filed.by_bucket();
        assert_eq!(buckets[&ChartBucket::Patient].len(), 1);
        assert_eq!(buckets[&ChartBucket::Problems].len(), 1);
        assert_eq!(buckets[&ChartBucket::Allergies].len(), 1);
        // medications + freeform-medications + in_house_rx = 3 in Medications!
        assert_eq!(buckets[&ChartBucket::Medications].len(), 3);
        assert_eq!(buckets[&ChartBucket::Procedures].len(), 1);
        assert_eq!(buckets[&ChartBucket::History].len(), 2);
        assert_eq!(buckets[&ChartBucket::Immunizations].len(), 1);
        assert_eq!(buckets[&ChartBucket::Vitals].len(), 1);
        // 1 encounter + 102 paged encounters + 1 sticky note = 104 in Notes!
        assert_eq!(buckets[&ChartBucket::Notes].len(), 104);
        assert_eq!(buckets[&ChartBucket::Visits].len(), 1);
        assert_eq!(buckets[&ChartBucket::Messages].len(), 1);
        assert_eq!(buckets[&ChartBucket::Documents].len(), 1);
        // 1 general + 2 tasks = 3 in Other!
        assert_eq!(buckets[&ChartBucket::Other].len(), 3);

        // Full note text merged into note-visit-1 and searchable via ChartRequest!
        let visit_note = filed
            .records
            .iter()
            .find(|r| r.record_id == "note-visit-1")
            .unwrap();
        assert!(visit_note.revision.readable_parts.iter().any(|p| {
            p.text
                .contains("BP well controlled on lisinopril since 01/2020")
        }));

        let revisions: Vec<SourceRevision> =
            filed.records.iter().map(|r| r.revision.clone()).collect();
        let search = ChartRequest::parse(
            SEARCH_TOOL,
            &json!({"query": "Atorvastatin", "record_type": "Medications"}),
            ChartScope::Chat,
        )
        .unwrap();
        let outcome = search.run(&ws, 1, &revisions).unwrap();
        let harness::engine::ToolOutcome::Returned(hit_json) = outcome else {
            panic!("expected search hit");
        };
        assert!(hit_json.contains("in_house_rx/note-rx-1"));

        let read_req = ChartRequest::parse(
            READ_TOOL,
            &json!({
                "references": [{"candidate_id": visit_note.revision.candidate_id, "revision": visit_note.revision.revision}],
                "max_result_bytes": 16384
            }),
            ChartScope::Chat,
        )
        .unwrap();
        let harness::engine::ToolOutcome::Returned(read_json) =
            read_req.run(&ws, 1, &revisions).unwrap()
        else {
            panic!("expected exact read");
        };
        assert!(read_json.contains("Annual Wellness Visit"));
    }

    #[tokio::test]
    async fn empty_partial_changed_and_failed_reads_stay_distinct_and_unknown_kinds_are_listed_in_other()
     {
        let pid = "pat-completeness-002";
        let ws = workspace_id(HINT_SOURCE_ID, pid);

        let mut reads = BTreeMap::new();
        reads.insert(
            "patient".to_owned(),
            complete_capture(json!({"id": pid, "first_name": "Noah", "last_name": "Chen"})),
        );
        reads.insert(
            "allergies".to_owned(),
            complete_capture(json!({"total": 0, "entities": []})),
        );
        reads.insert(
            "notes".to_owned(),
            HintReadCapture::Partial {
                body: json!({
                    "total": 150,
                    "data": [
                        {"id": "n-1", "patientId": pid, "noteType": "wearable_coach_digest", "title": "Coach Summary"}
                    ]
                }),
                sha256: "a".repeat(64),
                reason: "read_failed".to_owned(),
            },
        );
        reads.insert(
            "surgeries".to_owned(),
            HintReadCapture::Failed {
                reason: "read_transport_failed".to_owned(),
                http_status: None,
            },
        );
        reads.insert(
            "diagnoses".to_owned(),
            HintReadCapture::Changed {
                reason: "read_schema_changed".to_owned(),
                sha256: "b".repeat(64),
            },
        );
        reads.insert(
            "care_team_custom_widget".to_owned(),
            complete_capture(json!({
                "total": 1,
                "entities": [{"id": "cw-9", "title": "Custom Care Flag", "recordedAt": "03/2024"}]
            })),
        );

        let capture = HintPatientCapture {
            reads,
            full_notes: BTreeMap::new(),
        };
        let filed = file_patient_chart(&ws, pid, &capture).unwrap();
        assert!(!filed.is_complete());

        assert_eq!(filed.read_standings["allergies"], ReadStanding::Empty);
        assert_eq!(
            filed.read_standings["patient"],
            ReadStanding::Populated { record_count: 1 }
        );
        assert_eq!(
            filed.read_standings["notes"],
            ReadStanding::Partial {
                record_count: 1,
                reason: "read_failed".to_owned()
            }
        );
        assert_eq!(
            filed.read_standings["surgeries"],
            ReadStanding::Failed {
                reason: "read_transport_failed".to_owned(),
                http_status: None
            }
        );
        assert_eq!(
            filed.read_standings["diagnoses"],
            ReadStanding::Changed {
                reason: "read_schema_changed".to_owned()
            }
        );

        // Both unknown kinds (unknown section + unknown note category) landed in `Other` and are listed!
        assert_eq!(filed.unknown_kinds.len(), 2);
        let listed_kinds: BTreeSet<&str> = filed
            .unknown_kinds
            .iter()
            .map(|u| u.source_kind.as_str())
            .collect();
        assert_eq!(
            listed_kinds,
            BTreeSet::from(["care_team_custom_widget", "note:wearable_coach_digest"])
        );
        for notice in &filed.unknown_kinds {
            let rec = filed
                .records
                .iter()
                .find(|r| r.revision.candidate_id == notice.candidate_id)
                .unwrap();
            assert_eq!(rec.bucket, ChartBucket::Other);
        }

        // PatientSource::records refuses incomplete captures rather than treating failed/partial reads as empty!
        let mut charts = BTreeMap::new();
        charts.insert(pid.to_owned(), capture);
        let source = HintChartSource::new(
            vec![json!({"id": pid, "first_name": "Noah", "last_name": "Chen"})],
            charts,
        );
        let sp = source.patient(pid).await.unwrap();
        let err = source.records(&sp, &ws).await.unwrap_err();
        assert!(err.contains("incomplete"));
        assert!(err.contains("surgeries"));
        assert!(err.contains("notes"));
        assert!(err.contains("diagnoses"));
        assert!(err.contains("missing_read:vitals"));
    }

    #[tokio::test]
    async fn whole_chart_validation_refuses_missing_sections_and_missing_full_note_bodies() {
        let pid = "pat-whole-chart-guard";
        let ws = workspace_id(HINT_SOURCE_ID, pid);
        let roster = vec![json!({"id": pid, "first_name": "Aria", "last_name": "Patel"})];

        // 1. Empty HintPatientCapture::default() is refused and names every missing section.
        let empty_filed = file_patient_chart(&ws, pid, &HintPatientCapture::default()).unwrap();
        assert!(!empty_filed.is_complete());
        let empty_source = HintChartSource::new(
            roster.clone(),
            BTreeMap::from([(pid.to_owned(), HintPatientCapture::default())]),
        );
        let sp = empty_source.patient(pid).await.unwrap();
        let empty_err = empty_source.records(&sp, &ws).await.unwrap_err();
        for required in HINT_WHOLE_CHART_READS {
            assert!(
                empty_err.contains(&format!("missing_read:{required}")),
                "expected `{required}` in error `{empty_err}`"
            );
        }

        // Build a complete 16-section chart capture with one summary-only note.
        let mut complete_reads = BTreeMap::new();
        complete_reads.insert(
            "patient".to_owned(),
            complete_capture(json!({"id": pid, "first_name": "Aria", "last_name": "Patel"})),
        );
        for read_name in HINT_WHOLE_CHART_READS {
            if read_name == "patient" {
                continue;
            }
            let empty_envelope = match read_name {
                "appointments" => json!([]),
                "notes" => json!({
                    "total": 1,
                    "data": [{
                        "id": "note-summary-only-1",
                        "patientId": pid,
                        "noteType": "encounter",
                        "title": "Follow-up Visit",
                        "status": "signed"
                    }]
                }),
                "messages" | "patient-files" => json!({"total": 0, "data": []}),
                _ => json!({"total": 0, "entities": []}),
            };
            complete_reads.insert(read_name.to_owned(), complete_capture(empty_envelope));
        }

        // 2. All 16 section reads present, but `full_notes["note-summary-only-1"]` missing -> refused!
        let missing_note_capture = HintPatientCapture {
            reads: complete_reads.clone(),
            full_notes: BTreeMap::new(),
        };
        let filed_missing_note = file_patient_chart(&ws, pid, &missing_note_capture).unwrap();
        assert!(!filed_missing_note.is_complete());
        assert_eq!(
            filed_missing_note.missing_note_reads,
            vec!["note-summary-only-1".to_owned()]
        );
        let missing_note_source = HintChartSource::new(
            roster.clone(),
            BTreeMap::from([(pid.to_owned(), missing_note_capture)]),
        );
        let missing_note_err = missing_note_source.records(&sp, &ws).await.unwrap_err();
        assert!(missing_note_err.contains("missing_note:note-summary-only-1"));

        // 3. Adding `full_notes["note-summary-only-1"]` makes the capture complete, while dropping
        //    a single section (`"vitals"`) refuses again with `missing_read:vitals`.
        let full_notes = BTreeMap::from([(
            "note-summary-only-1".to_owned(),
            complete_capture(json!({
                "id": "note-summary-only-1",
                "patientId": pid,
                "body": "Complete signed note body."
            })),
        )]);
        let valid_capture = HintPatientCapture {
            reads: complete_reads.clone(),
            full_notes: full_notes.clone(),
        };
        let valid_source = HintChartSource::new(
            roster.clone(),
            BTreeMap::from([(pid.to_owned(), valid_capture)]),
        );
        let valid_revisions = valid_source.records(&sp, &ws).await.unwrap();
        assert_eq!(valid_revisions.len(), 2);

        let mut missing_vitals_reads = complete_reads.clone();
        missing_vitals_reads.remove("vitals");
        let missing_vitals_source = HintChartSource::new(
            roster.clone(),
            BTreeMap::from([(
                pid.to_owned(),
                HintPatientCapture {
                    reads: missing_vitals_reads,
                    full_notes,
                },
            )]),
        );
        let missing_vitals_err = missing_vitals_source.records(&sp, &ws).await.unwrap_err();
        assert!(missing_vitals_err.contains("missing_read:vitals"));

        // 4. A `notes` entry carrying inline readable text passes without a separate `full_notes` entry.
        let mut inline_reads = complete_reads;
        inline_reads.insert(
            "notes".to_owned(),
            complete_capture(json!({
                "total": 1,
                "data": [{
                    "id": "note-inline-1",
                    "patientId": pid,
                    "noteType": "encounter",
                    "title": "Inline Visit",
                    "body": "Inline body already present on list payload."
                }]
            })),
        );
        let inline_source = HintChartSource::new(
            roster,
            BTreeMap::from([(
                pid.to_owned(),
                HintPatientCapture {
                    reads: inline_reads,
                    full_notes: BTreeMap::new(),
                },
            )]),
        );
        let inline_revisions = inline_source.records(&sp, &ws).await.unwrap();
        assert_eq!(inline_revisions.len(), 2);
    }

    #[test]
    fn page_code_lab_note_files_into_results_labs_and_confirmed_vs_unconfirmed_table_is_complete() {
        let pid = "pat-lab-003";
        let ws = workspace_id(HINT_SOURCE_ID, pid);

        let page_code_lab_note = json!({
            "id": "note-lab-42",
            "patientId": pid,
            "noteType": "lab",
            "panelName": "Comprehensive Metabolic Panel",
            "status": "final",
            "specimenCollectedAt": "2026-11-01T08:15:00Z",
            "resultedAt": "2026-11-02T06:40:00Z",
            "labOrder": {
                "orderedAt": "2026-10-30T14:00:00Z",
                "loincCode": "24323-8",
                "status": "completed"
            },
            "labResults": [
                {"name": "Potassium", "value": "4.3", "unit": "mmol/L", "loincCode": "2823-3", "abnormalFlag": "N"},
                {"name": "Creatinine", "value": "0.92", "unit": "mg/dL", "loincCode": "2160-0", "abnormalFlag": "N"}
            ],
            "body": "CMP final: K 4.3 mmol/L, Cr 0.92 mg/dL."
        });

        let filed = file_hint_record(&ws, pid, "notes", &page_code_lab_note).unwrap();
        assert_eq!(filed.bucket, ChartBucket::LABS);
        assert_eq!(filed.bucket, ChartBucket::Results);
        assert_eq!(filed.confirmation, ConfirmationStatus::Unconfirmed);
        assert_eq!(filed.source_kind, "note:lab");
        assert_eq!(filed.status.as_deref(), Some("final"));
        assert_eq!(
            filed.title.as_deref(),
            Some("Comprehensive Metabolic Panel")
        );
        assert!(filed.codes.iter().any(|c| c.code == "24323-8"));
        assert!(filed.codes.iter().any(|c| c.code == "2823-3"));
        assert!(
            filed
                .dates
                .iter()
                .any(|d| d.role == DateRole::Collected && d.value == "2026-11-01T08:15:00Z")
        );
        assert!(
            filed
                .dates
                .iter()
                .any(|d| d.role == DateRole::Resulted && d.value == "2026-11-02T06:40:00Z")
        );

        let confirmed: Vec<&str> = confirmed_rows().iter().map(|r| r.source_name).collect();
        let unconfirmed: Vec<&str> = unconfirmed_rows().iter().map(|r| r.source_name).collect();

        assert_eq!(
            confirmed,
            vec![
                "patient",
                "diagnoses",
                "allergies",
                "medications",
                "freeform-medications",
                "in_house_rx",
                "surgeries",
                "social-history",
                "family-history",
                "vaccines",
                "general",
                "vitals",
                "note",
                "sticky_note",
                "tasks",
                "appointment",
            ]
        );
        assert_eq!(
            unconfirmed,
            vec![
                "note:lab",
                "fax",
                "form",
                "erx",
                "messages",
                "patient-files",
                "econsult",
                "referral",
                "order",
                "partner_record",
            ]
        );
        assert_eq!(confirmed.len() + unconfirmed.len(), hint_table().len());
    }

    #[test]
    fn deep_links_open_addressable_records_directly_and_disclose_chart_fallbacks() {
        let pid = "pat-links-004";
        let ws = workspace_id(HINT_SOURCE_ID, pid);

        let direct_note = file_hint_record(
            &ws,
            pid,
            "note",
            &json!({"id": "n-100", "title": "Progress Note"}),
        )
        .unwrap();
        assert_eq!(
            direct_note.link,
            HintLink::DirectRecord {
                url: "https://app.hint.com/patients/pat-links-004/notes/n-100".to_owned()
            }
        );

        let direct_appt = file_hint_record(
            &ws,
            pid,
            "appointments",
            &json!({"id": "ap-200", "title": "Acute Visit"}),
        )
        .unwrap();
        assert_eq!(
            direct_appt.link,
            HintLink::DirectRecord {
                url: "https://app.hint.com/patients/pat-links-004/appointments/ap-200".to_owned()
            }
        );

        let direct_file = file_hint_record(
            &ws,
            pid,
            "patient-files",
            &json!({"id": "fl-300", "filename": "discharge.pdf"}),
        )
        .unwrap();
        assert_eq!(
            direct_file.link,
            HintLink::DirectRecord {
                url: "https://app.hint.com/patients/pat-links-004/files/fl-300".to_owned()
            }
        );

        let direct_msg = file_hint_record(
            &ws,
            pid,
            "messages",
            &json!({"id": "m-400", "subject": "Question"}),
        )
        .unwrap();
        assert_eq!(
            direct_msg.link,
            HintLink::DirectRecord {
                url: "https://app.hint.com/patients/pat-links-004/messages/m-400".to_owned()
            }
        );

        let fallback_dx = file_hint_record(
            &ws,
            pid,
            "diagnoses",
            &json!({"id": "dx-500", "description": "Asthma"}),
        )
        .unwrap();
        assert!(fallback_dx.link.is_chart_fallback());
        assert_eq!(
            fallback_dx.link.url(),
            "https://app.hint.com/patients/pat-links-004"
        );

        let fallbacks = chart_fallback_kinds();
        let always_fallback_names: Vec<&str> = fallbacks
            .iter()
            .filter(|f| f.always_falls_back)
            .map(|f| f.source_name)
            .collect();
        assert_eq!(
            always_fallback_names,
            vec![
                "diagnoses",
                "allergies",
                "medications",
                "freeform-medications",
                "surgeries",
                "social-history",
                "family-history",
                "vaccines",
                "general",
                "tasks",
                "partner_record",
            ]
        );
        let conditional_fallback_names: Vec<&str> = fallbacks
            .iter()
            .filter(|f| !f.always_falls_back)
            .map(|f| f.source_name)
            .collect();
        assert_eq!(
            conditional_fallback_names,
            vec![
                "in_house_rx",
                "vitals",
                "sticky_note",
                "note:lab",
                "fax",
                "form",
                "erx",
                "econsult",
                "referral",
                "order",
            ]
        );
    }

    #[test]
    fn saved_records_equal_hint_payload_except_dropped_billing_and_rx_price_invoice_fields_and_preserve_typed_dates()
     {
        let pid = "pat-fidelity-005";
        let ws = workspace_id(HINT_SOURCE_ID, pid);

        // 1. Patient record drops ONLY billing fields; clinical & demographic fields match verbatim.
        let raw_patient = json!({
            "id": pid,
            "first_name": "Maya",
            "last_name": "Lin",
            "dob": "01/1979",
            "status": "active_member",
            "phones": [{"number": "+1-555-0100", "type": "mobile"}],
            "customTags": ["marathon-runner", "spanish-speaking"],
            "balance": 15000,
            "accountBalance": 15000,
            "stripeCustomerId": "cus_123",
            "paymentMethods": [{"id": "pm_1"}],
            "invoices": [{"id": "inv_1", "total": 15000}],
            "billingAddress": {"postalCode": "94107"},
            "membershipFee": 17500,
            "planPrice": 17500
        });
        let filed_patient = file_hint_record(&ws, pid, "patient", &raw_patient).unwrap();
        assert_eq!(
            filed_patient.exact_record,
            json!({
                "id": pid,
                "first_name": "Maya",
                "last_name": "Lin",
                "dob": "01/1979",
                "status": "active_member",
                "phones": [{"number": "+1-555-0100", "type": "mobile"}],
                "customTags": ["marathon-runner", "spanish-speaking"]
            })
        );
        assert_eq!(filed_patient.status.as_deref(), Some("active_member"));
        assert!(
            filed_patient
                .dates
                .iter()
                .any(|d| d.field == "dob" && d.value == "01/1979")
        );

        // 2. In-house Rx drops ONLY price & invoice fields; sig, quantity, status, and typed-text dates stay untouched!
        let raw_rx = json!({
            "id": "rx-77",
            "medicationName": "Amoxicillin 500 mg capsule",
            "sig": "Take 1 capsule by mouth three times daily for 10 days",
            "quantity": 30,
            "refills": 0,
            "lotNumber": "LOT-88A",
            "status": "dispensed_in_clinic",
            "startDate": "01/2020",
            "writtenAt": "01/2020",
            "price": 9.99,
            "unitPrice": 0.33,
            "totalPrice": 13.99,
            "wholesalePrice": 4.20,
            "dispensingFee": 4.00,
            "invoice": {"number": "INV-9"},
            "invoiceId": "inv_9",
            "invoiceNumber": "INV-9",
            "invoiceStatus": "paid",
            "chargeId": "ch_9",
            "amountCents": 1399
        });
        let filed_rx = file_hint_record(&ws, pid, "in_house_rx", &raw_rx).unwrap();
        assert_eq!(filed_rx.bucket, ChartBucket::Medications);
        assert_eq!(
            filed_rx.exact_record,
            json!({
                "id": "rx-77",
                "medicationName": "Amoxicillin 500 mg capsule",
                "sig": "Take 1 capsule by mouth three times daily for 10 days",
                "quantity": 30,
                "refills": 0,
                "lotNumber": "LOT-88A",
                "status": "dispensed_in_clinic",
                "startDate": "01/2020",
                "writtenAt": "01/2020"
            })
        );
        assert_eq!(filed_rx.status.as_deref(), Some("dispensed_in_clinic"));
        assert_eq!(
            filed_rx
                .dates
                .iter()
                .filter(|d| d.value == "01/2020")
                .count(),
            2
        );

        // 3. Non-redacted clinical record (diagnosis with typed-text date "01/2020") equals Hint JSON verbatim!
        let raw_dx = json!({
            "id": "dx-88",
            "patientId": pid,
            "description": "Migraine without aura",
            "icd10Code": "G43.009",
            "status": "chronic_intermittent",
            "onsetDate": "01/2020",
            "clinicianComment": "Triggered by sleep deprivation"
        });
        let filed_dx = file_hint_record(&ws, pid, "diagnoses", &raw_dx).unwrap();
        assert_eq!(filed_dx.exact_record, raw_dx);
        let parsed_saved: Value = serde_json::from_str(&filed_dx.revision.content).unwrap();
        assert_eq!(parsed_saved, raw_dx);
        assert_eq!(filed_dx.status.as_deref(), Some("chronic_intermittent"));
        assert_eq!(filed_dx.dates[0].value, "01/2020");
    }
}
