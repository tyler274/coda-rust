//! Physician operational truth. Every mutation locks its conversation in Postgres.
//! Construction never migrates or starts work. Expired attempts terminate as
//! interrupted; a claim creates a successor rather than replaying an attempt.
//! Hosted mutation tools and reply revision remain unavailable until their
//! product effect owners are ported. No process-memory persistence fallback.

/// Async callback implementation helper for product adapters.
pub use async_trait::async_trait;
use engine::{Admission, Candidate, Context, Event, Host, Invocation, Item, Usage};
/// Neutral execution types, reexported for the physician composition.
pub use harness::engine;
/// Existing API adapter; no second physician provider transport.
pub use harness::responses::Responses;
use sea_orm::{
    ConnectionTrait, Database, DatabaseConnection, DbBackend, DbErr, Statement, TransactionTrait,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

/// Explicit bounded restart activation, separate from construction.
pub mod recovery;

/// Authenticated product scope, supplied by the server, never the command body.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Scope {
    /// Practice tenant.
    pub practice: String,
    /// Signed-in physician.
    pub physician: String,
    /// Authorized patient workspace.
    pub workspace: String,
    /// Conversation identity.
    pub chat: String,
}

/// Accepted physician input, including timezone in the idempotency comparison.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Input {
    /// Client command identity, scoped to tenant and physician.
    pub key: String,
    /// Exact authored text.
    pub text: String,
    /// Physician's timezone.
    pub timezone: String,
}

/// A saved authored message, not execution telemetry.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Message {
    /// User or assistant.
    pub role: String,
    /// Exact authored text.
    pub text: String,
}

/// Product attempt with its immutable predecessor and durable evidence.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Attempt {
    /// Attempt identity.
    pub id: String,
    /// Exact previous attempt, if interrupted or steered.
    pub predecessor: Option<String>,
    /// Work identity shared by successor attempts.
    pub work: String,
    /// Input generation fenced at publication.
    pub revision: u64,
    /// Active, interrupted, cancelled, failed, or completed.
    pub status: String,
    /// Database-clock lease expiry in seconds.
    pub expires: i64,
    /// Ordered acknowledged engine evidence, internal only.
    pub events: Vec<Event>,
}

/// Durable conversation and work projection.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Conversation {
    /// Authored conversation only.
    pub messages: Vec<Message>,
    /// Current accepted input generation.
    pub revision: u64,
    /// Pending, active, held, failed, or completed.
    pub status: String,
    /// Current work identity; successors preserve it.
    pub work: String,
    /// All retained attempts, including interrupted attempts.
    pub attempts: Vec<Attempt>,
    /// Accepted session reference, never the bearer secret; absent legacy work is blocked.
    #[serde(default)]
    pub session_hash: Option<Vec<u8>>,
}

impl Default for Conversation {
    fn default() -> Self {
        Self {
            messages: vec![],
            revision: 0,
            status: "completed".into(),
            work: String::new(),
            attempts: vec![],
            session_hash: None,
        }
    }
}

/// A fenced claim; safe to pass to the shared engine, never authority by itself.
#[derive(Clone, Debug)]
pub struct Claim {
    /// Product scope.
    pub scope: Scope,
    /// Saved attempt identity.
    pub attempt: String,
    /// Input generation.
    pub revision: u64,
}

/// Durable accepted command outcome.
#[derive(Debug, PartialEq, Eq)]
pub enum Accepted {
    /// Input saved for execution.
    New,
    /// Exact original command already saved; do not dispatch it again.
    Duplicate,
}

/// Postgres physician runtime owner. Cloning shares connections, not state.
#[derive(Clone)]
pub struct Store {
    db: DatabaseConnection,
}

fn sql(query: &str, values: Vec<sea_orm::Value>) -> Statement {
    Statement::from_sql_and_values(DbBackend::Postgres, query, values)
}
fn error(message: &str) -> DbErr {
    DbErr::Custom(message.into())
}
fn keys(scope: &Scope) -> Vec<sea_orm::Value> {
    vec![
        scope.practice.clone().into(),
        scope.physician.clone().into(),
        scope.workspace.clone().into(),
        scope.chat.clone().into(),
    ]
}

impl Store {
    /// Connects only. Schema-owner credentials belong to the migration job.
    pub async fn connect(url: &str) -> Result<Self, DbErr> {
        let mut options = sea_orm::ConnectOptions::new(url);
        options.max_connections(8).sqlx_logging(false);
        Ok(Self {
            db: Database::connect(options).await?,
        })
    }

    /// Explicit additive, rerunnable migration, never invoked by connect.
    pub async fn migrate(&self) -> Result<(), DbErr> {
        let tx = self.db.begin().await?;
        tx.execute_unprepared("SELECT pg_advisory_xact_lock(804081)")
            .await?;
        tx.execute_unprepared(include_str!("runtime.sql")).await?;
        tx.commit().await
    }

    async fn locked(
        &self,
        scope: &Scope,
    ) -> Result<(sea_orm::DatabaseTransaction, Conversation, i64), DbErr> {
        if [
            &scope.practice,
            &scope.physician,
            &scope.workspace,
            &scope.chat,
        ]
        .iter()
        .any(|s| s.trim().is_empty())
        {
            return Err(error("empty physician scope"));
        }
        let tx = self.db.begin().await?;
        let row = tx.query_one_raw(sql("SELECT state, floor(extract(epoch FROM clock_timestamp()))::bigint AS now FROM physician_runtime_conversations WHERE practice=$1 AND physician=$2 AND workspace=$3 AND chat=$4 FOR UPDATE", keys(scope))).await?.ok_or_else(|| error("conversation not found"))?;
        let state: Value = row.try_get("", "state")?;
        let now = tx
            .query_one_raw(sql(
                "SELECT floor(extract(epoch FROM clock_timestamp()))::bigint AS now",
                vec![],
            ))
            .await?
            .ok_or_else(|| error("missing database clock"))?
            .try_get("", "now")?;
        Ok((
            tx,
            serde_json::from_value(state).map_err(|_| error("corrupt conversation"))?,
            now,
        ))
    }

    async fn save(
        tx: sea_orm::DatabaseTransaction,
        scope: &Scope,
        state: &Conversation,
    ) -> Result<(), DbErr> {
        let mut values = keys(scope);
        values.push(
            serde_json::to_value(state)
                .map_err(|_| error("invalid conversation"))?
                .into(),
        );
        tx.execute_raw(sql("UPDATE physician_runtime_conversations SET state=$5 WHERE practice=$1 AND physician=$2 AND workspace=$3 AND chat=$4", values)).await?;
        tx.commit().await
    }

    /// Atomically saves the command receipt and input. Colliding payloads fail.
    pub async fn accept(&self, scope: &Scope, input: &Input) -> Result<Accepted, DbErr> {
        self.accept_inner(scope, input, None).await
    }

    /// Accepts hosted input with its original session reference for restart authorization.
    pub async fn accept_authorized(
        &self,
        scope: &Scope,
        input: &Input,
        session_hash: Vec<u8>,
    ) -> Result<Accepted, DbErr> {
        if session_hash.len() != 32 {
            return Err(error("invalid session reference"));
        }
        self.accept_inner(scope, input, Some(session_hash)).await
    }

    async fn accept_inner(
        &self,
        scope: &Scope,
        input: &Input,
        session_hash: Option<Vec<u8>>,
    ) -> Result<Accepted, DbErr> {
        if [
            &input.key,
            &input.text,
            &input.timezone,
            &scope.practice,
            &scope.physician,
            &scope.workspace,
            &scope.chat,
        ]
        .iter()
        .any(|s| s.trim().is_empty())
        {
            return Err(error("empty command field"));
        }
        let tx = self.db.begin().await?;
        let command = serde_json::json!({"scope": scope, "input": input});
        let result = tx.execute_raw(sql("INSERT INTO physician_runtime_commands (practice, physician, key, command) VALUES ($1,$2,$3,$4) ON CONFLICT DO NOTHING", vec![scope.practice.clone().into(), scope.physician.clone().into(), input.key.clone().into(), command.clone().into()])).await?;
        if result.rows_affected() == 0 {
            let row = tx.query_one_raw(sql("SELECT command FROM physician_runtime_commands WHERE practice=$1 AND physician=$2 AND key=$3", vec![scope.practice.clone().into(), scope.physician.clone().into(), input.key.clone().into()])).await?.ok_or_else(|| error("missing command receipt"))?;
            let previous: Value = row.try_get("", "command")?;
            if previous != command {
                return Err(error("idempotency collision"));
            }
            tx.commit().await?;
            return Ok(Accepted::Duplicate);
        }
        let mut values = keys(scope);
        values.push(
            serde_json::to_value(Conversation::default())
                .unwrap()
                .into(),
        );
        tx.execute_raw(sql("INSERT INTO physician_runtime_conversations (practice,physician,workspace,chat,state) VALUES ($1,$2,$3,$4,$5) ON CONFLICT DO NOTHING", values)).await?;
        let row = tx.query_one_raw(sql("SELECT state FROM physician_runtime_conversations WHERE practice=$1 AND physician=$2 AND workspace=$3 AND chat=$4 FOR UPDATE", keys(scope))).await?.ok_or_else(|| error("missing conversation"))?;
        let mut state: Conversation = serde_json::from_value(row.try_get("", "state")?)
            .map_err(|_| error("corrupt conversation"))?;
        if state.status == "completed" {
            state.work = Uuid::new_v4().to_string();
        }
        if let Some(attempt) = state.attempts.last_mut().filter(|a| a.status == "active") {
            attempt.status = "interrupted".into();
        }
        state.revision += 1;
        state.session_hash = session_hash;
        state.messages.push(Message {
            role: "user".into(),
            text: input.text.clone(),
        });
        // New input steers ordinary work, but does not silently release intentional Stop.
        if state.status != "held" {
            state.status = "pending".into();
        }
        Self::save(tx, scope, &state).await?;
        Ok(Accepted::New)
    }

    /// Reads authoritative state using the complete tenant/physician binding.
    pub async fn conversation(&self, scope: &Scope) -> Result<Conversation, DbErr> {
        let (tx, state, _) = self.locked(scope).await?;
        tx.commit().await?;
        Ok(state)
    }

    /// Claims pending work or recovers an expired active attempt as its successor.
    /// Lease must cover the bounded engine deadline; no two live claims coexist.
    pub async fn claim(&self, scope: &Scope, lease_seconds: i64) -> Result<Option<Claim>, DbErr> {
        if !(1..=600).contains(&lease_seconds) {
            return Err(error("invalid lease duration"));
        }
        let (tx, mut state, now) = self.locked(scope).await?;
        if state.status == "active" && state.attempts.last().is_some_and(|a| a.expires <= now) {
            state.attempts.last_mut().unwrap().status = "interrupted".into();
            state.status = "pending".into();
        }
        if state.status != "pending" {
            tx.commit().await?;
            return Ok(None);
        }
        let id = Uuid::new_v4().to_string();
        state.attempts.push(Attempt {
            id: id.clone(),
            predecessor: state
                .attempts
                .last()
                .filter(|a| a.work == state.work)
                .map(|a| a.id.clone()),
            work: state.work.clone(),
            revision: state.revision,
            status: "active".into(),
            expires: now + lease_seconds,
            events: vec![],
        });
        state.status = "active".into();
        let claim = Claim {
            scope: scope.clone(),
            attempt: id,
            revision: state.revision,
        };
        Self::save(tx, scope, &state).await?;
        Ok(Some(claim))
    }

    fn current(state: &Conversation, claim: &Claim, now: i64) -> bool {
        state.status == "active"
            && state.revision == claim.revision
            && state
                .attempts
                .last()
                .is_some_and(|a| a.id == claim.attempt && a.status == "active" && a.expires > now)
    }

    /// Revalidates the accepted session using the real auth owner's transaction locks.
    pub async fn authorize(
        &self,
        scope: &Scope,
        authority: &mut dyn Authority,
    ) -> Result<(), DbErr> {
        let (tx, state, _) = self.locked(scope).await?;
        authority
            .lock_publication(&tx, scope, state.session_hash.as_deref())
            .await
            .map_err(|e| error(&e))?;
        tx.commit().await
    }

    /// Rechecks the current attempt against durable Stop, steering and expiry.
    pub async fn assert_current(&self, claim: &Claim) -> Result<(), DbErr> {
        let (tx, state, now) = self.locked(&claim.scope).await?;
        if !Self::current(&state, claim, now) {
            return Err(error("attempt no longer current"));
        }
        tx.commit().await
    }

    /// Acknowledges an ordered event only after commit. Exact replay is idempotent.
    pub async fn persist(&self, claim: &Claim, event: Event) -> Result<(), DbErr> {
        let (tx, mut state, now) = self.locked(&claim.scope).await?;
        if event.attempt_id != claim.attempt {
            return Err(error("event attempt mismatch"));
        }
        let current = Self::current(&state, claim, now);
        // Late provider/tool evidence is inspectable truth, not permission to
        // execute or publish. Never acknowledge a stale dispatch or candidate.
        if !current
            && !matches!(
                event.kind,
                engine::EventKind::ProviderReturned(_) | engine::EventKind::ToolReturned { .. }
            )
        {
            return Err(error("stale event"));
        }
        let attempt = state
            .attempts
            .iter_mut()
            .find(|a| a.id == claim.attempt && a.revision == claim.revision)
            .ok_or_else(|| error("unknown attempt"))?;
        let events = &mut attempt.events;
        if event.sequence == 0 {
            return Err(error("invalid event sequence"));
        }
        if let Some(previous) = events.get((event.sequence - 1) as usize) {
            if serde_json::to_value(previous).unwrap() != serde_json::to_value(&event).unwrap() {
                return Err(error("event collision"));
            }
        } else if event.sequence == events.len() as u64 + 1 {
            events.push(event);
        } else {
            return Err(error("event sequence gap"));
        }
        Self::save(tx, &claim.scope, &state).await
    }

    /// Publishes a candidate only if its acknowledged evidence and claim still match.
    pub async fn commit(&self, claim: &Claim, candidate: &Candidate) -> Result<(), DbErr> {
        self.commit_inner(claim, candidate, None).await
    }

    /// Locks the real auth owner on this same transaction through reply publication.
    pub async fn commit_authorized(
        &self,
        claim: &Claim,
        candidate: &Candidate,
        authority: &mut dyn Authority,
    ) -> Result<(), DbErr> {
        self.commit_inner(claim, candidate, Some(authority)).await
    }

    async fn commit_inner(
        &self,
        claim: &Claim,
        candidate: &Candidate,
        authority: Option<&mut dyn Authority>,
    ) -> Result<(), DbErr> {
        let (tx, mut state, _) = self.locked(&claim.scope).await?;
        if let Some(authority) = authority {
            authority
                .lock_publication(&tx, &claim.scope, state.session_hash.as_deref())
                .await
                .map_err(|e| error(&e))?;
        } else if state.session_hash.is_some() {
            return Err(error("hosted publication requires transactional authority"));
        }
        let now = tx
            .query_one_raw(sql(
                "SELECT floor(extract(epoch FROM clock_timestamp()))::bigint AS now",
                vec![],
            ))
            .await?
            .ok_or_else(|| error("missing clock"))?
            .try_get("", "now")?;
        if !Self::current(&state, claim, now) {
            return Err(error("stale completion"));
        }
        let attempt = state.attempts.last_mut().unwrap();
        if !matches!(attempt.events.last().map(|e| &e.kind), Some(engine::EventKind::Candidate { text }) if text == &candidate.text)
        {
            return Err(error("candidate lacks durable evidence"));
        }
        attempt.status = "completed".into();
        state.status = "completed".into();
        state.messages.push(Message {
            role: "assistant".into(),
            text: candidate.text.clone(),
        });
        Self::save(tx, &claim.scope, &state).await
    }

    /// Ends only the named still-open attempt. Failure does not restart itself.
    pub async fn interrupt(&self, claim: &Claim, failed: bool) -> Result<(), DbErr> {
        let (tx, mut state, _) = self.locked(&claim.scope).await?;
        if state
            .attempts
            .last()
            .is_some_and(|a| a.id == claim.attempt && a.status == "active")
        {
            state.attempts.last_mut().unwrap().status =
                if failed { "failed" } else { "interrupted" }.into();
            state.status = if failed { "failed" } else { "pending" }.into();
        }
        Self::save(tx, &claim.scope, &state).await
    }

    /// Intentional Stop is durable and cannot be released by restart or late output.
    pub async fn stop(&self, scope: &Scope) -> Result<(), DbErr> {
        let (tx, mut state, _) = self.locked(scope).await?;
        if state.status != "completed" {
            if let Some(a) = state.attempts.last_mut().filter(|a| a.status == "active") {
                a.status = "cancelled".into();
            }
            state.status = "held".into();
        }
        Self::save(tx, scope, &state).await
    }

    /// Explicit physician resume of held or failed work; never repeats completed work.
    pub async fn resume(&self, scope: &Scope) -> Result<(), DbErr> {
        let (tx, mut state, _) = self.locked(scope).await?;
        if state.status == "held" || state.status == "failed" {
            state.status = "pending".into();
        }
        Self::save(tx, scope, &state).await
    }

    /// Enumerates durable pending/expired work for explicit server activation.
    /// Caller must reauthorize each scope; construction never starts execution.
    pub async fn pending(&self) -> Result<Vec<Scope>, DbErr> {
        self.pending_batch(100, None).await
    }

    async fn pending_batch(
        &self,
        maximum: usize,
        after: Option<&Scope>,
    ) -> Result<Vec<Scope>, DbErr> {
        let mut values = vec![(maximum as i64).into()];
        let after = after.cloned().unwrap_or(Scope {
            practice: String::new(),
            physician: String::new(),
            workspace: String::new(),
            chat: String::new(),
        });
        values.extend(keys(&after));
        let rows = self.db.query_all_raw(sql("SELECT practice,physician,workspace,chat FROM physician_runtime_conversations WHERE (practice,physician,workspace,chat) > ($2,$3,$4,$5) AND (state->>'status' = 'pending' OR (state->>'status' = 'active' AND (state->'attempts'->-1->>'expires')::bigint <= floor(extract(epoch FROM clock_timestamp()))::bigint)) ORDER BY practice,physician,workspace,chat LIMIT $1", values)).await?;
        rows.into_iter()
            .map(|r| {
                Ok(Scope {
                    practice: r.try_get("", "practice")?,
                    physician: r.try_get("", "physician")?,
                    workspace: r.try_get("", "workspace")?,
                    chat: r.try_get("", "chat")?,
                })
            })
            .collect()
    }
}

/// Product access is checked on every dispatch and again before publication.
#[async_trait]
pub trait Authority: Send {
    /// Revalidates physician/session and patient workspace access.
    async fn check(&mut self, scope: &Scope) -> Result<(), String>;
    /// Hold the auth owner's revocation locks until the supplied transaction commits.
    /// Default is fail-closed, never a separate preflight substituted for a fence.
    async fn lock_publication(
        &mut self,
        _tx: &PublicationTransaction,
        _scope: &Scope,
        _session_hash: Option<&[u8]>,
    ) -> Result<(), String> {
        Err("transactional publication authority is unavailable".into())
    }
}

/// Transaction borrowed by the auth owner, never a separate connection.
pub type PublicationTransaction = sea_orm::DatabaseTransaction;

/// Physician Host for the one shared execution loop. Tools stay unavailable
/// until their real product effect/read owners can be supplied, never fake success.
pub struct PhysicianHost<A> {
    /// Durable owner.
    pub store: Store,
    /// Fenced attempt.
    pub claim: Claim,
    /// Current authorization owner.
    pub authority: A,
    /// Product-selected versioned instructions/skills.
    pub instructions: String,
}

#[async_trait]
impl<A: Authority> Host for PhysicianHost<A> {
    async fn prepare(&mut self, transcript: &[Item], _: &Usage) -> Result<Context, String> {
        self.assert_current().await?;
        let state = self
            .store
            .conversation(&self.claim.scope)
            .await
            .map_err(|e| e.to_string())?;
        let items = if transcript.is_empty() {
            state
                .messages
                .into_iter()
                .map(|m| Item::Message {
                    role: m.role,
                    text: m.text,
                })
                .collect()
        } else {
            // The engine returns the full prepared context, not only its delta.
            transcript.to_vec()
        };
        Ok(Context {
            instructions: self.instructions.clone(),
            items,
            tools: vec![],
        })
    }
    async fn assert_current(&mut self) -> Result<(), String> {
        self.authority.check(&self.claim.scope).await?;
        self.store
            .assert_current(&self.claim)
            .await
            .map_err(|e| e.to_string())
    }
    async fn persist(&mut self, event: Event) -> Result<(), String> {
        self.store
            .persist(&self.claim, event)
            .await
            .map_err(|e| e.to_string())
    }
    async fn admit(&mut self, _: &Invocation, _: &Value) -> Result<Admission, String> {
        Ok(Admission::Deny(
            "This hosted tool is not yet available".into(),
        ))
    }
    async fn execute(&mut self, _: &Invocation, _: Value) -> Result<String, String> {
        Err("No hosted tool effect owner is configured".into())
    }
}

impl<A: Authority> PhysicianHost<A> {
    /// Executes through the shared harness, then applies the product publication fence.
    pub async fn run(
        &mut self,
        provider: &mut dyn engine::Provider,
        limits: &engine::Limits,
        cancel: &engine::Cancellation,
    ) -> Result<(), String> {
        let attempt = self.claim.attempt.clone();
        let result = engine::run(provider, self, &attempt, vec![], limits, cancel).await;
        match result {
            Ok(candidate) => {
                self.assert_current().await?;
                self.store
                    .commit_authorized(&self.claim, &candidate, &mut self.authority)
                    .await
                    .map_err(|e| e.to_string())
            }
            Err(failure) => {
                self.store
                    .interrupt(&self.claim, failure.failure != engine::Failure::Cancelled)
                    .await
                    .map_err(|e| e.to_string())?;
                Err(format!("execution ended: {:?}", failure.failure))
            }
        }
    }
}
