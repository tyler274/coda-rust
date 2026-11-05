---
name: coda-verify-messaging
description: "Check a change to patient messages against its two TLA+ models: TLC on the README's agreed rules, then the charts read in the physician's words. Use when a change touches the patient-messaging spec or models, or the code that reads the practice's channel, links messages, reviews them or shows the reply."
---

# Verify patient messages

The spec is [verification/physician/messaging/README.md](../../../verification/physician/messaging/README.md).
Two models hold its rules, and they meet at the link:

- [Arrival.tla](../../../verification/physician/messaging/Arrival.tla): how a message reaches the right
  patient, the channel stopping, and moving a conversation. Its config is
  [Arrival.cfg](../../../verification/physician/messaging/Arrival.cfg).
- [Reply.tla](../../../verification/physician/messaging/Reply.tla): the reply once a conversation is
  linked, its drafts, copying, the practice's answer and closing it. Its config is
  [Reply.cfg](../../../verification/physician/messaging/Reply.cfg).

Each model lets the other's steps that touch it happen at any moment, so a rule that holds in one
holds whatever the other does. Each `INVARIANT` or `PROPERTY` in a config is one of the README's numbered rules,
quoted in its comment:

| Model | Rules |
| --- | --- |
| Arrival | 2 SuggestionsWithAccess, 3 LinksOnItsOwn, 4 RightPatient, 5 NeverInvisible, 6 OnlyTheReviewAnswers, 11 OneReview, 12 WhatWasSent, 13 TodaySaysSo, 14 NothingUnlinkedKept |
| Reply | 1 OnlyWithAccess, 6 OnlyTheReviewAnswers, 7 ConcernStays, 8 YourWords, 9 CopiesWhatYouSee, 10 CurrentReply, 12 WhatWasSent |

The harm when one breaks: a physician sees a patient they may not access, a patient's words reach
another patient, a patient's message goes unseen, a reply closes as answered when it wasn't, a
physician's words are lost or changed, or the history misstates what the practice sent.

## When a change needs this

Run it when a change touches the messaging README, either model or config, `ArrivalCharts.tla`,
`ReplyCharts.tla`, `words.mjs` or a chart's `.cfg` or `.mjs`. When code carries an action, the
action's comment names that code, and the code joins this list.

## Run

Each full check is too large for the `/statechart` skill's single worker, so run TLC directly, from
`verification/physician/messaging`, with its state folder outside the repo:

```sh
java -XX:+UseParallelGC -cp <tla2tools.jar> tlc2.TLC -workers auto -metadir <scratch>/arrival -config Arrival.cfg Arrival.tla
```

```sh
java -XX:+UseParallelGC -cp <tla2tools.jar> tlc2.TLC -workers auto -metadir <scratch>/reply -config Reply.cfg Reply.tla
```

Each passes when TLC finishes with no rule violated. On an 18-core Mac, Arrival reaches 163,696,658
states in about 18 minutes and Reply 202,837,665 in about 40, and each state folder grows past 10 GB,
so delete it when its run ends.

`Reply.cfg` leaves out disconnects: no Reply rule reads the channel or the line that says it's
disconnected, and a disconnect only delays Coda's next read, which the model already allows. The
disconnect steps stay in the model, and the disconnect chart draws them with every rule checked.

Then draw each chart with the pstack `/statechart` skill, its `.cfg` and `.mjs` on its Charts module,
render it as the skill says, and read it. Every rule must hold in each. A chart runs its model's own
`Spec` from `Init`, with every access setting, at the small scope its config sets, and says what the
physician reading it sees, as "you". Its `ACTION_CONSTRAINT` keeps the steps of its journey in their
likely order, such as Coda reading messages in the order they were sent; the full checks cover every
order. Each `.mjs` says what its chart leaves out.

| Chart | Module | Shows |
| --- | --- | --- |
| linking | ArrivalCharts | a message links on its own, or waits on Messages to match or Filtered for you to link it or leave it unlinked |
| correction | ArrivalCharts | you move a linked conversation to the other patient or to not a patient, or undo it, and the next message follows it |
| channel | ArrivalCharts | Coda can't read Spruce, or the practice disconnects it, beside the message |
| read | ArrivalCharts | a message Coda can't read |
| reply | ReplyCharts | Coda drafts the reply, you copy it, and the practice answers in Spruce or doesn't |
| draft | ReplyCharts | your draft when the patient writes again, and Coda's update |
| revise | ReplyCharts | asking Coda to revise, and restoring your version |
| two | ReplyCharts | two physicians share one reply |
| withheld | ReplyCharts | the conversation moves: the reply is withheld, you lose it if you may not access its new patient, and your draft stays yours |
| disconnect | ReplyCharts | Spruce disconnects while the reply is open |
| concern | ReplyCharts | a concern stays when the reply closes |

## Bounds and assumptions

Each config sets the scope its rules are checked at, and each model's header says what it takes as
one step and what it leaves to records. Which patients each physician may access, and who sees a
message before it's linked, are inputs set every way, since they're open in P2. TLC covers every
state of each finite model; it doesn't reach the code.

## Promises tests hold

These rest on stored records or the channel, which the models can't hold, so a test holds each:

- Coda reads each message once, however often the channel delivers it, and saves it before moving on.
- Each practice's channel has one reader at a time.
- A review starts once for its event, across restarts and retries.
- The patient's words are never changed, only formatted, with unsafe content stripped.
- The card shows the messages and the reply from one version.
- Undo restores the reply exactly.
- Moved messages are marked entered in error and never deleted, and what Coda wrote from them can't surface.
- One unreadable message, or one patient whose linking fails, never holds up the others.

## Checks seen failing

Each rule fails on a broken copy of its model, at the scope its config sets, with TLC's shortest path
as the steps:

| Rule | Model | Broken copy | Steps |
| --- | --- | --- | --- |
| SuggestionsWithAccess | Arrival | Messages to match shows Coda's suggestion to every viewer | 3 |
| LinksOnItsOwn | Arrival | a sender confirmed for two patients links the message to one of them | 3 |
| RightPatient | Arrival | a move leaves the history on the patient it left | 6 |
| NeverInvisible | Arrival | closing the reply leaves its messages open | 6 |
| OnlyTheReviewAnswers | Arrival | the review counts an automatic reply as the practice's answer | 7 |
| OneReview | Arrival | a move starts the new patient's review twice | 4 |
| WhatWasSent | Arrival | the history records every practice message as words written in the channel | 5 |
| TodaySaysSo | Arrival | Today stays quiet about an outage | 1 |
| NothingUnlinkedKept | Arrival | a completed disconnect keeps the practice's held messages | 4 |
| OnlyWithAccess | Reply | a move leaves the reply open for a physician who may not access the new patient | 6 |
| OnlyTheReviewAnswers | Reply | the review counts an automatic reply as the practice's answer | 6 |
| ConcernStays | Reply | No reply needed also closes the concern | 6 |
| YourWords | Reply | Coda's update replaces an edited draft | 10 |
| CopiesWhatYouSee | Reply | Copy reply copies Coda's reply, not the physician's own draft | 7 |
| CurrentReply | Reply | the card doesn't show a message read after the reply was drafted | 6 |
| WhatWasSent | Reply | the history records every practice message as words written in the channel | 2 |

## A failure

- **TLC breaks a rule.** The report gives the shortest path. If every step is what the README says,
  the design breaks the rule: take it to Dev, and never weaken the rule.
- **The code differs from a model on a path.** If the code does what the README says and the model
  doesn't, the model is wrong: fix the action from the README's words. If the code differs from both,
  it's a bug: replay the path as a test, see it fail, then fix the code.
- **An intended change.** Change the README with Dev first, then the models, then the code.
