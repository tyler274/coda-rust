//! USPSTF recommendation JSON parser.

use serde::Deserialize;

use crate::corpus::{
    DocumentIdentifier, IngestedDocument, IngestedUnit, LifecycleState, sha256_hex,
};
use crate::source::{ItemKey, OtherSource, Source};

use super::SourceParseError;

#[derive(Deserialize)]
struct RawUspstfRecommendation {
    title: String,
    #[serde(default)]
    grade: Option<String>,
    #[serde(default)]
    published_date: Option<String>,
    #[serde(default)]
    url: Option<String>,
    sections: Vec<RawUspstfSection>,
}

#[derive(Deserialize)]
struct RawUspstfSection {
    #[serde(default)]
    id: Option<String>,
    #[serde(default)]
    heading: Option<String>,
    text: String,
}

/// Parses a USPSTF JSON recommendation statement into a verified [`IngestedDocument`].
pub fn parse_recommendation_json(
    recommendation_id: &str,
    json: &str,
) -> Result<IngestedDocument, SourceParseError> {
    let raw: RawUspstfRecommendation =
        serde_json::from_str(json).map_err(|cause| SourceParseError::Json {
            collection: Source::Uspstf,
            cause,
        })?;

    if raw.title.trim().is_empty() {
        return Err(SourceParseError::NoReadableContent {
            collection: Source::Uspstf,
            item_key: recommendation_id.to_owned(),
            reason: "USPSTF recommendation missing title".into(),
        });
    }

    let canonical_uri = raw.url.unwrap_or_else(|| {
        format!("https://www.uspreventiveservicestaskforce.org/uspstf/recommendation/{recommendation_id}")
    });
    let mut units = Vec::new();
    for (ordinal, sec) in raw.sections.into_iter().enumerate() {
        let content = sec.text.split_whitespace().collect::<Vec<_>>().join(" ");
        if content.is_empty() {
            continue;
        }
        let sec_id = sec
            .id
            .filter(|s| !s.trim().is_empty())
            .unwrap_or_else(|| format!("rec-{ordinal}"));
        let label = match (sec.heading, raw.grade.as_deref()) {
            (Some(h), Some(g)) if ordinal == 0 => Some(format!("{h} (Grade {g})")),
            (Some(h), _) => Some(h),
            (None, Some(g)) if ordinal == 0 => Some(format!("Recommendation (Grade {g})")),
            (None, _) => None,
        };
        if let Ok(unit) = IngestedUnit::new(
            format!("{recommendation_id}-{sec_id}"),
            label.clone().unwrap_or_else(|| sec_id.clone()),
            "recommendation",
            units.len(),
            label,
            content,
            format!("{canonical_uri}#{sec_id}"),
        ) {
            units.push(unit);
        }
    }

    if units.is_empty() {
        return Err(SourceParseError::NoReadableContent {
            collection: Source::Uspstf,
            item_key: recommendation_id.to_owned(),
            reason: "USPSTF recommendation has zero non-empty sections".into(),
        });
    }

    let doc_hash = sha256_hex(json.as_bytes());
    let key = ItemKey::other(OtherSource::Uspstf, recommendation_id).map_err(|err| {
        SourceParseError::NoReadableContent {
            collection: Source::Uspstf,
            item_key: recommendation_id.to_owned(),
            reason: err.to_string(),
        }
    })?;

    Ok(IngestedDocument {
        key,
        native_document_id: recommendation_id.to_owned(),
        native_version_id: Some(doc_hash[..12].to_owned()),
        source_version_label: raw.grade.map(|g| format!("Grade {g}")),
        title: raw.title.trim().to_owned(),
        document_type: "preventive_guideline".into(),
        publication_date: raw.published_date.clone(),
        version_date: raw.published_date,
        lifecycle_state: LifecycleState::Current,
        source_status_code: None,
        native_destination_uri: canonical_uri.clone(),
        version_destination_uri: canonical_uri,
        rights_decision_id: format!(
            "uspstf-public-{}",
            sha256_hex(recommendation_id.as_bytes())[..12].to_owned()
        ),
        license_identifier: Some("public-domain-us-govt".into()),
        attribution_text: Some("U.S. Preventive Services Task Force".into()),
        may_display_excerpt: true,
        identifiers: vec![DocumentIdentifier {
            scheme: "uspstf-id".into(),
            raw_value: recommendation_id.to_owned(),
            normalized_value: Some(recommendation_id.trim().to_owned()),
        }],
        relationships: Vec::new(),
        units,
    })
}
