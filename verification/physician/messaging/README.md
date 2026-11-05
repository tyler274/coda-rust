# Patient messages

This folder is the spec for how a patient's message reaches the right patient in Coda Agents, and how its reply works until the practice has answered. The model is the agreed behavior, TLC checks that it keeps its rules, and the code follows it. [physician.md P3](../../../intent/physician.md#p3-a-patient-message-is-linked-a-reply-is-proposed-revised-and-handed-off) owns the intent this follows.

Spruce is the practice's channel in the MVP, and the spec holds for any channel a practice connects later: what differs between channels is configuration. Urgent messages, attachments, Ask Coda to reply and the Duet For my doctor message follow physician.md, outside this spec.

## How patient messages work

- **Arriving.** Coda reads the channel and never sends in it. Each message Coda reads is filed once, whatever order or how often the channel delivers it. Routine catching up is never shown. A message shows nowhere until the model decides whether it's patient mail, or for 15 minutes if it hasn't; mail it judges isn't from a patient goes to the Filtered list, where This is a patient brings it back.
- **Matching.** A conversation already confirmed for a patient routes to them, and so does a sender confirmed for exactly one patient, or a conversation the channel itself ties to exactly one patient. Any other message waits on Messages to match and the messaging manager's Today, with Coda's suggestion. Any physician can confirm it for a patient they may access, and the messaging manager for anyone; a suggestion never names a patient the viewer may not access. Leaving it unlinked covers only that message, so the conversation's next message waits again.
- **Reviewing.** Once linked, the message joins the patient's record and wakes their review, and the patient shows Coda is reviewing until it finishes. The review reads the whole conversation, including what the practice sent, and reads again if something new arrives first. It drafts a reply only where one would help; acknowledgments and resolved conversations end quietly.
- **The reply.** A conversation has one reply, answering every message the practice hasn't answered, and one Today row. Everyone who may access the patient and can reply in the conversation sees it. Each physician edits their own draft of it, or asks Coda to revise that draft, and Restore my version brings their text back. The card always shows the messages and the reply from one version, and every action acts on the text on screen.
- **The patient writes again.** The card says so at once. A draft nobody edited updates in place with one quiet line; an edited draft stays, and Coda's update replaces it only if the physician chooses it. A message Coda reads late, older than the reply, shows the same way without saying the patient wrote again.
- **Handing off.** The physician copies the reply and sends it in the channel, as it is, edited, or in their own words. Copying changes nothing about the reply; the other physicians see who copied it and when, until Coda sees what was sent.
- **The practice answers.** A message the practice sends in the conversation shows on the card at once and wakes the review. Only that review closes the reply, when it finds the patient answered, citing that message; an automatic reply or an acknowledgment doesn't. A draft Coda was still writing then is dropped. The patient's history records the words the practice actually sent.
- **No reply needed.** Any physician who sees the reply can close it without a reason, for everyone, with a short Undo. A concern about the same messages stays. A newer message's reply replaces it.
- **Wrong patient.** The messaging manager moves a confirmed conversation to another patient, or to not a patient. The whole conversation moves, and later messages route there. At once nothing of it stays readable on the patient it left, a physician who may not access the new patient loses its reply, and anything that relied on it is withheld until a review redrafts or closes it. The correction starts one review for each patient it affects. A physician's draft of the withheld reply stays theirs and never moves to the new patient's reply. Undo within a few seconds restores the reply exactly.
- **A message Coda can't read.** The patient goes on Today with a link to the message in the channel, until Coda reads it or someone marks it Done.
- **The channel stops.** If Coda can't read the channel, Today says so with the reason and when Coda last read it. Only a revoked grant or an administrator's block asks for Reconnect. A completed disconnect deletes messages never accepted into a patient's record; replies and drafts stay. An open reply says the channel is disconnected until someone marks it Done or Coda, reconnected, sees the reply.

## Rules

The model checks these in every state it reaches.

- A physician sees a conversation, its reply and their draft only while they may access its patient.
- A conversation reaches only the patient it's linked to: once it moves, nothing of it stays readable on the patient it left.
- Until it's settled, every patient message Coda filed shows to someone who can act on it.
- Only a review that read the practice's message closes a reply as answered. Copying, opening, an automatic reply or an acknowledgment never does.
- Closing a reply never closes a concern about the same messages.
- Coda never replaces a physician's draft unless they choose its update, and one physician never changes another's draft.
- A reply never misses a message Coda filed after it without the card saying so.
- One message, practice reply or correction starts at most one review for each patient it affects.
- What the patient's history says the practice sent is what the channel shows it sent.

These rest on stored records or the channel, which the model can't hold, and no test holds them yet:

- Coda saves each page it reads before it moves its place in the channel, and files an item delivered twice as one.
- Each practice's channel has one reader at a time.
- A review starts once for its event across restarts and retries.
- The patient's words are never changed, only formatted, with unsafe content stripped.
- Undo restores the reply exactly.
- A moved message is marked entered in error and never deleted, and what Coda wrote from it can't surface.
- One unreadable message, or one patient whose filing fails, never holds up the others.

## Files

| File | Holds |
| --- | --- |
| [Messaging.tla](Messaging.tla), [Messaging.cfg](Messaging.cfg) | The design as a TLA+ model: one conversation, the channel, the link, each patient's review, the shared reply, each physician's draft, and the patient's history, with the rules, one `INVARIANT` each. The config checks the rules with two patients and two physicians, one of whom may access only one patient, so a correction can take a reply from someone. |
| [chart.cfg](chart.cfg), [chart.mjs](chart.mjs) | The chart: one message and its reply, for one physician and one patient, in the physician's words. |

Check the rules, then draw the chart, from the repository root:

```bash
.agents/skills/statechart/scripts/statechart verification/physician/messaging/Messaging.tla
.agents/skills/statechart/scripts/statechart verification/physician/messaging/Messaging.tla --config verification/physician/messaging/chart.cfg --view verification/physician/messaging/chart.mjs
```

The chart leaves out a second physician, a second patient and the corrections they need, the patient writing twice, unreadable messages, concerns and disconnecting, and every message waits for a person to match it. It doesn't draw copying, revising, or the practice writing before Coda sees it, since none of them changes what the physician sees on Today. The rules are checked in both runs.

The channel owns the conversation and everything sent in it. The link owns which patient a conversation belongs to; each patient's review owns whether a reply would help and what it says; each physician owns their draft. Today is read from these through each physician's current access, never stored. The model lets the patient, the practice, Coda's reads and reviews, the physicians and the messaging manager act in any order, and the channel deliver late or go away. TLC checks the rules in every state the model reaches; it doesn't reach the code.
