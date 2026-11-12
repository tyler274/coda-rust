# Chat

How you, a physician, talk with Coda about one patient: how you send, steer, queue and stop, what you see while Coda works, and what each ending leaves. [physician.md](../../../intent/physician.md#p1-ask-about-a-patient-with-evidence) links here.

## Words

- **Conversation:** your chat with Coda about one patient.
- **Answer:** Coda's response to one or more of your messages. A finished answer is one Coda completed. A partial answer is what Coda had written when it was cut off.
- **Working:** Coda is producing an answer in this conversation.
- **Send now:** sends a message at once. While Coda is working it steers: Coda takes the message into the answer in progress.
- **Send next:** queues a message while Coda is working.
- **Queued message:** a message that waits for the answer in progress to end. Coda keeps it, not the browser.
- **Stopped:** you pressed Stop, and the work waits for Resume. The answer keeps its stopped mark after the work moves on.
- **Cut off:** Coda reached a limit before it finished the answer.
- **Couldn't finish:** Coda's own recovery is used up, or the model service refused the work.
- **Step:** one thing Coda did for an answer, such as a record it read or a search it ran, with how it ended.
- **Command:** one thing you ask Coda to do to one item: send a message, Stop, Resume, Try again, change or delete a queued message, archive a conversation.
- **Unconfirmed:** Coda's answer to a command never reached the page, and asking again on its own didn't settle it.
- **Archived:** a conversation you put away. It keeps everything and comes back.

## States and steps

The conversation's work:

| From | Step | To |
| --- | --- | --- |
| Idle | you send a message | Working |
| Working | you press Send now | Working: Coda takes the message in at once, and one answer covers it |
| Working | you press Send next | Working, and the message is queued |
| Working | Coda restarts | Working: the same answer continues on its own |
| Working | Coda finishes the answer, with nothing queued | Idle: the finished answer shows whole |
| Working | Coda finishes the answer, with messages queued | Working: Coda answers every queued message together in one reply |
| Working | you press Stop | Stopped: the answer is marked stopped with Resume, and queued text returns to the composer |
| Working | you open Discuss on a response | Stopped, the same as after Stop |
| Working | Coda reaches a limit | Cut off: what Coda had written shows, when there is any, with Try again, and queued messages stay queued |
| Working | Coda can't finish | Couldn't finish, with Try again, and queued messages stay queued |
| Stopped | you press Resume | Working: Coda continues the same answer |
| Stopped | you send a message | Working on that message. The stopped answer stays marked stopped, without Resume |
| Cut off, or Couldn't finish | you press Try again | Working: Coda continues the same answer, and queued messages wait for it |
| Cut off, or Couldn't finish | you press Send now on a queued message | Working on that message, and the other queued messages wait for it |

A queued message:

| From | Step | To |
| --- | --- | --- |
| Typed in the composer | you press Send next while Coda is working | Queued: it shows under the answer |
| Queued | you edit it and press Update | Queued, with the new text |
| Queued | you delete it | Gone |
| Queued | you press Send now on it | Sent |
| Queued | the answer before it ends finished | Sent, together with every other queued message |
| Queued | the answer before it is cut off, or Coda can't finish it | Queued |
| Queued | you press Stop | Back in the composer as text, and never sent |

A command:

| From | Step | To |
| --- | --- | --- |
| The item as it is | you act on it | Pending: the item is unchanged and shows that it is pending |
| Pending | Coda accepts the command | The item changes |
| Pending | Coda refuses the command | The item is unchanged and says why. Text you typed is kept |
| Pending | no answer reaches the page | Pending: Coda asks again on its own, with the same command |
| Pending | asking again didn't settle it | Unconfirmed: the item says so, with Check again |
| Unconfirmed | you press Check again | Pending |
| Pending, or Unconfirmed | you reload the page | The same |

The steps of one answer:

| From | Step | To |
| --- | --- | --- |
| Working | Coda starts something | A step shows as running |
| Running | it ends | The step shows how it ended |
| Running | you press Send now | The step still ends, and shows how it ended |
| Running | you press Stop | The step shows that it didn't finish |
| Steps showing | the answer ends | The steps fold into one line that still opens |
| Folded | you open the conversation later | The same steps, in the same order |

Putting a conversation away:

| From | Step | To |
| --- | --- | --- |
| In your list, with nothing working and nothing queued | you archive it | Archived, and Undo shows |
| In your list, with work in progress or a message queued | you archive it | Refused, and the page says why |
| Archived | you press Undo | In your list |
| Archived | you send a message in it, or press Resume or Try again | In your list |

## Rules

1. A conversation has at most one answer in progress.
2. Every message you send reaches Coda exactly once: a lost answer to a command, a reload or a restart never drops it and never sends it twice.
3. Coda accepts each command at most once, and an item changes only after Coda accepts its command.
4. One answer covers every message that steered it.
5. A queued message is sent only when the answer before it ends finished or when you press Send now on it.
6. Messages that leave the queue together get one reply.
7. After Stop, nothing runs in the conversation until you press Resume or send a message, and a restart doesn't change that.
8. Stop never sends a queued message and never loses its text: after Stop, the composer holds the text of every message that was queued.
9. A queued message shows on every device until it is sent, deleted or returned to the composer.
10. A stopped answer offers Resume, and an answer that was cut off or that Coda couldn't finish offers Try again; neither offers the other.
11. No part of an answer's text shows before the answer ends.
12. An item shows Unconfirmed only after Coda asked again on its own, and a pending or unconfirmed command is still there after a reload.
13. A conversation is archived only while nothing in it is working or queued.
14. Archiving a conversation changes nothing else: its messages, answers and steps stay, and so do the patient's attention items and follow-ups.

These rules rest on the code, its records and the screen, not on the order of steps, so the model doesn't check them:

- In every composer, Enter sends and Shift+Enter makes a new line. While Coda is working, the composer offers Send now and Send next: Enter is Send now, and Command+Enter, or Control+Enter on Windows, is Send next.
- Saving an edit to a queued message reads Update.
- A step is worded for a physician. It never shows a tool's name, the model's reasoning or other internal words.
- Pulsing dots show while Coda is working.
- A finished answer shows its citations, one Sources control, and Basis and limitations. A stopped answer and a partial answer are not finished work: they show no citation marks, no Sources and no Basis.
- The sidebar and the patient page read conversations from one list, so an archived conversation leaves both.
- You see and act on a conversation only while you may access its patient.

## Outside this spec

What happens inside a run when you steer, when Coda restarts or when a request is retried follows the [harness's guarantees](../../../intent/harness.md#guarantees). Citations, Sources and Basis and limitations follow [citations](../citations/README.md). Drafts in the composer, dictation, feedback and what Discuss opens follow [physician.md](../../../intent/physician.md). What a new message from the composer does while an answer is cut off or Coda couldn't finish it, and whether Try again stays after it, are not decided yet.
