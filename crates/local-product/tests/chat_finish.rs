//! A chat answer ends through `finish_answer`, against real Postgres. Run
//! explicitly with PHYSICIAN_RUNTIME_TEST_DATABASE_URL.
//!
//! The scripted provider answers each request with the next scripted turn and
//! keeps every request, so a test can read what the model was told.
use base64::Engine as _;
use engine::recipe::{Observer, Request};
use local_product::physician_sources::adapter::{
    AdapterRow, AdapterTable, AttachmentRule, CategoryMatch, UnmappedRecordRule, ValueExtractor,
    file_fhir_record, file_json_record,
};
use local_product::physician_sources::{
    Bucket, DateRole, READ_TOOL, SourcePatient, SourceRevision,
};
use local_product::runtime::delivery::Backoff;
use local_product::runtime::sources::SourcePull;
use local_product::runtime::{
    Accepted, Authority, Input, PhysicianHost, RuntimeError, Scope, Store, WorkStatus, async_trait,
    engine,
};
use serde_json::{Value, json};

const FINISH: &str = "finish_answer";

/// A potassium result.
const LAB: &str = r#"{"resourceType":"Observation","id":"k1","status":"final","code":{"text":"Potassium"},"valueQuantity":{"value":4.1,"unit":"mmol/L"},"effectiveDateTime":"2026-08-01"}"#;
const NOTE_TEXT: &str =
    "Seen in clinic on 2026-08-02. Potassium was normal. Plan: recheck in one month.";

fn note_content() -> String {
    let data = base64::engine::general_purpose::STANDARD.encode(NOTE_TEXT);
    format!(
        r#"{{"resourceType":"DocumentReference","id":"n1","status":"current","date":"2026-08-02","content":[{{"attachment":{{"contentType":"text/plain","data":"{data}"}}}}]}}"#
    )
}

/// The source that delivers the chart's FHIR records.
const EHR: &str = "ehr";

/// One FHIR record filed through the FHIR table, as a pull files it.
fn filed(workspace: &str, content: &str) -> SourceRevision {
    file_fhir_record(workspace, EHR, content, None).unwrap()
}

/// The version a saved citation names for `record`.
fn version(record: &SourceRevision) -> Value {
    json!({"candidate_id": record.candidate_id, "revision": record.revision})
}

/// One conversation with a lab and a note in its own workspace.
#[derive(Clone)]
struct Chat {
    store: Store,
    scope: Scope,
    lab: SourceRevision,
    note: SourceRevision,
}

impl Chat {
    async fn open() -> Self {
        let url = std::env::var("PHYSICIAN_RUNTIME_TEST_DATABASE_URL")
            .expect("explicit local Postgres URL required");
        let store = Store::connect(&url).await.unwrap();
        store.migrate().await.unwrap();
        let scope = Scope {
            practice: uuid::Uuid::new_v4().to_string(),
            physician: "physician-a".into(),
            workspace: uuid::Uuid::new_v4().to_string(),
            chat: "chat-a".into(),
        };
        let lab = filed(&scope.workspace, LAB);
        let note = filed(&scope.workspace, &note_content());
        let chat = Self {
            store,
            scope,
            lab,
            note,
        };
        chat.pull(&[]).await;
        chat.ask("ask-1", "What is the latest potassium?").await;
        chat
    }

    /// Files `content` and delivers it with the lab and the note in the
    /// source's next complete pull.
    async fn add(&self, content: &str) -> SourceRevision {
        let record = filed(&self.scope.workspace, content);
        self.pull(std::slice::from_ref(&record)).await;
        record
    }

    /// One complete pull of the chart: the lab, the note and `more`.
    async fn pull(&self, more: &[SourceRevision]) {
        let records: Vec<SourceRevision> = [self.lab.clone(), self.note.clone()]
            .into_iter()
            .chain(more.iter().cloned())
            .collect();
        let receipt = self
            .store
            .record_chart_with_receipt(
                &self.scope.workspace,
                &records,
                &SourcePull {
                    source_id: EHR.into(),
                    source_label: "EHR".into(),
                    patient: SourcePatient {
                        source_patient_id: "P-1".into(),
                        display_name: "Nadia Okafor".into(),
                        birth_date: Some("1984-02-19".into()),
                        sex_or_gender: None,
                        medical_record_numbers: Vec::new(),
                    },
                    pulled_at: Some(1_760_000_000),
                },
            )
            .await
            .unwrap();
        assert!(receipt.missing_candidates.is_empty(), "{receipt:?}");
    }

    async fn ask(&self, key: &str, text: &str) -> Accepted {
        accept(&self.store, &self.scope, key, text).await.unwrap()
    }

    /// Runs one attempt to its end. A failed attempt is settled, as hosting does.
    async fn attempt<A: Authority, P: engine::recipe::Provider>(
        &self,
        authority: A,
        provider: &mut P,
        max_calls: u64,
    ) -> Result<(), engine::Failure> {
        let claim = self.store.claim(&self.scope, 60).await.unwrap().unwrap();
        let cancel = engine::Cancellation::default();
        let result = PhysicianHost {
            store: self.store.clone(),
            claim: claim.clone(),
            authority,
            shown_steps: local_product::runtime::ShownSteps::new(()),
        }
        .run(provider, limits(max_calls), &cancel)
        .await;
        if let Err(error) = &result {
            self.store
                .settle_chat_attempt(
                    &claim,
                    engine::AttemptFailure::of(error, &cancel),
                    Backoff::IMMEDIATE,
                )
                .await
                .unwrap();
        }
        result
    }

    async fn messages(&self) -> Value {
        let state = self.store.conversation(&self.scope).await.unwrap();
        serde_json::to_value(&state.messages).unwrap()
    }

    async fn status(&self) -> WorkStatus {
        self.store.conversation(&self.scope).await.unwrap().status
    }

    /// Every chart read result saved in this conversation, in order.
    async fn reads(&self) -> Vec<Value> {
        let state = self.store.conversation(&self.scope).await.unwrap();
        let mut started = std::collections::HashSet::new();
        let mut reads = Vec::new();
        for event in state.attempts.iter().flat_map(|attempt| &attempt.events) {
            match &event.kind {
                engine::EventKind::ToolStarted(call) if call.name == READ_TOOL => {
                    started.insert(call.id.clone());
                }
                engine::EventKind::ToolReturned { call_id, result }
                    if started.contains(call_id) =>
                {
                    if let Some(output) = result.original() {
                        reads.push(serde_json::from_str(output).unwrap());
                    }
                }
                _ => {}
            }
        }
        reads
    }
}

fn limits(max_calls: u64) -> engine::RunLimits {
    engine::RunLimits {
        max_calls,
        max_response_bytes: 131_072,
        ..engine::RunLimits::default()
    }
}

struct Allowed;
#[async_trait]
impl Authority for Allowed {
    async fn lock_publication(
        &mut self,
        _: &local_product::runtime::PublicationTransaction,
        _: &Scope,
        _: Option<&[u8]>,
    ) -> Result<(), String> {
        Ok(())
    }
    async fn check(
        &mut self,
        _: &local_product::runtime::PublicationTransaction,
        _: &Scope,
        _: Option<&[u8]>,
    ) -> Result<(), String> {
        Ok(())
    }
}

/// The publication lock fails at the finish with this number, counted from
/// 1, as a lost worker does. Earlier finishes take it.
struct InterruptedAtFinish(u32);
#[async_trait]
impl Authority for InterruptedAtFinish {
    async fn check(
        &mut self,
        _: &local_product::runtime::PublicationTransaction,
        _: &Scope,
        _: Option<&[u8]>,
    ) -> Result<(), String> {
        Ok(())
    }
    async fn lock_publication(
        &mut self,
        _: &local_product::runtime::PublicationTransaction,
        _: &Scope,
        _: Option<&[u8]>,
    ) -> Result<(), String> {
        self.0 -= 1;
        if self.0 == 0 {
            Err("synthetic publication interrupted".into())
        } else {
            Ok(())
        }
    }
}

async fn accept(
    store: &Store,
    scope: &Scope,
    key: &str,
    text: &str,
) -> Result<Accepted, RuntimeError> {
    let binding = local_product::ModelBinding {
        model: "controlled".into(),
        effort: "none".into(),
        adapter: "fixed".into(),
        context_window_tokens: 272_000,
        max_input_tokens: 272_000,
    };
    let skills = local_product::physician_chat::PhysicianSkills::configured(None, false).unwrap();
    let recipe = local_product::physician_chat::recipe(
        &binding,
        local_product::physician_chat::INSTRUCTIONS,
        &skills,
    );
    let input = Input {
        key: key.into(),
        text: text.into(),
        timezone: "UTC".into(),
        scenario: None,
    };
    store
        .accept_authorized(scope, &input, vec![0; 32], &skills, &recipe, None)
        .await
}

fn turn(items: Vec<engine::Item>) -> engine::Turn {
    engine::Turn {
        items,
        receipt: json!({"controlled": true}),
        usage: Default::default(),
    }
}

fn text(text: &str) -> engine::Item {
    engine::Item::assistant(text)
}

fn call(id: &str, name: &str, arguments: Value) -> engine::Item {
    engine::Item::Call(engine::Invocation {
        id: id.into(),
        name: name.into(),
        arguments: arguments.to_string(),
    })
}

fn read(id: &str, record: &SourceRevision) -> engine::Item {
    read_with(id, record, 65_536)
}

fn read_with(id: &str, record: &SourceRevision, max_result_bytes: usize) -> engine::Item {
    call(
        id,
        READ_TOOL,
        json!({"references":[{"candidate_id":record.candidate_id,"revision":record.revision,"cursor":null}],"max_result_bytes":max_result_bytes}),
    )
}

/// A `finish_answer` call: the answer, its quotes as label and words, the
/// Basis and `cites_none`, with no description.
fn finish(
    id: &str,
    answer: &str,
    quotes: &[(&str, &str)],
    basis: &str,
    cites_none: bool,
) -> engine::Item {
    described_finish(id, answer, quotes, &[], basis, cites_none)
}

/// A `finish_answer` call that also hands in descriptions as label and words.
fn described_finish(
    id: &str,
    answer: &str,
    quotes: &[(&str, &str)],
    descriptions: &[(&str, &str)],
    basis: &str,
    cites_none: bool,
) -> engine::Item {
    call(
        id,
        FINISH,
        json!({
            "answer": answer,
            "quotes": quotes
                .iter()
                .map(|(label, quote)| json!({"label": label, "quote": quote}))
                .collect::<Vec<_>>(),
            "descriptions": descriptions
                .iter()
                .map(|(label, description)| json!({"label": label, "description": description}))
                .collect::<Vec<_>>(),
            "basis": basis,
            "cites_none": cites_none
        }),
    )
}

/// A finish that cites the lab's value by a quote of its content, with a
/// Basis that names the lab and the note.
fn good_finish(id: &str) -> engine::Item {
    finish(
        id,
        "Potassium was 4.1 mmol/L on 2026-08-01 [S1].",
        &[("S1", "\"value\":4.1")],
        "The potassium result [S1] and the clinic note [S2].",
        false,
    )
}

/// Answers each request with the next scripted turn and keeps the requests.
struct Script {
    turns: std::collections::VecDeque<engine::Turn>,
    requests: Vec<Request>,
}

impl Script {
    fn new(turns: impl IntoIterator<Item = Vec<engine::Item>>) -> Self {
        Self {
            turns: turns.into_iter().map(turn).collect(),
            requests: Vec::new(),
        }
    }

    /// What the model was told for `call_id`, from the first request that
    /// answered it.
    fn told(&self, call_id: &str) -> String {
        self.requests
            .iter()
            .flat_map(|request| &request.items)
            .find_map(|item| match item {
                engine::Item::MaterializedResult {
                    call_id: id,
                    output,
                } if id == call_id => Some(output.clone()),
                _ => None,
            })
            .unwrap_or_else(|| panic!("no result for {call_id}"))
    }
}

impl engine::recipe::Provider for Script {
    async fn exchange(
        &mut self,
        request: &Request,
        _: &engine::Cancellation,
        _: &engine::RunLimits,
        _: &mut dyn Observer,
    ) -> Result<engine::Turn, engine::Cause> {
        self.requests.push(request.clone());
        Ok(self
            .turns
            .pop_front()
            .unwrap_or_else(|| turn(vec![text("The script has no more turns.")])))
    }
}

/// Never answers: the retained exchange must settle the work.
struct MustNotDispatch;
impl engine::recipe::Provider for MustNotDispatch {
    async fn exchange(
        &mut self,
        _: &Request,
        _: &engine::Cancellation,
        _: &engine::RunLimits,
        _: &mut dyn Observer,
    ) -> Result<engine::Turn, engine::Cause> {
        panic!("the retained finish settles the work without a new request")
    }
}

fn assistant(messages: &Value) -> Vec<&Value> {
    messages
        .as_array()
        .unwrap()
        .iter()
        .filter(|message| message["role"] == "assistant")
        .collect()
}

/// H1. Each read record carries its label. Reads
/// that run together get distinct labels in the order they were asked, the
/// same version read again keeps its label, and the next work starts again
/// at S1.
#[tokio::test]
#[ignore = "requires explicit local Postgres"]
async fn read_results_carry_labels() {
    let chat = Chat::open().await;
    let mut script = Script::new([
        vec![read("lab", &chat.lab), read("note", &chat.note)],
        vec![read("lab-again", &chat.lab)],
        vec![good_finish("finish")],
    ]);
    chat.attempt(Allowed, &mut script, 10).await.unwrap();
    let reads = chat.reads().await;
    assert_eq!(reads.len(), 3, "{reads:#?}");
    // The two reads of the first turn run together and return in either
    // order; their labels follow the order the model asked for them.
    let first_of = |reference: &str| {
        reads
            .iter()
            .map(|read| &read["results"][0])
            .filter(|result| result["reference"] == reference)
            .collect::<Vec<_>>()
    };
    let labs = first_of(&chat.lab.candidate_id);
    let lab = labs[0];
    assert_eq!(lab["cite_as"], "S1", "{lab:#}");
    assert_eq!(labs[1]["cite_as"], "S1");
    let note = first_of(&chat.note.candidate_id)[0];
    assert_eq!(note["cite_as"], "S2");
    assert!(lab.get("fields").is_none(), "{lab:#}");
    assert_eq!(note["readable_parts"][0]["text"], NOTE_TEXT);

    // The next work restarts at S1, here for the note.
    chat.ask("ask-2", "What did the note say?").await;
    let mut script = Script::new([
        vec![read("next-note", &chat.note)],
        vec![finish(
            "next-finish",
            "The note says potassium was normal [S1].",
            &[("S1", "Potassium was normal.")],
            "The clinic note.",
            false,
        )],
    ]);
    chat.attempt(Allowed, &mut script, 10).await.unwrap();
    let reads = chat.reads().await;
    assert_eq!(reads[3]["results"][0]["cite_as"], "S1");
    assert_eq!(chat.status().await, WorkStatus::Completed);
}

/// H1. A record too large for one result arrives as passages. Each passage
/// keeps the record's label.
#[tokio::test]
#[ignore = "requires explicit local Postgres"]
async fn a_passage_keeps_its_record_label() {
    let chat = Chat::open().await;
    let long = format!(
        r#"{{"resourceType":"Observation","id":"big","status":"final","note":[{{"text":"{}"}}],"valueQuantity":{{"value":5.2}}}}"#,
        "a".repeat(6000)
    );
    let big = chat.add(&long).await;
    let mut script = Script::new([
        vec![read_with("big", &big, 4096)],
        vec![finish(
            "finish",
            "Nothing to report.",
            &[],
            "No record.",
            true,
        )],
    ]);
    chat.attempt(Allowed, &mut script, 10).await.unwrap();
    let passage = &chat.reads().await[0]["results"][0];
    assert_eq!(passage["outcome"], "passage", "{passage:#}");
    assert_eq!(passage["cite_as"], "S1");
}

/// H1. The size check counts the label. A record that fits its share of the
/// read only without its label does not arrive whole with a label past
/// `max_result_bytes`: the result the model gets stays inside it and keeps
/// the label.
#[tokio::test]
#[ignore = "requires explicit local Postgres"]
async fn a_labelled_read_stays_inside_max_result_bytes() {
    let chat = Chat::open().await;
    let workspace = chat.scope.workspace.clone();
    // The record whose unlabelled result is 4,090 bytes: whole without its
    // label, past 4,096 with it.
    let sized = |padding: usize| {
        let content = format!(
            r#"{{"resourceType":"Observation","id":"edge","note":[{{"text":"{}"}}]}}"#,
            "a".repeat(padding)
        );
        filed(&workspace, &content)
    };
    let unlabelled = |record: &SourceRevision| {
        local_product::physician_sources::exact_read(
            std::slice::from_ref(record),
            &workspace,
            std::slice::from_ref(&record.candidate_id),
            4096,
        )
        .unwrap()
        .text()
        .unwrap()
        .len()
    };
    let base = unlabelled(&sized(0));
    let record = sized(4090 - base);
    assert_eq!(unlabelled(&record), 4090);
    chat.pull(std::slice::from_ref(&record)).await;
    let mut script = Script::new([
        vec![read_with("edge", &record, 4096)],
        vec![finish(
            "finish",
            "Nothing to report.",
            &[],
            "No record.",
            true,
        )],
    ]);
    chat.attempt(Allowed, &mut script, 10).await.unwrap();
    let read = &chat.reads().await[0];
    let bytes = read.to_string().len();
    assert!(bytes <= 4096, "{bytes} bytes: {read:#}");
    assert_eq!(read["results"][0]["cite_as"], "S1", "{read:#}");
}

/// H2. A complete finish is saved: the text has no labels, each mark names
/// the version and its resolved place, the Basis is saved with its own
/// marks, which have no place, and the transcript shows the text. It reads
/// the same after a restart.
#[tokio::test]
#[ignore = "requires explicit local Postgres"]
async fn a_complete_finish_saves_marks_places_and_basis() {
    let chat = Chat::open().await;
    let mut script = Script::new([
        vec![read("lab", &chat.lab), read("note", &chat.note)],
        vec![text("Writing the answer."), good_finish("finish")],
    ]);
    chat.attempt(Allowed, &mut script, 10).await.unwrap();
    assert_eq!(chat.status().await, WorkStatus::Completed);
    let messages = chat.messages().await;
    let answers = assistant(&messages);
    assert_eq!(answers.len(), 1);
    let answer = answers[0];
    assert_eq!(
        answer["text"], "Potassium was 4.1 mmol/L on 2026-08-01.",
        "{answer:#}"
    );
    let lab = version(&chat.lab);
    let note = version(&chat.note);
    let in_lab = |words: &str| {
        let start = LAB.find(words).unwrap();
        json!({"passage":{"spans":[{"part":0,"start":start,"end":start + words.len()}]}})
    };
    assert_eq!(
        answer["kind"]["finished"]["marks"],
        json!([{"at":38,"source":lab,"place":in_lab("\"value\":4.1")}])
    );
    assert_eq!(
        answer["kind"]["finished"]["basis"],
        json!({
            "text":"The potassium result and the clinic note.",
            "marks":[
                {"at":20,"source":lab,"place":null},
                {"at":40,"source":note,"place":null}
            ]
        })
    );
    let state = chat.store.conversation(&chat.scope).await.unwrap();
    let transcript = state.transcript(|_| None).unwrap();
    assert!(matches!(
        &transcript[0].answer,
        physician_wire::AnswerState::Finished { text, .. }
            if text == "Potassium was 4.1 mmol/L on 2026-08-01."
    ));
    let reopened = Store::connect(&std::env::var("PHYSICIAN_RUNTIME_TEST_DATABASE_URL").unwrap())
        .await
        .unwrap();
    let saved = reopened.conversation(&chat.scope).await.unwrap();
    assert_eq!(serde_json::to_value(&saved.messages).unwrap(), messages);
}

/// A made-up source that is not FHIR: its own kind and id fields, and a
/// letter whose text comes as base64 pages.
const HARBOR_LETTER_PAGES: &[AttachmentRule] = &[AttachmentRule::Base64Text {
    items_path: "pages",
    label: "letter_page",
    content_type_path: "media",
    data_path: "body64",
}];

const HARBOR_SIGN_STATE: &[ValueExtractor] = &[ValueExtractor::Scalar("sign_state")];

const HARBOR_CLINIC_TABLE: AdapterTable = AdapterTable {
    source_id: "harbor-clinic",
    kind_field: "doc_class",
    id_field: "doc_no",
    version_field: None,
    category_extractors: &[ValueExtractor::Scalar("unit")],
    rows: &[AdapterRow {
        kind: "visit_letter",
        category: CategoryMatch::Any,
        bucket: Bucket::Notes,
        dates: &[(DateRole::Authored, "written_on")],
        status: HARBOR_SIGN_STATE,
        codes: &[],
        title: &[ValueExtractor::Scalar("heading")],
        attachments: HARBOR_LETTER_PAGES,
    }],
    unmapped: UnmappedRecordRule {
        bucket: Bucket::Other,
        dates: &[],
        status: HARBOR_SIGN_STATE,
        codes: &[],
        title: &[ValueExtractor::Scalar("heading")],
        attachments: &[],
    },
};

const HARBOR_LETTER_TEXT: &str = "Creatinine was 1.4 mg/dL today, up from 1.1 in March. Hold ibuprofen and recheck in two weeks.";

/// H2. A record from a source that is not FHIR is cited like any other: it
/// is filed through its source's adapter table and stored by a pull, the
/// chart read gives it a label, and a finish that quotes its letter text is
/// saved with a mark that names its version and the quote's place.
#[tokio::test]
#[ignore = "requires explicit local Postgres"]
async fn a_record_from_a_source_that_is_not_fhir_is_cited_by_its_version_and_place() {
    let chat = Chat::open().await;
    let pages = base64::engine::general_purpose::STANDARD.encode(HARBOR_LETTER_TEXT);
    let raw = format!(
        r#"{{"doc_class":"visit_letter","doc_no":"HL-31","unit":"nephrology","sign_state":"signed","heading":"Kidney follow-up","written_on":"2026-08-03","pages":[{{"media":"text/plain","body64":"{pages}"}}]}}"#
    );
    let record = file_json_record(
        &HARBOR_CLINIC_TABLE,
        &chat.scope.workspace,
        HARBOR_CLINIC_TABLE.source_id,
        &raw,
        None,
    )
    .unwrap();
    let candidate = "harbor-clinic/visit_letter/HL-31";
    assert_eq!(record.candidate_id, candidate);
    let receipt = chat
        .store
        .record_chart_with_receipt(
            &chat.scope.workspace,
            std::slice::from_ref(&record),
            &SourcePull {
                source_id: HARBOR_CLINIC_TABLE.source_id.into(),
                source_label: "Harbor Clinic".into(),
                patient: SourcePatient {
                    source_patient_id: "HC-7".into(),
                    display_name: "Nadia Okafor".into(),
                    birth_date: Some("1984-02-19".into()),
                    sex_or_gender: None,
                    medical_record_numbers: Vec::new(),
                },
                pulled_at: Some(1_760_000_000),
            },
        )
        .await
        .unwrap();
    assert_eq!(receipt.changed, 1);

    let quote = "up from 1.1 in March";
    let mut script = Script::new([
        vec![call(
            "letter",
            READ_TOOL,
            json!({"references":[{"candidate_id":candidate,"revision":record.revision,"cursor":null}],"max_result_bytes":65_536}),
        )],
        vec![finish(
            "finish",
            "Creatinine rose to 1.4 mg/dL [S1].",
            &[("S1", quote)],
            "The kidney follow-up letter [S1].",
            false,
        )],
    ]);
    let ran = chat.attempt(Allowed, &mut script, 10).await;
    assert!(
        ran.is_ok() && chat.status().await == WorkStatus::Completed,
        "the finish was not accepted: {ran:?}; the model was told: {}",
        script.told("finish")
    );

    let reads = chat.reads().await;
    assert_eq!(reads.len(), 1, "{reads:#?}");
    let read = &reads[0]["results"][0];
    assert_eq!(read["reference"], candidate, "{read:#}");
    assert_eq!(read["cite_as"], "S1", "{read:#}");
    assert_eq!(read["source_kind"]["kind"], "visit_letter", "{read:#}");
    assert_eq!(
        read["readable_parts"][0]["text"], HARBOR_LETTER_TEXT,
        "{read:#}"
    );

    let messages = chat.messages().await;
    let answers = assistant(&messages);
    assert_eq!(answers.len(), 1);
    let answer = answers[0];
    assert_eq!(
        answer["text"], "Creatinine rose to 1.4 mg/dL.",
        "{answer:#}"
    );
    let version = json!({"candidate_id": candidate, "revision": record.revision});
    // Part 0 is the record's own content; part 1 is its decoded letter page.
    let start = HARBOR_LETTER_TEXT.find(quote).unwrap();
    assert_eq!(
        answer["kind"]["finished"]["marks"],
        json!([{
            "at": "Creatinine rose to 1.4 mg/dL".len(),
            "source": version,
            "place": {"passage": {"spans": [{"part": 1, "start": start, "end": start + quote.len()}]}}
        }]),
        "{answer:#}"
    );
    assert_eq!(
        answer["kind"]["finished"]["basis"],
        json!({
            "text": "The kidney follow-up letter.",
            "marks": [{"at": "The kidney follow-up letter".len(), "source": version, "place": null}]
        })
    );
}

/// A label with no quote is the citation: it is saved the first time as a
/// mark that names its version and has no place.
#[tokio::test]
#[ignore = "requires explicit local Postgres"]
async fn a_label_with_no_quote_is_saved_as_a_citation_of_its_version() {
    let chat = Chat::open().await;
    let mut script = Script::new([
        vec![read("lab", &chat.lab)],
        vec![finish(
            "finish",
            "Potassium was 4.1 [S1].",
            &[],
            "The result.",
            false,
        )],
    ]);
    chat.attempt(Allowed, &mut script, 10).await.unwrap();
    assert_eq!(
        script.requests.len(),
        2,
        "accepted the first time: {}",
        script.told("finish")
    );
    let messages = chat.messages().await;
    let answers = assistant(&messages);
    assert_eq!(answers.len(), 1);
    assert_eq!(answers[0]["text"], "Potassium was 4.1.");
    assert_eq!(
        answers[0]["kind"]["finished"]["marks"],
        json!([{"at":17,"source":version(&chat.lab),"place":null}])
    );
}

/// A description handed in for a cited label is saved with that label's
/// version, in the model's words, and reads the same after a restart.
#[tokio::test]
#[ignore = "requires explicit local Postgres"]
async fn a_description_of_a_cited_source_is_saved_with_its_version() {
    let chat = Chat::open().await;
    let words = "Potassium 4.1 mmol/L, final, on 2026-08-01.";
    let mut script = Script::new([
        vec![read("lab", &chat.lab)],
        vec![described_finish(
            "finish",
            "Potassium was 4.1 [S1].",
            &[],
            &[("S1", words)],
            "The result.",
            false,
        )],
    ]);
    chat.attempt(Allowed, &mut script, 10).await.unwrap();
    assert_eq!(chat.status().await, WorkStatus::Completed);
    let messages = chat.messages().await;
    let answers = assistant(&messages);
    assert_eq!(answers.len(), 1);
    assert_eq!(
        answers[0]["kind"]["finished"]["descriptions"],
        json!([{"source":version(&chat.lab),"text":words}])
    );
    let reopened = Store::connect(&std::env::var("PHYSICIAN_RUNTIME_TEST_DATABASE_URL").unwrap())
        .await
        .unwrap();
    let saved = reopened.conversation(&chat.scope).await.unwrap();
    assert_eq!(serde_json::to_value(&saved.messages).unwrap(), messages);
}

/// A description never refuses a finish. One for a label no read gave, a
/// blank one, one for a source read but not cited, and two for the same
/// cited source: the finish is accepted at once, its marks are as without
/// them, and only the first description of the cited source is saved.
#[tokio::test]
#[ignore = "requires explicit local Postgres"]
async fn only_the_first_description_of_a_cited_source_is_kept() {
    let chat = Chat::open().await;
    let mut script = Script::new([
        vec![read("lab", &chat.lab), read("note", &chat.note)],
        vec![described_finish(
            "finish",
            "Potassium was 4.1 [S1].",
            &[],
            &[
                ("S9", "A source no read gave."),
                ("S1", "  "),
                ("S2", "The clinic note, read but not cited."),
                ("S1", "Potassium 4.1 mmol/L, final."),
                ("S1", "A second account of the same result."),
            ],
            "The result [S1].",
            false,
        )],
    ]);
    chat.attempt(Allowed, &mut script, 10).await.unwrap();
    assert_eq!(
        script.requests.len(),
        2,
        "accepted the first time: {}",
        script.told("finish")
    );
    let messages = chat.messages().await;
    let answers = assistant(&messages);
    assert_eq!(answers.len(), 1);
    let lab = version(&chat.lab);
    assert_eq!(
        answers[0]["kind"]["finished"],
        json!({
            "marks":[{"at":17,"source":lab,"place":null}],
            "basis":{"text":"The result.","marks":[{"at":10,"source":lab,"place":null}]},
            "descriptions":[{"source":lab,"text":"Potassium 4.1 mmol/L, final."}]
        })
    );
}

/// A letter whose readable text repeats a sentence, and whose own content
/// holds a phrase its text also holds.
const LETTER_TEXT: &str = "The patient's follow-up visit is booked.\nPotassium was normal in May. Potassium was normal in June.\nClinic letter about potassium.";

/// Opens a chat whose chart also holds the letter, as `DocumentReference/q1`.
async fn chat_with_letter() -> (Chat, SourceRevision) {
    let chat = Chat::open().await;
    let data = base64::engine::general_purpose::STANDARD.encode(LETTER_TEXT);
    let content = format!(
        r#"{{"resourceType":"DocumentReference","id":"q1","description":"Clinic letter about potassium","content":[{{"attachment":{{"contentType":"text/plain","data":"{data}"}}}}]}}"#
    );
    let letter = chat.add(&content).await;
    (chat, letter)
}

fn quoting(id: &str, quotes: &[&str]) -> engine::Item {
    let answer: String = quotes.iter().map(|_| "A fact [S1]. ").collect();
    let quotes: Vec<(&str, &str)> = quotes.iter().map(|quote| ("S1", *quote)).collect();
    finish(id, answer.trim_end(), &quotes, "The letter.", false)
}

fn span(part: usize, text: &str, words: &str, from: usize) -> Value {
    let start = from + text[from..].find(words).unwrap();
    json!({"part": part, "start": start, "end": start + words.len()})
}

/// A quote matches across line breaks, curly quotation marks and dash
/// variants, and its place is the source's own bytes. A quote whose words
/// occur twice saves both places; words in the record's content and in its
/// text save a place in each.
#[tokio::test]
#[ignore = "requires explicit local Postgres"]
async fn a_quote_saves_every_place_its_words_occur() {
    let (chat, letter) = chat_with_letter().await;
    let mut script = Script::new([
        vec![read("letter", &letter)],
        vec![quoting(
            "finish",
            &[
                "patient\u{2019}s follow\u{2013}up\n  visit",
                "Potassium was normal",
                "Clinic letter about potassium",
            ],
        )],
    ]);
    chat.attempt(Allowed, &mut script, 10).await.unwrap();
    let messages = chat.messages().await;
    let answers = assistant(&messages);
    assert_eq!(answers.len(), 1, "{messages:#}");
    let places: Vec<&Value> = answers[0]["kind"]["finished"]["marks"]
        .as_array()
        .unwrap()
        .iter()
        .map(|mark| &mark["place"]["passage"]["spans"])
        .collect();
    let twice = LETTER_TEXT.find("Potassium was normal").unwrap() + 1;
    assert_eq!(
        places,
        [
            &json!([span(1, LETTER_TEXT, "patient's follow-up visit", 0)]),
            &json!([
                span(1, LETTER_TEXT, "Potassium was normal", 0),
                span(1, LETTER_TEXT, "Potassium was normal", twice)
            ]),
            &json!([
                span(0, &letter.content, "Clinic letter about potassium", 0),
                span(1, LETTER_TEXT, "Clinic letter about potassium", 0)
            ]),
        ]
    );
}

/// A quote with one changed letter, or that differs only by case, is not in
/// the text: the finish is saved the first time, each mark names the letter
/// with no place, and the quote's words are nowhere in what is saved.
#[tokio::test]
#[ignore = "requires explicit local Postgres"]
async fn a_quote_with_a_changed_letter_or_case_is_saved_without_a_place() {
    let (chat, letter) = chat_with_letter().await;
    let mut script = Script::new([
        vec![read("letter", &letter)],
        vec![quoting(
            "changed",
            &["Potassium was nornal", "potassium was normal"],
        )],
    ]);
    chat.attempt(Allowed, &mut script, 10).await.unwrap();
    assert_eq!(
        script.requests.len(),
        2,
        "accepted the first time: {}",
        script.told("changed")
    );
    let messages = chat.messages().await;
    let answers = assistant(&messages);
    assert_eq!(answers.len(), 1);
    let letter = version(&letter);
    assert_eq!(
        answers[0]["kind"]["finished"]["marks"],
        json!([
            {"at":6,"source":letter,"place":null},
            {"at":14,"source":letter,"place":null}
        ])
    );
    let saved = messages.to_string();
    for words in ["nornal", "potassium was normal"] {
        assert!(!saved.contains(words), "{saved}");
    }
}

/// A result whose text has the spacing and invisible characters a source can
/// hold: no space before a unit, a soft hyphen inside a word, and a thin
/// space before a degree sign.
const SPACING_TEXT: &str = "Potassium 4.1mmol/L on review.\nSodium was 140 mmol/L.\nThe potas\u{AD}sium trend is flat.\nTemp 37.5\u{2009}\u{B0}C at noon.";

/// The result as a report: its text is the presented form, and its contained
/// Observation holds the value as compact JSON.
fn spacing_content() -> String {
    let data = base64::engine::general_purpose::STANDARD.encode(SPACING_TEXT);
    format!(
        r#"{{"resourceType":"DiagnosticReport","id":"w1","contained":[{{"resourceType":"Observation","id":"k","valueQuantity":{{"value":4.1,"unit":"mmol/L"}}}}],"presentedForm":[{{"contentType":"text/plain","data":"{data}"}}]}}"#
    )
}

/// Spacing never counts in a quote: a space added or removed, compact JSON
/// quoted with spaces, and a soft hyphen inside a word all match, and each
/// place is the source's own bytes, from the first matched character to the
/// last.
#[tokio::test]
#[ignore = "requires explicit local Postgres"]
async fn spacing_and_invisible_characters_never_count_in_a_quote() {
    let chat = Chat::open().await;
    let report = chat.add(&spacing_content()).await;
    let mut script = Script::new([
        vec![read("w1", &report)],
        vec![quoting(
            "spacing",
            &[
                "Potassium 4.1 mmol/L",
                "Sodium was 140mmol/L",
                "\"value\": 4.1, \"unit\": \"mmol/L\"",
                "potassium trend",
                "37.5 \u{B0}C",
            ],
        )],
    ]);
    chat.attempt(Allowed, &mut script, 10).await.unwrap();
    let messages = chat.messages().await;
    let answers = assistant(&messages);
    assert_eq!(answers.len(), 1, "{messages:#}");
    assert!(
        answers[0]["kind"]["finished"].is_object(),
        "{}",
        script.told("spacing")
    );
    let places: Vec<&Value> = answers[0]["kind"]["finished"]["marks"]
        .as_array()
        .unwrap()
        .iter()
        .map(|mark| &mark["place"]["passage"]["spans"])
        .collect();
    // The thin space (3 bytes) and the degree sign (2 bytes) lie inside the
    // last span: 37.5 is 4 bytes, C is 1.
    let degrees = SPACING_TEXT.find("37.5").unwrap();
    assert_eq!(&SPACING_TEXT[degrees..degrees + 10], "37.5\u{2009}\u{B0}C");
    assert_eq!(
        places,
        [
            &json!([span(1, SPACING_TEXT, "Potassium 4.1mmol/L", 0)]),
            &json!([span(1, SPACING_TEXT, "Sodium was 140 mmol/L", 0)]),
            &json!([span(
                0,
                &report.content,
                "\"value\":4.1,\"unit\":\"mmol/L\"",
                0
            )]),
            &json!([span(1, SPACING_TEXT, "potas\u{AD}sium trend", 0)]),
            &json!([{"part": 1, "start": degrees, "end": degrees + 10}]),
        ]
    );
}

/// H3. A label no read gave is refused by name, in the answer and in the
/// Basis, and nothing is saved until a finish without it arrives. A quote
/// that is missing, empty or not found is never named, and its mark is saved
/// with no place.
#[tokio::test]
#[ignore = "requires explicit local Postgres"]
async fn a_label_no_read_gave_is_refused_by_name_and_a_quote_never_is() {
    let (chat, letter) = chat_with_letter().await;
    // S1 is quoted with the escapes the model sees in the read result, which
    // are not the record's text; S2's quote is empty; S3 has no quote.
    let quotes = [
        ("S1", "\\\"value\\\":4.1"),
        ("S2", ""),
        ("S9", "\"value\":4.1"),
    ];
    let mut script = Script::new([
        vec![
            read("lab", &chat.lab),
            read("note", &chat.note),
            read("letter", &letter),
        ],
        vec![finish(
            "bad",
            "Potassium was 4.1 [S1]. It was normal [S2]. The visit is booked [S3]. Sodium was 140 [S9].",
            &quotes,
            "The records [S7].",
            false,
        )],
        vec![finish(
            "good",
            "Potassium was 4.1 [S1]. It was normal [S2]. The visit is booked [S3].",
            &quotes[..2],
            "The records.",
            false,
        )],
    ]);
    chat.attempt(Allowed, &mut script, 10).await.unwrap();
    let told = script.told("bad");
    assert!(told.contains("\"executed\":false"), "{told}");
    assert!(
        told.contains("answer: S9 is not a cite_as label from your reads."),
        "{told}"
    );
    assert!(
        told.contains("basis: S7 is not a cite_as label from your reads."),
        "{told}"
    );
    for unnamed in ["S1", "S2", "S3", "quote"] {
        assert!(!told.contains(unnamed), "{unnamed}: {told}");
    }
    let messages = chat.messages().await;
    let answers = assistant(&messages);
    assert_eq!(answers.len(), 1, "only the finish without S9 is saved");
    assert_eq!(
        answers[0]["text"],
        "Potassium was 4.1. It was normal. The visit is booked."
    );
    assert_eq!(
        answers[0]["kind"]["finished"]["marks"],
        json!([
            {"at":17,"source":version(&chat.lab),"place":null},
            {"at":32,"source":version(&chat.note),"place":null},
            {"at":53,"source":version(&letter),"place":null}
        ])
    );
}

/// A label is citable only when a read of this work returned its version.
/// One read asks for a lab revision that has changed and for the note: the
/// lab's receipt is unavailable and carries no label, and the note's carries
/// `S2`. A finish that cites `S1` is refused by name; one that cites `S2` is
/// saved.
#[tokio::test]
#[ignore = "requires explicit local Postgres"]
async fn a_label_whose_read_returned_no_record_is_refused_by_name() {
    let chat = Chat::open().await;
    // The lab with a changed value: a revision the chart does not hold.
    let changed = filed(&chat.scope.workspace, &LAB.replace("4.1", "3.9"));
    let mut script = Script::new([
        vec![call(
            "both",
            READ_TOOL,
            json!({"references":[
                {"candidate_id":changed.candidate_id,"revision":changed.revision,"cursor":null},
                {"candidate_id":chat.note.candidate_id,"revision":chat.note.revision,"cursor":null}
            ],"max_result_bytes":65_536}),
        )],
        vec![finish(
            "unreturned",
            "Potassium was 4.1 [S1].",
            &[],
            "The result.",
            false,
        )],
        vec![finish(
            "returned",
            "Potassium was normal [S2].",
            &[],
            "The clinic note.",
            false,
        )],
    ]);
    chat.attempt(Allowed, &mut script, 10).await.unwrap();
    let results = &chat.reads().await[0]["results"];
    assert_eq!(results[0]["outcome"], "unavailable", "{results:#}");
    assert!(results[0].get("cite_as").is_none(), "{results:#}");
    assert_eq!(results[1]["cite_as"], "S2", "{results:#}");
    assert_eq!(script.requests.len(), 3, "the finish citing S1 is refused");
    let told = script.told("unreturned");
    assert!(
        told.contains("answer: S1 is not a cite_as label from your reads."),
        "{told}"
    );
    let messages = chat.messages().await;
    let answers = assistant(&messages);
    assert_eq!(answers.len(), 1);
    assert_eq!(answers[0]["text"], "Potassium was normal.");
    assert_eq!(
        answers[0]["kind"]["finished"]["marks"],
        json!([{"at":20,"source":version(&chat.note),"place":null}])
    );
}

/// The place of the lab's words, as a saved span.
fn in_lab(words: &str) -> Value {
    let start = LAB.find(words).unwrap();
    json!({"part": 0, "start": start, "end": start + words.len()})
}

/// An answer that writes `[S1]` three times, with these quotes for it.
fn thrice(id: &str, quotes: &[&str]) -> engine::Item {
    let quotes: Vec<(&str, &str)> = quotes.iter().map(|quote| ("S1", *quote)).collect();
    finish(
        id,
        "Potassium was 4.1 [S1], in range [S1], and recent [S1].",
        &quotes,
        "The result.",
        false,
    )
}

/// A label written three times with one quote is accepted the first time,
/// and each of its three marks takes that quote's place.
#[tokio::test]
#[ignore = "requires explicit local Postgres"]
async fn one_quote_serves_every_mark_of_its_label() {
    let chat = Chat::open().await;
    let mut script = Script::new([
        vec![read("lab", &chat.lab)],
        vec![thrice("finish", &["\"value\":4.1"])],
    ]);
    chat.attempt(Allowed, &mut script, 10).await.unwrap();
    assert_eq!(
        script.requests.len(),
        2,
        "accepted the first time: {}",
        script.told("finish")
    );
    let messages = chat.messages().await;
    let answers = assistant(&messages);
    assert_eq!(answers.len(), 1);
    assert_eq!(
        answers[0]["text"],
        "Potassium was 4.1, in range, and recent."
    );
    let place = json!({"passage":{"spans":[in_lab("\"value\":4.1")]}});
    let marks: Vec<(&Value, &Value)> = answers[0]["kind"]["finished"]["marks"]
        .as_array()
        .unwrap()
        .iter()
        .map(|mark| (&mark["at"], &mark["place"]))
        .collect();
    assert_eq!(
        marks,
        [
            (&json!(17), &place),
            (&json!(27), &place),
            (&json!(39), &place)
        ]
    );
}

/// A label written once with two quotes is accepted the first time, and
/// its one mark holds both passages, in byte order.
#[tokio::test]
#[ignore = "requires explicit local Postgres"]
async fn one_mark_holds_every_quote_of_its_label() {
    let chat = Chat::open().await;
    let mut script = Script::new([
        vec![read("lab", &chat.lab)],
        vec![finish(
            "finish",
            "Potassium was 4.1 mmol/L on 2026-08-01 [S1].",
            &[
                ("S1", "\"effectiveDateTime\":\"2026-08-01\""),
                ("S1", "\"value\":4.1"),
            ],
            "The result.",
            false,
        )],
    ]);
    chat.attempt(Allowed, &mut script, 10).await.unwrap();
    assert_eq!(
        script.requests.len(),
        2,
        "accepted the first time: {}",
        script.told("finish")
    );
    let messages = chat.messages().await;
    let answers = assistant(&messages);
    assert_eq!(answers.len(), 1);
    let marks = &answers[0]["kind"]["finished"]["marks"];
    assert_eq!(marks.as_array().unwrap().len(), 1, "{marks:#}");
    assert_eq!(
        marks[0]["place"]["passage"]["spans"],
        json!([
            in_lab("\"value\":4.1"),
            in_lab("\"effectiveDateTime\":\"2026-08-01\"")
        ])
    );
}

/// Three marks and four quotes of one label are saved the first time, and
/// every mark holds the places of all four quotes, in byte order.
#[tokio::test]
#[ignore = "requires explicit local Postgres"]
async fn marks_and_quotes_of_other_counts_each_take_every_quote() {
    let chat = Chat::open().await;
    let quotes = [
        "\"value\":4.1",
        "\"effectiveDateTime\":\"2026-08-01\"",
        "\"status\":\"final\"",
        "\"text\":\"Potassium\"",
    ];
    let mut script = Script::new([
        vec![read("lab", &chat.lab)],
        vec![thrice("finish", &quotes)],
    ]);
    chat.attempt(Allowed, &mut script, 10).await.unwrap();
    assert_eq!(
        script.requests.len(),
        2,
        "accepted the first time: {}",
        script.told("finish")
    );
    let messages = chat.messages().await;
    let answers = assistant(&messages);
    assert_eq!(answers.len(), 1);
    assert_eq!(
        answers[0]["text"],
        "Potassium was 4.1, in range, and recent."
    );
    let spans = json!([
        in_lab("\"status\":\"final\""),
        in_lab("\"text\":\"Potassium\""),
        in_lab("\"value\":4.1"),
        in_lab("\"effectiveDateTime\":\"2026-08-01\""),
    ]);
    let marks: Vec<(&Value, &Value)> = answers[0]["kind"]["finished"]["marks"]
        .as_array()
        .unwrap()
        .iter()
        .map(|mark| (&mark["at"], &mark["place"]["passage"]["spans"]))
        .collect();
    assert_eq!(
        marks,
        [
            (&json!(17), &spans),
            (&json!(27), &spans),
            (&json!(39), &spans)
        ]
    );
}

/// H4. A label read before a fresh window still names its version: a finish
/// that cites it is saved the first time, without reading the record again,
/// as a mark with no place, because its text is not delivered now.
#[tokio::test]
#[ignore = "requires explicit local Postgres"]
async fn a_label_read_before_a_fresh_window_is_saved_without_a_place() {
    let chat = Chat::open().await;
    let mut script = Script::new([
        vec![read("lab", &chat.lab)],
        vec![call(
            "note",
            "save_working_note",
            json!({"text":"Potassium result is S1.","fresh_window":true}),
        )],
        vec![good_finish_lab_only("stale")],
    ]);
    chat.attempt(Allowed, &mut script, 10).await.unwrap();
    assert_eq!(
        script.requests.len(),
        3,
        "accepted the first time: {}",
        script.told("stale")
    );
    assert_eq!(chat.reads().await.len(), 1);
    let messages = chat.messages().await;
    let answers = assistant(&messages);
    assert_eq!(answers.len(), 1);
    assert_eq!(answers[0]["text"], "Potassium was 4.1.");
    assert_eq!(
        answers[0]["kind"]["finished"]["marks"],
        json!([{"at":17,"source":version(&chat.lab),"place":null}])
    );
}

/// H5. Two finishes refused for a label no read gave; the third is saved
/// with the valid marks, and the bad mark leaves the text.
#[tokio::test]
#[ignore = "requires explicit local Postgres"]
async fn the_third_finish_saves_valid_marks_and_drops_the_bad_one() {
    let chat = Chat::open().await;
    let bad = |id: &str| {
        finish(
            id,
            "Potassium was 4.1 [S1] and sodium 140 [S9].",
            &[("S1", "\"value\":4.1"), ("S9", "\"id\":\"k1\"")],
            "The result [S1].",
            false,
        )
    };
    let mut script = Script::new([
        vec![read("lab", &chat.lab)],
        vec![bad("first")],
        vec![bad("second")],
        vec![bad("third")],
    ]);
    chat.attempt(Allowed, &mut script, 10).await.unwrap();
    assert_eq!(script.requests.len(), 4, "the third finish ends the run");
    assert!(script.told("first").contains("S9"));
    assert!(script.told("second").contains("S9"));
    let messages = chat.messages().await;
    let answers = assistant(&messages);
    assert_eq!(answers.len(), 1);
    assert_eq!(answers[0]["text"], "Potassium was 4.1 and sodium 140.");
    let finished = &answers[0]["kind"]["finished"];
    assert_eq!(finished["marks"].as_array().unwrap().len(), 1);
    assert_eq!(finished["marks"][0]["at"], 17);
    assert_eq!(finished["basis"]["text"], "The result.");
    assert_eq!(finished["basis"]["marks"].as_array().unwrap().len(), 1);
}

/// H6. A finish with no Basis, an answer or Basis that is only labels, plain
/// text and a finish beside another call are refused every time, and none
/// counts toward the two refusals: the next finish with a bad citation is
/// still refused.
#[tokio::test]
#[ignore = "requires explicit local Postgres"]
async fn a_finish_without_its_parts_or_alone_is_refused_every_time() {
    let chat = Chat::open().await;
    let no_basis = |id: &str| {
        finish(
            id,
            "Potassium was 4.1 [S1].",
            &[("S1", "\"value\":4.1")],
            "  ",
            false,
        )
    };
    let bad_label = |id: &str| {
        finish(
            id,
            "Potassium was 4.1 [S9].",
            &[("S9", "\"value\":4.1")],
            "The result.",
            false,
        )
    };
    let mut script = Script::new([
        vec![read("lab", &chat.lab)],
        vec![no_basis("no-basis-1")],
        vec![text("Potassium was 4.1 mmol/L.")],
        vec![no_basis("no-basis-2"), read("mixed-read", &chat.lab)],
        vec![text("Potassium was 4.1 mmol/L, again.")],
        vec![no_basis("no-basis-3")],
        vec![finish(
            "labels-only-answer",
            " [S1] ",
            &[("S1", "\"value\":4.1")],
            "The result.",
            false,
        )],
        vec![finish(
            "labels-only-basis",
            "Potassium was 4.1 [S1].",
            &[("S1", "\"value\":4.1")],
            "[S1]",
            false,
        )],
        vec![bad_label("counted-1")],
        vec![bad_label("counted-2")],
        vec![good_finish_lab_only("good")],
    ]);
    chat.attempt(Allowed, &mut script, 20).await.unwrap();
    assert_eq!(script.requests.len(), 11);
    for refused in [
        "no-basis-1",
        "no-basis-2",
        "mixed-read",
        "no-basis-3",
        "labels-only-answer",
        "labels-only-basis",
    ] {
        assert!(
            script.told(refused).contains("\"executed\":false"),
            "{refused}: {}",
            script.told(refused)
        );
    }
    assert!(script.told("no-basis-1").contains("basis"));
    assert!(
        script
            .told("labels-only-answer")
            .contains("Give the complete answer in answer."),
        "{}",
        script.told("labels-only-answer")
    );
    assert!(
        script
            .told("labels-only-basis")
            .contains("Give the basis: what the answer rests on"),
        "{}",
        script.told("labels-only-basis")
    );
    assert!(script.told("counted-2").contains("S9"));
    let messages = chat.messages().await;
    let answers = assistant(&messages);
    assert_eq!(answers.len(), 1);
    assert_eq!(answers[0]["text"], "Potassium was 4.1.");
}

fn good_finish_lab_only(id: &str) -> engine::Item {
    finish(
        id,
        "Potassium was 4.1 [S1].",
        &[("S1", "\"value\":4.1")],
        "The result.",
        false,
    )
}

/// H7. `cites_none` is read only when neither the answer nor the Basis has a
/// label. With no label it saves with no citation; with no label and
/// `cites_none` false the finish is refused twice and the third is saved
/// with no citation; beside a label, in the answer or the Basis, the label
/// is saved as its citation.
#[tokio::test]
#[ignore = "requires explicit local Postgres"]
async fn cites_none_is_read_only_when_no_label_is_written() {
    /// Reads the lab, then hands in these finishes in turn. Gives the
    /// messages and the version a citation of the lab names.
    async fn run(finishes: Vec<engine::Item>) -> (Script, Value, Value) {
        let chat = Chat::open().await;
        let mut script = Script::new(
            std::iter::once(vec![read("lab", &chat.lab)])
                .chain(finishes.into_iter().map(|finish| vec![finish])),
        );
        chat.attempt(Allowed, &mut script, 10).await.unwrap();
        (script, chat.messages().await, version(&chat.lab))
    }
    let general = |id: &str, cites_none: bool| {
        finish(
            id,
            "Hyperkalaemia is a potassium above the reference range.",
            &[],
            "General medical knowledge.",
            cites_none,
        )
    };
    let uncited = json!({"marks":[],"basis":{"text":"General medical knowledge.","marks":[]},"descriptions":[]});

    let (script, messages, _) = run(vec![general("none", true)]).await;
    assert_eq!(script.requests.len(), 2, "{}", script.told("none"));
    let answers = assistant(&messages);
    assert_eq!(answers.len(), 1);
    assert_eq!(
        answers[0]["text"],
        "Hyperkalaemia is a potassium above the reference range."
    );
    assert_eq!(answers[0]["kind"]["finished"], uncited);

    let (script, messages, _) = run(vec![
        general("unlabelled-1", false),
        general("unlabelled-2", false),
        general("unlabelled-3", false),
    ])
    .await;
    for refused in ["unlabelled-1", "unlabelled-2"] {
        let told = script.told(refused);
        assert!(
            told.contains("\"executed\":false")
                && told.contains("Neither the answer nor the basis has a label."),
            "{refused}: {told}"
        );
    }
    assert_eq!(script.requests.len(), 4, "the third finish ends the run");
    let answers = assistant(&messages);
    assert_eq!(answers.len(), 1);
    assert_eq!(answers[0]["kind"]["finished"], uncited);

    let (script, messages, lab) = run(vec![finish(
        "answer-label",
        "Potassium was 4.1 [S1].",
        &[("S1", "\"value\":4.1")],
        "The result.",
        true,
    )])
    .await;
    assert_eq!(script.requests.len(), 2, "{}", script.told("answer-label"));
    let answers = assistant(&messages);
    assert_eq!(
        answers[0]["kind"]["finished"]["marks"],
        json!([{"at":17,"source":lab,"place":{"passage":{"spans":[in_lab("\"value\":4.1")]}}}])
    );

    let (script, messages, lab) = run(vec![finish(
        "basis-label",
        "Potassium is in the reference range.",
        &[],
        "The latest potassium [S1].",
        true,
    )])
    .await;
    assert_eq!(script.requests.len(), 2, "{}", script.told("basis-label"));
    let answers = assistant(&messages);
    assert_eq!(
        answers[0]["kind"]["finished"],
        json!({"marks":[],"basis":{"text":"The latest potassium.","marks":[{"at":20,"source":lab,"place":null}]},"descriptions":[]})
    );
}

/// H8. Stop before an accepted finish leaves no answer, citations or Basis.
#[tokio::test]
#[ignore = "requires explicit local Postgres"]
async fn stop_before_an_accepted_finish_saves_nothing() {
    struct StoppedWhileFinishing {
        chat: Chat,
        dispatches: u32,
    }
    impl engine::recipe::Provider for StoppedWhileFinishing {
        async fn exchange(
            &mut self,
            _: &Request,
            _: &engine::Cancellation,
            _: &engine::RunLimits,
            _: &mut dyn Observer,
        ) -> Result<engine::Turn, engine::Cause> {
            self.dispatches += 1;
            if self.dispatches == 1 {
                return Ok(turn(vec![read("lab", &self.chat.lab)]));
            }
            let state = self
                .chat
                .store
                .conversation(&self.chat.scope)
                .await
                .unwrap();
            self.chat
                .store
                .stop_authorized(&self.chat.scope, "stop", vec![0; 32], &state.work)
                .await
                .unwrap();
            Ok(turn(vec![good_finish_lab_only("finish")]))
        }
    }
    let chat = Chat::open().await;
    let mut provider = StoppedWhileFinishing {
        chat: chat.clone(),
        dispatches: 0,
    };
    assert!(chat.attempt(Allowed, &mut provider, 10).await.is_err());
    assert_eq!(chat.status().await, WorkStatus::Held);
    assert!(assistant(&chat.messages().await).is_empty());
}

/// H8. At the run limit the final request cannot call tools: the text is
/// kept as text only, with no labels, and a finish in that turn saves no
/// citations or Basis.
#[tokio::test]
#[ignore = "requires explicit local Postgres"]
async fn a_finish_at_the_run_limit_keeps_text_only() {
    let chat = Chat::open().await;
    let mut script = Script::new([
        vec![read("lab", &chat.lab)],
        vec![
            text("Potassium was 4.1 mmol/L [S1]."),
            good_finish_lab_only("final"),
        ],
    ]);
    chat.attempt(Allowed, &mut script, 2).await.unwrap();
    assert_eq!(chat.status().await, WorkStatus::Held);
    let messages = chat.messages().await;
    let answers = assistant(&messages);
    assert_eq!(answers.len(), 1);
    assert_eq!(answers[0]["kind"], "kept", "{:#}", answers[0]);
    assert_eq!(answers[0]["text"], "Potassium was 4.1 mmol/L.");
}

/// H8. A provider block after a refused finish holds the work with no
/// answer, citations or Basis.
#[tokio::test]
#[ignore = "requires explicit local Postgres"]
async fn a_provider_block_after_a_refused_finish_saves_nothing() {
    /// Reads, hands in a finish with a bad label, then the provider fails.
    struct RefusedThenFails(u32, SourceRevision);
    impl engine::recipe::Provider for RefusedThenFails {
        async fn exchange(
            &mut self,
            _: &Request,
            _: &engine::Cancellation,
            _: &engine::RunLimits,
            _: &mut dyn Observer,
        ) -> Result<engine::Turn, engine::Cause> {
            self.0 += 1;
            match self.0 {
                1 => Ok(turn(vec![read("lab", &self.1)])),
                2 => Ok(turn(vec![finish(
                    "bad",
                    "Potassium was 4.1 [S9].",
                    &[("S9", "\"value\":4.1")],
                    "The result.",
                    false,
                )])),
                _ => Err("synthetic provider failure".into()),
            }
        }
    }
    let chat = Chat::open().await;
    assert!(
        chat.attempt(Allowed, &mut RefusedThenFails(0, chat.lab.clone()), 10)
            .await
            .is_err()
    );
    // The provider then blocks the successor's first request.
    let claim = chat.store.claim(&chat.scope, 60).await.unwrap().unwrap();
    chat.store
        .persist(
            &claim,
            engine::Event {
                attempt_id: claim.attempt.clone(),
                sequence: 1,
                ordinal: 1,
                kind: engine::EventKind::ProviderBlocked {
                    reason: "Provider credits are exhausted.".into(),
                },
            },
        )
        .await
        .unwrap();
    assert_eq!(chat.status().await, WorkStatus::Held);
    assert!(assistant(&chat.messages().await).is_empty());
}

/// H9. A worker restart between refused finishes gives the same decisions:
/// the successor reads the two refusals from the run record and settles the
/// retained third finish, with its bad mark dropped, without a new request.
#[tokio::test]
#[ignore = "requires explicit local Postgres"]
async fn a_restart_between_refused_finishes_gives_the_same_decision() {
    let chat = Chat::open().await;
    let bad = |id: &str| {
        finish(
            id,
            "Potassium was 4.1 [S1] and sodium 140 [S9].",
            &[("S1", "\"value\":4.1"), ("S9", "\"id\":\"k1\"")],
            "The result.",
            false,
        )
    };
    let mut first = Script::new([
        vec![read("lab", &chat.lab)],
        vec![bad("first")],
        vec![bad("second")],
        vec![bad("third")],
    ]);
    assert!(
        chat.attempt(InterruptedAtFinish(3), &mut first, 10)
            .await
            .is_err(),
        "the worker is lost at the third finish"
    );
    assert!(assistant(&chat.messages().await).is_empty());
    assert!(first.told("second").contains("S9"));
    chat.attempt(Allowed, &mut MustNotDispatch, 10)
        .await
        .unwrap();
    let messages = chat.messages().await;
    let answers = assistant(&messages);
    assert_eq!(answers.len(), 1);
    assert_eq!(answers[0]["text"], "Potassium was 4.1 and sodium 140.");
    assert_eq!(
        answers[0]["kind"]["finished"]["marks"]
            .as_array()
            .unwrap()
            .len(),
        1
    );
}

/// A steer keeps the same work: labels stay, earlier reads stay delivered,
/// and a refusal before the steer still counts after it.
#[tokio::test]
#[ignore = "requires explicit local Postgres"]
async fn a_steer_keeps_labels_and_refusals_of_the_work() {
    struct SteeredAfterRefusal {
        chat: Chat,
        dispatches: u32,
    }
    impl engine::recipe::Provider for SteeredAfterRefusal {
        async fn exchange(
            &mut self,
            _: &Request,
            _: &engine::Cancellation,
            _: &engine::RunLimits,
            _: &mut dyn Observer,
        ) -> Result<engine::Turn, engine::Cause> {
            self.dispatches += 1;
            match self.dispatches {
                1 => Ok(turn(vec![read("lab", &self.chat.lab)])),
                2 => Ok(turn(vec![finish(
                    "before-steer",
                    "Potassium was 4.1 [S9].",
                    &[("S9", "\"value\":4.1")],
                    "The result.",
                    false,
                )])),
                _ => {
                    self.chat.ask("steer", "Also give the unit.").await;
                    Ok(turn(vec![text("Superseded.")]))
                }
            }
        }
    }
    let chat = Chat::open().await;
    let mut steered = SteeredAfterRefusal {
        chat: chat.clone(),
        dispatches: 0,
    };
    assert!(chat.attempt(Allowed, &mut steered, 10).await.is_err());
    let bad = |id: &str| {
        finish(
            id,
            "Potassium was 4.1 mmol/L [S1] [S9].",
            &[("S1", "\"value\":4.1"), ("S9", "\"id\":\"k1\"")],
            "The result.",
            false,
        )
    };
    let mut after = Script::new([vec![bad("after-1")], vec![bad("after-2")]]);
    chat.attempt(Allowed, &mut after, 10).await.unwrap();
    assert!(after.told("after-1").contains("S9"));
    assert_eq!(
        after.requests.len(),
        2,
        "the second finish after the steer is the third"
    );
    let messages = chat.messages().await;
    let answers = assistant(&messages);
    assert_eq!(answers.len(), 1);
    assert_eq!(answers[0]["text"], "Potassium was 4.1 mmol/L.");
    assert_eq!(
        answers[0]["kind"]["finished"]["marks"][0]["source"],
        version(&chat.lab)
    );
}

/// A captured unfamiliar Hint subtype remains Other and its native content can
/// support a saved citation, with the missing full-note detail visible in the read.
#[tokio::test]
#[ignore = "requires explicit local Postgres"]
async fn hint_other_is_cited_from_its_persisted_native_record_with_read_limits() {
    use local_product::physician_sources::{SourcePullOutput, hint::ChartCapture};
    use local_product::source_status::PartialDelivery;
    let chat = Chat::open().await;
    let capture = json!({"request":{"patient_id":"hint-patient"},"sections":[
        {"section":"patient","state":"done","result":{"body":{"items":[{"body":{"id":"hint-patient","first_name":"Nadia","last_name":"Okafor"},"unavailable_parts":[]}],"failed_parts":[]}}},
        {"section":"notes","state":"done","result":{"body":{"items":[{"body":{"noteId":"native-note","patientId":"hint-patient","noteType":"UNFAMILIAR","data":{"text":"Retained unfamiliar clinical evidence"}},"unavailable_parts":[{"part":"full note detail","reason":"full note detail was not acquired"}]}],"failed_parts":[]}}}
    ]});
    let SourcePullOutput::Partial {
        patient,
        records,
        failed_parts,
        succeeded_kinds,
    } = serde_json::from_value::<ChartCapture>(capture)
        .unwrap()
        .file(&chat.scope.workspace, "hint", "hint-patient")
    else {
        panic!("bounded capture");
    };
    chat.store
        .record_partial_pull(PartialDelivery {
            practice_id: &chat.scope.practice,
            workspace_id: &chat.scope.workspace,
            source_id: "hint",
            source_label: "Hint capture",
            patient: patient.as_ref(),
            records: &records,
            failed_parts: &failed_parts,
            succeeded_kinds: &succeeded_kinds,
            attempted_at: 1_760_000_000,
            ticket: None,
        })
        .await
        .unwrap();
    let record = records
        .iter()
        .find(|record| record.candidate_id == "hint/notes/native-note")
        .unwrap();
    let mut script = Script::new([
        vec![read("native", record)],
        vec![finish(
            "finish",
            "The captured evidence is retained [S1].",
            &[("S1", "Retained unfamiliar clinical evidence")],
            "The acquired note list; full detail was unavailable [S1].",
            false,
        )],
    ]);
    chat.attempt(Allowed, &mut script, 10).await.unwrap();
    assert_eq!(chat.status().await, WorkStatus::Completed);
    let reads = chat.reads().await;
    assert_eq!(reads[0]["results"][0]["bucket"], "other");
    assert!(
        reads[0]["results"][0]["unavailable_parts"][0]["reason"]
            .as_str()
            .unwrap()
            .contains("full note detail")
    );
    let reopened = Store::connect(&std::env::var("PHYSICIAN_RUNTIME_TEST_DATABASE_URL").unwrap())
        .await
        .unwrap();
    let saved = reopened.conversation(&chat.scope).await.unwrap();
    let messages = serde_json::to_value(saved.messages).unwrap();
    let answers = assistant(&messages);
    let mark = &answers[0]["kind"]["finished"]["marks"][0];
    assert_eq!(mark["source"], version(record));
    let start = record
        .content
        .find("Retained unfamiliar clinical evidence")
        .unwrap();
    assert_eq!(
        mark["place"]["passage"]["spans"],
        json!([{"part":0,"start":start,"end":start + "Retained unfamiliar clinical evidence".len()}])
    );
}
