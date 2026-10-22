//! Self-asserted directory profile; no verified badge or authority changes.
use dioxus::prelude::*;
use physician_client::PhysicianClient;
use physician_wire::{ClinicianSearch, ClinicianSummary, NpiClaimRequest, WhoamiResponse};

#[component]
pub(super) fn NpiProfile(
    client: PhysicianClient,
    mut whoami: Signal<Option<WhoamiResponse>>,
) -> Element {
    let mut open = use_signal(|| false);
    let mut last = use_signal(String::new);
    let mut state = use_signal(String::new);
    let mut city = use_signal(String::new);
    let mut results = use_signal(Vec::<ClinicianSummary>::new);
    let mut busy = use_signal(|| false);
    let mut message = use_signal(String::new);
    let mut override_name = use_signal(|| false);
    let current = whoami.read().as_ref().and_then(|a| a.npi.clone());
    let admin = whoami.read().as_ref().is_some_and(|a| a.admin);
    let search_client = client.clone();
    let clear_client = client.clone();
    rsx! {
        button { r#type: "button", onclick: move |_| open.set(!open()), "NPI profile" }
        if open() {
            section { class: "npi-profile", aria_label: "NPI profile",
                button { r#type: "button", onclick: move |_| open.set(false), "Close NPI profile" }
                h2 { "Public directory association" }
                p { "Self-asserted only. An NPPES lookup or name match does not verify your identity, license, credentials, or control of an NPI, and grants no account permissions." }
                if let Some(ref association) = current {
                    p { "{association.clinician.name} · NPI {association.clinician.npi}" }
                    if let Some(ref specialty) = association.clinician.specialty { p { "{specialty}" } }
                    if let Some(ref address) = association.clinician.practice_address { p { "{address}" } }
                    p { "Directory lookup checked at Unix time {association.lookup_checked_at}. Name consistency match: {association.name_matched}." }
                    button { r#type: "button", disabled: busy(), onclick: move |_| {
                        let client = clear_client.clone(); busy.set(true); message.set(String::new());
                        spawn(async move {
                            match client.clear_npi().await {
                                Ok(()) => { if let Some(account) = whoami.write().as_mut() { account.npi = None; } message.set("Association removed.".into()); }
                                Err(error) => message.set(error.to_string()),
                            }
                            busy.set(false);
                        });
                    }, "Remove association" }
                }
                form { onsubmit: move |event| {
                    event.prevent_default();
                    let client = search_client.clone();
                    let optional = |value: String| if value.trim().is_empty() { None } else { Some(value.trim().to_owned()) };
                    let query = ClinicianSearch { last_name: optional(last()), state: optional(state()), city: optional(city()), ..Default::default() };
                    busy.set(true); message.set(String::new()); results.set(Vec::new());
                    spawn(async move {
                        match client.search_clinicians(&query).await {
                            Ok(found) => { if found.is_empty() { message.set("No directory results. Try different criteria.".into()); } results.set(found); }
                            Err(error) => message.set(error.to_string()),
                        }
                        busy.set(false);
                    });
                },
                    label { "Family name", input { value: "{last}", oninput: move |event| last.set(event.value()), required: true } }
                    label { "State", input { value: "{state}", oninput: move |event| state.set(event.value()), maxlength: "2" } }
                    label { "City (optional)", input { value: "{city}", oninput: move |event| city.set(event.value()) } }
                    button { r#type: "submit", disabled: busy(), "Search NPPES" }
                }
                if admin {
                    label { input { r#type: "checkbox", checked: override_name(), onchange: move |event| override_name.set(event.checked()) }, "Bypass name consistency check (administrator only; not verification)" }
                }
                p { role: "status", aria_live: "polite", "{message}" }
                for clinician in results() {
                    if clinician.individual {
                        div { key: "{clinician.npi}",
                            p { "{clinician.name} · NPI {clinician.npi}" }
                            if let Some(ref specialty) = clinician.specialty { p { "{specialty}" } }
                            if let Some(ref address) = clinician.practice_address { p { "{address}" } }
                            button { r#type: "button", disabled: busy(), onclick: {
                                let client = client.clone();
                                move |_| {
                                    let client = client.clone();
                                    let request = NpiClaimRequest { npi: clinician.npi.clone(), override_name: override_name() };
                                    busy.set(true); message.set(String::new());
                                    spawn(async move {
                                        match client.claim_npi(&request).await {
                                            Ok(association) => { if let Some(account) = whoami.write().as_mut() { account.npi = Some(association); } message.set("Self-asserted association saved. Identity and credentials have not been verified.".into()); }
                                            Err(error) => message.set(error.to_string()),
                                        }
                                        busy.set(false);
                                    });
                                }
                            }, "Associate with my profile" }
                        }
                    }
                }
            }
        }
    }
}
