//! Feedback on one completed response, after the React app's `FeedbackControl`: a good or poor
//! rating, then optional reasons the person can toggle. Feedback never starts clinical work; the
//! host owns saving, its outcome and any recovery.

use dioxus::prelude::*;

use crate::{Button, ButtonVariant, Icon, IconName};

/// A rating of one response.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rating {
    /// A good response.
    Up,
    /// A poor response.
    Down,
}

/// The two ratings as pressed icon controls. The saved or pending rating is pressed; while a
/// rating saves, only it stays enabled. On pointer devices the pair shows while its message is
/// hovered or focused, or while a rating saves; on touch it always shows.
#[component]
pub fn FeedbackRating(
    selected: Option<Rating>,
    #[props(default)] busy: bool,
    #[props(default)] disabled: bool,
    on_rate: EventHandler<Rating>,
) -> Element {
    rsx! {
        div {
            class: "feedback-rating",
            role: "group",
            aria_label: "Rate this response",
            aria_busy: if busy { "true" },
            "data-active": if busy { "true" },
            for (rating, label, icon) in [
                (Rating::Up, "Good response", IconName::ThumbUp),
                (Rating::Down, "Poor response", IconName::ThumbDown),
            ] {
                Button {
                    key: "{label}",
                    variant: ButtonVariant::Quiet,
                    icon_only: true,
                    aria_label: label,
                    aria_pressed: if selected == Some(rating) { "true" } else { "false" },
                    disabled: disabled || busy && selected != Some(rating),
                    onclick: move |_| on_rate.call(rating),
                    Icon { name: icon }
                }
            }
        }
    }
}

/// One choice that is on or off, as a pill: a reason given with feedback, a filter. Pressed, it
/// carries a check; otherwise a plus.
#[component]
pub fn ToggleChip(
    pressed: bool,
    #[props(default)] disabled: bool,
    onclick: EventHandler<MouseEvent>,
    children: Element,
) -> Element {
    rsx! {
        Button {
            class: "toggle-chip",
            aria_pressed: if pressed { "true" } else { "false" },
            disabled,
            onclick: move |event| onclick.call(event),
            Icon { name: if pressed { IconName::Check } else { IconName::Plus } }
            {children}
        }
    }
}
