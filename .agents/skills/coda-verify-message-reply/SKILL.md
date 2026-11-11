---
name: coda-verify-message-reply
description: "Check a change to message reply against its TLA+ model with the pstack /statechart skill: each README rule checked by TLC and seen failing on its own broken copy, then the four charts read in the physician's words. Use when a change touches the message-reply README, MessageReply.tla, its configs or charts, or the code that reviews linked messages, writes or withholds the reply, or keeps drafts."
---

# Verify message reply

The spec is [verification/physician/message-reply/README.md](../../../verification/physician/message-reply/README.md).
[MessageReply.tla](../../../verification/physician/message-reply/MessageReply.tla), with
[MessageReply.cfg](../../../verification/physician/message-reply/MessageReply.cfg), holds rules 1
to 12. Each `INVARIANT` or `PROPERTY` quotes its README rule.

The model follows the README's two graphs, States and steps: each step is one action, and each state one value. The conversation's graph: each patient message is open
or handled, Coda is reviewing or not, and the reply is ready, withheld, closed with Undo, or none.
Each physician's graph: no draft, your text, or Coda's revision of it, on the reply it belongs to.
The card is not stored: `Card(d)` says what it shows from that state.

| # | Rule | The harm when it breaks |
| --- | --- | --- |
| 1 | OnlyWithAccess | A physician sees or acts on a reply of a patient they may not access. |
| 2 | OnlyTheReviewAnswers | A reply closes as answered when the practice didn't answer, an automatic reply counts as an answer, or a message the practice answered never ends handled. |
| 3 | ConcernStays | Closing a reply hides Coda's concern, or a move to not a patient leaves it. |
| 4 | YourDraft | Coda or another physician changes your draft, your draft shows on a reply you didn't write it for, or you see an old reply of Coda's. |
| 5 | CopiesWhatYouSee | "Copy reply" copies a text the card doesn't show, or copying changes the reply. |
| 6 | CardShowsNew | The card hides a new message, or doesn't say Coda is reviewing it. |
| 7 | OneReviewEach | One event starts two reviews, for example after a restart. |
| 8 | HistoryTrue | The patient's history records words the practice didn't send. |
| 9 | ShowsUntilHandled | A patient message shows to nobody who can act on it, or Coda never finishes reviewing. |
| 10 | WithheldAfterMove | After a move, the reply shows the moved messages' old text, or the old patient's review writes it. |
| 11 | NoteWhileDisconnected | An open reply doesn't say the channel is disconnected, or says so when it isn't. |
| 12 | UndoBringsBack | Undo brings back the wrong reply or messages, or brings back a reply that is no longer current. |

The README's rules after rule 12 rest on the code, its records and the screen, not on the order of
steps, so the model doesn't check them. No test holds them yet. They stay unverified until the
review, reply and card code and its tests exist.

## Coverage

The sync commit is 0065473e. It is the reference for full qualification, not evidence that a later
revision passed. No product code carries the model's steps. When code does, each action's comment
names that code, and the code joins "When a change needs this". Move the sync commit forward only
after full qualification, as `/maintain-formal-verification-skill` describes.

## When it's verified

Follow [local model runs](../../../verification/README.md#local-model-runs) for correction checks,
full qualification, resource limits and stopping conditions. Name the checked commit and keep
unfinished cases visible in the PR's evidence. The default configuration checks all twelve rules;
full qualification also covers every break, chart and enlarged bound below.

## When a change needs this

Run it when a change touches the message-reply README, `MessageReply.tla`, `MessageReply.cfg`,
`Charts.tla`, or a chart's `.cfg` or `.mjs`.

## Run

1. Check the model with TLC: `MessageReply.tla` with `MessageReply.cfg`. It passes when all twelve
   rules hold. Use `/statechart` with the local resource limits; the full state space exceeds the
   chart budget, so it checks the model without drawing a chart.
2. Select and explain correction breaks as [local model runs](../../../verification/README.md#local-model-runs)
   describes. Run them on scratch copies: against the rule alone, which must fail, and against all the other
   rules, which must hold. Full qualification runs every break. A new rule gets a new break.
   Preserve each exact scratch model and configuration, the command, its
   exit status and the complete TLC output, including the counterexample for the intended failure.
   Summary tables alone do not establish these checks.
3. Exercise partial answers on a scratch scenario: two patient messages arrive before one practice
   message, then the review reads it and finishes with a reply. Check both singleton answer sets
   (`{1}` and `{2}`): only that message ends answered, and the ready reply covers the other one.
   Preserve a reachable trace for each outcome; passing invariants alone do not establish that
   either path exists.
4. Check that each step in the README's tables is an action in the model, and each action a step.
5. For full qualification, run the model one size larger on each constant in turn:
   three patients, three physicians, three patient messages, two moves. Each run must find nothing.
   Preserve each enlarged configuration and its command, exit status and complete TLC output.
   Agree the execution budget first; an incomplete run is not a pass.
6. Draw affected charts with `/statechart` on `Charts.tla`, with their `.cfg` and `.mjs` as the view.
   Full qualification draws every chart.
   Render it as the skill says, and read it. All rules must hold in each chart.

| Chart | Starts | Shows |
| --- | --- | --- |
| chart | with no message | the patient writes, Coda reviews, and the reply ends answered, closed as needing no reply, or with no reply needed, with Undo |
| chart-draft | Coda's reply to one message is ready | you edit, ask Coda to revise and restore, while the patient writes again and Coda writes a newer reply |
| chart-move | as chart-draft, with your draft; another physician may access only patient A | Coda acts on a move to patient B or to not a patient, and patient B's review writes the reply again |
| chart-channel | as chart-draft | the channel is disconnected, and you mark the reply Done or Coda reconnects |

Each `.mjs` says what its chart leaves out.

## Bounds and assumptions

- The model header gives the scope: two patients, two physicians, two patient messages, one practice
  message and one move.
- Which patients each physician may access is an input, and TLC sets it in every
  way in which each patient is someone's. Physicians and patients are only names, so Init keeps one
  setting of each kind. Access does not change during a run.
- Time is fairness: a review that woke always starts, a running review always reads what is new and
  finishes however often Coda restarts, and Undo always goes away unless someone presses it.
- Message matching owns linking, moves and their Undo. Here "Coda acts on a move" is one step, after
  that Undo went away. The move also starts the review of the patient it left, as message matching
  says. That review reads the patient's records without the conversation and doesn't touch this
  reply, so the model leaves it out.
- Copying changes nothing, so the copied text is only in the step. The practice can send Coda's reply
  at any version so far, a physician's own text, its own words, or an automatic reply.
- The model keeps each physician's draft of the reply in front of them. Drafts of earlier replies
  stay as records, outside the model.
- Whether a reply would help, whether a review flags a concern, and which patient messages the
  practice's message answers are judgment: TLC tries each choice. `sent.answers` is that judged
  subset of messages already written, including the empty set. An automatic reply has the empty
  set. Only the review changes a message to answered; unanswered messages remain eligible for its reply.
- TLC covers every state of the finite model. It does not reach the code.

## Each rule fails on its own

| Rule | Break | Expected: rule alone | Expected: other rules |
| --- | --- | --- | --- |
| OnlyWithAccess | a physician who may not access the patient closes the reply | fails | hold |
| OnlyTheReviewAnswers | an automatic reply can answer the patient | fails | hold |
| ConcernStays | "No reply needed" also closes the concern | fails | hold |
| YourDraft | asking Coda to revise changes every physician's draft | fails | hold |
| CopiesWhatYouSee | "Copy reply" copies Coda's newest reply, not your draft | fails | hold |
| CardShowsNew | the card shows only the messages its reply answers | fails | hold |
| OneReviewEach | a restart requeues messages already consumed by the running review | fails | hold |
| HistoryTrue | the history records Coda's newest reply, not what the practice sent | fails | hold |
| ShowsUntilHandled | a review with no reply to write leaves the messages open | fails | hold |
| WithheldAfterMove | a move leaves the reply ready on the new patient | fails | hold |
| NoteWhileDisconnected | a withheld reply does not say the channel is disconnected | fails | hold |
| UndoBringsBack | Undo still shows after the patient writes again | fails | hold |

## A failure

- **TLC breaks a rule.** The report gives the shortest path. If each step is what the README says,
  the design breaks the rule: take the path to Dev, and never make the rule weaker. If a step can't
  happen by the README, the model is wrong: fix that action from the README's words.
- **A break passes its rule, or another rule catches it too.** The rule is not able to fail on its own.
  Find a break only it catches; if none exists, the rule repeats another: take it to Dev.
- **The code is different from the model on a path.** If the code does what the README says and the
  model doesn't, fix the model from the README's words. If the code is different from both, it is a
  bug: play the path as a test, see it fail, then fix the code.
- **An intended change.** Change the README with Dev first, then the model, then the code.
