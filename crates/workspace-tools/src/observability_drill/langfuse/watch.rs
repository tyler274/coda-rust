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
}

#[cfg(test)]
mod tests {
    use super::*;

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
