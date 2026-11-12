# Shared

How you mark records and write notes for your doctor in Duet, and send them to a practice you're connected to. [duet.md](../../../intent/duet.md#d4-for-my-doctor) links here.

## Words

- **Shared:** the tab that holds For my doctor and your practices.
- **Mark:** on or off, on anything you can open in You: a record, one value, a reading or a trend.
- **Note:** your own words for your doctor, as free text. A note can point at a record.
- **To send:** your marked records and the notes you haven't sent.
- **Connected practice:** a practice whose connection with you is live.
- **Send:** a fixed copy of the records and notes you chose, sent to one connected practice.
- **Current version:** a record as its source last gave it.
- **Sent:** Duet has saved the send.
- **Received by the care team:** the practice's agents have picked the send up.

## States and steps

A mark:

| From | Step | To |
| --- | --- | --- |
| Not marked | you mark it in You | Marked, and in To send |
| Not marked | you mark it and Duet can't save it | Not saved yet, and it says so |
| Not saved yet | Duet saves it | Marked, and in To send |
| Not saved yet | you unmark it | Not marked |
| Marked | you unmark it, in You or in Shared | Not marked |
| Marked | its source corrects the record | Marked, at the corrected version, and it says so |
| Marked | you send it | Not marked, and in that send |
| Not marked, sent before | you mark it again | Marked, and in To send |

A note:

| From | Step | To |
| --- | --- | --- |
| Not written | you write it, in Shared or from a record | In To send |
| In To send | you edit it | In To send, with your new words |
| In To send | Duet can't save it | In To send and not saved, and it says so, with your words kept |
| In To send and not saved | Duet saves it | In To send |
| In To send | you edited it on another phone | The newer version shows, and your unsaved words are kept |
| In To send | you delete it | Gone |
| In To send | you send it | In that send, and out of To send |

A send:

| From | Step | To |
| --- | --- | --- |
| To send has something | you tap Send, with everything chosen to start | Choosing what goes |
| Choosing | you take a record or a note out, or put it back | Choosing |
| Choosing | you pick the practice, when you have more than one | Choosing |
| Choosing | you leave without confirming | Nothing is sent, and To send is as it was |
| Choosing | you confirm, after Duet says a send can't be taken back | Sending |
| Sending | Duet saves it | Sent |
| Sending | Duet can't tell whether it saved | Couldn't confirm the send, with "Try again" |
| Couldn't confirm the send | you try again and Duet saves it | Sent, one time |
| Couldn't confirm the send | you try again and Duet still can't tell | Couldn't confirm the send, with "Try again" |
| Sending | the connection ends | Not sent, with the reason, and its marks and notes stay in To send |
| Sent | the practice's agents pick it up | Received by the care team |
| Sent | the connection ends first | Sent, with the date the connection ended. It never becomes Received |
| A record open in You | you send that one record | Choosing, with that record alone |

Send, in Shared:

| When | Shared shows |
| --- | --- |
| No connected practice | Marks and notes work. There is no Send, and Shared says no practice is connected |
| One connected practice | Send names that practice |
| More than one | You pick one for each send, and each send names its practice |
| The connection ends | No Send to that practice. Its earlier sends stay in the list |

## Rules

1. Nothing in For my doctor reaches a practice except in a send you confirmed.
2. A send goes to one practice, and only to a practice connected to you at that moment.
3. A send holds exactly the records and notes you chose, each record at its current version at that moment, and nothing changes it afterwards. A note written from a record takes that record with it.
4. A send reaches the practice at most once, however many times you try again.
5. A send clears the marks it carried and takes the notes it carried out of To send, and leaves the rest.
6. To send shows a record exactly while it's marked, and a note exactly until you send or delete it.
7. Duet shows Sent only after it saved the send, and Received by the care team only after the practice's agents picked it up. A send whose connection ended first stays Sent.
8. Before every send, Duet says that a send can't be taken back and isn't for urgent needs or for questions that need an answer.
9. A mark or a note never shows as saved unless Duet saved it, and Duet never replaces words you haven't saved.
10. A source correction never blocks a send and never changes an earlier send.
11. Joining a practice sends nothing from For my doctor, and leaving one removes no send from your list.

These rules rest on the code and its records, not on the order of steps, so the model doesn't check them:

- Sends show as a list, never as a chat thread.
- No status says a physician looked.
- Nothing else comes back into Duet from a send.
- A note has no prompt, categories or required form.
- A marked trend sends the readings in the range you were looking at.
- A lab's printed reference range doesn't show in For my doctor.
- There is no page, no PDF and no public link.
- Physicians don't approve or manage what you mark or write.

## Outside this spec

Records and readings in You follow [duet.md](../../../intent/duet.md#d2-records-come-in-and-stay-faithful). Joining a practice follows [invitation](../../connected-care/invitation/README.md), and leaving, discharge and what the practice does with a send follow [connected-care.md](../../../intent/connected-care.md#c16-sending-for-my-doctor). What happens to a mark when its source removes the record, and what a send shows when Duet can't tell whether it saved and the connection then ends, are not decided yet.
