# Duet

This is what we're building for patients: what Duet is, why it exists, and how it should behave. It describes the product as we intend it today. Open questions live in DECISIONS.md, and git keeps the history.

## What Duet is

Duet is a person's own health agent, on iPhone and Android. Think of the personal agents people already use, like Instinct, Muse, Hermes or OpenClaw, managed and maintained for them and focused on their health. The conversation is the product. It helps the person understand their health, use their connected history, remember and correct what matters, follow through, notice changes and get ready for care, without ever having to retell their story.

Duet is free, it stands on its own and it needs no doctor. Joining a practice is never required, and when a person's practice uses Coda, Duet connects to it through Connected care. Records plus AI are becoming free, so Duet doesn't compete on records. It competes on the person's connection to their own doctor.

Duet isn't a diagnostic service, a clinical alarm, a treatment authority or a replacement medical record. It explains health information and documented care faithfully, and it leaves care decisions to the person and their clinicians.

## The point

A person should feel understood by something that actually knows their health, does useful work for them without being asked, and makes the time with their doctor better. Everything Duet shows outside the conversation (Today, Plans, Reminders, Check-ins, You and For my doctor) exists so the things that matter live beyond the chat. Those surfaces are views of what Duet and the person keep, written by Duet's tool calls. The person changes them by telling Duet, or with a direct control as a shortcut, and Duet handles the consequences with judgment instead of dialogs.

## Principles

Coda's shared principles (PRINCIPLES.md) govern this spec. These are the ones specific to Duet.

1. **A managed health agent.** The conversation is the product, and every other surface is a layer over it.
2. **Free and standalone.** Duet works with no doctor and no invitation. A practice connection attaches to the person's own Duet account.
3. **One conversation, like texting.** There is one native thread, never a list of chats, and it feels as fluid as iMessage. Replies read like texts: short, with lists or headings only when asked. Everything Duet has for the person arrives as a message in that thread, and Today pins what's current.
4. **Useful with zero effort, and proactive.** Duet is useful before any records are connected, and the person never has to be proactive for Duet to help. Duet proposes as much as it usefully can, and its own work may take on anything it judges worth doing.
5. **A message the person sent is never lost.** It belongs to their account, not the device. Their newer message, or their own control, always takes over from background work.
6. **The person is in control.** Everything saved, reviewed or shared stays under their control. Nothing about them leaves Duet without their explicit choice, and Duet's private notes never leave.
7. **A living portrait, not a dashboard.** Today isn't a feed or a quota. Duet judges how many insights are worth pinning, and quiet counts as success. Quiet applies to observations; useful, actionable proposals are still offered.
8. **No pressure.** Duet has no health score, streak or progress percentage, and never pushes the person to disclose more. Missing information is shown honestly, never as a personal failure.
9. **Explain, never decide care.** Duet doesn't diagnose, prescribe, assess personal risk or urgency, or decide care. This limits what Duet tells the person. It never limits what Duet investigates or takes on.
10. **One AI with full reach.** Chat, Today, Check-ins and For my doctor all reach the same authorized records, memory, conversations and Plans. Code enforces identity, permissions, acceptance, versions, saves and sharing.
11. **Grounded, cited and faithful.** Facts and calculations come from evidence Duet actually received, with citations that reopen it. Records show as the source sent them. Every claim Duet makes, such as "read", "current" or "will try again", traces to recorded evidence, and Duet is honest about gaps and about how old its information is.
12. **No silence that looks like health.** Anything behind, stuck or degraded is shown, with its reason, to someone who can act.
13. **Live where you work; history as it was.** What the person is looking at updates live. Anything already said or sent keeps the version it used and is marked when a newer one exists.
14. **Plain, calm and smooth.** Duet follows the shared UI principles (PRINCIPLES.md U1–U9). It shows no internal or technical language and few controls. Work happens on the server, so the app stays responsive and new information arrives smoothly. Duet follows the phone's light or dark appearance.

## The concepts

**Duet's AI.** The single AI inside Duet, named "Duet's AI" to the person. It's a warm, capable health agent that learns about the person, helps them understand and act, and notices what changes. It's for the person's health only, not a general-purpose assistant. It is candid and willing to investigate and give a considered view of the evidence, the options and what to ask a clinician, and it never makes a personal diagnosis, urgency or treatment call. It runs only while the person has accepted the Terms, allowed AI and the operator switch is on.

**The conversation.** One continuous thread per person that never ends and survives interruption, reconnecting and restarts. It's where the person asks, explains, corrects and works things out. It holds their messages, Duet's replies, delivered Reminders and Check-ins, what Duet's own work noticed, proposals and news from their practice. It is purely chat, never a control panel. The person can delete a single message, like in iMessage, which removes only that message; what Duet knows lives in its memory, which the person corrects by telling Duet. Reading a message on any device marks it seen everywhere. Practice news stays in the thread after a connection ends.

**Today.** The person's care home. It pins new insights, accepted Plans, Reminders and Check-ins, the doctor's plan, appointments and the way into For my doctor. It also shows "Quiet", or "Couldn't check" with Retry, for Duet's latest own work. Today stays useful on quiet days; insights never hide accepted care, and a quiet day never prompts filler.

**Insight.** An observation Duet's own work wrote from the person's evidence, about what's worth noticing right now. It arrives in the thread as part of one Duet message, and Today pins it. It stays an observation: no diagnosis, risk, urgency or treatment. Once published it doesn't change; a correction is marked on it and the original stays as published. An insight isn't saved to For my doctor directly. The person adds it from the insight's detail or through chat, or accepts Duet's proposal. A quiet Earlier insights link under Today's insights opens past insights by week, newest first, each opening the same detail. Today itself never becomes a feed.

**Duet's own work.** Work Duet does without being asked. It looks at the person's data on a schedule, and beyond that it may take on anything it judges worth doing, such as following a trend or connecting the doctor's plan to readings. It isn't limited to accepted work. It runs under the person's consent, AI permission and current Terms; one AI permission covers all of Duet's AI, with no separate pause. Its output is one message in the thread, which Today pins, or nothing at all. It is never labeled a Check-in.

**Re-engagement.** When the person goes quiet, Duet reaches out only for a real reason in their life: new records or results, an upcoming appointment, a trend in their data, news from their practice, or something they were working on. It never sends a generic "come back" message. It's one message in the thread, alerting under the phone's own permission. If the person doesn't respond, Duet backs off by its own judgment, and its next message needs a new real reason. When they come back, Duet catches them up on what changed. Reaching out happens only inside the app; there's no email or SMS.

**Plan.** A saved routine with no schedule of its own. A Reminder or Check-in belongs to at most one Plan. A Plan's detail shows its checklist and the Reminders and Check-ins that belong to it, with no scores, streaks or done-ticks. A Plan and its follow-ups stay separate, so pausing a Reminder never removes the Plan or changes documented care instructions.

**Reminder.** A follow-up the person asked for. At its time it posts the person's agreed words unchanged, and Duet may add something useful after them. Reminders are set up through Duet's AI, so while AI isn't allowed they don't run: their times pass, and the next one after AI is allowed again delivers as normal.

**Check-in.** Later work the person explicitly asked Duet to do, or a proposal they accepted. When it's due, Duet does the work with current context and stays within the limits the person accepted. It runs until the person pauses or cancels it. If it keeps failing, Duet tells the person in the thread and judges whether to keep trying, pause it or ask them. A Check-in about a measurement shows its latest readings on its Today card.

**Notes and medications.** The person's own entries in You. In conversation Duet saves or changes them when asked. A medication they stop stays in their list, marked stopped with the date, and leaves "What I take". While connected, these entries reach the practice; Duet's private notes never do.

**Patient memory.** Duet's private, correctable understanding of the person: their preferences, goals, constraints and concerns. Today and the conversation build on the same memory. Duet corrects it in conversation and says what changed. It never leaves Duet.

**You.** The person's library of records, measurements, medications and visits, with exact values, dates and sources. It's a faithful data dump: no cleaning, curating or model sorting, laid out by each source's own structure so it never reads as confusing. Tapping a lab shows its trend as a line graph; each line comes from one whole source series and is never spliced, and a test from more than one lab shows one lab's line at a time with a switcher. A lab's printed range band and flag tag don't appear. Tapping a measurement shows its readings over time the same way, from one source series. Blood pressure shows both numbers together, and sleep shows each night. The person can search everything in You. Connecting a source starts here, under Sources & data, near the top, and those controls are always easy to find. The person can edit their display name but not their legal name.

**Source and citation.** A source is a connected patient portal, Apple Health or Health Connect, and the imported original always stays inspectable. A citation reopens exactly the evidence the AI received. In chat, each message with sources has one Sources button that opens a sheet.

**For my doctor.** One continuing workspace the patient owns, where they help their providers understand what matters, what changed, what they've tried and what they need next. It's useful with or without an appointment date. It's one live version everywhere: it shows each record's current version, and a save never erases a newer save. It's separate from the practice connection and reaches a practice only when the patient sends it. Physicians don't approve its wording or manage it.

**Appointment context.** Dates and details the patient provides or a connected practice's schedule supplies, attached to For my doctor.

**Your doctor's plan.** The documented care decisions and open items a connected practice sends back, with source, author, date and status. It's read only, lives on Today, and Duet explains it faithfully. With more than one connected practice, each practice's plan shows separately, labeled with its practice, and Duet explains them together when asked. When a connection ends, the plan stays, marked no longer updated.

**Shared.** The practice-connection tab. Practice sharing stays internal during beta, so the beta app shows no practice card or consent controls. It has no person-to-person People list or sharing notices.

**Connected care.** An invited practice's consented connection to the person's Duet account. Patients join a practice only through that practice's invitation and their consent, and joining links to the account they already have. A person may connect to more than one practice, each connection separate. Every Duet flow works without a connected practice.

**Standing notice.** The fixed statement of what Duet's AI is and its limits: "Duet's AI isn't a doctor. It explains health information and doesn't diagnose or treat. In an emergency, call 911." It's shown at sign-up and kept within reach in chat, without being repeated in every reply.

**Companion.** A small face the person picks during setup. It's Duet's face, not a second assistant, and it changes how Duet looks, not what it can do. Its name stays on the device and is never sent to the AI. It never reacts to health and plays one-shot animations only on the person's own actions.

## Journeys

### D1. Sign up, talk to Duet, keep everything across a restart

1. The person installs Duet from TestFlight on iPhone or from a Google Play testing link on Android. A public store listing is the goal.
2. They sign up on their own, with no invitation: Apple or email on iPhone, Google or email on Android, and email for practice invitations. Duet has no web surface; an invitation link opens the app.
3. They see the standing notice.
4. They accept the Terms with one explicit "I agree and continue", with no prechecked box. A quiet Sign out sits under this step for a wrong account or declined Terms.
5. They make one choice about AI: Allow AI or Not now. Not now keeps every non-AI part of the app working.
6. An optional last step offers to connect their health, with every available connector on one panel and Not now. Chat never waits on imports.
7. They land in the conversation. Records and doctor preparation are optional next steps.
8. They write a message, and Duet answers usefully even with no records connected. If records are still importing, chat works anyway, and Duet says plainly when what it has can't answer yet.
9. While a reply is coming, a typing indicator shows in its own bubble. There's no fake typing, progress figure or activity panel.
10. **Leaving mid-reply.** If they leave while Duet writes, a generic lock-screen alert arrives with the reply and opens the conversation at it. A reply that can't finish alerts too, as "Duet couldn't finish its reply".
11. **App restart.** The person is still signed in, the conversation is intact, and Duet reopens where they were, with any draft they left.
12. **Signing out before a message is confirmed.** The message still arrives and Duet replies; both show when they sign back in anywhere.
13. **Interrupted reply.** The person's message is kept, text already streamed is kept, and an action the reply already committed is neither erased nor done twice.
14. **Revoked session, or another account.** Either is refused, including after a restart.
15. **Duet unreachable.** After a short wait, "Connecting…" takes the place of the screen title, as in Messages, so nothing is pushed down or covered. The cached conversation and drafts stay usable while Duet checks the account.
16. **AI not allowed.** Empty chat shows a small symbol, the real reason, and the matching settings action or View Today. The draft is kept.

### D2. Records come in, citations reopen them, corrections change later answers

1. The person opens Sources & data in You, or the setup panel, and searches "Doctors & hospitals" by name, and Duet works out which portal vendor it is. The list shows exactly the portals that are switched on: Epic/MyChart and Oracle Health first, healow, and athena once its route works.
2. They can connect several organizations, including several MyChart organizations. Each one connects, refreshes, fails and disconnects on its own.
3. They let Duet read Apple Health on iPhone or Health Connect on Android. Wearables linked to Apple Health come through HealthKit, with one instruction to turn that on in the wearable's app.
4. Duet imports all the available history and never calls an import complete when it isn't. One unreadable record fails alone, with its reason, and the rest publish. A part that never becomes readable ends the import as partial, with the rest usable and the next daily refresh trying again.
5. The person browses You. Records show as the source sent them, including entered-in-error or cancelled, and Duet explains the status. Only exact duplicates are removed; a likely duplicate from two sources shows in the review sheet as "looks like the same result as …", and the person keeps one or both. A condition shown as current isn't also shown as past.
6. They ask about a record by typing or with "Ask about this", which brings that exact record or period into the conversation. The selected version and evidence window carry over, and a newer measurement doesn't replace an older insight's data.
7. Duet reads the exact record and answers with a citation, and the Sources sheet reopens what Duet received.
8. **Correcting Duet.** The person says something is wrong. Duet uses the correction from then on without losing the rest of their story, and says what changed.
9. **Correction at the source.** A correction or deletion at the source carries through to You and to later answers. The person's own wording or corrections are dated and attributed and never silently replace the imported original.
10. **Daily refresh.** Each portal refreshes daily, and one portal failing doesn't block the others.
11. **A connection stops working.** The source in You shows "Couldn't update since <day>" with Reconnect, and Duet may mention it in conversation when it matters. There's no separate alert.
12. **Missing on a fetch.** A record missing from one fetch, or an empty or failed read, is never treated as a deletion.
13. **Deleted in Apple Health.** Duet recalculates that day's total and drops it from current evidence if no readings remain. Earlier versions stay for citations that already use them.
14. **A source fails during an answer.** Duet answers from the evidence it has, says how old that information is, and explains the failure only when it limits the answer.
15. **Disconnect and reconnect.** The person can always disconnect, even while newer Terms wait. Disconnecting stops new imports and keeps the imported history, which the person can still delete, or they can delete the account. Reconnecting the same source carries on with that history.
16. **A vendor is switched off.** It leaves search and blocks new sign-ins only. Existing connections stay connected and refreshing, and disconnect always works.
17. **Restart while uploading.** Phone uploads not yet sent survive the restart.
18. **Health Connect.** Duet asks for every Health Connect data type Google approves, the person grants each in Google's permission sheet, and Duet judges what's useful. Unsupported data and background limits are stated to the person.

### D3. Duet's own work, Plans, Reminders and Check-ins

Duet's own work:

1. Duet's own work runs on the server and reads records, memory, conversations and Plans, just as chat can. A new person's first look starts right away.
2. Duet publishes only when something is useful now, actionable or a material change. What it found posts in the thread as one message, and Today pins the insight.
3. **Quiet or couldn't finish.** The thread says nothing. Today shows "Quiet", or "Couldn't check" with a Retry that runs the look again now. Both speak for Duet's latest work and clear when their time passes.
4. The person opens the insight and sees its evidence and its limits. From there they can Ask about this, or mark it Not accurate, which is kept as feedback and removes the insight from Today.
5. **A correction before publishing.** If the person corrects something unfinished work relied on, the stale result isn't published and later work uses the correction.
6. **Inactivity.** Accepted Reminders and Check-ins keep going when the person doesn't open Duet.

Today and chat:

7. The person picks a Today item and discusses it in chat, and the conversation carries the item's version and evidence window.
8. They accept a change. A receipt appears in chat only once the save is confirmed, and the saved item then shows in Today, linked to the conversation it came from. Unsent drafts and newer edits survive this.

Plans, Reminders and Check-ins:

9. The person asks for a Plan, Reminder or Check-in, or accepts one Duet proposed. A goal on its own never creates a schedule. The person's own request is the approval, with no extra Start tap, and text Duet retrieved never authorizes anything they didn't ask for.
10. The saved item appears in Today as current care.
11. When it's due, it posts as a message in the thread, after any reply Duet is in the middle of. It alerts wherever alerts are allowed, and the lock-screen text stays generic. Opening the notification goes straight to the message, after Duet checks it's the right account.
12. A Reminder posts the person's words, and Duet may add to them. A Check-in does its work with current context and may adjust its questions, timing, pausing and completion within the limits the person accepted, and meaningful changes are shown.
13. The person can edit, pause, resume or cancel any of them. A Plan deleted while open stays on screen and says it was removed.
14. **A Check-in can't finish.** Duet posts a fixed line in the thread with Try again, and the Check-in on Today shows it couldn't finish. If it keeps failing, Duet tells the person and judges whether to keep trying, pause it or ask them; a pause Duet chose stays distinct from one the person chose. A failed Check-in whose next time arrives while AI isn't allowed is retired.
15. **Interruption or restart.** An accepted Reminder or Check-in still runs exactly once.
16. **Newer Terms or AI not allowed.** AI work waits without being used up. Reminder and Check-in times that pass are skipped, and the next one delivers once AI is allowed again.
17. **Outage or backlog.** Duet acknowledges what it missed, offers whatever catch-up it judges useful, and never reports missed work as done.
18. **New information contradicts a Plan.** Duet prepares a correction for the person to review. Only follow-up that depends on the contradicted instruction is held.
19. **A Plan is removed.** Its accepted Reminders and Check-ins keep running. There's no dialog, and Duet follows up in the thread when it judges that useful.

Alerts:

20. Each Duet message alerts once, like a text, and a review puts everything it noticed in one message so it buzzes once.
21. Replies, replies that couldn't finish, Reminders, Check-ins, messages from Duet's own work and practice news alert when the person is away, with a generic lock-screen line, and open at their message in the thread. The phone's notification permission is the only switch, and Duet judges what's worth an alert.
22. Duet keeps no quiet hours of its own; the phone's Focus or Do Not Disturb decides.
23. **Shared phone.** Notifications follow whoever is signed in. Signing out removes that phone from the account's notifications and clears its pending ones. Two phones on one account each get notifications.

### D4. The For my doctor workspace

1. The person opens their For my doctor workspace from Today.
2. Things get added three ways: the person asks Duet to prepare or revise it; Duet proposes an update when it judges one useful, as a message with Save and Dismiss (replying "yes, add it" works too, and the workspace shows the same proposal); or the person adds something from You or through chat.
3. Duet picks the relevant evidence and drafts a concise, factual account with sources the person can inspect.
4. The person reviews the proposed update, sees what changed, edits it and chooses what to save. Nothing is saved without that review. If they save an edit while work is running, its proposal arrives marked "review again" against the latest version.
5. They add appointment context from their own information or a connected practice's schedule.
6. They save, and after an interruption or restart they reopen the same content.
7. **Sending to the practice.** While connected, the patient sends a saved version when they choose, with one Send to my practice that names the practice, and any question for the practice written in it. It reaches the practice once, as the version they sent, like a message from a known patient. Saving alone shares nothing, and what was sent keeps its version, marked when a newer one exists. On the practice's side, the patient's agent hands it to the practice's care team agent, and the practice's agents judge whether a physician needs to see it. Anything that comes back to the patient as a message has the physician's approval.
8. **Leaving mid-preparation.** A preparation the person asked for finishes in the background. Its result is set aside if its inputs or consent changed, and a failure needs an explicit retry.
9. **Conflicting appointment dates.** Both dates stay visible and preparation continues, and Duet judges when to clarify. If the date changes, the preparation is kept. A suggested follow-up is never shown as a booked appointment. When a connection ends, the practice's appointment goes away and the patient's own date stays; rejoining brings back any visit the practice's schedule still has ahead.
10. **Corrections and newer saves.** The workspace always shows and sends each record's current version, says when a record was corrected, and never blocks Send for it. Newer saves appear in any open editor, the person's unsaved words are kept, and only the same words changed in two places ask them to choose.
11. **AI not allowed.** The person can still view, add and edit; only AI preparation is unavailable.
12. **Attribution.** Documented care instructions stay attributed to their clinician and source. Duet's own writing is never labeled as physician-reviewed, and there's no physician approval step.
13. **Improving the writing.** Clinical advisors, then physicians at pilot practices, review representative summaries to improve Duet's writing. This creates no approval step.
14. There's no PDF download.

### D5. Export, deletion and sign-out

1. **Export.** The person exports everything Duet keeps about them, every kind of data with its source and date. Export never waits on a refresh or on newer Terms; it packages what Duet holds now and says what's still importing.
2. **Deletion.** The person deletes their account, which removes everything and stops all practice sharing.
3. **Interrupted deletion.** The deletion finishes, and neither old work nor a restored backup brings deleted or withdrawn data back. A deletion that didn't finish is offered to the person to finish.
4. **Sign out.** There are two scopes, this device and everywhere. Signing out everywhere ends every session while the account and its history stay, and Duet never says "signed out everywhere" until every session has ended. With unsent notes or medications, Duet tries to sync first, and if it can't, says how many entries would be lost and offers Stay signed in or Sign out anyway.
5. **Withdrawing AI isn't deletion.** The conversation and Duet's understanding of the person stay viewable, exportable and deletable.
6. **Rights never wait on Terms.** While newer Terms wait, Settings still open for export, disconnecting a source, leaving a practice, deleting the account and withdrawing AI. Everything else waits.

### Chat behavior

1. The person sends a message, it's kept safely and the composer is ready again. If sending definitely failed, they can edit it, and a retry keeps the same message.
2. **Steering.** A new message while Duet is still writing steers the reply in progress, and the reply answers both. It's never refused or queued, and there's no Stop button.
3. **New information mid-reply.** Duet takes in records and readings that arrive while it writes a reply or runs a Check-in, without asking whether to continue.
4. **Replying to a message.** The person swipes to reply with a quote, and Duet can do the same. There are no reactions beyond feedback.
5. **Feedback.** Long-press on a reply gives thumbs up, thumbs down, Copy and Select Text, and quiet thumbs show under the latest reply. Thumbs down takes an optional note, and tapping the chosen thumb again removes the feedback. Feedback stays in the person's account and is counted in metrics; sharing a reply with Coda comes after launch, under its own privacy terms.
6. **Attachments on an unsent message.** If an attached item is corrected, the message carries the corrected version with a small note. If it's deleted, it drops off with a quiet note. Once sent, the message keeps the version it carried.
7. **Notes and medications in conversation.** When asked, Duet saves or changes a note or medication right away, and its message shows exactly what it saved, with Undo. A note Duet drafts from something the person mentioned is a proposal to save or dismiss.
8. **Symptom questions.** Duet asks about what the person is experiencing so they can understand and describe it. It never narrows to a personal diagnosis, ranks the likely cause, judges urgency or says waiting is safe. Source-attributed general information, including published warning signs, stays available.
9. **Medication questions.** Duet explains documented instructions as written and helps the person confirm with their prescriber. It never suggests starting, stopping or changing a dose.
10. **Emergency or self-harm.** Duet posts its fixed safety line, 911 or 988, with its exact conditions and wording, as its own message. Duet may keep helping afterward but never replaces the line with its own assessment, and the same situation, paraphrased, downplayed or with records present, still triggers it.
11. The standing notice stays reachable from the conversation.

### AI permission and Terms

1. At sign-up, Terms come first and are accepted explicitly, and AI permission is its own choice after them.
2. No model call happens without accepted Terms, AI permission and the operator switch.
3. **Withdrawing permission.** The attempt in progress ends. Saved Plans, Reminders and Check-ins stay, and scheduled times that pass while AI isn't allowed are skipped. The chat shows the real reason and keeps drafts.
4. **Allowing it again.** Accepted Check-ins resume, and interrupted work restarts with current context.
5. **Newer Terms, or a new version of the AI permission.** AI work waits without being used up until the person accepts, and declining a new AI-permission version means AI isn't allowed. A build that can't accept the current Terms shows Update Duet.
6. Duet never hides that AI services process the person's information.
7. Product improvement and research have their own consent, off by default and withdrawable.

## Rules

Access and identity:

- Each person has one account, whatever the sign-in method, and joining a practice links to that account.
- Each request sees only its own person's data.
- There's no guest, anonymous or local-only mode.
- Duet is for adults 18 and over in the U.S.
- Patient-facing sources and connections stay separate from provider-facing ones.
- Duet has no doctor accounts; physicians sign in only through Coda Agents.

What leaves Duet:

- Nothing about the person leaves Duet without their explicit choice, and Duet's private notes never leave.
- A note from conversation reaches a practice only after the person saves it.
- Lock-screen text is generic, with no health content. Health content opens inside Duet.
- The app covers its screen whenever it's inactive, so the app switcher never shows health content, and a background launch on a locked iPhone never erases data.
- Duet never writes back to the person's sources.
- Continuous sensor data stays on the phone, but each reading the person takes, such as a blood-pressure reading, reaches the server as its own measurement. Every calculation that doesn't need raw samples runs on the server.
- Duet uses the same clinical library as Coda Agents, including outside clinical AI under a BAA, and the person's records. It uses no web search.
- Duet has no advertising, no cross-app tracking, no data broker and no App Tracking Transparency prompt, and it keeps the standard disclosures.
- Any use of data beyond the service is separately consented and de-identified.

Consent and data:

- Duet fails closed on access, consent and patient data, and fails gracefully on availability and single bad items. Showing the signed-in person's own saved content while Duet checks the account counts as availability.
- Past versions of Duet's memory are kept for 30 days, then deleted, and go with the account.
- Withdrawing AI permission ends the attempt in progress. Newer Terms or a new AI-permission version pause AI work without using it up.
- Content Duet retrieves can never grant permission for anything new.
- An accepted recurring Check-in runs only within its agreed scope.
- Deletion removes everything, stops all practice sharing, and survives interruption and backup restore.
- An imported original is never silently replaced. Values stay exactly as sent; a unit is converted only by an exact defined factor, with the original beside it.
- Records are never quietly dropped for not fitting a category, and a shortened list always says so.

Safety:

- The emergency (911) and self-harm (988) lines keep their exact conditions and wording, and any change is tested first against present, recently resolved, past and quoted symptoms, including real emergencies.
- Duet doesn't diagnose, prescribe, assess personal risk or urgency, or decide care.
- Insights are observations only.
- Duet never shows its internal machinery or model names; patient copy says "AI".
- A lab's printed reference range doesn't appear on For my doctor, and in chat Duet quotes it only when asked.

Authority:

- Code does the arithmetic. The model picks among the supported calculation types and any valid period, including one anchored to an event, and never writes new formulas.
- Code never stands in for judgment: the model owns clinical reasoning, and code owns guarantees.
- Physicians don't approve Duet's wording or manage how patients use Duet.
- Duet saves or changes follow-through only from the person's own request or an accepted proposal.
- A failed reply can't erase or repeat an action already committed, and a successful reply alone never proves that a change, a read or an outside effect happened.
- Saving, failure and reopening always show the actual state.

## Scope

In the MVP:

- Duet on iPhone and Android, running on coda-rust, connected through Connected care to one or more practices.
- Sign-up, the standing notice, Terms, AI permission, preferences, export, deletion and sign-out.
- One continuous conversation with citations, corrections and reply feedback.
- Portal records (Epic/MyChart, Oracle Health, healow, and athena once its route works), Apple HealthKit and Health Connect.
- Notes and medications the person enters.
- Today with Duet's own work, insights, Plans, Reminders, Check-ins, the doctor's plan, and push.
- The For my doctor workspace, with the patient's own and the connected practice's appointment context, sent to a connected practice by the patient.
- Clinical and regulatory qualification of the patient-facing functions before real patients.
- Everything free for everyone, with no paid tier or feature gating.

Not in the MVP, and absent from the app rather than shown as coming soon:

- Person-to-person sharing.
- Migrating existing Duet accounts and data.
- Patient-facing clinician lookup, saving a care team and release of information.
- A Duet web surface.
- PDF download.
- An activity, reasoning or tool-trace UI.
- Caregiver access.
- Manual lab, measurement and demographics entry.
- Technical record inspectors.
- A universal medical record.
- Genome data.
- Recording doctor visits.
- A role for MedDecider; the decisions-API idea waits with NPI matching.

Later:

- Sharing For my doctor with a physician outside Coda: the patient publishes reviewed material as a page, sees exactly what the doctor will get, shares a link with a separate passcode and a visible, editable expiry, and can revoke it. A saved update never changes a snapshot already shared.
- Clinician lookup against the NPI registry, then release of information, with a confident match and a clarifying question at its core. Nothing is saved, shared or sent to a clinician the patient hasn't confirmed.
- Document upload.
- A model organizing records in You.
- Google Calendar and Outlook as appointment context.
- CGM, after ingestion and daily portal refresh are solid.
- More wearables and patient-access sources, such as Oura, Whoop, Fitbit, Withings, Garmin, the VA and Medicare.
- Helping people navigate the health system with computer and browser use.
- Photo, PDF and voice input.
- A search index covered by a BAA, as the way past the clinical library.
- The jointly maintained shared space between patient and practice.
- A patient-state foundation model trained on data collected over time.

## Engineering decisions

These are how we build Duet. They shape its behavior but are engineering's to change.

**Model.** Duet's AI is one model whose effort depends on the work; background runs will probably run at max effort.

**Chat.** The chat should feel like texting, so we adopt a strong open-source chat implementation instead of building one from scratch.

**Schedules and budgets.** Duet's own work starts with a daily look, a weekly recap and a review when new records arrive. How often it runs, whether a new reading wakes it or waits for the next look, what it costs per person, and how and how often it retries are engineering decisions, as are capacity targets and any daily message limit. At a message limit, the composer shows one quiet line and keeps the draft. The starting capacity targets are 20 simultaneous chats, a reply starting within 30 seconds, Check-ins within 10 minutes and about 1,000 active people a day; they're targets, not measurements. Any backstop on one reply's tool calls is a runaway guard, never a quota on how the model investigates, and when ordinary questions hit it the fix is better tools.

**Runs.**

- When context fills, Duet keeps structural checkpoints of each request rather than a model-written note.
- A sent message is durable once accepted and is never lost to a model failure, the app exiting or a restart.
- A message that arrives while a run is active steers that run. Several runs may be active for one account at once, and Duet's own work doesn't wait for the person to stop chatting.
- A push only wakes work, and the work always pulls current truth.
- Anything background work has for the person goes through the conversation.
- An interrupted run resumes as the same run: streamed text is kept, and a committed action is reconciled, never duplicated. An interruption the person causes, including their own new message, never uses up an attempt.
- A permission change takes effect immediately: withdrawal ends the run in progress, and a pause holds work without using it up.
- A correction fences any unfinished work that read the corrected input, so nothing stale is published.
- A scheduled Reminder or Check-in fires exactly once, honors pause and cancel, and is skipped if its time passes while it can't deliver. Retry windows count only time Duet was allowed to work.
- Imports resume without duplicates and end in a stated outcome, and engineering can see which connection is overdue and why.
- Deletion and sign-out everywhere complete before they're reported done.
- Each run leaves a record the team can inspect. Run steps come only from real events and never reach Duet's UI, Today or notifications.

**Proof.** We prove Duet by running the jobs it must do as simulations across patient types and scenarios, local and synthetic first, with production traces added over time. A simulation never scripts the model's conclusions.
