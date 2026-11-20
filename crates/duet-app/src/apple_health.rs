//! Account-owned Apple Health source control in You.
use crate::{PatientSession, open_client};
use dioxus::prelude::*;
use duet_client::{
    apple_health::{AppleHealthError, IosAppleHealth as AppleHealth},
    health_upload::{RefreshStop, SyncPause},
};
use ui::Button;

fn sync_notice(reason: &SyncPause) -> &'static str {
    match reason {
        SyncPause::AccountChanged => "The signed-in account changed. Reopen Apple Health.",
        SyncPause::StorageUnavailable(_) => {
            "Could not access protected storage. If your device is locked, unlock it and retry."
        }
        SyncPause::NotConnected | SyncPause::StaleGeneration => {
            "Connect Apple Health again to resume imports."
        }
        SyncPause::Damaged => "The saved Apple Health queue could not be read.",
        SyncPause::Unauthorized => "Sign in again to continue the Apple Health import.",
    }
}

fn notice_for(error: &AppleHealthError) -> &'static str {
    match error {
        AppleHealthError::SessionRefused(_) => sync_notice(&SyncPause::Unauthorized),
        AppleHealthError::Stopped(stop) => match stop {
            RefreshStop::Paused(reason) => sync_notice(reason),
            RefreshStop::ReadFailed(_) | RefreshStop::Coverage(_) => {
                "Some Apple Health data could not be read."
            }
            RefreshStop::Rejected(..) => "Some Apple Health records were refused.",
            RefreshStop::Fenced(_) => "Some records belong to an earlier Apple Health connection.",
            RefreshStop::Parked => "Some Apple Health uploads reached their retry limit.",
            RefreshStop::WaitingForUpload => "Apple Health has records waiting to upload.",
        },
        AppleHealthError::Generation(_) => {
            "Could not check this account’s Apple Health connection."
        }
        AppleHealthError::DisconnectSync(_) => {
            "Apple Health disconnected on this iPhone, but Duet could not confirm with the server yet."
        }
        AppleHealthError::Permission(_) => "Apple Health could not finish the permission request.",
        AppleHealthError::Reader(_) => "Apple Health could not finish reading.",
        AppleHealthError::Clock(_) => "Could not check the device time for upload.",
    }
}

#[derive(Clone, Copy)]
enum Action {
    Connect,
    Refresh,
    Disconnect,
}

#[component]
pub(crate) fn AppleHealthControl(base_url: String, refreshed: Signal<u32>) -> Element {
    let patient = use_context::<PatientSession>();
    let open = move || AppleHealth::open_on_device(&patient.retention, &patient.fence);
    let mut connection = use_signal(open.clone());
    let mut busy = use_signal(|| false);
    let mut notice = use_signal(String::new);
    let state = |source: AppleHealth| Ok((source.connected()?, source));
    let connected = connection.read().clone().and_then(state);
    let mut run = move |source: AppleHealth, action| {
        if busy() {
            return;
        }
        let client = match open_client(base_url.clone(), (patient.access_token)()) {
            Ok(client) => client,
            Err(error) => {
                notice.set(error);
                return;
            }
        };
        busy.set(true);
        notice.set(String::new());
        spawn(async move {
            let result = async {
                match action {
                    Action::Connect => {
                        source.connect(&client).await?;
                        source.refresh(&client).await
                    }
                    Action::Refresh => source.refresh(&client).await,
                    Action::Disconnect => source.disconnect_and_sync(&client).await,
                }
            }
            .await;
            let next = *refreshed.peek() + 1;
            refreshed.set(next);
            if let Err(error) = result {
                notice.set(notice_for(&error).into());
            }
            busy.set(false);
        });
    };
    let auto_refresh = match &connected {
        Ok((true, source)) => Some(source.clone()),
        _ => None,
    };
    let first_refresh = run.clone();
    use_future(move || {
        let mut run = first_refresh.clone();
        let source = auto_refresh.clone();
        async move {
            if let Some(source) = source {
                run(source, Action::Refresh);
            }
        }
    });

    let (connected, source) = match connected {
        Ok(state) => state,
        Err(reason) => {
            let reason = SyncPause::from(reason);
            return rsx! {
                div { class: "source-row",
                    span { class: "source-name", "Apple Health" }
                    Button { disabled: busy(), onclick: move |_| {
                        notice.set(String::new());
                        let reopened = open();
                        connection.set(reopened.clone());
                        if let Ok((true, source)) = reopened.and_then(state) {
                            run(source, Action::Refresh);
                        }
                    }, "Retry" }
                }
                p { role: "alert", "{sync_notice(&reason)}" }
            };
        }
    };
    let import_source = source.clone();
    rsx! {
        div { class: "source-row",
            span { class: "source-name", "Apple Health" }
            if connected {
                Button { disabled: busy(), onclick: move |_| run(import_source.clone(), Action::Refresh), "Refresh" }
                Button { disabled: busy(), onclick: move |_| run(source.clone(), Action::Disconnect), "Disconnect" }
            } else {
                Button { disabled: busy(), onclick: move |_| run(import_source.clone(), Action::Connect), "Connect Apple Health" }
            }
        }
        p { class: "source-guidance", "{duet_client::APPLE_HEALTH_WEARABLE_INSTRUCTION}" }
        if busy() { p { role: "status", "Checking Apple Health…" } }
        if !notice().is_empty() { p { role: "alert", "{notice}" } }
    }
}
