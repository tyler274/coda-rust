//! The broker's decisions, apart from sockets: who may ask, the rate limit,
//! the cache and the keyring reads. [`Broker::handle`] turns one request
//! line from one peer into a reply and its audit record.
//!
//! The state lock is held only to read or change the state, never across a
//! keyring read, so one slow `secret-tool` call does not hold up requests
//! for other items or the owner's ping.

use std::collections::HashMap;
use std::sync::{Arc, Mutex, MutexGuard, PoisonError};
use std::time::Instant;

use super::audit::{Record, Served};
use super::cache::{Cache, Entry, Held};
use super::keyring::{Fallback, Keyring, KeyringError};
use super::limit::{Bucket, LOG_EVERY, Rate};
use super::peer::{Door, Peer, PeerMap, Principal};
use super::wire::{
    Asked, Denied, GhFallback, ItemReport, ItemState, Reply, Report, Request, Unavailable,
};
use super::{Item, Secret, Source};
use crate::error::chain;

pub struct Broker {
    peers: PeerMap,
    keyring: Arc<dyn Keyring>,
    fallback: Arc<dyn Fallback>,
    gh_fallback: GhFallback,
    state: Mutex<State>,
}

#[derive(Default)]
struct State {
    cache: Cache,
    /// Bumped by every reload, so a keyring read that began before one
    /// does not put what it read back into the cache.
    generation: u64,
    buckets: HashMap<Principal, Bucket>,
    /// When each rate-limited principal last got an audit line.
    limit_logged: HashMap<Principal, Instant>,
    fallback_used: bool,
}

/// A reply and the audit line it leaves; no line for a rate-limited request
/// whose principal was already logged within [`LOG_EVERY`].
pub struct Answer {
    pub reply: Reply,
    pub record: Option<Record>,
}

/// Why a source gave no secret, and the cause for the audit line.
struct Missed {
    reason: Unavailable,
    cause: Option<String>,
}

/// A secret the broker can serve.
enum Got {
    /// Read from its source now or within the cache's fresh window.
    Read(Entry),
    /// Kept from an earlier read because the keyring is locked now.
    Kept { entry: Entry, locked: Missed },
}

/// Where a request stands once the rate limit, the parse and the peer
/// check are done.
enum Admitted {
    Asked {
        kind: &'static str,
        asked: Asked,
    },
    Refused(Outcome),
    /// Rate limited, with the principal already logged within a minute.
    Quiet(Reply),
}

/// What a handled request produced, before it becomes an audit record.
struct Outcome {
    kind: &'static str,
    reply: Reply,
    reason: Option<String>,
    served: Option<Served>,
}

impl Outcome {
    fn denied(kind: &'static str, reason: Denied, tag: &str) -> Self {
        Self {
            kind,
            reply: Reply::Denied { reason },
            reason: Some(tag.to_owned()),
            served: None,
        }
    }
}

impl Broker {
    pub fn new(
        peers: PeerMap,
        keyring: Arc<dyn Keyring>,
        fallback: Arc<dyn Fallback>,
        gh_fallback: GhFallback,
    ) -> Self {
        Self {
            peers,
            keyring,
            fallback,
            gh_fallback,
            state: Mutex::new(State::default()),
        }
    }

    fn state(&self) -> MutexGuard<'_, State> {
        self.state.lock().unwrap_or_else(PoisonError::into_inner)
    }

    /// Drops the cache, as SIGHUP and the owner's `reload` request do.
    pub fn reload(&self) {
        let mut state = self.state();
        state.cache.clear();
        state.generation += 1;
    }

    /// The items this broker serves, in the order reports list them: the
    /// agent's, then the CI controller's on a host with the CI.
    fn served(&self) -> impl Iterator<Item = Item> + use<> {
        let ci = self.peers.ci_uid.map(|_| Item::GithubCi);
        Item::AGENT.into_iter().chain(ci)
    }

    /// Answers one request line that `peer` sent through `door`.
    pub fn handle(&self, door: Door, peer: Peer, line: &[u8], now: Instant) -> Answer {
        let principal = self.peers.classify(peer);
        let outcome = match self.admit(door, principal, line, now) {
            Admitted::Asked { kind, asked } => self.answer(kind, asked, now),
            Admitted::Refused(outcome) => outcome,
            Admitted::Quiet(reply) => {
                return Answer {
                    reply,
                    record: None,
                };
            }
        };
        let record = Record {
            door,
            peer,
            comm: super::peer::comm(peer.pid),
            principal,
            kind: outcome.kind,
            outcome: outcome.reply.outcome(),
            reason: outcome.reason,
            served: outcome.served,
        };
        Answer {
            reply: outcome.reply,
            record: Some(record),
        }
    }

    fn admit(&self, door: Door, principal: Principal, line: &[u8], now: Instant) -> Admitted {
        let rate = match principal {
            Principal::Owner => Rate::OWNER,
            Principal::Ci => Rate::CI,
            Principal::Agent | Principal::Stranger => Rate::AGENT,
        };
        let mut state = self.state();
        let bucket = state
            .buckets
            .entry(principal)
            .or_insert_with(|| Bucket::new(rate, now));
        if !bucket.take(now) {
            let last = state.limit_logged.get(&principal).copied();
            let limited = Outcome::denied("limited", Denied::RateLimited, "rate_limited");
            if last.is_some_and(|at| now.saturating_duration_since(at) < LOG_EVERY) {
                return Admitted::Quiet(limited.reply);
            }
            state.limit_logged.insert(principal, now);
            return Admitted::Refused(limited);
        }
        drop(state);
        let Ok(request) = serde_json::from_slice::<Request>(line) else {
            return Admitted::Refused(Outcome::denied("malformed", Denied::Malformed, "malformed"));
        };
        let kind = request.kind();
        let asked = request.asked();
        // The door comes first: the agent's socket never answers for the
        // CI token, even to the CI's own user.
        if !door.serves(asked.audience()) {
            return Admitted::Refused(Outcome::denied(kind, Denied::WrongPeer, "wrong_socket"));
        }
        if !principal.may_ask(asked.audience()) {
            return Admitted::Refused(Outcome::denied(kind, Denied::WrongPeer, "wrong_peer"));
        }
        Admitted::Asked { kind, asked }
    }

    fn answer(&self, kind: &'static str, asked: Asked, now: Instant) -> Outcome {
        match asked {
            Asked::Reload => {
                self.reload();
                Outcome {
                    kind,
                    reply: Reply::Reloaded,
                    reason: None,
                    served: None,
                }
            }
            Asked::Ping => Outcome {
                kind,
                reply: Reply::Report {
                    report: self.report(now),
                },
                reason: None,
                served: None,
            },
            Asked::Agent(secret) => self.grant(kind, secret.item(), now),
            Asked::CiToken => self.grant(kind, Item::GithubCi, now),
        }
    }

    fn grant(&self, kind: &'static str, item: Item, now: Instant) -> Outcome {
        let got = match self.secret(item, now) {
            Ok(got) => got,
            Err(missed) => {
                return Outcome {
                    kind,
                    reason: Some(missed.describe()),
                    reply: Reply::Unavailable {
                        reason: missed.reason,
                    },
                    served: None,
                };
            }
        };
        let (entry, reason) = match got {
            Got::Read(entry) => (entry, None),
            Got::Kept { entry, locked } => (entry, Some(format!("stale {}", locked.describe()))),
        };
        Outcome {
            kind,
            reason,
            served: Some(Served {
                source: entry.source,
                fingerprint: entry.secret.fingerprint(),
            }),
            reply: Reply::Granted {
                secret: entry.secret,
            },
        }
    }

    fn report(&self, now: Instant) -> Report {
        let items = self
            .served()
            .map(|item| ItemReport {
                item,
                state: match self.secret(item, now) {
                    Ok(Got::Read(entry)) => ItemState::Available {
                        source: entry.source,
                    },
                    Ok(Got::Kept { entry, .. }) => ItemState::Stale {
                        source: entry.source,
                        read_secs_ago: now.saturating_duration_since(entry.fetched_at).as_secs(),
                    },
                    Err(missed) => ItemState::Unavailable {
                        reason: missed.reason,
                    },
                },
            })
            .collect();
        Report {
            items,
            gh_fallback: self.gh_fallback,
            fallback_used: self.state().fallback_used,
        }
    }

    /// The cached entry while fresh; otherwise a new read from its source.
    /// A stale entry is served only while the keyring is locked. Any
    /// other answer, "not stored" above all, drops it.
    fn secret(&self, item: Item, now: Instant) -> Result<Got, Missed> {
        let (stale, generation) = {
            let state = self.state();
            let stale = match state.cache.held(item, now) {
                Held::Fresh(entry) => return Ok(Got::Read(entry.clone())),
                Held::Stale(entry) => Some(entry.clone()),
                Held::Nothing => None,
            };
            (stale, state.generation)
        };
        let fetched = self.fetch(item);
        let mut state = self.state();
        let current = state.generation == generation;
        match (fetched, stale) {
            (Ok((secret, source)), _) => {
                state.fallback_used |= source == Source::GhCli;
                let entry = Entry {
                    secret,
                    source,
                    fetched_at: now,
                };
                if current {
                    state.cache.insert(item, entry.clone());
                }
                Ok(Got::Read(entry))
            }
            (Err(locked), Some(entry)) if locked.reason == Unavailable::KeyringLocked => {
                Ok(Got::Kept { entry, locked })
            }
            (Err(missed), _) => {
                if current {
                    state.cache.remove(item);
                }
                Err(missed)
            }
        }
    }

    fn fetch(&self, item: Item) -> Result<(Secret, Source), Missed> {
        let falls_back = item == Item::GithubAgent && self.gh_fallback == GhFallback::On;
        match self.keyring.lookup(item) {
            Ok(Some(secret)) => Ok((secret, Source::Keyring)),
            Ok(None) if falls_back => self
                .fallback
                .gh_token()
                .map(|secret| (secret, Source::GhCli))
                .map_err(|error| Missed {
                    reason: Unavailable::FallbackFailed,
                    cause: Some(chain(&error)),
                }),
            Ok(None) => Err(Missed {
                reason: Unavailable::NotStored,
                cause: None,
            }),
            Err(error @ KeyringError::NotAToken { .. }) => Err(Missed {
                reason: Unavailable::NotAToken,
                cause: Some(chain(&error)),
            }),
            Err(error) => Err(Missed {
                reason: Unavailable::KeyringLocked,
                cause: Some(chain(&error)),
            }),
        }
    }
}

impl Missed {
    fn describe(&self) -> String {
        let tag = match self.reason {
            Unavailable::KeyringLocked => "keyring_locked",
            Unavailable::NotStored => "not_stored",
            Unavailable::FallbackFailed => "fallback_failed",
            Unavailable::NotAToken => "not_a_token",
        };
        match &self.cause {
            Some(cause) => format!("{tag} cause={cause:?}"),
            None => tag.to_owned(),
        }
    }
}
