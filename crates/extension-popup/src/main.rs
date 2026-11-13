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
    use extension_popup::face::{
        Action, Activity, Refusal, Reply, Request, Standing, endpoint_origin_pattern, face, refusal,
    };
    use serde::Deserialize;
    use serde_json::{Value, json};
    use ui::{
        Button, ButtonVariant, Icon, IconName, Menu, MenuItem, MenuSection, Notice, PageTitle,
        StatusLine, Stylesheet, TaskList, TextField, Tone, TopBar,
    };
    use wasm_bindgen::prelude::*;

    #[wasm_bindgen(module = "/assets/chrome.js")]
    extern "C" {
        #[wasm_bindgen(catch, js_name = send_message)]
        fn send_js(payload_json: &str) -> Result<js_sys::Promise, JsValue>;
        #[wasm_bindgen(catch, js_name = request_origins)]
        fn request_origins_js(origins_json: &str) -> Result<js_sys::Promise, JsValue>;
        fn open_source(entry_url: &str, tab_pattern: Option<&str>) -> js_sys::Promise;
    }

    async fn send(request: Request) -> Result<(Reply, Value), NoReply> {
        let wire = request.to_wire().to_string();
        let sent = send_js(&wire).map_err(NoReply::Unreachable)?;
        let answer = wasm_bindgen_futures::JsFuture::from(sent)
            .await
            .map_err(NoReply::Unreachable)?;
        let text = answer.as_string().ok_or_else(|| {
            NoReply::Unknown(serde::de::Error::custom("the worker's reply wasn't text"))
        })?;
        let words: Value = serde_json::from_str(&text).map_err(NoReply::Unknown)?;
        let reply = Reply::deserialize(&words).map_err(NoReply::Unknown)?;
        Ok((reply, words))
    }

    async fn request_origins(origins: &[String]) -> bool {
        let Ok(encoded) = serde_json::to_string(origins) else {
            return false;
        };
        let Ok(promise) = request_origins_js(&encoded) else {
            return false;
        };
        let Ok(value) = wasm_bindgen_futures::JsFuture::from(promise).await else {
            return false;
        };
        value.as_bool().unwrap_or(false)
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

    fn chrome_words(error: &JsValue) -> String {
        error
            .dyn_ref::<js_sys::Error>()
            .map(|error| String::from(error.message()))
            .or_else(|| error.as_string())
            .unwrap_or_else(|| format!("{error:?}"))
    }

    /// A notice the popup shows above the state: its tone, icon, title and detail.
    type Said = (Tone, IconName, &'static str, String);

    fn unreachable_note() -> Said {
        (
            Tone::Error,
            IconName::Alert,
            "The extension isn’t responding",
            "Reload it from chrome://extensions.".to_owned(),
        )
    }

    fn unknown_reply_note() -> Said {
        (
            Tone::Error,
            IconName::Alert,
            "The extension sent a reply Coda doesn’t understand",
            "Reload it from chrome://extensions.".to_owned(),
        )
    }

    fn copied_note() -> Said {
        (
            Tone::Primary,
            IconName::Check,
            "Diagnostics copied",
            "Paste them wherever you’re asking for help.".to_owned(),
        )
    }

    fn not_copied_note() -> Said {
        (
            Tone::Warning,
            IconName::Alert,
            "Couldn’t copy diagnostics",
            "Try again.".to_owned(),
        )
    }

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
        let mut refused = use_signal(|| None::<(Refusal, Value)>);
        let mut no_reply = use_signal(|| None::<Rc<NoReply>>);
        let mut told = use_signal(|| None::<(u64, Said)>);
        let mut tellings = use_signal(|| 0u64);
        let mut chords_endpoint = use_signal(String::new);
        let mut pairing_code = use_signal(String::new);
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

        let tell = move |said: Said, seconds: u64| async move {
            let telling = *tellings.peek() + 1;
            tellings.set(telling);
            told.set(Some((telling, said)));
            futures_timer::Delay::new(Duration::from_secs(seconds)).await;
            if told
                .peek()
                .as_ref()
                .is_some_and(|(shown, _)| *shown == telling)
            {
                told.set(None);
            }
        };

        let diagnostics = move || async move {
            let last = reply.peek().as_ref().map(|(_, words)| words.clone());
            let why = no_reply.peek().as_deref().map(ToString::to_string);
            let refusal_words = refused.peek().as_ref().map(|(_, words)| words.clone());
            let words = if why.is_none() && refusal_words.is_none() {
                last
            } else {
                Some(json!({ "no_reply": why, "refused": refusal_words, "last_reply": last }))
            };
            let Some(words) = words else { return };
            told.set(None);
            let copied = ui::copy_text(words.to_string()).await;
            match copied {
                Ok(()) => tell(copied_note(), 2).await,
                Err(_) => tell(not_copied_note(), 5).await,
            }
        };

        let go_to_source = move || async move {
            let Some((status, _)) = reply.peek().clone() else {
                return;
            };
            told.set(None);
            let opened = wasm_bindgen_futures::JsFuture::from(open_source(
                status.entry_url(),
                status.tab_pattern(),
            ))
            .await;
            if opened.is_err() {
                tell(
                    (
                        Tone::Warning,
                        IconName::Alert,
                        "Couldn’t open tab",
                        format!("Open {} yourself.", status.entry_url()),
                    ),
                    5,
                )
                .await;
            }
        };

        let pair_action = move || async move {
            let entered = chords_endpoint.peek().trim().to_owned();
            let Some(pattern) = endpoint_origin_pattern(&entered) else {
                let note = refusal(Refusal::InvalidEndpoint, "Coda");
                tell((note.tone, IconName::Alert, note.title, note.detail), 5).await;
                return;
            };
            let allowed = request_origins(&[pattern]).await;
            if !allowed {
                let note = refusal(Refusal::PermissionRequired, "Coda");
                tell((note.tone, IconName::Alert, note.title, note.detail), 5).await;
                return;
            }
            let code_val = pairing_code.peek().trim().to_owned();
            act(Request::Pair {
                endpoint: entered,
                code: (!code_val.is_empty()).then_some(code_val),
            })
            .await;
        };

        let connect_action = move || async move {
            if let Some((status, _)) = reply.peek().clone() {
                let allowed = request_origins(status.hosts()).await;
                if !allowed {
                    let note = refusal(Refusal::PermissionRequired, status.source_label());
                    tell((note.tone, IconName::Alert, note.title, note.detail), 5).await;
                    return;
                }
            }
            act(Request::Connect).await;
        };

        let status_snapshot = reply().map(|(status, _)| status);
        let current = status_snapshot.as_ref().map(face);
        let source_name = status_snapshot
            .as_ref()
            .map(|s| s.source_label().to_owned())
            .unwrap_or_else(|| "EHR".to_owned());
        let paired = status_snapshot.as_ref().is_some_and(Reply::is_paired);
        let activity = current
            .as_ref()
            .map(|shown| shown.activity)
            .unwrap_or_default();
        let live = activity == Activity::Reading;
        let running = activity != Activity::Idle;

        let note = told()
            .map(|(_, said)| said)
            .or_else(|| match no_reply().as_deref() {
                Some(NoReply::Unreachable(_)) => Some(unreachable_note()),
                Some(NoReply::Unknown(_)) => Some(unknown_reply_note()),
                None => refused().map(|(reason, _)| {
                    let built = refusal(reason, &source_name);
                    (built.tone, IconName::Alert, built.title, built.detail)
                }),
            });

        let body = match current {
            None if no_reply().is_some() => rsx! {},
            None => rsx! { StatusLine { message: "Checking connection…", pending: true } },
            Some(shown) => {
                let (last, changes) = arrivals.take();
                let changes = if last == shown.title {
                    changes
                } else {
                    changes + 1
                };
                arrivals.set((shown.title.clone(), changes));
                let arrival = if changes % 2 == 1 {
                    "popup-state popup-arrive"
                } else {
                    "popup-state popup-arrive-again"
                };
                let is_unpaired = status_snapshot
                    .as_ref()
                    .is_some_and(|s| s.standing() == Standing::Unpaired);
                rsx! {
                    div { class: "{arrival}",
                        PageTitle { eyebrow: shown.eyebrow, text: shown.title, detail: shown.detail, icon: shown.icon }
                        if is_unpaired {
                            TextField {
                                label: "Chords address",
                                value: chords_endpoint(),
                                on_input: move |next| chords_endpoint.set(next),
                                placeholder: "https://...",
                                disabled: busy(),
                            }
                            TextField {
                                label: "Pairing code",
                                value: pairing_code(),
                                on_input: move |next| pairing_code.set(next),
                                placeholder: "Optional for local bridge",
                                disabled: busy(),
                            }
                        }
                        if !shown.steps.is_empty() {
                            TaskList { tasks: shown.steps }
                        }
                    }
                    if let Some(offer) = shown.offer {
                        div { class: "popup-footer",
                            Button { variant: ButtonVariant::Primary,
                                busy: busy(),
                                pending_label: "Working…",
                                onclick: move |_| {
                                    let action = offer.action;
                                    async move {
                                        match action {
                                            Action::Pair => pair_action().await,
                                            Action::OpenSource => go_to_source().await,
                                            Action::Connect => connect_action().await,
                                        }
                                    }
                                },
                                Icon { name: match offer.action { Action::OpenSource => IconName::ExternalLink, Action::Pair | Action::Connect => IconName::Link } }
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
                                if status_snapshot.is_some() {
                                    MenuItem { icon: IconName::ExternalLink,
                                        onclick: move |_| async move { go_to_source().await },
                                        "Go to {source_name}"
                                    }
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
                            } else if paired {
                                MenuSection {
                                    MenuItem { icon: IconName::X, danger: true,
                                        onclick: move |_| async move { act(Request::Unpair).await },
                                        "Unpair extension"
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
