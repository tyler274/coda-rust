# Chat

How a conversation with Coda about one patient works: how messages are sent, steered, queued and stopped, what shows while Coda works, and what each ending leaves. You are the physician. [physician.md](../../../intent/physician.md#p1-ask-about-a-patient-with-evidence) links here.

## Words

- **Conversation:** your chat with Coda about one patient.
- **Answer:** Coda's response to one or more of your messages. A finished answer is one Coda completed. A partial answer is what Coda had written when it was cut off.
- **Working:** Coda is producing an answer in this conversation.
- **Command:** one thing you ask Coda to do to one item: send a message, Stop, Resume, Try again, Discuss, change or delete a queued message, archive a conversation.
- **Accept:** Coda takes a command and applies its effect, once. Every step below that reads "Coda accepts" happens at that moment, whether or not the page has learned of it yet. Coda refuses a command that no longer fits, such as Stop after the answer finished.
- **Send now:** a message sent at once. While Coda is working it steers: Coda takes the message into the answer in progress.
- **Send next:** a message queued while Coda is working.
- **Queued message:** a message that waits for the answer in progress to end. Coda keeps it, not the browser.
- **Stopped:** Coda accepted Stop, and the work waits for Resume. The answer keeps its stopped mark after the work moves on.
- **Cut off:** Coda reached a limit before it finished the answer.
- **Couldn't finish:** Coda's own recovery is used up, or the model service refused the work.
- **Step:** one thing Coda did for an answer, such as a record it read or a search it ran, with how it ended.
- **Pending:** the page sent a command and has not learned Coda's answer.
- **Unconfirmed:** the page asked again on its own and still has no answer.
- **Archived:** a conversation you put away. It keeps everything and comes back.

## States and steps

The conversation's work:

| From | Step | To |
| --- | --- | --- |
| Idle | Coda accepts a message | Working |
| Working | Coda accepts a message as Send now | Working: Coda takes the message in at once, and one answer covers it |
| Working | Coda accepts a message as Send next | Working, and the message is queued |
| Working | Coda restarts | Working: the same answer continues on its own |
| Working | Coda finishes the answer, with nothing queued | Idle: the finished answer shows whole |
| Working | Coda finishes the answer, with messages queued | Working: Coda answers every queued message together in one reply |
| Working | Coda accepts Stop | Stopped: the answer is marked stopped with Resume, and Coda returns the text of every queued message for the composer |
| Working | Coda accepts Discuss on a response | Stopped, the same as after Stop |
| Working | Coda reaches a limit | Cut off: what Coda had written shows, when there is any, with Try again, and queued messages stay queued |
| Working | Coda can't finish | Couldn't finish, with Try again, and queued messages stay queued |
| Stopped | Coda accepts Resume | Working: Coda continues the same answer |
| Stopped | Coda accepts a message | Working on that message. The stopped answer stays marked stopped, without Resume |
| Cut off, or Couldn't finish | Coda accepts Try again | Working: Coda continues the same answer, and queued messages wait for it |
| Cut off, or Couldn't finish | Coda accepts a message, or Send now on a queued message | Working on that message. The earlier answer keeps its mark, without Try again, and queued messages wait for the new answer |

A queued message:

| From | Step | To |
| --- | --- | --- |
| Not yet with Coda | Coda accepts it as Send next | Queued |
| Queued | Coda accepts your edit | Queued, with the new text |
| Queued | Coda accepts your delete | Gone |
| Queued | Coda accepts Send now on it | Sent |
| Queued | the answer before it ends finished | Sent, together with every other queued message |
| Queued | the answer before it is cut off, or Coda can't finish it | Queued |
| Queued | Coda accepts Stop | Returned: its text goes back to the composer, and it is never sent |

A command, as the page knows it:

| From | Step | To |
| --- | --- | --- |
| Not sent | you act on an item | Pending: the page shows the item as pending, and the item has not changed |
| Pending | the page learns that Coda accepted the command | Settled: the page shows the change |
| Pending | the page learns that Coda refused the command | Settled: the item is unchanged and says why. Text you typed is kept |
| Pending | Coda's answer is lost | Pending: the page asks again on its own, with the same command |
| Pending | the page asks again, and Coda answers | Settled: Coda gives the answer it first gave, with any text it returned, and applies nothing twice |
| Pending | the page asks again, and still no answer arrives | Unconfirmed: the item says so, with Check again |
| Unconfirmed | you press Check again | Pending |
| Pending, or Unconfirmed | you reload the page | The same |

The steps of one answer:

| From | Step | To |
| --- | --- | --- |
| Working | Coda starts something | A step shows as running |
| Running | it ends | The step shows how it ended |
| Running | Coda accepts a message as Send now | The step still ends, and shows how it ended |
| Running | Coda accepts Stop | The step shows that it was stopped |
| Working | Coda starts something | One line shows the step Coda is on; the steps sit folded under it |
| Folded | you press the line | The steps open, and stay open until you close them |
| Open | the answer ends | The steps stay open |
| Folded | you open the conversation later | The same steps, in the same order |

Putting a conversation away:

| From | Step | To |
| --- | --- | --- |
| In your list, with nothing working and nothing queued | Coda accepts Archive | Archived, and Undo shows |
| In your list, with work in progress or a message queued | you ask to archive it | Coda refuses, and the page says why |
| Archived | Coda accepts Undo | In your list |
| Archived | Coda accepts a message in it, or Resume or Try again | In your list |

## Rules

1. A conversation has at most one answer in progress.
2. Coda takes each message into the conversation at most once: a lost answer to a command, a reload or a restart never sends it twice.
3. A message you sent is never lost: until the page learns Coda's answer, the page still holds the message and its command.
4. Coda accepts each command at most once, and an item changes only when Coda accepts its command.
5. One answer covers every message that steered it.
6. A queued message is sent only when the answer before it ends finished or when Coda accepts Send now on it.
7. Messages that leave the queue together get one reply.
8. After Coda accepts Stop, nothing runs in the conversation until Coda accepts Resume or a message, and a restart doesn't change that.
9. Stop never sends a queued message and never loses its text: Coda's answer to Stop carries the text of every message that was queued, each time the page asks.
10. A queued message stays queued, on every device, until it is sent, deleted or returned.
11. An answer offers Resume only while its work is stopped, and Try again only while its work is cut off or couldn't finish.
12. No part of an answer's text shows before the answer ends.
13. An item shows Unconfirmed only after the page asked again on its own, and a pending or unconfirmed command is still there after a reload.
14. A conversation is archived only while nothing in it is working or queued.

These rules rest on the code, its records and the screen, not on the order of steps, so the model doesn't check them:

- In every composer, Enter sends and Shift+Enter makes a new line. While Coda is working, the composer offers Send now and Send next: Enter is Send now, and Command+Enter, or Control+Enter on Windows, is Send next.
- Saving an edit to a queued message reads Update.
- A step is worded for a physician. It never shows a tool's name, the model's reasoning or other internal words. A stopped step says that it was stopped, not that what it started failed: Coda may not know how that ended.
- Pulsing dots show while Coda is working.
- A finished answer shows its citations, Sources when it cites any, and Basis and limitations. A stopped answer and a partial answer are not finished work: they show no citation marks, no Sources and no Basis.
- Archiving a conversation changes nothing else: its messages, answers and steps stay, and so do the patient's attention items and follow-ups.
- The sidebar and the patient page read conversations from one list, so an archived conversation leaves both.
- You see and act on a conversation only while you may access its patient.
- When a new message replaces stopped, cut-off or unfinished work, the harness records that work as cancelled. The answer keeps the mark it had.

## Outside this spec

What happens inside a run when you steer, when Coda restarts or when a request is retried follows the [harness's guarantees](../../../intent/harness.md#guarantees). Citations, Sources and Basis and limitations follow [citations](../citations/README.md). Drafts in the composer, dictation, feedback and what Discuss opens follow [physician.md](../../../intent/physician.md). Records and messages that arrive while Coda is answering are not in this spec.
