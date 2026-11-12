//! Launches the popup in the extension's `options.html`. Built only for the web; see the crate
//! docs for the build.

fn main() {
    #[cfg(all(feature = "web", target_arch = "wasm32"))]
    dioxus::launch(popup::Popup);
}

#[cfg(all(feature = "web", target_arch = "wasm32"))]
mod popup {
    use std::cell::Cell;
    use std::rc::Rc;
    use std::time::Duration;

    use dioxus::prelude::*;
    use extension_popup::face::{Action, Activity, Millis, Refusal, Reply, Request, face, refusal};
    use serde::Deserialize;
    use serde_json::{Value, json};
    use ui::{
        Button, ButtonVariant, Icon, IconName, Menu, MenuItem, MenuSection, Notice, PageTitle,
        StatusLine, Stylesheet, TaskList, Tone, TopBar,
    };
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen(module = "/assets/chrome.js")]
    extern "C" {
        #[wasm_bindgen(catch, js_name = send)]
        fn send_js(action: &str) -> Result<js_sys::Promise, JsValue>;
        fn open_hint() -> js_sys::Promise;
        fn clock(ms: f64) -> String;
    }

    /// The worker's reply to `request`, both read and as it sent it, or why the popup has none.
    async fn send(request: Request) -> Result<(Reply, Value), NoReply> {
        let sent = send_js(request.name()).map_err(NoReply::Unreachable)?;
        let answer = wasm_bindgen_futures::JsFuture::from(sent)
            .await
            .map_err(NoReply::Unreachable)?;
        // chrome.js answers with the reply's JSON text.
        let text = answer.as_string().ok_or_else(|| {
            NoReply::Unknown(serde::de::Error::custom("the worker's reply wasn't text"))
        })?;
        let words: Value = serde_json::from_str(&text).map_err(NoReply::Unknown)?;
        let reply = Reply::deserialize(&words).map_err(NoReply::Unknown)?;
        Ok((reply, words))
    }

    /// Why the popup has no reply from the worker.
    #[derive(Debug, thiserror::Error)]
    enum NoReply {
        /// Chrome couldn't deliver the message, or the worker didn't answer: Chrome's own error.
        #[error("Chrome couldn't reach the worker: {}", chrome_words(.0))]
        Unreachable(JsValue),
        /// The worker answered, but not with a reply this popup reads.
        #[error("the worker's reply isn't one the popup reads: {0}")]
        Unknown(#[source] serde_json::Error),
    }

    /// Chrome's error in its own words: its message, its text, or what it is.
    fn chrome_words(error: &JsValue) -> String {
        error
            .dyn_ref::<js_sys::Error>()
            .map(|error| String::from(error.message()))
            .or_else(|| error.as_string())
            .unwrap_or_else(|| format!("{error:?}"))
    }

    /// A notice the popup shows above the state: its tone, icon, title and detail.
    type Said = (Tone, IconName, &'static str, &'static str);

    const UNREACHABLE: Said = (
        Tone::Error,
        IconName::Alert,
        "The extension isn’t responding",
        "Reload it from chrome://extensions.",
    );
    const UNKNOWN_REPLY: Said = (
        Tone::Error,
        IconName::Alert,
        "The extension sent a reply Coda doesn’t understand",
        "Reload it from chrome://extensions.",
    );
    const COPIED: Said = (
        Tone::Primary,
        IconName::Check,
        "Diagnostics copied",
        "Paste them wherever you’re asking for help.",
    );
    const NOT_COPIED: Said = (
        Tone::Warning,
        IconName::Alert,
        "Couldn’t copy diagnostics",
        "Try again.",
    );
    const NOT_OPENED: Said = (
        Tone::Warning,
        IconName::Alert,
        "Couldn’t open Hint",
        "Open app.hint.com yourself.",
    );

    /// Coda's ring of twelve dots, the mark on its icon, beside the name in the header. A wave runs
    /// round the ring while the popup waits on the worker, and slowly while Coda reads.
    #[component]
    fn BrandMark(working: bool, live: bool) -> Element {
        let dots = (0..12).map(|step| {
            let angle = f64::from(step) * std::f64::consts::TAU / 12.0;
            (step, 12.0 + 10.0 * angle.sin(), 12.0 - 10.0 * angle.cos())
        });
        let class = if working {
            "brand-mark is-working"
        } else if live {
            "brand-mark is-live"
        } else {
            "brand-mark"
        };
        rsx! {
            svg { class, width: "22", height: "22", view_box: "0 0 24 24", "aria-hidden": "true", fill: "#1a6b4a",
                for (step, x, y) in dots {
                    circle { key: "{step}", cx: "{x}", cy: "{y}", r: "1.7", style: "--dot: {step}" }
                }
            }
        }
    }

    #[component]
    pub fn Popup() -> Element {
        let mut reply = use_signal(|| None::<(Reply, Value)>);
        let mut busy = use_signal(|| false);
        // The last action's refusal, and the worker's reply as it sent it, until the next action.
        let mut refused = use_signal(|| None::<(Refusal, Value)>);
        // Why the popup last had no reply from the worker, until it has one again.
        let mut no_reply = use_signal(|| None::<Rc<NoReply>>);
        // What the last menu or button action reported, for a moment, numbered by its telling;
        // the standing state shows otherwise.
        let mut told = use_signal(|| None::<(u64, Said)>);
        let mut tellings = use_signal(|| 0u64);
        // The state's last title, and how many times it has changed since.
        let arrivals = use_hook(|| Rc::new(Cell::new((String::new(), 0u64))));
        use_future(move || async move {
            loop {
                if !*busy.peek() {
                    let status = send(Request::Status).await;
                    match status {
                        Ok(status) => {
                            reply.set(Some(status));
                            no_reply.set(None);
                        }
                        Err(why) => no_reply.set(Some(Rc::new(why))),
                    }
                }
                futures_timer::Delay::new(Duration::from_secs(1)).await;
            }
        });
        let act = move |request: Request| async move {
            busy.set(true);
            refused.set(None);
            told.set(None);
            let sent = send(request).await;
            match sent {
                Ok((answer, words)) => {
                    refused.set(answer.refused().map(|reason| (reason, words.clone())));
                    reply.set(Some((answer, words)));
                }
                Err(why) => no_reply.set(Some(Rc::new(why))),
            }
            busy.set(false);
        };
        // A newer telling, even of the same words, outlasts this one's timer.
        let tell = move |said: Said, seconds: u64| async move {
            let telling = *tellings.peek() + 1;
            tellings.set(telling);
            told.set(Some((telling, said)));
            futures_timer::Delay::new(Duration::from_secs(seconds)).await;
            if told.peek().is_some_and(|(shown, _)| shown == telling) {
                told.set(None);
            }
        };
        // The worker's last reply as it sent it; when the popup has no reply or the last action was
        // refused, why, beside the last reply.
        let diagnostics = move || async move {
            let last = reply.peek().as_ref().map(|(_, words)| words.clone());
            let why = no_reply.peek().as_deref().map(ToString::to_string);
            let refusal = refused.peek().as_ref().map(|(_, words)| words.clone());
            let words = if why.is_none() && refusal.is_none() {
                last
            } else {
                Some(json!({ "no_reply": why, "refused": refusal, "last_reply": last }))
            };
            let Some(words) = words else { return };
            told.set(None);
            let copied = ui::copy_text(words.to_string()).await;
            match copied {
                Ok(()) => tell(COPIED, 2).await,
                Err(_) => tell(NOT_COPIED, 5).await,
            }
        };
        let go_to_hint = move || async move {
            told.set(None);
            let opened = wasm_bindgen_futures::JsFuture::from(open_hint()).await;
            if opened.is_err() {
                tell(NOT_OPENED, 5).await;
            }
        };

        let current = reply()
            .map(|(status, _)| face(&status, Millis(js_sys::Date::now()), &|at| clock(at.0)));
        let activity = current
            .as_ref()
            .map(|face| face.activity)
            .unwrap_or_default();
        let live = activity == Activity::Reading;
        let running = activity != Activity::Idle;
        let note = told()
            .map(|(_, said)| said)
            .or_else(|| match no_reply().as_deref() {
                Some(NoReply::Unreachable(_)) => Some(UNREACHABLE),
                Some(NoReply::Unknown(_)) => Some(UNKNOWN_REPLY),
                None => refused().map(|(reason, _)| {
                    let note = refusal(reason);
                    (note.tone, IconName::Alert, note.title, note.detail)
                }),
            });

        let body = match current {
            None if no_reply().is_some() => rsx! {},
            None => rsx! { StatusLine { message: "Checking Hint…", pending: true } },
            Some(face) => {
                // Each new state takes the other of two identical arrivals, so the next one runs again.
                let (last, changes) = arrivals.take();
                let changes = if last == face.title {
                    changes
                } else {
                    changes + 1
                };
                arrivals.set((face.title.clone(), changes));
                let arrival = if changes % 2 == 1 {
                    "popup-state popup-arrive"
                } else {
                    "popup-state popup-arrive-again"
                };
                rsx! {
                    div { class: "{arrival}",
                        PageTitle { eyebrow: "Hint", text: face.title, detail: face.detail, icon: face.icon }
                        if !face.steps.is_empty() {
                            TaskList { tasks: face.steps }
                        }
                    }
                    if let Some(offer) = face.offer {
                        div { class: "popup-footer",
                            Button { variant: ButtonVariant::Primary,
                                busy: busy(),
                                pending_label: "Connecting…",
                                onclick: move |_| async move {
                                    match offer.action {
                                        Action::OpenHint => go_to_hint().await,
                                        Action::Connect => act(Request::Connect).await,
                                    }
                                },
                                Icon { name: match offer.action { Action::OpenHint => IconName::ExternalLink, Action::Connect => IconName::Link } }
                                "{offer.label}"
                            }
                        }
                    }
                }
            }
        };

        rsx! {
            Stylesheet { app: include_str!("popup.css") }
            main { class: "popup",
                TopBar { title: "Coda",
                    mark: rsx! { BrandMark { working: (reply().is_none() && no_reply().is_none()) || busy(), live } },
                    trailing: rsx! {
                        Button { variant: ButtonVariant::Quiet, icon_only: true,
                            opens: "popup-menu", aria_label: "More", aria_haspopup: "menu",
                            Icon { name: IconName::More }
                        }
                        Menu { id: "popup-menu", label: "Coda",
                            MenuSection {
                                MenuItem { icon: IconName::ExternalLink,
                                    onclick: move |_| async move { go_to_hint().await },
                                    "Go to Hint"
                                }
                                MenuItem { icon: IconName::Clipboard,
                                    disabled: reply().is_none() && no_reply().is_none(),
                                    onclick: move |_| async move { diagnostics().await },
                                    "Copy diagnostics"
                                }
                            }
                            if running {
                                MenuSection {
                                    MenuItem { icon: IconName::X, danger: true,
                                        onclick: move |_| async move { act(Request::Stop).await },
                                        "Disconnect"
                                    }
                                }
                            }
                        }
                    },
                }
                div { class: "popup-body",
                    if let Some((tone, icon, title, detail)) = note {
                        div { class: "popup-arrive",
                            Notice { tone, icon, title, detail }
                        }
                    }
                    {body}
                }
            }
        }
    }
}
