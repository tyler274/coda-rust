//! Spruce change reading against the loopback stand-in.
//!
//! These tests fail when the behavior they name is removed. They do not prove
//! the live Spruce service; `tests/live.rs` is that read-only check.

use std::time::Duration;

use chrono::{DateTime, SecondsFormat, Utc};
use spruce::api::Error;
use spruce::changes::{ConversationRead, Cursor, Observed, Outcome, Reader, UnreadCause};
use spruce::payload::{Item, ItemEntry, TranscriptionState};
use spruce::reconcile::reconcile;
use spruce::testing::{Delivery, FakeSpruce};

fn at(offset: i64) -> String {
    DateTime::<Utc>::from_timestamp(1_700_000_000 + offset, 0)
        .expect("timestamp")
        .to_rfc3339_opts(SecondsFormat::Secs, true)
}

fn message(conversation: &str, id: &str, text: &str, created: i64, activity: i64) -> Delivery {
    Delivery {
        conversation: conversation.into(),
        id: id.into(),
        text: text.into(),
        created_at: at(created),
        modified_at: at(created),
        activity_at: at(activity),
        transcription_id: None,
        transcription_text: None,
        transcription_status: None,
        transcription_summary: None,
    }
}

fn open(fake: &FakeSpruce, overlap: Duration, page_size: u32) -> Reader {
    Reader::new(fake.client())
        .with_overlap(overlap)
        .with_page_size(page_size)
        .expect("page size")
}

fn ids(changes: &[Observed]) -> Vec<&str> {
    changes
        .iter()
        .map(|observed| match observed {
            Observed::Message(item) => item.id(),
            Observed::Unreadable(envelope) => envelope.id.as_str(),
        })
        .collect()
}

async fn drain(reader: &Reader, mut cursor: Option<Cursor>) -> (Cursor, Vec<Observed>) {
    let mut all = Vec::new();
    for _ in 0..10 {
        match reader.changes(cursor.as_ref()).await.expect("changes") {
            Outcome::Page(page) => {
                let more = page.has_more;
                all.extend(page.changes);
                let next = page.cursor;
                if !more {
                    return (next, all);
                }
                cursor = Some(next);
            }
            Outcome::ReconnectRequired => panic!("credential refused"),
            Outcome::Unavailable => panic!("spruce unavailable"),
        }
    }
    panic!("change pass did not finish");
}

async fn all_items(reader: &Reader, conversation: &str) -> Vec<Item> {
    let mut page = None;
    let mut items = Vec::new();
    for _ in 0..10 {
        match reader
            .read_conversation(conversation, page.as_deref())
            .await
            .expect("read")
        {
            ConversationRead::Messages {
                items: observed,
                next_page,
                undecodable,
            } => {
                assert!(!undecodable, "conversation should be readable");
                for observed in observed {
                    let Observed::Message(item) = observed else {
                        panic!("expected a decoded item");
                    };
                    items.push(*item);
                }
                match next_page {
                    Some(token) => page = Some(token),
                    None => return items,
                }
            }
            other => panic!("unexpected read {other:?}"),
        }
    }
    panic!("conversation read did not finish");
}

#[tokio::test]
async fn the_first_pass_emits_no_history() {
    let fake = FakeSpruce::spawn().await;
    fake.deliver(message("t_old", "ti_old", "already there", 0, 0));
    let reader = open(&fake, Duration::ZERO, 50);
    let (_cursor, changes) = drain(&reader, None).await;
    assert!(changes.is_empty());
}

#[tokio::test]
async fn a_new_message_comes_through_and_history_does_not() {
    let fake = FakeSpruce::spawn().await;
    fake.deliver(message("t_old", "ti_old", "already there", 0, 0));
    let reader = open(&fake, Duration::ZERO, 50);
    let (cursor, changes) = drain(&reader, None).await;
    assert!(changes.is_empty());
    fake.deliver(message("t_new", "ti_new", "just arrived", 100, 100));
    let (_cursor, changes) = drain(&reader, Some(cursor)).await;
    assert_eq!(ids(&changes), ["ti_new"]);
}

#[tokio::test]
async fn a_duplicate_shows_the_same_version() {
    let fake = FakeSpruce::spawn().await;
    fake.deliver(message("t_new", "ti_new", "just arrived", 100, 100));
    let reader = open(&fake, Duration::ZERO, 50);
    let first = all_items(&reader, "t_new").await;
    let second = all_items(&reader, "t_new").await;
    assert_eq!(first.len(), 1);
    assert_eq!(first[0].version(), second[0].version());

    fake.show_twice("ti_new");
    let ConversationRead::Messages { items, .. } =
        reader.read_conversation("t_new", None).await.expect("read")
    else {
        panic!("expected messages");
    };
    assert_eq!(items.len(), 2);
    let Observed::Message(left) = &items[0] else {
        panic!("message")
    };
    let Observed::Message(right) = &items[1] else {
        panic!("message")
    };
    assert_eq!(left.id(), right.id());
    assert_eq!(left.version(), right.version());
}

#[tokio::test]
async fn the_overlap_window_catches_a_delayed_item() {
    let fake = FakeSpruce::spawn().await;
    fake.deliver(message("t_one", "ti_old", "already there", 0, 0));
    let wide = open(&fake, Duration::from_secs(120), 50);
    let (cursor, _) = drain(&wide, None).await;
    fake.deliver(message("t_one", "ti_current", "on time", 300, 300));
    let (cursor, changes) = drain(&wide, Some(cursor)).await;
    assert_eq!(ids(&changes), ["ti_current"]);

    fake.deliver(message("t_one", "ti_late", "listed late", 270, 310));
    let (_cursor, caught) = drain(&wide, Some(cursor.clone())).await;
    assert!(
        ids(&caught).contains(&"ti_late"),
        "overlap should reread the late item"
    );

    let tight = open(&fake, Duration::ZERO, 50);
    let (_cursor, missed) = drain(&tight, Some(cursor)).await;
    assert!(
        !ids(&missed).contains(&"ti_late"),
        "without overlap the late item is before the boundary"
    );
}

#[tokio::test]
async fn catch_up_replays_an_unsaved_page_and_resumes_without_loss() {
    let fake = FakeSpruce::spawn().await;
    fake.deliver(message("t_old", "ti_old", "already there", 0, 0));
    let reader = open(&fake, Duration::ZERO, 1);
    let (cursor, changes) = drain(&reader, None).await;
    assert!(changes.is_empty());

    fake.deliver(message("t_b", "ti_b", "second", 100, 100));
    fake.deliver(message("t_c", "ti_c", "third", 200, 200));

    let first = match reader.changes(Some(&cursor)).await.expect("page") {
        Outcome::Page(page) => page,
        other => panic!("unexpected {other:?}"),
    };
    let replay = match reader.changes(Some(&cursor)).await.expect("replay") {
        Outcome::Page(page) => page,
        other => panic!("unexpected {other:?}"),
    };
    assert!(first.has_more, "one item per page leaves another page");
    assert_eq!(ids(&first.changes), ids(&replay.changes));
    assert_eq!(ids(&first.changes), ["ti_c"]);

    let mut cursor = first.cursor;
    let mut rest = Vec::new();
    for _ in 0..10 {
        match reader.changes(Some(&cursor)).await.expect("resume") {
            Outcome::Page(page) => {
                rest.extend(page.changes);
                let more = page.has_more;
                cursor = page.cursor;
                if !more {
                    break;
                }
            }
            other => panic!("unexpected {other:?}"),
        }
    }
    let mut found = ids(&first.changes);
    found.extend(ids(&rest));
    found.sort();
    assert_eq!(found, ["ti_b", "ti_c"]);
    assert!(!found.contains(&"ti_old"));
}

#[tokio::test]
async fn an_edit_is_a_new_version_and_a_deletion_is_missing() {
    let fake = FakeSpruce::spawn().await;
    fake.deliver(message("t_one", "ti_edit", "before", 0, 0));
    fake.deliver(message("t_one", "ti_drop", "going", 10, 10));
    let reader = open(&fake, Duration::ZERO, 50);
    let before = all_items(&reader, "t_one").await;
    let edit_version = before
        .iter()
        .find(|item| item.id() == "ti_edit")
        .expect("edit")
        .version();
    let drop_version = before
        .iter()
        .find(|item| item.id() == "ti_drop")
        .expect("drop")
        .version();

    fake.edit("ti_edit", "after", &at(50));
    fake.delete("ti_drop");
    let after = all_items(&reader, "t_one").await;
    let diff = reconcile(
        &[
            ("ti_edit", edit_version.as_str()),
            ("ti_drop", drop_version.as_str()),
        ],
        &after,
    );
    assert_eq!(diff.edited.len(), 1);
    assert_eq!(diff.edited[0].id(), "ti_edit");
    assert_eq!(diff.edited[0].text, "after");
    assert_eq!(diff.missing, ["ti_drop".to_owned()]);
    assert!(
        diff.deleted.is_empty(),
        "absence is not a Spruce deletion flag"
    );
}

#[tokio::test]
async fn one_failing_conversation_leaves_the_others() {
    let fake = FakeSpruce::spawn().await;
    fake.deliver(message("t_old", "ti_old", "already there", 0, 0));
    let reader = open(&fake, Duration::ZERO, 50);
    let (cursor, _) = drain(&reader, None).await;
    fake.deliver(message("t_ok", "ti_ok", "readable", 100, 100));
    fake.deliver(message("t_bad", "ti_bad", "unreadable", 110, 110));
    fake.fail("t_bad");

    let Outcome::Page(page) = reader.changes(Some(&cursor)).await.expect("page") else {
        panic!("a conversation failure is not a failed pass");
    };
    assert_eq!(ids(&page.changes), ["ti_ok"]);
    assert_eq!(page.unread.len(), 1);
    assert_eq!(page.unread[0].conversation_id, "t_bad");
    assert_eq!(page.unread[0].cause, UnreadCause::TemporarilyUnavailable);
}

#[tokio::test]
async fn a_refused_credential_asks_for_reconnect() {
    let fake = FakeSpruce::spawn().await;
    fake.deliver(message("t_one", "ti_one", "hello", 0, 0));
    let reader = open(&fake, Duration::ZERO, 50);
    fake.reject_unauthorized();
    assert!(matches!(
        reader.changes(None).await.expect("401"),
        Outcome::ReconnectRequired
    ));

    let forbidden = FakeSpruce::spawn().await;
    forbidden.deliver(message("t_one", "ti_one", "hello", 0, 0));
    let reader = open(&forbidden, Duration::ZERO, 50);
    forbidden.forbid();
    assert!(matches!(
        reader.changes(None).await.expect("403"),
        Outcome::ReconnectRequired
    ));
}

#[tokio::test]
async fn a_cursor_this_crate_did_not_write_is_refused() {
    assert!(Cursor::decode("nope").is_err());
    assert!(
        Cursor::decode(r#"{"floor":"not-a-time","boundary":null,"page":null,"seen":null}"#)
            .is_err()
    );
    let stored = Cursor::decode(r#"{"floor":"2026-10-27T18:12:50Z","boundary":"2026-10-27T18:12:50Z","page":null,"seen":null}"#).expect("ours");
    assert_eq!(
        Cursor::decode(&stored.encode()).expect("round trip"),
        stored
    );
}

#[tokio::test]
async fn a_transcript_is_fetched_and_a_missing_one_keeps_the_item() {
    let fake = FakeSpruce::spawn().await;
    let mut heard = message("t_one", "ti_heard", "", 0, 0);
    heard.transcription_id = Some("tr_heard".into());
    heard.transcription_text = Some("heard the cough".into());
    fake.deliver(heard);
    let mut missing = message("t_one", "ti_missing", "", 10, 10);
    missing.transcription_id = Some("tr_missing".into());
    fake.deliver(missing);
    let reader = open(&fake, Duration::ZERO, 50);
    let items = all_items(&reader, "t_one").await;
    let heard = items
        .iter()
        .find(|item| item.id() == "ti_heard")
        .expect("heard");
    assert!(
        matches!(heard.transcription, Some(TranscriptionState::Ready { ref text, .. }) if text == "heard the cough")
    );
    let again = all_items(&reader, "t_one").await;
    assert_eq!(
        heard.version(),
        again
            .iter()
            .find(|item| item.id() == "ti_heard")
            .expect("heard")
            .version()
    );
    let missing = items
        .iter()
        .find(|item| item.id() == "ti_missing")
        .expect("missing");
    assert!(matches!(
        missing.transcription,
        Some(TranscriptionState::NotFound { .. })
    ));
}

#[tokio::test]
async fn a_fractional_activity_time_stays_history() {
    let fake = FakeSpruce::spawn().await;
    let mut existing = message("t_old", "ti_old", "already there", 0, 0);
    existing.created_at = "2026-10-27T18:12:50.400Z".into();
    existing.modified_at = existing.created_at.clone();
    existing.activity_at = existing.created_at.clone();
    fake.deliver(existing);
    let reader = open(&fake, Duration::ZERO, 50);
    let (cursor, changes) = drain(&reader, None).await;
    assert!(changes.is_empty());
    assert!(
        cursor.encode().contains("2026-10-27T18:12:50.400Z"),
        "the history floor must keep the fraction: {}",
        cursor.encode()
    );
    let (_cursor, changes) = drain(&reader, Some(cursor)).await;
    assert!(
        changes.is_empty(),
        "a message that existed at connect time must stay history"
    );
}

#[tokio::test]
async fn an_unfinished_transcript_is_not_a_finished_body() {
    let fake = FakeSpruce::spawn().await;
    for (id, item, status, text, summary) in [
        ("tr_pending", "ti_pending", "pending", "", None),
        ("tr_failed", "ti_failed", "failed", "", None),
        ("tr_silent", "ti_silent", "noSpeech", "", None),
        (
            "tr_done",
            "ti_done",
            "completed",
            "heard the cough",
            Some("cough"),
        ),
    ] {
        let mut delivery = message("t_one", item, "", 0, 0);
        delivery.transcription_id = Some(id.into());
        delivery.transcription_text = Some(text.into());
        delivery.transcription_status = Some(status.into());
        delivery.transcription_summary = summary.map(str::to_owned);
        fake.deliver(delivery);
    }
    let reader = open(&fake, Duration::ZERO, 50);
    let items = all_items(&reader, "t_one").await;
    let state = |id: &str| {
        items
            .iter()
            .find(|item| item.id() == id)
            .expect(id)
            .transcription
            .clone()
    };
    assert!(matches!(
        state("ti_pending"),
        Some(TranscriptionState::Unfinished { ref status, ref text, .. })
            if status == "pending" && text.is_empty()
    ));
    assert!(matches!(
        state("ti_failed"),
        Some(TranscriptionState::Unfinished { ref status, .. }) if status == "failed"
    ));
    assert!(matches!(
        state("ti_silent"),
        Some(TranscriptionState::Unfinished { ref status, .. }) if status == "noSpeech"
    ));
    assert!(matches!(
        state("ti_done"),
        Some(TranscriptionState::Ready { ref text, ref summary, .. })
            if text == "heard the cough" && summary.as_deref() == Some("cough")
    ));

    let pending_version = items
        .iter()
        .find(|item| item.id() == "ti_pending")
        .expect("pending")
        .version();
    let mut finished = message("t_one", "ti_pending", "", 0, 0);
    finished.transcription_id = Some("tr_pending".into());
    finished.transcription_text = Some("heard the cough".into());
    finished.transcription_status = Some("completed".into());
    fake.deliver(finished);
    let again = all_items(&reader, "t_one").await;
    let pending = again
        .iter()
        .find(|item| item.id() == "ti_pending")
        .expect("pending");
    assert!(matches!(
        &pending.transcription,
        Some(TranscriptionState::Ready { text, .. }) if text == "heard the cough"
    ));
    assert_ne!(pending.version(), pending_version);
}

#[tokio::test]
async fn an_oversized_body_is_not_a_value() {
    let fake = FakeSpruce::spawn().await;
    let error = fake
        .client()
        .with_max_bytes(8)
        .organization()
        .await
        .expect_err("cap");
    assert_eq!(error, Error::Contract);
}

#[tokio::test]
async fn a_deleted_item_reads_as_not_found() {
    let fake = FakeSpruce::spawn().await;
    fake.deliver(message("t_one", "ti_one", "hello", 0, 0));
    let ItemEntry::Item(item) = fake.client().get_item("ti_one").await.expect("item") else {
        panic!("expected a decoded item");
    };
    assert_eq!(item.id(), "ti_one");
    fake.delete("ti_one");
    let error = fake.client().get_item("ti_one").await.expect_err("gone");
    assert_eq!(error, Error::NotFound);
}

#[tokio::test]
async fn transient_attachment_bytes_download_without_keeping_signed_urls_and_isolate_failures() {
    use spruce::changes::AttachmentBytesOutcome;
    use spruce::testing::TestAttachment;

    let fake = FakeSpruce::spawn().await;
    fake.deliver(message(
        "t_att",
        "ti_att",
        "Attached fax and failing sibling",
        10,
        10,
    ));
    fake.attach_to_item(
        "ti_att",
        vec![
            TestAttachment {
                kind: "fax".into(),
                title: Some("Consult Fax".into()),
                mimetype: Some("application/pdf".into()),
                name: Some("consult.pdf".into()),
                bytes: Some(b"%PDF-1.4\nhello fax".to_vec()),
            },
            TestAttachment {
                kind: "image".into(),
                title: Some("Broken photo".into()),
                mimetype: Some("image/png".into()),
                name: Some("broken.png".into()),
                bytes: None,
            },
        ],
    );
    let reader = open(&fake, Duration::ZERO, 50);
    let items = all_items(&reader, "t_att").await;
    assert_eq!(items.len(), 1);
    let v_before = items[0].version();

    let fetched = reader
        .fetch_item_attachments(&items[0])
        .await
        .expect("fetch attachments");
    assert_eq!(fetched.len(), 2);
    assert!(matches!(
        &fetched[0],
        AttachmentBytesOutcome::Fetched { attachment, bytes }
            if attachment.name.as_deref() == Some("consult.pdf") && bytes == b"%PDF-1.4\nhello fax"
    ));
    assert!(matches!(
        &fetched[1],
        AttachmentBytesOutcome::Unavailable { attachment, .. }
            if attachment.name.as_deref() == Some("broken.png")
    ));
    assert_eq!(items[0].version(), v_before);
}

#[tokio::test]
async fn backlog_page_reads_pre_connection_items_when_requested() {
    let fake = FakeSpruce::spawn().await;
    fake.deliver(message("t_hist", "ti_old_1", "Before connect", 0, 0));
    let reader = open(&fake, Duration::ZERO, 50);
    let (cursor, changes) = drain(&reader, None).await;
    assert!(changes.is_empty());

    let backlog = reader
        .read_backlog_page(&cursor, None)
        .await
        .expect("backlog call")
        .expect("backlog page");
    assert_eq!(ids(&backlog.items), ["ti_old_1"]);
}
