# Message reply

How Coda's reply to a patient's message works, from the time its conversation is linked until the practice has answered. [Message matching](../message-matching/README.md) links the conversation first. [physician.md P3](../../../intent/physician.md#p3-a-patient-message-is-linked-a-reply-is-proposed-revised-and-handed-off) links here.

## Words

- **Conversation, linked:** as in [message matching](../message-matching/README.md#words).
- **Review:** Coda's work on one patient. It decides if a reply would help, and it writes the reply.
- **Reply:** the one reply Coda proposes for a conversation. It answers every patient message that the practice has not answered. It shows as one Today row and opens as the card.
- **Draft:** one physician's own version of the reply.
- **Handled:** a linked patient message is handled when the practice has answered it, when it needs no reply, when a physician marked it Done, or when its conversation moved to not a patient.

## Journey

Reviewing:

- A linked message wakes the patient's review. The patient shows Coda is reviewing until the review finishes.
- A review starts one time for its event, across restarts and retries.
- The review reads the full conversation, and what the practice sent too. It can cite all of it. If something new arrives first, it reads again.
- The review writes a reply only where one would help. Acknowledgments and resolved conversations get no reply and no Today row.

The reply:

- The Today row shows the start of the patient's newest words, Drafted a reply (with Flagged when Coda has a concern), and Review reply. It names the channel's sender, never the linked patient. The row goes away when the reply and any concern are both closed.
- Each physician who may access the patient sees the reply.
- The card shows the patient's unanswered messages, then what Coda flagged, then the reply, with Basis and Sources.
- Coda never changes the patient's words. It only formats them, and it removes unsafe content.
- Each physician edits their own draft, or asks Coda to revise it. Restore my version brings back their own text.
- The card shows the messages and the reply from one version. Each action acts on what the screen shows.

The patient writes again:

- The card shows the new message at once.
- A draft that nobody edited updates in place, with one line that says so.
- An edited draft stays. Coda's update replaces it only if the physician chooses the update.
- A message that Coda reads late, older than the reply, shows the same way. It does not say that the patient wrote again.

Sending:

- The physician copies the reply (Copy reply, Open in Spruce) and sends it in the channel: as it is, edited, or in their own words.
- Copying changes nothing about the reply, and Coda keeps no approval state. The other physicians see who copied it and when, until Coda sees what was sent.

The practice answers:

- A message the practice sends shows on the card at once and wakes the review.
- Only that review closes the reply as answered, and it cites the practice's message. An automatic reply or an acknowledgment does not close it.
- If Coda was still writing a draft, Coda drops that draft.
- The patient's history records the words the practice actually sent.

No reply needed:

- Any physician who sees the reply can close it for all physicians, with no reason, and with a short Undo.
- A concern about the same messages stays.
- The patient's next message gets a new reply.

After a move:

- When Coda acts on a move, a reply that used the moved messages is withheld, never blank, until a review writes it again or closes it.
- A physician's draft of the withheld reply stays theirs. It never moves to the new patient's reply.
- A physician who may not access the new patient loses the reply.
- A move to not a patient closes the reply and any concern about it.

The channel is disconnected:

- An open reply says that the channel is disconnected and links to the message. This stays until a physician marks it Done, or until Coda, reconnected, sees the reply.
- Replies and drafts stay under the record's policy.

## Rules

1. A physician sees a reply or their draft only while they may access its patient.
2. Only a review that read the practice's message closes a reply as answered, and an automatic reply never answers it.
3. Closing a reply never closes a concern about the same messages, unless the conversation moves to not a patient.
4. Coda never replaces a physician's draft unless they ask it to revise or choose its update, and one physician never changes another's draft.
5. Copy reply copies exactly the text the physician sees.
6. The card shows every patient message linked after its reply was written.
7. One message, practice message or move starts at most one review for each patient it affects.
8. What the patient's history says the practice sent is what the channel shows it sent.
9. Every linked patient message shows as Coda is reviewing, on its reply or on Today, to a physician who can act on it until it is handled.

## Outside this spec

Linking, moving and Undo of a move follow [message matching](../message-matching/README.md). Urgent messages, attachments, Ask Coda to reply and Duet's For my doctor follow [physician.md P3](../../../intent/physician.md#p3-a-patient-message-is-linked-a-reply-is-proposed-revised-and-handed-off). Which patients a physician may access, and whether a practice has roles, are open ([P2](../../../intent/DECISIONS.md)). This spec follows what P2 decides.
