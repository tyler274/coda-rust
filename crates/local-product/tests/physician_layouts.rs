//! Real Postgres proof of the source layout store, on PHYSICIAN_TEST_DATABASE_URL. A missing
//! database fails, never skips. Each test keys its rows under its own source id.
use local_product::physician_layouts::{LayoutKey, LayoutStoreError, Saved};
use local_product::runtime::Store;

async fn store() -> Store {
    let url = std::env::var("PHYSICIAN_TEST_DATABASE_URL")
        .expect("PHYSICIAN_TEST_DATABASE_URL required for the layout store proof");
    let store = Store::connect(&url).await.unwrap();
    store.migrate().await.unwrap();
    store.migrate().await.unwrap();
    store
}

fn source() -> String {
    format!("ehr-{}", uuid::Uuid::new_v4())
}

const LAYOUT: &str = r#"{"record_type":"Observation",  "sections":[]}"#;

#[tokio::test]
async fn a_saved_layout_reads_back_byte_for_byte_under_its_own_key() {
    let store = store().await;
    let source = source();
    let key = LayoutKey {
        source_id: &source,
        record_type: "Observation",
    };
    assert_eq!(store.layout_for(key).await.unwrap(), None);
    let Saved::Stored(saved) = store.save_layout(key, LAYOUT).await.unwrap() else {
        panic!("a first save stores");
    };
    assert_eq!(saved.layout, LAYOUT);
    assert_eq!(saved.sha256, local_product::sha256_hex(LAYOUT.as_bytes()));
    assert!(saved.created_at > 0);
    assert_eq!(store.layout_for(key).await.unwrap(), Some(saved));
    for other in [
        LayoutKey {
            source_id: &source,
            record_type: "Condition",
        },
        LayoutKey {
            source_id: "another-source",
            record_type: "Observation",
        },
    ] {
        assert_eq!(store.layout_for(other).await.unwrap(), None, "{other:?}");
    }
}

#[tokio::test]
async fn saving_the_same_bytes_again_changes_nothing() {
    let store = store().await;
    let source = source();
    let key = LayoutKey {
        source_id: &source,
        record_type: "Observation",
    };
    let Saved::Stored(first) = store.save_layout(key, LAYOUT).await.unwrap() else {
        panic!("a first save stores");
    };
    assert_eq!(
        store.save_layout(key, LAYOUT).await.unwrap(),
        Saved::Unchanged(first.clone())
    );
    assert_eq!(store.layout_for(key).await.unwrap(), Some(first));
}

#[tokio::test]
async fn a_different_layout_for_a_stored_key_is_refused_and_the_first_stays() {
    let store = store().await;
    let source = source();
    let key = LayoutKey {
        source_id: &source,
        record_type: "Observation",
    };
    let Saved::Stored(first) = store.save_layout(key, LAYOUT).await.unwrap() else {
        panic!("a first save stores");
    };
    // The same JSON value with different bytes is a different layout.
    let reformatted = r#"{"record_type":"Observation","sections":[]}"#;
    for different in [
        reformatted,
        r#"{"record_type":"Observation","sections":[{}]}"#,
    ] {
        match store.save_layout(key, different).await {
            Err(LayoutStoreError::Conflict { stored_sha256 }) => {
                assert_eq!(stored_sha256, first.sha256);
            }
            other => panic!("expected a conflict, got {other:?}"),
        }
    }
    assert_eq!(store.layout_for(key).await.unwrap(), Some(first));
}

#[tokio::test(flavor = "multi_thread", worker_threads = 4)]
async fn concurrent_first_saves_settle_on_one_layout() {
    let store = store().await;
    let source = source();
    let mut saves = tokio::task::JoinSet::new();
    for n in 0..8 {
        let store = store.clone();
        let source = source.clone();
        saves.spawn(async move {
            let layout = format!(r#"{{"record_type":"Observation","sections":[],"n":{n}}}"#);
            let key = LayoutKey {
                source_id: &source,
                record_type: "Observation",
            };
            store.save_layout(key, &layout).await
        });
    }
    let results = saves.join_all().await;
    let stored: Vec<_> = results
        .iter()
        .filter_map(|result| match result {
            Ok(Saved::Stored(layout)) => Some(layout.clone()),
            _ => None,
        })
        .collect();
    assert_eq!(stored.len(), 1, "{results:?}");
    assert!(
        results
            .iter()
            .filter(|result| !matches!(result, Ok(Saved::Stored(_))))
            .all(|result| matches!(result, Err(LayoutStoreError::Conflict { .. }))),
        "{results:?}"
    );
    let key = LayoutKey {
        source_id: &source,
        record_type: "Observation",
    };
    assert_eq!(
        store.layout_for(key).await.unwrap(),
        stored.into_iter().next()
    );
}

#[tokio::test]
async fn a_blank_key_or_a_layout_that_is_not_json_is_refused_before_storing() {
    let store = store().await;
    let source = source();
    let key = LayoutKey {
        source_id: &source,
        record_type: "Observation",
    };
    assert!(matches!(
        store.save_layout(key, "{not json").await,
        Err(LayoutStoreError::NotJson(_))
    ));
    assert_eq!(store.layout_for(key).await.unwrap(), None);
    for blank in [
        LayoutKey {
            source_id: "",
            record_type: "Observation",
        },
        LayoutKey {
            source_id: &source,
            record_type: "",
        },
    ] {
        assert!(matches!(
            store.save_layout(blank, LAYOUT).await,
            Err(LayoutStoreError::BlankKey)
        ));
        assert!(matches!(
            store.layout_for(blank).await,
            Err(LayoutStoreError::BlankKey)
        ));
    }
}

#[tokio::test]
async fn a_failed_read_is_an_error_not_an_absent_layout() {
    let store = store().await;
    let source = source();
    let key = LayoutKey {
        source_id: &source,
        record_type: "Observation",
    };
    store.save_layout(key, LAYOUT).await.unwrap();
    let url = std::env::var("PHYSICIAN_TEST_DATABASE_URL").unwrap();
    let db = sea_orm::Database::connect(&url).await.unwrap();
    use sea_orm::ConnectionTrait;
    use sea_orm::sea_query::{Alias, Expr, ExprTrait, Query};
    // The entity is the store's own; outside it the table is named, and the
    // stored bytes changed without their digest.
    let tamper = Query::update()
        .table(Alias::new("physician_source_layouts"))
        .value(Alias::new("layout"), format!("{LAYOUT} "))
        .and_where(Expr::col(Alias::new("source_id")).eq(source.as_str()))
        .to_owned();
    assert_eq!(db.execute(&tamper).await.unwrap().rows_affected(), 1);
    assert!(matches!(
        store.layout_for(key).await,
        Err(LayoutStoreError::Corrupt)
    ));
    db.close().await.unwrap();
    let closed = Store::from_connection({
        let db = sea_orm::Database::connect(&url).await.unwrap();
        db.clone().close().await.unwrap();
        db
    });
    let error = closed.layout_for(key).await.unwrap_err();
    assert!(matches!(error, LayoutStoreError::Database(_)), "{error:?}");
    assert!(std::error::Error::source(&error).is_some());
}
