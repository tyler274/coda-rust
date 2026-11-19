//! DailyMed Structured Product Labeling (SPL) XML parser.

use crate::corpus::{
    DocumentIdentifier, IngestedDocument, IngestedUnit, LifecycleState, sha256_hex,
};
use crate::source::{ItemKey, OtherSource, Source};

use super::{SourceParseError, child_element, collect_normalized_text};

/// Parses a DailyMed SPL XML document into a verified [`IngestedDocument`].
pub fn parse_spl_xml(setid: &str, xml: &str) -> Result<IngestedDocument, SourceParseError> {
    let doc = roxmltree::Document::parse(xml).map_err(|cause| SourceParseError::Xml {
        collection: Source::DailyMed,
        cause,
    })?;

    let root = doc.root_element();
    let title = child_element(root, "title")
        .map(collect_normalized_text)
        .filter(|s| !s.is_empty())
        .or_else(|| {
            doc.descendants()
                .find(|n| n.is_element() && n.tag_name().name() == "name")
                .map(collect_normalized_text)
                .filter(|s| !s.is_empty())
        })
        .ok_or_else(|| SourceParseError::NoReadableContent {
            collection: Source::DailyMed,
            item_key: setid.to_owned(),
            reason: "SPL document missing `<title>` or `<name>`".into(),
        })?;

    let effective_date = child_element(root, "effectiveTime")
        .and_then(|n| n.attribute("value"))
        .and_then(format_spl_date);
    let version_number = child_element(root, "versionNumber")
        .and_then(|n| n.attribute("value"))
        .map(str::to_owned);

    let canonical_uri = format!("https://dailymed.nlm.nih.gov/dailymed/drugInfo.cfm?setid={setid}");
    let mut units = Vec::new();
    let mut ordinal = 0_usize;

    for sec in doc
        .descendants()
        .filter(|n| n.is_element() && n.tag_name().name() == "section")
    {
        if sec.children().any(|c| {
            c.is_element()
                && c.tag_name().name() == "component"
                && child_element(c, "section").is_some()
        }) {
            continue;
        }
        let heading = child_element(sec, "title")
            .map(collect_normalized_text)
            .filter(|s| !s.is_empty());
        let text_body = child_element(sec, "text")
            .map(collect_normalized_text)
            .filter(|s| !s.is_empty());
        let Some(content) = text_body else {
            continue;
        };
        let loinc_code = child_element(sec, "code")
            .and_then(|c| c.attribute("code"))
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .map(str::to_owned)
            .unwrap_or_else(|| format!("sec-{ordinal}"));
        let locator = heading.clone().unwrap_or_else(|| loinc_code.clone());
        if let Ok(unit) = IngestedUnit::new(
            format!("{setid}-{loinc_code}-{ordinal}"),
            locator,
            "label_section",
            ordinal,
            heading,
            content,
            format!("{canonical_uri}#{loinc_code}"),
        ) {
            units.push(unit);
            ordinal += 1;
        }
    }

    if units.is_empty() {
        return Err(SourceParseError::NoReadableContent {
            collection: Source::DailyMed,
            item_key: setid.to_owned(),
            reason: "SPL document contains no readable `<section>/<text>` blocks".into(),
        });
    }

    let doc_hash = sha256_hex(xml.as_bytes());
    let key = ItemKey::other(OtherSource::DailyMed, setid).map_err(|err| {
        SourceParseError::NoReadableContent {
            collection: Source::DailyMed,
            item_key: setid.to_owned(),
            reason: err.to_string(),
        }
    })?;

    Ok(IngestedDocument {
        key,
        native_document_id: setid.to_owned(),
        native_version_id: version_number
            .clone()
            .or_else(|| Some(doc_hash[..12].to_owned())),
        source_version_label: version_number.map(|v| format!("Version {v}")),
        title,
        document_type: "drug_label".into(),
        publication_date: effective_date.clone(),
        version_date: effective_date,
        lifecycle_state: LifecycleState::Current,
        source_status_code: None,
        native_destination_uri: canonical_uri.clone(),
        version_destination_uri: canonical_uri,
        rights_decision_id: format!(
            "dailymed-public-{}",
            sha256_hex(setid.as_bytes())[..12].to_owned()
        ),
        license_identifier: Some("public-domain-us-govt".into()),
        attribution_text: Some("DailyMed (National Library of Medicine)".into()),
        may_display_excerpt: true,
        identifiers: vec![DocumentIdentifier {
            scheme: "spl-set-id".into(),
            raw_value: setid.to_owned(),
            normalized_value: Some(setid.trim().to_ascii_lowercase()),
        }],
        relationships: Vec::new(),
        units,
    })
}

fn format_spl_date(raw: &str) -> Option<String> {
    let trimmed = raw.trim();
    if trimmed.len() >= 8 && trimmed[..8].bytes().all(|b| b.is_ascii_digit()) {
        Some(format!(
            "{}-{}-{}",
            &trimmed[..4],
            &trimmed[4..6],
            &trimmed[6..8]
        ))
    } else if trimmed.len() >= 10 && trimmed.as_bytes()[4] == b'-' && trimmed.as_bytes()[7] == b'-'
    {
        Some(trimmed[..10].to_owned())
    } else {
        None
    }
}
