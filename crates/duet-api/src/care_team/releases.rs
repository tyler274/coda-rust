//! Private editable review templates. A typed name does not authenticate a signature.
#![allow(missing_docs)]
use super::*;
use chrono::NaiveDate;
#[derive(Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReleasePatient {
    pub name: String,
    pub date_of_birth: String,
    pub address: String,
    pub phone: String,
}
#[derive(Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReleaseParty {
    pub name: String,
    pub npi: Option<String>,
    pub address: String,
    pub phone: String,
    pub fax: String,
}
#[derive(Clone, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum ReleaseRecipient {
    Patient,
    CareTeamMember { member_id: Uuid },
    Other { party: ReleaseParty },
}
#[derive(Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SensitiveDisclosure {
    pub requested: bool,
    pub initials: String,
}
#[derive(Clone, Default, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct ReleaseScope {
    pub full_record: bool,
    pub date_start: Option<NaiveDate>,
    pub date_end: Option<NaiveDate>,
    pub visit_notes: bool,
    pub labs: bool,
    pub imaging: bool,
    pub medications: bool,
    pub mental_health: SensitiveDisclosure,
    pub substance_use: SensitiveDisclosure,
    pub hiv_sti: SensitiveDisclosure,
    pub genetic: SensitiveDisclosure,
}
/// Full editable body; PUT replaces it and clears any prior typed signature.
#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReleaseInput {
    pub discloser_member_id: Uuid,
    pub patient: Option<ReleasePatient>,
    pub recipient: ReleaseRecipient,
    pub scope: ReleaseScope,
    pub purpose: Option<String>,
    pub expires_on: Option<NaiveDate>,
    pub expiration_event: Option<String>,
    pub signed_name: Option<String>,
}
#[derive(Serialize, Deserialize)]
pub struct ReleaseView {
    pub id: Uuid,
    pub discloser_member_id: Uuid,
    pub patient: ReleasePatient,
    pub discloser: ReleaseParty,
    pub recipient: serde_json::Value,
    pub scope: ReleaseScope,
    pub purpose: String,
    pub expires_on: Option<NaiveDate>,
    pub expiration_event: Option<String>,
    pub signed_name: Option<String>,
    pub signed_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
fn validate(body: &ReleaseInput) -> Result<(), ApiError> {
    if body.expires_on.is_some() && body.expiration_event.is_some() {
        return Err(ApiError::Unprocessable(
            "Choose an expiration date or event",
        ));
    }
    if body.scope.date_start.is_some() != body.scope.date_end.is_some()
        || body.scope.date_start > body.scope.date_end
    {
        return Err(ApiError::Unprocessable("Invalid record date range"));
    }
    for value in [&body.purpose, &body.expiration_event, &body.signed_name]
        .into_iter()
        .flatten()
    {
        bounded(value, 2000)?;
    }
    if body
        .expiration_event
        .as_ref()
        .is_some_and(|v| v.trim().is_empty())
        || body
            .signed_name
            .as_ref()
            .is_some_and(|v| v.trim().is_empty())
    {
        return Err(ApiError::Unprocessable(
            "Blank expiration or typed signature",
        ));
    }
    if let Some(p) = &body.patient {
        for v in [&p.name, &p.date_of_birth, &p.address, &p.phone] {
            bounded(v, 2000)?;
        }
    }
    if let ReleaseRecipient::Other { party } = &body.recipient {
        for v in [&party.name, &party.address, &party.phone, &party.fax] {
            bounded(v, 2000)?;
        }
    }
    for sensitive in [
        &body.scope.mental_health,
        &body.scope.substance_use,
        &body.scope.hiv_sti,
        &body.scope.genetic,
    ] {
        bounded(&sensitive.initials, 40)?;
        if sensitive.requested && sensitive.initials.trim().is_empty() {
            return Err(ApiError::Unprocessable(
                "Sensitive requests require initials and separate review",
            ));
        }
        if !sensitive.requested && !sensitive.initials.is_empty() {
            return Err(ApiError::Unprocessable(
                "Initials require an explicit sensitive category request",
            ));
        }
    }
    if body.signed_name.is_some() && (body.expires_on.is_none() && body.expiration_event.is_none())
    {
        return Err(ApiError::Unprocessable(
            "Enter expiration before typing a signature",
        ));
    }
    Ok(())
}
fn party(clinician: npi_registry::Clinician) -> ReleaseParty {
    let address = clinician
        .practice_address
        .map(|a| {
            format!(
                "{} {}\n{}, {} {}",
                a.line1,
                a.line2.unwrap_or_default(),
                a.city,
                a.state,
                a.postal_code
            )
        })
        .unwrap_or_default();
    ReleaseParty {
        name: clinician.name,
        npi: Some(clinician.npi.to_string()),
        address,
        phone: clinician.phone.unwrap_or_default(),
        fax: clinician.fax.unwrap_or_default(),
    }
}
impl CareTeamStore {
    /// Create or replace a review template. Server time records typed-name entry, not signature validity.
    pub async fn save_release(
        &self,
        c: &AccountContext,
        id: Option<Uuid>,
        mut body: ReleaseInput,
    ) -> Result<ReleaseView, ApiError> {
        validate(&body)?;
        let tx = self.begin(c).await?;
        let existing = if let Some(id) = id {
            Some(owned_release(&tx, c, id).await?)
        } else {
            None
        };
        let discloser =
            party(member_view(owned_member(&tx, c, body.discloser_member_id).await?)?.clinician);
        let recipient = match &body.recipient {
            ReleaseRecipient::Patient => serde_json::json!({"kind":"patient"}),
            ReleaseRecipient::CareTeamMember { member_id } => {
                let party = party(member_view(owned_member(&tx, c, *member_id).await?)?.clinician);
                serde_json::json!({"kind":"care_team_member", "member_id":member_id, "party":party})
            }
            ReleaseRecipient::Other { party } => serde_json::json!({"kind":"other", "party":party}),
        };
        if body.patient.is_none() {
            // Existing profile owns only display_name; do not invent DOB/address/phone.
            let row = entity::profile::Entity::find_by_id(c.account_id)
                .one(&tx)
                .await
                .map_err(database_error)?
                .ok_or(ApiError::AbsentAccount)?;
            let name = row.display_name;
            body.patient = Some(ReleasePatient {
                name: name.unwrap_or_default(),
                ..Default::default()
            });
        }
        let now = Utc::now();
        let row = entity::release::Model {
            id: existing.as_ref().map(|r| r.id).unwrap_or_else(Uuid::new_v4),
            user_id: c.account_id,
            discloser_member_id: body.discloser_member_id,
            patient: json(&body.patient)?,
            discloser: json(&discloser)?,
            recipient,
            scope: json(&body.scope)?,
            purpose: body
                .purpose
                .filter(|s| !s.trim().is_empty())
                .unwrap_or_else(|| "At the request of the individual".into()),
            expires_on: body.expires_on,
            expiration_event: body.expiration_event,
            signed_at: body.signed_name.as_ref().map(|_| now),
            signed_name: body.signed_name,
            created_at: existing.as_ref().map(|r| r.created_at).unwrap_or(now),
            updated_at: now,
        };
        let active = row.into_active_model().reset_all();
        let result = if existing.is_some() {
            active.update(&tx).await
        } else {
            active.insert(&tx).await
        }
        .map_err(database_error)?;
        tx.commit().await.map_err(database_error)?;
        release_view(result)
    }
    /// Render while holding the same account lock as deletion; output is never persisted or publicly linked.
    pub async fn render_release(
        &self,
        c: &AccountContext,
        id: Uuid,
    ) -> Result<duet_page::PageLayout, ApiError> {
        let tx = self.begin(c).await?;
        let view = release_view(owned_release(&tx, c, id).await?)?;
        let layout = duet_page::layout_release(project(view)?);
        tx.commit().await.map_err(database_error)?;
        Ok(layout)
    }
}
async fn owned_release(
    tx: &DatabaseTransaction,
    c: &AccountContext,
    id: Uuid,
) -> Result<entity::release::Model, ApiError> {
    entity::release::Entity::find_by_id(id)
        .filter(entity::release::Column::UserId.eq(c.account_id))
        .one(tx)
        .await
        .map_err(database_error)?
        .ok_or(ApiError::NotFound)
}
fn decode<T: serde::de::DeserializeOwned>(v: serde_json::Value) -> Result<T, ApiError> {
    serde_json::from_value(v)
        .map_err(|_| ApiError::Unavailable("Stored release template unavailable"))
}
fn release_view(r: entity::release::Model) -> Result<ReleaseView, ApiError> {
    Ok(ReleaseView {
        id: r.id,
        discloser_member_id: r.discloser_member_id,
        patient: decode(r.patient)?,
        discloser: decode(r.discloser)?,
        recipient: r.recipient,
        scope: decode(r.scope)?,
        purpose: r.purpose,
        expires_on: r.expires_on,
        expiration_event: r.expiration_event,
        signed_name: r.signed_name,
        signed_at: r.signed_at,
        created_at: r.created_at,
        updated_at: r.updated_at,
    })
}
fn page_party(p: ReleaseParty) -> duet_page::ReleaseParty {
    duet_page::ReleaseParty {
        name: p.name,
        npi: p.npi,
        address: p.address,
        phone: p.phone,
        fax: p.fax,
    }
}
fn sensitive(s: SensitiveDisclosure) -> duet_page::SensitiveDisclosure {
    duet_page::SensitiveDisclosure {
        requested: s.requested,
        initials: s.initials,
    }
}
fn project(v: ReleaseView) -> Result<duet_page::ReleaseAuthorization, ApiError> {
    let recipient = match v.recipient["kind"].as_str() {
        Some("patient") => duet_page::ReleaseRecipient::Patient,
        Some("care_team_member") => duet_page::ReleaseRecipient::CareTeamMember(page_party(
            decode(v.recipient["party"].clone())?,
        )),
        Some("other") => {
            duet_page::ReleaseRecipient::Other(page_party(decode(v.recipient["party"].clone())?))
        }
        _ => return Err(ApiError::Unavailable("Stored recipient unavailable")),
    };
    let s = v.scope;
    Ok(duet_page::ReleaseAuthorization {
        patient: duet_page::ReleasePatient {
            name: v.patient.name,
            date_of_birth: v.patient.date_of_birth,
            address: v.patient.address,
            phone: v.patient.phone,
        },
        discloser: page_party(v.discloser),
        recipient,
        scope: duet_page::ReleaseScope {
            full_record: s.full_record,
            date_range: s
                .date_start
                .zip(s.date_end)
                .map(|(a, b)| (a.to_string(), b.to_string())),
            visit_notes: s.visit_notes,
            labs: s.labs,
            imaging: s.imaging,
            medications: s.medications,
            mental_health: sensitive(s.mental_health),
            substance_use: sensitive(s.substance_use),
            hiv_sti: sensitive(s.hiv_sti),
            genetic: sensitive(s.genetic),
        },
        purpose: Some(v.purpose),
        expiration: v
            .expires_on
            .map(|d| duet_page::ReleaseExpiration::Date(d.to_string()))
            .or_else(|| v.expiration_event.map(duet_page::ReleaseExpiration::Event)),
        signed_name: v.signed_name,
        signed_date: v.signed_at.map(|t| t.date_naive().to_string()),
        representative_authority: None,
    })
}
#[cfg(test)]
mod tests {
    use super::*;
    fn body() -> ReleaseInput {
        ReleaseInput {
            discloser_member_id: Uuid::new_v4(),
            patient: None,
            recipient: ReleaseRecipient::Patient,
            scope: ReleaseScope::default(),
            purpose: None,
            expires_on: None,
            expiration_event: None,
            signed_name: None,
        }
    }
    #[test]
    fn sensitive_requires_explicit_initials() {
        let mut b = body();
        b.scope.substance_use.requested = true;
        assert!(validate(&b).is_err());
        b.scope.substance_use.initials = "AB".into();
        assert!(validate(&b).is_ok());
    }
    #[test]
    fn expiration_and_range_are_unambiguous() {
        let mut b = body();
        b.signed_name = Some("Patient".into());
        assert!(validate(&b).is_err());
        b.expiration_event = Some("End of treatment".into());
        assert!(validate(&b).is_ok());
        b.expires_on = Some(NaiveDate::from_ymd_opt(2030, 1, 1).unwrap());
        assert!(validate(&b).is_err());
    }
    #[test]
    fn generated_request_matches_backend() {
        use duet_wire::models as wire;
        let recipient = wire::ReleaseRecipient::new(wire::release_recipient::Kind::Patient);
        let request = wire::ReleaseAuthorizationRequest::new(
            Uuid::new_v4(),
            recipient,
            wire::ReleaseScope::new(),
        );
        let input: ReleaseInput =
            serde_json::from_value(serde_json::to_value(request).unwrap()).unwrap();
        assert!(validate(&input).is_ok());
    }
    #[test]
    fn unknown_sensitive_categories_are_rejected() {
        assert!(
            serde_json::from_value::<ReleaseScope>(serde_json::json!({"psychotherapy_notes":true}))
                .is_err()
        );
    }
}
