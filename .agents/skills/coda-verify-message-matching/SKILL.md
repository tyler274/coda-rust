---
name: coda-verify-message-matching
description: "Check a change to message matching against its TLA+ model with the pstack /statechart skill: each README rule checked by TLC and seen failing on its own broken copy, then the six charts read in the physician's words. Use when a change touches the message-matching README, MessageMatching.tla, its configs or charts, or the code that reads the channel, decides, links or moves patient messages."
---

# Verify message matching

The spec is [verification/physician/message-matching/README.md](../../../verification/physician/message-matching/README.md).
[MessageMatching.tla](../../../verification/physician/message-matching/MessageMatching.tla), with
[MessageMatching.cfg](../../../verification/physician/message-matching/MessageMatching.cfg), holds
rules 1 to 14. Each `INVARIANT` or `PROPERTY` quotes its README rule. The model follows the README's
three graphs, States and steps: each step is one action, and each state one value.

| # | Rule | The harm when it breaks |
| --- | --- | --- |
| 1 | OnlyWithAccess | A physician sees a conversation of a patient they may not access. |
| 2 | SuggestionsWithAccess | "Messages to match" names a patient the viewer may not access. |
| 3 | JoinsItsPatient | A message in a linked conversation joins a different patient, so one patient's words go into another patient's record. |
| 4 | LinksOnItsOwn | A conversation links on its own to a patient that nothing confirmed, or that something else contradicts. |
| 5 | RightPatient | A moved conversation still shows on the patient it left. |
| 6 | NeverInvisible | A patient message shows to nobody: it stays undecided, or it counts as handled when nobody handled it. |
| 7 | UndoPutsBack | Undo leaves the conversation or its messages in the wrong place, or goes back only to the patient between two moves. |
| 8 | ActsAfterUndo | Coda changes records for a move that the physician then undoes, acts on a move that moved nothing, or never acts on a move that stands. |
| 9 | OnlyBetweenTheirPatients | A physician puts a message or a conversation on a patient they may not access, or takes it from one. |
| 10 | TodaySaysSo | Today hides that Coda can't read the channel, or asks you to reconnect when that won't help. |
| 11 | ClearedOnDisconnect | A completed disconnect leaves a message that was never linked to a patient. |
| 12 | ExactlyOnce | Coda keeps a message twice, or never keeps one it could read. |
| 13 | CantReadShows | A message Coda can't read shows nowhere, or goes away without being read or marked Done. |
| 14 | BacklogOnlyWhenAccepted | Coda reads messages from before it connected without your consent. |

The README's rules after rule 14 rest on the code and its records, not on the order of steps, so the
model doesn't check them. No test holds them yet. They stay unverified until the channel and matching
code and its tests exist.

## Coverage

The sync commit is 0065473e. It is the reference for full qualification, not evidence that a later
revision passed. No product code carries the model's steps. When code does, each action's comment
names that code, and the code joins "When a change needs this". Move the sync commit forward only
after full qualification, as `/maintain-formal-verification-skill` describes.

## When it's verified

Follow [local model runs](../../../verification/README.md#local-model-runs) for correction checks,
full qualification, resource limits and stopping conditions. Name the checked commit and keep
unfinished cases visible in the PR's evidence. The default configuration checks all fourteen rules;
full qualification also covers every break, chart and enlarged bound below.

## When a change needs this

Run it when a change touches the message-matching README, `MessageMatching.tla`,
`MessageMatching.cfg`, `Charts.tla`, or a chart's `.cfg` or `.mjs`.

## Run

1. Check the model with TLC: `MessageMatching.tla` with `MessageMatching.cfg`. It passes when all
   fourteen rules hold. Use `/statechart` with the local resource limits; the full state space
   exceeds the chart budget, so it checks the model without drawing a chart.
2. Select and explain correction breaks as [local model runs](../../../verification/README.md#local-model-runs)
   describes. Run them on scratch copies: against the rule alone, which must fail, and against all the other
   rules, which must hold. Full qualification runs every break. A new rule gets a new break.
   Preserve each exact scratch model and configuration, the command, its
   exit status and the complete TLC output, including the counterexample for the intended failure.
   Summary tables alone do not establish these checks.
3. Check that each step in the README's tables is an action in the model, and each action a step.
4. For full qualification, run the model one size larger on each constant in turn:
   three patients, three physicians, three messages, three moves. Each run must find nothing.
   Preserve each enlarged configuration and its command, exit status and complete TLC output.
   Agree the execution budget first; an incomplete run is not a pass.
5. Draw affected charts with `/statechart` on `Charts.tla`, with their `.cfg` and `.mjs` as the view.
   Full qualification draws every chart.
   Render it as the skill says, and read it. All rules must hold in each chart.

| Chart | Starts | Shows |
| --- | --- | --- |
| chart | with no message | a message links on its own, or waits on "Messages to match" or "Filtered" for a physician to link it or leave it unlinked |
| chart-move | linked to patient A, open for a physician who may access only A | you move it to patient B or to not a patient, then press Undo or let Undo go away, and the patient writes again while Undo shows |
| chart-move-again | as chart-move | you move it, then move it again while Undo shows or after Undo goes away |
| chart-channel | with no message | the channel works, stops for a while or is revoked, comes back, or is disconnected and connected again, with what Today says |
| chart-cant-read | with no message | Coda can't read the message, and reads it later or you mark it Done |
| chart-backlog | Coda has kept one message, not linked yet | you disconnect and connect again, Coda offers the backlog, and you accept it |

Each `.mjs` says what its chart leaves out.

## Bounds and assumptions

- The model header gives the scope: two patients, two physicians, two messages and two moves.
- Only the physician whose Undo shows can move the conversation again in those few seconds. Another
  physician moving it then is not modeled.
- The config declares no symmetry, so that TLC checks the time properties on every state.
- Which patients each physician may access is an input, and TLC sets it in every way in which each
  patient is someone's. Access does not change during a run. Any physician may act on a message
  before it is linked and may disconnect the channel, so rule 2 is checked for every physician.
- Time is fairness: 15 minutes always pass for a message that Coda has not decided about, Undo always
  goes away unless someone presses it, a channel Coda can't read for a while comes back, and Coda
  keeps reading while it can. The model doesn't count seconds or minutes.
- The model stores one link for each conversation, so it can't express messages of one conversation
  on two patients. The code must keep this.
- What Coda wrote from a conversation shows wherever the conversation shows, so rule 5 covers it with
  the conversation. The code must hide both at once.
- TLC covers every state of the finite model. It does not reach the code.

## Each rule fails on its own

| Rule | Break | Expected: rule alone | Expected: other rules |
| --- | --- | --- | --- |
| OnlyWithAccess | a move leaves it open for a physician who may not access the new patient | fails | hold |
| SuggestionsWithAccess | "Messages to match" shows the suggestion to every physician | fails | hold |
| JoinsItsPatient | while no Undo shows, a later message moves a linked conversation to another patient its viewers may access | fails | hold |
| LinksOnItsOwn | a sender confirmed for two patients links the message to one of them | fails | hold |
| RightPatient | a move leaves the conversation showing on the patient it left | fails | hold |
| NeverInvisible | the 15-minute fallback is missing, so a message can wait for Coda's decision for ever | fails | hold |
| UndoPutsBack | Undo takes back only the first message | fails | hold |
| ActsAfterUndo | Coda acts on the move at once, while Undo shows | fails | hold |
| OnlyBetweenTheirPatients | a physician links it to a patient they may not access | fails | hold |
| TodaySaysSo | Today asks you to "Reconnect" when Coda can't read the channel for a while | fails | hold |
| ClearedOnDisconnect | a disconnect deletes only waiting messages | fails | hold |
| ExactlyOnce | Coda moves on past a message before it keeps it | fails | hold |
| CantReadShows | Coda skips a message it can't read, so it shows nowhere | fails | hold |
| BacklogOnlyWhenAccepted | Coda reads the backlog when it only offered it | fails | hold |

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
