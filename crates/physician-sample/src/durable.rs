//! Hosted chat composition. Authoritative acceptance and snapshots share the
//! existing physician runtime; the broker is only a recoverable wakeup hint.
use crate::hosting::{ScenarioRefused, SimulationPractices};
use crate::queue::ChatQueue;
use local_product::runtime::{
    Accepted, Authority, Conversation, Input, Resumed, Scope, Stopped, Store,
    delivery::Delivery,
    reply::{RevisionAccepted, RevisionInput},
};
use std::sync::Arc;
use tracing::Instrument;

// Give a published wakeup one minute to claim work before the recovery sweep republishes it.
const DISPATCH_RETRY_SECONDS: i64 = 60;

/// Receipts reduced per compaction transaction.
const RECEIPT_BATCH: u64 = 100;
/// Compaction transactions per sweep, so one sweep stays bounded.
const RECEIPT_BATCHES: usize = 20;
/// Recovery ticks (10 s each) between compaction sweeps: one hour.
const COMPACTION_TICKS: u64 = 360;

/// Hosted chat owner used by the authenticated HTTP composition.
#[derive(Clone)]
pub struct DurableChat {
    /// Existing authoritative Postgres runtime.
    pub store: Store,
    skills: local_product::physician_chat::PhysicianSkills,
    chat_recipe: harness::engine::recipe::RequestRecipe,
    revision_recipe: harness::engine::recipe::RequestRecipe,
    queue: Arc<dyn ChatQueue>,
    /// Practices whose sessions may tag their inputs with a rehearsal scenario.
    simulation_practices: SimulationPractices,
}
impl DurableChat {
    /// Does not migrate schemas or start background work. No practice is a
    /// rehearsal practice until [`Self::with_simulation_practices`] names one.
    pub fn new(
        store: Store,
        queue: Arc<dyn ChatQueue>,
        skills: local_product::physician_chat::PhysicianSkills,
        roles: &local_product::ModelRoles,
    ) -> Self {
        Self {
            store,
            queue,
            chat_recipe: local_product::physician_chat::recipe(
                &roles.chat,
                local_product::physician_chat::INSTRUCTIONS,
                &skills,
            ),
            revision_recipe: local_product::runtime::reply::recipe(&roles.background, &skills),
            skills,
            simulation_practices: SimulationPractices::default(),
        }
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

    /// Saves the receipt/input/recovery identity before trying the broker.
    /// Publication failure does NOT turn accepted work into an HTTP failure:
    /// a bounded recovery sweep will retry it from Postgres after a crash too.
    pub async fn accept(
        &self,
        scope: &Scope,
        input: &Input,
        session_hash: Vec<u8>,
    ) -> Result<Accepted, String> {
        let accepted = self
            .store
            .accept_authorized(scope, input, session_hash, &self.skills, &self.chat_recipe)
            .await
            .map_err(|e| e.to_string())?;
        // A duplicate receipt is already accepted. Publishing again would run
        // the turn twice; the first wakeup, or recovery, still owns delivery.
        if matches!(accepted, Accepted::Duplicate) {
            return Ok(accepted);
        }
        let delivery = Delivery::chat_turn(scope.clone(), input.key.clone());
        if self.queue.publish(&delivery).await.is_ok() {
            self.store
                .dispatched(&delivery, DISPATCH_RETRY_SECONDS)
                .await
                .map_err(|e| e.to_string())?;
        } else {
            tracing::error!(event = "physician.queue.publish_failed", error.type = "publish",
                "accepted physician input awaits queue recovery");
        }
        Ok(accepted)
    }

    /// Saves a revision, pinning the workspace's current source observation
    /// from the runtime source store, before queueing it. Duplicate and
    /// refused commands never publish another wakeup.
    pub async fn accept_revision(
        &self,
        scope: &Scope,
        input: &RevisionInput,
        session_hash: Vec<u8>,
    ) -> Result<RevisionAccepted, String> {
        let accepted = self
            .store
            .accept_revision(
                scope,
                input,
                session_hash,
                &self.skills,
                &self.revision_recipe,
            )
            .await
            .map_err(|e| e.to_string())?;
        if accepted != RevisionAccepted::New {
            return Ok(accepted);
        }
        let delivery = Delivery::reply_revision(scope.clone(), input.key.clone());
        if self.queue.publish(&delivery).await.is_ok() {
            // Acceptance is committed already. Even a dispatch-marker failure
            // must not report accepted work as rejected; recovery can republish.
            if self
                .store
                .dispatched(&delivery, DISPATCH_RETRY_SECONDS)
                .await
                .is_err()
            {
                tracing::error!(
                    event = "physician.queue.dispatch_failed",
                    error.type = "database",
                    "accepted physician revision dispatch marker unavailable"
                );
            }
        } else {
            tracing::error!(event = "physician.queue.publish_failed", error.type = "publish",
                "accepted physician revision awaits queue recovery");
        }
        Ok(accepted)
    }

    /// The physician's Try again on a failed turn. The runtime checks the named
    /// work and run and saves the receipt; the wakeup then follows the same
    /// rule as [`Self::accept`]: publish, else leave it for the recovery sweep.
    pub async fn resume(
        &self,
        scope: &Scope,
        key: &str,
        session_hash: Vec<u8>,
        work_item_id: &str,
        expected_run_id: &str,
    ) -> Result<Resumed, String> {
        let resumed = self
            .store
            .resume_authorized(scope, key, session_hash, work_item_id, expected_run_id)
            .await
            .map_err(|e| e.to_string())?;
        let Resumed::Pending { delivery } = &resumed else {
            return Ok(resumed);
        };
        if self.queue.publish(delivery).await.is_ok() {
            self.store
                .dispatched(delivery, DISPATCH_RETRY_SECONDS)
                .await
                .map_err(|e| e.to_string())?;
        } else {
            tracing::error!(event = "physician.queue.publish_failed", error.type = "publish",
                "resumed physician input awaits queue recovery");
        }
        Ok(resumed)
    }

    /// The physician's Stop. The runtime holds the named work and saves the
    /// receipt. Nothing is published: a held turn is settled, and a duplicate
    /// Stop must not wake the worker.
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

    /// One bounded, fair recovery page. Returns the next keyset cursor.
    /// Runs in its own root `physician.recovery.sweep` span, so a republished
    /// wakeup carries the sweep's trace context, never a stale request's.
    pub async fn recover_page(&self, after: Option<&Scope>) -> Result<Option<Scope>, String> {
        let span = sweep_span("chat_turn");
        async {
            let page = self
                .store
                .dispatch_due(100, after)
                .await
                .inspect_err(|_| record_recovery_failure("database"))
                .map_err(|e| e.to_string())?;
            let mut republished = 0_u64;
            for delivery in page.deliveries {
                if self.queue.publish(&delivery).await.is_ok() {
                    republished += 1;
                    self.store
                        .dispatched(&delivery, DISPATCH_RETRY_SECONDS)
                        .await
                        .inspect_err(|_| record_recovery_failure("database"))
                        .map_err(|e| e.to_string())?;
                } else {
                    record_recovery_failure("publish");
                }
            }
            tracing::Span::current().record("coda.republished", republished);
            Ok(page.next)
        }
        .instrument(span)
        .await
    }

    /// Revision recovery has its own keyset, independent of chat pagination.
    pub async fn recover_revision_page(
        &self,
        after: Option<&Scope>,
    ) -> Result<Option<Scope>, String> {
        let span = sweep_span("reply_revision");
        async {
            let page = self
                .store
                .revisions_due(100, after)
                .await
                .inspect_err(|_| record_recovery_failure("database"))
                .map_err(|e| e.to_string())?;
            let mut republished = 0_u64;
            for delivery in page.deliveries {
                if self.queue.publish(&delivery).await.is_ok() {
                    republished += 1;
                    self.store
                        .dispatched(&delivery, DISPATCH_RETRY_SECONDS)
                        .await
                        .inspect_err(|_| record_recovery_failure("database"))
                        .map_err(|e| e.to_string())?;
                } else {
                    record_recovery_failure("publish");
                }
            }
            tracing::Span::current().record("coda.republished", republished);
            Ok(page.next)
        }
        .instrument(span)
        .await
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

    /// Explicitly starts recovery on the site's always-allocated CPU. Keep and
    /// abort the task on graceful shutdown. Construction never starts execution.
    /// Receipt compaction runs on the first tick and then hourly.
    pub fn start_recovery(self: Arc<Self>) -> tokio::task::JoinHandle<()> {
        tokio::spawn(async move {
            let mut after = None;
            let mut revision_after = None;
            let mut ticks = tokio::time::interval(std::time::Duration::from_secs(10));
            let mut tick: u64 = 0;
            loop {
                ticks.tick().await;
                if tick.is_multiple_of(COMPACTION_TICKS) {
                    let span = tracing::info_span!(parent: None, "physician.receipt_compaction",
                        otel.status_code = tracing::field::Empty,
                        error.type = tracing::field::Empty);
                    async {
                        match self.compact_receipts().await {
                            Ok(0) => {}
                            Ok(reduced) => {
                                tracing::info!(
                                    reduced,
                                    "physician command receipts reduced to digests"
                                )
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
                tick = tick.wrapping_add(1);
                // Failures are recorded inside each sweep's trace. Preserve
                // the last cursor so the next tick retries the same page.
                if let Ok(next) = self.recover_page(after.as_ref()).await {
                    after = next;
                }
                if let Ok(next) = self.recover_revision_page(revision_after.as_ref()).await {
                    revision_after = next;
                }
            }
        })
    }
}

/// A root span for one recovery page: a sweep is never caused by the request
/// that accepted the work it republishes.
fn sweep_span(kind: &'static str) -> tracing::Span {
    tracing::info_span!(
        parent: None,
        "physician.recovery.sweep",
        coda.delivery_kind = kind,
        coda.republished = tracing::field::Empty,
        otel.status_code = tracing::field::Empty,
        error.type = tracing::field::Empty,
    )
}

fn record_recovery_failure(class: &'static str) {
    let span = tracing::Span::current();
    span.record("otel.status_code", "ERROR");
    span.record("error.type", class);
    tracing::error!(event = "physician.recovery.failed", error.type = class,
        "physician recovery failed");
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
