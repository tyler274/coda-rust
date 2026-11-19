//! CDC Morbidity and Mortality Weekly Report (MMWR) XML parser.

use crate::corpus::{
    DocumentIdentifier, IngestedDocument, IngestedUnit, LifecycleState, sha256_hex,
};
use crate::source::{ItemKey, OtherSource, Source};

use super::{SourceParseError, child_element, collect_normalized_text};

/// Parses a CDC MMWR XML report into a verified [`IngestedDocument`].
pub fn parse_article_xml(mmwr_id: &str, xml: &str) -> Result<IngestedDocument, SourceParseError> {
    let doc = roxmltree::Document::parse(xml).map_err(|cause| SourceParseError::Xml {
        collection: Source::Mmwr,
        cause,
    })?;

    let root = doc.root_element();
    let title = doc
        .descendants()
        .find(|n| {
            n.is_element() && matches!(n.tag_name().name(), "title" | "article-title" | "headline")
        })
        .map(collect_normalized_text)
        .filter(|s| !s.is_empty())
        .ok_or_else(|| SourceParseError::NoReadableContent {
            collection: Source::Mmwr,
            item_key: mmwr_id.to_owned(),
            reason: "MMWR document missing `<title>` or `<headline>`".into(),
        })?;

    let pub_date = child_element(root, "published")
        .or_else(|| {
            doc.descendants()
                .find(|n| n.is_element() && n.tag_name().name() == "pub-date")
        })
        .map(collect_normalized_text)
        .filter(|s| s.len() >= 10)
        .map(|s| s[..10].to_owned());

    let canonical_uri = format!("https://www.cdc.gov/mmwr/volumes/{mmwr_id}.htm");
    let mut units = Vec::new();
    let mut ordinal = 0_usize;

    for sec in doc
        .descendants()
        .filter(|n| n.is_element() && matches!(n.tag_name().name(), "section" | "sec"))
    {
        let heading = child_element(sec, "title")
            .or_else(|| child_element(sec, "heading"))
            .map(collect_normalized_text)
            .filter(|s| !s.is_empty());
        let body = collect_normalized_text(sec);
        let content = match &heading {
            Some(h) if body.starts_with(h.as_str()) => body[h.len()..].trim().to_owned(),
            _ => body,
        };
        if content.trim().is_empty() {
            continue;
        }
        let sec_id = sec
            .attribute("id")
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .map(str::to_owned)
            .unwrap_or_else(|| format!("sec-{ordinal}"));
        if let Ok(unit) = IngestedUnit::new(
            format!("{mmwr_id}-{sec_id}"),
            heading.clone().unwrap_or_else(|| sec_id.clone()),
            "report_section",
            ordinal,
            heading,
            content,
            format!("{canonical_uri}#{sec_id}"),
        ) {
            units.push(unit);
            ordinal += 1;
        }
    }

    if units.is_empty() {
        return Err(SourceParseError::NoReadableContent {
            collection: Source::Mmwr,
            item_key: mmwr_id.to_owned(),
            reason: "MMWR report contains no readable sections".into(),
        });
    }

    let doc_hash = sha256_hex(xml.as_bytes());
    let key = ItemKey::other(OtherSource::Mmwr, mmwr_id).map_err(|err| {
        SourceParseError::NoReadableContent {
            collection: Source::Mmwr,
            item_key: mmwr_id.to_owned(),
            reason: err.to_string(),
        }
    })?;

    Ok(IngestedDocument {
        key,
        native_document_id: mmwr_id.to_owned(),
        native_version_id: Some(doc_hash[..12].to_owned()),
        source_version_label: pub_date.clone(),
        title,
        document_type: "mmwr_report".into(),
        publication_date: pub_date.clone(),
        version_date: pub_date,
        lifecycle_state: LifecycleState::Current,
        source_status_code: None,
        native_destination_uri: canonical_uri.clone(),
        version_destination_uri: canonical_uri,
        rights_decision_id: format!(
            "mmwr-public-{}",
            sha256_hex(mmwr_id.as_bytes())[..12].to_owned()
        ),
        license_identifier: Some("public-domain-us-govt".into()),
        attribution_text: Some("CDC Morbidity and Mortality Weekly Report".into()),
        may_display_excerpt: true,
        identifiers: vec![DocumentIdentifier {
            scheme: "mmwr-id".into(),
            raw_value: mmwr_id.to_owned(),
            normalized_value: Some(mmwr_id.trim().to_owned()),
        }],
        relationships: Vec::new(),
        units,
    })
}
