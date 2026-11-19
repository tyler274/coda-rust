//! PubMed Central (PMC) open-access JATS XML parser.

use crate::corpus::{
    DocumentIdentifier, IngestedDocument, IngestedUnit, LifecycleState, sha256_hex,
};
use crate::source::{ItemKey, OtherSource, Source};

use super::{SourceParseError, child_element, collect_normalized_text};

/// Parses a PMC JATS XML article into a verified [`IngestedDocument`].
///
/// Enforces an explicit `CC0` / `CC BY` open-access allowlist (rejecting missing
/// licenses, `-NC` / `-ND` / restrictive clauses, preprints, and author manuscripts)
/// per the clinical evidence collection policy.
pub fn parse_jats_xml(pmcid: &str, xml: &str) -> Result<IngestedDocument, SourceParseError> {
    let doc = roxmltree::Document::parse(xml).map_err(|cause| SourceParseError::Xml {
        collection: Source::Pmc,
        cause,
    })?;

    if is_excluded_article_type(&doc) {
        return Err(SourceParseError::RightsRefused {
            collection: Source::Pmc,
            item_key: pmcid.to_owned(),
            reason:
                "preprint or author-manuscript articles are excluded from PMC open-access excerpts"
                    .into(),
        });
    }

    let title = doc
        .descendants()
        .find(|n| n.is_element() && n.tag_name().name() == "article-title")
        .map(collect_normalized_text)
        .filter(|s| !s.is_empty())
        .ok_or_else(|| SourceParseError::NoReadableContent {
            collection: Source::Pmc,
            item_key: pmcid.to_owned(),
            reason: "missing `<article-title>`".into(),
        })?;

    let mut identifiers = vec![DocumentIdentifier {
        scheme: "pmcid".into(),
        raw_value: pmcid.to_owned(),
        normalized_value: Some(pmcid.trim().to_ascii_uppercase()),
    }];
    for id_node in doc
        .descendants()
        .filter(|n| n.is_element() && n.tag_name().name() == "article-id")
    {
        let pub_id_type = id_node.attribute("pub-id-type").unwrap_or_default().trim();
        let value = collect_normalized_text(id_node);
        if value.is_empty() {
            continue;
        }
        if matches!(pub_id_type, "pmid" | "doi" | "pmc") {
            let scheme = if pub_id_type == "pmc" {
                "pmcid"
            } else {
                pub_id_type
            };
            if !identifiers.iter().any(|existing| existing.scheme == scheme) {
                identifiers.push(DocumentIdentifier {
                    scheme: scheme.to_owned(),
                    raw_value: value.clone(),
                    normalized_value: Some(value),
                });
            }
        }
    }

    let license_node = doc
        .descendants()
        .find(|n| n.is_element() && n.tag_name().name() == "license")
        .ok_or_else(|| SourceParseError::RightsRefused {
            collection: Source::Pmc,
            item_key: pmcid.to_owned(),
            reason: "PMC article missing required `<license>` metadata".into(),
        })?;
    let license_attr = license_node
        .attribute("license-type")
        .or_else(|| license_node.attribute("xlink:href"))
        .map(|s| s.trim().to_owned())
        .filter(|s| !s.is_empty());
    let license_body = collect_normalized_text(license_node);
    let canonical_license =
        verify_pmc_open_access_license(pmcid, license_attr.as_deref(), &license_body)?;
    let attribution_text = (!license_body.is_empty()).then_some(license_body);

    let publication_date = extract_pmc_pub_date(&doc);
    let canonical_uri = format!("https://pmc.ncbi.nlm.nih.gov/articles/{pmcid}/");
    let mut units = Vec::new();
    let mut ordinal = 0_usize;

    if let Some(abs) = doc
        .descendants()
        .find(|n| n.is_element() && n.tag_name().name() == "abstract")
    {
        let abs_text = collect_normalized_text(abs);
        if !abs_text.is_empty() {
            units.push(IngestedUnit::new(
                format!("{pmcid}-abstract"),
                "abstract",
                "abstract",
                ordinal,
                Some("Abstract".into()),
                abs_text,
                format!("{canonical_uri}#abstract"),
            )?);
            ordinal += 1;
        }
    }

    for sec in doc
        .descendants()
        .filter(|n| n.is_element() && n.tag_name().name() == "sec")
    {
        // Avoid duplicating nested child `<sec>` text if parent `<sec>` contains sub-sections.
        if sec
            .children()
            .any(|c| c.is_element() && c.tag_name().name() == "sec")
        {
            continue;
        }
        let heading = child_element(sec, "title")
            .map(collect_normalized_text)
            .filter(|s| !s.is_empty());
        let mut paragraphs = Vec::new();
        for p in sec
            .children()
            .filter(|c| c.is_element() && c.tag_name().name() == "p")
        {
            let text = collect_normalized_text(p);
            if !text.is_empty() {
                paragraphs.push(text);
            }
        }
        if paragraphs.is_empty() {
            let fallback = collect_normalized_text(sec);
            if !fallback.is_empty() {
                paragraphs.push(fallback);
            }
        }
        let content = paragraphs.join("\n\n");
        if content.trim().is_empty() {
            continue;
        }
        let unit_id_attr = sec
            .attribute("id")
            .map(str::trim)
            .filter(|s| !s.is_empty())
            .map(str::to_owned)
            .unwrap_or_else(|| format!("sec-{ordinal}"));
        if let Ok(unit) = IngestedUnit::new(
            format!("{pmcid}-{unit_id_attr}"),
            heading.clone().unwrap_or_else(|| unit_id_attr.clone()),
            "section",
            ordinal,
            heading,
            content,
            format!("{canonical_uri}#{unit_id_attr}"),
        ) {
            units.push(unit);
            ordinal += 1;
        }
    }

    if units.is_empty() {
        return Err(SourceParseError::NoReadableContent {
            collection: Source::Pmc,
            item_key: pmcid.to_owned(),
            reason: "no readable abstract or body sections found".into(),
        });
    }

    let doc_hash = sha256_hex(xml.as_bytes());
    let key = ItemKey::other(OtherSource::Pmc, pmcid).map_err(|err| {
        SourceParseError::NoReadableContent {
            collection: Source::Pmc,
            item_key: pmcid.to_owned(),
            reason: err.to_string(),
        }
    })?;

    Ok(IngestedDocument {
        key,
        native_document_id: pmcid.to_owned(),
        native_version_id: Some(doc_hash[..12].to_owned()),
        source_version_label: publication_date.clone(),
        title,
        document_type: "journal_article".into(),
        publication_date: publication_date.clone(),
        version_date: publication_date,
        lifecycle_state: LifecycleState::Current,
        source_status_code: None,
        native_destination_uri: canonical_uri.clone(),
        version_destination_uri: canonical_uri,
        rights_decision_id: format!("pmc-oa-{}", sha256_hex(pmcid.as_bytes())[..12].to_owned()),
        license_identifier: Some(canonical_license),
        attribution_text: attribution_text.or_else(|| Some("PubMed Central Open Access".into())),
        may_display_excerpt: true,
        identifiers,
        relationships: Vec::new(),
        units,
    })
}

fn extract_pmc_pub_date(doc: &roxmltree::Document<'_>) -> Option<String> {
    let pub_date = doc
        .descendants()
        .find(|n| n.is_element() && n.tag_name().name() == "pub-date")?;
    let year = child_element(pub_date, "year").map(collect_normalized_text)?;
    let month = child_element(pub_date, "month")
        .map(collect_normalized_text)
        .unwrap_or_else(|| "01".into());
    let day = child_element(pub_date, "day")
        .map(collect_normalized_text)
        .unwrap_or_else(|| "01".into());
    let m: u32 = month.parse().unwrap_or(1).clamp(1, 12);
    let d: u32 = day.parse().unwrap_or(1).clamp(1, 31);
    Some(format!("{year}-{m:02}-{d:02}"))
}

fn is_excluded_article_type(doc: &roxmltree::Document<'_>) -> bool {
    let contains_excluded = |raw: &str| {
        let lower = raw.to_ascii_lowercase();
        lower.contains("preprint")
            || lower.contains("author-manuscript")
            || lower.contains("author manuscript")
    };
    if doc
        .root_element()
        .attribute("article-type")
        .is_some_and(contains_excluded)
    {
        return true;
    }
    doc.descendants().any(|n| {
        n.is_element()
            && matches!(n.tag_name().name(), "subject" | "article-meta")
            && (n.attribute("article-type").is_some_and(contains_excluded)
                || (n.tag_name().name() == "subject"
                    && contains_excluded(&collect_normalized_text(n))))
    })
}

fn verify_pmc_open_access_license(
    pmcid: &str,
    attr: Option<&str>,
    body: &str,
) -> Result<String, SourceParseError> {
    let combined =
        format!("{} {}", attr.unwrap_or_default().trim(), body.trim()).to_ascii_lowercase();

    let is_restricted = combined.contains("-nc")
        || combined.contains("by-nc")
        || combined.contains("-nd")
        || combined.contains("by-nd")
        || combined.contains("noncommercial")
        || combined.contains("non-commercial")
        || combined.contains("noderivatives")
        || combined.contains("no-derivatives")
        || combined.contains("all-rights-reserved")
        || combined.contains("all rights reserved")
        || combined.contains("restricted");

    if is_restricted {
        return Err(SourceParseError::RightsRefused {
            collection: Source::Pmc,
            item_key: pmcid.to_owned(),
            reason: format!(
                "PMC license `{}` restricts commercial reuse or derivatives",
                attr.unwrap_or(body)
            ),
        });
    }

    if combined.contains("cc0") || combined.contains("creativecommons.org/publicdomain/zero") {
        return Ok("CC0".to_owned());
    }
    if combined.contains("cc-by")
        || combined.contains("cc by")
        || combined.contains("creativecommons.org/licenses/by/")
    {
        return Ok(if combined.contains("4.0") {
            "CC-BY-4.0".to_owned()
        } else if combined.contains("3.0") {
            "CC-BY-3.0".to_owned()
        } else {
            "CC-BY".to_owned()
        });
    }

    Err(SourceParseError::RightsRefused {
        collection: Source::Pmc,
        item_key: pmcid.to_owned(),
        reason: format!(
            "PMC license `{}` is outside the CC0 / CC BY allowlist",
            attr.unwrap_or(body)
        ),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn pmc_with_license(article_type: &str, license_block: &str) -> String {
        format!(
            r#"<?xml version="1.0" encoding="UTF-8"?>
<article article-type="{article_type}">
  <front>
    <article-meta>
      <article-id pub-id-type="pmc">PMC777</article-id>
      <title-group><article-title>Test Study</article-title></title-group>
      <permissions>{license_block}</permissions>
      <abstract><p>Body text.</p></abstract>
    </article-meta>
  </front>
</article>"#
        )
    }

    #[test]
    fn allowlist_accepts_cc0_and_cc_by_and_rejects_restricted_missing_and_preprints() {
        assert!(
            parse_jats_xml(
                "PMC777",
                &pmc_with_license(
                    "research-article",
                    r#"<license license-type="cc-by">CC BY 4.0</license>"#
                )
            )
            .is_ok()
        );
        assert!(
            parse_jats_xml(
                "PMC777",
                &pmc_with_license(
                    "research-article",
                    r#"<license license-type="cc0">CC0 Public Domain</license>"#
                )
            )
            .is_ok()
        );

        for (case, xml) in [
            ("missing license", pmc_with_license("research-article", "")),
            (
                "generic open-access without CC0/CC-BY",
                pmc_with_license(
                    "research-article",
                    r#"<license license-type="open-access">Open Access</license>"#,
                ),
            ),
            (
                "cc-by-nc",
                pmc_with_license(
                    "research-article",
                    r#"<license license-type="cc-by-nc">CC BY-NC 4.0</license>"#,
                ),
            ),
            (
                "cc-by-nd",
                pmc_with_license(
                    "research-article",
                    r#"<license license-type="cc-by-nd">CC BY-ND 4.0</license>"#,
                ),
            ),
            (
                "preprint",
                pmc_with_license(
                    "preprint",
                    r#"<license license-type="cc-by">CC BY 4.0</license>"#,
                ),
            ),
            (
                "author-manuscript",
                pmc_with_license(
                    "author-manuscript",
                    r#"<license license-type="cc-by">CC BY 4.0</license>"#,
                ),
            ),
        ] {
            let err = parse_jats_xml("PMC777", &xml).expect_err(case);
            assert!(
                matches!(err, SourceParseError::RightsRefused { .. }),
                "{case} should fail with RightsRefused, got {err:?}"
            );
        }
    }
}
