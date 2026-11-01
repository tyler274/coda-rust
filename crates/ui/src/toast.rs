//! The toast: what just happened, floating at one layer over the page so it never moves what is
//! being read.

use std::time::Duration;

use dioxus::prelude::*;

use crate::{Button, ButtonVariant, EXIT_MS, Icon, IconName};

/// How long a toast stays, counted while the person is not pointing at or focused on it.
pub const TOAST_MS: u64 = 6_000;

/// What just happened, at the foot of the window: its icon, its words, and, named by `action`, what
/// can follow from it, such as Undo, heard as `on_action`. Its time pauses while it is pointed at or
/// focused; with `ends_in_ms` it ends exactly then instead, and nothing holds it longer. It leaves
/// the same way whether its time ran out or the action was pressed, then reports `on_gone`. One
/// stands at a time: the host keys it by what it reports, so a newer one replaces it.
#[component]
pub fn Toast(
    words: String,
    icon: Option<IconName>,
    action: Option<String>,
    on_action: Option<EventHandler<MouseEvent>>,
    on_gone: EventHandler,
    #[props(default)] ends_in_ms: Option<u64>,
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
    // A notice with an end never reads either, so pointing at it can't restart its time. Pointing and
    // focus are kept apart: leaving one while the other remains still holds it.
    use_effect(move || {
        let paused = ends_in_ms.is_none() && (pointed() || focused());
        let started = *clock.peek() + 1;
        clock.set(started);
        if paused || leaving() {
            return;
        }
        spawn(async move {
            futures_timer::Delay::new(Duration::from_millis(ends_in_ms.unwrap_or(TOAST_MS))).await;
            if *clock.peek() == started {
                leave();
            }
        });
    });
    rsx! {
        div {
            class: if leaving() { "toast is-leaving" } else { "toast" },
            role: "status",
            aria_live: "polite",
            "inert": leaving().then_some("true"),
            onpointerenter: move |_| pointed.set(true),
            onpointerleave: move |_| pointed.set(false),
            onfocusin: move |_| focused.set(true),
            onfocusout: move |_| focused.set(false),
            if let Some(name) = icon {
                Icon { name }
            }
            span { "{words}" }
            if let (Some(label), Some(on_action)) = (action, on_action) {
                Button {
                    variant: ButtonVariant::Secondary,
                    compact: true,
                    disabled: leaving(),
                    onclick: move |event| {
                        on_action.call(event);
                        leave();
                    },
                    "{label}"
                }
            }
        }
    }
}
