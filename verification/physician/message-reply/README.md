# Message reply

How Coda proposes a reply to a patient's message, how you, a physician, revise and copy it, and how Coda handles what the practice answers in Spruce. The practice sends in Spruce; Coda reads the conversation. [physician.md](../../../intent/physician.md#p3-a-patient-message-is-linked-a-reply-is-proposed-revised-and-handed-off) links here.

## Words

- **Conversation, linked:** as in [message matching](../message-matching/README.md#words).
- **Review:** Coda's work on one patient. A linked patient message, a practice message or a move wakes it. It reads the full conversation, with what the practice sent, closes what the practice answered, and decides if a reply would help.
- **Reply:** the one reply Coda proposes for a conversation. It answers the open messages its review read. Each physician who may access the patient sees it.
- **Draft:** your own version of a reply. It belongs to that reply.
- **Card:** what you see when you open the reply: the open messages, the reply or your draft, and whether Coda is reviewing.
- **Open, handled:** a linked patient message is open until it is handled. It is handled when a review closes it as answered by the practice, when it needs no reply, when a physician marks it Done, or when its conversation moves to not a patient. A move to another patient doesn't open a handled message again.
- **The practice's message:** what the practice sends in the channel: a reply a physician copied, as it is or edited, its own words, or an automatic reply.

## States and steps

The conversation:

| From | Step | To |
| --- | --- | --- |
| Any | the patient writes, the practice sends a message, or Coda acts on a move to another patient | Coda is reviewing, and a closed reply's Undo goes away. A move also stops the running review |
| Coda is reviewing | the review starts | It reads the full conversation, with what the practice sent |
| Coda is reviewing | Coda restarts | The same review must read again |
| Something new arrived, or Coda restarted | the review reads again | It reads the full conversation again |
| Coda is reviewing | the review finishes: a reply would help | Reply ready, with a concern if Coda flags one |
| Coda is reviewing | the review finishes: no reply would help | Handled: needs no reply, with a concern if Coda flags one |
| Coda is reviewing | the review finishes: the practice answered | Handled: answered |
| Coda is reviewing | the review finishes: nothing is open | Nothing open |
| Reply ready | you close it: no reply needed | Closed, Undo shows |
| Closed, Undo shows | you press Undo | Reply ready |
| Closed, Undo shows | Undo goes away, or a review finishes | Handled: needs no reply |
| Reply ready | Coda acts on a move to another patient | Withheld |
| Withheld | the new patient's review writes the reply | Reply ready |
| Reply ready or withheld | Coda acts on a move to not a patient | Handled, and the concern closes |
| Reply ready or withheld | the channel is disconnected | The reply says the channel is disconnected |
| Disconnected | you mark the reply Done | Handled: Done |
| Disconnected | Coda reconnects and sees the reply | Connected |
| The concern shows | you close the concern | The concern is closed |

Each physician's draft:

| From | Step | To |
| --- | --- | --- |
| No draft: you see Coda's newest reply | you edit | Your text |
| Your text | you ask Coda to revise | Coda's revision |
| Coda's revision | you restore your version | Your text |
| Your text or Coda's revision | Coda writes a newer reply | The same, and Coda offers its update |
| Coda offers its update | you choose the update | No draft |
| Any | "Copy reply" | The same: it copies the text you see |
| Any | Coda acts on a move | Your draft stays on the reply you wrote it for |

## Rules

1. You see a reply or your draft, and act on it, only while you may access its patient.
2. Only a review that read the practice's message closes a reply as answered, an automatic reply never answers it, and the messages a practice message answers always end handled.
3. Closing a reply never closes a concern about the same messages, and a move to not a patient closes both.
4. Coda never replaces your draft unless you ask it to revise or choose its update, one physician never changes another's draft, your draft shows only on the reply you wrote it for, and without a draft you see Coda's newest reply.
5. "Copy reply" copies exactly the text you see, and changes nothing else.
6. The card shows every patient message linked after its reply was written, and says "Coda is reviewing" until the reply answers it.
7. One message, practice message or move starts at most one review for each patient it affects.
8. What the patient's history says the practice sent is what the channel shows it sent.
9. Every linked patient message shows, as "Coda is reviewing" or on a ready reply that answers it, to each physician who may access its patient, until it's handled.
10. After Coda acts on a move to another patient, the reply is withheld until a review of the new patient writes it again, or it closes.
11. An open reply says the channel is disconnected exactly while it is.
12. "Undo" after "No reply needed" brings back the reply it closed, with its messages, and shows only until something new arrives in the conversation or a review finishes.

These rules rest on the code, its records and the screen, not on the order of steps, so the model doesn't check them:

- The Today row shows the start of the patient's newest words, "Drafted a reply", "Flagged" when Coda has a concern, and "Review reply". It goes away when the reply and any concern are both closed. A conversation with no open reply and no concern has no Today row.
- The Today row and the card name the channel's sender, never the linked patient.
- The card shows the open messages, then what Coda flagged, then the reply or your draft, with Basis and Sources. The messages and the reply on the card come from one version, and each action acts on the version the card shows.
- Coda never changes the patient's words. It only formats them, and it removes unsafe content.
- When Coda's newer reply replaces the reply you see, one line on the card says so. A message that Coda reads late, older than the reply, shows the same way, and the card doesn't say that the patient wrote again.
- A message the practice sends shows on the card at once. A review that closes a reply as answered cites the practice's message.
- The card offers "Copy reply" and "Open in Spruce". Coda keeps no approval state. After you copy the reply, the other physicians see who copied it and when, until Coda sees what the practice sent.
- A reply that says the channel is disconnected links to the message in the channel. Replies and drafts stay under the record's policy.

## Outside this spec

Linking, moving and Undo of a move follow [message matching](../message-matching/README.md). Urgent messages, attachments, "Ask Coda to reply" and Duet's For my doctor follow [physician.md](../../../intent/physician.md#p3-a-patient-message-is-linked-a-reply-is-proposed-revised-and-handed-off).
