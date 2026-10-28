//! The shared harness against the real Pub/Sub emulator.
//!
//! Run from the Nix dev shell: `cargo test -p pubsub --test emulator`.
//! Without `gcloud` the tests return; `CODA_REQUIRE_POSTGRES_TESTS` makes them fail.
use pubsub::testing::{Emulator, Outgoing, Subscription};

const PROJECT: &str = "coda-local-pubsub";

fn unique(label: &str) -> String {
    format!("{label}-{}", std::process::id())
}

#[tokio::test(flavor = "multi_thread")]
async fn rest_publish_pull_nack_and_ack_round_trip() {
    let Some(emulator) = Emulator::start(PROJECT).await else {
        return;
    };
    let topic_name = unique("round-trip");
    let topic = emulator.create_topic(&topic_name).await;
    let subscription = emulator
        .create_subscription(&topic_name, &Subscription::pull(&topic_name))
        .await;
    let ids = emulator
        .publish(&topic, &[Outgoing::data(b"one".to_vec())])
        .await;
    assert_eq!(ids.len(), 1);

    let first = emulator.pull(&subscription, 10).await;
    assert_eq!(first.len(), 1, "{}", emulator.tail());
    assert_eq!(first[0].message.data, b"one");
    assert_eq!(first[0].message.message_id, ids[0]);

    // A nack redelivers the same message id.
    emulator
        .modify_ack_deadline(&subscription, &[first[0].ack_id.clone()], 0)
        .await;
    let again = wait_pull(&emulator, &subscription).await;
    assert_eq!(again[0].message.message_id, ids[0]);

    emulator
        .ack(&subscription, &[again[0].ack_id.clone()])
        .await;
    assert!(emulator.pull(&subscription, 10).await.is_empty());
}

#[tokio::test(flavor = "multi_thread")]
async fn the_official_publisher_reaches_the_emulator_without_credentials() {
    let Some(emulator) = Emulator::start(PROJECT).await else {
        return;
    };
    let topic_name = unique("publisher");
    let topic = emulator.create_topic(&topic_name).await;
    let subscription = emulator
        .create_subscription(&topic_name, &Subscription::pull(&topic_name))
        .await;
    let publisher = emulator.publisher(&topic).await;
    let id = publisher
        .publish(google_cloud_pubsub::model::Message::new().set_data(b"from rust".to_vec()))
        .await
        .expect("publish through the official client");
    let received = wait_pull(&emulator, &subscription).await;
    assert_eq!(received[0].message.message_id, id);
    assert_eq!(received[0].message.data, b"from rust");
}

async fn wait_pull(emulator: &Emulator, subscription: &str) -> Vec<pubsub::testing::Received> {
    let deadline = std::time::Instant::now() + std::time::Duration::from_secs(20);
    loop {
        let received = emulator.pull(subscription, 10).await;
        if !received.is_empty() {
            return received;
        }
        assert!(
            std::time::Instant::now() < deadline,
            "nothing delivered on {subscription}: {}",
            emulator.tail()
        );
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    }
}
