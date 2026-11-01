//! Explicit conflict review. Unknown outcomes cannot be discarded through this surface.
use crate::PatientSession;
use dioxus::prelude::*;
use duet_client::doctor::{DoctorError, PendingDoctorOperation};
use ui::{Button, Disclosure};

#[component]
pub(crate) fn OperationRecovery(base_url: String, on_released: EventHandler) -> Element {
    let session = use_context::<PatientSession>();
    let mut reviewed = use_signal(|| None::<PendingDoctorOperation>);
    let mut review_text = use_signal(String::new);
    let mut busy = use_signal(|| false);
    let mut error = use_signal(String::new);
    let mut consent = use_signal(|| false);
    let mut revision = use_signal(|| 0u32);
    let _ = revision();
    let conflict = session
        .retention
        .conflicting_operation(&session.fence)
        .ok()
        .flatten();
    let reader = session.clone();
    let readback = move |_| {
        if busy() {
            return;
        }
        reviewed.set(None);
        consent.set(false);
        error.set(String::new());
        busy.set(true);
        let patient = reader.clone();
        let url = base_url.clone();
        spawn(async move {
            let operation = patient
                .retention
                .conflicting_operation(&patient.fence)
                .ok()
                .flatten();
            let result = async {
                let api = crate::open_client(url, patient.access_token.peek().clone()).map_err(|_| DoctorError::Unauthorized)?;
                let page = api.doctor_workspace().await?;
                let publication = api.doctor_share_status().await?;
                let publication_text = publication.map(|p| format!("Publication {} · generation {} · expires {} · revoked {}", p.id, p.generation, p.expires_at, p.revoked)).unwrap_or("No publication returned by the server.".into());
                let generation = match &operation { Some(PendingDoctorOperation::Publish(p)) => Some(p.expected_generation), Some(PendingDoctorOperation::Clear { expected_generation, .. }) => Some(*expected_generation), _ => None };
                let receipt = match generation.and_then(|g| i32::try_from(g).ok()) {
                    Some(g) => match api.doctor_save_receipt(g).await {
                        Ok(receipt) => format!("Exact page-save receipt for base {g}:\n{}", serde_json::to_string_pretty(&receipt).unwrap_or_default()),
                        Err(DoctorError::NotFound) => "No page-save receipt exists at that base. This is not a publication or deletion receipt, and does not prove those operations failed.".into(),
                        Err(e) => return Err(e),
                    },
                    None => "No page-save receipt requested for this operation.".into(),
                };
                Ok(format!("Current server page (not proof of the previous command outcome):\n{}\n\n{publication_text}\n\n{receipt}", serde_json::to_string_pretty(&page).unwrap_or_default()))
            }.await;
            if !patient.retention.session().is_current(&patient.fence) {
                return;
            }
            match result {
                Ok(text) => {
                    review_text.set(text);
                    reviewed.set(operation);
                }
                Err(e) => error.set(e.to_string()),
            }
            busy.set(false);
            revision.set(revision() + 1);
        });
    };
    let release = move |_| {
        if busy() || !consent() {
            return;
        }
        let Some(operation) = reviewed() else {
            return;
        };
        match session
            .retention
            .archive_conflicting_operation(&session.fence, &operation)
        {
            Ok(()) => {
                reviewed.set(None);
                consent.set(false);
                review_text.set(String::new());
                revision.set(revision() + 1);
                on_released.call(());
            }
            Err(e) => error.set(e),
        }
    };
    rsx! {
        Disclosure { title: "Recover or resolve a previous operation",
            if cfg!(target_arch = "wasm32") {
                p { role: "alert", "This browser keeps credentials, drafts and retry metadata in memory only. Reloading or closing loses the original request identity. After signing in again, read the server state below before making any new approval. Current state is not proof that a previous request succeeded. Do not recreate an uncertain operation as a retry." }
            }
            p { "Readback does not save, publish, revoke or delete anything." }
            Button { disabled: busy(), onclick: readback, "Read current page, publication and available receipt" }
            if !error().is_empty() { p { role: "alert", "{error}" } }
            if !review_text().is_empty() { pre { "{review_text}" } }
            if conflict.is_some() && reviewed() == conflict {
                p { "The exact retained command was refused as stale/conflicting. Review the state and receipt above. Ending its retry intent does not undo a prior write or revoke any link. Its command metadata is archived under this account; your local draft and server receipts are preserved." }
                label { input { r#type: "checkbox", checked: consent(), onchange: move |e| consent.set(e.checked()) } "I reviewed the current page and publication. End this conflicting retry intent; any replacement requires a new explicit approval." }
                Button { disabled: busy() || !consent(), onclick: release, "Archive conflicting intent and return to review" }
            } else if reviewed().is_none() {
                p { "Unknown or unsupported outcomes cannot be discarded here. Keep the original operation and retry/read back, or wait for server support. Unsaved drafts have a separate explicit discard control." }
            }
        }
    }
}
