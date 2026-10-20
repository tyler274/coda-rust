//! Explicit restart passes. No background task or provider is started by construction.
//! The composition calls a pass at startup and on a bounded timer; failures are
//! returned per scope, so one revoked session cannot strand other accepted work.
use super::*;

/// Product composition supplies authorized execution, not another model loop.
#[async_trait]
pub trait Executor: Send {
    /// Revalidate the original accepted session and patient before claiming work.
    async fn authorize(&mut self, scope: &Scope, session_hash: &[u8]) -> Result<(), String>;
    /// Execute exactly one claimed attempt through PhysicianHost and its fence.
    async fn execute(
        &mut self,
        store: Store,
        claim: Claim,
        session_hash: Vec<u8>,
    ) -> Result<(), String>;
}

/// Result of one bounded activation pass; failures never become completed work.
#[derive(Default, Debug)]
pub struct Report {
    /// Successfully executed attempts.
    pub completed: usize,
    /// Work blocked by authorization or execution, with its scope retained.
    pub blocked: Vec<(Scope, String)>,
    /// Pass to the next page so persistently blocked scopes cannot starve others.
    pub next: Option<Scope>,
}

impl Store {
    /// Restarts pending work, including expired leases, without unrelated input.
    /// At most `maximum` scopes are visited, sequentially (one active provider per
    /// dispatcher). Concurrent instances contend on the existing claim lock.
    /// Caller must independently gate activation on product qualification.
    pub async fn dispatch_pending(
        &self,
        executor: &mut dyn Executor,
        maximum: usize,
        lease_seconds: i64,
    ) -> Result<Report, DbErr> {
        self.dispatch_page(executor, maximum, lease_seconds, None)
            .await
    }

    /// Continues a keyset-paginated restart sweep. Begin a fresh sweep when next is None.
    pub async fn dispatch_page(
        &self,
        executor: &mut dyn Executor,
        maximum: usize,
        lease_seconds: i64,
        after: Option<&Scope>,
    ) -> Result<Report, DbErr> {
        if !(1..=100).contains(&maximum) || !(1..=600).contains(&lease_seconds) {
            return Err(error("invalid dispatcher bounds"));
        }
        let mut report = Report::default();
        let scopes = self.pending_batch(maximum, after).await?;
        if scopes.len() == maximum {
            report.next = scopes.last().cloned();
        }
        for scope in scopes {
            let state = self.conversation(&scope).await?;
            let Some(hash) = state.session_hash else {
                report
                    .blocked
                    .push((scope, "accepted session is missing".into()));
                continue;
            };
            if let Err(reason) = executor.authorize(&scope, &hash).await {
                report.blocked.push((scope, reason));
                continue;
            }
            let Some(claim) = self.claim(&scope, lease_seconds).await? else {
                continue;
            };
            // Steering between authorization and claim must not substitute the
            // previously checked session for the new accepted input's authority.
            let current = self.conversation(&scope).await?;
            if current.session_hash.as_ref() != Some(&hash) {
                self.interrupt(&claim, false).await?;
                report
                    .blocked
                    .push((scope, "accepted authority changed".into()));
                continue;
            }
            let execution = tokio::time::timeout(
                std::time::Duration::from_secs(lease_seconds as u64),
                executor.execute(self.clone(), claim.clone(), hash),
            )
            .await;
            let result = match execution {
                Ok(result) => result,
                Err(_) => {
                    self.interrupt(&claim, false).await?;
                    report.blocked.push((
                        scope,
                        "execution exceeded its lease; successor remains pending".into(),
                    ));
                    continue;
                }
            };
            match result {
                Ok(()) => {
                    let saved = self.conversation(&scope).await?;
                    if saved
                        .attempts
                        .iter()
                        .any(|a| a.id == claim.attempt && a.status == "completed")
                    {
                        report.completed += 1;
                    } else {
                        self.interrupt(&claim, true).await?;
                        report.blocked.push((
                            scope,
                            "executor returned without committed completion".into(),
                        ));
                    }
                }
                Err(reason) => {
                    self.interrupt(&claim, true).await?;
                    report.blocked.push((scope, reason));
                }
            }
        }
        Ok(report)
    }
}
