//! Shared physician-facing warning when a cross-patient list had to skip an
//! unchecked patient workspace due to an authority check failure (`A1.3`).

use dioxus::prelude::*;
use ui::{IconName, Notice, Tone};

/// User-facing warning displayed on any cross-patient list whose results may be
/// incomplete because one or more patient access checks failed.
pub(crate) const MAY_BE_MISSING_SOMEONE_COPY: &str = "This list may be missing someone.";

/// Renders a warning notice whenever `visible` is true on a cross-patient list.
#[component]
pub(crate) fn MayBeMissingSomeoneNotice(visible: bool) -> Element {
    if !visible {
        return rsx! {};
    }
    rsx! {
        div {
            class: "access-partial-notice",
            "data-may-be-missing-someone": "true",
            Notice {
                tone: Tone::Warning,
                icon: Some(IconName::Alert),
                title: MAY_BE_MISSING_SOMEONE_COPY.to_owned(),
            }
        }
    }
}
