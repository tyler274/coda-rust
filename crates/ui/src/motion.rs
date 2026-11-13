//! The crate's look and the motion its stylesheet cannot express alone: script-driven motion runs
//! on the stylesheet's own roles, and none of it runs under reduced motion, where the same end
//! state simply arrives. A list's selection is one fill that slides to the item chosen and otherwise stays
//! anchored to it.

use dioxus::prelude::*;

use crate::STYLESHEET;
use crate::script;

/// The crate's stylesheet with an app's own styles after it, and the crate's motion: each app's
/// root draws it once.
#[component]
pub fn Stylesheet(#[props(default)] app: &'static str) -> Element {
    use_hook(|| {
        // Before anything else runs a behaviour: a native webview needs the module loaded first.
        script::load();
        script::motion();
    });
    rsx! { style { "{STYLESHEET}{app}" } }
}
