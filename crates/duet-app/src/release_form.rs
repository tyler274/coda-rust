//! Explicit, patient-controlled records-request review. Never sends records to a recipient.
use crate::{
    PatientSession,
    care_team::{client, current},
    care_team_model::{ReleaseDraft, ReleaseIdentity, TextField, encode},
};
use dioxus::prelude::*;
use duet_client::care_team::*;

#[component]
pub(super) fn ReleaseForm(
    base_url: String,
    members: Vec<CareTeamMember>,
    open: bool,
    on_close: EventHandler<()>,
) -> Element {
    let patient = use_context::<PatientSession>();
    let mut draft = use_signal(|| ReleaseDraft::new(String::new()));
    let mut identity = use_signal(ReleaseIdentity::default);
    let mut busy = use_signal(|| false);
    let mut error = use_signal(String::new);
    let mut preview = use_signal(|| None::<String>);
    let mut saved_request = use_signal(|| None::<ReleaseAuthorizationRequest>);
    let mut saved_view = use_signal(|| None::<ReleaseAuthorizationView>);
    let mut reviewed = use_signal(|| false);
    let mut name_loaded = use_signal(|| false);
    let mut name_touched = use_signal(|| false);
    let mut profile_error = use_signal(|| false);
    let profile_url = base_url.clone();
    let profile_patient = patient.clone();
    use_effect(use_reactive!(|(open)| {
        if !open || *name_loaded.peek() {
            return;
        }
        name_loaded.set(true);
        let session = profile_patient.clone();
        let url = profile_url.clone();
        spawn(async move {
            // Same idempotent identity owner used by AccountRoot; sends no profile edits.
            let profile = match client(url, &session) {
                Ok(api) => api.ensure_account().await.ok(),
                Err(_) => None,
            };
            if !current(&session) {
                return;
            }
            if let Some(profile) = profile {
                if !name_touched() {
                    draft.write().set(
                        TextField::PatientName,
                        profile.display_name.flatten().unwrap_or_default(),
                    );
                }
            } else {
                profile_error.set(true);
            }
        });
    }));
    // Any edit invalidates review, including an edit changed back to its original text.
    use_effect(move || {
        let _ = draft();
        reviewed.set(false);
    });
    let request = draft().request(&members);
    let preview_current = request
        .as_ref()
        .ok()
        .is_some_and(|request| saved_request().as_ref() == Some(request));
    let save_patient = patient.clone();
    let save_url = base_url.clone();
    let save_members = members.clone();
    let save = move |_| {
        if busy() || !identity().may_save() {
            return;
        }
        let body = match draft().request(&save_members) {
            Ok(body) => body,
            Err(e) => {
                error.set(e);
                return;
            }
        };
        let session = save_patient.clone();
        let url = save_url.clone();
        let id = identity().id;
        busy.set(true);
        reviewed.set(false);
        preview.set(None);
        error.set(String::new());
        spawn(async move {
            let api = match client(url, &session) {
                Ok(api) => api,
                Err(e) => {
                    if current(&session) {
                        error.set(e.to_string());
                        busy.set(false);
                    }
                    return;
                }
            };
            let saved = match id {
                Some(id) => api.update_release_authorization(id, &body).await,
                None => api.create_release_authorization(&body).await,
            };
            if !current(&session) {
                return;
            }
            match saved {
                Ok(value) => {
                    // Capture identity BEFORE preview: a failed GET must never trigger a second POST.
                    identity.write().saved(value.id);
                    saved_view.set(Some(value.clone()));
                    saved_request.set(Some(body));
                    match api.release_authorization_preview(value.id).await {
                        Ok(svg) if current(&session) => preview.set(Some(svg)),
                        Err(e) if current(&session) => {
                            error.set(format!("Template saved. Preview unavailable: {e}"))
                        }
                        _ => return,
                    }
                }
                Err(e) => {
                    identity.write().failed(e);
                    error.set(e.to_string());
                }
            }
            if current(&session) {
                busy.set(false);
            }
        });
    };
    let download_patient = patient.clone();
    let download_url = base_url;
    let download = move |_| {
        if busy() || !reviewed() || !preview_current || preview().is_none() {
            return;
        }
        let Some(id) = identity().id else {
            return;
        };
        let session = download_patient.clone();
        let url = download_url.clone();
        busy.set(true);
        error.set(String::new());
        spawn(async move {
            let result = match client(url, &session) {
                Ok(api) => api.release_authorization_pdf(id).await,
                Err(e) => Err(e),
            };
            if !current(&session) {
                return;
            }
            match result {
                Ok(bytes) => match crate::export::present_pdf_named(
                    &bytes,
                    "records-request-review.pdf",
                ) {
                    Ok(()) => error.set(
                        "PDF handed to download/share. Nothing was sent to the recipient by Duet."
                            .into(),
                    ),
                    Err(e) => error.set(e),
                },
                Err(e) => error.set(e.to_string()),
            }
            busy.set(false);
        });
    };
    let waiting = crate::use_deferred_status(busy);
    rsx! {
        section { class: "care-release voice-card", aria_label: "Request my records review template",
            h3 { "Request my records" }
            p { "Prepare a review template, not an automatic records request. Duet does not transmit records or this form to anyone." }
            p { "A typed name is not an authenticated electronic signature. The form requires recipient and legal review before use; the recipient may require its own authorization or signature process." }
            p { "You may revoke an authorization as described on the form, subject to actions already taken in reliance on it. Treatment is not conditioned on signing. Disclosed information may be redisclosed; you are entitled to a copy. Read the full statements in the preview." }
            if profile_error() { p { "Your display name could not be loaded. Enter it below; other patient details are never inferred." } }
            if members.is_empty() { p { "Add a care-team member before choosing who will disclose records." } }
            fieldset { disabled: busy(),
                legend { "1. People and organizations" }
                label { class: "care-field", "Who should disclose the records?"
                    select { value: draft().discloser, onchange: move |e| draft.write().discloser = e.value(),
                        option { value: "", "Choose a care-team member" }
                        for member in &members { option { value: "{member.id}", "{member.clinician.name} · NPI {member.clinician.npi}" } }
                    }
                }
                for field in [TextField::PatientName, TextField::BirthDate, TextField::PatientAddress, TextField::PatientPhone] {
                    DraftField { field, draft, on_edit: move |_| name_touched.set(true) }
                }
                label { class: "care-field", "Who should receive the records?"
                    select { value: draft().recipient, onchange: move |e| draft.write().recipient = e.value(),
                        option { value: "patient", "Me (the patient above)" }
                        option { value: "other", "Another person or organization" }
                        for member in &members { option { value: "{member.id}", "{member.clinician.name} · NPI {member.clinician.npi}" } }
                    }
                }
                if draft().recipient == "other" {
                    for field in [TextField::RecipientName, TextField::RecipientAddress, TextField::RecipientPhone, TextField::RecipientFax] {
                        DraftField { field, draft, on_edit: move |_| {} }
                    }
                }
                p { "Directory addresses and contacts are public snapshots, not verified current destinations. Confirm them with the discloser and recipient before using the form." }
            }
            fieldset { disabled: busy(),
                legend { "2. Information to request" }
                for (index, label) in ["Full record (subject to exclusions below)", "Visit notes", "Labs", "Imaging", "Medications"].into_iter().enumerate() {
                    label { class: "care-check", input { r#type: "checkbox", checked: draft().categories[index], onchange: move |e| draft.write().categories[index] = e.checked() } "{label}" }
                }
                DraftField { field: TextField::DateStart, draft, on_edit: move |_| {} }
                DraftField { field: TextField::DateEnd, draft, on_edit: move |_| {} }
                p { "Psychotherapy notes are excluded and require separate authorization. Full record does not override sensitive-category exclusions." }
                p { "Sensitive-category initials do not ensure compliance with 42 CFR Part 2 or state law. These requests require separate recipient/legal review; additional consent or forms may be necessary." }
                for (index, label) in ["Mental health", "Substance use (42 CFR Part 2)", "HIV/STI", "Genetic information"].into_iter().enumerate() {
                    label { class: "care-check", input { r#type: "checkbox", checked: draft().sensitive[index], onchange: move |e| {
                        let mut value = draft.write(); value.sensitive[index] = e.checked(); if !e.checked() { value.initials[index].clear(); }
                    } } "Request {label}" }
                    if draft().sensitive[index] {
                        label { class: "care-field", "Separate initials for {label}"
                            input { value: draft().initials[index].clone(), maxlength: 40, oninput: move |e| draft.write().initials[index] = e.value() }
                        }
                    }
                }
            }
            fieldset { disabled: busy(),
                legend { "3. Purpose, expiration and typed name" }
                DraftField { field: TextField::Purpose, draft, on_edit: move |_| {} }
                label { class: "care-field", "Expiration type"
                    select { value: if draft().expiration_is_event { "event" } else { "date" }, onchange: move |e| draft.write().expiration_is_event = e.value() == "event",
                        option { value: "date", "Date" } option { value: "event", "Event" }
                    }
                }
                if draft().expiration_is_event { DraftField { field: TextField::ExpirationEvent, draft, on_edit: move |_| {} } }
                else { DraftField { field: TextField::ExpirationDate, draft, on_edit: move |_| {} } }
                DraftField { field: TextField::TypedName, draft, on_edit: move |_| {} }
                p { "Typed name is optional for a draft. It does not authenticate your identity or establish a legally valid electronic signature." }
            }
            if identity().uncertain_create {
                p { role: "alert", "We cannot confirm whether the new template was saved. Creating another is blocked in this session to avoid a duplicate. Keep this form open and contact support to reconcile the original request; do not resubmit it from another tab or session." }
            }
            if let Some(id) = identity().id { p { class: "status-detail", "Template reference: {id}. Saving edits updates this same template." } }
            button { r#type: "button", disabled: busy() || !identity().may_save(), onclick: save,
                if identity().id.is_some() { "Save edits and preview same template" } else { "Save review template and preview" }
            }
            p { class: "care-progress", role: "status", if waiting { "Preparing private review template…" } }
            if !error().is_empty() { p { role: "status", "{error}" } }
            if preview_current {
                if let Some(svg) = preview() {
                    h4 { "4. Review before downloading" }
                    if let Some(view) = saved_view() { AccessibleReview { view } }
                    // SVG is an inert image document, never injected as executable inner HTML.
                    img { class: "release-preview", src: "data:image/svg+xml;charset=utf-8,{encode(&svg)}", alt: "Records-request review template. Review patient, discloser, recipient, scope, expiration, typed name and required statements." }
                    label { class: "care-check",
                        input { r#type: "checkbox", checked: reviewed(), disabled: busy(), onchange: move |e| reviewed.set(e.checked()) }
                        "I reviewed the patient details, discloser, recipient, information, expiration and statements. I understand this needs recipient/legal review and is not an authenticated e-signature."
                    }
                    button { r#type: "button", disabled: busy() || !reviewed(), onclick: download, "Download or share review PDF" }
                    p { "The PDF contains private details. Store and share it carefully. Downloading is not delivery to your care team." }
                }
            } else if saved_request().is_some() {
                p { "Your edits have not been saved or reviewed. Save this same template again to preview the current content." }
            }
            button { r#type: "button", onclick: move |_| on_close.call(()), "Close records form (keep draft in this session)" }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn accessible_review_mounts_without_optional_directory_contacts() {
        let mut dom = VirtualDom::new(
            || rsx! { AccessibleReview { view: ReleaseAuthorizationView::default() } },
        );
        dom.rebuild_in_place();
    }
}

/// Text equivalent of the server-owned parties and scope, also usable by screen readers.
#[component]
fn AccessibleReview(view: ReleaseAuthorizationView) -> Element {
    let recipient = if view.recipient.kind == duet_wire::models::release_recipient::Kind::Patient {
        ReleaseParty::new(
            view.patient.name.clone(),
            view.patient.address.clone(),
            view.patient.phone.clone(),
            String::new(),
        )
    } else {
        view.recipient.party.as_deref().cloned().unwrap_or_default()
    };
    rsx! {
        div { class: "release-text-review",
            h5 { "Saved template details" }
            p { "Patient: {view.patient.name}. Date of birth: {view.patient.date_of_birth}. Address: {view.patient.address}. Phone: {view.patient.phone}." }
            p { "Discloser: {view.discloser.name}. NPI: {view.discloser.npi.as_deref().unwrap_or(\"Not provided\")}. Address: {view.discloser.address}. Phone: {view.discloser.phone}. Fax: {view.discloser.fax}." }
            p { "Recipient: {recipient.name}. Address: {recipient.address}. Phone: {recipient.phone}. Fax: {recipient.fax}." }
            p { "Purpose: {view.purpose}" }
            p { "Record dates: {view.scope.date_start.map(|d| d.to_string()).unwrap_or_else(|| \"Not restricted\".into())} through {view.scope.date_end.map(|d| d.to_string()).unwrap_or_else(|| \"Not restricted\".into())}" }
            ul {
                for (label, selected) in [("Full record", view.scope.full_record), ("Visit notes", view.scope.visit_notes), ("Labs", view.scope.labs), ("Imaging", view.scope.imaging), ("Medications", view.scope.medications)] {
                    li { "{label}: " if selected == Some(true) { "Requested (subject to exclusions)" } else { "Not selected" } }
                }
                for (label, sensitive) in [("Mental health", &view.scope.mental_health), ("Substance use", &view.scope.substance_use), ("HIV/STI", &view.scope.hiv_sti), ("Genetic information", &view.scope.genetic)] {
                    li { "{label}: " if let Some(value) = sensitive.as_ref().filter(|v| v.requested) { "Requested; initials {value.initials}; separate legal review required" } else { "Excluded" } }
                }
            }
            p { "Psychotherapy notes are excluded. Sensitive initials are not a guarantee of Part 2 or state-law compliance." }
            p { "Expiration: {view.expires_on.map(|d| d.to_string()).or_else(|| view.expiration_event.clone()).unwrap_or_default()}" }
            p { "Typed name: {view.signed_name.as_deref().unwrap_or(\"Not provided\")}. This is not an authenticated electronic signature." }
            p { "Typed-name timestamp: {view.signed_at.map(|d| d.to_string()).unwrap_or_else(|| \"Not provided\".into())}" }
        }
    }
}

#[component]
fn DraftField(
    field: TextField,
    mut draft: Signal<ReleaseDraft>,
    on_edit: EventHandler<()>,
) -> Element {
    rsx! { label { class: "care-field", "{field.label()}"
        input { r#type: field.input_type(), value: draft().get(field).to_owned(), maxlength: 2000,
            oninput: move |e| { on_edit.call(()); draft.write().set(field, e.value()); } }
    } }
}
