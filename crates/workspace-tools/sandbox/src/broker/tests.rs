use std::sync::Arc;
use std::time::Instant;

use super::cache::{FRESH_FOR, STALE_FOR};
use super::core::Broker;
use super::keyring::{Locked, Memory};
use super::peer::{Door, Peer, PeerMap};
use super::wire::{
    AgentSecret, Asked, Denied, GhFallback, ItemReport, ItemState, Reply, Report, Request,
    Unavailable,
};
use super::{Item, Secret, SecretError, Source};

/// Shaped like a fine-grained GitHub token: `github_pat_`, 22 characters,
/// `_`, 59 characters. Fake; it never authenticated anything.
const GITHUB_PAT: &str = concat!(
    "github_pat_",
    "11AAAAAAA0FakeFakeFake",
    "_",
    "FakeTokenForUnitTestsOnly",
    "0000000000000000000000000000000000",
);
/// Shaped like a Doppler service token. Fake.
const DOPPLER_ST: &str = concat!("dp.st.", "dev.FakeTokenForUnitTestsOnly000000000000");

const AGENT: Peer = Peer {
    uid: 656_360,
    pid: 1,
};
const OWNER: Peer = Peer { uid: 1000, pid: 1 };
const ROOT_IN_CONTAINER: Peer = Peer {
    uid: 655_360,
    pid: 1,
};

fn broker(memory: &Arc<Memory>, gh_fallback: GhFallback) -> Broker {
    let peers = PeerMap {
        agent_uid: 656_360,
        owner_uid: 1000,
        ci_uid: None,
    };
    Broker::new(peers, memory.clone(), memory.clone(), gh_fallback)
}

fn ask(broker: &Broker, peer: Peer, request: &Request, now: Instant) -> Reply {
    let line = serde_json::to_vec(request).expect("serializes");
    broker.handle(Door::Agent, peer, &line, now).reply
}

fn granted(text: &str) -> Reply {
    Reply::Granted {
        secret: Secret::new(text).expect("non-empty"),
    }
}

#[test]
fn request_wire_names_are_stable_and_unknown_kinds_are_rejected() {
    let git: Request =
        serde_json::from_str(r#"{"kind":"git_credential","protocol":"https","host":"github.com"}"#)
            .expect("git credential");
    assert_eq!(git.asked(), Asked::Agent(AgentSecret::Github));
    let ci: Request = serde_json::from_str(r#"{"kind":"ci_token"}"#).expect("ci token");
    assert_eq!(ci.asked(), Asked::CiToken);
    for bad in [
        r#"{"kind":"github_ci"}"#,
        r#"{"kind":"git_credential","protocol":"http","host":"github.com"}"#,
        r#"{"kind":"git_credential","protocol":"https","host":"gitlab.com"}"#,
        r#"{"kind":"git_credential","protocol":"https","host":"github.com","path":"x"}"#,
        "not json",
    ] {
        assert!(serde_json::from_str::<Request>(bad).is_err(), "{bad}");
    }
    let reply = serde_json::to_string(&Reply::Unavailable {
        reason: Unavailable::KeyringLocked,
    })
    .expect("serializes");
    assert_eq!(
        reply,
        r#"{"outcome":"unavailable","reason":"keyring_locked"}"#
    );
}

#[test]
fn agent_gets_its_tokens_and_the_audit_line_holds_no_secret() {
    let memory = Arc::new(Memory::with([
        (Item::GithubAgent, GITHUB_PAT),
        (Item::Doppler, DOPPLER_ST),
    ]));
    let broker = broker(&memory, GhFallback::Off);
    let now = Instant::now();
    let line = br#"{"kind":"gh_token"}"#;
    let answer = broker.handle(Door::Agent, AGENT, line, now);
    assert_eq!(answer.reply, granted(GITHUB_PAT));
    let wire = serde_json::to_string(&answer.reply).expect("serializes");
    assert_eq!(
        serde_json::from_str::<Reply>(&wire).expect("reads back"),
        granted(GITHUB_PAT)
    );
    let record = answer.record.expect("audited").to_string();
    assert!(
        record.contains("gh_token granted socket=agent peer=agent"),
        "{record}"
    );
    assert!(record.contains("source=keyring sha256="), "{record}");
    assert!(!record.contains(GITHUB_PAT), "{record}");
    assert_eq!(
        ask(&broker, AGENT, &Request::DopplerToken, now),
        granted(DOPPLER_ST)
    );
}

#[test]
fn a_token_is_printable_ascii_with_no_inner_space_or_line_break() {
    assert_eq!(
        Secret::new(&format!("{GITHUB_PAT}\n"))
            .expect("a token")
            .expose(),
        GITHUB_PAT
    );
    assert_eq!(Secret::new(" \n").err(), Some(SecretError::Empty));
    for (text, at) in [
        ("tok\nurl=https://evil", 3),
        ("tok\rpassword=x", 3),
        ("tok en", 3),
        ("tok\tx", 3),
        ("tok\u{7f}", 3),
        ("tök", 1),
    ] {
        assert_eq!(
            Secret::new(text).err(),
            Some(SecretError::NotPrintable { at }),
            "{text:?}"
        );
    }
    let injected = r#"{"outcome":"granted","secret":"tok\nurl=https://evil"}"#;
    assert!(
        serde_json::from_str::<Reply>(injected).is_err(),
        "the client refuses it too"
    );
}

#[test]
fn a_keyring_value_that_is_not_a_token_is_not_served() {
    let memory = Arc::new(Memory::default());
    memory.set_not_a_token(Item::GithubAgent);
    let broker = broker(&memory, GhFallback::Off);
    assert_eq!(
        ask(&broker, AGENT, &Request::GhToken, Instant::now()),
        Reply::Unavailable {
            reason: Unavailable::NotAToken
        }
    );
}

#[test]
fn only_the_container_agent_gets_secrets_and_only_the_owner_pings() {
    let memory = Arc::new(Memory::with([(Item::GithubAgent, "s")]));
    let broker = broker(&memory, GhFallback::Off);
    let now = Instant::now();
    let wrong = Reply::Denied {
        reason: Denied::WrongPeer,
    };
    for peer in [OWNER, ROOT_IN_CONTAINER] {
        assert_eq!(ask(&broker, peer, &Request::GhToken, now), wrong);
    }
    assert_eq!(ask(&broker, AGENT, &Request::Ping, now), wrong);
    assert_eq!(
        ask(&broker, ROOT_IN_CONTAINER, &Request::Reload, now),
        wrong
    );
    assert!(matches!(
        ask(&broker, OWNER, &Request::Ping, now),
        Reply::Report { .. }
    ));
}

#[test]
fn locked_keyring_is_unavailable_not_absent_and_a_cached_token_survives_it() {
    let memory = Arc::new(Memory::with([(Item::GithubAgent, "s")]));
    let broker = broker(&memory, GhFallback::On);
    let now = Instant::now();
    memory.set_locked(Locked::Locked);
    let locked = Reply::Unavailable {
        reason: Unavailable::KeyringLocked,
    };
    assert_eq!(ask(&broker, AGENT, &Request::GhToken, now), locked);
    memory.set_locked(Locked::Open);
    assert_eq!(ask(&broker, AGENT, &Request::GhToken, now), granted("s"));
    memory.set_locked(Locked::Locked);
    let stale = now + FRESH_FOR;
    let answer = broker.handle(Door::Agent, AGENT, br#"{"kind":"gh_token"}"#, stale);
    assert_eq!(answer.reply, granted("s"));
    let record = answer.record.expect("audited").to_string();
    assert!(record.contains("reason=stale keyring_locked"), "{record}");
    broker.reload();
    assert_eq!(ask(&broker, AGENT, &Request::GhToken, stale), locked);
}

#[test]
fn a_locked_keyring_keeps_a_cached_token_only_until_the_stale_cap() {
    let memory = Arc::new(Memory::with([(Item::GithubAgent, "s")]));
    let broker = broker(&memory, GhFallback::Off);
    let now = Instant::now();
    assert_eq!(ask(&broker, AGENT, &Request::GhToken, now), granted("s"));
    memory.set_locked(Locked::Locked);
    let last_stale = now + STALE_FOR - std::time::Duration::from_secs(1);
    assert_eq!(
        ask(&broker, AGENT, &Request::GhToken, last_stale),
        granted("s")
    );
    assert_eq!(
        ask(&broker, AGENT, &Request::GhToken, now + STALE_FOR),
        Reply::Unavailable {
            reason: Unavailable::KeyringLocked
        }
    );
}

#[test]
fn a_token_deleted_from_the_wallet_stops_being_served_once_stale() {
    let memory = Arc::new(Memory::with([(Item::GithubAgent, "s")]));
    let broker = broker(&memory, GhFallback::Off);
    let now = Instant::now();
    assert_eq!(ask(&broker, AGENT, &Request::GhToken, now), granted("s"));
    memory.edit(|state| state.items.remove(&Item::GithubAgent));
    let not_stored = Reply::Unavailable {
        reason: Unavailable::NotStored,
    };
    let stale = now + FRESH_FOR;
    assert_eq!(ask(&broker, AGENT, &Request::GhToken, stale), not_stored);
    memory.set_locked(Locked::Locked);
    assert_eq!(
        ask(&broker, AGENT, &Request::GhToken, stale),
        Reply::Unavailable {
            reason: Unavailable::KeyringLocked
        },
        "a later lock does not bring the deleted token back"
    );
}

#[test]
fn ping_reports_a_stale_token_and_its_age_while_the_keyring_is_locked() {
    let memory = Arc::new(Memory::with([
        (Item::GithubAgent, "s"),
        (Item::Doppler, "d"),
    ]));
    let broker = broker(&memory, GhFallback::Off);
    let now = Instant::now();
    assert_eq!(ask(&broker, AGENT, &Request::GhToken, now), granted("s"));
    memory.set_locked(Locked::Locked);
    let stale = now + FRESH_FOR;
    let Reply::Report { report } = ask(&broker, OWNER, &Request::Ping, stale) else {
        panic!("ping reports");
    };
    assert_eq!(
        report.items,
        vec![
            ItemReport {
                item: Item::GithubAgent,
                state: ItemState::Stale {
                    source: Source::Keyring,
                    read_secs_ago: FRESH_FOR.as_secs(),
                },
            },
            ItemReport {
                item: Item::Doppler,
                state: ItemState::Unavailable {
                    reason: Unavailable::KeyringLocked
                },
            },
            ItemReport {
                item: Item::ModelProxy,
                state: ItemState::Unavailable {
                    reason: Unavailable::KeyringLocked
                },
            },
        ]
    );
}

#[test]
fn a_reload_during_a_keyring_read_keeps_the_old_read_out_of_the_cache() {
    let memory = Arc::new(Memory::with([(Item::GithubAgent, "old")]));
    let broker = Arc::new(broker(&memory, GhFallback::Off));
    let now = Instant::now();
    let reloader = Arc::clone(&broker);
    let rotated = Arc::clone(&memory);
    memory.edit(|state| {
        state.during_lookup = Some(Box::new(move || {
            rotated.edit(|state| {
                state
                    .items
                    .insert(Item::GithubAgent, Secret::new("new").expect("a token"));
            });
            reloader.reload();
        }));
    });
    assert_eq!(ask(&broker, AGENT, &Request::GhToken, now), granted("old"));
    assert_eq!(ask(&broker, AGENT, &Request::GhToken, now), granted("new"));
}

#[test]
fn cache_hits_skip_the_keyring_and_reload_reads_it_again() {
    let memory = Arc::new(Memory::with([(Item::GithubAgent, "old")]));
    let broker = broker(&memory, GhFallback::Off);
    let now = Instant::now();
    assert_eq!(ask(&broker, AGENT, &Request::GhToken, now), granted("old"));
    memory.edit(|state| {
        state
            .items
            .insert(Item::GithubAgent, Secret::new("new").expect("non-empty"));
    });
    assert_eq!(ask(&broker, AGENT, &Request::GhToken, now), granted("old"));
    assert_eq!(memory.edit(|state| state.lookups), 1);
    assert_eq!(ask(&broker, OWNER, &Request::Reload, now), Reply::Reloaded);
    assert_eq!(ask(&broker, AGENT, &Request::GhToken, now), granted("new"));
}

#[test]
fn gh_fallback_serves_only_a_missing_github_agent_and_only_when_on() {
    let memory = Arc::new(Memory::default());
    memory.edit(|state| state.gh_token = Secret::new("gh-general").ok());
    let now = Instant::now();
    let off = broker(&memory, GhFallback::Off);
    let not_stored = Reply::Unavailable {
        reason: Unavailable::NotStored,
    };
    assert_eq!(ask(&off, AGENT, &Request::GhToken, now), not_stored);
    let on = broker(&memory, GhFallback::On);
    assert_eq!(
        ask(&on, AGENT, &Request::GhToken, now),
        granted("gh-general")
    );
    assert_eq!(ask(&on, AGENT, &Request::DopplerToken, now), not_stored);
    let Reply::Report { report } = ask(&on, OWNER, &Request::Ping, now) else {
        panic!("ping reports");
    };
    assert_eq!(
        report,
        Report {
            items: vec![
                ItemReport {
                    item: Item::GithubAgent,
                    state: ItemState::Available {
                        source: Source::GhCli
                    },
                },
                ItemReport {
                    item: Item::Doppler,
                    state: ItemState::Unavailable {
                        reason: Unavailable::NotStored
                    },
                },
                ItemReport {
                    item: Item::ModelProxy,
                    state: ItemState::Unavailable {
                        reason: Unavailable::NotStored
                    },
                },
            ],
            gh_fallback: GhFallback::On,
            fallback_used: true,
        }
    );
}

#[test]
fn malformed_and_rate_limited_requests_are_denied_and_logged_once() {
    let memory = Arc::new(Memory::with([(Item::GithubAgent, "s")]));
    let broker = broker(&memory, GhFallback::Off);
    let now = Instant::now();
    let malformed = broker.handle(Door::Agent, AGENT, b"{\"kind\":", now);
    assert_eq!(
        malformed.reply,
        Reply::Denied {
            reason: Denied::Malformed
        }
    );
    for _ in 0..9 {
        assert_eq!(ask(&broker, AGENT, &Request::GhToken, now), granted("s"));
    }
    let limited = broker.handle(Door::Agent, AGENT, br#"{"kind":"gh_token"}"#, now);
    assert_eq!(
        limited.reply,
        Reply::Denied {
            reason: Denied::RateLimited
        }
    );
    assert!(
        limited.record.is_some(),
        "the first limited request is logged"
    );
    let again = broker.handle(Door::Agent, AGENT, br#"{"kind":"gh_token"}"#, now);
    assert!(again.record.is_none(), "later ones within a minute are not");
    assert!(matches!(
        ask(&broker, OWNER, &Request::Ping, now),
        Reply::Report { .. }
    ));
}

const CI: Peer = Peer { uid: 993, pid: 1 };

fn broker_with_ci(memory: &Arc<Memory>) -> Broker {
    let peers = PeerMap {
        agent_uid: AGENT.uid,
        owner_uid: OWNER.uid,
        ci_uid: Some(CI.uid),
    };
    Broker::new(peers, memory.clone(), memory.clone(), GhFallback::Off)
}

fn ask_at(broker: &Broker, door: Door, peer: Peer, request: &Request, now: Instant) -> Reply {
    let line = serde_json::to_vec(request).expect("serializes");
    broker.handle(door, peer, &line, now).reply
}

#[test]
fn the_ci_door_serves_the_ci_token_to_the_ci_user_and_nothing_else() {
    let memory = Arc::new(Memory::with([
        (Item::GithubAgent, GITHUB_PAT),
        (Item::GithubCi, "ci-token"),
    ]));
    let broker = broker_with_ci(&memory);
    let now = Instant::now();
    let wrong = Reply::Denied {
        reason: Denied::WrongPeer,
    };
    let answer = broker.handle(Door::Ci, CI, br#"{"kind":"ci_token"}"#, now);
    assert_eq!(answer.reply, granted("ci-token"));
    let record = answer.record.expect("audited").to_string();
    assert!(
        record.contains("ci_token granted socket=ci peer=ci uid=993"),
        "{record}"
    );
    assert!(!record.contains("ci-token"), "{record}");

    for peer in [AGENT, ROOT_IN_CONTAINER, OWNER] {
        assert_eq!(
            ask_at(&broker, Door::Ci, peer, &Request::CiToken, now),
            wrong,
            "{peer:?} on the CI door"
        );
    }
    for request in [Request::GhToken, Request::DopplerToken, Request::Ping] {
        for peer in [CI, AGENT, OWNER] {
            assert_eq!(
                ask_at(&broker, Door::Ci, peer, &request, now),
                wrong,
                "{request:?} from {peer:?} on the CI door"
            );
        }
    }
}

#[test]
fn the_agent_door_never_serves_the_ci_token() {
    let memory = Arc::new(Memory::with([
        (Item::GithubAgent, GITHUB_PAT),
        (Item::GithubCi, "ci-token"),
    ]));
    let broker = broker_with_ci(&memory);
    let now = Instant::now();
    let wrong = Reply::Denied {
        reason: Denied::WrongPeer,
    };
    for peer in [CI, AGENT, ROOT_IN_CONTAINER, OWNER] {
        let line = br#"{"kind":"ci_token"}"#;
        let answer = broker.handle(Door::Agent, peer, line, now);
        assert_eq!(answer.reply, wrong, "{peer:?} on the agent door");
        let record = answer.record.expect("audited").to_string();
        assert!(record.contains("reason=wrong_socket"), "{record}");
    }
    assert_eq!(
        ask_at(&broker, Door::Agent, CI, &Request::GhToken, now),
        wrong,
        "the CI user gets no agent token"
    );
    assert_eq!(memory.edit(|state| state.lookups), 0, "no keyring read");
}

#[test]
fn ping_lists_the_ci_token_only_on_a_host_with_the_ci() {
    let memory = Arc::new(Memory::with([(Item::GithubAgent, "s")]));
    let now = Instant::now();
    let items = |broker: &Broker| {
        let Reply::Report { report } = ask(broker, OWNER, &Request::Ping, now) else {
            panic!("ping reports");
        };
        report
            .items
            .into_iter()
            .map(|entry| (entry.item, entry.state))
            .collect::<Vec<_>>()
    };
    let not_stored = ItemState::Unavailable {
        reason: Unavailable::NotStored,
    };
    let available = ItemState::Available {
        source: Source::Keyring,
    };
    assert_eq!(
        items(&broker(&memory, GhFallback::Off)),
        vec![
            (Item::GithubAgent, available),
            (Item::Doppler, not_stored),
            (Item::ModelProxy, not_stored),
        ]
    );
    assert_eq!(
        items(&broker_with_ci(&memory)),
        vec![
            (Item::GithubAgent, available),
            (Item::Doppler, not_stored),
            (Item::ModelProxy, not_stored),
            (Item::GithubCi, not_stored)
        ]
    );
}

#[test]
fn a_missing_ci_token_is_unavailable_and_the_gh_fallback_never_stands_in() {
    let memory = Arc::new(Memory::default());
    memory.edit(|state| state.gh_token = Secret::new("gh-general").ok());
    let peers = PeerMap {
        agent_uid: AGENT.uid,
        owner_uid: OWNER.uid,
        ci_uid: Some(CI.uid),
    };
    let broker = Broker::new(peers, memory.clone(), memory.clone(), GhFallback::On);
    assert_eq!(
        ask_at(&broker, Door::Ci, CI, &Request::CiToken, Instant::now()),
        Reply::Unavailable {
            reason: Unavailable::NotStored
        }
    );
}

#[test]
fn the_ci_user_has_its_own_rate_limit() {
    let memory = Arc::new(Memory::with([
        (Item::GithubAgent, "s"),
        (Item::GithubCi, "c"),
    ]));
    let broker = broker_with_ci(&memory);
    let now = Instant::now();
    for _ in 0..10 {
        assert_eq!(ask(&broker, AGENT, &Request::GhToken, now), granted("s"));
    }
    assert_eq!(
        ask(&broker, AGENT, &Request::GhToken, now),
        Reply::Denied {
            reason: Denied::RateLimited
        }
    );
    assert_eq!(
        ask_at(&broker, Door::Ci, CI, &Request::CiToken, now),
        granted("c"),
        "a runaway container does not starve the controller"
    );
}
