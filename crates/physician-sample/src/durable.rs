//! Hosted chat composition. Acceptance and snapshots share the physician
//! runtime, which also tells the workers: the store sends its notification
//! in the transaction that makes work runnable, so accepted work is never
//! left waiting on a second step here.
use crate::hosting::{ScenarioRefused, SimulationPractices};
use local_product::runtime::{
    Accepted, Authority, Conversation, Input, Resumed, Scope, Stopped, Store,
    delivery::TraceContext,
    reply::{RevisionAccepted, RevisionInput},
};
use std::collections::HashMap;
use std::sync::Arc;
use tracing::Instrument;

/// Receipts reduced per compaction transaction.
const RECEIPT_BATCH: u64 = 100;
/// Compaction transactions per sweep, so one sweep stays bounded.
const RECEIPT_BATCHES: usize = 20;
/// Time between compaction sweeps.
const COMPACTION_INTERVAL: std::time::Duration = std::time::Duration::from_secs(3600);

/// Hosted chat owner used by the authenticated HTTP composition.
#[derive(Clone)]
pub struct DurableChat {
    /// Existing authoritative Postgres runtime.
    pub store: Store,
    skills: local_product::physician_chat::PhysicianSkills,
    roles: local_product::ModelRoles,
    revision_recipe: harness::engine::recipe::RequestRecipe,
    /// Practices whose sessions may tag their inputs with a rehearsal scenario.
    simulation_practices: SimulationPractices,
}
impl DurableChat {
    /// Does not migrate schemas or start background work. No practice is a
    /// rehearsal practice until [`Self::with_simulation_practices`] names one.
    pub fn new(
        store: Store,
        skills: local_product::physician_chat::PhysicianSkills,
        roles: &local_product::ModelRoles,
    ) -> Self {
        Self {
            store,
            roles: roles.clone(),
            revision_recipe: local_product::runtime::reply::recipe(&roles.background, &skills),
            skills,
            simulation_practices: SimulationPractices::default(),
        }
    }

    /// Configured model roles for physician chat and background practice runs.
    pub fn roles(&self) -> &local_product::ModelRoles {
        &self.roles
    }

    /// Approved clinical skills catalog shared by chat and practice runs.
    pub fn skills(&self) -> &local_product::physician_chat::PhysicianSkills {
        &self.skills
    }

    /// The same owner, accepting scenario tags from `practices`' sessions.
    pub fn with_simulation_practices(mut self, practices: SimulationPractices) -> Self {
        self.simulation_practices = practices;
        self
    }

    /// Admits the scenario a session in `practice` sent with its command:
    /// none is always fine; one is only from a rehearsal practice.
    pub fn admit_scenario(
        &self,
        practice: &str,
        scenario: Option<String>,
    ) -> Result<Option<String>, ScenarioRefused> {
        let Some(scenario) = scenario else {
            return Ok(None);
        };
        self.simulation_practices
            .admit_scenario(practice, &scenario)?;
        Ok(Some(scenario))
    }

    /// Saves the receipt and input; the same transaction wakes the workers.
    /// A duplicate receipt is already accepted and wakes no one.
    pub async fn accept(
        &self,
        scope: &Scope,
        input: &Input,
        session_hash: Vec<u8>,
    ) -> Result<Accepted, String> {
        let chat_recipe = self
            .store
            .chat_recipe_with_understanding(
                &self.roles.chat,
                local_product::physician_chat::INSTRUCTIONS,
                &self.skills,
                &scope.practice,
                &scope.workspace,
            )
            .await
            .map_err(|e| e.to_string())?;
        self.store
            .accept_authorized(
                scope,
                input,
                session_hash,
                &self.skills,
                &chat_recipe,
                request_context(),
            )
            .await
            .map_err(|e| e.to_string())
    }

    /// Saves a revision, pinning the workspace's current source observation
    /// from the runtime source store. Duplicate and refused commands wake no
    /// one.
    pub async fn accept_revision(
        &self,
        scope: &Scope,
        input: &RevisionInput,
        session_hash: Vec<u8>,
    ) -> Result<RevisionAccepted, String> {
        self.store
            .accept_revision(
                scope,
                input,
                session_hash,
                &self.skills,
                &self.revision_recipe,
                request_context(),
            )
            .await
            .map_err(|e| e.to_string())
    }

    /// The physician's Try again on a failed turn. The runtime checks the
    /// named work and run, saves the receipt and wakes the workers.
    pub async fn resume(
        &self,
        scope: &Scope,
        key: &str,
        session_hash: Vec<u8>,
        work_item_id: &str,
        expected_run_id: &str,
    ) -> Result<Resumed, String> {
        self.store
            .resume_authorized(
                scope,
                key,
                session_hash,
                work_item_id,
                expected_run_id,
                request_context(),
            )
            .await
            .map_err(|e| e.to_string())
    }

    /// The physician's Stop. The runtime holds the named work and saves the
    /// receipt. A held turn is settled, so Stop wakes no one.
    pub async fn stop(
        &self,
        scope: &Scope,
        key: &str,
        session_hash: Vec<u8>,
        work_item_id: &str,
    ) -> Result<Stopped, String> {
        self.store
            .stop_authorized(scope, key, session_hash, work_item_id)
            .await
            .map_err(|error| error.to_string())
    }

    /// Reads only the scoped durable conversation; caller authenticates current
    /// read authority separately (the accepted session may since have expired).
    pub async fn snapshot(&self, scope: &Scope) -> Result<Conversation, String> {
        self.store
            .conversation(scope)
            .await
            .map_err(|e| e.to_string())
    }

    /// Reduces command receipts older than 90 days to their salted digest,
    /// in batches of [`RECEIPT_BATCH`], at most [`RECEIPT_BATCHES`] per call.
    /// Returns how many it reduced. Nothing is deleted.
    pub async fn compact_receipts(&self) -> Result<u64, String> {
        let mut total = 0;
        for _ in 0..RECEIPT_BATCHES {
            let reduced = self
                .store
                .compact_receipts(
                    local_product::runtime::receipt::RECEIPT_FULL_SECONDS,
                    RECEIPT_BATCH,
                )
                .await
                .map_err(|e| e.to_string())?;
            total += reduced;
            if reduced < RECEIPT_BATCH {
                break;
            }
        }
        Ok(total)
    }

    /// Starts hourly receipt compaction, with a first sweep at once. Keep
    /// and abort the task on graceful shutdown. Construction never starts it.
    pub fn start_compaction(self: Arc<Self>) -> tokio::task::JoinHandle<()> {
        tokio::spawn(async move {
            let mut ticks = tokio::time::interval(COMPACTION_INTERVAL);
            loop {
                ticks.tick().await;
                let span = tracing::info_span!(parent: None, "physician.receipt_compaction",
                    otel.status_code = tracing::field::Empty,
                    error.type = tracing::field::Empty);
                async {
                    match self.compact_receipts().await {
                        Ok(0) => {}
                        Ok(reduced) => {
                            tracing::info!(reduced, "physician command receipts reduced to digests")
                        }
                        Err(_) => {
                            tracing::Span::current().record("otel.status_code", "ERROR");
                            tracing::Span::current().record("error.type", "database");
                            tracing::error!(
                                event = "physician.receipt_compaction.failed",
                                error.type = "database",
                                "physician receipt compaction database unavailable"
                            );
                        }
                    }
                }
                .instrument(span)
                .await;
            }
        })
    }
}

/// The current request's W3C trace context, saved with the work it makes
/// runnable so the attempt that runs it can link back. `None` when the
/// request has no exported span.
fn request_context() -> Option<TraceContext> {
    let mut carrier = HashMap::new();
    telemetry::propagation::inject_current(&mut carrier);
    Some(TraceContext {
        traceparent: carrier.remove(telemetry::propagation::TRACEPARENT)?,
        tracestate: carrier
            .remove(telemetry::propagation::TRACESTATE)
            .filter(|state| !state.is_empty()),
    })
}

/// Uses the auth owner's read check and transactional publication fence.
pub struct RuntimeAuthority;
#[local_product::runtime::async_trait]
impl Authority for RuntimeAuthority {
    async fn check(
        &mut self,
        tx: &local_product::runtime::PublicationTransaction,
        scope: &Scope,
        hash: Option<&[u8]>,
    ) -> Result<(), String> {
        physician_auth::PostgresAuthStore::check_runtime_authority(
            tx,
            hash.ok_or("accepted session reference missing")?,
            &physician_auth::PhysicianIdentity {
                practice_id: scope.practice.clone(),
                physician_id: scope.physician.clone(),
            },
            &scope.workspace,
        )
        .await
        .map_err(|error| error.to_string())
    }
    async fn lock_publication(
        &mut self,
        tx: &local_product::runtime::PublicationTransaction,
        scope: &Scope,
        hash: Option<&[u8]>,
    ) -> Result<(), String> {
        physician_auth::PostgresAuthStore::lock_runtime_authority(
            tx,
            hash.ok_or("accepted session reference missing")?,
            &physician_auth::PhysicianIdentity {
                practice_id: scope.practice.clone(),
                physician_id: scope.physician.clone(),
            },
            &scope.workspace,
        )
        .await
        .map_err(|e| e.to_string())
    }
}
