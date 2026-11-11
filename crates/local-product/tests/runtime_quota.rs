//! Real-Postgres proof of the physician product's shared provider cooldown.
use std::time::{Duration, Instant};

use harness::provider::QuotaGate;
use local_product::runtime::Store;

#[tokio::test]
#[ignore = "requires explicit local Postgres"]
async fn provider_cooldown_is_shared_monotonic_and_survives_reconnection() {
    let url = std::env::var("PHYSICIAN_RUNTIME_TEST_DATABASE_URL")
        .expect("explicit local Postgres URL required");
    let first = Store::connect(&url).await.unwrap();
    first.migrate().await.unwrap();
    let second = Store::connect(&url).await.unwrap();
    let started = Instant::now();
    let (short, long) = tokio::join!(
        first.defer(Duration::from_secs(1)),
        second.defer(Duration::from_secs(3))
    );
    short.unwrap();
    long.unwrap();
    let wait = first.remaining().await.unwrap();
    assert!(
        wait >= Duration::from_secs(3)
            .saturating_sub(started.elapsed() + Duration::from_millis(25)),
        "a shorter concurrent deferral overwrote the longer one: {wait:?}"
    );
    second.defer(Duration::from_millis(1)).await.unwrap();
    // Explicit schema reruns must preserve a provider's outstanding cooldown.
    first.migrate().await.unwrap();
    drop(first);
    drop(second);

    let reopened = Store::connect(&url).await.unwrap();
    let wait = reopened.remaining().await.unwrap();
    assert!(
        wait >= Duration::from_secs(3)
            .saturating_sub(started.elapsed() + Duration::from_millis(25)),
        "reconnect or migration shortened the shared cooldown: {wait:?}"
    );
    println!(
        "Chords quota cross-connection readback: remaining={wait:?}, elapsed={:?}",
        started.elapsed()
    );
    // Leave the shared proof database ready for later product proofs. The
    // timestamp expires by its database clock; the test never resets the row.
    tokio::time::sleep(wait + Duration::from_millis(25)).await;
    assert_eq!(reopened.remaining().await.unwrap(), Duration::ZERO);
}
