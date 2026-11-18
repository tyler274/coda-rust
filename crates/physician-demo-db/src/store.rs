//! Typed `sea-query` repository backing all physician UI reads and interactive demo commands.

use crate::driver::{DbError, SqlRow, SqliteDriver};
use crate::schema::{
    Chats, Drafts, Patients, PracticeVisits, SourceRecords, TodayItems, UpdatesFeed, schema_ddl,
};
use physician_wire::{
    AnswerState, AttemptId, ChatSnapshot, ChatSnapshotResponse, DateRole, Exchange, ExchangeSteps,
    Found, HealthCapabilities, HealthResponse, MessageKey, PatientSearchResponse, PatientSearchRow,
    PatientWorkspaceResponse, PatientWorkspaceSummary, PatientWorkspacesResponse, PhysicianDraft,
    PhysicianMessage, PracticeAccountSummary, PracticeDayResponse, PracticeSourceState,
    RecentConversation, RecentConversationsResponse, RecordAbout, RecordDate, RecordGroup,
    RecordRead, ReplyDraftWire, ResponseFeedback, ResponseId, SaveFeedback, SavedAt, SearchPage,
    SnapshotFeedback, SourceVersion, Step, StepState, TodayConnection, TodayListResponse,
    UpdatesResponse, WhoamiResponse, WorkItemId, WorkspaceChat, WorkspacePlanItem,
};
use sea_query::{Cond, Expr, ExprTrait, OnConflict, Order, Query, SelectStatement};
use std::sync::atomic::{AtomicU64, Ordering};

const DEMO_DATE: &str = "2026-08-27";
pub(crate) const DEMO_NOW: &str = "2026-08-27T19:00:00.000Z";
pub(crate) const DEMO_ZONE: &str = "America/New_York";
const DEMO_BASE_UNIX: i64 = 1_787_857_200;
static SEQUENCE: AtomicU64 = AtomicU64::new(1);

fn next_seq() -> u64 {
    SEQUENCE.fetch_add(1, Ordering::Relaxed)
}

/// One source record retrieved from SQLite for chart inspection.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DemoSourceRecord {
    /// Record identifier within the patient workspace.
    pub id: String,
    /// Patient workspace owning the record.
    pub workspace_id: String,
    /// Shared record envelope group label.
    pub group: String,
    /// Clinical record title.
    pub title: String,
    /// Recorded date string.
    pub recorded_at: String,
    /// Full clinical record body text.
    pub body: String,
    /// Source label shown in inspection provenance.
    pub source_label: String,
}

/// Result of attempting an optimistic-concurrency composer draft save.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DraftSaveOutcome {
    /// Draft saved at the returned revision.
    Saved(PhysicianDraft),
    /// A newer revision already existed; nothing was overwritten.
    Conflict(PhysicianDraft),
}

/// SQLite-backed store powering the interactive browser demo and native host tests.
#[derive(Debug, Clone)]
pub struct DemoStore {
    driver: SqliteDriver,
}

impl DemoStore {
    /// Wraps a configured SQLite driver.
    pub fn new(driver: SqliteDriver) -> Self {
        Self { driver }
    }

    /// Initializes the SQLite tables on a newly opened native database.
    pub fn init_schema(&self) -> Result<(), DbError> {
        self.driver.exec_ddl(&schema_ddl())
    }

    /// Exposes the underlying driver for seeding helpers.
    pub fn driver(&self) -> &SqliteDriver {
        &self.driver
    }

    /// Ensures the browser SQLite engine has loaded the seed snapshot.
    pub async fn ensure_ready(&self) -> Result<(), DbError> {
        self.driver.ensure_ready().await
    }

    /// Restores the demo database to its pristine seeded snapshot.
    pub async fn reset(&self) -> Result<(), DbError> {
        self.driver.reset().await
    }

    async fn map_select<T>(
        &self,
        q: &SelectStatement,
        f: impl Fn(&SqlRow) -> Result<T, DbError>,
    ) -> Result<Vec<T>, DbError> {
        self.ensure_ready().await?;
        self.driver.select(q)?.iter().map(f).collect()
    }

    /// Returns static demo health metadata.
    pub fn health(&self) -> HealthResponse {
        HealthResponse {
            status: "ok".into(),
            retrying: Vec::new(),
            held_by_newer_release: Vec::new(),
            capabilities: Some(HealthCapabilities {
                connection_accounts: false,
            }),
        }
    }

    /// Returns the synthetic signed-in demo physician.
    pub fn whoami(&self) -> WhoamiResponse {
        let acct = PracticeAccountSummary {
            physician_id: "phys-demo-hannah".into(),
            display_name: "Dr. Hannah Alvarez".into(),
            email: "hannah@codahealth.example".into(),
            admin: true,
        };
        WhoamiResponse {
            physician_id: acct.physician_id.clone(),
            practice_id: "prac-coda-demo".into(),
            email: acct.email.clone(),
            display_name: acct.display_name.clone(),
            admin: true,
            practice_accounts: vec![acct],
        }
    }

    /// Queries active items for the Today list.
    pub async fn today(&self) -> Result<TodayListResponse, DbError> {
        self.today_filtered(None).await
    }

    /// Queries active Today items for one patient workspace.
    pub async fn today_for(&self, workspace_id: &str) -> Result<TodayListResponse, DbError> {
        self.today_filtered(Some(workspace_id)).await
    }

    async fn today_filtered(&self, ws: Option<&str>) -> Result<TodayListResponse, DbError> {
        let mut q = Query::select();
        q.column(TodayItems::RowJson)
            .from(TodayItems::Table)
            .and_where(Expr::col(TodayItems::Done).eq(0));
        if let Some(id) = ws {
            q.and_where(Expr::col(TodayItems::WorkspaceId).eq(id));
        }
        q.order_by(TodayItems::Urgency, Order::Asc)
            .order_by(TodayItems::WaitingSince, Order::Desc);
        let rows = self
            .map_select(&q, |r| Ok(serde_json::from_str(&r.get_str("row_json")?)?))
            .await?;
        let connections = ["spruce", "gmail"]
            .into_iter()
            .map(|c| TodayConnection {
                condition: "connected".into(),
                channel: Some(c.into()),
            })
            .collect();
        Ok(TodayListResponse {
            date: DEMO_DATE.into(),
            rows,
            as_of: 1,
            connections,
            incomplete: Some(false),
        })
    }

    /// Lists all patient workspaces in the demo database.
    pub async fn workspaces(&self) -> Result<PatientWorkspacesResponse, DbError> {
        let q = Query::select()
            .columns([Patients::WorkspaceId, Patients::DisplayName])
            .from(Patients::Table)
            .order_by(Patients::DisplayName, Order::Asc)
            .to_owned();
        let patients = self
            .map_select(&q, |r| {
                Ok(PatientWorkspaceSummary {
                    patient_workspace_id: r.get_str("workspace_id")?,
                    display_name: r.get_str("display_name")?,
                    cursor: 1,
                })
            })
            .await?;
        Ok(PatientWorkspacesResponse {
            patients,
            incomplete: Some(false),
        })
    }

    /// Queries the scheduled clinic visits for the Schedule rail.
    pub async fn practice_day(&self) -> Result<PracticeDayResponse, DbError> {
        let q = Query::select()
            .column(PracticeVisits::PayloadJson)
            .from(PracticeVisits::Table)
            .order_by(PracticeVisits::StartsAt, Order::Asc)
            .to_owned();
        let visits = self
            .map_select(&q, |r| {
                Ok(serde_json::from_str(&r.get_str("payload_json")?)?)
            })
            .await?;
        Ok(PracticeDayResponse {
            current_time: DEMO_NOW.into(),
            time_zone: DEMO_ZONE.into(),
            source_state: PracticeSourceState {
                state: "usable".into(),
            },
            visits,
        })
    }

    /// Queries recent conversations across seeded patients.
    pub async fn recent_conversations(&self) -> Result<RecentConversationsResponse, DbError> {
        let join = Expr::col((Chats::Table, Chats::WorkspaceId))
            .equals((Patients::Table, Patients::WorkspaceId));
        let q = Query::select()
            .column((Chats::Table, Chats::WorkspaceId))
            .column((Chats::Table, Chats::Title))
            .column((Patients::Table, Patients::DisplayName))
            .from(Chats::Table)
            .inner_join(Patients::Table, join)
            .order_by((Chats::Table, Chats::CreatedAt), Order::Desc)
            .limit(8)
            .to_owned();
        let conversations = self
            .map_select(&q, |r| {
                Ok(RecentConversation {
                    patient_workspace_id: Some(r.get_str("workspace_id")?),
                    title: Some(r.get_str("title")?),
                    patient_display_name: Some(r.get_str("display_name")?),
                })
            })
            .await?;
        Ok(RecentConversationsResponse { conversations })
    }

    /// Queries the Updates feed.
    pub async fn updates(&self) -> Result<UpdatesResponse, DbError> {
        let q = Query::select()
            .column(UpdatesFeed::PayloadJson)
            .from(UpdatesFeed::Table)
            .order_by(UpdatesFeed::CreatedAt, Order::Desc)
            .to_owned();
        let rows = self
            .map_select(&q, |r| {
                Ok(serde_json::from_str(&r.get_str("payload_json")?)?)
            })
            .await?;
        Ok(UpdatesResponse { rows })
    }

    /// Searches patients by name, MRN, or birth date using SQL `LIKE`.
    pub async fn search(&self, query_text: &str) -> Result<PatientSearchResponse, DbError> {
        let trimmed = query_text.trim();
        let pat = format!("%{trimmed}%");
        let mut stmt = Query::select();
        stmt.columns([
            Patients::WorkspaceId,
            Patients::DisplayName,
            Patients::BirthDate,
        ])
        .from(Patients::Table);
        if !trimmed.is_empty() {
            stmt.cond_where(
                Cond::any()
                    .add(Expr::col(Patients::DisplayName).like(&pat))
                    .add(Expr::col(Patients::Mrn).like(&pat))
                    .add(Expr::col(Patients::BirthDate).like(&pat)),
            );
        }
        stmt.order_by(Patients::DisplayName, Order::Asc).limit(25);
        let patients = self
            .map_select(&stmt, |r| {
                Ok(PatientSearchRow {
                    patient_workspace_id: r.get_str("workspace_id")?,
                    patient_display_name: r.get_str("display_name")?,
                    birth_date: Some(r.get_str("birth_date")?),
                    duet_linked: true,
                    sharing_consented: true,
                    cadence: Some("daily_when_changed".into()),
                })
            })
            .await?;
        Ok(PatientSearchResponse { patients })
    }

    /// Reads the complete workspace projection for `workspace_id`.
    pub async fn workspace(&self, workspace_id: &str) -> Result<PatientWorkspaceResponse, DbError> {
        self.ensure_ready().await?;
        let rows = self.driver.select_where(
            Patients::Table,
            &[Patients::WorkspaceJson],
            &[(Patients::WorkspaceId, workspace_id)],
        )?;
        let row = rows
            .first()
            .ok_or_else(|| not_found("patient workspace", workspace_id))?;
        let mut ws: PatientWorkspaceResponse =
            serde_json::from_str(&row.get_str("workspace_json")?)?;
        ws.chats = self.load_workspace_chats(workspace_id)?;
        Ok(ws)
    }

    async fn mutate_workspace<T>(
        &self,
        payload: &serde_json::Value,
        f: impl FnOnce(&mut PatientWorkspaceResponse) -> Result<T, DbError>,
    ) -> Result<(String, T), DbError> {
        let ws_id = required_str(payload, "patient_workspace_id")?;
        let mut ws = self.workspace(&ws_id).await?;
        let out = f(&mut ws)?;
        let json = serde_json::to_string(&ws)?;
        self.driver.update_where(
            Patients::Table,
            Patients::WorkspaceJson,
            json.into(),
            &[(Patients::WorkspaceId, ws_id.as_str())],
        )?;
        Ok((ws_id, out))
    }

    /// Queries one conversation transcript snapshot.
    pub async fn chat_snapshot(
        &self,
        ws_id: &str,
        chat_id: &str,
    ) -> Result<ChatSnapshotResponse, DbError> {
        self.ensure_ready().await?;
        let rows = self.driver.select_where(
            Chats::Table,
            &[Chats::SnapshotJson],
            &[(Chats::WorkspaceId, ws_id), (Chats::ChatId, chat_id)],
        )?;
        let snapshot = match rows.first() {
            Some(row) => serde_json::from_str(&row.get_str("snapshot_json")?)?,
            None => ChatSnapshot {
                transcript: Vec::new(),
                feedback: SnapshotFeedback::Loaded,
            },
        };
        Ok(ChatSnapshotResponse { snapshot })
    }

    /// Reads the saved composer draft for `(workspace_id, chat_id)`.
    pub async fn read_draft(&self, ws_id: &str, chat_id: &str) -> Result<PhysicianDraft, DbError> {
        self.ensure_ready().await?;
        let rows = self.driver.select_where(
            Drafts::Table,
            &[Drafts::Revision, Drafts::Text],
            &[(Drafts::WorkspaceId, ws_id), (Drafts::ChatId, chat_id)],
        )?;
        let Some(row) = rows.first() else {
            return Ok(PhysicianDraft {
                text: String::new(),
                revision: 0,
            });
        };
        Ok(PhysicianDraft {
            text: row.get_str("text")?,
            revision: row.get_i64("revision")?,
        })
    }

    /// Saves a composer draft if `expected_revision` matches the stored revision.
    pub async fn save_draft(
        &self,
        ws_id: &str,
        chat_id: &str,
        expected_revision: i64,
        text: &str,
    ) -> Result<DraftSaveOutcome, DbError> {
        self.ensure_ready().await?;
        let current = self.read_draft(ws_id, chat_id).await?;
        if current.revision != expected_revision {
            return Ok(DraftSaveOutcome::Conflict(current));
        }
        let next = expected_revision + 1;
        let conflict = OnConflict::columns([Drafts::WorkspaceId, Drafts::ChatId])
            .update_columns([Drafts::Revision, Drafts::Text])
            .to_owned();
        let insert = Query::insert()
            .into_table(Drafts::Table)
            .columns(Drafts::COLS.iter().copied())
            .values_panic([ws_id.into(), chat_id.into(), next.into(), text.into()])
            .on_conflict(conflict)
            .to_owned();
        self.driver.insert(&insert)?;
        Ok(DraftSaveOutcome::Saved(PhysicianDraft {
            text: text.into(),
            revision: next,
        }))
    }

    /// Persists physician feedback on a response inside a chat snapshot.
    pub async fn save_feedback(
        &self,
        ws_id: &str,
        chat_id: &str,
        response: &ResponseId,
        save: &SaveFeedback,
    ) -> Result<ResponseFeedback, DbError> {
        let mut snap = self.chat_snapshot(ws_id, chat_id).await?.snapshot;
        let mut saved = None;
        for ex in &mut snap.transcript {
            if let AnswerState::Finished {
                response_id,
                feedback,
                ..
            } = &mut ex.answer
                && response_id == response
            {
                let next = ResponseFeedback {
                    revision: feedback.as_ref().map_or(1, |p| p.revision + 1),
                    rating: save.rating,
                    details: save.details.clone(),
                };
                *feedback = Some(next.clone());
                saved = Some(next);
            }
        }
        let out = saved.ok_or_else(|| not_found("response", response.as_str()))?;
        self.driver.update_where(
            Chats::Table,
            Chats::SnapshotJson,
            serde_json::to_string(&snap)?.into(),
            &[(Chats::WorkspaceId, ws_id), (Chats::ChatId, chat_id)],
        )?;
        Ok(out)
    }

    /// Reads one reply draft with its current revision.
    pub async fn read_reply_draft(
        &self,
        ws_id: &str,
        reply_id: &str,
    ) -> Result<ReplyDraftWire, DbError> {
        let ws = self.workspace(ws_id).await?;
        let reply = ws
            .replies
            .iter()
            .find(|r| r.reply_work_id == reply_id)
            .ok_or_else(|| not_found("reply draft", reply_id))?;
        Ok(ReplyDraftWire {
            revision: reply.draft_revision.unwrap_or(1),
            text: reply.draft_text.clone(),
            restore_text: None,
            revision_run: None,
        })
    }

    /// Returns one full chart source record for citation inspection.
    pub async fn source_record(
        &self,
        ws_id: &str,
        record_id: &str,
    ) -> Result<DemoSourceRecord, DbError> {
        self.ensure_ready().await?;
        let rows = self.driver.select_where(
            SourceRecords::Table,
            SourceRecords::COLS,
            &[
                (SourceRecords::WorkspaceId, ws_id),
                (SourceRecords::Id, record_id),
            ],
        )?;
        let row = rows
            .first()
            .ok_or_else(|| not_found("source record", record_id))?;
        decode_source_record(row)
    }

    /// Executes an interactive physician command against the browser SQLite store.
    pub async fn command(&self, body: &serde_json::Value) -> Result<serde_json::Value, DbError> {
        self.ensure_ready().await?;
        let kind = required_str(body, "type")?;
        let payload = body.get("payload").unwrap_or(body);
        match kind.as_str() {
            "message.submit" => self.handle_message_submit(body, payload).await,
            "plan.decide_recommendation" => self.handle_decide_recommendation(payload).await,
            "reply.close" => self.handle_reply_close(payload).await,
            "reply.revise" => self.handle_reply_revise(payload).await,
            "concern.done" | "work.done" => self.handle_item_done(payload),
            "work.stop" | "work.resume" => Ok(serde_json::json!({ "ok": true })),
            _ => Err(DbError::InvalidCommand(format!(
                "unsupported demo command `{kind}`"
            ))),
        }
    }

    async fn handle_message_submit(
        &self,
        envelope: &serde_json::Value,
        payload: &serde_json::Value,
    ) -> Result<serde_json::Value, DbError> {
        let ws_id = required_str(payload, "patient_workspace_id")?;
        let chat_id = required_str(payload, "chat_id")?;
        let question = required_str(payload, "text")?;
        let key = envelope
            .get("idempotency_key")
            .and_then(serde_json::Value::as_str)
            .map(str::to_owned)
            .unwrap_or_else(|| format!("msg-{}", next_seq()));
        let terms = extract_search_terms(&question);
        let matched = self.search_patient_records(&ws_id, &terms)?;
        let mut snap = self.chat_snapshot(&ws_id, &chat_id).await?.snapshot;
        let is_new = snap.transcript.is_empty();
        let seq = next_seq();
        let reads: Vec<_> = matched
            .iter()
            .map(|r| {
                (
                    r.id.clone(),
                    parse_record_group(&r.group),
                    r.title.clone(),
                    r.recorded_at.clone(),
                )
            })
            .collect();
        let ts = DEMO_BASE_UNIX + i64::try_from(seq).unwrap_or(1);
        snap.transcript.push(build_exchange(
            "demo",
            &seq.to_string(),
            (key, question.clone(), ts),
            &terms.join(" "),
            &reads,
            false,
            synthesize_chart_answer(&question, &matched),
        ));
        let title = if is_new {
            truncate_title(&question)
        } else {
            self.existing_chat_title(&ws_id, &chat_id)?
                .unwrap_or_else(|| truncate_title(&question))
        };
        let insert = Query::insert()
            .into_table(Chats::Table)
            .columns(Chats::COLS.iter().copied())
            .values_panic([
                chat_id.as_str().into(),
                ws_id.as_str().into(),
                title.as_str().into(),
                DEMO_NOW.into(),
                serde_json::to_string(&snap)?.as_str().into(),
            ])
            .on_conflict(
                OnConflict::column(Chats::ChatId)
                    .update_column(Chats::SnapshotJson)
                    .to_owned(),
            )
            .to_owned();
        self.driver.insert(&insert)?;
        Ok(serde_json::json!({ "ok": true, "chat_id": chat_id }))
    }

    fn search_patient_records(
        &self,
        ws_id: &str,
        terms: &[String],
    ) -> Result<Vec<DemoSourceRecord>, DbError> {
        for active in [terms, &[][..]] {
            let mut stmt = Query::select();
            stmt.columns(SourceRecords::COLS.iter().copied())
                .from(SourceRecords::Table)
                .and_where(Expr::col(SourceRecords::WorkspaceId).eq(ws_id));
            if !active.is_empty() {
                let mut any_term = Cond::any();
                for term in active {
                    let pat = format!("%{term}%");
                    any_term = any_term
                        .add(Expr::col(SourceRecords::Title).like(&pat))
                        .add(Expr::col(SourceRecords::Body).like(&pat))
                        .add(Expr::col(SourceRecords::RecordGroup).like(&pat));
                }
                stmt.cond_where(any_term);
            }
            stmt.order_by(SourceRecords::RecordedAt, Order::Desc)
                .limit(4);
            let rows = self.driver.select(&stmt)?;
            if !rows.is_empty() || active.is_empty() {
                return rows.iter().map(decode_source_record).collect();
            }
        }
        Ok(Vec::new())
    }

    async fn handle_decide_recommendation(
        &self,
        payload: &serde_json::Value,
    ) -> Result<serde_json::Value, DbError> {
        let rec_id = required_str(payload, "recommendation_id")?;
        self.mutate_workspace(payload, |ws| {
            let pos = ws
                .undecided_recommendations
                .iter()
                .position(|r| r.recommendation_id == rec_id)
                .ok_or_else(|| not_found("recommendation", &rec_id))?;
            let chosen = ws.undecided_recommendations.remove(pos);
            ws.active_plan_items.push(WorkspacePlanItem {
                item_id: format!("plan-{}", next_seq()),
                patient_workspace_id: ws.patient_workspace_id.clone(),
                title: chosen.medication,
                detail: chosen.rationale,
                decided_at: DEMO_NOW.into(),
                source_recommendation_id: Some(chosen.recommendation_id),
            });
            Ok(())
        })
        .await?;
        Ok(serde_json::json!({ "ok": true }))
    }

    async fn handle_reply_close(
        &self,
        payload: &serde_json::Value,
    ) -> Result<serde_json::Value, DbError> {
        let reply_id = required_str(payload, "reply_work_id")?;
        let (ws_id, ()) = self
            .mutate_workspace(payload, |ws| {
                ws.replies.retain(|r| r.reply_work_id != reply_id);
                Ok(())
            })
            .await?;
        self.driver.update_where(
            TodayItems::Table,
            TodayItems::Done,
            1.into(),
            &[
                (TodayItems::WorkspaceId, &ws_id),
                (TodayItems::Kind, "reply"),
            ],
        )?;
        Ok(serde_json::json!({ "ok": true }))
    }

    async fn handle_reply_revise(
        &self,
        payload: &serde_json::Value,
    ) -> Result<serde_json::Value, DbError> {
        let reply_id = required_str(payload, "reply_work_id")?;
        let instruction = required_str(payload, "instruction")?;
        let expected = payload
            .get("expected_draft_revision")
            .and_then(serde_json::Value::as_u64)
            .unwrap_or(1);
        let (_, next_rev) = self
            .mutate_workspace(payload, |ws| {
                let reply = ws
                    .replies
                    .iter_mut()
                    .find(|r| r.reply_work_id == reply_id)
                    .ok_or_else(|| not_found("reply draft", &reply_id))?;
                let rev = reply.draft_revision.unwrap_or(1);
                if rev != expected {
                    return Err(DbError::InvalidCommand(
                        "reply draft revision changed".into(),
                    ));
                }
                let next = rev + 1;
                reply.draft_text = format!(
                    "{}\n\n[Updated per physician note: {}]",
                    reply.draft_text,
                    instruction.trim()
                );
                reply.draft_revision = Some(next);
                Ok(next)
            })
            .await?;
        Ok(serde_json::json!({ "ok": true, "draft_revision": next_rev }))
    }

    fn handle_item_done(&self, payload: &serde_json::Value) -> Result<serde_json::Value, DbError> {
        let item_id = required_str(payload, "item_id")?;
        self.driver.update_where(
            TodayItems::Table,
            TodayItems::Done,
            1.into(),
            &[(TodayItems::ItemId, &item_id)],
        )?;
        Ok(serde_json::json!({ "ok": true }))
    }

    fn existing_chat_title(&self, ws_id: &str, chat_id: &str) -> Result<Option<String>, DbError> {
        let rows = self.driver.select_where(
            Chats::Table,
            &[Chats::Title],
            &[(Chats::WorkspaceId, ws_id), (Chats::ChatId, chat_id)],
        )?;
        rows.first().map(|r| r.get_str("title")).transpose()
    }

    fn load_workspace_chats(&self, ws_id: &str) -> Result<Vec<WorkspaceChat>, DbError> {
        let q = Query::select()
            .columns([Chats::ChatId, Chats::Title, Chats::CreatedAt])
            .from(Chats::Table)
            .and_where(Expr::col(Chats::WorkspaceId).eq(ws_id))
            .order_by(Chats::CreatedAt, Order::Desc)
            .to_owned();
        self.driver
            .select(&q)?
            .iter()
            .map(|r| {
                Ok(WorkspaceChat {
                    chat_id: r.get_str("chat_id")?,
                    title: Some(r.get_str("title")?),
                    created_at: r.get_str("created_at")?,
                })
            })
            .collect()
    }
}

pub(crate) type ChartRead = (String, RecordGroup, String, String);

pub(crate) fn build_exchange(
    prefix: &str,
    suffix: &str,
    msg: (String, String, i64),
    query: &str,
    reads: &[ChartRead],
    more: bool,
    answer_text: String,
) -> Exchange {
    let (key, question, ts) = msg;
    let steps = if reads.is_empty() && query.is_empty() {
        Vec::new()
    } else {
        let read_records = reads
            .iter()
            .map(|(id, group, title, dt)| RecordRead::Whole {
                version: SourceVersion {
                    candidate_id: id.clone(),
                    revision: "1".into(),
                },
                about: RecordAbout {
                    group: Some(*group),
                    title: Some(title.clone()),
                    dates: vec![RecordDate {
                        role: Some(DateRole::Recorded),
                        value: dt.clone(),
                    }],
                },
            })
            .collect();
        vec![
            Step::SearchedChart {
                query: query.into(),
                group: None,
                page: SearchPage::First,
                state: StepState::Returned {
                    result: Found {
                        records: reads.len(),
                        more,
                    },
                },
            },
            Step::ReadRecords {
                requested: reads.len(),
                state: StepState::Returned {
                    result: read_records,
                },
            },
        ]
    };
    Exchange {
        work_item_id: WorkItemId(format!("wi-{prefix}-{suffix}")),
        messages: vec![PhysicianMessage {
            id: MessageKey(key),
            text: question,
            saved_at: SavedAt(ts),
        }],
        steps: ExchangeSteps::Read(steps),
        kept: Vec::new(),
        answer: AnswerState::Finished {
            response_id: ResponseId(format!("resp-{prefix}-{suffix}")),
            attempt_id: AttemptId(format!("att-{prefix}-{suffix}")),
            finished_at: SavedAt(ts + 2),
            text: answer_text,
            feedback: None,
        },
    }
}

fn not_found(kind: &'static str, id: &str) -> DbError {
    DbError::NotFound {
        kind,
        id: id.into(),
    }
}

fn required_str(value: &serde_json::Value, key: &'static str) -> Result<String, DbError> {
    value
        .get(key)
        .and_then(serde_json::Value::as_str)
        .map(str::to_owned)
        .ok_or_else(|| DbError::InvalidCommand(format!("missing `{key}`")))
}

fn decode_source_record(row: &SqlRow) -> Result<DemoSourceRecord, DbError> {
    Ok(DemoSourceRecord {
        id: row.get_str("id")?,
        workspace_id: row.get_str("workspace_id")?,
        group: row.get_str("record_group")?,
        title: row.get_str("title")?,
        recorded_at: row.get_str("recorded_at")?,
        body: row.get_str("body")?,
        source_label: row.get_str("source_label")?,
    })
}

fn extract_search_terms(question: &str) -> Vec<String> {
    const STOP: &[&str] = &[
        "the", "and", "for", "with", "what", "when", "where", "which", "this", "that", "have",
        "has", "had", "does", "his", "her", "their", "about", "from", "into", "are", "was", "were",
        "been", "how", "any", "all", "can", "show", "tell", "check", "latest", "recent", "patient",
        "chart", "please",
    ];
    let mut terms = Vec::new();
    for raw in question.split(|c: char| !c.is_alphanumeric() && c != '-') {
        let w = raw.trim().to_ascii_lowercase();
        if w.len() >= 3 && !STOP.contains(&w.as_str()) && !terms.contains(&w) {
            terms.push(w);
        }
        if terms.len() >= 6 {
            break;
        }
    }
    terms
}

fn synthesize_chart_answer(question: &str, records: &[DemoSourceRecord]) -> String {
    if records.is_empty() {
        return format!(
            "No matching chart records were found in SQLite for \"{question}\". Try asking about medications, labs, blood pressure, or recent visits."
        );
    }
    let mut out = format!(
        "From SQLite chart search across {} record(s):\n\n",
        records.len()
    );
    for (idx, rec) in records.iter().enumerate() {
        let excerpt: String = rec
            .body
            .split_whitespace()
            .take(34)
            .collect::<Vec<_>>()
            .join(" ");
        out.push_str(&format!(
            "[{}] {} ({}) — {}...\n\n",
            idx + 1,
            rec.title,
            rec.recorded_at,
            excerpt
        ));
    }
    out.trim_end().into()
}

fn truncate_title(question: &str) -> String {
    let t = question.trim();
    if t.chars().count() <= 44 {
        t.into()
    } else {
        format!("{}…", t.chars().take(43).collect::<String>())
    }
}

fn parse_record_group(raw: &str) -> RecordGroup {
    serde_json::from_value(serde_json::Value::String(raw.into())).unwrap_or(RecordGroup::Other)
}
