//! Hosted chat composition. Authoritative acceptance and snapshots share the
//! existing physician runtime; the broker is only a recoverable wakeup hint.
use crate::queue::ChatQueue;
use local_product::runtime::{
    Accepted, Authority, Conversation, Input, Resumed, Scope, Stopped, Store,
    delivery::Delivery,
    reply::{RevisionAccepted, RevisionInput},
};
use std::sync::Arc;

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
    queue: Arc<dyn ChatQueue>,
}
impl DurableChat {
    /// Does not migrate schemas or start background work.
    pub fn new(store: Store, queue: Arc<dyn ChatQueue>) -> Self {
        Self { store, queue }
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
            .accept_authorized(scope, input, session_hash)
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
                .dispatched(&delivery, 60)
                .await
                .map_err(|e| e.to_string())?;
        } else {
            tracing::warn!("accepted physician input awaits queue recovery");
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
            .accept_revision(scope, input, session_hash)
            .await
            .map_err(|e| e.to_string())?;
        if accepted != RevisionAccepted::New {
            return Ok(accepted);
        }
        let delivery = Delivery::reply_revision(scope.clone(), input.key.clone());
        if self.queue.publish(&delivery).await.is_ok() {
            // Acceptance is committed already. Even a dispatch-marker failure
            // must not report accepted work as rejected; recovery can republish.
            if self.store.dispatched(&delivery, 60).await.is_err() {
                tracing::warn!("accepted physician revision dispatch marker unavailable");
            }
        } else {
            tracing::warn!("accepted physician revision awaits queue recovery");
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
        let Resumed::Pending { input_id } = &resumed else {
            return Ok(resumed);
        };
        let delivery = Delivery::chat_turn(scope.clone(), input_id.clone());
        if self.queue.publish(&delivery).await.is_ok() {
            self.store
                .dispatched(&delivery, 60)
                .await
                .map_err(|e| e.to_string())?;
        } else {
            tracing::warn!("resumed physician input awaits queue recovery");
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
    pub async fn recover_page(&self, after: Option<&Scope>) -> Result<Option<Scope>, String> {
        let page = self
            .store
            .dispatch_due(100, after)
            .await
            .map_err(|e| e.to_string())?;
        for delivery in page.deliveries {
            if self.queue.publish(&delivery).await.is_ok() {
                self.store
                    .dispatched(&delivery, 60)
                    .await
                    .map_err(|e| e.to_string())?;
            } else {
                tracing::warn!("physician queue recovery publication failed");
            }
        }
        Ok(page.next)
    }

    /// Revision recovery has its own keyset, independent of chat pagination.
    pub async fn recover_revision_page(
        &self,
        after: Option<&Scope>,
    ) -> Result<Option<Scope>, String> {
        let page = self
            .store
            .revisions_due(100, after)
            .await
            .map_err(|e| e.to_string())?;
        for delivery in page.deliveries {
            if self.queue.publish(&delivery).await.is_ok() {
                self.store
                    .dispatched(&delivery, 60)
                    .await
                    .map_err(|e| e.to_string())?;
            } else {
                tracing::warn!("physician revision queue recovery publication failed");
            }
        }
        Ok(page.next)
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
                    match self.compact_receipts().await {
                        Ok(0) => {}
                        Ok(reduced) => {
                            tracing::info!(reduced, "physician command receipts reduced to digests")
                        }
                        Err(_) => {
                            tracing::warn!("physician receipt compaction database unavailable")
                        }
                    }
                }
                tick = tick.wrapping_add(1);
                match self.recover_page(after.as_ref()).await {
                    Ok(next) => after = next,
                    Err(_) => tracing::warn!("physician queue recovery database unavailable"),
                }
                match self.recover_revision_page(revision_after.as_ref()).await {
                    Ok(next) => revision_after = next,
                    Err(_) => tracing::warn!("physician revision recovery database unavailable"),
                }
            }
        })
    }
}

/// Uses the auth owner's transaction locks, never a cached session projection.
pub struct RuntimeAuthority {
    /// Runtime used to acquire the same conversation/publication transaction.
    pub store: Store,
}
#[local_product::runtime::async_trait]
impl Authority for RuntimeAuthority {
    async fn check(&mut self, scope: &Scope) -> Result<(), String> {
        self.store
            .clone()
            .authorize(scope, self)
            .await
            .map_err(|e| e.to_string())
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
