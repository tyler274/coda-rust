//! Sign-in news between this browser's Coda tabs. The session belongs to the
//! browser, but a tab that is already open learns of a change here: when one
//! tab redeems a link or signs out, a tab covered by the sign-in prompt picks
//! up the new session where it was, and a different physician's sign-in or a
//! sign-out reaches every tab (verification/physician/sign-in).
// The channel is browser-only; native builds keep the news type for its tests.
#![cfg_attr(not(target_arch = "wasm32"), allow(dead_code))]

use serde_json::{Value, json};

/// One piece of sign-in news.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum TabNews {
    /// A physician signed in on this browser with this newly minted session.
    SignedIn {
        /// Who signed in.
        physician_id: String,
        /// The bearer session their link minted.
        session: String,
    },
    /// A tab signed out.
    SignedOut,
}

impl TabNews {
    fn to_json(&self, from: &str) -> String {
        match self {
            Self::SignedIn {
                physician_id,
                session,
            } => json!({
                "from": from,
                "kind": "signed_in",
                "physician_id": physician_id,
                "session": session,
            }),
            Self::SignedOut => json!({ "from": from, "kind": "signed_out" }),
        }
        .to_string()
    }

    /// Reads news another tab sent; this tab's own news and anything
    /// malformed read as nothing.
    fn from_json(text: &str, me: &str) -> Option<Self> {
        let value: Value = serde_json::from_str(text).ok()?;
        if value.get("from").and_then(Value::as_str) == Some(me) {
            return None;
        }
        let field = |name: &str| {
            value
                .get(name)
                .and_then(Value::as_str)
                .filter(|text| !text.is_empty())
                .map(str::to_owned)
        };
        match value.get("kind").and_then(Value::as_str)? {
            "signed_in" => Some(Self::SignedIn {
                physician_id: field("physician_id")?,
                session: field("session")?,
            }),
            "signed_out" => Some(Self::SignedOut),
            _ => None,
        }
    }
}

#[cfg(target_arch = "wasm32")]
const CHANNEL: &str = "coda.physician.session";

/// This tab's name on the channel, so it ignores its own news.
#[cfg(target_arch = "wasm32")]
fn tab_id() -> &'static str {
    static TAB: std::sync::OnceLock<String> = std::sync::OnceLock::new();
    TAB.get_or_init(|| format!("tab-{}", js_sys::Math::random()))
}

/// Tells this browser's other Coda tabs.
#[cfg(target_arch = "wasm32")]
pub(crate) fn announce(news: &TabNews) {
    if let Ok(channel) = web_sys::BroadcastChannel::new(CHANNEL) {
        let _ = channel.post_message(&news.to_json(tab_id()).into());
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
pub(crate) fn listen(mut on_news: impl FnMut(TabNews) + 'static) -> Option<Listening> {
    use wasm_bindgen::JsCast;
    let channel = web_sys::BroadcastChannel::new(CHANNEL).ok()?;
    let handler = wasm_bindgen::closure::Closure::<dyn FnMut(web_sys::MessageEvent)>::new(
        move |event: web_sys::MessageEvent| {
            if let Some(news) = event
                .data()
                .as_string()
                .and_then(|text| TabNews::from_json(&text, tab_id()))
            {
                on_news(news);
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
    use super::TabNews;

    #[test]
    fn news_round_trips_between_tabs_and_a_tab_ignores_its_own() {
        for news in [
            TabNews::SignedIn {
                physician_id: "doc".into(),
                session: "codas-1".into(),
            },
            TabNews::SignedOut,
        ] {
            let sent = news.to_json("tab-a");
            assert_eq!(TabNews::from_json(&sent, "tab-b"), Some(news));
            assert_eq!(TabNews::from_json(&sent, "tab-a"), None);
        }
    }

    #[test]
    fn malformed_or_incomplete_news_reads_as_nothing() {
        for text in [
            "not json",
            r#"{"kind":"signed_in","physician_id":"doc"}"#,
            r#"{"kind":"signed_in","physician_id":"","session":"codas-1"}"#,
            r#"{"kind":"something_else"}"#,
        ] {
            assert_eq!(TabNews::from_json(text, "tab-b"), None, "{text}");
        }
    }
}
