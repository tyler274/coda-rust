//! Explicit selection from canonical inventory, never fabricated catalog revisions.
use crate::PatientSession;

#[component]
pub(crate) fn DeviceSources(
    base_url: String,
    blocked: bool,
    on_selected: EventHandler<duet_wire::models::DoctorCollection>,
    on_recheck: EventHandler,
) -> Element {
    let session = use_context::<PatientSession>();
    let mut reload = use_signal(|| 0u32);
    let mut busy = use_signal(|| false);
    let mut error = use_signal(String::new);
    let reader = session.clone();
    let url = base_url.clone();
    let snapshot = use_resource(move || {
        let _ = reload();
        let session = reader.clone();
        let url = url.clone();
        async move {
            let api = crate::open_client(url, session.access_token.peek().clone())
                .map_err(|_| duet_client::doctor::DoctorError::Unauthorized)?;
            let result = api.doctor_device_snapshot().await;
            if !session.retention.session().is_current(&session.fence) {
                return Err(duet_client::doctor::DoctorError::Unauthorized);
            }
            result
        }
    });
    rsx! {
        details {
            summary { "Select device measurements for this page" }
            p { "Review the complete dated measurement, units and source coverage. Selection is not approval or publication. If the latest snapshot changes during selection, reload and review the actual saved selection before saving your page." }
            button { disabled: busy(), onclick: move |_| reload.set(reload() + 1), "Refresh device snapshot" }
            if !error().is_empty() { p { role: "alert", "{error}" } }
            match snapshot() {
                None => rsx! { p { "Loading device snapshot…" } },
                Some(Err(duet_client::doctor::DoctorError::NotFound)) => rsx! { p { "No device snapshot has been uploaded." } },
                Some(Err(e)) => rsx! { p { role: "alert", "{e}" } },
                Some(Ok(snapshot)) => rsx! {
                    p { "Snapshot {snapshot.snapshot_id} · generated {snapshot.generated_at}" }
                    if snapshot.facts.is_empty() { p { "This snapshot contains no measurements." } }
                    for fact in &snapshot.facts {
                        {
                            let reviewed = snapshot.clone(); let patient = session.clone(); let url = base_url.clone();
                            let id = fact["id"].as_str().map(str::to_owned);
                            rsx! {
                                article {
                                    pre { "{serde_json::to_string_pretty(fact).unwrap_or_default()}" }
                                    if let Some(id) = id {
                                        button { disabled: busy() || blocked, onclick: move |_| {
                                            if busy() || blocked { return; }
                                            if !matches!(patient.retention.pending_operation(&patient.fence), Ok(None)) { error.set("Resolve the pending operation before selecting measurements.".into()); return; }
                                            busy.set(true); error.set(String::new()); let patient = patient.clone(); let url = url.clone(); let id = id.clone(); let reviewed = reviewed.clone();
                                            spawn(async move {
                                                let result = match crate::open_client(url, patient.access_token.peek().clone()) { Ok(api) => api.select_doctor_device_fact(&reviewed, &id).await, Err(_) => Err(duet_client::doctor::DoctorError::Unauthorized) };
                                                if !patient.retention.session().is_current(&patient.fence) { return; }
                                                match result { Ok(page) => on_selected.call(page), Err(e) => { error.set(format!("{e} Selection may have changed on the server. Reload and review the selected page before saving.")); on_recheck.call(()); } }
                                                busy.set(false);
                                            });
                                        }, "Select this device measurement" }
                                    }
                                }
                            }
                        }
                    }
                },
            }
        }
    }
}

use dioxus::prelude::*;
#[component]
pub(crate) fn DoctorSources(
    base_url: String,
    blocked: bool,
    on_selected: EventHandler<duet_wire::models::DoctorCollection>,
) -> Element {
    let session = use_context::<PatientSession>();
    let mut reload = use_signal(|| 0u32);
    let mut error = use_signal(String::new);
    let mut busy = use_signal(|| false);
    let url = base_url.clone();
    let patient = session.clone();
    let inventory = use_resource(move || {
        let _ = reload();
        let url = url.clone();
        let patient = patient.clone();
        async move {
            let api = crate::open_client(url, patient.access_token.peek().clone())
                .map_err(|_| duet_client::doctor::DoctorError::Unauthorized)?;
            let result = api.doctor_source_inventory().await;
            if !patient.retention.session().is_current(&patient.fence) {
                return Err(duet_client::doctor::DoctorError::Unauthorized);
            }
            result
        }
    });
    rsx! {
        details {
            summary { "Select source records for this page" }
            p { "Selection is not publication. Review the full literal record and save your page before sharing. Source changes will require renewed review." }
            if blocked { p { "Finish or discard your draft, and resolve pending operations, before changing the source selection." } }
            button { disabled: busy(), onclick: move |_| reload.set(reload() + 1), "Refresh source records" }
            if !error().is_empty() { p { role: "alert", "{error}" } }
            match inventory() {
                None => rsx! { p { "Loading canonical records…" } },
                Some(Err(e)) => rsx! { p { role: "alert", "{e}" } },
                Some(Ok(source)) => {
                    let mut health = source.health_evidence.into_iter(); let mut notes = source.clinical_notes.into_iter();
                    rsx! {
                        if source.canonical_revisions.is_empty() { p { "No source records available." } }
                        for address in source.canonical_revisions {
                            {
                                let payload = match address.entity { duet_wire::models::addressed_record_revision::Entity::HealthEvidence => health.next(), duet_wire::models::addressed_record_revision::Entity::ClinicalNote => notes.next() };
                                let url = base_url.clone(); let patient = session.clone();
                                rsx! {
                                    article {
                                        p { "Record revision {address.revision}" }
                                        if let Some(payload) = payload {
                                            pre { "{serde_json::to_string_pretty(&payload).unwrap_or_default()}" }
                                            button { disabled: blocked || busy(), onclick: move |_| {
                                                if blocked || busy() { return; }
                                                if !matches!(patient.retention.pending_operation(&patient.fence), Ok(None)) { error.set("Resolve the pending operation before selecting another source.".into()); return; }
                                                busy.set(true); error.set(String::new());
                                                let patient = patient.clone(); let url = url.clone();
                                                let selection = duet_wire::models::CanonicalRecordSelection::new(duet_wire::models::canonical_record_selection::Kind::Record, address.canonical_record_id, address.revision);
                                                spawn(async move {
                                                    let result = match crate::open_client(url, patient.access_token.peek().clone()) { Ok(api) => api.select_doctor_record(&selection).await, Err(_) => Err(duet_client::doctor::DoctorError::Unauthorized) };
                                                    if !patient.retention.session().is_current(&patient.fence) { return; }
                                                    match result { Ok(collection) => on_selected.call(collection), Err(e) => error.set(e.to_string()) }
                                                    busy.set(false);
                                                });
                                            }, "Select this record for my doctor" }
                                        } else { p { role: "alert", "Exact source payload unavailable; selection blocked." } }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
