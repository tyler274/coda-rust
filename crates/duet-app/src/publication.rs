//! Patient controls for one pinned, expiring publication. No capability persistence or telemetry.
use crate::PatientSession;
use dioxus::prelude::*;
use duet_client::doctor::{PendingDoctorOperation, PublicationView, PublishRequest};
use ui::{ActionRow, Button, ButtonVariant};

#[component]
pub(crate) fn PublicationControls(base_url: String, generation: i32) -> Element {
    let patient = use_context::<PatientSession>();
    let mut status = use_signal(|| None::<PublicationView>);
    let mut checked = use_signal(|| false);
    let mut busy = use_signal(|| false);
    let mut error = use_signal(String::new);
    let mut lifetime = use_signal(|| 604800i64);
    let mut confirm_replace = use_signal(|| false);
    let mut confirm_revoke = use_signal(|| false);
    let mut qr = use_signal(|| None::<String>);
    let mut retry = use_signal(|| 0u64);
    let recovery = patient.retention.pending_operation(&patient.fence);
    let mut pending = use_signal(|| recovery.clone().ok().flatten());
    let restore_error = recovery.err();
    let load_patient = patient.clone();
    let load_url = base_url.clone();
    use_effect(move || {
        let _ = retry();
        confirm_replace.set(false);
        confirm_revoke.set(false);
        let session = load_patient.clone();
        let url = load_url.clone();
        checked.set(false);
        qr.set(None);
        spawn(async move {
            let result = match crate::open_client(url, session.access_token.peek().clone()) {
                Ok(api) => api.doctor_share_status().await,
                Err(_) => Err(duet_client::doctor::DoctorError::Unauthorized),
            };
            if !session.retention.session().is_current(&session.fence) {
                return;
            }
            match result {
                Ok(value) => {
                    status.set(value);
                    checked.set(true);
                }
                Err(e) => {
                    status.set(None);
                    error.set(e.to_string());
                }
            }
        });
    });
    let dispatch_patient = patient.clone();
    let dispatch = move |operation: PendingDoctorOperation| {
        if busy() {
            return;
        }
        if let Err(e) = dispatch_patient
            .retention
            .retain_operation(&dispatch_patient.fence, &operation)
        {
            error.set(e);
            return;
        }
        pending.set(Some(operation.clone()));
        busy.set(true);
        qr.set(None);
        error.set(String::new());
        let session = dispatch_patient.clone();
        let url = base_url.clone();
        spawn(async move {
            let result = async {
                let api = crate::open_client(url, session.access_token.peek().clone())
                    .map_err(|_| duet_client::doctor::DoctorError::Unauthorized)?;
                match &operation {
                    PendingDoctorOperation::Publish(request) => {
                        let published =
                            api.create_doctor_share_link(request)
                                .await
                                .inspect_err(|&e| {
                                    if e == duet_client::doctor::DoctorError::Conflict {
                                        let _ = session
                                            .retention
                                            .mark_operation_conflict(&session.fence, &operation);
                                    }
                                })?;
                        // Independent active readback before displaying capabilities.
                        let current = api.doctor_share_status().await?;
                        if current.as_ref().map(|p| p.id) != Some(published.id) {
                            return Err(duet_client::doctor::DoctorError::Uncertain);
                        }
                        Ok(current)
                    }
                    PendingDoctorOperation::Revoke {
                        operation_id,
                        publication_id,
                    } => {
                        api.revoke_doctor_share_link(*operation_id, *publication_id)
                            .await
                            .inspect_err(|&e| {
                                if e == duet_client::doctor::DoctorError::Conflict {
                                    let _ = session
                                        .retention
                                        .mark_operation_conflict(&session.fence, &operation);
                                }
                            })?;
                        let after = api.doctor_share_status().await?;
                        if after.as_ref().is_some_and(|p| !p.revoked) {
                            return Err(duet_client::doctor::DoctorError::Uncertain);
                        }
                        Ok(after)
                    }
                    PendingDoctorOperation::Clear { .. } => {
                        Err(duet_client::doctor::DoctorError::Conflict)
                    }
                }
            }
            .await;
            if !session.retention.session().is_current(&session.fence) {
                return;
            }
            match result {
                Ok(value) => {
                    if let Err(e) = session.retention.finish_operation(&session.fence) {
                        error.set(e);
                    } else {
                        pending.set(None);
                    }
                    status.set(value);
                    checked.set(true);
                    confirm_replace.set(false);
                    confirm_revoke.set(false);
                }
                Err(e) => {
                    error.set(e.to_string());
                    checked.set(false);
                    status.set(None);
                }
            }
            busy.set(false);
        });
    };
    let mut publish = dispatch.clone();
    let mut revoke = dispatch.clone();
    let mut repeat = dispatch;
    let current = status();
    let active = current.as_ref().is_some_and(|p| {
        !p.revoked
            && chrono::DateTime::parse_from_rfc3339(&p.expires_at)
                .is_ok_and(|expiry| expiry > chrono::Utc::now())
    });
    rsx! {
        section {
            h2 { "Share this saved page" }
            p { "Links contain a pinned copy of the page you approve. Later edits do not change it. Revocation stops future access, but cannot erase downloaded copies." }
            if let Some(message) = restore_error { p { role: "alert", "{message}" } }
            if !error().is_empty() { p { role: "alert", "{error}" } }
            Button { disabled: busy(), onclick: move |_| retry.set(retry() + 1), "Check active link" }
            if let Some(operation) = pending() {
                p { role: "status", "An operation still needs confirmation. Its original identity and approval are retained; do not create a new request." }
                if !matches!(operation, PendingDoctorOperation::Clear { .. }) {
                    Button { disabled: busy(), onclick: move |_| repeat(operation.clone()), "Retry original operation" }
                } else { p { "Finish the pending Start fresh operation before sharing." } }
            }
            if checked() && pending().is_none() {
                if let Some(link) = current.clone() {
                    p { "Pinned page generation {link.generation} · expires {link.expires_at}" }
                    if link.revoked { p { "This publication was revoked." } }
                    if active {
                        LinkCopy { label: "Web link".to_owned(), value: link.web_url.clone() }
                        LinkCopy { label: "SMART Health Link".to_owned(), value: link.shl_url.clone() }
                        ActionRow {
                            Button { onclick: move |_| qr.set(Some(link.web_url.clone())), "Show web QR" }
                            Button { onclick: move |_| qr.set(Some(link.shl_url.clone())), "Show SMART Health Link QR" }
                            Button { danger: true, disabled: busy(), onclick: move |_| confirm_revoke.set(true), "Revoke link…" }
                        }
                        if confirm_revoke() {
                            p { "Stop future retrieval of both link formats? Downloaded copies remain with their recipients." }
                            ActionRow {
                                Button { variant: ButtonVariant::Primary, danger: true, disabled: busy(), onclick: move |_| revoke(PendingDoctorOperation::Revoke { operation_id: uuid::Uuid::new_v4(), publication_id: link.id }), "Confirm revoke" }
                                Button { onclick: move |_| confirm_revoke.set(false), "Keep link" }
                            }
                        }
                    }
                } else { p { "No publication is active." } }
                label { "Link lifetime"
                    select { disabled: busy(), value: "{lifetime}", onchange: move |e| { if let Ok(seconds) = e.value().parse::<i64>() { lifetime.set(seconds); } },
                        option { value: "3600", "1 hour" }
                        option { value: "86400", "1 day" }
                        option { value: "604800", "7 days" }
                        option { value: "2592000", "30 days" }
                    }
                }
                Button { disabled: busy(), onclick: move |_| confirm_replace.set(true), if active { "Republish reviewed page…" } else { "Publish reviewed page…" } }
                if confirm_replace() {
                    p { "Publish the reviewed saved page generation {generation}? This replaces and revokes the previous publication in both formats." }
                    ActionRow {
                        Button { variant: ButtonVariant::Primary, disabled: busy(), onclick: move |_| publish(PendingDoctorOperation::Publish(PublishRequest { operation_id: uuid::Uuid::new_v4(), expected_generation: generation.into(), expires_in_seconds: lifetime() })), "Confirm publish" }
                        Button { onclick: move |_| confirm_replace.set(false), "Cancel" }
                    }
                }
            }
            if let Some(value) = qr() {
                match qrcode::QrCode::new(value.as_bytes()) {
                    Ok(code) => rsx! { div { role: "img", aria_label: "Private share link QR code", dangerous_inner_html: code.render::<qrcode::render::svg::Color>().min_dimensions(256,256).build() } },
                    Err(_) => rsx! { p { role: "alert", "This link is too long for a QR code; copy the full link instead." } },
                }
                Button { onclick: move |_| qr.set(None), "Hide QR" }
            }
        }
    }
}
#[component]
fn LinkCopy(label: String, value: String) -> Element {
    let mut message = use_signal(String::new);
    let copy = value.clone();
    rsx! {
        label { "{label}" input { readonly: true, r#type: "text", value: "{value}", aria_label: "{label}" } }
        Button { onclick: move |_| {
            let text = copy.clone();
            spawn(async move { message.set(match copy_link(&text).await { Ok(()) => "Copied. Anyone with this link can read the approved page.".into(), Err(e) => e }); });
        }, "Copy {label}" }
        if !message().is_empty() { p { role: "status", "{message}" } }
    }
}
async fn copy_link(value: &str) -> Result<(), String> {
    #[cfg(target_arch = "wasm32")]
    {
        let window = web_sys::window().ok_or("Browser unavailable")?;
        wasm_bindgen_futures::JsFuture::from(window.navigator().clipboard().write_text(value))
            .await
            .map(|_| ())
            .map_err(|_| "Clipboard permission denied; select and copy the link manually".into())
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        duet_client::retention::copy_native_link(value)
    }
}
