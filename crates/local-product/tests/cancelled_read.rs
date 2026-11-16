//! A display read the browser cancels, against real Postgres and the
//! process's real pool. Run explicitly with PHYSICIAN_RUNTIME_TEST_DATABASE_URL.
//!
//! The page cancels a read it no longer needs, and the site then drops that
//! request's handler wherever it is waiting. Each test drops a `Store::read`
//! while its `BEGIN` has reached the server and its answer has not come
//! back, then uses the pool as the next requests do.
use std::future::Future;
use std::task::Poll;
use std::time::Duration;

use local_product::runtime::{Scope, Store};
use physician_auth::{AuthStore, Membership, PhysicianIdentity, PostgresAuthStore};
use sea_orm::sea_query::{Alias, Expr, ExprTrait, Query};
use sea_orm::{ConnectionTrait, Database, DatabaseConnection};

/// The pool's sessions as the server sees them: `(state, last statement)`.
async fn sessions(observer: &DatabaseConnection, application: &str) -> Vec<(String, String)> {
    let query = Query::select()
        .column(Alias::new("state"))
        .column(Alias::new("query"))
        .from(Alias::new("pg_stat_activity"))
        .and_where(Expr::col(Alias::new("application_name")).eq(application))
        .to_owned();
    observer
        .query_all(&query)
        .await
        .expect("read the pool's sessions")
        .into_iter()
        .map(|row| {
            (
                row.try_get::<String>("", "state")
                    .expect("decode the session's state"),
                row.try_get::<String>("", "query")
                    .expect("decode the session's query"),
            )
        })
        .collect()
}

/// A store on its own tagged pool, a second pool that watches it, and the
/// auth owner on the store's pool, as the site shares one pool.
async fn open() -> (Store, PostgresAuthStore, DatabaseConnection, Scope, String) {
    let url = std::env::var("PHYSICIAN_RUNTIME_TEST_DATABASE_URL").unwrap();
    let application = format!("cancelled-read-{}", uuid::Uuid::new_v4().simple());
    let separator = if url.contains('?') { '&' } else { '?' };
    let tagged = format!("{url}{separator}application_name={application}");
    PostgresAuthStore::connect_with_migrations(&url)
        .await
        .unwrap();
    let store = Store::connect(&tagged).await.unwrap();
    store.migrate().await.unwrap();
    let auth = PostgresAuthStore::from_connection(store.connection().clone());
    let observer = Database::connect(url.as_str()).await.unwrap();
    let scope = Scope {
        practice: uuid::Uuid::new_v4().to_string(),
        physician: "physician".into(),
        workspace: format!("pw-{}", uuid::Uuid::new_v4()),
        chat: format!("chat-{}", uuid::Uuid::new_v4()),
    };
    (store, auth, observer, scope, application)
}

/// Polls a display read until its `BEGIN` has run on the server, then drops
/// it there, as a dropped request handler does. Returns what the server shows
/// once the dropped connection is back in the pool.
async fn cancel_read_after_begin(
    store: &Store,
    observer: &DatabaseConnection,
    application: &str,
    scope: &Scope,
) -> Vec<(String, String)> {
    let mut read = Box::pin(store.read(scope));
    let mut polls = 0;
    loop {
        let step = std::future::poll_fn(|cx| Poll::Ready(read.as_mut().poll(cx))).await;
        assert!(
            step.is_pending(),
            "the read answered before it could be cancelled"
        );
        // The read is not polled while it waits: its BEGIN, once sent, runs on
        // the server and its answer stays unread.
        tokio::time::sleep(Duration::from_millis(100)).await;
        let seen = sessions(observer, application).await;
        if seen
            .iter()
            .any(|(state, query)| state == "idle in transaction" && query == "BEGIN")
        {
            break;
        }
        polls += 1;
        assert!(polls < 50, "the read never reached its BEGIN: {seen:?}");
    }
    drop(read);
    // The pool tests a returned connection in a task of its own. Wait for it,
    // at most 5 s, until no session is left inside a transaction.
    let mut seen = sessions(observer, application).await;
    for _ in 0..50 {
        if !seen.iter().any(|(state, _)| state == "idle in transaction") {
            break;
        }
        tokio::time::sleep(Duration::from_millis(100)).await;
        seen = sessions(observer, application).await;
    }
    seen
}

#[tokio::test]
#[ignore = "requires explicit local Postgres"]
async fn a_cancelled_display_read_leaves_the_next_read_working() {
    let (store, auth, observer, scope, application) = open().await;
    let returned = cancel_read_after_begin(&store, &observer, &application, &scope).await;
    assert!(
        !returned
            .iter()
            .any(|(state, _)| state == "idle in transaction"),
        "the pool took the cancelled read's connection back inside its transaction: {returned:?}",
    );
    // Every guarded request first notes the session's activity: a plain read
    // on the shared pool, outside any transaction.
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    auth.note_physician_activity("no-such-session", now)
        .await
        .unwrap();
    let after_plain = sessions(&observer, &application).await;
    let next = store.read(&scope).await;
    assert!(
        next.is_ok(),
        "the next display read failed: {}\n  pool after the cancel: {returned:?}\n  pool after a plain read: {after_plain:?}",
        next.err().map(|e| e.to_string()).unwrap_or_default(),
    );
}

#[tokio::test]
#[ignore = "requires explicit local Postgres"]
async fn a_cancelled_display_read_keeps_no_later_write_uncommitted() {
    let (store, auth, observer, scope, application) = open().await;
    let returned = cancel_read_after_begin(&store, &observer, &application, &scope).await;
    let physician = PhysicianIdentity {
        practice_id: scope.practice.clone(),
        physician_id: scope.physician.clone(),
    };
    // A write on the shared pool outside a transaction, reported as done.
    auth.place_on_panel(&physician, &scope.workspace)
        .await
        .unwrap();
    let after_write = sessions(&observer, &application).await;
    let elsewhere = PostgresAuthStore::from_connection(observer.clone());
    let seen_elsewhere = elsewhere.membership(&physician, &scope.workspace).await;
    let next_read = store
        .read(&scope)
        .await
        .map(|_| ())
        .map_err(|e| e.to_string());
    let kept = elsewhere.membership(&physician, &scope.workspace).await;
    assert!(
        seen_elsewhere == Membership::Allowed && kept == Membership::Allowed,
        "a write the pool reported as done:\n  seen from another connection: {seen_elsewhere:?}\n  \
         the next display read: {next_read:?}\n  kept after it: {kept:?}\n  \
         pool after the cancel: {returned:?}\n  pool after the write: {after_write:?}",
    );
}
