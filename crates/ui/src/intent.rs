//! Which element a moving mouse is heading for, before it arrives. An element takes part by
//! carrying a `data-intent` key; the host decides what heading for it starts. The path is read the
//! way ForesightJS reads it: the pointer's recent motion, carried a short time forward, crossing the
//! element's box. Touch and pen report nothing, since they arrive with no path to read.

use dioxus::prelude::*;

use crate::script;

/// How far ahead, in milliseconds, the pointer's path is carried.
const AHEAD_MS: u32 = 120;

/// While `watching`, calls `heading` with the `data-intent` key of each element a moving mouse heads
/// for, once per approach. Stops on its own once the component goes.
pub fn use_pointer_intent(watching: bool, heading: EventHandler<String>) {
    use_hook(move || {
        if !watching {
            return;
        }
        let keys = script::watch_intent(AHEAD_MS);
        spawn(async move {
            // A watch that fails has said why in the console; nothing is predicted then.
            while let Ok(key) = keys.next().await {
                heading.call(key);
            }
        });
    });
}
