//! A conversation's transcript as the page reads it, over real processes.
//!
//! One site and one worker share a disposable Postgres and Valkey, and a
//! scripted provider asks for chart searches and reads. A test holds a chart
//! read in flight by locking the patient's source version row from its own
//! connection: the read waits on that lock as it would on a slow chart, and
//! returns when the test lets go.

use std::path::PathBuf;
use std::time::{Duration, Instant};

use local_product::physician_sources::adapter::{file_fhir_record, file_json_record};
use local_product::runtime::Store;
use physician_wire::{
    AnswerState, Exchange, ExchangeSteps, Found, RecordAbout, RecordGroup, RecordRead, SearchPage,
    SourceVersion, Step, StepState, WorkResumeTarget,
};
use sea_orm::{ConnectionTrait, DatabaseTransaction, TransactionTrait};
use serde_json::{Value, json};

#[allow(dead_code)]
#[path = "../../local-product/tests/support/hint.rs"]
mod hint;
#[allow(dead_code)]
#[path = "../../local-product/tests/support/northwind.rs"]
mod northwind;
#[path = "scale/mod.rs"]
mod support;

/// The chart record the scripted provider searches for and reads, a FHIR
/// result filed through the FHIR adapter. Its marker word matches no fixture
/// record, so a search finds only it.
const SODIUM: &str = r#"{"resourceType":"Observation","id":"sodium","status":"final","category":[{"coding":[{"system":"http://terminology.hl7.org/CodeSystem/observation-category","code":"laboratory"}]}],"code":{"text":"Sodium","coding":[{"code":"2951-2"}]},"valueQuantity":{"value":139,"unit":"mmol/L"},"effectiveDateTime":"2026-10-01","note":[{"text":"transcriptproof"}]}"#;
/// The search text that finds only [`SODIUM`].
const QUERY: &str = "transcriptproof";

struct Home(PathBuf);

impl Drop for Home {
    fn drop(&mut self) {
        let _ = std::fs::remove_dir_all(&self.0);
    }
}

/// The site, the worker and the scripted provider, with one signed-in
/// physician and one new chat about the first fixture patient, whose chart
/// holds one sodium result.
struct Stack {
    database: support::Database,
    _valkey: support::Valkey,
    stub: support::ResponsesStub,
    site: support::Process,
    site_port: u16,
    worker: support::Process,
    worker_port: u16,
    session: String,
    chat: String,
    _home: Home,
}

impl Stack {
    /// Starts everything; `None` when no test Postgres is configured. Every
    /// provider response waits `delay_ms`, so a run is still working after
    /// each step returns.
    async fn start(label: &str, delay_ms: u64, rounds: &Value) -> Option<Self> {
        let database = support::Database::create(label).await?;
        support::staging_account(&database.url).await;
        let home = Home(std::env::temp_dir().join(format!(
            "physician-transcript-{label}-{}-{}",
            std::process::id(),
            uuid::Uuid::new_v4().simple()
        )));
        std::fs::create_dir_all(&home.0).expect("home");
        let valkey = support::Valkey::start().await;
        let stub = support::ResponsesStub::start(&home.0, 0, delay_ms, "[]").await;
        let sodium = sodium();
        let revision = sodium.revision.clone();
        Store::connect(&database.url)
            .await
            .expect("runtime store")
            .record_source(&sodium)
            .await
            .expect("chart record");
        let rounds = rounds.to_string().replace("{revision}", &revision);
        stub.tool_rounds(&serde_json::from_str(&rounds).unwrap());
        let (site_port, worker_port) = (support::free_port(), support::free_port());
        let redis = valkey.url();
        let mut worker = support::Process::spawn(
            "worker",
            support::worker_bin(),
            &[],
            &[
                ("PHYSICIAN_DATABASE_URL", database.url.as_str()),
                ("PHYSICIAN_REDIS_URL", redis.as_str()),
                (
                    "PHYSICIAN_WORKER_LISTEN_ADDR",
                    &format!("127.0.0.1:{worker_port}"),
                ),
                ("PHYSICIAN_RESPONSES_STUB_PORT", &stub.port),
                // A stopped worker's claim runs out soon, and a new worker
                // finds the work at once.
                ("PHYSICIAN_WORKER_LEASE_SECONDS", "5"),
                ("PHYSICIAN_WORKER_POLL_MS", "200"),
            ],
            &home.0,
        );
        support::wait_ready(&mut worker, worker_port).await;
        let mut site = support::Process::spawn(
            "site",
            support::site_bin(),
            &[],
            &[
                ("PHYSICIAN_DATABASE_URL", database.url.as_str()),
                ("PHYSICIAN_REDIS_URL", redis.as_str()),
                support::DURABLE,
                ("PHYSICIAN_AUTH", "required"),
                ("PHYSICIAN_LISTEN_ADDR", &format!("127.0.0.1:{site_port}")),
            ],
            &home.0,
        );
        support::wait_ready(&mut site, site_port).await;
        let deadline = Instant::now() + Duration::from_secs(15);
        while valkey.subscribers() < 2 {
            assert!(Instant::now() < deadline, "the site and worker on Valkey");
            tokio::time::sleep(Duration::from_millis(50)).await;
        }
        let session = support::sign_in(&format!("127.0.0.1:{site_port}")).await;
        Some(Self {
            database,
            _valkey: valkey,
            stub,
            site,
            site_port,
            worker,
            worker_port,
            session,
            chat: format!("chat-{}", uuid::Uuid::new_v4().simple()),
            _home: home,
        })
    }

    fn host(&self) -> String {
        format!("127.0.0.1:{}", self.site_port)
    }

    fn logs(&self) -> String {
        format!("{}\n{}", self.site.tail(), self.worker.tail())
    }

    async fn command(&self, kind: &str, key: &str, payload: Value) {
        let mut payload = payload;
        payload["patient_workspace_id"] = json!(workspace());
        payload["chat_id"] = json!(self.chat);
        let body = json!({"type": kind, "idempotency_key": key, "payload": payload});
        let (status, answer) =
            support::post(&self.host(), "/commands", Some(&self.session), &body).await;
        assert_eq!(
            (status, answer["outcome"].as_str()),
            (200, Some("accepted")),
            "{kind}: {answer}\n{}",
            self.logs()
        );
    }

    async fn send(&self, text: &str) {
        let key = format!("key-{}", uuid::Uuid::new_v4().simple());
        let payload = json!({"text": text, "physician_time_zone": "America/New_York"});
        self.command("message.submit", &key, payload).await;
    }

    async fn snapshot(&self) -> Value {
        let path = format!(
            "/patient-workspaces/{}/chats/{}/snapshot",
            workspace(),
            self.chat
        );
        let (status, body) = support::get_json(&self.host(), &path, &self.session).await;
        assert_eq!(status, 200, "{body}\n{}", self.logs());
        body["snapshot"].clone()
    }

    /// The physician's Today as the page reads it.
    async fn today(&self) -> Value {
        let (status, body) = support::get_json(&self.host(), "/today", &self.session).await;
        assert_eq!(status, 200, "{body}\n{}", self.logs());
        body
    }

    async fn transcript(&self) -> Vec<Exchange> {
        support::chat(&self.snapshot().await).transcript
    }

    /// Reads the transcript until `done` holds, and returns it.
    async fn wait(&self, what: &str, done: impl Fn(&[Exchange]) -> bool) -> Vec<Exchange> {
        let deadline = Instant::now() + Duration::from_secs(60);
        loop {
            let transcript = self.transcript().await;
            if done(&transcript) {
                return transcript;
            }
            assert!(
                Instant::now() < deadline,
                "{what}: {transcript:#?}\n{}",
                self.logs()
            );
            tokio::time::sleep(Duration::from_millis(50)).await;
        }
    }

    async fn events(&self) -> support::Sse {
        support::Sse::open(
            &self.host(),
            &format!(
                "/patient-workspaces/events?patient_workspace_id={}",
                workspace()
            ),
            &self.session,
        )
        .await
        .unwrap_or_else(|error| panic!("{error}\n{}", self.logs()))
    }

    /// Holds every chart search and read of the patient until the returned
    /// transaction ends.
    async fn hold_chart(&self) -> DatabaseTransaction {
        use sea_orm::sea_query::{Alias, Expr, ExprTrait, Query};
        let db = sea_orm::Database::connect(&self.database.url)
            .await
            .expect("hold connection");
        let hold = db.begin().await.expect("hold transaction");
        hold.execute(
            Query::select()
                .column(Alias::new("version"))
                .from(Alias::new("physician_runtime_source_versions"))
                .and_where(Expr::col(Alias::new("workspace")).eq(workspace()))
                .lock_exclusive(),
        )
        .await
        .expect("hold the chart");
        hold
    }
}

/// [`SODIUM`] filed through the FHIR adapter into the chart.
fn sodium() -> local_product::physician_sources::SourceRevision {
    file_fhir_record(workspace(), "synthetic-charts", SODIUM, None)
        .expect("the adapter files the record")
}

fn workspace() -> &'static str {
    physician_sample::WORKSPACE_IDS[0]
}

fn search_call() -> Value {
    json!({"name": "search_patient_information",
           "arguments": {"query": QUERY, "record_type": null, "after": null}})
}

fn read_call(revision: &str) -> Value {
    json!({"name": "read_patient_sources", "arguments": {
        "references": [{"candidate_id": "synthetic-charts/Observation/sodium", "revision": revision, "cursor": null}],
        "max_result_bytes": 4096}})
}

fn last(transcript: &[Exchange]) -> &Exchange {
    transcript.last().expect("an exchange")
}

/// The exchange's steps; every saved call in these tests reads as a step.
fn shown(exchange: &Exchange) -> &[Step] {
    match &exchange.steps {
        ExchangeSteps::Read(steps) => steps,
        ExchangeSteps::Unavailable => panic!("the steps were unavailable: {exchange:#?}"),
    }
}

fn search_state(step: &Step) -> Option<&StepState<Found>> {
    match step {
        Step::SearchedChart { state, .. } => Some(state),
        Step::ReadRecords { .. } | Step::ReadConversation { .. } => None,
    }
}

fn read_state(step: &Step) -> Option<&StepState<Vec<RecordRead>>> {
    match step {
        Step::ReadRecords { state, .. } => Some(state),
        Step::SearchedChart { .. } | Step::ReadConversation { .. } => None,
    }
}

/// The read of the sodium result, as its receipt names it.
fn sodium_read() -> StepState<Vec<RecordRead>> {
    StepState::Returned {
        result: vec![RecordRead::Whole {
            version: SourceVersion {
                candidate_id: "synthetic-charts/Observation/sodium".into(),
                revision: sodium().revision,
            },
            about: RecordAbout {
                group: Some(RecordGroup::Results),
                title: Some("Sodium".into()),
                dates: vec![physician_wire::RecordDate {
                    role: Some(physician_wire::DateRole::Effective),
                    value: "2026-10-01".into(),
                }],
            },
        }],
    }
}

/// S1, S2, S7 and S8. A skill read and a working note show no step. A chart
/// search shows as running while it waits, and the open page hears of it
/// while the answer is still being written. It ends, the page hears again,
/// and the read that follows shows as running after it. When the answer is
/// published the steps are unchanged, Today reads as it did before the
/// message, and the steps read the same after the site and the worker restart.
#[tokio::test]
async fn steps_show_live_in_start_order_and_stay_with_the_answer() {
    let rounds = json!([
        [
            {"name": "skill_view", "arguments": {"name": "patient-record-retrieval", "file_path": null}},
            {"name": "save_working_note", "arguments": {"text": "Check the latest sodium.", "fresh_window": false}},
            search_call()
        ],
        [read_call("{revision}")]
    ]);
    let Some(mut stack) = Stack::start("live_steps", 1500, &rounds).await else {
        return;
    };
    let mut events = stack.events().await;
    let today_before = stack.today().await;
    let hold = stack.hold_chart().await;
    stack.send("What is her latest sodium?").await;

    let searching = stack
        .wait("the search never showed as running", |transcript| {
            transcript.last().is_some_and(|exchange| {
                shown(exchange).first().and_then(search_state) == Some(&StepState::Running)
            })
        })
        .await;
    let exchange = last(&searching);
    assert!(
        matches!(exchange.answer, AnswerState::Working { .. }),
        "{exchange:#?}"
    );
    assert_eq!(
        shown(exchange).len(),
        1,
        "the skill read and the working note show no step: {exchange:#?}"
    );
    assert_eq!(
        shown(exchange)[0],
        Step::SearchedChart {
            query: QUERY.into(),
            group: None,
            page: SearchPage::First,
            state: StepState::Running,
        }
    );
    events
        .wait_for("event: workspace_changed", Duration::from_secs(10))
        .await
        .unwrap_or_else(|error| {
            panic!(
                "the page never heard the search start: {error}\n{}",
                stack.logs()
            )
        });
    assert!(
        matches!(
            last(&stack.transcript().await).answer,
            AnswerState::Working { .. }
        ),
        "the step notice came while the answer was still being written"
    );

    // The search returns. The page hears of it before the next provider
    // response, and the read that follows waits on the chart in turn.
    events.buffer.clear();
    hold.rollback().await.expect("release the chart");
    events
        .wait_for("event: workspace_changed", Duration::from_secs(10))
        .await
        .unwrap_or_else(|error| {
            panic!(
                "the page never heard the search end: {error}\n{}",
                stack.logs()
            )
        });
    let searched = stack.transcript().await;
    assert!(
        matches!(last(&searched).answer, AnswerState::Working { .. }),
        "{searched:#?}"
    );
    assert!(
        matches!(
            shown(last(&searched))[0],
            Step::SearchedChart {
                state: StepState::Returned { .. },
                ..
            }
        ),
        "{searched:#?}"
    );
    let hold = stack.hold_chart().await;
    let reading = stack
        .wait("the read never showed as running", |transcript| {
            transcript.last().is_some_and(|exchange| {
                shown(exchange).get(1).and_then(read_state) == Some(&StepState::Running)
            })
        })
        .await;
    let steps = shown(last(&reading));
    assert_eq!(steps.len(), 2, "{steps:#?}");
    assert!(
        matches!(
            search_state(&steps[0]),
            Some(StepState::Returned {
                result: Found {
                    records: 1,
                    more: false
                }
            })
        ),
        "steps stay in start order: {steps:#?}"
    );
    assert_eq!(
        steps[1],
        Step::ReadRecords {
            requested: 1,
            state: StepState::Running,
        }
    );
    hold.rollback().await.expect("release the chart");

    let answered = stack
        .wait("the answer never arrived", |transcript| {
            matches!(
                transcript.last().map(|exchange| &exchange.answer),
                Some(AnswerState::Finished { .. })
            )
        })
        .await;
    let exchange = last(&answered);
    let AnswerState::Finished { text, .. } = &exchange.answer else {
        unreachable!("waited for the answer")
    };
    assert_eq!(text, support::FIXED_TEXT);
    assert_eq!(shown(exchange).len(), 2, "{exchange:#?}");
    assert_eq!(shown(exchange)[0], shown(&searched[0])[0]);
    assert_eq!(read_state(&shown(exchange)[1]), Some(&sodium_read()));
    assert_eq!(
        stack.today().await,
        today_before,
        "the exchange, its chart search and its read changed Today"
    );

    stack.worker.restart();
    support::wait_ready(&mut stack.worker, stack.worker_port).await;
    stack.site.restart();
    support::wait_ready(&mut stack.site, stack.site_port).await;
    assert_eq!(
        stack.transcript().await,
        answered,
        "a restart changed the transcript"
    );
}

/// S3 and the event pair Stop while a tool runs. Stop holds the work: the
/// answer is stopped with Resume, and the read that never returned is
/// stopped, not failed. Resume continues the same exchange to its answer.
#[tokio::test]
async fn stop_while_a_read_runs_marks_it_stopped_and_resume_continues_the_exchange() {
    let Some(stack) = Stack::start("stop_step", 0, &json!([[read_call("{revision}")]])).await
    else {
        return;
    };
    let hold = stack.hold_chart().await;
    stack.send("What is her latest sodium?").await;
    let reading = stack
        .wait("the read never showed as running", |transcript| {
            transcript.last().is_some_and(|exchange| {
                shown(exchange).first().and_then(read_state) == Some(&StepState::Running)
            })
        })
        .await;
    let work = last(&reading).work_item_id.0.clone();
    stack
        .command(
            "work.stop",
            &format!("stop-{work}"),
            json!({"work_item_id": work}),
        )
        .await;
    let stopped = stack
        .wait("Stop never held the work", |transcript| {
            matches!(
                transcript.last().map(|exchange| &exchange.answer),
                Some(AnswerState::Stopped { .. })
            )
        })
        .await;
    let AnswerState::Stopped {
        resume:
            Some(WorkResumeTarget {
                work_item_id,
                expected_run_id,
            }),
    } = &last(&stopped).answer
    else {
        panic!("a stopped answer offers Resume: {stopped:#?}")
    };
    assert_eq!(work_item_id.as_str(), work);
    assert_eq!(
        read_state(&shown(last(&stopped))[0]),
        Some(&StepState::Stopped)
    );
    // The worker checks every second whether its attempt is still wanted, and
    // drops the waiting read once it sees Stop. Release the chart only after
    // that, so the read never returns.
    tokio::time::sleep(Duration::from_secs(2)).await;
    hold.rollback().await.expect("release the chart");

    stack
        .command(
            "work.resume",
            &format!("resume-{work}-{expected_run_id}"),
            json!({"work_item_id": work, "expected_run_id": expected_run_id}),
        )
        .await;
    let resumed = stack.transcript().await;
    assert_eq!(resumed.len(), 1, "Resume continues the same exchange");
    assert!(
        matches!(
            resumed[0].answer,
            AnswerState::Working { .. } | AnswerState::Finished { .. }
        ),
        "{resumed:#?}"
    );
    let answered = stack
        .wait("the resumed answer never arrived", |transcript| {
            matches!(
                transcript.last().map(|exchange| &exchange.answer),
                Some(AnswerState::Finished { .. })
            )
        })
        .await;
    assert_eq!(answered.len(), 1, "{answered:#?}");
    assert_eq!(
        read_state(&shown(&answered[0])[0]),
        Some(&StepState::Stopped),
        "the stopped read keeps its mark: {answered:#?}"
    );
}

/// S6 and the event pair a steer while a tool runs. The second message joins
/// the exchange, the read still ends with its result, and one answer covers
/// both messages.
#[tokio::test]
async fn a_steer_while_a_read_runs_joins_the_exchange_and_one_answer_covers_it() {
    let Some(stack) = Stack::start("steer_step", 0, &json!([[read_call("{revision}")]])).await
    else {
        return;
    };
    let hold = stack.hold_chart().await;
    stack.send("What is her latest sodium?").await;
    stack
        .wait("the read never showed as running", |transcript| {
            transcript.last().is_some_and(|exchange| {
                shown(exchange).first().and_then(read_state) == Some(&StepState::Running)
            })
        })
        .await;
    stack.send("And is it trending?").await;
    let steered = stack.transcript().await;
    assert_eq!(steered.len(), 1, "a steer joins the exchange: {steered:#?}");
    let texts: Vec<_> = steered[0]
        .messages
        .iter()
        .map(|message| message.text.as_str())
        .collect();
    assert_eq!(texts, ["What is her latest sodium?", "And is it trending?"]);
    hold.rollback().await.expect("release the chart");
    let answered = stack
        .wait("the steered answer never arrived", |transcript| {
            matches!(
                transcript.last().map(|exchange| &exchange.answer),
                Some(AnswerState::Finished { .. })
            )
        })
        .await;
    assert_eq!(answered.len(), 1, "one answer covers both: {answered:#?}");
    assert_eq!(answered[0].messages.len(), 2);
    assert_eq!(
        read_state(&shown(&answered[0])[0]),
        Some(&sodium_read()),
        "the read still ends with its result: {answered:#?}"
    );
}

/// The event pair a restart while a tool runs. The worker is stopped while a
/// read waits, so the read never returns; the work continues on a new worker
/// to its answer, and the read shows that how it ended is not known.
#[tokio::test]
async fn a_read_cut_by_a_restart_reads_as_interrupted_and_the_answer_continues() {
    let Some(mut stack) =
        Stack::start("restart_step", 0, &json!([[read_call("{revision}")]])).await
    else {
        return;
    };
    let hold = stack.hold_chart().await;
    stack.send("What is her latest sodium?").await;
    stack
        .wait("the read never showed as running", |transcript| {
            transcript.last().is_some_and(|exchange| {
                shown(exchange).first().and_then(read_state) == Some(&StepState::Running)
            })
        })
        .await;
    stack.worker.kill_now();
    hold.rollback().await.expect("release the chart");
    stack.worker.restart();
    support::wait_ready(&mut stack.worker, stack.worker_port).await;
    let answered = stack
        .wait("the answer never arrived after the restart", |transcript| {
            matches!(
                transcript.last().map(|exchange| &exchange.answer),
                Some(AnswerState::Finished { .. })
            )
        })
        .await;
    assert_eq!(answered.len(), 1, "{answered:#?}");
    assert_eq!(
        read_state(&shown(&answered[0])[0]),
        Some(&StepState::Interrupted),
        "{answered:#?}"
    );
}

/// A read of records from sources that are not FHIR names each by what that source's adapter
/// filed: its group, its whole title and its dates as the source wrote them. The records are
/// stored as each source's delivery, as a connected source's would be: the made-up portal's
/// intake sheet, filed through its table, and a social history entry and a visit note captured
/// from the Hint EHR and filed by the Hint filer.
#[tokio::test]
async fn a_read_names_records_from_sources_that_are_not_fhir() {
    let Some(stack) = Stack::start("second_source_step", 0, &json!([])).await else {
        return;
    };
    let store = Store::connect(&stack.database.url)
        .await
        .expect("runtime store");
    let patient = store
        .source_pulls(&[workspace().to_owned()])
        .await
        .expect("the chart's pulls")
        .remove(workspace())
        .and_then(|pulls| pulls.into_iter().next())
        .expect("the chart patient's pull")
        .patient;
    let portal = &northwind::NORTHWIND_PORTAL_TABLE;
    let sheet = file_json_record(
        portal,
        workspace(),
        portal.source_id,
        northwind::INTAKE_SHEET,
        None,
    )
    .expect("the adapter files the record");
    store
        .record_chart_with_receipt(
            workspace(),
            std::slice::from_ref(&sheet),
            &local_product::runtime::sources::SourcePull {
                source_id: portal.source_id.to_owned(),
                source_label: portal.source_id.to_owned(),
                patient,
                pulled_at: Some(1_787_846_400),
            },
        )
        .await
        .expect("the pull");
    let captured = hint::filed(workspace());
    hint::deliver(
        &store,
        physician_sample::STAGING_PRACTICE_ID,
        workspace(),
        &captured,
    )
    .await;
    let (mut references, mut versions) = (Vec::new(), Vec::new());
    for records in [
        vec![sheet],
        vec![
            captured.record(hint::SOCIAL_HISTORY).clone(),
            captured.record(hint::NOTE).clone(),
        ],
    ] {
        references.extend(records.iter().map(|record| {
            json!({"candidate_id": record.candidate_id, "revision": record.revision, "cursor": null})
        }));
        versions.extend(records.iter().map(|record| SourceVersion {
            candidate_id: record.candidate_id.clone(),
            revision: record.revision.clone(),
        }));
    }
    stack
        .stub
        .tool_rounds(&json!([[{"name": "read_patient_sources", "arguments": {
        "references": references, "max_result_bytes": 16384}}]]));
    stack
        .send("What did her intake sheet, her social history and her visit note say?")
        .await;
    let answered = stack
        .wait("the answer never arrived", |transcript| {
            matches!(
                transcript.last().map(|exchange| &exchange.answer),
                Some(AnswerState::Finished { .. })
            )
        })
        .await;
    let about = |version: &SourceVersion,
                 group,
                 title: &str,
                 dates: &[(physician_wire::DateRole, &str)]| RecordRead::Whole {
        version: version.clone(),
        about: RecordAbout {
            group: Some(group),
            title: Some(title.into()),
            dates: dates
                .iter()
                .map(|(role, value)| physician_wire::RecordDate {
                    role: Some(*role),
                    value: (*value).into(),
                })
                .collect(),
        },
    };
    assert_eq!(
        shown(last(&answered)),
        [Step::ReadRecords {
            requested: 3,
            state: StepState::Returned {
                result: vec![
                    about(
                        &versions[0],
                        RecordGroup::Forms,
                        "Pre-visit symptom check, cardiology clinic, after emergency department visit, with home medication list",
                        &[(
                            physician_wire::DateRole::Recorded,
                            "2026-03-22T08:45:00-05:00"
                        )],
                    ),
                    about(
                        &versions[1],
                        RecordGroup::History,
                        "Tobacco use",
                        &[(physician_wire::DateRole::Start, "2020")],
                    ),
                    about(
                        &versions[2],
                        RecordGroup::Notes,
                        "Annual wellness visit",
                        &[
                            (physician_wire::DateRole::Occurred, "2026-10-12"),
                            (physician_wire::DateRole::Recorded, "2026-10-12T17:00:00Z"),
                        ],
                    ),
                ],
            },
        }],
        "{answered:#?}"
    );
}

/// S5. A provider block leaves Coda unable to finish, with Try again on the
/// run it held. A limit keeps the answer Coda had written and offers Try
/// again: here the run spends its whole request budget on searches, and the
/// final request's answer is kept as a partial.
#[tokio::test]
async fn a_provider_block_and_a_limit_each_end_with_try_again() {
    let Some(stack) = Stack::start("endings", 0, &json!([])).await else {
        return;
    };
    stack.stub.reject_next(1);
    stack.send("What is her latest sodium?").await;
    let blocked = stack
        .wait("the provider block never ended the work", |transcript| {
            matches!(
                transcript.last().map(|exchange| &exchange.answer),
                Some(AnswerState::CouldntFinish { .. })
            )
        })
        .await;
    let work = last(&blocked).work_item_id.0.clone();
    assert!(
        matches!(
            &last(&blocked).answer,
            AnswerState::CouldntFinish { try_again: Some(target) } if target.work_item_id.as_str() == work
        ),
        "{blocked:#?}"
    );

    // One request short of the budget in searches; the next is the final one.
    let budget = harness::engine::RunLimits::default().max_calls;
    let searches: Vec<_> = (1..budget).map(|_| json!([search_call()])).collect();
    stack.stub.tool_rounds(&json!(searches));
    stack.send("Search everything first.").await;
    let limited = stack
        .wait("the run never reached its limit", |transcript| {
            transcript.len() == 2 && matches!(transcript[1].answer, AnswerState::CutOff { .. })
        })
        .await;
    assert_eq!(
        limited[0].answer,
        AnswerState::CouldntFinish { try_again: None },
        "the earlier answer keeps its mark without Try again"
    );
    let AnswerState::CutOff { try_again } = &limited[1].answer else {
        unreachable!("waited for a limit")
    };
    assert_eq!(limited[1].kept, [support::FIXED_TEXT]);
    assert!(limited[0].kept.is_empty(), "{limited:#?}");
    assert_eq!(
        try_again
            .as_ref()
            .map(|target| target.work_item_id.as_str()),
        Some(limited[1].work_item_id.as_str())
    );
    assert!(
        shown(&limited[1])
            .iter()
            .all(|step| matches!(search_state(step), Some(StepState::Returned { .. }))),
        "{:#?}",
        shown(&limited[1])
    );
}

/// Text a limit kept stays with its exchange. Try again starts a new run that
/// continues after the kept text, so the kept text shows before the new
/// run's answer while that run works and after it finishes.
#[tokio::test]
async fn text_a_limit_kept_stays_before_the_answer_try_again_finishes() {
    let budget = harness::engine::RunLimits::default().max_calls;
    let searches: Vec<_> = (1..budget).map(|_| json!([search_call()])).collect();
    let Some(stack) = Stack::start("kept", 0, &json!(searches)).await else {
        return;
    };
    const KEPT: &str = "The first part.";
    const REST: &str = "The rest.";
    const QUESTION: &str = "Search everything first.";
    stack.stub.reply(KEPT);
    stack.send(QUESTION).await;
    let limited = stack
        .wait("the run never reached its limit", |transcript| {
            matches!(
                transcript.last().map(|exchange| &exchange.answer),
                Some(AnswerState::CutOff { .. })
            )
        })
        .await;
    let AnswerState::CutOff {
        try_again: Some(target),
        ..
    } = &last(&limited).answer
    else {
        panic!("a cut-off answer offers Try again: {limited:#?}")
    };
    assert_eq!(
        support::chat_texts(&stack.snapshot().await),
        [QUESTION, KEPT],
        "the cut-off answer shows what it kept"
    );

    // The new run searches once more, held, and then answers.
    let mut rounds = searches.clone();
    rounds.push(json!([search_call()]));
    stack.stub.tool_rounds(&json!(rounds));
    stack.stub.reply(REST);
    let hold = stack.hold_chart().await;
    stack
        .command(
            "work.resume",
            &format!(
                "resume-{}-{}",
                target.work_item_id.as_str(),
                target.expected_run_id
            ),
            json!({"work_item_id": target.work_item_id, "expected_run_id": target.expected_run_id}),
        )
        .await;
    stack
        .wait("Try again never started a held search", |transcript| {
            transcript.last().is_some_and(|exchange| {
                matches!(exchange.answer, AnswerState::Working { .. })
                    && shown(exchange)
                        .iter()
                        .any(|step| search_state(step) == Some(&StepState::Running))
            })
        })
        .await;
    assert_eq!(
        support::chat_texts(&stack.snapshot().await),
        [QUESTION, KEPT],
        "while the new run works, the kept text still shows"
    );
    hold.rollback().await.expect("release the chart");
    let finished = stack
        .wait("the new run never finished", |transcript| {
            matches!(
                transcript.last().map(|exchange| &exchange.answer),
                Some(AnswerState::Finished { .. })
            )
        })
        .await;
    assert_eq!(finished.len(), 1, "Try again continues the same exchange");
    assert_eq!(finished[0].kept, [KEPT]);
    assert!(
        matches!(&finished[0].answer, AnswerState::Finished { text, .. } if text == REST),
        "{finished:#?}"
    );
    assert_eq!(
        support::chat_texts(&stack.snapshot().await),
        [QUESTION, KEPT, REST],
        "the kept text shows before the answer that continues it"
    );
}
