//! Postgres proof for the practice's Spruce channel: one row per message
//! version under replays, out-of-order deliveries and contending readers;
//! per-conversation failures that keep the last read; unreadable items kept
//! by id; files readable after links expire; disconnect, lost access, backlog
//! acceptance and the organization binding.
use super::*;
use crate::runtime::channel::intake::{self, Api, ConnectError, Intake, PassOutcome};
use crate::runtime::channel::{
    AccessLoss, AttachmentRecord, Backlog, Body, ChannelError, ChannelKey, Connection, Direction,
    OwedCause, PageWrite, ReadFailure, sha256_hex,
};
use chrono::{DateTime, SecondsFormat, Utc};
use spruce::testing::{CREDENTIAL, Delivery, FakeContact, FakeSpruce, ORGANIZATION_ID};

fn at(offset: i64) -> String {
    DateTime::<Utc>::from_timestamp(1_700_000_000 + offset, 0)
        .unwrap()
        .to_rfc3339_opts(SecondsFormat::Secs, true)
}

fn message(conversation: &str, id: &str, text: &str, created: i64) -> Delivery {
    Delivery {
        conversation: conversation.into(),
        id: id.into(),
        text: text.into(),
        created_at: at(created),
        modified_at: at(created),
        activity_at: at(created),
        transcription_id: None,
        transcription_text: None,
        transcription_status: None,
        transcription_summary: None,
    }
}

fn key() -> ChannelKey {
    ChannelKey::from_bytes([9; 32])
}

struct Setup {
    store: Store,
    fake: FakeSpruce,
    practice: String,
    intake: Intake,
}

async fn setup() -> Setup {
    let url = std::env::var("PHYSICIAN_RUNTIME_TEST_DATABASE_URL").unwrap();
    let store = Store::connect(&url).await.unwrap();
    store.migrate().await.unwrap();
    let fake = FakeSpruce::spawn().await;
    let mut intake = Intake::new(key(), Api::At(fake.base.clone()), "reader-a");
    intake.pages = 50;
    Setup {
        store,
        fake,
        practice: uuid::Uuid::new_v4().to_string(),
        intake,
    }
}

impl Setup {
    /// Connects, then runs the first pass that sets the history boundary.
    async fn connected(&self) {
        intake::connect(
            &self.store,
            &key(),
            &self.intake.api,
            &self.practice,
            "member-1",
            CREDENTIAL,
        )
        .await
        .unwrap();
        self.pass().await;
    }

    async fn pass(&self) -> PassOutcome {
        intake::pass(&self.store, &self.intake, &self.practice)
            .await
            .unwrap()
    }

    async fn texts(&self, conversation: &str) -> Vec<String> {
        self.store
            .channel_messages(&self.practice, conversation)
            .await
            .unwrap()
            .into_iter()
            .map(|version| match version.body {
                Body::Text { text } => text,
                other => format!("{other:?}"),
            })
            .collect()
    }

    async fn rows(&self, item: &str) -> usize {
        self.store
            .channel_message(&self.practice, item)
            .await
            .unwrap()
            .len()
    }
}

/// Every connection test reuses the stand-in's one organization, so each
/// starts by releasing it from whatever practice an earlier test left bound.
async fn release_organization(store: &Store) {
    use crate::runtime::entities::channel_connection;
    use sea_orm::QueryFilter;
    for row in channel_connection::Entity::find()
        .filter(channel_connection::Column::Organization.eq(ORGANIZATION_ID))
        .all(&store.db)
        .await
        .unwrap()
    {
        store.disconnect_channel(&row.practice).await.unwrap();
    }
}

#[tokio::test]
#[ignore = "requires explicit local Postgres"]
async fn each_message_is_kept_once_across_replays_order_and_contending_readers() {
    let s = setup().await;
    release_organization(&s.store).await;
    s.fake.deliver(message("c1", "before", "history", 0));
    s.connected().await;
    assert_eq!(
        s.rows("before").await,
        0,
        "history waits for the backlog offer"
    );

    s.fake.deliver(message("c1", "m2", "second", 20));
    s.fake.deliver(message("c2", "m3", "third", 30));
    s.fake.deliver(message("c1", "m1", "first", 10));
    s.fake.show_twice("m2");

    // Two readers contend: exactly one holds the lease at a time.
    let mut other = s.intake.clone();
    other.holder = "reader-b".into();
    let (a, b) = tokio::join!(
        intake::pass(&s.store, &s.intake, &s.practice),
        intake::pass(&s.store, &other, &s.practice)
    );
    let outcomes = [a.unwrap(), b.unwrap()];
    assert!(
        outcomes
            .iter()
            .any(|o| matches!(o, PassOutcome::Read { .. }))
    );
    // A crash after the commit but before the next page replays the same
    // read from the stored cursor; nothing is kept twice.
    s.pass().await;
    s.pass().await;
    for item in ["m1", "m2", "m3"] {
        assert_eq!(s.rows(item).await, 1, "{item}");
    }
    assert_eq!(s.texts("c1").await, ["first", "second"]);
    let cursor = s.store.channel_cursor(&s.practice).await.unwrap();
    assert!(cursor.is_some());
    println!("saved cursor: {}", cursor.unwrap());
}

#[tokio::test]
#[ignore = "requires explicit local Postgres"]
async fn a_stale_reader_cannot_commit_after_losing_its_lease() {
    let s = setup().await;
    release_organization(&s.store).await;
    s.connected().await;
    let lease = s
        .store
        .claim_channel(&s.practice, "stale", 60)
        .await
        .unwrap()
        .unwrap();
    assert!(
        s.store
            .claim_channel(&s.practice, "other", 60)
            .await
            .unwrap()
            .is_none()
    );
    // Disconnect clears the lease; the reader's next commit must fail.
    s.store.disconnect_channel(&s.practice).await.unwrap();
    let error = s
        .store
        .apply_channel_page(&lease, PageWrite::default())
        .await
        .unwrap_err();
    assert!(matches!(error, ChannelError::LeaseLost));
}

#[tokio::test]
#[ignore = "requires explicit local Postgres"]
async fn a_failing_conversation_leaves_the_others_and_keeps_the_last_read() {
    let s = setup().await;
    release_organization(&s.store).await;
    s.connected().await;
    s.fake.deliver(message("good", "g1", "fine", 10));
    s.fake.deliver(message("bad", "b1", "lost for now", 11));
    s.fake.fail("bad");
    s.pass().await;
    assert_eq!(s.texts("good").await, ["fine"]);
    let status = s.store.channel_status(&s.practice).await.unwrap();
    let read_at = status.last_read_at.expect("last read time");
    assert_eq!(status.unread.len(), 1);
    assert_eq!(status.unread[0].conversation, "bad");
    assert_eq!(
        status.unread[0].owed.cause,
        OwedCause::TemporarilyUnavailable
    );

    // Spruce stops answering entirely: the failure shows and the last read stays.
    s.fake.reject_unauthorized();
    s.fake.recover();
    let lease = s
        .store
        .claim_channel(&s.practice, "probe", 60)
        .await
        .unwrap()
        .unwrap();
    s.store.channel_unavailable(&lease).await.unwrap();
    s.store.release_channel(&lease).await.unwrap();
    let status = s.store.channel_status(&s.practice).await.unwrap();
    assert!(matches!(
        status.failure,
        Some(ReadFailure::Unavailable { .. })
    ));
    assert_eq!(status.last_read_at, Some(read_at));

    // Recovery reads the owed conversation and clears the failure.
    s.pass().await;
    assert_eq!(s.texts("bad").await, ["lost for now"]);
    let status = s.store.channel_status(&s.practice).await.unwrap();
    assert!(status.unread.is_empty());
    assert!(status.failure.is_none());
}

#[tokio::test]
#[ignore = "requires explicit local Postgres"]
async fn an_unreadable_item_stays_by_id_and_no_recording_is_fetched() {
    let s = setup().await;
    release_organization(&s.store).await;
    s.connected().await;
    s.fake.deliver(message("c1", "odd", "x", 10));
    s.fake.make_unreadable("odd");
    let mut voicemail = message("c1", "vm", "", 11);
    voicemail.transcription_id = Some("tr1".into());
    voicemail.transcription_text = Some("call me back".into());
    s.fake.deliver(voicemail);
    s.pass().await;
    let odd = s.store.channel_message(&s.practice, "odd").await.unwrap();
    assert_eq!(odd.len(), 1);
    assert_eq!(odd[0].body, Body::Unreadable);
    assert!(odd[0].app_url.as_deref().unwrap().contains("/message/odd"));
    let status = s.store.channel_status(&s.practice).await.unwrap();
    assert_eq!(status.unread[0].owed.cause, OwedCause::Undecodable);
    assert!(
        s.fake
            .requests()
            .iter()
            .all(|path| !str::contains(path, "recording") && !str::contains(path, "audio")),
        "no recording fetched"
    );
}

#[tokio::test]
#[ignore = "requires explicit local Postgres"]
async fn files_read_back_after_links_expire_and_edits_and_deletions_are_versions() {
    let s = setup().await;
    release_organization(&s.store).await;
    s.connected().await;
    s.fake.deliver(message("c1", "f1", "see attached", 10));
    s.fake.attach("f1", "scan.pdf", b"%PDF-1.7 bytes");
    s.fake.add_contact(
        "c1",
        FakeContact {
            id: "contact_1".into(),
            display_name: "Ada Patient".into(),
            phone_numbers: vec!["+15555550100".into()],
            date_of_birth: Some("1980-01-02".into()),
        },
    );
    s.pass().await;
    s.fake.expire_links();
    let saved = s.store.channel_message(&s.practice, "f1").await.unwrap();
    let AttachmentRecord::File { sha256, .. } = &saved[0].attachments[0] else {
        panic!("file saved");
    };
    assert_eq!(*sha256, sha256_hex(b"%PDF-1.7 bytes"));
    let bytes = s
        .store
        .channel_file(&s.practice, "c1", sha256)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(bytes, b"%PDF-1.7 bytes");
    let details = s
        .store
        .channel_conversation(&s.practice, "c1")
        .await
        .unwrap()
        .unwrap();
    assert_eq!(
        details.contacts[0].display_name.as_deref(),
        Some("Ada Patient")
    );

    // An edit and a deletion are new versions; the earlier ones stay. They
    // arrive through a full read of the conversation, as reconcile reports.
    s.fake.deliver(message("c1", "e1", "draft", 20));
    s.pass().await;
    s.fake.edit("e1", "final", &at(25));
    s.fake.flag_deleted("f1", &at(26));
    owe(&s, "c1").await;
    s.pass().await;
    let e1 = s.store.channel_message(&s.practice, "e1").await.unwrap();
    assert_eq!(e1.len(), 2);
    let f1 = s.store.channel_message(&s.practice, "f1").await.unwrap();
    assert_eq!(f1.len(), 2);
    assert!(!f1[0].deleted && f1[1].deleted);

    // An item missing from a later read is not deleted.
    s.fake.delete("e1");
    owe(&s, "c1").await;
    s.pass().await;
    assert_eq!(s.rows("e1").await, 2);
}

async fn owe(s: &Setup, conversation: &str) {
    let lease = s
        .store
        .claim_channel(&s.practice, "owe", 60)
        .await
        .unwrap()
        .unwrap();
    s.store
        .apply_channel_page(
            &lease,
            PageWrite {
                owed: vec![(conversation.into(), OwedCause::TemporarilyUnavailable)],
                ..Default::default()
            },
        )
        .await
        .unwrap();
    s.store.release_channel(&lease).await.unwrap();
}

#[tokio::test]
#[ignore = "requires explicit local Postgres"]
async fn replies_sent_in_spruce_are_saved_as_the_practice() {
    let s = setup().await;
    release_organization(&s.store).await;
    s.connected().await;
    s.fake.deliver(message("c1", "in", "question", 10));
    s.fake.deliver(message("c1", "out", "answer", 11));
    s.fake.mark_outbound("out");
    s.pass().await;
    let saved = s.store.channel_messages(&s.practice, "c1").await.unwrap();
    assert_eq!(saved[0].direction, Direction::Patient);
    assert_eq!(saved[1].direction, Direction::Practice);
    assert_eq!(saved[1].created_at.as_deref(), Some(at(11).as_str()));
}

#[tokio::test]
#[ignore = "requires explicit local Postgres"]
async fn disconnect_deletes_unlinked_messages_and_access_loss_resumes_on_reconnect() {
    let s = setup().await;
    release_organization(&s.store).await;
    s.connected().await;
    s.fake.deliver(message("linked", "l1", "keep", 10));
    s.fake.deliver(message("loose", "u1", "drop", 11));
    s.pass().await;
    s.store
        .link_channel_conversation(&s.practice, "linked", "patient-1")
        .await
        .unwrap();

    // Lost access asks for a reconnect and keeps the cursor.
    let cursor = s.store.channel_cursor(&s.practice).await.unwrap();
    s.fake.reject_unauthorized();
    assert_eq!(s.pass().await, PassOutcome::ReconnectRequired);
    assert!(matches!(
        s.store.channel_connection(&s.practice).await.unwrap(),
        Connection::ReconnectRequired {
            reason: AccessLoss::Revoked,
            ..
        }
    ));
    assert_eq!(s.store.channel_cursor(&s.practice).await.unwrap(), cursor);
    assert_eq!(s.pass().await, PassOutcome::NotReading);
    s.fake.recover();
    s.fake.deliver(message("loose", "u2", "while away", 12));
    intake::connect(
        &s.store,
        &key(),
        &s.intake.api,
        &s.practice,
        "member-2",
        CREDENTIAL,
    )
    .await
    .unwrap();
    s.pass().await;
    assert_eq!(s.texts("loose").await, ["drop", "while away"]);

    s.store.disconnect_channel(&s.practice).await.unwrap();
    assert_eq!(s.texts("linked").await, ["keep"]);
    assert!(s.texts("loose").await.is_empty());
    assert!(s.store.channel_cursor(&s.practice).await.unwrap().is_none());
    assert_eq!(
        s.store.channel_connection(&s.practice).await.unwrap(),
        Connection::Disconnected
    );
}

#[tokio::test]
#[ignore = "requires explicit local Postgres"]
async fn history_is_read_only_after_the_backlog_is_accepted() {
    let s = setup().await;
    release_organization(&s.store).await;
    s.fake.deliver(message("old", "h1", "from before", 0));
    s.connected().await;
    s.fake.deliver(message("old", "n1", "new", 50));
    s.pass().await;
    assert_eq!(s.texts("old").await, ["new"]);
    assert!(matches!(
        s.store.channel_connection(&s.practice).await.unwrap(),
        Connection::Reading {
            backlog: Backlog::Offered,
            ..
        }
    ));
    s.store.accept_channel_backlog(&s.practice).await.unwrap();
    s.pass().await; // lists the backlog and queues each conversation
    s.pass().await; // reads them in full
    assert_eq!(s.texts("old").await, ["from before", "new"]);
    assert!(matches!(
        s.store.channel_connection(&s.practice).await.unwrap(),
        Connection::Reading {
            backlog: Backlog::Read,
            ..
        }
    ));
}

#[tokio::test]
#[ignore = "requires explicit local Postgres"]
async fn the_organization_binds_to_one_practice_and_a_refused_credential_stores_nothing() {
    let s = setup().await;
    release_organization(&s.store).await;
    s.fake.reject_unauthorized();
    let refused = intake::connect(
        &s.store,
        &key(),
        &s.intake.api,
        &s.practice,
        "m",
        CREDENTIAL,
    )
    .await
    .unwrap_err();
    assert!(matches!(refused, ConnectError::Refused(_)));
    assert_eq!(
        s.store.channel_connection(&s.practice).await.unwrap(),
        Connection::Disconnected
    );
    s.fake.recover();
    let bound = intake::connect(
        &s.store,
        &key(),
        &s.intake.api,
        &s.practice,
        "m",
        CREDENTIAL,
    )
    .await
    .unwrap();
    assert!(matches!(
        bound,
        Connection::Reading { ref organization, ref bound_by, .. }
            if organization == ORGANIZATION_ID && bound_by == "m"
    ));

    // A restart reads the same binding back, and the credential only opens
    // with the channel key.
    let url = std::env::var("PHYSICIAN_RUNTIME_TEST_DATABASE_URL").unwrap();
    let restarted = Store::connect(&url).await.unwrap();
    assert_eq!(
        restarted.channel_connection(&s.practice).await.unwrap(),
        bound
    );
    let lease = restarted
        .claim_channel(&s.practice, "restart", 30)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(
        restarted.channel_credential(&key(), &lease).unwrap(),
        CREDENTIAL
    );
    assert!(
        restarted
            .channel_credential(&ChannelKey::from_bytes([1; 32]), &lease)
            .is_err()
    );
    restarted.release_channel(&lease).await.unwrap();

    let other = uuid::Uuid::new_v4().to_string();
    let taken = intake::connect(&s.store, &key(), &s.intake.api, &other, "m", CREDENTIAL)
        .await
        .unwrap_err();
    assert!(matches!(
        taken,
        ConnectError::Store(ChannelError::OrganizationTaken)
    ));
    s.store.disconnect_channel(&s.practice).await.unwrap();
}
