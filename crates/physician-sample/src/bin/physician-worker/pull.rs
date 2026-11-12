//! The pull loop. Postgres owns the work; this loop only finds it.
//!
//! A committed transaction that makes work runnable notifies
//! [`WORK_CHANNEL`](local_product::runtime::delivery::WORK_CHANNEL), and the
//! listener here starts that work at once. Notifications sent while the
//! listener is disconnected are lost, so the loop also reads the due work
//! every poll interval, and again whenever the listener resubscribes. A
//! worker that dies announces nothing, so each read also notes when the
//! soonest live lease runs out and reads again then: its turn is taken
//! within the lease, not a poll after it. A
//! delivery only names work: the claim decides whether this worker runs it,
//! so hearing of the same work twice, or on two workers, runs it once.
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use std::time::Duration;

use local_product::runtime::delivery::{Delivery, DeliveryKind, DispatchPage};
use local_product::runtime::{Scope, Store};
use sea_orm::DbErr;
use tokio::sync::{mpsc, watch};
use tokio::task::{Id, JoinSet};

use super::Worker;
use super::attempt::Next;

/// Due work read per query.
const PAGE: usize = 100;
/// The wait before a dropped listener resubscribes.
const RESUBSCRIBE_DELAY: Duration = Duration::from_secs(1);
/// Read past a lease's end, so the database's whole-second clock has passed it.
const LEASE_END_MARGIN: Duration = Duration::from_millis(200);
/// How long stopping waits for running attempts to hand their claims back.
/// Cloud Run ends the instance ten seconds after SIGTERM.
const DRAIN: Duration = Duration::from_secs(8);

/// Why the loop looks at work now.
enum Wake {
    /// This work became runnable, or its retry delay passed.
    Work(Delivery),
    /// The listener subscribed: anything announced before now was not heard.
    Subscribed,
}

/// Resolves once the worker has been told to stop.
pub(super) async fn stopping(mut stop: watch::Receiver<bool>) {
    while !*stop.borrow_and_update() {
        if stop.changed().await.is_err() {
            return;
        }
    }
}

/// The attempts this worker is running, at most `limit` at a time.
struct Running {
    limit: usize,
    tasks: JoinSet<Next>,
    work: HashMap<Id, Delivery>,
    claimed: HashSet<Delivery>,
}

impl Running {
    fn full(&self) -> bool {
        self.claimed.len() >= self.limit
    }

    /// Starts `delivery` unless this worker already runs it. False when the
    /// worker is at its limit: the work stays due and is found again.
    fn start(
        &mut self,
        worker: &Arc<Worker>,
        delivery: Delivery,
        stop: &watch::Receiver<bool>,
    ) -> bool {
        if self.claimed.contains(&delivery) {
            return true;
        }
        if self.full() {
            return false;
        }
        let (worker, stop, work) = (Arc::clone(worker), stop.clone(), delivery.clone());
        let task = self
            .tasks
            .spawn(async move { super::run_delivery(&worker, &work, stop).await });
        self.work.insert(task.id(), delivery.clone());
        self.claimed.insert(delivery);
        true
    }

    /// Forgets a finished attempt and says what it left to do. An attempt
    /// that panicked leaves its claim to run out.
    fn finished(
        &mut self,
        joined: Result<(Id, Next), tokio::task::JoinError>,
    ) -> Option<(Delivery, Next)> {
        let (id, next) = match joined {
            Ok((id, next)) => (id, next),
            Err(error) => {
                tracing::error!("physician worker attempt task ended abnormally");
                (error.id(), Next::Done)
            }
        };
        let delivery = self.work.remove(&id)?;
        self.claimed.remove(&delivery);
        Some((delivery, next))
    }
}

/// Runs due work until `stop`, then waits for the running attempts to hand
/// their claims back.
pub(super) async fn run(worker: Arc<Worker>, stop: watch::Receiver<bool>) {
    let (wake, mut woken) = mpsc::channel(256);
    let listener = tokio::spawn(listen(worker.store.clone(), wake.clone()));
    let mut poll = tokio::time::interval(worker.pace.poll);
    poll.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Delay);
    let mut running = Running {
        limit: worker.pace.concurrency,
        tasks: JoinSet::new(),
        work: HashMap::new(),
        claimed: HashSet::new(),
    };
    // Whether due work was left unstarted because the worker was full.
    let mut backlog = false;
    // When the soonest live lease runs out. A worker that dies sends nothing,
    // so its turn is found then rather than up to a poll later.
    let lease_end = tokio::time::sleep(Duration::ZERO);
    tokio::pin!(lease_end);
    let mut watching_lease = false;
    // Which kind of due work the next sweep reads first.
    let mut order = SweepOrder::default();
    loop {
        let sweep = tokio::select! {
            () = stopping(stop.clone()) => break,
            _ = poll.tick() => true,
            () = &mut lease_end, if watching_lease => true,
            Some(wake) = woken.recv() => match wake {
                Wake::Work(delivery) => {
                    backlog |= !running.start(&worker, delivery, &stop);
                    false
                }
                Wake::Subscribed => true,
            },
            Some(joined) = running.tasks.join_next_with_id() => {
                if let Some((delivery, Next::RetryAfter(delay))) = running.finished(joined) {
                    let wake = wake.clone();
                    tokio::spawn(async move {
                        tokio::time::sleep(delay).await;
                        // A closed channel means the worker is stopping.
                        let _ = wake.send(Wake::Work(delivery)).await;
                    });
                }
                backlog
            }
        };
        if sweep {
            // Read the soonest lease end before the due work: a lease that
            // ends between the two reads is due for the second, and every
            // lease still live at the first ends at or after the timer. A
            // failed read keeps a timer that has not fired; the poll covers
            // the rest.
            match worker.store.soonest_lease_end().await {
                Ok(Some(left)) => {
                    let at = tokio::time::Instant::now() + left + LEASE_END_MARGIN;
                    lease_end.as_mut().reset(at);
                    watching_lease = true;
                }
                Ok(None) => watching_lease = false,
                Err(_) => watching_lease &= !lease_end.is_elapsed(),
            }
            backlog = match start_due(&worker, &mut running, &stop, &mut order).await {
                Ok(left) => left,
                Err(_) => {
                    tracing::error!(
                        event = "physician.queue.dispatch_failed",
                        error.type = "database",
                        "physician worker could not read due work"
                    );
                    backlog
                }
            };
        }
    }
    listener.abort();
    // Every attempt watches the same signal and releases its own claim.
    let drained = tokio::time::timeout(DRAIN, async {
        while running.tasks.join_next().await.is_some() {}
    })
    .await;
    if drained.is_err() {
        tracing::warn!("physician worker stopped before every claim was handed back");
        running.tasks.shutdown().await;
    }
}

/// Which kind of due work a sweep reads first. A worker that fills up on the
/// first kind leaves the second unread, so the order alternates sweep by
/// sweep: a steady stream of chat turns cannot starve due reply revisions.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
enum SweepOrder {
    #[default]
    ChatTurnsFirst,
    RevisionsFirst,
}

impl SweepOrder {
    /// The kinds this sweep reads, in order, and flips the order for the next.
    fn next(&mut self) -> [DeliveryKind; 2] {
        let (kinds, following) = match self {
            Self::ChatTurnsFirst => (
                [DeliveryKind::ChatTurn, DeliveryKind::ReplyRevision],
                Self::RevisionsFirst,
            ),
            Self::RevisionsFirst => (
                [DeliveryKind::ReplyRevision, DeliveryKind::ChatTurn],
                Self::ChatTurnsFirst,
            ),
        };
        *self = following;
        kinds
    }
}

/// Starts the due work of each kind, in `order`, that the worker has room
/// for. True when it filled up with due work left over. A full worker reads
/// nothing and keeps its order, so the polls that pass while it is busy do not
/// decide which kind the next free slot goes to.
async fn start_due(
    worker: &Arc<Worker>,
    running: &mut Running,
    stop: &watch::Receiver<bool>,
    order: &mut SweepOrder,
) -> Result<bool, DbErr> {
    if running.full() {
        return Ok(true);
    }
    for kind in order.next() {
        let mut after: Option<Scope> = None;
        loop {
            if running.full() {
                return Ok(true);
            }
            let page = due(&worker.store, kind, after.as_ref()).await?;
            for delivery in page.deliveries {
                if !running.start(worker, delivery, stop) {
                    return Ok(true);
                }
            }
            match page.next {
                Some(next) => after = Some(next),
                None => break,
            }
        }
    }
    Ok(false)
}

async fn due(
    store: &Store,
    kind: DeliveryKind,
    after: Option<&Scope>,
) -> Result<DispatchPage, DbErr> {
    match kind {
        DeliveryKind::ChatTurn => store.dispatch_due(PAGE, after).await,
        DeliveryKind::ReplyRevision => store.revisions_due(PAGE, after).await,
    }
}

/// Forwards announced work to the loop, resubscribing whenever the
/// connection drops. Each subscription is announced first, so the loop reads
/// the due work that was announced while nothing was listening. Ends when
/// the loop does.
async fn listen(store: Store, wake: mpsc::Sender<Wake>) {
    loop {
        if let Ok(mut wakeups) = store.wakeups().await {
            if wake.send(Wake::Subscribed).await.is_err() {
                return;
            }
            while let Ok(Some(delivery)) = wakeups.next().await {
                if wake.send(Wake::Work(delivery)).await.is_err() {
                    return;
                }
            }
        }
        tracing::warn!("physician work listener disconnected; the poll covers the gap");
        tokio::time::sleep(RESUBSCRIBE_DELAY).await;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn each_sweep_reads_the_other_kind_first() {
        let mut order = SweepOrder::default();
        let firsts: Vec<_> = (0..4).map(|_| order.next()[0]).collect();
        assert_eq!(
            firsts,
            [
                DeliveryKind::ChatTurn,
                DeliveryKind::ReplyRevision,
                DeliveryKind::ChatTurn,
                DeliveryKind::ReplyRevision,
            ]
        );
    }
}
