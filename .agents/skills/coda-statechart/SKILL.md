---
name: coda-statechart
description: "Design one Coda journey from the intent specs as one model; check its rules and outcomes by searching every state, prove it with Verus, and hold the code to it. Invoke explicitly for journey design or model/code reconciliation."
disable-model-invocation: true
---

# Model one journey

Follow the [shared principles](../../../verification/README.md#principles).

One model holds the journey. Its rules are the sentences Dev agrees to, about what
the person using Coda sees, and its outcomes are what the spec walks through. The
spec in words lives only in the journey's README; everything checkable lives in the
model. One model answers one journey question, such as
"how does a physician sign in?". Label each screen with what is on it, such as
"Check your email", and each step with what the person does or sees happen, such as
"open the email button". Keep code names out of labels.

## Start from the spec

1. Read the journey's section of the [intent specs](../../../intent/README.md) and
   [PRINCIPLES.md](../../../intent/PRINCIPLES.md), and agree the behavior and its
   rules with Dev. A step nobody has decided stays out and is named in the hand-off;
   a product choice goes to DECISIONS.md and Dev. Existing code is evidence about the
   implementation; missing code never narrows the model. Once Dev agrees the model,
   its folder is the journey's spec: the folder's README says the behavior and the
   rules in plain words, the model carries them, and the product spec keeps a short
   summary that links to the folder.
2. A screen or step belongs when the reader sees or controls it, it is a limit on
   access, consent, data or communication, another party depends on it, or it orders
   what happens. Judgment, memory, prompts, schedules and retries are the agent
   harness, and stay out.
3. The model is the design. Add what the rules depend on: a stored value, a request
   in flight and what it carries, another tab or party, each as its own region. Let
   what we don't control, such as other tabs, Coda's answers and expiry, happen
   whenever it can; decide the rest from the spec and first principles, giving each
   fact one owner. When several of one actor take part, such as two tabs, write the
   actor once and generate each copy from it, so whatever one can do the other can
   too and the search finds the orders nobody listed. Write each case from the
   spec's words; never copy a step from the code, because a model that describes
   the code can only agree with it. Once code exists, each step's `code` note says
   where the code carries it out, and the code follows the model, never the
   reverse. State what the model takes as one step in its header.

## Write it

The model is `<journey>.mjs`, exporting `machine` built with
[model.mjs](../../../verification/model.mjs) and run by
[statechart.mjs](../../../verification/statechart.mjs).

4. List each region's states, the first one initial; regions are flat and each holds
   one state at a time. Give each event who causes it (`by`), its intent in plain
   words, and, for code, the `code` place. Write its `cases` in order: the first
   whose `when` holds applies, and `set` gives each region it changes a state, or a
   map from the region's current state to its next. Guards are `is`, `not`, `all`
   and `any`.
5. In `never`, write one rule per agreed sentence: the states that must never hold
   together, by region (a list where any of several states breaks it), and the
   sentence itself. The rule must say the whole sentence. When the model has no
   region for what the sentence names, such as "the patient you opened while signed
   out", add one that only records it: the rule reads it, and no `when` does. When
   a rule can only hold once a tab has heard or a request has answered, add those
   regions' quiet states to the rule and say so in the sentence. Never weaken a rule
   to make the model pass. A promise about a stored value the model can hold becomes
   a rule, with a region for that value. A promise that rests
   on records the model can't hold (identity, single use, collections) goes in the
   folder's README under the rules, naming the test or proof that holds it, or
   saying none does yet.
6. In `invariant`, write the facts that explain why the rules hold, each a guard
   true in every state: what a tab holds while it waits, what news is on its way.
   Verus proves them with the rules, case by case. In `reach`, write each outcome the
   spec walks through: its name and the states the model must pass through, in
   order. The search finds the path, so no file lists steps by hand.

Keep each journey in its own folder under `verification/<product>/`: its README and
its model, and nothing else in words.

## Check it

From the repository root:

7. `node verification/statechart.mjs check MODEL.mjs` reaches every state, prints
   the shortest path to each broken rule or false fact, and finds each outcome. A broken rule is a
   design bug: change the design at the fact's owner, and take a
   product choice to Dev. Then hold the code to the model: for each place the code
   differs, write a test that fails on the code, and change the code.
8. `node verification/statechart.mjs verus MODEL.mjs --module NAME` writes the Verus
   model next to it; then `python3 verification/check.py formal --spec
   .../NAME_proof.rs` proves the facts and rules hold at the start and survive every
   case of every event, so they hold in every state. A case that doesn't keep them
   means a fact is missing or wrong: strengthen the facts, never the proof's
   assumptions. `verus ... --check` fails when the model and the generated files
   differ.
9. A check you have never seen fail proves nothing. In `controls`, give each rule
    sentence one change a plausible bug would make to the design: `{ sentence,
    breaks, change(events) }`, where `breaks` says the bug in the reader's words and
    `change` edits a copy of the events. `check` makes each change and fails unless
    a rule with that sentence breaks; the model refuses a sentence with no control.
    A control that can't break its rule means the rule doesn't say its sentence:
    go back to step 5.
10. Hold the code to the model in the product: `node verification/statechart.mjs
    walks MODEL.mjs` covers every case of every event in short walks, and the
    journey's browser test plays each walk, doing what each event names and checking
    after every step that the page looks as the model says. Name that test in the
    folder's README.
11. Replay each bug path against the code, one step per event:
    `walks MODEL.mjs --paths '[["event", ...]]'` gives the model's state after each
    step, and the journey's bug-path browser test plays it through the same driver.
    Keep the path there once it fails on the code; it passes once the code is fixed.
    If a path doesn't fail on the code, assume the model is wrong until you show it
    isn't, and fix the model, not the rule.

## Hand off

For each bug path, give one line: the rule it breaks, its steps as `check` numbers
them, and whether it reproduced. Say what the model leaves out: what it takes as one
step, the timing it assumes, record promises no test holds yet, and anything not
checked against code. A checked draft is still a draft until Dev agrees with it.

When the code a model's notes point to changes, re-read it against the model, run
the walks and the bug paths, and say for each rule whether the model is in sync,
updated, or found a bug in the code.

Adapted from PStack's [statechart](https://github.com/cursor/plugins/blob/519d36aa98955bf680db01bcde5fdfba686dc901/pstack/skills/statechart/SKILL.md)
(rules as the reader's sentences, never weakened; bug paths replayed against code),
[create-formal-verification](https://github.com/cursor/plugins/blob/519d36aa98955bf680db01bcde5fdfba686dc901/pstack/skills/create-formal-verification-skill/SKILL.md)
(able to fail) and
[maintain-formal-verification](https://github.com/cursor/plugins/blob/519d36aa98955bf680db01bcde5fdfba686dc901/pstack/skills/maintain-formal-verification-skill/SKILL.md)
(in sync, updated or bug).
