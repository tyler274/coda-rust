//! `physician_sessions.expires_at` stores the idle deadline. The absolute cap
//! is seven days after `created_at` and is not a second column. A fixed
//! session also stores `ends_at`, a hard end activity never moves past.

/// What a recorded physician request did to the stored deadline.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SessionActivity {
    /// The stored deadline already covered this moment. Nothing was written.
    Covered,
    /// The stored deadline moved forward.
    Extended,
    /// The session was already over, or it is gone. Nothing was written.
    Ended,
}

/// What [`SessionLifetime::renew`] decides.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SessionRenewal {
    /// The stored deadline already covers this moment.
    Covered,
    /// The idle deadline should move to this Unix second.
    Extend {
        /// The new idle deadline, already clamped to the absolute cap.
        idle_deadline: u64,
    },
    /// The session no longer allows `now`, so nothing is written.
    Ended,
}

/// The idle deadline and the absolute cap for one session.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SessionLifetime {
    absolute_end: u64,
    idle_deadline: u64,
}

const HOUR: u64 = 60 * 60;
const IDLE_LIMIT: u64 = 12 * HOUR;
const ABSOLUTE_CAP: u64 = 7 * 24 * HOUR;
const ACTIVITY_QUANTUM: u64 = 15 * 60;

impl SessionLifetime {
    /// A new sign-in. The idle deadline is 12 hours out, inside the 7-day cap.
    pub fn mint(now: u64) -> Self {
        Self::from_persisted(now, now.saturating_add(IDLE_LIMIT), None)
    }

    /// A session that ends at `ends_at` however much it is used. Activity
    /// never moves its deadline; the seven-day cap still applies.
    pub fn fixed(now: u64, ends_at: u64) -> Self {
        Self::from_persisted(now, ends_at, Some(ends_at))
    }

    /// Rebuilds a lifetime from the mint time, the stored deadline and the
    /// stored hard end (`physician_sessions.ends_at`), if the session has one.
    pub fn from_persisted(created_at: u64, expires_at: u64, ends_at: Option<u64>) -> Self {
        let cap = created_at.saturating_add(ABSOLUTE_CAP);
        let absolute_end = ends_at.map_or(cap, |end| end.min(cap));
        Self {
            absolute_end,
            idle_deadline: expires_at.min(absolute_end),
        }
    }

    /// The same session with its idle deadline moved to `idle_deadline`,
    /// still inside its absolute end.
    pub fn with_idle_deadline(self, idle_deadline: u64) -> Self {
        Self {
            absolute_end: self.absolute_end,
            idle_deadline: idle_deadline.min(self.absolute_end),
        }
    }

    /// The Unix second stored in `physician_sessions.expires_at`.
    pub fn expires_at(self) -> u64 {
        self.idle_deadline
    }

    /// Whether `now` is strictly before the idle deadline.
    pub fn allows(self, now: u64) -> bool {
        now < self.idle_deadline
    }

    /// Whether activity at `now` should move the idle deadline.
    pub fn renew(self, now: u64) -> SessionRenewal {
        if !self.allows(now) {
            return SessionRenewal::Ended;
        }
        let desired = self.absolute_end.min(now.saturating_add(IDLE_LIMIT));
        if desired <= self.idle_deadline {
            return SessionRenewal::Covered;
        }
        let inside_quantum = desired < self.idle_deadline.saturating_add(ACTIVITY_QUANTUM);
        if inside_quantum && desired < self.absolute_end {
            return SessionRenewal::Covered;
        }
        SessionRenewal::Extend {
            idle_deadline: desired,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const NOW: u64 = 1_700_000_000;
    const TWELVE_HOURS: u64 = 12 * 60 * 60;
    const SEVEN_DAYS: u64 = 7 * 24 * 60 * 60;
    const FIFTEEN_MINUTES: u64 = 15 * 60;

    #[test]
    fn mint_allows_twelve_hours_and_stops_at_seven_days() {
        let life = SessionLifetime::mint(NOW);
        assert_eq!(life.expires_at(), NOW + TWELVE_HOURS);
        assert!(life.allows(NOW));
        assert!(life.allows(NOW + TWELVE_HOURS - 1));
        assert!(!life.allows(NOW + TWELVE_HOURS));

        let at_cap = SessionLifetime::from_persisted(NOW, NOW + SEVEN_DAYS, None);
        assert_eq!(at_cap.expires_at(), NOW + SEVEN_DAYS);
        assert!(at_cap.allows(NOW + SEVEN_DAYS - 1));
        assert!(!at_cap.allows(NOW + SEVEN_DAYS));
    }

    #[test]
    fn a_persisted_deadline_past_the_cap_is_clamped() {
        let life = SessionLifetime::from_persisted(NOW, NOW + 30 * 24 * 60 * 60, None);
        assert_eq!(life.expires_at(), NOW + SEVEN_DAYS);
    }

    #[test]
    fn renew_skips_a_slide_inside_fifteen_minutes() {
        let life = SessionLifetime::mint(NOW);
        assert_eq!(life.renew(NOW), SessionRenewal::Covered);
        assert_eq!(
            life.renew(NOW + FIFTEEN_MINUTES - 1),
            SessionRenewal::Covered
        );

        let idle_deadline = extend(life.renew(NOW + FIFTEEN_MINUTES));
        assert_eq!(idle_deadline, NOW + FIFTEEN_MINUTES + TWELVE_HOURS);
        let slid = SessionLifetime::from_persisted(NOW, idle_deadline, None);
        assert_eq!(
            slid.renew(NOW + FIFTEEN_MINUTES),
            SessionRenewal::Covered,
            "a second call at the same moment does not extend again"
        );
    }

    #[test]
    fn renew_pins_to_the_cap_inside_the_quantum() {
        let cap = NOW + SEVEN_DAYS;
        let idle = cap - 10 * 60;
        let life = SessionLifetime::from_persisted(NOW, idle, None);
        let now = idle - 1;
        assert!(life.allows(now));
        assert_eq!(extend(life.renew(now)), cap);

        let pinned = SessionLifetime::from_persisted(NOW, cap, None);
        assert_eq!(pinned.renew(now), SessionRenewal::Covered);
        assert_eq!(pinned.expires_at(), cap);
    }

    #[test]
    fn a_dead_session_does_not_renew() {
        let life = SessionLifetime::mint(NOW);
        let deadline = life.expires_at();
        assert_eq!(life.renew(deadline), SessionRenewal::Ended);
        assert_eq!(life.renew(deadline + 1), SessionRenewal::Ended);
        assert_eq!(life.expires_at(), deadline);
    }

    #[test]
    fn a_fixed_session_ends_on_time_however_much_it_is_used() {
        let end = NOW + FIFTEEN_MINUTES * 2;
        let life = SessionLifetime::fixed(NOW, end);
        assert_eq!(life.expires_at(), end);
        let mut now = NOW;
        while now < end {
            assert!(life.allows(now));
            assert_eq!(life.renew(now), SessionRenewal::Covered);
            now += 60;
        }
        assert!(!life.allows(end));
        assert_eq!(life.renew(end), SessionRenewal::Ended);

        let persisted = SessionLifetime::from_persisted(NOW, end, Some(end));
        assert_eq!(persisted, life);
        let stretched = SessionLifetime::from_persisted(NOW, NOW + TWELVE_HOURS, Some(end));
        assert_eq!(
            stretched.expires_at(),
            end,
            "a stored idle deadline cannot outlast ends_at"
        );
        assert_eq!(life.with_idle_deadline(NOW + TWELVE_HOURS), life);
    }

    fn extend(renewal: SessionRenewal) -> u64 {
        match renewal {
            SessionRenewal::Extend { idle_deadline } => idle_deadline,
            other => panic!("expected an extension, got {other:?}"),
        }
    }
}
