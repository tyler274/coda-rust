# Message matching

How a patient's message in the practice's channel gets to the right patient in Chords, and how you, a physician, fix a wrong link. [physician.md](../../../intent/physician.md#p3-a-patient-message-is-linked-a-reply-is-proposed-revised-and-handed-off) links here.

## Words

- **Conversation:** a thread in the channel between the practice and a patient, or a person who writes for the patient. It is linked to one patient at most.
- **Linked:** a conversation is linked to a patient, and its messages join that patient's record.
- **Waiting:** a patient message that isn't linked yet and waits for a physician to link it or leave it unlinked.
- **Suggestion:** the patient Coda thinks a waiting message is from.
- **Move:** you put a linked conversation on a different patient, or on not a patient. Moving it again while Undo shows is part of the same move.
- **Coda acts on a move:** Coda marks the moved messages entered in error, removes what it wrote from them, and starts one review for each patient the move affects.
- **Handled:** a patient message is handled when it's left unlinked or not from a patient, or when a completed disconnect deleted it. A linked message is handled as [the reply](../message-reply/README.md#words) says.
- **Can't read the channel:** the channel is connected, but Coda's reads fail for a while, or its grant is revoked or blocked.
- **The channel:** Spruce in the MVP. Another channel differs only in its configuration.
- **The channel ties a conversation to a patient:** in Spruce, its one outside participant is saved as the contact of exactly one patient, linked to exactly one EHR record that matches exactly one Coda patient.

## States and steps

A patient message:

| From | Step | To |
| --- | --- | --- |
| Not written | the patient writes in the channel | Not read |
| Not read | Coda reads it, the one time it keeps it | Coda decides, and it shows nowhere |
| Not read | Coda can't read it | On the patient's Today, with a link to it in the channel |
| Coda can't read it | Coda reads it now | Coda decides |
| Coda can't read it | you mark it Done | Done |
| Coda decides | Coda decides it's from a patient, or 15 minutes pass | It goes where the conversation sends it |
| Coda decides | Coda decides it isn't from a patient | On "Filtered" |
| On "Filtered" | you mark "This is a patient" | It goes where the conversation sends it |
| Waiting | you link it to a patient you may access | Linked, with the conversation's other waiting messages |
| Waiting | you leave it unlinked | Left unlinked; the next message waits again |

Where the conversation sends a message: in a linked conversation, it joins that patient. In a conversation moved to not a patient, it is not a patient's. Otherwise it links on its own when its sender is confirmed for exactly one patient, or the channel ties the conversation to exactly one patient, and nothing points to a different patient. Every other message waits on "Messages to match" and on Today, with Coda's suggestion and other choices.

The conversation:

| From | Step | To |
| --- | --- | --- |
| Not linked | a message links on its own, or you link one | Linked to that patient |
| Linked | you open it, or close it | Open, or closed, for you |
| Linked to patient A | you move it to patient B, or to not a patient | On the new place, and "Undo" shows for a few seconds |
| Undo shows | a message arrives | The message follows the conversation |
| Undo shows | you press Undo | Back on patient A, with any message that arrived |
| Undo shows | you move it again | Undo still goes back to patient A |
| Undo shows | you move it back to patient A | No move: Undo goes away, and Coda does nothing |
| Undo shows | Undo goes away | Coda acts on the move, one time |

The channel:

| From | Step | To |
| --- | --- | --- |
| Coda reads it | Coda moves on past a message it kept | Coda reads it, from its new place |
| Coda reads it | the channel delivers a message again | Nothing changes |
| Coda reads it | Coda can't read it for a while | Today says so, with the reason and the time Coda last read it |
| Coda can't read it for a while | the channel comes back | Coda reads it, from its place |
| Coda reads it | the grant is revoked or blocked | Today says so, and asks you to "Reconnect" |
| Revoked or blocked | you reconnect | Coda reads it, from its place |
| Any | you disconnect it | Disconnected: every message never linked is deleted |
| Disconnected | you connect it | Coda reads new messages, and offers to review the backlog |
| Backlog offered | you accept | Coda reads the backlog |

## Rules

1. You see a conversation only while you may access its patient.
2. A suggestion never names a patient you may not access.
3. A message joins only the patient its conversation is linked to.
4. A conversation that isn't linked yet links on its own when, and only when, its sender is confirmed for exactly one patient or the channel ties it to exactly one patient, and nothing points to a different patient.
5. After a conversation moves to a different patient, nothing of it stays readable on the patient it left.
6. After Coda decides, or after 15 minutes, every patient message shows to a physician who can act on it until it's handled.
7. Undo puts the conversation back on the patient it was on before the move.
8. Coda acts on a move only after Undo goes away, and then it always acts on it.
9. You link a message, or move a conversation, only between patients you may access.
10. Today says Coda can't read the channel exactly while it can't, and asks you to "Reconnect" only for a revoked grant or an administrator's block.
11. A completed disconnect leaves no message that was never linked to a patient.
12. Coda keeps each message exactly once, however many times and in whatever order the channel delivers it.
13. A message Coda can't read shows on Today until Coda reads it, you mark it Done, or a completed disconnect deletes it.
14. Coda reads a message from before it connected only after you accept the offer to review the backlog.

These rules rest on the code and its records, not on the order of steps, so the model doesn't check them:

- Coda never sends in the channel.
- Each practice's channel has one reader at a time.
- One message Coda can't read, or one patient whose linking fails, never stops the others.
- Coda decides whether a message is from a patient with no brittle filters.
- Scores never show as confidence.
- Coda reads an unlinked message only to decide who it's from, and a review never reads it.
- Coda never deletes moved messages, and nothing it wrote from them shows again.
- Coda never shows that it is catching up or retrying.
- After a completed disconnect, linked messages stay under the record's policy.

## Outside this spec

The reply, its drafts and the review follow [message reply](../message-reply/README.md), which starts once a conversation is linked. Urgent messages, attachments, "Ask Coda to reply" and Duet's For my doctor follow [physician.md](../../../intent/physician.md#p3-a-patient-message-is-linked-a-reply-is-proposed-revised-and-handed-off).
