# Patient messages

This folder is the spec for how a patient's message reaches the right patient in Coda Agents, and how its reply card works until the practice has answered. The model is the agreed behavior, TLC checks that it keeps its rules, and the code follows it. [physician.md P3](../../../intent/physician.md#p3-a-patient-message-is-linked-a-reply-is-proposed-revised-and-handed-off) owns the intent.

## What it is for

Patients write to the practice in its messaging channel. Coda adds no inbox and never sits in the channel's path. It reads what arrives, links it to one patient, brings it into that patient's review, and drafts a reply only where one would help. The physician sends in the channel, and Coda knows the work is done when it sees that reply. Acknowledgments and resolved conversations never reach the physician.

The spec holds for any channel the practice connects. Spruce is the channel in the MVP. What differs between channels is configuration, and the rules don't change with it:

| Channel setting | Spruce | In the model |
| --- | --- | --- |
| Who can reply in a conversation | Every physician with access to the patient | `Repliers` |
| Which conversations route without a person | One whose one outside participant is saved as the contact of exactly one patient, linked to exactly one EHR record that matches exactly one Coda patient | `ChannelIdentifies` |
| Who corrects a confirmed link | The messaging manager, the person who set up the connection | `Manager` |

## Who owns what

Each fact has one owner, and everything else reads it.

| Owner | Holds | Never |
| --- | --- | --- |
| The channel | The conversation and everything sent in it | Changed by Coda: Coda reads it and never sends |
| Intake | Which of the channel's items Coda has filed, in order | Shown to the physician while catching up |
| The link | Which patient a conversation belongs to | Set without a person where the channel can't name exactly one patient |
| The patient's review | Whether a reply would help, and the draft | Run for a patient the conversation isn't linked to |
| The reply card | One card for the conversation's unanswered messages: Coda's draft or the physician's edit | Given Coda's text over the physician's unless they choose it |
| The patient's history | What the practice actually sent | Filled from the draft |
| Today | Nothing of its own: it is read from the link, the card, the reviews and the channel, through each physician's current access | Stored as a separate list |

## Flows

Arriving and matching:

- **F1.** The patient writes in the channel.
- **F2.** Coda files a message in a linked conversation, and it routes on its own.
- **F3.** Coda files a message the channel says is exactly one patient's, and it routes on its own.
- **F4.** Coda files a message with no sure match, and it waits on the practice-wide Messages to match list and the messaging manager's Today.
- **F5.** Any physician confirms Coda's suggestion or picks another patient. The conversation is linked, and its waiting messages join that patient.
- **F6.** A reply the practice sends before the conversation is linked waits with the conversation. Once it is linked, the reply joins the patient's history and the messages it answered are settled.
- **F7.** A physician leaves a waiting message unlinked. That covers only that message; the conversation's next message waits again.

Reviewing:

- **F8.** The patient's review reads the whole linked conversation, including the practice's own replies, and reads again when something new arrives before it finishes.
- **F9.** The review drafts a reply, and one card opens, shared by everyone who may access the patient and can reply in the conversation.
- **F10.** The review decides no reply would help, and the message settles with no row.

The card:

- **F11.** A physician opens the reply from Today or the patient page.
- **F12.** A physician edits the reply, and the edit shows to everyone who sees the card.
- **F13.** A physician asks Coda to revise, which lets Coda replace their text. The revision finishes on the conversation as it stands then.
- **F14.** Editing during a revision stops it, and the physician's text stands.
- **F15.** When the patient writes again, the card says so at once, and an untouched draft then updates in place.
- **F16.** When the patient writes again after an edit, the physician's text stays and Coda's update is offered. The physician can take it.

Handing off:

- **F17.** A physician copies the reply. The row stays.
- **F18.** A physician sends in the channel: the copied draft, their edit, or their own words.
- **F19.** Coda sees the reply. The patient's history records the words actually sent, the messages before it settle, the card closes for everyone, and the review looks for anything left.
- **F20.** A physician marks No reply needed, and the card closes for everyone.
- **F21.** A message Coda can't read waits on Today with a link to the channel until someone marks it Done.

Wrong patient:

- **F22.** The messaging manager moves the whole conversation to the right patient. Later messages route there. The old patient's review lets go of it at once, history moves with it, and the card moves, withheld. Anyone who may not see the new patient loses the card.
- **F23.** The new patient's review redrafts the withheld reply, keeps the physician's edit and offers its update, or closes it.

Disconnecting:

- **F24.** The practice disconnects the channel. Reply drafts are deleted, and an open card stays empty and says the channel is disconnected. A review that finishes meanwhile opens its card empty.
- **F25.** The empty card waits until someone marks it Done, or until Coda, reconnected, sees the reply.

## Rules

The model checks these in every state it reaches.

| Rule | In the model |
| --- | --- |
| A physician has a reply open only while they may access its patient and can reply in its conversation. | `OnlyWithAccess` |
| A conversation reaches only the patient it is linked to: no other patient's review holds it, and its card and the practice's replies show only on that patient. | `RightPatient` |
| Until a person or Coda settles it, every patient message Coda filed shows to someone who can act: waiting for a match, or, to a physician with access, unreadable, being reviewed, on a card they can open, or behind the disconnected channel's line. | `NeverInvisible` |
| Coda never replaces the physician's edited reply unless they choose its update. | `YourWords` |
| A card's draft never misses something filed after it without saying the patient wrote again. | `CurrentDraft` |
| A message, a sent reply or a correction starts at most one review. | `OneReview` |
| What the patient's history says the practice sent is what the channel shows it sent. | `WhatWasSent` |

These rest on stored records or the channel, which the model can't hold. No test holds them yet:

- Intake recognizes an item delivered twice as one item, and saves a page before its cursor moves.
- Each practice's channel connection has one intake lease, so no item is filed twice or lost.
- Starting a review exactly once for its event survives restarts and retries.
- The patient's words are never changed, only formatted, with unsafe content stripped.
- Undo restores the reply exactly.
- A message moved off the wrong patient is marked entered in error and never deleted, and what Coda wrote from it there is marked and can't surface.
- A completed disconnect deletes messages never accepted into a record, and reply drafts; accepted evidence stays under the record's policy.

The model leaves out whether mail is from a patient at all, Coda's suggested patients, urgent unmatched messages, Ask Coda to reply, Undo, Flagged concerns, attachments, and access that changes other than through a correction.

## Files

| File | Holds |
| --- | --- |
| [Messaging.tla](Messaging.tla), [Messaging.cfg](Messaging.cfg) | The design as a TLA+ model, with the rules as `INVARIANT`s. Every step names the flow it carries out. The config checks one conversation with two patients, two physicians (one who may see only the first patient), two patient messages, one reply, one correction and one disconnect: the smallest scope at which every rule fails on a broken copy and every flow happens. |
| [Flows.tla](Flows.tla), [flows.cfg](flows.cfg), [scenes.mjs](scenes.mjs) | Each flow, claimed never to happen. The statechart skill refutes every claim with the shortest path on which the flow happens, told step by step: that path is the flow's scene. A flow that no step carries out has no scene. |
| [chart.cfg](chart.cfg), [chart.mjs](chart.mjs) | One message and one reply, for one physician and one patient: arriving, matching, the review, the card, copying, sending and the reply Coda sees. |
| [chart-again.cfg](chart-again.cfg), [chart-again.mjs](chart-again.mjs) | The patient writing again while the reply is open: the quiet update, the offer after an edit, and a revision that takes the news in or that an edit stops. |
| [chart-correction.cfg](chart-correction.cfg), [chart-correction.mjs](chart-correction.mjs) | Wrong patient: the conversation moves, Dr Bell loses a reply he may no longer see, and the withheld reply is redrafted, kept with the physician's edit, or closed. |
| [chart-disconnect.cfg](chart-disconnect.cfg), [chart-disconnect.mjs](chart-disconnect.mjs) | The practice disconnecting and reconnecting: the empty card, Done, and the reply Coda sees after reconnecting. |

From the repository root, check the rules, tell the scenes, and draw each chart:

```bash
.agents/skills/statechart/scripts/statechart verification/physician/messaging/Messaging.tla
.agents/skills/statechart/scripts/statechart verification/physician/messaging/Flows.tla --config verification/physician/messaging/flows.cfg --view verification/physician/messaging/scenes.mjs
.agents/skills/statechart/scripts/statechart verification/physician/messaging/Messaging.tla --config verification/physician/messaging/chart.cfg --view verification/physician/messaging/chart.mjs
```

Each focused chart runs the same way with its own `.cfg` and `.mjs`. Every chart is the same model at a smaller scope, and each leaves out the others' areas. The one-message chart draws revising, editing and No reply needed from the plain draft only, and shows the words sent only once Coda has seen the reply. The write-again chart doesn't draw copying, and draws No reply needed and revising from the first draft only. The correction chart doesn't draw revising or copying, and names Dr Bell's open reply but not Dr Ames's. The disconnect chart doesn't draw editing, revising or copying, and shows the channel's state only on the review and the card.

The model reaches no code yet. The code follows it: each bug path is replayed against the real product, and the feature gets a verify skill, once intake, linking and the card exist.
