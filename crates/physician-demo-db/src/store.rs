//! Typed `sea-query` repository backing all physician UI reads and interactive demo commands.

use crate::driver::{DbError, SqlRow, SqliteDriver};
use crate::schema::{
    Chats, Drafts, Patients, PracticeVisits, SourceRecords, SpruceMatchQueue, TodayItems,
    UpdatesFeed, schema_ddl,
};
use physician_wire::{
    AnswerState, AttemptId, ChatSnapshot, ChatSnapshotResponse, CitationOrigin, CitationTargetKind,
    CitedSourceWire, DateRole, Exchange, ExchangeSteps, Found, HealthCapabilities, HealthResponse,
    HistoryWorkOutcome, MessageKey, MessageSegment, PatientSearchResponse, PatientSearchRow,
    PatientWorkspaceResponse, PatientWorkspaceSummary, PatientWorkspacesResponse, PhysicianDraft,
    PhysicianMessage, PracticeAccountSummary, PracticeDayResponse, PracticeSourceState,
    RecentConversation, RecentConversationsResponse, RecordAbout, RecordDate, RecordGroup,
    RecordRead, ReplyDraftWire, ResponseFeedback, ResponseId, SaveFeedback, SavedAt, SearchPage,
    SnapshotFeedback, SourceVersion, Step, StepState, TodayConnection, TodayListResponse, TodayRow,
    UpdatesResponse, WhoamiResponse, WorkItemId, WorkspaceChat, WorkspacePlanItem,
    today_updates::{EarlierDayUpdatesWire, ItemCitationWire, SeenStandingWire, UpdateItemWire},
};
use sea_query::{Cond, Expr, ExprTrait, OnConflict, Order, Query, SelectStatement};
use std::collections::BTreeMap;
use std::sync::atomic::{AtomicU64, Ordering};

pub(crate) const DEMO_DATE: &str = "2026-08-27";
pub(crate) const DEMO_NOW: &str = "2026-08-27T19:00:00.000Z";
pub(crate) const DEMO_ZONE: &str = "America/New_York";
pub(crate) const DEMO_BASE_UNIX: i64 = 1_787_857_200;
static SEQUENCE: AtomicU64 = AtomicU64::new(1);

pub(crate) fn next_seq() -> u64 {
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
    pub(crate) driver: SqliteDriver,
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
                practice_sources: true,
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
        q.columns([TodayItems::RowJson, TodayItems::Seen])
            .from(TodayItems::Table)
            .and_where(Expr::col(TodayItems::Done).eq(0));
        if let Some(id) = ws {
            q.and_where(Expr::col(TodayItems::WorkspaceId).eq(id));
        }
        q.order_by(TodayItems::Urgency, Order::Asc)
            .order_by(TodayItems::WaitingSince, Order::Desc);
        let decoded: Vec<(TodayRow, bool)> = self
            .map_select(&q, |r| {
                let row: TodayRow = serde_json::from_str(&r.get_str("row_json")?)?;
                let seen_flag = r.get_i64("seen")? != 0 || row.seen;
                Ok((row, seen_flag))
            })
            .await?;
        let mut rows = Vec::with_capacity(decoded.len());
        let mut seen = Vec::new();
        for (mut row, is_seen) in decoded {
            if is_seen {
                row.seen = true;
                if row.standing.is_none() || matches!(row.standing, Some(SeenStandingWire::Unseen))
                {
                    row.standing = Some(SeenStandingWire::Seen {
                        seen_at: DEMO_NOW.into(),
                    });
                }
                seen.push(row.item_id.clone());
            }
            rows.push(row);
        }
        let unmatched_q = Query::select()
            .column(SpruceMatchQueue::MessageId)
            .from(SpruceMatchQueue::Table)
            .and_where(Expr::col(SpruceMatchQueue::Bucket).eq("unmatched"))
            .to_owned();
        let messages_to_match_count = self.driver.select(&unmatched_q)?.len();

        let quiet_q = Query::select()
            .column(UpdatesFeed::Id)
            .from(UpdatesFeed::Table)
            .and_where(Expr::col(UpdatesFeed::Bucket).eq("quiet"))
            .to_owned();
        let also_reviewed = self.driver.select(&quiet_q)?.len();

        let connections = ["spruce", "gmail"]
            .into_iter()
            .map(|c| TodayConnection {
                condition: "connected".into(),
                channel: Some(c.into()),
                source_id: None,
                source_label: None,
                reason: None,
                last_successful_read_at: None,
                recovery: None,
            })
            .collect();
        Ok(TodayListResponse {
            date: DEMO_DATE.into(),
            time_zone: Some(DEMO_ZONE.into()),
            rows,
            seen,
            as_of: 1,
            connections,
            incomplete: Some(false),
            also_reviewed,
            practice_outages: Vec::new(),
            failed_parts: Vec::new(),
            messages_to_match_count,
            may_be_missing_someone: false,
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
            may_be_missing_someone: false,
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
                source_label: Some("Demo Clinic Calendar".into()),
                snapshot_as_of: Some(DEMO_NOW.into()),
                synchronized_at: Some(DEMO_NOW.into()),
                freshness: Some("current".into()),
                last_error: None,
            },
            visits,
            may_be_missing_someone: false,
        })
    }

    /// Queries recent conversations across seeded patients.
    pub async fn recent_conversations(&self) -> Result<RecentConversationsResponse, DbError> {
        let join = Expr::col((Chats::Table, Chats::WorkspaceId))
            .equals((Patients::Table, Patients::WorkspaceId));
        let q = Query::select()
            .column((Chats::Table, Chats::ChatId))
            .column((Chats::Table, Chats::WorkspaceId))
            .column((Chats::Table, Chats::Title))
            .column((Chats::Table, Chats::CreatedAt))
            .column((Chats::Table, Chats::SnapshotJson))
            .column((Patients::Table, Patients::DisplayName))
            .from(Chats::Table)
            .inner_join(Patients::Table, join)
            .order_by((Chats::Table, Chats::CreatedAt), Order::Desc)
            .limit(12)
            .to_owned();
        let conversations = self
            .map_select(&q, |r| {
                let snap: ChatSnapshot = serde_json::from_str(&r.get_str("snapshot_json")?)?;
                let (summary, outcome) = summarize_snapshot(&snap);
                Ok(RecentConversation {
                    chat_id: Some(r.get_str("chat_id")?),
                    patient_workspace_id: Some(r.get_str("workspace_id")?),
                    title: Some(r.get_str("title")?),
                    summary,
                    outcome,
                    patient_display_name: Some(r.get_str("display_name")?),
                    updated_at: Some(r.get_str("created_at")?),
                    archived: snap.archived,
                })
            })
            .await?;
        Ok(RecentConversationsResponse {
            conversations,
            may_be_missing_someone: false,
        })
    }

    /// Queries the Updates feed.
    pub async fn updates(&self) -> Result<UpdatesResponse, DbError> {
        let q = Query::select()
            .columns([UpdatesFeed::Bucket, UpdatesFeed::PayloadJson])
            .from(UpdatesFeed::Table)
            .order_by(UpdatesFeed::CreatedAt, Order::Desc)
            .to_owned();
        let entries: Vec<(String, serde_json::Value)> = self
            .map_select(&q, |r| {
                Ok((
                    r.get_str("bucket")?,
                    serde_json::from_str(&r.get_str("payload_json")?)?,
                ))
            })
            .await?;
        let mut today_items = Vec::new();
        let mut reviewed_nothing_for_you = Vec::new();
        let mut yesterday_items = Vec::new();
        let mut legacy_rows = Vec::new();
        for (bucket, payload) in entries {
            if let Ok(item) = serde_json::from_value::<UpdateItemWire>(payload.clone()) {
                match bucket.as_str() {
                    "quiet" => reviewed_nothing_for_you.push(item),
                    "yesterday" => yesterday_items.push(item),
                    _ => today_items.push(item),
                }
            } else {
                legacy_rows.push(payload);
            }
        }
        let earlier_days = if yesterday_items.is_empty() {
            Vec::new()
        } else {
            vec![EarlierDayUpdatesWire {
                date: "2026-08-26".into(),
                label: "Yesterday".into(),
                items: yesterday_items,
            }]
        };
        Ok(UpdatesResponse {
            active_runs: Vec::new(),
            rows: legacy_rows,
            today_items,
            reviewed_nothing_for_you,
            earlier_days,
            failed_parts: Vec::new(),
            may_be_missing_someone: false,
        })
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
            Patients::Mrn,
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
                    mrn: Some(r.get_str("mrn")?),
                    duet_linked: true,
                    sharing_consented: true,
                    connection_status: physician_wire::ConnectionStatusWire::Connected,
                    cadence: Some("daily_when_changed".into()),
                })
            })
            .await?;
        Ok(PatientSearchResponse {
            patients,
            next_after: None,
            may_be_missing_someone: false,
            match_mode: Default::default(),
        })
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
                sources: Default::default(),
                transcript: Vec::new(),
                feedback: SnapshotFeedback::Loaded,
                queued: Vec::new(),
                archived: false,
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
            "message.queue_edit" => self.handle_queue_edit(payload).await,
            "message.withdraw" => self.handle_queue_withdraw(payload).await,
            "message.queue_send_now" => self.handle_queue_send_now(body, payload).await,
            "conversation.archive" => self.handle_conversation_archive(payload, true).await,
            "conversation.restore" => self.handle_conversation_archive(payload, false).await,
            "discussion.open" => self.handle_discussion_open(payload).await,
            "item.open" => self.handle_item_open(payload),
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
        let mut snap = self.chat_snapshot(&ws_id, &chat_id).await?.snapshot;
        let is_new = snap.transcript.is_empty();
        let seq = next_seq();
        let ts = DEMO_BASE_UNIX + i64::try_from(seq).unwrap_or(1);
        if payload.get("delivery").and_then(serde_json::Value::as_str) == Some("send_next") {
            snap.queued.push(physician_wire::QueuedMessageWire {
                id: MessageKey(key),
                text: question.clone(),
                saved_at: SavedAt(ts),
            });
            let title = self
                .existing_chat_title(&ws_id, &chat_id)?
                .unwrap_or_else(|| truncate_title(&question));
            self.upsert_chat_snapshot(&ws_id, &chat_id, &title, DEMO_NOW, &snap)?;
            return Ok(serde_json::json!({ "ok": true, "chat_id": chat_id }));
        }
        self.append_chart_exchange(&ws_id, &chat_id, &mut snap, is_new, (key, question, ts))
    }

    fn append_chart_exchange(
        &self,
        ws_id: &str,
        chat_id: &str,
        snap: &mut ChatSnapshot,
        is_new: bool,
        msg: (String, String, i64),
    ) -> Result<serde_json::Value, DbError> {
        let (key, question, ts) = msg;
        let terms = extract_search_terms(&question);
        let matched = self.search_patient_records(ws_id, &terms)?;
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
        let (answer_text, segments, basis, cited_map) =
            synthesize_chart_answer_with_citations(&question, &matched, "c");
        for (k, v) in cited_map {
            snap.sources.insert(k, v);
        }
        snap.transcript.push(build_exchange_with_segments(
            "demo",
            &seq.to_string(),
            (key, question.clone(), ts),
            &terms.join(" "),
            &reads,
            false,
            (answer_text, segments, basis),
        ));
        let title = if is_new {
            truncate_title(&question)
        } else {
            self.existing_chat_title(ws_id, chat_id)?
                .unwrap_or_else(|| truncate_title(&question))
        };
        self.upsert_chat_snapshot(ws_id, chat_id, &title, DEMO_NOW, snap)?;
        Ok(serde_json::json!({ "ok": true, "chat_id": chat_id }))
    }

    async fn handle_queue_edit(
        &self,
        payload: &serde_json::Value,
    ) -> Result<serde_json::Value, DbError> {
        let ws_id = required_str(payload, "patient_workspace_id")?;
        let chat_id = required_str(payload, "chat_id")?;
        let msg_id = required_str(payload, "message_id")?;
        let text = required_str(payload, "text")?;
        let mut snap = self.chat_snapshot(&ws_id, &chat_id).await?.snapshot;
        let Some(item) = snap.queued.iter_mut().find(|q| q.id.0 == msg_id) else {
            return Ok(serde_json::json!({
                "outcome": "rejected",
                "message": "Coda couldn’t update that queued message.",
            }));
        };
        item.text = text;
        let title = self
            .existing_chat_title(&ws_id, &chat_id)?
            .unwrap_or_else(|| "Conversation".into());
        self.upsert_chat_snapshot(&ws_id, &chat_id, &title, DEMO_NOW, &snap)?;
        Ok(serde_json::json!({
            "outcome": "accepted",
            "patient_workspace_id": ws_id,
            "chat_id": chat_id,
            "message_id": msg_id,
        }))
    }

    async fn handle_queue_withdraw(
        &self,
        payload: &serde_json::Value,
    ) -> Result<serde_json::Value, DbError> {
        let ws_id = required_str(payload, "patient_workspace_id")?;
        let chat_id = required_str(payload, "chat_id")?;
        let msg_id = required_str(payload, "message_id")?;
        let mut snap = self.chat_snapshot(&ws_id, &chat_id).await?.snapshot;
        let before = snap.queued.len();
        snap.queued.retain(|q| q.id.0 != msg_id);
        if snap.queued.len() == before {
            return Ok(serde_json::json!({
                "outcome": "rejected",
                "message": "Coda couldn’t remove that queued message.",
            }));
        }
        let title = self
            .existing_chat_title(&ws_id, &chat_id)?
            .unwrap_or_else(|| "Conversation".into());
        self.upsert_chat_snapshot(&ws_id, &chat_id, &title, DEMO_NOW, &snap)?;
        Ok(serde_json::json!({
            "outcome": "accepted",
            "patient_workspace_id": ws_id,
            "chat_id": chat_id,
            "message_id": msg_id,
        }))
    }

    async fn handle_queue_send_now(
        &self,
        _envelope: &serde_json::Value,
        payload: &serde_json::Value,
    ) -> Result<serde_json::Value, DbError> {
        let ws_id = required_str(payload, "patient_workspace_id")?;
        let chat_id = required_str(payload, "chat_id")?;
        let msg_id = required_str(payload, "message_id")?;
        let mut snap = self.chat_snapshot(&ws_id, &chat_id).await?.snapshot;
        let Some(pos) = snap.queued.iter().position(|q| q.id.0 == msg_id) else {
            return Ok(serde_json::json!({
                "outcome": "rejected",
                "message": "Coda couldn’t send that queued message.",
            }));
        };
        let queued = snap.queued.remove(pos);
        let is_new = snap.transcript.is_empty();
        self.append_chart_exchange(
            &ws_id,
            &chat_id,
            &mut snap,
            is_new,
            (queued.id.0, queued.text, queued.saved_at.0),
        )?;
        Ok(serde_json::json!({
            "outcome": "accepted",
            "patient_workspace_id": ws_id,
            "chat_id": chat_id,
            "message_id": msg_id,
        }))
    }

    async fn handle_conversation_archive(
        &self,
        payload: &serde_json::Value,
        archived: bool,
    ) -> Result<serde_json::Value, DbError> {
        let ws_id = required_str(payload, "patient_workspace_id")?;
        let chat_id = required_str(payload, "chat_id")?;
        let rows = self.driver.select_where(
            Chats::Table,
            &[Chats::Title, Chats::CreatedAt, Chats::SnapshotJson],
            &[
                (Chats::WorkspaceId, ws_id.as_str()),
                (Chats::ChatId, chat_id.as_str()),
            ],
        )?;
        let Some(row) = rows.first() else {
            return Err(not_found("conversation", &chat_id));
        };
        let mut snap: ChatSnapshot = serde_json::from_str(&row.get_str("snapshot_json")?)?;
        snap.archived = archived;
        self.upsert_chat_snapshot(
            &ws_id,
            &chat_id,
            &row.get_str("title")?,
            &row.get_str("created_at")?,
            &snap,
        )?;
        Ok(serde_json::json!({
            "outcome": "accepted",
            "patient_workspace_id": ws_id,
            "chat_id": chat_id,
        }))
    }

    async fn handle_discussion_open(
        &self,
        payload: &serde_json::Value,
    ) -> Result<serde_json::Value, DbError> {
        let ws_id = required_str(payload, "patient_workspace_id")?;
        let chat_id = required_str(payload, "chat_id")?;
        let _ = self.load_patient_name(&ws_id)?;
        let existing = self.driver.select_where(
            Chats::Table,
            &[Chats::ChatId],
            &[
                (Chats::WorkspaceId, ws_id.as_str()),
                (Chats::ChatId, chat_id.as_str()),
            ],
        )?;
        if existing.is_empty() {
            let snap = ChatSnapshot {
                sources: Default::default(),
                transcript: Vec::new(),
                feedback: SnapshotFeedback::Loaded,
                queued: Vec::new(),
                archived: false,
            };
            self.upsert_chat_snapshot(&ws_id, &chat_id, "Clinical discussion", DEMO_NOW, &snap)?;
        }
        Ok(serde_json::json!({
            "outcome": "accepted",
            "patient_workspace_id": ws_id,
            "chat_id": chat_id,
            "returned_messages": [],
        }))
    }

    fn handle_item_open(&self, payload: &serde_json::Value) -> Result<serde_json::Value, DbError> {
        let ws_id = required_str(payload, "patient_workspace_id")?;
        let item_id = required_str(payload, "item_id")?;
        self.mark_today_item_seen(&ws_id, &item_id)?;
        Ok(serde_json::json!({
            "outcome": "accepted",
            "patient_workspace_id": ws_id,
            "item_id": item_id,
        }))
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
            .columns([
                Chats::ChatId,
                Chats::Title,
                Chats::CreatedAt,
                Chats::SnapshotJson,
            ])
            .from(Chats::Table)
            .and_where(Expr::col(Chats::WorkspaceId).eq(ws_id))
            .order_by(Chats::CreatedAt, Order::Desc)
            .to_owned();
        self.driver
            .select(&q)?
            .iter()
            .map(|r| {
                let snap: ChatSnapshot = serde_json::from_str(&r.get_str("snapshot_json")?)?;
                let (summary, outcome) = summarize_snapshot(&snap);
                Ok(WorkspaceChat {
                    chat_id: r.get_str("chat_id")?,
                    title: Some(r.get_str("title")?),
                    summary,
                    outcome,
                    created_at: r.get_str("created_at")?,
                    archived: snap.archived,
                })
            })
            .collect()
    }

    pub(crate) fn load_patient_name(&self, ws_id: &str) -> Result<String, DbError> {
        let rows = self.driver.select_where(
            Patients::Table,
            &[Patients::DisplayName],
            &[(Patients::WorkspaceId, ws_id)],
        )?;
        let row = rows
            .first()
            .ok_or_else(|| not_found("patient workspace", ws_id))?;
        row.get_str("display_name")
    }

    pub(crate) fn load_patient_records(
        &self,
        ws_id: &str,
    ) -> Result<Vec<DemoSourceRecord>, DbError> {
        let q = Query::select()
            .columns(SourceRecords::COLS.iter().copied())
            .from(SourceRecords::Table)
            .and_where(Expr::col(SourceRecords::WorkspaceId).eq(ws_id))
            .order_by(SourceRecords::RecordedAt, Order::Desc)
            .to_owned();
        self.driver
            .select(&q)?
            .iter()
            .map(decode_source_record)
            .collect()
    }

    pub(crate) fn upsert_chat_snapshot(
        &self,
        ws_id: &str,
        chat_id: &str,
        title: &str,
        created_at: &str,
        snap: &ChatSnapshot,
    ) -> Result<(), DbError> {
        let insert = Query::insert()
            .into_table(Chats::Table)
            .columns(Chats::COLS.iter().copied())
            .values_panic([
                chat_id.into(),
                ws_id.into(),
                title.into(),
                created_at.into(),
                serde_json::to_string(snap)?.as_str().into(),
            ])
            .on_conflict(
                OnConflict::column(Chats::ChatId)
                    .update_columns([Chats::Title, Chats::SnapshotJson])
                    .to_owned(),
            )
            .to_owned();
        self.driver.insert(&insert)
    }

    pub(crate) fn mark_today_item_seen(&self, ws_id: &str, item_id: &str) -> Result<(), DbError> {
        let rows = self.driver.select_where(
            TodayItems::Table,
            &[TodayItems::RowJson],
            &[
                (TodayItems::WorkspaceId, ws_id),
                (TodayItems::ItemId, item_id),
            ],
        )?;
        if let Some(row) = rows.first() {
            self.driver.update_where(
                TodayItems::Table,
                TodayItems::Seen,
                1.into(),
                &[
                    (TodayItems::WorkspaceId, ws_id),
                    (TodayItems::ItemId, item_id),
                ],
            )?;
            let mut parsed: TodayRow = serde_json::from_str(&row.get_str("row_json")?)?;
            parsed.seen = true;
            parsed.standing = Some(SeenStandingWire::Seen {
                seen_at: DEMO_NOW.into(),
            });
            self.driver.update_where(
                TodayItems::Table,
                TodayItems::RowJson,
                serde_json::to_string(&parsed)?.into(),
                &[
                    (TodayItems::WorkspaceId, ws_id),
                    (TodayItems::ItemId, item_id),
                ],
            )?;
        }
        Ok(())
    }
}

pub(crate) type ChartRead = (String, RecordGroup, String, String);

pub(crate) fn build_exchange_with_segments(
    prefix: &str,
    suffix: &str,
    msg: (String, String, i64),
    query: &str,
    reads: &[ChartRead],
    more: bool,
    answer: (String, Vec<MessageSegment>, Vec<MessageSegment>),
) -> Exchange {
    let (key, question, ts) = msg;
    let (answer_text, segments, basis) = answer;
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
            segments,
            basis: (!basis.is_empty()).then_some(basis),
            newer_records: false,
            feedback: None,
        },
    }
}

pub(crate) fn build_cited_source(
    key: &str,
    number: usize,
    rec: &DemoSourceRecord,
) -> CitedSourceWire {
    let quote: String = rec
        .body
        .split_whitespace()
        .take(24)
        .collect::<Vec<_>>()
        .join(" ");
    CitedSourceWire {
        source_key: key.to_owned(),
        number,
        source_label: rec.source_label.clone(),
        title: rec.title.clone(),
        record_type: Some(rec.group.clone()),
        recorded_at: Some(rec.recorded_at.clone()),
        record_status: Some("final".into()),
        quote,
        body: rec.body.clone(),
        readable_parts: Vec::new(),
        fields: Vec::new(),
        raw_content: None,
        external_url: None,
        origin: CitationOrigin::SourceRecord,
    }
}

pub(crate) fn build_item_citation(rec: &DemoSourceRecord) -> ItemCitationWire {
    let excerpt: String = rec
        .body
        .split_whitespace()
        .take(24)
        .collect::<Vec<_>>()
        .join(" ");
    ItemCitationWire {
        candidate_id: rec.id.clone(),
        revision: "1".into(),
        source_id: "synthetic-ehr".into(),
        source_label: rec.source_label.clone(),
        label: rec.title.clone(),
        as_of: Some(rec.recorded_at.clone()),
        excerpt,
        body: rec.body.clone(),
        native_uri: None,
    }
}

pub(crate) fn synthesize_chart_answer_with_citations(
    question: &str,
    records: &[DemoSourceRecord],
    key_prefix: &str,
) -> (
    String,
    Vec<MessageSegment>,
    Vec<MessageSegment>,
    BTreeMap<String, CitedSourceWire>,
) {
    if records.is_empty() {
        let text = format!(
            "No matching chart records were found in SQLite for \"{question}\". Try asking about medications, labs, blood pressure, or recent visits."
        );
        return (
            text.clone(),
            vec![MessageSegment::Text { text }],
            Vec::new(),
            BTreeMap::new(),
        );
    }
    let mut text = format!(
        "From SQLite chart search across {} record(s):\n\n",
        records.len()
    );
    let mut segments = vec![MessageSegment::Text {
        text: format!(
            "From SQLite chart search across {} record(s): ",
            records.len()
        ),
    }];
    let mut sources = BTreeMap::new();
    for (idx, rec) in records.iter().enumerate() {
        let number = idx + 1;
        let key = format!("{key_prefix}{}-{}", next_seq(), number);
        let excerpt: String = rec
            .body
            .split_whitespace()
            .take(30)
            .collect::<Vec<_>>()
            .join(" ");
        text.push_str(&format!(
            "[{number}] {} ({}) — {}...\n\n",
            rec.title, rec.recorded_at, excerpt
        ));
        if idx > 0 {
            segments.push(MessageSegment::Text { text: " ".into() });
        }
        segments.push(MessageSegment::Text {
            text: format!("{} ({}): {excerpt}", rec.title, rec.recorded_at),
        });
        let cited = build_cited_source(&key, number, rec);
        segments.push(MessageSegment::Citation {
            source_key: key.clone(),
            number,
            label: format!("{} ({})", rec.title, rec.recorded_at),
            quote: cited.quote.clone(),
            target: CitationTargetKind::WholeRecord,
        });
        sources.insert(key, cited);
    }
    let basis = vec![MessageSegment::Text {
        text: format!(
            "Grounded directly in {} chart record(s) saved in the browser SQLite database.",
            records.len()
        ),
    }];
    (text.trim_end().into(), segments, basis, sources)
}

pub(crate) fn build_answer_segments_and_sources_for_follow_up(
    summary: &str,
    prompt: &str,
    records: &[DemoSourceRecord],
) -> (Exchange, BTreeMap<String, CitedSourceWire>) {
    let seq = next_seq();
    let reads: Vec<_> = records
        .iter()
        .take(2)
        .map(|r| {
            (
                r.id.clone(),
                parse_record_group(&r.group),
                r.title.clone(),
                r.recorded_at.clone(),
            )
        })
        .collect();
    let (_, segments, basis, sources) =
        synthesize_chart_answer_with_citations(summary, records.get(..2).unwrap_or(records), "fu");
    let answer_text = format!(
        "Paused follow-up \"{summary}\" for review. Ask below to update the plan or restart monitoring."
    );
    let ex = build_exchange_with_segments(
        "fu",
        &seq.to_string(),
        (
            format!("msg-fu-{seq}"),
            prompt.to_owned(),
            DEMO_BASE_UNIX + 10,
        ),
        summary,
        &reads,
        false,
        (answer_text, segments, basis),
    );
    (ex, sources)
}

fn summarize_snapshot(snap: &ChatSnapshot) -> (Option<String>, Option<HistoryWorkOutcome>) {
    for ex in snap.transcript.iter().rev() {
        if let AnswerState::Finished { text, .. } = &ex.answer {
            let preview = text.lines().next().unwrap_or("").trim();
            let short = if preview.chars().count() > 110 {
                format!("{}…", preview.chars().take(109).collect::<String>())
            } else {
                preview.to_owned()
            };
            return (Some(short), Some(HistoryWorkOutcome::Answered));
        }
    }
    (None, None)
}

pub(crate) fn not_found(kind: &'static str, id: &str) -> DbError {
    DbError::NotFound {
        kind,
        id: id.into(),
    }
}

pub(crate) fn required_str(
    value: &serde_json::Value,
    key: &'static str,
) -> Result<String, DbError> {
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

fn truncate_title(question: &str) -> String {
    let t = question.trim();
    if t.chars().count() <= 44 {
        t.into()
    } else {
        format!("{}…", t.chars().take(43).collect::<String>())
    }
}

pub(crate) fn parse_record_group(raw: &str) -> RecordGroup {
    serde_json::from_value(serde_json::Value::String(raw.into())).unwrap_or(RecordGroup::Other)
}
