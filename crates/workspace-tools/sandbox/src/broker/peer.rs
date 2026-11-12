//! Who is on the other end of a connection.
//!
//! The kernel reports `SO_PEERCRED` in the listener's user namespace. The
//! broker runs on the host, and the container runs with private users at
//! `uidBase`, so the container's agent (uid 1000 inside) arrives as host
//! uid `uidBase + 1000`, the config's `[container] agent_uid`, and
//! container root as `uidBase`. Container root can become the agent, so
//! the check keeps out other host users and other containers; it does not
//! split the container.
//!
//! The CI controller is a host system user outside every container, so its
//! uid arrives as it is. It has a socket of its own, a second [`Door`]: the
//! agent's door never answers for the CI token, whoever asks, and the CI's
//! door answers for nothing else.

use std::os::fd::AsFd;

use super::wire::Audience;

/// The host uids the broker accepts.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PeerMap {
    /// The container's agent as the host sees it.
    pub agent_uid: u32,
    /// The host user the broker runs as.
    pub owner_uid: u32,
    /// The host's CI controller user; `None` on a host without the CI.
    pub ci_uid: Option<u32>,
}

/// The socket a connection arrived on. A request for an audience the door
/// does not serve is refused whoever sent it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Door {
    /// Bound into the container: the agent's secrets, and the owner's
    /// report and reload.
    Agent,
    /// Reachable only by the CI controller's user: its GitHub token.
    Ci,
}

impl Door {
    pub fn serves(self, audience: Audience) -> bool {
        matches!(
            (self, audience),
            (Self::Agent, Audience::Agent | Audience::Owner) | (Self::Ci, Audience::Ci)
        )
    }

    pub fn name(self) -> &'static str {
        match self {
            Self::Agent => "agent",
            Self::Ci => "ci",
        }
    }
}

/// A connected peer, as the kernel names it on the host.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Peer {
    pub uid: u32,
    pub pid: i32,
}

/// What a peer is to the broker.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Principal {
    Agent,
    Owner,
    Ci,
    Stranger,
}

impl PeerMap {
    pub fn classify(self, peer: Peer) -> Principal {
        if peer.uid == self.agent_uid {
            Principal::Agent
        } else if peer.uid == self.owner_uid {
            Principal::Owner
        } else if self.ci_uid == Some(peer.uid) {
            Principal::Ci
        } else {
            Principal::Stranger
        }
    }
}

impl Principal {
    /// Whether this principal may send a request meant for `audience`.
    pub fn may_ask(self, audience: Audience) -> bool {
        matches!(
            (self, audience),
            (Self::Agent, Audience::Agent)
                | (Self::Owner, Audience::Owner)
                | (Self::Ci, Audience::Ci)
        )
    }

    pub fn name(self) -> &'static str {
        match self {
            Self::Agent => "agent",
            Self::Owner => "owner",
            Self::Ci => "ci",
            Self::Stranger => "stranger",
        }
    }
}

/// Reads the peer of a connected Unix socket.
pub fn of(socket: impl AsFd) -> std::io::Result<Peer> {
    let cred = rustix::net::sockopt::socket_peercred(socket)?;
    Ok(Peer {
        uid: cred.uid.as_raw(),
        pid: cred.pid.as_raw_nonzero().get(),
    })
}

/// The peer's `comm` as the host sees it, for the audit line; `?` when the
/// process is gone or hidden, which only blurs the audit line.
pub fn comm(pid: i32) -> String {
    std::fs::read_to_string(format!("/proc/{pid}/comm"))
        .map_or_else(|_| "?".to_owned(), |text| text.trim().to_owned())
}

#[cfg(test)]
mod tests {
    use super::*;

    const UID_BASE: u32 = 655_360;
    const CI_UID: u32 = 993;
    const MAP: PeerMap = PeerMap {
        agent_uid: UID_BASE + 1000,
        owner_uid: 1000,
        ci_uid: Some(CI_UID),
    };

    fn peer(uid: u32) -> Peer {
        Peer { uid, pid: 1 }
    }

    #[test]
    fn container_agent_maps_through_uid_base() {
        assert_eq!(MAP.classify(peer(656_360)), Principal::Agent);
        assert_eq!(MAP.classify(peer(1000)), Principal::Owner);
        assert_eq!(MAP.classify(peer(CI_UID)), Principal::Ci);
    }

    #[test]
    fn a_host_without_the_ci_has_no_ci_principal() {
        let map = PeerMap {
            ci_uid: None,
            ..MAP
        };
        assert_eq!(map.classify(peer(CI_UID)), Principal::Stranger);
    }

    #[test]
    fn each_door_serves_only_its_audiences() {
        assert!(Door::Agent.serves(Audience::Agent));
        assert!(Door::Agent.serves(Audience::Owner));
        assert!(!Door::Agent.serves(Audience::Ci));
        assert!(Door::Ci.serves(Audience::Ci));
        assert!(!Door::Ci.serves(Audience::Agent));
        assert!(!Door::Ci.serves(Audience::Owner));
    }

    #[test]
    fn container_root_other_users_and_other_containers_are_strangers() {
        let other_container = 2 * UID_BASE + 1000;
        for uid in [UID_BASE, 0, 1001, other_container, 2000] {
            assert_eq!(MAP.classify(peer(uid)), Principal::Stranger, "{uid}");
        }
    }

    #[test]
    fn each_principal_asks_only_its_audience() {
        assert!(Principal::Agent.may_ask(Audience::Agent));
        assert!(!Principal::Agent.may_ask(Audience::Owner));
        assert!(Principal::Owner.may_ask(Audience::Owner));
        assert!(!Principal::Owner.may_ask(Audience::Agent));
        assert!(Principal::Ci.may_ask(Audience::Ci));
        assert!(!Principal::Ci.may_ask(Audience::Agent));
        assert!(!Principal::Ci.may_ask(Audience::Owner));
        assert!(!Principal::Agent.may_ask(Audience::Ci));
        assert!(!Principal::Owner.may_ask(Audience::Ci));
        for audience in [Audience::Agent, Audience::Owner, Audience::Ci] {
            assert!(!Principal::Stranger.may_ask(audience));
        }
    }
}
