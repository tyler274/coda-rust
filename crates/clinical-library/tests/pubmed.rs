//! PubMed's status refresh against recorded NCBI responses and a real
//! Postgres.
//!
//! `fixtures/pubmed/efetch-known-statuses.xml` is NCBI's EFetch response,
//! recorded on 2026-11-09, for six PMIDs: a retracted article that also has
//! an expression of concern (9500320), a corrected article (35664904), a
//! corrected article with an expression of concern (38660323), an article
//! with no notices (34336033), a retraction notice that NLM also marks
//! retracted (33071502), and a PMID NCBI returns nothing for (90000000).
//!
//! A local server stands in for NCBI, answering from a script, so the tests
//! see exactly the requests the client sends and when.

use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use async_trait::async_trait;
use axum::Router;
use axum::extract::State;
use axum::http::{HeaderMap, StatusCode, header};
use axum::response::{IntoResponse, Response};
use axum::routing::post;
use clinical_library::pubmed::efetch::RecordError;
use clinical_library::pubmed::refresh::Cause;
use clinical_library::pubmed::{
    self, Eutils, FetchError, Notices, Pacing, Pmid, RefreshError, SetAside, StatusSource,
};
use clinical_library::{
    Acquired, ArticleState, DeferralReason, Ending, Failure, FailureDetail, HolderState, ItemKey,
    JobError, NightOutcome, RunId, SlotName, Source, Store, StoreError, Streak, WaitReason,
};
use tokio::time::Instant;

mod common;

use common::{Says, Scratch, complete, hold_slot};

const KNOWN: &str = include_str!("fixtures/pubmed/efetch-known-statuses.xml");

fn pmid(value: u64) -> Pmid {
    Pmid::new(value).unwrap()
}

fn known_pmids() -> Vec<Pmid> {
    [9500320, 35664904, 38660323, 34336033, 33071502, 90000000]
        .into_iter()
        .map(pmid)
        .collect()
}

/// The known fixture holding only the records for `wanted`, as NCBI answers
/// a request for just those.
fn only(wanted: &[u64]) -> String {
    let (head, rest) = KNOWN.split_once("<PubmedArticle>").unwrap();
    let (articles, tail) = rest.rsplit_once("</PubmedArticle>").unwrap();
    let kept: String = format!("<PubmedArticle>{articles}</PubmedArticle>")
        .split_inclusive("</PubmedArticle>")
        .filter(|article| {
            wanted
                .iter()
                .any(|pmid| article.contains(&format!("<PMID Version=\"1\">{pmid}</PMID>")))
        })
        .collect();
    format!("{head}{kept}{tail}")
}

/// One scripted answer from the stand-in for NCBI.
#[derive(Clone)]
struct Scripted {
    status: StatusCode,
    retry_after: Option<&'static str>,
    body: String,
}

impl Scripted {
    fn ok(body: &str) -> Self {
        Self {
            status: StatusCode::OK,
            retry_after: None,
            body: body.to_owned(),
        }
    }

    fn status(status: StatusCode, body: &str) -> Self {
        Self {
            status,
            retry_after: None,
            body: body.to_owned(),
        }
    }
}

#[derive(Default)]
struct Script {
    answers: Mutex<VecDeque<Scripted>>,
    /// When each request arrived, and its form.
    requests: Mutex<Vec<(Instant, String)>>,
}

impl Script {
    fn arrivals(&self) -> Vec<Instant> {
        self.requests
            .lock()
            .unwrap()
            .iter()
            .map(|(at, _)| *at)
            .collect()
    }
}

async fn answer(State(script): State<Arc<Script>>, body: String) -> Response {
    script.requests.lock().unwrap().push((Instant::now(), body));
    let next = script.answers.lock().unwrap().pop_front();
    let Some(next) = next else {
        return (StatusCode::INTERNAL_SERVER_ERROR, "script ran out").into_response();
    };
    let mut headers = HeaderMap::new();
    headers.insert(header::CONTENT_TYPE, "text/xml".parse().unwrap());
    if let Some(after) = next.retry_after {
        headers.insert(header::RETRY_AFTER, after.parse().unwrap());
    }
    (next.status, headers, next.body).into_response()
}

/// Starts a stand-in for NCBI that gives `answers` in order, and a client for
/// it paced by `pacing`.
async fn ncbi(answers: Vec<Scripted>, pacing: Pacing) -> (Eutils, Arc<Script>) {
    let script = Arc::new(Script {
        answers: Mutex::new(answers.into()),
        requests: Mutex::default(),
    });
    let app = Router::new()
        .route("/entrez/eutils/efetch.fcgi", post(answer))
        .with_state(script.clone());
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    tokio::spawn(async move { axum::serve(listener, app).await.unwrap() });
    let base = format!("http://{address}/entrez/eutils/");
    let client = Eutils::new(&base, "engineering@example.org", Some("test-key"), pacing).unwrap();
    (client, script)
}

/// Pacing for tests that aren't about pacing: no waits worth noticing.
fn quick() -> Pacing {
    Pacing {
        interval: Duration::from_millis(1),
        attempts: 3,
        backoff: Duration::from_millis(10),
        longest_wait: Duration::from_secs(5),
        timeout: Duration::from_secs(10),
        in_flight: 4,
    }
}

fn slot() -> SlotName {
    SlotName::new("pubmed-status").unwrap()
}

async fn hold(store: &Store) -> clinical_library::Lease {
    hold_slot(store, &slot(), RunId::new()).await
}

async fn notices(store: &Store, value: u64) -> Notices {
    store
        .pubmed_status(pmid(value))
        .await
        .unwrap()
        .unwrap_or_else(|| panic!("{value} has a stored status"))
        .record
        .notices()
}

async fn state(store: &Store, value: u64) -> Option<ArticleState> {
    store.item(&ItemKey::pubmed(pmid(value))).await.unwrap()
}

/// AC1: the stored status matches what NLM says, for retracted, corrected
/// and unremarkable articles; a PMID NLM sends nothing for is set aside, not
/// read as unretracted. A second night re-confirms unchanged records without
/// rewriting them.
#[tokio::test]
async fn stored_status_matches_nlm() {
    let scratch = Scratch::new().await;
    let store = scratch.store().await;
    let (client, script) = ncbi(vec![Scripted::ok(KNOWN), Scripted::ok(KNOWN)], quick()).await;

    let lease = hold(&store).await;
    let report = pubmed::refresh(&store, &lease, &client, &known_pmids())
        .await
        .unwrap();
    assert_eq!(report.done.len(), 5);
    assert!(matches!(
        report.set_aside.as_slice(),
        [(missing, SetAside::Outside { code, cause: Cause::Missing })]
            if *missing == pmid(90000000) && code.as_str() == "pubmed_record_missing"
    ));
    // A missing record is set aside, so the night defers rather than
    // succeeding, and it moved something.
    assert_eq!(
        complete(&store, lease).await,
        NightOutcome::deferred(DeferralReason::PublishedWithSetAside)
    );

    let retracted = Notices {
        retracted: true,
        ..Notices::default()
    };
    assert_eq!(
        notices(&store, 9500320).await,
        Notices {
            expression_of_concern: true,
            ..retracted
        }
    );
    assert_eq!(
        notices(&store, 35664904).await,
        Notices {
            corrected: true,
            ..Notices::default()
        }
    );
    assert_eq!(
        notices(&store, 38660323).await,
        Notices {
            corrected: true,
            expression_of_concern: true,
            ..Notices::default()
        }
    );
    assert_eq!(notices(&store, 34336033).await, Notices::default());
    assert_eq!(notices(&store, 33071502).await, retracted);
    assert_eq!(store.pubmed_status(pmid(90000000)).await.unwrap(), None);
    assert_eq!(state(&store, 9500320).await, Some(ArticleState::Admitted));
    assert!(matches!(
        state(&store, 90000000).await,
        Some(ArticleState::WaitingToRetry(retry)) if retry.failures() == 1
    ));

    // NLM's links are kept in its words and order, with their targets.
    let wakefield = store.pubmed_status(pmid(9500320)).await.unwrap().unwrap();
    let retractions: Vec<_> = wakefield
        .record
        .relations
        .iter()
        .filter(|relation| relation.ref_type == "RetractionIn")
        .map(|relation| relation.target)
        .collect();
    assert_eq!(retractions, [Some(pmid(15016483)), Some(pmid(20137807))]);
    assert!(
        wakefield
            .record
            .publication_types
            .contains(&"Retracted Publication".to_owned())
    );
    // The record is kept exactly as NLM sent it, so it can be read again.
    let sent = KNOWN
        .split_inclusive("</PubmedArticle>")
        .find(|article| article.contains("<PMID Version=\"1\">9500320</PMID>"))
        .unwrap();
    assert!(sent.trim_start().ends_with(&wakefield.record.record_xml));
    let reread = pubmed::efetch::read(
        format!(
            "<PubmedArticleSet>{}</PubmedArticleSet>",
            wakefield.record.record_xml
        )
        .as_bytes(),
        &[pmid(9500320)],
    )
    .unwrap();
    assert!(
        matches!(&reread[0].1, pubmed::efetch::Answer::Found(again) if *again == wakefield.record)
    );
    // The database refuses a record whose hash doesn't match it.
    let mismatch = scratch
        .refused(
            "update clinical_library_pubmed_status set record_xml = record_xml || ' ' \
             where pmid = 9500320",
        )
        .await;
    assert!(mismatch.contains("check"), "{mismatch}");

    // The next night re-confirms the same records without replacing them.
    let lease = hold(&store).await;
    let again = pubmed::refresh(&store, &lease, &client, &known_pmids())
        .await
        .unwrap();
    complete(&store, lease).await;
    assert_eq!(again.done.len(), 5);
    let confirmed = store.pubmed_status(pmid(9500320)).await.unwrap().unwrap();
    assert_eq!(confirmed.observed_at, wakefield.observed_at);
    assert!(confirmed.confirmed_at > wakefield.confirmed_at);
    assert_eq!(confirmed.record, wakefield.record);

    // Each request named the database, the tool, the contact and the key.
    let requests = script.requests.lock().unwrap().clone();
    assert_eq!(requests.len(), 2);
    let form = &requests[0].1;
    for part in [
        "db=pubmed",
        "retmode=xml",
        "tool=coda-clinical-library",
        "email=engineering%40example.org",
        "api_key=test-key",
        "id=9500320%2C33071502%2C34336033%2C35664904%2C38660323%2C90000000",
    ] {
        assert!(form.contains(part), "{part} in {form}");
    }
    scratch.drop().await;
}

/// The known fixture with 34336033's record broken: a link with no RefType,
/// which this reader can't read.
fn with_unreadable_record() -> String {
    let anchor = "<PMID Version=\"1\">34336033</PMID>";
    assert!(KNOWN.contains(anchor));
    KNOWN.replace(
        anchor,
        &format!(
            "{anchor}<CommentsCorrectionsList><CommentsCorrections>\
             <RefSource>x</RefSource></CommentsCorrections></CommentsCorrectionsList>"
        ),
    )
}

/// AC2: an article whose record can't be read is set aside, tells a person,
/// and keeps its last good status, while every other article finishes.
#[tokio::test]
async fn one_failing_article_is_set_aside_and_the_rest_finish() {
    let scratch = Scratch::new().await;
    let store = scratch.store().await;
    let pmids = &known_pmids()[..5];
    let (client, _) = ncbi(
        vec![Scripted::ok(KNOWN), Scripted::ok(&with_unreadable_record())],
        quick(),
    )
    .await;

    // A first night admits all five.
    let lease = hold(&store).await;
    let first = pubmed::refresh(&store, &lease, &client, pmids)
        .await
        .unwrap();
    assert_eq!(first.done.len(), 5);
    assert!(complete(&store, lease).await.success().is_some());
    let good = store.pubmed_status(pmid(34336033)).await.unwrap().unwrap();

    // The next night NLM's record for one of them can't be read.
    let lease = hold(&store).await;
    let report = pubmed::refresh(&store, &lease, &client, pmids)
        .await
        .unwrap();
    assert_eq!(report.done.len(), 4);
    assert!(matches!(
        report.set_aside.as_slice(),
        [(broken, SetAside::OwnDefect(RecordError::NoRefType))] if *broken == pmid(34336033)
    ));
    assert_eq!(
        complete(&store, lease).await,
        NightOutcome::deferred(DeferralReason::PublishedWithSetAside)
    );

    let broken = state(&store, 34336033).await.unwrap();
    assert_eq!(
        broken,
        ArticleState::WaitingForChange {
            reason: WaitReason::OwnDefect
        }
    );
    assert!(broken.tells_person_now());
    assert_eq!(
        store.pubmed_status(pmid(34336033)).await.unwrap(),
        Some(good),
        "the last good status stays"
    );
    assert!(notices(&store, 9500320).await.retracted);

    // A later night doesn't try it again until its reader changes.
    let others = only(&[9500320, 35664904, 38660323, 33071502]);
    let (client, script) = ncbi(vec![Scripted::ok(&others)], quick()).await;
    let lease = hold(&store).await;
    let later = pubmed::refresh(&store, &lease, &client, pmids)
        .await
        .unwrap();
    complete(&store, lease).await;
    assert_eq!(later.not_due, [pmid(34336033)]);
    let asked = script.requests.lock().unwrap()[0].1.clone();
    assert!(!asked.contains("34336033"));
    scratch.drop().await;
}

/// A source that fails every request the same way, standing in for an NCBI
/// outage.
struct Outage;

#[async_trait]
impl StatusSource for Outage {
    async fn efetch(&self, _: &[Pmid]) -> Result<Vec<u8>, FetchError> {
        Err(FetchError::Throttled)
    }

    fn in_flight(&self) -> usize {
        4
    }
}

/// AC2: an outage sets every new article aside to retry, a night that set
/// everything aside moved nothing and counts toward the stop rule, and an
/// admitted article keeps its status and state through it.
#[tokio::test]
async fn an_outage_sets_articles_aside_without_losing_status() {
    let scratch = Scratch::new().await;
    let store = scratch.store().await;
    let (client, _) = ncbi(vec![Scripted::ok(&only(&[9500320]))], quick()).await;
    let lease = hold(&store).await;
    let admitted = pubmed::refresh(&store, &lease, &client, &[pmid(9500320)])
        .await
        .unwrap();
    assert_eq!(admitted.done, [pmid(9500320)]);
    assert!(complete(&store, lease).await.success().is_some());

    let lease = hold(&store).await;
    let report = pubmed::refresh(&store, &lease, &Outage, &[pmid(9500320), pmid(34336033)])
        .await
        .unwrap();
    assert!(report.done.is_empty());
    assert_eq!(report.set_aside.len(), 2);
    for (_, why) in &report.set_aside {
        assert!(matches!(
            why,
            SetAside::Outside { code, cause: Cause::Fetch(_) } if code.as_str() == "ncbi_throttled"
        ));
    }
    let outcome = complete(&store, lease).await;
    assert_eq!(
        outcome,
        NightOutcome::deferred(DeferralReason::MovedNothing)
    );
    assert_eq!(outcome.streak(), Streak::Counts);

    assert_eq!(state(&store, 9500320).await, Some(ArticleState::Admitted));
    assert!(notices(&store, 9500320).await.retracted);
    assert!(matches!(
        state(&store, 34336033).await,
        Some(ArticleState::WaitingToRetry(retry)) if retry.failures() == 1
    ));
    scratch.drop().await;
}

/// A failed re-confirmation of an admitted article isn't tonight's work: it
/// leaves the article unconfirmed, which the freshness report shows, and
/// doesn't count against the night or toward the stop rule.
#[tokio::test]
async fn failed_reconfirms_dont_count_against_the_night() {
    let scratch = Scratch::new().await;
    let store = scratch.store().await;
    let (client, _) = ncbi(vec![Scripted::ok(&only(&[9500320]))], quick()).await;
    let lease = hold(&store).await;
    pubmed::refresh(&store, &lease, &client, &[pmid(9500320)])
        .await
        .unwrap();
    assert!(complete(&store, lease).await.success().is_some());
    let confirmed = store.pubmed_status(pmid(9500320)).await.unwrap().unwrap();

    let lease = hold(&store).await;
    let report = pubmed::refresh(&store, &lease, &Outage, &[pmid(9500320)])
        .await
        .unwrap();
    assert_eq!(report.set_aside.len(), 1);
    assert!(complete(&store, lease).await.success().is_some());
    assert_eq!(
        store.pubmed_status(pmid(9500320)).await.unwrap(),
        Some(confirmed),
        "unconfirmed tonight, with its status kept"
    );
    scratch.drop().await;
}

/// An article a person resolves after the night took the slot is the next
/// night's: tonight's refresh doesn't try it, even though its list names it,
/// and the night isn't held back by it.
#[tokio::test]
async fn an_article_resolved_mid_night_is_the_next_nights() {
    let scratch = Scratch::new().await;
    let store = scratch.store().await;
    let key = ItemKey::pubmed(pmid(9500320));

    // Three outside failures in one night put it in front of a person.
    let lease = hold(&store).await;
    store.qualify(&lease, &key).await.unwrap();
    let missing = clinical_library::ArticleEvent::OutsideFailure(
        clinical_library::FailureCode::new("pubmed_record_missing").unwrap(),
    );
    for _ in 0..3 {
        store.apply(&lease, &key, &missing).await.unwrap();
    }
    assert!(matches!(
        state(&store, 9500320).await,
        Some(ArticleState::NeedsPerson { .. })
    ));
    complete(&store, lease).await;

    // Tonight takes the slot, then a person resolves it.
    let lease = hold(&store).await;
    assert!(store.resolve_item(&key).await.unwrap().is_due());
    let report = pubmed::refresh(&store, &lease, &Outage, &[pmid(9500320)])
        .await
        .unwrap();
    assert_eq!(report.not_due, [pmid(9500320)]);
    assert!(report.set_aside.is_empty());
    assert!(complete(&store, lease).await.success().is_some());
    assert_eq!(
        state(&store, 9500320).await,
        Some(ArticleState::new_article()),
        "no attempt counted against it tonight"
    );

    // The next night tries it.
    let (client, _) = ncbi(vec![Scripted::ok(&only(&[9500320]))], quick()).await;
    let lease = hold(&store).await;
    let report = pubmed::refresh(&store, &lease, &client, &[pmid(9500320)])
        .await
        .unwrap();
    assert_eq!(report.done, [pmid(9500320)]);
    assert!(complete(&store, lease).await.success().is_some());
    scratch.drop().await;
}

/// A night that lost its slot to another run records nothing more: the
/// status write is refused with the slot check, in one transaction.
#[tokio::test]
async fn a_run_that_lost_its_slot_records_no_status() {
    let scratch = Scratch::new().await;
    let store = scratch.store().await;
    let old = hold(&store).await;
    match store
        .acquire(
            &slot(),
            &[Source::PubMed],
            RunId::new(),
            &Says(HolderState::Ended),
        )
        .await
        .unwrap()
    {
        Acquired::Holds(_) => {}
        other => panic!("an ended holder's slot is taken: {other:?}"),
    }
    let (client, _) = ncbi(vec![Scripted::ok(&only(&[9500320]))], quick()).await;
    let error = pubmed::refresh(&store, &old, &client, &[pmid(9500320)])
        .await
        .unwrap_err();
    assert!(matches!(
        error,
        RefreshError::Store(StoreError::LeaseLost { .. })
    ));
    assert_eq!(error.failure(), Failure::LostSlot);
    assert_eq!(store.pubmed_status(pmid(9500320)).await.unwrap(), None);
    assert_eq!(state(&store, 9500320).await, None);
    scratch.drop().await;
}

/// AC3: with a key, requests start at least 100 ms apart, which keeps under
/// NCBI's 10 a second.
#[tokio::test]
async fn requests_keep_within_ncbis_rate() {
    let answers = vec![Scripted::ok(&only(&[9500320])); 5];
    let (client, script) = ncbi(answers, Pacing::ncbi(true)).await;
    for _ in 0..5 {
        client.efetch(&[pmid(9500320)]).await.unwrap();
    }
    let arrivals = script.arrivals();
    for pair in arrivals.windows(2) {
        let gap = pair[1] - pair[0];
        // Arrival is a little after the client's start; allow for jitter in
        // the server noticing it.
        assert!(gap >= Duration::from_millis(95), "requests {gap:?} apart");
    }
    assert_eq!(Pacing::ncbi(false).interval, Duration::from_millis(334));
}

/// AC3: throttling waits as long as NCBI asks, server errors back off and
/// retry, and a refused request is never retried.
#[tokio::test]
async fn throttling_and_server_errors_back_off() {
    let throttled = Scripted {
        retry_after: Some("1"),
        ..Scripted::status(
            StatusCode::TOO_MANY_REQUESTS,
            r#"{"error":"API rate limit exceeded","count":"11"}"#,
        )
    };
    let (client, script) = ncbi(vec![throttled, Scripted::ok(KNOWN)], quick()).await;
    // The client returns NCBI's bytes; reading them against the request is
    // the refresh's job, so any body serves here.
    client.efetch(&[pmid(9500320)]).await.unwrap();
    let arrivals = script.arrivals();
    assert_eq!(arrivals.len(), 2);
    assert!(arrivals[1] - arrivals[0] >= Duration::from_millis(950));

    let unavailable = Scripted::status(StatusCode::SERVICE_UNAVAILABLE, "");
    let (client, script) = ncbi(
        vec![unavailable.clone(), unavailable, Scripted::ok(KNOWN)],
        quick(),
    )
    .await;
    client.efetch(&[pmid(9500320)]).await.unwrap();
    let arrivals = script.arrivals();
    assert_eq!(arrivals.len(), 3);
    // The second wait is twice the first.
    assert!(arrivals[1] - arrivals[0] >= Duration::from_millis(10));
    assert!(arrivals[2] - arrivals[1] >= Duration::from_millis(20));

    let down = Scripted::status(StatusCode::BAD_GATEWAY, "");
    let (client, script) = ncbi(vec![down; 3], quick()).await;
    let error = client.efetch(&[pmid(9500320)]).await.unwrap_err();
    assert!(matches!(error, FetchError::Server(StatusCode::BAD_GATEWAY)));
    assert!(error.is_outside());
    assert_eq!(error.code().as_str(), "ncbi_server_error");
    assert_eq!(script.arrivals().len(), 3, "tried the configured 3 times");

    let invalid_key = Scripted::status(
        StatusCode::BAD_REQUEST,
        r#"{"error":"API key invalid","api-key":"test-key","type":"invalid","status":"unknown"}"#,
    );
    let (client, script) = ncbi(vec![invalid_key], quick()).await;
    let error = client.efetch(&[pmid(9500320)]).await.unwrap_err();
    assert!(matches!(error, FetchError::Refused { .. }));
    // NCBI echoes the rejected key; the error never carries it.
    let shown = format!("{error} {error:?}");
    assert!(!shown.contains("test-key"), "{shown}");
    assert!(shown.contains("API key invalid"), "{shown}");
    assert!(!error.is_outside());
    assert_eq!(script.arrivals().len(), 1, "a refusal is never retried");
}

/// A request NCBI refuses, such as for an invalid key, fails the night
/// instead of setting every article aside.
#[tokio::test]
async fn a_refused_request_fails_the_night() {
    let scratch = Scratch::new().await;
    let store = scratch.store().await;
    let refused = Scripted::status(StatusCode::BAD_REQUEST, r#"{"error":"API key invalid"}"#);
    let (client, _) = ncbi(vec![refused], quick()).await;
    let lease = hold(&store).await;
    let error = pubmed::refresh(&store, &lease, &client, &[pmid(9500320)])
        .await
        .unwrap_err();
    let Failure::Code(code) = error.failure() else {
        panic!("a refusal fails the night with a code")
    };
    assert_eq!(code.as_str(), "ncbi_refused_request");
    let detail = FailureDetail::of(&error);
    assert!(!detail.as_str().contains("api_key"));
    store
        .finish(lease, Ending::Failed { code, detail })
        .await
        .unwrap();
    // The list qualified it, but no attempt counted against it.
    assert_eq!(
        state(&store, 9500320).await,
        Some(ArticleState::new_article()),
        "qualified and untried"
    );
    scratch.drop().await;
}

/// The known fixture with NLM newly marking 34336033 retracted, as a later
/// revision would.
fn with_new_retraction() -> String {
    let anchor = "<PMID Version=\"1\">34336033</PMID>";
    KNOWN.replace(
        anchor,
        &format!(
            "{anchor}<CommentsCorrectionsList><CommentsCorrections RefType=\"RetractionIn\">\
             <RefSource>Example notice</RefSource><PMID Version=\"1\">40000001</PMID>\
             </CommentsCorrections></CommentsCorrectionsList>"
        ),
    )
}

/// AC1: when NLM changes an admitted article's record, such as a new
/// retraction, the next night replaces the stored status and its links.
#[tokio::test]
async fn a_changed_record_replaces_the_stored_status() {
    let scratch = Scratch::new().await;
    let store = scratch.store().await;
    let pmids = &known_pmids()[..5];
    let (client, _) = ncbi(
        vec![Scripted::ok(KNOWN), Scripted::ok(&with_new_retraction())],
        quick(),
    )
    .await;

    let lease = hold(&store).await;
    let first = pubmed::refresh(&store, &lease, &client, pmids)
        .await
        .unwrap();
    assert_eq!(first.done.len(), 5);
    assert!(complete(&store, lease).await.success().is_some());
    let before = store.pubmed_status(pmid(34336033)).await.unwrap().unwrap();
    assert!(!before.record.notices().retracted);

    let lease = hold(&store).await;
    let second = pubmed::refresh(&store, &lease, &client, pmids)
        .await
        .unwrap();
    assert_eq!(second.done.len(), 5);
    assert!(complete(&store, lease).await.success().is_some());

    let after = store.pubmed_status(pmid(34336033)).await.unwrap().unwrap();
    assert!(after.record.notices().retracted);
    assert_ne!(after.record.record_sha256, before.record.record_sha256);
    assert!(after.observed_at > before.observed_at);
    assert_eq!(
        after.record.relations[0].target,
        Some(pmid(40000001)),
        "the new link is stored with its target"
    );
    assert_eq!(state(&store, 34336033).await, Some(ArticleState::Admitted));
    // The other articles were only re-confirmed.
    let unchanged = store.pubmed_status(pmid(9500320)).await.unwrap().unwrap();
    assert!(unchanged.confirmed_at > unchanged.observed_at);
    scratch.drop().await;
}
