# Duet graph decisions

Dev's dated product decisions the graph rests on. This file owns the *why*; the
graph ([duet.mmd](duet.mmd)) owns the behavior and cites these by source and
date. A later decision supersedes an earlier one only where it says so.

Sources, in precedence order: Dev in session, then Linear (the Duet and
Connected care projects, ENG-n), then the settled coda-app docs at 5a1dc47b
(north-star.md, behaviour-contract.md as A1-G2, ui-surface-map.md), then
mockups. The coda-app docs describe the iPhone app before the Rust migration;
where Linear has since changed a decision (sign-in methods, Android, the web),
Linear wins.

## Standing rules

- **Charter (Dev 2026-10-25).** The physician charter applied to Duet, plus the
  fixed safety lines. Duet's AI is a broadly capable personal health agent; its
  judgments are a "may". The graph sets hard limits only on access;
  communication standards; data and consent; and owner consistency. Each limit
  names what it protects. Communication standards for Duet: the AI never sends
  anything outside the conversation; whatever leaves Duet (the doctor page,
  sharing, a practice) goes only as the person reviewed it, and the person sends
  it.
- **Fixed safety lines (Dev 2026-10-25).** When the AI sees that something
  could be an emergency, or that self-harm is mentioned, its reply always shows
  the fixed line (911; 988 and 911), unchanged and without its own judgment of
  severity (north-star.md, September 23 rule). This is a graph guarantee; when
  to recognize a case stays the AI's judgment.
- **Ideal state, not infrastructure (Dev 2026-10-24, from the physician graph).**
  The graph is the ideal behavior of the full product for any person. It leaves
  out runtime capacity, retries, timeouts, interruptions, sync and milestone
  stopgaps. Failures from the outside world stay: a portal that is down, a
  permission the phone withdraws, a record that can't be read. What the graph
  assumes away is listed in [infrastructure-gaps.md](infrastructure-gaps.md).
- **MVP behavior, extended by connected graphs (Dev 2026-10-25).** The Duet
  graph holds what Linear puts in the MVP. Later functionality (sharing For my
  doctor outside Coda, several practices, stopping sharing without leaving)
  joins as interconnected graphs as it is built. Runtime stopgaps still stay
  out.
- **One receipt rule for your commands (physician decision 8; ui-surface-map.md,
  Reminder / Check-in).** A command changes its item only when Duet accepts it,
  and is refused when the item no longer allows it. One Duet applied though its
  answer was lost is never reported as failed; one that can't be confirmed says
  so, with Check again, and is never sent twice. Each item has one receipt, and
  a pending command ends at sign-out. Sending a message keeps its own outgoing
  states; composing and signing out of this device are local and have none.
  Sign out everywhere says done only once every session has ended.
- **Behavior, not presentation.** Layout, motion, the companion's face and copy
  belong to the surface map and the app, not the graph. The graph keeps what a
  person can do, what Duet does, and what is guaranteed.

## The account (step 1)

- One account per person whatever the sign-in method: Apple or email on iPhone,
  Google or email on Android, email on the web and for practice invitations;
  joining from an invitation links to the account you already have (ENG-161,
  ENG-111, Dev 2026-10-20). This supersedes north-star.md's Apple-only sign-in.
- Setup order: Terms, a display name only when the sign-in brought none, then
  the AI choice (north-star.md, Identity and setup, steps 1-4). Newer Terms
  published during setup send you back to agree to them. The one optional
  offer to connect your health records nothing when skipped (Dev 2026-09-30),
  so it is presentation, like picking the companion, and is left out;
  connecting itself is in Your health sources.
- Newer Terms gate the app: until you accept them, only accepting, signing out,
  deleting your account, exporting your data and withdrawing AI permission are
  available (north-star.md, Identity and setup; export, Dev 2026-10-26).
- You can change your display name at any time after setup; it is never
  verified. Duet holds no legal name (Dev 2026-10-26).
- The AI choice covers conversation, private memory and Duet's own reviews.
  The standing notice is shown with it: "Duet's AI isn't a doctor. It explains
  health information and doesn't diagnose or treat. In an emergency, call 911."
  (Duet project, Dev 2026-10-21).
- Turning AI off and newer Terms pause AI work without using it up; withdrawing
  permission ends it for good (D7, F3; Dev 2026-10-04, 2026-10-06).
- The AI permission is versioned like the Terms. A new version pauses AI work
  without using it up until you allow it; your earlier consent never covers a
  use you haven't seen. Withdrawing instead means AI is not allowed (Dev
  2026-10-25).
- One proactive-work control pauses Duet's own reviews and requested Reminders
  and Check-ins, keeping chat and memory (north-star.md, Requested
  follow-through).
- Sign out on this device leaves other devices and sharing alone; sign out
  everywhere ends every session and reports done only once they have ended
  (north-star.md sign-out; ENG-161).
- Deletion removes everything, stops all sharing including with a practice,
  finishes if interrupted, and is never brought back by old work or a restored
  backup (ENG-161, F2).
- Export (ENG-161, journey D5): you ask for everything Duet keeps about you,
  each item with its source and date (north-star.md, Source access, export and
  deletion lists what it covers); deleting your account ends it.
  Assumed, not yet decided by Dev: one export is prepared at a time; it ends
  ready to save or couldn't finish, and you ask again to retry; asking for a
  new export replaces the last one, even if the new one then fails.

## The conversation (step 2)

- **Chat works like texting an agent (Dev 2026-10-25: "like Muse, Instinct,
  OpenMuse").** You can always type and send; Duet keeps working while you
  talk; nothing blocks the thread. There are no durable task objects separate
  from the conversation's own work (Dev 2026-10-01, after the Instinct study).
- **A new message steers the reply (Dev 2026-10-25).** Sending while Duet is
  writing folds your message into the reply in progress, which answers both;
  there is one Send and no Stop (ui-surface-map.md, Reply activity, Dev
  2026-09-22). This supersedes north-star.md's cost rule ("a newer message ends
  an older unfinished reply") and behaviour-contract rule 6 ("both replies
  run").
- A sent message is never lost: Send moves your words into the outgoing
  message and empties the composer; only a definite refusal offers Try again
  or Edit, and an unconfirmed send keeps checking rather than failing (D1;
  ui-surface-map.md, Composer and message). Once sent, a message belongs to
  your account, not the device: signing out doesn't drop it, and it arrives
  and is answered (Dev 2026-10-25). Edit returns the words and quote,
  and the attached item only while it is still current. That the words
  survive is a data promise about the message's identity, for a linked data
  proof; the chart holds the states around it.
- A reply that can't finish shows unfinished with Try again, never silence. A
  reply ended by withdrawn AI permission shows unfinished without Try again
  (D1; Oct 4 rule 7). Turning AI off or newer Terms pause the reply, which
  continues afterwards (D7). A message that arrives after AI went off or newer
  Terms came starts a paused reply; one that arrives after permission was
  withdrawn shows unfinished without Try again. No message goes unanswered.
- The fixed safety line shows on a reply that pauses, as on one that finishes,
  fails or ends; Try again rewrites the reply, so the line is owed again.
- Composing needs a set-up account; sending also needs AI allowed, AI on and
  current Terms (north-star.md, Product structure: drafts survive while AI is
  unavailable).
- Feedback: 👍 or 👎 on the latest finished reply; 👎 offers optional reasons and
  a note that save with Done; tapping the chosen thumb again removes it
  (ui-surface-map.md, Reply feedback, Dev 2026-09-23). Whether the reasons sheet
  is open is presentation and is left out.
- Ask about this attaches the exact item to your draft; replying to a message
  quotes it (north-star.md, Product structure; Dev 2026-09-23).
- Modeling note: "Your latest message" tracks the newest message on its way, on
  the account;
  each earlier message keeps its own state the same way. Person-to-person
  Shared is removed (ENG-141, Dev 2026-10-21), so "sharing" in the graph means
  a practice connection.
- Conflict for Tyler, not the graph: ENG-191 refuses a new message while a
  model command is in flight for the account. The graph never refuses: a
  message sent while Duet writes steers the reply.

## Follow-through (step 3)

- Duet saves a Plan, Reminder or Check-in only from your current message or a
  proposal you accepted; a goal alone creates no schedule (Dev 2026-09-25;
  ENG-34).
- Reading a record or a provider's note adds no step: what you asked for in
  your own message is saved and runs, and nothing Duet read can ask for more
  than you did (ENG-34; Dev 2026-10-25, replacing the earlier rule from D11 and
  Dev 2026-09-25 that such follow-through waits for your Start).
- You edit, pause, resume and cancel follow-through, directly or in
  conversation; an edit, pause or cancel ends a run in progress for good
  (backend-architecture.md, Accepted follow-through: the person's edit, pause
  or cancel "remains permanent for the old assignment"; ENG-34). In conversation, Duet's AI changes, pauses, cancels or deletes only
  as your message asked, and never resumes or starts anything; only you do.
- A Reminder delivers your agreed words with no new AI; a Check-in uses Duet's
  AI. Both need your consent, the AI switch and current Terms, and pause with
  proactive work; both continue through inactivity (D10; north-star.md).
- A time that comes while AI permission is withdrawn, AI is off, newer Terms
  wait or proactive work is paused passes undelivered: on resume the next
  scheduled time delivers, and nothing stale arrives (Dev 2026-10-25). A time
  already due or running when AI work pauses continues afterwards (D7);
  withdrawing permission ends it (F3).
- A Check-in that can't finish shows unfinished with Try again, never silence
  (D1). It stays scheduled: its next time replaces it, delivered or passed
  undelivered, and pause, change or cancel work on it as on any other.
- A Plan is a routine with no schedule. A Reminder or Check-in serves at most
  one Plan, and the Plan's detail shows it. Duet's AI sets or clears that link
  when your request serves the Plan; you change it in conversation, not with a
  new control. Deleting the Plan leaves its Reminders and Check-ins running,
  unlinked. A Plan shows no progress, streak or results (north-star.md,
  Sep 28; kept, Dev 2026-10-26).
- Your own edit, pause, resume or cancel of a Reminder or Check-in makes a reply
  already writing look again, so it never overwrites your change (D9;
  behaviour-contract rule 16), as your Plan edits already do. Today, not the companion's card, is where
  Plans, Reminders and Check-ins live (ENG-34, ENG-168).
- Modeling note: one item per kind stands for each saved item of that kind.

## Duet's reviews and Today (step 4)

- Three kinds of review: a daily look once your local day completes, a weekly
  recap once a Saturday-to-Friday week with something new completes, and a
  review when new records arrive (Dev 2026-10-25, keeping coda-app's three).
  Each is decided once and may finish quietly; silence is success (E1-E5;
  north-star.md, Today). A look still owed or running for an older day or week
  gives way to the newer one.
- A running review looks again from a fresh snapshot, without being used up,
  when what it read changes: new records, a corrected record, your doctor's
  plan, your follow-through, a Plan (yours or one Duet saves as you asked), a
  Not accurate mark, or a message that Duet's AI judges corrects it (ENG-34). A
  running Check-in and a proposed update for your doctor do the same for
  corrections, your doctor's plan, Plans and Not accurate marks; a reply being
  written hears the ones you or your sources make, since Duet's own Plan saves
  happen inside it. Edits to For my doctor and your appointment restart
  nothing; see For my doctor.
- A corrected record is its own event, not an arrival (ENG-34, ENG-162; Dev
  2026-10-25): unfinished work must not publish from information that was
  corrected. The reply being written reads the new version before it finishes,
  a running Check-in or review looks again, the new-records review becomes due,
  and a proposed update for your doctor waits for your review again. What was
  already published keeps the version it used (E5). The graph doesn't track
  which record each piece of work read, so every unfinished producer looks
  again; work that didn't use the record loses nothing by it.
- Apple Health readings don't start a review on their own; they reach the next
  one (Dev 2026-10-06). Reprocessing Duet's own data is not an arrival (C5).
- A review may publish insights, at most three current across all reviews,
  write in the conversation, or stay quiet (E1). Published insights never change; Not accurate marks one corrected and
  Duet never replaces it on its own (E5).
- Today says Still building context before any decision, Quiet only after a
  real decision, and Couldn't check when the daily look fails; a quiet later
  decision never erases insights that are still current (E3; north-star.md).
  Once insights or a decision go stale, Today shows no current insights rather
  than a Quiet nobody decided. Not accurate works wherever an insight shows.
- Reviews are AI work and proactive work: they need your permission, the AI
  switch on, current Terms and proactive work on.

## For my doctor (step 5)

- One continuing workspace, useful without an appointment, that helps any
  clinician understand what matters, what changed, what you tried and what
  you need next (ENG-169, Dev 2026-10-24). This supersedes north-star.md's
  fixed one-page PDF; there is no PDF, and sharing outside Coda is later.
- It is yours: editing, saving and Start fresh work with AI off. Duet proposes
  whenever it judges an update useful, in conversation or after a review (a
  "may"; Dev 2026-10-25: the graph never limits Duet's judgment, and a proposal
  changes nothing until you review it). A proposal never saves by itself. Only
  your own exact question saves without review (north-star.md, For my doctor).
- Stale work never overwrites your newer edits (ENG-169). Your edits to For my
  doctor or your appointment (yours or the practice's) don't restart work that
  is running: a review or reply notes them, and an update it proposes
  afterwards, or one already waiting, must be reviewed again against your
  latest version before it saves (Dev 2026-10-26: "let it finish, review
  again").
- For my doctor stays in Duet; saving shares nothing. While you're connected
  you can send the saved version to your practice. It arrives like a message
  from you, and the practice's agents judge whether a physician needs to see
  it; later saves don't change what you sent (Dev 2026-10-26, replacing "every
  save reaches it" from 2026-10-25). This is the deliberate "send something to
  your practice" from 2026-10-25, not a separate feature.
- Appointment context: your own date, or one you mention in conversation;
  preparation never waits on it and a date change keeps what you prepared.
  The practice's schedule and conflicting dates join with the connection.

## Your practice (step 6)

- One live practice connection in the MVP: a practice invites, you join and
  consent once, and connected means sharing, with no stop or resume; it ends
  when you leave, the practice discharges you (with a dated notice), or your
  account is deleted. Your Duet stays yours either way (ENG-70, Dev
  2026-10-22).
- The connection is live: what you connected in Duet flows to the practice
  continuously, for its physician agents to read
  (Dev 2026-10-25; ENG-39). Duet's AI never shares anything, and its private
  notes never cross. Your answers to Check-ins about your doctor's plan also
  flow, into the practice's next review rather than as alerts; the practice's
  agents judge whether anything needs a physician (ENG-45). Phone readings, notes and medications you entered, and
  notes from conversation join the feed as their regions are written.
- Joining shares what you already have: your connected records reach the
  practice with the join (For my doctor only when you send it), on a first join and on joining
  again, and the practice's agents start on them (Dev 2026-10-25). How the
  practice catches up is the runtime's (infrastructure-gaps.md).
- Any invitation the practice still holds open can be accepted, even while it is
  being resent (Dev 2026-10-25).
- Leaving is always yours, even before newer Terms; joining needs a set-up
  account and current Terms.
- The physician graph owns the practice's side. The two graphs meet only through
  shared events, in [connected-care](../connected-care/connection.mmd), where
  rules hold both sides to one connection status.
- No "your care team looked" receipt (Dev 2026-10-25): with a live connection
  everything you share reaches the practice's agents, so a read receipt on each
  item would say nothing. Sending For my doctor is the deliberate way to ask
  the practice to look.

## Back from the practice (step 7)

- Your doctor's plan (ENG-45): documented care decisions and open items reach
  Duet only through the live connection, read-only, with source, author, date
  and status. Duet calls it "your doctor's plan", explains it faithfully and
  never invents or amends it; an undecided suggestion stays unconfirmed until
  the source records a decision (ENG-43). Physicians don't approve Duet's
  wording (Dev 2026-10-21). When the connection ends, the last plan stays,
  marked no longer updated; joining again brings it current.
- A change to your doctor's plan reaches work that read it, as a corrected
  record does: a reply being written looks again, a running Check-in or review
  starts over from the current plan before it publishes, and a proposed update
  for your doctor waits for you to review it again (ENG-34, ENG-45).
- Your answer to a Check-in about your doctor's plan reaches the practice with
  what you share, for its next review, never as an alert on its own (ENG-45;
  Dev 2026-10-25: "Carry on with the doctor's plan, Check-in answers and
  appointments"). In connected-care it is part of the live feed.
- Your practice's appointment (ENG-169, ENG-31): while connected, the
  practice's schedule shows beside any date you gave yourself; both stay
  visible when they conflict, and neither blocks preparation. A visit already
  ahead on the practice's schedule shows once you join. Duet never shows a practice appointment
  the schedule doesn't have ahead (connected-care checks it). A move keeps what you prepared, a
  cancellation means it is no longer booked, and when the connection ends it
  goes away while your own date stays (Dev 2026-10-26). The practice's schedule is the physician
  graph's; the two meet through shared events in connected-care.

## Your health sources (step 8)

- Two kinds of source (ENG-162; north-star.md, Connect your health): your
  phone's health data (Apple Health on iPhone, Health Connect on Android; Dev
  2026-10-11) and Doctors & hospitals, one search over the portals Duet can
  authorize end to end (Epic MyChart, Oracle Health and healow). Each
  organization you sign in to is authorized, refreshed, failed and
  disconnected on its own (B1); one organization stands for each. Connecting
  needs a set-up account and current Terms; it is never required.
- Cancelling a portal sign-in or declining phone access is not a failure
  (north-star.md, Connect your health).
- Every import ends: complete (with or without the source's warnings), partial
  with the unreadable items named, or a failure that keeps what you have and
  tries again at the next refresh. It never stays importing (B2, B5, B6; Dev
  2026-10-04). You can refresh on request; Duet refreshes at most once a day.
- Records are kept exactly as sent; a record missing from a later import is
  never deleted (B3, B4). A correction or a deletion at the source arrives as a
  correction: the old version stays for existing citations, and unfinished work
  looks again (A4, E5, ENG-34). A deletion in the phone's health app works the
  same way; lost access or an empty read is never a deletion (A4).
- Phone readings arrive one by one with their time and time zone (A2, A3) and
  reach the next review rather than starting one (Dev 2026-10-06). Portal
  records start the new-records review.
- On iPhone, Apple Health also brings in clinical records you already connected
  and approved in the Health app, after their own Apple permission. They arrive
  and change like portal records and start the new-records review. Duet never
  claims it can add, refresh or disconnect those provider accounts (north-star.md,
  Source access).
- The phone reads and uploads only while you're signed in on it; what's queued
  waits, and the connection and your history stay (behaviour-contract rule 12).
- A source that stops working shows it on the source, "couldn't update since"
  its last import, with Reconnect. Duet's AI may mention it in conversation;
  nothing else alerts you (Dev 2026-10-26). Turning off access in the phone's
  settings keeps what Duet has and shows on the source.
- Disconnecting stops new data at once and keeps everything already imported
  until you delete your account; reconnecting resumes the same history (F1;
  north-star.md, Source access). There is no separate control that deletes one
  source's history. Disconnecting is always yours, even before newer Terms, as
  leaving your practice is (Dev 2026-10-26).
- While you're connected to a practice, phone readings, Apple Health clinical
  records and their changes join the live feed with your records (ENG-39).

## Your screens (step 9)

The graph models what you see and open, as the physician graph does (Dev
2026-10-26: "yes, model the screens"). Sources: north-star.md (Product
structure, Today, You) and the original app's ui-surface-map.md (Navigation
map; Page and state inventory), with later decisions winning.

- Three tabs: Duet, Today and You. Shared is removed (ENG-141). Today is the
  home for insights and current care: Plans, Reminders, Check-ins, For my
  doctor and your doctor's plan (ENG-34, ENG-168), replacing the companion's
  card as the way in. Your doctor's plan lives on Today with them (Dev
  2026-10-26); the order and density of Today is still open (ENG-168).
- Each tab keeps what it has open. Moving around never changes your health
  data or Duet's work, and opening something never marks it done.
- Ask about this takes you to the conversation with that item, leaving the
  other tabs as they were.
- You holds the library (records groups, records, Measurements & trends,
  Health history, Earlier insights, search), Sources & data, and Settings. A
  record opens from its group, a search or Health history, and going back
  returns there.
- Settings and AI settings stay reachable while newer Terms wait, since
  exporting, deleting your account and withdrawing AI permission are allowed
  then; everything else waits for the Terms (Dev 2026-10-26).
- Supporting information shows the exact version a reply cited, with a note
  when a newer version exists, never the newer one in its place (C6, E5).
- Every read behaves the same way: a saved copy shows at once, loading shows
  only when nothing can, a failure keeps what is shown with one quiet line and
  Retry, and with nothing shown it says it couldn't load; never a false empty
  state (ui-surface-map.md, Navigation map).
- A Plan deleted while you have it open stays open and says it was removed;
  going back returns to Today (ui-surface-map.md, Plan; Dev 2026-10-26).

## Open, to add later

- **Pausing reviews while you're away (Dev 2026-10-25).** coda-app paused
  Duet's own reviews after 7 days without opening Duet (D10). Dev: the pause
  may not apply to someone who pays, and "I'm not sure about the pauses yet.
  Can we add that later?" No pause is in the graph. Reminders and Check-ins
  you asked for never pause for inactivity in any version. The daily portal
  refresh's old limit to people who opened Duet in the last 7 days (B7) is the
  same question and is also left out.

## Left out on purpose

- The operator's AI switch and the daily message allowance (north-star.md, cost
  controls): operations and cost, not product behavior.
- The app-update gate (G1, G2): release compatibility.
- Follow-through caps (three per reply, twenty open) and retry budgets (D3):
  bounds, not behavior.
- A newer message holding a Check-in until the chat is quiet (D2): scheduling;
  the Check-in still delivers once.
- Switching a portal vendor off (B8) and the import retry budget: operations
  and bounds. Existing connections stay listed and can always be disconnected.
