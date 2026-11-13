//! A conversation's transcript as the page reads it, over real processes.
//!
//! One site and one worker share a disposable Postgres and Valkey, and a
//! scripted provider asks for chart searches and reads. A test holds a chart
//! read in flight by locking the patient's source version row from its own
//! connection: the read waits on that lock as it would on a slow chart, and
//! returns when the test lets go.

use std::path::PathBuf;
use std::time::{Duration, Instant};

use local_product::physician_sources::{RecordDate, SourceRevision};
use local_product::runtime::Store;
use physician_wire::{AnswerState, Exchange, Found, RecordRead, Step, StepState, WorkResumeTarget};
use sea_orm::{ConnectionTrait, DatabaseTransaction, Statement, TransactionTrait};
use serde_json::{Value, json};

#[path = "scale/mod.rs"]
mod support;

/// The chart record the scripted provider searches for and reads. Its
/// marker word matches no fixture record, so a search finds only it.
const SODIUM: &str =
    r#"{"code":"2951-2","name":"sodium","value":139,"unit":"mmol/L","marker":"transcriptproof"}"#;
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
        let revision = local_product::sha256_hex(SODIUM.as_bytes());
        Store::connect(&database.url)
            .await
            .expect("runtime store")
            .record_source(&SourceRevision {
                workspace: workspace().into(),
                source_id: "synthetic-charts".into(),
                candidate_id: "Observation/sodium".into(),
                revision: revision.clone(),
                record_type: "Observation".into(),
                dates: vec![RecordDate {
                    name: "effective".into(),
                    value: "2026-10-01".into(),
                }],
                content: SODIUM.into(),
                sha256: revision.clone(),
                readable_parts: vec![],
                provenance: json!({"native_record": "Observation/sodium"}),
                envelope: None,
            })
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
        let db = sea_orm::Database::connect(&self.database.url)
            .await
            .expect("hold connection");
        let hold = db.begin().await.expect("hold transaction");
        hold.execute_raw(Statement::from_sql_and_values(
            sea_orm::DatabaseBackend::Postgres,
            "SELECT version FROM physician_runtime_source_versions WHERE workspace = $1 FOR UPDATE",
            [workspace().into()],
        ))
        .await
        .expect("hold the chart");
        hold
    }
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
        "references": [{"candidate_id": "Observation/sodium", "revision": revision, "cursor": null}],
        "max_result_bytes": 4096}})
}

fn last(transcript: &[Exchange]) -> &Exchange {
    transcript.last().expect("an exchange")
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
            record_type: "Observation".into(),
            dates: vec![physician_wire::RecordDate {
                name: "effective".into(),
                value: "2026-10-01".into(),
            }],
        }],
    }
}

/// S1, S2, S7 and S8. A skill read and a working note show no step. A chart
/// search shows as running while it waits, and the open page hears of it
/// while the answer is still being written. It ends, the page hears again,
/// and the read that follows shows as running after it. When the answer is
/// published the steps are unchanged, and they read the same after the site
/// and the worker restart.
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
    let hold = stack.hold_chart().await;
    stack.send("What is her latest sodium?").await;

    let searching = stack
        .wait("the search never showed as running", |transcript| {
            transcript.last().is_some_and(|exchange| {
                exchange.steps.first().and_then(search_state) == Some(&StepState::Running)
            })
        })
        .await;
    let exchange = last(&searching);
    assert!(
        matches!(exchange.answer, AnswerState::Working { .. }),
        "{exchange:#?}"
    );
    assert_eq!(
        exchange.steps.len(),
        1,
        "the skill read and the working note show no step: {exchange:#?}"
    );
    assert_eq!(
        exchange.steps[0],
        Step::SearchedChart {
            query: QUERY.into(),
            record_type: None,
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
            last(&searched).steps[0],
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
                exchange.steps.get(1).and_then(read_state) == Some(&StepState::Running)
            })
        })
        .await;
    let steps = &last(&reading).steps;
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
    assert_eq!(exchange.steps.len(), 2, "{exchange:#?}");
    assert_eq!(exchange.steps[0], searched[0].steps[0]);
    assert_eq!(read_state(&exchange.steps[1]), Some(&sodium_read()));

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
                exchange.steps.first().and_then(read_state) == Some(&StepState::Running)
            })
        })
        .await;
    let work = last(&reading).work_item_id.clone();
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
    assert_eq!(work_item_id, &work);
    assert_eq!(
        read_state(&last(&stopped).steps[0]),
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
        read_state(&answered[0].steps[0]),
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
                exchange.steps.first().and_then(read_state) == Some(&StepState::Running)
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
        read_state(&answered[0].steps[0]),
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
                exchange.steps.first().and_then(read_state) == Some(&StepState::Running)
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
        read_state(&answered[0].steps[0]),
        Some(&StepState::Interrupted),
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
    let work = last(&blocked).work_item_id.clone();
    assert!(
        matches!(
            &last(&blocked).answer,
            AnswerState::CouldntFinish { try_again: Some(target) } if target.work_item_id == work
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
        limited[1]
            .steps
            .iter()
            .all(|step| matches!(search_state(step), Some(StepState::Returned { .. }))),
        "{:#?}",
        limited[1].steps
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
            &format!("resume-{}-{}", target.work_item_id, target.expected_run_id),
            json!({"work_item_id": target.work_item_id, "expected_run_id": target.expected_run_id}),
        )
        .await;
    stack
        .wait("Try again never started a held search", |transcript| {
            transcript.last().is_some_and(|exchange| {
                matches!(exchange.answer, AnswerState::Working { .. })
                    && exchange
                        .steps
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
