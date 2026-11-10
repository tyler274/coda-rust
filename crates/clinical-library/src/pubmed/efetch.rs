//! Reading one EFetch response into status records.
//!
//! The response must be a `PubmedArticleSet` that answers only what was
//! asked. Each requested PMID gets exactly one answer: its record, missing
//! (NLM sent nothing for it), or unreadable (NLM sent a record this reader
//! can't read). One unreadable record never hides the others. Absence from a
//! response is never read as a retraction or a deletion.

use std::collections::{BTreeMap, BTreeSet};

use chrono::NaiveDate;
use roxmltree::{Document, Node, ParsingOptions};
use sha2::{Digest, Sha256};

use super::record::{InvalidPmid, Pmid, Relation, StatusRecord};

/// The largest response this reader accepts.
pub const MAX_RESPONSE_BYTES: usize = 16 * 1024 * 1024;

/// NLM's answer for one requested PMID.
#[derive(Clone, Debug)]
pub enum Answer {
    /// NLM's record.
    Found(StatusRecord),
    /// NLM sent no record for it.
    Missing,
    /// NLM sent a record for it that this reader couldn't read.
    Unreadable {
        /// The hash of the record as sent, so a later night can tell whether
        /// NLM changed it.
        record_sha256: [u8; 32],
        /// What was wrong with it.
        cause: RecordError,
    },
}

/// A response that answers nothing, because the whole of it is unusable.
#[derive(Debug, thiserror::Error)]
pub enum ResponseError {
    /// The response is larger than [`MAX_RESPONSE_BYTES`] or empty.
    #[error("the response is {0} bytes")]
    Size(usize),
    /// The response declares XML entities, which this reader refuses.
    #[error("the response declares XML entities")]
    Entities,
    /// The response isn't UTF-8.
    #[error("the response isn't UTF-8")]
    NotUtf8(#[from] std::str::Utf8Error),
    /// The response isn't XML.
    #[error("the response isn't XML")]
    NotXml(#[from] roxmltree::Error),
    /// The response isn't a `PubmedArticleSet`, such as an E-utilities error
    /// page.
    #[error("the response is a {0:?}, not a PubmedArticleSet")]
    UnexpectedRoot(String),
    /// The response holds something other than PubMed articles, book
    /// articles and deleted citations.
    #[error("the response holds a {0:?}")]
    UnexpectedElement(String),
    /// The response answers a PMID that wasn't asked, or one twice.
    #[error("the response answers {0} unasked or twice")]
    Unasked(Pmid),
}

/// What was wrong with one article's record.
#[derive(Clone, Debug, PartialEq, Eq, thiserror::Error)]
pub enum RecordError {
    /// The record has no `MedlineCitation`, or a book record no
    /// `BookDocument`.
    #[error("the record has no MedlineCitation or BookDocument")]
    NoCitation,
    /// The record has no publication types. NLM's format requires at least
    /// one, so a record without them can't be trusted to say whether the
    /// article was retracted.
    #[error("the record has no publication types")]
    NoPublicationTypes,
    /// The record's PMID, or a link's target PMID, isn't a PMID.
    #[error("the record holds an invalid PMID")]
    Pmid(#[from] InvalidPmid),
    /// A comments-and-corrections link has no `RefType`.
    #[error("a link has no RefType")]
    NoRefType,
    /// The record's revision date isn't a date.
    #[error("the revision date isn't a date")]
    RevisedDate,
}

/// Reads `body` as the answer to `requested`, in the order requested.
pub fn read(body: &[u8], requested: &[Pmid]) -> Result<Vec<(Pmid, Answer)>, ResponseError> {
    if body.is_empty() || body.len() > MAX_RESPONSE_BYTES {
        return Err(ResponseError::Size(body.len()));
    }
    if body.windows(8).any(|window| window == b"<!ENTITY") {
        return Err(ResponseError::Entities);
    }
    let text = std::str::from_utf8(body)?;
    // NLM's responses carry a DOCTYPE naming its DTD. The DTD is never
    // fetched, and entity declarations were refused above.
    let options = ParsingOptions {
        allow_dtd: true,
        ..ParsingOptions::default()
    };
    let document = Document::parse_with_options(text, options)?;
    let root = document.root_element();
    if root.tag_name().name() != "PubmedArticleSet" {
        return Err(ResponseError::UnexpectedRoot(
            root.tag_name().name().to_owned(),
        ));
    }

    let asked: BTreeSet<Pmid> = requested.iter().copied().collect();
    let mut answers = BTreeMap::new();
    for article in root.children().filter(Node::is_element) {
        let kind = match article.tag_name().name() {
            "PubmedArticle" => Kind::Journal,
            "PubmedBookArticle" => Kind::Book,
            // NLM's format allows a trailing list of deleted citations. It
            // answers no request, so a requested PMID it names reads as
            // missing.
            "DeleteCitation" => continue,
            other => return Err(ResponseError::UnexpectedElement(other.to_owned())),
        };
        let raw = &text[article.range()];
        let record_sha256: [u8; 32] = Sha256::digest(raw.as_bytes()).into();
        let (pmid, answer) = match status_record(article, kind, raw, record_sha256) {
            Ok(record) => (record.pmid, Answer::Found(record)),
            Err((Some(pmid), cause)) => (
                pmid,
                Answer::Unreadable {
                    record_sha256,
                    cause,
                },
            ),
            // A record whose own PMID can't be read can't be matched to a
            // request; the PMID it was meant to answer reads as missing.
            Err((None, _)) => continue,
        };
        if !asked.contains(&pmid) || answers.insert(pmid, answer).is_some() {
            return Err(ResponseError::Unasked(pmid));
        }
    }

    Ok(requested
        .iter()
        .map(|pmid| (*pmid, answers.remove(pmid).unwrap_or(Answer::Missing)))
        .collect())
}

/// Which of NLM's two record shapes a record is.
#[derive(Clone, Copy)]
enum Kind {
    /// A `PubmedArticle`: a journal article, whose `MedlineCitation` must
    /// hold at least one publication type and may hold correction links.
    Journal,
    /// A `PubmedBookArticle`, such as a book chapter: its `BookDocument` may
    /// hold publication types, and NLM's format gives it no correction links.
    Book,
}

/// Reads one `PubmedArticle` or `PubmedBookArticle`. On failure, says which
/// PMID it was for, if that much could be read.
fn status_record(
    article: Node<'_, '_>,
    kind: Kind,
    raw: &str,
    record_sha256: [u8; 32],
) -> Result<StatusRecord, (Option<Pmid>, RecordError)> {
    let body = match kind {
        Kind::Journal => "MedlineCitation",
        Kind::Book => "BookDocument",
    };
    let citation = child(article, body).ok_or((None, RecordError::NoCitation))?;
    let pmid = Pmid::parse(&text(child(citation, "PMID")))
        .map_err(|error| (None, RecordError::Pmid(error)))?;
    let fail = |cause| (Some(pmid), cause);

    let types: Vec<Node<'_, '_>> = match kind {
        Kind::Journal => children(citation, &["Article", "PublicationTypeList"]).collect(),
        Kind::Book => citation.children().filter(Node::is_element).collect(),
    };
    let publication_types: Vec<String> = types
        .into_iter()
        .filter(|node| node.tag_name().name() == "PublicationType")
        .map(|node| text(Some(node)))
        .filter(|kind| !kind.is_empty())
        .collect();
    if matches!(kind, Kind::Journal) && publication_types.is_empty() {
        return Err(fail(RecordError::NoPublicationTypes));
    }

    let mut relations = Vec::new();
    for link in children(citation, &["CommentsCorrectionsList"])
        .filter(|node| node.tag_name().name() == "CommentsCorrections")
    {
        let ref_type = link
            .attribute("RefType")
            .filter(|value| !value.is_empty())
            .ok_or(fail(RecordError::NoRefType))?
            .to_owned();
        let target = optional(child(link, "PMID"))
            .map(|value| Pmid::parse(&value))
            .transpose()
            .map_err(|error| fail(RecordError::Pmid(error)))?;
        relations.push(Relation {
            ref_type,
            target,
            source_reference: optional(child(link, "RefSource")),
            note: optional(child(link, "Note")),
        });
    }

    let revised = child(citation, "DateRevised")
        .map(|date| {
            let part = |name| text(child(date, name)).parse::<u32>().ok();
            let year = part("Year").and_then(|year| i32::try_from(year).ok());
            match (year, part("Month"), part("Day")) {
                (Some(year), Some(month), Some(day)) => NaiveDate::from_ymd_opt(year, month, day),
                _ => None,
            }
            .ok_or(fail(RecordError::RevisedDate))
        })
        .transpose()?;

    Ok(StatusRecord {
        pmid,
        publication_types,
        relations,
        revised,
        record_sha256,
        record_xml: raw.to_owned(),
    })
}

fn child<'a, 'input>(node: Node<'a, 'input>, name: &str) -> Option<Node<'a, 'input>> {
    node.children()
        .find(|child| child.is_element() && child.tag_name().name() == name)
}

/// The element children of the element reached by following `path`.
fn children<'a, 'input>(
    node: Node<'a, 'input>,
    path: &[&str],
) -> impl Iterator<Item = Node<'a, 'input>> {
    path.iter()
        .try_fold(node, |node, name| child(node, name))
        .into_iter()
        .flat_map(|parent| parent.children().filter(Node::is_element))
}

/// An element's text, trimmed; empty when the element is absent.
fn text(node: Option<Node<'_, '_>>) -> String {
    node.map(|node| {
        node.descendants()
            .filter(Node::is_text)
            .filter_map(|text| text.text())
            .collect::<String>()
            .trim()
            .to_owned()
    })
    .unwrap_or_default()
}

/// An element's text, or `None` when it is absent or empty.
fn optional(node: Option<Node<'_, '_>>) -> Option<String> {
    Some(text(node)).filter(|value| !value.is_empty())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn pmid(value: u64) -> Pmid {
        Pmid::new(value).unwrap()
    }

    fn article(pmid: &str, extra: &str) -> String {
        format!(
            "<PubmedArticle><MedlineCitation><PMID Version=\"1\">{pmid}</PMID>\
             <Article><PublicationTypeList><PublicationType>Journal Article</PublicationType>\
             </PublicationTypeList></Article>{extra}</MedlineCitation></PubmedArticle>"
        )
    }

    fn set(articles: &[String]) -> Vec<u8> {
        format!(
            "<?xml version=\"1.0\" ?>\n<!DOCTYPE PubmedArticleSet PUBLIC \"-//NLM//DTD PubMedArticle, 1st January 2025//EN\" \"https://dtd.nlm.nih.gov/ncbi/pubmed/out/pubmed_250101.dtd\">\n<PubmedArticleSet>{}</PubmedArticleSet>",
            articles.concat()
        )
        .into_bytes()
    }

    #[test]
    fn every_requested_pmid_gets_one_answer_in_order() {
        let body = set(&[article("2", ""), article("1", "")]);
        let answers = read(&body, &[pmid(1), pmid(2), pmid(3)]).unwrap();
        let order: Vec<_> = answers.iter().map(|(pmid, _)| pmid.get()).collect();
        assert_eq!(order, [1, 2, 3]);
        assert!(matches!(answers[0].1, Answer::Found(_)));
        assert!(matches!(answers[2].1, Answer::Missing));
    }

    #[test]
    fn one_unreadable_record_hides_no_other() {
        let broken = article(
            "2",
            "<CommentsCorrectionsList><CommentsCorrections><PMID>x</PMID>\
             </CommentsCorrections></CommentsCorrectionsList>",
        );
        let body = set(&[article("1", ""), broken]);
        let answers = read(&body, &[pmid(1), pmid(2)]).unwrap();
        assert!(matches!(answers[0].1, Answer::Found(_)));
        assert!(matches!(
            answers[1].1,
            Answer::Unreadable {
                cause: RecordError::NoRefType,
                ..
            }
        ));
    }

    #[test]
    fn a_response_that_answers_the_wrong_question_answers_nothing() {
        let unasked = set(&[article("9", "")]);
        assert!(matches!(
            read(&unasked, &[pmid(1)]),
            Err(ResponseError::Unasked(found)) if found == pmid(9)
        ));
        let twice = set(&[article("1", ""), article("1", "")]);
        assert!(matches!(
            read(&twice, &[pmid(1)]),
            Err(ResponseError::Unasked(found)) if found == pmid(1)
        ));
        let error_page = b"<eFetchResult><ERROR>Empty id list</ERROR></eFetchResult>";
        assert!(matches!(
            read(error_page, &[pmid(1)]),
            Err(ResponseError::UnexpectedRoot(_))
        ));
        let entities = b"<!DOCTYPE x [<!ENTITY a \"b\">]><PubmedArticleSet/>";
        assert!(matches!(
            read(entities, &[pmid(1)]),
            Err(ResponseError::Entities)
        ));
        assert!(matches!(
            read(b"<PubmedArticleSet>", &[pmid(1)]),
            Err(ResponseError::NotXml(_))
        ));
        assert!(matches!(read(b"", &[pmid(1)]), Err(ResponseError::Size(0))));
    }

    #[test]
    fn a_record_without_publication_types_is_unreadable() {
        let bare = "<PubmedArticle><MedlineCitation><PMID Version=\"1\">1</PMID>\
                    </MedlineCitation></PubmedArticle>"
            .to_owned();
        let answers = read(&set(&[bare]), &[pmid(1)]).unwrap();
        assert!(matches!(
            answers[0].1,
            Answer::Unreadable {
                cause: RecordError::NoPublicationTypes,
                ..
            }
        ));
    }

    /// NCBI's real response for a book chapter (20301484) and a journal
    /// article (34336033), recorded 2026-11-09. One book answers its own PMID
    /// and never hides the article beside it.
    #[test]
    fn book_records_answer_their_pmids() {
        let body = include_bytes!("../../tests/fixtures/pubmed/efetch-book-and-article.xml");
        let answers = read(body, &[pmid(20301484), pmid(34336033)]).unwrap();
        let [(_, Answer::Found(book)), (_, Answer::Found(article))] = answers.as_slice() else {
            panic!("both records read: {answers:?}");
        };
        assert_eq!(book.publication_types, ["Review"]);
        assert!(book.relations.is_empty());
        assert_eq!(book.notices(), super::super::Notices::default());
        assert_eq!(article.publication_types, ["Journal Article"]);
    }

    #[test]
    fn deleted_citations_answer_nothing() {
        let body = format!(
            "<PubmedArticleSet>{}<DeleteCitation><PMID Version=\"1\">2</PMID></DeleteCitation>\
             </PubmedArticleSet>",
            article("1", "")
        );
        let answers = read(body.as_bytes(), &[pmid(1), pmid(2)]).unwrap();
        assert!(matches!(answers[0].1, Answer::Found(_)));
        assert!(matches!(answers[1].1, Answer::Missing));
    }

    #[test]
    fn the_hash_follows_the_record_bytes() {
        let first = read(&set(&[article("1", "")]), &[pmid(1)]).unwrap();
        let revised = read(
            &set(&[article(
                "1",
                "<DateRevised><Year>2024</Year><Month>02</Month><Day>29</Day></DateRevised>",
            )]),
            &[pmid(1)],
        )
        .unwrap();
        let (Answer::Found(first), Answer::Found(revised)) = (&first[0].1, &revised[0].1) else {
            panic!("both records read");
        };
        assert_ne!(first.record_sha256, revised.record_sha256);
        // The hash is of the record exactly as kept.
        let kept: [u8; 32] = Sha256::digest(revised.record_xml.as_bytes()).into();
        assert_eq!(kept, revised.record_sha256);
        assert!(revised.record_xml.starts_with("<PubmedArticle>"));
        assert!(revised.record_xml.ends_with("</PubmedArticle>"));
        assert_eq!(revised.revised, NaiveDate::from_ymd_opt(2024, 2, 29));
    }
}
