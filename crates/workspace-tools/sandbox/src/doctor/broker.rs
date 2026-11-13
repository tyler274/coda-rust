//! The token broker's checks: its socket answers, which source serves each
//! token, the CI controller's included on a host with the CI, and whether
//! the `gh auth token` fallback is on or was used.

use super::{Fix, Outcome};
use crate::broker::error::BrokerError;
use crate::broker::wire::{GhFallback, ItemReport, ItemState, Report, Unavailable};
use crate::broker::{Item, Need, Source};
use crate::config::{Config, ModelProxy};
use crate::error::Error;
use crate::probe::Probe;

/// The user unit that runs `coda-sandbox broker serve`.
pub const UNIT: &str = "coda-sandbox-broker.service";

pub fn outcome(probe: &dyn Probe, config: &Config) -> Outcome {
    let broker = &config.broker;
    match probe.broker(&broker.socket) {
        Ok(report) => report_outcome(&report, config.model_proxy.as_ref()),
        Err(error @ BrokerError::Connect { .. }) => Outcome::Fail {
            detail: crate::error::chain(&error),
            fix: Fix::command("systemctl", ["--user", "restart", UNIT]),
        },
        Err(error) => Outcome::CannotCheck {
            cause: Error::Broker(error),
        },
    }
}

fn report_outcome(report: &Report, proxy: Option<&ModelProxy>) -> Outcome {
    let detail = describe(report, proxy);
    let state = |item| {
        report
            .items
            .iter()
            .find(|entry| entry.item == item)
            .map(|entry| entry.state)
    };
    if let Some(ItemState::Unavailable { reason }) = state(Item::GithubAgent) {
        return Outcome::Fail {
            detail,
            fix: unavailable_fix(Item::GithubAgent, reason),
        };
    }
    let via_gh = matches!(
        state(Item::GithubAgent),
        Some(
            ItemState::Available {
                source: Source::GhCli
            } | ItemState::Stale {
                source: Source::GhCli,
                ..
            }
        )
    );
    if via_gh || report.fallback_used || report.gh_fallback == GhFallback::On {
        return Outcome::Warn {
            detail: format!(
                "{detail}; the gh auth token fallback {}, and it reaches every repository you can",
                if report.fallback_used {
                    "served a token"
                } else {
                    "is on"
                },
            ),
            fix: Fix::instruction(
                "store the fine-grained token with `coda-sandbox broker store github-agent`, \
                 set services.coda-sandbox.broker.ghFallback = false, then `sudo nixos-rebuild switch`",
            ),
        };
    }
    for entry @ ItemReport { item, state } in &report.items {
        if absent_by_choice(entry, proxy) {
            continue;
        }
        match *state {
            ItemState::Stale { .. } => {
                return Outcome::Warn {
                    detail,
                    fix: unavailable_fix(*item, Unavailable::KeyringLocked),
                };
            }
            ItemState::Unavailable { reason } => {
                return Outcome::Warn {
                    detail,
                    fix: unavailable_fix(*item, reason),
                };
            }
            ItemState::Available { .. } => {}
        }
    }
    Outcome::Pass { detail }
}

/// An optional key that was never stored is the default: the agent CLI
/// keeps its own login instead, so it is neither listed nor warned about.
/// The model proxy's key is not optional on a host that configures it.
fn absent_by_choice(entry: &ItemReport, proxy: Option<&ModelProxy>) -> bool {
    ModelProxy::need(proxy, entry.item) == Need::Optional
        && entry.state
            == ItemState::Unavailable {
                reason: Unavailable::NotStored,
            }
}

fn unavailable_fix(item: Item, reason: Unavailable) -> Fix {
    match reason {
        Unavailable::NotStored | Unavailable::NotAToken => Fix::command(
            "coda-sandbox",
            [
                "broker".to_owned(),
                "store".to_owned(),
                item.name().to_owned(),
            ],
        ),
        Unavailable::KeyringLocked => Fix::instruction(
            "log in on the host's desktop so the wallet unlocks, then rerun doctor",
        ),
        Unavailable::FallbackFailed => {
            Fix::command("journalctl", ["--user", "-u", UNIT, "-n", "50"])
        }
    }
}

/// `github-agent: keyring; doppler: not stored`. An optional key that was
/// never stored is left out.
pub fn describe(report: &Report, proxy: Option<&ModelProxy>) -> String {
    report
        .items
        .iter()
        .filter(|entry| !absent_by_choice(entry, proxy))
        .map(|ItemReport { item, state }| {
            let state = match state {
                ItemState::Available { source } => source.name().to_owned(),
                ItemState::Stale {
                    source,
                    read_secs_ago,
                } => format!(
                    "{} cached {} min ago, keyring locked",
                    source.name(),
                    read_secs_ago / 60
                ),
                ItemState::Unavailable { reason } => match reason {
                    Unavailable::NotStored => "not stored".to_owned(),
                    Unavailable::KeyringLocked => "keyring locked".to_owned(),
                    Unavailable::FallbackFailed => "gh fallback failed".to_owned(),
                    Unavailable::NotAToken => "stored value is not a token".to_owned(),
                },
            };
            format!("{item}: {state}")
        })
        .collect::<Vec<_>>()
        .join("; ")
}
