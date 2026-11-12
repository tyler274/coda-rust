//! The optional observer roles. Besides waking on mentions like any member,
//! an observer takes a turn on its own timer, but only when there is
//! something to look at.

use std::time::Duration;

use crate::message::{Body, Message, Seq};
use crate::names::MemberName;
use crate::roster::{MemberState, Role, Roster};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum Observer {
    Narrator,
    Monitor,
}

impl Observer {
    pub(super) fn of(role: Role) -> Option<Self> {
        match role {
            Role::Narrator => Some(Self::Narrator),
            Role::Monitor => Some(Self::Monitor),
            Role::Coordinator | Role::Worker => None,
        }
    }

    pub(super) fn every(self, config: &super::SupervisorConfig) -> Duration {
        match self {
            Self::Narrator => config.narrator_every,
            Self::Monitor => config.monitor_every,
        }
    }

    /// Prepended to the member's brief on its first turn.
    pub(super) fn role_prompt(self, coordinator: &MemberName, every: Duration) -> String {
        let minutes = every.as_secs().div_ceil(60).max(1);
        match self {
            Self::Narrator => format!(
                "You are the swarm's narrator. You keep the human informed; you do not do \
                 the work or direct anyone. About every {minutes} min, when others have \
                 posted, you get a tick. On each tick, read the board since your last \
                 summary and post one short progress summary: what finished, what is in \
                 flight and who owns it, what is blocked, and anything that needs the \
                 human. Cite post numbers (#N) instead of quoting. Your posts are \
                 summaries and wake nobody, so never use them to ask a member for \
                 anything. If @{coordinator} relays feedback from the human, adjust your \
                 summaries to it."
            ),
            Self::Monitor => format!(
                "You are the swarm's monitor. You watch for trouble; you do not do the work. \
                 About every {minutes} min, while any worker is live, you get a tick. On \
                 each tick, call `swarm` with action status and read the board, then look \
                 for: a member working with no post for a long time, a member repeating the \
                 same post or step, a member stopped for quota, and a swarm where everyone \
                 is waiting on someone else. Post only when you find one, as one line \
                 starting with [stall], [loop] or [quota] that mentions @{coordinator} and \
                 names the member and the evidence (#N). Do not repeat an alert for a \
                 condition you already reported unless it got worse. When nothing is wrong, \
                 end your turn without posting."
            ),
        }
    }

    pub(super) fn tick_prompt(self, head: Seq) -> String {
        let role = match self {
            Self::Narrator => "narrator",
            Self::Monitor => "monitor",
        };
        format!("[{role} tick] The board is at #{head}. Do your periodic check now.")
    }

    /// Whether a tick is worth a turn. `moved` says someone else posted
    /// since the previous tick, so an observer's own posts never re-trigger
    /// it. The narrator needs movement. The monitor needs a live worker and
    /// either movement or a worker that is busy or paused, so a swarm that
    /// has gone quiet gets one last look and then no more ticks.
    pub(super) fn due(self, roster: &Roster, moved: bool) -> bool {
        match self {
            Self::Narrator => moved,
            Self::Monitor => {
                let mut workers = roster
                    .members
                    .values()
                    .filter(|m| m.role == Role::Worker && m.state.is_live());
                workers.any(|m| {
                    moved || matches!(m.state, MemberState::Working | MemberState::Stopped)
                })
            }
        }
    }

    /// Whether `message` counts as movement for the observer `name`.
    pub(super) fn moves(name: &MemberName, message: &Message) -> bool {
        message.agent != *name && matches!(message.body, Body::Post(_))
    }
}
