//! Opens every gallery section in Chromium through the documented command.
//!
//! The sections come from the page's own links, so a new family is covered the
//! moment the gallery shows it. A section fails on a panic, a console error,
//! sideways page overflow, or a heading or body that never appears. What only
//! keys can reach is walked here too, with real key presses. With
//! `UI_GALLERY_SCREENSHOTS` set to a folder, those walks leave a picture of what
//! they focused there. The UI library's module behaviour is walked with real input too: a
//! menu's keys, a dialog's close, a sheet's drag, a copy read back from the clipboard, a
//! popover's fade out under reduced motion, a selection fill through quick changes, and a screen
//! stack going back through history. A source pane is read for the places its citation marks.

// The shared client also carries helpers for journeys this test doesn't walk.
#[allow(dead_code)]
#[path = "../../mobile/tests/common/devtools.rs"]
mod devtools;
mod support;

use std::path::PathBuf;
use std::time::{Duration, Instant};

use base64::Engine;
use devtools::{Browser, DESKTOP, Key, PHONE};
use serde_json::Value;
use support::Gallery;

/// How long a section's entry motion and first scripted steps get to run.
const SETTLE: Duration = Duration::from_secs(2);

/// How many presses of Tab a walk gets to reach what it is after, past the page's other stops.
const TAB_STOPS: usize = 120;

#[test]
#[ignore = "browser qualification; run explicitly with --ignored"]
fn every_section_opens_at_desktop_and_phone_sizes() {
    let gallery = Gallery::start();
    let mut browser = Browser::launch();
    browser.watch_page_errors();
    let sections = sections(&mut browser, &gallery.url);

    let mut failures: Vec<String> = browser
        .take_page_errors()
        .into_iter()
        .map(|problem| format!("first load: {problem}"))
        .collect();
    for (name, size) in [("desktop", DESKTOP), ("phone", PHONE)] {
        browser.resize(size);
        for (slug, title) in &sections {
            let problems = open_section(&mut browser, &gallery.url, slug, title, size);
            println!(
                "{name} {}x{} #{slug}: {}",
                size.0,
                size.1,
                if problems.is_empty() { "ok" } else { "FAILED" }
            );
            failures.extend(
                problems
                    .into_iter()
                    .map(|problem| format!("{name} #{slug}: {problem}")),
            );
        }
    }
    println!(
        "opened {} sections at 2 sizes, {} failures",
        sections.len(),
        failures.len()
    );
    assert!(
        failures.is_empty(),
        "gallery sections failed:\n{}",
        failures.join("\n")
    );
}

/// A link swaps the section in place, and going back returns to the last one.
#[test]
#[ignore = "browser qualification; run explicitly with --ignored"]
fn links_and_going_back_change_the_section() {
    let gallery = Gallery::start();
    let mut browser = Browser::launch();
    browser.watch_page_errors();
    browser.text_until(&format!("{}#icons", gallery.url), "Icons");
    wait_for_heading(&mut browser, "Icons").expect("the linked section");

    browser.page().evaluate("window.sameDocument = true; 0");
    browser.click("a[href='#prose']");
    wait_for_heading(&mut browser, "Prose").expect("the section a link opens");
    let current = browser
        .page()
        .evaluate("document.querySelector('.gallery-family[aria-current]').textContent");
    assert_eq!(current.as_str(), Some("Prose"), "the link is not marked");

    browser.page().evaluate("history.back(); 0");
    wait_for_heading(&mut browser, "Icons").expect("the section before it");
    let kept = browser.page().evaluate("window.sameDocument === true");
    assert_eq!(
        kept.as_bool(),
        Some(true),
        "changing section reloaded the page"
    );
    assert_eq!(browser.take_page_errors(), Vec::<String>::new());
}

/// Replay entries throws the section away: its nodes and what was done to them go, and its entry
/// motion runs again.
#[test]
#[ignore = "browser qualification; run explicitly with --ignored"]
fn replay_entries_remounts_the_section() {
    let gallery = Gallery::start();
    let mut browser = Browser::launch();
    browser.watch_page_errors();

    browser.text_until(&format!("{}#inputs", gallery.url), "Inputs");
    wait_for_heading(&mut browser, "Inputs").expect("the section to replay");
    let checked =
        "document.querySelectorAll('.gallery-section input[type=checkbox]:checked').length";
    let first_state = browser.page().evaluate(checked);
    let ticked = browser.page().evaluate(
        "(() => { window.before = document.querySelector('.gallery-section'); \
         const box = document.querySelector('.gallery-section input[type=checkbox]:not(:checked)'); \
         box.click(); window.boxBefore = box; return box.checked; })()",
    );
    assert_eq!(ticked.as_bool(), Some(true), "the checkbox didn't tick");

    press_replay(&mut browser);
    wait_until_replaced(&mut browser);
    let reset = browser.page().evaluate(
        "(() => { const boxes = document.querySelectorAll('.gallery-section input[type=checkbox]'); \
         return !window.boxBefore.isConnected && boxes.length > 0; })()",
    );
    assert_eq!(
        reset.as_bool(),
        Some(true),
        "the checkbox survived the replay"
    );
    assert_eq!(
        browser.page().evaluate(checked),
        first_state,
        "the replayed section didn't return to its first state"
    );

    // Charts draw in on entry, so a replay there starts animations of its own.
    browser.click("a[href='#charts']");
    wait_for_heading(&mut browser, "Charts").expect("the section to replay");
    std::thread::sleep(SETTLE);
    browser.page().evaluate(
        "window.before = document.querySelector('.gallery-section'); window.entries = 0; \
         document.addEventListener('animationstart', () => { window.entries += 1; }, true); 0",
    );
    press_replay(&mut browser);
    wait_until_replaced(&mut browser);
    std::thread::sleep(SETTLE);
    let entries = browser.page().evaluate("window.entries");
    assert!(
        entries.as_u64().is_some_and(|count| count > 0),
        "the replay started no entry motion: {entries}"
    );
    assert_eq!(browser.take_page_errors(), Vec::<String>::new());
}

/// Slow motion plays what is running at quarter speed, and turning it off restores it.
#[test]
#[ignore = "browser qualification; run explicitly with --ignored"]
fn slow_motion_changes_the_pace_of_running_animations() {
    let gallery = Gallery::start();
    let mut browser = Browser::launch();
    browser.text_until(&format!("{}#status", gallery.url), "Status");
    wait_for_heading(&mut browser, "Status").expect("a section with running animations");
    let rates = "(() => { const all = document.getAnimations().map((a) => a.playbackRate); \
                 return [all.length, Math.min(...all), Math.max(...all)]; })()";
    let press = "document.querySelector('.gallery-header button[aria-pressed]').click(); 0";

    browser.page().evaluate(press);
    wait_for_value(&mut browser, rates, |rates| {
        rates[1] == 0.25 && rates[2] == 0.25
    })
    .expect("every animation at quarter speed");
    browser.page().evaluate(press);
    wait_for_value(&mut browser, rates, |rates| {
        rates[1] == 1.0 && rates[2] == 1.0
    })
    .expect("every animation back at full speed");
}

/// The components the rest of the gallery leaves out stand under their own names, in their states.
#[test]
#[ignore = "browser qualification; run explicitly with --ignored"]
fn tiles_spinners_and_retries_show_their_states() {
    let gallery = Gallery::start();
    let mut browser = Browser::launch();
    browser.watch_page_errors();

    browser.text_until(&format!("{}#icons", gallery.url), "Icons");
    wait_for_heading(&mut browser, "Icons").expect("the section with the tiles");
    assert_eq!(
        in_part(&mut browser, "Icon tiles", ".icon-tile:not(.is-tinted) svg"),
        1,
        "the neutral tile"
    );
    assert_eq!(
        in_part(&mut browser, "Icon tiles", ".icon-tile.is-tinted svg"),
        1,
        "the tinted tile"
    );

    browser.click("a[href='#status']");
    wait_for_heading(&mut browser, "Status").expect("the section with the spinner");
    assert_eq!(
        in_part(&mut browser, "Waiting on an action", ".spinner"),
        5,
        "the spinner bare and in its busy buttons"
    );
    assert_eq!(
        in_part(
            &mut browser,
            "Waiting on an action",
            "button[aria-busy=true] .spinner"
        ),
        4,
        "the spinner in its busy buttons"
    );

    assert_eq!(
        in_part(&mut browser, "Trying again", "button:not([disabled])"),
        1,
        "the retry that can be pressed"
    );
    assert_eq!(
        in_part(&mut browser, "Trying again", "button[aria-busy=true]"),
        1,
        "the retry that runs"
    );
    assert_eq!(
        in_part(&mut browser, "Trying again", "button[disabled]"),
        2,
        "the retry that runs and the one that can't yet"
    );

    // Pressing one holds it while the retry runs, then offers it again.
    browser.page().evaluate(
        "Array.from(document.querySelectorAll('.gallery-part')) \
         .find((part) => part.querySelector('.gallery-part-title').textContent === 'Trying again') \
         .querySelector('button:not([disabled])').click(); 0",
    );
    let running = "button[aria-busy=true]";
    wait_for_count(&mut browser, "Trying again", running, 2).expect("the pressed retry running");
    wait_for_count(&mut browser, "Trying again", running, 1).expect("the retry offered again");
    assert_eq!(browser.take_page_errors(), Vec::<String>::new());
}

/// Waits until the part titled `title` holds `count` of `selector`.
fn wait_for_count(
    browser: &mut Browser,
    title: &str,
    selector: &str,
    count: u64,
) -> Result<(), String> {
    let deadline = Instant::now() + Duration::from_secs(10);
    let mut seen = 0;
    while Instant::now() < deadline {
        seen = in_part(browser, title, selector);
        if seen == count {
            return Ok(());
        }
        std::thread::sleep(Duration::from_millis(25));
    }
    Err(format!("last saw {seen}"))
}

/// A button with words alone keeps its exact size while busy, so nothing around it moves, and its
/// spinner shows centred in it: each words-only button beside its busy twin, and the retry ready
/// beside the one running.
#[test]
#[ignore = "browser qualification; run explicitly with --ignored"]
fn a_busy_button_keeps_its_size_and_shows_its_wait() {
    let gallery = Gallery::start();
    let mut browser = open_at(&gallery, DESKTOP, "status", "Status");
    let pairs = browser.page().call_function(
        include_str!("busy_pairs.js"),
        &[
            Value::from("Waiting on an action"),
            Value::from("Trying again"),
        ],
    );
    let pairs = pairs.as_array().expect("the idle and busy buttons");
    assert_eq!(
        pairs.len(),
        4,
        "three idle and busy pairs and the retry: {pairs:?}"
    );
    for pair in pairs {
        assert_eq!(
            (&pair["same_size"], &pair["spinner"], &pair["centred"]),
            (&Value::Bool(true), &Value::Bool(true), &Value::Bool(true)),
            "{pair}"
        );
    }
}

/// A group of rings with none chosen stands empty: a ring fills only once it is chosen, while a
/// box over a list still fills when only some of it is chosen.
#[test]
#[ignore = "browser qualification; run explicitly with --ignored"]
fn a_ring_fills_only_once_it_is_chosen() {
    let gallery = Gallery::start();
    let mut browser = open_at(&gallery, DESKTOP, "inputs", "Inputs");
    let fills = browser
        .page()
        .call_function(include_str!("ring_fills.js"), &[]);
    assert_eq!(
        fills,
        serde_json::json!({ "none_chosen": true, "empty_rings": 1, "empty_unlike_chosen": true, "mixed_filled": true }),
        "every ring not chosen should stand empty, in every group"
    );
}

/// The screen stack keeps history's depth to the screens open over the tab that shows, so the
/// platform's own back closes the top screen; going back finds the screen beneath, and its scroll
/// place, as they were; a tab keeps its screens; focus moves to the layer on top when a screen opens
/// or closes, and stays on the tab pressed; a fast double press opens one screen; and under reduced
/// motion a closed screen still goes.
#[test]
#[ignore = "browser qualification; run explicitly with --ignored"]
fn the_screen_stack_goes_back_by_history_and_keeps_each_screen_as_it_was() {
    let gallery = Gallery::start();
    let mut browser = open_at(
        &gallery,
        PHONE,
        "buttons-and-navigation",
        "Buttons and navigation",
    );
    browser.watch_page_errors();
    let step = |browser: &mut Browser, action: &str, value: Value| {
        browser.page().call_function(
            include_str!("screen_stack.js"),
            &[Value::from("Screen stack"), Value::from(action), value],
        )
    };
    // After a screen opens or closes, the layer on top has focus.
    let stack = |depth: u64, layers: &[(&str, &str, bool)], scroll: u64| {
        let top = layers.iter().find(|(_, standing, _)| *standing == "top");
        serde_json::json!({ "depth": depth, "focus": top.map(|(title, _, _)| title), "layers": layers, "scroll": scroll })
    };

    let drawn = step(&mut browser, "read", Value::Null);
    assert_eq!(
        drawn["focus"],
        Value::Null,
        "on first draw no layer takes focus"
    );
    step(&mut browser, "open", Value::from("Visit note"));
    assert_eq!(
        step(&mut browser, "open", Value::from("Original record")),
        stack(
            2,
            &[
                ("Records", "under", false),
                ("Visit note", "under", false),
                ("Original record", "top", true),
                ("You", "away", false)
            ],
            0
        ),
        "two screens open"
    );
    assert_eq!(
        step(&mut browser, "history-back", Value::Null),
        stack(
            1,
            &[
                ("Records", "under", false),
                ("Visit note", "top", true),
                ("You", "away", false)
            ],
            0
        ),
        "history's back closes the top screen"
    );

    step(&mut browser, "back", Value::Null);
    step(&mut browser, "scroll", Value::from(400));
    step(&mut browser, "open", Value::from("Blood pressure"));
    let first = [("Records", "top", true), ("You", "away", false)];
    assert_eq!(
        step(&mut browser, "back", Value::Null),
        stack(0, &first, 400),
        "the in-app Back finds the list where it was scrolled"
    );

    let open = stack(
        1,
        &[
            ("Records", "under", false),
            ("Visit note", "top", true),
            ("You", "away", false),
        ],
        400,
    );
    assert_eq!(step(&mut browser, "open", Value::from("Visit note")), open);
    // A tab change leaves focus on the tab pressed.
    let mut you = stack(
        0,
        &[
            ("Records", "away", false),
            ("Visit note", "away", false),
            ("You", "top", true),
        ],
        400,
    );
    you["focus"] = Value::from("tab You");
    assert_eq!(
        step(&mut browser, "tab", Value::from("You")),
        you,
        "the other tab, with history at its depth"
    );
    let mut returned = open.clone();
    returned["focus"] = Value::from("tab Records");
    assert_eq!(
        step(&mut browser, "tab", Value::from("Records")),
        returned,
        "the first tab as it was left"
    );

    step(&mut browser, "back", Value::Null);
    let row = ".gallery-screens .screen-frame.is-top .list-row";
    let (x, y) = center(&mut browser, row);
    for kind in [
        "mousePressed",
        "mouseReleased",
        "mousePressed",
        "mouseReleased",
    ] {
        mouse(&mut browser, kind, x, y);
    }
    let opened = step(&mut browser, "read", Value::Null);
    assert_eq!(
        (&opened["depth"], opened["layers"].as_array().map(Vec::len)),
        (&Value::from(1), Some(3)),
        "a fast double press opens one screen: {opened}"
    );

    browser.emulate_reduced_motion();
    let settled = step(&mut browser, "back", Value::Null);
    assert_eq!(
        settled["layers"],
        serde_json::json!(first),
        "under reduced motion the closed screen goes"
    );
    assert_eq!(browser.take_page_errors(), Vec::<String>::new());
}

/// A step back in history lands a while after the stack asks for it, as on a busy phone: a press
/// made while it travels is kept, and the step is never taken for the person going back. The test's
/// own script holds each step until all of a turn's presses are made, so every press overlaps it.
#[test]
#[ignore = "browser qualification; run explicitly with --ignored"]
fn a_press_while_the_stacks_own_step_travels_is_kept() {
    let gallery = Gallery::start();
    let mut browser = open_at(
        &gallery,
        PHONE,
        "buttons-and-navigation",
        "Buttons and navigation",
    );
    browser.watch_page_errors();
    let quickly = |browser: &mut Browser, presses: Value| {
        browser.page().call_function(
            include_str!("screen_stack.js"),
            &[Value::from("Screen stack"), Value::from("quickly"), presses],
        )
    };
    let stands = |seen: &Value| (seen["depth"].clone(), seen["layers"].clone());
    let records = |depth: u64, layers: Value| (Value::from(depth), layers);
    let two_open = serde_json::json!([
        ["Records", "under", false],
        ["Visit note", "under", false],
        ["Original record", "top", true],
        ["You", "away", false]
    ]);

    quickly(
        &mut browser,
        serde_json::json!([["open", "Visit note"], ["open", "Original record"]]),
    );
    let returned = quickly(
        &mut browser,
        serde_json::json!([["tab", "You"], ["tab", "Records"]]),
    );
    assert_eq!(
        stands(&returned),
        records(2, two_open.clone()),
        "Records, chosen again while the step to You travels, keeps both its screens"
    );
    let you = quickly(&mut browser, serde_json::json!([["tab", "You"]]));
    assert_eq!(
        stands(&you),
        records(
            0,
            serde_json::json!([
                ["Records", "away", false],
                ["Visit note", "away", false],
                ["Original record", "away", false],
                ["You", "top", true]
            ])
        ),
        "You still finds its own first screen"
    );

    let before = quickly(&mut browser, serde_json::json!([["tab", "Records"]]));
    assert_eq!(stands(&before), records(2, two_open));
    let closed = quickly(
        &mut browser,
        serde_json::json!([["back", null], ["back", null]]),
    );
    let first = serde_json::json!([["Records", "top", true], ["You", "away", false]]);
    assert_eq!(
        (stands(&closed), &closed["length"]),
        (records(0, first), &before["length"]),
        "two Backs close together close both screens, and history grows no entry"
    );

    quickly(&mut browser, serde_json::json!([["open", "Visit note"]]));
    let reopened = quickly(
        &mut browser,
        serde_json::json!([["back", null], ["open", "Visit note"]]),
    );
    assert_eq!(
        stands(&reopened),
        records(
            1,
            serde_json::json!([
                ["Records", "under", false],
                ["Visit note", "top", true],
                ["You", "away", false]
            ])
        ),
        "a screen opened while the step back travels stays open"
    );
    assert_eq!(browser.take_page_errors(), Vec::<String>::new());
}

/// Under reduced motion Android's form cross-fades in place: going back, the closed screen stays in
/// the page where it stood while it fades, and goes once its fade ends, the screen beneath then
/// taking input.
#[test]
#[ignore = "browser qualification; run explicitly with --ignored"]
fn under_reduced_motion_a_closed_screen_fades_out_on_android() {
    let gallery = Gallery::start();
    let mut browser = Browser::launch();
    browser.watch_page_errors();
    browser.resize(PHONE);
    browser.emulate_reduced_motion();
    open(
        &mut browser,
        &gallery.url,
        "buttons-and-navigation",
        "Buttons and navigation",
    );
    let step = |browser: &mut Browser, action: &str, value: Value| {
        browser.page().call_function(
            include_str!("screen_stack.js"),
            &[Value::from("Screen stack"), Value::from(action), value],
        )
    };
    step(&mut browser, "form", Value::from("Android"));
    step(&mut browser, "open", Value::from("Visit note"));
    let back = step(&mut browser, "watched-back", Value::Null);
    // On each frame, the closed screen's opacity and its left edge from its resting place.
    let closing: Vec<Option<(f64, f64)>> = back["closing"]
        .as_array()
        .expect("the closed screen on each frame")
        .iter()
        .map(|seen| Some((seen.get(0)?.as_f64()?, seen.get(1)?.as_f64()?)))
        .collect();
    let fading = closing
        .iter()
        .any(|seen| seen.is_some_and(|(opacity, _)| opacity > 0.1 && opacity < 0.9));
    assert!(
        fading,
        "partway through its fade the closed screen is still in the page: {closing:?}"
    );
    assert!(
        closing.iter().flatten().all(|&(_, left)| left == 0.0),
        "the closed screen fades where it stood: {closing:?}"
    );
    assert_eq!(
        closing.last(),
        Some(&None),
        "once settled the closed screen is gone: {closing:?}"
    );
    assert_eq!(
        back["layers"],
        serde_json::json!([["Records", "top", true], ["You", "away", false]]),
        "the screen beneath takes input"
    );
    assert_eq!(browser.take_page_errors(), Vec::<String>::new());
}

/// An address that names no section opens the first rather than a blank page.
#[test]
#[ignore = "browser qualification; run explicitly with --ignored"]
fn an_unknown_address_opens_the_first_section() {
    let gallery = Gallery::start();
    let mut browser = Browser::launch();
    let (_, first) = sections(&mut browser, &gallery.url).remove(0);
    browser.page().navigate("about:blank");
    wait_until_blank(&mut browser);
    browser
        .page()
        .navigate(&format!("{}#no-such-family", gallery.url));
    wait_for_heading(&mut browser, &first).expect("the first section");
}

/// Tab reaches a code block too long for its frame, shows the ring, and the arrows move it.
#[test]
#[ignore = "browser qualification; run explicitly with --ignored"]
fn keys_scroll_a_code_block() {
    let gallery = Gallery::start();
    let mut browser = open_at(&gallery, PHONE, "agent-work", "Agent work");
    keys_scroll(
        &mut browser,
        ".gallery-agent > .code-block > .code-block-body",
        "code-block",
    );
}

/// A prose table with more columns than a phone's column holds keeps each figure it puts at the
/// end on one line with its unit, and Tab reaches it, shows the ring, and the arrows move it.
#[test]
#[ignore = "browser qualification; run explicitly with --ignored"]
fn keys_scroll_a_prose_table() {
    let gallery = Gallery::start();
    let mut browser = open_at(&gallery, PHONE, "prose", "Prose");
    let figures = browser.page().evaluate(
        "(() => { const cells = [...document.querySelector('.prose-table-scroll').querySelectorAll('td[data-align=end]')]; \
         const broken = cells.filter((cell) => { const range = document.createRange(); range.selectNodeContents(cell); \
           return new Set([...range.getClientRects()].filter((box) => box.width > 0).map((box) => Math.round(box.bottom))).size > 1; }); \
         return { figures: cells.length, broken: broken.map((cell) => cell.textContent.trim()) }; })()",
    );
    assert_eq!(
        figures,
        serde_json::json!({ "figures": 8, "broken": [] }),
        "figures broken over more than one line"
    );
    keys_scroll(&mut browser, ".prose-table-scroll", "prose-table");
}

/// The chat column's transcript and its composer share their left and right edges at every window
/// width, with the scrollbar's room kept on the transcript's sides. Read with classic scrollbars,
/// which take room; overlay scrollbars take none.
#[test]
#[ignore = "browser qualification; run explicitly with --ignored"]
fn the_transcript_and_the_composer_share_their_edges() {
    let gallery = Gallery::start();
    for width in [600, 768, 1024, 1280] {
        let mut browser = open_at(&gallery, (width, 900), "conversation", "Conversation");
        let edges = browser.page().evaluate(
            "(() => { const column = document.querySelector('.gallery-chat .chat-column'); \
             const scroller = column.querySelector('.conversation-scroll'); \
             const words = column.querySelector('.conversation-feed').getBoundingClientRect(); \
             const composer = column.querySelector('.composer').getBoundingClientRect(); \
             return { classic: scroller.offsetWidth > scroller.clientWidth, \
               left: Math.round(words.left - composer.left), right: Math.round(words.right - composer.right) }; })()",
        );
        assert_eq!(
            edges,
            serde_json::json!({ "classic": true, "left": 0, "right": 0 }),
            "the transcript's edges against the composer's in a window {width} wide"
        );
    }
}

/// Under reduced motion a pending line still waits its delay before it shows, so a read that
/// settles sooner never flashes it; it then shows at once, without its fade.
#[test]
#[ignore = "browser qualification; run explicitly with --ignored"]
fn under_reduced_motion_a_pending_line_still_waits_before_it_shows() {
    let gallery = Gallery::start();
    let mut browser = Browser::launch();
    browser.resize(DESKTOP);
    browser.emulate_reduced_motion();
    browser
        .page()
        .call_session("Page.enable", serde_json::json!({}));
    // Each pending line, as it first appears and twice after: whether it shows at each moment.
    browser.page().call_session(
        "Page.addScriptToEvaluateOnNewDocument",
        serde_json::json!({ "source": "window.pending = []; \
            const shows = (line) => getComputedStyle(line).visibility === 'visible' && parseFloat(getComputedStyle(line).opacity) > 0.5; \
            new MutationObserver(() => { for (const line of document.querySelectorAll('.pending-status')) { \
              if (line.seen) continue; line.seen = true; const at = [shows(line)]; window.pending.push(at); \
              setTimeout(() => at.push(shows(line)), 200); setTimeout(() => at.push(shows(line)), 700); } }) \
            .observe(document, { subtree: true, childList: true });" }),
    );
    browser.page().navigate(&format!("{}#status", gallery.url));
    wait_for_heading(&mut browser, "Status").expect("the section");
    std::thread::sleep(SETTLE);
    let seen = browser.page().evaluate("window.pending[0] ?? null");
    assert_eq!(
        seen,
        serde_json::json!([false, false, true]),
        "the pending line at first, after 200ms and after 700ms"
    );
}

/// Under reduced motion a popover fades out as it faded in and never changes size: partway through
/// its exit it is still shown, part faded, and on every frame it shows its scale is its resting one.
#[test]
#[ignore = "browser qualification; run explicitly with --ignored"]
fn under_reduced_motion_a_closing_popover_fades_without_scaling() {
    let gallery = Gallery::start();
    let mut browser = Browser::launch();
    browser.watch_page_errors();
    browser.emulate_reduced_motion();
    open(&mut browser, &gallery.url, "overlays", "Overlays");
    let popover = "document.getElementById('gallery-model-menu')";

    browser.press("[popovertarget='gallery-model-menu']");
    wait_true(
        &mut browser,
        &format!(
            "{popover}.matches(':popover-open') && getComputedStyle({popover}).opacity === '1'"
        ),
        "the popover to open and finish its fade",
    );
    // On every frame: whether it is open, whether it is shown, its opacity and its scale.
    browser.page().evaluate(&format!(
        "(() => {{ const popover = {popover}; window.frames = []; window.sampling = true; \
         const tick = () => {{ const style = getComputedStyle(popover); \
           window.frames.push([popover.matches(':popover-open'), style.display !== 'none', parseFloat(style.opacity), style.scale]); \
           if (window.sampling) requestAnimationFrame(tick); }}; \
         requestAnimationFrame(tick); return 0; }})()"
    ));
    // A few frames open first, so its resting scale is seen.
    std::thread::sleep(Duration::from_millis(100));
    browser.press_key(Key::ESCAPE);
    std::thread::sleep(Duration::from_millis(600));
    let seen = browser.page().evaluate(
        "(() => { window.sampling = false; const shown = window.frames.filter(([, shown]) => shown); \
         return { closed: window.frames.some(([open, shown]) => !open && !shown), \
           fadingOut: shown.some(([open, , opacity]) => !open && opacity > 0.05 && opacity < 0.95), \
           scales: [...new Set(shown.map(([, , , scale]) => scale))] }; })()",
    );
    assert_eq!(
        seen,
        serde_json::json!({ "closed": true, "fadingOut": true, "scales": ["1"] }),
        "a popover closing under reduced motion"
    );
    assert_eq!(browser.take_page_errors(), Vec::<String>::new());
}

/// A selection fill still moving when the selection moves again goes on from where it is drawn:
/// through five changes about 33ms apart from real keys, an arrow key held on a segmented control
/// and Enter on one row after another in a list, no frame shows the fill farther from the chosen
/// item than the frame before it did.
#[test]
#[ignore = "browser qualification; run explicitly with --ignored"]
fn a_moving_selection_fill_goes_on_from_where_it_is() {
    let gallery = Gallery::start();
    let mut browser = Browser::launch();
    browser.watch_page_errors();

    open(&mut browser, &gallery.url, "inputs", "Inputs");
    let segmented = fill_through(&mut browser, ".segmented", |browser, _| {
        browser.press_key(Key::ARROW_RIGHT);
    });

    open(&mut browser, &gallery.url, "rows", "Rows");
    let list = fill_through(
        &mut browser,
        ".gallery-rows .selection-host",
        |browser, change| {
            let row = [1, 2, 1, 0, 1][change];
            browser.page().evaluate(&format!(
                "document.querySelectorAll('.gallery-rows .selection-host button')[{row}].focus({{ preventScroll: true }}); 0"
            ));
            browser.press_key(Key::ENTER);
        },
    );
    assert_eq!(
        serde_json::json!({ "segmented": segmented, "list": list }),
        serde_json::json!({ "segmented": { "changes": 5, "farther": [] }, "list": { "changes": 5, "farther": [] } }),
        "the fill through five quick changes"
    );
    assert_eq!(browser.take_page_errors(), Vec::<String>::new());
}

/// Samples the fill of selection host `host` on every frame while `change` makes five changes of
/// its current item about 33ms apart, its focus on its current item first. The fill is where the
/// current item stands, carried by its slide: its translation and size. Answers how many changes
/// the frames saw, and each frame where the fill stood farther from the current item than in the
/// frame before.
fn fill_through(
    browser: &mut Browser,
    host: &str,
    mut change: impl FnMut(&mut Browser, usize),
) -> Value {
    browser.page().evaluate(&format!(
        "(() => {{ const host = document.querySelector({host:?}); \
         host.scrollIntoView({{ block: 'center', behavior: 'instant' }}); \
         const current = () => host.querySelector('[aria-current]:not([aria-current=\"false\"])'); \
         current().querySelector('input')?.focus({{ preventScroll: true }}); \
         window.fill = []; window.sampling = true; \
         const tick = () => {{ const item = current(); const style = getComputedStyle(host, '::after'); \
           const [x = 0, y = 0] = style.translate === 'none' ? [] : style.translate.split(' ').map(parseFloat); \
           const at = item.getBoundingClientRect(); const left = at.left + x, top = at.top + y; \
           window.fill.push({{ item, to: [at.left, at.top, at.right, at.bottom], \
             drawn: [left, top, left + parseFloat(style.width), top + parseFloat(style.height)] }}); \
           if (window.sampling) requestAnimationFrame(tick); }}; \
         requestAnimationFrame(tick); return 0; }})()"
    ));
    std::thread::sleep(Duration::from_millis(100));
    for step in 0..5 {
        change(browser, step);
        std::thread::sleep(Duration::from_millis(33));
    }
    std::thread::sleep(Duration::from_millis(800));
    browser.page().evaluate(
        "(() => { window.sampling = false; \
         const away = (drawn, to) => Math.max(...drawn.map((edge, index) => Math.abs(edge - to[index]))); \
         let changes = 0; const farther = []; \
         window.fill.slice(1).forEach((frame, index) => { const before = window.fill[index]; \
           if (frame.item !== before.item) changes += 1; \
           const was = away(before.drawn, frame.to), is = away(frame.drawn, frame.to); \
           if (is > was + 0.5) farther.push(`frame ${index + 1}: ${Math.round(was)}px then ${Math.round(is)}px`); }); \
         return { changes, farther }; })()",
    )
}

/// The section `slug`, titled `title`, in a window `size`, settled.
fn open_at(gallery: &Gallery, size: (u32, u32), slug: &str, title: &str) -> Browser {
    let mut browser = Browser::launch();
    browser.resize(size);
    browser.page().navigate(&format!("{}#{slug}", gallery.url));
    wait_for_heading(&mut browser, title).expect("the section");
    std::thread::sleep(SETTLE);
    browser
}

/// Walks to `selector` with Tab, where it can't fit, then scrolls it both ways while the page
/// around it stays put.
fn keys_scroll(browser: &mut Browser, selector: &str, picture: &str) {
    let target = format!("document.querySelector({selector:?})");
    let wider = browser
        .page()
        .evaluate(&format!("{target}.scrollWidth > {target}.clientWidth"));
    assert_eq!(
        wider.as_bool(),
        Some(true),
        "{selector} fits, so nothing to scroll"
    );

    let mut reached = false;
    for _ in 0..TAB_STOPS {
        browser.press_key(Key::TAB);
        let focused = browser
            .page()
            .evaluate(&format!("document.activeElement === {target}"));
        if focused.as_bool() == Some(true) {
            reached = true;
            break;
        }
    }
    assert!(reached, "Tab never reached {selector}");

    let own_stop = browser.page().evaluate(&format!("{target}.tabIndex"));
    assert_eq!(
        own_stop.as_i64(),
        Some(0),
        "{selector} is no stop of its own"
    );

    // The ring grows in from nothing, so it is read once it has settled.
    std::thread::sleep(SETTLE);
    let ring = browser.page().evaluate(&format!(
        "(() => {{ const style = getComputedStyle({target}); \
         return {target}.matches(':focus-visible') && style.outlineStyle === 'solid' \
           && parseFloat(style.outlineWidth) > 0; }})()"
    ));
    assert_eq!(ring.as_bool(), Some(true), "{selector} shows no focus ring");
    save_picture(browser, picture);

    let place = format!("Math.round({target}.getBoundingClientRect().left)");
    let before = browser.page().evaluate(&place);
    browser.press_key(Key::ARROW_RIGHT);
    let moved = wait_for_scroll(browser, &target, |left| left > 0.0);
    assert!(moved, "ArrowRight did not scroll {selector}");
    let around = browser.page().evaluate(&format!(
        "[scrollX, document.documentElement.scrollWidth > document.documentElement.clientWidth, {place}]"
    ));
    assert_eq!(
        around,
        serde_json::json!([0, false, before]),
        "the page moved sideways with {selector}"
    );
    browser.press_key(Key::ARROW_LEFT);
    let back = wait_for_scroll(browser, &target, |left| left == 0.0);
    assert!(back, "ArrowLeft did not scroll {selector} back");
}

/// Waits out the browser's smooth scroll for the scroll position to answer `arrived`.
fn wait_for_scroll(browser: &mut Browser, target: &str, arrived: impl Fn(f64) -> bool) -> bool {
    let deadline = Instant::now() + Duration::from_secs(5);
    while Instant::now() < deadline {
        let left = browser.page().evaluate(&format!("{target}.scrollLeft"));
        if left.as_f64().is_some_and(&arrived) {
            return true;
        }
        std::thread::sleep(Duration::from_millis(25));
    }
    false
}

/// Saves the page as it stands, when `UI_GALLERY_SCREENSHOTS` names a folder for it.
fn save_picture(browser: &mut Browser, name: &str) {
    let Some(folder) = std::env::var_os("UI_GALLERY_SCREENSHOTS") else {
        return;
    };
    let shot = browser.page().call_session(
        "Page.captureScreenshot",
        serde_json::json!({ "format": "png" }),
    );
    let encoded = shot["result"]["data"].as_str().expect("a screenshot");
    let png = base64::engine::general_purpose::STANDARD
        .decode(encoded)
        .expect("a screenshot in base64");
    let folder = PathBuf::from(folder);
    std::fs::create_dir_all(&folder).expect("the screenshot folder");
    std::fs::write(folder.join(format!("{name}.png")), png).expect("the screenshot");
}

/// Presses Replay entries in the page's header.
fn press_replay(browser: &mut Browser) {
    browser.click(".gallery-header .control-primary");
}

/// Waits for `window.before`, the section marked before a replay, to leave the page.
fn wait_until_replaced(browser: &mut Browser) {
    let deadline = Instant::now() + Duration::from_secs(5);
    while Instant::now() < deadline {
        let replaced = browser
            .page()
            .evaluate("!window.before.isConnected && !!document.querySelector('.gallery-section')");
        if replaced.as_bool() == Some(true) {
            return;
        }
        std::thread::sleep(Duration::from_millis(25));
    }
    panic!("Replay entries left the section's nodes in place");
}

/// Waits until `expression`, a list of numbers, satisfies `done`.
fn wait_for_value(
    browser: &mut Browser,
    expression: &str,
    done: impl Fn(&[f64]) -> bool,
) -> Result<(), String> {
    let deadline = Instant::now() + Duration::from_secs(10);
    let mut seen = Value::Null;
    while Instant::now() < deadline {
        seen = browser.page().evaluate(expression);
        let numbers: Vec<f64> = seen
            .as_array()
            .map(|items| items.iter().filter_map(Value::as_f64).collect())
            .unwrap_or_default();
        if done(&numbers) {
            return Ok(());
        }
        std::thread::sleep(Duration::from_millis(25));
    }
    Err(format!("last saw {seen}"))
}

/// How many of `selector` the part titled `title` holds.
fn in_part(browser: &mut Browser, title: &str, selector: &str) -> u64 {
    browser
        .page()
        .evaluate(&format!(
            "(() => {{ const part = Array.from(document.querySelectorAll('.gallery-part')) \
             .find((part) => part.querySelector('.gallery-part-title').textContent === {title:?}); \
             return part ? part.querySelectorAll({selector:?}).length : 0; }})()"
        ))
        .as_u64()
        .unwrap_or_default()
}

/// A menu opened from its control takes the arrow keys, Home and End, and a letter, skips nothing
/// it shows, and closes on a choice, which the page hears.
#[test]
#[ignore = "browser qualification; run explicitly with --ignored"]
fn a_menu_moves_by_its_keys_and_closes_on_a_choice() {
    let gallery = Gallery::start();
    let mut browser = Browser::launch();
    browser.watch_page_errors();
    open(&mut browser, &gallery.url, "overlays", "Overlays");

    browser.click("[popovertarget='gallery-reply-menu']");
    wait_true(
        &mut browser,
        "document.getElementById('gallery-reply-menu').matches(':popover-open')",
        "the menu to open",
    );
    for (key, focused) in [
        (Key::ARROW_DOWN, "Copy"),
        (Key::ARROW_DOWN, "Edit"),
        (Key::END, "Delete"),
        (Key::HOME, "Copy"),
        (Key::ARROW_UP, "Delete"),
        (Key::T, "Try again"),
    ] {
        browser.press_key(key);
        wait_true(
            &mut browser,
            &format!("document.activeElement.textContent.trim().startsWith({focused:?})"),
            &format!("{} to focus {focused}", key.key),
        );
    }
    browser.press_key(Key::ENTER);
    wait_true(
        &mut browser,
        "!document.getElementById('gallery-reply-menu').matches(':popover-open')",
        "the menu to close on a choice",
    );
    assert_eq!(browser.take_page_errors(), Vec::<String>::new());
}

/// A dialog opens modally with focus inside it, and Escape closes it through its exit motion: the
/// page hears the close and takes the dialog away.
#[test]
#[ignore = "browser qualification; run explicitly with --ignored"]
fn a_dialog_opens_modally_and_escape_closes_it() {
    let gallery = Gallery::start();
    let mut browser = Browser::launch();
    browser.watch_page_errors();
    open(&mut browser, &gallery.url, "overlays", "Overlays");

    click_button(&mut browser, "Open a dialog");
    wait_true(
        &mut browser,
        "(() => { const d = document.querySelector('dialog'); \
         return d !== null && d.matches(':modal') && d.contains(document.activeElement); })()",
        "the dialog to open modally with focus inside",
    );
    browser.press_key(Key::ESCAPE);
    wait_true(
        &mut browser,
        "document.querySelector('dialog') === null",
        "the page to take the closed dialog away",
    );
    assert_eq!(browser.take_page_errors(), Vec::<String>::new());
}

/// A sheet dragged a little by its header springs back and stays; dragged past half its height it
/// goes, and the page takes it away.
#[test]
#[ignore = "browser qualification; run explicitly with --ignored"]
fn a_sheet_springs_back_from_a_short_drag_and_goes_from_a_long_one() {
    let gallery = Gallery::start();
    let mut browser = Browser::launch();
    browser.watch_page_errors();
    browser.resize(PHONE);
    open(&mut browser, &gallery.url, "overlays", "Overlays");

    click_button(&mut browser, "Open a sheet");
    wait_true(
        &mut browser,
        "document.querySelector('dialog')?.open === true",
        "the sheet to open",
    );
    wait_true(
        &mut browser,
        "document.querySelector('dialog').getAnimations().length === 0",
        "the sheet's entry to finish",
    );
    let height = browser
        .page()
        .evaluate("document.querySelector('dialog').offsetHeight")
        .as_f64()
        .expect("the sheet's height");

    browser.drag(".dialog-header h2", height * 0.2);
    wait_true(
        &mut browser,
        "(() => { const d = document.querySelector('dialog'); \
         return d?.open === true && d.style.transform === '' && d.dataset.dragging === undefined; })()",
        "a short drag to spring back",
    );

    browser.drag(".dialog-header h2", height * 0.8);
    wait_true(
        &mut browser,
        "document.querySelector('dialog') === null",
        "a long drag to dismiss the sheet",
    );
    assert_eq!(browser.take_page_errors(), Vec::<String>::new());
}

/// Copy puts a message's words on the clipboard, and says so only once the clipboard has them:
/// pasted back, they are the message's.
#[test]
#[ignore = "browser qualification; run explicitly with --ignored"]
fn copy_puts_the_words_on_the_clipboard() {
    let gallery = Gallery::start();
    let mut browser = Browser::launch();
    browser.watch_page_errors();
    open(&mut browser, &gallery.url, "conversation", "Conversation");

    let copy = ".message-turn.is-person button[aria-label='Copy']";
    let words = browser.page().evaluate(&format!(
        "document.querySelector({copy:?}).closest('.message-turn').querySelector('.message p').textContent"
    ));
    browser.press(copy);
    wait_true(
        &mut browser,
        "document.querySelector(\".message-turn.is-person button[aria-label='Copied']\") !== null",
        "the control to say Copied",
    );
    // Read back as a person would: pasted into a field of its own.
    browser.page().evaluate(
        "(() => { const field = document.createElement('textarea'); field.id = 'pasted'; \
         document.body.append(field); field.focus(); return 0; })()",
    );
    browser.page().call_session(
        "Input.dispatchKeyEvent",
        serde_json::json!({ "type": "keyDown", "key": "v", "code": "KeyV", "commands": ["paste"] }),
    );
    let pasted = browser
        .page()
        .evaluate("document.getElementById('pasted').value");
    assert_eq!(pasted, words, "the clipboard held something else");
    assert_eq!(browser.take_page_errors(), Vec::<String>::new());
}

/// A message that gains its status and actions after it is on screen keeps what it holds: the steps
/// the person opened stay open and listed, in the same nodes.
#[test]
#[ignore = "browser qualification; run explicitly with --ignored"]
fn a_message_that_gains_its_status_and_actions_keeps_its_opened_steps() {
    let gallery = Gallery::start();
    let mut browser = Browser::launch();
    browser.watch_page_errors();
    open(&mut browser, &gallery.url, "conversation", "Conversation");

    // The scene's thread is the one its button stands in; the first read marks the line to know it later.
    let script = include_str!("arriving_answer.js");
    let words = [
        Value::from("The answer arrives"),
        Value::from("Answered at 9:42 AM"),
    ];
    let before = browser.page().call_function(script, &words);
    let (x, y) = (
        before["at"][0].as_f64().unwrap(),
        before["at"][1].as_f64().unwrap(),
    );
    for kind in ["mousePressed", "mouseReleased"] {
        mouse(&mut browser, kind, x, y);
    }
    wait_for(
        &mut browser,
        script,
        &words,
        |arrival| arrival["expanded"] == "true",
        "the steps to open",
    );

    click_button(&mut browser, "The answer arrives");
    wait_for(
        &mut browser,
        script,
        &words,
        |arrival| arrival["joined"] == true,
        "the status and actions to join the message",
    );
    let after = browser.page().call_function(script, &words);
    assert_eq!(
        serde_json::json!({ "sameTrace": after["sameTrace"], "expanded": after["expanded"],
            "stepsHidden": after["stepsHidden"], "steps": after["steps"] }),
        serde_json::json!({ "sameTrace": true, "expanded": "true", "stepsHidden": "false",
            "steps": ["Read the discharge summaryApril 25, 2024", "Read the basic metabolic panelApril 25, 2024"] }),
        "the opened steps, once the status and actions joined the message"
    );
    assert_eq!(browser.take_page_errors(), Vec::<String>::new());
}

/// Opens one section as a fresh page and waits for its heading.
fn open(browser: &mut Browser, url: &str, slug: &str, title: &str) {
    browser.text_until(&format!("{url}#{slug}"), title);
    wait_for_heading(browser, title).expect("the section");
}

/// Clicks the button whose words are `words`.
fn click_button(browser: &mut Browser, words: &str) {
    let found = browser.page().evaluate(&format!(
        "(() => {{ const button = Array.from(document.querySelectorAll('button')) \
         .find((node) => node.textContent.trim() === {words:?}); \
         if (!button) return false; button.click(); return true; }})()"
    ));
    assert_eq!(found.as_bool(), Some(true), "no button says {words:?}");
}

/// Waits for `expression` to hold in the page.
fn wait_true(browser: &mut Browser, expression: &str, what: &str) {
    let deadline = Instant::now() + Duration::from_secs(10);
    while Instant::now() < deadline {
        if browser.page().evaluate(expression).as_bool() == Some(true) {
            return;
        }
        std::thread::sleep(Duration::from_millis(25));
    }
    panic!("timed out waiting for {what}");
}

/// Calls the page function `script` with `args` until `holds` accepts what it returns, within the
/// time `wait_true` gives.
fn wait_for(
    browser: &mut Browser,
    script: &str,
    args: &[Value],
    holds: impl Fn(&Value) -> bool,
    what: &str,
) {
    let deadline = Instant::now() + Duration::from_secs(10);
    while Instant::now() < deadline {
        if holds(&browser.page().call_function(script, args)) {
            return;
        }
        std::thread::sleep(Duration::from_millis(25));
    }
    panic!("timed out waiting for {what}");
}

/// Real input through DevTools, as the page's own listeners receive it.
trait Input {
    fn press(&mut self, selector: &str);
    fn drag(&mut self, selector: &str, distance: f64);
}

impl Input for Browser {
    fn press(&mut self, selector: &str) {
        let (x, y) = center(self, selector);
        for kind in ["mousePressed", "mouseReleased"] {
            mouse(self, kind, x, y);
        }
    }

    /// Drags from the middle of `selector` straight down by `distance`, in steps a hand would make.
    fn drag(&mut self, selector: &str, distance: f64) {
        let (x, y) = center(self, selector);
        mouse(self, "mousePressed", x, y);
        let steps = 12;
        for step in 1..=steps {
            mouse(
                self,
                "mouseMoved",
                x,
                y + distance * f64::from(step) / f64::from(steps),
            );
            std::thread::sleep(Duration::from_millis(16));
        }
        // Held still before letting go, so the release carries no flick.
        std::thread::sleep(Duration::from_millis(150));
        mouse(self, "mouseMoved", x, y + distance);
        mouse(self, "mouseReleased", x, y + distance);
    }
}

/// The middle of `selector`, scrolled into view first so input lands on it.
fn center(browser: &mut Browser, selector: &str) -> (f64, f64) {
    let at = browser.page().evaluate(&format!(
        "(() => {{ const node = document.querySelector({selector:?}); if (!node) return null; \
         node.scrollIntoView({{ block: 'center', behavior: 'instant' }}); const box = node.getBoundingClientRect(); return [box.left + box.width / 2, box.top + box.height / 2]; }})()"
    ));
    let at = at
        .as_array()
        .unwrap_or_else(|| panic!("nothing matches {selector}"));
    (at[0].as_f64().unwrap(), at[1].as_f64().unwrap())
}

fn mouse(browser: &mut Browser, kind: &str, x: f64, y: f64) {
    browser.page().call_session(
        "Input.dispatchMouseEvent",
        serde_json::json!({
            "type": kind,
            "x": x,
            "y": y,
            "button": "left",
            "buttons": if kind == "mouseReleased" { 0 } else { 1 },
            "clickCount": if kind == "mouseMoved" { 0 } else { 1 },
        }),
    );
}

/// A preview follows what the page reports: it opens placed where the window lets it, Escape closes
/// it, and once it has gone the page has Escape back, so its listening stopped with it.
#[test]
#[ignore = "browser qualification; run explicitly with --ignored"]
fn a_preview_hears_the_page_until_it_goes() {
    let gallery = Gallery::start();
    let mut browser = Browser::launch();
    browser.watch_page_errors();
    open(
        &mut browser,
        &gallery.url,
        "citations-and-sources",
        "Citations and sources",
    );

    let (x, y) = center(&mut browser, ".citation-chip");
    mouse(&mut browser, "mouseMoved", x, y);
    wait_true(
        &mut browser,
        "document.querySelector('.citation-preview')?.style.visibility === 'visible'",
        "the preview to be placed",
    );
    browser.page().evaluate(
        "document.addEventListener('keydown', (event) => { window.pageHeard = event.key; }); 0",
    );
    browser.press_key(Key::ESCAPE);
    wait_true(
        &mut browser,
        "document.querySelector('.citation-preview') === null",
        "Escape to close the preview",
    );
    assert_eq!(
        browser.page().evaluate("window.pageHeard === undefined"),
        Value::Bool(true),
        "the preview should have taken Escape from the page"
    );
    browser.press_key(Key::ESCAPE);
    wait_true(
        &mut browser,
        "window.pageHeard === 'Escape'",
        "the page to have Escape back",
    );
    assert_eq!(browser.take_page_errors(), Vec::<String>::new());
}

/// A citation of a record as a whole shows the record with nothing marked and no notice, and its
/// preview says what the record is with no cited place.
#[test]
#[ignore = "browser qualification; run explicitly with --ignored"]
fn a_whole_record_citation_marks_nothing_and_gives_no_notice() {
    let gallery = Gallery::start();
    let mut browser = open_at(
        &gallery,
        DESKTOP,
        "citations-and-sources",
        "Citations and sources",
    );
    let (x, y) = center(
        &mut browser,
        ".gallery-reading ~ .gallery-reading button.citation-chip",
    );
    mouse(&mut browser, "mouseMoved", x, y);
    let label = Value::from("Source 1: Discharge summary");
    wait_for(
        &mut browser,
        include_str!("preview_reading.js"),
        std::slice::from_ref(&label),
        |preview| !preview.is_null(),
        "the whole record's preview",
    );
    assert_eq!(
        browser
            .page()
            .call_function(include_str!("preview_reading.js"), &[label]),
        serde_json::json!({ "about": "April 25, 2024Final", "places": 0 }),
        "the preview should say what the record is and show no cited place"
    );

    let reading = pane_reading(&mut browser, "Whole record");
    assert_eq!(
        (&reading["marks"], &reading["notice"], &reading["has_words"]),
        (&serde_json::json!([]), &Value::Null, &Value::Bool(true)),
        "the pane should show the record with nothing marked and no notice"
    );
}

/// A citation of two passages marks both, and the pane opens with the first in view though the
/// record runs past it.
#[test]
#[ignore = "browser qualification; run explicitly with --ignored"]
fn two_cited_passages_are_both_marked_and_the_first_is_in_view() {
    let gallery = Gallery::start();
    let mut browser = open_at(
        &gallery,
        DESKTOP,
        "citations-and-sources",
        "Citations and sources",
    );
    assert_eq!(
        pane_reading(&mut browser, "Two passages"),
        serde_json::json!({
            "labels": [],
            "marks": [
                "Left ventricular ejection fraction of 35 percent, reduced from 45 percent in March 2022.",
                "Elevated left ventricular filling pressures.",
            ],
            "notice": null,
            "has_words": true,
            "first_in_view": true,
            "overflows": true,
        }),
    );
}

/// A citation of two fields of a structured record marks both rows.
#[test]
#[ignore = "browser qualification; run explicitly with --ignored"]
fn two_cited_fields_are_both_marked() {
    let gallery = Gallery::start();
    let mut browser = open_at(
        &gallery,
        DESKTOP,
        "citations-and-sources",
        "Citations and sources",
    );
    let reading = pane_reading(&mut browser, "Two fields");
    assert_eq!(
        (&reading["marks"], &reading["notice"]),
        (&serde_json::json!(["Creatinine", "eGFR"]), &Value::Null),
    );
}

/// A citation of a passage in a labelled part of text and of a field marks both, in reading order,
/// under the record's part labels, and its preview shows both places.
#[test]
#[ignore = "browser qualification; run explicitly with --ignored"]
fn a_passage_in_a_labelled_part_and_a_field_are_both_marked() {
    let gallery = Gallery::start();
    let mut browser = open_at(
        &gallery,
        DESKTOP,
        "citations-and-sources",
        "Citations and sources",
    );
    let (x, y) = center(
        &mut browser,
        ".gallery-reading ~ .gallery-reading button.citation-chip[aria-label^='Source 7:']",
    );
    mouse(&mut browser, "mouseMoved", x, y);
    let label = Value::from("Source 7: Heart failure clinic note");
    wait_for(
        &mut browser,
        include_str!("preview_reading.js"),
        std::slice::from_ref(&label),
        |preview| !preview.is_null(),
        "the clinic note's preview",
    );
    assert_eq!(
        browser
            .page()
            .call_function(include_str!("preview_reading.js"), &[label])["places"],
        2,
        "the preview should show the passage and the field"
    );

    let reading = pane_reading(&mut browser, "Parts and fields");
    assert_eq!(
        (&reading["labels"], &reading["marks"], &reading["notice"]),
        (
            &serde_json::json!(["Interval history", "Assessment and plan"]),
            &serde_json::json!(["Continue furosemide 40 mg daily.", "Weight"]),
            &Value::Null,
        ),
    );
}

/// A cited place that can't be found shows its notice and marks nothing.
#[test]
#[ignore = "browser qualification; run explicitly with --ignored"]
fn a_place_that_cant_be_found_gives_its_notice_and_marks_nothing() {
    let gallery = Gallery::start();
    let mut browser = open_at(
        &gallery,
        DESKTOP,
        "citations-and-sources",
        "Citations and sources",
    );
    let reading = pane_reading(&mut browser, "Not found");
    assert_eq!(
        (&reading["marks"], &reading["notice"]),
        (
            &serde_json::json!([]),
            &Value::from("Couldn’t find the cited passage.")
        ),
    );
}

/// Opens the pane sample `sample` and reads its record once the pane has opened at its place.
fn pane_reading(browser: &mut Browser, sample: &str) -> Value {
    click_button(browser, sample);
    wait_for(
        browser,
        include_str!("pane_reading.js"),
        &[],
        |reading| !reading.is_null(),
        "the pane's record",
    );
    // The pane opens at its place two frames after the record shows.
    std::thread::sleep(Duration::from_millis(300));
    browser
        .page()
        .call_function(include_str!("pane_reading.js"), &[])
}

/// A copy the clipboard refuses, or that the browser can't even try, leaves the control as it was
/// and says why in the console: both reach Rust through the one way a behaviour answers.
#[test]
#[ignore = "browser qualification; run explicitly with --ignored"]
fn a_refused_copy_stays_uncopied_and_says_why() {
    let gallery = Gallery::start();
    let mut browser = Browser::launch();
    browser.watch_page_errors();
    open(&mut browser, &gallery.url, "conversation", "Conversation");

    let copy = ".message-turn.is-person button[aria-label='Copy']";
    for (refuse, why) in [
        (
            "Object.defineProperty(navigator.clipboard, 'writeText', { value: () => \
             Promise.reject(new DOMException('denied', 'NotAllowedError')) })",
            "coda ui: copy: NotAllowedError: denied",
        ),
        (
            "Object.defineProperty(navigator, 'clipboard', { value: undefined })",
            "coda ui: copy: TypeError",
        ),
    ] {
        browser.page().evaluate(&format!("{refuse}; 0"));
        browser.press(copy);
        let deadline = Instant::now() + Duration::from_secs(10);
        let mut errors = Vec::new();
        while errors.is_empty() && Instant::now() < deadline {
            errors = browser.take_page_errors();
            std::thread::sleep(Duration::from_millis(25));
        }
        assert!(
            errors.iter().any(|error| error.contains(why)),
            "the console should say {why:?}: {errors:?}"
        );
        assert_eq!(
            browser.page().evaluate(&format!(
                "document.querySelector({copy:?}) !== null \
                 && document.querySelector(\".message-turn.is-person button[aria-label='Copied']\") === null"
            )),
            Value::Bool(true),
            "a refused copy shouldn't say Copied"
        );
    }
}

/// The native webview's scripts, run here as Dioxus's native channel runs them: the module loads
/// from its source once, each kind of behaviour runs through it, and a module that won't load or a
/// behaviour that throws reaches its caller as why it has nothing. This holds the scripts to their
/// channel in Chromium; only a native build proves the webview's own import.
#[test]
#[ignore = "browser qualification; run explicitly with --ignored"]
fn the_native_scripts_run_every_kind_of_behaviour_and_say_why_they_cant() {
    let gallery = Gallery::start();
    let mut browser = Browser::launch();
    open(&mut browser, &gallery.url, "conversation", "Conversation");

    let script = format!(
        "const LOAD = {load}, NATIVE = {native}, BEHAVIOUR = {behaviour};\n{harness}\n0",
        load = Value::from(include_str!("../../ui/assets/native_load.js")),
        native = Value::from(include_str!("../../ui/assets/native.js")),
        behaviour = Value::from(include_str!("../../ui/assets/behaviour.js")),
        harness = include_str!("native_path.js"),
    );
    browser.page().evaluate(&format!("{{ {script} }}"));
    wait_true(
        &mut browser,
        "window.nativeSeen !== undefined",
        "the native scripts to finish",
    );
    let seen = browser.page().evaluate("window.nativeSeen");
    let says = |key: &str, words: &str| {
        let report = seen[key][0]["err"].as_str().unwrap_or_default();
        assert!(report.contains(words), "{key}: {report:?}: {seen}");
    };
    says("notLoaded", "the module is not loaded");
    assert!(
        seen["badLoad"]
            .as_str()
            .is_some_and(|said| said.starts_with("rejected")),
        "{seen}"
    );
    says("afterBadLoad", "SyntaxError");
    says("threw", "TypeError");
    for (key, expected) in [
        ("load", Value::from("returned")),
        ("secondLoad", Value::from("returned")),
        ("sameModule", Value::Bool(true)),
        ("sendErrors", serde_json::json!([])),
        ("ask", serde_json::json!([{ "ok": false }])),
        ("fired", Value::Bool(true)),
        ("modalFinishedEarly", Value::from("returned")),
        ("modalWaits", Value::Bool(true)),
        ("modal", serde_json::json!([{ "ok": true }])),
        ("stopped", Value::from("returned")),
        ("quietAfterStop", Value::Bool(true)),
    ] {
        assert_eq!(seen[key], expected, "{key}: {seen}");
    }
    assert!(
        seen["listen"].as_u64().is_some_and(|reports| reports > 0),
        "{seen}"
    );
}

/// The gallery's links, as `(slug, title)`.
fn sections(browser: &mut Browser, url: &str) -> Vec<(String, String)> {
    browser.text_until(url, "crates/ui");
    let links = browser.page().evaluate(
        "Array.from(document.querySelectorAll('.gallery-family'))\
         .map((link) => [link.getAttribute('href').slice(1), link.textContent])",
    );
    let sections: Vec<(String, String)> = links
        .as_array()
        .expect("a list of links")
        .iter()
        .map(|link| {
            let text = |index: usize| link[index].as_str().expect("link text").to_owned();
            (text(0), text(1))
        })
        .collect();
    assert!(
        sections.len() > 1,
        "the gallery listed no sections: {links}"
    );
    sections
}

/// Opens one section as a fresh page load and returns what is wrong with it.
fn open_section(
    browser: &mut Browser,
    url: &str,
    slug: &str,
    title: &str,
    size: (u32, u32),
) -> Vec<String> {
    // A navigation that changes only the fragment would keep the page, so leave it first.
    browser.page().navigate("about:blank");
    wait_until_blank(browser);
    browser.take_page_errors();
    browser.page().navigate(&format!("{url}#{slug}"));

    let mut problems = Vec::new();
    if let Err(problem) = wait_for_heading(browser, title) {
        problems.push(problem);
    } else {
        std::thread::sleep(SETTLE);
        problems.extend(wrong_size(browser, size));
        problems.extend(blank_body(browser));
        problems.extend(sideways_overflow(browser));
    }
    problems.extend(browser.take_page_errors());
    problems
}

fn wait_until_blank(browser: &mut Browser) {
    let deadline = Instant::now() + Duration::from_secs(10);
    while Instant::now() < deadline {
        let location = browser.page().evaluate("location.href");
        if location.as_str() == Some("about:blank") {
            return;
        }
        std::thread::sleep(Duration::from_millis(25));
    }
    panic!("the page never left the gallery");
}

/// Waits for the section's own heading.
fn wait_for_heading(browser: &mut Browser, title: &str) -> Result<(), String> {
    let deadline = Instant::now() + Duration::from_secs(30);
    let mut shown = Value::Null;
    while Instant::now() < deadline {
        shown = browser.page().evaluate(
            "(() => { const heading = document.querySelector('.gallery-title'); \
             return heading ? heading.textContent : ''; })()",
        );
        if shown.as_str() == Some(title) {
            return Ok(());
        }
        std::thread::sleep(Duration::from_millis(50));
    }
    Err(format!(
        "never showed its heading {title:?}, showing {shown}"
    ))
}

/// The page was not laid out at the size under test, so its overflow check would mean nothing.
fn wrong_size(browser: &mut Browser, size: (u32, u32)) -> Option<String> {
    let width = browser.page().evaluate("window.innerWidth");
    (width.as_u64() != Some(u64::from(size.0)))
        .then(|| format!("the viewport is {width}px wide, not {}px", size.0))
}

fn blank_body(browser: &mut Browser) -> Option<String> {
    let blank = browser.page().evaluate(
        "(() => { const stages = Array.from(document.querySelectorAll('.gallery-stage')); \
         return !stages.length || stages.some((stage) => !stage.childElementCount); })()",
    );
    (blank.as_bool() != Some(false)).then(|| "rendered nothing under a heading".to_owned())
}

/// The page scrolls sideways, with the elements reaching furthest past the edge.
fn sideways_overflow(browser: &mut Browser) -> Option<String> {
    let found = browser.page().evaluate(
        "(() => { const page = document.documentElement; \
         const over = page.scrollWidth - page.clientWidth; \
         if (over <= 0) return null; \
         const widest = Array.from(document.querySelectorAll('body *')) \
           .map((node) => [node.getBoundingClientRect().right - page.clientWidth, node]) \
           .filter(([past]) => past > 0).sort((a, b) => b[0] - a[0]).slice(0, 3) \
           .map(([past, node]) => `${node.tagName.toLowerCase()}.${node.getAttribute('class') ?? ''} +${Math.round(past)}px`); \
         return `the page overflows sideways by ${over}px: ${widest.join(', ')}`; })()",
    );
    found.as_str().map(str::to_owned)
}
