//! Sign-in news between this browser's Coda tabs. The session belongs to the
//! browser: the one it stores decides. When one tab stores a new session or
//! forgets it, it tells the others that the session changed, and each catches
//! up by reading what the browser stores (verification/physician/sign-in).
// The channel is browser-only; native builds keep the news type for its tests.
#![cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]

use serde_json::{Value, json};

/// The news one tab sends the others: this browser's session changed.
fn news(from: &str) -> String {
    json!({ "from": from, "kind": "changed" }).to_string()
}

/// Whether `text` is another tab's news. It names no session, so whatever it
/// says, the tab that hears it reads what the browser stores.
fn from_another_tab(text: &str, me: &str) -> bool {
    serde_json::from_str::<Value>(text)
        .ok()
        .and_then(|value| value.get("from").and_then(Value::as_str).map(str::to_owned))
        .is_some_and(|from| from != me)
}

#[cfg(target_arch = "wasm32")]
const CHANNEL: &str = "coda.physician.session";

/// This tab's name on the channel, so it ignores its own news.
#[cfg(target_arch = "wasm32")]
fn tab_id() -> &'static str {
    static TAB: std::sync::OnceLock<String> = std::sync::OnceLock::new();
    TAB.get_or_init(|| format!("tab-{}", js_sys::Math::random()))
}

/// Tells this browser's other Coda tabs that its session changed.
#[cfg(target_arch = "wasm32")]
pub(crate) fn announce() {
    if let Ok(channel) = web_sys::BroadcastChannel::new(CHANNEL) {
        let _ = channel.post_message(&news(tab_id()).into());
        channel.close();
    }
}

/// Listens until dropped.
#[cfg(target_arch = "wasm32")]
pub(crate) struct Listening {
    channel: web_sys::BroadcastChannel,
    _handler: wasm_bindgen::closure::Closure<dyn FnMut(web_sys::MessageEvent)>,
}

#[cfg(target_arch = "wasm32")]
impl Drop for Listening {
    fn drop(&mut self) {
        self.channel.set_onmessage(None);
        self.channel.close();
    }
}

/// Hears the other tabs' news.
#[cfg(target_arch = "wasm32")]
pub(crate) fn listen(mut on_news: impl FnMut() + 'static) -> Option<Listening> {
    use wasm_bindgen::JsCast;
    let channel = web_sys::BroadcastChannel::new(CHANNEL).ok()?;
    let handler = wasm_bindgen::closure::Closure::<dyn FnMut(web_sys::MessageEvent)>::new(
        move |event: web_sys::MessageEvent| {
            if event
                .data()
                .as_string()
                .is_some_and(|text| from_another_tab(&text, tab_id()))
            {
                on_news();
            }
        },
    );
    channel.set_onmessage(Some(handler.as_ref().unchecked_ref()));
    Some(Listening {
        channel,
        _handler: handler,
    })
}

#[cfg(test)]
mod tests {
    use super::{from_another_tab, news};

    #[test]
    fn a_tab_hears_another_tabs_news_and_ignores_its_own() {
        let sent = news("tab-a");
        assert!(from_another_tab(&sent, "tab-b"));
        assert!(!from_another_tab(&sent, "tab-a"));
    }

    #[test]
    fn malformed_news_reads_as_nothing() {
        for text in ["not json", r#"{"kind":"changed"}"#, r#"{"from":1}"#] {
            assert!(!from_another_tab(text, "tab-b"), "{text}");
        }
    }
}
