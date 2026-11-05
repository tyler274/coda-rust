# Patient messages

How a patient's message in the practice's channel reaches the right patient in Coda Agents, and how Coda's reply works until the practice has answered. Spruce is the channel in the MVP; another channel differs only in configuration. [physician.md P3](../../../intent/physician.md#p3-a-patient-message-is-linked-a-reply-is-proposed-revised-and-handed-off) links here.

## Words

- **Conversation:** a thread in the channel between the practice and a patient, or someone writing for them. It's linked to at most one patient.
- **Linked:** a conversation is linked to a patient, and its messages join that patient's record.
- **Review:** Coda's work on one patient. It decides whether a reply would help, and drafts it.
- **Reply:** the one reply Coda proposes for a conversation, answering every patient message the practice hasn't answered. It shows as one Today row, and opens as the card.
- **Draft:** one physician's own version of the reply.
- **Handled:** a patient message is handled once the practice has answered it, it needs no reply, it was left unlinked, it isn't from a patient, or someone marked it Done.

## Journey

Reading the channel:

- Coda reads the channel and never sends in it.
- Coda starts with new messages and offers to review the backlog.
- Coda reads each message once, whatever order or however often the channel delivers it, and saves it before moving on in the channel.
- Each practice's channel has one reader at a time.
- Catching up and retries are never shown.
- One unreadable message, or one patient whose linking fails, never holds up the others.

Linking:

- Coda decides whether a message is from a patient, with no brittle filters. Until it decides, or for 15 minutes, the message shows nowhere.
- A message Coda judges isn't from a patient goes to the Filtered list, where This is a patient brings it back.
- A message links on its own when its conversation or sender is confirmed for exactly one patient, or the channel ties its conversation to exactly one patient. In Spruce, that's a conversation whose one outside participant is saved as the contact of exactly one patient, linked to exactly one EHR record that matches exactly one Coda patient.
- Any other message waits on Messages to match and on Today, with Coda's suggested patient and alternatives. Scores never show as confidence.
- A physician links it to a patient they may access, or leaves it unlinked. A suggestion never names a patient the viewer may not access.
- Leaving a message unlinked covers only that message; the conversation's next message waits again.
- Later messages in a linked conversation link to the same patient.
- Coda reads an unlinked message only to decide who it's from. A review never reads it.

Reviewing:

- A linked message wakes the patient's review, and the patient shows Coda is reviewing until it finishes.
- A review starts once for its event, across restarts and retries.
- The review reads the whole conversation, including what the practice sent, and can cite any of it. It reads again if something new arrives first.
- The review drafts a reply only where one would help. Acknowledgments and resolved conversations get no reply and no Today row.

The reply:

- The Today row shows the start of the patient's newest words, Drafted a reply (with Flagged when Coda has a concern), and Review reply. It names the channel's sender, never the linked patient. It leaves once the reply and any concern are both closed.
- Every physician who may access the patient sees the reply.
- The card shows the patient's unanswered messages, then what Coda flagged, then the reply, with Basis and Sources.
- The patient's words are never changed, only formatted, with unsafe content stripped.
- Each physician edits their own draft, or asks Coda to revise it. Restore my version brings back their own text.
- The card shows the messages and the reply from one version, and every action acts on what's on screen.

The patient writes again:

- The card shows the new message at once.
- A draft nobody edited updates in place, with one line saying so.
- An edited draft stays. Coda's update replaces it only if the physician chooses it.
- A message Coda reads late, older than the reply, shows the same way without saying the patient wrote again.

Sending:

- The physician copies the reply (Copy reply, Open in Spruce) and sends it in the channel, as it is, edited, or in their own words.
- Copying changes nothing about the reply, and Coda keeps no approval state. The other physicians see who copied it and when, until Coda sees what was sent.

The practice answers:

- A message the practice sends shows on the card at once and wakes the review.
- Only that review closes the reply as answered, citing the practice's message. An automatic reply or an acknowledgment doesn't.
- A draft Coda was still writing then is dropped.
- The patient's history records the words the practice actually sent.

No reply needed:

- Any physician who sees the reply closes it for everyone, without a reason, with a short Undo.
- A concern about the same messages stays.
- The patient's next message gets a new reply.

Wrong patient:

- A physician who may access the patient moves a linked conversation to another patient they may access, or to not a patient. The whole conversation moves, and later messages follow it.
- At once nothing of it stays readable on the patient it left, and a physician who may not access the new patient loses its reply.
- The moved messages are marked entered in error and never deleted, and what Coda wrote from them can't surface.
- A reply that relied on them is withheld, never left blank, until a review redrafts or closes it. Each patient the move affects gets one review.
- A physician's draft of the withheld reply stays theirs and never moves to the new patient's reply.
- Undo within a few seconds restores the reply exactly.

A message Coda can't read:

- The patient goes on Today with a link to the message in the channel, until Coda reads it or someone marks it Done.

The channel stops:

- If Coda can't read the channel, Today says so, with the reason and when Coda last read it. Only a revoked grant or an administrator's block asks for Reconnect.
- Disconnecting and reconnecting loses and duplicates nothing.
- A completed disconnect deletes messages never linked to a patient. Linked messages stay under the record's policy, and so do replies and drafts.
- An open reply says the channel is disconnected and links to the message, until someone marks it Done or Coda, reconnected, sees the reply.

## Rules

1. A physician sees a conversation, its reply, their draft, or a suggestion naming a patient, only while they may access that patient.
2. A message links on its own only when its conversation or sender is confirmed for that one patient, or the channel ties its conversation to that one patient.
3. Once a conversation moves to another patient, nothing of it stays readable on the patient it left.
4. Once Coda has decided or 15 minutes have passed, every patient message shows to someone who can act on it until it's handled.
5. Only a review that read the practice's message closes a reply as answered.
6. Closing a reply never closes a concern about the same messages.
7. Coda never replaces a physician's draft unless they choose its update, and one physician never changes another's draft.
8. Copy reply copies exactly the text the physician sees.
9. The card shows every patient message linked after its reply was drafted.
10. One message, practice message or correction starts at most one review for each patient it affects.
11. What the patient's history says the practice sent is what the channel shows it sent.
12. While Coda can't read the channel, Today says so.
13. A completed disconnect leaves no message that was never linked to a patient.

## Outside this spec

Urgent messages, attachments, Ask Coda to reply and Duet's For my doctor follow [physician.md P3](../../../intent/physician.md#p3-a-patient-message-is-linked-a-reply-is-proposed-revised-and-handed-off). Which patients a physician may access, whether a practice has roles, who sees a message before it's linked, and who sees a patient's phone and email while matching are open ([P2](../../../intent/DECISIONS.md)); this spec follows whatever P2 decides.
