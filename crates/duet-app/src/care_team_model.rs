//! Mechanical care-team presentation and review-template input validation.
use chrono::NaiveDate;
use duet_client::care_team::*;
use duet_wire::models::{DirectoryAddress, release_recipient::Kind};
use uuid::Uuid;

pub(super) fn optional(value: &str) -> Option<String> {
    let value = value.trim();
    (!value.is_empty()).then(|| value.to_owned())
}

/// Accept only a dialable public number, never extensions, URI parameters or scripts.
pub(super) fn telephone(value: &str) -> Option<String> {
    let value = value.trim();
    if !value
        .chars()
        .all(|c| c.is_ascii_digit() || matches!(c, '+' | ' ' | '(' | ')' | '-' | '.'))
    {
        return None;
    }
    let normalized: String = value
        .chars()
        .filter(|c| c.is_ascii_digit() || *c == '+')
        .collect();
    let digits = normalized.strip_prefix('+').unwrap_or(&normalized);
    (digits.len() >= 7 && digits.len() <= 15 && digits.bytes().all(|c| c.is_ascii_digit()))
        .then(|| format!("tel:{normalized}"))
}

/// Percent encode every non-unreserved byte, keeping public addresses inside one query value.
pub(super) fn encode(value: &str) -> String {
    use std::fmt::Write;
    let mut result = String::new();
    for byte in value.bytes() {
        if byte.is_ascii_alphanumeric() || b"-._~".contains(&byte) {
            result.push(char::from(byte));
        } else {
            let _ = write!(result, "%{byte:02X}");
        }
    }
    result
}
pub(super) fn address(value: &DirectoryAddress) -> String {
    [
        Some(value.line1.as_str()),
        value.line2.as_deref(),
        Some(&value.city),
        Some(&value.state),
        Some(&value.postal_code),
    ]
    .into_iter()
    .flatten()
    .filter(|s| !s.trim().is_empty())
    .collect::<Vec<_>>()
    .join(", ")
}
pub(super) fn maps(value: &str) -> String {
    format!(
        "https://www.google.com/maps/search/?api=1&query={}",
        encode(value)
    )
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub(super) enum TextField {
    PatientName,
    BirthDate,
    PatientAddress,
    PatientPhone,
    RecipientName,
    RecipientAddress,
    RecipientPhone,
    RecipientFax,
    DateStart,
    DateEnd,
    Purpose,
    ExpirationDate,
    ExpirationEvent,
    TypedName,
}
impl TextField {
    pub(super) fn label(self) -> &'static str {
        match self {
            Self::PatientName => "Patient name",
            Self::BirthDate => "Date of birth (optional)",
            Self::PatientAddress => "Patient address (optional)",
            Self::PatientPhone => "Patient phone (optional)",
            Self::RecipientName => "Recipient name",
            Self::RecipientAddress => "Recipient address",
            Self::RecipientPhone => "Recipient phone",
            Self::RecipientFax => "Recipient fax",
            Self::DateStart => "Records from (optional)",
            Self::DateEnd => "Records through (optional)",
            Self::Purpose => "Purpose",
            Self::ExpirationDate => "Expiration date",
            Self::ExpirationEvent => "Expiration event",
            Self::TypedName => "Typed name (not an authenticated e-signature)",
        }
    }
    pub(super) fn input_type(self) -> &'static str {
        match self {
            Self::BirthDate | Self::DateStart | Self::DateEnd | Self::ExpirationDate => "date",
            _ => "text",
        }
    }
    fn index(self) -> usize {
        self as usize
    }
}

#[derive(Clone, Debug, PartialEq)]
pub(super) struct ReleaseDraft {
    pub discloser: String,
    /// "patient", "other", or the selected member UUID.
    pub recipient: String,
    pub text: [String; 14],
    pub categories: [bool; 5],
    pub sensitive: [bool; 4],
    pub initials: [String; 4],
    pub expiration_is_event: bool,
}
impl ReleaseDraft {
    pub fn new(display_name: String) -> Self {
        let mut value = Self {
            discloser: String::new(),
            recipient: "patient".into(),
            text: Default::default(),
            categories: [false; 5],
            sensitive: [false; 4],
            initials: Default::default(),
            expiration_is_event: false,
        };
        value.set(TextField::PatientName, display_name);
        value.set(
            TextField::Purpose,
            "At the request of the individual".into(),
        );
        value
    }
    pub fn get(&self, field: TextField) -> &str {
        &self.text[field.index()]
    }
    pub fn set(&mut self, field: TextField, value: String) {
        self.text[field.index()] = value;
    }
    pub fn request(
        &self,
        members: &[CareTeamMember],
    ) -> Result<ReleaseAuthorizationRequest, String> {
        let member = |id: &str| -> Result<Uuid, String> {
            let id = Uuid::parse_str(id).map_err(|_| "Choose a care-team member.")?;
            members
                .iter()
                .any(|m| m.id == id)
                .then_some(id)
                .ok_or_else(|| "This member is no longer on your care team. Choose again.".into())
        };
        let discloser = member(&self.discloser)?;
        if self.get(TextField::PatientName).trim().is_empty() {
            return Err("Enter the patient name for this template.".into());
        }
        let date = |field| -> Result<Option<NaiveDate>, String> {
            optional(self.get(field))
                .map(|s| {
                    NaiveDate::parse_from_str(&s, "%Y-%m-%d")
                        .map_err(|_| format!("Review {}.", field.label()))
                })
                .transpose()
        };
        let start = date(TextField::DateStart)?;
        let end = date(TextField::DateEnd)?;
        if start.is_some() != end.is_some() || start > end {
            return Err(
                "Enter both record dates in chronological order, or leave both blank.".into(),
            );
        }
        let (expires_on, expiration_event) = if self.expiration_is_event {
            (
                None,
                Some(
                    optional(self.get(TextField::ExpirationEvent))
                        .ok_or("Enter an expiration event.")?,
                ),
            )
        } else {
            (
                Some(date(TextField::ExpirationDate)?.ok_or("Choose an expiration date.")?),
                None,
            )
        };
        let recipient = match self.recipient.as_str() {
            "patient" => ReleaseRecipient::new(Kind::Patient),
            "other" => {
                if self.get(TextField::RecipientName).trim().is_empty() {
                    return Err("Enter the recipient name.".into());
                }
                ReleaseRecipient {
                    kind: Kind::Other,
                    member_id: None,
                    party: Some(Box::new(ReleaseParty::new(
                        self.get(TextField::RecipientName).trim().into(),
                        self.get(TextField::RecipientAddress).trim().into(),
                        self.get(TextField::RecipientPhone).trim().into(),
                        self.get(TextField::RecipientFax).trim().into(),
                    ))),
                }
            }
            id => ReleaseRecipient {
                kind: Kind::CareTeamMember,
                member_id: Some(member(id)?),
                party: None,
            },
        };
        let sensitive = |index: usize| -> Result<Option<Box<ReleaseSensitiveDisclosure>>, String> {
            if !self.sensitive[index] {
                return Ok(None);
            }
            let initials = optional(&self.initials[index])
                .ok_or("Each requested sensitive category needs separate initials and review.")?;
            if initials.chars().count() > 40 {
                return Err("Sensitive-category initials must be at most 40 characters.".into());
            }
            Ok(Some(Box::new(ReleaseSensitiveDisclosure {
                requested: true,
                initials,
            })))
        };
        if !self.categories.iter().any(|v| *v) && !self.sensitive.iter().any(|v| *v) {
            return Err("Choose the information to request.".into());
        }
        let scope = ReleaseScope {
            full_record: Some(self.categories[0]),
            visit_notes: Some(self.categories[1]),
            labs: Some(self.categories[2]),
            imaging: Some(self.categories[3]),
            medications: Some(self.categories[4]),
            date_start: start,
            date_end: end,
            mental_health: sensitive(0)?,
            substance_use: sensitive(1)?,
            hiv_sti: sensitive(2)?,
            genetic: sensitive(3)?,
        };
        Ok(ReleaseAuthorizationRequest {
            discloser_member_id: discloser,
            patient: Some(Box::new(ReleasePatient::new(
                self.get(TextField::PatientName).trim().into(),
                self.get(TextField::BirthDate).into(),
                self.get(TextField::PatientAddress).trim().into(),
                self.get(TextField::PatientPhone).trim().into(),
            ))),
            recipient: Box::new(recipient),
            scope: Box::new(scope),
            purpose: optional(self.get(TextField::Purpose)),
            expires_on,
            expiration_event,
            signed_name: optional(self.get(TextField::TypedName)),
        })
    }
}

/// A POST without an identified response cannot safely be repeated. PUT keeps its identity.
#[derive(Clone, Debug, Default, PartialEq)]
pub(super) struct ReleaseIdentity {
    pub id: Option<Uuid>,
    pub uncertain_create: bool,
}
impl ReleaseIdentity {
    pub fn may_save(&self) -> bool {
        !self.uncertain_create
    }
    pub fn saved(&mut self, id: Uuid) {
        self.id = Some(id);
        self.uncertain_create = false;
    }
    pub fn failed(&mut self, error: CareTeamError) {
        if self.id.is_none() && error == CareTeamError::Uncertain {
            self.uncertain_create = true;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    fn members() -> Vec<CareTeamMember> {
        vec![CareTeamMember {
            id: Uuid::from_u128(1),
            ..Default::default()
        }]
    }
    fn draft() -> ReleaseDraft {
        let mut d = ReleaseDraft::new("Ada".into());
        d.discloser = Uuid::from_u128(1).to_string();
        d.categories[2] = true;
        d.set(TextField::ExpirationDate, "2027-01-01".into());
        d
    }
    #[test]
    fn directory_links_cannot_inject_uri_parameters() {
        assert_eq!(
            telephone("+1 (212) 555-1212").as_deref(),
            Some("tel:+12125551212")
        );
        for bad in [
            "javascript:alert(1)",
            "2125551212;phone-context=evil",
            "2125551212 ext 4",
            "12+125551212",
            "++12125551212",
        ] {
            assert_eq!(telephone(bad), None);
        }
        assert_eq!(
            maps("a&query=b#c"),
            "https://www.google.com/maps/search/?api=1&query=a%26query%3Db%23c"
        );
    }
    #[test]
    fn only_display_name_is_prefilled() {
        let d = ReleaseDraft::new("Ada".into());
        assert_eq!(d.get(TextField::PatientName), "Ada");
        for field in [
            TextField::BirthDate,
            TextField::PatientAddress,
            TextField::PatientPhone,
            TextField::TypedName,
        ] {
            assert!(d.get(field).is_empty());
        }
        assert!(!d.categories.iter().any(|v| *v));
    }
    #[test]
    fn selected_member_must_still_exist() {
        assert!(draft().request(&[]).is_err());
        let mut d = draft();
        d.recipient = Uuid::from_u128(2).to_string();
        assert!(d.request(&members()).is_err());
    }
    #[test]
    fn range_and_expiration_are_explicit() {
        let mut d = draft();
        d.set(TextField::DateStart, "2026-01-01".into());
        assert!(d.request(&members()).is_err());
        d.set(TextField::DateEnd, "2025-01-01".into());
        assert!(d.request(&members()).is_err());
        d.set(TextField::DateEnd, "2026-02-01".into());
        d.expiration_is_event = true;
        assert!(d.request(&members()).is_err());
        d.set(
            TextField::ExpirationEvent,
            "End of this course of treatment".into(),
        );
        let r = d.request(&members()).unwrap();
        assert!(r.expires_on.is_none());
        assert!(r.expiration_event.is_some());
    }
    #[test]
    fn sensitive_initials_never_enable_a_category() {
        let mut d = draft();
        d.initials[1] = "AP".into();
        assert!(d.request(&members()).unwrap().scope.substance_use.is_none());
        d.sensitive[1] = true;
        d.initials[1].clear();
        assert!(d.request(&members()).is_err());
        d.initials[1] = "AP".into();
        assert!(
            d.request(&members())
                .unwrap()
                .scope
                .substance_use
                .unwrap()
                .requested
        );
    }
    #[test]
    fn recipient_kinds_preserve_server_snapshot_authority() {
        let mut d = draft();
        assert_eq!(d.request(&members()).unwrap().recipient.kind, Kind::Patient);
        d.recipient = d.discloser.clone();
        let r = d.request(&members()).unwrap();
        assert_eq!(r.recipient.member_id, Some(Uuid::from_u128(1)));
        assert!(r.recipient.party.is_none());
        d.recipient = "other".into();
        assert!(d.request(&members()).is_err());
        d.set(TextField::RecipientName, "Records office".into());
        assert_eq!(d.request(&members()).unwrap().recipient.kind, Kind::Other);
    }
    #[test]
    fn uncertain_post_blocks_duplicate_but_put_keeps_identity() {
        let mut state = ReleaseIdentity::default();
        state.failed(CareTeamError::Uncertain);
        assert!(!state.may_save());
        let mut known = ReleaseIdentity::default();
        known.saved(Uuid::from_u128(42));
        known.failed(CareTeamError::Uncertain);
        assert!(known.may_save());
        assert_eq!(known.id, Some(Uuid::from_u128(42)));
    }
}
