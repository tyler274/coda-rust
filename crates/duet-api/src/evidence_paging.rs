//! Account, incarnation, revision and query-bound finite evidence pagination.
use crate::{ApiError, evidence::EvidenceSnapshot, identity::AccountContext};
use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sha2::{Digest, Sha256};

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Cursor {
    owner: uuid::Uuid,
    incarnation: uuid::Uuid,
    revision: i64,
    generation: i32,
    query: String,
    offset: usize,
}
/// Query identity includes the endpoint and every filter, excluding only cursor.
pub fn query_key(endpoint: &str, raw: Option<&str>) -> Result<String, ApiError> {
    let url = reqwest::Url::parse(&format!("http://localhost/?{}", raw.unwrap_or_default()))
        .map_err(|_| ApiError::Unprocessable("invalid evidence query"))?;
    let mut pairs: Vec<_> = url
        .query_pairs()
        .filter(|(k, _)| k != "cursor")
        .map(|(k, v)| (k.into_owned(), v.into_owned()))
        .collect();
    pairs.sort();
    let bytes = serde_json::to_vec(&(endpoint, pairs))
        .map_err(|_| ApiError::Unprocessable("invalid evidence query"))?;
    Ok(Sha256::digest(bytes)
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect())
}
/// Bind every continuation to the original account and complete query.
pub fn offset(
    value: Option<&str>,
    context: &AccountContext,
    snapshot: &EvidenceSnapshot,
    query: &str,
) -> Result<usize, ApiError> {
    let Some(value) = value else { return Ok(0) };
    if value.len() > 2048 {
        return Err(ApiError::Conflict("invalid evidence cursor"));
    }
    let cursor: Cursor = URL_SAFE_NO_PAD
        .decode(value)
        .ok()
        .and_then(|b| serde_json::from_slice(&b).ok())
        .ok_or(ApiError::Conflict("invalid evidence cursor"))?;
    if cursor.owner != context.account_id
        || cursor.incarnation != context.incarnation
        || cursor.revision != snapshot.cursor
        || cursor.generation != snapshot.health_records_generation
        || cursor.query != query
    {
        return Err(ApiError::Conflict("evidence snapshot or query changed"));
    }
    Ok(cursor.offset)
}
/// Encode a continuation only when there is more complete membership to read.
pub fn next(
    context: &AccountContext,
    snapshot: &EvidenceSnapshot,
    query: &str,
    offset: usize,
    total: usize,
) -> Option<String> {
    (offset < total).then(|| {
        URL_SAFE_NO_PAD.encode(
            serde_json::to_vec(&Cursor {
                owner: context.account_id,
                incarnation: context.incarnation,
                revision: snapshot.cursor,
                generation: snapshot.health_records_generation,
                query: query.into(),
                offset,
            })
            .expect("cursor serialization"),
        )
    })
}
/// Whole records only; never truncate a payload to fit the four-megabyte budget.
pub fn end<T: Serialize>(records: &[T], offset: usize) -> Result<usize, ApiError> {
    if offset > records.len() {
        return Err(ApiError::Conflict("invalid evidence cursor offset"));
    }
    let mut bytes = 0;
    let mut end = offset;
    for record in records.iter().skip(offset).take(50) {
        let size = serde_json::to_vec(record)
            .map_err(|_| ApiError::Storage("invalid evidence".into()))?
            .len();
        if bytes + size > 3_900_000 {
            break;
        }
        bytes += size;
        end += 1;
    }
    if end == offset && offset < records.len() {
        return Err(ApiError::Conflict(
            "complete evidence record exceeds page bound",
        ));
    }
    Ok(end)
}
/// Literal nonempty coded identities. Labels never establish membership.
pub fn codes(payload: &Value) -> Vec<Value> {
    if payload
        .pointer("/catalog/resolution")
        .and_then(Value::as_str)
        == Some("conflict")
    {
        return vec![];
    }
    payload
        .get("concept_codings")
        .or_else(|| payload.get("coding_identities"))
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(|c| {
            let system = c["system"].as_str()?.trim();
            let code = c["code"].as_str()?.trim();
            if system.is_empty() || code.is_empty() {
                return None;
            }
            let known = [
                "http://snomed.info/sct",
                "http://hl7.org/fhir/sid/icd-10-cm",
                "http://www.nlm.nih.gov/research/umls/rxnorm",
                "http://hl7.org/fhir/sid/cvx",
            ];
            let system = known
                .into_iter()
                .find(|s| s.eq_ignore_ascii_case(system))
                .unwrap_or(system);
            Some(serde_json::json!({"system":system,"code":code,"version":c["version"]}))
        })
        .collect()
}
/// Supported grouping families from the original clinical-library contract.
pub fn kind(payload: &Value) -> Option<&str> {
    let concept = payload["concept_id"].as_str().unwrap_or("");
    let kind = payload
        .pointer("/clinical_record/kind")
        .and_then(Value::as_str)
        .or_else(|| payload["clinical_kind"].as_str())
        .or_else(|| {
            if concept.starts_with("medication:") {
                Some("medication")
            } else {
                concept.strip_prefix("clinical:")
            }
        });
    kind.filter(|k| ["condition", "medication", "allergy", "immunization"].contains(k))
}
/// Transitive exact-code component, independent of list filters and display categories.
pub fn members(
    records: &[crate::evidence::ExactEvidence],
    anchor: uuid::Uuid,
    group: &str,
) -> Result<Vec<uuid::Uuid>, ApiError> {
    if !["condition", "medication", "allergy", "immunization"].contains(&group) {
        return Err(ApiError::Unprocessable("unsupported clinical group"));
    }
    let anchor_record = records
        .iter()
        .find(|r| r.address.canonical_record_id == anchor && kind(&r.payload) == Some(group))
        .ok_or(ApiError::NotFound)?;
    let mut ids = vec![anchor];
    let mut identities = codes(&anchor_record.payload);
    loop {
        let before = ids.len();
        for record in records {
            if kind(&record.payload) != Some(group)
                || ids.contains(&record.address.canonical_record_id)
            {
                continue;
            }
            let current = codes(&record.payload);
            if current.iter().any(|c| identities.contains(c)) {
                ids.push(record.address.canonical_record_id);
                for c in current {
                    if !identities.contains(&c) {
                        identities.push(c);
                    }
                }
            }
        }
        if ids.len() == before {
            break;
        }
    }
    Ok(ids)
}
