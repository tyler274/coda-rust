//! Who is on the other end of a connection.
//!
//! The kernel reports `SO_PEERCRED` in the listener's user namespace. The
//! broker runs on the host, and the container runs with private users at
//! `uidBase`, so the container's agent (uid 1000 inside) arrives as host
//! uid `uidBase + 1000`, the config's `[container] agent_uid`, and
//! container root as `uidBase`. Container root can become the agent, so
//! the check keeps out other host users and other containers; it does not
//! split the container.

use std::os::fd::AsFd;

use super::wire::Audience;

/// The host uids the broker accepts.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PeerMap {
    /// The container's agent as the host sees it.
    pub agent_uid: u32,
    /// The host user the broker runs as.
    pub owner_uid: u32,
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
    Stranger,
}

impl PeerMap {
    pub fn classify(self, peer: Peer) -> Principal {
        if peer.uid == self.agent_uid {
            Principal::Agent
        } else if peer.uid == self.owner_uid {
            Principal::Owner
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
            (Self::Agent, Audience::Agent) | (Self::Owner, Audience::Owner)
        )
    }

    pub fn name(self) -> &'static str {
        match self {
            Self::Agent => "agent",
            Self::Owner => "owner",
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
    const MAP: PeerMap = PeerMap {
        agent_uid: UID_BASE + 1000,
        owner_uid: 1000,
    };

    fn peer(uid: u32) -> Peer {
        Peer { uid, pid: 1 }
    }

    #[test]
    fn container_agent_maps_through_uid_base() {
        assert_eq!(MAP.classify(peer(656_360)), Principal::Agent);
        assert_eq!(MAP.classify(peer(1000)), Principal::Owner);
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
        assert!(!Principal::Stranger.may_ask(Audience::Agent));
        assert!(!Principal::Stranger.may_ask(Audience::Owner));
    }
}
