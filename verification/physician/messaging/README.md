# Patient messages

This folder is the spec for how a patient's message reaches the right patient in Coda Agents, and how its reply card works until the practice has answered. The model is the agreed behavior, TLC checks that it keeps its rules, and the code follows it. [physician.md P3](../../../intent/physician.md#p3-a-patient-message-is-linked-a-reply-is-proposed-revised-and-handed-off) owns the intent this follows.

## What it is for

Patients already write to the practice in its messaging channel, and the work between visits piles up there. Coda doesn't add an inbox or sit in the channel's path. It reads what arrives, links it to one patient, brings it into that patient's review, and drafts a reply only where one would help. The physician sends in the channel, and Coda learns the work is done by seeing that reply. Acknowledgments and resolved conversations never reach the physician, because the measure is fewer things the physician touches, not faster drafting ([physician: The point](../../../intent/physician.md#the-point), Principle 10).

The spec holds for any channel the practice connects. Spruce is the channel in the MVP. What differs between channels is configuration, and the rules don't change with it:

| Channel setting | Spruce | In the model |
| --- | --- | --- |
| Who can reply in a conversation | Every physician with access to the patient | `Repliers` |
| Which conversations route without a person | One whose one outside participant is saved as the contact of exactly one patient, linked to exactly one EHR record that matches exactly one Coda patient | `ChannelIdentifies` |
| Who corrects a confirmed link | The messaging manager, the person who set up the connection | `Manager` |

## Who owns what

Each fact has one owner, and everything else reads it ([PRINCIPLES 11](../../../intent/PRINCIPLES.md)).

| Owner | Holds | Never |
| --- | --- | --- |
| The channel | The conversation and everything sent in it | Changed by Coda: Coda reads it and never sends |
| Intake | Which of the channel's items Coda has filed, in order | Shown to the physician while catching up |
| The link | Which patient a conversation belongs to | Set by a model score alone where P3.7 asks for a person |
| The patient's review | Whether a reply would help, and the draft | Run for a patient the conversation isn't linked to |
| The reply card | One card for the conversation's unanswered messages: Coda's draft or the physician's edit | Replaced with Coda's text unless the physician chooses it |
| The patient's history | What the practice actually sent | Filled from the draft |
| Today | Nothing of its own: it is read from the link, the card, the reviews and the channel, through each physician's current access | Stored as a separate list |

## How a message moves

- **Arriving.** The patient writes in the channel. Coda files each item in order while connected; catching up is never shown.
- **Matching.** A conversation already linked, or one the channel says is exactly one patient's, routes on its own. Every other message waits on the practice-wide Messages to match list and the messaging manager's Today, where any physician confirms Coda's suggestion, picks another patient or leaves it unlinked. Leaving it unlinked covers only that message. Confirming links the conversation: its waiting messages join the patient, and replies the practice already sent there go into the patient's history.
- **Reviewing.** A linked message wakes the patient's review, which reads the whole conversation, including the practice's own replies, and reads again whenever something new arrives before it finishes. It ends with a draft or with no reply needed. An acknowledgment gets no row.
- **The card.** A draft opens one card for the conversation, shared by everyone who may access the patient and can reply there. When the patient writes again, the card says so at once. An untouched draft then updates in place; an edited one keeps the physician's text and offers Coda's update. Asking Coda to revise is the physician's choice to let it replace their text, and editing during a revision cancels it.
- **Handing off.** Copy reply changes nothing Coda holds: the row stays. The physician sends in the channel, with the copied text, an edit, or words of their own. When Coda sees the reply, the history records the words actually sent, the messages before it are settled, the card closes for everyone, and the review wakes to see whether anything is left.
- **Settling without a reply.** No reply needed closes the card for everyone. A message Coda can't read waits on Today with a link to the channel until someone marks it Done.
- **Wrong patient.** The messaging manager moves the whole conversation. The old patient's review lets go of it at once, the card moves and is withheld until the new patient's review redrafts or closes it, history moves with it, and anyone who can't see the new patient loses the card. One correction starts one review, on the new patient.
- **Disconnecting.** Reply drafts are deleted. An open card stays without its draft, says the channel is disconnected, and waits for Done or for Coda, reconnected, to see the reply.

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

## Where each part comes from

Every state and step traces to [physician.md](../../../intent/physician.md); each action's comment in the model names its lines too.

| State or step | Source |
| --- | --- |
| The channel owns the conversation; Coda never sends | Principle 10; P3.1, P3.18; Rules: Authority and sending |
| Filing in order while connected; catching up never shown | P3.3, P3.4; Engineering decisions: Mail |
| Routing on its own, or waiting for a person | P3.7, P3.8, P3.9 |
| Leave unlinked covers one message | P3.9 |
| The whole conversation is evidence once linked | P3.11 |
| The review reads again before it finishes | P3.16; The concepts: Patient review |
| No reply needed is the review's call; no row for it | P3.12; PRINCIPLES 1, 4 |
| One card, shared by everyone who may access and reply | P3.13 |
| The patient writes again: quiet update, or offer when edited | P3.17 |
| Revise, and editing during a revision | P3.16 |
| Copy keeps the row; the row leaves when Coda sees the reply | P3.18, P3.19; Rules: Authority and sending |
| History records the words sent | P3.19; Principle 3 |
| No reply needed; Done on an unreadable message | P3: No reply needed, Message Coda can't read |
| Wrong patient moves the whole conversation; one review | P3: Wrong patient, Wrong link after use |
| Disconnect empties the card and keeps it | P3.5 |
| Access to cards and rows | P3.2; Rules: Access and privacy |
| Today read from the owners, never stored | PRINCIPLES 11; The concepts: Today |

## Files

| File | Holds |
| --- | --- |
| [Messaging.tla](Messaging.tla), [Messaging.cfg](Messaging.cfg) | The design as a TLA+ model: the channel, intake, the link, each patient's review, the card, history and who sees what, with the rules as `INVARIANT`s. The config checks one conversation with two patients, two physicians (one who may see only the first patient), two patient messages, one reply, one correction and one disconnect. Each is the smallest size at which every rule still fails on its broken copy and every journey above happens. |
| [chart.cfg](chart.cfg), [chart.mjs](chart.mjs) | The chart: one patient message and one reply, for one physician and one patient, in the physician's words. |

Check the rules, then draw the chart, from the repository root:

```bash
.agents/skills/statechart/scripts/statechart verification/physician/messaging/Messaging.tla
.agents/skills/statechart/scripts/statechart verification/physician/messaging/Messaging.tla --config verification/physician/messaging/chart.cfg --view verification/physician/messaging/chart.mjs
```

The chart leaves out a second patient message, a correction, a disconnect, an unreadable message and a second physician; the rules are checked against all of them. From the copied card and the physician's edit it draws only sending, from a revision only finishing, and it draws revising, editing and No reply needed from the plain draft. The words sent appear only once Coda has seen the reply, where the history shows them.

The model reaches no code yet. The code follows the model: each bug path found here is replayed against the real product, and the feature gets a verify skill, once the intake, linking and card exist.
