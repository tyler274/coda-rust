//! Filing for the Hint Chrome connector's acquired chart responses.
//!
//! This consumes acquired sections in the documented capture envelope. Acquisition stays
//! in Chrome; credentials never enter the filer. Native objects are retained as
//! JSON, without translating their clinical content into another schema.

use serde::Deserialize;
use serde_json::{Value, json};
use std::collections::{BTreeMap, BTreeSet, HashSet};

use super::adapter::{RecordFilingError, file_record};
use super::{
    Bucket, DateRole, FailedPart, FiledDate, RecordCode, RecordDate, RecordEnvelope, RecordVersion,
    SourceAdapterError, SourcePatient, SourcePullOutput, SourceRecordKind,
};
use crate::runtime::schedule::{
    AppointmentId, AppointmentStatus, AppointmentWindow, ScheduleDelivery, ScheduleReadIssue,
    ScheduleSourceError, SourceAppointment, SourceAppointmentParts, SourceId as ScheduleSourceId,
    SourcePatientId,
};

/// Canonical source identifier for Hint Health Chrome connector deliveries.
pub const HINT_SOURCE_ID: &str = "hint";

/// Physician-facing source label for Hint Health Chrome connector deliveries.
pub const HINT_SOURCE_LABEL: &str = "Hint Health";

/// Acquired Hint sections, after the acquisition owner applies its source policy.
#[derive(Debug, Deserialize)]
pub struct ChartCapture {
    request: ChartRequest,
    sections: Vec<CapturedSection>,
}

#[derive(Debug, Deserialize)]
struct ChartRequest {
    patient_id: String,
}

#[derive(Debug, Deserialize)]
struct CapturedSection {
    section: CapturedKind,
    #[serde(flatten)]
    acquisition: CaptureState,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "state", rename_all = "snake_case")]
enum CaptureState {
    Pending,
    Running,
    Done { result: Option<SectionResult> },
}

#[derive(Debug, Deserialize)]
#[serde(untagged)]
enum CapturedKind {
    Known(Section),
    Unknown(String),
}

#[derive(Debug, Deserialize)]
struct SectionResult {
    body: AcquiredSection,
}

#[derive(Debug, Deserialize)]
struct AcquiredSection {
    items: Vec<AcquiredItem>,
    failed_parts: Vec<FailedPart>,
}

#[derive(Debug, Deserialize)]
struct AcquiredItem {
    body: Value,
    unavailable_parts: Vec<FailedPart>,
}

#[derive(Clone, Copy, Debug, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
enum Section {
    Patient,
    Diagnoses,
    Allergies,
    Medications,
    FreeformMedications,
    Surgeries,
    SocialHistory,
    FamilyHistory,
    Vaccines,
    General,
    Notes,
    Tasks,
    Appointments,
    Files,
}

impl Section {
    const ALL: [Self; 14] = [
        Self::Patient,
        Self::Diagnoses,
        Self::Allergies,
        Self::Medications,
        Self::FreeformMedications,
        Self::Surgeries,
        Self::SocialHistory,
        Self::FamilyHistory,
        Self::Vaccines,
        Self::General,
        Self::Notes,
        Self::Tasks,
        Self::Appointments,
        Self::Files,
    ];

    fn name(self) -> &'static str {
        match self {
            Self::Patient => "patient",
            Self::Diagnoses => "diagnoses",
            Self::Allergies => "allergies",
            Self::Medications => "medications",
            Self::FreeformMedications => "freeform-medications",
            Self::Surgeries => "surgeries",
            Self::SocialHistory => "social-history",
            Self::FamilyHistory => "family-history",
            Self::Vaccines => "vaccines",
            Self::General => "general",
            Self::Notes => "notes",
            Self::Tasks => "tasks",
            Self::Appointments => "appointments",
            Self::Files => "files",
        }
    }

    fn bucket(self, record: &Value) -> Bucket {
        match self {
            Self::Patient => Bucket::Patient,
            Self::Diagnoses => Bucket::Problems,
            Self::Allergies => Bucket::Allergies,
            Self::Medications => Bucket::Medications,
            Self::SocialHistory | Self::FamilyHistory => Bucket::History,
            Self::Vaccines => Bucket::Immunizations,
            Self::Notes => match record["noteType"].as_str() {
                Some("NOTE") => Bucket::Notes,
                Some("VITALS") => Bucket::Vitals,
                _ => Bucket::Other,
            },
            Self::General
            | Self::Surgeries
            | Self::Tasks
            | Self::FreeformMedications
            | Self::Appointments
            | Self::Files => Bucket::Other,
        }
    }
}

impl ChartCapture {
    /// Files the actual captured sections for the requested patient. A failed
    /// item does not prevent siblings from reaching the existing delivery owner.
    /// The connector's bounded acquisition never implies a complete chart.
    pub fn file(
        self,
        workspace: &str,
        source_id: &str,
        requested_patient: &str,
    ) -> SourcePullOutput {
        if self.request.patient_id != requested_patient {
            return SourcePullOutput::Failed {
                error: SourceAdapterError::RecordFiling {
                    context: "Hint chart request".into(),
                    source: RecordFilingError::PatientMismatch {
                        expected: requested_patient.to_owned(),
                        actual: self.request.patient_id,
                    },
                },
            };
        }
        let mut records = Vec::new();
        let mut identities = HashSet::new();
        let mut failed_parts = Vec::new();
        for captured in &self.sections {
            if let CapturedKind::Unknown(name) = &captured.section {
                failed_parts.push(FailedPart {
                    part: name.clone(),
                    reason: "source section identity contract is not established".into(),
                });
            }
        }
        let mut patient = None;
        let mut succeeded_kinds = Vec::new();
        for section in Section::ALL {
            let Some(captured) = self
                .sections
                .iter()
                .find(|item| matches!(item.section, CapturedKind::Known(kind) if kind == section))
            else {
                failed_parts.push(failure(section, "section was not acquired"));
                continue;
            };
            let result = match &captured.acquisition {
                CaptureState::Done {
                    result: Some(result),
                } => result,
                CaptureState::Done { result: None } => {
                    failed_parts.push(failure(section, "section was not acquired"));
                    continue;
                }
                CaptureState::Pending | CaptureState::Running => {
                    failed_parts.push(failure(section, "section acquisition did not complete"));
                    continue;
                }
            };
            let before = failed_parts.len();
            failed_parts.extend(result.body.failed_parts.iter().map(|part| FailedPart {
                part: format!("{}: {}", section.name(), part.part),
                reason: part.reason.clone(),
            }));
            for (index, acquired) in result.body.items.iter().enumerate() {
                let item = &acquired.body;
                match file_item(workspace, source_id, requested_patient, section, acquired) {
                    Ok(record) => {
                        if !identities.insert(record.candidate_id.clone()) {
                            failed_parts.push(failure(section, &format!(
                                "repeated native identity {}; collection coverage is not established",
                                record.candidate_id
                            )));
                            continue;
                        }
                        if section == Section::Patient {
                            match patient_identity(item, requested_patient) {
                                Ok(identity) => patient = Some(identity),
                                Err(error) => {
                                    failed_parts.push(failure(section, &error.to_string()))
                                }
                            }
                        }
                        records.push(record);
                    }
                    Err(error) => failed_parts.push(FailedPart {
                        part: format!("{}[{index}]", section.name()),
                        reason: error.to_string(),
                    }),
                }
            }
            if section == Section::Patient && patient.is_none() && failed_parts.len() == before {
                failed_parts.push(failure(section, "patient identity was not acquired"));
            }
            if failed_parts.len() == before {
                succeeded_kinds.push(SourceRecordKind {
                    kind: section.name().into(),
                    category: None,
                });
            }
        }
        if failed_parts.is_empty()
            && let Some(patient) = patient.clone()
        {
            return SourcePullOutput::Complete { patient, records };
        }
        SourcePullOutput::Partial {
            patient,
            records,
            succeeded_kinds,
            failed_parts,
        }
    }
}

fn failure(section: Section, reason: &str) -> FailedPart {
    FailedPart {
        part: section.name().into(),
        reason: reason.into(),
    }
}

fn text<'a>(record: &'a Value, key: &str) -> Option<&'a str> {
    record
        .get(key)
        .and_then(Value::as_str)
        .filter(|text| !text.trim().is_empty())
}

fn patient_identity(record: &Value, patient_id: &str) -> Result<SourcePatient, RecordFilingError> {
    let name = text(record, "name")
        .map(str::to_owned)
        .or_else(|| {
            let names = ["first_name", "middle_name", "last_name"]
                .iter()
                .filter_map(|key| text(record, key))
                .collect::<Vec<_>>();
            (!names.is_empty()).then(|| names.join(" "))
        })
        .ok_or(RecordFilingError::MissingField("patient name"))?;
    Ok(SourcePatient {
        source_patient_id: patient_id.into(),
        display_name: name,
        birth_date: text(record, "dob").map(str::to_owned),
        sex_or_gender: text(record, "sex").map(str::to_owned),
        medical_record_numbers: Vec::new(),
    })
}

fn file_item(
    workspace: &str,
    source_id: &str,
    patient_id: &str,
    section: Section,
    acquired: &AcquiredItem,
) -> Result<super::SourceRevision, RecordFilingError> {
    let record = &acquired.body;
    let id_field = match section {
        Section::Patient => "id",
        Section::Notes => "noteId",
        Section::Files => "Key",
        Section::FreeformMedications => {
            return Err(RecordFilingError::MissingField(
                "qualified freeform medication identity",
            ));
        }
        Section::Diagnoses
        | Section::Allergies
        | Section::Medications
        | Section::Surgeries
        | Section::SocialHistory
        | Section::FamilyHistory
        | Section::Vaccines
        | Section::General
        | Section::Tasks
        | Section::Appointments => "ident",
    };
    let native_id = text(record, id_field).ok_or(RecordFilingError::MissingField(id_field))?;
    if section == Section::Appointments
        && !record["attendees"].as_array().is_some_and(|attendees| {
            attendees
                .iter()
                .any(|attendee| attendee["patient_id"].as_str() == Some(patient_id))
        })
    {
        let actual = record["attendees"]
            .as_array()
            .and_then(|attendees| {
                attendees
                    .iter()
                    .find_map(|attendee| text(attendee, "patient_id"))
            })
            .ok_or(RecordFilingError::MissingField(
                "appointment attendee patient_id",
            ))?;
        return Err(RecordFilingError::PatientMismatch {
            expected: patient_id.into(),
            actual: actual.into(),
        });
    }
    if section == Section::Files && !native_id.starts_with(&format!("public/{patient_id}/")) {
        return Err(RecordFilingError::PatientMismatch {
            expected: patient_id.into(),
            actual: native_id.into(),
        });
    }
    let binding = if section == Section::Patient {
        native_id
    } else {
        match record.get("patientId") {
            None | Some(Value::Null) => patient_id,
            Some(_) => {
                text(record, "patientId").ok_or(RecordFilingError::MissingField("patientId"))?
            }
        }
    };
    if binding != patient_id {
        return Err(RecordFilingError::PatientMismatch {
            expected: patient_id.into(),
            actual: binding.into(),
        });
    }
    let mut dates = Vec::new();
    let fields: &[(DateRole, &str)] = match section {
        Section::Patient => &[(DateRole::BirthDate, "dob")],
        Section::Notes => &[
            (DateRole::Occurred, "eventTimestamp"),
            (DateRole::Recorded, "recordCreationTimestamp"),
        ],
        Section::Medications | Section::Appointments => {
            &[(DateRole::Start, "start"), (DateRole::End, "end")]
        }
        Section::Allergies | Section::SocialHistory | Section::General => {
            &[(DateRole::Start, "startDate"), (DateRole::End, "endDate")]
        }
        Section::Diagnoses
        | Section::FreeformMedications
        | Section::Surgeries
        | Section::FamilyHistory
        | Section::Vaccines
        | Section::Tasks
        | Section::Files => &[],
    };
    for (role, field) in fields {
        if let Some(value) = text(record, field) {
            dates.push(FiledDate {
                role: *role,
                field: (*field).into(),
                value: value.into(),
            });
        }
    }
    let title = match section {
        Section::Notes => record.get("data").and_then(|data| text(data, "title")),
        Section::Medications => text(record, "productName"),
        Section::SocialHistory => text(record, "question"),
        Section::FamilyHistory => text(record, "diagnosis"),
        Section::Tasks => text(record, "text"),
        Section::Appointments => text(record, "title"),
        Section::Files => text(record, "Key"),
        Section::Patient
        | Section::Diagnoses
        | Section::Allergies
        | Section::FreeformMedications
        | Section::Surgeries
        | Section::Vaccines
        | Section::General => text(record, "name"),
    }
    .map(str::to_owned);
    let mut codes = Vec::new();
    let code_path = match section {
        Section::Diagnoses | Section::Allergies => Some("icd10CodeId"),
        Section::FamilyHistory => Some("icd10Code"),
        Section::Patient
        | Section::Medications
        | Section::FreeformMedications
        | Section::Surgeries
        | Section::SocialHistory
        | Section::Vaccines
        | Section::General
        | Section::Notes
        | Section::Tasks
        | Section::Appointments
        | Section::Files => None,
    };
    if let Some(field) = code_path
        && let Some(value) = record.get(field)
        && let Some(code) = text(value, "code")
    {
        codes.push(RecordCode {
            field: format!("{field}.code"),
            system: None,
            code: Some(code.into()),
            display: text(value, "description").map(str::to_owned),
            text: None,
        });
    }
    // The acquisition contract requires source exclusions before delivery.
    // The filer retains that admitted representation without a second policy.
    let raw = serde_json::to_string(record).expect("JSON value serializes");
    let envelope = RecordEnvelope {
        bucket: section.bucket(record),
        source_kind: SourceRecordKind {
            kind: section.name().into(),
            category: if section == Section::Notes {
                text(record, "noteType").map(str::to_owned)
            } else {
                None
            },
        },
        native_id: native_id.into(),
        version: RecordVersion::ContentHash(crate::sha256_hex(raw.as_bytes())),
        dates,
        status: text(record, "status").map(str::to_owned),
        codes,
        title,
        native_link: Some(native_link(section, patient_id, native_id)),
        unavailable_parts: acquired.unavailable_parts.clone(),
    };
    file_record(workspace, source_id, &raw, envelope, Vec::new())
}

// Notes, appointments and tasks have native record routes. Other source
// sections link to the patient chart, which is their available UI destination.
fn native_link(section: Section, patient_id: &str, native_id: &str) -> String {
    let mut url = reqwest::Url::parse("https://app.hint.com/").expect("fixed Hint origin");
    match section {
        Section::Notes => {
            url.path_segments_mut()
                .expect("base URL")
                .extend(["patients", patient_id, "notes", native_id]);
        }
        Section::Appointments => {
            url.set_path("/calendar");
            url.query_pairs_mut()
                .append_pair("view", "day")
                .append_pair("appointmentId", native_id);
        }
        Section::Tasks => {
            url.set_path("/tasks");
            url.query_pairs_mut().append_pair("taskId", native_id);
        }
        Section::Patient
        | Section::Diagnoses
        | Section::Allergies
        | Section::Medications
        | Section::FreeformMedications
        | Section::Surgeries
        | Section::SocialHistory
        | Section::FamilyHistory
        | Section::Vaccines
        | Section::General
        | Section::Files => {
            url.path_segments_mut()
                .expect("base URL")
                .extend(["patients", patient_id, "chart"]);
        }
    }
    url.into()
}

/// Error parsing a Hint Health `READS.roster` payload delivered by the Chrome connector.
#[derive(Debug, thiserror::Error)]
pub enum HintRosterError {
    /// Top-level payload was not a JSON array (or `{ patients/data: [...] }`).
    #[error("Hint roster payload must be a JSON array of patient objects")]
    ExpectedArray,
}

/// Parses a Hint Health roster delivery (`extension/chrome/readers/hint.js`
/// `READS.roster`) into validated [`SourcePatient`] records, isolating any
/// malformed individual entries so one bad patient never blocks the rest.
pub fn parse_hint_roster(
    payload: &Value,
) -> Result<(Vec<SourcePatient>, Vec<String>), HintRosterError> {
    let items = payload
        .as_array()
        .or_else(|| payload.get("patients").and_then(Value::as_array))
        .or_else(|| payload.get("data").and_then(Value::as_array))
        .ok_or(HintRosterError::ExpectedArray)?;

    let mut patients = Vec::new();
    let mut seen_ids = BTreeSet::new();
    let mut failures = Vec::new();
    for (idx, item) in items.iter().enumerate() {
        let Some(id) = text(item, "id") else {
            failures.push(format!("roster[{idx}]: missing required field `id`"));
            continue;
        };
        match patient_identity(item, id) {
            Ok(patient) => {
                if seen_ids.insert(patient.source_patient_id.clone()) {
                    patients.push(patient);
                }
            }
            Err(err) => failures.push(format!("Patient/{id}: {err}")),
        }
    }
    Ok((patients, failures))
}

/// Parses a Hint Health appointments payload (`extension/chrome/readers/hint.js`
/// `READS.appointments` array or `{ data/entities: [...] }`) into a
/// [`ScheduleDelivery`], isolating malformed entries without dropping valid visits.
pub fn parse_hint_appointments(
    source_id: &ScheduleSourceId,
    payload: &Value,
) -> Result<ScheduleDelivery, ScheduleSourceError> {
    let items = payload
        .as_array()
        .or_else(|| payload.get("entities").and_then(Value::as_array))
        .or_else(|| payload.get("data").and_then(Value::as_array))
        .or_else(|| payload.get("appointments").and_then(Value::as_array))
        .ok_or_else(|| ScheduleSourceError::Unavailable {
            source_id: source_id.clone(),
            reason: "Hint appointments payload must be a JSON array or envelope with data/entities"
                .to_owned(),
        })?;

    let mut appointments = Vec::new();
    let mut seen_digests: BTreeMap<AppointmentId, String> = BTreeMap::new();
    let mut issues = Vec::new();

    for (idx, item) in items.iter().enumerate() {
        match parse_one_hint_appointment(item) {
            Ok((appt, maybe_issue)) => {
                if let Some(issue) = maybe_issue {
                    issues.push(issue);
                }
                let id = appt.appointment_id().clone();
                if let Some(prior_sha) = seen_digests.get(&id) {
                    if prior_sha != appt.sha256() {
                        issues.push(
                            ScheduleReadIssue::new(
                                format!("HintAppointment/{id}"),
                                "duplicate Hint appointment id with conflicting content",
                            )
                            .expect("non-empty issue"),
                        );
                    }
                    continue;
                }
                seen_digests.insert(id, appt.sha256().to_owned());
                appointments.push(appt);
            }
            Err(reason) => {
                let target = text(item, "ident")
                    .or_else(|| text(item, "id"))
                    .map(|id| format!("HintAppointment/{id}"))
                    .unwrap_or_else(|| format!("HintAppointment[{idx}]"));
                issues.push(ScheduleReadIssue::new(target, reason).expect("non-empty issue"));
            }
        }
    }

    ScheduleDelivery::from_outcomes(appointments, issues).map_err(|err| {
        ScheduleSourceError::Unavailable {
            source_id: source_id.clone(),
            reason: err.to_string(),
        }
    })
}

fn parse_one_hint_appointment(
    item: &Value,
) -> Result<(SourceAppointment, Option<ScheduleReadIssue>), String> {
    let raw_id = text(item, "ident")
        .or_else(|| text(item, "id"))
        .ok_or_else(|| "Hint appointment missing `ident` or `id`".to_owned())?;
    let appointment_id = AppointmentId::new(raw_id).map_err(|e| e.to_string())?;

    let raw_status = text(item, "status").unwrap_or("scheduled").trim();
    let status = match raw_status.to_ascii_lowercase().as_str() {
        "booked" | "scheduled" | "confirmed" | "pending" | "arrived" | "checked_in"
        | "checked-in" | "in_progress" | "in-progress" => AppointmentStatus::Scheduled,
        "cancelled" | "canceled" | "deleted" | "entered-in-error" => AppointmentStatus::Cancelled,
        "completed" | "fulfilled" | "finished" => AppointmentStatus::Fulfilled,
        "no_show" | "no-show" | "noshow" => AppointmentStatus::NoShow,
        other => return Err(format!("unsupported Hint appointment status `{other}`")),
    };

    let start = text(item, "start")
        .or_else(|| text(item, "start_time"))
        .or_else(|| text(item, "starts_at"))
        .or_else(|| text(item, "start_at"))
        .ok_or_else(|| "Hint appointment missing `start`".to_owned())?;
    let end = text(item, "end")
        .or_else(|| text(item, "end_time"))
        .or_else(|| text(item, "ends_at"))
        .or_else(|| text(item, "end_at"))
        .ok_or_else(|| "Hint appointment missing `end`".to_owned())?;
    let window = AppointmentWindow::parse(start, end).map_err(|e| e.to_string())?;

    let label = text(item, "title")
        .or_else(|| text(item, "description"))
        .or_else(|| text(item, "reason"))
        .or_else(|| text(item, "appointment_type"))
        .unwrap_or("Hint appointment")
        .to_owned();

    let (source_patient_id, participant_issue) = extract_hint_appointment_patient(item, raw_id)?;
    let mut dates = vec![
        RecordDate {
            name: "start".to_owned(),
            value: start.to_owned(),
        },
        RecordDate {
            name: "end".to_owned(),
            value: end.to_owned(),
        },
    ];
    if let Some(updated) = text(item, "updated_at").or_else(|| text(item, "updatedAt")) {
        dates.push(RecordDate {
            name: "updated_at".to_owned(),
            value: updated.to_owned(),
        });
    }
    let raw_content = serde_json::to_string(item).unwrap_or_default();
    let appt = SourceAppointment::new(SourceAppointmentParts {
        appointment_id,
        source_revision: None,
        window,
        status,
        source_status: raw_status.to_owned(),
        label,
        source_patient_id,
        dates,
        content: raw_content,
        provenance: json!({
            "native_record": format!("HintAppointment/{raw_id}"),
            "native_link": native_link(Section::Appointments, "", raw_id),
        }),
    })
    .map_err(|e| e.to_string())?;
    Ok((appt, participant_issue))
}

fn extract_hint_appointment_patient(
    item: &Value,
    raw_id: &str,
) -> Result<(Option<SourcePatientId>, Option<ScheduleReadIssue>), String> {
    let mut found: Option<SourcePatientId> = None;
    if let Some(pid) = text(item, "patient_id").or_else(|| text(item, "patientId")) {
        found = Some(SourcePatientId::new(pid).map_err(|e| e.to_string())?);
    }
    if let Some(attendees) = item.get("attendees").and_then(Value::as_array) {
        for attendee in attendees {
            let Some(pid) = text(attendee, "patient_id").or_else(|| text(attendee, "patientId"))
            else {
                continue;
            };
            let parsed = SourcePatientId::new(pid).map_err(|e| e.to_string())?;
            if let Some(existing) = &found
                && existing != &parsed
            {
                let issue = ScheduleReadIssue::new(
                    format!("HintAppointment/{raw_id}"),
                    "Hint appointment lists multiple distinct patient attendees",
                )
                .expect("non-empty issue");
                return Ok((None, Some(issue)));
            }
            found = Some(parsed);
        }
    }
    Ok((found, None))
}
