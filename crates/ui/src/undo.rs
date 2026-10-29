//! The one undo notice, after the React app's `UndoNotice`: what just happened and Undo, floating
//! at one layer over the page so it never moves what is being read. What it undoes has already
//! landed; Undo is the server's own reverse, so leaving it loses only the chance to undo.

use std::time::Duration;

use dioxus::prelude::*;

use crate::{Button, ButtonVariant, EXIT_MS};

/// How long Undo stays, counted while the person is not pointing at or focused on it.
pub const UNDO_MS: u64 = 6_000;

/// What just happened, and Undo. Its own time pauses while it is pointed at or focused; a bound the
/// server sets (`expires_in_ms`) ends it exactly then, never held longer, so every Undo it shows is
/// one the server can honour. It leaves the same way whether its time ran out or Undo was pressed,
/// then reports `on_gone`. The host keys it by what it undoes, so a newer action replaces it.
#[component]
pub fn UndoNotice(
    words: String,
    on_undo: EventHandler<MouseEvent>,
    on_gone: EventHandler,
    #[props(default)] expires_in_ms: Option<u64>,
) -> Element {
    let mut pointed = use_signal(|| false);
    let mut focused = use_signal(|| false);
    let mut leaving = use_signal(|| false);
    let mut clock = use_signal(|| 0u64);
    let mut leave = move || {
        if leaving() {
            return;
        }
        leaving.set(true);
        spawn(async move {
            futures_timer::Delay::new(Duration::from_millis(EXIT_MS)).await;
            on_gone.call(());
        });
    };
    // Each change of pause or exit starts the count again from its full time and retires the last.
    // A bounded notice never reads either, so pointing at it can't restart its time. Pointing and
    // focus are kept apart: leaving one while the other remains still holds it.
    use_effect(move || {
        let paused = expires_in_ms.is_none() && (pointed() || focused());
        let started = *clock.peek() + 1;
        clock.set(started);
        if paused || leaving() {
            return;
        }
        spawn(async move {
            futures_timer::Delay::new(Duration::from_millis(expires_in_ms.unwrap_or(UNDO_MS)))
                .await;
            if *clock.peek() == started {
                leave();
            }
        });
    });
    rsx! {
        div {
            class: if leaving() { "undo-notice is-leaving" } else { "undo-notice" },
            role: "status",
            aria_live: "polite",
            "inert": leaving().then_some("true"),
            onpointerenter: move |_| pointed.set(true),
            onpointerleave: move |_| pointed.set(false),
            onfocusin: move |_| focused.set(true),
            onfocusout: move |_| focused.set(false),
            span { "{words}" }
            Button {
                variant: ButtonVariant::Secondary,
                compact: true,
                disabled: leaving(),
                onclick: move |event| {
                    on_undo.call(event);
                    leave();
                },
                "Undo"
            }
        }
    }
}
