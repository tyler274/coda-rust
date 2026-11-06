//! Persisted, minute-slot deduplication and exact alert hysteresis.
//! A generation-match write elects one publisher when scheduled jobs overlap.
use crate::{Observation, Target};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct State {
    pub minute: Option<u64>,
    pub targets: [Streak; 3],
}

#[derive(Clone, Debug, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct Streak {
    pub failures: u8,
    pub successes: u8,
    pub firing: bool,
}

impl State {
    /// Duplicate/older executions cannot advance the counters. A missing minute
    /// breaks a pending streak, but cannot recover an already firing incident.
    pub fn advance(&mut self, minute: u64, observations: &[Observation]) -> bool {
        if self.minute.is_some_and(|previous| minute <= previous) {
            return false;
        }
        if self.minute.is_none_or(|previous| minute != previous + 1) {
            for streak in &mut self.targets {
                streak.failures = 0;
                streak.successes = 0;
            }
        }
        for (index, target) in Target::ALL.iter().enumerate() {
            let streak = &mut self.targets[index];
            let Some(observation) = observations.iter().find(|o| o.target == *target) else {
                streak.failures = 0;
                streak.successes = 0;
                continue;
            };
            if observation.outcome.failed() {
                streak.successes = 0;
                streak.failures = streak.failures.saturating_add(1).min(3);
                if streak.failures == 3 {
                    streak.firing = true;
                }
            } else {
                streak.failures = 0;
                streak.successes = streak.successes.saturating_add(1).min(2);
                if streak.successes == 2 {
                    streak.firing = false;
                }
            }
        }
        self.minute = Some(minute);
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Outcome;
    fn sample(outcome: Outcome) -> [Observation; 1] {
        [Observation {
            target: Target::Physician,
            outcome,
            status: None,
            latency_ms: 0,
        }]
    }
    #[test]
    fn exact_three_failures_two_successes_and_duplicate_guard() {
        let mut state = State::default();
        for minute in 1..=3 {
            assert!(state.advance(minute, &sample(Outcome::AuthFailure)));
            assert!(!state.advance(minute, &sample(Outcome::Success)));
            assert_eq!(state.targets[0].firing, minute == 3);
        }
        state.advance(4, &sample(Outcome::Success));
        assert!(state.targets[0].firing);
        state.advance(5, &sample(Outcome::Success));
        assert!(!state.targets[0].firing);
    }
    #[test]
    fn gaps_and_interrupted_streaks_do_not_fire_or_recover() {
        let mut state = State::default();
        state.advance(1, &sample(Outcome::Timeout));
        state.advance(2, &sample(Outcome::Refused));
        state.advance(4, &sample(Outcome::TargetFailure));
        assert!(!state.targets[0].firing);
        state.advance(5, &sample(Outcome::Success));
        for minute in 6..=8 {
            state.advance(minute, &sample(Outcome::Refused));
        }
        assert!(state.targets[0].firing);
        state.advance(9, &sample(Outcome::Success));
        state.advance(11, &sample(Outcome::Success));
        assert!(state.targets[0].firing);
        state.advance(12, &[]);
        assert!(state.targets[0].firing);
        state.advance(13, &sample(Outcome::Success));
        assert!(state.targets[0].firing);
        state.advance(14, &sample(Outcome::Success));
        assert!(!state.targets[0].firing);
    }
}
