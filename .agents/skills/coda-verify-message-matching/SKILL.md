---
name: coda-verify-message-matching
description: "Check a change to message matching against its TLA+ model with the pstack /statechart skill: each modeled README rule checked by TLC, each journey line broken once to see a rule catch it, then the two charts read in the physician's words. Use when a change touches the message-matching README, MessageMatching.tla, its configs or charts, or the code that decides, links or moves patient messages."
---

# Verify message matching

The spec is [verification/physician/message-matching/README.md](../../../verification/physician/message-matching/README.md).
[MessageMatching.tla](../../../verification/physician/message-matching/MessageMatching.tla), with
[MessageMatching.cfg](../../../verification/physician/message-matching/MessageMatching.cfg), holds
rules 1 to 7. Each `INVARIANT` or `PROPERTY` quotes its README rule.

| # | Rule | The harm when it breaks |
| --- | --- | --- |
| 1 | OnlyWithAccess | A physician sees a conversation of a patient they may not access. |
| 2 | SuggestionsWithAccess | Messages to match names a patient the viewer may not access. |
| 3 | LinksOnItsOwn | A message joins a patient that nothing confirmed, or a linked conversation moves to a different patient without a physician, so one patient's words go into another patient's record. |
| 4 | RightPatient | A moved conversation still shows on the patient it left. |
| 5 | NeverInvisible | A patient message shows to nobody: it stays undecided, or it counts as handled when nobody handled it. |
| 6 | UndoPutsBack | Undo leaves the conversation or its messages in the wrong place, Undo never goes away, or Coda changes records for a move that the physician then undoes. |
| 7 | OnlyBetweenTheirPatients | A physician puts a message or a conversation on a patient they may not access, or takes it from one. |

Rules 8 and 9 (the channel stops) are not modeled. They are not order problems, so tests hold them.

## Coverage

The model describes the README as of the last commit that changed `MessageMatching.tla`. That
commit is the sync commit. No product code carries the model's steps. When code does, each action's
comment names that code, and the code joins "When a change needs this". Move the sync commit forward
only after the check passes and each journey break below is caught again, on a README or code that
changed since it, as `/maintain-formal-verification-skill` describes.

## When a change needs this

Run it when a change touches the message-matching README, `MessageMatching.tla`,
`MessageMatching.cfg`, `Charts.tla`, `chart.cfg`, `chart.mjs`, `chart-move.cfg` or `chart-move.mjs`.

## Run

1. Check the model with the pstack `/statechart` skill: `MessageMatching.tla` with
   `MessageMatching.cfg`. It passes when the report lists all seven rules and NoDeadlock under the
   rules that hold. TLC reaches 1,997 states in about 3 seconds. That is more than the chart budget,
   so this run draws no chart.
2. When the README or the model changed, break the model once for each journey line in the table
   below, in a scratch copy. Run each break against each rule on its own. Each break must make the
   rule in its row fail. A break that no rule catches means the model or a rule doesn't cover that
   line: take it to Dev, and don't call the change verified. A new journey line gets a new break.
3. Draw each chart with `/statechart` on `Charts.tla`, with its `.cfg` and its `.mjs` as the view.
   Render it as the skill says, and read it. All rules must hold in each chart.

| Chart | Starts | Shows |
| --- | --- | --- |
| chart | with no message | a message links on its own, or waits on Messages to match or Filtered for a physician to link it or leave it unlinked |
| chart-move | linked to patient A, open for a physician who may access only A | you move it to patient B or to not a patient, then press Undo or let Undo go away, and the patient writes again while Undo shows |

Each `.mjs` says what its chart leaves out.

## Bounds and assumptions

- The model header gives the scope: two patients, two physicians, two messages and one move.
- The config declares no symmetry, so that TLC checks the time properties in rules 5 and 6 on every state.
- Which patients each physician may access is open in P2. It is an input, and TLC sets it in every way in which each patient is someone's. Access does not change during a run.
- Which physicians see a message before it is linked is also open in P2. The model lets a physician act on it, and checks rule 2 for every physician. Who sees Messages to match follows P2.
- Time is fairness: 15 minutes always pass for a message that Coda has not decided about, and Undo always goes away unless someone presses it. The model doesn't count seconds or minutes.
- The model stores one link for each conversation, so it can't express messages of one conversation on two patients. The code must keep this, and its browser test checks it.
- TLC covers every state of the finite model. It does not reach the code.

## Journey lines and their checks

Each modeled journey line has a break: a mistake that code could make. The rule in each row catches
its break at the scope of `MessageMatching.cfg`:

| README section | Journey line | Break | Caught by |
| --- | --- | --- | --- |
| Coda decides | Until Coda decides, or for 15 minutes, the message shows nowhere | the 15-minute fallback is missing, so a message can wait for Coda's decision for ever | 5 |
| Coda decides | A message Coda thinks is not from a patient goes to the Filtered list | Coda drops it, so it shows on no list | 5 |
| Coda decides | This is a patient brings it back, and Coda links it as a patient's message | This is a patient links it to any patient, past Coda's linking | 3 |
| Linking | Links on its own: sender confirmed for exactly one patient | a sender confirmed for two patients links to one of them | 3 |
| Linking | When the sender or the channel points to a different patient, it waits | Coda links to the sender's patient when the channel points to another | 3 |
| Linking | Messages wait with Coda's suggestion | the suggestion shows to every physician | 2 |
| Linking | A physician links it to a patient they may access | they link it to a patient they may not access | 7 |
| Linking | The conversation is then linked, and its other waiting messages link too | the other waiting message stays waiting | 3 |
| Linking | Linking shows the conversation on its patient | it shows nowhere | 5 |
| Linking | Leaving a message unlinked covers only that message | every waiting message is left unlinked | 5 |
| Linking | The next message in the conversation waits again | the next message is left unlinked too | 5 |
| Linking | Later messages in a linked conversation link to the same patient | a later message relinks the conversation to its sender's patient | 3 |
| Wrong patient | A physician who may access the patient moves it | one who may not access it moves it | 7 |
| Wrong patient | to a different patient they may access | they move it to a patient they may not access | 7 |
| Wrong patient | The full conversation moves | a move to not a patient moves only the first message | 5 |
| Wrong patient | At once, nothing of it shows on the patient it left | it still shows there | 4 |
| Wrong patient | A physician who may not access the new patient loses it | it stays open for them | 1 |
| Wrong patient | For a few seconds, Undo shows | Undo never goes away | 6 |
| Wrong patient | Coda acts on the move only after Undo goes away | Coda acts at once | 6 |
| Wrong patient | Undo puts everything back: where it shows | it still shows on the new patient | 6 |
| Wrong patient | Undo puts everything back: its messages | the messages stay not a patient | 6 |
| Wrong patient | On Undo, a physician who may not access that patient loses it | it stays open for them | 1 |
| Wrong patient | A message that arrives while Undo shows follows the conversation | it goes to the patient it left | 4 |
| Wrong patient | Undo takes that message back with the conversation | Undo takes back only the first message | 6 |

These journey lines rest on records, the channel or the screen, so tests hold them:

- Reading the channel: Coda never sends in it; it starts with new messages and offers the backlog; it reads each message one time and saves it before it moves on; one reader for each channel; catching up and retries never show; one failure never stops the others.
- The Spruce tie: one outside participant, one contact, one EHR record, one Coda patient.
- Scores never show as confidence.
- A review never reads an unlinked message.
- Coda never deletes moved messages, and nothing it wrote from them shows again.
- A message Coda can't read, and the channel stops (rules 8 and 9).

## A failure

- **TLC breaks a rule.** The report gives the shortest path. If each step is what the README says,
  the design breaks the rule: take the path to Dev, and never make the rule weaker. If a step can't
  happen by the README, the model is wrong: fix that action from the README's words.
- **A break passes every rule.** The model or a rule doesn't cover that journey line. Take it to
  Dev with the break. Fix the model if the rule's words already cover the line; change a rule only
  with Dev.
- **The code is different from the model on a path.** If the code does what the README says and the
  model doesn't, fix the model from the README's words. If the code is different from both, it is a
  bug: play the path as a test, see it fail, then fix the code.
- **An intended change.** Change the README with Dev first, then the model, then the code.
