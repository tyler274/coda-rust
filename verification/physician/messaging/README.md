# Patient messages

This folder is the spec for how a patient's message reaches the right patient in Coda Agents, and how its reply card works until the practice has answered. The model is the agreed behavior, TLC checks that it keeps its rules, and the code follows it. [physician.md](../../../intent/physician.md#p3-a-patient-message-is-linked-a-reply-is-proposed-revised-and-handed-off) owns the intent this follows.

The spec holds for any channel the practice connects. Spruce is the channel in the MVP. Each channel comes through its own adapter, which tells Coda the channel's conversations, their messages and the replies the practice sent in them. What differs between channels is who can reply in a conversation and which conversations route without a person confirming, and the rules don't change with either. What the review reads, drafts and flags follows P3 and the patient review, outside this spec; here each is one step whose outcome Coda chooses.

## How patient messages work

- **Connecting.** The practice connects its channel once. Coda starts with new messages and offers an explicit option to review a backlog. Disconnecting and reconnecting loses and duplicates nothing. A completed disconnect deletes messages that were never accepted into a patient's record, and reply drafts. An open reply card stays without its draft, says the channel is disconnected and links to the message, until someone marks it Done or Coda, reconnected, sees the reply.
- **Matching.** A conversation already confirmed for one patient routes on its own, and so does one the channel identifies as exactly one patient's. Every other message gets Coda's suggestion and waits for a person: it goes to the messaging manager's Today and to the practice-wide Messages to match list, where any physician can confirm it, pick another patient or leave it unlinked. Leaving it unlinked covers only that message; the conversation's next message gets a suggestion and waits again. An unmatched message marked urgent leads Today as Now for everyone who sees it.
- **Linked.** One conversation belongs to one patient, and its later messages route to that patient. Once linked, the message joins the patient's record and wakes their review. Only that patient's review reads it.
- **A reply is proposed.** The review decides whether a reply would help. A proposed reply shows on Today as one row for the message, which leaves when the reply and any concern Coda flagged with it are both handled. Each message has one card, shared by everyone who may access the patient and can reply in that conversation; with Spruce, that's every physician with access. An edit shows to all of them, and one No reply needed clears it for all.
- **Revising.** The physician edits the reply, or asks Coda to revise it in place, and Restore my version brings their own text back. Anything new that arrives before the draft is shown is taken in first. If the patient writes again, an untouched draft updates in place with one quiet line; an edited one says the patient sent another message, and Coda's update replaces the physician's text only if they choose it. If no reply is ready, the physician can ask for one, and Coda always drafts one.
- **Handing off.** The physician copies the reply and sends it in the original conversation; Coda never sends patient mail. The row stays until Coda sees the sent reply. A reply the practice sent outside Coda shows on the card right away and wakes the review, which decides whether anything is left to track, and a draft Coda was still writing then is dropped and never shown. No reply needed closes the card with a short Undo, and a newer message's reply replaces a pending No reply needed.
- **Wrong patient.** The messaging manager uses Wrong patient? on a confirmed link, naming another patient or not a patient. The fix moves the whole conversation and applies going forward: its messages stop affecting the wrong patient and move to the right one, later messages route there, and one correction starts one review. Until the re-review decides, anything that relied on the moved messages is withheld, then redrafted or closed, and everything Coda wrote while its memory held them is marked and can't surface. Undo within a few seconds restores the reply exactly.
- **A message Coda can't read.** The patient goes on Today with a link to the message in the channel until Coda reads it or the physician marks it Done. A message Coda is holding or retrying is never invisible.
- **Access.** Each physician sees linked conversations only for patients they may access, and loses them at once when that access ends.

## Rules

The model checks these in every state it reaches.

- A physician sees a linked message, its card and its Today row only while they may access its patient.
- A message reaches only the patient its conversation is linked to: no other patient's review reads it, and after a correction nothing Coda wrote from it shows on the patient it left.
- Until a person handles it or its review settles it, every patient message shows to someone who can act on it: waiting for a match, being reviewed, unreadable, awaiting a reply, or with its channel disconnected.
- Coda never replaces the physician's edited reply unless they choose its update.
- A card never shows a draft that misses a newer message in its conversation without saying so, and never shows a draft that a sent reply overtook.
- A new message, a sent reply or a correction starts at most one review.

These rest on stored records or the channel, which the model can't hold, and no test holds them yet:

- The adapter recognizes a message delivered twice as one message.
- Each practice's channel connection has one intake lease, so no message is filed twice or lost.
- The patient's words are never changed, only formatted, with unsafe content stripped.
- Undo restores the reply exactly.
- A message moved off the wrong patient is marked entered in error and never deleted.
- A completed disconnect deletes messages never accepted into a record, and reply drafts; accepted evidence stays under the record's policy.
