//! You-tab public directory and private care team. AccountRoot owns keyed reset/cancellation.
use crate::{
    PatientSession,
    care_team_model::{address, maps, optional, telephone},
};
use dioxus::prelude::*;
use duet_client::{DuetClient, care_team::*};
use uuid::Uuid;

pub(super) fn current(session: &PatientSession) -> bool {
    session.retention.session().is_current(&session.fence)
}
pub(super) fn client(base: String, session: &PatientSession) -> Result<DuetClient, CareTeamError> {
    if !current(session) {
        return Err(CareTeamError::Unauthorized);
    }
    crate::open_client(base, session.access_token.peek().clone())
        .map_err(|_| CareTeamError::Unauthorized)
}

#[derive(Clone)]
enum Mutation {
    Add(String),
    Patch(Uuid, CareTeamPatchRequest),
    Remove(Uuid),
    Refresh(Uuid),
}

#[component]
pub(super) fn CareTeam(base_url: String, active: bool) -> Element {
    let session = use_context::<PatientSession>();
    let mut members = use_signal(Vec::<CareTeamMember>::new);
    let mut loaded = use_signal(|| false);
    let mut loading = use_signal(|| false);
    let mut busy = use_signal(|| false);
    let mut error = use_signal(String::new);
    let mut reload = use_signal(|| 0u64);
    let mut search_open = use_signal(|| false);
    let mut records_open = use_signal(|| false);
    // An ambiguous mutation requires a list read before any further mutations.
    let mut reconcile = use_signal(|| false);
    let load_session = session.clone();
    let load_url = base_url.clone();
    let mut last_load = use_signal(|| None::<u64>);
    use_effect(use_reactive!(|(active)| {
        let attempt = reload();
        if !active || *last_load.peek() == Some(attempt) {
            return;
        }
        last_load.set(Some(attempt));
        loading.set(true);
        let session = load_session.clone();
        let url = load_url.clone();
        spawn(async move {
            let result = match client(url, &session) {
                Ok(api) => api.care_team().await,
                Err(e) => Err(e),
            };
            if !current(&session) {
                return;
            }
            match result {
                Ok(value) => {
                    members.set(value);
                    loaded.set(true);
                    reconcile.set(false);
                    error.set(String::new());
                }
                Err(e) => {
                    error.set(e.to_string());
                }
            }
            loading.set(false);
        });
    }));
    let mutate_session = session.clone();
    let mutate_url = base_url.clone();
    let mutate = EventHandler::new(move |operation: Mutation| {
        if busy() || loading() || reconcile() {
            return;
        }
        busy.set(true);
        error.set(String::new());
        let session = mutate_session.clone();
        let url = mutate_url.clone();
        spawn(async move {
            let result = async {
                let api = client(url, &session)?;
                match operation {
                    Mutation::Add(npi) => api.add_care_team_member(&npi).await.map(Some),
                    Mutation::Patch(id, body) => {
                        api.patch_care_team_member(id, &body).await.map(Some)
                    }
                    Mutation::Refresh(id) => api.refresh_care_team_member(id).await.map(Some),
                    Mutation::Remove(id) => {
                        api.delete_care_team_member(id).await?;
                        if current(&session) {
                            members.write().retain(|m| m.id != id);
                        }
                        Ok(None)
                    }
                }
            }
            .await;
            if !current(&session) {
                return;
            }
            match result {
                Ok(Some(member)) => {
                    let mut rows = members.write();
                    if let Some(row) = rows.iter_mut().find(|r| r.id == member.id) {
                        *row = member;
                    } else {
                        rows.push(member);
                    }
                }
                Ok(None) => {}
                Err(e) => {
                    reconcile.set(matches!(
                        e,
                        CareTeamError::Uncertain
                            | CareTeamError::Conflict
                            | CareTeamError::NotFound
                    ));
                    error.set(e.to_string());
                }
            }
            busy.set(false);
        });
    });
    let waiting = crate::use_deferred_status(loading);
    let saving = crate::use_deferred_status(busy);
    let disabled = busy() || loading() || reconcile();
    rsx! {
        section { class: "care-team", aria_label: "Care team",
            header { class: "care-team-heading",
                h2 { "Care team" }
                button { r#type: "button", onclick: move |_| search_open.set(!search_open()), aria_expanded: search_open(), "Find a clinician" }
            }
            p { class: "status-detail", "Your saved clinicians and organizations. NPPES is a public directory, not verification of a physician, current contact details, or a relationship with you." }
            p { class: "care-progress", role: "status", if waiting { "Checking care team…" } else if saving { "Saving care-team change…" } }
            if !loaded() && !loading() { p { "Your care team is unavailable." } }
            if loaded() && members().is_empty() { p { "No care-team members yet. Search the public directory to add someone." } }
            if !error().is_empty() { p { role: "alert", "{error}" } }
            if reconcile() { p { "Reload your care team to check the outcome before making another change. No change was automatically retried." } }
            button { r#type: "button", disabled: loading() || busy(), onclick: move |_| reload.set(reload() + 1), "Reload care team" }
            if search_open() {
                SearchSheet { base_url: base_url.clone(), members: members(), disabled,
                    on_add: move |npi| mutate.call(Mutation::Add(npi)), on_close: move |_| search_open.set(false) }
            }
            div { class: "care-team-members",
                for member in members() {
                    MemberRow { key: "{member.id}", member, disabled,
                        on_mutate: mutate }
                }
            }
            button { r#type: "button", disabled: !loaded(), aria_expanded: records_open(), onclick: move |_| records_open.set(!records_open()), "Request my records" }
            // Keep the form mounted while hidden: closing it must not lose a known or uncertain POST identity.
            div { hidden: !records_open(),
                crate::release_form::ReleaseForm { base_url, members: members(), open: records_open(), on_close: move |_| records_open.set(false) }
            }
        }
    }
}

#[component]
fn SearchSheet(
    base_url: String,
    members: Vec<CareTeamMember>,
    disabled: bool,
    on_add: EventHandler<String>,
    on_close: EventHandler<()>,
) -> Element {
    let session = use_context::<PatientSession>();
    let mut query = use_signal(ClinicianSearchRequest::default);
    let mut results = use_signal(|| None::<Vec<DirectoryClinician>>);
    let mut busy = use_signal(|| false);
    let mut error = use_signal(String::new);
    let mut revision = use_signal(|| 0u64);
    let search = move |_| {
        if busy() {
            return;
        }
        let body = query();
        let generation = revision();
        let session = session.clone();
        let url = base_url.clone();
        busy.set(true);
        error.set(String::new());
        results.set(None);
        spawn(async move {
            let result = match client(url, &session) {
                Ok(api) => api.search_clinicians(&body).await,
                Err(e) => Err(e),
            };
            if !current(&session) {
                return;
            }
            busy.set(false);
            if revision() != generation {
                return;
            }
            match result {
                Ok(rows) => results.set(Some(rows)),
                Err(e) => error.set(e.to_string()),
            }
        });
    };
    let waiting = crate::use_deferred_status(busy);
    rsx! {
        section { class: "care-team-search voice-card", aria_label: "Search public NPPES directory",
            h3 { "Find a clinician or organization" }
            p { "Search by name and location. State alone is not enough. Optional specialty and city can narrow the results." }
            for (index, label, value) in [
                (0, "First name", query().first_name.unwrap_or_default()),
                (1, "Last name", query().last_name.unwrap_or_default()),
                (2, "Organization name", query().organization_name.unwrap_or_default()),
                (3, "State (two letters)", query().state.unwrap_or_default()),
                (4, "City (optional)", query().city.unwrap_or_default()),
                (5, "Specialty (optional)", query().taxonomy_description.unwrap_or_default()),
            ] {
                label { class: "care-field", "{label}"
                    input { value, maxlength: 200, oninput: move |event| {
                        revision.set(revision() + 1); results.set(None); error.set(String::new());
                        let value = optional(&event.value()); let mut q = query.write();
                        match index { 0 => q.first_name = value, 1 => q.last_name = value, 2 => q.organization_name = value,
                            3 => q.state = value.map(|v| v.to_ascii_uppercase()), 4 => q.city = value, _ => q.taxonomy_description = value }
                    } }
                }
            }
            p { class: "care-progress", role: "status", if waiting { "Searching public directory…" } }
            button { r#type: "button", disabled: busy(), onclick: search, "Search directory" }
            button { r#type: "button", onclick: move |_| on_close.call(()), "Close search" }
            if !error().is_empty() { p { role: "alert", "{error}" } }
            if let Some(rows) = results() {
                if rows.is_empty() { p { role: "status", "No matching directory entries. Try a different name or location." } }
                if rows.len() >= 10 { p { "Showing a limited set of matches; narrow your search if needed." } }
                for clinician in rows {
                    article { class: "care-result",
                        DirectorySummary { clinician: clinician.clone() }
                        button { r#type: "button", disabled: disabled || members.iter().any(|m| m.clinician.npi == clinician.npi),
                            aria_label: "Add {clinician.name} to care team", onclick: move |_| on_add.call(clinician.npi.clone()),
                            if members.iter().any(|m| m.clinician.npi == clinician.npi) { "Added" } else { "Add" }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn DirectorySummary(clinician: DirectoryClinician) -> Element {
    rsx! {
        h3 { "{clinician.name}" }
        if let Some(credential) = clinician.credential { p { "{credential}" } }
        if let Some(taxonomy) = clinician.primary_taxonomy { p { "{taxonomy.description}" } }
        if let Some(address) = clinician.practice_address { p { "{address.city}, {address.state}" } }
        p { class: "status-detail", "NPI {clinician.npi} · Public NPPES directory" }
    }
}

#[component]
fn MemberRow(member: CareTeamMember, disabled: bool, on_mutate: EventHandler<Mutation>) -> Element {
    let mut editing = use_signal(|| false);
    let mut role = use_signal(|| member.role.clone());
    let mut nickname = use_signal(|| member.nickname.clone());
    let mut note = use_signal(|| member.note.clone());
    let mut removing = use_signal(|| false);
    let mut copy_status = use_signal(String::new);
    let session = use_context::<PatientSession>();
    let clinician = &member.clinician;
    let id = member.id;
    let edit_member = member.clone();
    rsx! {
        article { class: "voice-card care-member",
            DirectorySummary { clinician: *clinician.clone() }
            if !member.nickname.is_empty() { p { "Your nickname: {member.nickname}" } }
            if !member.role.is_empty() { p { "Your role label: {member.role}" } }
            if !member.note.is_empty() { p { "{member.note}" } }
            p { class: "status-detail", "Directory snapshot refreshed {member.refreshed_at}. Confirm current details directly before sending records. Refreshing does not verify these contacts." }
            if let Some(phone) = &clinician.phone {
                if let Some(link) = telephone(phone) { a { href: link, "Call directory number: {phone}" } }
                else { p { "Directory phone: {phone} (dial manually after checking)" } }
            }
            if let Some(fax) = &clinician.fax {
                p { "Directory fax: {fax}" }
                button { r#type: "button", onclick: {
                    let fax = fax.clone(); let session = session.clone();
                    move |_| { let fax = fax.clone(); let session = session.clone(); spawn(async move {
                        if !current(&session) { return; }
                        let result = copy_contact(&fax).await;
                        if current(&session) { copy_status.set(match result { Ok(()) => "Fax number copied. Confirm it is current before use.".into(), Err(e) => e }); }
                    }); }
                }, "Copy fax" }
            }
            if let Some(practice) = &clinician.practice_address {
                a { href: maps(&address(practice)), target: "_blank", rel: "noopener noreferrer", "referrerpolicy": "no-referrer", "Open public address in maps: {address(practice)}" }
            }
            for endpoint in clinician.endpoints.iter().filter(|e| e.kind.eq_ignore_ascii_case("DIRECT")) {
                p { "Directory Direct address: {endpoint.address}" }
            }
            if clinician.endpoints.iter().any(|e| e.kind.eq_ignore_ascii_case("DIRECT")) {
                p { class: "status-detail", "Direct is secure clinical messaging, not ordinary email. Duet does not send records or PHI to this address." }
            }
            if !copy_status().is_empty() { p { role: "status", "{copy_status}" } }
            button { r#type: "button", disabled, onclick: move |_| {
                role.set(edit_member.role.clone()); nickname.set(edit_member.nickname.clone()); note.set(edit_member.note.clone()); editing.set(true);
            }, "Edit labels" }
            button { r#type: "button", disabled, onclick: move |_| on_mutate.call(Mutation::Refresh(id)), "Refresh public directory" }
            button { r#type: "button", disabled, onclick: move |_| removing.set(true), "Remove member" }
            if editing() {
                fieldset { disabled,
                    legend { "Your labels for this member" }
                    label { class: "care-field", "Role" input { value: role, maxlength: 500, oninput: move |e| role.set(e.value()) } }
                    label { class: "care-field", "Nickname" input { value: nickname, maxlength: 500, oninput: move |e| nickname.set(e.value()) } }
                    label { class: "care-field", "Private note" textarea { value: note, maxlength: 2000, oninput: move |e| note.set(e.value()) } }
                    button { r#type: "button", onclick: move |_| on_mutate.call(Mutation::Patch(id, CareTeamPatchRequest { role: Some(role()), nickname: Some(nickname()), note: Some(note()) })), "Save labels" }
                    button { r#type: "button", onclick: move |_| editing.set(false), "Close editor" }
                }
            }
            if removing() {
                p { "Remove this member? This also deletes saved release templates where this member is the discloser." }
                button { r#type: "button", disabled, onclick: move |_| on_mutate.call(Mutation::Remove(id)), "Confirm removal" }
                button { r#type: "button", onclick: move |_| removing.set(false), "Keep member" }
            }
        }
    }
}

async fn copy_contact(value: &str) -> Result<(), String> {
    #[cfg(target_arch = "wasm32")]
    {
        let window = web_sys::window()
            .ok_or("Clipboard unavailable. Select and copy the fax number manually.")?;
        wasm_bindgen_futures::JsFuture::from(window.navigator().clipboard().write_text(value))
            .await
            .map(|_| ())
            .map_err(|_| "Could not copy. Select and copy the fax number manually.".into())
    }
    #[cfg(not(target_arch = "wasm32"))]
    duet_client::retention::copy_native_link(value)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn directory_summary_mounts_with_missing_optional_fields() {
        let mut dom = VirtualDom::new(
            || rsx! { DirectorySummary { clinician: DirectoryClinician { name: "Public directory entry".into(), npi: "1234567893".into(), ..Default::default() } } },
        );
        dom.rebuild_in_place();
    }
}
