//! AC12: PubMed's requests overlap up to NCBI's rate, with one limiter
//! shared by every request in flight.
//!
//! A local stand-in for NCBI answers each EFetch request with a record for
//! every PMID it names, after the latency NCBI showed live (about 1.3 s for
//! 200 PMIDs), and records when each request arrived. The client's pacing is
//! NCBI's with a key: one request start every 100 ms.
//!
//! The stand-in runs on its own thread and runtime, so the night's writes
//! don't delay when it notices a request arrive.
//!
//! - `one_limiter_paces_every_request_in_flight`: 40 requests at once on one
//!   client start 100 ms apart and finish in about 4 s, not 52 s.
//! - `the_refresh_overlaps_its_requests`: a night over 40 batches overlaps
//!   its requests within the same pacing.
//! - `a_full_night_finishes_within_an_hour` is ignored by default: it runs a
//!   whole night of about 1.25 million PMIDs (6,250 requests) and takes
//!   about a quarter of an hour.

use std::sync::{Arc, Mutex};
use std::time::Duration;

use axum::Router;
use axum::extract::State;
use axum::http::{StatusCode, header};
use axum::response::{IntoResponse, Response};
use axum::routing::post;
use clinical_library::pubmed::client::BATCH;
use clinical_library::pubmed::{self, Eutils, Pacing, Pmid, RefreshError, StatusSource};
use clinical_library::{RunId, SlotName};
use tokio::time::Instant;

mod common;

use common::{Scratch, complete, hold_slot};

/// What NCBI showed live for one 200-PMID EFetch request.
const LATENCY: Duration = Duration::from_millis(1300);

/// A stand-in for NCBI that answers every request after `latency`.
struct StandIn {
    latency: Duration,
    refuse: bool,
    arrivals: Mutex<Vec<Instant>>,
}

impl StandIn {
    /// The gaps between consecutive arrivals, in order.
    fn gaps(&self) -> Vec<Duration> {
        let mut arrivals = self.arrivals.lock().unwrap().clone();
        arrivals.sort();
        arrivals.windows(2).map(|pair| pair[1] - pair[0]).collect()
    }

    fn requests(&self) -> usize {
        self.arrivals.lock().unwrap().len()
    }
}

/// One minimal PubMed record for `pmid`: what the reader needs.
fn record(pmid: &str) -> String {
    format!(
        "<PubmedArticle><MedlineCitation><PMID Version=\"1\">{pmid}</PMID><Article>\
         <PublicationTypeList><PublicationType UI=\"D016428\">Journal Article</PublicationType>\
         </PublicationTypeList></Article></MedlineCitation></PubmedArticle>"
    )
}

async fn answer(State(stand_in): State<Arc<StandIn>>, body: String) -> Response {
    stand_in.arrivals.lock().unwrap().push(Instant::now());
    if stand_in.refuse {
        return (StatusCode::BAD_REQUEST, "{\"error\":\"API key invalid\"}").into_response();
    }
    tokio::time::sleep(stand_in.latency).await;
    let ids = body
        .split('&')
        .find_map(|pair| pair.strip_prefix("id="))
        .unwrap_or_default()
        .replace("%2C", ",");
    let records: String = ids.split(',').map(record).collect();
    (
        [(header::CONTENT_TYPE, "text/xml")],
        format!("<?xml version=\"1.0\" ?><PubmedArticleSet>{records}</PubmedArticleSet>"),
    )
        .into_response()
}

async fn stand_in(latency: Duration, refuse: bool) -> (Eutils, Arc<StandIn>) {
    let stand_in = Arc::new(StandIn {
        latency,
        refuse,
        arrivals: Mutex::default(),
    });
    let app = Router::new()
        .route("/entrez/eutils/efetch.fcgi", post(answer))
        .with_state(Arc::clone(&stand_in));
    let listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
    listener.set_nonblocking(true).unwrap();
    let address = listener.local_addr().unwrap();
    std::thread::spawn(move || {
        let runtime = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap();
        runtime.block_on(async move {
            let listener = tokio::net::TcpListener::from_std(listener).unwrap();
            axum::serve(listener, app).await.unwrap();
        });
    });
    let base = format!("http://{address}/entrez/eutils/");
    let pacing = Pacing::ncbi(true);
    let client = Eutils::new(&base, "engineering@example.org", Some("test-key"), pacing).unwrap();
    (client, stand_in)
}

fn pmids(requests: usize) -> Vec<Pmid> {
    (1..=requests * BATCH)
        .map(|value| Pmid::new(value as u64).unwrap())
        .collect()
}

fn slot() -> SlotName {
    SlotName::new("pubmed-status").unwrap()
}

/// What a timed night showed.
struct Timed {
    requests: usize,
    elapsed: Duration,
    closest: Duration,
    done: usize,
}

/// Runs one night over `requests` full batches against the stand-in, and
/// times it.
async fn timed_night(requests: usize) -> Timed {
    let scratch = Scratch::new().await;
    let store = scratch.store().await;
    let (client, stand_in) = stand_in(LATENCY, false).await;
    let lease = hold_slot(&store, &slot(), RunId::new()).await;
    let list = pmids(requests);
    let started = Instant::now();
    let report = pubmed::refresh(&store, &lease, &client, &list)
        .await
        .unwrap();
    let elapsed = started.elapsed();
    assert!(complete(&store, lease).await.success().is_some());
    let closest = stand_in.gaps().into_iter().min().unwrap_or(Duration::MAX);
    let timed = Timed {
        requests: stand_in.requests(),
        elapsed,
        closest,
        done: report.done.len(),
    };
    println!(
        "{} requests for {} PMIDs in {:.1} s; closest request starts {:?} apart",
        timed.requests,
        timed.done,
        timed.elapsed.as_secs_f64(),
        timed.closest
    );
    scratch.drop().await;
    timed
}

/// The limit, less 2 ms for jitter in the stand-in noticing an arrival. The
/// limiter counts each interval from a request's actual start, so arrivals
/// come no closer than the limit.
const CLOSEST_ALLOWED: Duration = Duration::from_millis(98);

/// AC12: 40 requests in flight at once on one client share its limiter: they
/// start 100 ms apart and all finish in about 40 × 100 ms plus one request's
/// latency, not 40 × 1.3 s.
#[tokio::test]
async fn one_limiter_paces_every_request_in_flight() {
    let (client, stand_in) = stand_in(LATENCY, false).await;
    let client = Arc::new(client);
    let batches: Vec<Vec<Pmid>> = pmids(40).chunks(BATCH).map(<[Pmid]>::to_vec).collect();
    let started = Instant::now();
    let mut requests = tokio::task::JoinSet::new();
    for batch in batches {
        let client = Arc::clone(&client);
        requests.spawn(async move { client.efetch(&batch).await.map(|body| body.len()) });
    }
    while let Some(answered) = requests.join_next().await {
        assert!(answered.unwrap().unwrap() > 0);
    }
    let elapsed = started.elapsed();
    let gaps = stand_in.gaps();
    let closest = gaps.iter().min().copied().unwrap();
    let span: Duration = gaps.iter().sum();
    println!(
        "40 requests at once: all answered in {:.2} s; starts span {:.2} s, closest {closest:?} apart",
        elapsed.as_secs_f64(),
        span.as_secs_f64()
    );
    assert_eq!(stand_in.requests(), 40);
    assert!(closest >= CLOSEST_ALLOWED, "requests {closest:?} apart");
    let paced = Duration::from_millis(100) * 40 + LATENCY;
    assert!(
        elapsed < paced + Duration::from_secs(1),
        "took {elapsed:?}: paced is {paced:?}, one at a time {:?}",
        LATENCY * 40
    );
}

/// AC12: a night over 40 batches overlaps its requests within the same
/// pacing, so it takes a fraction of the 52 s that one request at a time
/// would. The rest of its time is the night's writes.
#[tokio::test]
async fn the_refresh_overlaps_its_requests() {
    let timed = timed_night(40).await;
    assert_eq!(timed.requests, 40);
    assert_eq!(timed.done, 40 * BATCH);
    assert!(
        timed.closest >= CLOSEST_ALLOWED,
        "requests {:?} apart",
        timed.closest
    );
    let serial = LATENCY * 40;
    assert!(
        timed.elapsed < serial / 3,
        "took {:?}: one at a time would be {serial:?}",
        timed.elapsed
    );
}

/// A refusal fails the night and stops the requests still to come: it
/// doesn't wait out the rest of the list.
#[tokio::test]
async fn a_refusal_stops_the_requests_in_flight() {
    let scratch = Scratch::new().await;
    let store = scratch.store().await;
    let (client, stand_in) = stand_in(LATENCY, true).await;
    let lease = hold_slot(&store, &slot(), RunId::new()).await;
    let error = pubmed::refresh(&store, &lease, &client, &pmids(200))
        .await
        .unwrap_err();
    assert!(matches!(error, RefreshError::Refused(_)));
    // At 10 a second, 200 requests would take 20 s; a refusal stops them
    // within the requests already started.
    let sent = stand_in.requests();
    assert!(
        sent < 60,
        "{sent} of 200 requests were sent after a refusal"
    );
    scratch.drop().await;
}

/// AC12: a whole night of about 1.25 million PMIDs finishes in under an
/// hour. Ignored by default because it takes about a quarter of an hour.
#[tokio::test]
#[ignore = "a full night of 6,250 requests takes about a quarter of an hour"]
async fn a_full_night_finishes_within_an_hour() {
    let timed = timed_night(6250).await;
    assert_eq!(timed.done, 6250 * BATCH);
    assert!(timed.closest >= CLOSEST_ALLOWED);
    assert!(
        timed.elapsed < Duration::from_secs(3600),
        "{:?}",
        timed.elapsed
    );
}
