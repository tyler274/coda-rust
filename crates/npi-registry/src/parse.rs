//! NPPES API v2.1 JSON to [`Clinician`].
//!
//! Pure functions with no I/O, so products can parse recorded responses in
//! their own tests and fakes.

use serde::Deserialize;
use serde_json::Value;

use crate::{
    Address, Clinician, ClinicianKind, Endpoint, Npi, PersonName, RegistryError, RegistryMessage,
    Taxonomy,
};

#[derive(Deserialize)]
struct RawResponse {
    #[serde(default)]
    results: Option<Vec<Value>>,
    #[serde(rename = "Errors", default)]
    errors: Option<Vec<RawMessage>>,
}

#[derive(Deserialize)]
struct RawMessage {
    #[serde(default)]
    description: Option<String>,
    #[serde(default)]
    field: Option<String>,
    #[serde(default)]
    number: Option<Value>,
}

#[derive(Deserialize)]
struct RawRecord {
    number: Value,
    enumeration_type: String,
    #[serde(default)]
    basic: Option<RawBasic>,
    #[serde(default)]
    addresses: Option<Vec<RawAddress>>,
    #[serde(default)]
    taxonomies: Option<Vec<RawTaxonomy>>,
    #[serde(default)]
    endpoints: Option<Vec<RawEndpoint>>,
}

#[derive(Deserialize, Default)]
struct RawBasic {
    #[serde(default)]
    name_prefix: Option<String>,
    #[serde(default)]
    first_name: Option<String>,
    #[serde(default)]
    middle_name: Option<String>,
    #[serde(default)]
    last_name: Option<String>,
    #[serde(default)]
    name_suffix: Option<String>,
    #[serde(default)]
    credential: Option<String>,
    #[serde(default)]
    organization_name: Option<String>,
    #[serde(default)]
    last_updated: Option<String>,
}

#[derive(Deserialize)]
struct RawAddress {
    #[serde(default)]
    address_purpose: Option<String>,
    #[serde(default)]
    address_1: Option<String>,
    #[serde(default)]
    address_2: Option<String>,
    #[serde(default)]
    city: Option<String>,
    #[serde(default)]
    state: Option<String>,
    #[serde(default)]
    postal_code: Option<String>,
    #[serde(default)]
    country_code: Option<String>,
    #[serde(default)]
    telephone_number: Option<String>,
    #[serde(default)]
    fax_number: Option<String>,
}

#[derive(Deserialize)]
struct RawTaxonomy {
    #[serde(default)]
    code: Option<String>,
    #[serde(default)]
    desc: Option<String>,
    #[serde(default)]
    primary: Option<bool>,
    #[serde(default)]
    state: Option<String>,
    #[serde(default)]
    license: Option<String>,
}

#[derive(Deserialize)]
struct RawEndpoint {
    #[serde(rename = "endpointType", default)]
    endpoint_type: Option<String>,
    #[serde(default)]
    endpoint: Option<String>,
    #[serde(rename = "endpointTypeDescription", default)]
    endpoint_type_description: Option<String>,
    #[serde(rename = "useDescription", default)]
    use_description: Option<String>,
}

/// Parses a whole NPPES API v2.1 response body.
///
/// A body carrying `Errors` becomes [`RegistryError::Rejected`]. A body with
/// no results is an empty list. Any malformed record fails the whole parse.
pub fn parse_response(body: &str) -> Result<Vec<Clinician>, RegistryError> {
    let raw: RawResponse =
        serde_json::from_str(body).map_err(|error| RegistryError::Decode(error.to_string()))?;
    if let Some(errors) = raw.errors.filter(|errors| !errors.is_empty()) {
        return Err(RegistryError::Rejected(
            errors.into_iter().map(message).collect(),
        ));
    }
    raw.results
        .unwrap_or_default()
        .iter()
        .map(parse_record)
        .collect()
}

/// Parses one element of the NPPES `results` array.
pub fn parse_record(record: &Value) -> Result<Clinician, RegistryError> {
    let raw = RawRecord::deserialize(record).map_err(|error| decode(error.to_string()))?;
    let number = match &raw.number {
        Value::String(text) => text.clone(),
        Value::Number(number) => number.to_string(),
        _ => return Err(decode("NPPES record number is not a string")),
    };
    let npi = Npi::parse(&number).map_err(|error| decode(error.to_string()))?;
    let kind = ClinicianKind::from_enumeration_type(&raw.enumeration_type)
        .ok_or_else(|| decode("unknown NPPES enumeration_type"))?;
    let basic = raw.basic.unwrap_or_default();

    let (name, person, credential) = match kind {
        ClinicianKind::Individual => {
            let person = PersonName {
                prefix: present(basic.name_prefix),
                first: present(basic.first_name).unwrap_or_default(),
                middle: present(basic.middle_name),
                last: present(basic.last_name).unwrap_or_default(),
                suffix: present(basic.name_suffix),
            };
            if person.last.is_empty() {
                return Err(decode("NPPES individual record has no last name"));
            }
            let name = [
                Some(person.first.as_str()),
                person.middle.as_deref(),
                Some(person.last.as_str()),
                person.suffix.as_deref(),
            ]
            .into_iter()
            .flatten()
            .filter(|part| !part.is_empty())
            .collect::<Vec<_>>()
            .join(" ");
            (name, Some(person), present(basic.credential))
        }
        ClinicianKind::Organization => {
            let name = present(basic.organization_name)
                .ok_or_else(|| decode("NPPES organization record has no name"))?;
            (name, None, None)
        }
    };

    let taxonomies: Vec<Taxonomy> = raw
        .taxonomies
        .unwrap_or_default()
        .into_iter()
        .filter_map(|taxonomy| {
            Some(Taxonomy {
                code: present(taxonomy.code)?,
                description: present(taxonomy.desc).unwrap_or_default(),
                primary: taxonomy.primary.unwrap_or(false),
                state: present(taxonomy.state),
                license: present(taxonomy.license),
            })
        })
        .collect();
    let primary_taxonomy = taxonomies
        .iter()
        .find(|taxonomy| taxonomy.primary)
        .or_else(|| taxonomies.first())
        .cloned();

    let mut practice_address = None;
    let mut mailing_address = None;
    for raw_address in raw.addresses.unwrap_or_default() {
        let purpose = raw_address.address_purpose.clone().unwrap_or_default();
        let Some(address) = address(raw_address) else {
            continue;
        };
        if purpose.eq_ignore_ascii_case("LOCATION") && practice_address.is_none() {
            practice_address = Some(address);
        } else if purpose.eq_ignore_ascii_case("MAILING") && mailing_address.is_none() {
            mailing_address = Some(address);
        }
    }
    let phone = practice_address
        .as_ref()
        .and_then(|address| address.phone.clone());
    let fax = practice_address
        .as_ref()
        .and_then(|address| address.fax.clone());

    let endpoints = raw
        .endpoints
        .unwrap_or_default()
        .into_iter()
        .filter_map(|endpoint| {
            Some(Endpoint {
                kind: present(endpoint.endpoint_type)?,
                address: present(endpoint.endpoint)?,
                kind_description: present(endpoint.endpoint_type_description),
                use_description: present(endpoint.use_description),
            })
        })
        .collect();

    Ok(Clinician {
        npi,
        kind,
        name,
        person,
        credential,
        primary_taxonomy,
        taxonomies,
        practice_address,
        mailing_address,
        phone,
        fax,
        endpoints,
        last_updated: present(basic.last_updated),
    })
}

fn address(raw: RawAddress) -> Option<Address> {
    Some(Address {
        line1: present(raw.address_1)?,
        line2: present(raw.address_2),
        city: present(raw.city).unwrap_or_default(),
        state: present(raw.state).unwrap_or_default(),
        postal_code: present(raw.postal_code)
            .map(|code| zip(&code))
            .unwrap_or_default(),
        country_code: present(raw.country_code),
        phone: present(raw.telephone_number),
        fax: present(raw.fax_number),
    })
}

/// NPPES publishes ZIP+4 as nine bare digits; show it as `12345-6789`.
fn zip(code: &str) -> String {
    if code.len() == 9 && code.bytes().all(|byte| byte.is_ascii_digit()) {
        format!("{}-{}", &code[..5], &code[5..])
    } else {
        code.to_owned()
    }
}

/// NPPES uses empty strings and `--` for absent values.
fn present(value: Option<String>) -> Option<String> {
    value
        .map(|text| text.trim().to_owned())
        .filter(|text| !text.is_empty() && text != "--")
}

fn message(raw: RawMessage) -> RegistryMessage {
    RegistryMessage {
        description: raw.description.unwrap_or_default(),
        field: present(raw.field),
        code: raw.number.and_then(|number| match number {
            Value::String(text) => present(Some(text)),
            Value::Number(number) => Some(number.to_string()),
            _ => None,
        }),
    }
}

fn decode(message: impl Into<String>) -> RegistryError {
    RegistryError::Decode(message.into())
}
