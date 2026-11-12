# Duet

This is what we're building for patients: what Duet is, why it exists, and how it should behave. It describes the product as we intend it today. Git keeps the history.

The MVP has one job: a patient of a Coda practice gets their records and their own words in front of their doctor, with almost no effort after setup. It is accounts, connected records and readings, and For my doctor in Shared, where the patient marks records, writes notes and sends them to a connected practice. General patient conversation, proactive care and AI preparation of For my doctor are the direction this spec describes, and the MVP is the first step toward it. Scope says what exists now and what is parked.

## What Duet is

Duet is a person's own health agent, on iPhone and Android. Think of the personal agents people already use, like Instinct, Muse, Hermes or OpenClaw, managed and maintained for them and focused on their health. The conversation is the product. It helps the person understand their health, use their connected history, remember and correct what matters, follow through, notice changes and get ready for care, without ever having to retell their story.

Duet is free, it stands on its own and it needs no doctor. Joining a practice is never required, and when a person's practice uses Coda, Duet connects to it through Connected care. Records plus AI are becoming free, so Duet doesn't compete on records. It competes on the person's connection to their own doctor.

Duet isn't a diagnostic service, a clinical alarm, a treatment authority or a replacement medical record. It explains health information and documented care faithfully, and it leaves care decisions to the person and their clinicians.

## The point

A person should feel understood by something that actually knows their health, does useful work for them without being asked, and makes the time with their doctor better. Everything Duet shows outside the conversation (Today, Plans, Reminders, Check-ins, You and For my doctor) exists so the things that matter live beyond the chat. Those surfaces are views of what Duet and the person keep, written by Duet's tool calls. The person changes them by telling Duet, or with a direct control as a shortcut, and Duet handles the consequences with judgment instead of dialogs.

## Principles

Coda's shared principles (README.md) govern this spec. These are the ones specific to Duet.

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
14. **Plain, calm and smooth.** Duet follows the shared UI principles (README.md U1–U9). It shows no internal or technical language and few controls. Work happens on the server, so the app stays responsive and new information arrives smoothly. Duet follows the phone's light or dark appearance unless the person chooses one.

## The concepts

Concepts for what comes after the MVP say what the thing is. Their rules are not decided yet.

**Duet's AI.** The single AI inside Duet, named "Duet's AI" to the person. It's a warm, capable health agent that learns about the person, helps them understand and act, and notices what changes. It's for the person's health only, not a general-purpose assistant. It is candid and willing to investigate and give a considered view of the evidence, the options and what to ask a clinician, and it never makes a personal diagnosis, urgency or treatment call.

**The conversation.** One continuous thread per person that never ends and survives interruption, reconnecting and restarts. It's where the person asks, explains, corrects and works things out.

**Today.** The person's care home. It pins what is current and stays useful on quiet days.

**Insight.** An observation Duet's own work wrote from the person's evidence, about what's worth noticing right now. It stays an observation: no diagnosis, risk, urgency or treatment.

**Duet's own work.** Work Duet does without being asked. It looks at the person's data on a schedule, and beyond that it may take on anything it judges worth doing, such as following a trend or connecting the doctor's plan to readings.

**Re-engagement.** When the person goes quiet, Duet reaches out only for a real reason in their life: new records or results, an upcoming appointment, a trend in their data, news from their practice, or something they were working on. It never sends a generic "come back" message.

**Plan.** A saved routine with no schedule of its own.

**Reminder.** A follow-up the person asked for.

**Check-in.** Later work the person explicitly asked Duet to do, or a proposal they accepted.

**Medications.** The person's own entries in You. A medication they stop stays in their list, marked stopped with the date, and leaves "What I take". While connected, these entries reach the practice.

**Patient memory.** Duet's private, correctable understanding of the person: their preferences, goals, constraints and concerns. It never leaves Duet.

**You.** The person's library of records, measurements, medications and visits, with exact values, dates and sources. It's a faithful data dump: no cleaning, curating or model sorting, laid out by each source's own structure so it never reads as confusing. Tapping a lab shows its trend as a line graph; each line comes from one whole source series and is never spliced, and a test from more than one lab shows one lab's line at a time with a switcher. A lab's printed range band and flag tag don't appear. Tapping a measurement shows its readings over time the same way, from one source series. Blood pressure shows both numbers together, and sleep shows each night. The person can search everything in You. Connecting a source starts here, under Sources & data, near the top, and those controls are always easy to find. The person can edit their display name but not their legal name.

**Source and citation.** A source is a connected patient portal, Apple Health or Health Connect, and the imported original always stays inspectable. A citation reopens exactly the evidence the AI received.

**For my doctor.** What the patient wants their doctor to notice: the records they marked and the notes they wrote. It lives in Shared and works with or without a connected practice. It is private, and nothing in it reaches a practice until the patient sends it. Physicians don't approve its wording or manage it.

**Mark.** The patient marks anything they can open in You: a record, one value, a reading or a trend. A mark is on or off. Marking changes nothing in the record and shares nothing.

**Note.** The patient's own words in For my doctor, as free text with no prompt, categories or required form. A note can point at a record. A note is private until the patient sends it.

**Send.** While connected, the patient sends marked records and notes to one named practice. A send is fixed once made and can't be taken back. It clears the marks it carried, and the patient can mark and send the same record again later. Each send shows Sent, then Received by the care team once the practice's agents have picked it up. Duet never says a physician saw it.

**Appointment context.** Dates and details the patient provides or a connected practice's schedule supplies, attached to For my doctor.

**Your doctor's plan.** The documented care decisions and open items a connected practice sends back, with source, author, date and status. It's read only, lives on Today, and Duet explains it faithfully.

**Shared.** The tab that holds For my doctor and the practice connection. It shows what is marked and what was sent. It works without a patient-side conversational agent. In the MVP it has no person-to-person People list and no publishing to doctors outside Coda.

**Connected care.** An invited practice's consented connection to the person's Duet account. Patients join a practice only through that practice's invitation and their consent, and joining links to the account they already have. A person may connect to more than one practice, each connection separate. Every Duet flow works without a connected practice.

**Standing notice.** The fixed statement of what Duet's AI is and its limits: "Duet's AI isn't a doctor. It explains health information and doesn't diagnose or treat. In an emergency, call 911." It arrives with Duet's AI. Duet shows it at sign-up and keeps it within reach in settings and in chat, without repeating it in every reply.

**Companion.** A small face the person picks during setup. It's Duet's face, not a second assistant, and it changes how Duet looks, not what it can do.

## Journeys

### D1. Sign up and stay signed in

After sign-up, the MVP opens on You, with Shared beside it.

1. The person installs Duet from the App Store on iPhone or the Play Store on Android.
2. They sign up on their own, with no invitation, by email on iPhone and Android: Duet emails a link that opens the app, with a short code as the fallback. A practice invitation arrives by email, as [Connected care](connected-care.md) describes. Duet has no web surface; an invitation link opens the app.
3. They accept the Terms with one explicit "I agree and continue", with no prechecked box. Accepting them also confirms the person is 18 or over and in the U.S. A quiet Sign out sits under this step for a wrong account or declined Terms.
4. An optional last step offers to connect their health, with every available connector on one panel and Not now.
5. **App restart.** The person is still signed in, and Duet reopens where they were, with any draft they left.
6. **Revoked session, or another account.** Either is refused, including after a restart. A phone whose session ended elsewhere leaves the account, and the sign-in screen says what it removed that Duet hadn't confirmed. A phone that can't reach Duet learns this only once it can, and keeps showing the account until then, behind the app's screen cover and the phone's own passcode.
7. **Duet unreachable.** After a short wait, "Connecting…" takes the place of the screen title, as in Messages, so nothing is pushed down or covered. The person's saved content and drafts stay usable while Duet checks the account.

Signing in, staying signed in and signing out of a phone, with their model, live in [sign-in](../verification/duet/sign-in/README.md).

### D2. Records come in and stay faithful

1. The person opens Sources & data in You, or the setup panel, and searches "Doctors & hospitals" by name, and Duet works out which portal vendor it is. The list shows exactly the portals that are switched on: Epic/MyChart and Oracle Health.
2. They can connect several organizations, including several MyChart organizations. Each one connects, refreshes, fails and disconnects on its own.
3. They let Duet read Apple Health on iPhone or Health Connect on Android. Wearables linked to Apple Health come through HealthKit, with one instruction to turn that on in the wearable's app.
4. Duet imports all the available history and never calls an import complete when it isn't. One unreadable record fails alone, with its reason, and the rest publish. A part that never becomes readable ends the import as partial, with the rest usable and the next daily refresh trying again.
5. The person browses You. Records show as the source sent them, including entered-in-error or cancelled, and Duet explains the status. Only exact duplicates are removed; a likely duplicate from two sources shows in the review sheet as "looks like the same result as …", and the person keeps one or both. A condition shown as current isn't also shown as past.
6. **Correction at the source.** A correction or deletion at the source carries through to You. Records from a source change only at the source. The person cannot edit or delete them in You.
7. **Daily refresh.** Each portal refreshes daily, and one portal failing doesn't block the others.
8. **A connection stops working.** The source in You shows "Couldn't update since <day>" with Reconnect. There's no separate alert.
9. **Missing on a fetch.** A record missing from one fetch, or an empty or failed read, is never treated as a deletion.
10. **Deleted in Apple Health.** Duet recalculates that day's total and drops it from current evidence if no readings remain. Earlier versions stay for sends that already carry them.
11. **Disconnect and reconnect.** The person can always disconnect, even while newer Terms wait. Disconnecting stops new imports and keeps the imported history. Reconnecting the same source carries on with that history.
12. **A vendor is switched off.** It leaves search and blocks new sign-ins only. Existing connections stay connected and refreshing, and disconnect always works.
13. **Restart while uploading.** Phone uploads not yet sent survive the restart.
14. **Health Connect.** Duet asks for every Health Connect data type Google approves, the person grants each in Google's permission sheet, and Duet judges what's useful. Unsupported data and background limits are stated to the person.

### D4. For my doctor

In the MVP, For my doctor is in Shared. The states, steps and rules of marks, notes and sends live in [shared](../verification/duet/shared/README.md).

1. The person opens anything in You and marks it for their doctor. Marking is one tap and needs no writing.
2. The person writes notes in their own words, as many as they want. A note needs no record. A note written from a record points at that record, shows the record's own label, and takes the record with it when sent.
3. Shared lists what is marked and not yet sent. The person can unmark a record and edit or delete a note. A note edited on another phone shows its newer version and keeps the person's unsaved words. A failed or unfinished save never shows as complete.
4. All of this works without a practice or an appointment, and none of it is shared.
5. While connected, Send names the practice and sends the marked records and notes the person chooses. One send can hold several. From a record, the person can also send that one record straight away. Without a live connection there is no Send. With more than one connected practice, the person picks one.
6. Before sending, Duet says that a send can't be taken back, and that it isn't for urgent needs or for questions that need an answer.
7. A send clears the marks it carried and takes the notes it carried out of the list. The person can mark the same record again later and send it again.
8. Shared lists each send with its date and status: Sent, then Received by the care team once the practice's agents have picked it up. Sends show as a list, never as a chat thread, so nothing suggests a reply is coming. The person can open a send and see exactly what went. No status claims that a physician looked at it. Nothing else comes back into Duet from a send in the MVP.
9. The practice's agents review a send like any shared record and judge whether a physician needs to see it.
10. Joining a practice sends nothing from For my doctor and asks nothing about it. What was marked stays marked, and Send becomes available.
11. Each record shows and sends its current version and says when its source corrected it. A source correction never blocks sending and never changes an earlier send. A failed or uncertain send says so honestly, and a retry never sends twice.
12. Ending the connection stops anything new from crossing. What the practice already received keeps where it came from and when. There is no PDF download.

### D5. Export, deletion and sign-out

1. **Export.** The person asks Coda for everything Duet keeps about them, and Coda sends it, every kind of data with its source and date. Export never waits on a refresh or on newer Terms.
2. **Deletion.** The person deletes their account, which removes everything and stops all practice sharing. The same email can sign up again later and gets a new, empty account.
3. **Interrupted deletion.** The deletion finishes, and neither old work nor a restored backup brings deleted or withdrawn data back. A deletion that didn't finish is offered to the person to finish.
4. **Sign out.** There are two scopes, this device and everywhere. Signing out everywhere ends every session while the account and its history stay, and Duet never says "signed out everywhere" until every session has ended. With unsent notes or medications, Duet tries to sync first, and if it can't confirm them, says how many entries it hasn't confirmed and that signing out removes them from this phone, and offers Stay signed in or Sign out anyway. Signing out of this device takes effect at once, even offline, and Duet hears of it as soon as the phone can reach it.
5. **Withdrawing AI isn't deletion.** The conversation and Duet's understanding of the person stay viewable, exportable and deletable.
6. **Rights never wait on Terms.** While newer Terms wait, Settings still open for disconnecting a source, leaving a practice and deleting the account. Everything else waits.

### AI permission and Terms

The MVP asks for Terms only. AI permission, its versions and the operator switch arrive with the first AI feature, and the rules below then apply.

1. At sign-up, Terms come first and are accepted explicitly, and AI permission is its own choice after them.
2. No model call happens without accepted Terms, AI permission and the operator switch.
3. **Newer Terms, or a new version of the AI permission.** AI work waits without being used up until the person accepts, and declining a new AI-permission version means AI isn't allowed. A build that can't accept the current Terms shows Update Duet.
4. Duet never hides that AI services process the person's information.
5. Product improvement and research have their own consent, off by default and withdrawable.

## Rules

Access and identity:

- Each person has one account, tied to their verified email, and joining a practice links to that account.
- Sessions don't expire with time. A session ends when the person signs out of that phone, signs out everywhere, or Duet revokes it.
- A signed-in person can change their email, and Duet confirms the new address with a link. Someone signed out who can no longer reach their email gets back in through support.
- Each request sees only its own person's data.
- There's no guest, anonymous or local-only mode.
- Duet is for adults 18 and over in the U.S.
- Patient-facing sources and connections stay separate from provider-facing ones.
- Duet has no doctor accounts; physicians sign in only through Chords.

What leaves Duet:

- Nothing about the person leaves Duet without their explicit choice, and Duet's private notes never leave.
- Lock-screen text is generic, with no health content. Health content opens inside Duet.
- Notifications follow whoever is signed in. Signing out removes that phone from the account's notifications and clears its pending ones. Two phones on one account each get notifications.
- The app covers its screen whenever it's inactive, so the app switcher never shows health content, and a background launch on a locked iPhone never erases data.
- Duet never writes back to the person's sources.
- Continuous sensor data stays on the phone, but each reading the person takes, such as a blood-pressure reading, reaches the server as its own measurement. Every calculation that doesn't need raw samples runs on the server.
- Duet uses the same clinical library as Chords, including outside clinical AI under a BAA, and the person's records. It uses no web search.
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

**Now.** What exists or is in flight:

- Duet on iPhone and Android, running on coda-rust, connected through Connected care to one or more practices.
- Sign-up by email, Terms, display name and appearance, deletion, and sign-out on this phone and everywhere.
- Portal records (Epic/MyChart and Oracle Health), Apple HealthKit and Health Connect.
- You, record search, original record details and measurement views with their values, units, dates, sources and corrections.
- For my doctor in Shared: marked records and notes, kept private until sent, and sends to a named connected practice with a Sent and Received status.
- Account and records use on their own, with no practice. Shared then says no practice is connected, and nothing else is designed for that person yet.

How it is built today:

- Every AI job runs on the [harness](harness.md), which owns how runs start, recover, manage context and end.
- Imports resume without duplicates and end in a stated outcome, and engineering can see which connection is overdue and why.
- Deletion and sign-out everywhere complete before they're reported done.

**Not in Duet.** These are absent from the app rather than shown as coming soon:

- Person-to-person sharing.
- Migrating existing Duet accounts and data.
- Patient-facing clinician lookup, saving a care team and release of information.
- A Duet web surface.
- PDF download.
- An activity, reasoning or tool-trace UI.
- Caregiver access.
- Manual lab, measurement and demographics entry.
- The person editing or deleting records that came from a source.
- Technical record inspectors.
- A universal medical record.
- Genome data.
- Recording doctor visits.
- A role for MedDecider; the decisions-API idea waits with NPI matching.

**Parked.** The direction is set above. Nobody is working on these, and we spec each one when we pick it up:

- Conversation with Duet's AI, with citations, corrections, reply feedback and private memory.
- AI permission, the standing notice and the companion, with the first AI feature. Whether the person can add or edit notes and medications in You while AI isn't allowed is undecided.
- Export in the app.
- healow, and athena once its route works.
- The push registration fix from the security review, with notifications.
- Today, insights, Plans, Reminders, Check-ins, re-engagement and their notifications.
- AI help preparing For my doctor, and appointment context.
- The doctor's plan in Duet, with its explanation.
- Sharing For my doctor with a physician outside Coda.
- Clinician lookup and release of information.
- Document upload.
- A model organizing records in You.
- Google Calendar and Outlook as appointment context.
- Calendar and email connections in Duet.
- Reference ranges on For my doctor, once their sourcing is better.
- CGM, after ingestion and daily portal refresh are solid.
- More wearables and patient-access sources, such as Oura, Whoop, Fitbit, Withings, Garmin, the VA and Medicare.
- Helping people navigate the health system with computer and browser use. The leaning is Duet's own browser loop first, with native computer use as the fallback.
- Photo, PDF and voice input.
- A search index covered by a BAA, as the way past the clinical library.
- The jointly maintained shared space between patient and practice.
- A patient-state foundation model trained on data collected over time.
