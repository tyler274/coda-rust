//! The four states an article is in, and the moves between them.
//!
//! Every article a source has qualified is in exactly one state in its
//! source's item records. The nightly only ever moves an article through
//! [`ArticleState::apply`], which refuses a move the rules don't allow, so no
//! code path can, for example, retry an article that is waiting for a person.

use std::fmt;

use crate::failure::FailureCode;

/// How many outside failures in a row put an article in front of a person.
///
/// An article rejected on its third failed attempt needs a person.
pub const RETRY_LIMIT: u8 = 3;

/// Where one article stands.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ArticleState {
    /// Admitted: the library holds its current content.
    Admitted,
    /// Waiting to retry: due on the next night. A newly qualified article
    /// starts here with no failures.
    WaitingToRetry(Retry),
    /// Waiting for a change: retrying would fail the same way, so it waits
    /// until its source content or the reader changes.
    WaitingForChange {
        /// Why it can't be admitted as it is.
        reason: WaitReason,
    },
    /// Needs a person: automatic attempts have stopped.
    NeedsPerson {
        /// The outside failure that reached the retry limit.
        last_failure: FailureCode,
    },
}

impl ArticleState {
    /// The state a newly qualified article starts in.
    pub const fn new_article() -> Self {
        Self::WaitingToRetry(Retry {
            failures: 0,
            last_failure: None,
        })
    }

    /// Whether a person should be told about this article now, rather than
    /// waiting for a retry limit. Our own defect is reported at once.
    pub const fn tells_person_now(&self) -> bool {
        matches!(
            self,
            Self::NeedsPerson { .. }
                | Self::WaitingForChange {
                    reason: WaitReason::OwnDefect
                }
        )
    }

    /// Whether tonight's run should try this article.
    pub const fn is_due(&self) -> bool {
        matches!(self, Self::WaitingToRetry(_))
    }

    /// Applies one move, or refuses it and leaves the state as it was.
    pub fn apply(&self, event: &ArticleEvent) -> Result<Self, InvalidMove> {
        use ArticleEvent as E;
        let next = match (self, event) {
            (Self::WaitingToRetry(_), E::Admitted) => Self::Admitted,
            (Self::WaitingToRetry(retry), E::OutsideFailure(code)) => {
                let failures = retry.failures.saturating_add(1);
                if failures >= RETRY_LIMIT {
                    Self::NeedsPerson {
                        last_failure: code.clone(),
                    }
                } else {
                    Self::WaitingToRetry(Retry {
                        failures,
                        last_failure: Some(code.clone()),
                    })
                }
            }
            (Self::WaitingToRetry(_), E::Refused(reason)) => {
                Self::WaitingForChange { reason: *reason }
            }
            // New content starts the retries over; nothing else does.
            (
                Self::Admitted | Self::WaitingToRetry(_) | Self::WaitingForChange { .. },
                E::SourceChanged,
            )
            | (Self::WaitingForChange { .. }, E::ReaderChanged)
            | (Self::NeedsPerson { .. }, E::PersonResolved) => Self::new_article(),
            _ => {
                return Err(InvalidMove {
                    from: self.kind(),
                    event: event.kind(),
                });
            }
        };
        Ok(next)
    }

    /// The state's name, as stored.
    pub const fn kind(&self) -> StateKind {
        match self {
            Self::Admitted => StateKind::Admitted,
            Self::WaitingToRetry(_) => StateKind::WaitingToRetry,
            Self::WaitingForChange { .. } => StateKind::WaitingForChange,
            Self::NeedsPerson { .. } => StateKind::NeedsPerson,
        }
    }
}

/// An article waiting to retry: how many outside failures in a row it has
/// had, always below [`RETRY_LIMIT`], and the latest. Only the article's
/// moves make one, so no caller can build a retry past the limit.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Retry {
    failures: u8,
    last_failure: Option<FailureCode>,
}

impl Retry {
    /// Outside failures in a row so far.
    pub const fn failures(&self) -> u8 {
        self.failures
    }

    /// The latest outside failure, if there has been one.
    pub fn last_failure(&self) -> Option<&FailureCode> {
        self.last_failure.as_ref()
    }

    /// Rebuilds a stored retry, or `None` if no move could have produced it:
    /// at or past the limit, or a failure count and latest failure that
    /// disagree.
    pub(crate) fn from_stored(failures: u8, last_failure: Option<FailureCode>) -> Option<Self> {
        (failures < RETRY_LIMIT && (failures == 0) == last_failure.is_none()).then_some(Self {
            failures,
            last_failure,
        })
    }
}

/// Why an article waits for a change instead of retrying.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WaitReason {
    /// The source has no text the reader can read, such as a PDF-only article.
    NoReadableText,
    /// Our own code couldn't admit an article that qualified. A person is
    /// told at once.
    OwnDefect,
    /// It qualified under a reader that has since been replaced.
    SupersededReader,
}

impl WaitReason {
    /// The stored name.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::NoReadableText => "no_readable_text",
            Self::OwnDefect => "own_defect",
            Self::SupersededReader => "superseded_reader",
        }
    }

    /// Reads a stored name.
    pub fn parse(value: &str) -> Option<Self> {
        [
            Self::NoReadableText,
            Self::OwnDefect,
            Self::SupersededReader,
        ]
        .into_iter()
        .find(|reason| reason.as_str() == value)
    }
}

/// Something that happened to an article.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ArticleEvent {
    /// Tonight's attempt admitted it.
    Admitted,
    /// Tonight's attempt failed for an outside reason worth retrying, such as
    /// a timeout or a server error.
    OutsideFailure(FailureCode),
    /// Tonight's attempt showed that retrying unchanged input would fail the
    /// same way.
    Refused(WaitReason),
    /// The source published new content for it.
    SourceChanged,
    /// The reader that parses it changed.
    ReaderChanged,
    /// A person resolved what stopped it.
    PersonResolved,
}

impl ArticleEvent {
    /// The event's name, for errors.
    pub const fn kind(&self) -> &'static str {
        match self {
            Self::Admitted => "admitted",
            Self::OutsideFailure(_) => "outside_failure",
            Self::Refused(_) => "refused",
            Self::SourceChanged => "source_changed",
            Self::ReaderChanged => "reader_changed",
            Self::PersonResolved => "person_resolved",
        }
    }
}

/// A state's name, as stored.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StateKind {
    /// See [`ArticleState::Admitted`].
    Admitted,
    /// See [`ArticleState::WaitingToRetry`].
    WaitingToRetry,
    /// See [`ArticleState::WaitingForChange`].
    WaitingForChange,
    /// See [`ArticleState::NeedsPerson`].
    NeedsPerson,
}

impl StateKind {
    /// The stored name.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Admitted => "admitted",
            Self::WaitingToRetry => "waiting_to_retry",
            Self::WaitingForChange => "waiting_for_change",
            Self::NeedsPerson => "needs_person",
        }
    }
}

impl fmt::Display for StateKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// A move the rules don't allow.
#[derive(Clone, Copy, Debug, PartialEq, Eq, thiserror::Error)]
#[error("an article that is {from} can't take the {event} move")]
pub struct InvalidMove {
    /// The state the article was in, unchanged.
    pub from: StateKind,
    /// The refused event.
    pub event: &'static str,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn code() -> FailureCode {
        FailureCode::new("ncbi_timeout").unwrap()
    }

    fn states() -> Vec<ArticleState> {
        let mut states = vec![
            ArticleState::Admitted,
            ArticleState::NeedsPerson {
                last_failure: code(),
            },
        ];
        for failures in 0..RETRY_LIMIT {
            let retry = Retry::from_stored(failures, (failures > 0).then(code)).unwrap();
            states.push(ArticleState::WaitingToRetry(retry));
        }
        for reason in [
            WaitReason::NoReadableText,
            WaitReason::OwnDefect,
            WaitReason::SupersededReader,
        ] {
            states.push(ArticleState::WaitingForChange { reason });
        }
        states
    }

    fn events() -> Vec<ArticleEvent> {
        vec![
            ArticleEvent::Admitted,
            ArticleEvent::OutsideFailure(code()),
            ArticleEvent::Refused(WaitReason::NoReadableText),
            ArticleEvent::Refused(WaitReason::OwnDefect),
            ArticleEvent::Refused(WaitReason::SupersededReader),
            ArticleEvent::SourceChanged,
            ArticleEvent::ReaderChanged,
            ArticleEvent::PersonResolved,
        ]
    }

    /// The rules each single move must keep, whatever the state.
    fn check_move(
        from: &ArticleState,
        event: &ArticleEvent,
        to: &Result<ArticleState, InvalidMove>,
    ) {
        let Ok(to) = to else {
            return;
        };
        // Only a person moves an article out of needs-a-person.
        if matches!(from, ArticleState::NeedsPerson { .. }) {
            assert_eq!(
                event,
                &ArticleEvent::PersonResolved,
                "{from:?} left by {event:?}"
            );
        }
        // Only a change moves an article out of waiting-for-a-change.
        if matches!(from, ArticleState::WaitingForChange { .. }) {
            assert!(
                matches!(
                    event,
                    ArticleEvent::SourceChanged | ArticleEvent::ReaderChanged
                ),
                "{from:?} left by {event:?}"
            );
        }
        // Only an attempt that was due admits an article.
        if matches!(to, ArticleState::Admitted) && from != &ArticleState::Admitted {
            assert!(from.is_due() && event == &ArticleEvent::Admitted);
        }
        // An article is never due with the retry limit reached.
        if let ArticleState::WaitingToRetry(retry) = to {
            assert!(retry.failures() < RETRY_LIMIT);
            assert_eq!(retry.failures() == 0, retry.last_failure().is_none());
        }
        // Nothing but tonight's attempt fails an article that wasn't due.
        if matches!(
            event,
            ArticleEvent::OutsideFailure(_) | ArticleEvent::Refused(_)
        ) {
            assert!(from.is_due(), "{from:?} failed without being tried");
        }
    }

    /// AC5: every move from every reachable state keeps the rules, and a
    /// refused move leaves the state unchanged. The state space is small, so
    /// this checks all of it rather than a random sample: it starts from
    /// every state and follows every move until no new state appears.
    #[test]
    fn every_reachable_move_keeps_the_rules() {
        let all_events = events();
        let mut seen = states();
        let mut frontier = seen.clone();
        let mut checked = 0usize;
        while !frontier.is_empty() {
            let mut next = Vec::new();
            for from in &frontier {
                for event in &all_events {
                    let before = from.clone();
                    let to = from.apply(event);
                    assert_eq!(from, &before, "apply changed its input");
                    check_move(from, event, &to);
                    checked += 1;
                    if let Ok(to) = to
                        && !seen.contains(&to)
                    {
                        seen.push(to.clone());
                        next.push(to);
                    }
                }
            }
            frontier = next;
        }
        // Every state and every event, at least once.
        assert!(checked >= states().len() * events().len());
        // Every state the rules can produce was among the starting states.
        assert_eq!(seen.len(), states().len());
    }

    #[test]
    fn the_third_outside_failure_needs_a_person() {
        let failure = ArticleEvent::OutsideFailure(code());
        let mut state = ArticleState::new_article();
        for _ in 1..RETRY_LIMIT {
            state = state.apply(&failure).unwrap();
            assert!(state.is_due());
        }
        state = state.apply(&failure).unwrap();
        assert_eq!(
            state,
            ArticleState::NeedsPerson {
                last_failure: code()
            }
        );
        assert!(state.tells_person_now());
        assert!(state.apply(&failure).is_err());
    }

    #[test]
    fn our_own_defect_waits_for_a_change_and_tells_a_person_at_once() {
        let state = ArticleState::new_article()
            .apply(&ArticleEvent::Refused(WaitReason::OwnDefect))
            .unwrap();
        assert!(state.tells_person_now());
        assert!(!state.is_due());
        assert_eq!(
            state.apply(&ArticleEvent::ReaderChanged).unwrap(),
            ArticleState::new_article()
        );
    }

    #[test]
    fn a_stored_retry_must_be_one_a_move_could_make() {
        assert!(Retry::from_stored(RETRY_LIMIT, Some(code())).is_none());
        assert!(Retry::from_stored(1, None).is_none());
        assert!(Retry::from_stored(0, Some(code())).is_none());
        assert!(Retry::from_stored(RETRY_LIMIT - 1, Some(code())).is_some());
    }

    #[test]
    fn invalid_moves_say_what_was_refused() {
        let error = ArticleState::Admitted
            .apply(&ArticleEvent::PersonResolved)
            .unwrap_err();
        assert_eq!(
            error.to_string(),
            "an article that is admitted can't take the person_resolved move"
        );
    }
}
