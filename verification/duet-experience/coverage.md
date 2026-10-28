# Duet app coverage

What the Rust Duet app shows today for each state of [the graph](duet.mmd).
The graph says what the product should do; this file says how much of it
`crates/duet-app` shows, so built, partial and missing UI stay explicit. SwiftUI
remains the shipping iPhone app; this table measures the Rust app only.

- **built:** you reach this state in the app the way the graph does, it shows
  what the graph means by it, and every control the graph gives you from it
  works, including controls elsewhere that only work while you're in it. A
  state that only says nothing is there yet (no quote, nothing attached) is
  built when the app tells it apart from the states beside it.
- **partial:** the app shows this state, but not all of that; the note says
  what's missing or different.
- **missing:** the app doesn't show this state. A screen with the same words
  for a different cause doesn't count.
- **not shown:** the graph state has nothing the person sees, and the app
  correctly shows nothing.

**Server data** says whether the wire already carries what the state needs (yes),
needs a server or wire change first (no), or doesn't apply (n/a). The wire is
`crates/duet-wire`, the contract the app calls; `crates/duet-api` serves only part
of it, and a note says so where that matters. **Where in the app** names the owner
by component, not line number.

Where the app differs from the graph as a whole: it has a fourth tab, Shared,
which the graph doesn't; Plans, Reminders and Check-ins open from their cards in
the Duet thread rather than from Today; and sign-in skips setup (Terms, display
name, the AI choice), so most account and AI-settings states are missing.

Check it from the repository root. It fails when the graph gains or drops a
state this file doesn't match:

```bash
node .agents/skills/coda-statechart/scripts/statechart.mjs coverage verification/duet-experience/duet.mmd verification/duet-experience/coverage.md
```

When the app or the graph changes a state, update its row in the same PR. The
check prints the current counts. To grade a row, list every way into the state
and every control from it, and check each in the app:

```bash
node .agents/skills/coda-statechart/scripts/statechart.mjs query verification/duet-experience/duet.mmd --state "Note saved"
```

# States by region

## Your account

| State | Status | Where in the app | Server data | Note |
| --- | --- | --- | --- | --- |
| No account | partial | DuetRoot (duet-app account.rs) | yes | Sign in with Apple creates the account on first sign-in (ensure_account); email sign-in, and Google on Android, are missing. |
| Agreeing to the Terms | missing | none | yes | Sign-in goes straight to the tabs; no Terms screen. The wire carries terms_accepted and accept_terms. |
| Choosing a display name | missing | none | yes | No name step; the wire carries display_name. |
| Choosing about AI | missing | none | yes | No AI choice step; the wire carries the Luna consent decision. |
| Deleting your account | missing | none | yes | No Delete account control. The wire has delete_account; duet-api doesn't serve it. |
| Set up | partial | PrivateRoot (duet-app lib.rs) | yes | A signed-in account gets the tabs, but the app never reads setup_complete, so an account that skipped setup looks set up. |

## Your account/Set up/AI permission

| State | Status | Where in the app | Server data | Note |
| --- | --- | --- | --- | --- |
| AI allowed | partial | ConversationTab (duet-app lib.rs) | yes | Chat works when the server allows it; the app has no AI permission screen of its own. |
| AI not allowed | partial | ConversationTab (duet-app lib.rs) | yes | The empty chat says to review AI permissions in Settings, but the app has no place to allow AI. |
| Newer AI permission to allow | missing | none | yes | The wire carries luna_consent_version; the app never compares it. |

## Your account/Set up/AI switch

| State | Status | Where in the app | Server data | Note |
| --- | --- | --- | --- | --- |
| AI on | partial | ConversationTab (duet-app lib.rs) | yes | Chat and Today work while intelligence is enabled; there is no switch. |
| AI off | partial | ConversationTab (duet-app lib.rs), TodayTab (duet-app lib.rs) | yes | Chat says it's turned off in Settings and Today says automatic reviews are paused, but there is no switch to turn AI back on. |

## Your account/Set up/Proactive work

| State | Status | Where in the app | Server data | Note |
| --- | --- | --- | --- | --- |
| Proactive work on | missing | none | yes | No proactive-work control; the wire carries proactive settings. |
| Proactive work paused | missing | none | yes | As above. |

## Your account/Set up/Terms

| State | Status | Where in the app | Server data | Note |
| --- | --- | --- | --- | --- |
| Current Terms accepted | partial | none | yes | The app never reads terms_accepted, so current and newer Terms look the same. |
| Newer Terms to accept | missing | none | yes | Nothing asks for newer Terms or pauses AI work until they're accepted. |

## Your account/Set up/Your latest message

| State | Status | Where in the app | Server data | Note |
| --- | --- | --- | --- | --- |
| Nothing on its way | built | ConversationTab (duet-app lib.rs) | yes | The composer, when the page allows a send. |
| Sending | partial | ConversationTab (duet-app lib.rs) | yes | Your words stay in the composer and Send is disabled until the server accepts them; the graph moves them into an outgoing message at once and leaves the composer free for the next one. |
| Not confirmed yet | partial | ConversationTab (duet-app lib.rs) | yes | A failed send keeps the draft and a retry of the same text reuses its id, but it shows the same unavailable line as a refusal. |
| Not sent, with Try again and Edit | partial | ConversationTab (duet-app lib.rs) | yes | The draft stays editable with an alert; sending again is the Send button, with no Try again. |
| Sent | built | ConversationTab (duet-app lib.rs) | yes | The accepted message joins the thread before the reload. |

## Your account/Set up/Duet's reply

| State | Status | Where in the app | Server data | Note |
| --- | --- | --- | --- | --- |
| No reply in progress | built | ConversationTab (duet-app lib.rs) | yes |  |
| Duet is writing | built | ConversationTab (duet-app lib.rs) | yes | The provisional reply streams as "drafting, not yet saved". |
| Reply paused | missing | none | yes | The wire's run states include paused; the app shows nothing for it. |
| Reply unfinished, with Try again | partial | ConversationTab (duet-app lib.rs) | yes | "Draft ended" shows when a run stops, with no Try again; duet-api has a resume route the app doesn't call. |
| Reply ended when AI permission was withdrawn | missing | none | yes |  |
| Reply finished.No feedback | partial | ConversationTab (duet-app lib.rs) | yes | The finished reply shows, with no Helpful or Not helpful controls. |
| Reply finished.Helpful | missing | none | yes | The wire has reply feedback; the app has no controls. |
| Reply finished.Not helpful | missing | none | yes | As above. |

## Your account/Set up/What the reply read

| State | Status | Where in the app | Server data | Note |
| --- | --- | --- | --- | --- |
| The reply read your latest edits | missing | none | no | Nothing records which edits a reply read. |
| You edited since the reply read | missing | none | no | As above. |

## Your account/Set up/Fixed safety line

| State | Status | Where in the app | Server data | Note |
| --- | --- | --- | --- | --- |
| No safety line owed | missing | none | n/a | The app has no safety line, so nothing tells this state from the next one. |
| Safety line owed in this reply | missing | none | no | duet-api asks the model to include the 911 and 988 lines in its instructions; nothing adds a fixed line. |
| Safety line shown | missing | none | no | As above. |

## Your account/Set up/The thread

| State | Status | Where in the app | Server data | Note |
| --- | --- | --- | --- | --- |
| Nothing new from Duet | missing | none | n/a | The app has no unseen marker, so nothing tells this state from the next one. |
| A new message from Duet, unseen | missing | none | no | No unseen marker on the tab or the thread. |

## Your account/Set up/Duet's own message

| State | Status | Where in the app | Server data | Note |
| --- | --- | --- | --- | --- |
| Nothing waiting to post | not shown | none | n/a | Ordering is the server's; the app shows the thread as stored. |
| A message waiting for the reply to end.Only Duet's own message waiting | not shown | none | n/a | As above. |
| A message waiting for the reply to end.What you asked for or your practice's news waiting | not shown | none | n/a | As above. |

## Your account/Set up/Alerts for Duet's own work

| State | Status | Where in the app | Server data | Note |
| --- | --- | --- | --- | --- |
| Duet's own work doesn't alert you | partial | YouTab (duet-app lib.rs) | yes | The New insight notifications switch is kept on this device only, not on the account. |
| Duet's own work alerts you | partial | YouTab (duet-app lib.rs) | yes | As above; only insights alert. |

## Your account/Set up/Reminder

| State | Status | Where in the app | Server data | Note |
| --- | --- | --- | --- | --- |
| No Reminder | built | ConversationTab (duet-app lib.rs) | yes |  |
| Reminder scheduled.Reminder active | partial | ConversationTab (duet-app lib.rs) | yes | A card in the thread opens the Reminder page with its schedule and Pause; Change and Cancel are missing. |
| Reminder paused.Reminder paused by you | partial | ConversationTab (duet-app lib.rs) | yes | The page says Paused and offers Resume reminder; Change and Cancel are missing. |
| Reminder completed | partial | ConversationTab (duet-app lib.rs) | yes | The wire's completed state shows like an active one; the page offers Pause. |
| Reminder cancelled | partial | ConversationTab (duet-app lib.rs) | yes | A cancelled Reminder's card in the thread says cancelled since then; the app reads no cancelled state on its page. |

## Your account/Set up/Check-in

| State | Status | Where in the app | Server data | Note |
| --- | --- | --- | --- | --- |
| No Check-in | built | ConversationTab (duet-app lib.rs) | yes |  |
| Check-in scheduled.Check-in active | partial | ConversationTab (duet-app lib.rs) | yes | Opens like a Reminder, headed Check-in, with Pause reminder; Change and Cancel are missing. |
| Check-in scheduled.Check-in couldn't finish, with Try again | missing | none | yes | The wire carries the last outcome; the page doesn't show it. |
| Check-in due | missing | none | yes |  |
| Check-in running | missing | none | yes |  |
| Check-in paused.Check-in paused by you | partial | ConversationTab (duet-app lib.rs) | yes | Paused, with Resume reminder; Change and Cancel are missing. |
| Check-in paused.Check-in paused after failing repeatedly | partial | ConversationTab (duet-app lib.rs) | yes | Shows Paused like a pause by you; the wire's pause_reason isn't read. |
| Check-in completed | partial | ConversationTab (duet-app lib.rs) | yes | Shows like an active one. |
| Check-in cancelled | partial | ConversationTab (duet-app lib.rs) | yes | As for a cancelled Reminder. |

## Your account/Set up/A Check-in waiting for your answer

| State | Status | Where in the app | Server data | Note |
| --- | --- | --- | --- | --- |
| No Check-in waiting for an answer | not shown | none | n/a |  |
| A delivered Check-in waits to post | not shown | none | n/a | Ordering is the server's. |
| A delivered Check-in waits for your answer | missing | none | no | A Check-in's question arrives as a message; nothing marks it as waiting for your answer or sends the answer to your practice. |
| A Check-in waits for your answer, and a newer one waits to post | missing | none | no | As above. |

## Your account/Set up/Your Plan

| State | Status | Where in the app | Server data | Note |
| --- | --- | --- | --- | --- |
| No Plan | built | ConversationTab (duet-app lib.rs) | yes |  |
| Plan saved | partial | ConversationTab (duet-app lib.rs) | yes | A card in the thread opens the Plan, with Edit plan and Remove plan; Today doesn't open it. |

## Your account/Set up/Reminder: its Plan

| State | Status | Where in the app | Server data | Note |
| --- | --- | --- | --- | --- |
| Reminder serves no Plan | missing | none | yes | The Reminder page never reads its Plan, so nothing tells this state from the next one. |
| Reminder serves your Plan | missing | none | yes | The wire carries routine_id; the Reminder page doesn't name its Plan. |

## Your account/Set up/Check-in: its Plan

| State | Status | Where in the app | Server data | Note |
| --- | --- | --- | --- | --- |
| Check-in serves no Plan | missing | none | yes | As for a Reminder. |
| Check-in serves your Plan | missing | none | yes | As for a Reminder. |

## Your account/Set up/Your export

| State | Status | Where in the app | Server data | Note |
| --- | --- | --- | --- | --- |
| No export | missing | none | yes | No Export control. The wire has conversation and intelligence exports; the app exports only the For my doctor PDF. |
| Preparing your export | missing | none | yes |  |
| Your export is ready to save | missing | none | yes |  |
| Your export couldn't finish; you can ask again | missing | none | yes |  |

## Your account/Set up/Duet's own work

| State | Status | Where in the app | Server data | Note |
| --- | --- | --- | --- | --- |
| Nothing Duet is working on | missing | none | yes | Today never shows running work, so nothing tells this state from the next ones. |
| Duet's own work running.Duet's own work running on your latest edits | missing | none | yes | The wire carries insight-cycle generation state; Today doesn't show running work. |
| Duet's own work running.Duet's own work running, you edited since it started | missing | none | no |  |

## Your account/Set up/What Duet pinned on Today

| State | Status | Where in the app | Server data | Note |
| --- | --- | --- | --- | --- |
| Nothing pinned | partial | TodayTab (duet-app lib.rs) | yes | Today shows no insight cards. Nothing pinned ever leaves when its time passes, so Today never returns here the way the graph does. |
| Duet's latest message pinned | partial | TodayTab (duet-app lib.rs) | no | Today shows insight cards from recent cycles, not Duet's latest message, and nothing expires them. |

## Your account/Set up/Duet's latest work on Today

| State | Status | Where in the app | Server data | Note |
| --- | --- | --- | --- | --- |
| Still building context | missing | none | yes |  |
| No status line | missing | none | yes | Today shows none of Duet's status lines (Still building context, Quiet, Couldn't check), so nothing tells this state from them. |
| Quiet | missing | none | yes | The app knows a cycle was quiet but shows no line for it. |
| Couldn't check, with Retry | missing | none | yes | Nothing reads failed work or offers its Retry. Today's "Duet couldn't check right now" with Try again is the Today load failure (Couldn't load Today, with Retry); its Try again only reloads Today. |

## Your account/Set up/For my doctor

| State | Status | Where in the app | Server data | Note |
| --- | --- | --- | --- | --- |
| Nothing for your doctor yet | built | ConversationTab (duet-app lib.rs) | yes |  |
| For my doctor saved | partial | ConversationTab (duet-app lib.rs) | yes | The saved page, Edit page, Start fresh, the PDF and a share link; Send to my practice is missing. |

## Your account/Set up/Duet's proposed update for your doctor

| State | Status | Where in the app | Server data | Note |
| --- | --- | --- | --- | --- |
| No update proposed | built | ConversationTab (duet-app lib.rs) | yes |  |
| An update proposed.An update to review | built | ConversationTab (duet-app lib.rs) | yes | Review proposed doctor page, with Approve and save or Reject. |
| An update proposed.An update to review again | missing | none | yes | Nothing says the proposal changed since you reviewed it. |

## Your account/Set up/Your note

| State | Status | Where in the app | Server data | Note |
| --- | --- | --- | --- | --- |
| No note | built | YouTab (duet-app lib.rs) | yes |  |
| Note saved | partial | YouTab (duet-app lib.rs) | yes | Notes show in the library, from Add health information, with Delete; Correct changes only the title, so the note's text can't be edited. |

## Your account/Set up/Duet's proposed note

| State | Status | Where in the app | Server data | Note |
| --- | --- | --- | --- | --- |
| No note proposed | missing | none | yes | The app shows no proposed notes, so nothing tells this state from the next one. |
| A note to review | missing | none | yes | duet-client can approve a proposed note; the app doesn't show proposals. |

## Your account/Set up/Your medication

| State | Status | Where in the app | Server data | Note |
| --- | --- | --- | --- | --- |
| No medication of yours | built | YouTab (duet-app lib.rs) | yes |  |
| Taking it | partial | YouTab (duet-app lib.rs) | yes | Added from New medication and shown in the library with Delete; Correct changes only the name, and nothing marks a medication stopped. |
| Stopped, with the date | missing | none | no | Nothing marks a medication stopped. |

## Your account/Set up/Your appointment

| State | Status | Where in the app | Server data | Note |
| --- | --- | --- | --- | --- |
| No appointment date | missing | none | no | The app and wire have no appointment. |
| Appointment date known | missing | none | no |  |

## Your account/Set up/Your practice's appointment

| State | Status | Where in the app | Server data | Note |
| --- | --- | --- | --- | --- |
| No appointment from your practice | missing | none | no |  |
| Appointment from your practice | missing | none | no |  |

## Your account/Set up/Your doctor's plan

| State | Status | Where in the app | Server data | Note |
| --- | --- | --- | --- | --- |
| No plan from your doctor | built | YouTab (duet-app lib.rs) | yes |  |
| Your doctor's plan, current | partial | YouTab (duet-app lib.rs) | yes | A read-only "your doctor's plan" section on You names the physician and version (duet-api care.rs); Today doesn't open it, and answering a Check-in about it is missing. |
| Your doctor's plan, no longer updated | missing | none | no | Nothing marks the plan as no longer updated after the connection ends. |

## This device

| State | Status | Where in the app | Server data | Note |
| --- | --- | --- | --- | --- |
| Signed out on this device | partial | DuetRoot (duet-app account.rs) | yes | Sign in with Apple; email sign-in is missing. |
| Signed in on this device | partial | DuetRoot (duet-app account.rs) | yes | With Sign out; Sign out everywhere is missing. |

## This device/Signed in on this device/Your draft

| State | Status | Where in the app | Server data | Note |
| --- | --- | --- | --- | --- |
| Composer empty | partial | ConversationTab (duet-app lib.rs) | n/a | A refused message's words stay in the composer, so there's no Edit to bring them back. |
| Draft written | built | ConversationTab (duet-app lib.rs) | n/a | Kept through a failed send; not kept across a reload. |

## This device/Signed in on this device/Item attached to your draft

| State | Status | Where in the app | Server data | Note |
| --- | --- | --- | --- | --- |
| Nothing attached | missing | none | n/a | The composer carries text only and has no Ask about this, so nothing tells this state from the next one. |
| An item attached | missing | none | no |  |

## This device/Signed in on this device/Quote in your draft

| State | Status | Where in the app | Server data | Note |
| --- | --- | --- | --- | --- |
| No quote | missing | none | n/a | The thread has no way to quote a message, so nothing tells this state from the next one. |
| Quoting a message | missing | none | no |  |

## This device/Signed in on this device/Your command on your setup

| State | Status | Where in the app | Server data | Note |
| --- | --- | --- | --- | --- |
| No command on your setup in flight | missing | none | yes | None of its commands exist in the app. |
| Agree and continue on your setup: sending; other commands on it wait | missing | none | yes |  |
| Couldn't confirm Agree and continue on your setup, with Check again | missing | none | yes |  |
| Save name on your setup: sending; other commands on it wait | missing | none | yes |  |
| Couldn't confirm Save name on your setup, with Check again | missing | none | yes |  |
| Allow AI on your setup: sending; other commands on it wait | missing | none | yes |  |
| Couldn't confirm Allow AI on your setup, with Check again | missing | none | yes |  |
| Not now on your setup: sending; other commands on it wait | missing | none | yes |  |
| Couldn't confirm Not now on your setup, with Check again | missing | none | yes |  |

## This device/Signed in on this device/Your command on your AI settings

| State | Status | Where in the app | Server data | Note |
| --- | --- | --- | --- | --- |
| No command on your AI settings in flight | missing | none | yes | None of its commands exist in the app. |
| Allow AI on your AI settings: sending; other commands on it wait | missing | none | yes |  |
| Couldn't confirm Allow AI on your AI settings, with Check again | missing | none | yes |  |
| Withdraw permission on your AI settings: sending; other commands on it wait | missing | none | yes |  |
| Couldn't confirm Withdraw permission on your AI settings, with Check again | missing | none | yes |  |
| Allow the new version on your AI settings: sending; other commands on it wait | missing | none | yes |  |
| Couldn't confirm Allow the new version on your AI settings, with Check again | missing | none | yes |  |
| Turn off on your AI settings: sending; other commands on it wait | missing | none | yes |  |
| Couldn't confirm Turn off on your AI settings, with Check again | missing | none | yes |  |
| Turn on on your AI settings: sending; other commands on it wait | missing | none | yes |  |
| Couldn't confirm Turn on on your AI settings, with Check again | missing | none | yes |  |
| Pause proactive work on your AI settings: sending; other commands on it wait | missing | none | yes |  |
| Couldn't confirm Pause proactive work on your AI settings, with Check again | missing | none | yes |  |
| Resume proactive work on your AI settings: sending; other commands on it wait | missing | none | yes |  |
| Couldn't confirm Resume proactive work on your AI settings, with Check again | missing | none | yes |  |

## This device/Signed in on this device/Your command on your account

| State | Status | Where in the app | Server data | Note |
| --- | --- | --- | --- | --- |
| No command on your account in flight | missing | none | yes | None of its commands exist in the app. |
| Change name on your account: sending; other commands on it wait | missing | none | yes |  |
| Couldn't confirm Change name on your account, with Check again | missing | none | yes |  |
| Accept the new Terms on your account: sending; other commands on it wait | missing | none | yes |  |
| Couldn't confirm Accept the new Terms on your account, with Check again | missing | none | yes |  |
| Delete account on your account: sending; other commands on it wait | missing | none | yes |  |
| Couldn't confirm Delete account on your account, with Check again | missing | none | yes |  |
| Export on your account: sending; other commands on it wait | missing | none | yes |  |
| Couldn't confirm Export on your account, with Check again | missing | none | yes |  |

## This device/Signed in on this device/Your command on the reply

| State | Status | Where in the app | Server data | Note |
| --- | --- | --- | --- | --- |
| No command on the reply in flight | missing | none | yes | None of its commands exist in the app. |
| Try again on the reply: sending; other commands on it wait | missing | none | yes |  |
| Couldn't confirm Try again on the reply, with Check again | missing | none | yes |  |

## This device/Signed in on this device/Your command on your feedback

| State | Status | Where in the app | Server data | Note |
| --- | --- | --- | --- | --- |
| No command on your feedback in flight | missing | none | yes | None of its commands exist in the app. |
| Helpful on your feedback: sending; other commands on it wait | missing | none | yes |  |
| Couldn't confirm Helpful on your feedback, with Check again | missing | none | yes |  |
| Not helpful on your feedback: sending; other commands on it wait | missing | none | yes |  |
| Couldn't confirm Not helpful on your feedback, with Check again | missing | none | yes |  |
| Remove on your feedback: sending; other commands on it wait | missing | none | yes |  |
| Couldn't confirm Remove on your feedback, with Check again | missing | none | yes |  |
| Done on your feedback: sending; other commands on it wait | missing | none | yes |  |
| Couldn't confirm Done on your feedback, with Check again | missing | none | yes |  |

## This device/Signed in on this device/Your command on the Reminder

| State | Status | Where in the app | Server data | Note |
| --- | --- | --- | --- | --- |
| No command on the Reminder in flight | partial | ConversationTab (duet-app lib.rs) | yes | Pause and Resume; Change and Cancel are missing. |
| Pause on the Reminder: sending; other commands on it wait | built | ConversationTab (duet-app lib.rs) | yes |  |
| Couldn't confirm Pause on the Reminder, with Check again | partial | ConversationTab (duet-app lib.rs) | yes | A failure line treats a lost answer like a refusal and offers no Check again. |
| Resume on the Reminder: sending; other commands on it wait | built | ConversationTab (duet-app lib.rs) | yes |  |
| Couldn't confirm Resume on the Reminder, with Check again | partial | ConversationTab (duet-app lib.rs) | yes | A failure line treats a lost answer like a refusal and offers no Check again. |
| Change on the Reminder: sending; other commands on it wait | missing | none | yes | The wire can change a follow-up; the app can't. |
| Couldn't confirm Change on the Reminder, with Check again | missing | none | yes | The wire can change a follow-up; the app can't. |
| Cancel on the Reminder: sending; other commands on it wait | missing | none | yes | The wire can cancel a follow-up; the app can't. |
| Couldn't confirm Cancel on the Reminder, with Check again | missing | none | yes | The wire can cancel a follow-up; the app can't. |

## This device/Signed in on this device/Your command on the Check-in

| State | Status | Where in the app | Server data | Note |
| --- | --- | --- | --- | --- |
| No command on the Check-in in flight | partial | ConversationTab (duet-app lib.rs) | yes | Pause and Resume; Change, Cancel and Try again are missing. |
| Try again on the Check-in: sending; other commands on it wait | missing | none | yes |  |
| Couldn't confirm Try again on the Check-in, with Check again | missing | none | yes |  |
| Pause on the Check-in: sending; other commands on it wait | built | ConversationTab (duet-app lib.rs) | yes |  |
| Couldn't confirm Pause on the Check-in, with Check again | partial | ConversationTab (duet-app lib.rs) | yes | A failure line treats a lost answer like a refusal and offers no Check again. |
| Resume on the Check-in: sending; other commands on it wait | built | ConversationTab (duet-app lib.rs) | yes |  |
| Couldn't confirm Resume on the Check-in, with Check again | partial | ConversationTab (duet-app lib.rs) | yes | A failure line treats a lost answer like a refusal and offers no Check again. |
| Change on the Check-in: sending; other commands on it wait | missing | none | yes |  |
| Couldn't confirm Change on the Check-in, with Check again | missing | none | yes |  |
| Cancel on the Check-in: sending; other commands on it wait | missing | none | yes |  |
| Couldn't confirm Cancel on the Check-in, with Check again | missing | none | yes |  |

## This device/Signed in on this device/Your command on the Plan

| State | Status | Where in the app | Server data | Note |
| --- | --- | --- | --- | --- |
| No command on the Plan in flight | built | ConversationTab (duet-app lib.rs) | yes |  |
| Save on the Plan: sending; other commands on it wait | built | ConversationTab (duet-app lib.rs) | yes |  |
| Couldn't confirm Save on the Plan, with Check again | partial | ConversationTab (duet-app lib.rs) | yes | A failure line treats a lost answer like a refusal and offers no Check again. |
| Delete on the Plan: sending; other commands on it wait | built | ConversationTab (duet-app lib.rs) | yes |  |
| Couldn't confirm Delete on the Plan, with Check again | partial | ConversationTab (duet-app lib.rs) | yes | A failure line treats a lost answer like a refusal and offers no Check again. |

## This device/Signed in on this device/Your command on your note

| State | Status | Where in the app | Server data | Note |
| --- | --- | --- | --- | --- |
| No command on the note in flight | partial | YouTab (duet-app lib.rs) | yes | Add and Delete; Correct changes only the title. |
| Add on the note: sending; other commands on it wait | built | YouTab (duet-app lib.rs) | yes |  |
| Couldn't confirm Add on the note, with Check again | partial | YouTab (duet-app lib.rs) | yes | A failure line treats a lost answer like a refusal and offers no Check again. |
| Save on the note: sending; other commands on it wait | partial | YouTab (duet-app lib.rs) | yes | Correct sends a new title while Delete waits; the note's text can't be changed. |
| Couldn't confirm Save on the note, with Check again | partial | YouTab (duet-app lib.rs) | yes | A failure line treats a lost answer like a refusal and offers no Check again. |
| Delete on the note: sending; other commands on it wait | built | YouTab (duet-app lib.rs) | yes |  |
| Couldn't confirm Delete on the note, with Check again | partial | YouTab (duet-app lib.rs) | yes | A failure line treats a lost answer like a refusal and offers no Check again. |

## This device/Signed in on this device/Your command on your medication

| State | Status | Where in the app | Server data | Note |
| --- | --- | --- | --- | --- |
| No command on the medication in flight | partial | YouTab (duet-app lib.rs) | yes | Add and Delete; Correct changes only the name, Stop is missing, and confirming is only a checkbox inside a For my doctor save. |
| Add on the medication: sending; other commands on it wait | built | YouTab (duet-app lib.rs) | yes |  |
| Couldn't confirm Add on the medication, with Check again | partial | YouTab (duet-app lib.rs) | yes | A failure line treats a lost answer like a refusal and offers no Check again. |
| Confirm on the medication: sending; other commands on it wait | partial | ConversationTab (duet-app lib.rs) | yes | Confirming happens only as a checkbox inside a For my doctor save, not on the medication itself. |
| Couldn't confirm Confirm on the medication, with Check again | partial | YouTab (duet-app lib.rs) | yes | A failure line treats a lost answer like a refusal and offers no Check again. |
| Save on the medication: sending; other commands on it wait | partial | YouTab (duet-app lib.rs) | yes | Correct sends a new name while Delete waits; dose, schedule and notes can't be changed. |
| Couldn't confirm Save on the medication, with Check again | partial | YouTab (duet-app lib.rs) | yes | A failure line treats a lost answer like a refusal and offers no Check again. |
| Stop on the medication: sending; other commands on it wait | missing | none | no | Nothing marks a medication stopped. |
| Couldn't confirm Stop on the medication, with Check again | missing | none | no | Nothing marks a medication stopped. |
| Delete on the medication: sending; other commands on it wait | built | YouTab (duet-app lib.rs) | yes |  |
| Couldn't confirm Delete on the medication, with Check again | partial | YouTab (duet-app lib.rs) | yes | A failure line treats a lost answer like a refusal and offers no Check again. |

## This device/Signed in on this device/Your command on the insight

| State | Status | Where in the app | Server data | Note |
| --- | --- | --- | --- | --- |
| No command on the insight in flight | missing | none | yes | None of its commands exist in the app. |
| Not accurate on the insight: sending; other commands on it wait | missing | none | yes | The wire can correct an insight; Today's cards have no control. |
| Couldn't confirm Not accurate on the insight, with Check again | missing | none | yes | The wire can correct an insight; Today's cards have no control. |

## This device/Signed in on this device/Your command on For my doctor

| State | Status | Where in the app | Server data | Note |
| --- | --- | --- | --- | --- |
| No command on For my doctor in flight | partial | ConversationTab (duet-app lib.rs) | yes | Save, Start fresh, Save the update and Dismiss the update; Send to my practice is missing. |
| Save on For my doctor: sending; other commands on it wait | built | ConversationTab (duet-app lib.rs) | yes |  |
| Couldn't confirm Save on For my doctor, with Check again | built | ConversationTab (duet-app lib.rs) | yes |  |
| Start fresh on For my doctor: sending; other commands on it wait | built | ConversationTab (duet-app lib.rs) | yes |  |
| Couldn't confirm Start fresh on For my doctor, with Check again | built | ConversationTab (duet-app lib.rs) | yes | "Duet couldn't confirm that For my doctor was cleared"; Start fresh resends the same retained operation. |
| Send to my practice on For my doctor: sending; other commands on it wait | missing | none | no | The app makes a share link and a PDF; nothing sends For my doctor to your practice. |
| Couldn't confirm Send to my practice on For my doctor, with Check again | missing | none | no | The app makes a share link and a PDF; nothing sends For my doctor to your practice. |
| Save the update on For my doctor: sending; other commands on it wait | built | ConversationTab (duet-app lib.rs) | yes |  |
| Couldn't confirm Save the update on For my doctor, with Check again | built | ConversationTab (duet-app lib.rs) | yes | The exact save is retained, with Retry exact original save and Reload saved page to reconcile, as for Save. |
| Dismiss the update on For my doctor: sending; other commands on it wait | built | ConversationTab (duet-app lib.rs) | yes |  |
| Couldn't confirm Dismiss the update on For my doctor, with Check again | partial | ConversationTab (duet-app lib.rs) | yes | A failure shows the error and reloads the page, which shows whether it went through; there's no Check again. |

## This device/Signed in on this device/Your command on your appointment

| State | Status | Where in the app | Server data | Note |
| --- | --- | --- | --- | --- |
| No command on your appointment in flight | missing | none | yes | None of its commands exist in the app. |
| Save date on your appointment: sending; other commands on it wait | missing | none | no |  |
| Couldn't confirm Save date on your appointment, with Check again | missing | none | no |  |
| Remove date on your appointment: sending; other commands on it wait | missing | none | no |  |
| Couldn't confirm Remove date on your appointment, with Check again | missing | none | no |  |

## This device/Signed in on this device/Your command on your practice

| State | Status | Where in the app | Server data | Note |
| --- | --- | --- | --- | --- |
| No command on your practice in flight | partial | YouTab (duet-app lib.rs), SharedTab (duet-app lib.rs) | yes | Share only turns sharing back on for a practice already linked, and Stop sharing keeps it linked; joining from an invitation never shows. |
| Share on your practice: sending; other commands on it wait | partial | YouTab (duet-app lib.rs), SharedTab (duet-app lib.rs) | yes | Share with my physician turns sharing back on for a practice already linked, and waits only on You; joining from an invitation (JoinScreen) is never shown. |
| Couldn't confirm Share on your practice, with Check again | partial | YouTab (duet-app lib.rs), SharedTab (duet-app lib.rs) | yes | A failure line treats a lost answer like a refusal and offers no Check again. |
| Leave on your practice: sending; other commands on it wait | partial | YouTab (duet-app lib.rs), SharedTab (duet-app lib.rs) | yes | Stop sharing waits only on You, and it keeps the practice linked instead of ending the connection. |
| Couldn't confirm Leave on your practice, with Check again | partial | YouTab (duet-app lib.rs), SharedTab (duet-app lib.rs) | yes | A failure line treats a lost answer like a refusal and offers no Check again. |

## This device/Signed in on this device/Your command on the organization

| State | Status | Where in the app | Server data | Note |
| --- | --- | --- | --- | --- |
| No command on the organization in flight | missing | none | yes | None of its commands exist in the app. |
| Refresh on the organization: sending; other commands on it wait | missing | none | yes |  |
| Couldn't confirm Refresh on the organization, with Check again | missing | none | yes |  |
| Disconnect on the organization: sending; other commands on it wait | missing | none | yes |  |
| Couldn't confirm Disconnect on the organization, with Check again | missing | none | yes |  |

## This device/Signed in on this device/Your command on your phone's health data

| State | Status | Where in the app | Server data | Note |
| --- | --- | --- | --- | --- |
| No command on your phone's health data in flight | missing | none | yes | None of its commands exist in the app. |
| Disconnect on your phone's health data: sending; other commands on it wait | missing | none | yes |  |
| Couldn't confirm Disconnect on your phone's health data, with Check again | missing | none | yes |  |

## This device/Signed in on this device/Your command on your alerts

| State | Status | Where in the app | Server data | Note |
| --- | --- | --- | --- | --- |
| No command on your alerts in flight | partial | YouTab (duet-app lib.rs) | yes | The New insight notifications switch is kept on this device, not your account. |
| Turn on on your alerts: sending; other commands on it wait | missing | none | yes | The New insight notifications switch changes on this device at once; nothing goes to your account, so nothing waits. |
| Couldn't confirm Turn on on your alerts, with Check again | missing | none | yes | The switch sends nothing to your account, so no lost answer shows and there is no Check again. |
| Turn off on your alerts: sending; other commands on it wait | missing | none | yes | As for Turn on. |
| Couldn't confirm Turn off on your alerts, with Check again | missing | none | yes | As for Turn on. |

## This device/Signed in on this device/Your command on the proposed note

| State | Status | Where in the app | Server data | Note |
| --- | --- | --- | --- | --- |
| No command on the proposed note in flight | missing | none | yes | None of its commands exist in the app. |
| Save on the proposed note: sending; other commands on it wait | missing | none | yes |  |
| Couldn't confirm Save on the proposed note, with Check again | missing | none | yes |  |
| Dismiss on the proposed note: sending; other commands on it wait | missing | none | yes |  |
| Couldn't confirm Dismiss on the proposed note, with Check again | missing | none | yes |  |

## This device/Signed in on this device/Tab

| State | Status | Where in the app | Server data | Note |
| --- | --- | --- | --- | --- |
| On Duet, the conversation | partial | PrivateRoot TabBar (duet-app lib.rs) | n/a | The app also has a Shared tab the graph doesn't. Settings and AI settings are missing. |
| On Today | partial | PrivateRoot TabBar (duet-app lib.rs) | n/a | Today opens nothing: insights, measurements, Plans, Reminders, Check-ins, For my doctor and your doctor's plan open from the thread or You, if at all. Settings, AI settings and Ask about this are missing. |
| On You | partial | PrivateRoot TabBar (duet-app lib.rs) | n/a | You is one page: records groups, records, Measurements & trends, Health history, Earlier insights, search, Sources & data and Ask about this are missing. |

## This device/Signed in on this device/Open on Today

| State | Status | Where in the app | Server data | Note |
| --- | --- | --- | --- | --- |
| Today itself | partial | TodayTab (duet-app lib.rs) | n/a | Plans, Reminders, Check-ins and your doctor's plan don't open from Today. |
| Your Plan open | partial | ConversationTab (duet-app lib.rs) | yes | The Plan opens from its card in the Duet thread, not from Today. |
| The Reminder open | partial | ConversationTab (duet-app lib.rs) | yes | As above. |
| The Check-in open | partial | ConversationTab (duet-app lib.rs) | yes | As above. |
| Your doctor's plan open | partial | YouTab (duet-app lib.rs) | yes | A section on You, not an item opened from Today. |
| Your Plan open, removed since | partial | ConversationTab (duet-app lib.rs) | yes | "This saved plan was removed." after your own removal in the thread; a removal elsewhere isn't noticed. |

## This device/Signed in on this device/What Today shows

| State | Status | Where in the app | Server data | Note |
| --- | --- | --- | --- | --- |
| Loading Today | built | TodayTab (duet-app lib.rs) | yes |  |
| Showing Today | built | TodayTab (duet-app lib.rs) | yes |  |
| Showing Today, with one quiet line and Retry | partial | TodayTab (duet-app lib.rs) | yes | A failed reload over shown insights keeps them with no line; over a quiet Today it shows the full unavailable alert. |
| Couldn't load Today, with Retry | built | TodayTab (duet-app lib.rs) | yes |  |

## This device/Signed in on this device/The item open on Today

| State | Status | Where in the app | Server data | Note |
| --- | --- | --- | --- | --- |
| Loading the item | missing | none | yes | Items open from the lists loaded with the thread; the app never loads the item itself, so it shows that copy however old. |
| Showing the item | partial | ConversationTab (duet-app lib.rs) | yes | The item shows over the thread it opened from; Today opens nothing and has nothing to go back to. |
| Showing the item, with one quiet line and Retry | missing | none | yes | Nothing says an open item may be out of date. |
| Couldn't load the item, with Retry | missing | none | yes | As for loading: with no load of its own, nothing can say it failed or offer Retry. |
| Nothing open on Today | partial | TodayTab (duet-app lib.rs) | n/a | Today opens nothing: insights, measurements, Plans, Reminders, Check-ins, For my doctor and your doctor's plan open from the thread or You, if at all. |

## This device/Signed in on this device/Your health library read

| State | Status | Where in the app | Server data | Note |
| --- | --- | --- | --- | --- |
| Loading your health library | built | YouTab (duet-app lib.rs) | yes |  |
| Showing your health library | built | YouTab (duet-app lib.rs) | yes |  |
| Showing your health library, with one quiet line and Retry | missing | none | yes | A failed reload keeps the library with no line. |
| Couldn't load your health library, with Retry | built | YouTab (duet-app lib.rs) | yes |  |

## This device/Signed in on this device/The page open on You

| State | Status | Where in the app | Server data | Note |
| --- | --- | --- | --- | --- |
| Loading the page | missing | none | yes | You is one page; records groups, Measurements, Health history, Earlier insights, search and Sources & data don't open. |
| Showing the page | missing | none | yes |  |
| Showing the page, with one quiet line and Retry | missing | none | yes |  |
| Couldn't load the page, with Retry | missing | none | yes |  |
| Only the library open | partial | YouTab (duet-app lib.rs) | yes | You is one page: none of its pages, search, Settings or AI settings open. |

## This device/Signed in on this device/The conversation read

| State | Status | Where in the app | Server data | Note |
| --- | --- | --- | --- | --- |
| Loading the conversation | built | ConversationTab (duet-app lib.rs) | yes |  |
| Showing the conversation | built | ConversationTab (duet-app lib.rs) | yes |  |
| Showing the conversation, with one quiet line and Retry | built | ConversationTab (duet-app lib.rs) | yes | The kept thread with "Chat isn't available right now" and Try again. |
| Couldn't load the conversation, with Retry | built | ConversationTab (duet-app lib.rs) | yes |  |

## This device/Signed in on this device/Open on Duet

| State | Status | Where in the app | Server data | Note |
| --- | --- | --- | --- | --- |
| The conversation itself | built | ConversationTab (duet-app lib.rs) | n/a |  |
| Supporting information open | partial | ConversationTab, SourceButton (duet-app lib.rs) | yes | A source chip opens the cited source over the thread; Ask about this is missing. |

## This device/Signed in on this device/Open on Duet/Supporting information open/The supporting information read

| State | Status | Where in the app | Server data | Note |
| --- | --- | --- | --- | --- |
| Loading the supporting information | built | load_source (duet-app lib.rs) | yes |  |
| Showing the supporting information | built | load_source (duet-app lib.rs) | yes |  |
| Showing the supporting information, with one quiet line and Retry | missing | none | yes | Each open loads afresh and keeps nothing, so a failed refresh can't keep what you were reading with a quiet line. |
| Couldn't load the supporting information, with Retry | partial | load_source (duet-app lib.rs) | yes | "This source couldn't be opened." with no Retry. |

## This device/Signed in on this device/Open on Duet/Supporting information open/The cited version

| State | Status | Where in the app | Server data | Note |
| --- | --- | --- | --- | --- |
| No newer version known | built | load_source (duet-app lib.rs) | yes |  |
| A note that a newer version exists | built | load_source (duet-app lib.rs) | yes | "This is the cited version. A newer version of this record is available." from the wire's superseded flag. |

## Your other devices

| State | Status | Where in the app | Server data | Note |
| --- | --- | --- | --- | --- |
| No other device signed in | not shown | none | n/a |  |
| Signed in on another device | missing | none | yes | No Sign out everywhere; the wire's session revoke isn't called. |

## Duet on this device

| State | Status | Where in the app | Server data | Note |
| --- | --- | --- | --- | --- |
| Duet on screen | not shown | none | n/a | The app doesn't report foreground use (the wire's record_foreground). |
| Duet in the background | partial | duet-notify | yes | Insight notifications are scheduled on the device whether or not Duet is open. |

## This device's alerts

| State | Status | Where in the app | Server data | Note |
| --- | --- | --- | --- | --- |
| Duet hasn't asked for alerts yet | built | YouTab (duet-app lib.rs) | n/a | Allow notifications. |
| Alerts allowed on this device | built | YouTab (duet-app lib.rs) | n/a |  |
| Alerts not allowed on this device | built | YouTab (duet-app lib.rs) | n/a | Open iPhone Settings. |

## Notification on this device

| State | Status | Where in the app | Server data | Note |
| --- | --- | --- | --- | --- |
| No notification waiting | not shown | none | n/a |  |
| A notification from Duet waiting | partial | duet-notify | yes | Only local insight notifications; messages, Check-ins and your practice's news don't notify. The wire has push registration. |

## Your practice

| State | Status | Where in the app | Server data | Note |
| --- | --- | --- | --- | --- |
| No practice connection | built | YouTab (duet-app lib.rs), SharedTab (duet-app lib.rs) | yes |  |
| A practice's invitation waits for you | missing | JoinScreen (duet-app lib.rs) | yes | JoinScreen exists but PrivateRoot never shows it (needs_join is false). |
| Connected to your practice, sharing | partial | YouTab (duet-app lib.rs), SharedTab (duet-app lib.rs) | yes | Sharing with my physician, with Stop sharing (duet-api care.rs). Stop sharing keeps the practice linked and offers Share again, where the graph's Leave ends the connection. |
| Connection ended.You left your practice | partial | SharedTab (duet-app lib.rs) | yes | Stop sharing withdraws consent and offers Share with my physician again; nothing says the connection ended. |
| Connection ended.Your practice ended the connection, with a dated notice | missing | none | no | No discharge or dated notice. |

## Records from your sources

| State | Status | Where in the app | Server data | Note |
| --- | --- | --- | --- | --- |
| No records from your sources yet | built | YouTab (duet-app lib.rs) | yes | "No records or measurements yet." |
| Records from your sources kept | partial | YouTab (duet-app lib.rs) | yes | The library shows them, but Correct and Delete let you rewrite or remove what your sources sent, which the graph keeps exactly as sent. |

## Doctors & hospitals: an organization

| State | Status | Where in the app | Server data | Note |
| --- | --- | --- | --- | --- |
| Organization not connected | missing | none | yes | No Doctors & hospitals in the app. The wire carries portal organizations and connections; duet-api doesn't serve them. |
| Signing in at the organization's portal | missing | none | yes |  |
| Organization connected.Importing your records | missing | none | yes |  |
| Organization connected.Up to date | missing | none | yes |  |
| Organization connected.Up to date except items it couldn't read, named | missing | none | yes |  |
| Organization connected.Couldn't update; your records stay, and the next refresh tries again | missing | none | yes |  |
| Organization connected.Couldn't update since its last import, with Reconnect | missing | none | yes |  |
| Organization connected.Signing in again at the portal | missing | none | yes |  |

## Your phone's health data

| State | Status | Where in the app | Server data | Note |
| --- | --- | --- | --- | --- |
| Phone not connected | built | YouTab (duet-app lib.rs) (duet-health) | yes | Allow HealthKit. |
| Phone connected.Bringing in your history | missing | none | yes | Readings show as they arrive; nothing says history is still coming. |
| Phone connected.Up to date from your phone | partial | TodayTab (duet-app lib.rs), YouTab (duet-app lib.rs) (duet-health) | yes | Your phone's readings, by source. The app counts you connected once you've asked for access (HealthKitBridge.swift) and shows current readings; nothing marks the history import finished or names gaps in coverage. |
| Access turned off in your phone's settings; your readings stay | missing | none | yes | The app can't tell access turned off from never granted, and doesn't say your readings stay. |
