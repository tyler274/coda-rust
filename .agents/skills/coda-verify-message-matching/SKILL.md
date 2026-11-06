---
name: coda-verify-message-matching
description: "Check a change to message matching against its TLA+ model with the pstack /statechart skill: each modeled README rule checked by TLC, then the two charts read in the physician's words. Use when a change touches the message-matching README, MessageMatching.tla, its configs or charts, or the code that decides, links or moves patient messages."
---

# Verify message matching

The spec is [verification/physician/message-matching/README.md](../../../verification/physician/message-matching/README.md).
[MessageMatching.tla](../../../verification/physician/message-matching/MessageMatching.tla), with
[MessageMatching.cfg](../../../verification/physician/message-matching/MessageMatching.cfg), holds
rules 1 to 6. Each `INVARIANT` or `PROPERTY` quotes its README rule.

| # | Rule | The harm when it breaks |
| --- | --- | --- |
| 1 | OnlyWithAccess | A physician sees a conversation of a patient they may not access. |
| 2 | SuggestionsWithAccess | Messages to match names a patient the viewer may not access. |
| 3 | LinksOnItsOwn | A message joins a patient that nothing confirmed, so one patient's words go into another patient's record. |
| 4 | RightPatient | A moved conversation still shows on the patient it left. |
| 5 | NeverInvisible | A linked message shows to nobody. |
| 6 | UndoPutsBack | Undo leaves the conversation in the wrong place, or Coda changes records for a move that the physician then undoes. |

Rules 7 and 8 (the channel stops) are not modeled. They are not order problems, so tests hold them.

## Coverage

The model describes the README as of the last commit that changed `MessageMatching.tla`. That
commit is the sync commit. No product code carries the model's steps. When code does, each action's
comment names that code, and the code joins "When a change needs this". Move the sync commit forward
only after the check passes on a README or code that changed since it, as
`/maintain-formal-verification-skill` describes.

## When a change needs this

Run it when a change touches the message-matching README, `MessageMatching.tla`,
`MessageMatching.cfg`, `Charts.tla`, `chart.cfg`, `chart.mjs`, `chart-move.cfg` or `chart-move.mjs`.

## Run

1. Check the model with the pstack `/statechart` skill: `MessageMatching.tla` with
   `MessageMatching.cfg`. It passes when the report lists all six rules and NoDeadlock under the
   rules that hold. TLC reaches 588 states in about 2 seconds. That is more than the chart budget,
   so this run draws no chart.
2. Draw each chart with `/statechart` on `Charts.tla`, with its `.cfg` and its `.mjs` as the view.
   Render it as the skill says, and read it. All rules must hold in each chart.

| Chart | Starts | Shows |
| --- | --- | --- |
| chart | with no message | a message links on its own, or waits on Messages to match or Filtered for a physician to link it or leave it unlinked |
| chart-move | linked to patient A, open for a physician who may access only A | you move it to patient B or to not a patient, then press Undo or let Undo go away, and the patient writes again while Undo shows |

Each `.mjs` says what its chart leaves out.

## Bounds and assumptions

- The model header gives the scope: two patients, two physicians, two messages and one move.
- Which patients each physician may access is open in P2. It is an input, and TLC sets it in every way in which each patient is someone's. Access does not change during a run.
- Which physicians see a message before it is linked is also open in P2. The model lets a physician act on it, and checks rule 2 for every physician.
- The checks are safety only. No rule says that something must happen in the end.
- TLC covers every state of the finite model. It does not reach the code.

## Promises tests hold

These rest on stored records or the channel, which the model can't hold:

- Coda reads each message one time, however often the channel delivers it, and saves it before it moves on.
- Each practice's channel has one reader at a time.
- Moved messages are marked entered in error and never deleted, and what Coda wrote from them doesn't show again.
- One unreadable message, or one patient whose linking fails, never stops the others.
- While Coda can't read the channel, Today says so (rule 7).
- A completed disconnect leaves no message that was never linked to a patient (rule 8).

## Checks seen failing

Each rule fails on a broken copy of the model, at the scope of `MessageMatching.cfg`. The steps
are TLC's shortest path to the break:

| # | Rule | Broken copy | Steps |
| --- | --- | --- | --- |
| 1 | OnlyWithAccess | a move leaves the conversation open for a physician who may not access the new patient | 4 |
| 2 | SuggestionsWithAccess | Messages to match shows the suggestion to every physician | 2 |
| 3 | LinksOnItsOwn | a sender confirmed for two patients links the message to one of them | 2 |
| 4 | RightPatient | a move to the other patient leaves the conversation showing on the patient it left | 3 |
| 5 | NeverInvisible | linking a message does not show the conversation on its patient | 2 |
| 6 | UndoPutsBack | Coda acts on the move at once, while Undo shows | 3 |

## A failure

- **TLC breaks a rule.** The report gives the shortest path. If each step is what the README says,
  the design breaks the rule: take the path to Dev, and never make the rule weaker. If a step can't
  happen by the README, the model is wrong: fix that action from the README's words.
- **The code is different from the model on a path.** If the code does what the README says and the
  model doesn't, fix the model from the README's words. If the code is different from both, it is a
  bug: play the path as a test, see it fail, then fix the code.
- **An intended change.** Change the README with Dev first, then the model, then the code.
