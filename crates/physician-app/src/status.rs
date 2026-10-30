//! The physician app's read and command states, each said on the shared status line.

use dioxus::prelude::*;
use ui::StatusLine;

/// Where one read stands, as the page shows it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ReadStage {
    /// The first read has not answered yet.
    Loading,
    /// The read answered and its content shows.
    Showing,
    /// A refresh failed: the last answer stays, under one quiet line with Retry.
    Stale,
    /// The first read failed: nothing to show, with Retry.
    Unavailable,
}

/// A read's own line beside its content: loading, one quiet line with Retry
/// over the last answer, or couldn't load with Retry. Showing adds nothing.
#[component]
pub fn ReadNotice(
    stage: ReadStage,
    loading: String,
    stale: String,
    unavailable: String,
    retry: String,
    on_retry: EventHandler,
) -> Element {
    match stage {
        ReadStage::Loading => rsx! { StatusLine { message: loading, pending: true } },
        ReadStage::Showing => rsx! {},
        ReadStage::Stale => rsx! { StatusLine { message: stale, retry, on_retry } },
        ReadStage::Unavailable => {
            rsx! { StatusLine { message: unavailable, alert: true, retry, on_retry } }
        }
    }
}

/// Where one command stands after it was pressed.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum CommandStage {
    /// Nothing is in flight.
    Idle,
    /// Sent and not yet answered.
    Sending,
    /// The answer never arrived, so whether it took is unknown; the host's check again action
    /// follows.
    Unconfirmed,
    /// It was answered and not taken, saying why.
    Refused(String),
}

/// A command's own line: sending, couldn't confirm with its check again action, or why
/// it was refused. Idle adds nothing.
#[component]
pub fn CommandNotice(
    stage: CommandStage,
    sending: String,
    unconfirmed: String,
    check_again: String,
    on_check_again: EventHandler,
) -> Element {
    match stage {
        CommandStage::Idle => rsx! {},
        CommandStage::Sending => rsx! { StatusLine { message: sending } },
        CommandStage::Unconfirmed => rsx! {
            StatusLine { message: unconfirmed, retry: check_again, on_retry: on_check_again }
        },
        CommandStage::Refused(reason) => rsx! { StatusLine { message: reason, alert: true } },
    }
}
