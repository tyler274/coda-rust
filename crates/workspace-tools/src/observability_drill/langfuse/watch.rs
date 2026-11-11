//! The bound every drill wait runs under: the exercise's 40-minute cap and
//! the operator's interrupt. Each poll pauses through [`Watch::pause`], so a
//! cap or a Ctrl-C ends the wait with an error the caller restores on, rather
//! than killing the process with a workload still scaled down.
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;

use chrono::{DateTime, Utc};
use signal_hook::consts::{SIGINT, SIGTERM};

use super::DrillError;
use crate::observability_drill::model::MAX_EXERCISE_SECONDS;

pub(super) struct Watch {
    deadline: DateTime<Utc>,
    interrupted: Arc<AtomicBool>,
}

impl Watch {
    /// Watches SIGINT and SIGTERM. A first signal sets the flag the next
    /// pause reads; a second one exits at once, for an operator who chooses
    /// not to wait for the restore.
    pub(super) fn install(opened: DateTime<Utc>) -> Result<Self, DrillError> {
        let interrupted = Arc::new(AtomicBool::new(false));
        for signal in [SIGINT, SIGTERM] {
            signal_hook::flag::register_conditional_shutdown(signal, 1, Arc::clone(&interrupted))
                .and_then(|_| signal_hook::flag::register(signal, Arc::clone(&interrupted)))
                .map_err(|source| DrillError::Io {
                    what: "the interrupt handler",
                    source,
                })?;
        }
        Ok(Self::new(opened, interrupted))
    }

    fn new(opened: DateTime<Utc>, interrupted: Arc<AtomicBool>) -> Self {
        Self {
            deadline: opened + chrono::Duration::seconds(MAX_EXERCISE_SECONDS),
            interrupted,
        }
    }

    /// Moves the cap to an exercise that opened at `opened`.
    pub(super) fn opened_at(&mut self, opened: DateTime<Utc>) {
        self.deadline = opened + chrono::Duration::seconds(MAX_EXERCISE_SECONDS);
    }

    /// Fails once the operator interrupted or the cap passed.
    pub(super) fn check(&self) -> Result<(), DrillError> {
        if self.interrupted.load(Ordering::SeqCst) {
            return Err(DrillError::Interrupted);
        }
        if Utc::now() > self.deadline {
            return Err(DrillError::Cap);
        }
        Ok(())
    }

    /// Sleeps `pause` in short steps, checking between them.
    pub(super) fn pause(&self, pause: Duration) -> Result<(), DrillError> {
        let step = Duration::from_millis(250);
        let mut left = pause;
        while !left.is_zero() {
            self.check()?;
            let now = left.min(step);
            std::thread::sleep(now);
            left = left.saturating_sub(now);
        }
        self.check()
    }

    /// Runs `read` up to `polls` times, `pause` apart, until it finds what it
    /// waits for. A read that times out or cannot connect counts as one more
    /// poll; any other error ends the wait at once. `Ok(None)` means every
    /// poll ran and the last one answered without it; if the last one failed
    /// to connect or timed out instead, that error is the answer.
    pub(super) fn poll<T>(
        &self,
        polls: u32,
        pause: Duration,
        mut read: impl FnMut(u32) -> Result<Option<T>, DrillError>,
    ) -> Result<Option<T>, DrillError> {
        let mut last = None;
        for poll in 0..polls {
            match read(poll) {
                Ok(Some(found)) => return Ok(Some(found)),
                Ok(None) => last = None,
                Err(error) if error.is_transient() => last = Some(error),
                Err(error) => return Err(error),
            }
            self.pause(pause)?;
        }
        match last {
            None => Ok(None),
            Some(error) => Err(DrillError::PollsFailed {
                polls,
                last: Box::new(error),
            }),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn watch() -> Watch {
        Watch::new(Utc::now(), Arc::new(AtomicBool::new(false)))
    }

    /// A real connect failure: nothing listens on a port just released.
    fn refused() -> DrillError {
        let port = std::net::TcpListener::bind("127.0.0.1:0")
            .and_then(|listener| listener.local_addr())
            .map(|address| address.port())
            .unwrap();
        let source = reqwest::blocking::get(format!("http://127.0.0.1:{port}/")).unwrap_err();
        DrillError::Request {
            what: "a refused read",
            source,
        }
    }

    #[test]
    fn a_transient_read_is_one_more_poll() {
        let mut reads = 0;
        let found = watch().poll(5, Duration::ZERO, |_| {
            reads += 1;
            if reads < 3 {
                Err(refused())
            } else {
                Ok(Some(reads))
            }
        });
        assert_eq!(found.unwrap(), Some(3));
    }

    #[test]
    fn a_status_error_ends_the_wait_at_once() {
        let mut reads = 0;
        let found: Result<Option<()>, _> = watch().poll(5, Duration::ZERO, |_| {
            reads += 1;
            Err(DrillError::Status {
                what: "a read",
                status: 503,
            })
        });
        assert!(matches!(found, Err(DrillError::Status { status: 503, .. })));
        assert_eq!(reads, 1);
    }

    #[test]
    fn polls_that_keep_failing_keep_the_last_cause() {
        let found: Result<Option<()>, _> = watch().poll(3, Duration::ZERO, |_| Err(refused()));
        let Err(DrillError::PollsFailed { polls, last }) = found else {
            panic!("expected PollsFailed, got {found:?}");
        };
        assert_eq!(polls, 3);
        assert!(matches!(*last, DrillError::Request { .. }));
        assert!(std::error::Error::source(&DrillError::PollsFailed { polls, last }).is_some());
    }

    #[test]
    fn an_answer_after_a_transient_read_is_a_plain_miss() {
        let found: Option<()> = watch()
            .poll(2, Duration::ZERO, |poll| {
                if poll == 0 { Err(refused()) } else { Ok(None) }
            })
            .unwrap();
        assert_eq!(found, None);
    }

    #[test]
    fn an_interrupt_or_the_cap_ends_a_wait() {
        let flag = Arc::new(AtomicBool::new(false));
        let watch = Watch::new(Utc::now(), Arc::clone(&flag));
        assert!(watch.pause(Duration::from_millis(1)).is_ok());
        flag.store(true, Ordering::SeqCst);
        assert!(matches!(
            watch.pause(Duration::from_secs(60)),
            Err(DrillError::Interrupted)
        ));

        let mut late = Watch::new(Utc::now(), Arc::new(AtomicBool::new(false)));
        late.opened_at(Utc::now() - chrono::Duration::seconds(MAX_EXERCISE_SECONDS + 1));
        assert!(matches!(late.check(), Err(DrillError::Cap)));
    }
}
