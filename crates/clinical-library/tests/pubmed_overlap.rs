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
//! - `the_refresh_overlaps_its_requests`: a night over 40 batches has many
//!   requests in flight at once, within the same pacing.
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

/// NCBI's pacing with a key: one request start every 100 ms.
const PACE: Duration = Duration::from_millis(100);

/// How far a run of arrivals may come in ahead of the pace: less than one
/// slot, for one arrival noted late. Two requests started together, or a
/// client 10% faster than the pace over 40 requests (390 ms), fail.
const AHEAD_ALLOWED: Duration = Duration::from_millis(75);

/// A stand-in for NCBI that answers every request after `latency`.
struct StandIn {
    latency: Duration,
    refuse: bool,
    arrivals: Mutex<Vec<Instant>>,
    in_flight: Mutex<InFlight>,
}

/// How many requests the stand-in is answering now, and the most at once.
#[derive(Default)]
struct InFlight {
    now: usize,
    peak: usize,
}

impl StandIn {
    /// The most requests the stand-in was answering at once.
    fn peak_in_flight(&self) -> usize {
        self.in_flight.lock().unwrap().peak
    }

    /// From the first request's arrival to the last one's.
    fn span(&self) -> Duration {
        let arrivals = self.arrivals();
        match (arrivals.first(), arrivals.last()) {
            (Some(first), Some(last)) => *last - *first,
            _ => Duration::ZERO,
        }
    }

    /// The most any run of arrivals came in faster than one start per
    /// `PACE`: over every earlier arrival and later one, how much less than
    /// `PACE` per start lies between them. The stand-in notes an arrival only
    /// once the request has reached it, so one arrival noted late brings the
    /// next one close by that lateness alone; a client faster than the pace
    /// falls further ahead with every request, and a burst after a pause is
    /// measured from the burst's own start, not the time the pause left
    /// over.
    fn ahead_of_pace(&self) -> Duration {
        let arrivals = self.arrivals();
        let mut ahead = Duration::ZERO;
        for (earlier, from) in arrivals.iter().enumerate() {
            for (starts, to) in (1u32..).zip(&arrivals[earlier + 1..]) {
                ahead = ahead.max((PACE * starts).saturating_sub(*to - *from));
            }
        }
        ahead
    }

    /// Every arrival, in order.
    fn arrivals(&self) -> Vec<Instant> {
        let mut arrivals = self.arrivals.lock().unwrap().clone();
        arrivals.sort();
        arrivals
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
    {
        let mut in_flight = stand_in.in_flight.lock().unwrap();
        in_flight.now += 1;
        in_flight.peak = in_flight.peak.max(in_flight.now);
    }
    tokio::time::sleep(stand_in.latency).await;
    stand_in.in_flight.lock().unwrap().now -= 1;
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
        in_flight: Mutex::default(),
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
    ahead: Duration,
    span: Duration,
    peak_in_flight: usize,
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
    let timed = Timed {
        requests: stand_in.requests(),
        elapsed,
        ahead: stand_in.ahead_of_pace(),
        span: stand_in.span(),
        peak_in_flight: stand_in.peak_in_flight(),
        done: report.done.len(),
    };
    println!(
        "{} requests for {} PMIDs in {:.1} s; starts span {:.2} s, at most {:?} ahead of pace; at most {} in flight",
        timed.requests,
        timed.done,
        timed.elapsed.as_secs_f64(),
        timed.span.as_secs_f64(),
        timed.ahead,
        timed.peak_in_flight
    );
    scratch.drop().await;
    timed
}

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
    let ahead = stand_in.ahead_of_pace();
    println!(
        "40 requests at once: all answered in {:.2} s; starts span {:.2} s, at most {ahead:?} ahead of pace",
        elapsed.as_secs_f64(),
        stand_in.span().as_secs_f64()
    );
    assert_eq!(stand_in.requests(), 40);
    assert!(
        ahead <= AHEAD_ALLOWED,
        "requests ran {ahead:?} ahead of pace"
    );
    let paced = PACE * 40 + LATENCY;
    assert!(
        elapsed < paced + Duration::from_secs(1),
        "took {elapsed:?}: paced is {paced:?}, one at a time {:?}",
        LATENCY * 40
    );
}

/// AC12: a night over 40 batches overlaps its requests within the same
/// pacing: the stand-in answers many at once, never ahead of the pace.
/// The night's wall time also holds its Postgres writes, which a loaded
/// machine slows, so the overlap is read from the stand-in rather than the
/// clock. `one_limiter_paces_every_request_in_flight` times the requests
/// alone.
#[tokio::test]
async fn the_refresh_overlaps_its_requests() {
    let timed = timed_night(40).await;
    assert_eq!(timed.requests, 40);
    assert_eq!(timed.done, 40 * BATCH);
    assert!(
        timed.ahead <= AHEAD_ALLOWED,
        "requests ran {:?} ahead of pace",
        timed.ahead
    );
    // At 100 ms a start and 1.3 s a request, about 13 are in flight at once;
    // one at a time would show 1.
    assert!(
        timed.peak_in_flight >= 8,
        "at most {} requests in flight",
        timed.peak_in_flight
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
    assert!(timed.ahead <= AHEAD_ALLOWED, "{:?} ahead", timed.ahead);
    assert!(
        timed.elapsed < Duration::from_secs(3600),
        "{:?}",
        timed.elapsed
    );
}
