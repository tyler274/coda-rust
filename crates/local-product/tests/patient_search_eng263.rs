//! Focused disposable-Postgres and structural proof for unified patient search
//! across saved chart records, earlier conversations and reply drafts, and
//! durable patient memory.

use std::collections::BTreeSet;
use std::error::Error as _;
use std::path::Path;
use std::sync::OnceLock;

#[path = "common/disposable_postgres.rs"]
mod disposable_postgres;
use disposable_postgres::DisposablePostgres;

use base64::Engine as _;
use local_product::ModelBinding;
use local_product::physician_chat::{self, PhysicianSkills};
use local_product::physician_sources::read::{
    AttachmentPart, PageNumber, PartState, ScanDocument, ScanDocumentState, ScanPageEntry,
};
use local_product::physician_sources::{
    READ_TOOL, ReadablePart, RecordDate, SourcePatient, SourceRevision,
};
use local_product::runtime::delivery::Delivery;
use local_product::runtime::engine::recipe::{Observer, Provider, Request};
use local_product::runtime::memory::{
    ClinicalDate, CorrectionId, CorrectionStanding, JobMemoryCaller, MemoryCorrection,
    MemoryMutation, MemoryMutationOutcome, OperatingIndex, OperatingIndexPatch, PatientAccess,
    PatientKey, RecordCorrection, ReplacementState, RetractionDisposition, SAVE_MEMORY_TOOL,
    SAVE_SUMMARY_TOOL, SearchAudienceScope, SourceMessageRef, TopicKey, WithdrawCorrection,
};
use local_product::runtime::patient_search::{
    ClinicalCode, HitStanding, MemoryEntryPointer, PatientSearchError, PatientSearchHit,
    PatientSearchOutcome, PatientSearchRequest, RecordBucket, SEARCH_TOOL, SearchAuthor,
    SearchCaller, SearchDomainFilter, SearchStandingScope, WorkHitKind, WorkReference, search_tool,
};
use local_product::runtime::reply::{FINISH_TOOL, RevisionAccepted, RevisionInput};
use local_product::runtime::sources::SourcePull;
use local_product::runtime::{
    Accepted, Authority, Input, PhysicianHost, PublicationTransaction, Scope, Store, async_trait,
    engine,
};
use physician_auth::{AuthStore, Membership, PhysicianIdentity, PostgresAuthStore, PracticeClock};
use sea_orm::sea_query::{Alias, Expr, ExprTrait, Query};
use sea_orm::{ConnectionTrait, Database};
use serde_json::{Value, json};

fn skills() -> &'static PhysicianSkills {
    static SKILLS: OnceLock<PhysicianSkills> = OnceLock::new();
    SKILLS.get_or_init(|| {
        let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../skills");
        PhysicianSkills::load(&root).expect("load workspace skills")
    })
}

fn chat_recipe() -> engine::recipe::RequestRecipe {
    let binding = ModelBinding {
        model: "fixture-resident".into(),
        effort: "high".into(),
        context_window_tokens: 272_000,
        max_input_tokens: 250_000,
        adapter: "fixture".into(),
    };
    physician_chat::recipe(&binding, physician_chat::INSTRUCTIONS, skills())
}

fn reply_recipe() -> engine::recipe::RequestRecipe {
    let binding = ModelBinding {
        model: "fixture-resident".into(),
        effort: "high".into(),
        context_window_tokens: 272_000,
        max_input_tokens: 250_000,
        adapter: "fixture".into(),
    };
    local_product::runtime::reply::recipe(&binding, skills())
}

/// Real SQL-backed [`Authority`] delegating directly to [`PostgresAuthStore`]
/// four-table join checks inside the caller's Postgres transaction.
struct SqlRuntimeAuthority;

#[async_trait]
impl Authority for SqlRuntimeAuthority {
    async fn check(
        &mut self,
        tx: &PublicationTransaction,
        scope: &Scope,
        session_hash: Option<&[u8]>,
    ) -> Result<(), String> {
        let hash = session_hash.ok_or_else(|| "missing session hash".to_owned())?;
        let identity = PhysicianIdentity {
            practice_id: scope.practice.clone(),
            physician_id: scope.physician.clone(),
        };
        PostgresAuthStore::check_runtime_authority(tx, hash, &identity, &scope.workspace)
            .await
            .map_err(|err| err.to_string())
    }

    async fn lock_publication(
        &mut self,
        tx: &PublicationTransaction,
        scope: &Scope,
        session_hash: Option<&[u8]>,
    ) -> Result<(), String> {
        let hash = session_hash.ok_or_else(|| "missing session hash".to_owned())?;
        let identity = PhysicianIdentity {
            practice_id: scope.practice.clone(),
            physician_id: scope.physician.clone(),
        };
        PostgresAuthStore::lock_runtime_authority(tx, hash, &identity, &scope.workspace)
            .await
            .map_err(|err| err.to_string())
    }
}

struct AllowScopeAuthority {
    practice: String,
    physician: String,
    workspace: String,
    session_hash: [u8; 32],
}

impl AllowScopeAuthority {
    fn new(scope: &Scope, session_hash: [u8; 32]) -> Self {
        Self {
            practice: scope.practice.clone(),
            physician: scope.physician.clone(),
            workspace: scope.workspace.clone(),
            session_hash,
        }
    }

    fn verify(&self, scope: &Scope, session_hash: Option<&[u8]>) -> Result<(), String> {
        if session_hash != Some(self.session_hash.as_slice()) {
            return Err("invalid session hash".into());
        }
        if scope.practice != self.practice
            || scope.physician != self.physician
            || scope.workspace != self.workspace
        {
            return Err("scope outside authorized patient workspace".into());
        }
        Ok(())
    }
}

#[async_trait]
impl Authority for AllowScopeAuthority {
    async fn check(
        &mut self,
        _tx: &PublicationTransaction,
        scope: &Scope,
        session_hash: Option<&[u8]>,
    ) -> Result<(), String> {
        self.verify(scope, session_hash)
    }

    async fn lock_publication(
        &mut self,
        _tx: &PublicationTransaction,
        scope: &Scope,
        session_hash: Option<&[u8]>,
    ) -> Result<(), String> {
        self.verify(scope, session_hash)
    }
}

struct ScriptedProvider {
    turns: Vec<engine::Turn>,
    seen_requests: Vec<Request>,
}

impl ScriptedProvider {
    fn new(turns: Vec<engine::Turn>) -> Self {
        Self {
            turns,
            seen_requests: Vec::new(),
        }
    }
}

impl Provider for ScriptedProvider {
    async fn exchange(
        &mut self,
        request: &Request,
        _cancel: &engine::Cancellation,
        _limits: &engine::RunLimits,
        _observations: &mut dyn Observer,
    ) -> Result<engine::Turn, engine::Cause> {
        self.seen_requests.push(request.clone());
        if self.turns.is_empty() {
            return Err("scripted provider ran out of turns".into());
        }
        Ok(self.turns.remove(0))
    }
}

fn call_turn(calls: Vec<(&str, &str, Value)>) -> engine::Turn {
    engine::Turn {
        items: calls
            .into_iter()
            .map(|(id, name, args)| {
                engine::Item::Call(engine::Invocation {
                    id: id.into(),
                    name: name.into(),
                    arguments: args.to_string(),
                })
            })
            .collect(),
        receipt: json!({ "turn": "tool_calls" }),
        usage: Default::default(),
    }
}

fn assistant_turn(text: &str) -> engine::Turn {
    engine::Turn {
        items: vec![engine::Item::assistant(text)],
        receipt: json!({ "turn": "assistant" }),
        usage: Default::default(),
    }
}

#[expect(
    clippy::too_many_arguments,
    reason = "synthetic chart fixture builder constructs full SourceRevision fields"
)]
fn make_source_revision(
    workspace: &str,
    source_id: &str,
    candidate_id: &str,
    revision: &str,
    record_type: &str,
    dates: Vec<(&str, &str)>,
    content_json: Value,
    readable_parts: Vec<(&str, &str)>,
) -> SourceRevision {
    let content = serde_json::to_string(&content_json).expect("encode source content");
    let sha256 = local_product::sha256_hex(content.as_bytes());
    SourceRevision {
        workspace: workspace.into(),
        source_id: source_id.into(),
        candidate_id: candidate_id.into(),
        revision: revision.into(),
        record_type: record_type.into(),
        dates: dates
            .into_iter()
            .map(|(name, value)| RecordDate {
                name: name.into(),
                value: value.into(),
            })
            .collect(),
        content,
        sha256,
        readable_parts: readable_parts
            .into_iter()
            .map(|(label, text)| ReadablePart {
                label: label.into(),
                text: text.into(),
            })
            .collect(),
        provenance: json!({
            "source_id": source_id,
            "native_id": candidate_id,
        }),
    }
}

struct RevisionFixture<'a> {
    key: &'a str,
    expected_revision: u64,
    instruction: &'a str,
    revised_text: &'a str,
}

async fn complete_reply_revision(
    store: &Store,
    scope: &Scope,
    session_hash: [u8; 32],
    authority: &mut dyn Authority,
    step: RevisionFixture<'_>,
) {
    let RevisionFixture {
        key,
        expected_revision,
        instruction,
        revised_text,
    } = step;
    let accepted = store
        .accept_revision(
            scope,
            &RevisionInput {
                key: key.into(),
                expected_revision,
                instruction: instruction.into(),
                timezone: "America/Los_Angeles".into(),
                scenario: None,
            },
            session_hash.to_vec(),
            skills(),
            &reply_recipe(),
        )
        .await
        .expect("accept revision");
    assert_eq!(accepted, RevisionAccepted::New);

    let delivery = Delivery::reply_revision(scope.clone(), key.into());
    let claim = store
        .claim_revision(&delivery, 60)
        .await
        .expect("claim revision")
        .expect("active revision claim");
    let turn = engine::Turn {
        items: vec![engine::Item::Call(engine::Invocation {
            id: format!("finish-{key}"),
            name: FINISH_TOOL.into(),
            arguments: json!({
                "text": revised_text,
                "review_basis": "Grounded in workspace chart"
            })
            .to_string(),
        })],
        receipt: json!({ "synthetic": true }),
        usage: Default::default(),
    };
    store
        .record_revision(
            &claim,
            engine::Event {
                attempt_id: claim.attempt.clone(),
                sequence: 1,
                ordinal: 1,
                kind: engine::EventKind::ProviderReturned(turn.clone()),
            },
        )
        .await
        .expect("record provider returned");
    store
        .record_revision(
            &claim,
            engine::Event {
                attempt_id: claim.attempt.clone(),
                sequence: 2,
                ordinal: 1,
                kind: engine::EventKind::Candidate { text: turn.text() },
            },
        )
        .await
        .expect("record candidate");
    let committed = store
        .commit_revision(&claim, &turn, authority)
        .await
        .expect("commit revision");
    assert!(matches!(
        committed,
        local_product::runtime::reply::RevisionCommit::Committed
    ));
}

struct SyntheticRecordCase {
    source: SourceRevision,
    expected_bucket: RecordBucket,
    word_query: &'static str,
    code_query: &'static str,
    expected_code: Option<&'static str>,
    expected_date_substring: &'static str,
    expected_author_substring: &'static str,
    body_sentinel: &'static str,
}

fn synthetic_chart_cases(workspace: &str) -> Vec<SyntheticRecordCase> {
    let note_b64 = base64::engine::general_purpose::STANDARD
        .encode("Endocrinology consult titration plan. NOTE_BODY_SENTINEL_CONSULT");
    let report_b64 = base64::engine::general_purpose::STANDARD
        .encode("Chest CT shows stable lingula nodule. REPORT_BODY_SENTINEL_RADIOLOGY");
    let scan_b64 = base64::engine::general_purpose::STANDARD
        .encode("Outside nephrology fax showing microalbuminuria. DOC_BODY_SENTINEL_FAX");

    vec![
        SyntheticRecordCase {
            source: make_source_revision(
                workspace,
                "epic-fhir",
                "01-patient-roger",
                "rev-pat-1",
                "Patient",
                vec![("birthDate", "1968-04-19")],
                json!({
                    "resourceType": "Patient",
                    "id": "pat-roger",
                    "identifier": [{
                        "system": "http://hospital.example.org/mrn",
                        "value": "MRN-884210"
                    }],
                    "name": [{"family": "Alvarez", "given": ["Roger"]}],
                    "birthDate": "1968-04-19",
                    "gender": "male",
                    "address": [{"text": "PATIENT_BODY_SENTINEL_742_MAPLE_AVE"}]
                }),
                vec![],
            ),
            expected_bucket: RecordBucket::Patient,
            word_query: "Alvarez",
            code_query: "MRN-884210",
            expected_code: None,
            expected_date_substring: "1968-04-19",
            expected_author_substring: "epic-fhir",
            body_sentinel: "PATIENT_BODY_SENTINEL_742_MAPLE_AVE",
        },
        SyntheticRecordCase {
            source: make_source_revision(
                workspace,
                "epic-fhir",
                "02-condition-dm2",
                "rev-cond-1",
                "Condition",
                vec![("onsetDateTime", "2021-09-14")],
                json!({
                    "resourceType": "Condition",
                    "id": "cond-dm2",
                    "clinicalStatus": {"coding": [{"code": "active"}]},
                    "category": [{"coding": [{"code": "problem-list-item"}]}],
                    "code": {
                        "coding": [
                            {"system": "http://hl7.org/fhir/sid/icd-10-cm", "code": "E11.65", "display": "Type 2 diabetes with hyperglycemia"},
                            {"system": "http://snomed.info/sct", "code": "44054006"}
                        ]
                    },
                    "onsetDateTime": "2021-09-14",
                    "recorder": {"display": "Dr. Elena Vance"},
                    "note": [{"text": "Persistent morningketosis COND_BODY_SENTINEL_HYPERGLYCEMIA"}]
                }),
                vec![],
            ),
            expected_bucket: RecordBucket::Problems,
            word_query: "morningketosis",
            code_query: "icd-10-cm:E11.65",
            expected_code: Some("E11.65"),
            expected_date_substring: "2021-09-14",
            expected_author_substring: "Dr. Elena Vance",
            body_sentinel: "COND_BODY_SENTINEL_HYPERGLYCEMIA",
        },
        SyntheticRecordCase {
            source: make_source_revision(
                workspace,
                "epic-fhir",
                "03-med-metformin",
                "rev-med-1",
                "MedicationRequest",
                vec![("authoredOn", "2026-09-02")],
                json!({
                    "resourceType": "MedicationRequest",
                    "status": "active",
                    "intent": "order",
                    "medicationCodeableConcept": {
                        "coding": [{"system": "http://www.nlm.nih.gov/research/umls/rxnorm", "code": "860975", "display": "Metformin hydrochloride extended release"}]
                    },
                    "authoredOn": "2026-09-02",
                    "requester": {"display": "Dr. Elena Vance"},
                    "dosageInstruction": [{"text": "MED_BODY_SENTINEL_1000MG_WITH_DINNER"}]
                }),
                vec![],
            ),
            expected_bucket: RecordBucket::Medications,
            word_query: "Metformin",
            code_query: "rxnorm:860975",
            expected_code: Some("860975"),
            expected_date_substring: "2026-09-02",
            expected_author_substring: "Dr. Elena Vance",
            body_sentinel: "MED_BODY_SENTINEL_1000MG_WITH_DINNER",
        },
        SyntheticRecordCase {
            source: make_source_revision(
                workspace,
                "epic-fhir",
                "04-obs-lab-a1c",
                "rev-lab-1",
                "Observation",
                vec![("effectiveDateTime", "2026-10-01T08:15:00Z")],
                json!({
                    "resourceType": "Observation",
                    "status": "final",
                    "category": [{"coding": [{"code": "laboratory"}]}],
                    "code": {
                        "coding": [{"system": "http://loinc.org", "code": "4548-4", "display": "Hemoglobin A1c"}]
                    },
                    "effectiveDateTime": "2026-10-01T08:15:00Z",
                    "performer": [{"display": "Quest Diagnostics Lab"}],
                    "valueQuantity": {"value": 8.4, "unit": "%", "code": "%"},
                    "note": [{"text": "LAB_BODY_SENTINEL_A1C_EIGHT_POINT_FOUR"}]
                }),
                vec![],
            ),
            expected_bucket: RecordBucket::Results,
            word_query: "Hemoglobin",
            code_query: "loinc:4548-4",
            expected_code: Some("4548-4"),
            expected_date_substring: "2026-10-01T08:15:00Z",
            expected_author_substring: "Quest Diagnostics Lab",
            body_sentinel: "LAB_BODY_SENTINEL_A1C_EIGHT_POINT_FOUR",
        },
        SyntheticRecordCase {
            source: make_source_revision(
                workspace,
                "epic-fhir",
                "05-obs-vital-bp",
                "rev-vital-1",
                "Observation",
                vec![("effectiveDateTime", "2026-10-01T09:05:00Z")],
                json!({
                    "resourceType": "Observation",
                    "status": "final",
                    "category": [{"coding": [{"code": "vital-signs"}]}],
                    "code": {
                        "coding": [{"system": "http://loinc.org", "code": "85354-9", "display": "Blood pressure panel"}]
                    },
                    "effectiveDateTime": "2026-10-01T09:05:00Z",
                    "performer": [{"display": "MA Carla Gomez"}],
                    "component": [
                        {
                            "code": {"coding": [{"system": "http://loinc.org", "code": "8480-6", "display": "Systolic cuff"}]},
                            "valueQuantity": {"value": 148, "unit": "mmHg"}
                        },
                        {
                            "code": {"coding": [{"system": "http://loinc.org", "code": "8462-4", "display": "Diastolic cuff"}]},
                            "valueQuantity": {"value": 92, "unit": "mmHg"}
                        }
                    ],
                    "note": [{"text": "VITAL_BODY_SENTINEL_SEATED_RIGHT_ARM"}]
                }),
                vec![],
            ),
            expected_bucket: RecordBucket::Vitals,
            word_query: "Diastolic cuff",
            code_query: "8480-6",
            expected_code: Some("8480-6"),
            expected_date_substring: "2026-10-01T09:05:00Z",
            expected_author_substring: "MA Carla Gomez",
            body_sentinel: "VITAL_BODY_SENTINEL_SEATED_RIGHT_ARM",
        },
        SyntheticRecordCase {
            source: make_source_revision(
                workspace,
                "epic-fhir",
                "06-diag-report-ct",
                "rev-dr-1",
                "DiagnosticReport",
                vec![("issued", "2026-09-28T16:40:00Z")],
                json!({
                    "resourceType": "DiagnosticReport",
                    "status": "final",
                    "code": {
                        "coding": [{"system": "http://loinc.org", "code": "24627-2", "display": "Chest CT"}]
                    },
                    "issued": "2026-09-28T16:40:00Z",
                    "performer": [{"display": "Dr. Amir Patel"}],
                    "presentedForm": [{"contentType": "text/plain", "data": report_b64}]
                }),
                vec![(
                    "presentedForm[0]",
                    "Chest CT shows stable lingulanodule. REPORT_BODY_SENTINEL_RADIOLOGY",
                )],
            ),
            expected_bucket: RecordBucket::Results,
            word_query: "lingulanodule",
            code_query: "24627-2",
            expected_code: Some("24627-2"),
            expected_date_substring: "2026-09-28T16:40:00Z",
            expected_author_substring: "Dr. Amir Patel",
            body_sentinel: "REPORT_BODY_SENTINEL_RADIOLOGY",
        },
        SyntheticRecordCase {
            source: make_source_revision(
                workspace,
                "epic-fhir",
                "07-docref-consult-note",
                "rev-note-1",
                "DocumentReference",
                vec![("date", "2026-09-30T11:20:00Z")],
                json!({
                    "resourceType": "DocumentReference",
                    "status": "current",
                    "category": [{"coding": [{"code": "clinical-note"}]}],
                    "type": {"coding": [{"system": "http://loinc.org", "code": "11488-4", "display": "Consult note"}]},
                    "date": "2026-09-30T11:20:00Z",
                    "author": [{"display": "Dr. Maya Lin"}],
                    "content": [{"attachment": {"contentType": "text/plain", "data": note_b64}}]
                }),
                vec![(
                    "attachment[0]",
                    "Endocrinology consult glargineplan. NOTE_BODY_SENTINEL_CONSULT",
                )],
            ),
            expected_bucket: RecordBucket::Notes,
            word_query: "glargineplan",
            code_query: "11488-4",
            expected_code: Some("11488-4"),
            expected_date_substring: "2026-09-30T11:20:00Z",
            expected_author_substring: "Dr. Maya Lin",
            body_sentinel: "NOTE_BODY_SENTINEL_CONSULT",
        },
        SyntheticRecordCase {
            source: make_source_revision(
                workspace,
                "epic-fhir",
                "08-docref-outside-scan",
                "rev-doc-1",
                "DocumentReference",
                vec![("date", "2026-09-18T13:00:00Z")],
                json!({
                    "resourceType": "DocumentReference",
                    "status": "current",
                    "category": [{"coding": [{"code": "outside-record"}]}],
                    "type": {"coding": [{"system": "http://loinc.org", "code": "34133-9", "display": "Summary of episode"}]},
                    "date": "2026-09-18T13:00:00Z",
                    "author": [{"display": "Mercy Nephrology Records"}],
                    "content": [{"attachment": {"contentType": "application/pdf", "data": scan_b64}}]
                }),
                vec![(
                    "page_1",
                    "Outside nephrology fax microalbuminuriawatch. DOC_BODY_SENTINEL_FAX",
                )],
            ),
            expected_bucket: RecordBucket::Documents,
            word_query: "microalbuminuriawatch",
            code_query: "34133-9",
            expected_code: Some("34133-9"),
            expected_date_substring: "2026-09-18T13:00:00Z",
            expected_author_substring: "Mercy Nephrology Records",
            body_sentinel: "DOC_BODY_SENTINEL_FAX",
        },
        SyntheticRecordCase {
            source: make_source_revision(
                workspace,
                "epic-fhir",
                "09-encounter-office",
                "rev-enc-1",
                "Encounter",
                vec![("period.start", "2026-08-20T14:00:00Z")],
                json!({
                    "resourceType": "Encounter",
                    "status": "finished",
                    "class": {"system": "http://terminology.hl7.org/CodeSystem/v3-ActCode", "code": "AMB"},
                    "type": [{"coding": [{"system": "http://snomed.info/sct", "code": "185349003", "display": "Encounter for checkup"}]}],
                    "period": {"start": "2026-08-20T14:00:00Z", "end": "2026-08-20T14:30:00Z"},
                    "participant": [{"individual": {"display": "Dr. Elena Vance"}}],
                    "reasonCode": [{"text": "Quarterly diabetesvisit ENCOUNTER_BODY_SENTINEL_FOLLOWUP"}]
                }),
                vec![],
            ),
            expected_bucket: RecordBucket::Visits,
            word_query: "diabetesvisit",
            code_query: "snomed:185349003",
            expected_code: Some("185349003"),
            expected_date_substring: "2026-08-20T14:00:00Z",
            expected_author_substring: "Dr. Elena Vance",
            body_sentinel: "ENCOUNTER_BODY_SENTINEL_FOLLOWUP",
        },
        SyntheticRecordCase {
            source: make_source_revision(
                workspace,
                "epic-fhir",
                "10-appointment-ophtho",
                "rev-appt-1",
                "Appointment",
                vec![("start", "2026-11-18T09:30:00Z")],
                json!({
                    "resourceType": "Appointment",
                    "status": "booked",
                    "serviceType": [{"coding": [{"system": "http://snomed.info/sct", "code": "394594003", "display": "Retinal screening"}]}],
                    "start": "2026-11-18T09:30:00Z",
                    "participant": [{"actor": {"display": "Dr. Hannah Cho"}}],
                    "description": "Dilated funduscheck APPT_BODY_SENTINEL_OPHTHO"
                }),
                vec![],
            ),
            expected_bucket: RecordBucket::Visits,
            word_query: "funduscheck",
            code_query: "394594003",
            expected_code: Some("394594003"),
            expected_date_substring: "2026-11-18T09:30:00Z",
            expected_author_substring: "Dr. Hannah Cho",
            body_sentinel: "APPT_BODY_SENTINEL_OPHTHO",
        },
        SyntheticRecordCase {
            source: make_source_revision(
                workspace,
                "epic-fhir",
                "11-procedure-colonoscopy",
                "rev-proc-1",
                "Procedure",
                vec![("performedDateTime", "2025-05-12")],
                json!({
                    "resourceType": "Procedure",
                    "status": "completed",
                    "code": {
                        "coding": [
                            {"system": "http://www.ama-assn.org/go/cpt", "code": "45378", "display": "Diagnostic colonoscopy"},
                            {"system": "http://snomed.info/sct", "code": "73761001"}
                        ]
                    },
                    "performedDateTime": "2025-05-12",
                    "performer": [{"actor": {"display": "Dr. Priya Nair"}}],
                    "note": [{"text": "Cecum reached sessileadenoma PROC_BODY_SENTINEL_POLYP"}]
                }),
                vec![],
            ),
            expected_bucket: RecordBucket::Procedures,
            word_query: "sessileadenoma",
            code_query: "cpt:45378",
            expected_code: Some("45378"),
            expected_date_substring: "2025-05-12",
            expected_author_substring: "Dr. Priya Nair",
            body_sentinel: "PROC_BODY_SENTINEL_POLYP",
        },
        SyntheticRecordCase {
            source: make_source_revision(
                workspace,
                "epic-fhir",
                "12-allergy-penicillin",
                "rev-alg-1",
                "AllergyIntolerance",
                vec![("recordedDate", "2019-03-04")],
                json!({
                    "resourceType": "AllergyIntolerance",
                    "clinicalStatus": {"coding": [{"code": "active"}]},
                    "verificationStatus": {"coding": [{"code": "confirmed"}]},
                    "code": {
                        "coding": [{"system": "http://www.nlm.nih.gov/research/umls/rxnorm", "code": "7980", "display": "Penicillin G"}]
                    },
                    "recordedDate": "2019-03-04",
                    "recorder": {"display": "Dr. Elena Vance"},
                    "reaction": [{"manifestation": [{"coding": [{"system": "http://snomed.info/sct", "code": "39579001", "display": "Anaphylaxisbronchospasm"}]}]}],
                    "note": [{"text": "ALLERGY_BODY_SENTINEL_AIRWAY_EDEMA"}]
                }),
                vec![],
            ),
            expected_bucket: RecordBucket::Allergies,
            word_query: "Anaphylaxisbronchospasm",
            code_query: "rxnorm:7980",
            expected_code: Some("7980"),
            expected_date_substring: "2019-03-04",
            expected_author_substring: "Dr. Elena Vance",
            body_sentinel: "ALLERGY_BODY_SENTINEL_AIRWAY_EDEMA",
        },
        SyntheticRecordCase {
            source: make_source_revision(
                workspace,
                "epic-fhir",
                "13-immunization-covid",
                "rev-imm-1",
                "Immunization",
                vec![("occurrenceDateTime", "2025-10-19")],
                json!({
                    "resourceType": "Immunization",
                    "status": "completed",
                    "vaccineCode": {
                        "coding": [{"system": "http://hl7.org/fhir/sid/cvx", "code": "208", "display": "SARS-CoV-2 mRNAbooster"}]
                    },
                    "occurrenceDateTime": "2025-10-19",
                    "performer": [{"actor": {"display": "Nurse Jordan Lee"}}],
                    "note": [{"text": "IMM_BODY_SENTINEL_LEFT_DELTOID"}]
                }),
                vec![],
            ),
            expected_bucket: RecordBucket::Immunizations,
            word_query: "mRNAbooster",
            code_query: "cvx:208",
            expected_code: Some("208"),
            expected_date_substring: "2025-10-19",
            expected_author_substring: "Nurse Jordan Lee",
            body_sentinel: "IMM_BODY_SENTINEL_LEFT_DELTOID",
        },
        SyntheticRecordCase {
            source: make_source_revision(
                workspace,
                "epic-fhir",
                "14-order-microalbumin",
                "rev-ord-1",
                "ServiceRequest",
                vec![("authoredOn", "2026-10-03")],
                json!({
                    "resourceType": "ServiceRequest",
                    "status": "active",
                    "intent": "order",
                    "code": {
                        "coding": [{"system": "http://loinc.org", "code": "14959-1", "display": "Urine microalbuminratio"}]
                    },
                    "authoredOn": "2026-10-03",
                    "requester": {"display": "Dr. Elena Vance"},
                    "note": [{"text": "ORDER_BODY_SENTINEL_FIRST_MORNING_VOID"}]
                }),
                vec![],
            ),
            expected_bucket: RecordBucket::Orders,
            word_query: "microalbuminratio",
            code_query: "loinc:14959-1",
            expected_code: Some("14959-1"),
            expected_date_substring: "2026-10-03",
            expected_author_substring: "Dr. Elena Vance",
            body_sentinel: "ORDER_BODY_SENTINEL_FIRST_MORNING_VOID",
        },
        SyntheticRecordCase {
            source: make_source_revision(
                workspace,
                "epic-fhir",
                "15-history-family",
                "rev-hist-1",
                "FamilyMemberHistory",
                vec![("date", "2024-01-10")],
                json!({
                    "resourceType": "FamilyMemberHistory",
                    "status": "completed",
                    "date": "2024-01-10",
                    "condition": [{"code": {"coding": [{"system": "http://snomed.info/sct", "code": "22298006", "display": "Prematurecoronary infarction"}]}}],
                    "note": [{"text": "HISTORY_BODY_SENTINEL_FATHER_AGE_51"}]
                }),
                vec![],
            ),
            expected_bucket: RecordBucket::History,
            word_query: "Prematurecoronary",
            code_query: "22298006",
            expected_code: None,
            expected_date_substring: "2024-01-10",
            expected_author_substring: "epic-fhir",
            body_sentinel: "HISTORY_BODY_SENTINEL_FATHER_AGE_51",
        },
        SyntheticRecordCase {
            source: make_source_revision(
                workspace,
                "epic-fhir",
                "16-form-phq9",
                "rev-form-1",
                "QuestionnaireResponse",
                vec![("authored", "2026-10-01")],
                json!({
                    "resourceType": "QuestionnaireResponse",
                    "status": "completed",
                    "code": {"coding": [{"system": "http://loinc.org", "code": "44249-1", "display": "PHQ9questionnaire"}]},
                    "authored": "2026-10-01",
                    "author": {"display": "Roger Alvarez"},
                    "item": [{"text": "FORM_BODY_SENTINEL_SLEEP_SCORE_TWO"}]
                }),
                vec![],
            ),
            expected_bucket: RecordBucket::Forms,
            word_query: "PHQ9questionnaire",
            code_query: "44249-1",
            expected_code: Some("44249-1"),
            expected_date_substring: "2026-10-01",
            expected_author_substring: "Roger Alvarez",
            body_sentinel: "FORM_BODY_SENTINEL_SLEEP_SCORE_TWO",
        },
        SyntheticRecordCase {
            source: make_source_revision(
                workspace,
                "spruce-mail",
                "17-message-spruce",
                "rev-msg-1",
                "Communication",
                vec![("sent", "2026-10-05T19:22:00Z")],
                json!({
                    "resourceType": "Communication",
                    "status": "completed",
                    "category": [{"coding": [{"system": "https://sprucehealth.com/category", "code": "patient-thread"}]}],
                    "sent": "2026-10-05T19:22:00Z",
                    "sender": {"display": "Roger Alvarez"},
                    "payload": [{"contentString": "Evening glucometerreading MSG_BODY_SENTINEL_DIZZINESS"}]
                }),
                vec![],
            ),
            expected_bucket: RecordBucket::Messages,
            word_query: "glucometerreading",
            code_query: "patient-thread",
            expected_code: None,
            expected_date_substring: "2026-10-05T19:22:00Z",
            expected_author_substring: "Roger Alvarez",
            body_sentinel: "MSG_BODY_SENTINEL_DIZZINESS",
        },
        SyntheticRecordCase {
            source: make_source_revision(
                workspace,
                "duet-bridge",
                "18-other-cgm-device",
                "rev-oth-1",
                "DeviceMetric",
                vec![("date", "2026-10-06")],
                json!({
                    "resourceType": "DeviceMetric",
                    "type": {"coding": [{"system": "urn:iso:std:iso:11073:10101", "code": "160196", "display": "Interstitialsensor telemetry"}]},
                    "date": "2026-10-06",
                    "note": [{"text": "OTHER_BODY_SENTINEL_CGM_CALIBRATION_DRIFT"}]
                }),
                vec![],
            ),
            expected_bucket: RecordBucket::Other,
            word_query: "Interstitialsensor",
            code_query: "160196",
            expected_code: Some("160196"),
            expected_date_substring: "2026-10-06",
            expected_author_substring: "duet-bridge",
            body_sentinel: "OTHER_BODY_SENTINEL_CGM_CALIBRATION_DRIFT",
        },
    ]
}

fn assert_no_chart_sentinels(serialized: &str, cases: &[SyntheticRecordCase]) {
    for case in cases {
        assert!(
            !serialized.contains(case.body_sentinel),
            "search hit leaked record body sentinel {}: {serialized}",
            case.body_sentinel
        );
    }
    for forbidden in ["8.4", "\"148\"", "\"92\"", "mmHg"] {
        assert!(
            !serialized.contains(forbidden),
            "search hit leaked quantitative measurement or unit {forbidden}: {serialized}"
        );
    }
}

#[tokio::test]
async fn ac1_words_and_codes_find_every_synthetic_chart_record_kind_without_body_leakage() {
    let Some(mut pg) = DisposablePostgres::acquire("patient-search-ac1").await else {
        return;
    };
    let store = pg.store().await;

    let scope = Scope {
        practice: "practice-ac1".into(),
        physician: "dr-vance".into(),
        workspace: "patient-roger-ac1".into(),
        chat: "chat-ac1".into(),
    };
    let session = [11_u8; 32];
    let mut authority = AllowScopeAuthority::new(&scope, session);

    let mut cases = synthetic_chart_cases(&scope.workspace);

    // Add two provenance-backed chart records with EMPTY `readable_parts` whose text lives in
    // `provenance["attachments"]` and `provenance["scans"]` resolved via `ProvenancePartReader`.
    let mut prov_attachment_source = make_source_revision(
        &scope.workspace,
        "epic-fhir",
        "19-prov-attachment-holter",
        "rev-prov-att-1",
        "DocumentReference",
        vec![("date", "2026-10-03")],
        json!({
            "resourceType": "DocumentReference",
            "status": "current",
            "type": {"coding": [{"system": "http://loinc.org", "code": "18748-4"}]},
            "author": [{"display": "Cardiology Holter Lab"}]
        }),
        vec![],
    );
    prov_attachment_source.provenance = json!({
        "source_id": "epic-fhir",
        "native_id": "19-prov-attachment-holter",
        "attachments": [AttachmentPart {
            attachment_id: "att-holter-prov".into(),
            label: "Holter Attachment Summary".into(),
            media_type: Some("text/plain".into()),
            state: PartState::Readable {
                text: "provenanceattachmentholter sinus rhythm PROVENANCE_ATTACHMENT_BODY_SENTINEL".into(),
            },
        }],
    });
    cases.push(SyntheticRecordCase {
        source: prov_attachment_source,
        expected_bucket: RecordBucket::Documents,
        word_query: "provenanceattachmentholter",
        code_query: "att-holter-prov",
        expected_code: Some("18748-4"),
        expected_date_substring: "2026-10-03",
        expected_author_substring: "Cardiology Holter Lab",
        body_sentinel: "PROVENANCE_ATTACHMENT_BODY_SENTINEL",
    });

    let mut prov_scan_source = make_source_revision(
        &scope.workspace,
        "epic-fhir",
        "20-prov-scan-fax",
        "rev-prov-scan-1",
        "Media",
        vec![("createdDateTime", "2026-10-04")],
        json!({
            "resourceType": "Media",
            "status": "completed",
            "operator": {"display": "Nephrology Fax Intake"}
        }),
        vec![],
    );
    prov_scan_source.provenance = json!({
        "source_id": "epic-fhir",
        "native_id": "20-prov-scan-fax",
        "scans": [ScanDocument {
            scan_id: "scan-neph-fax".into(),
            label: "Nephrology Fax Scan".into(),
            content: ScanDocumentState::Pages {
                pages: vec![
                    ScanPageEntry {
                        page: PageNumber::new(1).unwrap(),
                        state: PartState::Readable {
                            text: "Outside fax cover page".into(),
                        },
                    },
                    ScanPageEntry {
                        page: PageNumber::new(2).unwrap(),
                        state: PartState::Readable {
                            text: "provenancescanpagealbumin urine ACR 42 PROVENANCE_SCAN_PAGE_BODY_SENTINEL".into(),
                        },
                    },
                ],
            },
        }],
    });
    cases.push(SyntheticRecordCase {
        source: prov_scan_source,
        expected_bucket: RecordBucket::Documents,
        word_query: "provenancescanpagealbumin",
        code_query: "scan-neph-fax",
        expected_code: None,
        expected_date_substring: "2026-10-04",
        expected_author_substring: "Nephrology Fax Intake",
        body_sentinel: "PROVENANCE_SCAN_PAGE_BODY_SENTINEL",
    });

    let all_sources: Vec<SourceRevision> = cases.iter().map(|c| c.source.clone()).collect();
    store
        .import_sources(&all_sources)
        .await
        .expect("import synthetic chart");

    // Add 6 extra lab observations so total chart records = 26 (> SEARCH_PAGE_SIZE = 20)
    // to exercise keyset cursor continuation and stale-cursor fencing.
    for extra in 1..=6 {
        let extra_obs = make_source_revision(
            &scope.workspace,
            "epic-fhir",
            &format!("90-extra-obs-{extra:02}"),
            &format!("rev-extra-{extra}"),
            "Observation",
            vec![("effectiveDateTime", "2026-10-02")],
            json!({
                "resourceType": "Observation",
                "status": "final",
                "category": [{"coding": [{"code": "laboratory"}]}],
                "code": {"coding": [{"system": "http://loinc.org", "code": "2160-0"}]},
                "valueQuantity": {"value": 1.1, "unit": "mg/dL"}
            }),
            vec![],
        );
        store
            .record_source(&extra_obs)
            .await
            .expect("record extra obs");
    }

    let mut covered_buckets = BTreeSet::new();
    for case in &cases {
        covered_buckets.insert(case.expected_bucket);

        for query_str in [case.word_query, case.code_query] {
            let request = PatientSearchRequest {
                query: query_str.into(),
                record_type: None,
                bucket: None,
                domain: SearchDomainFilter::All,
                standing: SearchStandingScope::CurrentOnly,
                after: None,
            };
            let outcome = store
                .search_patient_authorized(
                    &scope,
                    &session,
                    SearchAudienceScope::PhysicianFacing,
                    &request,
                    &mut authority,
                )
                .await
                .expect("authorized search succeeds");

            let PatientSearchOutcome::Permitted(page) = outcome else {
                panic!("expected Permitted page for query {query_str}");
            };
            let serialized = serde_json::to_string(&page).expect("serialize page");
            assert_no_chart_sentinels(&serialized, &cases);
            assert!(!serialized.contains("mg/dL"));

            let matched = page
                .results
                .iter()
                .find_map(|hit| match hit {
                    PatientSearchHit::ChartRecord {
                        reference,
                        record_type,
                        bucket,
                        codes,
                        standing,
                        dates,
                        authors,
                    } if reference.candidate_id == case.source.candidate_id => Some((
                        reference,
                        record_type,
                        bucket,
                        codes,
                        standing,
                        dates,
                        authors,
                    )),
                    _ => None,
                })
                .unwrap_or_else(|| {
                    panic!(
                        "query `{query_str}` did not find candidate `{}`; got: {serialized}",
                        case.source.candidate_id
                    )
                });

            let (reference, record_type, bucket, codes, standing, dates, authors) = matched;
            assert_eq!(reference.source_id, case.source.source_id);
            assert_eq!(reference.revision, case.source.revision);
            assert_eq!(reference.cursor, None);
            assert_eq!(record_type, &case.source.record_type);
            assert_eq!(*bucket, case.expected_bucket);
            assert_eq!(standing, &HitStanding::Current);

            if let Some(expected_code) = case.expected_code {
                assert!(
                    codes
                        .iter()
                        .any(|ClinicalCode { code, .. }| code == expected_code),
                    "expected code {expected_code} in {codes:?}"
                );
            }
            assert!(
                dates
                    .iter()
                    .any(|d| d.value.contains(case.expected_date_substring)),
                "expected date {} in {dates:?}",
                case.expected_date_substring
            );
            let authors_json = serde_json::to_string(authors).expect("serialize authors");
            assert!(
                authors_json.contains(case.expected_author_substring),
                "expected author {} in {authors_json}",
                case.expected_author_substring
            );
        }
    }

    assert_eq!(
        covered_buckets.len(),
        RecordBucket::ALL.len(),
        "every product bucket must be covered"
    );

    // Pagination across 24 records (page 1 = 20, page 2 = 4) and stale-cursor detection.
    let browse_p1 = match store
        .search_patient_authorized(
            &scope,
            &session,
            SearchAudienceScope::PhysicianFacing,
            &PatientSearchRequest::default(),
            &mut authority,
        )
        .await
        .expect("page 1")
    {
        PatientSearchOutcome::Permitted(p) => p,
        other => panic!("unexpected {other:?}"),
    };
    assert_eq!(browse_p1.results.len(), 20);
    let cursor = browse_p1.next.clone().expect("has second page cursor");

    let browse_p2 = match store
        .search_patient_authorized(
            &scope,
            &session,
            SearchAudienceScope::PhysicianFacing,
            &PatientSearchRequest {
                after: Some(cursor.clone()),
                ..PatientSearchRequest::default()
            },
            &mut authority,
        )
        .await
        .expect("page 2")
    {
        PatientSearchOutcome::Permitted(p) => p,
        other => panic!("unexpected {other:?}"),
    };
    assert_eq!(browse_p2.results.len(), 6);
    assert!(browse_p2.next.is_none());

    // Exercise `PhysicianHost` end-to-end:
    // Turn 1 searches via `search_patient_information` (returns references for consult note + provenance attachment + provenance scan, zero body).
    // Turn 2 passes those exact references to `read_patient_sources` (opens full note, attachment, and scan page bodies).
    let recipe = chat_recipe();
    assert_eq!(
        store
            .accept_authorized(
                &scope,
                &Input {
                    key: "cmd-ac1-chat".into(),
                    text: "What did Dr. Maya Lin recommend in the endocrinology consult?".into(),
                    timezone: "America/Los_Angeles".into(),
                    scenario: None,
                },
                session.to_vec(),
                skills(),
                &recipe,
            )
            .await
            .expect("accept chat"),
        Accepted::New
    );

    let claim = store
        .claim(&scope, 60)
        .await
        .expect("claim")
        .expect("live claim");
    let mut provider = ScriptedProvider::new(vec![
        call_turn(vec![
            (
                "call-search-1",
                SEARCH_TOOL,
                json!({
                    "query": "11488-4",
                    "record_type": "DocumentReference",
                    "bucket": "notes",
                    "after": null
                }),
            ),
            (
                "call-search-prov-att",
                SEARCH_TOOL,
                json!({
                    "query": "provenanceattachmentholter",
                    "after": null
                }),
            ),
            (
                "call-search-prov-scan",
                SEARCH_TOOL,
                json!({
                    "query": "provenancescanpagealbumin",
                    "after": null
                }),
            ),
        ]),
        call_turn(vec![(
            "call-read-1",
            READ_TOOL,
            json!({
                "references": [
                    {
                        "source_id": "epic-fhir",
                        "candidate_id": "07-docref-consult-note",
                        "revision": "rev-note-1",
                        "cursor": null
                    },
                    {
                        "source_id": "epic-fhir",
                        "candidate_id": "19-prov-attachment-holter",
                        "revision": "rev-prov-att-1",
                        "cursor": null
                    },
                    {
                        "source_id": "epic-fhir",
                        "candidate_id": "20-prov-scan-fax",
                        "revision": "rev-prov-scan-1",
                        "cursor": null
                    }
                ],
                "max_result_bytes": 16384
            }),
        )]),
        assistant_turn("Dr. Maya Lin's 2026-09-30 consult note outlined the glargine plan."),
    ]);

    PhysicianHost {
        store: store.clone(),
        claim,
        authority: AllowScopeAuthority::new(&scope, session),
    }
    .run(
        &mut provider,
        engine::RunLimits::default(),
        &engine::Cancellation::default(),
    )
    .await
    .expect("host run succeeds");

    let search_tool_output = serde_json::to_string(&provider.seen_requests[1].items).unwrap();
    assert!(
        search_tool_output.contains("07-docref-consult-note")
            && search_tool_output.contains("rev-note-1")
            && search_tool_output.contains("19-prov-attachment-holter")
            && search_tool_output.contains("20-prov-scan-fax")
    );
    assert!(
        !search_tool_output.contains("NOTE_BODY_SENTINEL_CONSULT")
            && !search_tool_output.contains("PROVENANCE_ATTACHMENT_BODY_SENTINEL")
            && !search_tool_output.contains("PROVENANCE_SCAN_PAGE_BODY_SENTINEL"),
        "search tool output must never carry note, attachment, or scan bodies before read_patient_sources"
    );

    let read_tool_output = serde_json::to_string(&provider.seen_requests[2].items).unwrap();
    assert!(
        read_tool_output.contains("NOTE_BODY_SENTINEL_CONSULT")
            && read_tool_output.contains("PROVENANCE_ATTACHMENT_BODY_SENTINEL")
            && read_tool_output.contains("PROVENANCE_SCAN_PAGE_BODY_SENTINEL"),
        "explicit read_patient_sources call opens note, provenance attachment, and provenance scan page bodies"
    );

    // Mutate workspace sources and confirm the old cursor is refused as stale.
    store
        .record_source(&make_source_revision(
            &scope.workspace,
            "epic-fhir",
            "99-fresh-arrival",
            "rev-fresh-1",
            "Observation",
            vec![("effectiveDateTime", "2026-10-07")],
            json!({"resourceType": "Observation", "code": "2160-0"}),
            vec![],
        ))
        .await
        .expect("record fresh arrival");
    let stale = store
        .search_patient_authorized(
            &scope,
            &session,
            SearchAudienceScope::PhysicianFacing,
            &PatientSearchRequest {
                after: Some(cursor),
                ..PatientSearchRequest::default()
            },
            &mut authority,
        )
        .await;
    assert!(matches!(stale, Err(PatientSearchError::StaleCursor)));

    pg.stop_immediate().await;
}

#[tokio::test]
async fn ac2_same_search_finds_earlier_work_and_memory_versions_with_dates_and_authors() {
    let Some(mut pg) = DisposablePostgres::acquire("patient-search-ac2").await else {
        return;
    };
    let store = pg.store().await;

    let practice = "practice-ac2".to_owned();
    let workspace = "patient-claire-ac2".to_owned();
    let patient = PatientKey::new(&practice, &workspace).expect("patient key");
    let session = [22_u8; 32];

    // 1. Seed a chart record mentioning `cardiorenalplan`.
    store
        .record_source(&make_source_revision(
            &workspace,
            "epic-fhir",
            "lab-egfr-1",
            "rev-egfr-1",
            "Observation",
            vec![("effectiveDateTime", "2026-10-10T07:30:00Z")],
            json!({
                "resourceType": "Observation",
                "status": "final",
                "category": [{"coding": [{"code": "laboratory"}]}],
                "code": {"coding": [{"system": "http://loinc.org", "code": "33914-3", "display": "eGFR cardiorenalplan"}]},
                "performer": [{"display": "Quest Lab"}],
                "note": [{"text": "SECRET_CHART_BODY_AC2"}]
            }),
            vec![],
        ))
        .await
        .expect("seed chart record");

    // 2. Seed an earlier completed conversation (`chat-prior`) where physician & Coda discuss `cardiorenalplan`,
    //    AND write memory v3 (summary + updated topic) inside that conversation.
    //    First seed memory v1 (PracticeJob operating index) and v2 (PhysicianDirect correction) so memory has v1, v2, v3!
    let mut v1_topics = std::collections::BTreeMap::new();
    v1_topics.insert(
        TopicKey::parse("renal_trajectory").unwrap(),
        "cardiorenalplan stage3a PRIVATE_OP_NOTE_V1".to_owned(),
    );
    let v1_caller = JobMemoryCaller::new("patient-review", "review-job-42").unwrap();
    let v1_out = store
        .mutate_patient_memory_in_job(
            &patient,
            &v1_caller,
            "call-v1",
            MemoryMutation::UpdateUnderstanding(
                OperatingIndexPatch::new(v1_topics, BTreeSet::new()).unwrap(),
            ),
        )
        .await
        .expect("memory v1");
    assert!(matches!(v1_out, MemoryMutationOutcome::Applied(_)));

    let prior_chat_scope = Scope {
        practice: practice.clone(),
        physician: "dr-vance".into(),
        workspace: workspace.clone(),
        chat: "chat-prior-oct".into(),
    };
    let mut chat_auth = AllowScopeAuthority::new(&prior_chat_scope, session);
    let recipe = chat_recipe();
    assert_eq!(
        store
            .accept_authorized(
                &prior_chat_scope,
                &Input {
                    key: "msg-user-oct-1".into(),
                    text: "Let's review the cardiorenalplan before starting dapagliflozin. SECRET_USER_MSG_BODY".into(),
                    timezone: "America/Los_Angeles".into(),
                    scenario: None,
                },
                session.to_vec(),
                skills(),
                &recipe,
            )
            .await
            .expect("accept prior chat"),
        Accepted::New
    );

    // Memory v2: direct physician correction referencing `msg-user-oct-1`.
    let v2_out = store
        .mutate_patient_memory_authorized(
            &prior_chat_scope,
            "cmd-mem-v2",
            &session,
            MemoryMutation::RecordCorrection(RecordCorrection {
                correction_id: Some(CorrectionId::parse("corr-dapagliflozin").unwrap()),
                recorded_on: ClinicalDate::parse("2026-10-11").unwrap(),
                source_message: SourceMessageRef::new(
                    "chat-prior-oct",
                    "msg-user-oct-1",
                    "dr-vance",
                )
                .unwrap(),
                statement:
                    "cardiorenalplan hold SGLT2i during acute dehydration SECRET_CORR_STATEMENT"
                        .into(),
                supersedes: None,
                understanding_patch: None,
            }),
            &mut chat_auth,
        )
        .await
        .expect("memory v2");
    assert!(matches!(v2_out, MemoryMutationOutcome::Applied(_)));

    // Run `chat-prior-oct` so Coda saves memory v3 (summary + updated operating topic) and publishes assistant reply.
    let claim_prior = store
        .claim(&prior_chat_scope, 60)
        .await
        .expect("claim prior")
        .expect("live claim");
    let mut prior_provider = ScriptedProvider::new(vec![
        call_turn(vec![
            (
                "call-mem-v3-topic",
                SAVE_MEMORY_TOOL,
                json!({
                    "upsert": [{
                        "topic": "renal_trajectory",
                        "note": "cardiorenalplan resume SGLT2i once euvolemic PRIVATE_OP_NOTE_V3"
                    }],
                    "remove": []
                }),
            ),
            (
                "call-mem-v3-summary",
                SAVE_SUMMARY_TOOL,
                json!({
                    "content": "cardiorenalplan stabilized with hydration protocol SECRET_SUMMARY_PROSE",
                    "references": [{
                        "kind": "chart_record",
                        "source_id": "epic-fhir",
                        "candidate_id": "lab-egfr-1",
                        "revision": "rev-egfr-1",
                        "label": "eGFR panel",
                        "record_date": "2026-10-10"
                    }],
                    "dates": {
                        "written_at": "2026-10-12",
                        "reviewed_at": "2026-10-12",
                        "pulled_at": "2026-10-10"
                    }
                }),
            ),
        ]),
        assistant_turn(
            "Agreed on the cardiorenalplan sick-day hold criteria. SECRET_ASSISTANT_MSG_BODY",
        ),
    ]);
    PhysicianHost {
        store: store.clone(),
        claim: claim_prior,
        authority: AllowScopeAuthority::new(&prior_chat_scope, session),
    }
    .run(
        &mut prior_provider,
        engine::RunLimits::default(),
        &engine::Cancellation::default(),
    )
    .await
    .expect("complete prior chat");

    // 3. Seed reply-draft work (`reply-spruce-1`) with an archived earlier revision AND a current completed revision.
    let reply_scope = Scope {
        practice: practice.clone(),
        physician: "dr-vance".into(),
        workspace: workspace.clone(),
        chat: "reply-spruce-1".into(),
    };
    let mut reply_auth = AllowScopeAuthority::new(&reply_scope, session);
    store
        .open_draft_authorized(
            &reply_scope,
            "Initial draft text SECRET_INITIAL_DRAFT",
            &session,
            &mut reply_auth,
        )
        .await
        .expect("open draft")
        .expect("authorized draft");

    complete_reply_revision(
        &store,
        &reply_scope,
        session,
        &mut reply_auth,
        RevisionFixture {
            key: "rev-cmd-first",
            expected_revision: 1,
            instruction: "Include cardiorenalplan hydration guidance",
            revised_text: "First revised reply on cardiorenalplan SECRET_ARCHIVED_DRAFT_BODY",
        },
    )
    .await;
    complete_reply_revision(
        &store,
        &reply_scope,
        session,
        &mut reply_auth,
        RevisionFixture {
            key: "rev-cmd-second",
            expected_revision: 2,
            instruction: "Tighten cardiorenalplan lab follow-up date",
            revised_text: "Current revised reply on cardiorenalplan SECRET_CURRENT_DRAFT_BODY",
        },
    )
    .await;

    // 4. Unified search for `"cardiorenalplan"` with `CurrentAndHistorical` across all domains as an AgentJob
    //    (and via `PhysicianHost` `search_patient_information`).
    let agent_outcome = store
        .search_patient_for_access(
            &patient,
            &SearchCaller::ChatJob {
                physician_id: "dr-vance".into(),
            },
            PatientAccess::ClinicalPanel,
            &PatientSearchRequest {
                query: "cardiorenalplan".into(),
                record_type: None,
                bucket: None,
                domain: SearchDomainFilter::All,
                standing: SearchStandingScope::CurrentAndHistorical,
                after: None,
            },
        )
        .await
        .expect("agent search");

    let PatientSearchOutcome::Permitted(agent_page) = agent_outcome else {
        panic!("expected Permitted page");
    };
    let agent_json = serde_json::to_string(&agent_page).expect("serialize agent page");

    // Zero body/prose leakage across all 3 owners!
    for secret in [
        "SECRET_CHART_BODY_AC2",
        "SECRET_USER_MSG_BODY",
        "SECRET_ASSISTANT_MSG_BODY",
        "SECRET_INITIAL_DRAFT",
        "SECRET_ARCHIVED_DRAFT_BODY",
        "SECRET_CURRENT_DRAFT_BODY",
        "SECRET_CORR_STATEMENT",
        "SECRET_SUMMARY_PROSE",
        "PRIVATE_OP_NOTE_V1",
        "PRIVATE_OP_NOTE_V3",
    ] {
        assert!(
            !agent_json.contains(secret),
            "search hit leaked protected body text `{secret}`: {agent_json}"
        );
    }

    // Verify ChartRecord hit is present with date + author.
    assert!(agent_page.results.iter().any(|hit| matches!(
        hit,
        PatientSearchHit::ChartRecord { reference, standing: HitStanding::Current, dates, authors, .. }
            if reference.candidate_id == "lab-egfr-1"
            && !dates.is_empty()
            && !authors.is_empty()
    )));

    // Verify EarlierWork hits: user turn, assistant reply, current saved draft, and archived reply revision — all with dates & authors!
    assert!(agent_page.results.iter().any(|hit| matches!(
        hit,
        PatientSearchHit::EarlierWork {
            reference: WorkReference::ConversationMessage { chat_id, message_id, .. },
            work_kind: WorkHitKind::ConversationUserMessage,
            standing: HitStanding::Current,
            dates,
            authors,
        } if chat_id == "chat-prior-oct"
            && message_id == "msg-user-oct-1"
            && !dates.is_empty()
            && serde_json::to_string(authors).unwrap().contains("dr-vance")
    )));
    assert!(agent_page.results.iter().any(|hit| matches!(
        hit,
        PatientSearchHit::EarlierWork {
            reference: WorkReference::ConversationMessage { chat_id, .. },
            work_kind: WorkHitKind::ConversationAssistantReply,
            standing: HitStanding::Current,
            dates,
            authors,
        } if chat_id == "chat-prior-oct"
            && !dates.is_empty()
            && serde_json::to_string(authors).unwrap().contains("coda_chat")
    )));
    assert!(agent_page.results.iter().any(|hit| matches!(
        hit,
        PatientSearchHit::EarlierWork {
            reference: WorkReference::ReplyDraft { reply_id, draft_revision: 3, .. },
            work_kind: WorkHitKind::SavedReplyDraft,
            standing: HitStanding::Current,
            dates,
            authors,
        } if reply_id == "reply-spruce-1"
            && !dates.is_empty()
            && serde_json::to_string(authors).unwrap().contains("coda_reply_revision")
    )));
    assert!(agent_page.results.iter().any(|hit| matches!(
        hit,
        PatientSearchHit::EarlierWork {
            reference: WorkReference::ReplyRevisionHistory { reply_id, ordinal: 1, command_key },
            work_kind: WorkHitKind::ArchivedReplyRevision,
            standing: HitStanding::Superseded { .. },
            dates,
            authors,
        } if reply_id == "reply-spruce-1"
            && command_key == "rev-cmd-first"
            && dates.len() >= 2
            && serde_json::to_string(authors).unwrap().contains("dr-vance")
    )));

    // Verify PatientMemory hits span v1 (PracticeJob, Prior -> Superseded), v2 (PhysicianDirect correction),
    // v3 (ChatJob operating topic), and v4 (ChatJob summary + current snapshot) — all with dates & authors!
    let memory_versions_found: BTreeSet<u64> = agent_page
        .results
        .iter()
        .filter_map(|hit| match hit {
            PatientSearchHit::PatientMemory {
                reference,
                dates,
                authors,
                ..
            } => {
                assert!(
                    !dates.is_empty(),
                    "memory hit must carry dates: {reference:?}"
                );
                assert!(
                    !authors.is_empty(),
                    "memory hit must carry authors: {reference:?}"
                );
                Some(reference.version.get())
            }
            _ => None,
        })
        .collect();
    assert_eq!(
        memory_versions_found,
        BTreeSet::from([1, 2, 3, 4]),
        "historical search must find v1 (PracticeJob), v2 (PhysicianDirect), v3 (ChatJob topic), and v4 (ChatJob summary)"
    );

    // 5. Physician-facing search (`SearchAudienceScope::PhysicianFacing`) finds ChartRecord,
    //    EarlierWork, Correction, and PatientSummary, while structurally excluding ALL `OperatingTopic` items!
    let physician_outcome = store
        .search_patient_authorized(
            &prior_chat_scope,
            &session,
            SearchAudienceScope::PhysicianFacing,
            &PatientSearchRequest {
                query: "cardiorenalplan".into(),
                record_type: None,
                bucket: None,
                domain: SearchDomainFilter::All,
                standing: SearchStandingScope::CurrentAndHistorical,
                after: None,
            },
            &mut chat_auth,
        )
        .await
        .expect("physician-facing search");
    let PatientSearchOutcome::Permitted(physician_page) = physician_outcome else {
        panic!("expected Permitted physician page");
    };
    let physician_json = serde_json::to_string(&physician_page).unwrap();
    assert!(
        !physician_json.contains("operating_topic") && !physician_json.contains("renal_trajectory"),
        "physician-facing search must never expose private operating-index topics: {physician_json}"
    );
    assert!(physician_page.results.iter().any(|hit| matches!(
        hit,
        PatientSearchHit::PatientMemory {
            reference,
            dates,
            ..
        } if matches!(reference.entry, MemoryEntryPointer::Correction { .. })
            && dates.iter().any(|d| d.value == "2026-10-11")
    )));
    assert!(physician_page.results.iter().any(|hit| matches!(
        hit,
        PatientSearchHit::PatientMemory {
            reference,
            standing: HitStanding::Current,
            dates,
            ..
        } if matches!(reference.entry, MemoryEntryPointer::PatientSummary { .. })
            && dates.iter().any(|d| d.value == "2026-10-12")
    )));

    // 6. Verify `SearchCaller::PracticeJob` pagination across two physicians sharing identical
    //    `(chat_id, idx, message_id)` preserves both hits across a keyset page boundary!
    for filler_idx in 0..19 {
        store
            .record_source(&make_source_revision(
                &workspace,
                "epic-fhir",
                &format!("00-keyset-filler-{filler_idx:02}"),
                &format!("rev-keyset-{filler_idx}"),
                "Observation",
                vec![("effectiveDateTime", "2026-10-10")],
                json!({
                    "resourceType": "Observation",
                    "code": "2160-0",
                    "note": [{"text": "practicejobkeysetcollision"}]
                }),
                vec![],
            ))
            .await
            .unwrap();
    }
    for colliding_physician in ["dr-alpha-coll", "dr-zeta-coll"] {
        let coll_scope = Scope {
            practice: practice.clone(),
            physician: colliding_physician.into(),
            workspace: workspace.clone(),
            chat: "shared-local-chat-id".into(),
        };
        store
            .accept_authorized(
                &coll_scope,
                &Input {
                    key: "same-msg-key".into(),
                    text: "practicejobkeysetcollision shared note".into(),
                    timezone: "UTC".into(),
                    scenario: None,
                },
                session.to_vec(),
                skills(),
                &recipe,
            )
            .await
            .unwrap();
    }
    let job_p1 = match store
        .search_patient_for_access(
            &patient,
            &SearchCaller::PracticeJob {
                job_kind: "patient-review".into(),
                job_id: "job-keyset-1".into(),
            },
            PatientAccess::ClinicalPanel,
            &PatientSearchRequest {
                query: "practicejobkeysetcollision".into(),
                ..PatientSearchRequest::default()
            },
        )
        .await
        .unwrap()
    {
        PatientSearchOutcome::Permitted(p) => p,
        other => panic!("unexpected {other:?}"),
    };
    assert_eq!(job_p1.results.len(), 20);
    let job_cursor = job_p1.next.expect("second page cursor for practice job");
    let job_p2 = match store
        .search_patient_for_access(
            &patient,
            &SearchCaller::PracticeJob {
                job_kind: "patient-review".into(),
                job_id: "job-keyset-1".into(),
            },
            PatientAccess::ClinicalPanel,
            &PatientSearchRequest {
                query: "practicejobkeysetcollision".into(),
                after: Some(job_cursor),
                ..PatientSearchRequest::default()
            },
        )
        .await
        .unwrap()
    {
        PatientSearchOutcome::Permitted(p) => p,
        other => panic!("unexpected {other:?}"),
    };
    assert_eq!(
        job_p2.results.len(),
        1,
        "second physician's colliding (chat_id, idx, message_id) must survive keyset pagination"
    );
    let combined_authors: BTreeSet<String> = job_p1
        .results
        .iter()
        .chain(job_p2.results.iter())
        .filter_map(|hit| match hit {
            PatientSearchHit::EarlierWork { authors, .. } => match authors.first() {
                Some(SearchAuthor::Physician { physician_id }) => Some(physician_id.clone()),
                _ => None,
            },
            _ => None,
        })
        .collect();
    assert_eq!(
        combined_authors,
        BTreeSet::from(["dr-alpha-coll".to_owned(), "dr-zeta-coll".to_owned()])
    );

    pg.stop_immediate().await;
}

#[tokio::test]
async fn ac3_corrected_and_withdrawn_records_work_and_memory_stop_showing_as_current() {
    let Some(mut pg) = DisposablePostgres::acquire("patient-search-ac3").await else {
        return;
    };
    let store = pg.store().await;

    let scope = Scope {
        practice: "practice-ac3".into(),
        physician: "dr-vance".into(),
        workspace: "patient-ac3".into(),
        chat: "chat-ac3".into(),
    };
    let session = [33_u8; 32];
    let mut auth = AllowScopeAuthority::new(&scope, session);
    let pull = SourcePull {
        source_id: "epic-fhir".into(),
        source_label: "Epic".into(),
        patient: SourcePatient {
            source_patient_id: "pat-ac3".into(),
            display_name: "Patient AC3".into(),
            birth_date: Some("1975-06-01".into()),
            sex_or_gender: Some("female".into()),
            medical_record_numbers: vec![],
        },
        pulled_at: 1_760_000_000,
    };

    // 1. Initial chart delivery:
    //    - `obs-potassium` rev-1 (`hemolyzedpotassium`)
    //    - `med-warfarin` rev-1 (`discontinuedwarfarin`)
    //    - `alg-sulfa` rev-1 (`enteredinerrorsulfa` with FHIR verificationStatus = entered-in-error)
    let k_rev1 = make_source_revision(
        &scope.workspace,
        "epic-fhir",
        "obs-potassium",
        "rev-k-1",
        "Observation",
        vec![("effectiveDateTime", "2026-10-01T08:00:00Z")],
        json!({
            "resourceType": "Observation",
            "status": "preliminary",
            "code": {"coding": [{"system": "http://loinc.org", "code": "2823-3"}]},
            "note": [{"text": "hemolyzedpotassium specimen"}]
        }),
        vec![],
    );
    let warfarin_rev1 = make_source_revision(
        &scope.workspace,
        "epic-fhir",
        "med-warfarin",
        "rev-warf-1",
        "MedicationRequest",
        vec![("authoredOn", "2026-05-01")],
        json!({
            "resourceType": "MedicationRequest",
            "status": "active",
            "medicationCodeableConcept": {"text": "discontinuedwarfarin 5mg"}
        }),
        vec![],
    );
    let sulfa_error = make_source_revision(
        &scope.workspace,
        "epic-fhir",
        "alg-sulfa",
        "rev-sulfa-1",
        "AllergyIntolerance",
        vec![("recordedDate", "2026-05-02")],
        json!({
            "resourceType": "AllergyIntolerance",
            "verificationStatus": {"coding": [{"code": "entered-in-error"}]},
            "code": {"text": "enteredinerrorsulfa rash"}
        }),
        vec![],
    );
    store
        .record_chart(
            &scope.workspace,
            &[k_rev1, warfarin_rev1, sulfa_error.clone()],
            &pull,
        )
        .await
        .expect("delivery 1");

    // 2. Second chart delivery:
    //    - `obs-potassium` corrected to `rev-k-2` (`verifiedpotassium`)
    //    - `med-warfarin` omitted -> withdrawn (`revision: None`)
    //    - `alg-sulfa` kept (still `entered-in-error`)
    let k_rev2 = make_source_revision(
        &scope.workspace,
        "epic-fhir",
        "obs-potassium",
        "rev-k-2",
        "Observation",
        vec![("effectiveDateTime", "2026-10-01T10:00:00Z")],
        json!({
            "resourceType": "Observation",
            "status": "amended",
            "code": {"coding": [{"system": "http://loinc.org", "code": "2823-3"}]},
            "note": [{"text": "verifiedpotassium repeat draw normal"}]
        }),
        vec![],
    );
    store
        .record_chart(&scope.workspace, &[k_rev2, sulfa_error], &pull)
        .await
        .expect("delivery 2");

    // 3. Earlier work:
    //    - Reply draft with archived revision (`archiveddraftwording`), cancelled revision (`cancelledinstructionwording`),
    //      and current saved draft (`currentapproveddraftwording`).
    let reply_scope = Scope {
        chat: "reply-ac3".into(),
        ..scope.clone()
    };
    let mut reply_auth = AllowScopeAuthority::new(&reply_scope, session);
    store
        .open_draft_authorized(&reply_scope, "seedreplytext", &session, &mut reply_auth)
        .await
        .unwrap()
        .unwrap();
    complete_reply_revision(
        &store,
        &reply_scope,
        session,
        &mut reply_auth,
        RevisionFixture {
            key: "ac3-rev-1",
            expected_revision: 1,
            instruction: "Write archiveddraftwording",
            revised_text: "archiveddraftwording body",
        },
    )
    .await;
    complete_reply_revision(
        &store,
        &reply_scope,
        session,
        &mut reply_auth,
        RevisionFixture {
            key: "ac3-rev-2",
            expected_revision: 2,
            instruction: "Write interim wording",
            revised_text: "interim wording",
        },
    )
    .await;
    assert_eq!(
        store
            .accept_revision(
                &reply_scope,
                &RevisionInput {
                    key: "ac3-rev-cancelled".into(),
                    expected_revision: 3,
                    instruction: "cancelledinstructionwording".into(),
                    timezone: "UTC".into(),
                    scenario: None,
                },
                session.to_vec(),
                skills(),
                &reply_recipe(),
            )
            .await
            .unwrap(),
        RevisionAccepted::New
    );
    // Manual physician save cancels `ac3-rev-cancelled` and establishes `currentapproveddraftwording`.
    store
        .save_draft(
            &reply_scope,
            3,
            "currentapproveddraftwording",
            &session,
            &mut reply_auth,
        )
        .await
        .expect("manual save cancels pending revision");

    // 4. Patient memory:
    //    - Record correction `corr-1` (`supersededstatinnote`)
    //    - Supersede `corr-1` with `corr-2` (`currentrosuvastatinnote`)
    //    - Record correction `corr-3` (`withdrawnallergy Shellfish` -> `withdrawnshellfishnote`) and withdraw it
    //    - Write version with `retractedwrongchartnote` and retract that version!
    let recipe = chat_recipe();
    store
        .accept_authorized(
            &scope,
            &Input {
                key: "msg-ac3-1".into(),
                text: "Update corrections".into(),
                timezone: "UTC".into(),
                scenario: None,
            },
            session.to_vec(),
            skills(),
            &recipe,
        )
        .await
        .unwrap();
    let msg_ref = SourceMessageRef::new(&scope.chat, "msg-ac3-1", &scope.physician).unwrap();

    store
        .mutate_patient_memory_authorized(
            &scope,
            "mem-c1",
            &session,
            MemoryMutation::RecordCorrection(RecordCorrection {
                correction_id: Some(CorrectionId::parse("corr-1").unwrap()),
                recorded_on: ClinicalDate::parse("2026-10-01").unwrap(),
                source_message: msg_ref.clone(),
                statement: "supersededstatinnote".into(),
                supersedes: None,
                understanding_patch: None,
            }),
            &mut auth,
        )
        .await
        .unwrap();

    store
        .mutate_patient_memory_authorized(
            &scope,
            "mem-c2",
            &session,
            MemoryMutation::RecordCorrection(RecordCorrection {
                correction_id: Some(CorrectionId::parse("corr-2").unwrap()),
                recorded_on: ClinicalDate::parse("2026-10-02").unwrap(),
                source_message: msg_ref.clone(),
                statement: "currentrosuvastatinnote".into(),
                supersedes: Some(CorrectionId::parse("corr-1").unwrap()),
                understanding_patch: None,
            }),
            &mut auth,
        )
        .await
        .unwrap();

    store
        .mutate_patient_memory_authorized(
            &scope,
            "mem-c3",
            &session,
            MemoryMutation::RecordCorrection(RecordCorrection {
                correction_id: Some(CorrectionId::parse("corr-3").unwrap()),
                recorded_on: ClinicalDate::parse("2026-10-03").unwrap(),
                source_message: msg_ref.clone(),
                statement: "withdrawnshellfishnote".into(),
                supersedes: None,
                understanding_patch: None,
            }),
            &mut auth,
        )
        .await
        .unwrap();

    store
        .mutate_patient_memory_authorized(
            &scope,
            "mem-c3-withdraw",
            &session,
            MemoryMutation::WithdrawCorrection(WithdrawCorrection {
                correction_id: CorrectionId::parse("corr-3").unwrap(),
                withdrawn_on: ClinicalDate::parse("2026-10-04").unwrap(),
                source_message: msg_ref.clone(),
                reason: "Confirmed negative challenge".into(),
                understanding_patch: None,
            }),
            &mut auth,
        )
        .await
        .unwrap();

    let bad_write = store
        .mutate_patient_memory_authorized(
            &scope,
            "mem-bad-v5",
            &session,
            MemoryMutation::RecordCorrection(RecordCorrection {
                correction_id: Some(CorrectionId::parse("corr-wrong-patient").unwrap()),
                recorded_on: ClinicalDate::parse("2026-10-05").unwrap(),
                source_message: msg_ref.clone(),
                statement: "retractedwrongchartnote".into(),
                supersedes: None,
                understanding_patch: None,
            }),
            &mut auth,
        )
        .await
        .unwrap();
    let MemoryMutationOutcome::Applied(bad_receipt) = bad_write else {
        panic!("expected applied v5");
    };
    let retracted = store
        .mutate_patient_memory_authorized(
            &scope,
            "cmd-retract-v5",
            &session,
            MemoryMutation::RetractVersion {
                target_version: bad_receipt.version,
                retracted_on: ClinicalDate::parse("2026-10-06").unwrap(),
                reason: "wrong chart import".into(),
                disposition: RetractionDisposition::ReplaceCurrent(ReplacementState {
                    operating_index: OperatingIndex::default(),
                    corrections: vec![
                        MemoryCorrection {
                            correction_id: CorrectionId::parse("corr-1").unwrap(),
                            recorded_on: ClinicalDate::parse("2026-10-01").unwrap(),
                            source_message: msg_ref.clone(),
                            statement: "supersededstatinnote".into(),
                            standing: CorrectionStanding::Superseded {
                                superseded_by: CorrectionId::parse("corr-2").unwrap(),
                            },
                        },
                        MemoryCorrection {
                            correction_id: CorrectionId::parse("corr-2").unwrap(),
                            recorded_on: ClinicalDate::parse("2026-10-02").unwrap(),
                            source_message: msg_ref.clone(),
                            statement: "currentrosuvastatinnote".into(),
                            standing: CorrectionStanding::Active,
                        },
                        MemoryCorrection {
                            correction_id: CorrectionId::parse("corr-3").unwrap(),
                            recorded_on: ClinicalDate::parse("2026-10-03").unwrap(),
                            source_message: msg_ref.clone(),
                            statement: "withdrawnshellfishnote".into(),
                            standing: CorrectionStanding::Withdrawn {
                                withdrawn_on: ClinicalDate::parse("2026-10-04").unwrap(),
                                source_message: msg_ref,
                                reason: "Confirmed negative challenge".into(),
                            },
                        },
                    ],
                    summary: None,
                }),
            },
            &mut auth,
        )
        .await
        .unwrap();
    assert!(matches!(retracted, MemoryMutationOutcome::Applied(_)));

    // A. Verify `CurrentOnly` excludes EVERY corrected, deleted/withdrawn, entered-in-error,
    //    archived, cancelled, and retracted item!
    for non_current_term in [
        "hemolyzedpotassium",
        "discontinuedwarfarin",
        "enteredinerrorsulfa",
        "archiveddraftwording",
        "cancelledinstructionwording",
        "supersededstatinnote",
        "withdrawnshellfishnote",
        "retractedwrongchartnote",
    ] {
        let out = store
            .search_patient_authorized(
                &scope,
                &session,
                SearchAudienceScope::AgentJob,
                &PatientSearchRequest {
                    query: non_current_term.into(),
                    standing: SearchStandingScope::CurrentOnly,
                    ..PatientSearchRequest::default()
                },
                &mut auth,
            )
            .await
            .unwrap();
        let PatientSearchOutcome::Permitted(page) = out else {
            panic!("expected Permitted");
        };
        assert!(
            page.results.is_empty(),
            "non-current term `{non_current_term}` must not appear in CurrentOnly search, got: {:?}",
            page.results
        );
    }

    // Confirm replacement/active items DO show as `HitStanding::Current` in `CurrentOnly`!
    for active_term in [
        "verifiedpotassium",
        "currentapproveddraftwording",
        "currentrosuvastatinnote",
    ] {
        let out = store
            .search_patient_authorized(
                &scope,
                &session,
                SearchAudienceScope::AgentJob,
                &PatientSearchRequest {
                    query: active_term.into(),
                    standing: SearchStandingScope::CurrentOnly,
                    ..PatientSearchRequest::default()
                },
                &mut auth,
            )
            .await
            .unwrap();
        let PatientSearchOutcome::Permitted(page) = out else {
            panic!("expected Permitted");
        };
        assert_eq!(
            page.results.len(),
            1,
            "active term `{active_term}` must appear once"
        );
        for hit in &page.results {
            let standing = match hit {
                PatientSearchHit::ChartRecord { standing, .. }
                | PatientSearchHit::EarlierWork { standing, .. }
                | PatientSearchHit::PatientMemory { standing, .. } => standing,
            };
            assert_eq!(standing, &HitStanding::Current);
            if active_term == "currentapproveddraftwording" {
                match hit {
                    PatientSearchHit::EarlierWork {
                        reference:
                            WorkReference::ReplyDraft {
                                reply_id,
                                draft_revision,
                                command_key,
                            },
                        work_kind: WorkHitKind::SavedReplyDraft,
                        dates,
                        authors,
                        ..
                    } => {
                        assert_eq!(reply_id, "reply-ac3");
                        assert_eq!(*draft_revision, 4);
                        assert_eq!(
                            *command_key, None,
                            "manual save after cancelled revision must not attach cancelled command_key"
                        );
                        assert!(
                            dates.is_empty(),
                            "manual save must not carry cancelled command accepted_at: {dates:?}"
                        );
                        assert_eq!(
                            authors,
                            &vec![SearchAuthor::Physician {
                                physician_id: "dr-vance".into(),
                            }]
                        );
                    }
                    other => panic!("expected SavedReplyDraft hit, got {other:?}"),
                }
            }
        }
    }

    // B. Verify `CurrentAndHistorical` surfaces superseded/withdrawn items ONLY with non-Current standing,
    //    and NEVER surfaces retracted memory versions at all!
    for superseded_term in [
        "hemolyzedpotassium",
        "archiveddraftwording",
        "supersededstatinnote",
    ] {
        let PatientSearchOutcome::Permitted(page) = store
            .search_patient_authorized(
                &scope,
                &session,
                SearchAudienceScope::AgentJob,
                &PatientSearchRequest {
                    query: superseded_term.into(),
                    standing: SearchStandingScope::CurrentAndHistorical,
                    ..PatientSearchRequest::default()
                },
                &mut auth,
            )
            .await
            .unwrap()
        else {
            panic!("expected Permitted");
        };
        assert!(!page.results.is_empty(), "{superseded_term} in historical");
        for hit in &page.results {
            let standing = match hit {
                PatientSearchHit::ChartRecord { standing, .. }
                | PatientSearchHit::EarlierWork { standing, .. }
                | PatientSearchHit::PatientMemory { standing, .. } => standing,
            };
            assert!(
                matches!(standing, HitStanding::Superseded { .. }),
                "expected Superseded for `{superseded_term}`, got {standing:?}"
            );
        }
    }

    for withdrawn_term in [
        "discontinuedwarfarin",
        "enteredinerrorsulfa",
        "cancelledinstructionwording",
        "withdrawnshellfishnote",
    ] {
        let PatientSearchOutcome::Permitted(page) = store
            .search_patient_authorized(
                &scope,
                &session,
                SearchAudienceScope::AgentJob,
                &PatientSearchRequest {
                    query: withdrawn_term.into(),
                    standing: SearchStandingScope::CurrentAndHistorical,
                    ..PatientSearchRequest::default()
                },
                &mut auth,
            )
            .await
            .unwrap()
        else {
            panic!("expected Permitted");
        };
        assert!(!page.results.is_empty(), "{withdrawn_term} in historical");
        for hit in &page.results {
            let standing = match hit {
                PatientSearchHit::ChartRecord { standing, .. }
                | PatientSearchHit::EarlierWork { standing, .. }
                | PatientSearchHit::PatientMemory { standing, .. } => standing,
            };
            assert!(
                !matches!(standing, HitStanding::Current),
                "`{withdrawn_term}` must never show as Current, got {standing:?}"
            );
        }
        assert!(page.results.iter().any(|hit| {
            let standing = match hit {
                PatientSearchHit::ChartRecord { standing, .. }
                | PatientSearchHit::EarlierWork { standing, .. }
                | PatientSearchHit::PatientMemory { standing, .. } => standing,
            };
            matches!(standing, HitStanding::Withdrawn)
        }));
    }

    let PatientSearchOutcome::Permitted(retracted_hist) = store
        .search_patient_authorized(
            &scope,
            &session,
            SearchAudienceScope::AgentJob,
            &PatientSearchRequest {
                query: "retractedwrongchartnote".into(),
                standing: SearchStandingScope::CurrentAndHistorical,
                ..PatientSearchRequest::default()
            },
            &mut auth,
        )
        .await
        .unwrap()
    else {
        panic!("expected Permitted");
    };
    assert!(
        retracted_hist.results.is_empty(),
        "retracted memory version must never appear even in historical search"
    );

    pg.stop_immediate().await;
}

#[tokio::test]
async fn ac4_cross_practice_unauthorized_and_revoked_access_yield_no_results_and_failures_are_distinct_from_empty()
 {
    let Some(mut pg) = DisposablePostgres::acquire("patient-search-ac4").await else {
        return;
    };
    let store = pg.store().await;
    let auth = pg.auth_store().await;

    let now_secs = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    // Set up two practices (`practice-alpha` and `practice-beta`) in real `PostgresAuthStore`:
    // - `practice-alpha`: `alice@alpha.test` (panel: `ws-alpha-panel` ONLY) and `bob@alpha.test` (panel: `ws-alpha-panel` + `ws-alpha-offpanel`)
    // - `practice-beta`: `carol@beta.test` (panel: `ws-beta-foreign`)
    auth.ensure_practice("practice-alpha", PracticeClock::RealTime)
        .await
        .unwrap();
    auth.ensure_practice("practice-beta", PracticeClock::RealTime)
        .await
        .unwrap();

    auth.register_patient("practice-alpha", "ws-alpha-panel")
        .await
        .unwrap();
    auth.register_patient("practice-alpha", "ws-alpha-offpanel")
        .await
        .unwrap();
    auth.register_patient("practice-alpha", "ws-alpha-corrupt")
        .await
        .unwrap();
    auth.register_patient("practice-alpha", "ws-alpha-corrupt-parts")
        .await
        .unwrap();
    auth.register_patient("practice-beta", "ws-beta-foreign")
        .await
        .unwrap();

    let alice_inv = auth
        .invite(
            "practice-alpha",
            "alice@alpha.test",
            "Dr. Alice",
            true,
            now_secs,
        )
        .await
        .unwrap();
    let (alice_id, alice_token) = auth
        .redeem_invitation("alice@alpha.test", &alice_inv, now_secs)
        .await
        .unwrap();
    let alice_hash: [u8; 32] = PostgresAuthStore::session_reference(&alice_token)
        .try_into()
        .unwrap();

    let bob_inv = auth
        .invite(
            "practice-alpha",
            "bob@alpha.test",
            "Dr. Bob",
            false,
            now_secs,
        )
        .await
        .unwrap();
    let (bob_id, bob_token) = auth
        .redeem_invitation("bob@alpha.test", &bob_inv, now_secs)
        .await
        .unwrap();
    let bob_hash: [u8; 32] = PostgresAuthStore::session_reference(&bob_token)
        .try_into()
        .unwrap();

    let carol_inv = auth
        .invite(
            "practice-beta",
            "carol@beta.test",
            "Dr. Carol",
            true,
            now_secs,
        )
        .await
        .unwrap();
    let (carol_id, carol_token) = auth
        .redeem_invitation("carol@beta.test", &carol_inv, now_secs)
        .await
        .unwrap();
    let carol_hash: [u8; 32] = PostgresAuthStore::session_reference(&carol_token)
        .try_into()
        .unwrap();

    auth.set_membership(&alice_id, "ws-alpha-panel", Membership::Allowed)
        .await
        .unwrap();
    auth.set_membership(&alice_id, "ws-alpha-corrupt", Membership::Allowed)
        .await
        .unwrap();
    auth.set_membership(&alice_id, "ws-alpha-corrupt-parts", Membership::Allowed)
        .await
        .unwrap();
    auth.set_membership(&bob_id, "ws-alpha-panel", Membership::Allowed)
        .await
        .unwrap();
    auth.set_membership(&bob_id, "ws-alpha-offpanel", Membership::Allowed)
        .await
        .unwrap();
    auth.set_membership(&carol_id, "ws-beta-foreign", Membership::Allowed)
        .await
        .unwrap();

    // Seed overlapping `"sharedsentinel"` across chart, conversations, drafts, and memory
    // in ALL THREE workspaces (including Dr. Bob's personal chat/draft on shared `ws-alpha-panel`!).
    for (practice, physician, hash, ws, label) in [
        (
            "practice-alpha",
            alice_id.physician_id.as_str(),
            alice_hash,
            "ws-alpha-panel",
            "alice-panel",
        ),
        (
            "practice-alpha",
            bob_id.physician_id.as_str(),
            bob_hash,
            "ws-alpha-panel",
            "bob-shared-panel",
        ),
        (
            "practice-alpha",
            bob_id.physician_id.as_str(),
            bob_hash,
            "ws-alpha-offpanel",
            "alpha-offpanel",
        ),
        (
            "practice-beta",
            carol_id.physician_id.as_str(),
            carol_hash,
            "ws-beta-foreign",
            "beta-foreign",
        ),
    ] {
        store
            .record_source(&make_source_revision(
                ws,
                "epic-fhir",
                &format!("cand-{label}"),
                &format!("rev-{label}"),
                "Observation",
                vec![("effectiveDateTime", "2026-10-12")],
                json!({"resourceType": "Observation", "code": "2160-0", "note": [{"text": "sharedsentinel chart"}]}),
                vec![],
            ))
            .await
            .unwrap();

        let chat_scope = Scope {
            practice: practice.into(),
            physician: physician.into(),
            workspace: ws.into(),
            chat: format!("chat-{label}"),
        };
        store
            .accept_authorized(
                &chat_scope,
                &Input {
                    key: format!("msg-{label}"),
                    text: format!("sharedsentinel conversation for {label}"),
                    timezone: "UTC".into(),
                    scenario: None,
                },
                hash.to_vec(),
                skills(),
                &chat_recipe(),
            )
            .await
            .unwrap();

        let reply_scope = Scope {
            chat: format!("reply-{label}"),
            ..chat_scope.clone()
        };
        store
            .open_draft_authorized(
                &reply_scope,
                &format!("sharedsentinel draft for {label}"),
                &hash,
                &mut SqlRuntimeAuthority,
            )
            .await
            .unwrap()
            .unwrap();

        store
            .mutate_patient_memory_authorized(
                &chat_scope,
                &format!("mem-{label}"),
                &hash,
                MemoryMutation::RecordCorrection(RecordCorrection {
                    correction_id: Some(CorrectionId::parse(format!("corr-{label}")).unwrap()),
                    recorded_on: ClinicalDate::parse("2026-10-12").unwrap(),
                    source_message: SourceMessageRef::new(
                        &chat_scope.chat,
                        format!("msg-{label}"),
                        physician,
                    )
                    .unwrap(),
                    statement: format!("sharedsentinel memory for {label}"),
                    supersedes: None,
                    understanding_patch: None,
                }),
                &mut SqlRuntimeAuthority,
            )
            .await
            .unwrap();
    }

    let query_shared = PatientSearchRequest {
        query: "sharedsentinel".into(),
        ..PatientSearchRequest::default()
    };

    // 1. Alice searching her authorized `ws-alpha-panel` gets ONLY `ws-alpha-panel` chart + memory
    //    + Alice's personal chat/draft — NEVER Bob's personal chat/draft, NEVER `ws-alpha-offpanel`, NEVER `ws-beta-foreign`!
    let alice_panel_scope = Scope {
        practice: "practice-alpha".into(),
        physician: alice_id.physician_id.clone(),
        workspace: "ws-alpha-panel".into(),
        chat: "chat-alice-search".into(),
    };
    let PatientSearchOutcome::Permitted(alice_page) = store
        .search_patient_authorized(
            &alice_panel_scope,
            &alice_hash,
            SearchAudienceScope::PhysicianFacing,
            &query_shared,
            &mut SqlRuntimeAuthority,
        )
        .await
        .unwrap()
    else {
        panic!("Alice authorized on ws-alpha-panel");
    };
    let alice_json = serde_json::to_string(&alice_page).unwrap();
    assert!(alice_json.contains("cand-alice-panel"));
    assert!(alice_json.contains("chat-alice-panel"));
    assert!(alice_json.contains("reply-alice-panel"));
    let alice_work_json: String = alice_page
        .results
        .iter()
        .filter(|hit| matches!(hit, PatientSearchHit::EarlierWork { .. }))
        .map(|hit| serde_json::to_string(hit).unwrap())
        .collect::<Vec<_>>()
        .join("\n");
    assert!(
        !alice_work_json.contains("chat-bob-shared-panel")
            && !alice_work_json.contains("reply-bob-shared-panel"),
        "Alice must never see Bob's personal conversations or reply drafts in earlier_work: {alice_work_json}"
    );
    assert!(
        !alice_json.contains("reply-bob-shared-panel"),
        "Bob's personal reply draft id must never appear anywhere in Alice's search results: {alice_json}"
    );
    assert!(
        !alice_json.contains("offpanel") && !alice_json.contains("beta-foreign"),
        "zero leakage from off-panel or foreign practice workspaces: {alice_json}"
    );

    // 2. Cross-practice workspace attempt (Alice attempting to query `ws-beta-foreign`, both under `practice-alpha` and `practice-beta`) -> Unauthorized!
    for attempted_practice in ["practice-alpha", "practice-beta"] {
        let foreign_attempt = Scope {
            practice: attempted_practice.into(),
            physician: alice_id.physician_id.clone(),
            workspace: "ws-beta-foreign".into(),
            chat: "chat-cross-tenant".into(),
        };
        let out = store
            .search_patient_authorized(
                &foreign_attempt,
                &alice_hash,
                SearchAudienceScope::PhysicianFacing,
                &query_shared,
                &mut SqlRuntimeAuthority,
            )
            .await
            .unwrap();
        assert!(
            matches!(out, PatientSearchOutcome::Unauthorized { .. }),
            "cross-practice query must be Unauthorized, got {out:?}"
        );
    }

    // 3. Same-practice off-panel patient (`ws-alpha-offpanel`) -> Unauthorized both via SQL session authority
    //    and via `PatientAccess::ClinicDemographic` / `PatientAccess::Denied`!
    let offpanel_attempt = Scope {
        practice: "practice-alpha".into(),
        physician: alice_id.physician_id.clone(),
        workspace: "ws-alpha-offpanel".into(),
        chat: "chat-offpanel".into(),
    };
    assert!(matches!(
        store
            .search_patient_authorized(
                &offpanel_attempt,
                &alice_hash,
                SearchAudienceScope::PhysicianFacing,
                &query_shared,
                &mut SqlRuntimeAuthority,
            )
            .await
            .unwrap(),
        PatientSearchOutcome::Unauthorized { .. }
    ));

    let offpanel_key = PatientKey::new("practice-alpha", "ws-alpha-offpanel").unwrap();
    for denied_standing in [PatientAccess::ClinicDemographic, PatientAccess::Denied] {
        assert!(matches!(
            store
                .search_patient_for_access(
                    &offpanel_key,
                    &SearchCaller::PhysicianFacing {
                        physician_id: alice_id.physician_id.clone(),
                    },
                    denied_standing,
                    &query_shared,
                )
                .await
                .unwrap(),
            PatientSearchOutcome::Unauthorized { .. }
        ));
    }

    // 4. Live access revocation:
    //    - Accept a chat on `ws-alpha-panel` for Alice, claim it, then remove `ws-alpha-panel` from Alice's panel!
    //    - Both direct `search_patient_authorized` AND `PhysicianHost::run` stop immediately with zero results leaked!
    store
        .accept_authorized(
            &alice_panel_scope,
            &Input {
                key: "msg-before-revoke".into(),
                text: "Search sharedsentinel".into(),
                timezone: "UTC".into(),
                scenario: None,
            },
            alice_hash.to_vec(),
            skills(),
            &chat_recipe(),
        )
        .await
        .unwrap();
    let live_claim = store.claim(&alice_panel_scope, 60).await.unwrap().unwrap();

    auth.set_membership(&alice_id, "ws-alpha-panel", Membership::Denied)
        .await
        .unwrap();

    assert!(matches!(
        store
            .search_patient_authorized(
                &alice_panel_scope,
                &alice_hash,
                SearchAudienceScope::PhysicianFacing,
                &query_shared,
                &mut SqlRuntimeAuthority,
            )
            .await
            .unwrap(),
        PatientSearchOutcome::Unauthorized { .. }
    ));

    let mut revoked_provider = ScriptedProvider::new(vec![call_turn(vec![(
        "call-revoked-search",
        SEARCH_TOOL,
        json!({"query": "sharedsentinel"}),
    )])]);
    let host_err = PhysicianHost {
        store: store.clone(),
        claim: live_claim,
        authority: SqlRuntimeAuthority,
    }
    .run(
        &mut revoked_provider,
        engine::RunLimits::default(),
        &engine::Cancellation::default(),
    )
    .await;
    assert!(
        host_err.is_err(),
        "PhysicianHost must halt immediately when panel membership is revoked"
    );
    assert!(
        revoked_provider.seen_requests.is_empty(),
        "provider must not even be called after panel revocation"
    );

    // Restore panel membership, verify search works again, then revoke Alice's session (`end_session`) -> Unauthorized!
    auth.set_membership(&alice_id, "ws-alpha-panel", Membership::Allowed)
        .await
        .unwrap();
    assert!(matches!(
        store
            .search_patient_authorized(
                &alice_panel_scope,
                &alice_hash,
                SearchAudienceScope::PhysicianFacing,
                &query_shared,
                &mut SqlRuntimeAuthority,
            )
            .await
            .unwrap(),
        PatientSearchOutcome::Permitted(_)
    ));

    auth.end_session(&alice_token).await.unwrap();
    assert!(matches!(
        store
            .search_patient_authorized(
                &alice_panel_scope,
                &alice_hash,
                SearchAudienceScope::PhysicianFacing,
                &query_shared,
                &mut SqlRuntimeAuthority,
            )
            .await
            .unwrap(),
        PatientSearchOutcome::Unauthorized { .. }
    ));

    // Mint a fresh session for Alice to test Empty Result vs Operational/Integrity Failure!
    let alice_reinvite_tok = auth
        .begin_sign_in("alice@alpha.test", now_secs + 1)
        .await
        .unwrap();
    let (_, alice_token_2) = auth
        .complete_sign_in("alice@alpha.test", &alice_reinvite_tok, now_secs + 2)
        .await
        .unwrap();
    let alice_hash_2: [u8; 32] = PostgresAuthStore::session_reference(&alice_token_2)
        .try_into()
        .unwrap();

    // 5a. Legitimate empty result (`Ok(Permitted(page))` with `results: []`).
    let empty_outcome = store
        .search_patient_authorized(
            &alice_panel_scope,
            &alice_hash_2,
            SearchAudienceScope::PhysicianFacing,
            &PatientSearchRequest {
                query: "unmatched_term_99999".into(),
                ..PatientSearchRequest::default()
            },
            &mut SqlRuntimeAuthority,
        )
        .await
        .expect("empty search is Ok, not Err");
    let PatientSearchOutcome::Permitted(empty_page) = empty_outcome else {
        panic!("expected Permitted empty page");
    };
    assert!(empty_page.results.is_empty());
    assert!(empty_page.next.is_none());

    // 5b. Corrupt stored `SourceRevision` SHA-256 digest -> `Err(PatientSearchError::CorruptSourceIntegrity)`, NEVER `Ok(empty)`!
    store
        .record_source(&make_source_revision(
            "ws-alpha-corrupt",
            "epic-fhir",
            "cand-tampered",
            "rev-tampered",
            "Observation",
            vec![("effectiveDateTime", "2026-10-12")],
            json!({"resourceType": "Observation", "code": "2160-0"}),
            vec![],
        ))
        .await
        .unwrap();

    let tamper_conn = Database::connect(pg.url()).await.unwrap();
    let bad_json = json!({
        "workspace": "ws-alpha-corrupt",
        "source_id": "epic-fhir",
        "candidate_id": "cand-tampered",
        "revision": "rev-tampered",
        "record_type": "Observation",
        "dates": [],
        "content": "{\"tampered\":true}",
        "sha256": "0000000000000000000000000000000000000000000000000000000000000000",
        "readable_parts": [],
        "provenance": {"source_id": "epic-fhir"}
    });
    let update_stmt = Query::update()
        .table(Alias::new("physician_runtime_sources"))
        .value(Alias::new("revision"), Expr::val(bad_json))
        .and_where(Expr::col(Alias::new("workspace")).eq("ws-alpha-corrupt"))
        .to_owned();
    tamper_conn.execute(&update_stmt).await.unwrap();
    drop(tamper_conn);

    let corrupt_scope = Scope {
        workspace: "ws-alpha-corrupt".into(),
        ..alice_panel_scope.clone()
    };
    let corrupt_res = store
        .search_patient_authorized(
            &corrupt_scope,
            &alice_hash_2,
            SearchAudienceScope::PhysicianFacing,
            &PatientSearchRequest {
                query: "unmatched_term_99999".into(),
                ..PatientSearchRequest::default()
            },
            &mut SqlRuntimeAuthority,
        )
        .await;
    assert!(
        matches!(
            corrupt_res,
            Err(PatientSearchError::CorruptSourceIntegrity { .. })
        ),
        "tampered stored record must fail with CorruptSourceIntegrity, never read as empty: {corrupt_res:?}"
    );

    // 5b-ii. Valid SHA-256 `SourceRevision` with malformed `provenance.scans` or `provenance.attachments`
    //        -> `Err(PatientSearchError::PartResolution(_))` preserving `#[source]`, NEVER `Ok(empty)`!
    let mut broken_scans_rev = make_source_revision(
        "ws-alpha-corrupt-parts",
        "epic-fhir",
        "cand-broken-scans",
        "rev-broken-scans-1",
        "Media",
        vec![("createdDateTime", "2026-10-12")],
        json!({"resourceType": "Media", "status": "completed"}),
        vec![],
    );
    broken_scans_rev.provenance = json!({
        "source_id": "epic-fhir",
        "scans": "not-a-scan-array"
    });
    store.record_source(&broken_scans_rev).await.unwrap();

    let corrupt_parts_scope = Scope {
        workspace: "ws-alpha-corrupt-parts".into(),
        ..alice_panel_scope.clone()
    };
    let broken_scans_res = store
        .search_patient_authorized(
            &corrupt_parts_scope,
            &alice_hash_2,
            SearchAudienceScope::PhysicianFacing,
            &PatientSearchRequest {
                query: "unmatched_term_99999".into(),
                ..PatientSearchRequest::default()
            },
            &mut SqlRuntimeAuthority,
        )
        .await;
    match broken_scans_res {
        Err(err @ PatientSearchError::PartResolution(_)) => {
            assert!(
                err.source().is_some(),
                "PartResolution must preserve RecordReadError as #[source]"
            );
        }
        other => panic!(
            "malformed provenance.scans must fail with PartResolution, never read as empty: {other:?}"
        ),
    }

    let mut broken_att_rev = make_source_revision(
        "ws-alpha-corrupt-parts",
        "epic-fhir",
        "cand-broken-scans",
        "rev-broken-att-2",
        "DocumentReference",
        vec![("date", "2026-10-12")],
        json!({"resourceType": "DocumentReference", "status": "current"}),
        vec![],
    );
    broken_att_rev.provenance = json!({
        "source_id": "epic-fhir",
        "attachments": [{"invalid_attachment_shape": true}]
    });
    store.record_source(&broken_att_rev).await.unwrap();

    let broken_att_res = store
        .search_patient_authorized(
            &corrupt_parts_scope,
            &alice_hash_2,
            SearchAudienceScope::PhysicianFacing,
            &PatientSearchRequest {
                query: "unmatched_term_99999".into(),
                ..PatientSearchRequest::default()
            },
            &mut SqlRuntimeAuthority,
        )
        .await;
    assert!(
        matches!(broken_att_res, Err(PatientSearchError::PartResolution(_))),
        "malformed provenance.attachments must fail with PartResolution, never read as empty: {broken_att_res:?}"
    );

    // 5c. Stop Postgres -> operational DB failure returns `Err(PatientSearchError::Database(_))`, NEVER `Ok(empty)`!
    pg.stop_immediate().await;
    let db_down_res = store
        .search_patient_authorized(
            &alice_panel_scope,
            &alice_hash_2,
            SearchAudienceScope::PhysicianFacing,
            &PatientSearchRequest {
                query: "unmatched_term_99999".into(),
                ..PatientSearchRequest::default()
            },
            &mut SqlRuntimeAuthority,
        )
        .await;
    assert!(
        matches!(db_down_res, Err(PatientSearchError::Database(_))),
        "database outage must fail with PatientSearchError::Database, never read as empty: {db_down_res:?}"
    );
}

fn json_value_type(value: &Value) -> &'static str {
    match value {
        Value::Null => "null",
        Value::Bool(_) => "boolean",
        Value::Number(num) => {
            if num.is_i64() || num.is_u64() {
                "integer"
            } else {
                "number"
            }
        }
        Value::String(_) => "string",
        Value::Array(_) => "array",
        Value::Object(_) => "object",
    }
}

fn matches_json_schema(schema: &Value, value: &Value) -> bool {
    if let Some(any_of) = schema.get("anyOf").and_then(Value::as_array) {
        return any_of
            .iter()
            .any(|branch| matches_json_schema(branch, value));
    }
    if let Some(expected_type) = schema.get("type") {
        let actual = json_value_type(value);
        let type_ok = match expected_type {
            Value::String(kind) => kind == actual || (kind == "number" && actual == "integer"),
            Value::Array(kinds) => kinds.iter().any(|k| {
                k.as_str()
                    .is_some_and(|kind| kind == actual || (kind == "number" && actual == "integer"))
            }),
            _ => false,
        };
        if !type_ok {
            return false;
        }
    }
    if let Some(allowed) = schema.get("enum").and_then(Value::as_array)
        && !allowed.contains(value)
    {
        return false;
    }
    match value {
        Value::String(text) => {
            if let Some(min_len) = schema.get("minLength").and_then(Value::as_u64)
                && (text.chars().count() as u64) < min_len
            {
                return false;
            }
            if let Some(max_len) = schema.get("maxLength").and_then(Value::as_u64)
                && (text.len() as u64) > max_len
            {
                return false;
            }
        }
        Value::Object(map) => {
            if let Some(required) = schema.get("required").and_then(Value::as_array) {
                for req in required {
                    let Some(key) = req.as_str() else {
                        return false;
                    };
                    if !map.contains_key(key) {
                        return false;
                    }
                }
            }
            let props = schema.get("properties").and_then(Value::as_object);
            if schema.get("additionalProperties") == Some(&Value::Bool(false))
                && map
                    .keys()
                    .any(|key| props.is_none_or(|defined| !defined.contains_key(key)))
            {
                return false;
            }
            if let Some(defined) = props {
                for (key, field_val) in map {
                    if let Some(field_schema) = defined.get(key)
                        && !matches_json_schema(field_schema, field_val)
                    {
                        return false;
                    }
                }
            }
        }
        Value::Array(_) | Value::Number(_) | Value::Null | Value::Bool(_) => {}
    }
    true
}

#[test]
fn search_tool_schema_accepts_valid_queries_and_rejects_invalid_shapes() {
    let tool = search_tool();
    assert_eq!(tool.name, SEARCH_TOOL);

    for valid in [
        json!({"query": "2160-0", "record_type": "Observation", "after": null}),
        json!({"query": ""}),
        json!({
            "query": "metformin",
            "record_type": null,
            "bucket": "medications",
            "domain": "all",
            "standing": "current_and_historical",
            "after": {
                "snapshot_token": "abc123",
                "sort_key": "0\u{001f}med-1"
            }
        }),
    ] {
        assert!(
            matches_json_schema(&tool.parameters, &valid),
            "expected valid schema match for {valid}"
        );
    }

    for invalid in [
        json!({}),
        json!({"query": "a", "unknown_field": true}),
        json!({"query": "a", "record_type": ""}),
        json!({"query": "a", "bucket": "not_a_bucket"}),
        json!({"query": "a", "domain": "wrong_domain"}),
        json!({"query": "a", "standing": "wrong_standing"}),
        json!({"query": "a", "after": {"snapshot_token": ""}}),
        json!({"query": "a", "after": {"snapshot_token": "tok", "sort_key": "k", "extra": 1}}),
    ] {
        assert!(
            !matches_json_schema(&tool.parameters, &invalid),
            "expected schema rejection for {invalid}"
        );
    }
}
