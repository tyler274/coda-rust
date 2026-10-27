//! Gmail push-wake semantics on the real Pub/Sub emulator.
//!
//! Evidence kind: a controlled test of the real `gmail::push` router and
//! `gmail::history` reader, delivered by the real Pub/Sub emulator, against the
//! `gmail::testing::FakeGmail` stand-in for the Gmail API. It does not prove
//! push OIDC (the emulator has no IAM), real Gmail, hosted Pub/Sub, or the
//! product's durable cursor and intake (ENG-7).
//!
//! The only test-owned state is [`Intake`]: a cursor, a seen-message set and an
//! effect log. It stands in for the product's durable intake store, and commits
//! the cursor with compare-and-set the way a stored cursor would be guarded.
//!
//! Run from the Nix dev shell: `cargo test -p gmail --test push_wake`, or under
//! the stack's emulator with
//! `scripts/physician-scale-stack.py --dependencies-only -- cargo test -p gmail --test push_wake`.
//! Without `gcloud` and without `PUBSUB_EMULATOR_HOST` the tests return;
//! `CODA_REQUIRE_POSTGRES_TESTS` makes them fail instead.
use std::collections::HashSet;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use gmail::history::{Cursor, History, ListError};
use gmail::push::Routes;
use gmail::testing::{self, FakeGmail};
use pubsub::testing::{Emulator, Outgoing, Subscription};

const PROJECT: &str = "coda-local-gmail";
const CLINIC: &str = "Clinic@CodaHealth.test";

/// Stands in for the product's durable intake store (ENG-7).
#[derive(Default)]
struct Intake {
    cursor: Option<Cursor>,
    seen: HashSet<String>,
    effects: Vec<String>,
    expired: bool,
}

/// One connected mailbox: the real reader and the stand-in store.
struct Connection {
    fake: FakeGmail,
    history: History,
    intake: Mutex<Intake>,
    /// Pages applied so far, across every wake.
    pages: AtomicUsize,
    /// Crash after applying this page number, before committing the cursor.
    crash_after_page: AtomicUsize,
}

#[derive(Debug)]
enum Stop {
    Crash,
    Gmail(ListError),
}

impl Connection {
    /// Connects a mailbox: its first cursor is Gmail's current history id.
    async fn connect() -> Arc<Self> {
        Self::connect_with(|endpoints| endpoints).await
    }

    /// [`connect`](Self::connect) with the reader's endpoints rewritten.
    async fn connect_with(rewrite: impl FnOnce(gmail::Endpoints) -> gmail::Endpoints) -> Arc<Self> {
        let fake = testing::spawn(false).await;
        let history = History::refresh(rewrite(fake.endpoints()), "client", "secret", "refresh");
        let cursor = history.first_cursor().await.expect("first cursor");
        Arc::new(Self {
            fake,
            history,
            intake: Mutex::new(Intake {
                cursor: Some(cursor),
                ..Intake::default()
            }),
            pages: AtomicUsize::new(0),
            crash_after_page: AtomicUsize::new(0),
        })
    }

    /// A wake: read from the stored cursor to the last page.
    async fn wake(&self) -> Result<(), Stop> {
        let mut cursor = self
            .intake
            .lock()
            .unwrap()
            .cursor
            .clone()
            .expect("connected");
        loop {
            let page = match self.history.list(&cursor).await {
                Ok(page) => page,
                Err(ListError::CursorExpired) => {
                    self.intake.lock().unwrap().expired = true;
                    return Err(Stop::Gmail(ListError::CursorExpired));
                }
                Err(error) => return Err(Stop::Gmail(error)),
            };
            {
                let mut intake = self.intake.lock().unwrap();
                for id in &page.messages_added {
                    if intake.seen.insert(id.clone()) {
                        intake.effects.push(id.clone());
                    }
                }
            }
            let applied = self.pages.fetch_add(1, Ordering::SeqCst) + 1;
            if self
                .crash_after_page
                .compare_exchange(applied, 0, Ordering::SeqCst, Ordering::SeqCst)
                .is_ok()
            {
                return Err(Stop::Crash);
            }
            {
                let mut intake = self.intake.lock().unwrap();
                // Another delivery already moved the cursor; never move it back.
                if intake.cursor.as_ref() != Some(&cursor) {
                    return Ok(());
                }
                intake.cursor = Some(page.next.clone());
            }
            if !page.has_more {
                return Ok(());
            }
            cursor = page.next;
        }
    }

    fn effects(&self) -> Vec<String> {
        self.intake.lock().unwrap().effects.clone()
    }

    fn starts(&self) -> Vec<u64> {
        self.fake.mailbox.lock().unwrap().starts.clone()
    }

    fn reads(&self) -> usize {
        self.fake.mailbox.lock().unwrap().reads
    }

    fn deliver(&self, ids: &[&str]) -> u64 {
        let mut mailbox = self.fake.mailbox.lock().unwrap();
        for id in ids {
            mailbox.deliver(id);
        }
        mailbox.history_id
    }
}

/// A push endpoint in front of the routed connections.
struct Endpoint {
    routes: Routes<usize>,
    connections: Vec<Arc<Connection>>,
    /// `(message id, data, status answered)` per push delivery.
    deliveries: Mutex<Vec<(String, Vec<u8>, u16)>>,
    /// Answer 500 to this many routed deliveries before handling any.
    refuse: AtomicUsize,
    url: String,
}

impl Endpoint {
    async fn bind(mailboxes: Vec<(&str, Arc<Connection>)>) -> Arc<Self> {
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let url = format!("http://{}/push", listener.local_addr().unwrap());
        let routes = Routes::new(
            mailboxes
                .iter()
                .enumerate()
                .map(|(index, (address, _))| (*address, index)),
        );
        let endpoint = Arc::new(Self {
            routes,
            connections: mailboxes.into_iter().map(|(_, c)| c).collect(),
            deliveries: Mutex::new(Vec::new()),
            refuse: AtomicUsize::new(0),
            url,
        });
        let app = axum::Router::new()
            .route("/push", axum::routing::post(push))
            .with_state(Arc::clone(&endpoint));
        tokio::spawn(async move { axum::serve(listener, app).await.unwrap() });
        endpoint
    }

    fn deliveries_of(&self, data: &[u8]) -> Vec<u16> {
        self.deliveries
            .lock()
            .unwrap()
            .iter()
            .filter(|(_, delivered, _)| delivered == data)
            .map(|(_, _, status)| *status)
            .collect()
    }
}

/// Pub/Sub acks a push on 2xx and redelivers on anything else.
async fn push(
    axum::extract::State(endpoint): axum::extract::State<Arc<Endpoint>>,
    body: axum::body::Bytes,
) -> axum::http::StatusCode {
    use axum::http::StatusCode;
    let Ok(message) = pubsub::envelope::decode(&body) else {
        return StatusCode::NO_CONTENT;
    };
    let status = match endpoint.routes.route(&message) {
        // Unknown mailbox or not Gmail's shape: ack, read nothing.
        None => StatusCode::NO_CONTENT,
        Some(_)
            if endpoint
                .refuse
                .fetch_update(Ordering::SeqCst, Ordering::SeqCst, |n| n.checked_sub(1))
                .is_ok() =>
        {
            StatusCode::INTERNAL_SERVER_ERROR
        }
        Some(index) => match endpoint.connections[*index].wake().await {
            Ok(()) => StatusCode::NO_CONTENT,
            // The product resynchronizes; redelivering the wake cannot help.
            Err(Stop::Gmail(ListError::CursorExpired)) => StatusCode::NO_CONTENT,
            Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
        },
    };
    endpoint
        .deliveries
        .lock()
        .unwrap()
        .push((message.message_id, message.data, status.as_u16()));
    status
}

fn wake(address: &str, history_id: &str) -> Vec<u8> {
    serde_json::to_vec(&serde_json::json!({"emailAddress": address, "historyId": history_id}))
        .unwrap()
}

fn unique(label: &str) -> String {
    format!("{label}-{}", std::process::id())
}

async fn eventually(what: &str, emulator: &Emulator, mut done: impl FnMut() -> bool) {
    let deadline = Instant::now() + Duration::from_secs(45);
    while !done() {
        assert!(
            Instant::now() < deadline,
            "timed out: {what}\n{}",
            emulator.tail()
        );
        tokio::time::sleep(Duration::from_millis(50)).await;
    }
}

/// A topic and a push subscription to `endpoint` that retries after 1-2 s.
async fn push_topic(emulator: &Emulator, label: &str, endpoint: &Endpoint) -> String {
    let name = unique(label);
    let topic = emulator.create_topic(&name).await;
    emulator
        .create_subscription(&name, &Subscription::push(&name, &endpoint.url).retry(1, 2))
        .await;
    topic
}

#[tokio::test(flavor = "multi_thread")]
async fn a_push_is_a_wake_only() {
    let Some(emulator) = Emulator::start(PROJECT).await else {
        return;
    };
    let clinic = Connection::connect().await;
    let endpoint = Endpoint::bind(vec![(CLINIC, Arc::clone(&clinic))]).await;
    let topic = push_topic(&emulator, "wake-only", &endpoint).await;
    let first = clinic.starts();
    assert!(first.is_empty());
    let connected_at = clinic.fake.mailbox.lock().unwrap().history_id;
    let latest = clinic.deliver(&["m1"]);

    // A historyId far ahead moves nothing by itself.
    let ahead = wake("clinic@codahealth.TEST", "999999999");
    emulator
        .publish(&topic, &[Outgoing::data(ahead.clone())])
        .await;
    eventually("ahead wake handled", &emulator, || {
        endpoint.deliveries_of(&ahead) == [204]
    })
    .await;
    assert_eq!(clinic.effects(), ["m1"]);
    assert_eq!(clinic.starts(), [connected_at]);

    // Unknown mailbox and malformed payloads: acked, nothing read, nothing changed.
    let reads = clinic.reads();
    let strangers = [
        wake("someone-else@codahealth.test", &latest.to_string()),
        br#"{"emailAddress":"clinic@codahealth.test","historyId":17}"#.to_vec(),
        b"not json".to_vec(),
    ];
    let messages: Vec<Outgoing> = strangers.iter().cloned().map(Outgoing::data).collect();
    emulator.publish(&topic, &messages).await;
    eventually("strangers acked", &emulator, || {
        strangers
            .iter()
            .all(|data| !endpoint.deliveries_of(data).is_empty())
    })
    .await;
    // A refused push would come back within the 1-2 s retry backoff.
    tokio::time::sleep(Duration::from_secs(3)).await;
    for data in &strangers {
        assert_eq!(endpoint.deliveries_of(data), [204], "redelivered");
    }
    assert_eq!(clinic.reads(), reads, "a stranger's push read the mailbox");
    assert_eq!(clinic.effects(), ["m1"]);

    // The next wake reads from where history said, not where a push said.
    let next = wake(CLINIC, "1");
    emulator
        .publish(&topic, &[Outgoing::data(next.clone())])
        .await;
    eventually("next wake", &emulator, || {
        endpoint.deliveries_of(&next) == [204]
    })
    .await;
    assert_eq!(clinic.starts(), [connected_at, latest]);
}

#[tokio::test(flavor = "multi_thread")]
async fn duplicate_and_reordered_wakes_apply_each_message_once() {
    let Some(emulator) = Emulator::start(PROJECT).await else {
        return;
    };
    let clinic = Connection::connect().await;
    // Concurrent readers of one stored cursor overlap, like two worker instances.
    clinic.fake.mailbox.lock().unwrap().read_delay = Duration::from_millis(300);
    let routes = Routes::new([(CLINIC, 0usize)]);
    let name = unique("duplicates");
    let topic = emulator.create_topic(&name).await;
    let subscription = emulator
        .create_subscription(&name, &Subscription::pull(&name))
        .await;

    let first = clinic.deliver(&["m1"]).to_string();
    let same = wake(CLINIC, &first);
    emulator
        .publish(
            &topic,
            &[Outgoing::data(same.clone()), Outgoing::data(same)],
        )
        .await;
    let both = pull_at_least(&emulator, &subscription, 2).await;
    assert_eq!(both.len(), 2);
    assert!(both.iter().all(|r| routes.route(&r.message).is_some()));
    let (one, two) = tokio::join!(clinic.wake(), clinic.wake());
    assert!(one.is_ok() && two.is_ok(), "{one:?} {two:?}");
    ack_all(&emulator, &subscription, &both).await;
    assert_eq!(clinic.effects(), ["m1"], "a duplicate wake applied twice");

    // A newer wake, then the stale one after it.
    let second = clinic.deliver(&["m2"]).to_string();
    emulator
        .publish(
            &topic,
            &[
                Outgoing::data(wake(CLINIC, &second)),
                Outgoing::data(wake(CLINIC, &first)),
            ],
        )
        .await;
    let later = pull_at_least(&emulator, &subscription, 2).await;
    for received in &later {
        assert!(routes.route(&received.message).is_some());
        clinic.wake().await.expect("wake");
    }
    ack_all(&emulator, &subscription, &later).await;
    assert_eq!(clinic.effects(), ["m1", "m2"]);
    let starts = clinic.starts();
    assert!(
        starts.windows(2).all(|pair| pair[0] <= pair[1]),
        "the cursor moved backward: {starts:?}"
    );
    assert_eq!(starts.last().map(u64::to_string), Some(second));
}

#[tokio::test(flavor = "multi_thread")]
async fn a_refused_push_is_redelivered_and_a_crash_before_commit_rereads_the_page() {
    let Some(emulator) = Emulator::start(PROJECT).await else {
        return;
    };
    let clinic = Connection::connect().await;
    clinic.fake.mailbox.lock().unwrap().page_size = 1;
    let endpoint = Endpoint::bind(vec![(CLINIC, Arc::clone(&clinic))]).await;
    let topic = push_topic(&emulator, "redelivery", &endpoint).await;

    // The endpoint answers 500 once: Pub/Sub redelivers the same message.
    endpoint.refuse.store(1, Ordering::SeqCst);
    let history_id = clinic.deliver(&["m1"]).to_string();
    let refused = wake(CLINIC, &history_id);
    emulator
        .publish(&topic, &[Outgoing::data(refused.clone())])
        .await;
    eventually("refused wake redelivered", &emulator, || {
        endpoint.deliveries_of(&refused) == [500, 204]
    })
    .await;
    assert_eq!(clinic.effects(), ["m1"]);

    // Page 2 of 3 is applied, then the process dies before storing the cursor.
    let pages = clinic.pages.load(Ordering::SeqCst);
    clinic.crash_after_page.store(pages + 2, Ordering::SeqCst);
    let history_id = clinic.deliver(&["m2", "m3", "m4"]).to_string();
    let crashed = wake(CLINIC, &history_id);
    emulator
        .publish(&topic, &[Outgoing::data(crashed.clone())])
        .await;
    eventually("crashed wake redelivered", &emulator, || {
        endpoint.deliveries_of(&crashed) == [500, 204]
    })
    .await;
    assert_eq!(clinic.effects(), ["m1", "m2", "m3", "m4"]);
}

#[tokio::test(flavor = "multi_thread")]
async fn a_nacked_pull_is_redelivered_without_repeating_effects() {
    let Some(emulator) = Emulator::start(PROJECT).await else {
        return;
    };
    let clinic = Connection::connect().await;
    clinic.fake.mailbox.lock().unwrap().page_size = 1;
    let routes = Routes::new([(CLINIC, 0usize)]);
    let name = unique("nack");
    let topic = emulator.create_topic(&name).await;
    let subscription = emulator
        .create_subscription(&name, &Subscription::pull(&name))
        .await;

    let history_id = clinic.deliver(&["m1", "m2", "m3"]).to_string();
    let ids = emulator
        .publish(&topic, &[Outgoing::data(wake(CLINIC, &history_id))])
        .await;
    clinic.crash_after_page.store(2, Ordering::SeqCst);

    let first = pull_at_least(&emulator, &subscription, 1).await;
    assert!(routes.route(&first[0].message).is_some());
    assert!(matches!(clinic.wake().await, Err(Stop::Crash)));
    emulator
        .modify_ack_deadline(&subscription, &[first[0].ack_id.clone()], 0)
        .await;

    let again = pull_at_least(&emulator, &subscription, 1).await;
    assert_eq!(again[0].message.message_id, ids[0]);
    clinic.wake().await.expect("redelivered wake");
    ack_all(&emulator, &subscription, &again).await;
    assert_eq!(clinic.effects(), ["m1", "m2", "m3"]);
    assert!(emulator.pull(&subscription, 10).await.is_empty());
}

#[tokio::test(flavor = "multi_thread")]
async fn a_poison_wake_dead_letters_while_healthy_wakes_complete() {
    let Some(emulator) = Emulator::start(PROJECT).await else {
        return;
    };
    let clinic = Connection::connect().await;
    // A mailbox whose history read always fails: nothing listens there.
    let broken = Connection::connect_with(|endpoints| gmail::Endpoints {
        history: "http://127.0.0.1:9/gmail/v1/users/me/history".into(),
        ..endpoints
    })
    .await;
    let endpoint = Endpoint::bind(vec![
        (CLINIC, Arc::clone(&clinic)),
        ("broken@codahealth.test", Arc::clone(&broken)),
    ])
    .await;
    let name = unique("dead-letter");
    let dead = format!("{name}-dead-letter");
    let topic = emulator.create_topic(&name).await;
    emulator.create_topic(&dead).await;
    let dead_subscription = emulator
        .create_subscription(&dead, &Subscription::pull(&dead))
        .await;
    emulator
        .create_subscription(
            &name,
            &Subscription::push(&name, &endpoint.url)
                .retry(1, 2)
                .dead_letter(&dead, 5),
        )
        .await;

    let poison = wake("broken@codahealth.test", "5");
    let healthy = wake(CLINIC, &clinic.deliver(&["m1"]).to_string());
    emulator
        .publish(
            &topic,
            &[
                Outgoing::data(poison.clone()),
                Outgoing::data(healthy.clone()),
            ],
        )
        .await;
    eventually("healthy wake done", &emulator, || {
        endpoint.deliveries_of(&healthy) == [204]
    })
    .await;
    assert_eq!(clinic.effects(), ["m1"]);

    let deadline = Instant::now() + Duration::from_secs(60);
    let forwarded = loop {
        let received = emulator.pull(&dead_subscription, 10).await;
        if !received.is_empty() {
            break received;
        }
        assert!(
            Instant::now() < deadline,
            "poison never dead-lettered: {:?}\n{}",
            endpoint.deliveries_of(&poison),
            emulator.tail()
        );
        tokio::time::sleep(Duration::from_millis(200)).await;
    };
    assert_eq!(forwarded.len(), 1);
    assert_eq!(forwarded[0].message.data, poison);
    let attempts = endpoint.deliveries_of(&poison);
    assert!(
        attempts.len() >= 5 && attempts.iter().all(|status| *status == 500),
        "{attempts:?}"
    );
    assert!(broken.effects().is_empty());

    // Healthy work keeps flowing after the poison left.
    let after = wake(CLINIC, &clinic.deliver(&["m2"]).to_string());
    emulator
        .publish(&topic, &[Outgoing::data(after.clone())])
        .await;
    eventually("wake after dead letter", &emulator, || {
        endpoint.deliveries_of(&after) == [204]
    })
    .await;
    assert_eq!(clinic.effects(), ["m1", "m2"]);
}

#[tokio::test(flavor = "multi_thread")]
async fn the_rust_publisher_keeps_one_ordering_key_in_publish_order() {
    let Some(emulator) = Emulator::start(PROJECT).await else {
        return;
    };
    let name = unique("ordering");
    let topic = emulator.create_topic(&name).await;
    let subscription = emulator
        .create_subscription(&name, &Subscription::pull(&name).ordered())
        .await;
    let publisher = emulator.publisher(&topic).await;
    let sent: Vec<Vec<u8>> = (1..=8)
        .map(|n| wake(CLINIC, &(1000 + n).to_string()))
        .collect();
    let handles: Vec<_> = sent
        .iter()
        .map(|data| {
            publisher.publish(
                google_cloud_pubsub::model::Message::new()
                    .set_data(data.clone())
                    .set_ordering_key("clinic@codahealth.test"),
            )
        })
        .collect();
    for handle in handles {
        handle.await.expect("ordered publish");
    }

    let mut received = Vec::new();
    let deadline = Instant::now() + Duration::from_secs(30);
    while received.len() < sent.len() {
        assert!(Instant::now() < deadline, "only {}", received.len());
        let batch = emulator.pull(&subscription, 10).await;
        for message in &batch {
            assert_eq!(
                message.message.ordering_key.as_deref(),
                Some("clinic@codahealth.test")
            );
            received.push(message.message.data.clone());
        }
        ack_all(&emulator, &subscription, &batch).await;
        if batch.is_empty() {
            tokio::time::sleep(Duration::from_millis(100)).await;
        }
    }
    assert_eq!(received, sent);
}

#[tokio::test(flavor = "multi_thread")]
async fn an_expired_cursor_is_surfaced_and_the_wake_is_not_retried() {
    let Some(emulator) = Emulator::start(PROJECT).await else {
        return;
    };
    let clinic = Connection::connect().await;
    let endpoint = Endpoint::bind(vec![(CLINIC, Arc::clone(&clinic))]).await;
    let topic = push_topic(&emulator, "expired", &endpoint).await;
    let history_id = {
        let mut mailbox = clinic.fake.mailbox.lock().unwrap();
        mailbox.deliver("m1");
        mailbox.expire_through_current();
        mailbox.history_id.to_string()
    };
    let expired = wake(CLINIC, &history_id);
    emulator
        .publish(&topic, &[Outgoing::data(expired.clone())])
        .await;
    eventually("expired wake handled", &emulator, || {
        endpoint.deliveries_of(&expired) == [204]
    })
    .await;
    assert!(clinic.intake.lock().unwrap().expired);
    assert!(clinic.effects().is_empty(), "read past an expired cursor");
}

async fn pull_at_least(
    emulator: &Emulator,
    subscription: &str,
    count: usize,
) -> Vec<pubsub::testing::Received> {
    let mut received = Vec::new();
    let deadline = Instant::now() + Duration::from_secs(30);
    while received.len() < count {
        assert!(
            Instant::now() < deadline,
            "pulled {} of {count}\n{}",
            received.len(),
            emulator.tail()
        );
        received.extend(emulator.pull(subscription, 10).await);
        if received.len() < count {
            tokio::time::sleep(Duration::from_millis(100)).await;
        }
    }
    received
}

async fn ack_all(emulator: &Emulator, subscription: &str, received: &[pubsub::testing::Received]) {
    if received.is_empty() {
        return;
    }
    let ids: Vec<String> = received.iter().map(|r| r.ack_id.clone()).collect();
    emulator.ack(subscription, &ids).await;
}
