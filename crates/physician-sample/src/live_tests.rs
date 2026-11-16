use super::live;

#[test]
fn chat_titles_are_one_line_and_cut_at_a_word() {
    assert_eq!(live::chat_title("  About\n the   labs "), "About the labs");
    let title = live::chat_title(
        "Please review the potassium trend and tell me whether the latest result changes the plan",
    );
    assert_eq!(
        title,
        "Please review the potassium trend and tell me whether the…"
    );
    assert!(title.chars().count() <= live::CHAT_TITLE_CHARS + 1);
    let unbroken = "x".repeat(80);
    assert_eq!(
        live::chat_title(&unbroken).chars().count(),
        live::CHAT_TITLE_CHARS + 1
    );
}

/// The keep-alive interval the change stream keeps (`live::KEEP_ALIVE_EVERY`), restated so the
/// tests measure the beat rather than read it back.
const KEEP_ALIVE: std::time::Duration = std::time::Duration::from_secs(15);

/// The frame the change stream sends for a change of workspace `pw-n`.
fn changed(n: u32) -> String {
    format!("event: workspace_changed\ndata: {{\"patient_workspace_id\":\"pw-{n}\"}}\n\n")
}

/// The change stream sends an empty comment every 15 s from the moment it opens, whether or not
/// changes pass, each change still arrives as its own frame, and the stream ends when the changes
/// end. Changes arrive every 10 s, closer together than the beat, so a timer that waits for a
/// quiet spell would never send a comment. The clock is tokio's paused one, so no test waits in
/// real time.
#[tokio::test(start_paused = true)]
async fn the_change_stream_keeps_alive_on_a_fixed_beat_while_changes_pass_and_ends_with_them() {
    use std::sync::Arc;
    use std::time::Duration;

    use axum::extract::State;
    use futures_util::StreamExt;
    use physician_bus::{Bus, Event, LocalBus};
    use tokio::time::{Instant, sleep_until, timeout};

    // The page takes this long without bytes as a dropped connection.
    let silence = Duration::from_secs(30);
    let every = Duration::from_secs(10);
    let bus: Arc<dyn Bus> = Arc::new(LocalBus::new(8));
    let response = live::events(State(Arc::clone(&bus))).await;
    assert_eq!(response.headers()["content-type"], "text/event-stream");
    // The eight frames expected and at most one more: a stream that fails to end shows here as a
    // ninth frame, so the test fails instead of waiting without end.
    let mut frames = response.into_body().into_data_stream().take(9);
    let opened = Instant::now();

    // Five changes, the last a resync, then the bus goes away: its subscription closes, and with
    // it the changes. This task holds the only handle on the bus.
    let changes = tokio::spawn(async move {
        for n in 1..=5_u32 {
            sleep_until(opened + every * n).await;
            let change = if n == 5 {
                Event::Resync
            } else {
                Event::WorkspaceChanged {
                    workspace_id: format!("pw-{n}"),
                }
            };
            bus.publish(change).await.unwrap();
        }
    });

    let mut heard = Vec::new();
    while let Some(frame) = timeout(silence, frames.next())
        .await
        .expect("the stream stayed silent as long as the page waits")
    {
        let frame = frame.expect("the frame's bytes");
        heard.push((opened.elapsed(), String::from_utf8(frame.to_vec()).unwrap()));
    }
    changes.await.unwrap();

    let mut expected = vec![
        (every, changed(1)),
        (KEEP_ALIVE, ":\n\n".to_owned()),
        (every * 2, changed(2)),
        (every * 3, changed(3)),
        (KEEP_ALIVE * 2, ":\n\n".to_owned()),
        (every * 4, changed(4)),
        (KEEP_ALIVE * 3, ":\n\n".to_owned()),
        (every * 5, "event: resync\ndata: {}\n\n".to_owned()),
    ];
    // A change and a comment due at the same instant (30 s) may come in either order.
    expected.sort();
    heard.sort();
    assert_eq!(heard, expected);
    // The stream ended when the changes ended, at the last change and not at a later beat.
    assert_eq!(opened.elapsed(), every * 5);
}

/// A due beat is not held back by changes that are ready. The stream is open longer than one
/// beat while its reader takes nothing, and several changes wait: the first frame the reader
/// takes is the comment, then each change, and the stream ends with them. A sibling of the test
/// above, not a part of it: that one times each frame from the stream's opening for a reader that
/// takes bytes at once, and a reader that holds back moves every one of those times.
#[tokio::test(start_paused = true)]
async fn a_due_beat_goes_out_before_changes_that_are_ready() {
    use std::sync::Arc;
    use std::time::Duration;

    use axum::extract::State;
    use futures_util::StreamExt;
    use physician_bus::{Bus, Event, LocalBus};

    let bus: Arc<dyn Bus> = Arc::new(LocalBus::new(8));
    let response = live::events(State(Arc::clone(&bus))).await;
    let frames = response.into_body().into_data_stream();
    for n in 1..=3_u32 {
        bus.publish(Event::WorkspaceChanged {
            workspace_id: format!("pw-{n}"),
        })
        .await
        .unwrap();
    }
    // The changes end once they are taken: this was the only handle on the bus.
    drop(bus);
    tokio::time::sleep(KEEP_ALIVE + Duration::from_secs(1)).await;

    // The four frames expected and at most one more: a stream that fails to end shows here as a
    // fifth frame, so the test fails instead of waiting without end.
    let heard: Vec<String> = frames
        .take(5)
        .map(|frame| String::from_utf8(frame.expect("the frame's bytes").to_vec()).unwrap())
        .collect()
        .await;
    assert_eq!(
        heard,
        [":\n\n".to_owned(), changed(1), changed(2), changed(3)]
    );
}

/// A signed-in reader keeps hearing its stream while the practice's bus carries changes for a
/// patient it may not see: the sign-in layer drops those frames after they were framed, so they
/// must not be what keeps the stream alive. The reader's body gets a comment at least every 15 s
/// and nothing else. The set-up is the sign-in layer's own (the fixture practice on a memory
/// store, the route `required_router` serves), in process, on tokio's paused clock.
#[tokio::test(start_paused = true)]
async fn a_signed_in_reader_hears_the_beat_while_only_changes_it_may_not_see_pass() {
    use std::sync::Arc;
    use std::time::Duration;

    use axum::body::Body;
    use axum::http::{Request, StatusCode, header};
    use axum::routing::get;
    use front_door::{FrontDoorStore, MemoryFrontDoorStore};
    use futures_util::StreamExt;
    use physician_auth::{AuthStore, Membership, MemoryAuthStore, PhysicianIdentity};
    use physician_bus::{Bus, Event, LocalBus};
    use tokio::time::{Instant, sleep_until, timeout};
    use tower::ServiceExt;

    const EMAIL: &str = "colleague@codahealth.test";
    // tokio's timers fire at millisecond granularity (`tokio::time::sleep`: "operates at
    // millisecond granularity"), so a beat may land up to 1 ms after its instant.
    const MARGIN: Duration = Duration::from_millis(1);
    let store = Arc::new(MemoryAuthStore::new());
    store.seed_account(
        crate::STAGING_PRACTICE_ID,
        "colleague",
        EMAIL,
        "Colleague Physician",
        false,
    );
    let auth = crate::auth::Auth::over_fixture(
        Arc::clone(&store) as Arc<dyn AuthStore>,
        Arc::new(MemoryFrontDoorStore::new()) as Arc<dyn FrontDoorStore>,
    )
    .await;
    let bus: Arc<dyn Bus> = Arc::new(LocalBus::new(64));
    let app = crate::auth::required_router(
        axum::Router::new().route(
            "/patient-workspaces/events",
            get(live::events).with_state(Arc::clone(&bus)),
        ),
        auth,
    );
    // The reader's panel holds one patient; W is another of the practice's patients.
    let reader = PhysicianIdentity {
        practice_id: crate::STAGING_PRACTICE_ID.to_owned(),
        physician_id: "colleague".to_owned(),
    };
    store
        .set_membership(&reader, crate::WORKSPACE_IDS[0], Membership::Allowed)
        .await
        .unwrap();
    let w = crate::WORKSPACE_IDS[1];
    let token = store
        .begin_sign_in(EMAIL, crate::auth::now())
        .await
        .unwrap();
    let (_, session) = store
        .complete_sign_in(EMAIL, &token, crate::auth::now())
        .await
        .unwrap();

    let request = Request::get("/patient-workspaces/events")
        .header(header::AUTHORIZATION, format!("Bearer {session}"))
        .body(Body::empty())
        .unwrap();
    let response = app.oneshot(request).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let mut body = response.into_body().into_data_stream();
    let opened = Instant::now();

    // Changes for W every 5 s for 35 s.
    let every = Duration::from_secs(5);
    let publisher = tokio::spawn({
        let bus = Arc::clone(&bus);
        async move {
            for n in 1..=7_u32 {
                sleep_until(opened + every * n).await;
                bus.publish(Event::WorkspaceChanged {
                    workspace_id: w.to_owned(),
                })
                .await
                .unwrap();
            }
        }
    });

    let mut last = opened;
    let mut heard = 0;
    while last.duration_since(opened) < every * 7 {
        let chunk = timeout(KEEP_ALIVE + MARGIN, body.next())
            .await
            .expect("the reader heard nothing for longer than the keep-alive interval")
            .expect("the stream stays open")
            .expect("the chunk's bytes");
        assert_eq!(String::from_utf8(chunk.to_vec()).unwrap(), ":\n\n");
        last = Instant::now();
        heard += 1;
    }
    publisher.await.unwrap();
    // Comments at 15, 30 and 45 s; none of W's seven changes reached the reader.
    assert_eq!(heard, 3);
}
