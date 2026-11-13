//! Drives the gallery's chat window in Chromium as a person would: typing in the real composer,
//! pressing its keys, pointing and choosing with the mouse, and scrolling with the wheel. Coda's
//! side is the gallery's script. Each test guards what a person would see break: a turn that never
//! finishes or shows its answer early, progress in the composer, a queued message sent too soon or
//! lost, a stop that sends, a source that opens in the wrong place or moves the reading, a Sources
//! list over its button or the composer, a link to the original that scrolls away, a reply that
//! pulls a reader who had scrolled back, a tall reply that arrives showing its end, a header rule
//! that stops short of a narrow area, a phone's working toolbar on two rows, a small table whose
//! results hide their units behind a sideways scroll, steps that stand open while Coda works, a
//! transcript that jumps or bounces as work arrives, a message sent with Enter that arrives without
//! motion, a move back or a selection that the end pulls away, a press ended on the scrollbar or
//! outside the window that leaves the end unfollowed, a jump to a growing reply that leaps or stops
//! short, a citation preview away from its citation, and a transcript that eases under reduced
//! motion.

// The shared client also carries helpers for journeys this test doesn't walk.
#[allow(dead_code)]
#[path = "../../mobile/tests/common/devtools.rs"]
mod devtools;
mod support;

use std::time::{Duration, Instant};

use devtools::{Browser, Key};
use serde_json::{Value, json};
use support::Gallery;

/// The browser's size around a stage of 1280 × 800, which the scene opens at.
const AROUND_DESKTOP: (u32, u32) = (1440, 1000);

/// How long a scripted run may take: its steps, its answer and slack for a loaded machine.
const RUN_TIME: Duration = Duration::from_secs(30);

const COMPOSER: &str = ".scene-stage .composer-input";
/// A finished answer's Sources button.
const SOURCES_BUTTON: &str = "button.sources-button[popovertarget]";
/// The list the Sources button opens, by its role and name.
const SOURCES_LIST: &str = "[role=dialog][aria-label=Sources]";
const SCROLLER: &str = ".scene-stage .conversation-scroll";
/// The newest turn; the queue can follow it in the feed.
const LAST_TURN: &str = ".scene-stage .scene-turn:nth-last-child(1 of .scene-turn)";

/// What the composer shows besides the words being written: any other words, how many things keep
/// moving in it, and its controls.
const COMPOSER_SHOWS: &str = "(() => { const composer = document.querySelector('.scene-stage .composer'); \
    const rest = composer.cloneNode(true); rest.querySelectorAll('button, textarea').forEach((node) => node.remove()); \
    return { words: rest.textContent.trim(), \
    moving: composer.getAnimations({ subtree: true }).filter((motion) => motion.effect?.getComputedTiming().iterations === Infinity).length, \
    controls: [...composer.querySelectorAll('button')].filter((node) => node.getClientRects().length > 0).map((node) => node.textContent.trim()) }; })()";

/// Records every transition that starts on the page from its first moment, so entry motion can be
/// told apart from rest.
const RECORD_MOTION: &str = "window.motion = []; \
    document.addEventListener('transitionrun', (event) => window.motion.push(event.target), true);";

/// A typed question, sent with the Send button: the person's words join the transcript, Coda's
/// steps arrive one at a time with the newest running, folded under one line, no answer words show
/// until the answer arrives whole with Sources and Basis, and the steps stay folded. While Coda
/// works the composer holds
/// only its controls, with no words of the work and nothing moving. The new turn enters with
/// motion; the earlier turn, there from the start, never runs an entry.
#[test]
#[ignore = "browser qualification; run explicitly with --ignored"]
fn a_turn_shows_steps_one_at_a_time_then_the_whole_answer() {
    let gallery = Gallery::start();
    let mut browser = open_scene(&gallery, AROUND_DESKTOP);

    press(&mut browser, COMPOSER);
    type_words(&mut browser, "What was her potassium at discharge?");
    press(&mut browser, ".scene-stage .composer button[type='submit']");
    wait_true(
        &mut browser,
        &format!(
            "[...document.querySelectorAll('{LAST_TURN} .message-person')].some((m) => m.textContent.includes('potassium at discharge'))"
        ),
        "the question in the transcript",
    );

    // Watch the run until its answer arrives.
    let deadline = Instant::now() + RUN_TIME;
    let mut counts: Vec<u64> = Vec::new();
    loop {
        let seen = browser.page().evaluate(&format!(
            "(() => {{ const turn = document.querySelector('{LAST_TURN}'); \
             const steps = [...turn.querySelectorAll('.work-step')]; \
             return {{ steps: steps.length, runningLast: steps.length > 0 && steps[steps.length - 1].classList.contains('is-running'), \
             running: turn.querySelectorAll('.work-step.is-running').length, \
             working: !!turn.querySelector('.work-trace.is-working'), words: !!turn.querySelector('.answer-words'), \
             stepsShown: turn.querySelector('.work-trace > .disclosure-region')?.getBoundingClientRect().height > 0, \
             composer: {COMPOSER_SHOWS} }}; }})()"
        ));
        if seen["working"] == true {
            assert_eq!(
                seen["stepsShown"], false,
                "the steps stood open while Coda worked"
            );
            assert_eq!(
                seen["composer"],
                json!({ "words": "", "moving": 0, "controls": ["Stop", "Send next⌘↵", "Send now↵"] }),
                "the composer showed more than its controls while Coda worked"
            );
            assert_eq!(
                seen["words"], false,
                "answer words showed while Coda worked"
            );
            assert_eq!(seen["running"], 1, "more than one step ran at once");
            assert_eq!(
                seen["runningLast"], true,
                "the running step isn't the newest"
            );
            let steps = seen["steps"].as_u64().unwrap_or(0);
            if counts.last() != Some(&steps) {
                counts.push(steps);
            }
        } else if seen["words"] == true {
            break;
        }
        assert!(Instant::now() < deadline, "the answer never arrived");
        std::thread::sleep(Duration::from_millis(40));
    }
    assert!(
        counts.len() >= 3,
        "steps didn't arrive one by one: {counts:?}"
    );
    assert!(
        counts.windows(2).all(|pair| pair[1] == pair[0] + 1),
        "steps arrived out of turn: {counts:?}"
    );

    let ended = browser.page().evaluate(&format!(
        "(() => {{ const turn = document.querySelector('{LAST_TURN}'); \
         const controls = [...turn.querySelectorAll('.answer-ending-row > button')].map((c) => c.textContent.trim()); \
         return {{ controls, words: turn.querySelector('.answer-words').textContent, \
         folded: turn.querySelector('.work-trace').dataset.state, steps: turn.querySelectorAll('.work-step').length }}; }})()"
    ));
    assert_eq!(ended["folded"], "closed", "the steps didn't fold");
    assert_eq!(
        ended["steps"].as_u64(),
        counts.last().copied(),
        "the folded line lost steps"
    );
    assert!(
        ended["words"]
            .as_str()
            .is_some_and(|words| words.contains("4.0 mmol/L")),
        "the answer's words are missing: {ended}"
    );
    assert_eq!(
        ended["controls"],
        json!(["Basis and limitations", "Sources3"]),
        "the answer's footer"
    );

    let motion = browser.page().evaluate(
        "(() => { const [earlier, latest] = [document.querySelector('.scene-turn'), document.querySelector('.scene-turn:nth-last-child(1 of .scene-turn)')]; \
         const entry = (target) => target.matches?.('.message, .answer-ending'); \
         return { earlier: window.motion.filter((t) => entry(t) && earlier.contains(t)).length, \
         latest: window.motion.filter((t) => entry(t) && latest.contains(t)).length }; })()",
    );
    assert!(
        motion["latest"].as_u64() > Some(0),
        "the new turn ran no entry, so the recorder saw nothing: {motion}"
    );
    assert_eq!(
        motion["earlier"], 0,
        "the earlier turn ran an entry it should stand at rest for"
    );
    assert_eq!(browser.take_page_errors(), Vec::<String>::new());
}

/// Send next while Coda works queues the message under the answer, not in the transcript; once the
/// answer arrives the queue is sent as the next turn and Coda starts on it.
#[test]
#[ignore = "browser qualification; run explicitly with --ignored"]
fn send_next_queues_until_the_answer_then_sends() {
    let gallery = Gallery::start();
    let mut browser = open_scene(&gallery, AROUND_DESKTOP);

    ask(&mut browser, "What was her potassium at discharge?");
    wait_working(&mut browser);
    type_words(&mut browser, "And her kidney function?");
    press_enter(&mut browser, Modifier::Meta);

    wait_true(
        &mut browser,
        "document.querySelector('.scene-stage .conversation-feed > .message-queue')?.textContent.includes('kidney function') === true",
        "the queued message under the answer",
    );
    assert_eq!(
        browser
            .page()
            .evaluate(&sent_count("kidney function"))
            .as_u64(),
        Some(0),
        "a queued message was sent at once"
    );
    assert_eq!(
        browser
            .page()
            .evaluate("document.querySelector('.scene-stage .chat-dock .message-queue') === null"),
        Value::Bool(true),
        "the queue stood in the dock by default"
    );
    assert_eq!(
        browser
            .page()
            .evaluate(&format!("document.querySelector('{COMPOSER}').value")),
        "",
        "the queued words stayed in the composer"
    );

    wait_true_for(
        &mut browser,
        &format!(
            "(() => {{ const turn = document.querySelector('{LAST_TURN}'); \
             return turn.textContent.includes('kidney function') && !!turn.querySelector('.work-trace.is-working'); }})()"
        ),
        "the queued message sent as the next turn, with Coda working on it",
        RUN_TIME,
    );
    assert_eq!(
        browser
            .page()
            .evaluate(&sent_count("kidney function"))
            .as_u64(),
        Some(1)
    );
    assert_eq!(
        browser
            .page()
            .evaluate("document.querySelector('.scene-stage .message-queue') === null"),
        Value::Bool(true),
        "the queue outlived its sending"
    );
    assert_eq!(browser.take_page_errors(), Vec::<String>::new());
}

/// Stop while a message is queued: the answer shows as stopped with Resume and nothing of a
/// finished answer, the queued words are back in the composer, the queue is gone, and nothing was
/// sent, even after the run would have finished.
#[test]
#[ignore = "browser qualification; run explicitly with --ignored"]
fn stop_returns_the_queue_to_the_composer_and_sends_nothing() {
    let gallery = Gallery::start();
    let mut browser = open_scene(&gallery, AROUND_DESKTOP);

    ask(&mut browser, "What was her potassium at discharge?");
    wait_working(&mut browser);
    type_words(&mut browser, "And her kidney function?");
    press_enter(&mut browser, Modifier::Control);
    wait_true(
        &mut browser,
        "!!document.querySelector('.scene-stage .message-queue')",
        "the queued message",
    );
    press_button(&mut browser, ".scene-stage .composer", "Stop");

    wait_true(
        &mut browser,
        &format!("!!document.querySelector('{LAST_TURN} .answer-stopped')"),
        "the stopped ending",
    );
    let stopped = browser.page().evaluate(&format!(
        "(() => {{ const turn = document.querySelector('{LAST_TURN}'); \
         return {{ resume: [...turn.querySelectorAll('button')].some((b) => b.textContent.trim() === 'Resume'), \
         words: !!turn.querySelector('.answer-words'), controls: turn.querySelectorAll('.answer-ending-row').length, \
         marks: turn.querySelectorAll('.citation-chip').length, \
         composer: document.querySelector('{COMPOSER}').value, \
         queue: !!document.querySelector('.scene-stage .message-queue') }}; }})()"
    ));
    assert_eq!(
        stopped,
        json!({ "resume": true, "words": false, "controls": 0, "marks": 0,
            "composer": "And her kidney function?", "queue": false }),
        "the stopped turn"
    );
    // Long enough for the run and a queued turn to have gone on had Stop not held them.
    std::thread::sleep(Duration::from_secs(8));
    assert_eq!(
        browser
            .page()
            .evaluate(&sent_count("kidney function"))
            .as_u64(),
        Some(0),
        "Stop sent the queued message"
    );
    assert_eq!(
        browser.page().evaluate(&format!(
            "!!document.querySelector('{LAST_TURN} .answer-stopped')"
        )),
        Value::Bool(true),
        "the stopped answer went on"
    );
    assert_eq!(browser.take_page_errors(), Vec::<String>::new());
}

/// Choosing a citation where there is room opens its source beside the transcript, marked at the
/// cited passage and in sight; the reading doesn't move as the transcript narrows; closing gives
/// focus back to the citation.
#[test]
#[ignore = "browser qualification; run explicitly with --ignored"]
fn a_citation_opens_its_source_beside_the_reading_without_moving_it() {
    let gallery = Gallery::start();
    let mut browser = open_scene(&gallery, AROUND_DESKTOP);

    // Two turns, then scrolled back, so the reader is in the middle of the transcript.
    ask(&mut browser, "What was her potassium at discharge?");
    wait_answer(&mut browser);
    // The transcript eases to the answer; what is pressed next stands still first.
    wait_at_end(&mut browser);
    scroll_back(&mut browser, 360.0, 150.0);
    let before = top_turn(&mut browser);

    let (x, y) = visible_citation(&mut browser, 1);
    click_at(&mut browser, x, y);
    wait_true(
        &mut browser,
        "!!document.querySelector('.scene-stage .source-reading-pane .source-text mark')",
        "the source, read, at its passage",
    );
    std::thread::sleep(Duration::from_millis(500));
    let opened = browser.page().evaluate(
        "(() => { const stage = document.querySelector('.scene-stage'); \
         const main = stage.querySelector('.source-reading-main'); \
         const pane = stage.querySelector('.source-reading-pane').getBoundingClientRect(); \
         const body = stage.querySelector('.source-pane-body').getBoundingClientRect(); \
         const mark = stage.querySelector('.source-pane-body mark').getBoundingClientRect(); \
         return { beside: pane.left >= main.getBoundingClientRect().right - 1 && getComputedStyle(main).visibility === 'visible', \
         markInSight: mark.top >= body.top && mark.bottom <= body.bottom }; })()",
    );
    assert_eq!(
        opened,
        json!({ "beside": true, "markInSight": true }),
        "the open source"
    );
    let after = top_turn(&mut browser);
    assert_eq!(after.0, before.0, "another turn came to the top");
    assert!(
        (after.1 - before.1).abs() <= 2.0,
        "the turn at the top moved from {} to {}",
        before.1,
        after.1
    );

    press(&mut browser, ".scene-stage .source-pane-close");
    wait_true(
        &mut browser,
        "document.querySelector('.scene-stage .source-reading-pane') === null",
        "the source to close",
    );
    wait_true(
        &mut browser,
        "document.activeElement === window.chosen",
        "focus back on the citation",
    );
    assert_eq!(browser.take_page_errors(), Vec::<String>::new());
}

/// Where there isn't room, the source covers the transcript and offers Back.
#[test]
#[ignore = "browser qualification; run explicitly with --ignored"]
fn without_room_the_source_covers_the_transcript_with_back() {
    let gallery = Gallery::start();
    let mut browser = open_scene(&gallery, (1100, 1000));
    browser.page().evaluate(
        "document.querySelector('.scene-tools').scrollIntoView({ block: 'start', behavior: 'instant' }); 0",
    );
    press_text(&mut browser, ".scene-tools .segmented-option", "900 × 800");
    wait_true(
        &mut browser,
        "document.querySelector('.scene-stage').getBoundingClientRect().width === 900",
        "the stage at 900 wide",
    );
    // Up to the earlier answer's citation of its discharge summary.
    let (x, y) = middle(&mut browser, SCROLLER);
    wheel(&mut browser, x, y, -1200.0);
    std::thread::sleep(Duration::from_millis(300));
    let (x, y) = visible_citation(&mut browser, 1);
    click_at(&mut browser, x, y);
    wait_true(
        &mut browser,
        "!!document.querySelector('.scene-stage .source-reading-pane .source-text mark')",
        "the source, read",
    );
    let covered = browser.page().evaluate(
        "(() => { const stage = document.querySelector('.scene-stage'); \
         const back = stage.querySelector('.source-pane-back'); \
         return { back: back.getClientRects().length > 0 && back.textContent.trim(), \
         reader: getComputedStyle(stage.querySelector('.source-reading-main')).visibility }; })()",
    );
    assert_eq!(
        covered,
        json!({ "back": "Back", "reader": "hidden" }),
        "the source over the transcript"
    );
    assert_eq!(browser.take_page_errors(), Vec::<String>::new());
}

/// A reader scrolled back while a reply arrives stays where they are; the way to the new reply
/// stands centred on the transcript and takes them to the reply's top, focused. At 1280 and 390.
#[test]
#[ignore = "browser qualification; run explicitly with --ignored"]
fn a_reply_leaves_a_reader_who_scrolled_back_and_jump_reaches_it() {
    let gallery = Gallery::start();
    for (size, width) in [("1280 × 800", 1280), ("390 × 800", 390)] {
        let mut browser = open_scene(&gallery, AROUND_DESKTOP);
        choose_stage(&mut browser, size, width);

        ask(&mut browser, "What was her potassium at discharge?");
        wait_working(&mut browser);
        scroll_back(&mut browser, 600.0, 150.0);
        let before = top_turn(&mut browser);

        wait_true_for(
            &mut browser,
            &format!("!!document.querySelector('{LAST_TURN} .answer-words')"),
            "the reply",
            RUN_TIME,
        );
        wait_true(
            &mut browser,
            "!!document.querySelector('.scene-stage .jump-to-latest')",
            "the way to the new reply",
        );
        let after = top_turn(&mut browser);
        assert_eq!(after.0, before.0, "the reply moved the reading at {width}");
        assert!(
            (after.1 - before.1).abs() <= 2.0,
            "the reading moved from {} to {} at {width}",
            before.1,
            after.1
        );
        let centred = browser.page().evaluate(&format!(
            "(() => {{ const jump = document.querySelector('.scene-stage .jump-to-latest').getBoundingClientRect(); \
             const region = document.querySelector('{SCROLLER}').getBoundingClientRect(); \
             return Math.abs((jump.left + jump.right) / 2 - (region.left + region.right) / 2) <= 1; }})()"
        ));
        assert_eq!(
            centred,
            Value::Bool(true),
            "the jump control isn't centred on the transcript at {width}"
        );

        press(&mut browser, ".scene-stage .jump-to-latest");
        wait_true(
            &mut browser,
            &format!(
                "(() => {{ const scroller = document.querySelector('{SCROLLER}').getBoundingClientRect(); \
                 const replies = document.querySelectorAll('{SCROLLER} .message-assistant'); const reply = replies[replies.length - 1]; \
                 const top = reply.getBoundingClientRect().top; \
                 return document.activeElement === reply && top >= scroller.top - 2 && top <= scroller.bottom - 48; }})()"
            ),
            "the new reply's top in view, focused",
        );
        assert_eq!(browser.take_page_errors(), Vec::<String>::new());
    }
}

/// A reply taller than the transcript, arriving while the reader follows the end, shows from its
/// top: its top stands at the transcript's top once motion ends, short of the end, with no way to
/// the new reply offered and focus still in the composer. Scrolled to the end again, the next
/// question's working line is in sight. The stage at 390 × 500.
#[test]
#[ignore = "browser qualification; run explicitly with --ignored"]
fn a_reply_taller_than_the_transcript_arrives_at_its_top() {
    let gallery = Gallery::start();
    let mut browser = open_scene(&gallery, AROUND_DESKTOP);
    choose_stage(&mut browser, "390 × 800", 390);
    browser.page().evaluate(
        "(() => { const stage = document.querySelector('.scene-stage'); stage.style.height = '500px'; \
         stage.scrollIntoView({ block: 'start', behavior: 'instant' }); return 0; })()",
    );
    std::thread::sleep(Duration::from_millis(300));

    ask(&mut browser, "What was her potassium at discharge?");
    wait_working(&mut browser);
    wait_answer(&mut browser);
    std::thread::sleep(Duration::from_millis(1200));
    let placed = browser.page().evaluate(&format!(
        "(() => {{ const scroller = document.querySelector('{SCROLLER}'); \
         const replies = scroller.querySelectorAll('.message-assistant'); const reply = replies[replies.length - 1]; \
         const box = reply.getBoundingClientRect(); \
         return {{ taller: box.height > scroller.clientHeight, \
           atTop: Math.abs(box.top - scroller.getBoundingClientRect().top) <= 1, \
           atEnd: scroller.scrollHeight - scroller.clientHeight - scroller.scrollTop <= 1, \
           jump: !!document.querySelector('.scene-stage .jump-to-latest'), \
           composerFocused: document.activeElement === document.querySelector('{COMPOSER}') }}; }})()"
    ));
    assert_eq!(
        placed,
        json!({ "taller": true, "atTop": true, "atEnd": false, "jump": false, "composerFocused": true }),
        "a reply taller than the transcript, as it arrives"
    );

    let (x, y) = middle(&mut browser, SCROLLER);
    wheel(&mut browser, x, y, 1200.0);
    wait_at_end(&mut browser);
    ask(&mut browser, "Show me the whole panel.");
    wait_working(&mut browser);
    wait_true(
        &mut browser,
        &format!(
            "(() => {{ const view = document.querySelector('{SCROLLER}').getBoundingClientRect(); \
             const line = document.querySelector('{LAST_TURN} .work-trace-line').getBoundingClientRect(); \
             return line.top >= view.top && line.bottom <= view.bottom; }})()"
        ),
        "the next working line in sight",
    );
    assert_eq!(browser.take_page_errors(), Vec::<String>::new());
}

/// Choosing the second source in an answer's Sources list opens that source in the pane.
#[test]
#[ignore = "browser qualification; run explicitly with --ignored"]
fn the_sources_list_opens_the_source_chosen() {
    let gallery = Gallery::start();
    let mut browser = open_scene(&gallery, AROUND_DESKTOP);

    ask(&mut browser, "What was her potassium at discharge?");
    wait_answer(&mut browser);
    // The transcript eases to the answer; what is pressed next stands still first.
    wait_at_end(&mut browser);
    press(&mut browser, &format!("{LAST_TURN} {SOURCES_BUTTON}"));
    wait_sources_list(&mut browser);
    press(
        &mut browser,
        &format!("{LAST_TURN} .sources-index li:nth-child(2) button"),
    );
    wait_true(
        &mut browser,
        "!!document.querySelector('.scene-stage .source-reading-pane .record-fields mark')",
        "the chosen source, read",
    );
    let pane = browser.page().evaluate(
        "(() => { const pane = document.querySelector('.scene-stage .source-reading-pane'); \
         return { eyebrow: pane.querySelector('.source-pane-heading small').textContent, \
         title: pane.querySelector('h2').textContent, \
         marked: pane.querySelector('.record-fields mark').closest('.record-field').querySelector('dt').textContent, \
         list: !!document.querySelector('.scene-stage .popover:popover-open') }; })()",
    );
    assert_eq!(
        pane,
        json!({ "eyebrow": "Source 2", "title": "Basic metabolic panel", "marked": "Potassium", "list": false }),
        "the source the list opened"
    );
    assert_eq!(browser.take_page_errors(), Vec::<String>::new());
}

/// With its button in the lower half of the window, the newest answer's Sources list opens above
/// it, its left edge at the button's, over neither the button nor the composer. Tab reaches its
/// rows, and Escape closes it with focus back on the button.
#[test]
#[ignore = "browser qualification; run explicitly with --ignored"]
fn a_low_sources_list_opens_above_its_button_and_clear_of_the_composer() {
    let gallery = Gallery::start();
    let mut browser = open_scene(&gallery, AROUND_DESKTOP);

    ask(&mut browser, "What was her potassium at discharge?");
    wait_answer(&mut browser);
    // The transcript eases to the answer; what is pressed next stands still first.
    wait_at_end(&mut browser);
    std::thread::sleep(Duration::from_millis(600));
    press(&mut browser, &format!("{LAST_TURN} {SOURCES_BUTTON}"));
    wait_sources_list(&mut browser);
    std::thread::sleep(Duration::from_millis(400));
    let placed = browser.page().evaluate(&format!(
        "(() => {{ const turn = document.querySelector('{LAST_TURN}'); \
         const button = turn.querySelector('{SOURCES_BUTTON}').getBoundingClientRect(); \
         const list = turn.querySelector('{SOURCES_LIST}').getBoundingClientRect(); \
         const composer = document.querySelector('.scene-stage .composer').getBoundingClientRect(); \
         return {{ low: (button.top + button.bottom) / 2 > innerHeight / 2, above: list.bottom <= button.top, \
         clearOfComposer: list.bottom <= composer.top, inWindow: list.top >= 0, \
         alignedLeft: Math.abs(list.left - button.left) <= 1 }}; }})()"
    ));
    assert_eq!(
        placed,
        json!({ "low": true, "above": true, "clearOfComposer": true, "inWindow": true, "alignedLeft": true }),
        "the Sources list beside its low button"
    );

    browser.press_key(Key::TAB);
    assert_eq!(
        browser.page().evaluate(&format!(
            "!!document.activeElement.closest('{LAST_TURN} {SOURCES_LIST}')"
        )),
        Value::Bool(true),
        "Tab didn't reach the list's rows"
    );
    browser.press_key(Key::ESCAPE);
    wait_true(
        &mut browser,
        &format!(
            "!document.querySelector('{LAST_TURN} {SOURCES_LIST}:popover-open') \
             && document.activeElement === document.querySelector('{LAST_TURN} {SOURCES_BUTTON}')"
        ),
        "Escape to close the list, focus back on its button",
    );
    assert_eq!(browser.take_page_errors(), Vec::<String>::new());
}

/// A source whose record links to its own system keeps that link at the pane's foot, below the
/// body, in the same place while the body scrolls to its end.
#[test]
#[ignore = "browser qualification; run explicitly with --ignored"]
fn the_link_to_the_original_stays_put_while_the_source_scrolls() {
    let gallery = Gallery::start();
    let mut browser = open_scene(&gallery, AROUND_DESKTOP);

    ask(&mut browser, "What was her potassium at discharge?");
    wait_answer(&mut browser);
    // The transcript eases to the answer; what is pressed next stands still first.
    wait_at_end(&mut browser);
    press(&mut browser, &format!("{LAST_TURN} {SOURCES_BUTTON}"));
    wait_sources_list(&mut browser);
    press(
        &mut browser,
        &format!("{LAST_TURN} {SOURCES_LIST} li:nth-child(2) button"),
    );
    wait_true(
        &mut browser,
        "!!document.querySelector('.scene-stage .source-reading-pane .record-fields mark')",
        "the panel, read",
    );
    // Its technical details open, so the body runs past the pane.
    press_button(
        &mut browser,
        ".scene-stage .source-pane-body",
        "Technical details",
    );
    std::thread::sleep(Duration::from_millis(500));
    let before = original_link(&mut browser);
    let (x, y) = middle(&mut browser, ".scene-stage .source-pane-body");
    wheel(&mut browser, x, y, 2400.0);
    wait_true(
        &mut browser,
        "(() => { const body = document.querySelector('.scene-stage .source-pane-body'); \
         return body.scrollHeight - body.clientHeight > 0 && body.scrollTop >= body.scrollHeight - body.clientHeight - 1; })()",
        "the body scrolled to its end",
    );
    std::thread::sleep(Duration::from_millis(300));
    let after = original_link(&mut browser);
    assert_eq!(
        before["belowBody"],
        Value::Bool(true),
        "the link isn't under the body: {before}"
    );
    assert_eq!(
        after, before,
        "the link moved or left the pane as the body scrolled"
    );
    assert_eq!(browser.take_page_errors(), Vec::<String>::new());
}

/// The header's hairline spans the column the composer stands in, and where the chat area is
/// narrower than the reading measure it runs from edge to edge of the area, its words inset to the
/// composer's edges. Room stands above it. On a phone its facts wrap, each whole.
#[test]
#[ignore = "browser qualification; run explicitly with --ignored"]
fn the_header_rule_spans_the_column_and_a_narrow_area_edge_to_edge() {
    let gallery = Gallery::start();
    let mut browser = open_scene(&gallery, AROUND_DESKTOP);
    let wide = header_against_area(&mut browser);
    assert_eq!(
        wide,
        json!({ "ruleOnDock": true, "edgeToEdge": false, "wordsOnDock": true, "roomAbove": true }),
        "the header at 1280"
    );

    choose_stage(&mut browser, "900 × 800", 900);
    let narrow = header_against_area(&mut browser);
    assert_eq!(
        narrow,
        json!({ "ruleOnDock": false, "edgeToEdge": true, "wordsOnDock": true, "roomAbove": true }),
        "the header at 900"
    );

    // On a phone the facts wrap, each whole, set apart by a gap, the label quieter than its value.
    choose_stage(&mut browser, "390 × 800", 390);
    let facts = header_facts(&mut browser);
    assert_eq!(
        facts,
        json!({
            "words": ["77 years", "DOB Mar 3, 1947", "Female", "MRN 00004471"],
            "wraps": true, "eachOneLine": true, "separators": false, "labelsQuieter": true,
        }),
        "the header's facts at 390"
    );
    assert_eq!(browser.take_page_errors(), Vec::<String>::new());
}

/// On a touch phone, while Coda works, the composer's controls stand in one row with no key hints.
#[test]
#[ignore = "browser qualification; run explicitly with --ignored"]
fn on_a_touch_phone_the_working_toolbar_is_one_row_without_key_hints() {
    let gallery = Gallery::start();
    let mut browser = open_scene(&gallery, AROUND_DESKTOP);
    browser.page().call_session(
        "Emulation.setTouchEmulationEnabled",
        json!({ "enabled": true, "maxTouchPoints": 5 }),
    );
    choose_stage(&mut browser, "390 × 800", 390);
    ask(&mut browser, "What was her potassium at discharge?");
    wait_working(&mut browser);
    let toolbar = browser.page().evaluate(
        "(() => { const bar = document.querySelector('.scene-stage .composer-toolbar'); \
         const shown = [...bar.querySelectorAll('button')].filter((node) => node.getClientRects().length > 0); \
         const middles = new Set(shown.map((node) => { const box = node.getBoundingClientRect(); return Math.round(box.top + box.height / 2); })); \
         return { touch: matchMedia('(pointer: coarse)').matches, rows: middles.size, \
         hints: [...bar.querySelectorAll('.key-hint')].filter((node) => node.getClientRects().length > 0).length, \
         controls: shown.map((node) => node.innerText.trim()) }; })()",
    );
    assert_eq!(
        toolbar,
        json!({ "touch": true, "rows": 1, "hints": 0, "controls": ["Stop", "Send next", "Send now"] }),
        "the working toolbar on a phone"
    );
    assert_eq!(browser.take_page_errors(), Vec::<String>::new());
}

/// At 390 wide, an answer's small table fits its column: each result reads whole with its unit on
/// one line, inside the frame, lined up at the column's end, and nothing scrolls sideways.
#[test]
#[ignore = "browser qualification; run explicitly with --ignored"]
fn a_small_table_answer_shows_each_result_with_its_unit_at_390() {
    let gallery = Gallery::start();
    let mut browser = open_scene(&gallery, AROUND_DESKTOP);
    choose_stage(&mut browser, "390 × 800", 390);
    ask(&mut browser, "What was her potassium at discharge?");
    wait_answer(&mut browser);
    ask(&mut browser, "What else was on that panel?");
    wait_answer(&mut browser);
    let frame = format!("{LAST_TURN} .answer-words .prose-table-scroll");
    wait_true(
        &mut browser,
        &format!("!!document.querySelector('{frame}')"),
        "the table answer",
    );
    browser.page().evaluate(&format!(
        "document.querySelector('{frame}').scrollIntoView({{ block: 'center', behavior: 'instant' }}); 0"
    ));
    std::thread::sleep(Duration::from_millis(300));
    // A result counts as read when its words stand on one line, wholly inside the frame.
    let shown = browser.page().evaluate(&format!(
        "(() => {{ const frame = document.querySelector('{frame}'); const edge = frame.getBoundingClientRect(); \
         const scroller = document.querySelector('{SCROLLER}'); \
         const lines = [...frame.querySelectorAll('td:last-child')].map((cell) => {{ \
           const range = document.createRange(); range.selectNodeContents(cell); \
           return {{ words: cell.textContent.trim(), rects: [...range.getClientRects()].filter((box) => box.width > 0) }}; }}); \
         return {{ read: lines.filter(({{ rects }}) => new Set(rects.map((box) => Math.round(box.bottom))).size === 1 \
           && rects.every((box) => box.left >= edge.left && box.right <= edge.right)) \
           .map(({{ words }}) => words), \
         linedUp: new Set(lines.map(({{ rects }}) => Math.round(rects[rects.length - 1].right))).size === 1, \
         tableAcross: frame.scrollWidth > frame.clientWidth, transcriptAcross: scroller.scrollWidth > scroller.clientWidth }}; }})()"
    ));
    assert_eq!(
        shown,
        json!({ "read": ["138 mmol/L", "4.0 mmol/L", "4.0 mmol/L", "124 µmol/L"], "linedUp": true,
            "tableAcross": false, "transcriptAcross": false }),
        "the table's results at 390"
    );
    assert_eq!(browser.take_page_errors(), Vec::<String>::new());
}

/// While Coda works, pressing the line opens its steps; they stay open as more steps arrive and
/// once the answer arrives, and nothing in sight jumps or bounces as it does. At 1280 and 900, and
/// at 390.
#[test]
#[ignore = "browser qualification; run explicitly with --ignored"]
fn the_steps_the_person_opened_stay_open_and_still_as_the_answer_arrives() {
    let gallery = Gallery::start();
    let mut seen = Vec::new();
    for (size, width) in [("1280 × 800", 1280), ("900 × 800", 900), ("390 × 800", 390)] {
        let mut browser = open_scene(&gallery, AROUND_DESKTOP);
        choose_stage(&mut browser, size, width);
        send_by_pointer(&mut browser, "What was her potassium at discharge?");
        wait_working(&mut browser);
        wait_settled(&mut browser);
        press(&mut browser, &format!("{LAST_TURN} .work-trace-line"));
        let state = format!("document.querySelector('{LAST_TURN} .work-trace').dataset.state");
        wait_true(
            &mut browser,
            &format!("{state} === 'open'"),
            "the steps to open",
        );
        let first = browser
            .page()
            .evaluate(&format!(
                "document.querySelectorAll('{LAST_TURN} .work-step').length"
            ))
            .as_u64()
            .unwrap_or(0);
        wait_true(
            &mut browser,
            &format!("document.querySelectorAll('{LAST_TURN} .work-step').length > {first}"),
            "another step",
        );
        let working = browser.page().evaluate(&state);
        std::thread::sleep(Duration::from_millis(500));
        let arrival = moves_through(&mut browser, wait_answer);
        let ended = browser.page().evaluate(&state);
        seen.push(json!({ "width": width, "whileWorking": working, "arrival": arrival, "afterAnswer": ended }));
        assert_eq!(browser.take_page_errors(), Vec::<String>::new());
    }
    let opened = |width: u32| json!({ "width": width, "whileWorking": "open", "arrival": still(), "afterAnswer": "open" });
    assert_eq!(
        seen,
        vec![opened(1280), opened(900), opened(390)],
        "steps the person opened, as the answer arrives"
    );
}

/// With the reader at the end, nothing already in sight jumps or bounces as work arrives: the
/// last turn's top and its work line move by small steps in one direction while the answer arrives
/// under the working line, while Stop puts its ending with Resume, and while the person opens and
/// closes the steps, and the reader ends at the end. A frame that carries most of a move at once
/// is a jump; a move back after a move is a bounce. Driven by pointer.
#[test]
#[ignore = "browser qualification; run explicitly with --ignored"]
fn the_transcript_moves_without_jumps_as_work_arrives() {
    let gallery = Gallery::start();
    let mut browser = open_scene(&gallery, AROUND_DESKTOP);

    send_by_pointer(&mut browser, "What was her potassium at discharge?");
    wait_working(&mut browser);
    let arrival = moves_through(&mut browser, wait_answer);
    assert_eq!(
        arrival,
        still(),
        "the answer arriving under the working line"
    );

    send_by_pointer(&mut browser, "Has her weight changed since?");
    wait_working(&mut browser);
    std::thread::sleep(Duration::from_millis(700));
    let stop = moves_through(&mut browser, |browser| {
        press_button(browser, ".scene-stage .composer", "Stop")
    });
    assert_eq!(stop, still(), "Stop putting its ending with Resume");

    let line = format!("{LAST_TURN} .work-trace-line");
    let opening = moves_through(&mut browser, |browser| press(browser, &line));
    assert_eq!(opening, still(), "the person opening the steps");
    let closing = moves_through(&mut browser, |browser| press(browser, &line));
    assert_eq!(closing, still(), "the person closing the steps");
    assert_eq!(browser.take_page_errors(), Vec::<String>::new());
}

/// A message typed and sent with Enter, the pointer never moved, arrives as one sent by pointer
/// does: the sent bubble, the work line and the answer each run their entry, and the transcript
/// eases to them without a jump or a bounce.
#[test]
#[ignore = "browser qualification; run explicitly with --ignored"]
fn a_message_sent_with_enter_arrives_with_motion() {
    let gallery = Gallery::start();
    let mut browser = open_scene(&gallery, AROUND_DESKTOP);
    browser
        .page()
        .evaluate(&format!("document.querySelector('{COMPOSER}').focus(); 0"));
    type_words(&mut browser, "What was her potassium at discharge?");
    let sent = moves_through(&mut browser, |browser| {
        browser.press_key(Key::ENTER);
        wait_working(browser);
    });
    let answer = moves_through(&mut browser, wait_answer);
    let entries = browser.page().evaluate(&format!(
        "(() => {{ const turn = document.querySelector('{LAST_TURN}'); \
         const ran = (selector) => window.motion.some((target) => target === turn.querySelector(selector)); \
         return {{ bubble: ran('.message-person'), line: ran('.work-trace'), answer: ran('.answer-ending') }}; }})()"
    ));
    assert_eq!(
        json!({ "sent": sent, "answer": answer, "entries": entries }),
        json!({ "sent": still_by("keyboard"), "answer": still_by("keyboard"),
            "entries": { "bubble": true, "line": true, "answer": true } }),
        "a message sent with Enter"
    );
    assert_eq!(browser.take_page_errors(), Vec::<String>::new());
}

/// A move back lets go of the end at once, by intent rather than distance: a wheel turned back
/// with content growing in the same frame, before any scroll lands, leaves the reader where they
/// were; and after one ArrowUp pressed in the transcript, growth at the end leaves them there too.
#[test]
#[ignore = "browser qualification; run explicitly with --ignored"]
fn a_move_back_lets_go_even_when_content_grows_in_the_same_frame() {
    let gallery = Gallery::start();
    let mut browser = open_scene(&gallery, AROUND_DESKTOP);
    send_by_pointer(&mut browser, "What was her potassium at discharge?");
    wait_answer(&mut browser);
    wait_at_end(&mut browser);
    // The wheel and the growth land in one frame; the wheel's own scroll has not landed yet.
    let wheel_held = browser.page().evaluate(&format!(
        "(() => {{ const scroller = document.querySelector('{SCROLLER}'); \
         scroller.dispatchEvent(new WheelEvent('wheel', {{ deltaY: -100, bubbles: true }})); \
         scroller.firstElementChild.lastElementChild.style.paddingBottom = '240px'; \
         return scroller.scrollTop; }})()"
    ));
    std::thread::sleep(Duration::from_millis(900));
    let after_wheel = scroll_top(&mut browser);
    // Back at the end, then one ArrowUp, and growth after it.
    browser.page().evaluate(&format!(
        "(() => {{ const scroller = document.querySelector('{SCROLLER}'); \
         scroller.firstElementChild.lastElementChild.style.paddingBottom = ''; \
         scroller.scrollTop = scroller.scrollHeight; return 0; }})()"
    ));
    wait_at_end(&mut browser);
    let end = scroll_top(&mut browser);
    // The key pressed with focus in the transcript, on the answer's rating, so the browser scrolls
    // the transcript and the transcript hears the key.
    let focused = browser.page().evaluate(&format!(
        "(() => {{ const rating = [...document.querySelectorAll(\"{SCROLLER} button[aria-label='Good response']\")].at(-1); \
         rating.focus({{ preventScroll: true }}); return document.activeElement === rating; }})()"
    ));
    assert_eq!(focused, json!(true), "focus on the answer's rating");
    browser.press_key(Key::ARROW_UP);
    std::thread::sleep(Duration::from_millis(300));
    let key_held = scroll_top(&mut browser);
    browser.page().evaluate(&format!(
        "document.querySelector('{SCROLLER}').firstElementChild.lastElementChild.style.paddingBottom = '240px'; 0"
    ));
    std::thread::sleep(Duration::from_millis(900));
    let after_key = scroll_top(&mut browser);
    let key_moved_back = matches!((end, key_held), (Some(end), Some(held)) if held < end);
    assert_eq!(
        json!({ "wheel": after_wheel, "keyMovedBack": key_moved_back, "key": after_key }),
        json!({ "wheel": wheel_held.as_f64().map(f64::round), "keyMovedBack": true, "key": key_held }),
        "where the reader stood after moving back, with growth"
    );
    assert_eq!(browser.take_page_errors(), Vec::<String>::new());
}

/// The transcript's scroll position, rounded.
fn scroll_top(browser: &mut Browser) -> Option<f64> {
    browser
        .page()
        .evaluate(&format!("document.querySelector('{SCROLLER}').scrollTop"))
        .as_f64()
        .map(f64::round)
}

/// While the person is selecting words in the transcript, growth at the end does not pull the
/// view; once they let go, the transcript follows the end again.
#[test]
#[ignore = "browser qualification; run explicitly with --ignored"]
fn a_selection_being_made_is_not_pulled_away_by_growth() {
    let gallery = Gallery::start();
    let mut browser = open_scene(&gallery, AROUND_DESKTOP);
    send_by_pointer(&mut browser, "What was her potassium at discharge?");
    wait_answer(&mut browser);
    wait_at_end(&mut browser);
    let start = browser.page().evaluate(&format!(
        "(() => {{ const words = document.querySelector('{LAST_TURN} .answer-words p'); \
         const range = document.createRange(); range.selectNodeContents(words); \
         const box = range.getClientRects()[0]; return [box.left + 4, box.top + box.height / 2]; }})()"
    ));
    let (x, y) = (start[0].as_f64().unwrap(), start[1].as_f64().unwrap());
    mouse(&mut browser, "mouseMoved", x, y, 0);
    mouse(&mut browser, "mousePressed", x, y, 1);
    for step in 1..=3 {
        mouse(&mut browser, "mouseMoved", x + 20.0 * f64::from(step), y, 1);
    }
    let held = browser.page().evaluate(&format!(
        "(() => {{ const scroller = document.querySelector('{SCROLLER}'); \
         scroller.firstElementChild.lastElementChild.style.paddingBottom = '240px'; return scroller.scrollTop; }})()"
    ));
    std::thread::sleep(Duration::from_millis(900));
    let during = browser.page().evaluate(&format!(
        "(() => {{ const selection = getSelection(); \
         return [document.querySelector('{SCROLLER}').scrollTop, !selection.isCollapsed]; }})()"
    ));
    mouse(&mut browser, "mouseReleased", x + 60.0, y, 0);
    std::thread::sleep(Duration::from_millis(900));
    let after = browser.page().evaluate(&format!(
        "(() => {{ const s = document.querySelector('{SCROLLER}'); return s.scrollHeight - s.clientHeight - s.scrollTop <= 1; }})()"
    ));
    assert_eq!(
        json!({ "top": during[0].as_f64().map(f64::round), "selecting": during[1], "followsAfter": after }),
        json!({ "top": held.as_f64().map(f64::round), "selecting": true, "followsAfter": true }),
        "growth while words were being selected"
    );
    assert_eq!(browser.take_page_errors(), Vec::<String>::new());
}

/// A press that ends, however it ends, does not leave following waiting: after the transcript's
/// own scrollbar thumb is dragged up a little and back down to the end, and after a press in the
/// words is dragged out of the window and let go there, growth at the end is followed with no
/// further pointer event. Which pointer events reached the page is part of what it reports.
#[test]
#[ignore = "browser qualification; run explicitly with --ignored"]
fn a_press_that_ends_on_the_scrollbar_or_outside_the_window_leaves_following_free() {
    let gallery = Gallery::start();
    let mut browser = open_scene(&gallery, AROUND_DESKTOP);
    // Headless Chromium hides scrollbars unless asked; shown, they are classic ones that take room.
    browser
        .page()
        .call_session("Emulation.setScrollbarsHidden", json!({ "hidden": false }));
    send_by_pointer(&mut browser, "What was her potassium at discharge?");
    wait_answer(&mut browser);
    wait_at_end(&mut browser);
    browser.page().evaluate(&format!(
        "(() => {{ window.pointerEvents = []; const s = document.querySelector('{SCROLLER}'); \
         for (const kind of ['pointerdown', 'pointerup', 'pointercancel']) addEventListener(kind, (event) => {{ \
           const content = s.getBoundingClientRect().left + s.clientLeft + s.clientWidth; \
           window.pointerEvents.push(`${{event.type}}@${{Math.round(event.clientX)}},${{Math.round(event.clientY)}}${{event.clientX > content ? ' in the scrollbar' : ''}}`); }}, true); \
         return 0; }})()"
    ));
    // The thumb stands at the foot of the track, at the end; its middle is pressed.
    let thumb = browser.page().evaluate(&format!(
        "(() => {{ const s = document.querySelector('{SCROLLER}'); const box = s.getBoundingClientRect(); \
         const content = box.left + s.clientLeft + s.clientWidth; \
         const bar = box.right - parseFloat(getComputedStyle(s).borderRightWidth) - content; \
         const middle = (s.scrollTop + s.clientHeight / 2) / s.scrollHeight * s.clientHeight; \
         return [content + bar / 2, box.top + s.clientTop + middle, bar]; }})()"
    ));
    let (x, y) = (thumb[0].as_f64().unwrap(), thumb[1].as_f64().unwrap());
    let top_before = scroll_top(&mut browser);
    mouse(&mut browser, "mouseMoved", x, y, 0);
    mouse(&mut browser, "mousePressed", x, y, 1);
    for step in 1..=4 {
        mouse(&mut browser, "mouseMoved", x, y - 10.0 * f64::from(step), 1);
    }
    std::thread::sleep(Duration::from_millis(200));
    let dragged_back = scroll_top(&mut browser) < top_before;
    for step in 1..=8 {
        mouse(&mut browser, "mouseMoved", x, y + 40.0 * f64::from(step), 1);
    }
    mouse(&mut browser, "mouseReleased", x, y + 320.0, 0);
    std::thread::sleep(Duration::from_millis(300));
    let thumb_events = browser.page().evaluate("window.pointerEvents.splice(0)");
    let follows = |browser: &mut Browser| {
        browser.page().evaluate(&format!(
            "(() => {{ const s = document.querySelector('{SCROLLER}'); \
             s.firstElementChild.lastElementChild.style.paddingBottom = `${{(parseFloat(s.firstElementChild.lastElementChild.style.paddingBottom) || 0) + 240}}px`; \
             return 0; }})()"
        ));
        std::thread::sleep(Duration::from_millis(900));
        browser.page().evaluate(&format!(
            "(() => {{ const s = document.querySelector('{SCROLLER}'); return s.scrollHeight - s.clientHeight - s.scrollTop <= 1; }})()"
        ))
    };
    let after_thumb = follows(&mut browser);
    // A press in the transcript, dragged up and out of the window and let go there.
    let start = browser.page().evaluate(&format!(
        "(() => {{ const s = document.querySelector('{SCROLLER}'); const box = s.getBoundingClientRect(); \
         return [box.left + s.clientLeft + 40, box.top + s.clientTop + s.clientHeight / 2]; }})()"
    ));
    let (x, y) = (start[0].as_f64().unwrap(), start[1].as_f64().unwrap());
    mouse(&mut browser, "mouseMoved", x, y, 0);
    mouse(&mut browser, "mousePressed", x, y, 1);
    for step in 1..=4 {
        mouse(
            &mut browser,
            "mouseMoved",
            x,
            y - (y + 40.0) * f64::from(step) / 4.0,
            1,
        );
    }
    mouse(&mut browser, "mouseReleased", x, -40.0, 0);
    std::thread::sleep(Duration::from_millis(300));
    let outside_events = browser.page().evaluate("window.pointerEvents.splice(0)");
    browser.page().evaluate(&format!(
        "(() => {{ getSelection().removeAllRanges(); const s = document.querySelector('{SCROLLER}'); s.scrollTop = s.scrollHeight; return 0; }})()"
    ));
    wait_at_end(&mut browser);
    let after_outside = follows(&mut browser);
    let seen = json!({
        "scrollbar": thumb[2], "thumbDraggedBack": dragged_back, "thumbEvents": thumb_events,
        "followsAfterThumb": after_thumb, "outsideEvents": outside_events, "followsAfterOutside": after_outside,
    });
    eprintln!("pointer events seen: {seen}");
    assert_eq!(
        json!({ "thumbDraggedBack": dragged_back, "followsAfterThumb": after_thumb, "followsAfterOutside": after_outside }),
        json!({ "thumbDraggedBack": true, "followsAfterThumb": true, "followsAfterOutside": true }),
        "following after a press that ended on the scrollbar or outside the window; seen: {seen}"
    );
    assert_eq!(browser.take_page_errors(), Vec::<String>::new());
}

/// The jump control moves the transcript to the new reply without a jump, even while the reply
/// keeps growing on the way, and brings its top to the transcript's top; it grows enough that its
/// top can get there. No frame carries more than a third of the way.
#[test]
#[ignore = "browser qualification; run explicitly with --ignored"]
fn the_jump_reaches_a_reply_that_is_still_growing() {
    let gallery = Gallery::start();
    let mut browser = open_scene(&gallery, AROUND_DESKTOP);
    send_by_pointer(&mut browser, "What was her potassium at discharge?");
    wait_working(&mut browser);
    scroll_back(&mut browser, 600.0, 150.0);
    wait_answer(&mut browser);
    wait_true(
        &mut browser,
        "!!document.querySelector('.scene-stage .jump-to-latest')",
        "the way to the new reply",
    );
    // From the press, the reply grows a little every frame for most of a second, and where its top
    // stands is sampled every frame.
    browser.page().evaluate(&format!(
        "(() => {{ const scroller = document.querySelector('{SCROLLER}'); \
         const replies = scroller.querySelectorAll('.message-assistant'); const reply = replies[replies.length - 1]; \
         const until = performance.now() + 700; let grown = 0; window.offsets = []; window.sampling = true; \
         const grow = () => {{ grown += 8; reply.style.paddingBottom = `${{grown}}px`; if (performance.now() < until) requestAnimationFrame(grow); }}; \
         const tick = () => {{ window.offsets.push(reply.getBoundingClientRect().top - scroller.getBoundingClientRect().top); \
           if (window.sampling) requestAnimationFrame(tick); }}; \
         requestAnimationFrame(grow); requestAnimationFrame(tick); return 0; }})()"
    ));
    press(&mut browser, ".scene-stage .jump-to-latest");
    std::thread::sleep(Duration::from_millis(1500));
    let reached = browser.page().evaluate(&format!(
        "(() => {{ window.sampling = false; const offsets = window.offsets; \
         const steps = offsets.slice(1).map((offset, index) => offset - offsets[index]); \
         const replies = document.querySelectorAll('{SCROLLER} .message-assistant'); const reply = replies[replies.length - 1]; \
         return {{ smooth: steps.every((step) => Math.abs(step) <= Math.max(6, offsets[0] / 3)), \
           oneWay: steps.every((step) => step <= 0.5), reached: Math.abs(offsets[offsets.length - 1]) <= 2, \
           focused: document.activeElement === reply }}; }})()"
    ));
    assert_eq!(
        reached,
        json!({ "smooth": true, "oneWay": true, "reached": true, "focused": true }),
        "the jump to a reply that kept growing"
    );
    assert_eq!(browser.take_page_errors(), Vec::<String>::new());
}

/// Opened from a citation in the chat window, its column offset from the window's edge, the
/// preview stands against its citation: just under or over it, overlapping it across, and inside
/// the window.
#[test]
#[ignore = "browser qualification; run explicitly with --ignored"]
fn a_citation_preview_opens_against_its_citation_in_the_chat_column() {
    let gallery = Gallery::start();
    let mut browser = open_scene(&gallery, AROUND_DESKTOP);
    let (x, y) = visible_citation(&mut browser, 1);
    mouse(&mut browser, "mouseMoved", x, y, 0);
    wait_true(
        &mut browser,
        "!!document.querySelector('.scene-stage .citation-preview[data-side]')",
        "the preview",
    );
    std::thread::sleep(Duration::from_millis(500));
    let placed = browser.page().evaluate(
        "(() => { const chip = window.chosen.getBoundingClientRect(); \
         const preview = document.querySelector('.scene-stage .citation-preview[data-side]').getBoundingClientRect(); \
         const column = document.querySelector('.scene-stage .chat-column').getBoundingClientRect(); \
         const gap = preview.top >= chip.bottom ? preview.top - chip.bottom : chip.top - preview.bottom; \
         return { columnOffset: column.left > 100, near: gap >= 0 && gap <= 16, \
           across: preview.left <= chip.right && preview.right >= chip.left, \
           inside: preview.left >= 0 && preview.right <= innerWidth && preview.top >= 0 && preview.bottom <= innerHeight }; })()",
    );
    assert_eq!(
        placed,
        json!({ "columnOffset": true, "near": true, "across": true, "inside": true }),
        "the preview against its citation"
    );
    assert_eq!(browser.take_page_errors(), Vec::<String>::new());
}

/// Under reduced motion the sent message and the answer still fade in, but neither moves: in
/// every frame each stands untranslated and unscaled while its opacity passes through values
/// between none and full. The transcript reaches its end at once as the answer arrives: no frame
/// after the first one that grew stands short of the end.
#[test]
#[ignore = "browser qualification; run explicitly with --ignored"]
fn under_reduced_motion_messages_fade_without_moving_and_the_transcript_moves_at_once() {
    let gallery = Gallery::start();
    let mut browser = open_scene(&gallery, AROUND_DESKTOP);
    browser.emulate_reduced_motion();
    browser.page().evaluate(&format!(
        "(() => {{ const scroller = document.querySelector('{SCROLLER}'); const before = document.querySelector('{LAST_TURN}'); \
         window.seen = {{ bubble: [], answer: [], short: [] }}; window.sampling = true; \
         const look = (node, into) => {{ if (!node) return; const style = getComputedStyle(node); \
           into.push([parseFloat(style.opacity), style.transform === 'none' || style.transform === 'matrix(1, 0, 0, 1, 0, 0)']); }}; \
         const tick = () => {{ const turn = document.querySelector('{LAST_TURN}'); \
           if (turn !== before) {{ look(turn.querySelector('.message-person'), window.seen.bubble); look(turn.querySelector('.answer-ending'), window.seen.answer); }} \
           window.seen.short.push(scroller.scrollHeight - scroller.clientHeight - scroller.scrollTop > 1); \
           if (window.sampling) requestAnimationFrame(tick); }}; \
         requestAnimationFrame(tick); return 0; }})()"
    ));
    send_by_pointer(&mut browser, "What was her potassium at discharge?");
    wait_working(&mut browser);
    browser.page().evaluate("window.seen.short = []; 0");
    wait_answer(&mut browser);
    std::thread::sleep(Duration::from_millis(600));
    let seen = browser.page().evaluate(
        "(() => { window.sampling = false; let run = 0, longest = 0; \
         for (const short of window.seen.short) { run = short ? run + 1 : 0; longest = Math.max(longest, run); } \
         const fades = (frames) => frames.filter(([opacity]) => opacity > 0.05 && opacity < 0.95).length >= 2 \
           && frames[frames.length - 1][0] === 1; \
         const still = (frames) => frames.every(([, unmoved]) => unmoved); \
         return { bubble: { fades: fades(window.seen.bubble), still: still(window.seen.bubble) }, \
           answer: { fades: fades(window.seen.answer), still: still(window.seen.answer) }, transcriptAtOnce: longest <= 1 }; })()",
    );
    assert_eq!(
        seen,
        json!({ "bubble": { "fades": true, "still": true }, "answer": { "fades": true, "still": true }, "transcriptAtOnce": true }),
        "a message and its answer under reduced motion"
    );
    assert_eq!(browser.take_page_errors(), Vec::<String>::new());
}

/// What `moves_through` finds when nothing jumps or bounces and the reader ends in their place,
/// driven by pointer.
fn still() -> Value {
    still_by("pointer")
}

/// What `moves_through` finds when nothing jumps or bounces and the reader ends in their place,
/// the page driven by `interaction`.
fn still_by(interaction: &str) -> Value {
    json!({ "interaction": interaction, "jumps": [], "bounces": [], "inPlace": true })
}

/// Waits until the transcript stands at its end and has stopped moving.
fn wait_at_end(browser: &mut Browser) {
    wait_true(
        browser,
        &format!(
            "(() => {{ const s = document.querySelector('{SCROLLER}'); return s.scrollHeight - s.clientHeight - s.scrollTop <= 1; }})()"
        ),
        "the transcript at its end",
    );
    std::thread::sleep(Duration::from_millis(600));
}

/// One mouse event of `kind` at `x`, `y`, with `buttons` held.
fn mouse(browser: &mut Browser, kind: &str, x: f64, y: f64, buttons: u8) {
    browser.page().call_session(
        "Input.dispatchMouseEvent",
        json!({ "type": kind, "x": x, "y": y, "buttons": buttons,
            "button": if kind == "mouseMoved" && buttons == 0 { "none" } else { "left" },
            "clickCount": if kind == "mouseMoved" { 0 } else { 1 } }),
    );
}

/// Samples the top of the turn that was last when it began, and that turn's work line, on every
/// frame while `act` runs and for a moment after. A move is a run of frames that move; it names
/// each move with a frame that carried most of its travel that way, and each move that turned back.
/// A sampling that starts ends the one before it, which may not yet have seen its own end. The
/// reader's place is the end, or the newest reply's top at the transcript's top when the end would
/// take that top out of sight.
fn moves_through(browser: &mut Browser, act: impl FnOnce(&mut Browser)) -> Value {
    browser.page().evaluate(&format!(
        "(() => {{ const scroller = document.querySelector('{SCROLLER}'); const frames = window.frames = []; window.sampling = true; \
         const turn = document.querySelector('{LAST_TURN}'); \
         const tick = () => {{ const view = scroller.getBoundingClientRect().top; \
           const line = turn?.querySelector('.work-trace-line'); \
           frames.push([turn ? turn.getBoundingClientRect().top - view : null, line ? line.getBoundingClientRect().top - view : null]); \
           if (window.sampling && window.frames === frames) requestAnimationFrame(tick); }}; \
         requestAnimationFrame(tick); return 0; }})()"
    ));
    act(browser);
    std::thread::sleep(Duration::from_millis(900));
    browser.page().evaluate(&format!(
        "(() => {{ window.sampling = false; const scroller = document.querySelector('{SCROLLER}'); \
         const jumps = [], bounces = []; \
         for (const [edge, name] of [[0, 'top'], [1, 'line']]) {{ \
           const at = window.frames.map((frame) => frame[edge]).filter((value) => value !== null); \
           const moves = []; let run = []; \
           at.slice(1).forEach((value, index) => {{ const step = value - at[index]; \
             if (Math.abs(step) >= 1) run.push(step); else if (run.length) {{ moves.push(run); run = []; }} }}); \
           if (run.length) moves.push(run); \
           for (const move of moves) {{ \
             const travel = (sign) => move.filter((step) => Math.sign(step) === sign).reduce((sum, step) => sum + Math.abs(step), 0); \
             const jump = move.find((step) => Math.abs(step) > Math.max(6, travel(Math.sign(step)) * 0.6)); \
             if (jump !== undefined) jumps.push(`${{name}} ${{Math.round(jump)}}px of ${{Math.round(travel(Math.sign(jump)))}}px`); \
             const back = move.find((step) => Math.sign(step) !== Math.sign(move[0])); \
             if (back !== undefined) bounces.push(`${{name}} ${{Math.round(move[0])}}px then ${{Math.round(back)}}px`); }} }} \
         const replies = scroller.querySelectorAll('.message-assistant'); const reply = replies[replies.length - 1]; \
         const top = reply ? reply.getBoundingClientRect().top - scroller.getBoundingClientRect().top : 0; \
         const atEnd = scroller.scrollHeight - scroller.clientHeight - scroller.scrollTop <= 1; \
         const atReply = Math.abs(top) <= 1 && scroller.scrollHeight - scroller.scrollTop - top > scroller.clientHeight; \
         return {{ interaction: document.documentElement.dataset.interaction, jumps, bounces, inPlace: atEnd || atReply }}; }})()"
    ))
}

/// Clicks into the composer, types `words` and presses Send, so the pointer is what drives the
/// page.
fn send_by_pointer(browser: &mut Browser, words: &str) {
    press(browser, COMPOSER);
    type_words(browser, words);
    press(browser, ".scene-stage .composer button[type='submit']");
}

/// Sets the stage to the size whose button says `size`, `width` wide.
fn choose_stage(browser: &mut Browser, size: &str, width: u32) {
    browser.page().evaluate(
        "document.querySelector('.scene-tools').scrollIntoView({ block: 'start', behavior: 'instant' }); 0",
    );
    press_text(browser, ".scene-tools .segmented-option", size);
    wait_true(
        browser,
        &format!(
            "document.querySelector('.scene-stage').getBoundingClientRect().width === {width}"
        ),
        "the stage at its size",
    );
    browser.page().evaluate(
        "document.querySelector('.scene-stage').scrollIntoView({ block: 'start', behavior: 'instant' }); 0",
    );
    std::thread::sleep(Duration::from_millis(300));
}

/// The header's facts: their words, whether they take more than one row, whether each stands on one line,
/// whether any separator character stands between them, and whether each label is quieter than
/// its value.
fn header_facts(browser: &mut Browser) -> Value {
    browser.page().evaluate(
        "(() => { const facts = [...document.querySelectorAll('.scene-stage .chat-header-fact')]; \
         const line = parseFloat(getComputedStyle(facts[0]).lineHeight); \
         const labels = facts.map((fact) => fact.querySelector('.chat-header-fact-label')).filter(Boolean); \
         return { words: facts.map((fact) => fact.textContent), \
         wraps: new Set(facts.map((fact) => Math.round(fact.getBoundingClientRect().top))).size > 1, \
         eachOneLine: facts.every((fact) => fact.getBoundingClientRect().height <= line + 1), \
         separators: /[·•|]/.test(document.querySelector('.scene-stage .chat-header-facts').textContent), \
         labelsQuieter: labels.length === 2 && labels.every((label) => getComputedStyle(label).color !== getComputedStyle(label.parentElement).color) }; })()",
    )
}

/// The header's hairline and words against the chat area and the dock's column, and whether a
/// target's height stands above it.
fn header_against_area(browser: &mut Browser) -> Value {
    browser.page().evaluate(
        "(() => { const stage = document.querySelector('.scene-stage'); \
         const area = stage.querySelector('.chat-column').getBoundingClientRect(); \
         const inner = stage.querySelector('.chat-header-inner'); const rule = inner.getBoundingClientRect(); \
         const style = getComputedStyle(inner); const dock = stage.querySelector('.chat-dock').getBoundingClientRect(); \
         const near = (a, b) => Math.abs(a - b) <= 1; \
         const target = parseFloat(getComputedStyle(document.documentElement).getPropertyValue('--target')); \
         return { ruleOnDock: near(rule.left, dock.left) && near(rule.right, dock.right), \
         edgeToEdge: near(rule.left, area.left) && near(rule.right, area.right), \
         wordsOnDock: near(rule.left + parseFloat(style.paddingLeft), dock.left) && near(rule.right - parseFloat(style.paddingRight), dock.right), \
         roomAbove: rule.top - area.top >= target }; })()",
    )
}

/// Where the pane's link to the original stands: its top, whether it sits under the body and
/// inside the pane.
fn original_link(browser: &mut Browser) -> Value {
    browser.page().evaluate(
        "(() => { const pane = document.querySelector('.scene-stage .source-reading-pane'); \
         const link = [...pane.querySelectorAll('a')].find((node) => node.textContent.trim() === 'Open in Epic'); \
         const box = link.getBoundingClientRect(), body = pane.querySelector('.source-pane-body').getBoundingClientRect(), \
         frame = pane.getBoundingClientRect(); \
         return { top: Math.round(box.top), belowBody: box.top >= body.bottom, inPane: box.bottom <= frame.bottom }; })()",
    )
}

/// The chat window in a fresh page, the browser `size`, its motion recorded from the start.
fn open_scene(gallery: &Gallery, size: (u32, u32)) -> Browser {
    let mut browser = Browser::launch();
    browser.watch_page_errors();
    browser.resize(size);
    browser.page().call_session("Page.enable", json!({}));
    browser.page().call_session(
        "Page.addScriptToEvaluateOnNewDocument",
        json!({ "source": RECORD_MOTION }),
    );
    browser.text_until(&format!("{}#chat-window", gallery.url), "Chat window");
    wait_true(
        &mut browser,
        &format!("!!document.querySelector('{COMPOSER}')"),
        "the chat window",
    );
    // Brought into view once, so what the tests point at is on screen.
    browser.page().evaluate(
        "document.querySelector('.scene-stage').scrollIntoView({ block: 'start', behavior: 'instant' }); 0",
    );
    std::thread::sleep(Duration::from_millis(400));
    browser
}

/// Clicks into the composer, types `words` and presses Enter.
fn ask(browser: &mut Browser, words: &str) {
    press(browser, COMPOSER);
    type_words(browser, words);
    browser.press_key(Key::ENTER);
}

fn wait_answer(browser: &mut Browser) {
    wait_true_for(
        browser,
        &format!("!!document.querySelector('{LAST_TURN} .answer-words')"),
        "the answer",
        RUN_TIME,
    );
}

/// Turns the wheel up over the transcript by `pixels`, until the reader stands more than `room`
/// from the end.
fn scroll_back(browser: &mut Browser, pixels: f64, room: f64) {
    let (x, y) = middle(browser, SCROLLER);
    wheel(browser, x, y, -pixels);
    wait_true(
        browser,
        &format!(
            "(() => {{ const s = document.querySelector('{SCROLLER}'); return s.scrollHeight - s.clientHeight - s.scrollTop > {room}; }})()"
        ),
        "the reader to have scrolled back",
    );
    std::thread::sleep(Duration::from_millis(300));
}

/// The middle of a citation of source `number` in sight in the transcript, remembered as
/// `window.chosen` so focus can be checked against it later.
fn visible_citation(browser: &mut Browser, number: u32) -> (f64, f64) {
    let at = browser.page().evaluate(&format!(
        "(() => {{ const view = document.querySelector('{SCROLLER}').getBoundingClientRect(); \
         const chip = [...document.querySelectorAll(\"{SCROLLER} .answer-words button.citation-chip[data-source-number='{number}']\")] \
         .find((node) => {{ const box = node.getBoundingClientRect(); return box.top >= view.top && box.bottom <= view.bottom; }}); \
         if (!chip) return null; window.chosen = chip; const box = chip.getBoundingClientRect(); \
         return [box.left + box.width / 2, box.top + box.height / 2]; }})()"
    ));
    let at = at
        .as_array()
        .unwrap_or_else(|| panic!("no citation of source {number} in sight"));
    (at[0].as_f64().unwrap(), at[1].as_f64().unwrap())
}

/// Waits for the newest answer's Sources list to stand open and in sight, placed beside its button.
fn wait_sources_list(browser: &mut Browser) {
    wait_true(
        browser,
        &format!(
            "document.querySelector('{LAST_TURN} {SOURCES_LIST}:popover-open')?.checkVisibility({{ visibilityProperty: true }}) === true"
        ),
        "the Sources list, in sight",
    );
}

fn wait_working(browser: &mut Browser) {
    wait_true(
        browser,
        &format!("!!document.querySelector('{LAST_TURN} .work-trace.is-working')"),
        "Coda to be working",
    );
}

/// Waits until the transcript stands at its end and the last turn's work line has finished
/// arriving, so the line is in sight and still where a person would press it.
fn wait_settled(browser: &mut Browser) {
    wait_true(
        browser,
        &format!(
            "(() => {{ const scroller = document.querySelector('{SCROLLER}'); \
             const trace = document.querySelector('{LAST_TURN} .work-trace'); \
             return scroller.scrollHeight - scroller.clientHeight - scroller.scrollTop <= 1 \
               && !!trace && trace.getAnimations().length === 0; }})()"
        ),
        "the transcript to settle at its end",
    );
}

/// How many of the person's messages in the transcript hold `words`.
fn sent_count(words: &str) -> String {
    format!(
        "[...document.querySelectorAll('.scene-stage .scene-turn .message-person')].filter((m) => m.textContent.includes({words:?})).length"
    )
}

/// The turn nearest the top of the transcript's view, by its place among the turns, and how far
/// its top stands from the view's.
fn top_turn(browser: &mut Browser) -> (u64, f64) {
    let seen = browser.page().evaluate(&format!(
        "(() => {{ const scroller = document.querySelector('{SCROLLER}'); const top = scroller.getBoundingClientRect().top; \
         const turns = [...scroller.querySelectorAll('.scene-turn')]; \
         const index = turns.findIndex((turn) => turn.getBoundingClientRect().bottom > top); \
         return [index, turns[index].getBoundingClientRect().top - top]; }})()"
    ));
    (
        seen[0].as_u64().expect("a turn in view"),
        seen[1].as_f64().expect("its offset"),
    )
}

/// Types `words` as the keyboard would into what has focus.
fn type_words(browser: &mut Browser, words: &str) {
    browser
        .page()
        .call_session("Input.insertText", json!({ "text": words }));
}

#[derive(Clone, Copy)]
enum Modifier {
    Control,
    Meta,
}

/// Presses Enter with `modifier` held.
fn press_enter(browser: &mut Browser, modifier: Modifier) {
    let modifiers = match modifier {
        Modifier::Control => 2,
        Modifier::Meta => 4,
    };
    for kind in ["rawKeyDown", "keyUp"] {
        browser.page().call_session(
            "Input.dispatchKeyEvent",
            json!({ "type": kind, "key": "Enter", "code": "Enter", "windowsVirtualKeyCode": 13, "modifiers": modifiers }),
        );
    }
}

/// The middle of what `selector` names, where it stands now.
fn middle(browser: &mut Browser, selector: &str) -> (f64, f64) {
    let at = browser.page().evaluate(&format!(
        "(() => {{ const node = document.querySelector({selector:?}); if (!node) return null; \
         const box = node.getBoundingClientRect(); return [box.left + box.width / 2, box.top + box.height / 2]; }})()"
    ));
    let at = at
        .as_array()
        .unwrap_or_else(|| panic!("nothing matches {selector}"));
    (at[0].as_f64().unwrap(), at[1].as_f64().unwrap())
}

/// Presses and releases the mouse on the middle of what `selector` names.
fn press(browser: &mut Browser, selector: &str) {
    let (x, y) = middle(browser, selector);
    click_at(browser, x, y);
}

/// Presses the button inside `scope` whose words are `words`.
fn press_button(browser: &mut Browser, scope: &str, words: &str) {
    let at = browser.page().evaluate(&format!(
        "(() => {{ const button = [...document.querySelector({scope:?}).querySelectorAll('button')] \
         .find((node) => node.getClientRects().length > 0 && node.textContent.trim() === {words:?}); \
         if (!button) return null; const box = button.getBoundingClientRect(); return [box.left + box.width / 2, box.top + box.height / 2]; }})()"
    ));
    let at = at
        .as_array()
        .unwrap_or_else(|| panic!("no button {words:?} in {scope}"));
    click_at(browser, at[0].as_f64().unwrap(), at[1].as_f64().unwrap());
}

/// Presses what inside `scope` shows `words`.
fn press_text(browser: &mut Browser, scope: &str, words: &str) {
    let at = browser.page().evaluate(&format!(
        "(() => {{ const node = [...document.querySelectorAll({scope:?})].find((node) => node.textContent.trim() === {words:?}); \
         if (!node) return null; const box = node.getBoundingClientRect(); return [box.left + box.width / 2, box.top + box.height / 2]; }})()"
    ));
    let at = at
        .as_array()
        .unwrap_or_else(|| panic!("nothing in {scope} says {words:?}"));
    click_at(browser, at[0].as_f64().unwrap(), at[1].as_f64().unwrap());
}

fn click_at(browser: &mut Browser, x: f64, y: f64) {
    for kind in ["mouseMoved", "mousePressed", "mouseReleased"] {
        browser.page().call_session(
            "Input.dispatchMouseEvent",
            json!({
                "type": kind, "x": x, "y": y, "button": if kind == "mouseMoved" { "none" } else { "left" },
                "buttons": if kind == "mousePressed" { 1 } else { 0 },
                "clickCount": if kind == "mouseMoved" { 0 } else { 1 },
            }),
        );
    }
}

/// Turns the mouse wheel over `x`, `y` by `delta` pixels, up when negative, in a few notches.
fn wheel(browser: &mut Browser, x: f64, y: f64, delta: f64) {
    for _ in 0..4 {
        browser.page().call_session(
            "Input.dispatchMouseEvent",
            json!({ "type": "mouseWheel", "x": x, "y": y, "deltaX": 0, "deltaY": delta / 4.0 }),
        );
        std::thread::sleep(Duration::from_millis(30));
    }
}

fn wait_true(browser: &mut Browser, expression: &str, what: &str) {
    wait_true_for(browser, expression, what, Duration::from_secs(10));
}

fn wait_true_for(browser: &mut Browser, expression: &str, what: &str, within: Duration) {
    let deadline = Instant::now() + within;
    while Instant::now() < deadline {
        if browser.page().evaluate(expression).as_bool() == Some(true) {
            return;
        }
        std::thread::sleep(Duration::from_millis(40));
    }
    panic!("timed out waiting for {what}");
}
