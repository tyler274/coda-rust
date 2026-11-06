# Message matching

How a patient's message in the practice's channel gets to the right patient in Coda Agents, and how a physician corrects a wrong link. Spruce is the channel in the MVP. Another channel is different only in its configuration. [physician.md P3](../../../intent/physician.md#p3-a-patient-message-is-linked-a-reply-is-proposed-revised-and-handed-off) links here. When a conversation is linked, [the reply](../message-reply/README.md) starts.

## Words

- **Conversation:** a thread in the channel between the practice and a patient, or a person who writes for the patient. It is linked to one patient at most.
- **Linked:** a conversation is linked to a patient. Its messages join the record of that patient.
- **Suggestion:** the patient Coda thinks a waiting message is from.
- **Move:** a physician puts a linked conversation on a different patient, or on not a patient.
- **Coda acts on a move:** Coda marks the moved messages entered in error, removes what it wrote from them, and starts one review for each patient the move affects.
- **Handled:** a patient message is handled when it is left unlinked or not from a patient, or when a completed disconnect deleted it. A linked message is handled as [the reply](../message-reply/README.md#words) says. Until then, it shows on its patient.

## Journey

Reading the channel:

- Coda reads the channel. It never sends in it.
- Coda starts with new messages and offers to review the backlog.
- Coda reads each message one time, in any order and however many times the channel delivers it. It saves the message before it moves on in the channel.
- Each practice's channel has one reader at a time.
- Coda never shows catching up or retries.
- One unreadable message, or one patient whose linking fails, never stops the others.

Coda decides:

- Coda decides if a message is from a patient. It uses no brittle filters. Until Coda decides, or for 15 minutes, the message shows nowhere.
- A message that Coda thinks is not from a patient goes to the Filtered list. This is a patient brings it back, and Coda then links it as a patient's message.

Linking:

- A message links on its own when one of these is true:
  - Its conversation is already linked.
  - Its sender is confirmed for exactly one patient.
  - The channel ties its conversation to exactly one patient.
- When the sender or the channel points to a different patient, the message waits for a physician.
- In Spruce, the channel ties a conversation to a patient when the one outside participant is saved as the contact of exactly one patient. That contact is linked to exactly one EHR record, and that record matches exactly one Coda patient.
- All other messages wait on Messages to match and on Today, with Coda's suggestion and other choices. Scores never show as confidence.
- A physician links a waiting message to a patient they may access. The conversation is then linked, and its other waiting messages link too.
- A physician can leave a waiting message unlinked. This covers only that message. The next message in the conversation waits again.
- Later messages in a linked conversation link to the same patient.
- Coda reads an unlinked message only to decide who it is from. A review never reads it.

Wrong patient:

- A physician who may access the patient moves a linked conversation to a different patient they may access, or to not a patient. The full conversation moves, and later messages follow it.
- At once, nothing of the conversation, and nothing Coda wrote from it, shows on the patient it left. A physician who may not access the new patient loses it.
- For a few seconds, Undo shows. Coda acts on the move only after Undo goes away, so Undo puts everything back as it was. A physician who may not access the patient it goes back to loses it.
- A message that arrives while Undo shows follows the conversation. Undo takes that message back with the conversation.
- Coda never deletes moved messages. Nothing Coda wrote from them shows again.

A message Coda can't read:

- The patient goes on Today with a link to the message in the channel. The row stays until Coda reads the message or a physician marks it Done.

The channel stops:

- If Coda can't read the channel, Today says so, with the reason and the time Coda last read it. Only a revoked grant or an administrator's block asks for Reconnect.
- Disconnecting and reconnecting loses nothing and duplicates nothing.
- A completed disconnect deletes all messages that were never linked to a patient. Linked messages stay under the record's policy.

## Rules

1. A physician sees a conversation only while they may access its patient.
2. A suggestion never names a patient the viewer may not access.
3. A message joins only the patient its conversation is linked to. A conversation that is not linked yet links on its own only when its sender is confirmed for exactly one patient or the channel ties it to exactly one patient, and nothing points to a different patient.
4. After a conversation moves to a different patient, nothing of it stays readable on the patient it left.
5. After Coda decides, or after 15 minutes, every patient message shows to a physician who can act on it until it is handled.
6. Undo puts the conversation back on the patient it left, and Coda acts on a move only after Undo goes away.
7. A physician links a message, or moves a conversation, only between patients they may access.
8. While Coda can't read the channel, Today says so.
9. A completed disconnect leaves no message that was never linked to a patient.

## Outside this spec

The reply, its drafts and the review follow [the reply](../message-reply/README.md). Urgent messages, attachments, Ask Coda to reply and Duet's For my doctor follow [physician.md P3](../../../intent/physician.md#p3-a-patient-message-is-linked-a-reply-is-proposed-revised-and-handed-off). These are open ([P2](../../../intent/DECISIONS.md)), and this spec follows what P2 decides:

- which patients a physician may access, and whether a practice has roles;
- which physicians see a message before it is linked;
- who sees a patient's phone and email during matching.
