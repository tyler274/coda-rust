//! Hosted chat composition. Authoritative acceptance and snapshots share the
//! existing physician runtime; the broker is only a recoverable wakeup hint.
use crate::queue::ChatQueue;
use local_product::runtime::{
    Accepted, Authority, Conversation, Input, Resumed, Scope, Store,
    delivery::Delivery,
    reply::{RevisionAccepted, RevisionInput},
};
use std::sync::Arc;

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

    /// Saves a revision and its exact source observation before queueing it.
    /// Duplicate and refused commands never publish another wakeup.
    pub async fn accept_revision(
        &self,
        scope: &Scope,
        input: &RevisionInput,
        session_hash: Vec<u8>,
        sources: Vec<local_product::physician_sources::SourceRevision>,
    ) -> Result<RevisionAccepted, String> {
        let accepted = self
            .store
            .accept_revision(scope, input, session_hash, sources)
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

    /// Explicitly starts recovery on the site's always-allocated CPU. Keep and
    /// abort the task on graceful shutdown. Construction never starts execution.
    pub fn start_recovery(self: Arc<Self>) -> tokio::task::JoinHandle<()> {
        tokio::spawn(async move {
            let mut after = None;
            let mut revision_after = None;
            let mut ticks = tokio::time::interval(std::time::Duration::from_secs(10));
            loop {
                ticks.tick().await;
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
